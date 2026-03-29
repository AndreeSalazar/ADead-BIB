# ADead-BIB — Arquitectura 2: Header Intelligence System
## Para Opus 4.6 — C y C++ únicamente

**Autor:** Eddi Andreé Salazar Matos  
**Fecha:** Marzo 2026  
**Prioridad:** C primero, C++ segundo  
**Objetivo:** Eliminar verbosidad sin sacrificar control ni performance

---

## El Problema que Resuelve

C y C++ son estrictamente tipados — correcto y necesario.
Pero la verbosidad mata productividad:

```c
// Sin Header Intelligence — verboso:
unsigned int GL_TRIANGLES   = 0x0004;
unsigned int GL_FLOAT       = 0x1406;
unsigned int GL_STATIC_DRAW = 0x88B4;
void* pfn_glDrawArrays      = nullptr;
void* pfn_glGenBuffers      = nullptr;
// ... 200 líneas de setup antes de dibujar un triángulo

// Con Header Intelligence — directo:
#include <openglz.h>
oglz_draw_cube();  // El compilador ya sabe todo lo anterior
```

El binario generado es IDÉNTICO en ambos casos.
La diferencia está en el tiempo del desarrollador.

---

## Principio Fundamental

```
Header Intelligence ≠ Muleta
Header Intelligence = Memoria del compilador

El compilador ADead-BIB absorbe el conocimiento
de la header en compile time.
El binario resultante no lleva overhead.
El developer no escribe boilerplate.
Todos ganan. Nadie pierde rendimiento.
```

---

## Arquitectura General

```
ADead-BIB Header Intelligence System
─────────────────────────────────────────────────────

[Código fuente C/C++]
    │
    │  #include <nombre.h>
    ▼
[Header Parser — ADead-BIB]
    │
    ├── Lee firmas de funciones
    ├── Lee anotaciones semánticas (@ub, @opt, @doc)
    ├── Lee implementaciones inline si existen
    ├── Registra en knowledge base del compilador
    └── Habilita Ctrl+Click con documentación completa
    │
    ▼
[Compilador ADead-BIB con knowledge base activa]
    │
    ├── Optimiza según hints de la header
    ├── Detecta UBs específicos del dominio
    ├── Genera código óptimo (AVX2 si aplica)
    └── Elimina código muerto automáticamente
    │
    ▼
[Binario final — igual de rápido, menos verbosidad]
```

---

## Estructura de Carpetas

```
ADead-BIB/
└── include/
    ├── c/                    ← Headers C (C99/C11)
    │   ├── core/
    │   │   ├── adead.h       ← Header maestro C
    │   │   ├── types.h       ← Tipos fundamentales
    │   │   ├── memory.h      ← malloc/free/memcpy mejorado
    │   │   ├── string.h      ← Operaciones de string
    │   │   ├── math.h        ← Matemática con AVX2
    │   │   ├── io.h          ← printf/scanf/file ops
    │   │   └── platform.h    ← Detección Win32/Linux/FastOS
    │   ├── gpu/
    │   │   ├── openglz.h     ← OpenGL 1.0→4.7 unificado
    │   │   ├── vulkan_bridge.h
    │   │   ├── dx12_bridge.h
    │   │   ├── cuda_bridge.h
    │   │   └── mantle_bridge.h
    │   ├── os/
    │   │   ├── win32.h       ← Win32 API sin SDK
    │   │   ├── fastos.h      ← FastOS syscalls
    │   │   └── posix.h       ← POSIX compatible
    │   └── security/
    │       └── bg.h          ← Binary Guardian hooks
    │
    └── cpp/                  ← Headers C++ (encima de C)
        ├── core/
        │   ├── adead.hpp     ← Header maestro C++
        │   ├── types.hpp     ← Tipos + templates básicos
        │   ├── memory.hpp    ← Smart pointers sin STL
        │   └── string.hpp    ← String class propia
        ├── gpu/
        │   └── openglz.hpp   ← OpenGLZ con RAII
        └── patterns/
            ├── singleton.hpp
            ├── observer.hpp
            └── pool.hpp
```

---

## Formato de Header Inteligente

### Anatomía de una header ADead-BIB:

