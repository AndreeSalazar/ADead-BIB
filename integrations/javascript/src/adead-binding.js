/**
 * ADead-BIB JavaScript Binding
 * ============================
 * Author: Eddi Andreé Salazar Matos
 * Email: eddi.salazar.dev@gmail.com
 * Made with ❤️ in Peru 🇵🇪
 * 
 * Integración de JavaScript/Node.js con ADead-BIB
 * para rendimiento extremo en cómputo intensivo.
 */

const { spawn, execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

/**
 * Configuración por defecto
 */
const DEFAULT_CONFIG = {
    useGPU: true,
    cacheCompiled: true,
    compilerPath: path.join(__dirname, '..', '..', 'target', 'release', 'adeadc.exe'),
    tempDir: path.join(__dirname, '..', 'temp'),
    wasmPath: path.join(__dirname, '..', 'lib'),
};

/**
 * Clase principal para interactuar con ADead-BIB
 */
class ADeadBIB {
    constructor(config = {}) {
        this.config = { ...DEFAULT_CONFIG, ...config };
        this.cache = new Map();
        this.stats = {
            compilations: 0,
            executions: 0,
            cacheHits: 0,
            totalTimeMs: 0,
        };
        
        // Crear directorio temporal si no existe
        if (!fs.existsSync(this.config.tempDir)) {
            fs.mkdirSync(this.config.tempDir, { recursive: true });
        }
        
        console.log('🔥 ADead-BIB JS Binding initialized');
    }
    
    /**
     * Compila código ADead-BIB a binario
     * @param {string} code - Código ADead-BIB
     * @returns {Buffer} - Binario compilado
     */
    compile(code) {
        const start = performance.now();
        
        // Verificar cache
        const cacheKey = this._hash(code);
        if (this.config.cacheCompiled && this.cache.has(cacheKey)) {
            this.stats.cacheHits++;
            return this.cache.get(cacheKey);
        }
        
        // Escribir código temporal
        const tempFile = path.join(this.config.tempDir, `temp_${Date.now()}.adB`);
        const outFile = path.join(this.config.tempDir, `out_${Date.now()}.exe`);
        
        fs.writeFileSync(tempFile, code);
        
        try {
            // Compilar con ADead-BIB
            execSync(`"${this.config.compilerPath}" build "${tempFile}" -o "${outFile}"`, {
                stdio: 'pipe',
            });
            
            // Leer binario
            const binary = fs.readFileSync(outFile);
            
            // Cachear
            if (this.config.cacheCompiled) {
                this.cache.set(cacheKey, binary);
            }
            
            this.stats.compilations++;
            this.stats.totalTimeMs += performance.now() - start;
            
            return binary;
        } finally {
            // Limpiar archivos temporales
            try {
                fs.unlinkSync(tempFile);
                if (fs.existsSync(outFile)) fs.unlinkSync(outFile);
            } catch (e) {}
        }
    }
    
    /**
     * Ejecuta un binario compilado
     * @param {Buffer|string} binary - Binario o código a ejecutar
     * @param {Array} args - Argumentos
     * @returns {any} - Resultado
     */
    execute(binary, args = []) {
        const start = performance.now();
        
        // Si es código, compilar primero
        if (typeof binary === 'string') {
            binary = this.compile(binary);
        }
        
        // Escribir binario temporal
        const tempExe = path.join(this.config.tempDir, `exec_${Date.now()}.exe`);
        fs.writeFileSync(tempExe, binary);
        
        try {
            // Ejecutar
            const result = execSync(`"${tempExe}" ${args.join(' ')}`, {
                encoding: 'utf8',
                stdio: 'pipe',
            });
            
            this.stats.executions++;
            this.stats.totalTimeMs += performance.now() - start;
            
            return this._parseResult(result);
        } finally {
            try {
                fs.unlinkSync(tempExe);
            } catch (e) {}
        }
    }
    
    /**
     * Multiplicación de matrices optimizada
     * @param {Float32Array|Array} a - Matriz A
     * @param {Float32Array|Array} b - Matriz B
     * @returns {Float32Array} - Resultado
     */
    matmul(a, b) {
        const sizeA = Math.sqrt(a.length);
        const sizeB = Math.sqrt(b.length);
        
        if (sizeA !== sizeB) {
            throw new Error('Matrices must be square and same size');
        }
        
        const size = sizeA;
        
        // Para matrices pequeñas, usar JS
        if (size < 64) {
            return this._matmulJS(a, b, size);
        }
        
        // Para matrices grandes, usar ADead-BIB
        const code = `
def main():
    # Matrix multiplication ${size}x${size}
    result = matmul_gpu(${size})
    print(result)
`;
        
        // Por ahora, usar implementación JS optimizada
        // TODO: Integrar con WASM/Native cuando esté listo
        return this._matmulJS(a, b, size);
    }
    
    /**
     * Transformer Attention
     * @param {Object} opts - Opciones {query, key, value, heads, dim}
     * @returns {Float32Array} - Output
     */
    attention(opts) {
        const { query, key, value, heads = 8, dim = 64 } = opts;
        
        // Implementación simplificada de attention
        const seqLen = query.length / dim;
        const scale = 1.0 / Math.sqrt(dim);
        
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
    
    /**
     * Tokenización rápida
     * @param {string} text - Texto a tokenizar
     * @param {Object} opts - Opciones
     * @returns {Int32Array} - Tokens
     */
    tokenize(text, opts = {}) {
        const { vocab = null, maxLength = 512 } = opts;
        
        // Tokenización simple por palabras
        const words = text.toLowerCase()
            .replace(/[^\w\s]/g, ' ')
            .split(/\s+/)
            .filter(w => w.length > 0);
        
        // Si hay vocabulario, mapear
        if (vocab) {
            const tokens = words.map(w => vocab[w] || 0);
            return new Int32Array(tokens.slice(0, maxLength));
        }
        
        // Sin vocabulario, usar hash simple
        const tokens = words.map(w => this._simpleHash(w) % 30000);
        return new Int32Array(tokens.slice(0, maxLength));
    }
    
    /**
     * Benchmark de una operación
     * @param {string} operation - Nombre de operación
     * @param {number} size - Tamaño
     * @returns {Object} - Resultados
     */
    benchmark(operation, size = 1024) {
        const iterations = 10;
        const times = [];
        
        console.log(`\n📊 Benchmark: ${operation} (size=${size})`);
        console.log('-'.repeat(40));
        
        for (let i = 0; i < iterations; i++) {
            const start = performance.now();
            
            switch (operation) {
                case 'matmul':
                    const a = new Float32Array(size * size).fill(1);
                    const b = new Float32Array(size * size).fill(1);
                    this.matmul(a, b);
                    break;
                    
                case 'attention':
                    const q = new Float32Array(size * 64).fill(0.1);
                    const k = new Float32Array(size * 64).fill(0.1);
                    const v = new Float32Array(size * 64).fill(0.1);
                    this.attention({ query: q, key: k, value: v });
                    break;
                    
                case 'tokenize':
                    const text = 'hello world '.repeat(size);
                    this.tokenize(text);
                    break;
                    
                default:
                    throw new Error(`Unknown operation: ${operation}`);
            }
            
            times.push(performance.now() - start);
        }
        
        const avg = times.reduce((a, b) => a + b, 0) / times.length;
        const min = Math.min(...times);
        const max = Math.max(...times);
        
        console.log(`   Avg: ${avg.toFixed(2)} ms`);
        console.log(`   Min: ${min.toFixed(2)} ms`);
        console.log(`   Max: ${max.toFixed(2)} ms`);
        
        return { operation, size, avg, min, max, iterations };
    }
    
    /**
     * Obtiene estadísticas
     * @returns {Object}
     */
    getStats() {
        return {
            ...this.stats,
            cacheSize: this.cache.size,
        };
    }
    
    // =========================================================================
    // Métodos privados
    // =========================================================================
    
    _hash(str) {
        let hash = 0;
        for (let i = 0; i < str.length; i++) {
            const char = str.charCodeAt(i);
            hash = ((hash << 5) - hash) + char;
            hash = hash & hash;
        }
        return hash.toString(16);
    }
    
    _simpleHash(str) {
        let hash = 5381;
        for (let i = 0; i < str.length; i++) {
            hash = ((hash << 5) + hash) + str.charCodeAt(i);
        }
        return Math.abs(hash);
    }
    
    _parseResult(output) {
        const trimmed = output.trim();
        
        // Intentar parsear como número
        const num = parseFloat(trimmed);
        if (!isNaN(num)) return num;
        
        // Intentar parsear como JSON
        try {
            return JSON.parse(trimmed);
        } catch (e) {}
        
        // Retornar como string
        return trimmed;
    }
    
    _matmulJS(a, b, size) {
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
}

/**
 * Funciones de utilidad exportadas
 */
function matmul(a, b) {
    const adead = new ADeadBIB();
    return adead.matmul(a, b);
}

function attention(opts) {
    const adead = new ADeadBIB();
    return adead.attention(opts);
}

function tokenize(text, opts) {
    const adead = new ADeadBIB();
    return adead.tokenize(text, opts);
}

// Exportar
module.exports = {
    ADeadBIB,
    matmul,
    attention,
    tokenize,
};
