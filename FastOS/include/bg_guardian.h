/* ============================================================
 * BG — Binary Guardian Interface for FastOS
 * ============================================================
 * C interface to the Rust Binary Guardian security system.
 *
 * This header mirrors types from the BG Rust crate:
 *   BG — Binary Guardian/src/policy.rs    → SecurityLevel, Verdict
 *   BG — Binary Guardian/src/arch_map.rs  → ArchitectureMap, Capabilities
 *   BG — Binary Guardian/src/analyzer.rs  → BinaryGuardian::analyze()
 *
 * BG provides:
 *   - ISA-level binary analysis (deterministic, O(n) build)
 *   - Capability mapping (what hardware a binary touches)
 *   - Policy enforcement (approve/deny execution)
 *   - Deterministic security (same binary + same policy = same verdict)
 *
 * Integration: FastOS loader calls BG before executing any binary.
 * FFI: security/bg/ Rust crate → libfg_fastos.a → linked into kernel.
 *
 * Compile with ADead-BIB:
 *   adb cc security/bg_core.c --flat --org=0x100000
 *   adb step security/bg_core.c   ← verify 7-phase pipeline
 * ============================================================ */

#ifndef BG_GUARDIAN_H
#define BG_GUARDIAN_H

#include "boot_types.h"

/* ============================================================
 * Security Levels
 * ============================================================ */

typedef enum {
    BG_LEVEL_KERNEL    = 0,  /* Full hardware access */
    BG_LEVEL_DRIVER    = 1,  /* Limited hardware, no direct memory */
    BG_LEVEL_SERVICE   = 2,  /* System services, no hardware */
    BG_LEVEL_USER      = 3,  /* User applications */
    BG_LEVEL_SANDBOX   = 4,  /* Restricted sandbox */
    BG_LEVEL_UNTRUSTED = 5   /* Maximum restrictions */
} BgSecurityLevel;

/* ============================================================
 * Verdict Types
 * ============================================================ */

typedef enum {
    BG_VERDICT_APPROVE = 0,  /* Binary is safe to execute */
    BG_VERDICT_DENY    = 1,  /* Binary violates policy */
    BG_VERDICT_WARN    = 2,  /* Binary has warnings but can run */
    BG_VERDICT_UNKNOWN = 3   /* Cannot determine (corrupted?) */
} BgVerdict;

/* ============================================================
 * Hardware Access Flags
 * ============================================================ */

#define BG_HW_NONE          0x00000000
#define BG_HW_MEMORY        0x00000001  /* Direct memory access */
#define BG_HW_PORTS         0x00000002  /* I/O ports */
#define BG_HW_INTERRUPTS    0x00000004  /* Interrupt control */
#define BG_HW_DMA           0x00000008  /* DMA access */
#define BG_HW_PCI           0x00000010  /* PCI configuration */
#define BG_HW_DISK          0x00000020  /* Disk I/O */
#define BG_HW_NETWORK       0x00000040  /* Network access */
#define BG_HW_GPU           0x00000080  /* GPU access */
#define BG_HW_KEYBOARD      0x00000100  /* Keyboard input */
#define BG_HW_MOUSE         0x00000200  /* Mouse input */
#define BG_HW_TIMER         0x00000400  /* Timer/RTC */
#define BG_HW_SERIAL        0x00000800  /* Serial ports */
#define BG_HW_USB           0x00001000  /* USB devices */
#define BG_HW_AUDIO         0x00002000  /* Audio devices */

/* ============================================================
 * Instruction Classes
 * ============================================================ */

#define BG_INST_ARITHMETIC  0x00000001
#define BG_INST_LOGIC       0x00000002
#define BG_INST_MEMORY      0x00000004
#define BG_INST_CONTROL     0x00000008
#define BG_INST_STACK       0x00000010
#define BG_INST_STRING      0x00000020
#define BG_INST_FLOAT       0x00000040
#define BG_INST_SIMD        0x00000080
#define BG_INST_SYSTEM      0x00000100  /* Privileged */
#define BG_INST_IO          0x00000200  /* I/O ports */
#define BG_INST_INTERRUPT   0x00000400  /* INT/IRET */
#define BG_INST_CRYPTO      0x00000800  /* AES-NI, SHA */

/* ============================================================
 * Violation Types
 * ============================================================ */

typedef enum {
    BG_VIOLATION_NONE = 0,
    BG_VIOLATION_PRIVILEGED_INSTRUCTION,
    BG_VIOLATION_UNAUTHORIZED_PORT,
    BG_VIOLATION_UNAUTHORIZED_MEMORY,
    BG_VIOLATION_UNAUTHORIZED_INTERRUPT,
    BG_VIOLATION_UNAUTHORIZED_SYSCALL,
    BG_VIOLATION_SELF_MODIFYING_CODE,
    BG_VIOLATION_STACK_MANIPULATION,
    BG_VIOLATION_IMPORT_MISMATCH,
    BG_VIOLATION_STRUCTURAL_ANOMALY
} BgViolationType;