```c
// ============================================================
// memory.h — ADead-BIB Memory Intelligence Header
// Autor: Eddi Andreé Salazar Matos
// Dominio: C99/C11
// ============================================================

#pragma once
#ifndef ADEAD_MEMORY_H
#define ADEAD_MEMORY_H

#include <types.h>

// ── Anotaciones semánticas ──────────────────────────────────
// @ub   = Undefined Behavior conocido — UB detector lo vigila
// @opt  = Hint de optimización para el compilador
// @doc  = Documentación inline (Ctrl+Click)
// @safe = Versión segura que detecta errores automáticamente
// @fast = Versión máximo rendimiento sin checks
// ────────────────────────────────────────────────────────────

// ── Tipos base que ADead-BIB ya conoce ──────────────────────
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;
typedef signed char        i8;
typedef signed short       i16;
typedef signed int         i32;
typedef signed long long   i64;
typedef float              f32;
typedef double             f64;
// ────────────────────────────────────────────────────────────

// ── Funciones con anotaciones completas ─────────────────────

/**
 * @doc  Allocate size bytes de memoria en el heap
 * @ub   Retorno sin verificar = potencial null deref
 * @ub   size=0 comportamiento implementation-defined
 * @opt  ADead-BIB alinea a 16 bytes para SSE automáticamente
 * @safe Usar adead_malloc_safe() para check automático
 */
void* adead_malloc(u64 size);

/**
 * @doc  Liberar memoria previamente allocada
 * @ub   ptr no allocado con adead_malloc = comportamiento indefinido
 * @ub   double free = crash o corrupción
 * @ub   ptr=NULL en C99 es válido (no-op)
 * @opt  ADead-BIB marca el slot como libre para reutilización
 */
void adead_free(void* ptr);

/**
 * @doc  Versión segura — verifica ptr automáticamente
 * @safe No hace double free, reporta si ptr es inválido
 * @opt  Mismo rendimiento que adead_free en release build
 */
void adead_free_safe(void** ptr);

/**
 * @doc  Copiar n bytes de src a dst
 * @ub   Regiones solapadas = usar adead_memmove()
 * @opt  Usa AVX2 (256-bit) automáticamente en Ryzen 5 5600X
 *       Copia 32 bytes por ciclo vs 8 bytes de memcpy clásico
 */
void* adead_memcpy(void* dst, const void* src, u64 n);

/**
 * @doc  Inicializar n bytes de ptr con valor val
 * @opt  Usa AVX2 para bloques > 32 bytes automáticamente
 * @opt  Desenrollado de loop en compile time si n es constante
 */
void* adead_memset(void* ptr, i32 val, u64 n);

// ── Macros de conveniencia — eliminan verbosidad ─────────────

// Allocar un struct tipado — sin casteo manual
#define ADEAD_NEW(type)         ((type*)adead_malloc(sizeof(type)))
#define ADEAD_NEW_ARRAY(type,n) ((type*)adead_malloc(sizeof(type)*(n)))
#define ADEAD_FREE(ptr)         adead_free_safe((void**)&(ptr))

// ── Versión aliases estándar — compatibilidad C99/C11 ────────
// El compilador redirige malloc/free a las versiones ADead
#define malloc(n)      adead_malloc(n)
#define free(p)        adead_free_safe((void**)&(p))
#define memcpy(d,s,n)  adead_memcpy(d,s,n)
#define memset(p,v,n)  adead_memset(p,v,n)

#endif // ADEAD_MEMORY_H
```

---

## Header Principal — adead.h (C) / adead.hpp (C++)

```c
// adead.h — Una sola include para todo
// #include <adead.h> reemplaza 90% de los includes habituales

#pragma once
#ifndef ADEAD_H
#define ADEAD_H

// Core siempre incluido
#include <c/core/types.h>
#include <c/core/memory.h>
#include <c/core/string.h>
#include <c/core/io.h>

// Platform detection automática
#include <c/core/platform.h>

// GPU — solo si se usa (dead code elimination)
#ifdef ADEAD_USE_OPENGL
    #include <c/gpu/openglz.h>
#endif

#ifdef ADEAD_USE_VULKAN
    #include <c/gpu/vulkan_bridge.h>
#endif

#ifdef ADEAD_USE_CUDA
    #include <c/gpu/cuda_bridge.h>
#endif

// OS target
#ifdef ADEAD_TARGET_FASTOS
    #include <c/os/fastos.h>
#elif defined(ADEAD_TARGET_WIN32)
    #include <c/os/win32.h>
#elif defined(ADEAD_TARGET_POSIX)
    #include <c/os/posix.h>
#endif

// Security — Binary Guardian
#ifdef ADEAD_USE_BG
    #include <c/security/bg.h>
#endif

#endif // ADEAD_H
```

---

## openglz.h — El Header Más Importante

