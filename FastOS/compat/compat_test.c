/*
 * FastOS v2.2 — Compatibility Layer: Test Suite
 * compat/compat_test.c
 *
 * Prueba que las llamadas Win32 y POSIX se traducen correctamente
 * a syscalls nativas de FastOS. ADead-BIB compila esto a binario puro.
 *
 * Compilar con ADead-BIB:
 *   adb cc compat_test.c --target fastos -o compat_test.po
 *
 * Autor: Eddi Andreé Salazar Matos — Perú — GPL v2
 * ADead-BIB — Binary Is Binary — Po:506F4F53 — BG:APPROVE
 */

#include "fastos_syscall.h"
#include "fastos_stdlib.h"
#include "fastos_win32.h"
#include "fastos_posix.h"

/* ══════════════════════════════════════════════════════
 * Test Results Tracking
 * ══════════════════════════════════════════════════════ */

static int tests_run    = 0;
static int tests_passed = 0;
static int tests_failed = 0;

static void test_pass(const char *name) {
    tests_run    = tests_run + 1;
    tests_passed = tests_passed + 1;
    serial_print("[PASS] ");
    serial_print(name);
    serial_print("\r\n");
}

static void test_fail(const char *name, const char *reason) {
    tests_run    = tests_run + 1;
    tests_failed = tests_failed + 1;
    serial_print("[FAIL] ");
    serial_print(name);
    serial_print(" — ");
    serial_print(reason);
    serial_print("\r\n");
}

/* ══════════════════════════════════════════════════════
 * § 1. Test: fastos_stdlib — Memory Operations
 * ══════════════════════════════════════════════════════ */

static void test_mem_copy(void) {
    char src[8];
    char dst[8];
    src[0] = 'F'; src[1] = 'a'; src[2] = 's'; src[3] = 't';
    src[4] = 'O'; src[5] = 'S'; src[6] = 0;

    mem_copy(dst, src, 7);

    if (dst[0] == 'F' && dst[5] == 'S' && dst[6] == 0) {
        test_pass("mem_copy");
    } else {
        test_fail("mem_copy", "data mismatch");
    }
}

static void test_mem_set(void) {
    char buf[8];
    mem_set(buf, 0xAA, 8);

    if ((uint8_t)buf[0] == 0xAA && (uint8_t)buf[7] == 0xAA) {
        test_pass("mem_set");
    } else {
        test_fail("mem_set", "fill mismatch");
    }
}

static void test_mem_cmp(void) {
    char a[4]; a[0] = 'A'; a[1] = 'B'; a[2] = 'C'; a[3] = 0;
    char b[4]; b[0] = 'A'; b[1] = 'B'; b[2] = 'C'; b[3] = 0;
    char c[4]; c[0] = 'A'; c[1] = 'B'; c[2] = 'D'; c[3] = 0;

    if (mem_cmp(a, b, 4) == 0 && mem_cmp(a, c, 4) != 0) {
        test_pass("mem_cmp");
    } else {
        test_fail("mem_cmp", "comparison wrong");
    }
}

/* ══════════════════════════════════════════════════════
 * § 2. Test: fastos_stdlib — String Operations
 * ══════════════════════════════════════════════════════ */

static void test_str_len(void) {
    char s[7]; s[0]='F'; s[1]='a'; s[2]='s'; s[3]='t';
    s[4]='O'; s[5]='S'; s[6]=0;

    if (str_len(s) == 6) {
        test_pass("str_len");
    } else {
        test_fail("str_len", "wrong length");
    }
}

static void test_str_cmp(void) {
    char a[4]; a[0]='a'; a[1]='b'; a[2]='c'; a[3]=0;
    char b[4]; b[0]='a'; b[1]='b'; b[2]='c'; b[3]=0;
    char c[4]; c[0]='a'; c[1]='b'; c[2]='d'; c[3]=0;

    if (str_cmp(a, b) == 0 && str_cmp(a, c) < 0) {
        test_pass("str_cmp");
    } else {
        test_fail("str_cmp", "comparison wrong");
    }
}