/* ============================================================
 * Analysis Result Structure
 * ============================================================ */

typedef struct {
    BgVerdict verdict;
    BgSecurityLevel required_level;
    u32 hardware_access;      /* BG_HW_* flags */
    u32 instruction_classes;  /* BG_INST_* flags */
    u32 violation_count;
    BgViolationType first_violation;
    u32 code_size;
    u32 data_size;
    u32 import_count;
    u32 export_count;
} BgAnalysisResult;

/* ============================================================
 * Security Policy Structure
 * ============================================================ */

typedef struct {
    BgSecurityLevel level;
    u32 allowed_hardware;     /* BG_HW_* flags */
    u32 allowed_instructions; /* BG_INST_* flags */
    u32 max_memory_access;    /* Maximum memory range */
    u32 allowed_ports_start;
    u32 allowed_ports_end;
    u8 allow_self_modify;
    u8 allow_stack_exec;
    u8 allow_raw_syscall;
    u8 reserved;
} BgSecurityPolicy;

/* ============================================================
 * BG API Functions (implemented in Rust, called from C)
 * ============================================================ */

/* Initialize BG system */
void bg_init(void);

/* Analyze a binary in memory */
BgAnalysisResult bg_analyze(const void* binary, u32 size);

/* Check if binary is allowed under a policy */
BgVerdict bg_check_policy(const BgAnalysisResult* result, 
                          const BgSecurityPolicy* policy);

/* Get default policy for a security level */
BgSecurityPolicy bg_get_default_policy(BgSecurityLevel level);

/* Get human-readable violation description */
const char* bg_violation_string(BgViolationType violation);

/* Get human-readable verdict description */
const char* bg_verdict_string(BgVerdict verdict);

/* ============================================================
 * Convenience Macros
 * ============================================================ */

#define BG_IS_APPROVED(result) ((result).verdict == BG_VERDICT_APPROVE)
#define BG_IS_DENIED(result)   ((result).verdict == BG_VERDICT_DENY)
#define BG_HAS_VIOLATIONS(result) ((result).violation_count > 0)

#define BG_NEEDS_KERNEL(result) \
    ((result).required_level == BG_LEVEL_KERNEL)

#define BG_TOUCHES_HARDWARE(result, hw) \
    (((result).hardware_access & (hw)) != 0)

#define BG_USES_PRIVILEGED(result) \
    (((result).instruction_classes & BG_INST_SYSTEM) != 0)

/* ============================================================
 * Default Policies
 * ============================================================ */

/* Kernel: full access */
#define BG_POLICY_KERNEL { \
    .level = BG_LEVEL_KERNEL, \
    .allowed_hardware = 0xFFFFFFFF, \
    .allowed_instructions = 0xFFFFFFFF, \
    .max_memory_access = 0xFFFFFFFF, \
    .allowed_ports_start = 0, \
    .allowed_ports_end = 0xFFFF, \
    .allow_self_modify = 1, \
    .allow_stack_exec = 1, \
    .allow_raw_syscall = 1 \
}

/* Driver: hardware but no arbitrary memory */
#define BG_POLICY_DRIVER { \
    .level = BG_LEVEL_DRIVER, \
    .allowed_hardware = BG_HW_PORTS | BG_HW_PCI | BG_HW_INTERRUPTS, \
    .allowed_instructions = 0xFFFFFFFF & ~BG_INST_SYSTEM, \
    .max_memory_access = 0x10000000, \
    .allowed_ports_start = 0, \
    .allowed_ports_end = 0xFFFF, \
    .allow_self_modify = 0, \
    .allow_stack_exec = 0, \
    .allow_raw_syscall = 0 \
}

/* User: no hardware, no privileged instructions */
#define BG_POLICY_USER { \
    .level = BG_LEVEL_USER, \
    .allowed_hardware = BG_HW_NONE, \
    .allowed_instructions = BG_INST_ARITHMETIC | BG_INST_LOGIC | \
                           BG_INST_MEMORY | BG_INST_CONTROL | \
                           BG_INST_STACK | BG_INST_STRING | \
                           BG_INST_FLOAT | BG_INST_SIMD, \
    .max_memory_access = 0x1000000, \
    .allowed_ports_start = 0, \
    .allowed_ports_end = 0, \
    .allow_self_modify = 0, \
    .allow_stack_exec = 0, \
    .allow_raw_syscall = 0 \
}

/* Sandbox: maximum restrictions */
#define BG_POLICY_SANDBOX { \
    .level = BG_LEVEL_SANDBOX, \
    .allowed_hardware = BG_HW_NONE, \
    .allowed_instructions = BG_INST_ARITHMETIC | BG_INST_LOGIC | \
                           BG_INST_CONTROL | BG_INST_STACK, \
    .max_memory_access = 0x100000, \
    .allowed_ports_start = 0, \
    .allowed_ports_end = 0, \
    .allow_self_modify = 0, \
    .allow_stack_exec = 0, \
    .allow_raw_syscall = 0 \
}

#endif /* BG_GUARDIAN_H */
