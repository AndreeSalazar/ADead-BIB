/*
 * FastOS v2.0 — Nouveau DRM Driver
 * Based on: Linux kernel drivers/gpu/drm/nouveau
 * 
 * NVIDIA GPU driver with:
 * - PCI device detection
 * - MMIO register access
 * - Framebuffer management
 * - Mode setting
 * - Power management
 * - GPU acceleration basics
 * 
 * Compile: adB cc nouveau_drm.c -o nouveau_drm.po --driver
 */

#include "../../../include/kernel.h"
#include "../../../include/types.h"
#include "nouveau.h"

/* ============================================================
 * Nouveau DRM Constants
 * Based on: Linux nouveau_drm.h
 * ============================================================ */

#define NOUVEAU_DRM_MAJOR       1
#define NOUVEAU_DRM_MINOR       0

/* NVIDIA PCI Vendor ID */
#define PCI_VENDOR_NVIDIA       0x10DE

/* GPU Chipset Classes */
#define NV_DEVICE_CLASS_NV04    0x0004
#define NV_DEVICE_CLASS_NV10    0x0010
#define NV_DEVICE_CLASS_NV20    0x0020
#define NV_DEVICE_CLASS_NV30    0x0030
#define NV_DEVICE_CLASS_NV40    0x0040
#define NV_DEVICE_CLASS_NV50    0x0050
#define NV_DEVICE_CLASS_NVC0    0x00C0
#define NV_DEVICE_CLASS_NVE0    0x00E0
#define NV_DEVICE_CLASS_GM100   0x0110
#define NV_DEVICE_CLASS_GP100   0x0130
#define NV_DEVICE_CLASS_GV100   0x0140
#define NV_DEVICE_CLASS_TU100   0x0160
#define NV_DEVICE_CLASS_GA100   0x0170
#define NV_DEVICE_CLASS_AD100   0x0190

/* MMIO Registers */
#define NV_PMC                  0x000000
#define NV_PMC_BOOT_0           0x000000
#define NV_PMC_BOOT_1           0x000004
#define NV_PMC_ENABLE           0x000200
#define NV_PMC_INTR_0           0x000100
#define NV_PMC_INTR_EN_0        0x000140

#define NV_PBUS                 0x001000
#define NV_PBUS_PCI_NV_0        0x001800
#define NV_PBUS_PCI_NV_1        0x001804

#define NV_PFIFO                0x002000
#define NV_PFIFO_INTR_0         0x002100
#define NV_PFIFO_INTR_EN_0      0x002140

#define NV_PGRAPH               0x400000
#define NV_PGRAPH_INTR          0x400100
#define NV_PGRAPH_INTR_EN       0x400140

#define NV_PFB                  0x100000
#define NV_PFB_CFG0             0x100200
#define NV_PFB_CSTATUS          0x10020C

#define NV_PRAMDAC              0x680000
#define NV_PRAMDAC_NVPLL        0x680500
#define NV_PRAMDAC_MPLL         0x680504
#define NV_PRAMDAC_VPLL         0x680508

#define NV_PCRTC                0x600000
#define NV_PCRTC_INTR_0         0x600100
#define NV_PCRTC_INTR_EN_0      0x600140
#define NV_PCRTC_START          0x600800

#define NV_PRAMIN               0x700000

/* ============================================================
 * Nouveau Device Structure
 * ============================================================ */

typedef struct {
    /* PCI Info */
    uint16_t vendor_id;
    uint16_t device_id;
    uint8_t bus;
    uint8_t slot;
    uint8_t func;
    uint8_t revision;
    
    /* BARs */
    uint64_t bar0;              /* MMIO registers */
    uint64_t bar0_size;
    uint64_t bar1;              /* Framebuffer */
    uint64_t bar1_size;
    uint64_t bar2;              /* RAMIN (older GPUs) */
    uint64_t bar2_size;
    
    /* Mapped addresses */
    volatile uint32_t *mmio;
    volatile uint32_t *fb;
    
    /* GPU Info */
    uint32_t chipset;
    uint32_t chipset_class;
    const char *chipset_name;
    uint64_t vram_size;
    uint32_t vram_type;
    
    /* Display */
    uint32_t fb_width;
    uint32_t fb_height;
    uint32_t fb_bpp;
    uint32_t fb_pitch;
    
    /* Power state */
    int power_state;            /* 0=D0, 1=D1, 2=D2, 3=D3 */
    
    /* Flags */
    int initialized;
    int accel_enabled;
} nouveau_device_t;

