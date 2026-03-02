/*
 * FastOS v2.0 — Po Loader with BG Integration
 * Hybrid kernel loader inspired by Windows/Linux
 * 
 * Loads Po executables with Binary Guardian verification
 */

#include "../include/kernel.h"
#include "../include/types.h"
#include "../include/po.h"

/* ============================================================
 * Po Validation
 * ============================================================ */

int po_validate(const po_header_t *header) {
    if (!header) return -1;
    
    /* Check magic */
    if (header->magic != PO_MAGIC) {
        kprintf("[PO] Invalid magic: 0x%08X (expected 0x%08X)\n", 
                header->magic, PO_MAGIC);
        return -1;
    }
    
    /* Check version */
    if (header->version > PO_VERSION) {
        kprintf("[PO] Unsupported version: 0x%04X\n", header->version);
        return -1;
    }
    
    /* Check machine type */
    if (header->machine != PO_MACHINE_X64) {
        kprintf("[PO] Unsupported machine: %d (expected x64)\n", header->machine);
        return -1;
    }
    
    /* Check word size */
    if (header->word_size != 64) {
        kprintf("[PO] Unsupported word size: %d\n", header->word_size);
        return -1;
    }
    
    return 0;
}

/* ============================================================
 * Section Access
 * ============================================================ */

const po_section_t* po_get_section(const po_header_t *header, 
                                    po_section_type_t type) {
    if (!header || header->section_count == 0) return NULL;
    
    const po_section_t *sections = (const po_section_t*)
        ((const uint8_t*)header + header->section_offset);
    
    for (uint16_t i = 0; i < header->section_count; i++) {
        if (sections[i].type == type) {
            return &sections[i];
        }
    }
    
    return NULL;
}

const char* po_get_string(const po_header_t *header, uint32_t offset) {
    if (!header || header->string_table == 0) return NULL;
    return (const char*)((const uint8_t*)header + header->string_table + offset);
}

/* ============================================================
 * BG Integration — Binary Guardian Verification
 * ============================================================ */

/* Instruction classification (simplified BG logic) */
typedef enum {
    INSTR_SAFE,
    INSTR_RESTRICTED,
    INSTR_PRIVILEGED,
} instr_class_t;

/* Classify x86-64 instruction by first byte(s) */
static instr_class_t classify_instruction(const uint8_t *code, size_t *len) {
    uint8_t op = code[0];
    *len = 1;  /* Default length */
    
    /* Privileged instructions (Ring 0 only) */
    switch (op) {
        case 0xFA: return INSTR_PRIVILEGED;  /* CLI */
        case 0xFB: return INSTR_PRIVILEGED;  /* STI */
        case 0xF4: return INSTR_PRIVILEGED;  /* HLT */
        case 0x0F:
            if (code[1] == 0x01) {
                uint8_t modrm = code[2];
                /* LGDT, LIDT, LLDT, LTR, etc. */
                if ((modrm & 0x38) <= 0x18) {
                    *len = 3;
                    return INSTR_PRIVILEGED;
                }
            }
            if (code[1] == 0x20 || code[1] == 0x22) {
                *len = 3;
                return INSTR_PRIVILEGED;  /* MOV CR */
            }
            if (code[1] == 0x30 || code[1] == 0x32) {
                *len = 2;
                return INSTR_PRIVILEGED;  /* WRMSR, RDMSR */
            }
            break;
    }
    
    /* Restricted instructions (syscalls, interrupts) */
    switch (op) {
        case 0xCD: *len = 2; return INSTR_RESTRICTED;  /* INT n */
        case 0xCE: return INSTR_RESTRICTED;            /* INTO */
        case 0x0F:
            if (code[1] == 0x05) {
                *len = 2;
                return INSTR_RESTRICTED;  /* SYSCALL */
            }
            if (code[1] == 0x34) {
                *len = 2;
                return INSTR_RESTRICTED;  /* SYSENTER */
            }
            break;
    }
    
    /* IO instructions */
    switch (op) {
        case 0xE4: case 0xE5:  /* IN imm8 */
        case 0xE6: case 0xE7:  /* OUT imm8 */
        case 0xEC: case 0xED:  /* IN DX */
        case 0xEE: case 0xEF:  /* OUT DX */
            return INSTR_RESTRICTED;
    }
    
    /* Everything else is safe */
    return INSTR_SAFE;
}

