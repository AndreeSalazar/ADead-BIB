/* ============================================================
 * FastOS Po v8.0 Loader — Implementation
 * ============================================================
 * Loads, verifies, and dispatches .Po executables.
 *
 * Pipeline:
 *   1. po_load(data, size)
 *   2. Parse PoHeaderV8 (32 bytes)
 *   3. bg256_verify_po() — full BG 256-bit pre-execution gate
 *   4. If BG_APPROVE → copy code to execution slot
 *   5. Record in loaded table
 *   6. Return PO_OK
 *
 * Execution slots:
 *   8 slots × 1MB each at PO_EXEC_BASE (16MB)
 *   Slot N address: 0x1000000 + N * 0x100000
 *   No overlap, no MMU — flat physical.
 *
 * Dependencies:
 *   bg256.h/bg256.c — BG 256-bit verification
 *   heap.h/heap.c   — memory allocation (future)
 *
 * Author: Eddi Andreé Salazar Matos — Lima, Perú
 * FastOS v3.1
 * ============================================================ */

/* ============================================================
 * Global state — loaded binary table
 * ============================================================ */
static PoLoaded po_slots[PO_MAX_LOADED];
static int po_slot_count;

/* ============================================================
 * Internal helpers
 * ============================================================ */

/* Copy memory — no libc */
static void po_memcpy(void *dst, const void *src, unsigned int n) {
    unsigned char *d = (unsigned char *)dst;
    const unsigned char *s = (const unsigned char *)src;
    unsigned int i = 0;
    while (i < n) { d[i] = s[i]; i++; }
}

/* Zero memory */
static void po_memset(void *dst, int val, unsigned int n) {
    unsigned char *d = (unsigned char *)dst;
    unsigned int i = 0;
    while (i < n) { d[i] = (unsigned char)val; i++; }
}

/* Serial output helpers */
static void po_serial_str(const char *s) {
    while (*s) { __outb(0x3F8, (unsigned char)*s); s++; }
}

static void po_serial_hex32(unsigned int val) {
    int i; int nibble; char c;
    po_serial_str("0x");
    i = 7;
    while (i >= 0) {
        nibble = (val >> (i * 4)) & 0xF;
        if (nibble < 10) { c = (char)(48 + nibble); }
        else { c = (char)(55 + nibble); }
        __outb(0x3F8, (unsigned char)c);
        i--;
    }
}

static void po_serial_dec(unsigned int val) {
    char buf[12]; int pos = 0; int i;
    if (val == 0) { __outb(0x3F8, '0'); return; }
    while (val > 0) { buf[pos] = (char)(48 + (val % 10)); val = val / 10; pos++; }
    i = pos - 1;
    while (i >= 0) { __outb(0x3F8, (unsigned char)buf[i]); i--; }
}

/* ============================================================
 * po_loader_init() — Initialize the Po loader
 *
 * Zeros all execution slots. Called once at boot.
 * ============================================================ */
static void po_loader_init(void) {
    int i;

    po_slot_count = 0;
    i = 0;
    while (i < PO_MAX_LOADED) {
        po_slots[i].active = 0;
        po_slots[i].exec_addr = 0;
        po_slots[i].exec_size = 0;
        po_slots[i].data_addr = 0;
        po_slots[i].data_size = 0;
        po_slots[i].bits = 0;
        po_slots[i].bg_stamp = 0;
        po_slots[i].hash = 0;
        i++;
    }

    po_serial_str("PoLoader: INIT ");
    po_serial_dec(PO_MAX_LOADED);
    po_serial_str(" slots @ ");
    po_serial_hex32(PO_EXEC_BASE);
    po_serial_str("\r\n");
}

/* ============================================================
 * po_load() — Load and verify a Po binary
 *
 * Full pipeline:
 *   1. Size check (min 32 bytes header)
 *   2. BG 256-bit verification (magic, stamp, signatures)
 *   3. Find free execution slot
 *   4. Copy code section to slot address
 *   5. Copy data section after code
 *   6. Record loaded binary metadata
 *
 * data: pointer to raw Po binary in memory
 * size: total binary size in bytes
 * Returns: PO_OK on success, PO_ERR_* on failure
 * ============================================================ */
