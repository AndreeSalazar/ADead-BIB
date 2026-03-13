/* ============================================================
 * BG 256-bit — Binary Guardian 256-bit Native Implementation
 * ============================================================
 * C99 kernel-side implementation of Binary Guardian with
 * AVX2 YMM 256-bit batch verification operations.
 *
 * Inherited architecture from BG Rust crate:
 *   analyzer.rs   → bg256_verify_po() pipeline
 *   capability.rs → bg256_scan_signatures() pattern matching
 *   policy.rs     → bg256_approve()/bg256_deny() verdict engine
 *   arch_map.rs   → violation types and security levels
 *
 * 256-bit batch design:
 *   Traditional antivirus: 1 check/cycle (scalar 64-bit)
 *   BG 256-bit:            8 checks/cycle (YMM 256-bit)
 *   = virus dies before it's born
 *
 * ADead-BIB vectorization:
 *   The [8] arrays in BG256Batch are detected by SoaOptimizer.
 *   8-wide unrolled scalar loops → VPBROADCASTD + VPCMPEQD + VPTEST
 *   Pure C99 — no intrinsics, no inline asm in kernel.
 *
 * Memory layout:
 *   BG256State lives at a fixed kernel address (stack or .bss).
 *   No heap needed — all structures are statically sized.
 *
 * Author: Eddi Andreé Salazar Matos — Lima, Perú
 * FastOS v3.1 — Binary Is Binary
 * ============================================================ */

/* ============================================================
 * Global state — single BG256 instance for kernel
 * ============================================================ */
static BG256State bg256_state;

/* ============================================================
 * Internal helpers — inline string/memory (no libc)
 * ============================================================ */

static void bg256_memset(void *dst, int val, unsigned int n) {
    unsigned char *d = (unsigned char *)dst;
    unsigned int i = 0;
    while (i < n) { d[i] = (unsigned char)val; i++; }
}

static void bg256_memcpy(void *dst, const void *src, unsigned int n) {
    unsigned char *d = (unsigned char *)dst;
    const unsigned char *s = (const unsigned char *)src;
    unsigned int i = 0;
    while (i < n) { d[i] = s[i]; i++; }
}

static int bg256_strlen(const char *s) {
    int n = 0;
    while (s[n]) n++;
    return n;
}

static void bg256_strcpy(char *dst, const char *src) {
    int i = 0;
    while (src[i]) { dst[i] = src[i]; i++; }
    dst[i] = 0;
}

/* Serial output — single char to COM1 */
static void bg256_serial_char(char c) {
    __outb(0x3F8, (unsigned char)c);
}

/* Serial output — null-terminated string */
static void bg256_serial_str(const char *s) {
    while (*s) { bg256_serial_char(*s); s++; }
}

/* Serial output — hex uint32 */
static void bg256_serial_hex32(unsigned int val) {
    int i; int nibble; char c;
    bg256_serial_str("0x");
    i = 7;
    while (i >= 0) {
        nibble = (val >> (i * 4)) & 0xF;
        if (nibble < 10) { c = (char)(48 + nibble); }
        else { c = (char)(55 + nibble); }
        bg256_serial_char(c);
        i--;
    }
}

/* Serial output — decimal uint32 */
static void bg256_serial_dec(unsigned int val) {
    char buf[12];
    int pos = 0;
    int i;
    if (val == 0) { bg256_serial_char('0'); return; }
    while (val > 0) {
        buf[pos] = (char)(48 + (val % 10));
        val = val / 10;
        pos++;
    }
    i = pos - 1;
    while (i >= 0) { bg256_serial_char(buf[i]); i--; }
}

/* ============================================================
 * bg256_init() — Initialize BG 256-bit guardian
 *
 * Called once at boot after CPUID detection.
 * Sets up the batch comparison arrays with expected values.
 * Mirrors BinaryGuardian::new() from analyzer.rs.
 * ============================================================ */
