/*
 * FastOS v2.0 — Nouveau-inspired NVIDIA GPU Driver
 * ADead-BIB Native Operating System
 * 
 * Implementation of basic NVIDIA GPU support.
 */

#include "nouveau.h"

/* Known NVIDIA Device IDs and their families */
typedef struct {
    uint16_t device_id;
    nv_family_t family;
    const char *name;
} nv_device_info_t;

static const nv_device_info_t nv_devices[] = {
    /* Tesla (G80-GT21x) */
    { 0x0400, NV_FAMILY_TESLA, "GeForce 8600 GTS" },
    { 0x0421, NV_FAMILY_TESLA, "GeForce 8500 GT" },
    { 0x0600, NV_FAMILY_TESLA, "GeForce 8800 GTS" },
    
    /* Fermi (GF1xx) */
    { 0x06C0, NV_FAMILY_FERMI, "GeForce GTX 465" },
    { 0x0DC4, NV_FAMILY_FERMI, "GeForce GTS 450" },
    { 0x1200, NV_FAMILY_FERMI, "GeForce GTX 560 Ti" },
    
    /* Kepler (GK1xx) */
    { 0x0FC6, NV_FAMILY_KEPLER, "GeForce GTX 650" },
    { 0x1180, NV_FAMILY_KEPLER, "GeForce GTX 680" },
    { 0x1185, NV_FAMILY_KEPLER, "GeForce GTX 660" },
    { 0x11C0, NV_FAMILY_KEPLER, "GeForce GTX 660" },
    
    /* Maxwell (GM1xx, GM2xx) */
    { 0x1380, NV_FAMILY_MAXWELL, "GeForce GTX 750 Ti" },
    { 0x1401, NV_FAMILY_MAXWELL, "GeForce GTX 960" },
    { 0x1B80, NV_FAMILY_MAXWELL, "GeForce GTX 980" },
    
    /* Pascal (GP1xx) */
    { 0x1B80, NV_FAMILY_PASCAL, "GeForce GTX 1080" },
    { 0x1B81, NV_FAMILY_PASCAL, "GeForce GTX 1070" },
    { 0x1C02, NV_FAMILY_PASCAL, "GeForce GTX 1060 3GB" },
    { 0x1C03, NV_FAMILY_PASCAL, "GeForce GTX 1060 6GB" },
    { 0x1C81, NV_FAMILY_PASCAL, "GeForce GTX 1050" },
    { 0x1C82, NV_FAMILY_PASCAL, "GeForce GTX 1050 Ti" },
    
    /* Turing (TU1xx) */
    { 0x1E04, NV_FAMILY_TURING, "GeForce RTX 2080 Ti" },
    { 0x1E07, NV_FAMILY_TURING, "GeForce RTX 2080" },
    { 0x1E82, NV_FAMILY_TURING, "GeForce RTX 2080" },
    { 0x1F02, NV_FAMILY_TURING, "GeForce RTX 2070" },
    { 0x1F07, NV_FAMILY_TURING, "GeForce RTX 2060" },
    { 0x2182, NV_FAMILY_TURING, "GeForce GTX 1660 Ti" },
    { 0x2184, NV_FAMILY_TURING, "GeForce GTX 1660" },
    
    /* Ampere (GA1xx) */
    { 0x2204, NV_FAMILY_AMPERE, "GeForce RTX 3090" },
    { 0x2206, NV_FAMILY_AMPERE, "GeForce RTX 3080" },
    { 0x2208, NV_FAMILY_AMPERE, "GeForce RTX 3080 Ti" },
    { 0x2484, NV_FAMILY_AMPERE, "GeForce RTX 3070" },
    { 0x2488, NV_FAMILY_AMPERE, "GeForce RTX 3070 Ti" },
    { 0x2503, NV_FAMILY_AMPERE, "GeForce RTX 3060" },
    { 0x2504, NV_FAMILY_AMPERE, "GeForce RTX 3060 Ti" },
    
    /* Ada Lovelace (AD1xx) */
    { 0x2684, NV_FAMILY_ADA, "GeForce RTX 4090" },
    { 0x2704, NV_FAMILY_ADA, "GeForce RTX 4080" },
    { 0x2782, NV_FAMILY_ADA, "GeForce RTX 4070 Ti" },
    { 0x2786, NV_FAMILY_ADA, "GeForce RTX 4070" },
    { 0x2860, NV_FAMILY_ADA, "GeForce RTX 4060" },
    
    { 0, NV_FAMILY_UNKNOWN, NULL }
};

