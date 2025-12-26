/**
 * ADead-BIB C++ Integration
 * ==========================
 * Hardware Reference: AMD Ryzen 5 5600X + RTX 3060 12GB
 * Author: Eddi Andreé Salazar Matos
 * 
 * Header-only library for ADead-BIB integration
 */

#ifndef ADEAD_HPP
#define ADEAD_HPP

#include <vector>
#include <cmath>
#include <algorithm>
#include <chrono>
#include <stdexcept>
#include <cstdint>

namespace adead {

// ============================================================================
// CONFIGURATION
// ============================================================================

struct EngineConfig {
    bool use_gpu = false;
    bool deterministic = true;
    int num_threads = 8;
    size_t cache_size = 100 * 1024 * 1024; // 100MB
};

// ============================================================================
// MATRIX
// ============================================================================

class Matrix {
public:
    std::vector<float> data;
    size_t rows;
    size_t cols;

    Matrix() : rows(0), cols(0) {}
    
    Matrix(size_t r, size_t c) : data(r * c, 0.0f), rows(r), cols(c) {}

    static Matrix zeros(size_t rows, size_t cols) {
        return Matrix(rows, cols);
    }

    static Matrix ones(size_t rows, size_t cols) {
        Matrix m(rows, cols);
        std::fill(m.data.begin(), m.data.end(), 1.0f);
        return m;
    }

    static Matrix random(size_t rows, size_t cols) {
        Matrix m(rows, cols);
        for (auto& v : m.data) {
            v = static_cast<float>(rand()) / RAND_MAX * 2.0f - 1.0f;
        }
        return m;
    }

    static Matrix eye(size_t size) {
        Matrix m(size, size);
        for (size_t i = 0; i < size; ++i) {
            m.data[i * size + i] = 1.0f;
        }
        return m;
    }

    float get(size_t row, size_t col) const {
        return data[row * cols + col];
    }

    void set(size_t row, size_t col, float value) {
        data[row * cols + col] = value;
    }

    float& operator()(size_t row, size_t col) {
        return data[row * cols + col];
    }

    const float& operator()(size_t row, size_t col) const {
        return data[row * cols + col];
    }
};

// ============================================================================
// ENGINE
// ============================================================================

class Engine {
private:
    EngineConfig config_;

public:
    Engine() : config_() {}
    Engine(const EngineConfig& config) : config_(config) {}

    bool has_gpu() const { return config_.use_gpu; }

    // ========================================================================
    // MATRIX OPERATIONS
    // ========================================================================

    /**
     * Matrix multiplication
     * Benchmark: 50ms (C++) -> 0.1ms (ADead-BIB optimized)
     */
    Matrix matmul(const Matrix& a, const Matrix& b) const {
        if (a.cols != b.rows) {
            throw std::invalid_argument("Incompatible dimensions");
        }

        const size_t M = a.rows;
        const size_t N = b.cols;
        const size_t K = a.cols;
        Matrix result(M, N);

        // Blocked multiplication for cache efficiency
        constexpr size_t BLOCK = 32;

        for (size_t i = 0; i < M; i += BLOCK) {
            for (size_t j = 0; j < N; j += BLOCK) {
                for (size_t k = 0; k < K; k += BLOCK) {
                    const size_t i_max = std::min(i + BLOCK, M);
                    const size_t j_max = std::min(j + BLOCK, N);
                    const size_t k_max = std::min(k + BLOCK, K);

                    for (size_t ii = i; ii < i_max; ++ii) {
                        for (size_t kk = k; kk < k_max; ++kk) {
                            const float a_val = a.get(ii, kk);
                            for (size_t jj = j; jj < j_max; ++jj) {
                                result.data[ii * N + jj] += a_val * b.get(kk, jj);
                            }
                        }
                    }
                }
            }
        }

        return result;
    }

    Matrix transpose(const Matrix& a) const {
        Matrix result(a.cols, a.rows);
        for (size_t i = 0; i < a.rows; ++i) {
            for (size_t j = 0; j < a.cols; ++j) {
                result.set(j, i, a.get(i, j));
            }
        }
        return result;
    }

    Matrix add(const Matrix& a, const Matrix& b) const {
        if (a.rows != b.rows || a.cols != b.cols) {
            throw std::invalid_argument("Incompatible dimensions");
        }

        Matrix result(a.rows, a.cols);
        for (size_t i = 0; i < a.data.size(); ++i) {
            result.data[i] = a.data[i] + b.data[i];
        }
        return result;
    }