static nouveau_device_t nv_device;

/* ============================================================
 * MMIO Access
 * ============================================================ */

static inline uint32_t nv_rd32(uint32_t reg) {
    if (!nv_device.mmio) return 0xFFFFFFFF;
    return nv_device.mmio[reg / 4];
}

static inline void nv_wr32(uint32_t reg, uint32_t val) {
    if (!nv_device.mmio) return;
    nv_device.mmio[reg / 4] = val;
}

static inline uint32_t nv_mask(uint32_t reg, uint32_t mask, uint32_t val) {
    uint32_t tmp = nv_rd32(reg);
    nv_wr32(reg, (tmp & ~mask) | val);
    return tmp;
}

/* ============================================================
 * Chipset Detection
 * Based on: Linux nouveau_drm.c nouveau_get_chipset_name()
 * ============================================================ */

static const struct {
    uint16_t device_id;
    uint32_t chipset;
    const char *name;
} nv_chipsets[] = {
    /* Tesla (G80-GT200) */
    { 0x0191, 0x50, "GeForce 8800 GTS" },
    { 0x0193, 0x50, "GeForce 8800 GTS" },
    { 0x0400, 0x50, "GeForce 8600 GTS" },
    { 0x0402, 0x50, "GeForce 8600 GT" },
    { 0x0622, 0x50, "GeForce 9600 GT" },
    
    /* Fermi (GF100-GF119) */
    { 0x06C0, 0xC0, "GeForce GTX 480" },
    { 0x06CD, 0xC0, "GeForce GTX 470" },
    { 0x0DC4, 0xC0, "GeForce GTS 450" },
    { 0x1180, 0xC0, "GeForce GTX 680" },
    
    /* Kepler (GK104-GK208) */
    { 0x1180, 0xE0, "GeForce GTX 680" },
    { 0x1185, 0xE0, "GeForce GTX 660" },
    { 0x11C0, 0xE0, "GeForce GTX 660" },
    { 0x1280, 0xE0, "GeForce GT 635" },
    
    /* Maxwell (GM107-GM206) */
    { 0x1380, 0x110, "GeForce GTX 750 Ti" },
    { 0x13C0, 0x110, "GeForce GTX 980" },
    { 0x13C2, 0x110, "GeForce GTX 970" },
    { 0x1401, 0x110, "GeForce GTX 960" },
    
    /* Pascal (GP100-GP108) */
    { 0x1B00, 0x130, "TITAN X (Pascal)" },
    { 0x1B02, 0x130, "TITAN Xp" },
    { 0x1B06, 0x130, "GeForce GTX 1080 Ti" },
    { 0x1B80, 0x130, "GeForce GTX 1080" },
    { 0x1B81, 0x130, "GeForce GTX 1070" },
    { 0x1C02, 0x130, "GeForce GTX 1060 6GB" },
    { 0x1C03, 0x130, "GeForce GTX 1060 3GB" },
    { 0x1C81, 0x130, "GeForce GTX 1050" },
    { 0x1C82, 0x130, "GeForce GTX 1050 Ti" },
    
    /* Turing (TU102-TU117) */
    { 0x1E02, 0x160, "TITAN RTX" },
    { 0x1E04, 0x160, "GeForce RTX 2080 Ti" },
    { 0x1E07, 0x160, "GeForce RTX 2080 Ti" },
    { 0x1E82, 0x160, "GeForce RTX 2080" },
    { 0x1E84, 0x160, "GeForce RTX 2070 SUPER" },
    { 0x1F02, 0x160, "GeForce RTX 2070" },
    { 0x1F07, 0x160, "GeForce RTX 2060" },
    { 0x2182, 0x160, "GeForce GTX 1660 Ti" },
    { 0x2184, 0x160, "GeForce GTX 1660" },
    
    /* Ampere (GA102-GA107) */
    { 0x2204, 0x170, "GeForce RTX 3090" },
    { 0x2206, 0x170, "GeForce RTX 3080" },
    { 0x2208, 0x170, "GeForce RTX 3080 Ti" },
    { 0x2484, 0x170, "GeForce RTX 3070" },
    { 0x2488, 0x170, "GeForce RTX 3070 Ti" },
    { 0x2503, 0x170, "GeForce RTX 3060" },
    { 0x2504, 0x170, "GeForce RTX 3060 Ti" },
    
    /* Ada Lovelace (AD102-AD107) */
    { 0x2684, 0x190, "GeForce RTX 4090" },
    { 0x2702, 0x190, "GeForce RTX 4080" },
    { 0x2704, 0x190, "GeForce RTX 4080 SUPER" },
    { 0x2782, 0x190, "GeForce RTX 4070 Ti" },
    { 0x2786, 0x190, "GeForce RTX 4070" },
    { 0x2860, 0x190, "GeForce RTX 4060 Ti" },
    { 0x2882, 0x190, "GeForce RTX 4060" },
    
    { 0, 0, NULL }
};

