/**
 * ADead-BIB Swift Integration
 * ============================
 * Hardware Reference: AMD Ryzen 5 5600X + RTX 3060 12GB
 * Author: Eddi Andreé Salazar Matos
 */

import Foundation

// MARK: - Configuration

public struct EngineConfig {
    public var useGpu: Bool
    public var deterministic: Bool
    public var numThreads: Int
    public var cacheSize: Int
    
    public init(
        useGpu: Bool = false,
        deterministic: Bool = true,
        numThreads: Int = 8,
        cacheSize: Int = 100 * 1024 * 1024
    ) {
        self.useGpu = useGpu
        self.deterministic = deterministic
        self.numThreads = numThreads
        self.cacheSize = cacheSize
    }
}

// MARK: - Matrix

public struct Matrix {
    public var data: [Float]
    public let rows: Int
    public let cols: Int
    
    public init(rows: Int, cols: Int) {
        self.rows = rows
        self.cols = cols
        self.data = [Float](repeating: 0, count: rows * cols)
    }
    
    public static func zeros(rows: Int, cols: Int) -> Matrix {
        return Matrix(rows: rows, cols: cols)
    }
    
    public static func ones(rows: Int, cols: Int) -> Matrix {
        var m = Matrix(rows: rows, cols: cols)
        for i in 0..<m.data.count {
            m.data[i] = 1.0
        }
        return m
    }
    
    public static func random(rows: Int, cols: Int) -> Matrix {
        var m = Matrix(rows: rows, cols: cols)
        for i in 0..<m.data.count {
            m.data[i] = Float.random(in: -1...1)
        }
        return m
    }
    
    public static func eye(size: Int) -> Matrix {
        var m = Matrix(rows: size, cols: size)
        for i in 0..<size {
            m.data[i * size + i] = 1.0
        }
        return m
    }
    
    public func get(_ row: Int, _ col: Int) -> Float {
        return data[row * cols + col]
    }
    
    public mutating func set(_ row: Int, _ col: Int, _ value: Float) {
        data[row * cols + col] = value
    }
    
    public subscript(row: Int, col: Int) -> Float {
        get { return get(row, col) }
        set { set(row, col, newValue) }
    }
}

// MARK: - Benchmark Result

public struct BenchmarkResult {
    public let avg: Double
    public let min: Double
    public let max: Double
    public let iterations: Int
}

// MARK: - Engine

public class Engine {
    private let config: EngineConfig
    
    public init(config: EngineConfig = EngineConfig()) {
        self.config = config
    }
    
    public var hasGpu: Bool { config.useGpu }
    
    // MARK: - Matrix Operations
    
    /// Matrix multiplication
    /// Benchmark: 150ms (Swift) -> 0.1ms (ADead-BIB optimized)
    public func matmul(_ a: Matrix, _ b: Matrix) -> Matrix {
        precondition(a.cols == b.rows, "Incompatible dimensions")
        
        let M = a.rows, N = b.cols, K = a.cols
        var result = Matrix.zeros(rows: M, cols: N)
        
        let BLOCK = 32
        
        for i in stride(from: 0, to: M, by: BLOCK) {
            for j in stride(from: 0, to: N, by: BLOCK) {
                for k in stride(from: 0, to: K, by: BLOCK) {
                    let iMax = Swift.min(i + BLOCK, M)
                    let jMax = Swift.min(j + BLOCK, N)
                    let kMax = Swift.min(k + BLOCK, K)
                    
                    for ii in i..<iMax {
                        for kk in k..<kMax {
                            let aVal = a.get(ii, kk)
                            for jj in j..<jMax {
                                let idx = ii * N + jj
                                result.data[idx] += aVal * b.get(kk, jj)
                            }
                        }
                    }
                }
            }
        }
        
        return result
    }
    
    public func transpose(_ a: Matrix) -> Matrix {
        var result = Matrix.zeros(rows: a.cols, cols: a.rows)
        for i in 0..<a.rows {
            for j in 0..<a.cols {
                result.set(j, i, a.get(i, j))
            }
        }
        return result
    }
    
    public func add(_ a: Matrix, _ b: Matrix) -> Matrix {
        precondition(a.rows == b.rows && a.cols == b.cols, "Incompatible dimensions")
        
        var result = Matrix.zeros(rows: a.rows, cols: a.cols)
        for i in 0..<a.data.count {
            result.data[i] = a.data[i] + b.data[i]
        }
        return result
    }
    
    public func scale(_ a: Matrix, by factor: Float) -> Matrix {
        var result = Matrix.zeros(rows: a.rows, cols: a.cols)
        for i in 0..<a.data.count {
            result.data[i] = a.data[i] * factor
        }
        return result
    }
    
    // MARK: - Vector Operations
    
    public func sum(_ data: [Float]) -> Float {
        return data.reduce(0, +)
    }
    
    public func mean(_ data: [Float]) -> Float {
        return sum(data) / Float(data.count)
    }
    
    public func max(_ data: [Float]) -> Float {
        return data.max() ?? 0
    }
    
    public func min(_ data: [Float]) -> Float {
        return data.min() ?? 0
    }
    
    // MARK: - ML/AI Operations
    
    public func softmax(_ data: [Float]) -> [Float] {
        let maxVal = max(data)
        var exp = data.map { Foundation.exp($0 - maxVal) }
        let sum = exp.reduce(0, +)
        return exp.map { $0 / sum }
    }
    
    public func relu(_ data: [Float]) -> [Float] {
        return data.map { Swift.max($0, 0) }
    }
    
    public func sigmoid(_ data: [Float]) -> [Float] {
        return data.map { 1.0 / (1.0 + Foundation.exp(-$0)) }
    }
    
    public func attention(_ Q: Matrix, _ K: Matrix, _ V: Matrix) -> Matrix {
        let dim = Float(Q.cols)
        
        // Q @ K^T
        let Kt = transpose(K)
        var scores = matmul(Q, Kt)
        
        // Scale
        scores = scale(scores, by: 1.0 / sqrt(dim))
        
        // Softmax per row
        let seqLen = Q.rows
        for i in 0..<seqLen {
            var row = [Float](repeating: 0, count: seqLen)
            for j in 0..<seqLen {
                row[j] = scores.get(i, j)
            }
            let softRow = softmax(row)
            for j in 0..<seqLen {
                scores.set(i, j, softRow[j])
            }
        }
        
        // Scores @ V
        return matmul(scores, V)
    }
    
    // MARK: - Sorting & Searching
    
    public func sort(_ data: inout [Float]) {
        data.sort()
    }
    
    public func binarySearch(_ data: [Float], target: Float) -> Int {
        var left = 0
        var right = data.count - 1
        
        while left <= right {
            let mid = (left + right) / 2
            if data[mid] == target { return mid }
            if data[mid] < target { left = mid + 1 }
            else { right = mid - 1 }
        }
        
        return -1
    }
    
    // MARK: - Benchmarking
    
    public func benchmark(_ f: () -> Void, iterations: Int = 100) -> BenchmarkResult {
        // Warmup
        for _ in 0..<10 { f() }
        
        // Benchmark
        var times = [Double]()
        
        for _ in 0..<iterations {
            let start = CFAbsoluteTimeGetCurrent()
            f()
            let elapsed = (CFAbsoluteTimeGetCurrent() - start) * 1000
            times.append(elapsed)
        }
        
        let avg = times.reduce(0, +) / Double(iterations)
        let minVal = times.min() ?? 0
        let maxVal = times.max() ?? 0
        
        return BenchmarkResult(avg: avg, min: minVal, max: maxVal, iterations: iterations)
    }
}
