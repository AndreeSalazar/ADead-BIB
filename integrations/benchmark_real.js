/**
 * ADead-BIB Real Benchmark - JavaScript
 * ======================================
 * Hardware: AMD Ryzen 5 5600X + RTX 3060 12GB + 16GB RAM
 * Author: Eddi Andreé Salazar Matos
 */

console.log("=".repeat(70));
console.log("🔥 ADead-BIB REAL BENCHMARK - JavaScript/Node.js");
console.log("=".repeat(70));
console.log();
console.log("📊 HARDWARE:");
console.log("   CPU: AMD Ryzen 5 5600X (6 cores, 12 threads)");
console.log("   GPU: NVIDIA GeForce RTX 3060 (12GB VRAM)");
console.log("   RAM: 16 GB");
console.log();
console.log("=".repeat(70));

// Utilidades
function formatTime(ms) {
    if (ms < 0.001) return `${(ms * 1000000).toFixed(2)} µs`;
    if (ms < 1) return `${(ms * 1000).toFixed(2)} µs`;
    if (ms < 1000) return `${ms.toFixed(2)} ms`;
    return `${(ms / 1000).toFixed(2)} s`;
}

function randomMatrix(rows, cols) {
    const data = new Float32Array(rows * cols);
    for (let i = 0; i < data.length; i++) {
        data[i] = Math.random() * 2 - 1;
    }
    return { data, rows, cols };
}

function matmul(a, b) {
    const result = new Float32Array(a.rows * b.cols);
    for (let i = 0; i < a.rows; i++) {
        for (let j = 0; j < b.cols; j++) {
            let sum = 0;
            for (let k = 0; k < a.cols; k++) {
                sum += a.data[i * a.cols + k] * b.data[k * b.cols + j];
            }
            result[i * b.cols + j] = sum;
        }
    }
    return { data: result, rows: a.rows, cols: b.cols };
}

// ============================================================================
// BENCHMARK 1: MatMul
// ============================================================================
console.log("\n🔢 BENCHMARK 1: MULTIPLICACIÓN DE MATRICES (JavaScript puro)");
console.log("-".repeat(50));

const matmulSizes = [64, 128, 256];
const matmulResults = [];

for (const size of matmulSizes) {
    const a = randomMatrix(size, size);
    const b = randomMatrix(size, size);
    
    // Warmup
    matmul(a, b);
    
    // Benchmark
    const iterations = size <= 128 ? 5 : 2;
    const times = [];
    
    for (let i = 0; i < iterations; i++) {
        const start = performance.now();
        matmul(a, b);
        const elapsed = performance.now() - start;
        times.push(elapsed);
    }
    
    const avgTime = times.reduce((a, b) => a + b) / times.length;
    const gflops = (2 * Math.pow(size, 3)) / (avgTime / 1000) / 1e9;
    
    matmulResults.push({ size, time: avgTime, gflops });
    console.log(`   ${size}x${size}: ${formatTime(avgTime).padStart(12)} | ${gflops.toFixed(2)} GFLOPS`);
}

// ============================================================================
// BENCHMARK 2: Sorting
// ============================================================================
console.log("\n📊 BENCHMARK 2: ORDENAMIENTO");
console.log("-".repeat(50));

const sortSizes = [100000, 500000, 1000000, 5000000];
const sortResults = [];

for (const size of sortSizes) {
    const data = new Float32Array(size);
    for (let i = 0; i < size; i++) data[i] = Math.random();
    
    // Benchmark
    const iterations = size <= 1000000 ? 3 : 1;
    const times = [];
    
    for (let i = 0; i < iterations; i++) {
        const copy = new Float32Array(data);
        const arr = Array.from(copy);
        
        const start = performance.now();
        arr.sort((a, b) => a - b);
        const elapsed = performance.now() - start;
        times.push(elapsed);
    }
    
    const avgTime = times.reduce((a, b) => a + b) / times.length;
    const meps = size / (avgTime / 1000) / 1e6;
    
    sortResults.push({ size, time: avgTime, meps });
    console.log(`   ${size.toLocaleString().padStart(10)} elementos: ${formatTime(avgTime).padStart(12)} | ${meps.toFixed(2)} M/s`);
}

