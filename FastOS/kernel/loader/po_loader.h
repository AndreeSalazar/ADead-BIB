/* ============================================================
 * FastOS Po v8.0 Loader — Header
 * ============================================================
 * Loads and verifies .Po executables (FastOS native format).
 *
 * Po v8.0 header: 32 bytes
 *   Offset  Size  Field
 *   ------  ----  ----------------
 *     0       4   magic: 0x506F4F53 ("PoOS")
 *     4       1   version: 0x80 = v8.0
 *     5       1   bits: 16|64|128|0xFF=256
 *     6       2   ymm_used: bitmask YMM0-YMM15
 *     8       4   code_offset
 *    12       4   code_size
 *    16       4   data_offset
 *    20       4   data_size
 *    24       4   soa_map: offset to SoA descriptor
 *    28       4   bg_stamp: FNV-1a hash for BG
 *
 * Load pipeline:
 *   1. Read PoHeaderV8 from data
 *   2. bg256_verify_po() — full BG 256-bit verification
 *   3. If APPROVE → copy code/data to execution region
 *   4. Dispatch based on bits field (64 vs 256)
 *   5. Return entry point address
 *
 * "Virus dies before it's born" — BG256 verifies BEFORE load.
 *
 * Author: Eddi Andreé Salazar Matos — Lima, Perú
 * FastOS v3.1
 * ============================================================ */

#ifndef PO_LOADER_H
#define PO_LOADER_H

/* ============================================================
 * PoHeaderV8 — Po executable format v8.0 (32 bytes)
 * ============================================================ */
typedef struct {
    unsigned int   magic;       /* 0x506F4F53 "PoOS" */
    unsigned char  version;     /* 0x80 = v8.0 */
    unsigned char  bits;        /* 16/64/128/255(=256) */
    unsigned short ymm_used;    /* bitmask YMM0-YMM15 */
    unsigned int   code_off;    /* offset to code section */
    unsigned int   code_size;   /* code section size */
    unsigned int   data_off;    /* offset to data section */
    unsigned int   data_size;   /* data section size */
    unsigned int   soa_map;     /* offset to SoA descriptor table */
    unsigned int   bg_stamp;    /* FNV-1a hash — BG verification */
} PoHeaderV8;

/* Po execution region — where loaded code lives */
#define PO_EXEC_BASE    0x1000000   /* 16MB — execution space start */
#define PO_EXEC_SIZE    0x0800000   /* 8MB available for loaded code */
#define PO_MAX_LOADED   8           /* max simultaneously loaded .Po */

/* Load result codes */
#define PO_OK           0   /* loaded and verified successfully */
#define PO_ERR_SIZE     1   /* binary too small or too large */
#define PO_ERR_BG_DENY  2   /* BG 256-bit denied the binary */
#define PO_ERR_VERSION  3   /* unsupported Po version */
#define PO_ERR_BOUNDS   4   /* code/data section out of bounds */
#define PO_ERR_FULL     5   /* no free execution slots */
#define PO_ERR_BITS     6   /* unsupported bits mode */

/* Loaded binary descriptor */
typedef struct {
    unsigned int active;        /* 1 = slot in use */
    unsigned int exec_addr;     /* physical address of loaded code */
    unsigned int exec_size;     /* size of loaded code */
    unsigned int data_addr;     /* physical address of loaded data */
    unsigned int data_size;     /* size of loaded data */
    unsigned int bits;          /* 64 or 256 mode */
    unsigned int bg_stamp;      /* verified BG stamp */
    unsigned long long hash;    /* full FNV-1a hash */
} PoLoaded;

/* ============================================================
 * Function Prototypes
 * ============================================================ */

/* Initialize loader — call once at boot */
static void po_loader_init(void);

/* Load a Po binary from memory
 * data: raw binary data (starts with PoHeaderV8)
 * size: total binary size
 * Returns: PO_OK on success, PO_ERR_* on failure */
static int po_load(void *data, unsigned int size);

/* Unload a previously loaded binary by slot index */
static void po_unload(int slot);

/* Get info about a loaded binary */
static PoLoaded *po_get_loaded(int slot);

/* Get number of currently loaded binaries */
static int po_loaded_count(void);

#endif /* PO_LOADER_H */