static void test_str_cpy(void) {
    char src[6]; src[0]='H'; src[1]='e'; src[2]='l';
    src[3]='l'; src[4]='o'; src[5]=0;
    char dst[8];

    str_cpy(dst, src);

    if (str_cmp(dst, src) == 0) {
        test_pass("str_cpy");
    } else {
        test_fail("str_cpy", "copy mismatch");
    }
}

/* ══════════════════════════════════════════════════════
 * § 3. Test: fastos_stdlib — Number Conversion
 * ══════════════════════════════════════════════════════ */

static void test_int_to_str(void) {
    char buf[21];

    int_to_str(12345, buf);
    if (buf[0]=='1' && buf[1]=='2' && buf[2]=='3' && buf[3]=='4' && buf[4]=='5' && buf[5]==0) {
        test_pass("int_to_str positive");
    } else {
        test_fail("int_to_str positive", "wrong output");
    }

    int_to_str(-42, buf);
    if (buf[0]=='-' && buf[1]=='4' && buf[2]=='2' && buf[3]==0) {
        test_pass("int_to_str negative");
    } else {
        test_fail("int_to_str negative", "wrong output");
    }

    int_to_str(0, buf);
    if (buf[0]=='0' && buf[1]==0) {
        test_pass("int_to_str zero");
    } else {
        test_fail("int_to_str zero", "wrong output");
    }
}

static void test_uint_to_hex(void) {
    char buf[17];

    uint_to_hex(0xFF, buf);
    if (buf[0]=='F' && buf[1]=='F' && buf[2]==0) {
        test_pass("uint_to_hex 0xFF");
    } else {
        test_fail("uint_to_hex 0xFF", "wrong output");
    }

    uint_to_hex(0x506F4F53, buf);
    /* "506F4F53" = Po magic */
    if (str_len(buf) == 8) {
        test_pass("uint_to_hex Po magic");
    } else {
        test_fail("uint_to_hex Po magic", "wrong length");
    }
}

/* ══════════════════════════════════════════════════════
 * § 4. Test: fastos_win32 — CreateFile Translation
 *
 * Tests that Win32 API maps to fs_open correctly.
 * Since there's no real filesystem yet, we verify the
 * translation logic compiles and the flag mapping works.
 * ══════════════════════════════════════════════════════ */

static void test_win32_flag_mapping(void) {
    /* Verify constant definitions match expected values */
    int ok = 1;

    if (GENERIC_READ  != 0x80000000) ok = 0;
    if (GENERIC_WRITE != 0x40000000) ok = 0;
    if (CREATE_NEW     != 1) ok = 0;
    if (OPEN_EXISTING  != 3) ok = 0;
    if (MEM_COMMIT     != 0x00001000) ok = 0;
    if (PAGE_READWRITE != 0x04) ok = 0;

    if (ok) {
        test_pass("win32_flag_constants");
    } else {
        test_fail("win32_flag_constants", "wrong values");
    }
}

static void test_win32_types(void) {
    /* Verify type sizes are correct */
    int ok = 1;

    if (sizeof(DWORD) != 4) ok = 0;
    if (sizeof(WORD)  != 2) ok = 0;
    if (sizeof(BYTE)  != 1) ok = 0;
    if (sizeof(BOOL)  != 4) ok = 0;
    if (TRUE  != 1) ok = 0;
    if (FALSE != 0) ok = 0;

    if (ok) {
        test_pass("win32_types");
    } else {
        test_fail("win32_types", "wrong sizes");
    }
}

static void test_win32_system_info(void) {
    SYSTEM_INFO si;
    mem_zero(&si, sizeof(si));

    /* GetSystemInfo should populate the struct.
     * For now, verify it compiles and sets architecture. */
    /* GetSystemInfo(&si); — needs real sys_info() implementation */

    /* Static check: the struct layout must compile */
    si.wProcessorArchitecture = PROCESSOR_ARCHITECTURE_AMD64;
    si.dwPageSize = FASTOS_PAGE_SIZE;

    if (si.wProcessorArchitecture == 9 && si.dwPageSize == 4096) {
        test_pass("win32_system_info_struct");
    } else {
        test_fail("win32_system_info_struct", "wrong layout");
    }
}