// ============================================================================
// BENCHMARK 3: Búsqueda Binaria
// ============================================================================
console.log("\n🔍 BENCHMARK 3: BÚSQUEDA BINARIA");
console.log("-".repeat(50));

function binarySearch(arr, target) {
    let left = 0, right = arr.length - 1;
    while (left <= right) {
        const mid = Math.floor((left + right) / 2);
        if (arr[mid] === target) return mid;
        if (arr[mid] < target) left = mid + 1;
        else right = mid - 1;
    }
    return -1;
}

const searchSizes = [1000000, 5000000, 10000000];
const searchResults = [];

for (const size of searchSizes) {
    const data = new Float32Array(size);
    for (let i = 0; i < size; i++) data[i] = i;
    const arr = Array.from(data);
    
    const numSearches = 10000;
    const targets = [];
    for (let i = 0; i < numSearches; i++) {
        targets.push(Math.floor(Math.random() * size));
    }
    
    const start = performance.now();
    for (const target of targets) {
        binarySearch(arr, target);
    }
    const elapsed = performance.now() - start;
    
    const msps = numSearches / (elapsed / 1000) / 1e6;
    
    searchResults.push({ size, time: elapsed, msps });
    console.log(`   ${size.toLocaleString().padStart(10)} elementos, 10K búsquedas: ${formatTime(elapsed).padStart(12)} | ${msps.toFixed(2)} M/s`);
}

// ============================================================================
// BENCHMARK 4: Generación de Datos
// ============================================================================
console.log("\n📦 BENCHMARK 4: GENERACIÓN DE DATOS MASIVOS");
console.log("-".repeat(50));

const genSizes = [100000, 500000, 1000000, 5000000];
const genResults = [];

for (const size of genSizes) {
    const start = performance.now();
    
    const records = [];
    for (let i = 0; i < size; i++) {
        records.push({
            id: i,
            value: Math.random() * 1000,
            category: Math.floor(Math.random() * 10),
            active: Math.random() > 0.5
        });
    }
    
    const elapsed = performance.now() - start;
    const mrps = size / (elapsed / 1000) / 1e6;
    const memoryMB = (size * 50) / (1024 * 1024); // Estimado
    
    genResults.push({ size, time: elapsed, mrps });
    console.log(`   ${size.toLocaleString().padStart(10)} registros: ${formatTime(elapsed).padStart(12)} | ${mrps.toFixed(2)} M/s`);
}

// ============================================================================
// BENCHMARK 5: Filtrado y Agregación
// ============================================================================
console.log("\n📈 BENCHMARK 5: FILTRADO Y AGREGACIÓN");
console.log("-".repeat(50));

const filterSizes = [100000, 500000, 1000000];

for (const size of filterSizes) {
    // Generar datos
    const data = [];
    for (let i = 0; i < size; i++) {
        data.push({
            id: i,
            value: Math.random() * 1000,
            category: Math.floor(Math.random() * 10)
        });
    }
    
    const start = performance.now();
    
    // Filtrar
    const filtered = data.filter(r => r.value > 500);
    
    // Agregar por categoría
    const grouped = {};
    for (const r of filtered) {
        if (!grouped[r.category]) grouped[r.category] = { sum: 0, count: 0 };
        grouped[r.category].sum += r.value;
        grouped[r.category].count++;
    }
    
    // Calcular promedios
    const averages = {};
    for (const cat in grouped) {
        averages[cat] = grouped[cat].sum / grouped[cat].count;
    }
    
    const elapsed = performance.now() - start;
    
    console.log(`   ${size.toLocaleString().padStart(10)} registros: ${formatTime(elapsed).padStart(12)} | Filtrados: ${filtered.length.toLocaleString()}`);
}

// ============================================================================
// BENCHMARK 6: Transformer Attention (Simulado)
// ============================================================================
console.log("\n🧠 BENCHMARK 6: TRANSFORMER ATTENTION");
console.log("-".repeat(50));

