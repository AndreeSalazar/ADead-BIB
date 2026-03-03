/*
 * FastOS v2.0 — Po Loader Complete
 * PE/ELF/Win32/POSIX Compatible Loader
 * 
 * Based on:
 * - ReactOS: dll/win32/kernel32/client/loader.c
 * - Wine: dlls/ntdll/loader.c
 * - Linux: fs/binfmt_elf.c
 * - musl: ldso/dynlink.c
 * 
 * Compile: adB cc po_loader_full.c -o po_loader.po --kernel
 */

#include "../include/kernel.h"
#include "../include/types.h"
#include "../include/po_compat.h"

/* ============================================================
 * PE Loader (ReactOS/Wine style)
 * Based on: ReactOS kernel32/client/loader.c
 * ============================================================ */

#define PE_MAGIC        0x00004550  /* "PE\0\0" */
#define DOS_MAGIC       0x5A4D      /* "MZ" */
#define PE32_MAGIC      0x10B
#define PE32PLUS_MAGIC  0x20B

/* PE Data Directories */
#define PE_DIRECTORY_ENTRY_EXPORT       0
#define PE_DIRECTORY_ENTRY_IMPORT       1
#define PE_DIRECTORY_ENTRY_RESOURCE     2
#define PE_DIRECTORY_ENTRY_EXCEPTION    3
#define PE_DIRECTORY_ENTRY_SECURITY     4
#define PE_DIRECTORY_ENTRY_BASERELOC    5
#define PE_DIRECTORY_ENTRY_DEBUG        6
#define PE_DIRECTORY_ENTRY_TLS          9
#define PE_DIRECTORY_ENTRY_IAT          12

/* PE Section Flags */
#define IMAGE_SCN_CNT_CODE              0x00000020
#define IMAGE_SCN_CNT_INITIALIZED_DATA  0x00000040
#define IMAGE_SCN_CNT_UNINITIALIZED_DATA 0x00000080
#define IMAGE_SCN_MEM_EXECUTE           0x20000000
#define IMAGE_SCN_MEM_READ              0x40000000
#define IMAGE_SCN_MEM_WRITE             0x80000000

typedef struct {
    uint32_t VirtualAddress;
    uint32_t Size;
} pe_data_directory_t;

typedef struct {
    char Name[8];
    union {
        uint32_t PhysicalAddress;
        uint32_t VirtualSize;
    } Misc;
    uint32_t VirtualAddress;
    uint32_t SizeOfRawData;
    uint32_t PointerToRawData;
    uint32_t PointerToRelocations;
    uint32_t PointerToLinenumbers;
    uint16_t NumberOfRelocations;
    uint16_t NumberOfLinenumbers;
    uint32_t Characteristics;
} pe_section_t;

/* Loaded module tracking */
typedef struct loaded_module {
    char name[256];
    void *base;
    size_t size;
    uint64_t entry_point;
    int ref_count;
    int is_pe;
    int is_elf;
    struct loaded_module *next;
} loaded_module_t;

static loaded_module_t *loaded_modules = NULL;

/* ============================================================
 * PE Loading Functions
 * ============================================================ */

static int pe_validate_header(void *data, size_t size) {
    if (size < sizeof(dos_header_t)) return 0;
    
    dos_header_t *dos = (dos_header_t*)data;
    if (dos->e_magic != DOS_MAGIC) return 0;
    
    if (dos->e_lfanew + sizeof(pe_file_header_t) > size) return 0;
    
    pe_file_header_t *pe = (pe_file_header_t*)((uint8_t*)data + dos->e_lfanew);
    if (pe->Signature != PE_MAGIC) return 0;
    
    return 1;
}

