// ============================================================
// ADead-BIB GPU Stdlib — FastOS GPU Header
// ============================================================
// Header C para compilar código que use GPU en FastOS
// ============================================================

/// GPU header para FastOS - acceso directo a RTX via PCI
pub const HEADER_FASTOS_GPU: &str = r#"
// ============================================================
// FastOS GPU Header — RTX 3060 via PCI
// ============================================================
// Compilado por ADead-BIB — Sin CUDA Toolkit
// ============================================================

#ifndef FASTOS_GPU_H
#define FASTOS_GPU_H

typedef unsigned char u8;
typedef unsigned short u16;
typedef unsigned int u32;
typedef unsigned long long u64;

// ============================================================
// GPU Detection & Init
// ============================================================
int gpu_init(void);
int gpu_detect(void);
void gpu_print_info(void);

// ============================================================
// GPU State Queries
// ============================================================
u64 gpu_get_bar0(void);
u64 gpu_get_bar1(void);
u32 gpu_get_vram_mb(void);
u8 gpu_is_detected(void);
u16 gpu_get_device_id(void);

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
void gpu_test_pattern(void);

// ============================================================
// Color Macros
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

// ============================================================
// GPU Sync
// ============================================================
typedef struct {
    u32 fence;
    u8 completed;
} GpuAsyncOp;

typedef struct {
    u64 gpu_addr;
    void* cpu_addr;
    u64 size;
} GpuBuffer;

typedef struct {
    u32 grid_x, grid_y, grid_z;
    u32 block_x, block_y, block_z;
    u64 kernel_addr;
    u64 args_addr;
} GpuDispatch;

void gpu_cmd_draw_pixel(u32 x, u32 y, u32 color);
void gpu_cmd_fill_rect(u32 x, u32 y, u32 w, u32 h, u32 color);
u32 gpu_cmd_fence(void);
void gpu_execute_commands(void);
void gpu_wait_fence(u32 fence_value);
void gpu_sync(void);

GpuAsyncOp gpu_async_fill_rect(u32 x, u32 y, u32 w, u32 h, u32 color);
int gpu_async_is_complete(GpuAsyncOp* op);
void gpu_async_wait(GpuAsyncOp* op);

GpuBuffer gpu_alloc_buffer(u64 size);
void gpu_copy_to_gpu(GpuBuffer* buf, void* src, u64 size);
void gpu_copy_from_gpu(void* dst, GpuBuffer* buf, u64 size);

void gpu_dispatch_kernel(GpuDispatch* dispatch);

u32 gpu_get_queue_pending(void);
u32 gpu_get_fence_value(void);
u32 gpu_get_completed_fence(void);

#endif // FASTOS_GPU_H
"#;

/// Verifica si un símbolo es del módulo GPU
pub fn is_gpu_symbol(name: &str) -> bool {
    matches!(name,
        "gpu_init" | "gpu_detect" | "gpu_print_info" |
        "gpu_get_bar0" | "gpu_get_bar1" | "gpu_get_vram_mb" |
        "gpu_is_detected" | "gpu_get_device_id" |
        "gpu_vram_write32" | "gpu_vram_read32" |
        "gpu_set_pixel" | "gpu_fill_rect" | "gpu_hline" | "gpu_vline" | "gpu_rect" |
        "gpu_test_pattern" |
        "gpu_cmd_draw_pixel" | "gpu_cmd_fill_rect" | "gpu_cmd_fence" |
        "gpu_execute_commands" | "gpu_wait_fence" | "gpu_sync" |
        "gpu_async_fill_rect" | "gpu_async_is_complete" | "gpu_async_wait" |
        "gpu_alloc_buffer" | "gpu_copy_to_gpu" | "gpu_copy_from_gpu" |
        "gpu_dispatch_kernel" |
        "gpu_get_queue_pending" | "gpu_get_fence_value" | "gpu_get_completed_fence" |
        "GpuAsyncOp" | "GpuBuffer" | "GpuDispatch"
    )
}

/// Obtiene el header GPU
pub fn get_gpu_header() -> &'static str {
    HEADER_FASTOS_GPU
}
