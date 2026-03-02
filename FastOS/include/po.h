/*
 * FastOS v2.0 — Po Executable Format
 * Portable Object - Native executable format for FastOS
 * 
 * Hybrid design inspired by:
 * - Windows PE (Portable Executable)
 * - Linux ELF (Executable and Linkable Format)
 * - Win32 API compatibility layer
 * 
 * Features:
 * - Binary Guardian (BG) integration for security
 * - Native FastOS syscall interface
 * - Win32 API translation layer
 * - ELF-style dynamic linking
 * - Security levels (Kernel, Driver, Service, User, Sandbox)
 * - VBE/GPU driver loading support
 * - Nouveau NVIDIA integration
 * 
 * Compiled by: ADead-BIB (C is Master, Rust is Safety)
 */

#ifndef _FASTOS_PO_H
#define _FASTOS_PO_H

#include "types.h"

/* ============================================================
 * Po Magic Numbers
 * ============================================================ */

#define PO_MAGIC        0x506F4F53  /* "PoOS" in little-endian */
#define PO_VERSION      0x0100      /* Version 1.0 */

/* ============================================================
 * Po File Types (inspired by ELF e_type + PE characteristics)
 * ============================================================ */

typedef enum {
    PO_TYPE_NONE       = 0,    /* Unknown/Invalid */
    PO_TYPE_EXEC       = 1,    /* Executable (like ELF ET_EXEC, PE EXE) */
    PO_TYPE_DYN        = 2,    /* Shared library (like ELF ET_DYN, PE DLL) */
    PO_TYPE_DRIVER     = 3,    /* Kernel driver (like Windows SYS) */
    PO_TYPE_KERNEL     = 4,    /* Kernel module */
    PO_TYPE_OBJECT     = 5,    /* Relocatable object (like ELF ET_REL) */
    PO_TYPE_CORE       = 6,    /* Core dump */
} po_type_t;

/* ============================================================
 * Po Machine Types (x86-64 focused, expandable)
 * ============================================================ */

typedef enum {
    PO_MACHINE_NONE    = 0,
    PO_MACHINE_X86     = 1,    /* i386 */
    PO_MACHINE_X64     = 2,    /* x86-64 (AMD64) */
    PO_MACHINE_ARM     = 3,    /* ARM 32-bit */
    PO_MACHINE_ARM64   = 4,    /* ARM 64-bit (AArch64) */
    PO_MACHINE_RISCV   = 5,    /* RISC-V */
} po_machine_t;

/* ============================================================
 * Po Security Levels (integrated with BG — Binary Guardian)
 * ============================================================ */

typedef enum {
    PO_SECURITY_KERNEL   = 0,  /* Ring 0 — Full access */
    PO_SECURITY_DRIVER   = 1,  /* Ring 1 — IO + restricted */
    PO_SECURITY_SERVICE  = 2,  /* Ring 2 — Syscalls only */
    PO_SECURITY_USER     = 3,  /* Ring 3 — Safe + syscalls */
    PO_SECURITY_SANDBOX  = 4,  /* Ring 3 — Minimal, no syscalls */
} po_security_t;

/* ============================================================
 * Po Flags
 * ============================================================ */

#define PO_FLAG_EXECUTABLE    (1 << 0)   /* Can be executed */
#define PO_FLAG_RELOCATABLE   (1 << 1)   /* Position independent */
#define PO_FLAG_SIGNED        (1 << 2)   /* Has digital signature */
#define PO_FLAG_COMPRESSED    (1 << 3)   /* Sections are compressed */
#define PO_FLAG_DEBUG         (1 << 4)   /* Contains debug info */
#define PO_FLAG_STRIPPED      (1 << 5)   /* Symbols stripped */
#define PO_FLAG_BG_VERIFIED   (1 << 6)   /* Pre-verified by BG */
#define PO_FLAG_STATIC        (1 << 7)   /* Statically linked */

/* ============================================================
 * Po Header — Main file header (64 bytes)
 * ============================================================ */