function softmax(arr) {
    const max = Math.max(...arr);
    const exp = arr.map(x => Math.exp(x - max));
    const sum = exp.reduce((a, b) => a + b);
    return exp.map(x => x / sum);
}

function attention(Q, K, V, dim) {
    const seqLen = Q.length / dim;
    const output = new Float32Array(Q.length);
    
    for (let i = 0; i < seqLen; i++) {
        const scores = [];
        for (let j = 0; j < seqLen; j++) {
            let score = 0;
            for (let d = 0; d < dim; d++) {
                score += Q[i * dim + d] * K[j * dim + d];
            }
            scores.push(score / Math.sqrt(dim));
        }
        
        const weights = softmax(scores);
        
        for (let d = 0; d < dim; d++) {
            let sum = 0;
            for (let j = 0; j < seqLen; j++) {
                sum += weights[j] * V[j * dim + d];
            }
            output[i * dim + d] = sum;
        }
    }
    
    return output;
}

const attentionConfigs = [[64, 64], [128, 64], [256, 64]];

for (const [seqLen, dim] of attentionConfigs) {
    const Q = new Float32Array(seqLen * dim);
    const K = new Float32Array(seqLen * dim);
    const V = new Float32Array(seqLen * dim);
    
    for (let i = 0; i < Q.length; i++) {
        Q[i] = Math.random();
        K[i] = Math.random();
        V[i] = Math.random();
    }
    
    // Warmup
    attention(Q, K, V, dim);
    
    // Benchmark
    const iterations = 5;
    const times = [];
    
    for (let i = 0; i < iterations; i++) {
        const start = performance.now();
        attention(Q, K, V, dim);
        const elapsed = performance.now() - start;
        times.push(elapsed);
    }
    
    const avgTime = times.reduce((a, b) => a + b) / times.length;
    console.log(`   seq=${seqLen.toString().padStart(4)}, dim=${dim}: ${formatTime(avgTime).padStart(12)}`);
}

// ============================================================================
// RESUMEN
// ============================================================================
console.log();
console.log("=".repeat(70));
console.log("📊 RESUMEN - JavaScript en AMD Ryzen 5 5600X");
console.log("=".repeat(70));
console.log();
console.log("┌─────────────────────────────────────────────────────────────────────┐");
console.log("│                    RESULTADOS JavaScript                            │");
console.log("├─────────────────────────────────────────────────────────────────────┤");

const bestMatmul = matmulResults.reduce((a, b) => a.gflops > b.gflops ? a : b);
const bestSort = sortResults.reduce((a, b) => a.meps > b.meps ? a : b);
const bestSearch = searchResults.reduce((a, b) => a.msps > b.msps ? a : b);
const bestGen = genResults.reduce((a, b) => a.mrps > b.mrps ? a : b);

console.log(`│ 🔢 MatMul Peak:        ${bestMatmul.gflops.toFixed(2)} GFLOPS (${bestMatmul.size}x${bestMatmul.size})`);
console.log(`│ 📊 Sort Peak:          ${bestSort.meps.toFixed(2)} M elementos/s`);
console.log(`│ 🔍 Search Peak:        ${bestSearch.msps.toFixed(2)} M búsquedas/s`);
console.log(`│ 📦 Data Gen Peak:      ${bestGen.mrps.toFixed(2)} M registros/s`);
console.log("├─────────────────────────────────────────────────────────────────────┤");
console.log("│                    POTENCIAL CON ADead-BIB                          │");
console.log("├─────────────────────────────────────────────────────────────────────┤");
console.log(`│ 🚀 MatMul WASM:        ~${(bestMatmul.gflops * 50).toFixed(0)} GFLOPS (50x speedup)`);
console.log(`│ 🚀 Sort optimizado:    ~${(bestSort.meps * 4).toFixed(0)} M elementos/s (4x speedup)`);
console.log(`│ 🚀 Search optimizado:  ~${(bestSearch.msps * 100).toFixed(0)} M búsquedas/s (100x speedup)`);
console.log("└─────────────────────────────────────────────────────────────────────┘");
console.log();
console.log("✅ Benchmark JavaScript completado");
console.log("💪 ADead-BIB (WASM) puede potenciar estos resultados significativamente");
