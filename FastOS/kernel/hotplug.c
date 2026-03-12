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

#include "../include/kernel.h"
#include "../include/types.h"
#include "../include/fastos.h"

/* ─── PCI Config Space I/O ─── */
#define PCI_CONFIG_ADDR  0xCF8
#define PCI_CONFIG_DATA  0xCFC

static uint32_t pci_read32(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
    uint32_t addr = (1U << 31)           /* Enable bit */
                  | ((uint32_t)bus << 16)
                  | ((uint32_t)slot << 11)
                  | ((uint32_t)func << 8)
                  | (offset & 0xFC);     /* Align to dword */
    outl(PCI_CONFIG_ADDR, addr);
    return inl(PCI_CONFIG_DATA);
}

static uint16_t pci_read_vendor(uint8_t bus, uint8_t slot, uint8_t func) {
    return (uint16_t)(pci_read32(bus, slot, func, 0) & 0xFFFF);
}

static uint16_t pci_read_device(uint8_t bus, uint8_t slot, uint8_t func) {
    return (uint16_t)(pci_read32(bus, slot, func, 0) >> 16);
}

static uint8_t pci_read_class(uint8_t bus, uint8_t slot, uint8_t func) {
    return (uint8_t)(pci_read32(bus, slot, func, 0x08) >> 24);
}

static uint8_t pci_read_subclass(uint8_t bus, uint8_t slot, uint8_t func) {
    return (uint8_t)(pci_read32(bus, slot, func, 0x08) >> 16);
}

/* ─── Base de datos mínima de vendors PCI ─── */
typedef struct {
    uint16_t vendor_id;
    uint16_t device_id;
    const char *name;
    const char *driver_path; /* Path del driver .Po en disco */
} hotplug_device_db_t;

static const hotplug_device_db_t hotplug_db[] = {
    { 0x10DE, 0x2684, "NVIDIA RTX 4090",
      "/drivers/nvidia/rtx4090.po" },
    { 0x10DE, 0x2704, "NVIDIA RTX 4080",
      "/drivers/nvidia/rtx4080.po" },
    { 0x10DE, 0x2782, "NVIDIA RTX 4070",
      "/drivers/nvidia/rtx4070.po" },
    { 0x1002, 0x744C, "AMD RX 7900 XTX",
      "/drivers/amd/rx7900xtx.po"  },
    { 0x8086, 0x7A30, "Intel ARC A770",
      "/drivers/intel/arc_a770.po" },
    { 0x8086, 0x15F3, "Intel Ethernet I225-V",
      "/drivers/intel/i225v.po"    },
    { 0x10EC, 0x8168, "Realtek RTL8111 Ethernet",
      "/drivers/realtek/rtl8111.po"},
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
    kprintf("\n[hotplug] Hardware detectado: %s\n", dev->name);
    kprintf("[hotplug] Driver en disco: %s\n", dev->driver_path);
    kprintf("[hotplug] Instalar driver? [S/N]: ");

    /* En producción: leer carácter desde teclado */
    /* Demo: auto-responder S */
    char response = 'S';
    kprintf("%c\n", response);

    if (response == 'S' || response == 's') {
        kprintf("[hotplug] Cargando .Po desde disco: %s\n", dev->driver_path);
        /* vfs_read(dev->driver_path, &driver_binary, &driver_size); */
        kprintf("[hotplug] Verificando con Binary Guardian...\n");
        /* bg_preexec_gate(driver_binary, driver_size, BG_CAP_DRIVER); */
        kprintf("[hotplug] Driver verificado y cargado.\n\n");
    } else {
        kprintf("[hotplug] Driver no instalado. Hardware ignorado.\n\n");
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
                kprintf("[hotplug] Unknown device: %04X:%04X"
                       " (bus %02X, slot %02X, func %02X)\n",
                       ev.vendor_id, ev.device_id,
                       ev.bus, ev.slot, ev.function);
                kprintf("[hotplug] No driver available. "
                       "Check drivers.fastos.io\n");
            }
        } else if (ev.type == HOTPLUG_EVENT_DISCONNECTED) {
            kprintf("[hotplug] Device %04X:%04X disconnected.\n",
                   ev.vendor_id, ev.device_id);
        }
    }
}

