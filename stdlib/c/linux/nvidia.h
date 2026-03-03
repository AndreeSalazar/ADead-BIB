/*
 * ADead-BIB Standard Library
 * linux/nvidia.h - NVIDIA GPU Driver Interface
 * 
 * Based on: NVIDIA open-gpu-kernel-modules, Nouveau
 * For FastOS with POP_OS! style NVIDIA integration
 */

#ifndef _ADEAD_LINUX_NVIDIA_H
#define _ADEAD_LINUX_NVIDIA_H

#include "../stdint.h"
#include "drm.h"

/* ============================================================
 * NVIDIA GPU Classes (from open-gpu-kernel-modules)
 * ============================================================ */

/* Architecture codes */
#define NV_ARCH_TESLA       0x50   /* GT200 */
#define NV_ARCH_FERMI       0xC0   /* GF100 */
#define NV_ARCH_KEPLER      0xE0   /* GK104 */
#define NV_ARCH_MAXWELL     0x110  /* GM107 */
#define NV_ARCH_PASCAL      0x130  /* GP100 */
#define NV_ARCH_VOLTA       0x140  /* GV100 */
#define NV_ARCH_TURING      0x160  /* TU102 */
#define NV_ARCH_AMPERE      0x170  /* GA102 */
#define NV_ARCH_ADA         0x190  /* AD102 */
#define NV_ARCH_HOPPER      0x180  /* GH100 */
#define NV_ARCH_BLACKWELL   0x1A0  /* GB100 - Future */

/* Device IDs (common GPUs) */
#define NV_DEVICE_RTX_4090      0x2684
#define NV_DEVICE_RTX_4080      0x2704
#define NV_DEVICE_RTX_4070_TI   0x2782
#define NV_DEVICE_RTX_4070      0x2786
#define NV_DEVICE_RTX_4060_TI   0x2803
#define NV_DEVICE_RTX_4060      0x2882
#define NV_DEVICE_RTX_3090      0x2204
#define NV_DEVICE_RTX_3080      0x2206
#define NV_DEVICE_RTX_3070      0x2484
#define NV_DEVICE_RTX_3060      0x2503
#define NV_DEVICE_RTX_2080_TI   0x1E04
#define NV_DEVICE_RTX_2080      0x1E82
#define NV_DEVICE_RTX_2070      0x1F02
#define NV_DEVICE_GTX_1080_TI   0x1B06
#define NV_DEVICE_GTX_1080      0x1B80
#define NV_DEVICE_GTX_1070      0x1B81
#define NV_DEVICE_GTX_1060      0x1C03

/* ============================================================
 * NVIDIA Memory Types
 * ============================================================ */

typedef enum {
    NV_MEM_TYPE_UNKNOWN = 0,
    NV_MEM_TYPE_GDDR5 = 1,
    NV_MEM_TYPE_GDDR5X = 2,
    NV_MEM_TYPE_GDDR6 = 3,
    NV_MEM_TYPE_GDDR6X = 4,
    NV_MEM_TYPE_HBM2 = 5,
    NV_MEM_TYPE_HBM2E = 6,
    NV_MEM_TYPE_HBM3 = 7,
} nv_mem_type_t;

/* ============================================================
 * NVIDIA GPU Info Structure
 * ============================================================ */

struct nv_gpu_info {
    uint32_t device_id;
    uint32_t vendor_id;      /* Always 0x10DE for NVIDIA */
    uint32_t subsystem_id;
    uint32_t revision;
    
    uint32_t architecture;   /* NV_ARCH_* */
    uint32_t implementation;
    
    char name[64];
    
    /* Memory */
    uint64_t vram_size;      /* In bytes */
    uint32_t vram_type;      /* nv_mem_type_t */
    uint32_t vram_bus_width; /* In bits */
    
    /* Clocks (MHz) */
    uint32_t gpu_clock_base;
    uint32_t gpu_clock_boost;
    uint32_t mem_clock;
    