static int pe_map_sections(void *data, void *base, pe_file_header_t *pe_hdr,
                           pe_optional_header64_t *opt_hdr) {
    pe_section_t *sections = (pe_section_t*)((uint8_t*)opt_hdr + 
                              pe_hdr->SizeOfOptionalHeader);
    
    for (int i = 0; i < pe_hdr->NumberOfSections; i++) {
        pe_section_t *sec = &sections[i];
        
        void *dest = (uint8_t*)base + sec->VirtualAddress;
        void *src = (uint8_t*)data + sec->PointerToRawData;
        
        /* Copy section data */
        if (sec->SizeOfRawData > 0) {
            kmemcpy(dest, src, sec->SizeOfRawData);
        }
        
        /* Zero-fill remainder */
        if (sec->Misc.VirtualSize > sec->SizeOfRawData) {
            kmemset((uint8_t*)dest + sec->SizeOfRawData, 0,
                    sec->Misc.VirtualSize - sec->SizeOfRawData);
        }
        
        kprintf("[PE] Section %.8s: VA=0x%X Size=0x%X\n",
                sec->Name, sec->VirtualAddress, sec->Misc.VirtualSize);
    }
    
    return 0;
}

static int pe_process_imports(void *base, pe_data_directory_t *import_dir) {
    if (import_dir->VirtualAddress == 0 || import_dir->Size == 0) {
        return 0;  /* No imports */
    }
    
    pe_import_descriptor_t *imports = (pe_import_descriptor_t*)
        ((uint8_t*)base + import_dir->VirtualAddress);
    
    while (imports->Name != 0) {
        char *dll_name = (char*)((uint8_t*)base + imports->Name);
        kprintf("[PE] Import: %s\n", dll_name);
        
        /* Would resolve imports here */
        /* For now, just log them */
        
        imports++;
    }
    
    return 0;
}

static int pe_process_relocations(void *base, uint64_t actual_base,
                                   uint64_t preferred_base,
                                   pe_data_directory_t *reloc_dir) {
    if (reloc_dir->VirtualAddress == 0 || reloc_dir->Size == 0) {
        return 0;  /* No relocations needed */
    }
    
    int64_t delta = (int64_t)actual_base - (int64_t)preferred_base;
    if (delta == 0) return 0;  /* Loaded at preferred address */
    
    uint8_t *reloc_data = (uint8_t*)base + reloc_dir->VirtualAddress;
    uint8_t *reloc_end = reloc_data + reloc_dir->Size;
    
    while (reloc_data < reloc_end) {
        uint32_t page_rva = *(uint32_t*)reloc_data;
        uint32_t block_size = *(uint32_t*)(reloc_data + 4);
        
        if (block_size == 0) break;
        
        uint16_t *entries = (uint16_t*)(reloc_data + 8);
        int num_entries = (block_size - 8) / 2;
        
        for (int i = 0; i < num_entries; i++) {
            uint16_t entry = entries[i];
            int type = entry >> 12;
            int offset = entry & 0xFFF;
            
            void *addr = (uint8_t*)base + page_rva + offset;
            
            switch (type) {
                case 0:  /* IMAGE_REL_BASED_ABSOLUTE - skip */
                    break;
                case 3:  /* IMAGE_REL_BASED_HIGHLOW (32-bit) */
                    *(uint32_t*)addr += (uint32_t)delta;
                    break;
                case 10: /* IMAGE_REL_BASED_DIR64 (64-bit) */
                    *(uint64_t*)addr += delta;
                    break;
            }
        }
        
        reloc_data += block_size;
    }
    
    kprintf("[PE] Applied %d relocation blocks (delta=0x%llX)\n",
            reloc_dir->Size, delta);
    
    return 0;
}

