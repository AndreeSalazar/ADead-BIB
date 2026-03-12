/* FastOS v2.1 — Memory Initialization (Reference Module)
 * ADead-BIB Native OS — AMD Ryzen 5 5600X + DDR4-3200
 *
 * E820 map at physical 0x20000 (stage2.asm deposits it there)
 * Format: uint16 count at offset 0, then 24-byte entries
 * Entry: base(8) + length(8) + type(4) + acpi(4)
 *
 * Simple bump allocator starting at 0x200000 (2MB)
 * Kernel lives at 0x100000 (1MB), heap starts above it
 *
 * DDR4-3200 specs for Ryzen 5 5600X:
 *   - Dual channel, 2x8GB = 16GB typical
 *   - CL16-20-20-38 (XMP)
 *   - Bandwidth: 51.2 GB/s peak
 *   - Infinity Fabric 1:1 at 1600MHz FCLK
 *
 * Integration: kernel.c reads E820 inline via __store16/__inl
 * This file serves as reference for future linker-based builds.
 */

/* E820 memory types */
#define E820_USABLE       1
#define E820_RESERVED     2
#define E820_ACPI_RECL    3
#define E820_ACPI_NVS     4
#define E820_BAD          5

/* Physical addresses */
#define E820_MAP_ADDR     0x20000
#define HEAP_BASE         0x200000
#define HEAP_SIZE         0x800000

/* Bump allocator state (global) */
int heap_ptr = 0x200000;

/* Read 32-bit value from physical address */
int mem_read32(int addr) {
    return __inl(addr);
}

/* Inline E820 reader for kernel_main():
 *   int e820_count;
 *   int e820_base_lo, e820_base_hi;
 *   int e820_len_lo, e820_len_hi;
 *   int e820_type;
 *   int total_mb;
 *
 *   // Read count from 0x20000
 *   e820_count = *(uint16_t*)0x20000;  // via __load16 or manual
 *
 *   // Each entry at 0x20002 + i*24:
 *   //   +0: base_lo (4 bytes)
 *   //   +4: base_hi (4 bytes)
 *   //   +8: len_lo  (4 bytes)
 *   //  +12: len_hi  (4 bytes)
 *   //  +16: type    (4 bytes)
 *   //  +20: acpi    (4 bytes)
 */

/* Simple bump malloc (inline version for kernel_main):
 *   int alloc_ptr = 0x200000;
 *   // malloc(size):
 *   int result = alloc_ptr;
 *   alloc_ptr = alloc_ptr + size;
 *   // align to 8:
 *   alloc_ptr = (alloc_ptr + 7) & ~7;
 */
