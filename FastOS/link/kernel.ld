/* ============================================================
 * FastOS — Linker Script
 * ============================================================
 * Combines ADead-BIB hardware code + Rust kernel into one binary.
 * Kernel loaded at 0x100000 (1MB) by stage2 bootloader.
 * ============================================================ */

ENTRY(kernel_main)

SECTIONS
{
    . = 0x100000;

    .text.boot : {
        *(.text.boot)
    }

    .text : {
        *(.text .text.*)
    }

    .rodata : ALIGN(4096) {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(4096) {
        *(.data .data.*)
    }

    .bss : ALIGN(4096) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    __kernel_end = .;

    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
    }
}
