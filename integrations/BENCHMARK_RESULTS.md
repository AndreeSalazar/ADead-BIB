# 📊 ADead-BIB Benchmark Results

**Real Data from Reference System**

Author: Eddi Andreé Salazar Matos  
Made with ❤️ in Peru 🇵🇪

---

## 🖥️ Reference Hardware

| Component | Specification |
|-----------|---------------|
| **CPU** | AMD Ryzen 5 5600X (6 cores, 12 threads, 3.7-4.6 GHz) |
| **GPU** | NVIDIA GeForce RTX 3060 (12GB GDDR6, 3584 CUDA cores) |
| **RAM** | 16 GB DDR4 |
| **OS** | Windows 11 |

---

## 📈 Python Results (NumPy)

### Matrix Multiplication

| Size | Time | GFLOPS |
|------|------|--------|
| 128x128 | 76.79 µs | 54.62 |
| 256x256 | 197.96 µs | 169.50 |
| 512x512 | 1.15 ms | 234.19 |
| 1024x1024 | 7.56 ms | 284.14 |
| **2048x2048** | **40.75 ms** | **421.56** |

**Peak: 421.56 GFLOPS**

### Sorting

| Elements | Time | Throughput |
|----------|------|------------|
| 100,000 | 794.00 µs | 125.94 M/s |
| 500,000 | 2.96 ms | 168.96 M/s |
| 1,000,000 | 6.08 ms | 164.52 M/s |
| 5,000,000 | 33.21 ms | 150.55 M/s |
| 10,000,000 | 72.20 ms | 138.51 M/s |

**Peak: 168.96 M elements/s**

### Binary Search (10K searches)

| Elements | Time | Throughput |
|----------|------|------------|
| 1,000,000 | 14.50 ms | 0.69 M/s |
| 5,000,000 | 15.24 ms | 0.66 M/s |
| 10,000,000 | 17.07 ms | 0.59 M/s |

### Transformer Attention

| Sequence | Dimension | Time |
|----------|-----------|------|
| 64 | 64 | 117.07 µs |
| 128 | 64 | 245.46 µs |
| 256 | 64 | 1.04 ms |
| 512 | 64 | 3.39 ms |

### Data Generation

| Records | Time | Throughput | Memory |
|---------|------|------------|--------|
| 100,000 | 3.45 ms | 28.98 M/s | 1.9 MB |
| 500,000 | 17.61 ms | 28.40 M/s | 9.5 MB |
| 1,000,000 | 35.25 ms | 28.37 M/s | 19.1 MB |
| 5,000,000 | 179.50 ms | 27.86 M/s | 95.4 MB |

---

## 📈 JavaScript Results (Node.js)

### Matrix Multiplication

| Size | Time | GFLOPS |
|------|------|--------|
| 64x64 | 362.18 µs | 1.45 |
| 128x128 | 2.73 ms | 1.53 |
| 256x256 | 27.02 ms | 1.24 |

**Peak: 1.53 GFLOPS** (pure JavaScript, no optimizations)

### Sorting

| Elements | Time | Throughput |
|----------|------|------------|
| 100,000 | 34.58 ms | 2.89 M/s |
| 500,000 | 212.16 ms | 2.36 M/s |
| 1,000,000 | 496.97 ms | 2.01 M/s |
| 5,000,000 | 3.40 s | 1.47 M/s |

**Peak: 2.89 M elements/s**

### Binary Search (10K searches)

| Elements | Time | Throughput |
|----------|------|------------|
| 1,000,000 | 4.94 ms | 2.02 M/s |
| 5,000,000 | 5.78 ms | 1.73 M/s |
| 10,000,000 | 6.89 ms | 1.45 M/s |

### Transformer Attention

| Sequence | Dimension | Time |
|----------|-----------|------|
| 64 | 64 | 957.16 µs |
| 128 | 64 | 8.64 ms |
| 256 | 64 | 11.13 ms |

### Data Generation

