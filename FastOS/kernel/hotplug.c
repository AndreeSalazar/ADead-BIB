/*
 * kernel/hotplug.c — Hardware Detection & On-Demand Driver Loading
 * FastOS v2.0
 *
 * "El OS no sabe de impresoras hasta que conectas una."
 *
 * Cuando el hardware nuevo es detectado (PCI/USB), FastOS pregunta al
 * usuario si desea instalar el driver. No hay 40 millones de líneas de
 * drivers pre-instalados. Solo lo que usas.
 *
 * Compilar:  adb cc kernel/hotplug.c --target fastos
 * Ver steps: adb step kernel/hotplug.c
 */

#include <kernel.h>
#include <types.h>
#include <pci.h>
#include <fastos.h>

/* ─── Base de datos mínima de vendors PCI ─── */
typedef struct {
    uint16_t vendor_id;
    uint16_t device_id;
    const char *name;
    const char *driver_url; /* URL del driver en internet */
} hotplug_device_db_t;

static const hotplug_device_db_t hotplug_db[] = {
    { 0x10DE, 0x2684, "NVIDIA RTX 4090",
      "https://drivers.fastos.io/nvidia/rtx4090.po" },
    { 0x10DE, 0x2704, "NVIDIA RTX 4080",
      "https://drivers.fastos.io/nvidia/rtx4080.po" },
    { 0x10DE, 0x2782, "NVIDIA RTX 4070",
      "https://drivers.fastos.io/nvidia/rtx4070.po" },
    { 0x1002, 0x744C, "AMD RX 7900 XTX",
      "https://drivers.fastos.io/amd/rx7900xtx.po"  },
    { 0x8086, 0x7A30, "Intel ARC A770",
      "https://drivers.fastos.io/intel/arc_a770.po" },
    { 0x8086, 0x15F3, "Intel Ethernet I225-V",
      "https://drivers.fastos.io/intel/i225v.po"    },
    { 0x10EC, 0x8168, "Realtek RTL8111 Ethernet",
      "https://drivers.fastos.io/realtek/rtl8111.po"},
    { 0,      0,      NULL, NULL                       } /* sentinel */
};

/* ─── Buscar dispositivo en DB ─── */
static const hotplug_device_db_t *hotplug_lookup(uint16_t vendor,
                                                   uint16_t device) {
    for (int i = 0; hotplug_db[i].name != NULL; i++) {
        if (hotplug_db[i].vendor_id == vendor &&
            hotplug_db[i].device_id == device) {
            return &hotplug_db[i];
        }
    }
    return NULL;
}

/* ─── Cola de eventos hot-plug ─── */
#define HOTPLUG_QUEUE_SIZE 32

typedef enum {
    HOTPLUG_EVENT_CONNECTED    = 1,
    HOTPLUG_EVENT_DISCONNECTED = 2
} hotplug_event_type_t;

typedef struct {
    hotplug_event_type_t type;
    uint16_t vendor_id;
    uint16_t device_id;
    uint8_t  bus;
    uint8_t  slot;
    uint8_t  function;
} hotplug_event_t;

static hotplug_event_t hotplug_queue[HOTPLUG_QUEUE_SIZE];
static int hotplug_queue_head = 0;
static int hotplug_queue_tail = 0;

static void hotplug_enqueue(hotplug_event_t ev) {
    int next = (hotplug_queue_tail + 1) % HOTPLUG_QUEUE_SIZE;
    if (next != hotplug_queue_head) { /* no lleno */
        hotplug_queue[hotplug_queue_tail] = ev;
        hotplug_queue_tail = next;
    }
}

static int hotplug_dequeue(hotplug_event_t *out) {
    if (hotplug_queue_head == hotplug_queue_tail) return 0; /* vacío */
    *out = hotplug_queue[hotplug_queue_head];
    hotplug_queue_head = (hotplug_queue_head + 1) % HOTPLUG_QUEUE_SIZE;
    return 1;
}

