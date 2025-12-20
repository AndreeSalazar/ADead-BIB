# 📚 Guías de Implementación: ADead-BIB + Python IA

## 🎯 Objetivo
Escalar el sistema de IA para uso general con bajo consumo de RAM.

---

## 📋 Índice de Guías

| # | Guía | Estado | Prioridad |
|---|------|--------|-----------|
| 1 | BPE Tokenizador | ✅ Completado | Alta |
| 2 | Funciones Matriciales en ADead-BIB | ✅ Completado | Alta |
| 3 | Vocabulario 10K-50K | ✅ Completado | Media |
| 4 | Embeddings Pre-entrenados | ✅ Completado | Media |
| 5 | Integración Ollama/llama.cpp | ✅ Completado | Baja |

---

# Guía 1: BPE Tokenizador ✅

## Estado: COMPLETADO

### Qué se implementó
- Tokenizador BPE (Byte Pair Encoding)
- Vocabulario expandible automáticamente
- Caché de embeddings (93% hit rate)

### Archivo
`python/ai_scalable.py`

### Resultados
```
UNK ratio: 0% (antes 25-50%)
Tokens/segundo: 271
RAM: 0.82 MB
```

---

# Guía 2: Funciones Matriciales en ADead-BIB

## Estado: ✅ COMPLETADO

### Funciones Implementadas

| Función | Descripción | Ejemplo |
|---------|-------------|---------|
| `dot(a,b,c,d)` | Producto punto | dot(2,3,4,5) = 26 |
| `dot6(...)` | Producto punto 6 elementos | - |
| `sum_sq(a,b,...)` | Suma de cuadrados | sum_sq(3,4) = 25 |
| `norm_sq(a,b,...)` | Norma al cuadrado | norm_sq(3,4) = 25 |
| `weighted_sum(v,w,...)` | Suma ponderada | weighted_sum(10,2,20,3) = 80 |
| `relu(x)` | Activación ReLU | relu(-3) = 0 |
| `sigmoid_approx(x)` | Sigmoid aproximado | - |
| `softmax_max(...)` | Máximo para softmax | - |
| `scale(x,f)` | Escalar x*f/100 | scale(200,50) = 100 |
| `lerp(a,b,t)` | Interpolación lineal | lerp(0,100,50) = 50 |

### Objetivo Original
Mover operaciones matemáticas pesadas de Python/NumPy a ADead-BIB para mayor rendimiento.

### Funciones a implementar

#### 2.1 `matmul(A, B)` - Multiplicación de matrices
```python
# En ADead-BIB
def matmul(a: array, b: array) -> array:
    # Multiplicación optimizada O(n³)
    result = []
    for i in range(rows_a):
        for j in range(cols_b):
            sum = 0
            for k in range(cols_a):
                sum = sum + a[i][k] * b[k][j]
            result[i][j] = sum
    return result
```

#### 2.2 `dot(a, b)` - Producto punto
```python
def dot(a: array, b: array) -> float:
    sum = 0
    for i in range(len(a)):
        sum = sum + a[i] * b[i]
    return sum
```

#### 2.3 `softmax(x)` - Función softmax
```python
def softmax(x: array) -> array:
    max_x = max(x)
    exp_x = []
    sum_exp = 0
    for i in range(len(x)):
        exp_x[i] = exp(x[i] - max_x)
        sum_exp = sum_exp + exp_x[i]
    for i in range(len(x)):
        exp_x[i] = exp_x[i] / sum_exp
    return exp_x
```

#### 2.4 `relu(x)` - Activación ReLU
```python
def relu(x: array) -> array:
    result = []
    for i in range(len(x)):
        if x[i] > 0:
            result[i] = x[i]
        else:
            result[i] = 0
    return result
```

### Pasos de implementación

1. **Agregar soporte para arrays 2D en AST**
   - Archivo: `src/rust/frontend/ast.rs`
   - Agregar: `Array2D { rows: usize, cols: usize, data: Vec<Expr> }`

2. **Implementar funciones built-in**
   - Archivo: `src/rust/backend/codegen.rs`
   - Agregar: `emit_matmul`, `emit_dot`, `emit_softmax`

3. **Optimizar con SIMD (opcional)**
   - Usar instrucciones SSE/AVX para operaciones vectoriales

