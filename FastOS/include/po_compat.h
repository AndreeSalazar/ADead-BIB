/*
 * FastOS v2.0 — Po Format Compatibility Layer
 * PE/ELF/Win32/Linux compatibility for Po executables
 * 
 * Based on: ReactOS PE loader, Linux ELF loader
 * Philosophy: Po = PE + ELF hybrid with FastOS extensions
 * 
 * Compile: adB cc po_compat.c -o po_compat.o --kernel
 */

#ifndef PO_COMPAT_H
#define PO_COMPAT_H

#include "types.h"

/* ============================================================
 * PE (Windows) Compatibility Structures
 * Based on: ReactOS dll/win32/kernel32/client/loader.c
 * ============================================================ */

/* DOS Header (MZ) */
typedef struct {
    uint16_t e_magic;           /* "MZ" = 0x5A4D */
    uint16_t e_cblp;
    uint16_t e_cp;
    uint16_t e_crlc;
    uint16_t e_cparhdr;
    uint16_t e_minalloc;
    uint16_t e_maxalloc;
    uint16_t e_ss;
    uint16_t e_sp;
    uint16_t e_csum;
    uint16_t e_ip;
    uint16_t e_cs;
    uint16_t e_lfarlc;
    uint16_t e_ovno;
    uint16_t e_res[4];
    uint16_t e_oemid;
    uint16_t e_oeminfo;
    uint16_t e_res2[10];
    uint32_t e_lfanew;          /* Offset to PE header */
} __packed dos_header_t;

/* PE Signature + COFF Header */
typedef struct {
    uint32_t Signature;         /* "PE\0\0" = 0x00004550 */
    uint16_t Machine;
    uint16_t NumberOfSections;
    uint32_t TimeDateStamp;
    uint32_t PointerToSymbolTable;
    uint32_t NumberOfSymbols;
    uint16_t SizeOfOptionalHeader;
    uint16_t Characteristics;
} __packed pe_file_header_t;

/* PE Optional Header (64-bit) */
typedef struct {
    uint16_t Magic;             /* 0x20B for PE32+ */
    uint8_t  MajorLinkerVersion;
    uint8_t  MinorLinkerVersion;
    uint32_t SizeOfCode;
    uint32_t SizeOfInitializedData;
    uint32_t SizeOfUninitializedData;
    uint32_t AddressOfEntryPoint;
    uint32_t BaseOfCode;
    uint64_t ImageBase;
    uint32_t SectionAlignment;
    uint32_t FileAlignment;
    uint16_t MajorOperatingSystemVersion;
    uint16_t MinorOperatingSystemVersion;
    uint16_t MajorImageVersion;
    uint16_t MinorImageVersion;
    uint16_t MajorSubsystemVersion;
    uint16_t MinorSubsystemVersion;
    uint32_t Win32VersionValue;
    uint32_t SizeOfImage;
    uint32_t SizeOfHeaders;
    uint32_t CheckSum;
    uint16_t Subsystem;
    uint16_t DllCharacteristics;
    uint64_t SizeOfStackReserve;
    uint64_t SizeOfStackCommit;
    uint64_t SizeOfHeapReserve;
    uint64_t SizeOfHeapCommit;
    uint32_t LoaderFlags;
    uint32_t NumberOfRvaAndSizes;
} __packed pe_optional_header64_t;

/* PE Section Header */
typedef struct {
    char     Name[8];
    uint32_t VirtualSize;
    uint32_t VirtualAddress;
    uint32_t SizeOfRawData;
    uint32_t PointerToRawData;
    uint32_t PointerToRelocations;
    uint32_t PointerToLinenumbers;
    uint16_t NumberOfRelocations;
    uint16_t NumberOfLinenumbers;
    uint32_t Characteristics;
} __packed pe_section_header_t;

/* PE Import Directory */
typedef struct {
    uint32_t OriginalFirstThunk;
    uint32_t TimeDateStamp;
    uint32_t ForwarderChain;
    uint32_t Name;
    uint32_t FirstThunk;
} __packed pe_import_descriptor_t;

