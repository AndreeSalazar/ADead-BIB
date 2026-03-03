/*
 * ADead-BIB Standard Library
 * linux/drm.h - Direct Rendering Manager (DRM/KMS)
 * 
 * Based on: Linux kernel DRM headers
 * For FastOS GPU/display support (Nouveau, NVIDIA)
 */

#ifndef _ADEAD_LINUX_DRM_H
#define _ADEAD_LINUX_DRM_H

#include "../stdint.h"
#include "../sys/types.h"

/* ============================================================
 * DRM Types
 * ============================================================ */

typedef uint32_t drm_handle_t;
typedef uint32_t drm_context_t;
typedef uint32_t drm_drawable_t;
typedef uint32_t drm_magic_t;

/* ============================================================
 * DRM Structures
 * ============================================================ */

/* Version info */
struct drm_version {
    int version_major;
    int version_minor;
    int version_patchlevel;
    size_t name_len;
    char *name;
    size_t date_len;
    char *date;
    size_t desc_len;
    char *desc;
};

/* Unique identifier */
struct drm_unique {
    size_t unique_len;
    char *unique;
};

/* Authentication */
struct drm_auth {
    drm_magic_t magic;
};

/* IRQ control */
struct drm_control {
    enum {
        DRM_ADD_COMMAND,
        DRM_RM_COMMAND,
        DRM_INST_HANDLER,
        DRM_UNINST_HANDLER
    } func;
    int irq;
};

/* Map types */
enum drm_map_type {
    _DRM_FRAME_BUFFER = 0,
    _DRM_REGISTERS = 1,
    _DRM_SHM = 2,
    _DRM_AGP = 3,
    _DRM_SCATTER_GATHER = 4,
    _DRM_CONSISTENT = 5
};

/* Map flags */
enum drm_map_flags {
    _DRM_RESTRICTED = 0x01,
    _DRM_READ_ONLY = 0x02,
    _DRM_LOCKED = 0x04,
    _DRM_KERNEL = 0x08,
    _DRM_WRITE_COMBINING = 0x10,
    _DRM_CONTAINS_LOCK = 0x20,
    _DRM_REMOVABLE = 0x40,
    _DRM_DRIVER = 0x80
};

/* Memory map */
struct drm_map {
    unsigned long offset;
    unsigned long size;
    enum drm_map_type type;
    enum drm_map_flags flags;
    void *handle;
    int mtrr;
};

/* ============================================================
 * GEM (Graphics Execution Manager)
 * ============================================================ */

/* GEM close */
struct drm_gem_close {
    uint32_t handle;
    uint32_t pad;
};

/* GEM flink (create name) */
struct drm_gem_flink {
    uint32_t handle;
    uint32_t name;
};

/* GEM open (from name) */
struct drm_gem_open {
    uint32_t name;
    uint32_t handle;
    uint64_t size;
};

/* ============================================================
 * Mode Setting (KMS)
 * ============================================================ */

/* Mode info */
struct drm_mode_modeinfo {
    uint32_t clock;
    uint16_t hdisplay;
    uint16_t hsync_start;
    uint16_t hsync_end;
    uint16_t htotal;
    uint16_t hskew;
    uint16_t vdisplay;
    uint16_t vsync_start;
    uint16_t vsync_end;
    uint16_t vtotal;
    uint16_t vscan;
    uint32_t vrefresh;
    uint32_t flags;
    uint32_t type;
    char name[32];
};

/* Mode flags */
#define DRM_MODE_FLAG_PHSYNC    (1<<0)
#define DRM_MODE_FLAG_NHSYNC    (1<<1)
#define DRM_MODE_FLAG_PVSYNC    (1<<2)
#define DRM_MODE_FLAG_NVSYNC    (1<<3)
#define DRM_MODE_FLAG_INTERLACE (1<<4)
#define DRM_MODE_FLAG_DBLSCAN   (1<<5)
#define DRM_MODE_FLAG_CSYNC     (1<<6)
#define DRM_MODE_FLAG_PCSYNC    (1<<7)
#define DRM_MODE_FLAG_NCSYNC    (1<<8)
#define DRM_MODE_FLAG_HSKEW     (1<<9)
#define DRM_MODE_FLAG_DBLCLK    (1<<12)
#define DRM_MODE_FLAG_CLKDIV2   (1<<13)

