/**
 * useMatMul - Hook para multiplicación de matrices
 * =================================================
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

import { useState, useCallback } from 'react';

/**
 * Hook especializado para multiplicación de matrices
 * @returns {Object} - API del hook
 */
export function useMatMul() {
    const [result, setResult] = useState(null);
    const [loading, setLoading] = useState(false);
    const [timeMs, setTimeMs] = useState(null);
    const [error, setError] = useState(null);
    
    /**
     * Multiplica dos matrices
     * @param {Float32Array} a - Matriz A
     * @param {Float32Array} b - Matriz B
     * @param {number} size - Tamaño de las matrices (asume cuadradas)
     * @returns {Promise<Float32Array>} - Resultado
     */
    const multiply = useCallback(async (a, b, size) => {
        setLoading(true);
        setError(null);
        
        const start = performance.now();
        
        try {
            const output = new Float32Array(size * size);
            
            // Multiplicación optimizada con blocking para mejor cache
            const blockSize = 32;
            
            for (let ii = 0; ii < size; ii += blockSize) {
                for (let jj = 0; jj < size; jj += blockSize) {
                    for (let kk = 0; kk < size; kk += blockSize) {
                        const iMax = Math.min(ii + blockSize, size);
                        const jMax = Math.min(jj + blockSize, size);
                        const kMax = Math.min(kk + blockSize, size);
                        
                        for (let i = ii; i < iMax; i++) {
                            for (let j = jj; j < jMax; j++) {
                                let sum = output[i * size + j];
                                for (let k = kk; k < kMax; k++) {
                                    sum += a[i * size + k] * b[k * size + j];
                                }
                                output[i * size + j] = sum;
                            }
                        }
                    }
                }
            }
            
            const end = performance.now();
            setTimeMs(end - start);
            setResult(output);
            
            return output;
        } catch (err) {
            setError(err.message);
            throw err;
        } finally {
            setLoading(false);
        }
    }, []);
    
    /**
     * Multiplica matrices con transposición de B (más eficiente)
     */
    const multiplyTransposed = useCallback(async (a, bT, size) => {
        setLoading(true);
        const start = performance.now();
        
        try {
            const output = new Float32Array(size * size);
            
            for (let i = 0; i < size; i++) {
                for (let j = 0; j < size; j++) {
                    let sum = 0;
                    for (let k = 0; k < size; k++) {
                        sum += a[i * size + k] * bT[j * size + k];
                    }
                    output[i * size + j] = sum;
                }
            }
            
            const end = performance.now();
            setTimeMs(end - start);
            setResult(output);
            
            return output;
        } finally {
            setLoading(false);
        }
    }, []);
    
    /**
     * Benchmark de MatMul
     */
    const benchmark = useCallback(async (size, iterations = 5) => {
        const times = [];
        
        for (let i = 0; i < iterations; i++) {
            const a = new Float32Array(size * size);
            const b = new Float32Array(size * size);
            
            for (let j = 0; j < size * size; j++) {
                a[j] = Math.random();
                b[j] = Math.random();
            }
            
            const start = performance.now();
            await multiply(a, b, size);
            times.push(performance.now() - start);
        }
        
        const avg = times.reduce((a, b) => a + b, 0) / times.length;
        const gflops = (2 * size * size * size) / (avg * 1e6);
        
        return { size, avgMs: avg, gflops, iterations };
    }, [multiply]);
    
    return {
        multiply,
        multiplyTransposed,
        benchmark,
        result,
        loading,
        timeMs,
        error,
    };
}

export default useMatMul;
