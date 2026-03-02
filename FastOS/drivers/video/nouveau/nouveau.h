/*
 * FastOS v2.0 — Nouveau-inspired NVIDIA GPU Driver
 * ADead-BIB Native Operating System
 * 
 * Based on the open-source Nouveau project (freedesktop.org)
 * Provides basic GPU detection and framebuffer support for NVIDIA GPUs.
 */

#ifndef _FASTOS_NOUVEAU_H
#define _FASTOS_NOUVEAU_H

#include "../../include/types.h"
#include "../../include/kernel.h"

/* NVIDIA Vendor ID */
#define NVIDIA_VENDOR_ID 0x10DE

/* NVIDIA GPU Families (codenames) */
typedef enum {
    NV_FAMILY_UNKNOWN = 0,
    NV_FAMILY_TESLA,      /* G80-GT21x (2006-2010) */
    NV_FAMILY_FERMI,      /* GF1xx (2010-2012) */
    NV_FAMILY_KEPLER,     /* GK1xx (2012-2014) */
    NV_FAMILY_MAXWELL,    /* GM1xx, GM2xx (2014-2016) */
    NV_FAMILY_PASCAL,     /* GP1xx (2016-2018) */
    NV_FAMILY_VOLTA,      /* GV1xx (2017-2018) */
    NV_FAMILY_TURING,     /* TU1xx (2018-2020) */
    NV_FAMILY_AMPERE,     /* GA1xx (2020-2022) */
    NV_FAMILY_ADA,        /* AD1xx (2022-present) */
    NV_FAMILY_BLACKWELL,  /* GB1xx (2024-present) */
} nv_family_t;

/* GPU Device Info */
typedef struct {
    uint16_t vendor_id;
    uint16_t device_id;
    uint8_t  bus;
    uint8_t  slot;
    uint8_t  func;
    nv_family_t family;
    const char *name;
    uint32_t vram_size;     /* In MB */
    phys_addr_t bar0;       /* MMIO registers */
    phys_addr_t bar1;       /* Framebuffer */
    size_t bar0_size;
    size_t bar1_size;
} nv_device_t;

/* Framebuffer Info */
typedef struct {
    phys_addr_t base;
    uint32_t width;
    uint32_t height;
    uint32_t pitch;
    uint8_t  bpp;           /* Bits per pixel */
} nv_framebuffer_t;

/* MMIO Register Access */
static inline uint32_t nv_rd32(nv_device_t *dev, uint32_t reg) {
    return *((volatile uint32_t*)(dev->bar0 + reg));
}

static inline void nv_wr32(nv_device_t *dev, uint32_t reg, uint32_t val) {
    *((volatile uint32_t*)(dev->bar0 + reg)) = val;
}

/* Common NVIDIA Registers */
#define NV_PMC_BOOT_0       0x000000  /* Boot register - chip ID */
#define NV_PMC_ENABLE       0x000200  /* Engine enable */
#define NV_PMC_INTR_0       0x000100  /* Interrupt status */
#define NV_PMC_INTR_EN_0    0x000140  /* Interrupt enable */

#define NV_PBUS_PCI_NV_0    0x001800  /* PCI config space mirror */
#define NV_PBUS_PCI_NV_1    0x001804
#define NV_PBUS_PCI_NV_19   0x00184C  /* BAR0 */

#define NV_PFB_CFG0         0x100200  /* Framebuffer config */
#define NV_PFB_CSTATUS      0x10020C  /* Memory status */

/* Display Registers */
#define NV_PDISP_FE_HW_SYS_CAP 0x610010

/* Function Declarations */

/* Initialize Nouveau driver */
int nouveau_init(void);

/* Detect NVIDIA GPU on PCI bus */
int nouveau_detect(nv_device_t *dev);

/* Get GPU family from device ID */
nv_family_t nouveau_get_family(uint16_t device_id);

/* Get GPU name string */
const char* nouveau_get_name(uint16_t device_id);

/* Initialize framebuffer */
int nouveau_fb_init(nv_device_t *dev, nv_framebuffer_t *fb);

/* Set display mode */
int nouveau_set_mode(nv_device_t *dev, uint32_t width, uint32_t height, uint8_t bpp);

/* Read VRAM size */
uint32_t nouveau_get_vram_size(nv_device_t *dev);

/* GPU power management (Pop!_OS style) */
typedef enum {
    NV_POWER_OFF,       /* GPU powered off */
    NV_POWER_ON,        /* GPU powered on */
    NV_POWER_HYBRID,    /* Hybrid mode (integrated + discrete) */
} nv_power_mode_t;

int nouveau_set_power(nv_device_t *dev, nv_power_mode_t mode);
nv_power_mode_t nouveau_get_power(nv_device_t *dev);

#endif /* _FASTOS_NOUVEAU_H */
