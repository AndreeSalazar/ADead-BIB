/**
 * ADead-BIB Universal Runtime - Main Implementation
 * ==================================================
 * Author: Eddi Andreé Salazar Matos
 * Email: eddi.salazar.dev@gmail.com
 * Made with love in Peru
 */

#include "runtime.h"
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

/* ============================================================
 * Version
 * ============================================================ */

static const char* VERSION_STRING = "ADead-BIB Runtime 1.0.0";

void adead_version(u32* major, u32* minor, u32* patch) {
    if (major) *major = ADEAD_VERSION_MAJOR;
    if (minor) *minor = ADEAD_VERSION_MINOR;
    if (patch) *patch = ADEAD_VERSION_PATCH;
}

const char* adead_version_string(void) {
    return VERSION_STRING;
}

/* ============================================================
 * CPU Backend (Default)
 * ============================================================ */

typedef struct {
    usize allocated;
    usize peak;
} CPUBackendCtx;

static ADeadError cpu_init(void** ctx) {
    CPUBackendCtx* c = (CPUBackendCtx*)malloc(sizeof(CPUBackendCtx));
    if (!c) return ADEAD_ERROR_OUT_OF_MEMORY;
    c->allocated = 0;
    c->peak = 0;
    *ctx = c;
    return ADEAD_OK;
}

static void cpu_shutdown(void* ctx) {
    free(ctx);
}

static void* cpu_alloc(void* ctx, usize size) {
    CPUBackendCtx* c = (CPUBackendCtx*)ctx;
    void* ptr = malloc(size);
    if (ptr) {
        c->allocated += size;
        if (c->allocated > c->peak) c->peak = c->allocated;
    }
    return ptr;
}

static void cpu_free(void* ctx, void* ptr) {
    (void)ctx;
    free(ptr);
}

static void cpu_copy_h2d(void* ctx, void* dst, const void* src, usize size) {
    (void)ctx;
    memcpy(dst, src, size);
}

static void cpu_copy_d2h(void* ctx, void* dst, const void* src, usize size) {
    (void)ctx;
    memcpy(dst, src, size);
}

static void cpu_matmul(void* ctx, const f32* a, const f32* b, f32* c,
                       i32 m, i32 n, i32 k) {
    (void)ctx;
    /* MatMul naive pero funcional */
    for (i32 i = 0; i < m; i++) {
        for (i32 j = 0; j < n; j++) {
            f32 sum = 0.0f;
            for (i32 l = 0; l < k; l++) {
                sum += a[i * k + l] * b[l * n + j];
            }
            c[i * n + j] = sum;
        }
    }
}

static void cpu_add(void* ctx, const f32* a, const f32* b, f32* c, i32 size) {
    (void)ctx;
    for (i32 i = 0; i < size; i++) {
        c[i] = a[i] + b[i];
    }
}

static void cpu_relu(void* ctx, const f32* in, f32* out, i32 size) {
    (void)ctx;
    for (i32 i = 0; i < size; i++) {
        out[i] = in[i] > 0.0f ? in[i] : 0.0f;
    }
}

static void cpu_softmax(void* ctx, const f32* in, f32* out, i32 rows, i32 cols) {
    (void)ctx;
    for (i32 r = 0; r < rows; r++) {
        const f32* row_in = in + r * cols;
        f32* row_out = out + r * cols;
        
        /* Find max for numerical stability */
        f32 max_val = row_in[0];
        for (i32 c = 1; c < cols; c++) {
            if (row_in[c] > max_val) max_val = row_in[c];
        }
        
        /* Compute exp and sum */
        f32 sum = 0.0f;
        for (i32 c = 0; c < cols; c++) {
            row_out[c] = expf(row_in[c] - max_val);
            sum += row_out[c];
        }
        
        /* Normalize */
        for (i32 c = 0; c < cols; c++) {
            row_out[c] /= sum;
        }
    }
}

