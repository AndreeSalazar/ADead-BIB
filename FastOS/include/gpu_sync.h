// ============================================================
// FastOS GPU Sync Header — CPU ↔ GPU Synchronization
// ============================================================

#ifndef FASTOS_GPU_SYNC_H
#define FASTOS_GPU_SYNC_H

#include "types.h"

// ============================================================
// Async Operation Handle
// ============================================================
typedef struct {
    u32 fence;
    u8 completed;
} GpuAsyncOp;

// ============================================================
// GPU Buffer
// ============================================================
typedef struct {
    u64 gpu_addr;
    void* cpu_addr;
    u64 size;
} GpuBuffer;

// ============================================================
// GPU Dispatch (for compute kernels)
// ============================================================
typedef struct {
    u32 grid_x, grid_y, grid_z;
    u32 block_x, block_y, block_z;
    u64 kernel_addr;
    u64 args_addr;
} GpuDispatch;

// ============================================================
// Command Submission
// ============================================================
void gpu_cmd_draw_pixel(u32 x, u32 y, u32 color);
void gpu_cmd_fill_rect(u32 x, u32 y, u32 w, u32 h, u32 color);
u32 gpu_cmd_fence(void);

// ============================================================
// Command Execution
// ============================================================
void gpu_execute_commands(void);

// ============================================================
// Synchronization
// ============================================================
void gpu_wait_fence(u32 fence_value);
void gpu_sync(void);

// ============================================================
// Async Operations
// ============================================================
GpuAsyncOp gpu_async_fill_rect(u32 x, u32 y, u32 w, u32 h, u32 color);
int gpu_async_is_complete(GpuAsyncOp* op);
void gpu_async_wait(GpuAsyncOp* op);

// ============================================================
// Memory Transfer
// ============================================================
GpuBuffer gpu_alloc_buffer(u64 size);
void gpu_copy_to_gpu(GpuBuffer* buf, void* src, u64 size);
void gpu_copy_from_gpu(void* dst, GpuBuffer* buf, u64 size);

// ============================================================
// Kernel Dispatch
// ============================================================
void gpu_dispatch_kernel(GpuDispatch* dispatch);

// ============================================================
// Status
// ============================================================
u32 gpu_get_queue_pending(void);
u32 gpu_get_fence_value(void);
u32 gpu_get_completed_fence(void);

#endif // FASTOS_GPU_SYNC_H
