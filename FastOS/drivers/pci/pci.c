/*
 * FastOS v2.0 — PCI Bus Driver
 * ADead-BIB Native Operating System
 */

#include "../../include/kernel.h"
#include "../../include/types.h"

/* PCI Configuration Space Ports */
#define PCI_CONFIG_ADDRESS 0xCF8
#define PCI_CONFIG_DATA    0xCFC

/* PCI Configuration Space Offsets */
#define PCI_VENDOR_ID       0x00
#define PCI_DEVICE_ID       0x02
#define PCI_COMMAND         0x04
#define PCI_STATUS          0x06
#define PCI_REVISION_ID     0x08
#define PCI_PROG_IF         0x09
#define PCI_SUBCLASS        0x0A
#define PCI_CLASS           0x0B
#define PCI_CACHE_LINE_SIZE 0x0C
#define PCI_LATENCY_TIMER   0x0D
#define PCI_HEADER_TYPE     0x0E
#define PCI_BIST            0x0F
#define PCI_BAR0            0x10
#define PCI_BAR1            0x14
#define PCI_BAR2            0x18
#define PCI_BAR3            0x1C
#define PCI_BAR4            0x20
#define PCI_BAR5            0x24
#define PCI_INTERRUPT_LINE  0x3C
#define PCI_INTERRUPT_PIN   0x3D

/* PCI Device Classes */
#define PCI_CLASS_UNCLASSIFIED     0x00
#define PCI_CLASS_STORAGE          0x01
#define PCI_CLASS_NETWORK          0x02
#define PCI_CLASS_DISPLAY          0x03
#define PCI_CLASS_MULTIMEDIA       0x04
#define PCI_CLASS_MEMORY           0x05
#define PCI_CLASS_BRIDGE           0x06
#define PCI_CLASS_COMMUNICATION    0x07
#define PCI_CLASS_SYSTEM           0x08
#define PCI_CLASS_INPUT            0x09
#define PCI_CLASS_DOCKING          0x0A
#define PCI_CLASS_PROCESSOR        0x0B
#define PCI_CLASS_SERIAL           0x0C

/* PCI Device Structure */
typedef struct {
    uint8_t  bus;
    uint8_t  slot;
    uint8_t  func;
    uint16_t vendor_id;
    uint16_t device_id;
    uint8_t  class_code;
    uint8_t  subclass;
    uint8_t  prog_if;
    uint8_t  revision;
    uint8_t  header_type;
    uint8_t  interrupt_line;
    uint8_t  interrupt_pin;
    uint32_t bar[6];
} pci_device_t;

/* Maximum devices we can track */
#define PCI_MAX_DEVICES 256
static pci_device_t pci_devices[PCI_MAX_DEVICES];
static int pci_device_count = 0;

/* Read 32-bit value from PCI config space */
uint32_t pci_read32(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
    uint32_t address = (1 << 31) | (bus << 16) | (slot << 11) | 
                       (func << 8) | (offset & 0xFC);
    outl(PCI_CONFIG_ADDRESS, address);
    return inl(PCI_CONFIG_DATA);
}

/* Read 16-bit value from PCI config space */
uint16_t pci_read16(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
    uint32_t val = pci_read32(bus, slot, func, offset);
    return (val >> ((offset & 2) * 8)) & 0xFFFF;
}

/* Read 8-bit value from PCI config space */
uint8_t pci_read8(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
    uint32_t val = pci_read32(bus, slot, func, offset);
    return (val >> ((offset & 3) * 8)) & 0xFF;
}

/* Write 32-bit value to PCI config space */
void pci_write32(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset, uint32_t value) {
    uint32_t address = (1 << 31) | (bus << 16) | (slot << 11) | 
                       (func << 8) | (offset & 0xFC);
    outl(PCI_CONFIG_ADDRESS, address);
    outl(PCI_CONFIG_DATA, value);
}

/* Get class name string */
const char* pci_class_name(uint8_t class_code) {
    switch (class_code) {
        case PCI_CLASS_UNCLASSIFIED:  return "Unclassified";
        case PCI_CLASS_STORAGE:       return "Storage";
        case PCI_CLASS_NETWORK:       return "Network";
        case PCI_CLASS_DISPLAY:       return "Display";
        case PCI_CLASS_MULTIMEDIA:    return "Multimedia";
        case PCI_CLASS_MEMORY:        return "Memory";
        case PCI_CLASS_BRIDGE:        return "Bridge";
        case PCI_CLASS_COMMUNICATION: return "Communication";
        case PCI_CLASS_SYSTEM:        return "System";
        case PCI_CLASS_INPUT:         return "Input";
        case PCI_CLASS_DOCKING:       return "Docking";
        case PCI_CLASS_PROCESSOR:     return "Processor";
        case PCI_CLASS_SERIAL:        return "Serial Bus";
        default:                      return "Unknown";
    }
}

