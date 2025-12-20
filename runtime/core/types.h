/**
 * ADead-BIB Universal Runtime - Core Types
 * =========================================
 * Author: Eddi Andreé Salazar Matos
 * Email: eddi.salazar.dev@gmail.com
 * Made with love in Peru
 * 
 * Tipos fundamentales del runtime, diseñados para ser
 * deterministas y compatibles con todos los lenguajes.
 */

#ifndef ADEAD_TYPES_H
#define ADEAD_TYPES_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================
 * Tipos básicos
 * ============================================================ */

typedef int8_t   i8;
typedef int16_t  i16;
typedef int32_t  i32;
typedef int64_t  i64;

typedef uint8_t  u8;
typedef uint16_t u16;
typedef uint32_t u32;
typedef uint64_t u64;

typedef float    f32;
typedef double   f64;

typedef size_t   usize;
typedef ptrdiff_t isize;

/* ============================================================
 * Constantes del runtime
 * ============================================================ */

#define ADEAD_VERSION_MAJOR 1
#define ADEAD_VERSION_MINOR 0
#define ADEAD_VERSION_PATCH 0

#define ADEAD_BLOCK_SIZE    4096        /* 4KB blocks */
#define ADEAD_MAX_BLOCKS    65536       /* 256MB max pool */
#define ADEAD_ALIGNMENT     16          /* 16-byte alignment */

#define ADEAD_MAX_TENSORS   1024        /* Max tensors in memory */
#define ADEAD_MAX_OPS       4096        /* Max operations in queue */

/* ============================================================
 * Backends disponibles
 * ============================================================ */

typedef enum {
    ADEAD_BACKEND_CPU    = 0,
    ADEAD_BACKEND_CUDA   = 1,
    ADEAD_BACKEND_VULKAN = 2,
    ADEAD_BACKEND_AUTO   = 255,  /* Selección automática */
} ADeadBackend;

/* ============================================================
 * Códigos de error
 * ============================================================ */

typedef enum {
    ADEAD_OK                    = 0,
    ADEAD_ERROR_INIT            = -1,
    ADEAD_ERROR_MEMORY          = -2,
    ADEAD_ERROR_INVALID_OP      = -3,
    ADEAD_ERROR_BACKEND         = -4,
    ADEAD_ERROR_VULKAN          = -5,
    ADEAD_ERROR_CUDA            = -6,
    ADEAD_ERROR_SHADER          = -7,
    ADEAD_ERROR_OUT_OF_MEMORY   = -8,
    ADEAD_ERROR_INVALID_PARAM   = -9,
} ADeadError;

/* ============================================================
 * Tipos de datos para tensores
 * ============================================================ */

typedef enum {
    ADEAD_DTYPE_F32  = 0,   /* float32 */
    ADEAD_DTYPE_F64  = 1,   /* float64 */
    ADEAD_DTYPE_I32  = 2,   /* int32 */
    ADEAD_DTYPE_I64  = 3,   /* int64 */
    ADEAD_DTYPE_U8   = 4,   /* uint8 */
    ADEAD_DTYPE_I8   = 5,   /* int8 */
    ADEAD_DTYPE_F16  = 6,   /* float16 (half) */
    ADEAD_DTYPE_BF16 = 7,   /* bfloat16 */
} ADeadDType;

/* ============================================================
 * Tensor descriptor
 * ============================================================ */

#define ADEAD_MAX_DIMS 8

typedef struct {
    void*       data;               /* Puntero a datos */
    u64         shape[ADEAD_MAX_DIMS]; /* Dimensiones */
    u64         strides[ADEAD_MAX_DIMS]; /* Strides en bytes */
    u32         ndim;               /* Número de dimensiones */
    ADeadDType  dtype;              /* Tipo de datos */
    ADeadBackend device;            /* Dispositivo (CPU/GPU) */
    u64         size_bytes;         /* Tamaño total en bytes */
} ADeadTensor;

/* ============================================================
 * Opcodes del runtime
 * ============================================================ */