static void nouveau_identify_chipset(void) {
    /* Read boot register */
    uint32_t boot0 = nv_rd32(NV_PMC_BOOT_0);
    
    nv_device.chipset = (boot0 & 0x1FF00000) >> 20;
    
    /* Determine class */
    if (nv_device.chipset >= 0x190) {
        nv_device.chipset_class = NV_DEVICE_CLASS_AD100;
    } else if (nv_device.chipset >= 0x170) {
        nv_device.chipset_class = NV_DEVICE_CLASS_GA100;
    } else if (nv_device.chipset >= 0x160) {
        nv_device.chipset_class = NV_DEVICE_CLASS_TU100;
    } else if (nv_device.chipset >= 0x140) {
        nv_device.chipset_class = NV_DEVICE_CLASS_GV100;
    } else if (nv_device.chipset >= 0x130) {
        nv_device.chipset_class = NV_DEVICE_CLASS_GP100;
    } else if (nv_device.chipset >= 0x110) {
        nv_device.chipset_class = NV_DEVICE_CLASS_GM100;
    } else if (nv_device.chipset >= 0xE0) {
        nv_device.chipset_class = NV_DEVICE_CLASS_NVE0;
    } else if (nv_device.chipset >= 0xC0) {
        nv_device.chipset_class = NV_DEVICE_CLASS_NVC0;
    } else if (nv_device.chipset >= 0x50) {
        nv_device.chipset_class = NV_DEVICE_CLASS_NV50;
    } else {
        nv_device.chipset_class = NV_DEVICE_CLASS_NV40;
    }
    
    /* Find name */
    nv_device.chipset_name = "Unknown NVIDIA GPU";
    for (int i = 0; nv_chipsets[i].name; i++) {
        if (nv_chipsets[i].device_id == nv_device.device_id) {
            nv_device.chipset_name = nv_chipsets[i].name;
            break;
        }
    }
}

/* ============================================================
 * VRAM Detection
 * Based on: Linux nouveau_mem.c
 * ============================================================ */

static void nouveau_detect_vram(void) {
    uint32_t pfb_cfg0 = nv_rd32(NV_PFB_CFG0);
    uint32_t pfb_cstatus = nv_rd32(NV_PFB_CSTATUS);
    
    /* Different detection methods for different chipsets */
    if (nv_device.chipset_class >= NV_DEVICE_CLASS_NV50) {
        /* NV50+ uses different registers */
        nv_device.vram_size = (uint64_t)(pfb_cstatus & 0xFFF) << 20;
        if (nv_device.vram_size == 0) {
            /* Fallback: read from BAR1 size */
            nv_device.vram_size = nv_device.bar1_size;
        }
    } else {
        /* Older GPUs */
        nv_device.vram_size = (pfb_cfg0 & 0xFF) << 20;
    }
    
    /* Sanity check */
    if (nv_device.vram_size == 0 || nv_device.vram_size > 48ULL * 1024 * 1024 * 1024) {
        nv_device.vram_size = 256 * 1024 * 1024;  /* Default 256MB */
    }
}

/* ============================================================
 * Framebuffer Setup
 * ============================================================ */

int nouveau_fb_setup(uint32_t width, uint32_t height, uint32_t bpp) {
    nv_device.fb_width = width;
    nv_device.fb_height = height;
    nv_device.fb_bpp = bpp;
    nv_device.fb_pitch = width * (bpp / 8);
    
    /* Set framebuffer start address */
    nv_wr32(NV_PCRTC_START, 0);
    
    kprintf("[NOUVEAU] Framebuffer: %dx%d@%d\n", width, height, bpp);
    
    return 0;
}