/* Check if device exists */
int pci_device_exists(uint8_t bus, uint8_t slot, uint8_t func) {
    uint16_t vendor = pci_read16(bus, slot, func, PCI_VENDOR_ID);
    return vendor != 0xFFFF;
}

/* Scan a single device */
void pci_scan_device(uint8_t bus, uint8_t slot, uint8_t func) {
    if (!pci_device_exists(bus, slot, func)) return;
    if (pci_device_count >= PCI_MAX_DEVICES) return;
    
    pci_device_t *dev = &pci_devices[pci_device_count++];
    
    dev->bus = bus;
    dev->slot = slot;
    dev->func = func;
    dev->vendor_id = pci_read16(bus, slot, func, PCI_VENDOR_ID);
    dev->device_id = pci_read16(bus, slot, func, PCI_DEVICE_ID);
    dev->class_code = pci_read8(bus, slot, func, PCI_CLASS);
    dev->subclass = pci_read8(bus, slot, func, PCI_SUBCLASS);
    dev->prog_if = pci_read8(bus, slot, func, PCI_PROG_IF);
    dev->revision = pci_read8(bus, slot, func, PCI_REVISION_ID);
    dev->header_type = pci_read8(bus, slot, func, PCI_HEADER_TYPE);
    dev->interrupt_line = pci_read8(bus, slot, func, PCI_INTERRUPT_LINE);
    dev->interrupt_pin = pci_read8(bus, slot, func, PCI_INTERRUPT_PIN);
    
    /* Read BARs */
    for (int i = 0; i < 6; i++) {
        dev->bar[i] = pci_read32(bus, slot, func, PCI_BAR0 + (i * 4));
    }
}

/* Scan all PCI buses */
void pci_scan_all(void) {
    pci_device_count = 0;
    
    for (int bus = 0; bus < 256; bus++) {
        for (int slot = 0; slot < 32; slot++) {
            if (pci_device_exists(bus, slot, 0)) {
                pci_scan_device(bus, slot, 0);
                
                /* Check for multi-function device */
                uint8_t header = pci_read8(bus, slot, 0, PCI_HEADER_TYPE);
                if (header & 0x80) {
                    for (int func = 1; func < 8; func++) {
                        pci_scan_device(bus, slot, func);
                    }
                }
            }
        }
    }
}

/* Find device by vendor/device ID */
pci_device_t* pci_find_device(uint16_t vendor_id, uint16_t device_id) {
    for (int i = 0; i < pci_device_count; i++) {
        if (pci_devices[i].vendor_id == vendor_id &&
            pci_devices[i].device_id == device_id) {
            return &pci_devices[i];
        }
    }
    return NULL;
}

/* Find device by class */
pci_device_t* pci_find_class(uint8_t class_code, uint8_t subclass) {
    for (int i = 0; i < pci_device_count; i++) {
        if (pci_devices[i].class_code == class_code &&
            pci_devices[i].subclass == subclass) {
            return &pci_devices[i];
        }
    }
    return NULL;
}

/* Get device count */
int pci_get_device_count(void) {
    return pci_device_count;
}

/* Get device by index */
pci_device_t* pci_get_device(int index) {
    if (index < 0 || index >= pci_device_count) return NULL;
    return &pci_devices[index];
}

/* ============================================================
 * VENDOR IDs - Comprehensive list for any PC
 * ============================================================ */
