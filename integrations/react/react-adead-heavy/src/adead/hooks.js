/**
 * ADead-BIB React Hooks
 * ======================
 * Hooks para integrar ADead-BIB con React
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

import { useState, useCallback, useRef } from 'react';
import { adead } from './core';

/**
 * Hook principal para operaciones ADead-BIB
 */
export function useADead() {
    const [loading, setLoading] = useState(false);
    const [result, setResult] = useState(null);
    const [error, setError] = useState(null);
    const [timeMs, setTimeMs] = useState(null);

    const execute = useCallback(async (operation, ...args) => {
        setLoading(true);
        setError(null);

        const start = performance.now();

        try {
            let res;
            switch (operation) {
                case 'matmul':
                    res = adead.matmul(...args);
                    break;
                case 'sort':
                    res = adead.sort(...args);
                    break;
                case 'search':
                    res = adead.binarySearch(...args);
                    break;
                case 'filter':
                    res = adead.filterData(...args);
                    break;
                case 'aggregate':
                    res = adead.aggregate(...args);
                    break;
                case 'statistics':
                    res = adead.statistics(...args);
                    break;
                case 'attention':
                    res = adead.attention(...args);
                    break;
                case 'tokenize':
                    res = adead.tokenize(...args);
                    break;
                case 'generate':
                    res = adead.generateTestData(...args);
                    break;
                default:
                    throw new Error(`Unknown operation: ${operation}`);
            }

            const time = performance.now() - start;
            setTimeMs(time);
            setResult(res);
            return { data: res, timeMs: time };
        } catch (err) {
            setError(err.message);
            throw err;
        } finally {
            setLoading(false);
        }
    }, []);

    return { execute, loading, result, error, timeMs };
}

/**
 * Hook para datos pesados con paginación virtual
 */
export function useHeavyData(initialCount = 0) {
    const [data, setData] = useState([]);
    const [loading, setLoading] = useState(false);
    const [stats, setStats] = useState(null);
    const [generationTime, setGenerationTime] = useState(null);

    const generate = useCallback((count, type = 'sales') => {
        setLoading(true);
        const start = performance.now();

        // Usar setTimeout para no bloquear UI
        setTimeout(() => {
            const newData = adead.generateTestData(count, type);
            const time = performance.now() - start;

            setData(newData);
            setGenerationTime(time);
            setLoading(false);

            // Calcular estadísticas si es numérico
            if (type === 'numeric') {
                const s = adead.statistics(newData);
                setStats(s);
            }
        }, 0);
    }, []);

    const filter = useCallback((predicate) => {
        const start = performance.now();
        const filtered = adead.filterData(data, predicate);
        return {
            data: filtered,
            timeMs: performance.now() - start,
            count: filtered.length,
        };
    }, [data]);

    const aggregate = useCallback((groupKey, valueKey) => {
        const start = performance.now();
        const aggregated = adead.aggregate(data, groupKey, valueKey);
        return {
            data: aggregated,
            timeMs: performance.now() - start,
        };
    }, [data]);

    const sort = useCallback((key, ascending = true) => {
        const start = performance.now();
        const sorted = [...data].sort((a, b) => {
            const va = a[key];
            const vb = b[key];
            return ascending ? va - vb : vb - va;
        });
        return {
            data: sorted,
            timeMs: performance.now() - start,
        };
    }, [data]);

    return {
        data,
        loading,
        stats,
        generationTime,
        generate,
        filter,
        aggregate,
        sort,
        count: data.length,
    };
}

/**
 * Hook para benchmarks
 */
export function useBenchmark() {
    const [results, setResults] = useState([]);
    const [running, setRunning] = useState(false);
    const [currentTest, setCurrentTest] = useState('');

    const runBenchmark = useCallback(async (tests) => {
        setRunning(true);
        setResults([]);

        const newResults = [];

        for (const test of tests) {
            setCurrentTest(test.name);

            // Warmup
            test.fn();

            // Benchmark
            const times = [];
            for (let i = 0; i < (test.iterations || 5); i++) {
                const start = performance.now();
                test.fn();
                times.push(performance.now() - start);
            }

            const avg = times.reduce((a, b) => a + b, 0) / times.length;
            const min = Math.min(...times);
            const max = Math.max(...times);

            newResults.push({
                name: test.name,
                avgMs: avg,
                minMs: min,
                maxMs: max,
                iterations: test.iterations || 5,
            });

            setResults([...newResults]);

            // Pequeña pausa para actualizar UI
            await new Promise(r => setTimeout(r, 10));
        }

        setRunning(false);
        setCurrentTest('');

        return newResults;
    }, []);

    return { results, running, currentTest, runBenchmark };
}

export default { useADead, useHeavyData, useBenchmark };
