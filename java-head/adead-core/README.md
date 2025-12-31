# Java-Head / ADead-Core

> **ADead-BIB + Java** = Media Platform Experiment
> 
> Experimentación para crear una plataforma de media estilo Netflix
> usando Java como frontend/backend y ADead-BIB para procesamiento de alto rendimiento.

---

## ⚠️ CONTRATO ADead-BIB (OBLIGATORIO LEER)

```
╔══════════════════════════════════════════════════════════════╗
║                 ADead-BIB KERNEL CONTRACT                    ║
╠══════════════════════════════════════════════════════════════╣
║                                                              ║
║  Los kernels ADead-BIB:                                      ║
║                                                              ║
║    ✗ NO se optimizan                                         ║
║    ✗ NO se eliminan                                          ║
║    ✗ NO se reordenan                                         ║
║    ✓ Hacen EXACTAMENTE lo escrito                            ║
║                                                              ║
║  Esto existe porque:                                         ║
║                                                              ║
║    1. Predecibilidad > Velocidad máxima                      ║
║    2. Jitter bajo > Throughput alto                          ║
║    3. Determinismo > Optimización especulativa               ║
║                                                              ║
║  Si escribes:                                                ║
║    emit![0x48, 0x31, 0xC0]  // xor rax, rax                  ║
║                                                              ║
║  Obtienes:                                                   ║
║    0x48 0x31 0xC0                                            ║
║                                                              ║
║  Sin excepciones. Sin "optimizaciones inteligentes".         ║
║  Binary Is Binary.                                           ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
```

### ¿Por qué importa para Media/Streaming?

| Problema con JIT | Solución ADead-BIB |
|------------------|-------------------|
| GC pauses impredecibles | Sin GC, memoria explícita |
| JIT warmup variable | Código compilado desde el inicio |
| Deoptimización sorpresa | Sin optimización especulativa |
| Jitter alto en p99/worst | Jitter mínimo y predecible |

---

## 📊 Métricas Visibles

El sistema incluye métricas obligatorias:

```
╔══════════════════════════════════════════════════════════════╗
║  METRICS: Java + ADead-BIB                                   ║
╠══════════════════════════════════════════════════════════════╣
║  Total Frames: 1000                                          ║
╠══════════════════════════════════════════════════════════════╣
║  CALL LATENCY (ms)                                           ║
║    avg:    0.012 | p50:    0.010 | p95:    0.015             ║
║    p99:    0.020 | worst:    0.025                           ║
╠══════════════════════════════════════════════════════════════╣
║  EXECUTION TIME (ms)                                         ║
║    avg:    2.100 | p50:    2.050 | p95:    2.200             ║
║    p99:    2.300 | worst:    2.500                           ║
╠══════════════════════════════════════════════════════════════╣
║  JITTER (ms)                                                 ║
║    avg:    0.050 | p95:    0.100 | worst:    0.150           ║
╚══════════════════════════════════════════════════════════════╝
```

### Ejecutar Benchmark

```bash
# Compilar
cd java-head/adead-core
mvn package -DskipTests

# Ejecutar benchmark
java -cp target/classes com.adead.media.benchmark.BenchmarkRunner

# Con más frames
java -cp target/classes com.adead.media.benchmark.BenchmarkRunner --frames 500 --warmup 100
```

---

## 🎯 Objetivo

Crear una base para una plataforma de streaming de media que combine:

- **Java**: Backend robusto, APIs REST, gestión de usuarios
- **ADead-BIB**: Procesamiento de video/audio de alto rendimiento (codecs, transcoding)

---

## 🏗️ Arquitectura Propuesta