#define PCI_VENDOR_NVIDIA      0x10DE
#define PCI_VENDOR_AMD         0x1022
#define PCI_VENDOR_AMD_ATI     0x1002  /* AMD/ATI GPUs */
#define PCI_VENDOR_INTEL       0x8086
#define PCI_VENDOR_REALTEK     0x10EC
#define PCI_VENDOR_SAMSUNG     0x144D
#define PCI_VENDOR_WESTERN_DIG 0x15B7
#define PCI_VENDOR_BROADCOM    0x14E4
#define PCI_VENDOR_QUALCOMM    0x17CB
#define PCI_VENDOR_MARVELL     0x1B4B
#define PCI_VENDOR_ASROCK      0x1849
#define PCI_VENDOR_ASUS        0x1043
#define PCI_VENDOR_MSI         0x1462
#define PCI_VENDOR_GIGABYTE    0x1458
#define PCI_VENDOR_CORSAIR     0x1B1C
#define PCI_VENDOR_KINGSTON    0x2646
#define PCI_VENDOR_CRUCIAL     0xC0A9
#define PCI_VENDOR_SANDISK     0x15B7
#define PCI_VENDOR_SEAGATE     0x1BB1
#define PCI_VENDOR_PHISON      0x1987
#define PCI_VENDOR_SILICON_MOT 0x1095
#define PCI_VENDOR_VIA         0x1106
#define PCI_VENDOR_CREATIVE    0x1102
#define PCI_VENDOR_LOGITECH    0x046D

/* ============================================================
 * NVIDIA GPU Device IDs - All generations
 * ============================================================ */
/* RTX 40 Series (Ada Lovelace) */
#define PCI_DEVICE_RTX4090     0x2684
#define PCI_DEVICE_RTX4080     0x2704
#define PCI_DEVICE_RTX4070TI   0x2782
#define PCI_DEVICE_RTX4070     0x2786
#define PCI_DEVICE_RTX4060TI   0x2860
#define PCI_DEVICE_RTX4060     0x2882

/* RTX 30 Series (Ampere) */
#define PCI_DEVICE_RTX3090TI   0x2203
#define PCI_DEVICE_RTX3090     0x2204
#define PCI_DEVICE_RTX3080TI   0x2208
#define PCI_DEVICE_RTX3080     0x2206
#define PCI_DEVICE_RTX3070TI   0x2482
#define PCI_DEVICE_RTX3070     0x2484
#define PCI_DEVICE_RTX3060TI   0x2486
#define PCI_DEVICE_RTX3060     0x2503
#define PCI_DEVICE_RTX3060_LHR 0x2504
#define PCI_DEVICE_RTX3050     0x2507

/* RTX 20 Series (Turing) */
#define PCI_DEVICE_RTX2080TI   0x1E04
#define PCI_DEVICE_RTX2080S    0x1E82
#define PCI_DEVICE_RTX2080     0x1E07
#define PCI_DEVICE_RTX2070S    0x1E84
#define PCI_DEVICE_RTX2070     0x1F02
#define PCI_DEVICE_RTX2060S    0x1F08
#define PCI_DEVICE_RTX2060     0x1F07

/* GTX 16 Series (Turing) */
#define PCI_DEVICE_GTX1660TI   0x2182
#define PCI_DEVICE_GTX1660S    0x21C4
#define PCI_DEVICE_GTX1660     0x2184
#define PCI_DEVICE_GTX1650S    0x2187
#define PCI_DEVICE_GTX1650     0x1F82

/* GTX 10 Series (Pascal) */
#define PCI_DEVICE_GTX1080TI   0x1B06
#define PCI_DEVICE_GTX1080     0x1B80
#define PCI_DEVICE_GTX1070TI   0x1B82
#define PCI_DEVICE_GTX1070     0x1B81
#define PCI_DEVICE_GTX1060     0x1C03
#define PCI_DEVICE_GTX1050TI   0x1C82
#define PCI_DEVICE_GTX1050     0x1C81

/* ============================================================
 * AMD/ATI GPU Device IDs
 * ============================================================ */
/* RX 7000 Series (RDNA 3) */
#define PCI_DEVICE_RX7900XTX   0x744C
#define PCI_DEVICE_RX7900XT    0x744C
#define PCI_DEVICE_RX7800XT    0x7480
#define PCI_DEVICE_RX7700XT    0x7480
#define PCI_DEVICE_RX7600      0x7480

/* RX 6000 Series (RDNA 2) */
#define PCI_DEVICE_RX6950XT    0x73A5
#define PCI_DEVICE_RX6900XT    0x73BF
#define PCI_DEVICE_RX6800XT    0x73BF
#define PCI_DEVICE_RX6800      0x73BF
#define PCI_DEVICE_RX6700XT    0x73DF
#define PCI_DEVICE_RX6600XT    0x73FF
#define PCI_DEVICE_RX6600      0x73FF

/* RX 5000 Series (RDNA 1) */
#define PCI_DEVICE_RX5700XT    0x731F
#define PCI_DEVICE_RX5700      0x731F
#define PCI_DEVICE_RX5600XT    0x731F
#define PCI_DEVICE_RX5500XT    0x7340

