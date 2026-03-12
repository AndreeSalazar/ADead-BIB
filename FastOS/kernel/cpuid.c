/*
 * kernel/cpuid.c — CPU Feature Detection
 * FastOS v2.0
 *
 * Detects and reports CPU capabilities at runtime via CPUID.
 * Called by kernel_main() after term_init() to display what
 * stage2 activated: SSE (128-bit XMM) and AVX2 (256-bit YMM).
 *
 * Compilar: adb step kernel/cpuid.c
 */

#include "../include/kernel.h"
#include "../include/types.h"

/* ─── CPU Feature Flags (global, readable by any subsystem) ─── */
static uint32_t cpu_features_ecx = 0;   /* CPUID.1:ECX */
static uint32_t cpu_features_edx = 0;   /* CPUID.1:EDX */
static uint32_t cpu_ext_ebx     = 0;   /* CPUID.7:EBX */

/* Feature bit positions — CPUID leaf 1, ECX */
#define CPUID_ECX_SSE3      (1 <<  0)
#define CPUID_ECX_SSSE3     (1 <<  9)
#define CPUID_ECX_SSE41     (1 << 19)
#define CPUID_ECX_SSE42     (1 << 20)
#define CPUID_ECX_AES       (1 << 25)
#define CPUID_ECX_XSAVE     (1 << 26)
#define CPUID_ECX_OSXSAVE   (1 << 27)
#define CPUID_ECX_AVX       (1 << 28)

/* Feature bit positions — CPUID leaf 1, EDX */
#define CPUID_EDX_FPU       (1 <<  0)
#define CPUID_EDX_MMX       (1 << 23)
#define CPUID_EDX_SSE       (1 << 25)
#define CPUID_EDX_SSE2      (1 << 26)

/* Feature bit positions — CPUID leaf 7, EBX */
#define CPUID_EBX_AVX2      (1 <<  5)
#define CPUID_EBX_BMI1      (1 <<  3)
#define CPUID_EBX_BMI2      (1 <<  8)

/* ─── Vendor String ─── */
static char cpu_vendor[13];
static char cpu_brand[49];

static void cpuid_read_vendor(void) {
    uint32_t eax, ebx, ecx, edx;
    cpuid(0, &eax, &ebx, &ecx, &edx);

    /* Vendor string is EBX-EDX-ECX (not EBX-ECX-EDX!) */
    *(uint32_t *)&cpu_vendor[0] = ebx;
    *(uint32_t *)&cpu_vendor[4] = edx;
    *(uint32_t *)&cpu_vendor[8] = ecx;
    cpu_vendor[12] = '\0';
}

static void cpuid_read_brand(void) {
    uint32_t eax, ebx, ecx, edx;

    /* Check if extended CPUID is supported */
    cpuid(0x80000000, &eax, &ebx, &ecx, &edx);
    if (eax < 0x80000004) {
        cpu_brand[0] = '\0';
        return;
    }

    /* Brand string is in leaves 0x80000002-0x80000004 */
    uint32_t *brand = (uint32_t *)cpu_brand;
    cpuid(0x80000002, &brand[0], &brand[1], &brand[2], &brand[3]);
    cpuid(0x80000003, &brand[4], &brand[5], &brand[6], &brand[7]);
    cpuid(0x80000004, &brand[8], &brand[9], &brand[10], &brand[11]);
    cpu_brand[48] = '\0';
}

/* ─── API Pública ─── */

int cpu_has_sse(void)   { return (cpu_features_edx & CPUID_EDX_SSE)  != 0; }
int cpu_has_sse2(void)  { return (cpu_features_edx & CPUID_EDX_SSE2) != 0; }
int cpu_has_sse3(void)  { return (cpu_features_ecx & CPUID_ECX_SSE3) != 0; }
int cpu_has_sse41(void) { return (cpu_features_ecx & CPUID_ECX_SSE41)!= 0; }
int cpu_has_sse42(void) { return (cpu_features_ecx & CPUID_ECX_SSE42)!= 0; }
int cpu_has_avx(void)   { return (cpu_features_ecx & CPUID_ECX_AVX)  != 0; }
int cpu_has_avx2(void)  { return (cpu_ext_ebx & CPUID_EBX_AVX2)     != 0; }
int cpu_has_aes(void)   { return (cpu_features_ecx & CPUID_ECX_AES)  != 0; }

const char *cpu_get_vendor(void) { return cpu_vendor; }
const char *cpu_get_brand(void)  { return cpu_brand; }

/* ─── Inicialización — llamada por kernel_main() ─── */
void cpuid_init(void) {
    uint32_t eax, ebx, ecx, edx;

    /* Read vendor and brand */
    cpuid_read_vendor();
    cpuid_read_brand();

    /* Leaf 1: feature flags */
    cpuid(1, &eax, &ebx, &ecx, &edx);
    cpu_features_ecx = ecx;
    cpu_features_edx = edx;

    /* Leaf 7: extended features (AVX2, BMI, etc.) */
    cpuid(7, &eax, &ebx, &ecx, &edx);
    cpu_ext_ebx = ebx;

    /* Report */
    kprintf("[CPU]  %s\n", cpu_vendor);
    if (cpu_brand[0] != '\0') {
        kprintf("[CPU]  %s\n", cpu_brand);
    }

    /* SIMD capabilities report */
    kprintf("[CPU]  SSE:%s SSE2:%s SSE3:%s SSE4.1:%s SSE4.2:%s\n",
            cpu_has_sse()  ? "Y" : "N",
            cpu_has_sse2() ? "Y" : "N",
            cpu_has_sse3() ? "Y" : "N",
            cpu_has_sse41() ? "Y" : "N",
            cpu_has_sse42() ? "Y" : "N");
    kprintf("[CPU]  AVX:%s AVX2:%s AES-NI:%s\n",
            cpu_has_avx()  ? "Y" : "N",
            cpu_has_avx2() ? "Y" : "N",
            cpu_has_aes()  ? "Y" : "N");

    if (cpu_has_avx2()) {
        kprintf("[CPU]  256-bit YMM registers ACTIVE\n");
    } else if (cpu_has_sse()) {
        kprintf("[CPU]  128-bit XMM registers ACTIVE (no AVX2)\n");
    }
}
