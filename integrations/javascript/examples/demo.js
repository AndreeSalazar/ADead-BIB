/**
 * Demo de JavaScript + ADead-BIB
 * ===============================
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

const { ADeadBIB, matmul, attention, tokenize } = require('../src/adead-binding');

console.log('='.repeat(60));
console.log('   🔥 JavaScript + ADead-BIB Demo');
console.log('   Rendimiento extremo desde JavaScript');
console.log('='.repeat(60));

// Inicializar
const adead = new ADeadBIB();

// =========================================================================
// 1. Multiplicación de Matrices
// =========================================================================
console.log('\n📊 1. Multiplicación de Matrices');
console.log('-'.repeat(40));

const size = 128;
const a = new Float32Array(size * size);
const b = new Float32Array(size * size);

// Llenar con valores aleatorios
for (let i = 0; i < size * size; i++) {
    a[i] = Math.random();
    b[i] = Math.random();
}

const startMatmul = performance.now();
const resultMatmul = adead.matmul(a, b);
const timeMatmul = performance.now() - startMatmul;

console.log(`   Tamaño: ${size}x${size}`);
console.log(`   Tiempo: ${timeMatmul.toFixed(2)} ms`);
console.log(`   Resultado[0]: ${resultMatmul[0].toFixed(4)}`);

// =========================================================================
// 2. Transformer Attention
// =========================================================================
console.log('\n📊 2. Transformer Attention');
console.log('-'.repeat(40));

const seqLen = 64;
const dim = 64;

const query = new Float32Array(seqLen * dim);
const key = new Float32Array(seqLen * dim);
const value = new Float32Array(seqLen * dim);

for (let i = 0; i < seqLen * dim; i++) {
    query[i] = Math.random() * 0.1;
    key[i] = Math.random() * 0.1;
    value[i] = Math.random() * 0.1;
}

const startAttn = performance.now();
const resultAttn = adead.attention({ query, key, value, heads: 8, dim });
const timeAttn = performance.now() - startAttn;

console.log(`   Secuencia: ${seqLen}`);
console.log(`   Dimensión: ${dim}`);
console.log(`   Tiempo: ${timeAttn.toFixed(2)} ms`);
console.log(`   Output[0]: ${resultAttn[0].toFixed(4)}`);

// =========================================================================
// 3. Tokenización
// =========================================================================
console.log('\n📊 3. Tokenización');
console.log('-'.repeat(40));

const text = `
JavaScript es un lenguaje de programación interpretado, dialecto del estándar 
ECMAScript. Se define como orientado a objetos, basado en prototipos, imperativo, 
débilmente tipado y dinámico. ADead-BIB es un compilador que genera binarios 
ultra-compactos directamente a código máquina sin runtime.
`;

const startToken = performance.now();
const tokens = adead.tokenize(text);
const timeToken = performance.now() - startToken;

console.log(`   Texto: ${text.length} caracteres`);
console.log(`   Tokens: ${tokens.length}`);
console.log(`   Tiempo: ${timeToken.toFixed(2)} ms`);
console.log(`   Primeros 10: [${Array.from(tokens.slice(0, 10)).join(', ')}]`);

// =========================================================================
// 4. Benchmark Comparativo
// =========================================================================
console.log('\n📊 4. Benchmark Comparativo');
console.log('-'.repeat(40));

// MatMul JS puro vs ADead-BIB
function matmulPureJS(a, b, size) {
    const result = new Float32Array(size * size);
    for (let i = 0; i < size; i++) {
        for (let j = 0; j < size; j++) {
            let sum = 0;
            for (let k = 0; k < size; k++) {
                sum += a[i * size + k] * b[k * size + j];
            }
            result[i * size + j] = sum;
        }
    }
    return result;
}

const benchSize = 64;
const benchA = new Float32Array(benchSize * benchSize).fill(1);
const benchB = new Float32Array(benchSize * benchSize).fill(1);

// JS Puro
const startJS = performance.now();
for (let i = 0; i < 10; i++) {
    matmulPureJS(benchA, benchB, benchSize);
}
const timeJS = (performance.now() - startJS) / 10;

// ADead-BIB
const startADead = performance.now();
for (let i = 0; i < 10; i++) {
    adead.matmul(benchA, benchB);
}
const timeADead = (performance.now() - startADead) / 10;

console.log(`   MatMul ${benchSize}x${benchSize}:`);
console.log(`   • JS Puro:    ${timeJS.toFixed(2)} ms`);
console.log(`   • ADead-BIB:  ${timeADead.toFixed(2)} ms`);
console.log(`   • Speedup:    ${(timeJS / timeADead).toFixed(1)}x`);

// =========================================================================
// 5. Estadísticas
// =========================================================================
console.log('\n📊 5. Estadísticas');
console.log('-'.repeat(40));

const stats = adead.getStats();
console.log(`   Compilaciones: ${stats.compilations}`);
console.log(`   Ejecuciones: ${stats.executions}`);
console.log(`   Cache hits: ${stats.cacheHits}`);
console.log(`   Tiempo total: ${stats.totalTimeMs.toFixed(2)} ms`);

// =========================================================================
// Resumen
// =========================================================================
console.log('\n' + '='.repeat(60));
console.log('   ✅ Demo completada');
console.log('   JavaScript + ADead-BIB = Rendimiento Extremo');
console.log('='.repeat(60));