/* Build BG Architecture Map for code section */
static int bg_analyze_code(const uint8_t *code, size_t size, po_bg_map_t *map) {
    if (!code || !map) return -1;
    
    /* Initialize map */
    map->safe_count = 0;
    map->restricted_count = 0;
    map->privileged_count = 0;
    map->syscall_count = 0;
    map->io_port_count = 0;
    map->indirect_calls = 0;
    map->indirect_jumps = 0;
    map->far_jumps = 0;
    
    /* Scan code (simplified — real impl would use proper decoder) */
    size_t offset = 0;
    while (offset < size) {
        size_t len = 1;
        instr_class_t cls = classify_instruction(&code[offset], &len);
        
        switch (cls) {
            case INSTR_SAFE:
                map->safe_count++;
                break;
            case INSTR_RESTRICTED:
                map->restricted_count++;
                /* Count syscalls */
                if (code[offset] == 0x0F && code[offset+1] == 0x05) {
                    map->syscall_count++;
                }
                /* Count IO */
                if ((code[offset] >= 0xE4 && code[offset] <= 0xE7) ||
                    (code[offset] >= 0xEC && code[offset] <= 0xEF)) {
                    map->io_port_count++;
                }
                break;
            case INSTR_PRIVILEGED:
                map->privileged_count++;
                break;
        }
        
        /* Check for indirect control flow */
        if (code[offset] == 0xFF) {
            uint8_t modrm = code[offset + 1];
            uint8_t reg = (modrm >> 3) & 7;
            if (reg == 2) map->indirect_calls++;  /* CALL r/m */
            if (reg == 4) map->indirect_jumps++;  /* JMP r/m */
        }
        
        offset += len;
    }
    
    /* Determine required security level */
    if (map->privileged_count > 0) {
        map->max_security_level = PO_SECURITY_KERNEL;
    } else if (map->io_port_count > 0) {
        map->max_security_level = PO_SECURITY_DRIVER;
    } else if (map->syscall_count > 0) {
        map->max_security_level = PO_SECURITY_USER;
    } else {
        map->max_security_level = PO_SECURITY_SANDBOX;
    }
    
    map->verified = 1;
    return 0;
}

/* Verify Po file against required security level */
int po_verify_bg(const po_header_t *header, po_security_t required_level) {
    if (!header) return -1;
    
    /* Check if already has BG map */
    const po_section_t *bg_section = po_get_section(header, PO_SECTION_BG_MAP);
    if (bg_section && bg_section->size >= sizeof(po_bg_map_t)) {
        const po_bg_map_t *map = (const po_bg_map_t*)
            ((const uint8_t*)header + bg_section->offset);
        
        if (map->verified) {
            /* Use pre-computed map */
            if (map->max_security_level <= required_level) {
                kprintf("[BG] Pre-verified: APPROVED (level %d <= %d)\n",
                        map->max_security_level, required_level);
                return 0;
            } else {
                kprintf("[BG] Pre-verified: DENIED (level %d > %d)\n",
                        map->max_security_level, required_level);
                return -1;
            }
        }
    }
    
    /* Need to analyze code section */
    const po_section_t *code_section = po_get_section(header, PO_SECTION_CODE);
    if (!code_section) {
        kprintf("[BG] No code section found\n");
        return -1;
    }
    
    const uint8_t *code = (const uint8_t*)header + code_section->offset;
    po_bg_map_t map;
    
    if (bg_analyze_code(code, code_section->size, &map) != 0) {
        kprintf("[BG] Analysis failed\n");
        return -1;
    }
    
    kprintf("[BG] Analysis: safe=%d restricted=%d privileged=%d\n",
            map.safe_count, map.restricted_count, map.privileged_count);
    kprintf("[BG] Syscalls=%d IO=%d IndirectCalls=%d IndirectJumps=%d\n",
            map.syscall_count, map.io_port_count, 
            map.indirect_calls, map.indirect_jumps);
    
    if (map.max_security_level <= required_level) {
        kprintf("[BG] Verdict: APPROVED (level %d <= %d)\n",
                map.max_security_level, required_level);
        return 0;
    } else {
        kprintf("[BG] Verdict: DENIED (level %d > %d)\n",
                map.max_security_level, required_level);
        return -1;
    }
}