    /* Compute */
    uint32_t sm_count;       /* Streaming Multiprocessors */
    uint32_t cuda_cores;
    uint32_t tensor_cores;
    uint32_t rt_cores;       /* Ray tracing cores */
    
    /* Display */
    uint32_t num_heads;      /* Display outputs */
    uint32_t max_displays;
    
    /* Power */
    uint32_t tdp_watts;
    uint32_t power_limit_min;
    uint32_t power_limit_max;
    
    /* Temperature */
    uint32_t temp_current;
    uint32_t temp_target;
    uint32_t temp_max;
    
    /* Features */
    uint32_t features;
};

/* Feature flags */
#define NV_FEATURE_CUDA         (1 << 0)
#define NV_FEATURE_NVENC        (1 << 1)
#define NV_FEATURE_NVDEC        (1 << 2)
#define NV_FEATURE_RTX          (1 << 3)
#define NV_FEATURE_DLSS         (1 << 4)
#define NV_FEATURE_NVLINK       (1 << 5)
#define NV_FEATURE_RESIZABLE_BAR (1 << 6)
#define NV_FEATURE_GSP          (1 << 7)  /* GPU System Processor */

/* ============================================================
 * NVIDIA Channel/Context
 * ============================================================ */

struct nv_channel {
    uint32_t channel_id;
    uint32_t class_id;
    uint64_t pushbuf_addr;
    uint64_t pushbuf_size;
    uint64_t notifier_addr;
    uint32_t flags;
};

/* Channel classes */
#define NV_CHANNEL_GPFIFO_KEPLER    0xA06F
#define NV_CHANNEL_GPFIFO_MAXWELL   0xB06F
#define NV_CHANNEL_GPFIFO_PASCAL    0xC06F
#define NV_CHANNEL_GPFIFO_VOLTA     0xC36F
#define NV_CHANNEL_GPFIFO_TURING    0xC46F
#define NV_CHANNEL_GPFIFO_AMPERE    0xC56F
#define NV_CHANNEL_GPFIFO_ADA       0xC76F

/* ============================================================
 * NVIDIA Memory Objects
 * ============================================================ */

struct nv_mem_object {
    uint64_t handle;
    uint64_t size;
    uint64_t gpu_addr;
    uint64_t cpu_addr;
    uint32_t flags;
    uint32_t domain;
};

/* Memory domains */
#define NV_MEM_DOMAIN_VRAM      0x01
#define NV_MEM_DOMAIN_GART      0x02
#define NV_MEM_DOMAIN_CPU       0x04

/* Memory flags */
#define NV_MEM_FLAG_CONTIG      (1 << 0)
#define NV_MEM_FLAG_MAPPABLE    (1 << 1)
#define NV_MEM_FLAG_COHERENT    (1 << 2)
#define NV_MEM_FLAG_UNCACHED    (1 << 3)
#define NV_MEM_FLAG_WC          (1 << 4)  /* Write-combining */

/* ============================================================
 * NVIDIA Compute (CUDA-like)
 * ============================================================ */

struct nv_compute_class {
    uint32_t class_id;
    uint32_t sm_version;
    uint32_t warp_size;
    uint32_t max_threads_per_block;
    uint32_t max_blocks_per_sm;
    uint32_t shared_mem_per_sm;
    uint32_t registers_per_sm;
};

/* Compute classes */
#define NV_COMPUTE_KEPLER       0xA0C0
#define NV_COMPUTE_MAXWELL      0xB0C0
#define NV_COMPUTE_PASCAL       0xC0C0
#define NV_COMPUTE_VOLTA        0xC3C0
#define NV_COMPUTE_TURING       0xC5C0
#define NV_COMPUTE_AMPERE       0xC6C0
#define NV_COMPUTE_ADA          0xC9C0

/* ============================================================
 * NVIDIA Display Engine
 * ============================================================ */

struct nv_display_info {
    uint32_t head_id;
    uint32_t connector_type;
    uint32_t connected;
    
