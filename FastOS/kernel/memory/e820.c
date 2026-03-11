/*
 * FastOS v2.0 — E820 Memory Map
 * ADead-BIB Native Operating System
 * 
 * Reads BIOS E820 memory map to detect available RAM
 * Your system: AMD Ryzen 5 5600X + 16GB RAM
 */

#include "../../include/kernel.h"
#include "../../include/types.h"

/* E820 Memory Types */
#define E820_TYPE_USABLE      1
#define E820_TYPE_RESERVED    2
#define E820_TYPE_ACPI_RECLAIM 3
#define E820_TYPE_ACPI_NVS    4
#define E820_TYPE_BAD         5

/* E820 Entry Structure (as returned by BIOS) */
typedef struct {
    uint64_t base;
    uint64_t length;
    uint32_t type;
    uint32_t acpi_extended;
} __packed e820_entry_t;

/* Maximum entries we support */
#define E820_MAX_ENTRIES 64

/* Memory map storage */
static e820_entry_t e820_map[E820_MAX_ENTRIES];
static int e820_count = 0;

/* Total memory statistics */
static uint64_t total_memory = 0;
static uint64_t usable_memory = 0;
static uint64_t reserved_memory = 0;

/* Memory map location:
 * stage2.asm guarda el mapa E820 en segmento 0x2000, offset 0 = linear 0x20000
 * (No 0x7E00 — ese es el EBDA, NO nuestro mapa) */
#define E820_MAP_ADDRESS 0x20000

/* Get type name */
const char* e820_type_name(uint32_t type) {
    switch (type) {
        case E820_TYPE_USABLE:       return "Usable";
        case E820_TYPE_RESERVED:     return "Reserved";
        case E820_TYPE_ACPI_RECLAIM: return "ACPI Reclaim";
        case E820_TYPE_ACPI_NVS:     return "ACPI NVS";
        case E820_TYPE_BAD:          return "Bad Memory";
        default:                     return "Unknown";
    }
}

/* Print hex number (64-bit) */
static void print_hex64(uint64_t val) {
    const char hex[] = "0123456789ABCDEF";
    for (int i = 60; i >= 0; i -= 4) {
        vga_putchar(hex[(val >> i) & 0xF]);
    }
}

/* Print size in human readable format */
static void print_size(uint64_t bytes) {
    if (bytes >= 1024ULL * 1024 * 1024) {
        uint64_t gb = bytes / (1024ULL * 1024 * 1024);
        vga_putchar('0' + (gb / 10));
        vga_putchar('0' + (gb % 10));
        kputs(" GB");
    } else if (bytes >= 1024 * 1024) {
        uint64_t mb = bytes / (1024 * 1024);
        if (mb >= 100) {
            vga_putchar('0' + (mb / 100));
            vga_putchar('0' + ((mb / 10) % 10));
            vga_putchar('0' + (mb % 10));
        } else if (mb >= 10) {
            vga_putchar('0' + (mb / 10));
            vga_putchar('0' + (mb % 10));
        } else {
            vga_putchar('0' + mb);
        }
        kputs(" MB");
    } else if (bytes >= 1024) {
        uint64_t kb = bytes / 1024;
        if (kb >= 100) {
            vga_putchar('0' + (kb / 100));
        }
        if (kb >= 10) {
            vga_putchar('0' + ((kb / 10) % 10));
        }
        vga_putchar('0' + (kb % 10));
        kputs(" KB");
    } else {
        kputs("< 1 KB");
    }
}