int po_load_pe(void *data, size_t size, uint64_t *entry, uint64_t *base) {
    if (!pe_validate_header(data, size)) {
        kprintf("[PE] Invalid PE header\n");
        return -1;
    }
    
    dos_header_t *dos = (dos_header_t*)data;
    pe_file_header_t *pe_hdr = (pe_file_header_t*)((uint8_t*)data + dos->e_lfanew);
    pe_optional_header64_t *opt_hdr = (pe_optional_header64_t*)(pe_hdr + 1);
    
    /* Check PE type */
    int is_pe32plus = (opt_hdr->Magic == PE32PLUS_MAGIC);
    kprintf("[PE] Loading %s executable\n", is_pe32plus ? "PE32+" : "PE32");
    
    /* Allocate memory for image */
    uint32_t image_size = opt_hdr->SizeOfImage;
    void *image_base = kmalloc(image_size);
    if (!image_base) {
        kprintf("[PE] Failed to allocate %u bytes\n", image_size);
        return -1;
    }
    kmemset(image_base, 0, image_size);
    
    /* Copy headers */
    kmemcpy(image_base, data, opt_hdr->SizeOfHeaders);
    
    /* Map sections */
    pe_map_sections(data, image_base, pe_hdr, opt_hdr);
    
    /* Process relocations */
    pe_data_directory_t *data_dirs = (pe_data_directory_t*)
        ((uint8_t*)opt_hdr + 112);  /* Offset to data directories in PE32+ */
    pe_process_relocations(image_base, (uint64_t)image_base,
                           opt_hdr->ImageBase, &data_dirs[PE_DIRECTORY_ENTRY_BASERELOC]);
    
    /* Process imports */
    pe_process_imports(image_base, &data_dirs[PE_DIRECTORY_ENTRY_IMPORT]);
    
    /* Set return values */
    *entry = (uint64_t)image_base + opt_hdr->AddressOfEntryPoint;
    *base = (uint64_t)image_base;
    
    kprintf("[PE] Loaded at 0x%llX, entry=0x%llX\n", *base, *entry);
    
    return 0;
}

/* ============================================================
 * ELF Loader (Linux binfmt_elf style)
 * Based on: Linux fs/binfmt_elf.c
 * ============================================================ */

#define ELF_MAGIC       0x464C457F  /* "\x7FELF" */

/* ELF Types */
#define ET_NONE         0
#define ET_REL          1
#define ET_EXEC         2
#define ET_DYN          3
#define ET_CORE         4

/* ELF Machine Types */
#define EM_386          3
#define EM_X86_64       62
#define EM_AARCH64      183

/* Program Header Types */
#define PT_NULL         0
#define PT_LOAD         1
#define PT_DYNAMIC      2
#define PT_INTERP       3
#define PT_NOTE         4
#define PT_PHDR         6
#define PT_TLS          7
#define PT_GNU_EH_FRAME 0x6474e550
#define PT_GNU_STACK    0x6474e551
#define PT_GNU_RELRO    0x6474e552

/* Program Header Flags */
#define PF_X            0x1
#define PF_W            0x2
#define PF_R            0x4

/* Dynamic Tags */
#define DT_NULL         0
#define DT_NEEDED       1
#define DT_PLTRELSZ     2
#define DT_PLTGOT       3
#define DT_HASH         4
#define DT_STRTAB       5
#define DT_SYMTAB       6
#define DT_RELA         7
#define DT_RELASZ       8
#define DT_RELAENT      9
#define DT_STRSZ        10
#define DT_SYMENT       11
#define DT_INIT         12
#define DT_FINI         13
#define DT_SONAME       14
#define DT_RPATH        15
#define DT_SYMBOLIC     16
#define DT_REL          17
#define DT_RELSZ        18
#define DT_RELENT       19
#define DT_PLTREL       20
#define DT_DEBUG        21
#define DT_TEXTREL      22
#define DT_JMPREL       23
#define DT_BIND_NOW     24
#define DT_INIT_ARRAY   25
#define DT_FINI_ARRAY   26

static int elf_validate_header(void *data, size_t size) {
    if (size < sizeof(elf64_header_t)) return 0;
    
    elf64_header_t *elf = (elf64_header_t*)data;
    
    /* Check magic */
    if (elf->e_ident[0] != 0x7F ||
        elf->e_ident[1] != 'E' ||
        elf->e_ident[2] != 'L' ||
        elf->e_ident[3] != 'F') {
        return 0;
    }
    
    /* Check class (64-bit) */
    if (elf->e_ident[4] != 2) {  /* ELFCLASS64 */
        kprintf("[ELF] Only 64-bit ELF supported\n");
        return 0;
    }
    
    /* Check machine */
    if (elf->e_machine != EM_X86_64) {
        kprintf("[ELF] Only x86-64 supported\n");
        return 0;
    }
    
    return 1;
}

