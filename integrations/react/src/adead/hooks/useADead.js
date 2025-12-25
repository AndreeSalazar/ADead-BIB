/**
 * useADead - Hook principal para ADead-BIB en React
 * ==================================================
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

import { useState, useCallback, useRef, useEffect } from 'react';

/**
 * Hook principal para ejecutar operaciones ADead-BIB
 * @param {Object} options - Opciones de configuración
 * @returns {Object} - API del hook
 */
export function useADead(options = {}) {
    const [loading, setLoading] = useState(false);
    const [result, setResult] = useState(null);
    const [error, setError] = useState(null);
    const [progress, setProgress] = useState(0);
    const [stats, setStats] = useState({
        operations: 0,
        totalTimeMs: 0,
        avgTimeMs: 0,
    });
    
    const workerRef = useRef(null);
    const abortControllerRef = useRef(null);
    
    // Inicializar worker
    useEffect(() => {
        // En un entorno real, cargaríamos el Web Worker aquí
        // workerRef.current = new Worker('./adead.worker.js');
        
        return () => {
            if (workerRef.current) {
                workerRef.current.terminate();
            }
        };
    }, []);
    
    /**
     * Ejecuta una operación ADead-BIB
     * @param {string} operation - Nombre de la operación
     * @param {any} data - Datos de entrada
     * @returns {Promise<any>} - Resultado
     */
    const compute = useCallback(async (operation, data) => {
        setLoading(true);
        setError(null);
        setProgress(0);
        
        const startTime = performance.now();
        abortControllerRef.current = new AbortController();
        
        try {
            let computeResult;
            
            switch (operation) {
                case 'matmul':
                    computeResult = await executeMatMul(data);
                    break;
                case 'attention':
                    computeResult = await executeAttention(data);
                    break;
                case 'tokenize':
                    computeResult = await executeTokenize(data);
                    break;
                case 'sort':
                    computeResult = await executeSort(data);
                    break;
                case 'search':
                    computeResult = await executeSearch(data);
                    break;
                default:
                    computeResult = await executeGeneric(operation, data);
            }
            
            const endTime = performance.now();
            const timeMs = endTime - startTime;
            
            // Actualizar estadísticas
            setStats(prev => ({
                operations: prev.operations + 1,
                totalTimeMs: prev.totalTimeMs + timeMs,
                avgTimeMs: (prev.totalTimeMs + timeMs) / (prev.operations + 1),
            }));
            
            setResult(computeResult);
            setProgress(100);
            
            return computeResult;
        } catch (err) {
            setError(err.message);
            throw err;
        } finally {
            setLoading(false);
        }
    }, []);
    
    /**
     * Cancela la operación actual
     */
    const cancel = useCallback(() => {
        if (abortControllerRef.current) {
            abortControllerRef.current.abort();
        }
        setLoading(false);
        setProgress(0);
    }, []);
    
    /**
     * Limpia el resultado y error
     */
    const reset = useCallback(() => {
        setResult(null);
        setError(null);
        setProgress(0);
    }, []);
    
    return {
        compute,
        loading,
        result,
        error,
        progress,
        cancel,
        reset,
        stats,
    };
}

// =========================================================================
// Funciones de ejecución internas
// =========================================================================

async function executeMatMul(data) {
    const { a, b, size } = data;
    const result = new Float32Array(size * size);
    
    // Multiplicación de matrices optimizada
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

async function executeAttention(data) {
    const { query, key, value, dim = 64 } = data;
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

async function executeTokenize(data) {
    const { text, maxLength = 512 } = data;
    
    const words = text.toLowerCase()
        .replace(/[^\w\s]/g, ' ')
        .split(/\s+/)
        .filter(w => w.length > 0);
    
    // Hash simple para tokens
    const tokens = words.map(w => {
        let hash = 5381;
        for (let i = 0; i < w.length; i++) {
            hash = ((hash << 5) + hash) + w.charCodeAt(i);
        }
        return Math.abs(hash) % 30000;
    });
    
    return new Int32Array(tokens.slice(0, maxLength));
}

async function executeSort(data) {
    const { array } = data;
    return Float64Array.from(array).sort();
}

async function executeSearch(data) {
    const { array, target } = data;
    
    // Búsqueda binaria
    let left = 0, right = array.length - 1;
    while (left <= right) {
        const mid = (left + right) >>> 1;
        if (array[mid] === target) return mid;
        if (array[mid] < target) left = mid + 1;
        else right = mid - 1;
    }
    return -1;
}

async function executeGeneric(operation, data) {
    console.log(`Executing generic operation: ${operation}`);
    return data;
}

export default useADead;