/* PE Export Directory */
typedef struct {
    uint32_t Characteristics;
    uint32_t TimeDateStamp;
    uint16_t MajorVersion;
    uint16_t MinorVersion;
    uint32_t Name;
    uint32_t Base;
    uint32_t NumberOfFunctions;
    uint32_t NumberOfNames;
    uint32_t AddressOfFunctions;
    uint32_t AddressOfNames;
    uint32_t AddressOfNameOrdinals;
} __packed pe_export_directory_t;

/* ============================================================
 * ELF (Linux) Compatibility Structures
 * Based on: Linux kernel fs/binfmt_elf.c
 * ============================================================ */

/* ELF Header (64-bit) */
typedef struct {
    uint8_t  e_ident[16];       /* ELF magic + class + endian */
    uint16_t e_type;
    uint16_t e_machine;
    uint32_t e_version;
    uint64_t e_entry;
    uint64_t e_phoff;
    uint64_t e_shoff;
    uint32_t e_flags;
    uint16_t e_ehsize;
    uint16_t e_phentsize;
    uint16_t e_phnum;
    uint16_t e_shentsize;
    uint16_t e_shnum;
    uint16_t e_shstrndx;
} __packed elf64_header_t;

/* ELF Program Header */
typedef struct {
    uint32_t p_type;
    uint32_t p_flags;
    uint64_t p_offset;
    uint64_t p_vaddr;
    uint64_t p_paddr;
    uint64_t p_filesz;
    uint64_t p_memsz;
    uint64_t p_align;
} __packed elf64_phdr_t;

/* ELF Section Header */
typedef struct {
    uint32_t sh_name;
    uint32_t sh_type;
    uint64_t sh_flags;
    uint64_t sh_addr;
    uint64_t sh_offset;
    uint64_t sh_size;
    uint32_t sh_link;
    uint32_t sh_info;
    uint64_t sh_addralign;
    uint64_t sh_entsize;
} __packed elf64_shdr_t;

/* ELF Symbol */
typedef struct {
    uint32_t st_name;
    uint8_t  st_info;
    uint8_t  st_other;
    uint16_t st_shndx;
    uint64_t st_value;
    uint64_t st_size;
} __packed elf64_sym_t;

/* ELF Dynamic Entry */
typedef struct {
    int64_t  d_tag;
    uint64_t d_val;
} __packed elf64_dyn_t;

/* ============================================================
 * Po Format — Unified Header (PE + ELF + FastOS)
 * ============================================================ */

#define PO_MAGIC        0x506F4F53  /* "PoOS" */
#define PO_VERSION      0x0200

/* Po can wrap PE, ELF, or be native */
typedef enum {
    PO_FORMAT_NATIVE    = 0,    /* Pure Po format */
    PO_FORMAT_PE        = 1,    /* PE wrapper */
    PO_FORMAT_ELF       = 2,    /* ELF wrapper */
    PO_FORMAT_PE_ELF    = 3,    /* Dual PE+ELF (fat binary) */
} po_format_t;

/* Po Unified Header */
typedef struct {
    /* Po identification */
    uint32_t po_magic;          /* "PoOS" */
    uint16_t po_version;        /* Format version */
    uint16_t po_format;         /* po_format_t */
    
    /* Compatibility info */
    uint32_t pe_offset;         /* Offset to PE header (0 if none) */
    uint32_t elf_offset;        /* Offset to ELF header (0 if none) */
    uint32_t po_native_offset;  /* Offset to native Po data */
    
    /* FastOS extensions */
    uint64_t entry_point;       /* Entry point RVA */
    uint64_t base_address;      /* Preferred load address */
    uint32_t image_size;        /* Total image size */
    uint32_t flags;             /* Po flags */
    
    /* Binary Guardian */
    uint8_t  security_level;    /* BG security level */
    uint8_t  bg_verified;       /* BG verification status */
    uint16_t bg_flags;          /* BG flags */
    uint32_t bg_hash[8];        /* SHA-256 hash */
    
    /* Rust safety layer */
    uint32_t rust_metadata;     /* Rust safety metadata offset */
    uint32_t rust_size;         /* Rust metadata size */
    
    /* Reserved */
    uint8_t  reserved[32];
} __packed po_unified_header_t;

/* ============================================================
 * Win32 API Compatibility Layer
 * Based on: ReactOS kernel32
 * ============================================================ */

