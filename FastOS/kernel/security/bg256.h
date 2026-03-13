/* ============================================================
 * BG 256-bit — Binary Guardian 256-bit Native Header
 * ============================================================
 * Deterministic ISA-Level Capability Guardian — AVX2 YMM Native
 *
 * Inherited from: BG — Binary Guardian (Rust crate)
 * C99 kernel-side implementation with 256-bit batch operations.
 *
 * Pipeline (mirrors Rust crate):
 *   Binary → Loader → ISA Decoder → Capability Map
 *        → Architecture Map → Policy Engine → APPROVE / DENY
 *
 * 256-bit advantage:
 *   VPCMPEQD ymm0, ymm1, ymm2 → 1 cycle on Zen3
 *   = 8 × uint32 comparisons simultaneous
 *   = 8 Po magic checks per cycle
 *   = at 4.6GHz: 4,600,000,000 checks/second
 *   = "virus dies before it's born"
 *
 * SecurityLevel mapping (from policy.rs):
 *   Kernel  = Ring 0 (everything allowed)
 *   Driver  = Ring 1 (IO + interrupts, no MSR/descriptors)
 *   Service = Ring 2 (no direct IO, authorized syscalls only)
 *   User    = Ring 3 (maximum restrictions)
 *
 * ViolationType mapping (from policy.rs):
 *   PRIVILEGED_INSTRUCTION, UNAUTHORIZED_SYSCALL,
 *   UNAUTHORIZED_IO, SELF_MODIFYING_CODE, RWX_MEMORY,
 *   HIDDEN_ENTRY_POINT, PROCESS_INJECTION, etc.
 *
 * Author: Eddi Andreé Salazar Matos — Lima, Perú 🇵🇪
 * FastOS v3.1 — Binary Is Binary 💀🦈
 * ============================================================ */

#ifndef BG256_H
#define BG256_H

/* ============================================================
 * Constants
 * ============================================================ */

/* Po executable magic — always verified first */
#define BG_PO_MAGIC         0x506F4F53  /* "PoOS" */

/* Verdicts (mirrors Rust Verdict enum) */
#define BG_APPROVE          0x00000001
#define BG_DENY             0x00000000
#define BG_QUARANTINE       0x00000002

/* Security levels (mirrors Rust SecurityLevel enum) */
#define BG_LEVEL_KERNEL     0   /* Ring 0 — everything allowed */
#define BG_LEVEL_DRIVER     1   /* Ring 1 — IO + restricted */
#define BG_LEVEL_SERVICE    2   /* Ring 2 — no direct IO */
#define BG_LEVEL_USER       3   /* Ring 3 — max restrictions */

/* Threat levels — severity of detected issues */
#define BG_THREAT_NONE      0   /* Clean binary */
#define BG_THREAT_LOW       1   /* Minor policy deviation */
#define BG_THREAT_MEDIUM    2   /* Suspicious patterns */
#define BG_THREAT_HIGH      3   /* Known exploit patterns */
#define BG_THREAT_CRITICAL  4   /* Active shellcode/ROP */
#define BG_THREAT_EXTREME   5   /* Self-modifying + RWX */
#define BG_THREAT_APT       6   /* APT-level sophistication */
#define BG_THREAT_ALIEN     7   /* Unknown — quarantine */

/* Violation types (mirrors Rust ViolationType enum) */
#define BG_VIOL_NONE                 0x0000
#define BG_VIOL_PRIVILEGED_INSN      0x0001
#define BG_VIOL_UNAUTHORIZED_SYSCALL 0x0002
#define BG_VIOL_UNAUTHORIZED_IO      0x0004
#define BG_VIOL_SELF_MODIFYING       0x0008
#define BG_VIOL_RWX_MEMORY           0x0010
#define BG_VIOL_HIDDEN_ENTRY         0x0020
#define BG_VIOL_PROCESS_INJECTION    0x0040
#define BG_VIOL_TIMING_ATTACK        0x0080
#define BG_VIOL_FAR_JUMP             0x0100
#define BG_VIOL_EXCESSIVE_INDIRECT   0x0200
#define BG_VIOL_OVERLAPPING_SECTIONS 0x0400
#define BG_VIOL_INVALID_ENTRY        0x0800
#define BG_VIOL_UNAUTHORIZED_HW      0x1000

/* 256-bit batch size: 8 × uint32 per YMM register */
#define BG256_BATCH_SIZE    8

/* FNV-1a constants (64-bit) */
#define BG_FNV_OFFSET       0xCBF29CE484222325ULL
#define BG_FNV_PRIME        0x00000100000001B3ULL

/* Shellcode signatures — common patterns (first 4 bytes) */
#define BG_SIG_NOP_SLED     0x90909090  /* NOP sled */
#define BG_SIG_INT3_SLED    0xCCCCCCCC  /* INT3 breakpoint sled */
#define BG_SIG_SYSCALL_PAD  0x0F050F05  /* syscall;syscall */
#define BG_SIG_RET_SLED     0xC3C3C3C3  /* RET sled (ROP) */

/* Maximum watched memory regions */
#define BG256_MAX_WATCH     16

