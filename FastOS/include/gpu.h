// ============================================================
// FastOS GPU Driver Header — RTX 3060 via PCI
// ============================================================
// Comunicación directa CPU ↔ GPU sin drivers NVIDIA
// ============================================================

#ifndef FASTOS_GPU_H
#define FASTOS_GPU_H

#include "types.h"

// ============================================================
// GPU Detection & Init
// ============================================================
int gpu_init(void);
int gpu_detect(void);
void gpu_print_info(void);

// ============================================================
// GPU State Queries
// ============================================================
u64 gpu_get_bar0(void);      // MMIO registers base
u64 gpu_get_bar1(void);      // VRAM base
u32 gpu_get_vram_mb(void);   // VRAM size in MB
u8 gpu_is_detected(void);    // 1 if GPU found
u16 gpu_get_device_id(void); // PCI device ID

// ============================================================
// VRAM Access
// ============================================================
void gpu_vram_write32(u64 offset, u32 value);
u32 gpu_vram_read32(u64 offset);

// ============================================================
// Framebuffer Drawing
// ============================================================
void gpu_set_pixel(u32 x, u32 y, u32 color);
void gpu_fill_rect(u32 x, u32 y, u32 w, u32 h, u32 color);
void gpu_hline(u32 x, u32 y, u32 len, u32 color);
void gpu_vline(u32 x, u32 y, u32 len, u32 color);
void gpu_rect(u32 x, u32 y, u32 w, u32 h, u32 color);

// ============================================================
// GPU Test
// ============================================================
void gpu_test_pattern(void);

// ============================================================
// Color Macros (ARGB format)
// ============================================================
#define GPU_COLOR_RED     0x00FF0000
#define GPU_COLOR_GREEN   0x0000FF00
#define GPU_COLOR_BLUE    0x000000FF
#define GPU_COLOR_WHITE   0x00FFFFFF
#define GPU_COLOR_BLACK   0x00000000
#define GPU_COLOR_YELLOW  0x00FFFF00
#define GPU_COLOR_CYAN    0x0000FFFF
#define GPU_COLOR_MAGENTA 0x00FF00FF

#define GPU_RGB(r, g, b) (((r) << 16) | ((g) << 8) | (b))

#endif // FASTOS_GPU_H