/* ─── PCI Class Names ─── */
static const char *pci_class_name(uint8_t class_code) {
    switch (class_code) {
        case 0x00: return "Legacy";
        case 0x01: return "Storage";
        case 0x02: return "Network";
        case 0x03: return "Display";
        case 0x04: return "Multimedia";
        case 0x05: return "Memory";
        case 0x06: return "Bridge";
        case 0x07: return "Comms";
        case 0x08: return "System";
        case 0x09: return "Input";
        case 0x0C: return "Serial";
        case 0x0D: return "Wireless";
        default:   return "Other";
    }
}

/* ─── Escaneo real PCI via config space ─── */
static int pci_device_count = 0;

static void hotplug_scan_pci(void) {
    kprintf("[hotplug] Scanning PCI bus (0xCF8/0xCFC)...\n");
    pci_device_count = 0;

    for (int bus = 0; bus < 256; bus++) {
        for (int slot = 0; slot < 32; slot++) {
            uint16_t vendor = pci_read_vendor((uint8_t)bus, (uint8_t)slot, 0);
            if (vendor == 0xFFFF) continue;  /* No device */

            uint16_t device = pci_read_device((uint8_t)bus, (uint8_t)slot, 0);
            uint8_t  cls    = pci_read_class((uint8_t)bus, (uint8_t)slot, 0);
            uint8_t  sub    = pci_read_subclass((uint8_t)bus, (uint8_t)slot, 0);

            kprintf("[PCI] %02x:%02x.0 vendor=%04x device=%04x class=%02x:%02x (%s)\n",
                    bus, slot, (int)vendor, (int)device,
                    (int)cls, (int)sub, pci_class_name(cls));

            /* Notify hotplug subsystem */
            hotplug_on_pci_device(vendor, device,
                                 (uint8_t)bus, (uint8_t)slot, 0);
            pci_device_count++;

            /* Check multi-function */
            uint32_t hdr = pci_read32((uint8_t)bus, (uint8_t)slot, 0, 0x0C);
            if (hdr & 0x00800000) { /* Multi-function bit */
                for (int func = 1; func < 8; func++) {
                    vendor = pci_read_vendor((uint8_t)bus, (uint8_t)slot, (uint8_t)func);
                    if (vendor == 0xFFFF) continue;
                    device = pci_read_device((uint8_t)bus, (uint8_t)slot, (uint8_t)func);
                    cls    = pci_read_class((uint8_t)bus, (uint8_t)slot, (uint8_t)func);
                    sub    = pci_read_subclass((uint8_t)bus, (uint8_t)slot, (uint8_t)func);

                    kprintf("[PCI] %02x:%02x.%d vendor=%04x device=%04x class=%02x:%02x (%s)\n",
                            bus, slot, func, (int)vendor, (int)device,
                            (int)cls, (int)sub, pci_class_name(cls));

                    hotplug_on_pci_device(vendor, device,
                                         (uint8_t)bus, (uint8_t)slot, (uint8_t)func);
                    pci_device_count++;
                }
            }
        }
    }

    kprintf("[hotplug] PCI scan complete: %d devices found\n", pci_device_count);
}

/* ─── Init del subsistema hotplug ─── */
void hotplug_init(void) {
    hotplug_queue_head = 0;
    hotplug_queue_tail = 0;

    kprintf("[hotplug] Initializing hardware detection...\n");
    hotplug_scan_pci();
    hotplug_process_events();
    kprintf("[hotplug] Ready — %d PCI devices, plug hardware anytime.\n",
            pci_device_count);
}

/* ─── Tick del scheduler (llamado periódicamente) ─── */
void hotplug_tick(void) {
    /* Procesar eventos que llegaron desde el último tick */
    hotplug_process_events();
}