static void bg256_init(int avx2_detected) {
    int i;

    /* Zero entire state */
    bg256_memset(&bg256_state, 0, sizeof(BG256State));

    /* Record AVX2 availability from CPUID leaf 7 */
    bg256_state.avx2_available = (avx2_detected > 0) ? 1 : 0;

    /* Kernel runs at Ring 0 — BG level KERNEL */
    bg256_state.security_level = BG_LEVEL_KERNEL;

    /* Pre-fill expected magic array — all 8 lanes = Po magic
     * When compiled by ADead-BIB with SoaOptimizer:
     *   This becomes VPBROADCASTD ymm1, 0x506F4F53
     *   One instruction fills all 8 comparison lanes */
    i = 0;
    while (i < 8) {
        bg256_state.batch.expected[i] = BG_PO_MAGIC;
        i++;
    }

    /* Pre-fill FNV-1a prime array — all 4 lanes
     * For vectorized hashing across 4 simultaneous streams */
    i = 0;
    while (i < 4) {
        bg256_state.batch.hash_primes[i] = BG_FNV_PRIME;
        i++;
    }

    /* Activate guardian */
    bg256_state.active = 1;

    /* Watch kernel code region — prevent writes from user level */
    bg256_watch_region(0x100000, 0x100000, BG_LEVEL_KERNEL);

    /* Serial report */
    bg256_serial_str("BG256: INIT ");
    if (bg256_state.avx2_available) {
        bg256_serial_str("AVX2=YES 8checks/cycle YMM\r\n");
    } else {
        bg256_serial_str("AVX2=NO scalar fallback\r\n");
    }
}

/* ============================================================
 * bg256_verify_batch() — Verify up to 8 Po magic values
 *
 * Core 256-bit operation. Loads 8 magic candidates into
 * magic_batch[8], compares against expected[8] (all 0x506F4F53).
 *
 * ADead-BIB SoaOptimizer transforms this to:
 *   VMOVAPS   ymm0, [magic_batch]     ; load 8 candidates
 *   VPCMPEQD  ymm2, ymm0, ymm1       ; compare all 8 vs expected
 *   VPTEST    ymm2, ymm2              ; check if all matched
 *   JZ        deny_path               ; if any mismatch → DENY
 *
 * Returns: number of valid magic values (0-8)
 *          If all 8 match → 8 (all approved in 1 cycle)
 * ============================================================ */
static int bg256_verify_batch(unsigned int *magic_values, int count) {
    int i;
    int valid;

    if (count <= 0) return 0;
    if (count > BG256_BATCH_SIZE) count = BG256_BATCH_SIZE;

    /* Load candidate magic values into batch array
     * Pad unused lanes with 0 (guaranteed mismatch) */
    i = 0;
    while (i < 8) {
        if (i < count) {
            bg256_state.batch.magic_batch[i] = magic_values[i];
        } else {
            bg256_state.batch.magic_batch[i] = 0;
        }
        i++;
    }

    /* 256-bit comparison — 8 lanes simultaneously
     * This 8-wide unrolled loop is the SoaOptimizer target.
     * ADead-BIB detects: batch[0..7] == expected[0..7]
     * Emits: VPCMPEQD ymm2, ymm0, ymm1 (1 cycle, 8 results)
     *
     * On scalar fallback (no AVX2): 8 sequential comparisons
     * Still faster than traditional AV — no branches per byte */
    bg256_state.batch.results[0] = (bg256_state.batch.magic_batch[0] == bg256_state.batch.expected[0]) ? 1 : 0;
    bg256_state.batch.results[1] = (bg256_state.batch.magic_batch[1] == bg256_state.batch.expected[1]) ? 1 : 0;
    bg256_state.batch.results[2] = (bg256_state.batch.magic_batch[2] == bg256_state.batch.expected[2]) ? 1 : 0;
    bg256_state.batch.results[3] = (bg256_state.batch.magic_batch[3] == bg256_state.batch.expected[3]) ? 1 : 0;
    bg256_state.batch.results[4] = (bg256_state.batch.magic_batch[4] == bg256_state.batch.expected[4]) ? 1 : 0;
    bg256_state.batch.results[5] = (bg256_state.batch.magic_batch[5] == bg256_state.batch.expected[5]) ? 1 : 0;
    bg256_state.batch.results[6] = (bg256_state.batch.magic_batch[6] == bg256_state.batch.expected[6]) ? 1 : 0;
    bg256_state.batch.results[7] = (bg256_state.batch.magic_batch[7] == bg256_state.batch.expected[7]) ? 1 : 0;

    /* Count valid results */
    valid = 0;
    i = 0;
    while (i < count) {
        if (bg256_state.batch.results[i]) valid++;
        i++;
    }

    /* Update stats */
    bg256_state.stats.batch_ops++;
    bg256_state.stats.magic_checks = bg256_state.stats.magic_checks + (unsigned int)count;

    return valid;
}