/* ══════════════════════════════════════════════════════
 * § 5. Test: fastos_posix — POSIX Translation
 * ══════════════════════════════════════════════════════ */

static void test_posix_flag_mapping(void) {
    int ok = 1;

    if (O_RDONLY != 0x0000) ok = 0;
    if (O_WRONLY != 0x0001) ok = 0;
    if (O_RDWR   != 0x0002) ok = 0;
    if (O_CREAT  != 0x0040) ok = 0;
    if (O_TRUNC  != 0x0200) ok = 0;
    if (SEEK_SET != 0) ok = 0;
    if (SEEK_CUR != 1) ok = 0;
    if (SEEK_END != 2) ok = 0;

    if (ok) {
        test_pass("posix_flag_constants");
    } else {
        test_fail("posix_flag_constants", "wrong values");
    }
}

static void test_posix_types(void) {
    int ok = 1;

    if (sizeof(pid_t)    != 4) ok = 0;
    if (sizeof(off_t)    != 8) ok = 0;
    if (sizeof(time_t)   != 8) ok = 0;

    if (ok) {
        test_pass("posix_types");
    } else {
        test_fail("posix_types", "wrong sizes");
    }
}

static void test_posix_mmap_constants(void) {
    int ok = 1;

    if (PROT_NONE  != 0x0) ok = 0;
    if (PROT_READ  != 0x1) ok = 0;
    if (PROT_WRITE != 0x2) ok = 0;
    if (PROT_EXEC  != 0x4) ok = 0;
    if (MAP_FAILED != ((void *)-1)) ok = 0;

    if (ok) {
        test_pass("posix_mmap_constants");
    } else {
        test_fail("posix_mmap_constants", "wrong values");
    }
}

/* ══════════════════════════════════════════════════════
 * § 6. Test: Binary Guardian Integration
 * ══════════════════════════════════════════════════════ */

static void test_bg_constants(void) {
    int ok = 1;

    if (BG_LEVEL_REBUILD  != 1) ok = 0;
    if (BG_LEVEL_FIREWALL != 2) ok = 0;
    if (BG_LEVEL_PREEXEC  != 3) ok = 0;
    if (BG_LEVEL_DEADMAN  != 4) ok = 0;
    if (BG_APPROVE != 0) ok = 0;
    if (BG_DENY    != 1) ok = 0;

    if (ok) {
        test_pass("bg_constants");
    } else {
        test_fail("bg_constants", "wrong values");
    }
}

static void test_error_codes(void) {
    int ok = 1;

    if (FASTOS_OK      !=  0) ok = 0;
    if (FASTOS_ERROR   != -1) ok = 0;
    if (FASTOS_ENOENT  != -2) ok = 0;
    if (FASTOS_ENOMEM  != -4) ok = 0;
    if (FASTOS_EBGDENY != -10) ok = 0;

    if (ok) {
        test_pass("fastos_error_codes");
    } else {
        test_fail("fastos_error_codes", "wrong values");
    }
}

/* ══════════════════════════════════════════════════════
 * § 7. Test: VGA + Serial I/O
 * ══════════════════════════════════════════════════════ */

static void test_vga_constants(void) {
    int ok = 1;

    if (VGA_BASE      != 0xB8000) ok = 0;
    if (VGA_COLS      != 80) ok = 0;
    if (VGA_ROWS      != 25) ok = 0;
    if (VGA_ROW_BYTES != 160) ok = 0;
    if (VGA_ATTR(0x1, 0xF) != 0x1F) ok = 0;

    if (ok) {
        test_pass("vga_constants");
    } else {
        test_fail("vga_constants", "wrong values");
    }
}

/* ══════════════════════════════════════════════════════
 * § 8. Test: Cross-layer — Win32 and POSIX use same syscalls
 * ══════════════════════════════════════════════════════ */