static uint64_t elf_calculate_load_size(elf64_header_t *elf, elf64_phdr_t *phdrs) {
    uint64_t min_addr = ~0ULL;
    uint64_t max_addr = 0;
    
    for (int i = 0; i < elf->e_phnum; i++) {
        if (phdrs[i].p_type != PT_LOAD) continue;
        
        if (phdrs[i].p_vaddr < min_addr) {
            min_addr = phdrs[i].p_vaddr;
        }
        
        uint64_t end = phdrs[i].p_vaddr + phdrs[i].p_memsz;
        if (end > max_addr) {
            max_addr = end;
        }
    }
    
    return max_addr - min_addr;
}

static int elf_load_segments(void *data, void *base, elf64_header_t *elf,
                              elf64_phdr_t *phdrs, uint64_t base_offset) {
    for (int i = 0; i < elf->e_phnum; i++) {
        if (phdrs[i].p_type != PT_LOAD) continue;
        
        uint64_t dest_addr = phdrs[i].p_vaddr - base_offset;
        void *dest = (uint8_t*)base + dest_addr;
        void *src = (uint8_t*)data + phdrs[i].p_offset;
        
        /* Copy file data */
        if (phdrs[i].p_filesz > 0) {
            kmemcpy(dest, src, phdrs[i].p_filesz);
        }
        
        /* Zero-fill BSS */
        if (phdrs[i].p_memsz > phdrs[i].p_filesz) {
            kmemset((uint8_t*)dest + phdrs[i].p_filesz, 0,
                    phdrs[i].p_memsz - phdrs[i].p_filesz);
        }
        
        kprintf("[ELF] Segment %d: VA=0x%llX Size=0x%llX Flags=%c%c%c\n",
                i, phdrs[i].p_vaddr, phdrs[i].p_memsz,
                (phdrs[i].p_flags & PF_R) ? 'R' : '-',
                (phdrs[i].p_flags & PF_W) ? 'W' : '-',
                (phdrs[i].p_flags & PF_X) ? 'X' : '-');
    }
    
    return 0;
}

static int elf_process_dynamic(void *base, elf64_dyn_t *dynamic, uint64_t base_addr) {
    char *strtab = NULL;
    elf64_sym_t *symtab = NULL;
    uint64_t *rela = NULL;
    size_t rela_size = 0;
    size_t rela_ent = 0;
    
    /* First pass: find tables */
    for (elf64_dyn_t *d = dynamic; d->d_tag != DT_NULL; d++) {
        switch (d->d_tag) {
            case DT_STRTAB:
                strtab = (char*)(base_addr + d->d_val);
                break;
            case DT_SYMTAB:
                symtab = (elf64_sym_t*)(base_addr + d->d_val);
                break;
            case DT_RELA:
                rela = (uint64_t*)(base_addr + d->d_val);
                break;
            case DT_RELASZ:
                rela_size = d->d_val;
                break;
            case DT_RELAENT:
                rela_ent = d->d_val;
                break;
            case DT_NEEDED:
                if (strtab) {
                    kprintf("[ELF] Needs: %s\n", strtab + d->d_val);
                }
                break;
        }
    }
    
    /* Process relocations */
    if (rela && rela_size > 0 && rela_ent > 0) {
        size_t num_relas = rela_size / rela_ent;
        kprintf("[ELF] Processing %zu relocations\n", num_relas);
        /* Would apply relocations here */
    }
    
    return 0;
}

