/*
 * ADead-BIB Standard Library
 * linux/fb.h - Framebuffer Interface
 * 
 * Based on: Linux kernel framebuffer headers
 * For FastOS display output
 */

#ifndef _ADEAD_LINUX_FB_H
#define _ADEAD_LINUX_FB_H

#include "../stdint.h"

/* ============================================================
 * Framebuffer Structures
 * ============================================================ */

/* Fixed screen info */
struct fb_fix_screeninfo {
    char id[16];              /* Identification string */
    unsigned long smem_start; /* Physical address of framebuffer */
    uint32_t smem_len;        /* Length of framebuffer */
    uint32_t type;            /* FB_TYPE_* */
    uint32_t type_aux;        /* Interleave for interleaved planes */
    uint32_t visual;          /* FB_VISUAL_* */
    uint16_t xpanstep;        /* Zero if no hardware panning */
    uint16_t ypanstep;        /* Zero if no hardware panning */
    uint16_t ywrapstep;       /* Zero if no hardware ywrap */
    uint32_t line_length;     /* Length of a line in bytes */
    unsigned long mmio_start; /* Physical address of MMIO */
    uint32_t mmio_len;        /* Length of MMIO */
    uint32_t accel;           /* Acceleration type */
    uint16_t capabilities;
    uint16_t reserved[2];
};

/* Variable screen info */
struct fb_var_screeninfo {
    uint32_t xres;            /* Visible resolution */
    uint32_t yres;
    uint32_t xres_virtual;    /* Virtual resolution */
    uint32_t yres_virtual;
    uint32_t xoffset;         /* Offset from virtual to visible */
    uint32_t yoffset;
    
    uint32_t bits_per_pixel;
    uint32_t grayscale;       /* 0 = color, 1 = grayscale */
    
    struct fb_bitfield {
        uint32_t offset;      /* Beginning of bitfield */
        uint32_t length;      /* Length of bitfield */
        uint32_t msb_right;   /* MSB is right */
    } red, green, blue, transp;
    
    uint32_t nonstd;          /* Non-standard pixel format */
    uint32_t activate;        /* FB_ACTIVATE_* */
    uint32_t height;          /* Height in mm */
    uint32_t width;           /* Width in mm */
    uint32_t accel_flags;     /* Acceleration flags */
    
    /* Timing */
    uint32_t pixclock;        /* Pixel clock in ps */
    uint32_t left_margin;     /* Time from sync to picture */
    uint32_t right_margin;    /* Time from picture to sync */
    uint32_t upper_margin;    /* Time from sync to picture */
    uint32_t lower_margin;
    uint32_t hsync_len;       /* Horizontal sync length */
    uint32_t vsync_len;       /* Vertical sync length */
    uint32_t sync;            /* FB_SYNC_* */
    uint32_t vmode;           /* FB_VMODE_* */
    uint32_t rotate;          /* Angle to rotate counter-clockwise */
    uint32_t colorspace;
    uint32_t reserved[4];
};

/* Color map entry */
struct fb_cmap {
    uint32_t start;           /* First entry */
    uint32_t len;             /* Number of entries */
    uint16_t *red;
    uint16_t *green;
    uint16_t *blue;
    uint16_t *transp;
};

/* ============================================================
 * FB Types
 * ============================================================ */

#define FB_TYPE_PACKED_PIXELS       0
#define FB_TYPE_PLANES              1
#define FB_TYPE_INTERLEAVED_PLANES  2
#define FB_TYPE_TEXT                3
#define FB_TYPE_VGA_PLANES          4
#define FB_TYPE_FOURCC              5

/* ============================================================
 * FB Visuals
 * ============================================================ */

#define FB_VISUAL_MONO01            0
#define FB_VISUAL_MONO10            1
#define FB_VISUAL_TRUECOLOR         2
#define FB_VISUAL_PSEUDOCOLOR       3
#define FB_VISUAL_DIRECTCOLOR       4
#define FB_VISUAL_STATIC_PSEUDOCOLOR 5
#define FB_VISUAL_FOURCC            6

/* ============================================================
 * FB Activate
 * ============================================================ */

