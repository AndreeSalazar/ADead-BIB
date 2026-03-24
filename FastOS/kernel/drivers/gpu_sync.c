// ============================================================
// FastOS GPU Sync — CPU ↔ GPU Synchronization
// ============================================================
// Sincronización entre CPU y GPU para operaciones paralelas
// ============================================================

#include "../include/types.h"
#include "../include/gpu.h"

// ============================================================
// GPU Command Queue
// ============================================================
#define GPU_CMD_QUEUE_SIZE 256

typedef enum {
    GPU_CMD_NOP = 0,
    GPU_CMD_DRAW_PIXEL,
    GPU_CMD_FILL_RECT,
    GPU_CMD_DRAW_LINE,
    GPU_CMD_SYNC,
    GPU_CMD_FENCE,
} GpuCmdType;

typedef struct {
    GpuCmdType type;
    u32 params[8];
} GpuCommand;

typedef struct {
    GpuCommand commands[GPU_CMD_QUEUE_SIZE];
    volatile u32 head;
    volatile u32 tail;
    volatile u32 fence_value;
    volatile u32 completed_fence;
} GpuCommandQueue;

static GpuCommandQueue cmd_queue = {0};

// ============================================================
// Command Queue Operations
// ============================================================
static int queue_push(GpuCommand* cmd) {
    u32 next_head = (cmd_queue.head + 1) % GPU_CMD_QUEUE_SIZE;
    
    if (next_head == cmd_queue.tail) {
        // Queue full
        return 0;
    }
    
    cmd_queue.commands[cmd_queue.head] = *cmd;
    cmd_queue.head = next_head;
    
    return 1;
}

static int queue_pop(GpuCommand* cmd) {
    if (cmd_queue.head == cmd_queue.tail) {
        // Queue empty
        return 0;
    }
    
    *cmd = cmd_queue.commands[cmd_queue.tail];
    cmd_queue.tail = (cmd_queue.tail + 1) % GPU_CMD_QUEUE_SIZE;
    
    return 1;
}

// ============================================================
// GPU Command Submission
// ============================================================
void gpu_cmd_draw_pixel(u32 x, u32 y, u32 color) {
    GpuCommand cmd = {
        .type = GPU_CMD_DRAW_PIXEL,
        .params = {x, y, color, 0, 0, 0, 0, 0}
    };
    queue_push(&cmd);
}

void gpu_cmd_fill_rect(u32 x, u32 y, u32 w, u32 h, u32 color) {
    GpuCommand cmd = {
        .type = GPU_CMD_FILL_RECT,
        .params = {x, y, w, h, color, 0, 0, 0}
    };
    queue_push(&cmd);
}

u32 gpu_cmd_fence(void) {
    cmd_queue.fence_value++;
    
    GpuCommand cmd = {
        .type = GPU_CMD_FENCE,
        .params = {cmd_queue.fence_value, 0, 0, 0, 0, 0, 0, 0}
    };
    queue_push(&cmd);
    
    return cmd_queue.fence_value;
}

// ============================================================
// GPU Command Execution (called by GPU driver)
// ============================================================
void gpu_execute_commands(void) {
    GpuCommand cmd;
    
    while (queue_pop(&cmd)) {
        switch (cmd.type) {
            case GPU_CMD_DRAW_PIXEL:
                gpu_set_pixel(cmd.params[0], cmd.params[1], cmd.params[2]);
                break;
                
            case GPU_CMD_FILL_RECT:
                gpu_fill_rect(cmd.params[0], cmd.params[1], 
                             cmd.params[2], cmd.params[3], cmd.params[4]);
                break;
                
            case GPU_CMD_FENCE:
                cmd_queue.completed_fence = cmd.params[0];
                break;
                
            case GPU_CMD_SYNC:
                // Memory barrier
                __asm__ volatile ("mfence" ::: "memory");
                break;
                
            default:
                break;
        }
    }
}