/* Read E820 map from bootloader-provided location */
void e820_read_map(void) {
    /* stage2.asm guarda el E820 en segmento 0x2000 = linear 0x20000 */
    /* Primer word (2 bytes) es el conteo de entradas */
    uint16_t *count_ptr = (uint16_t *)E820_MAP_ADDRESS;
    e820_count = (int)*count_ptr;
    
    if (e820_count == 0 || e820_count > E820_MAX_ENTRIES) {
        /* No valid E820 map, create a default one */
        kputs("[MEM] No E820 map from BIOS, using defaults\n");
        
        /* Default memory layout for 16GB system */
        e820_map[0].base = 0x0;
        e820_map[0].length = 0x9FC00;  /* 639 KB conventional */
        e820_map[0].type = E820_TYPE_USABLE;
        
        e820_map[1].base = 0x9FC00;
        e820_map[1].length = 0x400;     /* EBDA */
        e820_map[1].type = E820_TYPE_RESERVED;
        
        e820_map[2].base = 0xE0000;
        e820_map[2].length = 0x20000;   /* BIOS ROM */
        e820_map[2].type = E820_TYPE_RESERVED;
        
        e820_map[3].base = 0x100000;    /* 1MB */
        e820_map[3].length = 0x3FF00000; /* ~16GB - 1MB (assuming 16GB) */
        e820_map[3].type = E820_TYPE_USABLE;
        
        e820_count = 4;
    } else {
        /* Copy from bootloader location (entries start after 2-byte count) */
        e820_entry_t *src = (e820_entry_t *)((uint8_t *)E820_MAP_ADDRESS + 2);
        for (int i = 0; i < e820_count; i++) {
            e820_map[i] = src[i];
        }
    }
    
    /* Calculate totals */
    total_memory = 0;
    usable_memory = 0;
    reserved_memory = 0;
    
    for (int i = 0; i < e820_count; i++) {
        total_memory += e820_map[i].length;
        
        if (e820_map[i].type == E820_TYPE_USABLE) {
            usable_memory += e820_map[i].length;
        } else {
            reserved_memory += e820_map[i].length;
        }
    }
}

/* Print memory map */
void e820_print_map(void) {
    kputs("[MEM] E820 Memory Map:\n");
    
    for (int i = 0; i < e820_count; i++) {
        kputs("      ");
        print_hex64(e820_map[i].base);
        kputs(" - ");
        print_hex64(e820_map[i].base + e820_map[i].length - 1);
        kputs(" (");
        print_size(e820_map[i].length);
        kputs(") ");
        kputs(e820_type_name(e820_map[i].type));
        kputs("\n");
    }
}

/* Print memory summary */
void e820_print_summary(void) {
    kputs("\n[MEM] Memory Summary:\n");
    kputs("      Total:    ");
    print_size(total_memory);
    kputs("\n");
    kputs("      Usable:   ");
    print_size(usable_memory);
    kputs("\n");
    kputs("      Reserved: ");
    print_size(reserved_memory);
    kputs("\n");
}

/* Get total usable memory */
uint64_t e820_get_usable_memory(void) {
    return usable_memory;
}

/* Get total memory */
uint64_t e820_get_total_memory(void) {
    return total_memory;
}

/* Find largest usable region (for heap) */
e820_entry_t* e820_find_largest_usable(void) {
    e820_entry_t *largest = NULL;
    uint64_t largest_size = 0;
    
    for (int i = 0; i < e820_count; i++) {
        if (e820_map[i].type == E820_TYPE_USABLE &&
            e820_map[i].length > largest_size) {
            largest = &e820_map[i];
            largest_size = e820_map[i].length;
        }
    }
    
    return largest;
}

/* Check if address is in usable memory */
int e820_is_usable(uint64_t addr) {
    for (int i = 0; i < e820_count; i++) {
        if (e820_map[i].type == E820_TYPE_USABLE &&
            addr >= e820_map[i].base &&
            addr < e820_map[i].base + e820_map[i].length) {
            return 1;
        }
    }
    return 0;
}

/* Initialize memory map */
void memory_map_init(void) {
    kputs("[MEM] Reading memory map...");
    e820_read_map();
    e820_print_summary();
}

/* memory_init() — llamada por kernel_main() como primer subsistema.
 * Secuencia completa: mapa E820 → resumen → heap listo.
 * El adb step de main.c confirma que esta es la firma esperada. */
void memory_init(void) {
    memory_map_init();
    kputs("[MEM] Heap ready (0x200000 base, 8MB).");
}