```
┌─────────────────────────────────────────────────────────────────┐
│                    Media Platform (Netflix-style)               │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                    JAVA LAYER                            │   │
│  │                                                          │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │   │
│  │  │   REST API   │  │   Auth/Users │  │   Catalog    │   │   │
│  │  │  (Spring)    │  │   (JWT)      │  │   (DB)       │   │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘   │   │
│  │                                                          │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │   │
│  │  │   Streaming  │  │   CDN        │  │   Analytics  │   │   │
│  │  │   Service    │  │   Manager    │  │   Service    │   │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘   │   │
│  └─────────────────────────────────────────────────────────┘   │
│                              │                                  │
│                              ▼                                  │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                  ADead-BIB LAYER (JNI/FFI)               │   │
│  │                                                          │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │   │
│  │  │   Video      │  │   Audio      │  │   GPU        │   │   │
│  │  │   Transcoder │  │   Processor  │  │   Accelerate │   │   │
│  │  │   (CPU)      │  │   (CPU)      │  │   (HEX)      │   │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘   │   │
│  │                                                          │   │
│  │  Binario directo x86-64 + GPU HEX para máximo rendimiento│   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 📁 Estructura del Proyecto

```
java-head/
└── adead-core/
    ├── README.md                    # Este archivo
    ├── pom.xml                      # Maven config
    │
    ├── src/
    │   ├── main/
    │   │   ├── java/
    │   │   │   └── com/adead/media/
    │   │   │       ├── Application.java       # Entry point
    │   │   │       ├── api/                   # REST Controllers
    │   │   │       ├── service/               # Business logic
    │   │   │       ├── model/                 # Entities
    │   │   │       ├── repository/            # Data access
    │   │   │       └── native/                # JNI bridge to ADead-BIB
    │   │   │
    │   │   └── resources/
    │   │       ├── application.yml
    │   │       └── native/                    # ADead-BIB binaries
    │   │
    │   └── test/
    │       └── java/
    │           └── com/adead/media/
    │
    └── native/                      # ADead-BIB source for JNI
        ├── transcoder.adB           # Video transcoding
        ├── audio_processor.adB      # Audio processing
        └── gpu_accelerate.adB       # GPU acceleration
```

---

## 🔗 Integración Java ↔ ADead-BIB

### Opción 1: JNI (Java Native Interface)
```java
public class ADeadTranscoder {
    static {
        System.loadLibrary("adead_transcoder");
    }
    
    // Métodos nativos implementados en ADead-BIB
    public native byte[] transcodeVideo(byte[] input, String format);
    public native byte[] processAudio(byte[] input, int sampleRate);
    public native void gpuAccelerate(long bufferPtr, int size);
}
```

### Opción 2: Process/CLI
```java
public class ADeadCLI {
    public void transcode(Path input, Path output) {
        ProcessBuilder pb = new ProcessBuilder(
            "adeadc", "run", "transcoder.adB",
            "--input", input.toString(),
            "--output", output.toString()
        );
        pb.start().waitFor();
    }
}
```

### Opción 3: Socket/IPC
```java
// Java envía comandos, ADead-BIB procesa
Socket socket = new Socket("localhost", 9999);
socket.getOutputStream().write(videoData);
byte[] processed = socket.getInputStream().readAllBytes();
```

---

## 🎬 Casos de Uso Media

### 1. Video Transcoding
```
Input: video.mp4 (1080p, H.264)
       ↓
ADead-BIB (CPU/GPU):
  - Decode H.264 (bytes directos)
  - Resize/Scale
  - Encode H.265/VP9/AV1
       ↓
Output: video_720p.mp4, video_480p.mp4, video_360p.mp4
```

### 2. Adaptive Bitrate Streaming
```
Master video
    ↓
ADead-BIB genera múltiples calidades:
  - 4K (15 Mbps)
  - 1080p (8 Mbps)
  - 720p (5 Mbps)
  - 480p (2.5 Mbps)
  - 360p (1 Mbps)
    ↓
HLS/DASH manifest
```

### 3. Audio Processing
```
Input: audio.wav
       ↓
ADead-BIB:
  - Normalización
  - Compresión dinámica
  - Encode AAC/Opus
       ↓
Output: audio.aac (múltiples bitrates)
```

---

## 🚀 Próximos Pasos

1. **[ ] Crear pom.xml** con dependencias Spring Boot
2. **[ ] Implementar REST API básica** para catálogo
3. **[ ] Crear JNI bridge** a ADead-BIB
4. **[ ] Implementar transcoder.adB** básico
5. **[ ] Prueba de concepto** video transcoding

---

## 💡 ¿Por qué ADead-BIB para Media?

| Aspecto | Java Solo | Java + ADead-BIB |
|---------|-----------|------------------|
| **Transcoding** | FFmpeg (proceso externo) | Binario directo, sin overhead |
| **GPU** | Complejo (JCuda, etc.) | HEX directo a GPU |
| **Latencia** | Alta (JVM + GC) | Mínima (bytes directos) |
| **Tamaño** | Grande (JVM) | Pequeño (~KB) |
| **Control** | Limitado | Total (bytes) |

---

## 📝 Notas

Este es un proyecto experimental para explorar la integración de:
- **Java** para lógica de negocio y APIs
- **ADead-BIB** para procesamiento de alto rendimiento

La idea es que Java maneje todo lo que hace bien (web, APIs, DB) y ADead-BIB
maneje lo que requiere máximo rendimiento (video, audio, GPU).

---

**ADead-BIB + Java = Media Platform**
**Lo mejor de ambos mundos**