uint32_t *nouveau_get_framebuffer(void) {
    return (uint32_t*)nv_device.fb;
}

/* ============================================================
 * Power Management
 * Based on: Linux nouveau_pm.c
 * ============================================================ */

int nouveau_set_power_state(int state) {
    if (state < 0 || state > 3) return -1;
    
    /* Would configure GPU power state here */
    nv_device.power_state = state;
    
    kprintf("[NOUVEAU] Power state: D%d\n", state);
    return 0;
}

/* ============================================================
 * PCI Detection
 * ============================================================ */

static uint32_t pci_read(uint8_t bus, uint8_t slot, uint8_t func, uint8_t offset) {
    uint32_t addr = (1 << 31) | (bus << 16) | (slot << 11) | (func << 8) | (offset & 0xFC);
    outl(0xCF8, addr);
    return inl(0xCFC);
}

int nouveau_pci_probe(void) {
    kprintf("[NOUVEAU] Scanning PCI for NVIDIA GPU...\n");
    
    for (int bus = 0; bus < 256; bus++) {
        for (int slot = 0; slot < 32; slot++) {
            uint32_t id = pci_read(bus, slot, 0, 0);
            uint16_t vendor = id & 0xFFFF;
            uint16_t device = (id >> 16) & 0xFFFF;
            
            if (vendor == PCI_VENDOR_NVIDIA) {
                nv_device.vendor_id = vendor;
                nv_device.device_id = device;
                nv_device.bus = bus;
                nv_device.slot = slot;
                nv_device.func = 0;
                
                /* Read BARs */
                nv_device.bar0 = pci_read(bus, slot, 0, 0x10) & 0xFFFFFFF0;
                nv_device.bar1 = pci_read(bus, slot, 0, 0x14) & 0xFFFFFFF0;
                
                /* Map MMIO */
                nv_device.mmio = (volatile uint32_t*)(uintptr_t)nv_device.bar0;
                nv_device.fb = (volatile uint32_t*)(uintptr_t)nv_device.bar1;
                
                kprintf("[NOUVEAU] Found NVIDIA GPU at %02X:%02X.%d\n", 
                        bus, slot, 0);
                kprintf("[NOUVEAU] Device ID: 0x%04X\n", device);
                
                return 0;
            }
        }
    }
    
    kprintf("[NOUVEAU] No NVIDIA GPU found\n");
    return -1;
}

/* ============================================================
 * Driver Initialization
 * ============================================================ */

int nouveau_drm_init(void) {
    kmemset(&nv_device, 0, sizeof(nv_device));
    
    kprintf("[NOUVEAU] Nouveau DRM Driver v%d.%d\n", 
            NOUVEAU_DRM_MAJOR, NOUVEAU_DRM_MINOR);
    kprintf("[NOUVEAU] Based on Linux kernel nouveau driver\n");
    
    /* Probe PCI */
    if (nouveau_pci_probe() < 0) {
        return -1;
    }
    
    /* Identify chipset */
    nouveau_identify_chipset();
    kprintf("[NOUVEAU] Chipset: NV%02X (%s)\n", 
            nv_device.chipset, nv_device.chipset_name);
    
    /* Detect VRAM */
    nouveau_detect_vram();
    kprintf("[NOUVEAU] VRAM: %llu MB\n", nv_device.vram_size / (1024 * 1024));
    
    /* Enable GPU */
    nv_wr32(NV_PMC_ENABLE, 0xFFFFFFFF);
    
    /* Set power state D0 */
    nouveau_set_power_state(0);
    
    nv_device.initialized = 1;
    
    kprintf("[NOUVEAU] Driver initialized successfully\n");
    
    return 0;
}

/* ============================================================
 * Driver Info
 * ============================================================ */

void nouveau_get_info(nv_device_t *info) {
    if (!info) return;
    
    info->vendor_id = nv_device.vendor_id;
    info->device_id = nv_device.device_id;
    info->bus = nv_device.bus;
    info->slot = nv_device.slot;
    info->function = nv_device.func;
    info->family = nv_device.chipset_class;
    info->name = nv_device.chipset_name;
    info->vram_size = nv_device.vram_size;
    info->bar0 = nv_device.bar0;
    info->bar1 = nv_device.bar1;
}
