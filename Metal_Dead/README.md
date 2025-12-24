# ⚡ Metal-Dead

**IA Personal Ultra-Eficiente con Pensamiento Crítico para ADead-BIB**

Author: Eddi Andreé Salazar Matos  
Email: eddi.salazar.dev@gmail.com  
Made with ❤️ in Peru 🇵🇪

---

## 🚀 Inicio Rápido

```powershell
cd ADead-BIB

# Chat estándar
python -m Metal_Dead

# Con GPU (CUDA)
python -m Metal_Dead --gpu

# GPU MAX (Flash Attention + BF16 + Tensor Cores)
python -m Metal_Dead --gpu-max

# 🧠 MODO INTELIGENTE (Pensamiento Crítico)
python -m Metal_Dead --smart

# 🔥 MÁXIMO PODER (Inteligencia + GPU MAX)
python -m Metal_Dead --smart-gpu

# Demo del sistema
python -m Metal_Dead --demo

# Benchmark
python -m Metal_Dead --benchmark
```

---

## 📁 Estructura

```
Metal_Dead/
├── __init__.py           # Módulo principal
├── __main__.py           # Punto de entrada
├── core/
│   ├── metal_dead.py     # Sistema principal
│   ├── memory.py         # Memoria persistente
│   ├── context.py        # Contexto personal
│   ├── tokenizer.py      # Tokenizador
│   └── model.py          # Transformer ligero
├── integrations/
│   ├── gpu_compute.py    # GPU básico
│   ├── gpu_advanced.py   # GPU MAX (Flash Attention)
│   └── adead_accelerator.py  # ADead-BIB
├── ui/
│   ├── chat.py           # Chat interactivo
│   └── cli.py            # Línea de comandos
└── data/                 # Datos persistentes
```

---

## ⚡ Características

| Característica | Descripción |
|----------------|-------------|
| **Ultra-Ligero** | < 1 MB de RAM |
| **GPU Acelerado** | CUDA + Flash Attention |
| **BF16/FP16** | Tensor Cores en RTX |
| **Memoria Persistente** | Recuerda conversaciones |
| **Aprendizaje** | Aprende sobre ti |
| **Sin Runtime** | Diseñado para ADead-BIB |
| **🧠 Pensamiento Crítico** | Razona antes de responder |
| **📚 Base de Conocimiento** | 13+ temas integrados |
| **🎯 Detección de Intención** | Entiende qué quieres |
| **💭 Análisis de Sentimiento** | Detecta tu estado de ánimo |

---

## 📊 Rendimiento (RTX 3060)

| Operación | CPU | GPU | Speedup |
|-----------|-----|-----|---------|
| MatMul 1024 | 10ms | 5ms | **2x** |
| Attention 512 | 4ms | 1ms | **4x** |
| Chat | 1ms | 0.5ms | **2x** |

---

## 💬 Comandos de Chat

- **Conversación normal** - Solo escribe
- **"me llamo [nombre]"** - Aprende tu nombre
- **"me gusta [algo]"** - Aprende intereses
- **"recuerda que [algo]"** - Guarda información
- **"busca [tema]"** - Busca en memorias
- **/memoria** - Ver estadísticas
- **/perfil** - Tu perfil
- **/ayuda** - Ayuda
- **/salir** - Salir

---

## 🔧 Requisitos

```powershell
# Básico
pip install numpy colorama

# GPU (opcional pero recomendado)
pip install torch --index-url https://download.pytorch.org/whl/cu121
```

---

Made with ⚡ for ADead-BIB