    uint32_t native_width;
    uint32_t native_height;
    uint32_t refresh_rate;
    
    char edid_name[64];
    uint8_t edid_data[256];
    uint32_t edid_size;
};

/* Display classes */
#define NV_DISP_CORE_KEPLER     0x907D
#define NV_DISP_CORE_MAXWELL    0x947D
#define NV_DISP_CORE_PASCAL     0x957D
#define NV_DISP_CORE_VOLTA      0xC37D
#define NV_DISP_CORE_TURING     0xC57D
#define NV_DISP_CORE_AMPERE     0xC67D
#define NV_DISP_CORE_ADA        0xC77D

/* ============================================================
 * NVIDIA Video Encode/Decode (NVENC/NVDEC)
 * ============================================================ */

struct nv_video_caps {
    uint32_t nvenc_version;
    uint32_t nvdec_version;
    
    /* Encode caps */
    uint32_t enc_h264 : 1;
    uint32_t enc_h265 : 1;
    uint32_t enc_av1 : 1;
    uint32_t enc_vp9 : 1;
    
    /* Decode caps */
    uint32_t dec_h264 : 1;
    uint32_t dec_h265 : 1;
    uint32_t dec_av1 : 1;
    uint32_t dec_vp9 : 1;
    uint32_t dec_mpeg2 : 1;
    uint32_t dec_vc1 : 1;
    
    uint32_t max_encode_width;
    uint32_t max_encode_height;
    uint32_t max_decode_width;
    uint32_t max_decode_height;
};

/* ============================================================
 * NVIDIA IOCTLs (Nouveau-compatible)
 * ============================================================ */

#define DRM_NOUVEAU_GETPARAM    0x00
#define DRM_NOUVEAU_SETPARAM    0x01
#define DRM_NOUVEAU_CHANNEL_ALLOC 0x02
#define DRM_NOUVEAU_CHANNEL_FREE  0x03
#define DRM_NOUVEAU_GROBJ_ALLOC   0x04
#define DRM_NOUVEAU_NOTIFIEROBJ_ALLOC 0x05
#define DRM_NOUVEAU_GPUOBJ_FREE   0x06
#define DRM_NOUVEAU_NVIF          0x07
#define DRM_NOUVEAU_GEM_NEW       0x40
#define DRM_NOUVEAU_GEM_PUSHBUF   0x41
#define DRM_NOUVEAU_GEM_CPU_PREP  0x42
#define DRM_NOUVEAU_GEM_CPU_FINI  0x43
#define DRM_NOUVEAU_GEM_INFO      0x44

/* Nouveau params */
#define NOUVEAU_GETPARAM_PCI_VENDOR    0
#define NOUVEAU_GETPARAM_PCI_DEVICE    1
#define NOUVEAU_GETPARAM_BUS_TYPE      2
#define NOUVEAU_GETPARAM_FB_SIZE       3
#define NOUVEAU_GETPARAM_AGP_SIZE      4
#define NOUVEAU_GETPARAM_CHIPSET_ID    5
#define NOUVEAU_GETPARAM_VM_VRAM_BASE  6
#define NOUVEAU_GETPARAM_GRAPH_UNITS   7
#define NOUVEAU_GETPARAM_PTIMER_TIME   8
#define NOUVEAU_GETPARAM_HAS_BO_USAGE  9
#define NOUVEAU_GETPARAM_HAS_PAGEFLIP  10

struct drm_nouveau_getparam {
    uint64_t param;
    uint64_t value;
};

struct drm_nouveau_gem_new {
    struct {
        uint32_t domain;
        uint32_t tile_mode;
        uint32_t tile_flags;
    } info;
    uint32_t channel_hint;
    uint32_t align;
    uint64_t size;
    uint32_t handle;
    uint32_t map_handle;
};

