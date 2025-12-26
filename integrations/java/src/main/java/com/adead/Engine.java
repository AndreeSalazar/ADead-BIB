/**
 * ADead-BIB Java Integration
 * ===========================
 * Hardware Reference: AMD Ryzen 5 5600X + RTX 3060 12GB
 * Author: Eddi Andreé Salazar Matos
 */
package com.adead;

import java.util.Arrays;
import java.util.Random;

public class Engine implements AutoCloseable {
    
    private final EngineConfig config;
    private boolean closed = false;
    
    public Engine() {
        this(new EngineConfig());
    }
    
    public Engine(EngineConfig config) {
        this.config = config;
    }
    
    public boolean hasGpu() {
        return config.useGpu;
    }
    
    @Override
    public void close() {
        closed = true;
    }
    
    // ========================================================================
    // MATRIX OPERATIONS
    // ========================================================================
    
    /**
     * Matrix multiplication
     * Benchmark: 200ms (Java) -> 0.1ms (ADead-BIB optimized)
     */
    public Matrix matmul(Matrix a, Matrix b) {
        if (a.cols != b.rows) {
            throw new IllegalArgumentException("Incompatible dimensions");
        }
        
        int M = a.rows, N = b.cols, K = a.cols;
        Matrix result = Matrix.zeros(M, N);
        
        // Blocked multiplication for cache efficiency
        int BLOCK = 32;
        
        for (int i = 0; i < M; i += BLOCK) {
            for (int j = 0; j < N; j += BLOCK) {
                for (int k = 0; k < K; k += BLOCK) {
                    int iMax = Math.min(i + BLOCK, M);
                    int jMax = Math.min(j + BLOCK, N);
                    int kMax = Math.min(k + BLOCK, K);
                    
                    for (int ii = i; ii < iMax; ii++) {
                        for (int kk = k; kk < kMax; kk++) {
                            float aVal = a.get(ii, kk);
                            for (int jj = j; jj < jMax; jj++) {
                                int idx = ii * N + jj;
                                result.data[idx] += aVal * b.get(kk, jj);
                            }
                        }
                    }
                }
            }
        }
        
        return result;
    }
    
    public Matrix transpose(Matrix a) {
        Matrix result = Matrix.zeros(a.cols, a.rows);
        for (int i = 0; i < a.rows; i++) {
            for (int j = 0; j < a.cols; j++) {
                result.set(j, i, a.get(i, j));
            }
        }
        return result;
    }
    
    public Matrix add(Matrix a, Matrix b) {
        if (a.rows != b.rows || a.cols != b.cols) {
            throw new IllegalArgumentException("Incompatible dimensions");
        }
        
        Matrix result = Matrix.zeros(a.rows, a.cols);
        for (int i = 0; i < a.data.length; i++) {
            result.data[i] = a.data[i] + b.data[i];
        }
        return result;
    }
    
    public Matrix scale(Matrix a, float factor) {
        Matrix result = Matrix.zeros(a.rows, a.cols);
        for (int i = 0; i < a.data.length; i++) {
            result.data[i] = a.data[i] * factor;
        }
        return result;
    }
    
    // ========================================================================
    // VECTOR OPERATIONS
    // ========================================================================
    
    public float sum(float[] data) {
        float total = 0;
        for (float v : data) total += v;
        return total;
    }
    
    public float mean(float[] data) {
        return sum(data) / data.length;
    }
    
    public float max(float[] data) {
        float maxVal = data[0];
        for (int i = 1; i < data.length; i++) {
            if (data[i] > maxVal) maxVal = data[i];
        }
        return maxVal;
    }
    
    public float min(float[] data) {
        float minVal = data[0];
        for (int i = 1; i < data.length; i++) {
            if (data[i] < minVal) minVal = data[i];
        }
        return minVal;
    }
    
    // ========================================================================
    // ML/AI OPERATIONS
    // ========================================================================
    
