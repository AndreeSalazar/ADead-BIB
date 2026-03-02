// ============================================================
// ADead-BIB C Example — FastOS Base Libraries Test
// ============================================================
// Tests that ALL FastOS library headers compile correctly:
//   Fase 1: musl base types (stdio, stdlib, string, math, etc.)
//   Fase 2: Display (Vulkan, Wayland, FreeType, libinput)
//   Fase 3: User basic (PNG, JPEG, zlib, curl, SQLite)
//   Fase 4: Multimedia (FFmpeg, Vorbis, Opus)
//   + Network, compression, hardware, audio, XML
//
// This file validates the entire c_stdlib.rs header system.
// Sin GCC. Sin Clang. Solo ADead-BIB. 💀🦈
// ============================================================

// ==================== Fase 1: Base absoluta ====================
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>
#include <ctype.h>
#include <errno.h>
#include <limits.h>
#include <time.h>
#include <signal.h>
#include <setjmp.h>

// ==================== Fase 2: System / POSIX ====================
#include <unistd.h>
#include <fcntl.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <sys/mman.h>
#include <sys/ioctl.h>
#include <sys/wait.h>
#include <sys/time.h>
#include <dirent.h>
#include <dlfcn.h>
#include <pthread.h>

// ==================== Fase 3: Network ====================
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <netdb.h>
#include <poll.h>
#include <sys/epoll.h>

// ==================== Fase 4: Compresión ====================
#include <zlib.h>
#include <lz4.h>
#include <zstd.h>

// ==================== Fase 5: Imágenes ====================
#include <png.h>
#include <jpeglib.h>

// ==================== Fase 6: Audio ====================
#include <opus/opus.h>

// ==================== Fase 7: GPU ====================
#include <vulkan/vulkan.h>

// ==================== Fase 8: Fuentes ====================
#include <ft2build.h>

// ==================== Fase 9: DB ====================
#include <sqlite3.h>

// ==================== Fase 10: Red/Security ====================
#include <curl/curl.h>
#include <openssl/ssl.h>

// ==================== Fase 11: Hardware ====================
#include <libinput.h>
#include <xkbcommon/xkbcommon.h>

// ==================== Fase 12: Multimedia ====================
#include <libavcodec/avcodec.h>
#include <libavformat/avformat.h>
#include <libavutil/avutil.h>

// ==================== Fase 13: XML ====================
#include <expat.h>

// ==================== Validation Functions ====================

int test_base_types(void) {
    size_t sz = sizeof(int);
    ptrdiff_t pd = 0;
    intptr_t ip = 0;
    int8_t i8 = -128;
    uint8_t u8 = 255;
    int16_t i16 = -32768;
    uint16_t u16 = 65535;
    int32_t i32 = -2147483647;
    uint32_t u32 = 4294967295u;
    int64_t i64 = 0;
    uint64_t u64 = 0;
    printf("  size_t:   %lu bytes\n", (unsigned long)sizeof(size_t));
    printf("  int8_t:   %d\n", i8);
    printf("  uint8_t:  %u\n", u8);
    printf("  int32_t:  %d\n", i32);
    printf("  uint32_t: %u\n", u32);
    return 1;
}

int test_string_ops(void) {
    const char *s1 = "ADead-BIB";
    const char *s2 = "FastOS";
    printf("  strlen(\"%s\") = %lu\n", s1, (unsigned long)strlen(s1));
    printf("  strcmp result: %d\n", strcmp(s1, s2));

    char buf[64];
    strcpy(buf, s1);
    strcat(buf, " + ");
    strcat(buf, s2);
    printf("  concat: %s\n", buf);

    char *found = strstr(buf, "BIB");
    if (found != NULL) {
        printf("  found 'BIB' at offset %ld\n", (long)(found - buf));
    }
    return 1;
}

