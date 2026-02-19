/* ============================================================
 * ADead-OS — Linker Script
 * ============================================================
 * Combines ADead-BIB (base) + Rust (kernel) + C (compat)
 * into a single flat binary for bare-metal execution.
 *
 * Memory layout:
 *   0x7C00 - 0x7DFF : Stage 1 boot sector (512 bytes)
 *   0x8000 - 0x9FFF : Stage 2 mode switch (8KB)
 *   0x100000+       : Kernel (Rust + ADead-BIB runtime)
 *
 * NO ASM in the build. ADead-BIB is the assembler.
 * ============================================================ */

ENTRY(kernel_main)

SECTIONS
{
    /* Kernel loads at 1MB (after conventional memory) */
    . = 0x100000;

    /* Code section — ADead-BIB boot code first, then Rust */
    .text : ALIGN(4096)
    {
        *(.text.boot)       /* ADead-BIB boot/hardware code */
        *(.text)            /* All code */
        *(.text.*)          /* Rust code sections */
    }

    /* Read-only data */
    .rodata : ALIGN(4096)
    {
        *(.rodata)
        *(.rodata.*)
    }

    /* Initialized data */
    .data : ALIGN(4096)
    {
        *(.data)
        *(.data.*)
    }

    /* BSS (zero-initialized) */
    .bss : ALIGN(4096)
    {
        __bss_start = .;
        *(COMMON)
        *(.bss)
        *(.bss.*)
        __bss_end = .;
    }

    /* Kernel end marker */
    __kernel_end = .;

    /* Discard unnecessary sections */
    /DISCARD/ :
    {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
    }
}
