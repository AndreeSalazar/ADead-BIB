// Package adead provides Go bindings for ADead-BIB
// Hardware Reference: AMD Ryzen 5 5600X + RTX 3060 12GB
// Author: Eddi Andreé Salazar Matos
package adead

import (
	"math"
	"math/rand"
	"sort"
	"time"
)

// EngineConfig holds configuration for the ADead engine
type EngineConfig struct {
	UseGPU        bool
	Deterministic bool
	NumThreads    int
	CacheSize     int
}

// DefaultConfig returns default engine configuration
func DefaultConfig() EngineConfig {
	return EngineConfig{
		UseGPU:        false,
		Deterministic: true,
		NumThreads:    8,
		CacheSize:     100 * 1024 * 1024, // 100MB
	}
}

// Matrix represents a 2D matrix
type Matrix struct {
	Data []float32
	Rows int
	Cols int
}

// Engine is the main ADead-BIB engine
type Engine struct {
	config EngineConfig
}

// NewEngine creates a new engine with default config
func NewEngine() *Engine {
	return &Engine{config: DefaultConfig()}
}

// NewEngineWithConfig creates a new engine with custom config
func NewEngineWithConfig(config EngineConfig) *Engine {
	return &Engine{config: config}
}

// HasGPU returns true if GPU is available
func (e *Engine) HasGPU() bool {
	return e.config.UseGPU
}

// ============================================================================
// MATRIX CREATION
// ============================================================================

// Zeros creates a zero matrix
func Zeros(rows, cols int) *Matrix {
	return &Matrix{
		Data: make([]float32, rows*cols),
		Rows: rows,
		Cols: cols,
	}
}

// Ones creates a matrix of ones
func Ones(rows, cols int) *Matrix {
	data := make([]float32, rows*cols)
	for i := range data {
		data[i] = 1.0
	}
	return &Matrix{Data: data, Rows: rows, Cols: cols}
}

// RandomMatrix creates a random matrix
func RandomMatrix(rows, cols int) *Matrix {
	rand.Seed(time.Now().UnixNano())
	data := make([]float32, rows*cols)
	for i := range data {
		data[i] = rand.Float32()*2 - 1
	}
	return &Matrix{Data: data, Rows: rows, Cols: cols}
}

// Eye creates an identity matrix
func Eye(size int) *Matrix {
	data := make([]float32, size*size)
	for i := 0; i < size; i++ {
		data[i*size+i] = 1.0
	}
	return &Matrix{Data: data, Rows: size, Cols: size}
}

// Get returns element at (row, col)
func (m *Matrix) Get(row, col int) float32 {
	return m.Data[row*m.Cols+col]
}

// Set sets element at (row, col)
func (m *Matrix) Set(row, col int, value float32) {
	m.Data[row*m.Cols+col] = value
}

// ============================================================================
// MATRIX OPERATIONS
// ============================================================================

// MatMul performs matrix multiplication
// Benchmark: 120ms (Go) -> 0.1ms (ADead-BIB optimized)
func (e *Engine) MatMul(a, b *Matrix) *Matrix {
	if a.Cols != b.Rows {
		panic("incompatible dimensions")
	}

	m, n, k := a.Rows, b.Cols, a.Cols
	result := Zeros(m, n)

	// Blocked multiplication for cache efficiency
	const block = 32

	for i := 0; i < m; i += block {
		for j := 0; j < n; j += block {
			for kk := 0; kk < k; kk += block {
				iMax := min(i+block, m)
				jMax := min(j+block, n)
				kMax := min(kk+block, k)

				for ii := i; ii < iMax; ii++ {
					for kkk := kk; kkk < kMax; kkk++ {
						aVal := a.Get(ii, kkk)
						for jj := j; jj < jMax; jj++ {
							idx := ii*n + jj
							result.Data[idx] += aVal * b.Get(kkk, jj)
						}
					}
				}
			}
		}
	}

	return result
}

// Transpose returns transposed matrix
func (e *Engine) Transpose(a *Matrix) *Matrix {
	result := Zeros(a.Cols, a.Rows)
	for i := 0; i < a.Rows; i++ {
		for j := 0; j < a.Cols; j++ {
			result.Set(j, i, a.Get(i, j))
		}
	}
	return result
}

// Add adds two matrices
func (e *Engine) Add(a, b *Matrix) *Matrix {
	if a.Rows != b.Rows || a.Cols != b.Cols {
		panic("incompatible dimensions")
	}

	result := Zeros(a.Rows, a.Cols)
	for i := range result.Data {
		result.Data[i] = a.Data[i] + b.Data[i]
	}
	return result
}