```c
// openglz.h — OpenGL 1.0→4.7 + backends unificados
// Sin GLFW. Sin SDL. Sin dependencias.
// ADead-BIB sabe todo lo necesario en compile time.

#pragma once
#ifndef ADEAD_OPENGLZ_H
#define ADEAD_OPENGLZ_H

#include <c/core/types.h>

// ── Constantes GL — el compilador las conoce todas ──────────
// No necesitas declararlas en tu código
// Ctrl+Click en cualquiera → documentación completa

// GL 1.0 core
#define GL_TRIANGLES         0x0004
#define GL_QUADS             0x0007
#define GL_COLOR_BUFFER_BIT  0x00004000
#define GL_DEPTH_BUFFER_BIT  0x00000100
#define GL_DEPTH_TEST        0x0B71
#define GL_LEQUAL            0x0203

// GL 2.0 — Shaders
#define GL_VERTEX_SHADER     0x8B31
#define GL_FRAGMENT_SHADER   0x8B30
#define GL_COMPILE_STATUS    0x8B81
#define GL_LINK_STATUS       0x8B82

// GL 3.0 — VAO/VBO
#define GL_ARRAY_BUFFER      0x8892
#define GL_STATIC_DRAW       0x88B4
#define GL_FLOAT             0x1406

// ... GL 4.0→4.7 — ADead-BIB los conoce todos

// ── API de alto nivel — OpenGLZ ─────────────────────────────

typedef struct OGLZContext  OGLZContext;
typedef struct OGLZWindow   OGLZWindow;
typedef struct OGLZShader   OGLZShader;
typedef struct OGLZMesh     OGLZMesh;
typedef struct OGLZTexture  OGLZTexture;

/**
 * @doc  Crear ventana con contexto OpenGL
 * @doc  Sin GLFW. Sin SDL. Win32/FastOS/Linux automático.
 * @opt  ADead-BIB detecta la plataforma en compile time
 * @ub   width/height <= 0 = comportamiento indefinido
 */
OGLZWindow* oglz_create_window(i32 width, i32 height, const char* title);

/**
 * @doc  Cargar GL 2.0+ functions automáticamente
 * @doc  wglGetProcAddress/glXGetProcAddress según plataforma
 * @opt  Las funciones se cargan una vez y se cachean
 */
i32 oglz_init_gl20(OGLZWindow* win);

/**
 * @doc  Compilar shader desde GLSL string
 * @doc  Soporta GLSL 110→460 automáticamente
 * @ub   src=NULL = crash en runtime
 * @safe ADead-BIB verifica compilación y reporta errores
 */
OGLZShader* oglz_shader_from_glsl(const char* vert_src, const char* frag_src);

/**
 * @doc  Compilar shader desde HLSL — traducción automática
 * @opt  ADead-BIB traduce HLSL→GLSL→SPIR-V según backend
 */
OGLZShader* oglz_shader_from_hlsl(const char* vert_src, const char* frag_src);

/**
 * @doc  Crear mesh de cubo unitario con colores por cara
 * @opt  VAO/VBO generados una vez, reutilizados infinito
 */
OGLZMesh* oglz_mesh_cube(void);

/**
 * @doc  Crear mesh desde array de vértices custom
 * @ub   verts=NULL o count=0 = comportamiento indefinido
 * @opt  ADead-BIB alinea el buffer a 32 bytes para AVX2
 */
OGLZMesh* oglz_mesh_from_verts(const f32* verts, u32 count, u32 stride);

/**
 * @doc  Limpiar framebuffer
 * @opt  Combina glClear(COLOR|DEPTH) automáticamente
 */
void oglz_clear(void);

/**
 * @doc  Dibujar mesh con shader
 * @opt  Bind VAO + UseProgram + DrawArrays en una llamada
 */
void oglz_draw(OGLZMesh* mesh, OGLZShader* shader);

/**
 * @doc  Swap buffers + poll de eventos
 * @doc  Retorna 0 cuando la ventana se cierra
 */
i32 oglz_running(OGLZWindow* win);

/**
 * @doc  Cleanup completo — libera todos los recursos GL
 */
void oglz_destroy(OGLZWindow* win);

// ── Macros de loop estándar ──────────────────────────────────

#define OGLZ_MAIN_LOOP(win) while(oglz_running(win))

// ── Shaders built-in — listos para usar ─────────────────────

// Shader de color plano — sin iluminación
extern const char* OGLZ_SHADER_FLAT_VERT;
extern const char* OGLZ_SHADER_FLAT_FRAG;

// Shader Blinn-Phong — iluminación estándar
extern const char* OGLZ_SHADER_PHONG_VERT;
extern const char* OGLZ_SHADER_PHONG_FRAG;

// Shader PBR básico — Physically Based Rendering
extern const char* OGLZ_SHADER_PBR_VERT;
extern const char* OGLZ_SHADER_PBR_FRAG;

#endif // ADEAD_OPENGLZ_H
```