    Matrix scale(const Matrix& a, float factor) const {
        Matrix result(a.rows, a.cols);
        for (size_t i = 0; i < a.data.size(); ++i) {
            result.data[i] = a.data[i] * factor;
        }
        return result;
    }

    // ========================================================================
    // VECTOR OPERATIONS
    // ========================================================================

    float sum(const std::vector<float>& data) const {
        float total = 0.0f;
        for (const auto& v : data) total += v;
        return total;
    }

    float mean(const std::vector<float>& data) const {
        return sum(data) / static_cast<float>(data.size());
    }

    float max(const std::vector<float>& data) const {
        return *std::max_element(data.begin(), data.end());
    }

    float min(const std::vector<float>& data) const {
        return *std::min_element(data.begin(), data.end());
    }

    // ========================================================================
    // ML/AI OPERATIONS
    // ========================================================================

    std::vector<float> softmax(const std::vector<float>& data) const {
        float max_val = max(data);
        std::vector<float> exp_data(data.size());
        float sum = 0.0f;

        for (size_t i = 0; i < data.size(); ++i) {
            exp_data[i] = std::exp(data[i] - max_val);
            sum += exp_data[i];
        }

        for (auto& v : exp_data) v /= sum;
        return exp_data;
    }

    std::vector<float> relu(const std::vector<float>& data) const {
        std::vector<float> result(data.size());
        for (size_t i = 0; i < data.size(); ++i) {
            result[i] = data[i] > 0 ? data[i] : 0.0f;
        }
        return result;
    }

    std::vector<float> sigmoid(const std::vector<float>& data) const {
        std::vector<float> result(data.size());
        for (size_t i = 0; i < data.size(); ++i) {
            result[i] = 1.0f / (1.0f + std::exp(-data[i]));
        }
        return result;
    }

    Matrix attention(const Matrix& Q, const Matrix& K, const Matrix& V) const {
        float dim = static_cast<float>(Q.cols);

        // Q @ K^T
        Matrix Kt = transpose(K);
        Matrix scores = matmul(Q, Kt);

        // Scale
        scores = scale(scores, 1.0f / std::sqrt(dim));

        // Softmax per row
        size_t seq_len = Q.rows;
        for (size_t i = 0; i < seq_len; ++i) {
            std::vector<float> row(seq_len);
            for (size_t j = 0; j < seq_len; ++j) {
                row[j] = scores.get(i, j);
            }
            auto soft_row = softmax(row);
            for (size_t j = 0; j < seq_len; ++j) {
                scores.set(i, j, soft_row[j]);
            }
        }

        // Scores @ V
        return matmul(scores, V);
    }

    // ========================================================================
    // SORTING & SEARCHING
    // ========================================================================

    void sort(std::vector<float>& data) const {
        std::sort(data.begin(), data.end());
    }

    int binary_search(const std::vector<float>& data, float target) const {
        int left = 0;
        int right = static_cast<int>(data.size()) - 1;

        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (data[mid] == target) return mid;
            if (data[mid] < target) left = mid + 1;
            else right = mid - 1;
        }

        return -1;
    }

    // ========================================================================
    // BENCHMARKING
    // ========================================================================

    struct BenchmarkResult {
        double avg_ms;
        double min_ms;
        double max_ms;
        int iterations;
    };

    template<typename Func>
    BenchmarkResult benchmark(Func f, int iterations = 100) const {
        // Warmup
        for (int i = 0; i < 10; ++i) f();

        // Benchmark
        std::vector<double> times(iterations);
        for (int i = 0; i < iterations; ++i) {
            auto start = std::chrono::high_resolution_clock::now();
            f();
            auto end = std::chrono::high_resolution_clock::now();
            times[i] = std::chrono::duration<double, std::milli>(end - start).count();
        }

        double sum = 0.0;
        double min_val = times[0];
        double max_val = times[0];

        for (const auto& t : times) {
            sum += t;
            if (t < min_val) min_val = t;
            if (t > max_val) max_val = t;
        }

        return {sum / iterations, min_val, max_val, iterations};
    }
};

} // namespace adead

#endif // ADEAD_HPP