static void cpu_attention(void* ctx, const f32* q, const f32* k, const f32* v,
                          f32* out, i32 batch, i32 heads, i32 seq, i32 dim) {
    (void)ctx;
    f32 scale = 1.0f / sqrtf((f32)dim);
    
    /* Simplified attention for single head */
    for (i32 b = 0; b < batch; b++) {
        for (i32 h = 0; h < heads; h++) {
            i32 offset = (b * heads + h) * seq * dim;
            const f32* q_ptr = q + offset;
            const f32* k_ptr = k + offset;
            const f32* v_ptr = v + offset;
            f32* out_ptr = out + offset;
            
            /* Compute attention scores */
            f32* scores = (f32*)malloc(seq * seq * sizeof(f32));
            
            for (i32 i = 0; i < seq; i++) {
                for (i32 j = 0; j < seq; j++) {
                    f32 dot = 0.0f;
                    for (i32 d = 0; d < dim; d++) {
                        dot += q_ptr[i * dim + d] * k_ptr[j * dim + d];
                    }
                    scores[i * seq + j] = dot * scale;
                }
            }
            
            /* Softmax */
            cpu_softmax(ctx, scores, scores, seq, seq);
            
            /* Weighted sum */
            for (i32 i = 0; i < seq; i++) {
                for (i32 d = 0; d < dim; d++) {
                    f32 sum = 0.0f;
                    for (i32 j = 0; j < seq; j++) {
                        sum += scores[i * seq + j] * v_ptr[j * dim + d];
                    }
                    out_ptr[i * dim + d] = sum;
                }
            }
            
            free(scores);
        }
    }
}

static void cpu_layernorm(void* ctx, const f32* in, f32* out, i32 batch, i32 dim) {
    (void)ctx;
    f32 eps = 1e-5f;
    
    for (i32 b = 0; b < batch; b++) {
        const f32* row_in = in + b * dim;
        f32* row_out = out + b * dim;
        
        /* Compute mean */
        f32 mean = 0.0f;
        for (i32 d = 0; d < dim; d++) {
            mean += row_in[d];
        }
        mean /= dim;
        
        /* Compute variance */
        f32 var = 0.0f;
        for (i32 d = 0; d < dim; d++) {
            f32 diff = row_in[d] - mean;
            var += diff * diff;
        }
        var /= dim;
        
        /* Normalize */
        f32 inv_std = 1.0f / sqrtf(var + eps);
        for (i32 d = 0; d < dim; d++) {
            row_out[d] = (row_in[d] - mean) * inv_std;
        }
    }
}

static void cpu_sync(void* ctx) {
    (void)ctx;
    /* CPU is synchronous, nothing to do */
}

static usize cpu_memory_available(void* ctx) {
    (void)ctx;
    return 16ULL * 1024 * 1024 * 1024; /* Assume 16GB */
}

static usize cpu_memory_used(void* ctx) {
    CPUBackendCtx* c = (CPUBackendCtx*)ctx;
    return c ? c->allocated : 0;
}

static ADeadBackendVTable cpu_vtable = {
    .name = "CPU",
    .init = cpu_init,
    .shutdown = cpu_shutdown,
    .alloc = cpu_alloc,
    .free = cpu_free,
    .copy_h2d = cpu_copy_h2d,
    .copy_d2h = cpu_copy_d2h,
    .matmul = cpu_matmul,
    .add = cpu_add,
    .relu = cpu_relu,
    .softmax = cpu_softmax,
    .attention = cpu_attention,
    .layernorm = cpu_layernorm,
    .sync = cpu_sync,
    .memory_available = cpu_memory_available,
    .memory_used = cpu_memory_used,
};

/* ============================================================
 * Runtime Implementation
 * ============================================================ */