/* ============================================================
 * Po Loader — Load executable into memory
 * ============================================================ */

int po_load(const uint8_t *file_data, size_t file_size,
            void **load_addr, size_t *load_size) {
    if (!file_data || file_size < sizeof(po_header_t)) {
        return -1;
    }
    
    const po_header_t *header = (const po_header_t*)file_data;
    
    /* Validate header */
    if (po_validate(header) != 0) {
        return -1;
    }
    
    kprintf("[PO] Loading %s (type=%d, machine=%d)\n",
            header->type == PO_TYPE_EXEC ? "executable" :
            header->type == PO_TYPE_DYN ? "library" :
            header->type == PO_TYPE_DRIVER ? "driver" : "unknown",
            header->type, header->machine);
    
    /* Verify with BG based on file type */
    po_security_t required = PO_SECURITY_USER;
    if (header->type == PO_TYPE_DRIVER) {
        required = PO_SECURITY_DRIVER;
    } else if (header->type == PO_TYPE_KERNEL) {
        required = PO_SECURITY_KERNEL;
    }
    
    /* Check declared security level */
    if (header->security > required) {
        kprintf("[PO] Binary requires higher privilege (declared=%d, allowed=%d)\n",
                header->security, required);
        return -1;
    }
    
    /* BG verification */
    if (!(header->flags & PO_FLAG_BG_VERIFIED)) {
        if (po_verify_bg(header, required) != 0) {
            kprintf("[PO] BG verification failed — execution denied\n");
            return -1;
        }
    }
    
    /* Calculate total memory needed */
    *load_size = header->image_size;
    
    /* Allocate memory (simplified — real impl uses VMM) */
    /* For now, just return the entry point info */
    *load_addr = (void*)header->entry;
    
    kprintf("[PO] Entry point: 0x%016llX\n", header->entry);
    kprintf("[PO] Image size: %llu bytes\n", header->image_size);
    kprintf("[PO] Sections: %d\n", header->section_count);
    
    /* Load sections */
    const po_section_t *sections = (const po_section_t*)
        (file_data + header->section_offset);
    
    for (uint16_t i = 0; i < header->section_count; i++) {
        const po_section_t *sec = &sections[i];
        const char *name = po_get_string(header, sec->name);
        
        kprintf("  [%d] %s: vaddr=0x%llX size=%llu flags=0x%llX\n",
                i, name ? name : "(null)",
                sec->vaddr, sec->size, sec->flags);
    }
    
    return 0;
}

/* ============================================================
 * Execute Po Binary
 * ============================================================ */

typedef void (*entry_func_t)(void);

int po_execute(const uint8_t *file_data, size_t file_size) {
    void *load_addr;
    size_t load_size;
    
    if (po_load(file_data, file_size, &load_addr, &load_size) != 0) {
        return -1;
    }
    
    kprintf("[PO] Executing at 0x%p...\n", load_addr);
    
    /* Jump to entry point */
    entry_func_t entry = (entry_func_t)load_addr;
    entry();
    
    return 0;
}
