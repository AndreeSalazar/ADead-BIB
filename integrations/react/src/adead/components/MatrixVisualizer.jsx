/**
 * MatrixVisualizer - Componente para visualizar matrices
 * =======================================================
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

import React, { useRef, useEffect } from 'react';

/**
 * Visualizador de matrices usando Canvas
 */
export function MatrixVisualizer({ 
    data, 
    width = 400, 
    height = 400,
    colorScale = 'viridis',
    showValues = false,
}) {
    const canvasRef = useRef(null);
    
    useEffect(() => {
        if (!data || !canvasRef.current) return;
        
        const canvas = canvasRef.current;
        const ctx = canvas.getContext('2d');
        
        // Determinar dimensiones de la matriz
        const size = Math.sqrt(data.length);
        if (!Number.isInteger(size)) {
            console.error('MatrixVisualizer: data debe ser una matriz cuadrada');
            return;
        }
        
        // Encontrar min/max para normalización
        let min = Infinity, max = -Infinity;
        for (let i = 0; i < data.length; i++) {
            min = Math.min(min, data[i]);
            max = Math.max(max, data[i]);
        }
        
        const range = max - min || 1;
        
        // Tamaño de cada celda
        const cellWidth = width / size;
        const cellHeight = height / size;
        
        // Dibujar matriz
        for (let i = 0; i < size; i++) {
            for (let j = 0; j < size; j++) {
                const value = data[i * size + j];
                const normalized = (value - min) / range;
                
                // Color según escala
                const color = getColor(normalized, colorScale);
                
                ctx.fillStyle = color;
                ctx.fillRect(j * cellWidth, i * cellHeight, cellWidth, cellHeight);
                
                // Mostrar valores si está habilitado y las celdas son suficientemente grandes
                if (showValues && cellWidth > 30 && cellHeight > 20) {
                    ctx.fillStyle = normalized > 0.5 ? '#000' : '#fff';
                    ctx.font = `${Math.min(cellWidth, cellHeight) * 0.3}px monospace`;
                    ctx.textAlign = 'center';
                    ctx.textBaseline = 'middle';
                    ctx.fillText(
                        value.toFixed(1),
                        j * cellWidth + cellWidth / 2,
                        i * cellHeight + cellHeight / 2
                    );
                }
            }
        }
        
        // Borde
        ctx.strokeStyle = '#333';
        ctx.lineWidth = 2;
        ctx.strokeRect(0, 0, width, height);
        
    }, [data, width, height, colorScale, showValues]);
    
    if (!data) {
        return (
            <div style={styles.placeholder}>
                <p>📊 Sin datos para visualizar</p>
                <p style={{ fontSize: '0.8rem', color: '#666' }}>
                    Pasa una matriz (Float32Array) al prop "data"
                </p>
            </div>
        );
    }
    
    return (
        <div style={styles.container}>
            <canvas 
                ref={canvasRef} 
                width={width} 
                height={height}
                style={styles.canvas}
            />
            <div style={styles.legend}>
                <span>Min</span>
                <div style={{
                    ...styles.legendBar,
                    background: getLegendGradient(colorScale),
                }} />
                <span>Max</span>
            </div>
        </div>
    );
}

// Escalas de color
function getColor(value, scale) {
    const v = Math.max(0, Math.min(1, value));
    
    switch (scale) {
        case 'viridis':
            return viridis(v);
        case 'plasma':
            return plasma(v);
        case 'inferno':
            return inferno(v);
        case 'magma':
            return magma(v);
        case 'grayscale':
            const g = Math.round(v * 255);
            return `rgb(${g},${g},${g})`;
        case 'heatmap':
            return heatmap(v);
        default:
            return viridis(v);
    }
}

function viridis(t) {
    const r = Math.round(68 + t * (253 - 68));
    const g = Math.round(1 + t * (231 - 1));
    const b = Math.round(84 + t * (37 - 84));
    return `rgb(${r},${g},${b})`;
}

function plasma(t) {
    const r = Math.round(13 + t * (240 - 13));
    const g = Math.round(8 + t * (249 - 8));
    const b = Math.round(135 + t * (33 - 135));
    return `rgb(${r},${g},${b})`;
}

function inferno(t) {
    const r = Math.round(0 + t * (252 - 0));
    const g = Math.round(0 + t * (255 - 0));
    const b = Math.round(4 + t * (164 - 4));
    return `rgb(${r},${g},${b})`;
}

function magma(t) {
    const r = Math.round(0 + t * (252 - 0));
    const g = Math.round(0 + t * (253 - 0));
    const b = Math.round(4 + t * (191 - 4));
    return `rgb(${r},${g},${b})`;
}

function heatmap(t) {
    if (t < 0.25) {
        const v = t / 0.25;
        return `rgb(0,0,${Math.round(v * 255)})`;
    } else if (t < 0.5) {
        const v = (t - 0.25) / 0.25;
        return `rgb(0,${Math.round(v * 255)},255)`;
    } else if (t < 0.75) {
        const v = (t - 0.5) / 0.25;
        return `rgb(${Math.round(v * 255)},255,${Math.round((1 - v) * 255)})`;
    } else {
        const v = (t - 0.75) / 0.25;
        return `rgb(255,${Math.round((1 - v) * 255)},0)`;
    }
}

function getLegendGradient(scale) {
    const colors = [];
    for (let i = 0; i <= 10; i++) {
        colors.push(getColor(i / 10, scale));
    }
    return `linear-gradient(to right, ${colors.join(', ')})`;
}

const styles = {
    container: {
        display: 'inline-block',
    },
    canvas: {
        borderRadius: '4px',
        boxShadow: '0 2px 8px rgba(0,0,0,0.3)',
    },
    placeholder: {
        width: '400px',
        height: '400px',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        backgroundColor: '#1a1a2e',
        borderRadius: '8px',
        color: '#888',
    },
    legend: {
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        gap: '10px',
        marginTop: '10px',
        fontSize: '0.8rem',
        color: '#888',
    },
    legendBar: {
        width: '200px',
        height: '12px',
        borderRadius: '4px',
    },
};

export default MatrixVisualizer;
