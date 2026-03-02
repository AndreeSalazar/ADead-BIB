/*
 * FastOS v2.0 — Stage 1 Bootloader (MBR)
 * 512 bytes - Loads stage2 from disk
 * 
 * Compile: adB cc stage1.c -o stage1.bin --flat --org=0x7C00
 */

/* BIOS loads us at 0x7C00 */
#define STAGE1_ADDR  0x7C00
#define STAGE2_ADDR  0x8000
#define STAGE2_SECTORS 32

/* Video memory */
#define VGA_TEXT ((volatile char*)0xB8000)

/* Assembly helpers via raw bytes */
static void outb_asm(unsigned short port, unsigned char val) {
    __asm__ volatile("outb %0, %1" : : "a"(val), "Nd"(port));
}

static unsigned char inb_asm(unsigned short port) {
    unsigned char ret;
    __asm__ volatile("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

/* Print character at position */
static void putchar_at(char c, int x, int y, unsigned char color) {
    int offset = (y * 80 + x) * 2;
    VGA_TEXT[offset] = c;
    VGA_TEXT[offset + 1] = color;
}

/* Print string */
static void print(const char *s, int x, int y, unsigned char color) {
    while (*s) {
        putchar_at(*s++, x++, y, color);
    }
}

/* Clear screen */
static void clear_screen(unsigned char color) {
    for (int i = 0; i < 80 * 25 * 2; i += 2) {
        VGA_TEXT[i] = ' ';
        VGA_TEXT[i + 1] = color;
    }
}

/* Read sectors using BIOS INT 13h (via inline asm) */
static int read_sectors(unsigned char drive, unsigned int lba, 
                        unsigned char count, void *buffer) {
    /* Convert LBA to CHS for INT 13h */
    unsigned int cylinder = lba / (2 * 18);
    unsigned int temp = lba % (2 * 18);
    unsigned int head = temp / 18;
    unsigned int sector = temp % 18 + 1;
    
    /* INT 13h AH=02h: Read sectors */
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
    /* Clear screen - blue background */
    clear_screen(0x1F);
    
    /* Print boot message */
    print("FastOS Boot", 34, 10, 0x1E);
    print("Loading stage2...", 31, 12, 0x1F);
    
    /* Read stage2 from disk (sectors 2-33) */
    if (read_sectors(0x80, 1, STAGE2_SECTORS, (void*)STAGE2_ADDR) != 0) {
        print("Disk read error!", 32, 14, 0x4F);
        while (1) __asm__ volatile("hlt");
    }
    
    print("Jumping to stage2...", 30, 14, 0x1A);
    
    /* Jump to stage2 */
    void (*stage2)(void) = (void (*)(void))STAGE2_ADDR;
    stage2();
    
    /* Should never reach here */
    while (1) __asm__ volatile("hlt");
}

/* Boot signature - must be at offset 510 */
__attribute__((section(".bootsig")))
const unsigned short boot_signature = 0xAA55;