struct drm_nouveau_gem_info {
    uint32_t handle;
    uint32_t domain;
    uint64_t size;
    uint64_t offset;
    uint64_t map_handle;
    uint32_t tile_mode;
    uint32_t tile_flags;
};

/* ============================================================
 * POP_OS! / System76 Power Management
 * ============================================================ */

/* Power profiles (like system76-power) */
typedef enum {
    NV_POWER_PROFILE_BATTERY = 0,
    NV_POWER_PROFILE_BALANCED = 1,
    NV_POWER_PROFILE_PERFORMANCE = 2,
} nv_power_profile_t;

/* Graphics mode (hybrid/integrated/nvidia) */
typedef enum {
    NV_GRAPHICS_INTEGRATED = 0,  /* Intel/AMD only */
    NV_GRAPHICS_HYBRID = 1,      /* PRIME render offload */
    NV_GRAPHICS_NVIDIA = 2,      /* NVIDIA only */
    NV_GRAPHICS_COMPUTE = 3,     /* Headless compute */
} nv_graphics_mode_t;

struct nv_power_state {
    nv_power_profile_t profile;
    nv_graphics_mode_t graphics_mode;
    
    uint32_t gpu_power_state;    /* D0, D1, D2, D3 */
    uint32_t gpu_clock_current;
    uint32_t mem_clock_current;
    uint32_t power_draw_watts;
    uint32_t fan_speed_percent;
    
    uint32_t runtime_pm_enabled;
    uint32_t runtime_pm_suspended;
};

/* ============================================================
 * Helper Functions (declarations)
 * ============================================================ */

/* GPU detection */
int nv_detect_gpu(struct nv_gpu_info *info);
int nv_get_gpu_count(void);
int nv_get_gpu_info(int index, struct nv_gpu_info *info);

/* Memory management */
int nv_alloc_memory(struct nv_mem_object *obj);
int nv_free_memory(struct nv_mem_object *obj);
int nv_map_memory(struct nv_mem_object *obj, void **cpu_addr);
int nv_unmap_memory(struct nv_mem_object *obj);

/* Channel/Context */
int nv_create_channel(struct nv_channel *ch);
int nv_destroy_channel(struct nv_channel *ch);
int nv_submit_pushbuf(struct nv_channel *ch, uint64_t *cmds, uint32_t count);

/* Display */
int nv_get_display_info(int head, struct nv_display_info *info);
int nv_set_display_mode(int head, struct drm_mode_modeinfo *mode);

/* Power management */
int nv_get_power_state(struct nv_power_state *state);
int nv_set_power_profile(nv_power_profile_t profile);
int nv_set_graphics_mode(nv_graphics_mode_t mode);

/* Architecture helpers */
static inline const char* nv_arch_name(uint32_t arch) {
    switch (arch) {
        case NV_ARCH_TESLA:   return "Tesla";
        case NV_ARCH_FERMI:   return "Fermi";
        case NV_ARCH_KEPLER:  return "Kepler";
        case NV_ARCH_MAXWELL: return "Maxwell";
        case NV_ARCH_PASCAL:  return "Pascal";
        case NV_ARCH_VOLTA:   return "Volta";
        case NV_ARCH_TURING:  return "Turing";
        case NV_ARCH_AMPERE:  return "Ampere";
        case NV_ARCH_ADA:     return "Ada Lovelace";
        case NV_ARCH_HOPPER:  return "Hopper";
        default:              return "Unknown";
    }
}

static inline uint32_t nv_cuda_cores_per_sm(uint32_t arch) {
    switch (arch) {
        case NV_ARCH_KEPLER:  return 192;
        case NV_ARCH_MAXWELL: return 128;
        case NV_ARCH_PASCAL:  return 64;
        case NV_ARCH_VOLTA:   return 64;
        case NV_ARCH_TURING:  return 64;
        case NV_ARCH_AMPERE:  return 128;
        case NV_ARCH_ADA:     return 128;
        default:              return 64;
    }
}

#endif /* _ADEAD_LINUX_NVIDIA_H */
