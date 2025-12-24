# 🤖 IA-Personal para ADead-BIB

**Sistema de IA Personal Ultra-Ligero**

> Tu asistente personal que aprende de ti, recuerda tus conversaciones y se integra con ADead-BIB.

---

## 🇵🇪 Made with ❤️ in Peru

**Author:** Eddi Andreé Salazar Matos  
**Email:** eddi.salazar.dev@gmail.com  
**Version:** 1.0.0

---

## ✨ Características

| Característica | Descripción |
|----------------|-------------|
| **Memoria Persistente** | Recuerda conversaciones entre sesiones |
| **Contexto Personal** | Aprende tu nombre, intereses y preferencias |
| **Aprendizaje Continuo** | Mejora con cada interacción |
| **Ultra-Ligero** | Solo ~0.5 MB de RAM |
| **100% Privado** | Todo se procesa localmente |
| **Integración ADead-BIB** | Operaciones aceleradas sin runtime |
| **Integración Ollama** | LLM local para respuestas avanzadas |

---

## 🚀 Inicio Rápido

### Desde la carpeta IA_Personal
```powershell
cd IA_Personal
python -m IA_Personal
```

### Modos de Ejecución
```powershell
# Chat estándar
python -m IA_Personal

# Modo turbo (más rápido)
python -m IA_Personal --turbo

# 🎮 Con aceleración GPU (CUDA) - NUEVO!
python -m IA_Personal --gpu

# Con aceleración ADead-BIB
python -m IA_Personal --adead

# Con Ollama LLM
python -m IA_Personal --ollama

# Todas las integraciones (GPU + Ollama)
python -m IA_Personal --full

# Demo del sistema
python -m IA_Personal --demo

# Benchmark de rendimiento
python -m IA_Personal --benchmark

# Benchmark GPU específico
python -m IA_Personal --gpu --benchmark

# Información del sistema
python -m IA_Personal --info
```

---

## 📁 Estructura del Proyecto

```
IA_Personal/
├── __init__.py              # Módulo principal
├── __main__.py              # Punto de entrada CLI
├── README.md                # Esta documentación
│
├── core/                    # Núcleo del sistema
│   ├── __init__.py
│   ├── ia_personal.py       # Sistema principal
│   ├── memory.py            # Memoria persistente
│   ├── context.py           # Contexto personal
│   ├── tokenizer.py         # Tokenizador inteligente
│   └── model.py             # Transformer ligero
│
├── integrations/            # Integraciones externas
│   ├── __init__.py
│   ├── adead_accelerator.py # Aceleración ADead-BIB
│   └── ollama_chat.py       # Integración Ollama LLM
│
├── ui/                      # Interfaces de usuario
│   ├── __init__.py
│   ├── chat.py              # Chat interactivo
│   └── cli.py               # Línea de comandos
│
└── data/                    # Datos persistentes (auto-generado)
    ├── memories.json        # Memorias guardadas
    ├── profile.json         # Perfil del usuario
    ├── adead_cache/         # Cache de binarios
    └── exports/             # Conversaciones exportadas
```

---

## 💬 Comandos de Chat

### Comandos Especiales
| Comando | Descripción |
|---------|-------------|
| `/ayuda` | Muestra ayuda |
| `/memoria` | Estadísticas de memoria |
| `/perfil` | Tu perfil personal |
| `/buscar [texto]` | Busca en memorias |
| `/exportar` | Exporta la conversación |
| `/stats` | Estadísticas del sistema |
| `/ollama` | Info de Ollama |
| `/limpiar` | Limpia la pantalla |
| `/salir` | Termina el chat |

### Frases de Aprendizaje
| Frase | Acción |
|-------|--------|
| "Me llamo [nombre]" | Aprende tu nombre |
| "Me gusta [algo]" | Aprende tus intereses |
| "Recuerda que [algo]" | Guarda información |

---

## 🔧 Uso Programático

### Uso Básico
```python
from IA_Personal import IAPersonal

ia = IAPersonal()
response = ia.chat("Hola, me llamo Carlos")
print(response)
```

### Con Aceleración ADead-BIB
```python
from IA_Personal import IAPersonalADead

ia = IAPersonalADead()
ia.chat("Hola")
ia.benchmark_acceleration()
```

### Con Ollama LLM
```python
from IA_Personal import IAPersonalOllama

ia = IAPersonalOllama(ollama_model="tinyllama")
response = ia.chat("Explica qué es la inteligencia artificial")
print(response)
```

### Configuración Personalizada
```python
from IA_Personal import IAPersonal, IAPersonalConfig

config = IAPersonalConfig(
    vocab_size=15000,
    embed_dim=128,
    num_layers=2,
    temperature=0.7,
    max_memory_items=1000,
)

ia = IAPersonal(config)
```

---

## 📊 Rendimiento

| Métrica | Valor |
|---------|-------|
| **RAM Total** | ~0.5-1.3 MB |
| **Vocabulario** | 289+ tokens |
| **Tiempo de Respuesta** | <50 ms |
| **Agregar 100 memorias** | ~100 ms |
| **Buscar 100 veces** | ~11 ms |

### 🎮 Rendimiento GPU (RTX 3060 12GB)

| Operación | CPU (ms) | GPU (ms) | Speedup |
|-----------|----------|----------|---------|
| MatMul 256x256 | 0.50 | 0.45 | 1.1x |
| MatMul 512x512 | 2.30 | 1.48 | 1.6x |
| MatMul 1024x1024 | 10.51 | 5.19 | **2.0x** |
| Attention 128x64 | 0.48 | 0.37 | 1.3x |
| Attention 256x128 | 0.85 | 0.46 | 1.9x |
| Attention 512x256 | 4.12 | 0.93 | **4.4x** |

**Speedup promedio: 2.0x** con GPU activada

---

## 🦙 Integración Ollama

Para respuestas avanzadas usando LLMs locales:

### Instalación
```powershell
# Instalar Ollama
winget install Ollama.Ollama

# Iniciar servidor
ollama serve

# Descargar modelo
ollama pull tinyllama
```

### Uso
```powershell
python -m IA_Personal --ollama
```

Las preguntas complejas (explica, qué es, escribe, etc.) se responden automáticamente con Ollama.

---

## ⚡ Integración ADead-BIB

Aceleración de operaciones matemáticas usando binarios nativos:

```powershell
python -m IA_Personal --adead
```

Operaciones aceleradas:
- Softmax
- GELU
- Layer Normalization
- Multiplicación de matrices

---

## 🔒 Privacidad

- **100% Local**: Todos los datos en tu máquina
- **Sin Internet**: No requiere conexión
- **Sin Telemetría**: No se envían datos
- **Datos Tuyos**: Puedes ver/editar/eliminar

---

## 📝 Licencia

Apache 2.0 - Libre para uso personal y comercial.

---

**¡Disfruta tu IA Personal!** 🚀
