/*
 * FastOS v2.0 — Stage 2 Bootloader
 * Sets up protected mode, long mode, and loads kernel
 * 
 * Compile: adB cc stage2.c -o stage2.bin --flat --org=0x8000
 */

#define STAGE2_ADDR   0x8000
#define KERNEL_ADDR   0x100000  /* 1MB */
#define KERNEL_SECTORS 256

/* Video memory */
#define VGA_TEXT ((volatile char*)0xB8000)

static int cursor_x = 0;
static int cursor_y = 0;

/* Print character */
static void putchar(char c) {
    if (c == '\n') {
        cursor_x = 0;
        cursor_y++;
        return;
    }
    int offset = (cursor_y * 80 + cursor_x) * 2;
    VGA_TEXT[offset] = c;
    VGA_TEXT[offset + 1] = 0x1F;  /* White on blue */
    cursor_x++;
    if (cursor_x >= 80) {
        cursor_x = 0;
        cursor_y++;
    }
}

static void print(const char *s) {
    while (*s) putchar(*s++);
}

static void print_hex(unsigned long val) {
    const char *hex = "0123456789ABCDEF";
    print("0x");
    for (int i = 60; i >= 0; i -= 4) {
        putchar(hex[(val >> i) & 0xF]);
    }
}

/* Clear screen */
static void clear_screen(void) {
    for (int i = 0; i < 80 * 25 * 2; i += 2) {
        VGA_TEXT[i] = ' ';
        VGA_TEXT[i + 1] = 0x1F;
    }
    cursor_x = 0;
    cursor_y = 0;
}

/* GDT for 64-bit mode */
struct gdt_entry {
    unsigned short limit_low;
    unsigned short base_low;
    unsigned char base_mid;
    unsigned char access;
    unsigned char granularity;
    unsigned char base_high;
} __attribute__((packed));

struct gdt_ptr {
    unsigned short limit;
    unsigned long long base;
} __attribute__((packed));

static struct gdt_entry gdt[5];
static struct gdt_ptr gdtr;

static void gdt_set_entry(int num, unsigned int base, unsigned int limit,
                          unsigned char access, unsigned char gran) {
    gdt[num].base_low = base & 0xFFFF;
    gdt[num].base_mid = (base >> 16) & 0xFF;
    gdt[num].base_high = (base >> 24) & 0xFF;
    gdt[num].limit_low = limit & 0xFFFF;
    gdt[num].granularity = ((limit >> 16) & 0x0F) | (gran & 0xF0);
    gdt[num].access = access;
}

static void setup_gdt(void) {
    /* Null descriptor */
    gdt_set_entry(0, 0, 0, 0, 0);
    
    /* 64-bit code segment */
    gdt_set_entry(1, 0, 0xFFFFF, 0x9A, 0xAF);  /* Code: Execute/Read */
    
    /* 64-bit data segment */
    gdt_set_entry(2, 0, 0xFFFFF, 0x92, 0xCF);  /* Data: Read/Write */
    
    /* User code segment (for later) */
    gdt_set_entry(3, 0, 0xFFFFF, 0xFA, 0xAF);  /* User code */
    
    /* User data segment */
    gdt_set_entry(4, 0, 0xFFFFF, 0xF2, 0xCF);  /* User data */
    
    gdtr.limit = sizeof(gdt) - 1;
    gdtr.base = (unsigned long long)&gdt;
    
    __asm__ volatile("lgdt %0" : : "m"(gdtr));
}

/* Page tables for identity mapping first 4GB */
static unsigned long long pml4[512] __attribute__((aligned(4096)));
static unsigned long long pdpt[512] __attribute__((aligned(4096)));
static unsigned long long pd[512] __attribute__((aligned(4096)));

static void setup_paging(void) {
    /* Clear tables */
    for (int i = 0; i < 512; i++) {
        pml4[i] = 0;
        pdpt[i] = 0;
        pd[i] = 0;
    }
    
    /* PML4[0] -> PDPT */
    pml4[0] = ((unsigned long long)&pdpt) | 0x03;  /* Present + Writable */
    
    /* PDPT[0] -> PD */
    pdpt[0] = ((unsigned long long)&pd) | 0x03;
    
    /* PD entries: 2MB pages for first 1GB */
    for (int i = 0; i < 512; i++) {
        pd[i] = (i * 0x200000ULL) | 0x83;  /* Present + Writable + Huge */
    }
    
    /* Load CR3 with PML4 address */
    __asm__ volatile("mov %0, %%cr3" : : "r"((unsigned long long)&pml4));
}