/* PCI Configuration */
#define PCI_CONFIG_ADDRESS 0xCF8
#define PCI_CONFIG_DATA    0xCFC

static uint32_t pci_config_read(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
    uint32_t address = (1 << 31) | (bus << 16) | (slot << 11) | 
                       (func << 8) | (offset & 0xFC);
    outl(PCI_CONFIG_ADDRESS, address);
    return inl(PCI_CONFIG_DATA);
}

static void pci_config_write(uint8_t bus, uint8_t slot, uint8_t func, 
                             uint8_t offset, uint32_t value) {
    uint32_t address = (1 << 31) | (bus << 16) | (slot << 11) | 
                       (func << 8) | (offset & 0xFC);
    outl(PCI_CONFIG_ADDRESS, address);
    outl(PCI_CONFIG_DATA, value);
}

/* Get GPU family from device ID */
nv_family_t nouveau_get_family(uint16_t device_id) {
    for (int i = 0; nv_devices[i].name != NULL; i++) {
        if (nv_devices[i].device_id == device_id) {
            return nv_devices[i].family;
        }
    }
    
    /* Fallback: guess family from device ID range */
    if (device_id >= 0x2600) return NV_FAMILY_ADA;
    if (device_id >= 0x2200) return NV_FAMILY_AMPERE;
    if (device_id >= 0x1E00) return NV_FAMILY_TURING;
    if (device_id >= 0x1B00) return NV_FAMILY_PASCAL;
    if (device_id >= 0x1300) return NV_FAMILY_MAXWELL;
    if (device_id >= 0x0F00) return NV_FAMILY_KEPLER;
    if (device_id >= 0x0600) return NV_FAMILY_FERMI;
    
    return NV_FAMILY_TESLA;
}

/* Get GPU name string */
const char* nouveau_get_name(uint16_t device_id) {
    for (int i = 0; nv_devices[i].name != NULL; i++) {
        if (nv_devices[i].device_id == device_id) {
            return nv_devices[i].name;
        }
    }
    return "Unknown NVIDIA GPU";
}

/* Detect NVIDIA GPU on PCI bus */
int nouveau_detect(nv_device_t *dev) {
    for (int bus = 0; bus < 256; bus++) {
        for (int slot = 0; slot < 32; slot++) {
            for (int func = 0; func < 8; func++) {
                uint32_t vendor_device = pci_config_read(bus, slot, func, 0);
                uint16_t vendor = vendor_device & 0xFFFF;
                uint16_t device = (vendor_device >> 16) & 0xFFFF;
                
                if (vendor == NVIDIA_VENDOR_ID) {
                    /* Check if it's a VGA controller */
                    uint32_t class_code = pci_config_read(bus, slot, func, 8);
                    uint8_t base_class = (class_code >> 24) & 0xFF;
                    
                    if (base_class == 0x03) {  /* Display controller */
                        dev->vendor_id = vendor;
                        dev->device_id = device;
                        dev->bus = bus;
                        dev->slot = slot;
                        dev->func = func;
                        dev->family = nouveau_get_family(device);
                        dev->name = nouveau_get_name(device);
                        
                        /* Read BARs */
                        uint32_t bar0 = pci_config_read(bus, slot, func, 0x10);
                        uint32_t bar1 = pci_config_read(bus, slot, func, 0x14);
                        
                        dev->bar0 = bar0 & ~0xF;  /* Mask flags */
                        dev->bar1 = bar1 & ~0xF;
                        
                        return 0;  /* Success */
                    }
                }
            }
        }
    }
    return -1;  /* Not found */
}

