/*
 * FastOS v2.0 — PCI Header
 * ADead-BIB Native Operating System
 */

#ifndef _FASTOS_PCI_H
#define _FASTOS_PCI_H

#include "types.h"

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

/* PCI Functions */
void pci_init(void);
void pci_enumerate(void);
void pci_scan_all(void);

uint32_t pci_read32(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset);
uint16_t pci_read16(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset);
uint8_t pci_read8(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset);
void pci_write32(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset, uint32_t value);

pci_device_t* pci_find_device(uint16_t vendor_id, uint16_t device_id);
pci_device_t* pci_find_class(uint8_t class_code, uint8_t subclass);
int pci_get_device_count(void);
pci_device_t* pci_get_device(int index);

const char* pci_class_name(uint8_t class_code);
const char* pci_vendor_name(uint16_t vendor_id);
void pci_print_device(pci_device_t *dev);

/* Hardware detection */
pci_device_t* pci_get_nvidia_gpu(void);
pci_device_t* pci_get_ahci_controller(void);
pci_device_t* pci_get_nvme_controller(void);

/* Known Vendor IDs */
#define PCI_VENDOR_NVIDIA      0x10DE
#define PCI_VENDOR_AMD         0x1022
#define PCI_VENDOR_INTEL       0x8086
#define PCI_VENDOR_REALTEK     0x10EC

/* Device Classes */
#define PCI_CLASS_STORAGE      0x01
#define PCI_CLASS_NETWORK      0x02
#define PCI_CLASS_DISPLAY      0x03
#define PCI_CLASS_MULTIMEDIA   0x04
#define PCI_CLASS_BRIDGE       0x06
#define PCI_CLASS_SERIAL       0x0C

#endif /* _FASTOS_PCI_H */
