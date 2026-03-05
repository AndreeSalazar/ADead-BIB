// ============================================================
// Canon C99 — §6.10 Preprocesador
// ============================================================
// Intención: El preprocesador opera ANTES de la compilación.
// Reemplaza texto, incluye archivos, y habilita compilación
// condicional. No genera code — transforma source text.
//
// C99 §6.10: Preprocessing directives
//   #define  — Definir macro o constante
//   #undef   — Eliminar definición
//   #ifdef   — Si está definido
//   #ifndef  — Si NO está definido
//   #if      — Expresión condicional
//   #elif    — Else if
//   #else    — Else
//   #endif   — Fin de condicional
// ============================================================

#include <stdio.h>

// --- Constantes con #define ---
#define MAX_SIZE 100
#define PI 3
#define VERSION_MAJOR 4
#define VERSION_MINOR 0

// --- Macro funcional ---
#define SQUARE(x) ((x) * (x))
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define ABS(x) ((x) < 0 ? -(x) : (x))

// --- Compilación condicional ---
#define PLATFORM_WINDOWS 1

#ifdef PLATFORM_WINDOWS
#define PATH_SEP '\\'
#else
#define PATH_SEP '/'
#endif

// --- Feature flags ---
#define FEATURE_LOGGING 1
#define FEATURE_DEBUG 0

// --- Guard pattern ---
#ifndef NULL
#define NULL 0
#endif

int main() {
    printf("=== Canon C99: Preprocesador ===\n\n");

    // --- Constantes ---
    printf("MAX_SIZE = %d\n", MAX_SIZE);
    printf("PI = %d\n", PI);
    printf("VERSION = %d.%d\n", VERSION_MAJOR, VERSION_MINOR);

    // --- Macros funcionales ---
    printf("\nMacros:\n");
    printf("  SQUARE(5) = %d\n", SQUARE(5));
    printf("  SQUARE(3+2) = %d\n", SQUARE(3 + 2));
    printf("  MAX(10, 20) = %d\n", MAX(10, 20));
    printf("  MIN(10, 20) = %d\n", MIN(10, 20));
    printf("  ABS(-42) = %d\n", ABS(-42));
    printf("  ABS(42) = %d\n", ABS(42));

    // --- Compilación condicional ---
    printf("\nPlatform:\n");
#ifdef PLATFORM_WINDOWS
    printf("  Windows detected\n");
    printf("  PATH_SEP = '\\'\n");
#else
    printf("  Non-Windows detected\n");
    printf("  PATH_SEP = '/'\n");
#endif

    // --- Feature flags ---
    printf("\nFeatures:\n");
#if FEATURE_LOGGING
    printf("  Logging: enabled\n");
#else
    printf("  Logging: disabled\n");
#endif

#if FEATURE_DEBUG
    printf("  Debug: enabled\n");
#else
    printf("  Debug: disabled\n");
#endif

    // --- #ifndef guard ---
    int *ptr = NULL;
    printf("\nNULL = %d\n", ptr == NULL);

    // --- Verificación ---
    int pass = 0;
    int total = 0;

    total++; if (MAX_SIZE == 100)        { pass++; } else { printf("FAIL: MAX_SIZE\n"); }
    total++; if (SQUARE(5) == 25)        { pass++; } else { printf("FAIL: SQUARE\n"); }
    total++; if (MAX(10, 20) == 20)      { pass++; } else { printf("FAIL: MAX\n"); }
    total++; if (MIN(10, 20) == 10)      { pass++; } else { printf("FAIL: MIN\n"); }
    total++; if (ABS(-42) == 42)         { pass++; } else { printf("FAIL: ABS\n"); }
    total++; if (SQUARE(3 + 2) == 25)    { pass++; } else { printf("FAIL: SQUARE paren\n"); }

    printf("\n%d/%d passed\n", pass, total);
    return 0;
}