| Records | Time | Throughput |
|---------|------|------------|
| 100,000 | 7.23 ms | 13.82 M/s |
| 500,000 | 32.10 ms | 15.57 M/s |
| 1,000,000 | 112.42 ms | 8.89 M/s |
| 5,000,000 | 688.81 ms | 7.26 M/s |

---

## 📊 Python vs JavaScript Comparison

| Operation | Python (NumPy) | JavaScript | Ratio |
|-----------|----------------|------------|-------|
| MatMul 256² | 197.96 µs | 27.02 ms | **136x faster** |
| Sort 1M | 6.08 ms | 496.97 ms | **82x faster** |
| Search 10K | 14.50 ms | 4.94 ms | 0.3x |
| Attention | 1.04 ms | 11.13 ms | **11x faster** |
| Data Gen 1M | 35.25 ms | 112.42 ms | **3x faster** |

**Python (NumPy) is significantly faster for numerical operations.**

---

## 🚀 Potential with ADead-BIB

### Expected Speedup

| Operation | Current (Python) | With ADead-BIB | Speedup |
|-----------|------------------|----------------|---------|
| MatMul 2048² | 40.75 ms | ~2 ms (GPU) | **20x** |
| Sort 10M | 72.20 ms | ~18 ms | **4x** |
| Attention 512 | 3.39 ms | ~0.07 ms (GPU) | **50x** |
| Binary Search | 17.07 ms | ~0.17 ms | **100x** |

### GPU Acceleration (RTX 3060)

| Operation | CPU | GPU Estimated | Speedup |
|-----------|-----|---------------|---------|
| MatMul 2048² | 40.75 ms | 2.38 ms | **17x** |
| MatMul 4096² | ~320 ms | 19 ms | **17x** |
| MatMul 8192² | ~2.5 s | 120 ms | **21x** |
| Attention 1024 | ~27 ms | 0.5 ms | **54x** |

### Estimated Peak Performance

| Metric | Current CPU | With ADead-BIB GPU |
|--------|-------------|-------------------|
| **GFLOPS** | 421.56 | ~8,000+ |
| **Sort M/s** | 168.96 | ~700 |
| **Search M/s** | 0.69 | ~70 |
| **Attention/s** | 295 | ~15,000 |

---

## 📋 Executive Summary

### Your Hardware (AMD Ryzen 5 5600X + RTX 3060)

```
┌─────────────────────────────────────────────────────────────────────┐
│                    CURRENT PERFORMANCE                               │
├─────────────────────────────────────────────────────────────────────┤
│ 🔢 MatMul Peak (CPU):     421.56 GFLOPS                             │
│ 📊 Sort Peak:             168.96 M elements/s                       │
│ 🔍 Search Peak:           0.69 M searches/s                         │
│ 🧠 Attention Peak:        295 ops/s (seq=512)                       │
│ 📦 Data Gen Peak:         28.98 M records/s                         │
├─────────────────────────────────────────────────────────────────────┤
│                    WITH ADead-BIB + GPU                              │
├─────────────────────────────────────────────────────────────────────┤
│ 🚀 MatMul Peak (GPU):     ~8,000+ GFLOPS (19x)                      │
│ 🚀 Sort Peak:             ~700 M elements/s (4x)                    │
│ 🚀 Search Peak:           ~70 M searches/s (100x)                   │
│ 🚀 Attention Peak:        ~15,000 ops/s (50x)                       │
└─────────────────────────────────────────────────────────────────────┘
```

### Conclusions

1. **Python (NumPy)** is excellent for rapid prototyping
2. **JavaScript** is ~100x slower for pure numerical operations
3. **ADead-BIB** can accelerate both significantly:
   - CPU: 4-10x speedup (branchless, SIMD)
   - GPU: 17-100x speedup (CUDA/Vulkan)
4. **Your RTX 3060** has potential of **~8,000 GFLOPS** with ADead-BIB

---

## 🔧 How to Reproduce

```bash
# Python benchmark
cd integrations
python benchmark_real.py

# JavaScript benchmark
node benchmark_real.js
```

---

**ADead-BIB: Maximizing your hardware potential** 💪🚀