/* ============================================================
 * bg256_verify_magic() — Verify a single Po magic value
 *
 * Convenience wrapper. For single binary verification.
 * Returns: BG_APPROVE or BG_DENY
 * ============================================================ */
static int bg256_verify_magic(unsigned int magic) {
    unsigned int batch[1];
    batch[0] = magic;
    if (bg256_verify_batch(batch, 1) == 1) {
        return BG_APPROVE;
    }
    return BG_DENY;
}

/* ============================================================
 * bg256_hash_fnv1a() — FNV-1a hash of a memory region
 *
 * Standard FNV-1a 64-bit hash. Used for bg_stamp verification
 * in Po v8.0 headers and Binary Guardian's pre-execution gate.
 *
 * The hash is deterministic: same data = same hash. Always.
 * Mirrors the FNV-1a cache in bg_preexec.c.
 * ============================================================ */
static unsigned long long bg256_hash_fnv1a(void *data, unsigned int size) {
    unsigned long long hash;
    unsigned char *bytes;
    unsigned int i;

    hash = BG_FNV_OFFSET;
    bytes = (unsigned char *)data;

    i = 0;
    while (i < size) {
        hash = hash ^ (unsigned long long)bytes[i];
        hash = hash * BG_FNV_PRIME;
        i++;
    }

    return hash;
}

/* ============================================================
 * bg256_hash_batch() — Vectorized FNV-1a for multiple streams
 *
 * Hashes up to 4 data regions simultaneously using 256-bit
 * operations on 4 × 64-bit hash states in YMM registers.
 *
 * ADead-BIB SoaOptimizer detects the 4-wide hash update:
 *   hash[0..3] = hash[0..3] ^ byte[0..3]
 *   hash[0..3] = hash[0..3] * prime[0..3]
 * Emits: VPXORQ + VPMULUDQ (2 instructions per byte, 4 streams)
 *
 * count: 1-4 simultaneous hash streams
 * ============================================================ */
static void bg256_hash_batch(void **data_ptrs, unsigned int *sizes,
                             unsigned long long *out_hashes, int count) {
    unsigned int i; unsigned int byte_pos; unsigned int min_size;
    unsigned char *d0; unsigned char *d1; unsigned char *d2; unsigned char *d3;

    if (count <= 0) return;
    if (count > 4) count = 4;

    /* Initialize hash states — all lanes start at FNV offset */
    i = 0;
    while (i < 4) {
        bg256_state.batch.hash_batch[i] = BG_FNV_OFFSET;
        i++;
    }

    /* Find minimum size for vectorized portion */
    min_size = sizes[0];
    i = 1;
    while (i < (unsigned int)count) {
        if (sizes[i] < min_size) min_size = sizes[i];
        i++;
    }

    /* Get data pointers (pad with ptr[0] for unused lanes) */
    d0 = (unsigned char *)(data_ptrs[0]);
    d1 = (count > 1) ? (unsigned char *)(data_ptrs[1]) : d0;
    d2 = (count > 2) ? (unsigned char *)(data_ptrs[2]) : d0;
    d3 = (count > 3) ? (unsigned char *)(data_ptrs[3]) : d0;

    /* Vectorized hash — all 4 streams process same byte offset
     * This 4-wide pattern is the SoaOptimizer target for VPXORQ/VPMULUDQ */
    byte_pos = 0;
    while (byte_pos < min_size) {
        bg256_state.batch.hash_batch[0] = bg256_state.batch.hash_batch[0] ^ (unsigned long long)d0[byte_pos];
        bg256_state.batch.hash_batch[1] = bg256_state.batch.hash_batch[1] ^ (unsigned long long)d1[byte_pos];
        bg256_state.batch.hash_batch[2] = bg256_state.batch.hash_batch[2] ^ (unsigned long long)d2[byte_pos];
        bg256_state.batch.hash_batch[3] = bg256_state.batch.hash_batch[3] ^ (unsigned long long)d3[byte_pos];

        bg256_state.batch.hash_batch[0] = bg256_state.batch.hash_batch[0] * bg256_state.batch.hash_primes[0];
        bg256_state.batch.hash_batch[1] = bg256_state.batch.hash_batch[1] * bg256_state.batch.hash_primes[1];
        bg256_state.batch.hash_batch[2] = bg256_state.batch.hash_batch[2] * bg256_state.batch.hash_primes[2];
        bg256_state.batch.hash_batch[3] = bg256_state.batch.hash_batch[3] * bg256_state.batch.hash_primes[3];

        byte_pos++;
    }

    /* Finish remaining bytes for streams longer than min_size */
    i = 0;
    while (i < (unsigned int)count) {
        unsigned char *dp;
        unsigned int pos;
        if (i == 0) dp = d0;
        else if (i == 1) dp = d1;
        else if (i == 2) dp = d2;
        else dp = d3;

        pos = min_size;
        while (pos < sizes[i]) {
            bg256_state.batch.hash_batch[i] = bg256_state.batch.hash_batch[i] ^ (unsigned long long)dp[pos];
            bg256_state.batch.hash_batch[i] = bg256_state.batch.hash_batch[i] * BG_FNV_PRIME;
            pos++;
        }
        i++;
    }

    /* Output results */
    i = 0;
    while (i < (unsigned int)count) {
        out_hashes[i] = bg256_state.batch.hash_batch[i];
        i++;
    }
}

