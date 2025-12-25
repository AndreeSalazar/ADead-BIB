/**
 * ADead-BIB Core para React
 * ==========================
 * Motor de cómputo pesado optimizado
 * Author: Eddi Andreé Salazar Matos
 * Made with ❤️ in Peru 🇵🇪
 */

export class ADeadCore {
    constructor() {
        this.stats = {
            operations: 0,
            totalTimeMs: 0,
            dataProcessed: 0,
        };
    }

    /**
     * Multiplicación de matrices optimizada con blocking
     */
    matmul(a, b, size) {
        const result = new Float32Array(size * size);
        const blockSize = 32;

        for (let ii = 0; ii < size; ii += blockSize) {
            for (let jj = 0; jj < size; jj += blockSize) {
                for (let kk = 0; kk < size; kk += blockSize) {
                    const iMax = Math.min(ii + blockSize, size);
                    const jMax = Math.min(jj + blockSize, size);
                    const kMax = Math.min(kk + blockSize, size);

                    for (let i = ii; i < iMax; i++) {
                        for (let j = jj; j < jMax; j++) {
                            let sum = result[i * size + j];
                            for (let k = kk; k < kMax; k++) {
                                sum += a[i * size + k] * b[k * size + j];
                            }
                            result[i * size + j] = sum;
                        }
                    }
                }
            }
        }

        return result;
    }

    /**
     * Ordenamiento ultra-rápido con Typed Arrays
     */
    sort(data) {
        return Float64Array.from(data).sort();
    }

    /**
     * Búsqueda binaria optimizada
     */
    binarySearch(sortedArray, target) {
        let left = 0, right = sortedArray.length - 1;
        while (left <= right) {
            const mid = (left + right) >>> 1;
            if (sortedArray[mid] === target) return mid;
            if (sortedArray[mid] < target) left = mid + 1;
            else right = mid - 1;
        }
        return -1;
    }

    /**
     * Filtrado masivo optimizado
     */
    filterData(data, predicate) {
        const result = [];
        const len = data.length;
        for (let i = 0; i < len; i++) {
            if (predicate(data[i])) {
                result.push(data[i]);
            }
        }
        return result;
    }

    /**
     * Agregación de datos masivos
     */
    aggregate(data, groupKey, valueKey) {
        const groups = new Map();
        const len = data.length;

        for (let i = 0; i < len; i++) {
            const item = data[i];
            const key = item[groupKey];
            const value = item[valueKey];

            if (groups.has(key)) {
                const g = groups.get(key);
                g.sum += value;
                g.count++;
                g.min = Math.min(g.min, value);
                g.max = Math.max(g.max, value);
            } else {
                groups.set(key, {
                    sum: value,
                    count: 1,
                    min: value,
                    max: value,
                });
            }
        }

        const result = [];
        for (const [key, g] of groups) {
            result.push({
                [groupKey]: key,
                sum: g.sum,
                avg: g.sum / g.count,
                count: g.count,
                min: g.min,
                max: g.max,
            });
        }

        return result;
    }

    /**
     * Estadísticas de array numérico
     */
    statistics(data) {
        const len = data.length;
        if (len === 0) return null;

        let sum = 0, min = Infinity, max = -Infinity;
        for (let i = 0; i < len; i++) {
            const v = data[i];
            sum += v;
            min = Math.min(min, v);
            max = Math.max(max, v);
        }

        const mean = sum / len;

        let variance = 0;
        for (let i = 0; i < len; i++) {
            const diff = data[i] - mean;
            variance += diff * diff;
        }
        variance /= len;

        return {
            count: len,
            sum,
            mean,
            min,
            max,
            variance,
            stdDev: Math.sqrt(variance),
        };
    }

    /**
     * Transformer Attention
     */
    attention(query, key, value, dim) {
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

    /**
     * Tokenización rápida
     */
    tokenize(text, maxLength = 512) {
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

    /**
     * Genera datos de prueba masivos
     */
    generateTestData(count, type = 'sales') {
        const data = [];
        const categories = ['Electronics', 'Clothing', 'Food', 'Books', 'Sports', 'Home', 'Auto', 'Health'];
        const regions = ['North', 'South', 'East', 'West', 'Central'];
        const years = [2020, 2021, 2022, 2023, 2024];

        for (let i = 0; i < count; i++) {
            if (type === 'sales') {
                data.push({
                    id: i,
                    category: categories[Math.floor(Math.random() * categories.length)],
                    region: regions[Math.floor(Math.random() * regions.length)],
                    year: years[Math.floor(Math.random() * years.length)],
                    amount: Math.random() * 10000,
                    quantity: Math.floor(Math.random() * 100) + 1,
                    profit: Math.random() * 2000 - 500,
                });
            } else if (type === 'numeric') {
                data.push(Math.random() * 1000000);
            }
        }

        return data;
    }
}

export const adead = new ADeadCore();
export default adead;