---

## Ejemplo de Uso — Antes vs Después

### ANTES — Sin Header Intelligence (200+ líneas):

```c
// main.c — verboso, sin headers inteligentes
extern "C" {
    void* wglCreateContext(void* hdc);
    void* wglGetProcAddress(const char* name);
    // ... 30 declaraciones más
}
unsigned int GL_TRIANGLES = 0x0004;
// ... 20 constantes más
void* pfn_glCreateShader = nullptr;
// ... 15 punteros más

int main() {
    // 50 líneas de setup de ventana Win32
    // 30 líneas de setup OpenGL context
    // 20 líneas de cargar funciones GL 2.0+
    // 40 líneas de setup VAO/VBO
    // FINALMENTE tu código de verdad
}
```

### DESPUÉS — Con Header Intelligence (20 líneas):

```c
// main.c — con headers inteligentes ADead-BIB
#define ADEAD_USE_OPENGL
#include <adead.h>

// Shaders inline
const char* VERT = "#version 330 core\n"
    "layout(location=0) in vec3 aPos;\n"
    "layout(location=1) in vec3 aColor;\n"
    "out vec3 vColor;\n"
    "uniform mat4 uMVP;\n"
    "void main(){ gl_Position=uMVP*vec4(aPos,1.0); vColor=aColor; }\n";

const char* FRAG = "#version 330 core\n"
    "in vec3 vColor; out vec4 FragColor;\n"
    "void main(){ FragColor=vec4(vColor,1.0); }\n";

int main() {
    OGLZWindow* win    = oglz_create_window(820, 640, "FastOS OpenGLZ");
    OGLZShader* shader = oglz_shader_from_glsl(VERT, FRAG);
    OGLZMesh*   cube   = oglz_mesh_cube();

    OGLZ_MAIN_LOOP(win) {
        oglz_clear();
        oglz_draw(cube, shader);
    }

    oglz_destroy(win);
    return 0;
}
// Binario generado: mismo tamaño, mismo rendimiento ✅
```

---

## UB Detector por Header — Específico por Dominio

```
Cuando incluyes <memory.h>:
  UB detector activa:
  → use-after-free detection
  → double-free detection  
  → null deref check
  → buffer overflow estático

Cuando incluyes <openglz.h>:
  UB detector activa además:
  → shader compile sin verificar
  → draw sin VAO bound
  → uniform location -1 sin check
  → contexto GL no activo

Cuando incluyes <fastos.h>:
  UB detector activa además:
  → syscall desde wrong ring
  → MMIO access sin mmap previo
  → interrupt handler stack overflow
```

---

## --step Mode con Headers

```
cargo run --bin adb -- c main.c --step

[STEP 1/7] Preprocessor
  #include <adead.h> → expandido
  #include <openglz.h> → 847 símbolos cargados
  tiempo: 0.1ms ✅

[STEP 2/7] Lexer
  tokens: 312 (vs 2,847 sin headers inteligentes)
  reducción: 89% menos tokens que escribir ✅

[STEP 3/7] Parser
  AST nodes: 127
  funciones detectadas: 4 (main + 3 oglz_*)
  tiempo: 0.4ms ✅

[STEP 4/7] Header Knowledge Base
  openglz.h: 847 símbolos conocidos
  memory.h:  23 funciones conocidas
  UBs vigilados: 34 patrones activos
  tiempo: 0.0ms (ya en memoria) ✅

[STEP 5/7] UB Detector
  oglz_shader_from_glsl: VERT y FRAG no son NULL ✅
  oglz_mesh_cube: sin parámetros — seguro ✅
  oglz_draw: dentro de OGLZ_MAIN_LOOP — contexto válido ✅
  Sin UBs detectados ✅

[STEP 6/7] ISA Compiler + Optimizer
  AVX2 detectado: Ryzen 5 5600X ✅
  adead_memcpy → instrucciones VMOVDQU 256-bit
  oglz_clear → glClear inline expandido
  dead code eliminado: 0 bytes de overhead

[STEP 7/7] PE Builder (Windows) / ELF Builder (FastOS)
  .text: 8,192 bytes
  .data: 4,096 bytes  
  .rdata: 3,840 bytes (strings + constantes GL)
  total: ~16KB ✅

→ main.exe generado sin errores
```