/* ============================================================
 * Intel GPU Device IDs
 * ============================================================ */
#define PCI_DEVICE_ARC_A770    0x56A0
#define PCI_DEVICE_ARC_A750    0x56A1
#define PCI_DEVICE_ARC_A380    0x56A5
#define PCI_DEVICE_IRIS_XE     0x9A49
#define PCI_DEVICE_UHD_630     0x3E92
#define PCI_DEVICE_UHD_770     0x4680

/* Storage subclasses */
#define PCI_SUBCLASS_SATA      0x06
#define PCI_SUBCLASS_NVME      0x08

/* Global GPU and storage info */
static pci_device_t *g_nvidia_gpu = NULL;
static pci_device_t *g_ahci_controller = NULL;
static pci_device_t *g_nvme_controller = NULL;

/* Get vendor name - Comprehensive list */
const char* pci_vendor_name(uint16_t vendor_id) {
    switch (vendor_id) {
        case PCI_VENDOR_NVIDIA:      return "NVIDIA";
        case PCI_VENDOR_AMD:         return "AMD";
        case PCI_VENDOR_AMD_ATI:     return "AMD/ATI";
        case PCI_VENDOR_INTEL:       return "Intel";
        case PCI_VENDOR_REALTEK:     return "Realtek";
        case PCI_VENDOR_SAMSUNG:     return "Samsung";
        case PCI_VENDOR_WESTERN_DIG: return "WD";
        case PCI_VENDOR_BROADCOM:    return "Broadcom";
        case PCI_VENDOR_QUALCOMM:    return "Qualcomm";
        case PCI_VENDOR_MARVELL:     return "Marvell";
        case PCI_VENDOR_ASROCK:      return "ASRock";
        case PCI_VENDOR_ASUS:        return "ASUS";
        case PCI_VENDOR_MSI:         return "MSI";
        case PCI_VENDOR_GIGABYTE:    return "Gigabyte";
        case PCI_VENDOR_CORSAIR:     return "Corsair";
        case PCI_VENDOR_KINGSTON:    return "Kingston";
        case PCI_VENDOR_PHISON:      return "Phison";
        case PCI_VENDOR_VIA:         return "VIA";
        case PCI_VENDOR_CREATIVE:    return "Creative";
        default:                     return "Unknown";
    }
}

/* Get NVIDIA GPU name - All generations */
const char* nvidia_gpu_name(uint16_t device_id) {
    switch (device_id) {
        /* RTX 40 Series */
        case PCI_DEVICE_RTX4090:     return "GeForce RTX 4090";
        case PCI_DEVICE_RTX4080:     return "GeForce RTX 4080";
        case PCI_DEVICE_RTX4070TI:   return "GeForce RTX 4070 Ti";
        case PCI_DEVICE_RTX4070:     return "GeForce RTX 4070";
        case PCI_DEVICE_RTX4060TI:   return "GeForce RTX 4060 Ti";
        case PCI_DEVICE_RTX4060:     return "GeForce RTX 4060";
        
        /* RTX 30 Series */
        case PCI_DEVICE_RTX3090TI:   return "GeForce RTX 3090 Ti";
        case PCI_DEVICE_RTX3090:     return "GeForce RTX 3090";
        case PCI_DEVICE_RTX3080TI:   return "GeForce RTX 3080 Ti";
        case PCI_DEVICE_RTX3080:     return "GeForce RTX 3080";
        case PCI_DEVICE_RTX3070TI:   return "GeForce RTX 3070 Ti";
        case PCI_DEVICE_RTX3070:     return "GeForce RTX 3070";
        case PCI_DEVICE_RTX3060TI:   return "GeForce RTX 3060 Ti";
        case PCI_DEVICE_RTX3060:     return "GeForce RTX 3060";
        case PCI_DEVICE_RTX3060_LHR: return "GeForce RTX 3060 LHR";
        case PCI_DEVICE_RTX3050:     return "GeForce RTX 3050";
        
        /* RTX 20 Series */
        case PCI_DEVICE_RTX2080TI:   return "GeForce RTX 2080 Ti";
        case PCI_DEVICE_RTX2080S:    return "GeForce RTX 2080 Super";
        case PCI_DEVICE_RTX2080:     return "GeForce RTX 2080";
        case PCI_DEVICE_RTX2070S:    return "GeForce RTX 2070 Super";
        case PCI_DEVICE_RTX2070:     return "GeForce RTX 2070";
        case PCI_DEVICE_RTX2060S:    return "GeForce RTX 2060 Super";
        case PCI_DEVICE_RTX2060:     return "GeForce RTX 2060";
        
        /* GTX 16 Series */
        case PCI_DEVICE_GTX1660TI:   return "GeForce GTX 1660 Ti";
        case PCI_DEVICE_GTX1660S:    return "GeForce GTX 1660 Super";
        case PCI_DEVICE_GTX1660:     return "GeForce GTX 1660";
        case PCI_DEVICE_GTX1650S:    return "GeForce GTX 1650 Super";
        case PCI_DEVICE_GTX1650:     return "GeForce GTX 1650";
        
        /* GTX 10 Series */
        case PCI_DEVICE_GTX1080TI:   return "GeForce GTX 1080 Ti";
        case PCI_DEVICE_GTX1080:     return "GeForce GTX 1080";
        case PCI_DEVICE_GTX1070TI:   return "GeForce GTX 1070 Ti";
        case PCI_DEVICE_GTX1070:     return "GeForce GTX 1070";
        case PCI_DEVICE_GTX1060:     return "GeForce GTX 1060";
        case PCI_DEVICE_GTX1050TI:   return "GeForce GTX 1050 Ti";
        case PCI_DEVICE_GTX1050:     return "GeForce GTX 1050";
        
        default:                     return "GeForce GPU";
    }
}