#define FB_ACTIVATE_NOW         0
#define FB_ACTIVATE_NXTOPEN     1
#define FB_ACTIVATE_TEST        2
#define FB_ACTIVATE_MASK        15
#define FB_ACTIVATE_VBL         16
#define FB_ACTIVATE_ALL         64
#define FB_ACTIVATE_FORCE       128
#define FB_ACTIVATE_INV_MODE    256

/* ============================================================
 * FB Sync
 * ============================================================ */

#define FB_SYNC_HOR_HIGH_ACT    1
#define FB_SYNC_VERT_HIGH_ACT   2
#define FB_SYNC_EXT             4
#define FB_SYNC_COMP_HIGH_ACT   8
#define FB_SYNC_BROADCAST       16
#define FB_SYNC_ON_GREEN        32

/* ============================================================
 * FB VMode
 * ============================================================ */

#define FB_VMODE_NONINTERLACED  0
#define FB_VMODE_INTERLACED     1
#define FB_VMODE_DOUBLE         2
#define FB_VMODE_ODD_FLD_FIRST  4
#define FB_VMODE_MASK           255
#define FB_VMODE_YWRAP          256
#define FB_VMODE_SMOOTH_XPAN    512
#define FB_VMODE_CONUPDATE      512

/* ============================================================
 * FB IOCTLs
 * ============================================================ */

#define FBIOGET_VSCREENINFO 0x4600
#define FBIOPUT_VSCREENINFO 0x4601
#define FBIOGET_FSCREENINFO 0x4602
#define FBIOGETCMAP         0x4604
#define FBIOPUTCMAP         0x4605
#define FBIOPAN_DISPLAY     0x4606
#define FBIO_CURSOR         0x4608
#define FBIOGET_CON2FBMAP   0x460F
#define FBIOPUT_CON2FBMAP   0x4610
#define FBIOBLANK           0x4611
#define FBIOGET_VBLANK      0x4612
#define FBIO_ALLOC          0x4613
#define FBIO_FREE           0x4614
#define FBIOGET_GLYPH       0x4615
#define FBIOGET_HWCINFO     0x4616
#define FBIOPUT_MODEINFO    0x4617
#define FBIOGET_DISPINFO    0x4618
#define FBIO_WAITFORVSYNC   0x4620

/* ============================================================
 * Blank Modes
 * ============================================================ */

#define FB_BLANK_UNBLANK        0
#define FB_BLANK_NORMAL         1
#define FB_BLANK_VSYNC_SUSPEND  2
#define FB_BLANK_HSYNC_SUSPEND  3
#define FB_BLANK_POWERDOWN      4

/* ============================================================
 * Cursor
 * ============================================================ */

struct fbcurpos {
    uint16_t x;
    uint16_t y;
};

struct fb_cursor {
    uint16_t set;
    uint16_t enable;
    uint16_t rop;
    const char *mask;
    struct fbcurpos hot;
    struct fb_image {
        uint32_t dx;
        uint32_t dy;
        uint32_t width;
        uint32_t height;
        uint32_t fg_color;
        uint32_t bg_color;
        uint8_t depth;
        const char *data;
        struct fb_cmap cmap;
    } image;
};

#define FB_CUR_SETIMAGE  0x01
#define FB_CUR_SETPOS    0x02
#define FB_CUR_SETHOT    0x04
#define FB_CUR_SETCMAP   0x08
#define FB_CUR_SETSHAPE  0x10
#define FB_CUR_SETSIZE   0x20
#define FB_CUR_SETALL    0xFF

/* ============================================================
 * Acceleration Flags
 * ============================================================ */

#define FB_ACCELF_TEXT      1

