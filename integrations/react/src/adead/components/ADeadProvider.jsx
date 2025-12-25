/**
 * ADeadProvider - Context Provider para ADead-BIB
 * ================================================
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

import React, { createContext, useContext, useState, useEffect, useCallback, useRef } from 'react';

// Contexto
const ADeadContext = createContext(null);

// Configuración por defecto
const DEFAULT_CONFIG = {
    workers: 4,
    useGPU: false,
    wasmPath: '/wasm/adead.wasm',
    cacheResults: true,
    maxCacheSize: 100,
};

/**
 * Provider que envuelve la aplicación y provee acceso a ADead-BIB
 */
export function ADeadProvider({ children, config = {} }) {
    const mergedConfig = { ...DEFAULT_CONFIG, ...config };
    
    const [ready, setReady] = useState(false);
    const [gpuAvailable, setGpuAvailable] = useState(false);
    const [stats, setStats] = useState({
        operations: 0,
        totalTimeMs: 0,
        cacheHits: 0,
    });
    
    const workersRef = useRef([]);
    const cacheRef = useRef(new Map());
    
    // Inicializar
    useEffect(() => {
        const init = async () => {
            // Detectar GPU (WebGPU)
            if (mergedConfig.useGPU && navigator.gpu) {
                try {
                    const adapter = await navigator.gpu.requestAdapter();
                    if (adapter) {
                        setGpuAvailable(true);
                        console.log('🚀 WebGPU disponible');
                    }
                } catch (e) {
                    console.log('⚠️ WebGPU no disponible');
                }
            }
            
            // Inicializar pool de workers (simulado)
            // En producción, aquí cargaríamos los Web Workers reales
            console.log(`🔧 Inicializando ${mergedConfig.workers} workers`);
            
            setReady(true);
            console.log('✅ ADead-BIB Provider listo');
        };
        
        init();
        
        return () => {
            // Cleanup workers
            workersRef.current.forEach(w => w.terminate?.());
        };
    }, [mergedConfig.workers, mergedConfig.useGPU]);
    
    // Ejecutar operación
    const execute = useCallback(async (operation, data) => {
        const start = performance.now();
        
        // Verificar cache
        const cacheKey = `${operation}-${JSON.stringify(data).slice(0, 100)}`;
        if (mergedConfig.cacheResults && cacheRef.current.has(cacheKey)) {
            setStats(prev => ({ ...prev, cacheHits: prev.cacheHits + 1 }));
            return cacheRef.current.get(cacheKey);
        }
        
        // Ejecutar operación
        let result;
        switch (operation) {
            case 'matmul':
                result = executeMatMul(data);
                break;
            case 'attention':
                result = executeAttention(data);
                break;
            case 'tokenize':
                result = executeTokenize(data);
                break;
            default:
                result = data;
        }
        
        const timeMs = performance.now() - start;
        
        // Actualizar stats
        setStats(prev => ({
            operations: prev.operations + 1,
            totalTimeMs: prev.totalTimeMs + timeMs,
            cacheHits: prev.cacheHits,
        }));
        
        // Cachear resultado
        if (mergedConfig.cacheResults) {
            if (cacheRef.current.size >= mergedConfig.maxCacheSize) {
                const firstKey = cacheRef.current.keys().next().value;
                cacheRef.current.delete(firstKey);
            }
            cacheRef.current.set(cacheKey, result);
        }
        
        return result;
    }, [mergedConfig.cacheResults, mergedConfig.maxCacheSize]);
    
    // Limpiar cache
    const clearCache = useCallback(() => {
        cacheRef.current.clear();
    }, []);
    
    // Valor del contexto
    const value = {
        ready,
        gpuAvailable,
        config: mergedConfig,
        stats,
        execute,
        clearCache,
    };
    
    return (
        <ADeadContext.Provider value={value}>
            {children}
        </ADeadContext.Provider>
    );
}

/**
 * Hook para acceder al contexto de ADead-BIB
 */
export function useADeadContext() {
    const context = useContext(ADeadContext);
    if (!context) {
        throw new Error('useADeadContext debe usarse dentro de ADeadProvider');
    }
    return context;
}

// =========================================================================
// Funciones de ejecución internas
// =========================================================================

function executeMatMul(data) {
    const { a, b, size } = data;
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

function executeAttention(data) {
    const { query, key, value, dim = 64 } = data;
    const seqLen = query.length / dim;
    const scale = 1.0 / Math.sqrt(dim);
    
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

function executeTokenize(data) {
    const { text, maxLength = 512 } = data;
    
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

export default ADeadProvider;
