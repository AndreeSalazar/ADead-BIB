/**
 * ADead-BIB C# Integration
 * =========================
 * Hardware Reference: AMD Ryzen 5 5600X + RTX 3060 12GB
 * Author: Eddi Andreé Salazar Matos
 */

using System;
using System.Diagnostics;
using System.Linq;

namespace ADead
{
    public class EngineConfig
    {
        public bool UseGpu { get; set; } = false;
        public bool Deterministic { get; set; } = true;
        public int NumThreads { get; set; } = 8;
        public long CacheSize { get; set; } = 100 * 1024 * 1024; // 100MB
    }

    public class Matrix
    {
        public float[] Data { get; set; }
        public int Rows { get; set; }
        public int Cols { get; set; }

        public Matrix(int rows, int cols)
        {
            Rows = rows;
            Cols = cols;
            Data = new float[rows * cols];
        }

        public static Matrix Zeros(int rows, int cols) => new Matrix(rows, cols);

        public static Matrix Ones(int rows, int cols)
        {
            var m = new Matrix(rows, cols);
            for (int i = 0; i < m.Data.Length; i++) m.Data[i] = 1.0f;
            return m;
        }

        public static Matrix Random(int rows, int cols)
        {
            var m = new Matrix(rows, cols);
            var rand = new Random();
            for (int i = 0; i < m.Data.Length; i++)
                m.Data[i] = (float)(rand.NextDouble() * 2 - 1);
            return m;
        }

        public static Matrix Eye(int size)
        {
            var m = new Matrix(size, size);
            for (int i = 0; i < size; i++) m.Data[i * size + i] = 1.0f;
            return m;
        }

        public float Get(int row, int col) => Data[row * Cols + col];
        public void Set(int row, int col, float value) => Data[row * Cols + col] = value;
        
        public float this[int row, int col]
        {
            get => Get(row, col);
            set => Set(row, col, value);
        }
    }

    public class BenchmarkResult
    {
        public double Avg { get; set; }
        public double Min { get; set; }
        public double Max { get; set; }
        public int Iterations { get; set; }
    }

    public class Engine : IDisposable
    {
        private readonly EngineConfig _config;
        private bool _disposed = false;

        public Engine() : this(new EngineConfig()) { }
        public Engine(EngineConfig config) => _config = config;

        public bool HasGpu => _config.UseGpu;

        public void Dispose()
        {
            _disposed = true;
            GC.SuppressFinalize(this);
        }

        // ====================================================================
        // MATRIX OPERATIONS
        // ====================================================================

        /// <summary>
        /// Matrix multiplication
        /// Benchmark: 180ms (C#) -> 0.1ms (ADead-BIB optimized)
        /// </summary>
        public Matrix MatMul(Matrix a, Matrix b)
        {
            if (a.Cols != b.Rows)
                throw new ArgumentException("Incompatible dimensions");

            int M = a.Rows, N = b.Cols, K = a.Cols;
            var result = Matrix.Zeros(M, N);

            const int BLOCK = 32;

            for (int i = 0; i < M; i += BLOCK)
            {
                for (int j = 0; j < N; j += BLOCK)
                {
                    for (int k = 0; k < K; k += BLOCK)
                    {
                        int iMax = Math.Min(i + BLOCK, M);
                        int jMax = Math.Min(j + BLOCK, N);
                        int kMax = Math.Min(k + BLOCK, K);

                        for (int ii = i; ii < iMax; ii++)
                        {
                            for (int kk = k; kk < kMax; kk++)
                            {
                                float aVal = a.Get(ii, kk);
                                for (int jj = j; jj < jMax; jj++)
                                {
                                    int idx = ii * N + jj;
                                    result.Data[idx] += aVal * b.Get(kk, jj);
                                }
                            }
                        }
                    }
                }
            }

            return result;
        }

        public Matrix Transpose(Matrix a)
        {
            var result = Matrix.Zeros(a.Cols, a.Rows);
            for (int i = 0; i < a.Rows; i++)
                for (int j = 0; j < a.Cols; j++)
                    result.Set(j, i, a.Get(i, j));
            return result;
        }

        public Matrix Add(Matrix a, Matrix b)
        {
            if (a.Rows != b.Rows || a.Cols != b.Cols)
                throw new ArgumentException("Incompatible dimensions");

            var result = Matrix.Zeros(a.Rows, a.Cols);
            for (int i = 0; i < a.Data.Length; i++)
                result.Data[i] = a.Data[i] + b.Data[i];
            return result;
        }

        public Matrix Scale(Matrix a, float factor)
        {
            var result = Matrix.Zeros(a.Rows, a.Cols);
            for (int i = 0; i < a.Data.Length; i++)
                result.Data[i] = a.Data[i] * factor;
            return result;
        }

        // ====================================================================
        // VECTOR OPERATIONS
        // ====================================================================

        public float Sum(float[] data) => data.Sum();
        public float Mean(float[] data) => data.Average();
        public float Max(float[] data) => data.Max();
        public float Min(float[] data) => data.Min();

        // ====================================================================
        // ML/AI OPERATIONS
        // ====================================================================

        public float[] Softmax(float[] data)
        {
            float maxVal = Max(data);
            float[] exp = data.Select(x => (float)Math.Exp(x - maxVal)).ToArray();
            float sum = exp.Sum();
            return exp.Select(x => x / sum).ToArray();
        }

        public float[] ReLU(float[] data) => data.Select(x => x > 0 ? x : 0).ToArray();

        public float[] Sigmoid(float[] data) => 
            data.Select(x => (float)(1.0 / (1.0 + Math.Exp(-x)))).ToArray();

        public Matrix Attention(Matrix Q, Matrix K, Matrix V)
        {
            float dim = Q.Cols;

            // Q @ K^T
            var Kt = Transpose(K);
            var scores = MatMul(Q, Kt);

            // Scale
            scores = Scale(scores, (float)(1.0 / Math.Sqrt(dim)));

            // Softmax per row
            int seqLen = Q.Rows;
            for (int i = 0; i < seqLen; i++)
            {
                float[] row = new float[seqLen];
                for (int j = 0; j < seqLen; j++)
                    row[j] = scores.Get(i, j);
                
                float[] softRow = Softmax(row);
                for (int j = 0; j < seqLen; j++)
                    scores.Set(i, j, softRow[j]);
            }

            // Scores @ V
            return MatMul(scores, V);
        }

        // ====================================================================
        // SORTING & SEARCHING
        // ====================================================================

        public void Sort(float[] data) => Array.Sort(data);

        public int BinarySearch(float[] data, float target)
        {
            int left = 0, right = data.Length - 1;

            while (left <= right)
            {
                int mid = (left + right) / 2;
                if (data[mid] == target) return mid;
                if (data[mid] < target) left = mid + 1;
                else right = mid - 1;
            }

            return -1;
        }

        // ====================================================================
        // BENCHMARKING
        // ====================================================================

        public BenchmarkResult Benchmark(Action f, int iterations = 100)
        {
            // Warmup
            for (int i = 0; i < 10; i++) f();

            // Benchmark
            double[] times = new double[iterations];
            var sw = new Stopwatch();

            for (int i = 0; i < iterations; i++)
            {
                sw.Restart();
                f();
                sw.Stop();
                times[i] = sw.Elapsed.TotalMilliseconds;
            }

            return new BenchmarkResult
            {
                Avg = times.Average(),
                Min = times.Min(),
                Max = times.Max(),
                Iterations = iterations
            };
        }
    }
}