---

## Sistema de Expansión — Para Agregar Más

La arquitectura permite expandir sin romper nada:

```c
// Cualquier developer puede agregar su propia header:
// mylib.h

#pragma once
#ifndef MYLIB_H
#define MYLIB_H

#include <adead.h>  // Base ADead-BIB

// Anotaciones semánticas propias
/**
 * @doc  Mi función custom
 * @ub   param <= 0 = comportamiento indefinido
 * @opt  Usar siempre con buffer alineado a 32 bytes
 */
void mylib_do_something(i32 param);

// Macros de conveniencia propias
#define MYLIB_CALL(x) mylib_do_something(x)

#endif // MYLIB_H

// El compilador ADead-BIB:
// → Lee las anotaciones @doc, @ub, @opt
// → Las integra al UB detector
// → Las muestra en Ctrl+Click
// → Las considera en optimizaciones
// ✅ Sin modificar el compilador core
```

---

## Instrucciones para Opus 4.6

### Orden de implementación — PRIORIDAD C:

**Fase 1 — Core C (primero):**
1. `include/c/core/types.h` — tipos u8→u64, i8→i64, f32, f64
2. `include/c/core/memory.h` — adead_malloc/free/memcpy/memset
3. `include/c/core/string.h` — adead_strlen/strcpy/strcmp/strcat
4. `include/c/core/io.h` — adead_printf/scanf wrappers
5. `include/c/core/platform.h` — detección Win32/FastOS/Linux
6. `include/c/core/adead.h` — header maestro que incluye todo

**Fase 2 — GPU C:**
7. `include/c/gpu/openglz.h` — API OpenGLZ completa
8. Implementación de `oglz_create_window()` para Win32
9. Implementación de `oglz_init_gl20()` con loader
10. Implementación de `oglz_shader_from_glsl()`
11. Implementación de `oglz_mesh_cube()` y `oglz_draw()`

**Fase 3 — C++ encima de C:**
12. `include/cpp/core/adead.hpp` — wrappers C++ de todo lo anterior
13. `include/cpp/core/memory.hpp` — RAII wrappers
14. `include/cpp/gpu/openglz.hpp` — OGLZContext con RAII

### Reglas críticas para Opus:

```
1. C primero — siempre
   C++ depende de que C esté completo y testeado

2. Cada header tiene sus tests en tests/c/
   Zero warnings, zero errors antes de continuar

3. Las anotaciones @doc, @ub, @opt son obligatorias
   en toda función pública

4. Los macros de conveniencia NO pueden tener overhead
   El binario generado debe ser idéntico con o sin ellos

5. adead.h es el único include que necesita el usuario final
   Todo lo demás se activa con #define ADEAD_USE_*

6. Compatibilidad C99/C11 estricta en headers C
   Sin extensiones GNU, sin MSVC específico

7. --step mode debe mostrar qué headers están activas
   y cuántos símbolos cargó cada una
```

---

## El Resultado Final

```
Usuario escribe:
  #define ADEAD_USE_OPENGL
  #include <adead.h>
  
  int main() {
      OGLZWindow* w = oglz_create_window(800, 600, "Mi App");
      OGLZMesh*   m = oglz_mesh_cube();
      OGLZShader* s = oglz_shader_from_glsl(OGLZ_SHADER_PHONG_VERT,
                                             OGLZ_SHADER_PHONG_FRAG);
      OGLZ_MAIN_LOOP(w) {
          oglz_clear();
          oglz_draw(m, s);
      }
      oglz_destroy(w);
  }

ADead-BIB genera:
  → Ventana Win32 nativa ✅
  → Contexto OpenGL completo ✅  
  → Shaders Blinn-Phong compilados ✅
  → Cubo girando con iluminación ✅
  → Sin GLFW. Sin SDL. Sin dependencias ✅
  → ~16KB binario final ✅
  → Corre en Windows, FastOS, Linux ✅

Sin verbosidad.
Sin muletas.
Sin caja negra.
ADead-BIB ya sabe. 🦈💀🔥
```

---

*Arquitectura_2.md — ADead-BIB Header Intelligence System*  
*Eddi Andreé Salazar Matos — Marzo 2026*  
*"El compilador ya sabe. Tú solo escribes lo importante."*
