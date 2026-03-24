// ============================================================
// reduction.cu — Parallel Reduction CUDead-BIB
// ============================================================
// Suma todos los elementos de un array
// Compilar: adb cuda reduction.cu
// ============================================================

#include <cudead.h>
#include <stdio.h>
#include <stdlib.h>

#define BLOCK_SIZE 256

// ── Kernel: Reducción por bloque ──
__cudead_kernel__ void reduce_sum(
    const float* __restrict__ input,
    float* __restrict__ output,
    int n
) {
    __shared__ float partial[BLOCK_SIZE];

    int tid = threadIdx.x;
    int idx = cudead_global_idx();

    // Cargar datos a shared memory
    partial[tid] = (idx < n) ? input[idx] : 0.0f;

    // OBLIGATORIO: sync después de escribir shared
    cudead_block_sync();

    // Reducción en árbol
    for (int s = blockDim.x / 2; s > 0; s >>= 1) {
        if (tid < s) {
            partial[tid] += partial[tid + s];
        }
        // OBLIGATORIO: sync en cada paso
        cudead_block_sync();
    }

    // Thread 0 escribe resultado del bloque
    if (tid == 0) {
        output[blockIdx.x] = partial[0];
    }
}

// ── Host ──
int main(void) {
    cudead_gpu_print();

    const int N = 1024 * 1024;  // 1M elementos
    const size_t bytes = N * sizeof(float);

    printf("\n=== Parallel Reduction ===\n");
    printf("N = %d elementos\n", N);

    // CPU memory
    float* h_input = (float*)malloc(bytes);

    // Inicializar: todos 1.0 → suma = N
    for (int i = 0; i < N; i++) {
        h_input[i] = 1.0f;
    }

    // Calcular número de bloques
    int grid = (N + BLOCK_SIZE - 1) / BLOCK_SIZE;
    size_t partial_bytes = grid * sizeof(float);

    float* h_partial = (float*)malloc(partial_bytes);

    // VRAM
    GpuPtr d_input   = cudead_malloc(bytes);
    GpuPtr d_partial = cudead_malloc(partial_bytes);

    // H2D
    cudead_push(h_input, d_input, bytes);

    printf("\nPaso 1: %d bloques, cada uno suma %d elementos\n", 
           grid, BLOCK_SIZE);

    // Launch primera reducción
    cudead_launch(reduce_sum, grid, BLOCK_SIZE, d_input, d_partial, N);
    cudead_sync();

    // D2H resultados parciales
    cudead_pull(d_partial, h_partial, partial_bytes);

    // Suma final en CPU (pocos elementos)
    float total = 0.0f;
    for (int i = 0; i < grid; i++) {
        total += h_partial[i];
    }

    printf("\nResultado:\n");
    printf("  Suma GPU = %.1f\n", total);
    printf("  Esperado = %.1f\n", (float)N);

    if (total == (float)N) {
        printf("  OK ✓\n");
    } else {
        printf("  ERROR: diferencia = %.1f\n", total - (float)N);
    }

    // Free
    cudead_free(d_input);
    cudead_free(d_partial);
    free(h_input);
    free(h_partial);

    return (total == (float)N) ? 0 : 1;
}