ADeadError adead_init(ADeadRuntime* rt, ADeadBackend backend) {
    if (!rt) {
        return ADEAD_ERROR_INVALID_PARAM;
    }
    
    memset(rt, 0, sizeof(ADeadRuntime));
    
    /* Initialize memory manager */
    ADeadError err = adead_memory_init(&rt->memory, 
                                       64 * 1024 * 1024,  /* 64MB scratch */
                                       16 * 1024 * 1024); /* 16MB pool */
    if (err != ADEAD_OK) {
        return err;
    }
    
    /* Select backend */
    switch (backend) {
        case ADEAD_BACKEND_CPU:
        case ADEAD_BACKEND_AUTO:
            rt->backend_vtable = &cpu_vtable;
            rt->active_backend = ADEAD_BACKEND_CPU;
            break;
            
        case ADEAD_BACKEND_CUDA:
            /* TODO: Implement CUDA backend */
            rt->backend_vtable = &cpu_vtable; /* Fallback to CPU */
            rt->active_backend = ADEAD_BACKEND_CPU;
            break;
            
        case ADEAD_BACKEND_VULKAN:
            /* TODO: Implement Vulkan backend */
            rt->backend_vtable = &cpu_vtable; /* Fallback to CPU */
            rt->active_backend = ADEAD_BACKEND_CPU;
            break;
            
        default:
            adead_memory_destroy(&rt->memory);
            return ADEAD_ERROR_INVALID_PARAM;
    }
    
    /* Initialize backend */
    err = rt->backend_vtable->init(&rt->backend_ctx);
    if (err != ADEAD_OK) {
        adead_memory_destroy(&rt->memory);
        return err;
    }
    
    rt->initialized = 1;
    return ADEAD_OK;
}

ADeadError adead_init_auto(ADeadRuntime* rt) {
    /* Try Vulkan first, then CUDA, then CPU */
    /* For now, just use CPU */
    return adead_init(rt, ADEAD_BACKEND_AUTO);
}

void adead_shutdown(ADeadRuntime* rt) {
    if (!rt || !rt->initialized) {
        return;
    }
    
    if (rt->backend_vtable && rt->backend_ctx) {
        rt->backend_vtable->shutdown(rt->backend_ctx);
    }
    
    adead_memory_destroy(&rt->memory);
    memset(rt, 0, sizeof(ADeadRuntime));
}

ADeadBackend adead_get_backend(const ADeadRuntime* rt) {
    return rt ? rt->active_backend : ADEAD_BACKEND_CPU;
}

const char* adead_get_backend_name(const ADeadRuntime* rt) {
    if (!rt || !rt->backend_vtable) {
        return "Unknown";
    }
    return rt->backend_vtable->name;
}

/* ============================================================
 * Tensor Implementation
 * ============================================================ */

ADeadError adead_tensor_create(ADeadRuntime* rt, ADeadTensor* tensor,
                               const u64* shape, u32 ndim, ADeadDType dtype) {
    if (!rt || !tensor || !shape || ndim == 0 || ndim > ADEAD_MAX_DIMS) {
        return ADEAD_ERROR_INVALID_PARAM;
    }
    
    memset(tensor, 0, sizeof(ADeadTensor));
    
    /* Calculate size */
    u64 numel = 1;
    for (u32 i = 0; i < ndim; i++) {
        tensor->shape[i] = shape[i];
        numel *= shape[i];
    }
    
    /* Calculate strides (row-major) */
    u32 elem_size = adead_dtype_size(dtype);
    tensor->strides[ndim - 1] = elem_size;
    for (i32 i = ndim - 2; i >= 0; i--) {
        tensor->strides[i] = tensor->strides[i + 1] * shape[i + 1];
    }
    
    tensor->ndim = ndim;
    tensor->dtype = dtype;
    tensor->device = rt->active_backend;
    tensor->size_bytes = numel * elem_size;
    
    /* Allocate memory */
    tensor->data = rt->backend_vtable->alloc(rt->backend_ctx, tensor->size_bytes);
    if (!tensor->data) {
        return ADEAD_ERROR_OUT_OF_MEMORY;
    }
    
    return ADEAD_OK;
}

void adead_tensor_destroy(ADeadRuntime* rt, ADeadTensor* tensor) {
    if (!rt || !tensor || !tensor->data) {
        return;
    }
    
    rt->backend_vtable->free(rt->backend_ctx, tensor->data);
    memset(tensor, 0, sizeof(ADeadTensor));
}