/* Win32 Handle Types */
typedef void* HANDLE;
typedef void* HMODULE;
typedef void* HINSTANCE;
typedef uint32_t DWORD;
typedef int32_t LONG;
typedef uint16_t WORD;
typedef uint8_t BYTE;
typedef int BOOL;
typedef char* LPSTR;
typedef const char* LPCSTR;
typedef void* LPVOID;

#define TRUE  1
#define FALSE 0
#define NULL  ((void*)0)

#define INVALID_HANDLE_VALUE ((HANDLE)-1)

/* Win32 LoadLibrary flags */
#define LOAD_LIBRARY_AS_DATAFILE        0x00000002
#define LOAD_WITH_ALTERED_SEARCH_PATH   0x00000008
#define LOAD_IGNORE_CODE_AUTHZ_LEVEL    0x00000010
#define LOAD_LIBRARY_AS_IMAGE_RESOURCE  0x00000020

/* Win32 Memory Protection */
#define PAGE_NOACCESS           0x01
#define PAGE_READONLY           0x02
#define PAGE_READWRITE          0x04
#define PAGE_WRITECOPY          0x08
#define PAGE_EXECUTE            0x10
#define PAGE_EXECUTE_READ       0x20
#define PAGE_EXECUTE_READWRITE  0x40

/* Win32 Memory Allocation */
#define MEM_COMMIT              0x1000
#define MEM_RESERVE             0x2000
#define MEM_DECOMMIT            0x4000
#define MEM_RELEASE             0x8000

/* ============================================================
 * Po Compatibility Functions
 * ============================================================ */

/* Detect binary format */
po_format_t po_detect_format(void *data, size_t size);

/* Load PE executable */
int po_load_pe(void *data, size_t size, uint64_t *entry, uint64_t *base);

/* Load ELF executable */
int po_load_elf(void *data, size_t size, uint64_t *entry, uint64_t *base);

/* Load native Po executable */
int po_load_native(void *data, size_t size, uint64_t *entry, uint64_t *base);

/* Unified loader */
int po_load_any(void *data, size_t size, uint64_t *entry, uint64_t *base);

/* Win32 API emulation */
HMODULE po_LoadLibraryA(LPCSTR lpLibFileName);
HMODULE po_LoadLibraryExA(LPCSTR lpLibFileName, HANDLE hFile, DWORD dwFlags);
BOOL po_FreeLibrary(HMODULE hLibModule);
void* po_GetProcAddress(HMODULE hModule, LPCSTR lpProcName);

/* ELF dynamic linker */
int po_elf_relocate(void *base, elf64_dyn_t *dynamic);
void* po_elf_dlopen(const char *filename, int flags);
void* po_elf_dlsym(void *handle, const char *symbol);
int po_elf_dlclose(void *handle);

/* ============================================================
 * Po Section Types (unified)
 * ============================================================ */

#define PO_SEC_CODE     0x00000001  /* Executable code */
#define PO_SEC_DATA     0x00000002  /* Initialized data */
#define PO_SEC_BSS      0x00000004  /* Uninitialized data */
#define PO_SEC_RODATA   0x00000008  /* Read-only data */
#define PO_SEC_IMPORT   0x00000010  /* Import table */
#define PO_SEC_EXPORT   0x00000020  /* Export table */
#define PO_SEC_RELOC    0x00000040  /* Relocations */
#define PO_SEC_DEBUG    0x00000080  /* Debug info */
#define PO_SEC_RUST     0x00000100  /* Rust metadata */
#define PO_SEC_BG       0x00000200  /* Binary Guardian data */

/* ============================================================
 * Po Subsystems (like PE subsystem)
 * ============================================================ */

#define PO_SUBSYS_NATIVE        0   /* FastOS native */
#define PO_SUBSYS_CONSOLE       1   /* Console application */
#define PO_SUBSYS_GUI           2   /* GUI application */
#define PO_SUBSYS_DRIVER        3   /* Kernel driver */
#define PO_SUBSYS_WIN32         4   /* Win32 compatibility */
#define PO_SUBSYS_POSIX         5   /* POSIX compatibility */

#endif /* PO_COMPAT_H */