typedef struct {
    /* Identification (16 bytes) */
    uint32_t magic;              /* PO_MAGIC = "PoOS" */
    uint16_t version;            /* Format version */
    uint8_t  type;               /* po_type_t */
    uint8_t  machine;            /* po_machine_t */
    uint8_t  security;           /* po_security_t — required level */
    uint8_t  endian;             /* 0 = little, 1 = big */
    uint8_t  word_size;          /* 32 or 64 */
    uint8_t  abi_version;        /* ABI version */
    uint32_t flags;              /* PO_FLAG_* */
    
    /* Entry point and sections (24 bytes) */
    uint64_t entry;              /* Entry point virtual address */
    uint32_t section_offset;     /* Offset to section table */
    uint16_t section_count;      /* Number of sections */
    uint16_t section_size;       /* Size of each section header */
    uint32_t string_table;       /* Offset to string table */
    
    /* Sizes (16 bytes) */
    uint64_t image_size;         /* Total image size in memory */
    uint32_t header_size;        /* Size of this header */
    uint32_t checksum;           /* CRC32 of entire file */
    
    /* Reserved (8 bytes) */
    uint64_t reserved;
} __packed po_header_t;

_Static_assert(sizeof(po_header_t) == 64, "po_header_t must be 64 bytes");

/* ============================================================
 * Po Section Header (48 bytes)
 * ============================================================ */

typedef enum {
    PO_SECTION_NULL     = 0,    /* Unused */
    PO_SECTION_CODE     = 1,    /* Executable code (.text) */
    PO_SECTION_DATA     = 2,    /* Initialized data (.data) */
    PO_SECTION_RODATA   = 3,    /* Read-only data (.rodata) */
    PO_SECTION_BSS      = 4,    /* Uninitialized data (.bss) */
    PO_SECTION_SYMTAB   = 5,    /* Symbol table */
    PO_SECTION_STRTAB   = 6,    /* String table */
    PO_SECTION_RELA     = 7,    /* Relocations with addend */
    PO_SECTION_DYNAMIC  = 8,    /* Dynamic linking info */
    PO_SECTION_NOTE     = 9,    /* Notes/metadata */
    PO_SECTION_IMPORT   = 10,   /* Import table (like PE) */
    PO_SECTION_EXPORT   = 11,   /* Export table (like PE) */
    PO_SECTION_RESOURCE = 12,   /* Resources (icons, etc.) */
    PO_SECTION_DEBUG    = 13,   /* Debug information */
    PO_SECTION_BG_MAP   = 14,   /* BG Architecture Map (pre-computed) */
} po_section_type_t;

/* Section flags */
#define PO_SHF_WRITE      (1 << 0)   /* Writable */
#define PO_SHF_ALLOC      (1 << 1)   /* Occupies memory */
#define PO_SHF_EXEC       (1 << 2)   /* Executable */
#define PO_SHF_MERGE      (1 << 3)   /* Can be merged */
#define PO_SHF_STRINGS    (1 << 4)   /* Contains strings */
#define PO_SHF_COMPRESSED (1 << 5)   /* Compressed */

typedef struct {
    uint32_t name;               /* Offset into string table */
    uint32_t type;               /* po_section_type_t */
    uint64_t flags;              /* PO_SHF_* */
    uint64_t vaddr;              /* Virtual address */
    uint64_t offset;             /* File offset */
    uint64_t size;               /* Size in file */
    uint64_t mem_size;           /* Size in memory (>= size for BSS) */
    uint32_t link;               /* Related section index */
    uint32_t info;               /* Extra info */
    uint64_t align;              /* Alignment requirement */
} __packed po_section_t;

_Static_assert(sizeof(po_section_t) == 56, "po_section_t must be 56 bytes");

/* ============================================================
 * Po Symbol (24 bytes)
 * ============================================================ */

typedef enum {
    PO_SYM_NOTYPE  = 0,
    PO_SYM_OBJECT  = 1,    /* Data object */
    PO_SYM_FUNC    = 2,    /* Function */
    PO_SYM_SECTION = 3,    /* Section */
    PO_SYM_FILE    = 4,    /* Source file */
} po_sym_type_t;

typedef enum {
    PO_SYM_LOCAL   = 0,    /* Local symbol */
    PO_SYM_GLOBAL  = 1,    /* Global symbol */
    PO_SYM_WEAK    = 2,    /* Weak symbol */
    PO_SYM_EXPORT  = 3,    /* Exported symbol */
    PO_SYM_IMPORT  = 4,    /* Imported symbol */
} po_sym_bind_t;