ADeadError adead_tensor_copy_from(ADeadRuntime* rt, ADeadTensor* tensor,
                                  const void* data, usize size) {
    if (!rt || !tensor || !data) {
        return ADEAD_ERROR_INVALID_PARAM;
    }
    
    if (size > tensor->size_bytes) {
        size = tensor->size_bytes;
    }
    
    rt->backend_vtable->copy_h2d(rt->backend_ctx, tensor->data, data, size);
    return ADEAD_OK;
}

ADeadError adead_tensor_copy_to(ADeadRuntime* rt, const ADeadTensor* tensor,
                                void* data, usize size) {
    if (!rt || !tensor || !data) {
        return ADEAD_ERROR_INVALID_PARAM;
    }
    
    if (size > tensor->size_bytes) {
        size = tensor->size_bytes;
    }
    
    rt->backend_vtable->copy_d2h(rt->backend_ctx, data, tensor->data, size);
    return ADEAD_OK;
}

/* ============================================================
 * Operations Implementation
 * ============================================================ */

ADeadError adead_matmul(ADeadRuntime* rt,
                        const ADeadTensor* a, const ADeadTensor* b,
                        ADeadTensor* c) {
    if (!rt || !a || !b || !c) {
        return ADEAD_ERROR_INVALID_PARAM;
    }
    
    /* Assume 2D matrices for now */
    i32 m = (i32)a->shape[0];
    i32 k = (i32)a->shape[1];
    i32 n = (i32)b->shape[1];
    
    rt->backend_vtable->matmul(rt->backend_ctx,
                               (const f32*)a->data,
                               (const f32*)b->data,
                               (f32*)c->data,
                               m, n, k);
    return ADEAD_OK;
}

ADeadError adead_add(ADeadRuntime* rt,
                     const ADeadTensor* a, const ADeadTensor* b,
                     ADeadTensor* c) {
    if (!rt || !a || !b || !c) {
        return ADEAD_ERROR_INVALID_PARAM;
    }
    
    i32 size = (i32)adead_tensor_numel(a);
    rt->backend_vtable->add(rt->backend_ctx,
                            (const f32*)a->data,
                            (const f32*)b->data,
                            (f32*)c->data,
                            size);
    return ADEAD_OK;
}

ADeadError adead_relu(ADeadRuntime* rt,
                      const ADeadTensor* in, ADeadTensor* out) {
    if (!rt || !in || !out) {
        return ADEAD_ERROR_INVALID_PARAM;
    }
    
    i32 size = (i32)adead_tensor_numel(in);
    rt->backend_vtable->relu(rt->backend_ctx,
                             (const f32*)in->data,
                             (f32*)out->data,
                             size);
    return ADEAD_OK;
}

ADeadError adead_softmax(ADeadRuntime* rt,
                         const ADeadTensor* in, ADeadTensor* out) {
    if (!rt || !in || !out) {
        return ADEAD_ERROR_INVALID_PARAM;
    }
    
    i32 rows = (i32)in->shape[0];
    i32 cols = (i32)in->shape[1];
    rt->backend_vtable->softmax(rt->backend_ctx,
                                (const f32*)in->data,
                                (f32*)out->data,
                                rows, cols);
    return ADEAD_OK;
}

void adead_sync(ADeadRuntime* rt) {
    if (rt && rt->backend_vtable) {
        rt->backend_vtable->sync(rt->backend_ctx);
    }
}

usize adead_memory_available(const ADeadRuntime* rt) {
    if (!rt || !rt->backend_vtable) {
        return 0;
    }
    return rt->backend_vtable->memory_available(rt->backend_ctx);
}

usize adead_memory_used(const ADeadRuntime* rt) {
    if (!rt || !rt->backend_vtable) {
        return 0;
    }
    return rt->backend_vtable->memory_used(rt->backend_ctx);
}