### Ejemplo de uso
```python
# Python llama a ADead-BIB para cálculo pesado
from adead_ffi import ADeadBIB

adead = ADeadBIB()

# Código ADead-BIB para multiplicación
code = '''
def main():
    a = [[1, 2], [3, 4]]
    b = [[5, 6], [7, 8]]
    c = matmul(a, b)
    print(c[0][0])  # 19
    print(c[1][1])  # 50
'''

result = adead.run_code(code)
```

### Beneficios esperados
- 2-5x más rápido que NumPy para matrices pequeñas
- Menor overhead de llamadas
- Control total sobre optimizaciones

---

# Guía 3: Vocabulario 10K-50K

## Estado: PENDIENTE

### Objetivo
Expandir vocabulario para cubrir más palabras y reducir UNK a <1%.

### Pasos

#### 3.1 Descargar corpus de entrenamiento
```bash
# Opción 1: Wikipedia dump (recomendado)
wget https://dumps.wikimedia.org/enwiki/latest/enwiki-latest-pages-articles.xml.bz2

# Opción 2: Common Crawl (más grande)
# https://commoncrawl.org/

# Opción 3: Corpus propio
# Recopilar textos de tu dominio
```

#### 3.2 Preprocesar corpus
```python
import re

def preprocess(text):
    # Limpiar HTML
    text = re.sub(r'<[^>]+>', '', text)
    # Normalizar espacios
    text = re.sub(r'\s+', ' ', text)
    # Minúsculas
    text = text.lower()
    return text
```

#### 3.3 Entrenar BPE con más fusiones
```python
from ai_scalable import BPETokenizer

# Cargar corpus
with open('corpus.txt', 'r', encoding='utf-8') as f:
    texts = f.readlines()

# Entrenar con más fusiones
tokenizer = BPETokenizer(vocab_size=50000)
tokenizer.train(texts, num_merges=45000)

# Guardar vocabulario
tokenizer.save('vocab_50k.json')
```

#### 3.4 Guardar/Cargar vocabulario
```python
import json

class BPETokenizer:
    def save(self, path):
        data = {
            'vocab': self.vocab,
            'merges': {f"{k[0]}|{k[1]}": v for k, v in self.merges.items()}
        }
        with open(path, 'w') as f:
            json.dump(data, f)
    
    def load(self, path):
        with open(path, 'r') as f:
            data = json.load(f)
        self.vocab = data['vocab']
        self.inv_vocab = {v: k for k, v in self.vocab.items()}
        self.merges = {tuple(k.split('|')): v for k, v in data['merges'].items()}
```

### RAM estimada
| Vocabulario | RAM Embeddings (128 dim, float16) |
|-------------|-----------------------------------|
| 10,000 | 2.5 MB |
| 30,000 | 7.5 MB |
| 50,000 | 12.5 MB |

---

# Guía 4: Embeddings Pre-entrenados

## Estado: PENDIENTE

### Objetivo
Usar embeddings con semántica real en lugar de aleatorios.

### Opciones

#### 4.1 GloVe (Recomendado para bajo RAM)
```bash
# Descargar GloVe 50d (66 MB comprimido)
wget https://nlp.stanford.edu/data/glove.6B.zip
unzip glove.6B.zip
```

```python
def load_glove(path, vocab, embed_dim=50):
    """Carga embeddings GloVe para vocabulario existente."""
    embeddings = np.random.randn(len(vocab), embed_dim).astype(np.float16) * 0.02
    
    with open(path, 'r', encoding='utf-8') as f:
        for line in f:
            parts = line.strip().split()
            word = parts[0]
            if word in vocab:
                idx = vocab[word]
                vector = np.array([float(x) for x in parts[1:]], dtype=np.float16)
                embeddings[idx] = vector
    
    return embeddings
```

#### 4.2 FastText (Mejor para palabras raras)
```bash
pip install fasttext
```

```python
import fasttext

# Descargar modelo pre-entrenado
model = fasttext.load_model('cc.en.300.bin')

def get_fasttext_embedding(word):
    return model.get_word_vector(word)
```

#### 4.3 Cuantización para bajo RAM
```python
def quantize_embeddings(embeddings, bits=8):
    """Cuantiza embeddings a int8 para reducir RAM 4x."""
    min_val = embeddings.min()
    max_val = embeddings.max()
    scale = (max_val - min_val) / (2**bits - 1)
    
    quantized = ((embeddings - min_val) / scale).astype(np.uint8)
    
    return quantized, min_val, scale

def dequantize(quantized, min_val, scale):
    return quantized.astype(np.float32) * scale + min_val
```