typedef struct {
    uint32_t name;         /* Offset into string table */
    uint8_t  type;         /* po_sym_type_t */
    uint8_t  bind;         /* po_sym_bind_t */
    uint16_t section;      /* Section index */
    uint64_t value;        /* Symbol value/address */
    uint64_t size;         /* Symbol size */
} __packed po_symbol_t;

_Static_assert(sizeof(po_symbol_t) == 24, "po_symbol_t must be 24 bytes");

/* ============================================================
 * Po Relocation (24 bytes)
 * ============================================================ */

typedef enum {
    PO_REL_NONE     = 0,
    PO_REL_64       = 1,    /* 64-bit absolute */
    PO_REL_32       = 2,    /* 32-bit absolute */
    PO_REL_32S      = 3,    /* 32-bit signed */
    PO_REL_PC32     = 4,    /* PC-relative 32-bit */
    PO_REL_PC64     = 5,    /* PC-relative 64-bit */
    PO_REL_GOT32    = 6,    /* GOT entry */
    PO_REL_PLT32    = 7,    /* PLT entry */
} po_rel_type_t;

typedef struct {
    uint64_t offset;       /* Location to apply relocation */
    uint32_t type;         /* po_rel_type_t */
    uint32_t symbol;       /* Symbol index */
    int64_t  addend;       /* Addend value */
} __packed po_reloc_t;

_Static_assert(sizeof(po_reloc_t) == 24, "po_reloc_t must be 24 bytes");

/* ============================================================
 * Po Import/Export Entry (16 bytes)
 * ============================================================ */

typedef struct {
    uint32_t name;         /* Symbol name offset */
    uint32_t library;      /* Library name offset (for imports) */
    uint64_t address;      /* Address/ordinal */
} __packed po_import_t;

typedef struct {
    uint32_t name;         /* Symbol name offset */
    uint32_t ordinal;      /* Export ordinal */
    uint64_t address;      /* Function address */
} __packed po_export_t;

/* ============================================================
 * Po BG Map Section — Pre-computed Binary Guardian analysis
 * ============================================================ */

typedef struct {
    /* Instruction classification counts */
    uint32_t safe_count;
    uint32_t restricted_count;
    uint32_t privileged_count;
    
    /* Memory characteristics */
    uint32_t rwx_regions;        /* Suspicious RWX sections */
    uint32_t self_modifying;     /* Self-modifying code detected */
    
    /* Syscall/IO info */
    uint32_t syscall_count;
    uint32_t io_port_count;
    
    /* Control flow */
    uint32_t indirect_calls;
    uint32_t indirect_jumps;
    uint32_t far_jumps;
    
    /* Security verdict */
    uint8_t  max_security_level; /* Minimum required security level */
    uint8_t  verified;           /* 1 if BG verified this map */
    uint16_t reserved;
    
    /* Hash of code section for integrity */
    uint8_t  code_hash[32];      /* SHA-256 of code section */
} __packed po_bg_map_t;

/* ============================================================
 * Po Loader Functions
 * ============================================================ */

/* Validate Po header */
int po_validate(const po_header_t *header);

/* Get section by type */
const po_section_t* po_get_section(const po_header_t *header, 
                                    po_section_type_t type);

/* Get section by name */
const po_section_t* po_get_section_by_name(const po_header_t *header,
                                            const char *name);

/* Get string from string table */
const char* po_get_string(const po_header_t *header, uint32_t offset);

/* Load Po file into memory */
int po_load(const uint8_t *file_data, size_t file_size, 
            void **load_addr, size_t *load_size);

/* Verify with BG before execution */
int po_verify_bg(const po_header_t *header, po_security_t required_level);

/* ============================================================
 * Po Builder Functions (for compiler)
 * ============================================================ */

/* Create new Po file */
po_header_t* po_create(po_type_t type, po_machine_t machine);

/* Add section */
int po_add_section(po_header_t *header, const char *name,
                   po_section_type_t type, uint64_t flags,
                   const void *data, size_t size);

/* Add symbol */
int po_add_symbol(po_header_t *header, const char *name,
                  po_sym_type_t type, po_sym_bind_t bind,
                  uint16_t section, uint64_t value, uint64_t size);

/* Finalize and write */
int po_finalize(po_header_t *header, uint8_t **output, size_t *output_size);

#endif /* _FASTOS_PO_H */