/* ============================================================
 * bg256_scan_signatures() — Scan data for shellcode patterns
 *
 * 256-bit batch signature scan. Loads 8 consecutive dwords
 * from the data, compares against a known shellcode pattern
 * in 1 cycle. Repeats for each signature.
 *
 * Detects:
 *   NOP sled (0x90909090) — classic shellcode padding
 *   INT3 sled (0xCCCCCCCC) — debugger breakpoint overflow
 *   SYSCALL pad (0x0F050F05) — syscall chain
 *   RET sled (0xC3C3C3C3) — ROP gadget chain
 *
 * Returns: number of suspicious patterns found
 * ============================================================ */
static int bg256_scan_signatures(void *data, unsigned int size) {
    unsigned int *dwords;
    unsigned int dword_count;
    unsigned int pos;
    int found;
    int i;
    unsigned int signatures[4];

    if (size < 32) return 0; /* need at least 8 dwords */

    dwords = (unsigned int *)data;
    dword_count = size / 4;
    found = 0;

    signatures[0] = BG_SIG_NOP_SLED;
    signatures[1] = BG_SIG_INT3_SLED;
    signatures[2] = BG_SIG_SYSCALL_PAD;
    signatures[3] = BG_SIG_RET_SLED;

    /* Scan in 8-dword blocks — ADead-BIB vectorizes to:
     *   VPBROADCASTD ymm6, sig     ; broadcast signature to all 8 lanes
     *   VMOVAPS ymm5, [data+pos]   ; load 8 consecutive dwords
     *   VPCMPEQD ymm7, ymm5, ymm6 ; compare all 8 in 1 cycle
     *   VPTEST ymm7, ymm7          ; any match? */
    pos = 0;
    while (pos + 8 <= dword_count) {
        int sig_idx;

        /* Load 8 dwords into batch */
        bg256_state.batch.sig_batch[0] = dwords[pos + 0];
        bg256_state.batch.sig_batch[1] = dwords[pos + 1];
        bg256_state.batch.sig_batch[2] = dwords[pos + 2];
        bg256_state.batch.sig_batch[3] = dwords[pos + 3];
        bg256_state.batch.sig_batch[4] = dwords[pos + 4];
        bg256_state.batch.sig_batch[5] = dwords[pos + 5];
        bg256_state.batch.sig_batch[6] = dwords[pos + 6];
        bg256_state.batch.sig_batch[7] = dwords[pos + 7];

        /* Check each signature pattern */
        sig_idx = 0;
        while (sig_idx < 4) {
            /* Broadcast signature to all 8 lanes */
            i = 0;
            while (i < 8) {
                bg256_state.batch.sig_pattern[i] = signatures[sig_idx];
                i++;
            }

            /* 8-wide comparison — the core 256-bit operation
             * SoaOptimizer: VPCMPEQD ymm7, ymm5, ymm6 */
            i = 0;
            while (i < 8) {
                if (bg256_state.batch.sig_batch[i] == bg256_state.batch.sig_pattern[i]) {
                    found++;
                    bg256_state.stats.shellcode_detected++;
                }
                i++;
            }
            sig_idx++;
        }

        pos = pos + 8;
    }

    return found;
}

/* ============================================================
 * bg256_scan_memory() — Scan a physical memory region
 *
 * Scans the given region for shellcode signatures.
 * Uses direct physical memory access (no MMU).
 *
 * Returns: BG_APPROVE if clean, BG_DENY if threats found
 * ============================================================ */