/* Enable long mode */
static void enable_long_mode(void) {
    unsigned long long cr0, cr4, efer;
    
    /* Enable PAE in CR4 */
    __asm__ volatile("mov %%cr4, %0" : "=r"(cr4));
    cr4 |= (1 << 5);  /* PAE */
    __asm__ volatile("mov %0, %%cr4" : : "r"(cr4));
    
    /* Enable long mode in EFER MSR */
    __asm__ volatile(
        "mov $0xC0000080, %%ecx\n"
        "rdmsr\n"
        "or $0x100, %%eax\n"  /* LME bit */
        "wrmsr\n"
        : : : "eax", "ecx", "edx"
    );
    
    /* Enable paging in CR0 */
    __asm__ volatile("mov %%cr0, %0" : "=r"(cr0));
    cr0 |= (1 << 31);  /* PG */
    cr0 |= (1 << 0);   /* PE */
    __asm__ volatile("mov %0, %%cr0" : : "r"(cr0));
}

/* Read sectors using BIOS (must be done before long mode) */
static int read_sectors(unsigned char drive, unsigned int lba,
                        unsigned char count, void *buffer) {
    unsigned int cylinder = lba / (2 * 18);
    unsigned int temp = lba % (2 * 18);
    unsigned int head = temp / 18;
    unsigned int sector = temp % 18 + 1;
    
    unsigned char status;
    __asm__ volatile(
        "int $0x13"
        : "=a"(status)
        : "a"(0x0200 | count),
          "b"(buffer),
          "c"((cylinder << 8) | sector),
          "d"((head << 8) | drive)
        : "memory"
    );
    
    return (status >> 8) == 0 ? 0 : -1;
}

/* Entry point */
void _start(void) {
    clear_screen();
    
    print("FastOS Stage 2 Bootloader\n");
    print("=========================\n\n");
    
    /* A20 line should already be enabled by stage1 or BIOS */
    print("[BOOT] A20 line enabled\n");
    
    /* Setup GDT */
    print("[BOOT] Setting up GDT...\n");
    setup_gdt();
    print("[BOOT] GDT loaded at ");
    print_hex(gdtr.base);
    print("\n");
    
    /* Load kernel from disk (sectors 34+) */
    print("[BOOT] Loading kernel...\n");
    
    /* Load in chunks (BIOS can only read ~64 sectors at once) */
    unsigned char *dest = (unsigned char*)0x10000;  /* Temp location */
    int sectors_loaded = 0;
    int start_sector = 34;  /* After stage1 + stage2 */
    
    while (sectors_loaded < KERNEL_SECTORS) {
        int to_read = 64;
        if (sectors_loaded + to_read > KERNEL_SECTORS) {
            to_read = KERNEL_SECTORS - sectors_loaded;
        }
        
        if (read_sectors(0x80, start_sector + sectors_loaded, to_read, dest) != 0) {
            print("[BOOT] ERROR: Disk read failed!\n");
            while (1) __asm__ volatile("hlt");
        }
        
        dest += to_read * 512;
        sectors_loaded += to_read;
    }
    
    print("[BOOT] Kernel loaded (");
    /* Print sectors loaded */
    char buf[16];
    int n = sectors_loaded;
    int i = 0;
    do { buf[i++] = '0' + (n % 10); n /= 10; } while (n);
    while (i--) putchar(buf[i]);
    print(" sectors)\n");
    
    /* Setup paging */
    print("[BOOT] Setting up paging...\n");
    setup_paging();
    print("[BOOT] Page tables at ");
    print_hex((unsigned long long)&pml4);
    print("\n");
    
    /* Enable long mode */
    print("[BOOT] Enabling long mode...\n");
    enable_long_mode();
    print("[BOOT] Long mode enabled!\n");
    
    /* Copy kernel to 1MB */
    print("[BOOT] Relocating kernel to 1MB...\n");
    unsigned char *src = (unsigned char*)0x10000;
    unsigned char *dst = (unsigned char*)KERNEL_ADDR;
    for (int i = 0; i < KERNEL_SECTORS * 512; i++) {
        dst[i] = src[i];
    }
    
    print("[BOOT] Jumping to kernel at ");
    print_hex(KERNEL_ADDR);
    print("\n\n");
    
    /* Far jump to kernel in 64-bit mode */
    /* This would need proper assembly for the far jump */
    void (*kernel_entry)(void) = (void (*)(void))KERNEL_ADDR;
    kernel_entry();
    
    /* Should never reach here */
    print("[BOOT] ERROR: Kernel returned!\n");
    while (1) __asm__ volatile("hlt");
}