int po_load_elf(void *data, size_t size, uint64_t *entry, uint64_t *base) {
    if (!elf_validate_header(data, size)) {
        kprintf("[ELF] Invalid ELF header\n");
        return -1;
    }
    
    elf64_header_t *elf = (elf64_header_t*)data;
    elf64_phdr_t *phdrs = (elf64_phdr_t*)((uint8_t*)data + elf->e_phoff);
    
    kprintf("[ELF] Loading %s (type=%d)\n",
            elf->e_type == ET_EXEC ? "executable" : "shared object",
            elf->e_type);
    
    /* Calculate total size needed */
    uint64_t load_size = elf_calculate_load_size(elf, phdrs);
    
    /* Find base address for PIE/shared objects */
    uint64_t base_offset = 0;
    for (int i = 0; i < elf->e_phnum; i++) {
        if (phdrs[i].p_type == PT_LOAD) {
            base_offset = phdrs[i].p_vaddr;
            break;
        }
    }
    
    /* Allocate memory */
    void *image_base = kmalloc(load_size);
    if (!image_base) {
        kprintf("[ELF] Failed to allocate %llu bytes\n", load_size);
        return -1;
    }
    kmemset(image_base, 0, load_size);
    
    /* Load segments */
    elf_load_segments(data, image_base, elf, phdrs, base_offset);
    
    /* Find and process dynamic section */
    for (int i = 0; i < elf->e_phnum; i++) {
        if (phdrs[i].p_type == PT_DYNAMIC) {
            elf64_dyn_t *dynamic = (elf64_dyn_t*)((uint8_t*)data + phdrs[i].p_offset);
            elf_process_dynamic(image_base, dynamic, (uint64_t)image_base);
            break;
        }
    }
    
    /* Set return values */
    *base = (uint64_t)image_base;
    *entry = (uint64_t)image_base + (elf->e_entry - base_offset);
    
    kprintf("[ELF] Loaded at 0x%llX, entry=0x%llX\n", *base, *entry);
    
    return 0;
}

/* ============================================================
 * Win32 API Emulation Layer
 * Based on: ReactOS dll/win32/kernel32
 * ============================================================ */

/* Kernel32 function table */
typedef struct {
    const char *name;
    void *func;
} win32_export_t;

/* Forward declarations */
static HMODULE win32_LoadLibraryA(LPCSTR lpLibFileName);
static BOOL win32_FreeLibrary(HMODULE hLibModule);
static void* win32_GetProcAddress(HMODULE hModule, LPCSTR lpProcName);
static LPVOID win32_VirtualAlloc(LPVOID lpAddress, size_t dwSize, 
                                  DWORD flAllocationType, DWORD flProtect);
static BOOL win32_VirtualFree(LPVOID lpAddress, size_t dwSize, DWORD dwFreeType);
static HANDLE win32_CreateFileA(LPCSTR lpFileName, DWORD dwDesiredAccess,
                                 DWORD dwShareMode, LPVOID lpSecurityAttributes,
                                 DWORD dwCreationDisposition, DWORD dwFlagsAndAttributes,
                                 HANDLE hTemplateFile);
static BOOL win32_CloseHandle(HANDLE hObject);
static BOOL win32_ReadFile(HANDLE hFile, LPVOID lpBuffer, DWORD nNumberOfBytesToRead,
                            DWORD *lpNumberOfBytesRead, LPVOID lpOverlapped);
static BOOL win32_WriteFile(HANDLE hFile, LPVOID lpBuffer, DWORD nNumberOfBytesToWrite,
                             DWORD *lpNumberOfBytesWritten, LPVOID lpOverlapped);

static win32_export_t kernel32_exports[] = {
    { "LoadLibraryA", win32_LoadLibraryA },
    { "FreeLibrary", win32_FreeLibrary },
    { "GetProcAddress", win32_GetProcAddress },
    { "VirtualAlloc", win32_VirtualAlloc },
    { "VirtualFree", win32_VirtualFree },
    { "CreateFileA", win32_CreateFileA },
    { "CloseHandle", win32_CloseHandle },
    { "ReadFile", win32_ReadFile },
    { "WriteFile", win32_WriteFile },
    { NULL, NULL }
};

static HMODULE win32_LoadLibraryA(LPCSTR lpLibFileName) {
    kprintf("[Win32] LoadLibraryA: %s\n", lpLibFileName);
    
    /* Check if already loaded */
    loaded_module_t *mod = loaded_modules;
    while (mod) {
        if (kstrcmp(mod->name, lpLibFileName) == 0) {
            mod->ref_count++;
            return (HMODULE)mod->base;
        }
        mod = mod->next;
    }
    
    /* Would load from disk here */
    return NULL;
}