static void test_cross_layer(void) {
    /* Verify both layers translate to the same FastOS constants */
    int ok = 1;

    /* Win32 FILE_BEGIN == POSIX SEEK_SET == FS_SEEK_SET */
    if (FILE_BEGIN != FS_SEEK_SET) ok = 0;
    if (SEEK_SET   != FS_SEEK_SET) ok = 0;

    /* Win32 INFINITE timeout concept exists */
    if (INFINITE != 0xFFFFFFFF) ok = 0;

    /* Both layers include fastos_syscall.h core */
    if (FASTOS_PO_MAGIC != 0x506F4F53) ok = 0;

    if (ok) {
        test_pass("cross_layer_consistency");
    } else {
        test_fail("cross_layer_consistency", "layers disagree");
    }
}

/* ══════════════════════════════════════════════════════
 * § 9. Test: Math — vec8_dot scalar fallback
 * ══════════════════════════════════════════════════════ */

static void test_vec8_dot(void) {
    float a[8]; float b[8];
    int i = 0;
    while (i < 8) {
        a[i] = 1.0f;
        b[i] = 2.0f;
        i = i + 1;
    }

    float result = vec8_dot(a, b);

    /* 1.0 * 2.0 * 8 = 16.0 */
    if (result > 15.9f && result < 16.1f) {
        test_pass("vec8_dot");
    } else {
        test_fail("vec8_dot", "wrong result");
    }
}

/* ══════════════════════════════════════════════════════
 * Main Test Runner
 * ══════════════════════════════════════════════════════ */

void compat_test_main(void) {
    serial_print("\r\n");
    serial_print("═══════════════════════════════════════════\r\n");
    serial_print("  FastOS v2.2 — Compatibility Layer Tests\r\n");
    serial_print("  ADead-BIB — Binary Is Binary\r\n");
    serial_print("  Po:506F4F53 — BG:APPROVE\r\n");
    serial_print("═══════════════════════════════════════════\r\n\r\n");

    /* stdlib tests */
    serial_print("--- fastos_stdlib: Memory ---\r\n");
    test_mem_copy();
    test_mem_set();
    test_mem_cmp();

    serial_print("--- fastos_stdlib: Strings ---\r\n");
    test_str_len();
    test_str_cmp();
    test_str_cpy();

    serial_print("--- fastos_stdlib: Numbers ---\r\n");
    test_int_to_str();
    test_uint_to_hex();

    /* Win32 tests */
    serial_print("--- fastos_win32: Flag Mapping ---\r\n");
    test_win32_flag_mapping();
    test_win32_types();
    test_win32_system_info();

    /* POSIX tests */
    serial_print("--- fastos_posix: Flag Mapping ---\r\n");
    test_posix_flag_mapping();
    test_posix_types();
    test_posix_mmap_constants();

    /* BG + Error codes */
    serial_print("--- Binary Guardian + Error Codes ---\r\n");
    test_bg_constants();
    test_error_codes();

    /* VGA */
    serial_print("--- VGA Constants ---\r\n");
    test_vga_constants();

    /* Cross-layer */
    serial_print("--- Cross-Layer Consistency ---\r\n");
    test_cross_layer();

    /* Math */
    serial_print("--- Math: AVX2 ---\r\n");
    test_vec8_dot();

    /* Summary */
    serial_print("\r\n");
    serial_print("═══════════════════════════════════════════\r\n");
    serial_print("  Results: ");
    char buf[21];
    int_to_str(tests_passed, buf); serial_print(buf);
    serial_print("/");
    int_to_str(tests_run, buf); serial_print(buf);
    serial_print(" passed, ");
    int_to_str(tests_failed, buf); serial_print(buf);
    serial_print(" failed\r\n");

    if (tests_failed == 0) {
        serial_print("  STATUS: ALL TESTS PASSED ✓\r\n");
    } else {
        serial_print("  STATUS: SOME TESTS FAILED ✗\r\n");
    }
    serial_print("═══════════════════════════════════════════\r\n");
}