/* Connector types */
#define DRM_MODE_CONNECTOR_Unknown      0
#define DRM_MODE_CONNECTOR_VGA          1
#define DRM_MODE_CONNECTOR_DVII         2
#define DRM_MODE_CONNECTOR_DVID         3
#define DRM_MODE_CONNECTOR_DVIA         4
#define DRM_MODE_CONNECTOR_Composite    5
#define DRM_MODE_CONNECTOR_SVIDEO       6
#define DRM_MODE_CONNECTOR_LVDS         7
#define DRM_MODE_CONNECTOR_Component    8
#define DRM_MODE_CONNECTOR_9PinDIN      9
#define DRM_MODE_CONNECTOR_DisplayPort  10
#define DRM_MODE_CONNECTOR_HDMIA        11
#define DRM_MODE_CONNECTOR_HDMIB        12
#define DRM_MODE_CONNECTOR_TV           13
#define DRM_MODE_CONNECTOR_eDP          14
#define DRM_MODE_CONNECTOR_VIRTUAL      15
#define DRM_MODE_CONNECTOR_DSI          16
#define DRM_MODE_CONNECTOR_DPI          17
#define DRM_MODE_CONNECTOR_WRITEBACK    18
#define DRM_MODE_CONNECTOR_SPI          19
#define DRM_MODE_CONNECTOR_USB          20

/* Encoder types */
#define DRM_MODE_ENCODER_NONE   0
#define DRM_MODE_ENCODER_DAC    1
#define DRM_MODE_ENCODER_TMDS   2
#define DRM_MODE_ENCODER_LVDS   3
#define DRM_MODE_ENCODER_TVDAC  4
#define DRM_MODE_ENCODER_VIRTUAL 5
#define DRM_MODE_ENCODER_DSI    6
#define DRM_MODE_ENCODER_DPMST  7
#define DRM_MODE_ENCODER_DPI    8

/* Card resources */
struct drm_mode_card_res {
    uint64_t fb_id_ptr;
    uint64_t crtc_id_ptr;
    uint64_t connector_id_ptr;
    uint64_t encoder_id_ptr;
    uint32_t count_fbs;
    uint32_t count_crtcs;
    uint32_t count_connectors;
    uint32_t count_encoders;
    uint32_t min_width;
    uint32_t max_width;
    uint32_t min_height;
    uint32_t max_height;
};

/* CRTC */
struct drm_mode_crtc {
    uint64_t set_connectors_ptr;
    uint32_t count_connectors;
    uint32_t crtc_id;
    uint32_t fb_id;
    uint32_t x;
    uint32_t y;
    uint32_t gamma_size;
    uint32_t mode_valid;
    struct drm_mode_modeinfo mode;
};

/* Connector */
struct drm_mode_get_connector {
    uint64_t encoders_ptr;
    uint64_t modes_ptr;
    uint64_t props_ptr;
    uint64_t prop_values_ptr;
    uint32_t count_modes;
    uint32_t count_props;
    uint32_t count_encoders;
    uint32_t encoder_id;
    uint32_t connector_id;
    uint32_t connector_type;
    uint32_t connector_type_id;
    uint32_t connection;
    uint32_t mm_width;
    uint32_t mm_height;
    uint32_t subpixel;
    uint32_t pad;
};

/* Connection status */
#define DRM_MODE_CONNECTED         1
#define DRM_MODE_DISCONNECTED      2
#define DRM_MODE_UNKNOWNCONNECTION 3

/* Framebuffer */
struct drm_mode_fb_cmd {
    uint32_t fb_id;
    uint32_t width;
    uint32_t height;
    uint32_t pitch;
    uint32_t bpp;
    uint32_t depth;
    uint32_t handle;
};

struct drm_mode_fb_cmd2 {
    uint32_t fb_id;
    uint32_t width;
    uint32_t height;
    uint32_t pixel_format;
    uint32_t flags;
    uint32_t handles[4];
    uint32_t pitches[4];
    uint32_t offsets[4];
    uint64_t modifier[4];
};

/* Page flip */
struct drm_mode_crtc_page_flip {
    uint32_t crtc_id;
    uint32_t fb_id;
    uint32_t flags;
    uint32_t reserved;
    uint64_t user_data;
};