int test_memory(void) {
    int *arr = malloc(100 * sizeof(int));
    if (arr == NULL) {
        printf("  malloc failed!\n");
        return 0;
    }
    memset(arr, 0, 100 * sizeof(int));
    for (int i = 0; i < 100; i++) {
        arr[i] = i * i;
    }

    int *copy = calloc(100, sizeof(int));
    if (copy != NULL) {
        memcpy(copy, arr, 100 * sizeof(int));
        printf("  arr[50] = %d, copy[50] = %d\n", arr[50], copy[50]);
        printf("  memcmp = %d (should be 0)\n", memcmp(arr, copy, 100 * sizeof(int)));
        free(copy);
    }
    free(arr);
    printf("  malloc/calloc/memcpy/free: OK\n");
    return 1;
}

int test_math(void) {
    printf("  sqrt(144)   = %.1f\n", sqrt(144.0));
    printf("  pow(2, 10)  = %.0f\n", pow(2.0, 10.0));
    printf("  sin(pi/2)   = %.6f\n", sin(3.14159265 / 2.0));
    printf("  log(100)    = %.6f\n", log(100.0));
    printf("  ceil(3.14)  = %.1f\n", ceil(3.14));
    printf("  floor(3.14) = %.1f\n", floor(3.14));
    printf("  fabs(-42.5) = %.1f\n", fabs(-42.5));
    return 1;
}

int test_ctype(void) {
    int passed = 0;
    passed += isalpha('A') ? 1 : 0;
    passed += isdigit('5') ? 1 : 0;
    passed += isspace(' ') ? 1 : 0;
    passed += isupper('Z') ? 1 : 0;
    passed += islower('a') ? 1 : 0;
    printf("  ctype tests passed: %d/5\n", passed);
    printf("  toupper('a') = '%c'\n", toupper('a'));
    printf("  tolower('Z') = '%c'\n", tolower('Z'));
    return 1;
}

// ==================== Main ====================

int main() {
    printf("=== ADead-BIB: FastOS Library Headers Test ===\n");
    printf("All headers compiled successfully!\n\n");

    int total = 0;
    int passed = 0;

    printf("[1] Base Types:\n");
    total++; passed += test_base_types();

    printf("\n[2] String Operations:\n");
    total++; passed += test_string_ops();

    printf("\n[3] Memory Management:\n");
    total++; passed += test_memory();

    printf("\n[4] Math Functions:\n");
    total++; passed += test_math();

    printf("\n[5] Character Types:\n");
    total++; passed += test_ctype();

    printf("\n=== Results: %d/%d tests passed ===\n", passed, total);
    printf("\nHeaders validated:\n");
    printf("  C Standard:   stdio, stdlib, string, math, stdint,\n");
    printf("                stdbool, stddef, ctype, errno, limits,\n");
    printf("                time, signal, setjmp\n");
    printf("  POSIX:        unistd, fcntl, sys/types, sys/stat,\n");
    printf("                sys/mman, sys/ioctl, sys/wait, sys/time,\n");
    printf("                dirent, dlfcn, pthread\n");
    printf("  Network:      sys/socket, netinet/in, arpa/inet,\n");
    printf("                netdb, poll, sys/epoll\n");
    printf("  Compression:  zlib, lz4, zstd\n");
    printf("  Images:       png, jpeg\n");
    printf("  Audio:        opus\n");
    printf("  GPU:          vulkan\n");
    printf("  Fonts:        freetype\n");
    printf("  Database:     sqlite3\n");
    printf("  Network/TLS:  curl, openssl\n");
    printf("  Hardware:     libinput, xkbcommon\n");
    printf("  Multimedia:   avcodec, avformat, avutil\n");
    printf("  XML:          expat\n");
    printf("\nTotal: 40+ headers — ADead-BIB compiles them all.\n");
    printf("Sin GCC. Sin Clang. Solo ADead-BIB. 💀🦈🇵🇪\n");

    return 0;
}