### RAM con cuantización
| Vocabulario | float16 | int8 | Reducción |
|-------------|---------|------|-----------|
| 50,000 x 128 | 12.5 MB | 6.25 MB | 50% |
| 50,000 x 64 | 6.25 MB | 3.12 MB | 50% |

---

# Guía 5: Integración Ollama/llama.cpp

## Estado: PENDIENTE

### Objetivo
Usar modelos de lenguaje reales para generación de alta calidad.

### Opción A: Ollama (Más fácil)

#### Instalación
```bash
# Windows
winget install Ollama.Ollama

# Descargar modelo pequeño (1.1 GB)
ollama pull tinyllama
```

#### Uso desde Python
```python
import ollama

def chat_ollama(message):
    response = ollama.chat(
        model='tinyllama',
        messages=[{'role': 'user', 'content': message}]
    )
    return response['message']['content']

# Integrar con ADead-BIB para pre/post procesamiento
from adead_ffi import ADeadBIB

class HybridAI:
    def __init__(self):
        self.adead = ADeadBIB()
    
    def process(self, text):
        # Pre-procesamiento rápido con ADead-BIB
        tokens = self.adead.tokenize(text)
        
        # Inferencia con Ollama
        response = chat_ollama(text)
        
        # Post-procesamiento con ADead-BIB
        return self.adead.format(response)
```

### Opción B: llama.cpp (Más control)

#### Instalación
```bash
pip install llama-cpp-python
```

#### Descargar modelo GGUF
```bash
# TinyLlama cuantizado (700 MB)
wget https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/resolve/main/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf
```

#### Uso
```python
from llama_cpp import Llama

llm = Llama(
    model_path="tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf",
    n_ctx=2048,
    n_threads=4
)

def generate(prompt, max_tokens=100):
    output = llm(prompt, max_tokens=max_tokens)
    return output['choices'][0]['text']
```

### Comparación

| Aspecto | Ollama | llama.cpp |
|---------|--------|-----------|
| Facilidad | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| Control | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| RAM mínima | 2 GB | 1 GB |
| Velocidad | Buena | Excelente |

---

# 🚀 Orden de Implementación Recomendado

```
Semana 1: Guía 2 - Funciones matriciales
    └── Agregar matmul, dot, softmax a ADead-BIB
    └── Probar rendimiento vs NumPy

Semana 2: Guía 3 - Vocabulario grande
    └── Entrenar BPE con corpus grande
    └── Guardar/cargar vocabulario

Semana 3: Guía 4 - Embeddings pre-entrenados
    └── Cargar GloVe
    └── Cuantizar a int8

Semana 4: Guía 5 - Integración con modelos reales
    └── Instalar Ollama
    └── Crear pipeline híbrido
```

---

# ✅ Checklist de Progreso

- [x] Guía 1: BPE Tokenizador
- [x] Guía 2: Funciones matriciales (10 funciones nuevas)
- [x] Guía 3: Vocabulario escalable (vocabulary.py)
- [x] Guía 4: Embeddings semánticos (embeddings.py)
- [x] Guía 5: Integración Ollama (ollama_integration.py)

---

## 📁 Archivos Implementados

| Archivo | Descripción |
|---------|-------------|
| `ai_complete.py` | IA básica (0.19 MB RAM) |
| `ai_scalable.py` | IA con BPE (0.82 MB RAM) |
| `vocabulary.py` | Sistema de vocabulario escalable |
| `embeddings.py` | Embeddings semánticos + cuantización |
| `ollama_integration.py` | Integración con Ollama |
| `adead_ffi.py` | FFI Python ↔ ADead-BIB |

## 🎯 Funciones Matriciales en ADead-BIB

| Función | Uso |
|---------|-----|
| `dot(a,b,c,d)` | Producto punto |
| `sum_sq(a,b,...)` | Suma de cuadrados |
| `norm_sq(a,b,...)` | Norma al cuadrado |
| `weighted_sum(v,w,...)` | Suma ponderada |
| `relu(x)` | Activación ReLU |
| `sigmoid_approx(x)` | Sigmoid aproximado |
| `scale(x,f)` | Escalar |
| `lerp(a,b,t)` | Interpolación lineal |

---

**Meta alcanzada:** IA escalable con <1 MB RAM, 0% UNK, embeddings semánticos.