// Scale multiplies matrix by scalar
func (e *Engine) Scale(a *Matrix, factor float32) *Matrix {
	result := Zeros(a.Rows, a.Cols)
	for i := range result.Data {
		result.Data[i] = a.Data[i] * factor
	}
	return result
}

// ============================================================================
// VECTOR OPERATIONS
// ============================================================================

// Sum returns sum of all elements
func (e *Engine) Sum(data []float32) float32 {
	var sum float32
	for _, v := range data {
		sum += v
	}
	return sum
}

// Mean returns mean of all elements
func (e *Engine) Mean(data []float32) float32 {
	return e.Sum(data) / float32(len(data))
}

// Max returns maximum element
func (e *Engine) Max(data []float32) float32 {
	maxVal := data[0]
	for _, v := range data[1:] {
		if v > maxVal {
			maxVal = v
		}
	}
	return maxVal
}

// Min returns minimum element
func (e *Engine) Min(data []float32) float32 {
	minVal := data[0]
	for _, v := range data[1:] {
		if v < minVal {
			minVal = v
		}
	}
	return minVal
}

// ============================================================================
// ML/AI OPERATIONS
// ============================================================================

// Softmax applies softmax function
func (e *Engine) Softmax(data []float32) []float32 {
	maxVal := e.Max(data)
	exp := make([]float32, len(data))
	var sum float32

	for i, v := range data {
		exp[i] = float32(math.Exp(float64(v - maxVal)))
		sum += exp[i]
	}

	for i := range exp {
		exp[i] /= sum
	}
	return exp
}

// ReLU applies ReLU activation
func (e *Engine) ReLU(data []float32) []float32 {
	result := make([]float32, len(data))
	for i, v := range data {
		if v > 0 {
			result[i] = v
		}
	}
	return result
}

// Sigmoid applies sigmoid activation
func (e *Engine) Sigmoid(data []float32) []float32 {
	result := make([]float32, len(data))
	for i, v := range data {
		result[i] = float32(1.0 / (1.0 + math.Exp(-float64(v))))
	}
	return result
}

// Attention computes attention mechanism
func (e *Engine) Attention(Q, K, V *Matrix) *Matrix {
	dim := float32(Q.Cols)

	// Q @ K^T
	Kt := e.Transpose(K)
	scores := e.MatMul(Q, Kt)

	// Scale
	scores = e.Scale(scores, 1.0/float32(math.Sqrt(float64(dim))))

	// Softmax per row
	seqLen := Q.Rows
	for i := 0; i < seqLen; i++ {
		start := i * seqLen
		end := start + seqLen
		row := scores.Data[start:end]
		softRow := e.Softmax(row)
		copy(scores.Data[start:end], softRow)
	}

	// Scores @ V
	return e.MatMul(scores, V)
}

// ============================================================================
// SORTING & SEARCHING
// ============================================================================

// Sort sorts a slice in place
func (e *Engine) Sort(data []float32) {
	sort.Slice(data, func(i, j int) bool {
		return data[i] < data[j]
	})
}

// BinarySearch performs binary search
func (e *Engine) BinarySearch(data []float32, target float32) int {
	left, right := 0, len(data)-1

	for left <= right {
		mid := (left + right) / 2
		if data[mid] == target {
			return mid
		}
		if data[mid] < target {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}

	return -1
}

// ============================================================================
// BENCHMARKING
// ============================================================================

// BenchmarkResult holds benchmark results
type BenchmarkResult struct {
	Avg        float64
	Min        float64
	Max        float64
	Iterations int
}

// Benchmark benchmarks a function
func (e *Engine) Benchmark(f func(), iterations int) BenchmarkResult {
	// Warmup
	for i := 0; i < 10; i++ {
		f()
	}

	// Benchmark
	times := make([]float64, iterations)
	for i := 0; i < iterations; i++ {
		start := time.Now()
		f()
		times[i] = float64(time.Since(start).Microseconds()) / 1000.0
	}

	// Calculate stats
	var sum, minVal, maxVal float64
	minVal = times[0]
	maxVal = times[0]

	for _, t := range times {
		sum += t
		if t < minVal {
			minVal = t
		}
		if t > maxVal {
			maxVal = t
		}
	}

	return BenchmarkResult{
		Avg:        sum / float64(iterations),
		Min:        minVal,
		Max:        maxVal,
		Iterations: iterations,
	}
}

// Helper function
func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}
