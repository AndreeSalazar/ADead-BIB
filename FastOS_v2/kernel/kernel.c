/* FastOS_v2 Kernel - Minimal for REAL HARDWARE testing
 * Just prints to VGA to prove kernel is alive
 */

void kernel_main(void) {
    int p;
    int i;

    /* === IMMEDIATE VGA OUTPUT === */
    /* Write ">>> KERNEL OK <<<" at row 5 in bright green */
    p = 0xB8000 + 5 * 160;

    /* >>> */
    __store16(p, 0, 0x0A3E);
    __store16(p, 2, 0x0A3E);
    __store16(p, 4, 0x0A3E);
    __store16(p, 6, 0x0A20);

    /* KERNEL */
    __store16(p, 8, 0x0A4B);
    __store16(p, 10, 0x0A45);
    __store16(p, 12, 0x0A52);
    __store16(p, 14, 0x0A4E);
    __store16(p, 16, 0x0A45);
    __store16(p, 18, 0x0A4C);
    __store16(p, 20, 0x0A20);

    /* OK */
    __store16(p, 22, 0x0A4F);
    __store16(p, 24, 0x0A4B);
    __store16(p, 26, 0x0A20);

    /* <<< */
    __store16(p, 28, 0x0A3C);
    __store16(p, 30, 0x0A3C);
    __store16(p, 32, 0x0A3C);

    /* === Serial output for debug === */
    __outb(0x3F9, 0x00);
    __outb(0x3FB, 0x80);
    __outb(0x3F8, 0x01);
    __outb(0x3F9, 0x00);
    __outb(0x3FB, 0x03);
    __outb(0x3FA, 0xC7);
    __outb(0x3FC, 0x0B);
    /* "KERNEL\r\n" */
    __outb(0x3F8, 'K');
    __outb(0x3F8, 'E');
    __outb(0x3F8, 'R');
    __outb(0x3F8, 'N');
    __outb(0x3F8, 'E');
    __outb(0x3F8, 'L');
    __outb(0x3F8, 13);
    __outb(0x3F8, 10);

    /* === Simple shell prompt === */
    p = 0xB8000 + 7 * 160;
    /* "C:\FastOS> " */
    __store16(p, 0, 0x0F43);
    __store16(p, 2, 0x0F3A);
    __store16(p, 4, 0x0F5C);
    __store16(p, 6, 0x0F46);
    __store16(p, 8, 0x0F61);
    __store16(p, 10, 0x0F73);
    __store16(p, 12, 0x0F74);
    __store16(p, 14, 0x0F4F);
    __store16(p, 16, 0x0F53);
    __store16(p, 18, 0x0A3E);
    __store16(p, 20, 0x0F20);

    /* Halt - kernel stays alive */
    __cli();
    while (1) {
        __hlt();
    }
}
