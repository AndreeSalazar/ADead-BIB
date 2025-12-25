/**
 * useTokenizer - Hook para tokenización rápida
 * =============================================
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

import { useState, useCallback } from 'react';

/**
 * Hook especializado para tokenización de texto
 * @returns {Object} - API del hook
 */
export function useTokenizer() {
    const [tokens, setTokens] = useState(null);
    const [loading, setLoading] = useState(false);
    const [timeMs, setTimeMs] = useState(null);
    const [error, setError] = useState(null);
    const [stats, setStats] = useState({
        totalTokens: 0,
        uniqueTokens: 0,
        avgTokenLength: 0,
    });
    
    /**
     * Tokeniza texto
     * @param {string} text - Texto a tokenizar
     * @param {Object} opts - Opciones
     * @returns {Promise<Int32Array>} - Tokens
     */
    const tokenize = useCallback(async (text, opts = {}) => {
        const { maxLength = 512, vocab = null } = opts;
        
        setLoading(true);
        setError(null);
        
        const start = performance.now();
        
        try {
            // Preprocesar texto
            const cleaned = text.toLowerCase()
                .replace(/[^\w\s]/g, ' ')
                .replace(/\s+/g, ' ')
                .trim();
            
            const words = cleaned.split(' ').filter(w => w.length > 0);
            
            // Generar tokens
            let tokenArray;
            if (vocab) {
                tokenArray = words.map(w => vocab[w] || 0);
            } else {
                // Hash simple
                tokenArray = words.map(w => {
                    let hash = 5381;
                    for (let i = 0; i < w.length; i++) {
                        hash = ((hash << 5) + hash) + w.charCodeAt(i);
                    }
                    return Math.abs(hash) % 30000;
                });
            }
            
            const result = new Int32Array(tokenArray.slice(0, maxLength));
            
            // Calcular estadísticas
            const uniqueSet = new Set(tokenArray);
            const avgLen = words.reduce((sum, w) => sum + w.length, 0) / words.length;
            
            setStats({
                totalTokens: result.length,
                uniqueTokens: uniqueSet.size,
                avgTokenLength: avgLen,
            });
            
            const end = performance.now();
            setTimeMs(end - start);
            setTokens(result);
            
            return result;
        } catch (err) {
            setError(err.message);
            throw err;
        } finally {
            setLoading(false);
        }
    }, []);
    
    /**
     * Detokeniza (convierte tokens a texto)
     * @param {Int32Array} tokenArray - Tokens
     * @param {Object} reverseVocab - Vocabulario inverso
     * @returns {string} - Texto
     */
    const detokenize = useCallback((tokenArray, reverseVocab) => {
        if (!reverseVocab) {
            return `[${tokenArray.length} tokens]`;
        }
        
        return Array.from(tokenArray)
            .map(t => reverseVocab[t] || '[UNK]')
            .join(' ');
    }, []);
    
    /**
     * Cuenta tokens sin almacenar resultado
     * @param {string} text - Texto
     * @returns {number} - Número de tokens
     */
    const countTokens = useCallback((text) => {
        const words = text.toLowerCase()
            .replace(/[^\w\s]/g, ' ')
            .split(/\s+/)
            .filter(w => w.length > 0);
        return words.length;
    }, []);
    
    /**
     * Benchmark de tokenización
     */
    const benchmark = useCallback(async (textSize, iterations = 5) => {
        const times = [];
        const text = 'hello world programming javascript react '.repeat(textSize / 40);
        
        for (let i = 0; i < iterations; i++) {
            const start = performance.now();
            await tokenize(text);
            times.push(performance.now() - start);
        }
        
        const avg = times.reduce((a, b) => a + b, 0) / times.length;
        const tokensPerSec = (textSize / 5) / (avg / 1000);
        
        return { textSize, avgMs: avg, tokensPerSec, iterations };
    }, [tokenize]);
    
    return {
        tokenize,
        detokenize,
        countTokens,
        benchmark,
        tokens,
        loading,
        timeMs,
        error,
        stats,
    };
}

export default useTokenizer;
