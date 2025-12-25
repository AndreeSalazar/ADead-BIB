/**
 * React + ADead-BIB Heavy Data Demo
 * ==================================
 * Demostración de rendimiento con datos masivos
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

import { useState, useCallback, useMemo } from 'react';
import { useHeavyData, useBenchmark, useADead } from './adead/hooks';
import { adead } from './adead/core';
import './App.css';

function App() {
  const [activeTab, setActiveTab] = useState('data');
  
  return (
    <div className="app">
      <header className="header">
        <h1>🔥 React + ADead-BIB + Bun</h1>
        <p>Demostración de rendimiento con datos masivos</p>
      </header>
      
      <nav className="nav">
        {['data', 'benchmark', 'matrix', 'attention'].map(tab => (
          <button
            key={tab}
            onClick={() => setActiveTab(tab)}
            className={`nav-btn ${activeTab === tab ? 'active' : ''}`}
          >
            {tab === 'data' && '📊 Datos Pesados'}
            {tab === 'benchmark' && '⚡ Benchmark'}
            {tab === 'matrix' && '🔢 Matrices'}
            {tab === 'attention' && '🧠 Attention'}
          </button>
        ))}
      </nav>
      
      <main className="main">
        {activeTab === 'data' && <HeavyDataDemo />}
        {activeTab === 'benchmark' && <BenchmarkDemo />}
        {activeTab === 'matrix' && <MatrixDemo />}
        {activeTab === 'attention' && <AttentionDemo />}
      </main>
      
      <footer className="footer">
        <p>Made with ❤️ in Peru 🇵🇪 | ADead-BIB + React + Bun</p>
      </footer>
    </div>
  );
}

function HeavyDataDemo() {
  const { data, loading, generationTime, generate, filter, aggregate, count } = useHeavyData();
  const [dataCount, setDataCount] = useState(100000);
  const [filterResult, setFilterResult] = useState(null);
  const [aggregateResult, setAggregateResult] = useState(null);
  
  const handleGenerate = () => {
    generate(dataCount, 'sales');
  };
  
  const handleFilter = () => {
    const result = filter(item => item.amount > 5000);
    setFilterResult(result);
  };
  
  const handleAggregate = () => {
    const result = aggregate('category', 'amount');
    setAggregateResult(result);
  };
  
  return (
    <div className="demo-section">
      <h2>📊 Procesamiento de Datos Masivos</h2>
      
      <div className="controls">
        <label>
          Cantidad de registros:
          <select value={dataCount} onChange={e => setDataCount(Number(e.target.value))}>
            <option value={10000}>10,000</option>
            <option value={100000}>100,000</option>
            <option value={500000}>500,000</option>
            <option value={1000000}>1,000,000</option>
            <option value={5000000}>5,000,000</option>
          </select>
        </label>
        
        <button onClick={handleGenerate} disabled={loading} className="btn primary">
          {loading ? '⏳ Generando...' : '🚀 Generar Datos'}
        </button>
      </div>
      
      {count > 0 && (
        <div className="results">
          <div className="stat-card">
            <h3>📈 Datos Generados</h3>
            <p className="big-number">{count.toLocaleString()}</p>
            <p className="small">registros</p>
            <p className="time">⏱️ {generationTime?.toFixed(2)} ms</p>
          </div>
          
          <div className="actions">
            <button onClick={handleFilter} className="btn">
              🔍 Filtrar (amount &gt; 5000)
            </button>
            <button onClick={handleAggregate} className="btn">
              📊 Agregar por Categoría
            </button>
          </div>
          
          {filterResult && (
            <div className="result-card">
              <h4>🔍 Resultado del Filtro</h4>
              <p>Encontrados: <strong>{filterResult.count.toLocaleString()}</strong> registros</p>
              <p>Tiempo: <strong>{filterResult.timeMs.toFixed(2)} ms</strong></p>
            </div>
          )}
          
          {aggregateResult && (
            <div className="result-card">
              <h4>📊 Agregación por Categoría</h4>
              <p>Tiempo: <strong>{aggregateResult.timeMs.toFixed(2)} ms</strong></p>
              <table className="data-table">
                <thead>
                  <tr>
                    <th>Categoría</th>
                    <th>Total</th>
                    <th>Promedio</th>
                    <th>Count</th>
                  </tr>
                </thead>
                <tbody>
                  {aggregateResult.data.slice(0, 10).map((row, i) => (
                    <tr key={i}>
                      <td>{row.category}</td>
                      <td>${row.sum.toLocaleString(undefined, {maximumFractionDigits: 0})}</td>
                      <td>${row.avg.toFixed(2)}</td>
                      <td>{row.count.toLocaleString()}</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
          
          <div className="sample-data">
            <h4>📋 Muestra de Datos (primeros 5)</h4>
            <pre>{JSON.stringify(data.slice(0, 5), null, 2)}</pre>
          </div>
        </div>
      )}
    </div>
  );
}

function BenchmarkDemo() {
  const { results, running, currentTest, runBenchmark } = useBenchmark();
  
  const tests = useMemo(() => [
    {
      name: 'Generar 100K registros',
      fn: () => adead.generateTestData(100000, 'sales'),
      iterations: 3,
    },
    {
      name: 'Generar 500K registros',
      fn: () => adead.generateTestData(500000, 'sales'),
      iterations: 3,
    },
    {
      name: 'Ordenar 1M números',
      fn: () => {
        const arr = new Float64Array(1000000);
        for (let i = 0; i < 1000000; i++) arr[i] = Math.random();
        return adead.sort(arr);
      },
      iterations: 3,
    },
    {
      name: 'MatMul 256x256',
      fn: () => {
        const size = 256;
        const a = new Float32Array(size * size);
        const b = new Float32Array(size * size);
        for (let i = 0; i < size * size; i++) {
          a[i] = Math.random();
          b[i] = Math.random();
        }
        return adead.matmul(a, b, size);
      },
      iterations: 5,
    },
    {
      name: 'Attention seq=256',
      fn: () => {
        const seqLen = 256;
        const dim = 64;
        const q = new Float32Array(seqLen * dim);
        const k = new Float32Array(seqLen * dim);
        const v = new Float32Array(seqLen * dim);
        for (let i = 0; i < seqLen * dim; i++) {
          q[i] = Math.random() * 0.1;
          k[i] = Math.random() * 0.1;
          v[i] = Math.random() * 0.1;
        }
        return adead.attention(q, k, v, dim);
      },
      iterations: 5,
    },
    {
      name: 'Tokenizar 100K chars',
      fn: () => {
        const text = 'hello world programming react adead bib '.repeat(2500);
        return adead.tokenize(text);
      },
      iterations: 5,
    },
  ], []);
  
  return (
    <div className="demo-section">
      <h2>⚡ Benchmark de Rendimiento</h2>
      
      <button 
        onClick={() => runBenchmark(tests)} 
        disabled={running}
        className="btn primary"
      >
        {running ? `⏳ ${currentTest}...` : '🚀 Ejecutar Benchmarks'}
      </button>
      
      {results.length > 0 && (
        <table className="benchmark-table">
          <thead>
            <tr>
              <th>Test</th>
              <th>Promedio</th>
              <th>Min</th>
              <th>Max</th>
              <th>Iteraciones</th>
            </tr>
          </thead>
          <tbody>
            {results.map((r, i) => (
              <tr key={i}>
                <td>{r.name}</td>
                <td className="time">{r.avgMs.toFixed(2)} ms</td>
                <td>{r.minMs.toFixed(2)} ms</td>
                <td>{r.maxMs.toFixed(2)} ms</td>
                <td>{r.iterations}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}

function MatrixDemo() {
  const { execute, loading, timeMs } = useADead();
  const [size, setSize] = useState(128);
  const [result, setResult] = useState(null);
  
  const handleMatMul = async () => {
    const a = new Float32Array(size * size);
    const b = new Float32Array(size * size);
    for (let i = 0; i < size * size; i++) {
      a[i] = Math.random();
      b[i] = Math.random();
    }
    
    const res = await execute('matmul', a, b, size);
    setResult(res);
  };
  
  const gflops = result ? (2 * size * size * size) / (result.timeMs * 1e6) : 0;
  
  return (
    <div className="demo-section">
      <h2>🔢 Multiplicación de Matrices</h2>
      
      <div className="controls">
        <label>
          Tamaño:
          <select value={size} onChange={e => setSize(Number(e.target.value))}>
            <option value={64}>64x64</option>
            <option value={128}>128x128</option>
            <option value={256}>256x256</option>
            <option value={512}>512x512</option>
          </select>
        </label>
        
        <button onClick={handleMatMul} disabled={loading} className="btn primary">
          {loading ? '⏳ Calculando...' : '🚀 Multiplicar'}
        </button>
      </div>
      
      {result && (
        <div className="results">
          <div className="stat-card">
            <h3>📊 Resultado</h3>
            <p>Tamaño: <strong>{size}x{size}</strong></p>
            <p>Operaciones: <strong>{(2 * size * size * size).toLocaleString()}</strong></p>
            <p>Tiempo: <strong>{result.timeMs.toFixed(2)} ms</strong></p>
            <p>Rendimiento: <strong>{gflops.toFixed(2)} GFLOPS</strong></p>
          </div>
        </div>
      )}
    </div>
  );
}

function AttentionDemo() {
  const { execute, loading } = useADead();
  const [seqLen, setSeqLen] = useState(128);
  const [result, setResult] = useState(null);
  const dim = 64;
  
  const handleAttention = async () => {
    const q = new Float32Array(seqLen * dim);
    const k = new Float32Array(seqLen * dim);
    const v = new Float32Array(seqLen * dim);
    
    for (let i = 0; i < seqLen * dim; i++) {
      q[i] = Math.random() * 0.1;
      k[i] = Math.random() * 0.1;
      v[i] = Math.random() * 0.1;
    }
    
    const res = await execute('attention', q, k, v, dim);
    setResult(res);
  };
  
  return (
    <div className="demo-section">
      <h2>🧠 Transformer Attention</h2>
      
      <div className="controls">
        <label>
          Secuencia:
          <select value={seqLen} onChange={e => setSeqLen(Number(e.target.value))}>
            <option value={64}>64</option>
            <option value={128}>128</option>
            <option value={256}>256</option>
            <option value={512}>512</option>
          </select>
        </label>
        
        <button onClick={handleAttention} disabled={loading} className="btn primary">
          {loading ? '⏳ Calculando...' : '🚀 Ejecutar Attention'}
        </button>
      </div>
      
      {result && (
        <div className="results">
          <div className="stat-card">
            <h3>📊 Resultado</h3>
            <p>Secuencia: <strong>{seqLen}</strong> tokens</p>
            <p>Dimensión: <strong>{dim}</strong></p>
            <p>Tiempo: <strong>{result.timeMs.toFixed(2)} ms</strong></p>
            <p>Output size: <strong>{result.data.length.toLocaleString()}</strong> elementos</p>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
