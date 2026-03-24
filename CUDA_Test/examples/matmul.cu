// ============================================================
// matmul.cu — Matrix Multiplication CUDead-BIB
// ============================================================
// C = A × B usando shared memory
// Compilar: adb cuda matmul.cu
// ============================================================

#include <cudead.h>
#include <stdio.h>
#include <stdlib.h>

#define TILE_SIZE 16

// ── Kernel con Shared Memory ──
__cudead_kernel__ void matmul(
    const float* __restrict__ A,
    const float* __restrict__ B,
    float* __restrict__ C,
    int M, int N, int K
) {
    __shared__ float As[TILE_SIZE][TILE_SIZE];
    __shared__ float Bs[TILE_SIZE][TILE_SIZE];

    int row = blockIdx.y * TILE_SIZE + threadIdx.y;
    int col = blockIdx.x * TILE_SIZE + threadIdx.x;

    float sum = 0.0f;

    // Loop over tiles
    for (int t = 0; t < (K + TILE_SIZE - 1) / TILE_SIZE; t++) {
        // Load tile from A
        int a_col = t * TILE_SIZE + threadIdx.x;
        if (row < M && a_col < K) {
            As[threadIdx.y][threadIdx.x] = A[row * K + a_col];
        } else {
            As[threadIdx.y][threadIdx.x] = 0.0f;
        }

        // Load tile from B
        int b_row = t * TILE_SIZE + threadIdx.y;
        if (b_row < K && col < N) {
            Bs[threadIdx.y][threadIdx.x] = B[b_row * N + col];
        } else {
            Bs[threadIdx.y][threadIdx.x] = 0.0f;
        }

        // OBLIGATORIO: sync antes de usar shared memory
        cudead_block_sync();

        // Compute partial dot product
        for (int k = 0; k < TILE_SIZE; k++) {
            sum += As[threadIdx.y][k] * Bs[k][threadIdx.x];
        }

        // OBLIGATORIO: sync antes de siguiente tile
        cudead_block_sync();
    }

    // Write result
    if (row < M && col < N) {
        C[row * N + col] = sum;
    }
}

// ── Host ──
int main(void) {
    cudead_gpu_print();

    const int M = 512;  // rows of A, C
    const int N = 512;  // cols of B, C
    const int K = 512;  // cols of A, rows of B

    printf("\n=== Matrix Multiplication ===\n");
    printf("A: %d x %d\n", M, K);
    printf("B: %d x %d\n", K, N);
    printf("C: %d x %d\n", M, N);

    size_t bytes_A = M * K * sizeof(float);
    size_t bytes_B = K * N * sizeof(float);
    size_t bytes_C = M * N * sizeof(float);

    // CPU memory
    float* h_A = (float*)malloc(bytes_A);
    float* h_B = (float*)malloc(bytes_B);
    float* h_C = (float*)malloc(bytes_C);

    // Initialize
    for (int i = 0; i < M * K; i++) h_A[i] = 1.0f;
    for (int i = 0; i < K * N; i++) h_B[i] = 1.0f;

    // VRAM alloc
    GpuPtr d_A = cudead_malloc(bytes_A);
    GpuPtr d_B = cudead_malloc(bytes_B);
    GpuPtr d_C = cudead_malloc(bytes_C);

    // H2D
    cudead_push(h_A, d_A, bytes_A);
    cudead_push(h_B, d_B, bytes_B);

    // Launch config 2D
    LaunchConfig cfg = {
        .grid_x  = (N + TILE_SIZE - 1) / TILE_SIZE,
        .grid_y  = (M + TILE_SIZE - 1) / TILE_SIZE,
        .grid_z  = 1,
        .block_x = TILE_SIZE,
        .block_y = TILE_SIZE,
        .block_z = 1,
        .shared_mem = 2 * TILE_SIZE * TILE_SIZE * sizeof(float)
    };

    printf("\nLaunch: grid(%d,%d) block(%d,%d)\n", 
           cfg.grid_x, cfg.grid_y, cfg.block_x, cfg.block_y);

    cudead_launch_3d(matmul, cfg, d_A, d_B, d_C, M, N, K);
    cudead_sync();

    // D2H
    cudead_pull(d_C, h_C, bytes_C);

    // Verify: each element should be K (sum of 1.0 * 1.0, K times)
    int errors = 0;
    for (int i = 0; i < M * N; i++) {
        if (h_C[i] != (float)K) {
            if (errors < 5) {
                printf("ERROR: h_C[%d] = %.1f, expected %.1f\n", 
                       i, h_C[i], (float)K);
            }
            errors++;
        }
    }

    if (errors == 0) {
        printf("OK: Matrix multiplication correct ✓\n");
        printf("h_C[0] = %.1f (expected %.1f)\n", h_C[0], (float)K);
    } else {
        printf("FAIL: %d errors\n", errors);
    }

    // Free
    cudead_free(d_A);
    cudead_free(d_B);
    cudead_free(d_C);
    free(h_A);
    free(h_B);
    free(h_C);

    return errors == 0 ? 0 : 1;
}