/* Acceleration types */
#define FB_ACCEL_NONE           0
#define FB_ACCEL_ATARIBLITT     1
#define FB_ACCEL_AMIGABLITT     2
#define FB_ACCEL_S3_TRIO64      3
#define FB_ACCEL_NCR_77C32BLT   4
#define FB_ACCEL_S3_VIRGE       5
#define FB_ACCEL_ATI_MACH64GX   6
#define FB_ACCEL_DEC_TGA        7
#define FB_ACCEL_ATI_MACH64CT   8
#define FB_ACCEL_ATI_MACH64VT   9
#define FB_ACCEL_ATI_MACH64GT   10
#define FB_ACCEL_SUN_CREATOR    11
#define FB_ACCEL_SUN_CGSIX      12
#define FB_ACCEL_SUN_LEO        13
#define FB_ACCEL_IMS_TWINTURBO  14
#define FB_ACCEL_3DLABS_PERMEDIA2 15
#define FB_ACCEL_MATROX_MGA2064W 16
#define FB_ACCEL_MATROX_MGA1064SG 17
#define FB_ACCEL_MATROX_MGA2164W 18
#define FB_ACCEL_MATROX_MGA2164W_AGP 19
#define FB_ACCEL_MATROX_MGAG100 20
#define FB_ACCEL_MATROX_MGAG200 21
#define FB_ACCEL_SUN_CG14       22
#define FB_ACCEL_SUN_BWTWO      23
#define FB_ACCEL_SUN_CGTHREE    24
#define FB_ACCEL_SUN_TCX        25
#define FB_ACCEL_MATROX_MGAG400 26
#define FB_ACCEL_NV3            27
#define FB_ACCEL_NV4            28
#define FB_ACCEL_NV5            29
#define FB_ACCEL_CT_6555x       30
#define FB_ACCEL_3DFX_BANSHEE   31
#define FB_ACCEL_ATI_RAGE128    32
#define FB_ACCEL_IGS_CYBER2000  33
#define FB_ACCEL_IGS_CYBER2010  34
#define FB_ACCEL_IGS_CYBER5000  35
#define FB_ACCEL_SIS_GLAMOUR    36
#define FB_ACCEL_3DLABS_PERMEDIA3 37
#define FB_ACCEL_ATI_RADEON     38
#define FB_ACCEL_I810           39
#define FB_ACCEL_SIS_GLAMOUR_2  40
#define FB_ACCEL_SIS_XABRE      41
#define FB_ACCEL_I830           42
#define FB_ACCEL_NV_10          43
#define FB_ACCEL_NV_20          44
#define FB_ACCEL_NV_30          45
#define FB_ACCEL_NV_40          46
#define FB_ACCEL_XGI_VOLARI_V   47
#define FB_ACCEL_XGI_VOLARI_Z   48
#define FB_ACCEL_OMAP1610       49
#define FB_ACCEL_TRIDENT_TGUI   50
#define FB_ACCEL_TRIDENT_3DIMAGE 51
#define FB_ACCEL_TRIDENT_BLADE3D 52
#define FB_ACCEL_TRIDENT_BLADEXP 53
#define FB_ACCEL_CIRRUS_ALPINE  54
#define FB_ACCEL_NEOMAGIC_NM2070 90
#define FB_ACCEL_NEOMAGIC_NM2090 91
#define FB_ACCEL_NEOMAGIC_NM2093 92
#define FB_ACCEL_NEOMAGIC_NM2097 93
#define FB_ACCEL_NEOMAGIC_NM2160 94
#define FB_ACCEL_NEOMAGIC_NM2200 95
#define FB_ACCEL_NEOMAGIC_NM2230 96
#define FB_ACCEL_NEOMAGIC_NM2360 97
#define FB_ACCEL_NEOMAGIC_NM2380 98
#define FB_ACCEL_PXA3XX         99
#define FB_ACCEL_SAVAGE4        0x80
#define FB_ACCEL_SAVAGE3D       0x81
#define FB_ACCEL_SAVAGE3D_MV    0x82
#define FB_ACCEL_SAVAGE2000     0x83
#define FB_ACCEL_SAVAGE_MX_MV   0x84
#define FB_ACCEL_SAVAGE_MX      0x85
#define FB_ACCEL_SAVAGE_IX_MV   0x86
#define FB_ACCEL_SAVAGE_IX      0x87
#define FB_ACCEL_PROSAVAGE_PM   0x88
#define FB_ACCEL_PROSAVAGE_KM   0x89
#define FB_ACCEL_S3TWISTER_P    0x8a
#define FB_ACCEL_S3TWISTER_K    0x8b
#define FB_ACCEL_SUPERSAVAGE    0x8c
#define FB_ACCEL_PROSAVAGE_DDR  0x8d
#define FB_ACCEL_PROSAVAGE_DDRK 0x8e
#define FB_ACCEL_PUV3_UNIGFX    0xa0

#endif /* _ADEAD_LINUX_FB_H */
