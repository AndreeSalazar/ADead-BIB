/* ============================================================
 * FastOS v2.0 — Linker Script
 * Kernel loaded at 0x100000 (1MB mark)
 * Target: x86_64, no standard libraries
 * ============================================================ */

OUTPUT_FORMAT("elf64-x86-64")
OUTPUT_ARCH(i386:x86-64)
ENTRY(kernel_entry)

SECTIONS
{
    . = 0x100000;

    __kernel_start = .;

    /* Code section — kernel_entry.asm first, then C code */
    .text ALIGN(4096) : {
        *(.text.entry)      /* kernel_entry.asm — must be first */
        *(.text .text.*)
    }

    /* Read-only data */
    .rodata ALIGN(4096) : {
        *(.rodata .rodata.*)
    }

    /* Initialized data */
    .data ALIGN(4096) : {
        *(.data .data.*)
    }

    /* Uninitialized data */
    .bss ALIGN(4096) : {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    __kernel_end = .;

    /* Discard unnecessary sections */
    /DISCARD/ : {
        *(.comment)
        *(.note*)
        *(.eh_frame*)
    }
}