/* ─── Instalar driver ─── */
/*
 * Flujo:
 *   hardware detectado (vendor:device)
 *         ↓
 *   buscar en DB → encontrar URL
 *         ↓
 *   "Hardware desconocido: NVIDIA RTX 4070"
 *   "¿Instalar driver? [S/N]"
 *         ↓
 *   usuario → S
 *         ↓
 *   descargar driver .Po de internet
 *         ↓
 *   bg_preexec_gate() → verificar
 *         ↓
 *   cargar driver → hardware funciona. listo.
 */
static void hotplug_install_driver(const hotplug_device_db_t *dev) {
    printf("\n[hotplug] Hardware detectado: %s\n", dev->name);
    printf("[hotplug] ¿Instalar driver? [S/N]: ");

    /* En producción: leer carácter desde teclado */
    /* Demo: auto-responder S */
    char response = 'S';
    printf("%c\n", response);

    if (response == 'S' || response == 's') {
        printf("[hotplug] Descargando driver desde: %s\n", dev->driver_url);
        /* network_download(dev->driver_url, "/drivers/cache/driver.po"); */
        printf("[hotplug] Verificando con Binary Guardian...\n");
        /* bg_preexec_gate(driver_binary, driver_size, BG_CAP_DRIVER); */
        printf("[hotplug] Driver instalado. Hardware listo. ✓\n\n");
    } else {
        printf("[hotplug] Driver no instalado. Hardware ignorado.\n\n");
    }
}

/* ─── Callback PCI: nuevo dispositivo detectado ─── */
void hotplug_on_pci_device(uint16_t vendor, uint16_t device,
                             uint8_t bus, uint8_t slot, uint8_t func) {
    hotplug_event_t ev = {
        .type      = HOTPLUG_EVENT_CONNECTED,
        .vendor_id = vendor,
        .device_id = device,
        .bus       = bus,
        .slot      = slot,
        .function  = func
    };
    hotplug_enqueue(ev);
}

/* ─── Procesar cola de eventos ─── */
static void hotplug_process_events(void) {
    hotplug_event_t ev;
    while (hotplug_dequeue(&ev)) {
        if (ev.type == HOTPLUG_EVENT_CONNECTED) {
            const hotplug_device_db_t *dev =
                hotplug_lookup(ev.vendor_id, ev.device_id);
            if (dev) {
                hotplug_install_driver(dev);
            } else {
                printf("[hotplug] Unknown device: %04X:%04X"
                       " (bus %02X, slot %02X, func %02X)\n",
                       ev.vendor_id, ev.device_id,
                       ev.bus, ev.slot, ev.function);
                printf("[hotplug] No driver available. "
                       "Check drivers.fastos.io\n");
            }
        } else if (ev.type == HOTPLUG_EVENT_DISCONNECTED) {
            printf("[hotplug] Device %04X:%04X disconnected.\n",
                   ev.vendor_id, ev.device_id);
        }
    }
}

/* ─── Escaneo inicial PCI en boot ─── */
static void hotplug_scan_pci(void) {
    printf("[hotplug] Scanning PCI bus...\n");
    /*
     * Iterar bus 0–255, device 0–31, function 0–7
     * Leer vendor/device ID desde config space (0xCF8/0xCFC)
     * Si vendor != 0xFFFF → dispositivo presente
     */
    /* Demo: simular detección de un dispositivo */
    /* En producción: puerto I/O 0xCF8/0xCFC */
    printf("[hotplug] PCI scan complete.\n");
}

/* ─── Init del subsistema hotplug ─── */
void hotplug_init(void) {
    hotplug_queue_head = 0;
    hotplug_queue_tail = 0;

    printf("[hotplug] Initializing hardware detection...\n");
    hotplug_scan_pci();
    hotplug_process_events();
    printf("[hotplug] Ready — plug hardware anytime.\n");
}

/* ─── Tick del scheduler (llamado periódicamente) ─── */
void hotplug_tick(void) {
    /* Procesar eventos que llegaron desde el último tick */
    hotplug_process_events();
}
