/**
 * Helpers y utilidades para ADead-BIB en React
 * =============================================
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

/**
 * Crea una matriz de Float32Array con valores aleatorios
 * @param {number} rows - Número de filas
 * @param {number} cols - Número de columnas
 * @returns {Float32Array}
 */
export function createRandomMatrix(rows, cols) {
    const size = rows * cols;
    const matrix = new Float32Array(size);
    for (let i = 0; i < size; i++) {
        matrix[i] = Math.random();
    }
    return matrix;
}

/**
 * Crea una matriz identidad
 * @param {number} size - Tamaño de la matriz
 * @returns {Float32Array}
 */
export function createIdentityMatrix(size) {
    const matrix = new Float32Array(size * size);
    for (let i = 0; i < size; i++) {
        matrix[i * size + i] = 1;
    }
    return matrix;
}

/**
 * Transpone una matriz
 * @param {Float32Array} matrix - Matriz original
 * @param {number} rows - Número de filas
 * @param {number} cols - Número de columnas
 * @returns {Float32Array}
 */
export function transposeMatrix(matrix, rows, cols) {
    const result = new Float32Array(rows * cols);
    for (let i = 0; i < rows; i++) {
        for (let j = 0; j < cols; j++) {
            result[j * rows + i] = matrix[i * cols + j];
        }
    }
    return result;
}

/**
 * Formatea un número grande
 * @param {number} num - Número a formatear
 * @returns {string}
 */
export function formatNumber(num) {
    if (num >= 1e9) return (num / 1e9).toFixed(2) + 'B';
    if (num >= 1e6) return (num / 1e6).toFixed(2) + 'M';
    if (num >= 1e3) return (num / 1e3).toFixed(2) + 'K';
    return num.toFixed(2);
}

/**
 * Formatea tiempo en milisegundos
 * @param {number} ms - Milisegundos
 * @returns {string}
 */
export function formatTime(ms) {
    if (ms < 1) return `${(ms * 1000).toFixed(2)} µs`;
    if (ms < 1000) return `${ms.toFixed(2)} ms`;
    return `${(ms / 1000).toFixed(2)} s`;
}

/**
 * Calcula GFLOPS para multiplicación de matrices
 * @param {number} size - Tamaño de la matriz
 * @param {number} timeMs - Tiempo en milisegundos
 * @returns {number}
 */
export function calculateGFLOPS(size, timeMs) {
    const flops = 2 * size * size * size;
    return flops / (timeMs * 1e6);
}

/**
 * Debounce para funciones
 * @param {Function} func - Función a debounce
 * @param {number} wait - Tiempo de espera en ms
 * @returns {Function}
 */
export function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

/**
 * Throttle para funciones
 * @param {Function} func - Función a throttle
 * @param {number} limit - Límite de tiempo en ms
 * @returns {Function}
 */
export function throttle(func, limit) {
    let inThrottle;
    return function executedFunction(...args) {
        if (!inThrottle) {
            func(...args);
            inThrottle = true;
            setTimeout(() => inThrottle = false, limit);
        }
    };
}

/**
 * Verifica si WebGPU está disponible
 * @returns {Promise<boolean>}
 */
export async function isWebGPUAvailable() {
    if (!navigator.gpu) return false;
    try {
        const adapter = await navigator.gpu.requestAdapter();
        return !!adapter;
    } catch {
        return false;
    }
}

/**
 * Verifica si Web Workers están disponibles
 * @returns {boolean}
 */
export function isWebWorkerAvailable() {
    return typeof Worker !== 'undefined';
}

/**
 * Obtiene información del hardware
 * @returns {Object}
 */
export function getHardwareInfo() {
    return {
        cores: navigator.hardwareConcurrency || 4,
        memory: navigator.deviceMemory || 'unknown',
        platform: navigator.platform,
        userAgent: navigator.userAgent,
    };
}

/**
 * Convierte ArrayBuffer a Base64
 * @param {ArrayBuffer} buffer
 * @returns {string}
 */
export function arrayBufferToBase64(buffer) {
    let binary = '';
    const bytes = new Uint8Array(buffer);
    for (let i = 0; i < bytes.byteLength; i++) {
        binary += String.fromCharCode(bytes[i]);
    }
    return btoa(binary);
}

/**
 * Convierte Base64 a ArrayBuffer
 * @param {string} base64
 * @returns {ArrayBuffer}
 */
export function base64ToArrayBuffer(base64) {
    const binary = atob(base64);
    const bytes = new Uint8Array(binary.length);
    for (let i = 0; i < binary.length; i++) {
        bytes[i] = binary.charCodeAt(i);
    }
    return bytes.buffer;
}

export default {
    createRandomMatrix,
    createIdentityMatrix,
    transposeMatrix,
    formatNumber,
    formatTime,
    calculateGFLOPS,
    debounce,
    throttle,
    isWebGPUAvailable,
    isWebWorkerAvailable,
    getHardwareInfo,
    arrayBufferToBase64,
    base64ToArrayBuffer,
};