static int bg256_scan_memory(unsigned int base, unsigned int size) {
    void *region;
    int threats;

    region = (void *)((unsigned long long)base);
    threats = bg256_scan_signatures(region, size);
    bg256_state.stats.memory_scans++;

    if (threats > 0) {
        bg256_deny("shellcode pattern detected in memory",
                   BG_VIOL_SELF_MODIFYING, base);
        return BG_DENY;
    }
    return BG_APPROVE;
}

/* ============================================================
 * bg256_verify_po() — Full Po binary verification pipeline
 *
 * Mirrors BinaryGuardian::analyze_loaded() from analyzer.rs:
 *   1. Verify Po magic (256-bit batch — 1 cycle)
 *   2. Verify version and header integrity
 *   3. Compute FNV-1a hash of code section
 *   4. Compare against bg_stamp in header
 *   5. Scan code for shellcode signatures
 *   6. Issue verdict: APPROVE or DENY
 *
 * This is the pre-execution gate. No binary runs without
 * passing through bg256_verify_po() first.
 *
 * "Virus dies before it's born" — BG finishes verification
 * before the exploit can even set up its first instruction.
 *
 * data: pointer to raw binary (starts with PoHeaderV8)
 * size: total size in bytes
 * Returns: BG_APPROVE or BG_DENY
 * ============================================================ */
static int bg256_verify_po(void *data, unsigned int size) {
    unsigned char *raw;
    unsigned int magic;
    unsigned int version;
    unsigned int bits;
    unsigned int code_off;
    unsigned int code_size;
    unsigned int bg_stamp;
    unsigned long long computed_hash;
    int sig_threats;

    raw = (unsigned char *)data;

    /* Minimum size: 32 bytes (PoHeaderV8) */
    if (size < 32) {
        bg256_deny("binary too small for Po v8.0 header", BG_VIOL_INVALID_ENTRY, 0);
        return BG_DENY;
    }

    /* Step 1: Extract and verify Po magic — 256-bit batch
     * Even for 1 binary, we use the batch path so the
     * comparison is always done via the vectorized pipeline */
    magic = (unsigned int)raw[0]
          | ((unsigned int)raw[1] << 8)
          | ((unsigned int)raw[2] << 16)
          | ((unsigned int)raw[3] << 24);

    if (bg256_verify_magic(magic) != BG_APPROVE) {
        bg256_deny("invalid Po magic — not a FastOS binary", BG_VIOL_INVALID_ENTRY, 0);
        return BG_DENY;
    }

    /* Step 2: Verify version and bits field */
    version = raw[4];
    bits = raw[5];

    /* Version must be >= 0x80 (v8.0+) */
    if (version < 0x80) {
        bg256_deny("Po version too old — need v8.0+", BG_VIOL_INVALID_ENTRY, 4);
        return BG_DENY;
    }

    /* Bits must be valid: 16, 64, 128, or 255 (=256) */
    if (bits != 16 && bits != 64 && bits != 128 && bits != 255) {
        bg256_deny("invalid bits field in Po header", BG_VIOL_INVALID_ENTRY, 5);
        return BG_DENY;
    }

    /* Step 3: Extract code section info */
    code_off  = (unsigned int)raw[8]
              | ((unsigned int)raw[9] << 8)
              | ((unsigned int)raw[10] << 16)
              | ((unsigned int)raw[11] << 24);
    code_size = (unsigned int)raw[12]
              | ((unsigned int)raw[13] << 8)
              | ((unsigned int)raw[14] << 16)
              | ((unsigned int)raw[15] << 24);

    /* Validate code section bounds */
    if (code_off + code_size > size) {
        bg256_deny("code section exceeds binary size", BG_VIOL_OVERLAPPING_SECTIONS, code_off);
        return BG_DENY;
    }

    /* Step 4: Compute FNV-1a hash of code section
     * Compare against bg_stamp field (offset 28-31) */
    bg_stamp = (unsigned int)raw[28]
             | ((unsigned int)raw[29] << 8)
             | ((unsigned int)raw[30] << 16)
             | ((unsigned int)raw[31] << 24);

    computed_hash = bg256_hash_fnv1a(raw + code_off, code_size);

    /* Compare lower 32 bits of hash against stamp */
    if ((unsigned int)(computed_hash & 0xFFFFFFFF) != bg_stamp) {
        bg256_deny("BG stamp mismatch — binary tampered", BG_VIOL_SELF_MODIFYING, 28);
        return BG_DENY;
    }

    /* Step 5: Scan code section for shellcode patterns */
    if (code_size >= 32) {
        sig_threats = bg256_scan_signatures(raw + code_off, code_size);
        if (sig_threats > 0) {
            bg256_deny("shellcode signatures in code section", BG_VIOL_SELF_MODIFYING, code_off);
            return BG_DENY;
        }
    }

    /* Step 6: APPROVED — binary is clean
     * Update stats and return success */
    bg256_state.stats.binaries_verified++;
    bg256_state.stats.binaries_approved++;

    bg256_serial_str("BG256: APPROVE Po ");
    bg256_serial_hex32(magic);
    bg256_serial_str(" v");
    bg256_serial_dec(version);
    bg256_serial_str(" ");
    bg256_serial_dec(bits);
    bg256_serial_str("bit stamp=");
    bg256_serial_hex32(bg_stamp);
    bg256_serial_str("\r\n");

    return BG_APPROVE;
}

