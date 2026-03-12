/* ================================================================
 * FastOS v2.1 Kernel - 100% ADead-BIB C Compiler (Inline Edition)
 * ================================================================
 * All code inline in kernel_main for reliability.
 * Inspired by Windows NT boot display + Linux VT console.
 *
 * __store32: VGA writes (32-bit = char+attr + space padding)
 * __inb/__outb: PS/2 keyboard polling
 * __cpuid_eax/ebx/ecx/edx: CPU feature detection
 * __cli: disable interrupts (no IDT configured)
 *
 * VGA Layout: 0xB8000, 80x25, 2 bytes/cell, 160 bytes/row
 * Each __store32 writes 4 bytes: low16=char|attr, high16=0x1F20
 * Characters placed at 4-byte stride (every other cell gets pad)
 * ================================================================ */

void kernel_main(void) {
    int i;
    int sc;
    int key;
    int ascii;
    int cursor;
    int srow;
    int p;
    int clen;
    int c0;
    int c1;
    int c2;
    int c3;
    int c4;
    int orow;
    int vb;
    int vd;
    int vc;
    int maxl;
    int avx2;
    int tmp;
    int ch;
    int nibble;
    int brand_a;
    int brand_b;
    int brand_c;
    int brand_d;

    __cli();

    /* ============================================================
     * PHASE 1: Clear screen blue
     * ============================================================ */
    i = 0;
    while (i < 4000) {
        __store32(0xB8000, i, 0x1F201F20);
        i = i + 4;
    }

    /* ============================================================
     * PHASE 2: CPUID Detection
     * ============================================================ */
    maxl = __cpuid_eax(0);
    vb = __cpuid_ebx(0);
    vc = __cpuid_ecx(0);
    vd = __cpuid_edx(0);

    avx2 = 0;
    if (maxl > 6) {
        tmp = __cpuid_ebx(7);
        if (tmp & 32) { avx2 = 1; }
    }

    /* ============================================================
     * PHASE 3: Boot Banner (Row 0)
     * "FastOS v2.1" yellow on blue
     * ============================================================ */
    __store32(0xB8000, 0, 0x1F201E46);    /* F */
    __store32(0xB8000, 4, 0x1F201E61);    /* a */
    __store32(0xB8000, 8, 0x1F201E73);    /* s */
    __store32(0xB8000, 12, 0x1F201E74);   /* t */
    __store32(0xB8000, 16, 0x1F201E4F);   /* O */
    __store32(0xB8000, 20, 0x1F201E53);   /* S */
    __store32(0xB8000, 24, 0x1F201F20);   /* space */
    __store32(0xB8000, 28, 0x1F201F76);   /* v */
    __store32(0xB8000, 32, 0x1F201F32);   /* 2 */
    __store32(0xB8000, 36, 0x1F201F2E);   /* . */
    __store32(0xB8000, 40, 0x1F201F31);   /* 1 */

    /* Row 0 right side: 256-bit or 128-bit */
    if (avx2 > 0) {
        __store32(0xB8000, 120, 0x1F200E32);  /* 2 yellow */
        __store32(0xB8000, 124, 0x1F200E35);  /* 5 */
        __store32(0xB8000, 128, 0x1F200E36);  /* 6 */
        __store32(0xB8000, 132, 0x1F200E2D);  /* - */
        __store32(0xB8000, 136, 0x1F200A62);  /* b green */
        __store32(0xB8000, 140, 0x1F200A69);  /* i */
        __store32(0xB8000, 144, 0x1F200A74);  /* t */
    }
    if (avx2 == 0) {
        __store32(0xB8000, 120, 0x1F200C31);  /* 1 red */
        __store32(0xB8000, 124, 0x1F200C32);  /* 2 */
        __store32(0xB8000, 128, 0x1F200C38);  /* 8 */
        __store32(0xB8000, 132, 0x1F200C2D);  /* - */
        __store32(0xB8000, 136, 0x1F200C62);  /* b */
        __store32(0xB8000, 140, 0x1F200C69);  /* i */
        __store32(0xB8000, 144, 0x1F200C74);  /* t */
    }

    /* ============================================================
     * PHASE 4: CPU Vendor String (Row 1)
     * CPUID leaf 0: EBX:EDX:ECX = "AuthenticAMD" (LE)
     * ============================================================ */
    p = 160;
    /* EBX 4 chars */
    ch = vb & 0xFF;
    if (ch > 31) { __store32(0xB8000, p, 0x1F200B00 | ch); } p = p + 4;
    ch = (vb >> 8) & 0xFF;
    if (ch > 31) { __store32(0xB8000, p, 0x1F200B00 | ch); } p = p + 4;
    ch = (vb >> 16) & 0xFF;
    if (ch > 31) { __store32(0xB8000, p, 0x1F200B00 | ch); } p = p + 4;
    ch = (vb >> 24) & 0xFF;
    if (ch > 31) { __store32(0xB8000, p, 0x1F200B00 | ch); } p = p + 4;
    /* EDX 4 chars */
    ch = vd & 0xFF;
    if (ch > 31) { __store32(0xB8000, p, 0x1F200B00 | ch); } p = p + 4;
    ch = (vd >> 8) & 0xFF;
    if (ch > 31) { __store32(0xB8000, p, 0x1F200B00 | ch); } p = p + 4;
    ch = (vd >> 16) & 0xFF;
    if (ch > 31) { __store32(0xB8000, p, 0x1F200B00 | ch); } p = p + 4;
    ch = (vd >> 24) & 0xFF;
    if (ch > 31) { __store32(0xB8000, p, 0x1F200B00 | ch); } p = p + 4;
    /* ECX 4 chars */
    ch = vc & 0xFF;
    if (ch > 31) { __store32(0xB8000, p, 0x1F200B00 | ch); } p = p + 4;
    ch = (vc >> 8) & 0xFF;
    if (ch > 31) { __store32(0xB8000, p, 0x1F200B00 | ch); } p = p + 4;
    ch = (vc >> 16) & 0xFF;
    if (ch > 31) { __store32(0xB8000, p, 0x1F200B00 | ch); } p = p + 4;
    ch = (vc >> 24) & 0xFF;
    if (ch > 31) { __store32(0xB8000, p, 0x1F200B00 | ch); }

    /* ============================================================
     * PHASE 5: CPU Brand String (Row 2-3)
     * CPUID leaves 0x80000002, 0x80000003, 0x80000004
     * = 48 ASCII chars like "AMD EPYC-Milan Processor"
     * ============================================================ */
    tmp = __cpuid_eax(0x80000000);
    if (tmp > 0x80000004) {
        p = 320;
        /* Leaf 0x80000002 */
        brand_a = __cpuid_eax(0x80000002);
        brand_b = __cpuid_ebx(0x80000002);
        brand_c = __cpuid_ecx(0x80000002);
        brand_d = __cpuid_edx(0x80000002);
        ch = brand_a & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_a >> 8) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_a >> 16) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_a >> 24) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = brand_b & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_b >> 8) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_b >> 16) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_b >> 24) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = brand_c & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_c >> 8) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_c >> 16) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_c >> 24) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = brand_d & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_d >> 8) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_d >> 16) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_d >> 24) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;

        /* Leaf 0x80000003 */
        brand_a = __cpuid_eax(0x80000003);
        brand_b = __cpuid_ebx(0x80000003);
        brand_c = __cpuid_ecx(0x80000003);
        brand_d = __cpuid_edx(0x80000003);
        ch = brand_a & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_a >> 8) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_a >> 16) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_a >> 24) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = brand_b & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_b >> 8) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_b >> 16) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_b >> 24) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = brand_c & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_c >> 8) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_c >> 16) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_c >> 24) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = brand_d & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_d >> 8) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_d >> 16) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_d >> 24) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;

        /* Leaf 0x80000004 */
        brand_a = __cpuid_eax(0x80000004);
        brand_b = __cpuid_ebx(0x80000004);
        brand_c = __cpuid_ecx(0x80000004);
        brand_d = __cpuid_edx(0x80000004);
        ch = brand_a & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_a >> 8) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_a >> 16) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_a >> 24) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = brand_b & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_b >> 8) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_b >> 16) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_b >> 24) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = brand_c & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_c >> 8) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_c >> 16) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_c >> 24) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = brand_d & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_d >> 8) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_d >> 16) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); } p = p + 4;
        ch = (brand_d >> 24) & 0xFF; if (ch > 31) { __store32(0xB8000, p, 0x1F200F00 | ch); }
    }

    /* ============================================================
     * PHASE 6: Features (Row 4)
     * AVX2, SSE4.2, AES-NI detection
     * ============================================================ */
    p = 640;
    if (avx2 > 0) {
        __store32(0xB8000, p, 0x1F200A41); p = p + 4;  /* A green */
        __store32(0xB8000, p, 0x1F200A56); p = p + 4;  /* V */
        __store32(0xB8000, p, 0x1F200A58); p = p + 4;  /* X */
        __store32(0xB8000, p, 0x1F200A32); p = p + 8;  /* 2 + skip */
    }
    if (avx2 == 0) {
        __store32(0xB8000, p, 0x1F200C53); p = p + 4;  /* S red */
        __store32(0xB8000, p, 0x1F200C53); p = p + 4;  /* S */
        __store32(0xB8000, p, 0x1F200C45); p = p + 8;  /* E + skip */
    }
    /* SSE4.2: CPUID leaf 1, ECX bit 20 */
    tmp = __cpuid_ecx(1);
    if (tmp & 0x100000) {
        __store32(0xB8000, p, 0x1F200A53); p = p + 4;  /* S */
        __store32(0xB8000, p, 0x1F200A53); p = p + 4;  /* S */
        __store32(0xB8000, p, 0x1F200A45); p = p + 4;  /* E */
        __store32(0xB8000, p, 0x1F200A34); p = p + 4;  /* 4 */
        __store32(0xB8000, p, 0x1F200A2E); p = p + 4;  /* . */
        __store32(0xB8000, p, 0x1F200A32); p = p + 8;  /* 2 */
    }
    /* AES-NI: ECX bit 25 */
    if (tmp & 0x2000000) {
        __store32(0xB8000, p, 0x1F200A41); p = p + 4;  /* A */
        __store32(0xB8000, p, 0x1F200A45); p = p + 4;  /* E */
        __store32(0xB8000, p, 0x1F200A53); p = p + 8;  /* S */
    }

    /* ============================================================
     * PHASE 7: "Kernel OK" + "ADead-BIB" (Row 5)
     * ============================================================ */
    __store32(0xB8000, 800, 0x1F200A4B);   /* K green */
    __store32(0xB8000, 804, 0x1F200A65);   /* e */
    __store32(0xB8000, 808, 0x1F200A72);   /* r */
    __store32(0xB8000, 812, 0x1F200A6E);   /* n */
    __store32(0xB8000, 816, 0x1F200A65);   /* e */
    __store32(0xB8000, 820, 0x1F200A6C);   /* l */
    __store32(0xB8000, 824, 0x1F201F20);   /* space */
    __store32(0xB8000, 828, 0x1F200A4F);   /* O */
    __store32(0xB8000, 832, 0x1F200A4B);   /* K */
    __store32(0xB8000, 840, 0x1F200B41);   /* A cyan */
    __store32(0xB8000, 844, 0x1F200B44);   /* D */
    __store32(0xB8000, 848, 0x1F200B65);   /* e */
    __store32(0xB8000, 852, 0x1F200B61);   /* a */
    __store32(0xB8000, 856, 0x1F200B64);   /* d */
    __store32(0xB8000, 860, 0x1F200B2D);   /* - */
    __store32(0xB8000, 864, 0x1F200B42);   /* B */
    __store32(0xB8000, 868, 0x1F200B49);   /* I */
    __store32(0xB8000, 872, 0x1F200B42);   /* B */

    /* ============================================================
     * PHASE 8: Shell Init (Row 7)
     * ============================================================ */
    srow = 7;
    p = srow * 160;
    __store32(0xB8000, p, 0x1F200F3E);     /* > white */
    __store32(0xB8000, p + 4, 0x1F200F20); /* space */
    cursor = p + 8;
    clen = 0;
    c0 = 0; c1 = 0; c2 = 0; c3 = 0; c4 = 0;

    /* ============================================================
     * PHASE 9: Main Loop - PS/2 Keyboard Polling
     * Like Linux kernel keyboard.c: poll status port, read scancode
     * ============================================================ */
    while (1) {
        sc = __inb(0x64);
        if (sc & 1) {
            key = __inb(0x60);
            if (key > 0) {
                if (key < 128) {
                    /* ---- ENTER (0x1C) ---- */
                    if (key == 0x1C) {
                        orow = srow + 1;

                        /* Command: help (104,101,108,112) */
                        if (c0 == 104) {
                            if (c1 == 101) {
                                if (c2 == 108) {
                                    if (c3 == 112) {
                                        p = orow * 160;
                                        __store32(0xB8000, p, 0x1F200E43); p = p + 4;  /* C yellow */
                                        __store32(0xB8000, p, 0x1F200E6F); p = p + 4;  /* o */
                                        __store32(0xB8000, p, 0x1F200E6D); p = p + 4;  /* m */
                                        __store32(0xB8000, p, 0x1F200E6D); p = p + 4;  /* m */
                                        __store32(0xB8000, p, 0x1F200E61); p = p + 4;  /* a */
                                        __store32(0xB8000, p, 0x1F200E6E); p = p + 4;  /* n */
                                        __store32(0xB8000, p, 0x1F200E64); p = p + 4;  /* d */
                                        __store32(0xB8000, p, 0x1F200E73); p = p + 4;  /* s */
                                        __store32(0xB8000, p, 0x1F200E3A);              /* : */
                                        p = (orow + 1) * 160;
                                        __store32(0xB8000, p + 8, 0x1F200F68);  /* h */
                                        __store32(0xB8000, p + 12, 0x1F200F65); /* e */
                                        __store32(0xB8000, p + 16, 0x1F200F6C); /* l */
                                        __store32(0xB8000, p + 20, 0x1F200F70); /* p */
                                        __store32(0xB8000, p + 32, 0x1F200773); /* s */
                                        __store32(0xB8000, p + 36, 0x1F200768); /* h */
                                        __store32(0xB8000, p + 40, 0x1F20076F); /* o */
                                        __store32(0xB8000, p + 44, 0x1F200777); /* w */
                                        p = (orow + 2) * 160;
                                        __store32(0xB8000, p + 8, 0x1F200F63);  /* c */
                                        __store32(0xB8000, p + 12, 0x1F200F70); /* p */
                                        __store32(0xB8000, p + 16, 0x1F200F75); /* u */
                                        __store32(0xB8000, p + 32, 0x1F200743); /* C */
                                        __store32(0xB8000, p + 36, 0x1F200750); /* P */
                                        __store32(0xB8000, p + 40, 0x1F200755); /* U */
                                        __store32(0xB8000, p + 48, 0x1F200769); /* i */
                                        __store32(0xB8000, p + 52, 0x1F20076E); /* n */
                                        __store32(0xB8000, p + 56, 0x1F200766); /* f */
                                        __store32(0xB8000, p + 60, 0x1F20076F); /* o */
                                        p = (orow + 3) * 160;
                                        __store32(0xB8000, p + 8, 0x1F200F63);  /* c */
                                        __store32(0xB8000, p + 12, 0x1F200F6C); /* l */
                                        __store32(0xB8000, p + 16, 0x1F200F65); /* e */
                                        __store32(0xB8000, p + 20, 0x1F200F61); /* a */
                                        __store32(0xB8000, p + 24, 0x1F200F72); /* r */
                                        __store32(0xB8000, p + 32, 0x1F200763); /* c */
                                        __store32(0xB8000, p + 36, 0x1F20076C); /* l */
                                        __store32(0xB8000, p + 40, 0x1F200765); /* e */
                                        __store32(0xB8000, p + 44, 0x1F200761); /* a */
                                        __store32(0xB8000, p + 48, 0x1F200772); /* r */
                                        p = (orow + 4) * 160;
                                        __store32(0xB8000, p + 8, 0x1F200F76);  /* v */
                                        __store32(0xB8000, p + 12, 0x1F200F65); /* e */
                                        __store32(0xB8000, p + 16, 0x1F200F72); /* r */
                                        __store32(0xB8000, p + 32, 0x1F200776); /* v */
                                        __store32(0xB8000, p + 36, 0x1F200765); /* e */
                                        __store32(0xB8000, p + 40, 0x1F200772); /* r */
                                        __store32(0xB8000, p + 44, 0x1F200773); /* s */
                                        __store32(0xB8000, p + 48, 0x1F200769); /* i */
                                        __store32(0xB8000, p + 52, 0x1F20076F); /* o */
                                        __store32(0xB8000, p + 56, 0x1F20076E); /* n */
                                        srow = orow + 6;
                                    }
                                }
                            }
                        }

                        /* Command: cpu (99,112,117) */
                        if (c0 == 99) {
                            if (c1 == 112) {
                                if (c2 == 117) {
                                    if (c3 == 0) {
                                        /* Re-show vendor + brand on output rows */
                                        p = orow * 160;
                                        __store32(0xB8000, p, 0x1F200E43); p = p + 4;  /* C */
                                        __store32(0xB8000, p, 0x1F200E50); p = p + 4;  /* P */
                                        __store32(0xB8000, p, 0x1F200E55); p = p + 4;  /* U */
                                        __store32(0xB8000, p, 0x1F200E3A); p = p + 8;  /* : */
                                        /* Vendor */
                                        ch = vb & 0xFF; __store32(0xB8000, p, 0x1F200F00 | ch); p = p + 4;
                                        ch = (vb >> 8) & 0xFF; __store32(0xB8000, p, 0x1F200F00 | ch); p = p + 4;
                                        ch = (vb >> 16) & 0xFF; __store32(0xB8000, p, 0x1F200F00 | ch); p = p + 4;
                                        ch = (vb >> 24) & 0xFF; __store32(0xB8000, p, 0x1F200F00 | ch); p = p + 4;
                                        ch = vd & 0xFF; __store32(0xB8000, p, 0x1F200F00 | ch); p = p + 4;
                                        ch = (vd >> 8) & 0xFF; __store32(0xB8000, p, 0x1F200F00 | ch); p = p + 4;
                                        ch = (vd >> 16) & 0xFF; __store32(0xB8000, p, 0x1F200F00 | ch); p = p + 4;
                                        ch = (vd >> 24) & 0xFF; __store32(0xB8000, p, 0x1F200F00 | ch); p = p + 4;
                                        ch = vc & 0xFF; __store32(0xB8000, p, 0x1F200F00 | ch); p = p + 4;
                                        ch = (vc >> 8) & 0xFF; __store32(0xB8000, p, 0x1F200F00 | ch); p = p + 4;
                                        ch = (vc >> 16) & 0xFF; __store32(0xB8000, p, 0x1F200F00 | ch); p = p + 4;
                                        ch = (vc >> 24) & 0xFF; __store32(0xB8000, p, 0x1F200F00 | ch);
                                        /* Features line */
                                        p = (orow + 1) * 160;
                                        if (avx2 > 0) {
                                            __store32(0xB8000, p, 0x1F200A41); p = p + 4;
                                            __store32(0xB8000, p, 0x1F200A56); p = p + 4;
                                            __store32(0xB8000, p, 0x1F200A58); p = p + 4;
                                            __store32(0xB8000, p, 0x1F200A32); p = p + 8;
                                        }
                                        __store32(0xB8000, p, 0x1F200A53); p = p + 4;  /* S */
                                        __store32(0xB8000, p, 0x1F200A53); p = p + 4;  /* S */
                                        __store32(0xB8000, p, 0x1F200A45); p = p + 4;  /* E */
                                        __store32(0xB8000, p, 0x1F200A34); p = p + 4;  /* 4 */
                                        __store32(0xB8000, p, 0x1F200A2E); p = p + 4;  /* . */
                                        __store32(0xB8000, p, 0x1F200A32); p = p + 8;  /* 2 */
                                        __store32(0xB8000, p, 0x1F200A41); p = p + 4;  /* A */
                                        __store32(0xB8000, p, 0x1F200A45); p = p + 4;  /* E */
                                        __store32(0xB8000, p, 0x1F200A53);              /* S */
                                        srow = orow + 3;
                                    }
                                }
                            }
                        }

                        /* Command: clear (99,108,101,97,114) */
                        if (c0 == 99) {
                            if (c1 == 108) {
                                if (c2 == 101) {
                                    if (c3 == 97) {
                                        if (c4 == 114) {
                                            /* Re-clear and re-draw boot banner */
                                            i = 0;
                                            while (i < 4000) {
                                                __store32(0xB8000, i, 0x1F201F20);
                                                i = i + 4;
                                            }
                                            __store32(0xB8000, 0, 0x1F201E46);
                                            __store32(0xB8000, 4, 0x1F201E61);
                                            __store32(0xB8000, 8, 0x1F201E73);
                                            __store32(0xB8000, 12, 0x1F201E74);
                                            __store32(0xB8000, 16, 0x1F201E4F);
                                            __store32(0xB8000, 20, 0x1F201E53);
                                            __store32(0xB8000, 24, 0x1F201F20);
                                            __store32(0xB8000, 28, 0x1F201F76);
                                            __store32(0xB8000, 32, 0x1F201F32);
                                            __store32(0xB8000, 36, 0x1F201F2E);
                                            __store32(0xB8000, 40, 0x1F201F31);
                                            srow = 3;
                                        }
                                    }
                                }
                            }
                        }

                        /* Command: ver (118,101,114) */
                        if (c0 == 118) {
                            if (c1 == 101) {
                                if (c2 == 114) {
                                    if (c3 == 0) {
                                        p = orow * 160;
                                        __store32(0xB8000, p, 0x1F200E46); p = p + 4;
                                        __store32(0xB8000, p, 0x1F200E61); p = p + 4;
                                        __store32(0xB8000, p, 0x1F200E73); p = p + 4;
                                        __store32(0xB8000, p, 0x1F200E74); p = p + 4;
                                        __store32(0xB8000, p, 0x1F200E4F); p = p + 4;
                                        __store32(0xB8000, p, 0x1F200E53); p = p + 8;
                                        __store32(0xB8000, p, 0x1F200F76); p = p + 4;
                                        __store32(0xB8000, p, 0x1F200F32); p = p + 4;
                                        __store32(0xB8000, p, 0x1F200F2E); p = p + 4;
                                        __store32(0xB8000, p, 0x1F200F31);
                                        srow = orow + 2;
                                    }
                                }
                            }
                        }

                        /* Scroll reset if past row 23 */
                        if (srow > 23) {
                            i = 0;
                            while (i < 4000) {
                                __store32(0xB8000, i, 0x1F201F20);
                                i = i + 4;
                            }
                            __store32(0xB8000, 0, 0x1F201E46);
                            __store32(0xB8000, 4, 0x1F201E61);
                            __store32(0xB8000, 8, 0x1F201E73);
                            __store32(0xB8000, 12, 0x1F201E74);
                            __store32(0xB8000, 16, 0x1F201E4F);
                            __store32(0xB8000, 20, 0x1F201E53);
                            __store32(0xB8000, 24, 0x1F201F20);
                            __store32(0xB8000, 28, 0x1F201F76);
                            __store32(0xB8000, 32, 0x1F201F32);
                            __store32(0xB8000, 36, 0x1F201F2E);
                            __store32(0xB8000, 40, 0x1F201F31);
                            srow = 3;
                        }

                        /* New prompt */
                        p = srow * 160;
                        __store32(0xB8000, p, 0x1F200F3E);
                        __store32(0xB8000, p + 4, 0x1F200F20);
                        cursor = p + 8;
                        clen = 0;
                        c0 = 0; c1 = 0; c2 = 0; c3 = 0; c4 = 0;
                    }

                    /* ---- BACKSPACE (0x0E) ---- */
                    if (key == 0x0E) {
                        if (clen > 0) {
                            clen = clen - 1;
                            cursor = cursor - 4;
                            __store32(0xB8000, cursor, 0x1F201F20);
                            if (clen == 0) { c0 = 0; }
                            if (clen == 1) { c1 = 0; }
                            if (clen == 2) { c2 = 0; }
                            if (clen == 3) { c3 = 0; }
                            if (clen == 4) { c4 = 0; }
                        }
                    }

                    /* ---- Regular keys ---- */
                    ascii = 0;
                    if (key == 0x10) { ascii = 113; }
                    if (key == 0x11) { ascii = 119; }
                    if (key == 0x12) { ascii = 101; }
                    if (key == 0x13) { ascii = 114; }
                    if (key == 0x14) { ascii = 116; }
                    if (key == 0x15) { ascii = 121; }
                    if (key == 0x16) { ascii = 117; }
                    if (key == 0x17) { ascii = 105; }
                    if (key == 0x18) { ascii = 111; }
                    if (key == 0x19) { ascii = 112; }
                    if (key == 0x1E) { ascii = 97; }
                    if (key == 0x1F) { ascii = 115; }
                    if (key == 0x20) { ascii = 100; }
                    if (key == 0x21) { ascii = 102; }
                    if (key == 0x22) { ascii = 103; }
                    if (key == 0x23) { ascii = 104; }
                    if (key == 0x24) { ascii = 106; }
                    if (key == 0x25) { ascii = 107; }
                    if (key == 0x26) { ascii = 108; }
                    if (key == 0x2C) { ascii = 122; }
                    if (key == 0x2D) { ascii = 120; }
                    if (key == 0x2E) { ascii = 99; }
                    if (key == 0x2F) { ascii = 118; }
                    if (key == 0x30) { ascii = 98; }
                    if (key == 0x31) { ascii = 110; }
                    if (key == 0x32) { ascii = 109; }
                    if (key == 0x39) { ascii = 32; }
                    if (key == 0x0C) { ascii = 45; }
                    if (key == 0x34) { ascii = 46; }
                    /* Numbers */
                    if (key == 0x02) { ascii = 49; }
                    if (key == 0x03) { ascii = 50; }
                    if (key == 0x04) { ascii = 51; }
                    if (key == 0x05) { ascii = 52; }
                    if (key == 0x06) { ascii = 53; }
                    if (key == 0x07) { ascii = 54; }
                    if (key == 0x08) { ascii = 55; }
                    if (key == 0x09) { ascii = 56; }
                    if (key == 0x0A) { ascii = 57; }
                    if (key == 0x0B) { ascii = 48; }

                    if (ascii > 0) {
                        if (clen < 30) {
                            __store32(0xB8000, cursor, 0x1F200F00 | ascii);
                            cursor = cursor + 4;
                            if (clen == 0) { c0 = ascii; }
                            if (clen == 1) { c1 = ascii; }
                            if (clen == 2) { c2 = ascii; }
                            if (clen == 3) { c3 = ascii; }
                            if (clen == 4) { c4 = ascii; }
                            clen = clen + 1;
                        }
                    }
                }
            }
        }
    }
}
