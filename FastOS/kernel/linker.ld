/* ============================================================
 * FastOS Kernel — Linker Script
 * ============================================================
 * Kernel loaded at 1MB (0x100000) by stage2 bootloader.
 * Sections: .text, .rodata, .data, .bss
 * Entry point: kernel_main
 * ============================================================ */

ENTRY(kernel_main)

SECTIONS
{
    /* Kernel starts at 1MB */
    . = 0x100000;

    /* Code section */
    .text ALIGN(4K) :
    {
        *(.text .text.*)
    }

    /* Read-only data */
    .rodata ALIGN(4K) :
    {
        *(.rodata .rodata.*)
    }

    /* Initialized data */
    .data ALIGN(4K) :
    {
        *(.data .data.*)
    }

    /* Uninitialized data (zeroed) */
    .bss ALIGN(4K) :
    {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    /* Kernel end marker */
    __kernel_end = .;

    /* Discard unwanted sections */
    /DISCARD/ :
    {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
    }
}