typedef enum {
    /* Control */
    ADEAD_OP_NOP        = 0x00000000,
    ADEAD_OP_HALT       = 0x000000FF,
    
    /* Memoria */
    ADEAD_OP_ALLOC      = 0x00010000,
    ADEAD_OP_FREE       = 0x00010001,
    ADEAD_OP_COPY       = 0x00010002,
    ADEAD_OP_ZERO       = 0x00010003,
    
    /* Aritmética básica */
    ADEAD_OP_ADD        = 0x00020000,
    ADEAD_OP_SUB        = 0x00020001,
    ADEAD_OP_MUL        = 0x00020002,
    ADEAD_OP_DIV        = 0x00020003,
    ADEAD_OP_NEG        = 0x00020004,
    ADEAD_OP_ABS        = 0x00020005,
    
    /* Matrices */
    ADEAD_OP_MATMUL     = 0x00030000,
    ADEAD_OP_TRANSPOSE  = 0x00030001,
    ADEAD_OP_DOT        = 0x00030002,
    ADEAD_OP_OUTER      = 0x00030003,
    
    /* Activaciones */
    ADEAD_OP_RELU       = 0x00040000,
    ADEAD_OP_SIGMOID    = 0x00040001,
    ADEAD_OP_TANH       = 0x00040002,
    ADEAD_OP_SOFTMAX    = 0x00040003,
    ADEAD_OP_GELU       = 0x00040004,
    
    /* Transformer */
    ADEAD_OP_ATTENTION  = 0x00050000,
    ADEAD_OP_LAYERNORM  = 0x00050001,
    ADEAD_OP_FFN        = 0x00050002,
    ADEAD_OP_EMBEDDING  = 0x00050003,
    
    /* GPU específico */
    ADEAD_OP_GPU_INIT   = 0xC0DA0001,
    ADEAD_OP_GPU_ALLOC  = 0xC0DA0010,
    ADEAD_OP_GPU_FREE   = 0xC0DA0011,
    ADEAD_OP_GPU_COPY_H2D = 0xC0DA0012,
    ADEAD_OP_GPU_COPY_D2H = 0xC0DA0013,
    ADEAD_OP_GPU_MATMUL = 0xC0DA0020,
    ADEAD_OP_GPU_SYNC   = 0xC0DA00F0,
    
    /* Vulkan específico */
    ADEAD_OP_VK_INIT    = 0x56000001,
    ADEAD_OP_VK_ALLOC   = 0x56000010,
    ADEAD_OP_VK_FREE    = 0x56000011,
    ADEAD_OP_VK_COMPUTE = 0x56000020,
    ADEAD_OP_VK_SUBMIT  = 0x56000030,
    ADEAD_OP_VK_SYNC    = 0x560000F0,
} ADeadOpcode;

/* ============================================================
 * Instrucción del runtime
 * ============================================================ */

typedef struct {
    ADeadOpcode opcode;
    u32 operands[4];
    u32 flags;
} ADeadInstruction;

/* ============================================================
 * Utilidades
 * ============================================================ */

/* Tamaño de un dtype en bytes */
static inline u32 adead_dtype_size(ADeadDType dtype) {
    switch (dtype) {
        case ADEAD_DTYPE_F32:  return 4;
        case ADEAD_DTYPE_F64:  return 8;
        case ADEAD_DTYPE_I32:  return 4;
        case ADEAD_DTYPE_I64:  return 8;
        case ADEAD_DTYPE_U8:   return 1;
        case ADEAD_DTYPE_I8:   return 1;
        case ADEAD_DTYPE_F16:  return 2;
        case ADEAD_DTYPE_BF16: return 2;
        default: return 0;
    }
}

/* Calcular número total de elementos */
static inline u64 adead_tensor_numel(const ADeadTensor* t) {
    u64 n = 1;
    for (u32 i = 0; i < t->ndim; i++) {
        n *= t->shape[i];
    }
    return n;
}

#ifdef __cplusplus
}
#endif

#endif /* ADEAD_TYPES_H */
