/**
 * useAttention - Hook para Transformer Attention
 * ===============================================
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

import { useState, useCallback } from 'react';

/**
 * Hook especializado para operaciones de Attention
 * @returns {Object} - API del hook
 */
export function useAttention() {
    const [result, setResult] = useState(null);
    const [loading, setLoading] = useState(false);
    const [timeMs, setTimeMs] = useState(null);
    const [error, setError] = useState(null);
    
    /**
     * Ejecuta scaled dot-product attention
     * @param {Object} opts - Opciones {query, key, value, heads, dim}
     * @returns {Promise<Float32Array>} - Output
     */
    const attention = useCallback(async (opts) => {
        const { query, key, value, heads = 1, dim = 64 } = opts;
        
        setLoading(true);
        setError(null);
        
        const start = performance.now();
        
        try {
            const seqLen = query.length / dim;
            const scale = 1.0 / Math.sqrt(dim / heads);
            const headDim = dim / heads;
            
            const output = new Float32Array(seqLen * dim);
            
            // Multi-head attention
            for (let h = 0; h < heads; h++) {
                const headOffset = h * headDim;
                
                // Scores para esta cabeza
                const scores = new Float32Array(seqLen * seqLen);
                
                // Q * K^T
                for (let i = 0; i < seqLen; i++) {
                    for (let j = 0; j < seqLen; j++) {
                        let sum = 0;
                        for (let k = 0; k < headDim; k++) {
                            const qIdx = i * dim + headOffset + k;
                            const kIdx = j * dim + headOffset + k;
                            sum += query[qIdx] * key[kIdx];
                        }
                        scores[i * seqLen + j] = sum * scale;
                    }
                }
                
                // Softmax por fila
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
                for (let i = 0; i < seqLen; i++) {
                    for (let k = 0; k < headDim; k++) {
                        let sum = 0;
                        for (let j = 0; j < seqLen; j++) {
                            const vIdx = j * dim + headOffset + k;
                            sum += scores[i * seqLen + j] * value[vIdx];
                        }
                        output[i * dim + headOffset + k] = sum;
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
     * Self-attention (Q=K=V)
     */
    const selfAttention = useCallback(async (input, heads = 8, dim = 64) => {
        return attention({ query: input, key: input, value: input, heads, dim });
    }, [attention]);
    
    /**
     * Benchmark de attention
     */
    const benchmark = useCallback(async (seqLen, dim = 64, iterations = 5) => {
        const times = [];
        
        for (let i = 0; i < iterations; i++) {
            const query = new Float32Array(seqLen * dim);
            const key = new Float32Array(seqLen * dim);
            const value = new Float32Array(seqLen * dim);
            
            for (let j = 0; j < seqLen * dim; j++) {
                query[j] = Math.random() * 0.1;
                key[j] = Math.random() * 0.1;
                value[j] = Math.random() * 0.1;
            }
            
            const start = performance.now();
            await attention({ query, key, value, dim });
            times.push(performance.now() - start);
        }
        
        const avg = times.reduce((a, b) => a + b, 0) / times.length;
        
        return { seqLen, dim, avgMs: avg, iterations };
    }, [attention]);
    
    return {
        attention,
        selfAttention,
        benchmark,
        result,
        loading,
        timeMs,
        error,
    };
}

export default useAttention;