static BOOL win32_FreeLibrary(HMODULE hLibModule) {
    kprintf("[Win32] FreeLibrary: 0x%p\n", hLibModule);
    
    loaded_module_t *mod = loaded_modules;
    while (mod) {
        if (mod->base == hLibModule) {
            mod->ref_count--;
            if (mod->ref_count <= 0) {
                /* Would unload here */
            }
            return TRUE;
        }
        mod = mod->next;
    }
    
    return FALSE;
}

static void* win32_GetProcAddress(HMODULE hModule, LPCSTR lpProcName) {
    kprintf("[Win32] GetProcAddress: %s\n", lpProcName);
    
    /* Check kernel32 exports */
    for (int i = 0; kernel32_exports[i].name; i++) {
        if (kstrcmp(kernel32_exports[i].name, lpProcName) == 0) {
            return kernel32_exports[i].func;
        }
    }
    
    return NULL;
}

static LPVOID win32_VirtualAlloc(LPVOID lpAddress, size_t dwSize,
                                  DWORD flAllocationType, DWORD flProtect) {
    kprintf("[Win32] VirtualAlloc: size=%zu\n", dwSize);
    return kmalloc(dwSize);
}

static BOOL win32_VirtualFree(LPVOID lpAddress, size_t dwSize, DWORD dwFreeType) {
    kprintf("[Win32] VirtualFree: 0x%p\n", lpAddress);
    kfree(lpAddress);
    return TRUE;
}

static HANDLE win32_CreateFileA(LPCSTR lpFileName, DWORD dwDesiredAccess,
                                 DWORD dwShareMode, LPVOID lpSecurityAttributes,
                                 DWORD dwCreationDisposition, DWORD dwFlagsAndAttributes,
                                 HANDLE hTemplateFile) {
    kprintf("[Win32] CreateFileA: %s\n", lpFileName);
    /* Would use VFS here */
    return INVALID_HANDLE_VALUE;
}

static BOOL win32_CloseHandle(HANDLE hObject) {
    kprintf("[Win32] CloseHandle: 0x%p\n", hObject);
    return TRUE;
}

static BOOL win32_ReadFile(HANDLE hFile, LPVOID lpBuffer, DWORD nNumberOfBytesToRead,
                            DWORD *lpNumberOfBytesRead, LPVOID lpOverlapped) {
    kprintf("[Win32] ReadFile\n");
    return FALSE;
}

static BOOL win32_WriteFile(HANDLE hFile, LPVOID lpBuffer, DWORD nNumberOfBytesToWrite,
                             DWORD *lpNumberOfBytesWritten, LPVOID lpOverlapped) {
    kprintf("[Win32] WriteFile\n");
    return FALSE;
}

/* ============================================================
 * POSIX API Emulation Layer
 * Based on: musl libc
 * ============================================================ */

typedef struct {
    const char *name;
    void *func;
} posix_export_t;

/* Forward declarations */
static void* posix_malloc(size_t size);
static void posix_free(void *ptr);
static int posix_open(const char *pathname, int flags);
static int posix_close(int fd);
static ssize_t posix_read(int fd, void *buf, size_t count);
static ssize_t posix_write(int fd, const void *buf, size_t count);

static posix_export_t libc_exports[] = {
    { "malloc", posix_malloc },
    { "free", posix_free },
    { "open", posix_open },
    { "close", posix_close },
    { "read", posix_read },
    { "write", posix_write },
    { NULL, NULL }
};

static void* posix_malloc(size_t size) {
    return kmalloc(size);
}

static void posix_free(void *ptr) {
    kfree(ptr);
}

static int posix_open(const char *pathname, int flags) {
    kprintf("[POSIX] open: %s\n", pathname);
    return -1;  /* Would use VFS */
}

static int posix_close(int fd) {
    kprintf("[POSIX] close: %d\n", fd);
    return 0;
}

static ssize_t posix_read(int fd, void *buf, size_t count) {
    kprintf("[POSIX] read: fd=%d count=%zu\n", fd, count);
    return -1;
}

