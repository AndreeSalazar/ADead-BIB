/**
 * Benchmark JavaScript + ADead-BIB
 * ==================================
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

const { ADeadBIB } = require('../src/adead-binding');

console.log('='.repeat(70));
console.log('   📊 Benchmark: JavaScript + ADead-BIB');
console.log('   Comparación de rendimiento');
console.log('='.repeat(70));

const adead = new ADeadBIB();

// =========================================================================
// Benchmark de MatMul
// =========================================================================
console.log('\n🔢 Matrix Multiplication Benchmark');
console.log('-'.repeat(50));

const sizes = [32, 64, 128, 256];

for (const size of sizes) {
    const a = new Float32Array(size * size);
    const b = new Float32Array(size * size);
    
    for (let i = 0; i < size * size; i++) {
        a[i] = Math.random();
        b[i] = Math.random();
    }
    
    // Warmup
    adead.matmul(a, b);
    
    // Benchmark
    const iterations = 5;
    const times = [];
    
    for (let i = 0; i < iterations; i++) {
        const start = performance.now();
        adead.matmul(a, b);
        times.push(performance.now() - start);
    }
    
    const avg = times.reduce((a, b) => a + b, 0) / times.length;
    const gflops = (2 * size * size * size) / (avg * 1e6);
    
    console.log(`   ${size}x${size}: ${avg.toFixed(2)} ms (${gflops.toFixed(2)} GFLOPS)`);
}

// =========================================================================
// Benchmark de Attention
// =========================================================================
console.log('\n🧠 Transformer Attention Benchmark');
console.log('-'.repeat(50));

const seqLengths = [32, 64, 128, 256];
const dim = 64;

for (const seqLen of seqLengths) {
    const query = new Float32Array(seqLen * dim);
    const key = new Float32Array(seqLen * dim);
    const value = new Float32Array(seqLen * dim);
    
    for (let i = 0; i < seqLen * dim; i++) {
        query[i] = Math.random() * 0.1;
        key[i] = Math.random() * 0.1;
        value[i] = Math.random() * 0.1;
    }
    
    // Warmup
    adead.attention({ query, key, value, dim });
    
    // Benchmark
    const iterations = 5;
    const times = [];
    
    for (let i = 0; i < iterations; i++) {
        const start = performance.now();
        adead.attention({ query, key, value, dim });
        times.push(performance.now() - start);
    }
    
    const avg = times.reduce((a, b) => a + b, 0) / times.length;
    console.log(`   seq=${seqLen}, dim=${dim}: ${avg.toFixed(2)} ms`);
}

// =========================================================================
// Benchmark de Tokenización
// =========================================================================
console.log('\n📝 Tokenization Benchmark');
console.log('-'.repeat(50));

const textSizes = [100, 1000, 10000, 50000];

for (const textSize of textSizes) {
    const text = 'hello world programming javascript adead '.repeat(textSize / 40);
    
    // Warmup
    adead.tokenize(text);
    
    // Benchmark
    const iterations = 5;
    const times = [];
    
    for (let i = 0; i < iterations; i++) {
        const start = performance.now();
        const tokens = adead.tokenize(text);
        times.push(performance.now() - start);
    }
    
    const avg = times.reduce((a, b) => a + b, 0) / times.length;
    const tokensPerSec = (textSize / 5) / (avg / 1000);
    
    console.log(`   ${textSize} chars: ${avg.toFixed(2)} ms (${(tokensPerSec / 1000).toFixed(1)}K tok/s)`);
}

// =========================================================================
// Resumen
// =========================================================================
console.log('\n' + '='.repeat(70));
console.log('   ✅ Benchmark completado');
console.log('');
console.log('   💡 Nota: Estos benchmarks usan la implementación JS optimizada.');
console.log('   Con WASM/Native el rendimiento será aún mayor.');
console.log('='.repeat(70));