#define DRM_MODE_PAGE_FLIP_EVENT 0x01
#define DRM_MODE_PAGE_FLIP_ASYNC 0x02

/* ============================================================
 * DRM IOCTLs
 * ============================================================ */

#define DRM_IOCTL_BASE 'd'

#define DRM_IOCTL_VERSION           _IOWR(DRM_IOCTL_BASE, 0x00, struct drm_version)
#define DRM_IOCTL_GET_UNIQUE        _IOWR(DRM_IOCTL_BASE, 0x01, struct drm_unique)
#define DRM_IOCTL_GET_MAGIC         _IOR(DRM_IOCTL_BASE, 0x02, struct drm_auth)
#define DRM_IOCTL_AUTH_MAGIC        _IOW(DRM_IOCTL_BASE, 0x11, struct drm_auth)
#define DRM_IOCTL_SET_MASTER        _IO(DRM_IOCTL_BASE, 0x1e)
#define DRM_IOCTL_DROP_MASTER       _IO(DRM_IOCTL_BASE, 0x1f)

#define DRM_IOCTL_GEM_CLOSE         _IOW(DRM_IOCTL_BASE, 0x09, struct drm_gem_close)
#define DRM_IOCTL_GEM_FLINK         _IOWR(DRM_IOCTL_BASE, 0x0a, struct drm_gem_flink)
#define DRM_IOCTL_GEM_OPEN          _IOWR(DRM_IOCTL_BASE, 0x0b, struct drm_gem_open)

#define DRM_IOCTL_MODE_GETRESOURCES _IOWR(DRM_IOCTL_BASE, 0xA0, struct drm_mode_card_res)
#define DRM_IOCTL_MODE_GETCRTC      _IOWR(DRM_IOCTL_BASE, 0xA1, struct drm_mode_crtc)
#define DRM_IOCTL_MODE_SETCRTC      _IOWR(DRM_IOCTL_BASE, 0xA2, struct drm_mode_crtc)
#define DRM_IOCTL_MODE_GETCONNECTOR _IOWR(DRM_IOCTL_BASE, 0xA7, struct drm_mode_get_connector)
#define DRM_IOCTL_MODE_ADDFB        _IOWR(DRM_IOCTL_BASE, 0xAE, struct drm_mode_fb_cmd)
#define DRM_IOCTL_MODE_RMFB         _IOWR(DRM_IOCTL_BASE, 0xAF, uint32_t)
#define DRM_IOCTL_MODE_PAGE_FLIP    _IOWR(DRM_IOCTL_BASE, 0xB0, struct drm_mode_crtc_page_flip)
#define DRM_IOCTL_MODE_ADDFB2       _IOWR(DRM_IOCTL_BASE, 0xB8, struct drm_mode_fb_cmd2)

/* ============================================================
 * Pixel Formats (fourcc)
 * ============================================================ */

#define fourcc_code(a, b, c, d) \
    ((uint32_t)(a) | ((uint32_t)(b) << 8) | ((uint32_t)(c) << 16) | ((uint32_t)(d) << 24))

#define DRM_FORMAT_XRGB8888 fourcc_code('X', 'R', '2', '4')
#define DRM_FORMAT_ARGB8888 fourcc_code('A', 'R', '2', '4')
#define DRM_FORMAT_XBGR8888 fourcc_code('X', 'B', '2', '4')
#define DRM_FORMAT_ABGR8888 fourcc_code('A', 'B', '2', '4')
#define DRM_FORMAT_RGB565   fourcc_code('R', 'G', '1', '6')
#define DRM_FORMAT_BGR565   fourcc_code('B', 'G', '1', '6')
#define DRM_FORMAT_XRGB2101010 fourcc_code('X', 'R', '3', '0')
#define DRM_FORMAT_ARGB2101010 fourcc_code('A', 'R', '3', '0')

/* YUV formats */
#define DRM_FORMAT_NV12     fourcc_code('N', 'V', '1', '2')
#define DRM_FORMAT_NV21     fourcc_code('N', 'V', '2', '1')
#define DRM_FORMAT_YUV420   fourcc_code('Y', 'U', '1', '2')
#define DRM_FORMAT_YUV422   fourcc_code('Y', 'U', '1', '6')

#endif /* _ADEAD_LINUX_DRM_H */
