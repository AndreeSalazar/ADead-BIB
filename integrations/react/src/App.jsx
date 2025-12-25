/**
 * App de Ejemplo: React + ADead-BIB
 * ==================================
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

import React, { useState, useCallback } from 'react';
import { ADeadProvider, useADeadContext } from './adead/components/ADeadProvider';
import { BenchmarkPanel } from './adead/components/BenchmarkPanel';
import { MatrixVisualizer } from './adead/components/MatrixVisualizer';
import { useMatMul } from './adead/hooks/useMatMul';
import { useAttention } from './adead/hooks/useAttention';
import { useTokenizer } from './adead/hooks/useTokenizer';
import { createRandomMatrix, formatTime, calculateGFLOPS } from './adead/utils/helpers';

function AppContent() {
    const { ready, gpuAvailable, stats } = useADeadContext();
    const [activeTab, setActiveTab] = useState('matmul');
    
    return (
        <div style={styles.app}>
            <header style={styles.header}>
                <h1 style={styles.title}>⚛️ React + ADead-BIB</h1>
                <p style={styles.subtitle}>Rendimiento nativo en aplicaciones React</p>
                <div style={styles.status}>
                    <span style={styles.badge}>
                        {ready ? '✅ Listo' : '⏳ Cargando...'}
                    </span>
                    <span style={styles.badge}>
                        {gpuAvailable ? '🚀 GPU' : '💻 CPU'}
                    </span>
                    <span style={styles.badge}>
                        📊 {stats.operations} ops
                    </span>
                </div>
            </header>
            
            <nav style={styles.nav}>
                {['matmul', 'attention', 'tokenizer', 'benchmark'].map(tab => (
                    <button
                        key={tab}
                        onClick={() => setActiveTab(tab)}
                        style={{
                            ...styles.navButton,
                            backgroundColor: activeTab === tab ? '#00d4ff' : 'transparent',
                            color: activeTab === tab ? '#1a1a2e' : '#fff',
                        }}
                    >
                        {tab === 'matmul' && '🔢 MatMul'}
                        {tab === 'attention' && '🧠 Attention'}
                        {tab === 'tokenizer' && '📝 Tokenizer'}
                        {tab === 'benchmark' && '📊 Benchmark'}
                    </button>
                ))}
            </nav>
            
            <main style={styles.main}>
                {activeTab === 'matmul' && <MatMulDemo />}
                {activeTab === 'attention' && <AttentionDemo />}
                {activeTab === 'tokenizer' && <TokenizerDemo />}
                {activeTab === 'benchmark' && <BenchmarkPanel />}
            </main>
            
            <footer style={styles.footer}>
                <p>Made with ❤️ in Peru 🇵🇪 | ADead-BIB © 2024</p>
            </footer>
        </div>
    );
}

function MatMulDemo() {
    const { multiply, result, loading, timeMs } = useMatMul();
    const [size, setSize] = useState(64);
    
    const handleMultiply = useCallback(async () => {
        const a = createRandomMatrix(size, size);
        const b = createRandomMatrix(size, size);
        await multiply(a, b, size);
    }, [multiply, size]);
    
    return (
        <div style={styles.demo}>
            <h2>🔢 Multiplicación de Matrices</h2>
            
            <div style={styles.controls}>
                <label>
                    Tamaño: 
                    <select 
                        value={size} 
                        onChange={e => setSize(Number(e.target.value))}
                        style={styles.select}
                    >
                        <option value={32}>32x32</option>
                        <option value={64}>64x64</option>
                        <option value={128}>128x128</option>
                        <option value={256}>256x256</option>
                    </select>
                </label>
                
                <button 
                    onClick={handleMultiply} 
                    disabled={loading}
                    style={styles.button}
                >
                    {loading ? '⏳ Calculando...' : '🚀 Multiplicar'}
                </button>
            </div>
            
            {timeMs && (
                <div style={styles.results}>
                    <p>⏱️ Tiempo: <strong>{formatTime(timeMs)}</strong></p>
                    <p>📊 Rendimiento: <strong>{calculateGFLOPS(size, timeMs).toFixed(2)} GFLOPS</strong></p>
                </div>
            )}
            
            {result && (
                <div style={styles.visualization}>
                    <h3>Resultado:</h3>
                    <MatrixVisualizer 
                        data={result} 
                        width={300} 
                        height={300}
                        colorScale="viridis"
                    />
                </div>
            )}
        </div>
    );
}

function AttentionDemo() {
    const { attention, result, loading, timeMs } = useAttention();
    const [seqLen, setSeqLen] = useState(64);
    const dim = 64;
    
    const handleAttention = useCallback(async () => {
        const query = createRandomMatrix(seqLen, dim);
        const key = createRandomMatrix(seqLen, dim);
        const value = createRandomMatrix(seqLen, dim);
        
        await attention({ query, key, value, dim });
    }, [attention, seqLen]);
    
    return (
        <div style={styles.demo}>
            <h2>🧠 Transformer Attention</h2>
            
            <div style={styles.controls}>
                <label>
                    Secuencia: 
                    <select 
                        value={seqLen} 
                        onChange={e => setSeqLen(Number(e.target.value))}
                        style={styles.select}
                    >
                        <option value={32}>32</option>
                        <option value={64}>64</option>
                        <option value={128}>128</option>
                        <option value={256}>256</option>
                    </select>
                </label>
                
                <button 
                    onClick={handleAttention} 
                    disabled={loading}
                    style={styles.button}
                >
                    {loading ? '⏳ Calculando...' : '🚀 Ejecutar Attention'}
                </button>
            </div>
            
            {timeMs && (
                <div style={styles.results}>
                    <p>⏱️ Tiempo: <strong>{formatTime(timeMs)}</strong></p>
                    <p>📊 Secuencia: <strong>{seqLen}</strong> tokens</p>
                    <p>📐 Dimensión: <strong>{dim}</strong></p>
                </div>
            )}
            
            {result && (
                <div style={styles.visualization}>
                    <h3>Output (primeros {Math.min(64, seqLen)}x{dim}):</h3>
                    <MatrixVisualizer 
                        data={result.slice(0, 64 * dim)} 
                        width={300} 
                        height={300}
                        colorScale="plasma"
                    />
                </div>
            )}
        </div>
    );
}

function TokenizerDemo() {
    const { tokenize, tokens, loading, timeMs, stats } = useTokenizer();
    const [text, setText] = useState('React + ADead-BIB permite crear aplicaciones web ultra-rápidas combinando la facilidad de React con el rendimiento nativo de ADead-BIB.');
    
    const handleTokenize = useCallback(async () => {
        await tokenize(text);
    }, [tokenize, text]);
    
    return (
        <div style={styles.demo}>
            <h2>📝 Tokenización</h2>
            
            <div style={styles.controls}>
                <textarea
                    value={text}
                    onChange={e => setText(e.target.value)}
                    style={styles.textarea}
                    rows={4}
                    placeholder="Escribe texto para tokenizar..."
                />
                
                <button 
                    onClick={handleTokenize} 
                    disabled={loading}
                    style={styles.button}
                >
                    {loading ? '⏳ Tokenizando...' : '🚀 Tokenizar'}
                </button>
            </div>
            
            {timeMs && (
                <div style={styles.results}>
                    <p>⏱️ Tiempo: <strong>{formatTime(timeMs)}</strong></p>
                    <p>📊 Total tokens: <strong>{stats.totalTokens}</strong></p>
                    <p>🔤 Tokens únicos: <strong>{stats.uniqueTokens}</strong></p>
                    <p>📏 Longitud promedio: <strong>{stats.avgTokenLength.toFixed(1)}</strong> chars</p>
                </div>
            )}
            
            {tokens && (
                <div style={styles.tokenList}>
                    <h3>Tokens (primeros 20):</h3>
                    <div style={styles.tokens}>
                        {Array.from(tokens.slice(0, 20)).map((t, i) => (
                            <span key={i} style={styles.token}>{t}</span>
                        ))}
                        {tokens.length > 20 && <span style={styles.more}>+{tokens.length - 20} más</span>}
                    </div>
                </div>
            )}
        </div>
    );
}

function App() {
    return (
        <ADeadProvider config={{ workers: 4, useGPU: true }}>
            <AppContent />
        </ADeadProvider>
    );
}

const styles = {
    app: {
        fontFamily: 'system-ui, -apple-system, sans-serif',
        backgroundColor: '#0f0f23',
        minHeight: '100vh',
        color: '#fff',
    },
    header: {
        textAlign: 'center',
        padding: '40px 20px',
        background: 'linear-gradient(135deg, #1a1a2e 0%, #16213e 100%)',
    },
    title: {
        fontSize: '2.5rem',
        margin: 0,
        background: 'linear-gradient(90deg, #00d4ff, #00ff88)',
        WebkitBackgroundClip: 'text',
        WebkitTextFillColor: 'transparent',
    },
    subtitle: {
        color: '#888',
        marginTop: '10px',
    },
    status: {
        marginTop: '20px',
        display: 'flex',
        justifyContent: 'center',
        gap: '10px',
    },
    badge: {
        backgroundColor: '#1a1a2e',
        padding: '8px 16px',
        borderRadius: '20px',
        fontSize: '0.9rem',
    },
    nav: {
        display: 'flex',
        justifyContent: 'center',
        gap: '10px',
        padding: '20px',
        backgroundColor: '#1a1a2e',
    },
    navButton: {
        padding: '12px 24px',
        border: '1px solid #00d4ff',
        borderRadius: '8px',
        cursor: 'pointer',
        fontSize: '1rem',
        transition: 'all 0.2s',
    },
    main: {
        maxWidth: '800px',
        margin: '0 auto',
        padding: '20px',
    },
    demo: {
        backgroundColor: '#1a1a2e',
        borderRadius: '12px',
        padding: '30px',
    },
    controls: {
        display: 'flex',
        flexDirection: 'column',
        gap: '15px',
        marginBottom: '20px',
    },
    select: {
        marginLeft: '10px',
        padding: '8px 16px',
        borderRadius: '6px',
        backgroundColor: '#16213e',
        color: '#fff',
        border: '1px solid #333',
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
    },
    textarea: {
        width: '100%',
        padding: '12px',
        borderRadius: '8px',
        backgroundColor: '#16213e',
        color: '#fff',
        border: '1px solid #333',
        resize: 'vertical',
        fontFamily: 'inherit',
    },
    results: {
        backgroundColor: '#16213e',
        padding: '20px',
        borderRadius: '8px',
        marginBottom: '20px',
    },
    visualization: {
        textAlign: 'center',
    },
    tokenList: {
        marginTop: '20px',
    },
    tokens: {
        display: 'flex',
        flexWrap: 'wrap',
        gap: '8px',
    },
    token: {
        backgroundColor: '#00d4ff',
        color: '#1a1a2e',
        padding: '4px 12px',
        borderRadius: '4px',
        fontSize: '0.9rem',
        fontFamily: 'monospace',
    },
    more: {
        color: '#888',
        padding: '4px 12px',
    },
    footer: {
        textAlign: 'center',
        padding: '40px 20px',
        color: '#666',
    },
};

export default App;