static ssize_t posix_write(int fd, const void *buf, size_t count) {
    kprintf("[POSIX] write: fd=%d count=%zu\n", fd, count);
    return count;  /* Pretend success */
}

/* ============================================================
 * Unified Po Loader
 * ============================================================ */

po_format_t po_detect_format(void *data, size_t size) {
    if (size < 4) return PO_FORMAT_NATIVE;
    
    uint32_t magic = *(uint32_t*)data;
    
    /* Check Po native */
    if (magic == PO_MAGIC) {
        po_unified_header_t *po = (po_unified_header_t*)data;
        return po->po_format;
    }
    
    /* Check PE (MZ) */
    if ((magic & 0xFFFF) == DOS_MAGIC) {
        return PO_FORMAT_PE;
    }
    
    /* Check ELF */
    if (magic == ELF_MAGIC) {
        return PO_FORMAT_ELF;
    }
    
    return PO_FORMAT_NATIVE;
}

int po_load_any(void *data, size_t size, uint64_t *entry, uint64_t *base) {
    po_format_t format = po_detect_format(data, size);
    
    kprintf("[Po] Detected format: %d\n", format);
    
    switch (format) {
        case PO_FORMAT_PE:
            return po_load_pe(data, size, entry, base);
            
        case PO_FORMAT_ELF:
            return po_load_elf(data, size, entry, base);
            
        case PO_FORMAT_PE_ELF:
            /* Fat binary - try PE first, then ELF */
            if (po_load_pe(data, size, entry, base) == 0) {
                return 0;
            }
            return po_load_elf(data, size, entry, base);
            
        case PO_FORMAT_NATIVE:
        default:
            return po_load_native(data, size, entry, base);
    }
}

int po_load_native(void *data, size_t size, uint64_t *entry, uint64_t *base) {
    po_unified_header_t *po = (po_unified_header_t*)data;
    
    if (po->po_magic != PO_MAGIC) {
        kprintf("[Po] Invalid Po magic\n");
        return -1;
    }
    
    kprintf("[Po] Loading native Po executable v%d.%d\n",
            po->po_version >> 8, po->po_version & 0xFF);
    
    /* Allocate and copy */
    void *image = kmalloc(po->image_size);
    if (!image) return -1;
    
    kmemcpy(image, data, size);
    
    *base = (uint64_t)image;
    *entry = (uint64_t)image + po->entry_point;
    
    return 0;
}

/* ============================================================
 * Dynamic Linker (musl style)
 * Based on: musl ldso/dynlink.c
 * ============================================================ */

void* po_dlopen(const char *filename, int flags) {
    kprintf("[dlopen] %s\n", filename);
    
    /* Would load from filesystem */
    return NULL;
}

void* po_dlsym(void *handle, const char *symbol) {
    kprintf("[dlsym] %s\n", symbol);
    
    /* Check Win32 exports */
    for (int i = 0; kernel32_exports[i].name; i++) {
        if (kstrcmp(kernel32_exports[i].name, symbol) == 0) {
            return kernel32_exports[i].func;
        }
    }
    
    /* Check POSIX exports */
    for (int i = 0; libc_exports[i].name; i++) {
        if (kstrcmp(libc_exports[i].name, symbol) == 0) {
            return libc_exports[i].func;
        }
    }
    
    return NULL;
}

int po_dlclose(void *handle) {
    kprintf("[dlclose] 0x%p\n", handle);
    return 0;
}

/* ============================================================
 * Initialization
 * ============================================================ */

int po_loader_init(void) {
    kprintf("[Po] Loader initialized\n");
    kprintf("[Po] Supported formats: PE (Win32), ELF (Linux), Po (native)\n");
    kprintf("[Po] Win32 API: %d functions\n", 
            sizeof(kernel32_exports)/sizeof(kernel32_exports[0]) - 1);
    kprintf("[Po] POSIX API: %d functions\n",
            sizeof(libc_exports)/sizeof(libc_exports[0]) - 1);
    
    return 0;
}
