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

/* Initialize PCI subsystem */
void pci_init(void) {
    kputs("[PCI] Scanning buses...");
    pci_scan_all();
    kprintf("[PCI] Found %d devices\n", pci_device_count);
}
