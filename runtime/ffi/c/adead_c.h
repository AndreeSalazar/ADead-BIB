/**
 * ADead-BIB Universal Runtime - C FFI
 * ====================================
 * Author: Eddi Andreé Salazar Matos
 * Email: eddi.salazar.dev@gmail.com
 * Made with love in Peru
 * 
 * Header para usar el runtime desde C puro.
 * API simplificada y directa.
 */

#ifndef ADEAD_C_H
#define ADEAD_C_H

#include "../../core/runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================
 * Simplified C API
 * ============================================================ */

/* Global runtime (para uso simple) */
extern ADeadRuntime* adead_global_runtime;

/* Inicializar runtime global */
int adead_c_init(int backend);

/* Destruir runtime global */
void adead_c_shutdown(void);

/* ============================================================
 * Simple Tensor API
 * ============================================================ */

typedef struct {
    float* data;
    int rows;
    int cols;
} ADeadMatrix;

/* Crear matriz */
ADeadMatrix* adead_c_matrix_create(int rows, int cols);

/* Destruir matriz */
void adead_c_matrix_destroy(ADeadMatrix* m);

/* Copiar datos a matriz */
void adead_c_matrix_set(ADeadMatrix* m, const float* data);

/* Obtener datos de matriz */
void adead_c_matrix_get(const ADeadMatrix* m, float* data);

/* ============================================================
 * Simple Operations
 * ============================================================ */

/* C = A @ B */
void adead_c_matmul(const ADeadMatrix* a, const ADeadMatrix* b, ADeadMatrix* c);

/* C = A + B */
void adead_c_add(const ADeadMatrix* a, const ADeadMatrix* b, ADeadMatrix* c);

/* out = ReLU(in) */
void adead_c_relu(const ADeadMatrix* in, ADeadMatrix* out);

/* out = Softmax(in) */
void adead_c_softmax(const ADeadMatrix* in, ADeadMatrix* out);

/* Sincronizar */
void adead_c_sync(void);

/* ============================================================
 * Info
 * ============================================================ */

/* Obtener nombre del backend */
const char* adead_c_backend_name(void);

/* Memoria disponible */
size_t adead_c_memory_available(void);

/* Memoria usada */
size_t adead_c_memory_used(void);

#ifdef __cplusplus
}
#endif

#endif /* ADEAD_C_H */
