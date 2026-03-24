// ============================================================
// vecadd.cu — Vector Addition CUDead-BIB
// ============================================================
// Ejemplo básico: C[i] = A[i] + B[i]
// Compilar: adb cuda vecadd.cu
// ============================================================

#include <cudead.h>
#include <stdio.h>
#include <stdlib.h>

// ── Kernel ──
__cudead_kernel__ void vectorAdd(
    const float* __restrict__ A,
    const float* __restrict__ B,
    float* __restrict__ C,
    int n
) {
    int idx = cudead_global_idx();
    
    // Boundary check OBLIGATORIO
    if (idx < n) {
        C[idx] = A[idx] + B[idx];
    }
}

// ── Host ──
int main(void) {
    // Mostrar GPU detectada
    cudead_gpu_print();

    const int N = 1024 * 1024;  // 1M elementos
    const size_t bytes = N * sizeof(float);

    printf("\n=== Vector Addition ===\n");
    printf("N = %d elementos\n", N);
    printf("Bytes = %zu\n", bytes);

    // CPU memory
    float* h_A = (float*)malloc(bytes);
    float* h_B = (float*)malloc(bytes);
    float* h_C = (float*)malloc(bytes);

    // Inicializar datos
    for (int i = 0; i < N; i++) {
        h_A[i] = (float)i;
        h_B[i] = (float)(i * 2);
    }

    // PRIMITIVA 1: alloca VRAM
    printf("\n[1] cudead_malloc...\n");
    GpuPtr d_A = cudead_malloc(bytes);
    GpuPtr d_B = cudead_malloc(bytes);
    GpuPtr d_C = cudead_malloc(bytes);

    printf("    d_A: addr=0x%llx size=%llu\n", d_A.addr, d_A.size);
    printf("    d_B: addr=0x%llx size=%llu\n", d_B.addr, d_B.size);
    printf("    d_C: addr=0x%llx size=%llu\n", d_C.addr, d_C.size);

    // PRIMITIVA 3: CPU → GPU
    printf("\n[3] cudead_push (H2D)...\n");
    cudead_push(h_A, d_A, bytes);
    cudead_push(h_B, d_B, bytes);

    // PRIMITIVA 5: launch
    int block = 256;  // RTX 3060 óptimo
    int grid  = (N + block - 1) / block;

    printf("\n[5] cudead_launch...\n");
    printf("    grid  = %d\n", grid);
    printf("    block = %d\n", block);
    printf("    total threads = %d\n", grid * block);

    cudead_launch(vectorAdd, grid, block, d_A, d_B, d_C, N);

    // PRIMITIVA 6: sync
    printf("\n[6] cudead_sync...\n");
    cudead_sync();

    // PRIMITIVA 4: GPU → CPU
    printf("\n[4] cudead_pull (D2H)...\n");
    cudead_pull(d_C, h_C, bytes);

    // Verificar resultados
    printf("\n=== Verificación ===\n");
    int errors = 0;
    for (int i = 0; i < N; i++) {
        float expected = h_A[i] + h_B[i];
        if (h_C[i] != expected) {
            if (errors < 5) {
                printf("ERROR: h_C[%d] = %.1f, expected %.1f\n", 
                       i, h_C[i], expected);
            }
            errors++;
        }
    }

    if (errors == 0) {
        printf("OK: Todos los %d elementos correctos ✓\n", N);
        printf("h_C[0] = %.1f (expected %.1f)\n", h_C[0], h_A[0] + h_B[0]);
        printf("h_C[N-1] = %.1f (expected %.1f)\n", 
               h_C[N-1], h_A[N-1] + h_B[N-1]);
    } else {
        printf("FAIL: %d errores de %d elementos\n", errors, N);
    }

    // PRIMITIVA 2: free VRAM
    printf("\n[2] cudead_free...\n");
    cudead_free(d_A);
    cudead_free(d_B);
    cudead_free(d_C);

    // Free CPU
    free(h_A);
    free(h_B);
    free(h_C);

    printf("\n=== Done ===\n");
    return errors == 0 ? 0 : 1;
}
