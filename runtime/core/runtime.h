/**
 * ADead-BIB Universal Runtime - Main API
 * =======================================
 * Author: Eddi Andreé Salazar Matos
 * Email: eddi.salazar.dev@gmail.com
 * Made with love in Peru
 * 
 * API principal del runtime universal.
 * Compatible con C, C++, Rust, Python, Zig.
 */

#ifndef ADEAD_RUNTIME_H
#define ADEAD_RUNTIME_H

#include "types.h"
#include "memory.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================
 * Runtime Context
 * ============================================================ */

typedef struct ADeadBackendVTable ADeadBackendVTable;

typedef struct {
    ADeadMemoryManager  memory;
    ADeadBackend        active_backend;
    ADeadBackendVTable* backend_vtable;
    void*               backend_ctx;
    u32                 flags;
    int                 initialized;
} ADeadRuntime;

/* ============================================================
 * Backend Virtual Table (Interface)
 * ============================================================ */

struct ADeadBackendVTable {
    const char* name;
    
    /* Lifecycle */
    ADeadError (*init)(void** ctx);
    void (*shutdown)(void* ctx);
    
    /* Memory */
    void* (*alloc)(void* ctx, usize size);
    void (*free)(void* ctx, void* ptr);
    void (*copy_h2d)(void* ctx, void* dst, const void* src, usize size);
    void (*copy_d2h)(void* ctx, void* dst, const void* src, usize size);
    
    /* Operations */
    void (*matmul)(void* ctx, const f32* a, const f32* b, f32* c, i32 m, i32 n, i32 k);
    void (*add)(void* ctx, const f32* a, const f32* b, f32* c, i32 size);
    void (*relu)(void* ctx, const f32* in, f32* out, i32 size);
    void (*softmax)(void* ctx, const f32* in, f32* out, i32 rows, i32 cols);
    
    /* Transformer ops */
    void (*attention)(void* ctx, const f32* q, const f32* k, const f32* v,
                      f32* out, i32 batch, i32 heads, i32 seq, i32 dim);
    void (*layernorm)(void* ctx, const f32* in, f32* out, i32 batch, i32 dim);
    
    /* Sync */
    void (*sync)(void* ctx);
    
    /* Info */
    usize (*memory_available)(void* ctx);
    usize (*memory_used)(void* ctx);
};

/* ============================================================
 * Runtime API
 * ============================================================ */

/* Inicializar runtime con backend específico */
ADeadError adead_init(ADeadRuntime* rt, ADeadBackend backend);

/* Inicializar con selección automática de backend */
ADeadError adead_init_auto(ADeadRuntime* rt);

/* Destruir runtime */
void adead_shutdown(ADeadRuntime* rt);

/* Obtener backend activo */
ADeadBackend adead_get_backend(const ADeadRuntime* rt);

/* Obtener nombre del backend */
const char* adead_get_backend_name(const ADeadRuntime* rt);

/* ============================================================
 * Tensor API
 * ============================================================ */

/* Crear tensor */
ADeadError adead_tensor_create(ADeadRuntime* rt, ADeadTensor* tensor,
                               const u64* shape, u32 ndim, ADeadDType dtype);

/* Destruir tensor */
void adead_tensor_destroy(ADeadRuntime* rt, ADeadTensor* tensor);

/* Copiar datos a tensor */
ADeadError adead_tensor_copy_from(ADeadRuntime* rt, ADeadTensor* tensor,
                                  const void* data, usize size);

/* Copiar datos desde tensor */
ADeadError adead_tensor_copy_to(ADeadRuntime* rt, const ADeadTensor* tensor,
                                void* data, usize size);

/* Mover tensor a dispositivo */
ADeadError adead_tensor_to_device(ADeadRuntime* rt, ADeadTensor* tensor,
                                  ADeadBackend device);

/* ============================================================
 * Operations API
 * ============================================================ */

/* Multiplicación de matrices: C = A @ B */
ADeadError adead_matmul(ADeadRuntime* rt,
                        const ADeadTensor* a, const ADeadTensor* b,
                        ADeadTensor* c);

/* Suma elemento a elemento: C = A + B */
ADeadError adead_add(ADeadRuntime* rt,
                     const ADeadTensor* a, const ADeadTensor* b,
                     ADeadTensor* c);

/* ReLU: out = max(0, in) */
ADeadError adead_relu(ADeadRuntime* rt,
                      const ADeadTensor* in, ADeadTensor* out);

/* Softmax por filas */
ADeadError adead_softmax(ADeadRuntime* rt,
                         const ADeadTensor* in, ADeadTensor* out);

/* Attention: scaled dot-product attention */
ADeadError adead_attention(ADeadRuntime* rt,
                           const ADeadTensor* q, const ADeadTensor* k,
                           const ADeadTensor* v, ADeadTensor* out);

/* Layer normalization */
ADeadError adead_layernorm(ADeadRuntime* rt,
                           const ADeadTensor* in, ADeadTensor* out);

/* ============================================================
 * Synchronization
 * ============================================================ */

/* Sincronizar operaciones pendientes */
void adead_sync(ADeadRuntime* rt);

/* ============================================================
 * Memory Info
 * ============================================================ */

/* Memoria disponible en el backend activo */
usize adead_memory_available(const ADeadRuntime* rt);

/* Memoria usada en el backend activo */
usize adead_memory_used(const ADeadRuntime* rt);

/* ============================================================
 * Execution API (Opcodes)
 * ============================================================ */

/* Ejecutar una instrucción */
ADeadError adead_execute(ADeadRuntime* rt, const ADeadInstruction* inst);

/* Ejecutar batch de instrucciones */
ADeadError adead_execute_batch(ADeadRuntime* rt,
                               const ADeadInstruction* insts, usize count);

/* ============================================================
 * Version Info
 * ============================================================ */

/* Obtener versión del runtime */
void adead_version(u32* major, u32* minor, u32* patch);

/* Obtener string de versión */
const char* adead_version_string(void);

#ifdef __cplusplus
}
#endif

#endif /* ADEAD_RUNTIME_H */