/* Get AMD GPU name */
const char* amd_gpu_name(uint16_t device_id) {
    switch (device_id) {
        /* RX 7000 Series */
        case PCI_DEVICE_RX7900XTX:   return "Radeon RX 7900 XTX";
        case PCI_DEVICE_RX7800XT:    return "Radeon RX 7800 XT";
        case PCI_DEVICE_RX7600:      return "Radeon RX 7600";
        
        /* RX 6000 Series */
        case PCI_DEVICE_RX6950XT:    return "Radeon RX 6950 XT";
        case PCI_DEVICE_RX6900XT:    return "Radeon RX 6900 XT";
        case PCI_DEVICE_RX6700XT:    return "Radeon RX 6700 XT";
        case PCI_DEVICE_RX6600XT:    return "Radeon RX 6600 XT";
        
        /* RX 5000 Series */
        case PCI_DEVICE_RX5700XT:    return "Radeon RX 5700 XT";
        case PCI_DEVICE_RX5500XT:    return "Radeon RX 5500 XT";
        
        default:                     return "Radeon GPU";
    }
}

/* Get Intel GPU name */
const char* intel_gpu_name(uint16_t device_id) {
    switch (device_id) {
        case PCI_DEVICE_ARC_A770:    return "Arc A770";
        case PCI_DEVICE_ARC_A750:    return "Arc A750";
        case PCI_DEVICE_ARC_A380:    return "Arc A380";
        case PCI_DEVICE_IRIS_XE:     return "Iris Xe";
        case PCI_DEVICE_UHD_630:     return "UHD 630";
        case PCI_DEVICE_UHD_770:     return "UHD 770";
        default:                     return "Intel Graphics";
    }
}

/* Get any GPU name based on vendor */
const char* gpu_get_name(uint16_t vendor_id, uint16_t device_id) {
    switch (vendor_id) {
        case PCI_VENDOR_NVIDIA:  return nvidia_gpu_name(device_id);
        case PCI_VENDOR_AMD_ATI: return amd_gpu_name(device_id);
        case PCI_VENDOR_INTEL:   return intel_gpu_name(device_id);
        default:                 return "Unknown GPU";
    }
}

/* Print hex number */
static void print_hex16(uint16_t val) {
    const char hex[] = "0123456789ABCDEF";
    vga_putchar(hex[(val >> 12) & 0xF]);
    vga_putchar(hex[(val >> 8) & 0xF]);
    vga_putchar(hex[(val >> 4) & 0xF]);
    vga_putchar(hex[val & 0xF]);
}

static void print_hex8(uint8_t val) {
    const char hex[] = "0123456789ABCDEF";
    vga_putchar(hex[(val >> 4) & 0xF]);
    vga_putchar(hex[val & 0xF]);
}