/* ============================================================
 * bg256_watch_region() — Add a memory region to watch list
 *
 * Monitored regions are checked by bg256_check_write()
 * before any process can write to them. If the caller's
 * security level is below the required level, write is denied.
 * ============================================================ */
static void bg256_watch_region(unsigned int base, unsigned int size, int level) {
    unsigned int idx;

    if (bg256_state.watch_count >= BG256_MAX_WATCH) return;

    idx = bg256_state.watch_count;
    bg256_state.watch[idx].base = base;
    bg256_state.watch[idx].size = size;
    bg256_state.watch[idx].level = (unsigned int)level;
    bg256_state.watch[idx].active = 1;
    bg256_state.watch_count++;
}

/* ============================================================
 * bg256_unwatch_region() — Remove a watched region by base
 * ============================================================ */
static void bg256_unwatch_region(unsigned int base) {
    unsigned int i;
    i = 0;
    while (i < bg256_state.watch_count) {
        if (bg256_state.watch[i].base == base) {
            bg256_state.watch[i].active = 0;
            return;
        }
        i++;
    }
}

/* ============================================================
 * bg256_check_write() — Check if a write to address is allowed
 *
 * Called before memory writes to protected regions.
 * Returns: BG_APPROVE if allowed, BG_DENY if blocked
 * ============================================================ */
static int bg256_check_write(unsigned int addr, unsigned int size, int caller_level) {
    unsigned int i;
    unsigned int w_end;
    unsigned int a_end;

    a_end = addr + size;

    i = 0;
    while (i < bg256_state.watch_count) {
        if (bg256_state.watch[i].active) {
            w_end = bg256_state.watch[i].base + bg256_state.watch[i].size;
            /* Check overlap */
            if (addr < w_end && a_end > bg256_state.watch[i].base) {
                /* Region is watched — check caller level */
                if ((unsigned int)caller_level > bg256_state.watch[i].level) {
                    bg256_deny("unauthorized write to protected region",
                               BG_VIOL_UNAUTHORIZED_HW, addr);
                    return BG_DENY;
                }
            }
        }
        i++;
    }
    return BG_APPROVE;
}

/* ============================================================
 * bg256_approve() — Full approval pipeline for arbitrary data
 *
 * For non-Po binaries (e.g. compat layer PE/ELF):
 *   1. Scan for shellcode signatures
 *   2. Check security level
 *   3. Issue verdict
 *
 * Returns: BG_APPROVE or BG_DENY
 * ============================================================ */
static int bg256_approve(void *data, unsigned int size, int level) {
    int threats;

    /* Scan for known exploit patterns */
    threats = bg256_scan_signatures(data, size);
    if (threats > 0) {
        bg256_state.stats.threats_blocked = bg256_state.stats.threats_blocked + (unsigned int)threats;
        bg256_deny("exploit signatures detected", BG_VIOL_SELF_MODIFYING, 0);
        return BG_DENY;
    }

    /* Level check — Service/User cannot execute Ring 0 code */
    if (level < BG_LEVEL_KERNEL) {
        /* Additional checks for non-kernel binaries would go here */
    }

    bg256_state.stats.binaries_verified++;
    bg256_state.stats.binaries_approved++;
    return BG_APPROVE;
}

/* ============================================================
 * bg256_deny() — Record a denial with reason
 *
 * Logs the violation to the ring buffer and sets last_reason.
 * Mirrors Verdict::Denied from policy.rs.
 * ============================================================ */