static int po_load(void *data, unsigned int size) {
    unsigned char *raw;
    PoHeaderV8 hdr;
    int slot;
    int i;
    unsigned int slot_base;
    unsigned int slot_max;
    unsigned long long full_hash;

    raw = (unsigned char *)data;

    /* Step 1: Size validation */
    if (size < 32) {
        po_serial_str("PoLoader: ERR too small (");
        po_serial_dec(size);
        po_serial_str(" bytes)\r\n");
        return PO_ERR_SIZE;
    }

    if (size > 0x100000) { /* 1MB max per binary */
        po_serial_str("PoLoader: ERR too large\r\n");
        return PO_ERR_SIZE;
    }

    /* Step 2: BG 256-bit verification — THE GATE
     * bg256_verify_po() checks: magic, version, bits, bounds,
     * FNV-1a stamp, shellcode signatures.
     * If denied → binary NEVER loads. Period.
     * "Virus dies before it's born" */
    if (bg256_verify_po(data, size) != BG_APPROVE) {
        po_serial_str("PoLoader: BG256 DENIED\r\n");
        return PO_ERR_BG_DENY;
    }

    /* Step 3: Parse header (BG already validated structure) */
    hdr.magic    = (unsigned int)raw[0]
                 | ((unsigned int)raw[1] << 8)
                 | ((unsigned int)raw[2] << 16)
                 | ((unsigned int)raw[3] << 24);
    hdr.version  = raw[4];
    hdr.bits     = raw[5];
    hdr.ymm_used = (unsigned short)raw[6]
                 | ((unsigned short)raw[7] << 8);
    hdr.code_off = (unsigned int)raw[8]
                 | ((unsigned int)raw[9] << 8)
                 | ((unsigned int)raw[10] << 16)
                 | ((unsigned int)raw[11] << 24);
    hdr.code_size = (unsigned int)raw[12]
                  | ((unsigned int)raw[13] << 8)
                  | ((unsigned int)raw[14] << 16)
                  | ((unsigned int)raw[15] << 24);
    hdr.data_off = (unsigned int)raw[16]
                 | ((unsigned int)raw[17] << 8)
                 | ((unsigned int)raw[18] << 16)
                 | ((unsigned int)raw[19] << 24);
    hdr.data_size = (unsigned int)raw[20]
                  | ((unsigned int)raw[21] << 8)
                  | ((unsigned int)raw[22] << 16)
                  | ((unsigned int)raw[23] << 24);
    hdr.soa_map  = (unsigned int)raw[24]
                 | ((unsigned int)raw[25] << 8)
                 | ((unsigned int)raw[26] << 16)
                 | ((unsigned int)raw[27] << 24);
    hdr.bg_stamp = (unsigned int)raw[28]
                 | ((unsigned int)raw[29] << 8)
                 | ((unsigned int)raw[30] << 16)
                 | ((unsigned int)raw[31] << 24);

    /* Step 4: Find free execution slot */
    slot = -1;
    i = 0;
    while (i < PO_MAX_LOADED) {
        if (po_slots[i].active == 0) {
            slot = i;
            i = PO_MAX_LOADED; /* break */
        }
        i++;
    }

    if (slot < 0) {
        po_serial_str("PoLoader: ERR no free slots\r\n");
        return PO_ERR_FULL;
    }

    /* Step 5: Copy code section to execution slot
     * Slot address: PO_EXEC_BASE + slot * 1MB */
    slot_base = PO_EXEC_BASE + (unsigned int)slot * 0x100000;
    slot_max = 0x100000; /* 1MB per slot */

    /* Bounds check: code + data must fit in slot */
    if (hdr.code_size + hdr.data_size > slot_max) {
        po_serial_str("PoLoader: ERR code+data exceeds slot\r\n");
        return PO_ERR_BOUNDS;
    }

    /* Zero the slot first (clean execution environment) */
    po_memset((void *)((unsigned long long)slot_base), 0, slot_max);

    /* Copy code section */
    if (hdr.code_size > 0) {
        po_memcpy(
            (void *)((unsigned long long)slot_base),
            raw + hdr.code_off,
            hdr.code_size
        );
    }

    /* Copy data section after code */
    if (hdr.data_size > 0) {
        po_memcpy(
            (void *)((unsigned long long)(slot_base + hdr.code_size)),
            raw + hdr.data_off,
            hdr.data_size
        );
    }

    /* Step 6: Compute full hash and record metadata */
    full_hash = bg256_hash_fnv1a(data, size);

    po_slots[slot].active = 1;
    po_slots[slot].exec_addr = slot_base;
    po_slots[slot].exec_size = hdr.code_size;
    po_slots[slot].data_addr = slot_base + hdr.code_size;
    po_slots[slot].data_size = hdr.data_size;
    po_slots[slot].bits = (unsigned int)hdr.bits;
    po_slots[slot].bg_stamp = hdr.bg_stamp;
    po_slots[slot].hash = full_hash;

    po_slot_count++;

    /* Report */
    po_serial_str("PoLoader: LOADED slot ");
    po_serial_dec((unsigned int)slot);
    po_serial_str(" @ ");
    po_serial_hex32(slot_base);
    po_serial_str(" code=");
    po_serial_dec(hdr.code_size);
    po_serial_str(" data=");
    po_serial_dec(hdr.data_size);
    po_serial_str(" bits=");
    po_serial_dec((unsigned int)hdr.bits);
    po_serial_str(" ymm=");
    po_serial_hex32((unsigned int)hdr.ymm_used);
    po_serial_str("\r\n");

    return PO_OK;
}

/* ============================================================
 * po_unload() — Free an execution slot
 * ============================================================ */
static void po_unload(int slot) {
    if (slot < 0 || slot >= PO_MAX_LOADED) return;
    if (po_slots[slot].active == 0) return;

    /* Zero the execution memory (security) */
    po_memset(
        (void *)((unsigned long long)po_slots[slot].exec_addr),
        0,
        0x100000
    );

    po_slots[slot].active = 0;
    po_slots[slot].exec_addr = 0;
    po_slots[slot].exec_size = 0;
    po_slot_count--;

    po_serial_str("PoLoader: UNLOADED slot ");
    po_serial_dec((unsigned int)slot);
    po_serial_str("\r\n");
}

/* ============================================================
 * po_get_loaded() — Get loaded binary info
 * ============================================================ */
static PoLoaded *po_get_loaded(int slot) {
    if (slot < 0 || slot >= PO_MAX_LOADED) return (PoLoaded *)0;
    if (po_slots[slot].active == 0) return (PoLoaded *)0;
    return &po_slots[slot];
}

/* ============================================================
 * po_loaded_count() — Number of currently loaded binaries
 * ============================================================ */
static int po_loaded_count(void) {
    return po_slot_count;
}