/* Maximum violation log entries */
#define BG256_MAX_VIOLATIONS 32

/* ============================================================
 * Types
 * ============================================================ */

/* Forward declaration for Po header (defined in kernel.h) */
/* typedef struct PoHeaderV8 PoHeaderV8; */

/* BG256 Violation entry — single detected violation */
typedef struct {
    unsigned int type;          /* BG_VIOL_* bitmask */
    unsigned int offset;        /* byte offset in binary where found */
    unsigned int severity;      /* BG_THREAT_* level */
    unsigned int reserved;      /* alignment padding */
} BG256Violation;

/* BG256 batch state — 256-bit aligned for YMM operations
 * Core structure: holds 8 values for simultaneous comparison.
 * ADead-BIB SoaOptimizer detects the [8] arrays and emits:
 *   VPBROADCASTD ymm1, expected → broadcast magic to all 8 lanes
 *   VMOVAPS ymm0, [magic_batch] → load 8 candidates
 *   VPCMPEQD ymm2, ymm0, ymm1  → compare all 8 in 1 cycle
 *   VPTEST ymm2, ymm2           → test result
 */
typedef struct {
    /* Batch magic verification — 8 simultaneous Po magic checks */
    unsigned int magic_batch[8];    /* 8 magic values → YMM0 (32 bytes) */
    unsigned int expected[8];       /* 8 × 0x506F4F53 → YMM1 (32 bytes) */
    unsigned int results[8];        /* comparison results → YMM2 (32 bytes) */

    /* Batch hash — FNV-1a 256-bit vectorized (4 × 64-bit lanes) */
    unsigned long long hash_batch[4];   /* 4 hashes → YMM3 (32 bytes) */
    unsigned long long hash_primes[4];  /* 4 × FNV prime → YMM4 (32 bytes) */

    /* Batch signature scan — 8 simultaneous pattern matches */
    unsigned int sig_batch[8];      /* 8 dwords from memory → YMM5 */
    unsigned int sig_pattern[8];    /* 8 × pattern to match → YMM6 */
} BG256Batch;

/* BG256 watched memory region */
typedef struct {
    unsigned int base;          /* physical start address */
    unsigned int size;          /* region size in bytes */
    unsigned int level;         /* BG_LEVEL_* required to write */
    unsigned int active;        /* 1 = watching, 0 = inactive */
} BG256WatchRegion;

/* BG256 statistics — running counters */
typedef struct {
    unsigned int binaries_verified;     /* total binaries checked */
    unsigned int binaries_approved;     /* passed verification */
    unsigned int binaries_denied;       /* failed verification */
    unsigned int threats_blocked;       /* active threats stopped */
    unsigned int batch_ops;             /* 256-bit batch ops executed */
    unsigned int memory_scans;          /* memory regions scanned */
    unsigned int shellcode_detected;    /* shellcode patterns found */
    unsigned int magic_checks;          /* total Po magic comparisons */
} BG256Stats;

/* BG256 full state — the guardian */
typedef struct {
    /* Current status */
    unsigned int active;            /* 1 = BG256 running */
    unsigned int security_level;    /* current kernel level */
    unsigned int avx2_available;    /* 1 = AVX2 detected at boot */

    /* 256-bit batch engine (32-byte aligned) */
    BG256Batch batch;

    /* Statistics */
    BG256Stats stats;

    /* Violation log (ring buffer) */
    BG256Violation violations[BG256_MAX_VIOLATIONS];
    unsigned int violation_count;
    unsigned int violation_next;    /* ring buffer write index */

    /* Watched memory regions */
    BG256WatchRegion watch[BG256_MAX_WATCH];
    unsigned int watch_count;

    /* Reason string for last deny */
    char last_reason[64];
} BG256State;

/* ============================================================
 * Function Prototypes
 * ============================================================ */

/* Initialization */
static void bg256_init(int avx2_detected);

/* Core verification — 256-bit batch operations */
static int  bg256_verify_batch(unsigned int *magic_values, int count);
static int  bg256_verify_po(void *data, unsigned int size);
static int  bg256_verify_magic(unsigned int magic);

/* FNV-1a hashing — vectorized */
static unsigned long long bg256_hash_fnv1a(void *data, unsigned int size);
static void bg256_hash_batch(void **data_ptrs, unsigned int *sizes,
                             unsigned long long *out_hashes, int count);

/* Memory scanning — shellcode/exploit detection */
static int  bg256_scan_memory(unsigned int base, unsigned int size);
static int  bg256_scan_signatures(void *data, unsigned int size);

/* Watch regions — memory write monitoring */
static void bg256_watch_region(unsigned int base, unsigned int size, int level);
static void bg256_unwatch_region(unsigned int base);
static int  bg256_check_write(unsigned int addr, unsigned int size, int caller_level);

/* Verdict */
static int  bg256_approve(void *data, unsigned int size, int level);
static void bg256_deny(const char *reason, unsigned int viol_type, unsigned int offset);

/* Reporting */
static void bg256_report_serial(void);

/* Statistics query */
static BG256Stats *bg256_get_stats(void);

#endif /* BG256_H */
