/**
 * ADead-BIB Web Worker
 * =====================
 * Worker para ejecutar operaciones pesadas sin bloquear el main thread
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

// Estado del worker
let wasmModule = null;
let isReady = false;

// Inicializar
self.onmessage = async function(e) {
    const { type, id, operation, data } = e.data;
    
    switch (type) {
        case 'init':
            await initialize(e.data.wasmPath);
            self.postMessage({ type: 'ready', id });
            break;
            
        case 'execute':
            try {
                const result = await executeOperation(operation, data);
                self.postMessage({ type: 'result', id, result });
            } catch (error) {
                self.postMessage({ type: 'error', id, error: error.message });
            }
            break;
            
        case 'cancel':
            // Cancelar operación actual (si es posible)
            break;
            
        default:
            self.postMessage({ type: 'error', id, error: `Unknown type: ${type}` });
    }
};

async function initialize(wasmPath) {
    try {
        // En producción, cargaríamos el módulo WASM aquí
        // const response = await fetch(wasmPath);
        // const buffer = await response.arrayBuffer();
        // wasmModule = await WebAssembly.instantiate(buffer);
        
        isReady = true;
        console.log('Worker inicializado');
    } catch (error) {
        console.error('Error inicializando worker:', error);
        throw error;
    }
}

async function executeOperation(operation, data) {
    const start = performance.now();
    let result;
    
    switch (operation) {
        case 'matmul':
            result = matmul(data.a, data.b, data.size);
            break;
            
        case 'attention':
            result = attention(data.query, data.key, data.value, data.dim);
            break;
            
        case 'tokenize':
            result = tokenize(data.text, data.maxLength);
            break;
            
        case 'sort':
            result = sortArray(data.array);
            break;
            
        case 'search':
            result = binarySearch(data.array, data.target);
            break;
            
        default:
            throw new Error(`Unknown operation: ${operation}`);
    }
    
    const timeMs = performance.now() - start;
    
    return { data: result, timeMs };
}

// =========================================================================
// Operaciones
// =========================================================================

function matmul(a, b, size) {
    const result = new Float32Array(size * size);
    
    // Multiplicación con blocking para mejor cache
    const blockSize = 32;
    
    for (let ii = 0; ii < size; ii += blockSize) {
        for (let jj = 0; jj < size; jj += blockSize) {
            for (let kk = 0; kk < size; kk += blockSize) {
                const iMax = Math.min(ii + blockSize, size);
                const jMax = Math.min(jj + blockSize, size);
                const kMax = Math.min(kk + blockSize, size);
                
                for (let i = ii; i < iMax; i++) {
                    for (let j = jj; j < jMax; j++) {
                        let sum = result[i * size + j];
                        for (let k = kk; k < kMax; k++) {
                            sum += a[i * size + k] * b[k * size + j];
                        }
                        result[i * size + j] = sum;
                    }
                }
            }
        }
    }
    
    return result;
}

function attention(query, key, value, dim) {
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

function tokenize(text, maxLength = 512) {
    const words = text.toLowerCase()
        .replace(/[^\w\s]/g, ' ')
        .split(/\s+/)
        .filter(w => w.length > 0);
    
    const tokens = words.map(w => {
        let hash = 5381;
        for (let i = 0; i < w.length; i++) {
            hash = ((hash << 5) + hash) + w.charCodeAt(i);
        }
        return Math.abs(hash) % 30000;
    });
    
    return new Int32Array(tokens.slice(0, maxLength));
}

function sortArray(array) {
    return Float64Array.from(array).sort();
}

function binarySearch(array, target) {
    let left = 0, right = array.length - 1;
    while (left <= right) {
        const mid = (left + right) >>> 1;
        if (array[mid] === target) return mid;
        if (array[mid] < target) left = mid + 1;
        else right = mid - 1;
    }
    return -1;
}
