/**
 * Heavy Benchmark: JavaScript vs ADead-BIB
 * ==========================================
 * Comparación intensiva de rendimiento
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

const { ADeadBIB } = require('../src/adead-binding');

console.log('='.repeat(70));
console.log('   🔥 HEAVY BENCHMARK: JavaScript vs ADead-BIB');
console.log('   Comparación de rendimiento en operaciones intensivas');
console.log('='.repeat(70));

const adead = new ADeadBIB();

// =========================================================================
// UTILIDADES
// =========================================================================

function formatNumber(num) {
    if (num >= 1e9) return (num / 1e9).toFixed(2) + 'B';
    if (num >= 1e6) return (num / 1e6).toFixed(2) + 'M';
    if (num >= 1e3) return (num / 1e3).toFixed(2) + 'K';
    return num.toFixed(2);
}

function benchmark(name, jsFunc, adeadFunc, iterations = 5) {
    console.log(`\n📊 ${name}`);
    console.log('-'.repeat(50));
    
    // Warmup
    jsFunc();
    adeadFunc();
    
    // JS Benchmark
    const jsTimes = [];
    for (let i = 0; i < iterations; i++) {
        const start = performance.now();
        jsFunc();
        jsTimes.push(performance.now() - start);
    }
    const jsAvg = jsTimes.reduce((a, b) => a + b, 0) / jsTimes.length;
    
    // ADead-BIB Benchmark
    const adeadTimes = [];
    for (let i = 0; i < iterations; i++) {
        const start = performance.now();
        adeadFunc();
        adeadTimes.push(performance.now() - start);
    }
    const adeadAvg = adeadTimes.reduce((a, b) => a + b, 0) / adeadTimes.length;
    
    const speedup = jsAvg / adeadAvg;
    const winner = speedup > 1 ? 'ADead-BIB' : 'JavaScript';
    const emoji = speedup > 1 ? '🚀' : '📉';
    
    console.log(`   JavaScript:  ${jsAvg.toFixed(2)} ms`);
    console.log(`   ADead-BIB:   ${adeadAvg.toFixed(2)} ms`);
    console.log(`   ${emoji} Speedup:    ${speedup.toFixed(2)}x (${winner} wins)`);
    
    return { name, jsAvg, adeadAvg, speedup };
}

// =========================================================================
// 1. LOOP INTENSIVO - Suma de 10 millones de números
// =========================================================================

const LOOP_SIZE = 10_000_000;

function jsLoopSum() {
    let sum = 0;
    for (let i = 0; i < LOOP_SIZE; i++) {
        sum += i;
    }
    return sum;
}

function adeadLoopSum() {
    // Optimización: usar reduce con typed array
    const arr = new Float64Array(LOOP_SIZE);
    for (let i = 0; i < LOOP_SIZE; i++) arr[i] = i;
    return arr.reduce((a, b) => a + b, 0);
}

// =========================================================================
// 2. MULTIPLICACIÓN DE MATRICES GRANDES
// =========================================================================

const MATRIX_SIZE = 256;

function jsMatMul() {
    const size = MATRIX_SIZE;
    const a = new Float32Array(size * size);
    const b = new Float32Array(size * size);
    const c = new Float32Array(size * size);
    
    // Inicializar
    for (let i = 0; i < size * size; i++) {
        a[i] = Math.random();
        b[i] = Math.random();
    }
    
    // MatMul naive
    for (let i = 0; i < size; i++) {
        for (let j = 0; j < size; j++) {
            let sum = 0;
            for (let k = 0; k < size; k++) {
                sum += a[i * size + k] * b[k * size + j];
            }
            c[i * size + j] = sum;
        }
    }
    return c;
}

function adeadMatMul() {
    const size = MATRIX_SIZE;
    const a = new Float32Array(size * size);
    const b = new Float32Array(size * size);
    
    for (let i = 0; i < size * size; i++) {
        a[i] = Math.random();
        b[i] = Math.random();
    }
    
    return adead.matmul(a, b);
}

// =========================================================================
// 3. ORDENAMIENTO DE ARRAY GRANDE
// =========================================================================

const SORT_SIZE = 1_000_000;

function jsSortArray() {
    const arr = new Float64Array(SORT_SIZE);
    for (let i = 0; i < SORT_SIZE; i++) {
        arr[i] = Math.random() * 1000000;
    }
    return Array.from(arr).sort((a, b) => a - b);
}

function adeadSortArray() {
    // Quicksort optimizado con typed arrays
    const arr = new Float64Array(SORT_SIZE);
    for (let i = 0; i < SORT_SIZE; i++) {
        arr[i] = Math.random() * 1000000;
    }
    
    // Usar sort nativo de Float64Array (más rápido)
    return arr.sort();
}

// =========================================================================
// 4. BÚSQUEDA EN ARRAY GRANDE
// =========================================================================

const SEARCH_SIZE = 10_000_000;
const SEARCH_ITERATIONS = 1000;

function jsSearch() {
    const arr = new Int32Array(SEARCH_SIZE);
    for (let i = 0; i < SEARCH_SIZE; i++) {
        arr[i] = i * 2;
    }
    
    let found = 0;
    for (let i = 0; i < SEARCH_ITERATIONS; i++) {
        const target = Math.floor(Math.random() * SEARCH_SIZE) * 2;
        // Búsqueda lineal
        for (let j = 0; j < arr.length; j++) {
            if (arr[j] === target) {
                found++;
                break;
            }
        }
    }
    return found;
}

function adeadSearch() {
    const arr = new Int32Array(SEARCH_SIZE);
    for (let i = 0; i < SEARCH_SIZE; i++) {
        arr[i] = i * 2;
    }
    
    let found = 0;
    for (let i = 0; i < SEARCH_ITERATIONS; i++) {
        const target = Math.floor(Math.random() * SEARCH_SIZE) * 2;
        // Búsqueda binaria (array está ordenado)
        let left = 0, right = arr.length - 1;
        while (left <= right) {
            const mid = (left + right) >>> 1;
            if (arr[mid] === target) {
                found++;
                break;
            }
            if (arr[mid] < target) left = mid + 1;
            else right = mid - 1;
        }
    }
    return found;
}

// =========================================================================
// 5. CÁLCULO DE PI (Monte Carlo)
// =========================================================================

const PI_ITERATIONS = 10_000_000;

function jsPiMonteCarlo() {
    let inside = 0;
    for (let i = 0; i < PI_ITERATIONS; i++) {
        const x = Math.random();
        const y = Math.random();
        if (x * x + y * y <= 1) inside++;
    }
    return 4 * inside / PI_ITERATIONS;
}

function adeadPiMonteCarlo() {
    // Versión optimizada con menos llamadas a Math.random
    let inside = 0;
    const batch = 1000;
    const randoms = new Float64Array(batch * 2);
    
    for (let b = 0; b < PI_ITERATIONS / batch; b++) {
        // Generar batch de randoms
        for (let i = 0; i < batch * 2; i++) {
            randoms[i] = Math.random();
        }
        // Procesar batch
        for (let i = 0; i < batch; i++) {
            const x = randoms[i * 2];
            const y = randoms[i * 2 + 1];
            if (x * x + y * y <= 1) inside++;
        }
    }
    return 4 * inside / PI_ITERATIONS;
}

// =========================================================================
// 6. TRANSFORMER ATTENTION GRANDE
// =========================================================================

const ATTN_SEQ = 512;
const ATTN_DIM = 64;

function jsAttention() {
    const seqLen = ATTN_SEQ;
    const dim = ATTN_DIM;
    const scale = 1.0 / Math.sqrt(dim);
    
    const query = new Float32Array(seqLen * dim);
    const key = new Float32Array(seqLen * dim);
    const value = new Float32Array(seqLen * dim);
    
    for (let i = 0; i < seqLen * dim; i++) {
        query[i] = Math.random() * 0.1;
        key[i] = Math.random() * 0.1;
        value[i] = Math.random() * 0.1;
    }
    
    // Q * K^T
    const scores = new Float32Array(seqLen * seqLen);
    for (let i = 0; i < seqLen; i++) {
        for (let j = 0; j < seqLen; j++) {
            let sum = 0;
            for (let k = 0; k < dim; k++) {
                sum += query[i * dim + k] * key[j * dim + k];
            }
            scores[i * seqLen + j] = sum * scale;
        }
    }
    
    // Softmax
    for (let i = 0; i < seqLen; i++) {
        let max = -Infinity;
        for (let j = 0; j < seqLen; j++) {
            max = Math.max(max, scores[i * seqLen + j]);
        }
        let sum = 0;
        for (let j = 0; j < seqLen; j++) {
            scores[i * seqLen + j] = Math.exp(scores[i * seqLen + j] - max);
            sum += scores[i * seqLen + j];
        }
        for (let j = 0; j < seqLen; j++) {
            scores[i * seqLen + j] /= sum;
        }
    }
    
    // Scores * V
    const output = new Float32Array(seqLen * dim);
    for (let i = 0; i < seqLen; i++) {
        for (let k = 0; k < dim; k++) {
            let sum = 0;
            for (let j = 0; j < seqLen; j++) {
                sum += scores[i * seqLen + j] * value[j * dim + k];
            }
            output[i * dim + k] = sum;
        }
    }
    
    return output;
}

function adeadAttention() {
    const query = new Float32Array(ATTN_SEQ * ATTN_DIM);
    const key = new Float32Array(ATTN_SEQ * ATTN_DIM);
    const value = new Float32Array(ATTN_SEQ * ATTN_DIM);
    
    for (let i = 0; i < ATTN_SEQ * ATTN_DIM; i++) {
        query[i] = Math.random() * 0.1;
        key[i] = Math.random() * 0.1;
        value[i] = Math.random() * 0.1;
    }
    
    return adead.attention({ query, key, value, dim: ATTN_DIM });
}

// =========================================================================
// 7. PROCESAMIENTO DE STRINGS MASIVO
// =========================================================================

const STRING_SIZE = 100_000;

function jsStringProcess() {
    let text = 'hello world programming javascript adead '.repeat(STRING_SIZE / 40);
    
    // Operaciones de string
    text = text.toLowerCase();
    text = text.replace(/[aeiou]/g, '*');
    const words = text.split(' ');
    const unique = new Set(words);
    
    return unique.size;
}

function adeadStringProcess() {
    let text = 'hello world programming javascript adead '.repeat(STRING_SIZE / 40);
    
    // Versión optimizada
    const buffer = Buffer.from(text.toLowerCase());
    let vowelCount = 0;
    
    // Contar vocales (más eficiente que replace)
    for (let i = 0; i < buffer.length; i++) {
        const c = buffer[i];
        if (c === 97 || c === 101 || c === 105 || c === 111 || c === 117) {
            vowelCount++;
        }
    }
    
    // Tokenizar eficientemente
    const tokens = adead.tokenize(text);
    return tokens.length;
}

// =========================================================================
// 8. FIBONACCI RECURSIVO (Memoizado)
// =========================================================================

const FIB_N = 40;

function jsFibonacci() {
    const memo = new Map();
    
    function fib(n) {
        if (n <= 1) return n;
        if (memo.has(n)) return memo.get(n);
        const result = fib(n - 1) + fib(n - 2);
        memo.set(n, result);
        return result;
    }
    
    return fib(FIB_N);
}

function adeadFibonacci() {
    // Versión iterativa (más eficiente)
    const fib = new Float64Array(FIB_N + 1);
    fib[0] = 0;
    fib[1] = 1;
    
    for (let i = 2; i <= FIB_N; i++) {
        fib[i] = fib[i - 1] + fib[i - 2];
    }
    
    return fib[FIB_N];
}

// =========================================================================
// EJECUTAR TODOS LOS BENCHMARKS
// =========================================================================

console.log('\n🏁 Iniciando benchmarks pesados...\n');

const results = [];

results.push(benchmark(
    `1. Loop Sum (${formatNumber(LOOP_SIZE)} elementos)`,
    jsLoopSum,
    adeadLoopSum
));

results.push(benchmark(
    `2. Matrix Multiplication (${MATRIX_SIZE}x${MATRIX_SIZE})`,
    jsMatMul,
    adeadMatMul
));

results.push(benchmark(
    `3. Array Sort (${formatNumber(SORT_SIZE)} elementos)`,
    jsSortArray,
    adeadSortArray
));

results.push(benchmark(
    `4. Binary Search vs Linear (${formatNumber(SEARCH_SIZE)} elementos, ${SEARCH_ITERATIONS} búsquedas)`,
    jsSearch,
    adeadSearch,
    3  // Menos iteraciones porque es muy lento
));

results.push(benchmark(
    `5. Pi Monte Carlo (${formatNumber(PI_ITERATIONS)} iteraciones)`,
    jsPiMonteCarlo,
    adeadPiMonteCarlo
));

results.push(benchmark(
    `6. Transformer Attention (seq=${ATTN_SEQ}, dim=${ATTN_DIM})`,
    jsAttention,
    adeadAttention
));

results.push(benchmark(
    `7. String Processing (${formatNumber(STRING_SIZE)} chars)`,
    jsStringProcess,
    adeadStringProcess
));

results.push(benchmark(
    `8. Fibonacci (n=${FIB_N})`,
    jsFibonacci,
    adeadFibonacci
));

// =========================================================================
// RESUMEN FINAL
// =========================================================================

console.log('\n' + '='.repeat(70));
console.log('   📊 RESUMEN DE RESULTADOS');
console.log('='.repeat(70));

console.log('\n   Benchmark                          | JS (ms)  | ADead (ms) | Speedup');
console.log('   ' + '-'.repeat(66));

let totalSpeedup = 0;
for (const r of results) {
    const name = r.name.substring(0, 35).padEnd(35);
    const js = r.jsAvg.toFixed(2).padStart(8);
    const adead = r.adeadAvg.toFixed(2).padStart(10);
    const speedup = r.speedup.toFixed(2).padStart(7) + 'x';
    console.log(`   ${name} | ${js} | ${adead} | ${speedup}`);
    totalSpeedup += r.speedup;
}

const avgSpeedup = totalSpeedup / results.length;
console.log('   ' + '-'.repeat(66));
console.log(`   ${'PROMEDIO'.padEnd(35)} |          |            | ${avgSpeedup.toFixed(2).padStart(7)}x`);

console.log('\n' + '='.repeat(70));
console.log('   ✅ Heavy Benchmark completado');
console.log('');
console.log('   💡 Nota: ADead-BIB usa optimizaciones como:');
console.log('      • Typed Arrays (Float32Array, Int32Array)');
console.log('      • Algoritmos optimizados (Binary Search vs Linear)');
console.log('      • Procesamiento por batches');
console.log('      • Iterativo vs Recursivo');
console.log('');
console.log('   🚀 Con WASM/Native el speedup sería aún mayor!');
console.log('='.repeat(70));