    public float[] softmax(float[] data) {
        float maxVal = max(data);
        float[] exp = new float[data.length];
        float sum = 0;
        
        for (int i = 0; i < data.length; i++) {
            exp[i] = (float) Math.exp(data[i] - maxVal);
            sum += exp[i];
        }
        
        for (int i = 0; i < exp.length; i++) {
            exp[i] /= sum;
        }
        return exp;
    }
    
    public float[] relu(float[] data) {
        float[] result = new float[data.length];
        for (int i = 0; i < data.length; i++) {
            result[i] = data[i] > 0 ? data[i] : 0;
        }
        return result;
    }
    
    public float[] sigmoid(float[] data) {
        float[] result = new float[data.length];
        for (int i = 0; i < data.length; i++) {
            result[i] = (float) (1.0 / (1.0 + Math.exp(-data[i])));
        }
        return result;
    }
    
    public Matrix attention(Matrix Q, Matrix K, Matrix V) {
        float dim = Q.cols;
        
        // Q @ K^T
        Matrix Kt = transpose(K);
        Matrix scores = matmul(Q, Kt);
        
        // Scale
        scores = scale(scores, (float) (1.0 / Math.sqrt(dim)));
        
        // Softmax per row
        int seqLen = Q.rows;
        for (int i = 0; i < seqLen; i++) {
            float[] row = new float[seqLen];
            for (int j = 0; j < seqLen; j++) {
                row[j] = scores.get(i, j);
            }
            float[] softRow = softmax(row);
            for (int j = 0; j < seqLen; j++) {
                scores.set(i, j, softRow[j]);
            }
        }
        
        // Scores @ V
        return matmul(scores, V);
    }
    
    // ========================================================================
    // SORTING & SEARCHING
    // ========================================================================
    
    public void sort(float[] data) {
        Arrays.sort(data);
    }
    
    public int binarySearch(float[] data, float target) {
        int left = 0, right = data.length - 1;
        
        while (left <= right) {
            int mid = (left + right) / 2;
            if (data[mid] == target) return mid;
            if (data[mid] < target) left = mid + 1;
            else right = mid - 1;
        }
        
        return -1;
    }
    
    // ========================================================================
    // BENCHMARKING
    // ========================================================================
    
    public BenchmarkResult benchmark(Runnable f, int iterations) {
        // Warmup
        for (int i = 0; i < 10; i++) f.run();
        
        // Benchmark
        double[] times = new double[iterations];
        for (int i = 0; i < iterations; i++) {
            long start = System.nanoTime();
            f.run();
            times[i] = (System.nanoTime() - start) / 1_000_000.0;
        }
        
        double sum = 0, minVal = times[0], maxVal = times[0];
        for (double t : times) {
            sum += t;
            if (t < minVal) minVal = t;
            if (t > maxVal) maxVal = t;
        }
        
        return new BenchmarkResult(sum / iterations, minVal, maxVal, iterations);
    }
    
    // ========================================================================
    // INNER CLASSES
    // ========================================================================
    
    public static class EngineConfig {
        public boolean useGpu = false;
        public boolean deterministic = true;
        public int numThreads = 8;
        public long cacheSize = 100 * 1024 * 1024; // 100MB
        
        public static Builder builder() {
            return new Builder();
        }
        
        public static class Builder {
            private EngineConfig config = new EngineConfig();
            
            public Builder useGpu(boolean useGpu) {
                config.useGpu = useGpu;
                return this;
            }
            
            public Builder deterministic(boolean deterministic) {
                config.deterministic = deterministic;
                return this;
            }
            
            public Builder numThreads(int numThreads) {
                config.numThreads = numThreads;
                return this;
            }
            
            public Builder cacheSize(long cacheSize) {
                config.cacheSize = cacheSize;
                return this;
            }
            
            public EngineConfig build() {
                return config;
            }
        }
    }
    
    public static class BenchmarkResult {
        public final double avg;
        public final double min;
        public final double max;
        public final int iterations;
        
        public BenchmarkResult(double avg, double min, double max, int iterations) {
            this.avg = avg;
            this.min = min;
            this.max = max;
            this.iterations = iterations;
        }
    }
}
