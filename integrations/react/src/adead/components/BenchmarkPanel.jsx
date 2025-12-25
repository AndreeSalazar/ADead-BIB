/**
 * BenchmarkPanel - Componente para benchmarks visuales
 * =====================================================
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

import React, { useState, useCallback } from 'react';
import { useMatMul } from '../hooks/useMatMul';
import { useAttention } from '../hooks/useAttention';
import { useTokenizer } from '../hooks/useTokenizer';

/**
 * Panel de benchmarks para visualizar rendimiento
 */
export function BenchmarkPanel({ 
    operations = ['matmul', 'attention', 'tokenize'],
    sizes = [64, 128, 256]
}) {
    const [results, setResults] = useState([]);
    const [running, setRunning] = useState(false);
    const [currentOp, setCurrentOp] = useState('');
    
    const { benchmark: benchMatMul } = useMatMul();
    const { benchmark: benchAttention } = useAttention();
    const { benchmark: benchTokenize } = useTokenizer();
    
    const runBenchmarks = useCallback(async () => {
        setRunning(true);
        setResults([]);
        
        const newResults = [];
        
        for (const op of operations) {
            for (const size of sizes) {
                setCurrentOp(`${op} (${size})`);
                
                let result;
                try {
                    switch (op) {
                        case 'matmul':
                            result = await benchMatMul(size, 3);
                            newResults.push({
                                operation: 'MatMul',
                                size: `${size}x${size}`,
                                time: result.avgMs.toFixed(2),
                                metric: `${result.gflops.toFixed(2)} GFLOPS`,
                            });
                            break;
                        case 'attention':
                            result = await benchAttention(size, 64, 3);
                            newResults.push({
                                operation: 'Attention',
                                size: `seq=${size}`,
                                time: result.avgMs.toFixed(2),
                                metric: '-',
                            });
                            break;
                        case 'tokenize':
                            result = await benchTokenize(size * 100, 3);
                            newResults.push({
                                operation: 'Tokenize',
                                size: `${size * 100} chars`,
                                time: result.avgMs.toFixed(2),
                                metric: `${(result.tokensPerSec / 1000).toFixed(1)}K tok/s`,
                            });
                            break;
                        default:
                            break;
                    }
                } catch (err) {
                    console.error(`Error en ${op}:`, err);
                }
                
                setResults([...newResults]);
            }
        }
        
        setRunning(false);
        setCurrentOp('');
    }, [operations, sizes, benchMatMul, benchAttention, benchTokenize]);
    
    return (
        <div style={styles.container}>
            <h2 style={styles.title}>📊 ADead-BIB Benchmark Panel</h2>
            
            <button 
                onClick={runBenchmarks} 
                disabled={running}
                style={{
                    ...styles.button,
                    opacity: running ? 0.6 : 1,
                }}
            >
                {running ? `⏳ ${currentOp}...` : '🚀 Ejecutar Benchmarks'}
            </button>
            
            {results.length > 0 && (
                <table style={styles.table}>
                    <thead>
                        <tr>
                            <th style={styles.th}>Operación</th>
                            <th style={styles.th}>Tamaño</th>
                            <th style={styles.th}>Tiempo (ms)</th>
                            <th style={styles.th}>Rendimiento</th>
                        </tr>
                    </thead>
                    <tbody>
                        {results.map((r, i) => (
                            <tr key={i} style={i % 2 === 0 ? styles.rowEven : styles.rowOdd}>
                                <td style={styles.td}>{r.operation}</td>
                                <td style={styles.td}>{r.size}</td>
                                <td style={styles.td}>{r.time}</td>
                                <td style={styles.td}>{r.metric}</td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            )}
            
            <p style={styles.footer}>
                💡 ADead-BIB: Rendimiento nativo en React
            </p>
        </div>
    );
}

const styles = {
    container: {
        fontFamily: 'system-ui, -apple-system, sans-serif',
        padding: '20px',
        backgroundColor: '#1a1a2e',
        borderRadius: '12px',
        color: '#eee',
        maxWidth: '600px',
    },
    title: {
        margin: '0 0 20px 0',
        fontSize: '1.5rem',
        color: '#00d4ff',
    },
    button: {
        backgroundColor: '#00d4ff',
        color: '#1a1a2e',
        border: 'none',
        padding: '12px 24px',
        borderRadius: '8px',
        fontSize: '1rem',
        fontWeight: 'bold',
        cursor: 'pointer',
        marginBottom: '20px',
    },
    table: {
        width: '100%',
        borderCollapse: 'collapse',
        marginTop: '10px',
    },
    th: {
        backgroundColor: '#16213e',
        padding: '12px',
        textAlign: 'left',
        borderBottom: '2px solid #00d4ff',
    },
    td: {
        padding: '10px 12px',
        borderBottom: '1px solid #333',
    },
    rowEven: {
        backgroundColor: '#1a1a2e',
    },
    rowOdd: {
        backgroundColor: '#16213e',
    },
    footer: {
        marginTop: '20px',
        fontSize: '0.9rem',
        color: '#888',
        textAlign: 'center',
    },
};

export default BenchmarkPanel;
