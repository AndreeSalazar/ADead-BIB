/**
 * ADead-BIB FFI C/C++ Header
 * ==========================
 * Author: Eddi Andreé Salazar Matos
 * Email: eddi.salazar.dev@gmail.com
 * Made with ❤️ in Peru 🇵🇪
 * 
 * ADead-BIB como cabeza principal ABI para C/C++.
 * Header para usar funciones exportadas de ADead-BIB.
 */

#ifndef ADEAD_BIB_H
#define ADEAD_BIB_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================
 * TIPOS BÁSICOS
 * ============================================ */

typedef int64_t adead_int;
typedef double adead_float;
typedef char adead_char;
typedef void* adead_ptr;

/* ============================================
 * FUNCIONES MATEMÁTICAS EXPORTADAS
 * ============================================ */

/* Suma dos enteros */
adead_int adead_sum(adead_int a, adead_int b);

/* Multiplica dos enteros */
adead_int adead_mul(adead_int a, adead_int b);

/* Retorna el máximo */
adead_int adead_max(adead_int a, adead_int b);

/* Retorna el mínimo */
adead_int adead_min(adead_int a, adead_int b);

/* Valor absoluto */
adead_int adead_abs(adead_int x);

/* Factorial */
adead_int adead_factorial(adead_int n);

/* Fibonacci */
adead_int adead_fib(adead_int n);

/* ============================================
 * FUNCIONES DE MEMORIA
 * ============================================ */

/* Tamaño de int en bytes */
adead_int adead_sizeof_int(void);

/* ============================================
 * FUNCIONES DE UTILIDAD
 * ============================================ */

/* Versión del ABI (320 = v3.2.0) */
adead_int adead_version(void);

/* ============================================
 * MACROS DE CONVENIENCIA
 * ============================================ */

#define ADEAD_VERSION 320
#define ADEAD_VERSION_MAJOR 3
#define ADEAD_VERSION_MINOR 2
#define ADEAD_VERSION_PATCH 0

/* Calling convention */
#ifdef _WIN32
    #define ADEAD_CALL __cdecl
#else
    #define ADEAD_CALL
#endif

/* Export/Import */
#ifdef ADEAD_BUILD_DLL
    #ifdef _WIN32
        #define ADEAD_API __declspec(dllexport)
    #else
        #define ADEAD_API __attribute__((visibility("default")))
    #endif
#else
    #ifdef _WIN32
        #define ADEAD_API __declspec(dllimport)
    #else
        #define ADEAD_API
    #endif
#endif

#ifdef __cplusplus
}
#endif

#endif /* ADEAD_BIB_H */
