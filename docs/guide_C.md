# ADead-BIB — Guía de Compilación C

## Requisitos

- **ADead-BIB** compilado (`cargo build --release`)
- Windows x86-64 (target nativo)
- No requiere GCC, Clang, ni LLVM

## Comandos Básicos

### Compilar un archivo C

```bash
adB cc hello.c                    # Genera hello.exe
adB cc hello.c -o output.exe      # Nombre de salida personalizado
adB hello.c                       # Auto-detecta lenguaje C por extensión
```

### Compilar y ejecutar

```bash
adB run hello.c                   # Compila + ejecuta automáticamente
```

### Modo paso a paso (Step Mode)

```bash
adB cc hello.c -step              # Muestra cada fase del compilador
adB step hello.c                  # Atajo para step mode
```

### Modo estricto

```bash
adB cc hello.c -Wstrict           # Bit-widths enforced, todo UB = error
```

## Estándares Soportados

| Estándar | Soporte |
|----------|---------|
| C99      | Completo |
| C11      | Completo |
| C17      | Parcial (compatible C11) |

## Características del Lenguaje C

### Tipos soportados

```c
// Enteros
char c = 'A';           // 1 byte [-128, 127]
short s = 1000;          // 2 bytes [-32768, 32767]
int i = 42;              // 4 bytes
long l = 100000L;        // 4/8 bytes
long long ll = 9999LL;   // 8 bytes

// Sin signo
unsigned char uc = 255;
unsigned int ui = 4000000000U;

// Flotantes
float f = 3.14f;
double d = 3.14159265;

// Punteros
int* ptr = &i;
void* vp = NULL;
```

### Estructuras y Enums

```c
struct Point {
    int x;
    int y;
};

enum Color { Red, Green, Blue };

typedef struct {
    float real;
    float imag;
} Complex;
```

### Control de flujo

```c
// if/else, for, while, do-while, switch
for (int i = 0; i < 10; i++) {
    printf("%d\n", i);
}

switch (color) {
    case Red:   printf("rojo\n"); break;
    case Green: printf("verde\n"); break;
    default:    printf("otro\n"); break;
}
```

### Punteros y Arrays

```c
int arr[10];
int* p = arr;
p[3] = 42;           // indexación
*p = 100;             // dereference
int** pp = &p;        // puntero a puntero

// Memoria dinámica
int* buf = (int*)malloc(100 * sizeof(int));
free(buf);
```

### Preprocesador

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdint.h>

#define MAX_SIZE 1024
#define SQUARE(x) ((x) * (x))

#ifdef DEBUG
    printf("debug mode\n");
#endif
```

## Headers Estándar Disponibles

| Header | Funciones principales |
|--------|----------------------|
| `<stdio.h>` | printf, scanf, fprintf, fopen, fclose |
| `<stdlib.h>` | malloc, free, atoi, abs, exit |
| `<string.h>` | strlen, strcpy, strcmp, memcpy, memset |
| `<math.h>` | sin, cos, sqrt, pow, fabs |
| `<stdint.h>` | int8_t, int16_t, int32_t, int64_t |
| `<stdbool.h>` | bool, true, false |
| `<limits.h>` | INT_MAX, INT_MIN, CHAR_MAX |
| `<errno.h>` | errno, ERANGE, EINVAL |
| `<assert.h>` | assert() |
| `<ctype.h>` | isalpha, isdigit, toupper, tolower |

## Detección de UB (Undefined Behavior)

ADead-BIB detecta automáticamente:

| Categoría | Ejemplo | Severidad |
|-----------|---------|-----------|
| División por cero | `int x = 5 / 0;` | Error |
| Shift overflow | `int x = 1 << 64;` | Warning |
| Null pointer deref | `int x = *NULL;` | Error |
| Array out of bounds | `arr[100]` (si detectable) | Warning |
| Format string mismatch | `printf("%d %d", 1)` | Warning |
| Loop infinito | `while(1){}` sin break | Warning |

### Modo Estricto (`-Wstrict`)

En modo estricto, se agregan:

- **Bit-width enforcement**: `char c = 256` → error
- **Narrowing conversions**: `(char)300` → error
- **Signed overflow**: `INT_MAX + 1` → error
- Todos los warnings → errors

## Pipeline de Compilación

```
.c → Preprocessor → Lexer → Parser → UB Detector → IR → ISA (x86-64) → PE (.exe)
```

### Fases en Step Mode

```
Phase 0: Preprocessor   — #include, #define expansion
Phase 1: Lexer          — Tokenización
Phase 2: Parser         — AST generation
Phase 3: Semantic        — Symbol resolution
Phase 4: UB Detection   — Análisis estático
Phase 5: IR Generation  — Lowering a Program IR
Phase 6: ISA Compiler   — x86-64 machine code
Phase 7: PE Builder     — Windows executable
Phase 8: Output         — Escritura del .exe
```

## Ejemplo Completo

```c
#include <stdio.h>
#include <stdlib.h>

struct Node {
    int value;
    struct Node* next;
};

int sum_list(struct Node* head) {
    int total = 0;
    while (head != NULL) {
        total += head->value;
        head = head->next;
    }
    return total;
}

int main() {
    printf("ADead-BIB C Compiler\n");
    printf("2 + 3 = %d\n", 2 + 3);
    return 0;
}
```

Compilar:

```bash
adB run example.c
```

---

*ADead-BIB v9.0 — C Compiler Complete*
*No LLVM. No GCC. Pure bits.*