/* Read VRAM size from GPU */
uint32_t nouveau_get_vram_size(nv_device_t *dev) {
    if (dev->bar0 == 0) return 0;
    
    /* Read from PFB_CSTATUS register */
    uint32_t cstatus = nv_rd32(dev, NV_PFB_CSTATUS);
    
    /* VRAM size is encoded in bits - varies by family */
    switch (dev->family) {
        case NV_FAMILY_KEPLER:
        case NV_FAMILY_MAXWELL:
        case NV_FAMILY_PASCAL:
        case NV_FAMILY_TURING:
        case NV_FAMILY_AMPERE:
        case NV_FAMILY_ADA:
            return (cstatus & 0xFFF) * 1024 * 1024;  /* In bytes */
        default:
            return (cstatus >> 12) * 1024 * 1024;
    }
}

/* Initialize framebuffer */
int nouveau_fb_init(nv_device_t *dev, nv_framebuffer_t *fb) {
    if (dev->bar1 == 0) return -1;
    
    fb->base = dev->bar1;
    fb->width = 1920;   /* Default - should query EDID */
    fb->height = 1080;
    fb->bpp = 32;
    fb->pitch = fb->width * (fb->bpp / 8);
    
    return 0;
}

/* GPU Power Management (Pop!_OS style) */
static nv_power_mode_t current_power_mode = NV_POWER_ON;

int nouveau_set_power(nv_device_t *dev, nv_power_mode_t mode) {
    /* In a real driver, this would:
     * - For NV_POWER_OFF: Disable GPU, save state
     * - For NV_POWER_ON: Enable GPU, restore state
     * - For NV_POWER_HYBRID: Configure PRIME/Optimus
     */
    
    switch (mode) {
        case NV_POWER_OFF:
            /* Disable interrupts */
            nv_wr32(dev, NV_PMC_INTR_EN_0, 0);
            /* Disable engines */
            nv_wr32(dev, NV_PMC_ENABLE, 0);
            break;
            
        case NV_POWER_ON:
            /* Enable engines */
            nv_wr32(dev, NV_PMC_ENABLE, 0xFFFFFFFF);
            /* Enable interrupts */
            nv_wr32(dev, NV_PMC_INTR_EN_0, 0xFFFFFFFF);
            break;
            
        case NV_POWER_HYBRID:
            /* Hybrid mode - keep GPU in low power until needed */
            break;
    }
    
    current_power_mode = mode;
    return 0;
}

nv_power_mode_t nouveau_get_power(nv_device_t *dev) {
    (void)dev;  /* Unused for now */
    return current_power_mode;
}

/* Initialize Nouveau driver */
int nouveau_init(void) {
    nv_device_t dev;
    
    if (nouveau_detect(&dev) != 0) {
        return -1;  /* No NVIDIA GPU found */
    }
    
    /* Log detection */
    kprintf("[NOUVEAU] Detected: %s\n", dev.name);
    kprintf("[NOUVEAU] Device ID: 0x%04X\n", dev.device_id);
    kprintf("[NOUVEAU] BAR0: 0x%08X\n", (uint32_t)dev.bar0);
    kprintf("[NOUVEAU] BAR1: 0x%08X\n", (uint32_t)dev.bar1);
    
    /* Get VRAM size */
    dev.vram_size = nouveau_get_vram_size(&dev) / (1024 * 1024);
    kprintf("[NOUVEAU] VRAM: %d MB\n", dev.vram_size);
    
    /* Set power mode */
    nouveau_set_power(&dev, NV_POWER_ON);
    
    return 0;
}