/* Print device info */
void pci_print_device(pci_device_t *dev) {
    kputs("  [");
    print_hex8(dev->bus);
    kputs(":");
    print_hex8(dev->slot);
    kputs(".");
    vga_putchar('0' + dev->func);
    kputs("] ");
    
    kputs(pci_vendor_name(dev->vendor_id));
    kputs(" ");
    
    /* Special handling for GPUs (NVIDIA, AMD, Intel) */
    if (dev->class_code == PCI_CLASS_DISPLAY) {
        kputs(gpu_get_name(dev->vendor_id, dev->device_id));
    } else {
        kputs(pci_class_name(dev->class_code));
    }
    
    kputs(" (");
    print_hex16(dev->vendor_id);
    kputs(":");
    print_hex16(dev->device_id);
    kputs(")\n");
}

/* Enumerate and categorize all devices */
void pci_enumerate(void) {
    kputs("[PCI] Enumerating hardware...\n");
    pci_scan_all();
    
    int gpu_count = 0;
    int storage_count = 0;
    int network_count = 0;
    int bridge_count = 0;
    
    for (int i = 0; i < pci_device_count; i++) {
        pci_device_t *dev = &pci_devices[i];
        
        switch (dev->class_code) {
            case PCI_CLASS_DISPLAY:
                gpu_count++;
                /* Detect any GPU: NVIDIA, AMD, or Intel */
                if (dev->vendor_id == PCI_VENDOR_NVIDIA) {
                    g_nvidia_gpu = dev;
                    kputs("[GPU] NVIDIA ");
                    kputs(nvidia_gpu_name(dev->device_id));
                    kputs(" detected!\n");
                } else if (dev->vendor_id == PCI_VENDOR_AMD_ATI) {
                    kputs("[GPU] AMD ");
                    kputs(amd_gpu_name(dev->device_id));
                    kputs(" detected!\n");
                } else if (dev->vendor_id == PCI_VENDOR_INTEL) {
                    kputs("[GPU] Intel ");
                    kputs(intel_gpu_name(dev->device_id));
                    kputs(" detected!\n");
                } else {
                    kputs("[GPU] ");
                    kputs(pci_vendor_name(dev->vendor_id));
                    kputs(" GPU detected\n");
                }
                kputs("      BAR0: 0x");
                print_hex16((dev->bar[0] >> 16) & 0xFFFF);
                print_hex16(dev->bar[0] & 0xFFFF);
                kputs("\n");
                break;
                
            case PCI_CLASS_STORAGE:
                storage_count++;
                if (dev->subclass == PCI_SUBCLASS_SATA) {
                    g_ahci_controller = dev;
                    kputs("[STORAGE] AHCI Controller at ");
                    print_hex8(dev->bus);
                    kputs(":");
                    print_hex8(dev->slot);
                    kputs("\n");
                } else if (dev->subclass == PCI_SUBCLASS_NVME) {
                    g_nvme_controller = dev;
                    kputs("[STORAGE] NVMe Controller at ");
                    print_hex8(dev->bus);
                    kputs(":");
                    print_hex8(dev->slot);
                    kputs("\n");
                }
                break;
                
            case PCI_CLASS_NETWORK:
                network_count++;
                kputs("[NET] ");
                kputs(pci_vendor_name(dev->vendor_id));
                kputs(" Network Adapter\n");
                break;
                
            case PCI_CLASS_BRIDGE:
                bridge_count++;
                break;
        }
    }
    
    kputs("\n[PCI] Summary:\n");
    kputs("      Total devices: ");
    vga_putchar('0' + (pci_device_count / 10));
    vga_putchar('0' + (pci_device_count % 10));
    kputs("\n");
    kputs("      GPUs: ");
    vga_putchar('0' + gpu_count);
    kputs("  Storage: ");
    vga_putchar('0' + storage_count);
    kputs("  Network: ");
    vga_putchar('0' + network_count);
    kputs("  Bridges: ");
    vga_putchar('0' + bridge_count);
    kputs("\n");
}

/* Get detected NVIDIA GPU */
pci_device_t* pci_get_nvidia_gpu(void) {
    return g_nvidia_gpu;
}

/* Get detected AHCI controller */
pci_device_t* pci_get_ahci_controller(void) {
    return g_ahci_controller;
}

/* Get detected NVMe controller */
pci_device_t* pci_get_nvme_controller(void) {
    return g_nvme_controller;
}

/* Initialize PCI subsystem */
void pci_init(void) {
    kputs("[PCI] Scanning buses...\n");
    pci_scan_all();
    kputs("[PCI] Found ");
    vga_putchar('0' + (pci_device_count / 10));
    vga_putchar('0' + (pci_device_count % 10));
    kputs(" devices\n");
}