// ============================================================
// CPU-GPU Synchronization
// ============================================================
void gpu_wait_fence(u32 fence_value) {
    // Spin wait until fence is completed
    while (cmd_queue.completed_fence < fence_value) {
        // Execute pending commands
        gpu_execute_commands();
        
        // CPU pause to reduce power consumption
        __asm__ volatile ("pause");
    }
}

void gpu_sync(void) {
    // Submit fence and wait for it
    u32 fence = gpu_cmd_fence();
    gpu_wait_fence(fence);
}

// ============================================================
// Async GPU Operations
// ============================================================
typedef struct {
    u32 fence;
    u8 completed;
} GpuAsyncOp;

GpuAsyncOp gpu_async_fill_rect(u32 x, u32 y, u32 w, u32 h, u32 color) {
    gpu_cmd_fill_rect(x, y, w, h, color);
    
    GpuAsyncOp op = {
        .fence = gpu_cmd_fence(),
        .completed = 0
    };
    
    return op;
}

int gpu_async_is_complete(GpuAsyncOp* op) {
    if (op->completed) return 1;
    
    // Check if fence is done
    if (cmd_queue.completed_fence >= op->fence) {
        op->completed = 1;
        return 1;
    }
    
    // Execute some commands
    gpu_execute_commands();
    
    return cmd_queue.completed_fence >= op->fence;
}

void gpu_async_wait(GpuAsyncOp* op) {
    gpu_wait_fence(op->fence);
    op->completed = 1;
}

// ============================================================
// GPU Memory Transfer
// ============================================================
typedef struct {
    u64 gpu_addr;
    void* cpu_addr;
    u64 size;
} GpuBuffer;

GpuBuffer gpu_alloc_buffer(u64 size) {
    static u64 next_offset = 0;
    
    GpuBuffer buf = {
        .gpu_addr = gpu_get_bar1() + next_offset,
        .cpu_addr = (void*)(gpu_get_bar1() + next_offset),
        .size = size
    };
    
    // Align to 256 bytes
    next_offset += (size + 255) & ~255ULL;
    
    return buf;
}

void gpu_copy_to_gpu(GpuBuffer* buf, void* src, u64 size) {
    // Direct memory copy (CPU can access VRAM via BAR1)
    u8* dst = (u8*)buf->cpu_addr;
    u8* s = (u8*)src;
    
    for (u64 i = 0; i < size; i++) {
        dst[i] = s[i];
    }
    
    // Memory barrier
    __asm__ volatile ("mfence" ::: "memory");
}

void gpu_copy_from_gpu(void* dst, GpuBuffer* buf, u64 size) {
    u8* d = (u8*)dst;
    u8* src = (u8*)buf->cpu_addr;
    
    for (u64 i = 0; i < size; i++) {
        d[i] = src[i];
    }
}

// ============================================================
// GPU Kernel Dispatch (for compute)
// ============================================================
typedef struct {
    u32 grid_x, grid_y, grid_z;
    u32 block_x, block_y, block_z;
    u64 kernel_addr;
    u64 args_addr;
} GpuDispatch;

// Note: Actual kernel execution requires GPU command submission
// via NVIDIA's proprietary interface. This is a placeholder
// for the dispatch structure.
void gpu_dispatch_kernel(GpuDispatch* dispatch) {
    // In a real implementation, this would:
    // 1. Write dispatch parameters to GPU MMIO
    // 2. Trigger kernel execution
    // 3. Wait for completion
    
    // For now, we just record the dispatch
    (void)dispatch;
}

// ============================================================
// GPU Status
// ============================================================
u32 gpu_get_queue_pending(void) {
    if (cmd_queue.head >= cmd_queue.tail) {
        return cmd_queue.head - cmd_queue.tail;
    } else {
        return GPU_CMD_QUEUE_SIZE - cmd_queue.tail + cmd_queue.head;
    }
}

u32 gpu_get_fence_value(void) {
    return cmd_queue.fence_value;
}

u32 gpu_get_completed_fence(void) {
    return cmd_queue.completed_fence;
}