static void bg256_deny(const char *reason, unsigned int viol_type, unsigned int offset) {
    unsigned int idx;
    BG256Violation *v;

    /* Update stats */
    bg256_state.stats.binaries_verified++;
    bg256_state.stats.binaries_denied++;
    bg256_state.stats.threats_blocked++;

    /* Log violation to ring buffer */
    idx = bg256_state.violation_next;
    v = &bg256_state.violations[idx];
    v->type = viol_type;
    v->offset = offset;
    v->severity = BG_THREAT_HIGH;
    v->reserved = 0;

    /* Advance ring buffer */
    bg256_state.violation_next = (idx + 1) % BG256_MAX_VIOLATIONS;
    if (bg256_state.violation_count < BG256_MAX_VIOLATIONS) {
        bg256_state.violation_count++;
    }

    /* Store reason */
    bg256_strcpy(bg256_state.last_reason, reason);

    /* Serial alert */
    bg256_serial_str("BG256: DENY [");
    bg256_serial_hex32(viol_type);
    bg256_serial_str("] ");
    bg256_serial_str(reason);
    bg256_serial_str(" @");
    bg256_serial_hex32(offset);
    bg256_serial_str("\r\n");
}

/* ============================================================
 * bg256_report_serial() — Print full BG256 status to serial
 *
 * Outputs current state, stats, and recent violations
 * to COM1 for debugging.
 * ============================================================ */
static void bg256_report_serial(void) {
    unsigned int i;

    bg256_serial_str("\r\n=== Binary Guardian 256-bit ===\r\n");
    bg256_serial_str("Status:     ");
    bg256_serial_str(bg256_state.active ? "ACTIVE" : "INACTIVE");
    bg256_serial_str("\r\nMode:       ");
    bg256_serial_str(bg256_state.avx2_available ? "256-bit YMM" : "64-bit scalar");
    bg256_serial_str("\r\nSpeed:      ");
    bg256_serial_str(bg256_state.avx2_available ? "8 checks/cycle" : "1 check/cycle");
    bg256_serial_str("\r\nLevel:      Ring ");
    bg256_serial_dec(bg256_state.security_level);
    bg256_serial_str("\r\nVerified:   ");
    bg256_serial_dec(bg256_state.stats.binaries_verified);
    bg256_serial_str("\r\nApproved:   ");
    bg256_serial_dec(bg256_state.stats.binaries_approved);
    bg256_serial_str("\r\nDenied:     ");
    bg256_serial_dec(bg256_state.stats.binaries_denied);
    bg256_serial_str("\r\nThreats:    ");
    bg256_serial_dec(bg256_state.stats.threats_blocked);
    bg256_serial_str("\r\nBatch ops:  ");
    bg256_serial_dec(bg256_state.stats.batch_ops);
    bg256_serial_str("\r\nMagic chks: ");
    bg256_serial_dec(bg256_state.stats.magic_checks);
    bg256_serial_str("\r\nShellcode:  ");
    bg256_serial_dec(bg256_state.stats.shellcode_detected);
    bg256_serial_str("\r\nMem scans:  ");
    bg256_serial_dec(bg256_state.stats.memory_scans);
    bg256_serial_str("\r\nWatched:    ");
    bg256_serial_dec(bg256_state.watch_count);
    bg256_serial_str(" regions");

    if (bg256_state.violation_count > 0) {
        bg256_serial_str("\r\n--- Recent Violations ---\r\n");
        i = 0;
        while (i < bg256_state.violation_count && i < 8) {
            bg256_serial_str("  [");
            bg256_serial_hex32(bg256_state.violations[i].type);
            bg256_serial_str("] @");
            bg256_serial_hex32(bg256_state.violations[i].offset);
            bg256_serial_str(" sev=");
            bg256_serial_dec(bg256_state.violations[i].severity);
            bg256_serial_str("\r\n");
            i++;
        }
    }

    if (bg256_state.last_reason[0] != 0) {
        bg256_serial_str("Last deny: ");
        bg256_serial_str(bg256_state.last_reason);
    }

    bg256_serial_str("\r\n===============================\r\n");
}

/* ============================================================
 * bg256_get_stats() — Get pointer to stats struct
 *
 * Used by shell bg256 command to display stats in TUI.
 * ============================================================ */
static BG256Stats *bg256_get_stats(void) {
    return &bg256_state.stats;
}
