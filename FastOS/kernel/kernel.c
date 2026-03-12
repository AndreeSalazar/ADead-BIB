/* FastOS v2.1 Kernel - ADead-BIB C (Inline, __store16 constant offsets)
 * __store16(base, CONST, val) for proper 2-byte VGA cells
 * __store32(base, var, val) only for screen clear and dynamic writes
 * __inl/__outl for PCI, __inb for PS/2 keyboard, __cpuid for CPU */

void kernel_main(void) {
    int i; int sc; int key; int ascii; int cursor; int srow; int p;
    int clen; int c0; int c1; int c2; int c3; int c4; int orow;
    int vb; int vd; int vc; int maxl; int avx2; int tmp; int ch;
    int nibble; int brand_a; int brand_b; int brand_c; int brand_d;
    int pci_addr; int pci_val; int pci_dev; int pci_vendor; int pci_device;
    int n; int hexval;

    __cli();

    /* Clear screen blue */
    i = 0;
    while (i < 4000) {
        __store32(0xB8000, i, 0x1F201F20);
        i = i + 4;
    }

    /* CPUID */
    maxl = __cpuid_eax(0);
    vb = __cpuid_ebx(0);
    vc = __cpuid_ecx(0);
    vd = __cpuid_edx(0);
    avx2 = 0;
    if (maxl > 6) {
        tmp = __cpuid_ebx(7);
        if (tmp & 32) { avx2 = 1; }
    }

    /* Row 0: "FastOS v2.1" using __store16 constant offsets (2-byte stride) */
    __store16(0xB8000, 0, 0x1E46);
    __store16(0xB8000, 2, 0x1E61);
    __store16(0xB8000, 4, 0x1E73);
    __store16(0xB8000, 6, 0x1E74);
    __store16(0xB8000, 8, 0x1E4F);
    __store16(0xB8000, 10, 0x1E53);
    __store16(0xB8000, 12, 0x1F20);
    __store16(0xB8000, 14, 0x1F76);
    __store16(0xB8000, 16, 0x1F32);
    __store16(0xB8000, 18, 0x1F2E);
    __store16(0xB8000, 20, 0x1F31);

    /* Row 0 right: "256-bit" or "128-bit" */
    if (avx2 > 0) {
        __store16(0xB8000, 130, 0x0E32);
        __store16(0xB8000, 132, 0x0E35);
        __store16(0xB8000, 134, 0x0E36);
        __store16(0xB8000, 136, 0x0E2D);
        __store16(0xB8000, 138, 0x0A62);
        __store16(0xB8000, 140, 0x0A69);
        __store16(0xB8000, 142, 0x0A74);
    }
    if (avx2 == 0) {
        __store16(0xB8000, 130, 0x0C31);
        __store16(0xB8000, 132, 0x0C32);
        __store16(0xB8000, 134, 0x0C38);
        __store16(0xB8000, 136, 0x0C2D);
        __store16(0xB8000, 138, 0x0C62);
        __store16(0xB8000, 140, 0x0C69);
        __store16(0xB8000, 142, 0x0C74);
    }

    /* Row 1: Vendor string from CPUID (EBX:EDX:ECX = AuthenticAMD) */
    ch = vb & 0xFF;        __store16(0xB8000, 160, 0x0B00 | ch);
    ch = (vb >> 8) & 0xFF; __store16(0xB8000, 162, 0x0B00 | ch);
    ch = (vb >> 16) & 0xFF; __store16(0xB8000, 164, 0x0B00 | ch);
    ch = (vb >> 24) & 0xFF; __store16(0xB8000, 166, 0x0B00 | ch);
    ch = vd & 0xFF;        __store16(0xB8000, 168, 0x0B00 | ch);
    ch = (vd >> 8) & 0xFF; __store16(0xB8000, 170, 0x0B00 | ch);
    ch = (vd >> 16) & 0xFF; __store16(0xB8000, 172, 0x0B00 | ch);
    ch = (vd >> 24) & 0xFF; __store16(0xB8000, 174, 0x0B00 | ch);
    ch = vc & 0xFF;        __store16(0xB8000, 176, 0x0B00 | ch);
    ch = (vc >> 8) & 0xFF; __store16(0xB8000, 178, 0x0B00 | ch);
    ch = (vc >> 16) & 0xFF; __store16(0xB8000, 180, 0x0B00 | ch);
    ch = (vc >> 24) & 0xFF; __store16(0xB8000, 182, 0x0B00 | ch);

    /* Row 2-3: Brand string (CPUID 0x80000002-4, 48 chars) */
    tmp = __cpuid_eax(0x80000000);
    if (tmp > 0x80000004) {
        brand_a = __cpuid_eax(0x80000002);
        brand_b = __cpuid_ebx(0x80000002);
        brand_c = __cpuid_ecx(0x80000002);
        brand_d = __cpuid_edx(0x80000002);
        ch = brand_a & 0xFF;        if (ch > 31) { __store16(0xB8000, 320, 0x0F00 | ch); }
        ch = (brand_a >> 8) & 0xFF;  if (ch > 31) { __store16(0xB8000, 322, 0x0F00 | ch); }
        ch = (brand_a >> 16) & 0xFF; if (ch > 31) { __store16(0xB8000, 324, 0x0F00 | ch); }
        ch = (brand_a >> 24) & 0xFF; if (ch > 31) { __store16(0xB8000, 326, 0x0F00 | ch); }
        ch = brand_b & 0xFF;        if (ch > 31) { __store16(0xB8000, 328, 0x0F00 | ch); }
        ch = (brand_b >> 8) & 0xFF;  if (ch > 31) { __store16(0xB8000, 330, 0x0F00 | ch); }
        ch = (brand_b >> 16) & 0xFF; if (ch > 31) { __store16(0xB8000, 332, 0x0F00 | ch); }
        ch = (brand_b >> 24) & 0xFF; if (ch > 31) { __store16(0xB8000, 334, 0x0F00 | ch); }
        ch = brand_c & 0xFF;        if (ch > 31) { __store16(0xB8000, 336, 0x0F00 | ch); }
        ch = (brand_c >> 8) & 0xFF;  if (ch > 31) { __store16(0xB8000, 338, 0x0F00 | ch); }
        ch = (brand_c >> 16) & 0xFF; if (ch > 31) { __store16(0xB8000, 340, 0x0F00 | ch); }
        ch = (brand_c >> 24) & 0xFF; if (ch > 31) { __store16(0xB8000, 342, 0x0F00 | ch); }
        ch = brand_d & 0xFF;        if (ch > 31) { __store16(0xB8000, 344, 0x0F00 | ch); }
        ch = (brand_d >> 8) & 0xFF;  if (ch > 31) { __store16(0xB8000, 346, 0x0F00 | ch); }
        ch = (brand_d >> 16) & 0xFF; if (ch > 31) { __store16(0xB8000, 348, 0x0F00 | ch); }
        ch = (brand_d >> 24) & 0xFF; if (ch > 31) { __store16(0xB8000, 350, 0x0F00 | ch); }
        brand_a = __cpuid_eax(0x80000003);
        brand_b = __cpuid_ebx(0x80000003);
        brand_c = __cpuid_ecx(0x80000003);
        brand_d = __cpuid_edx(0x80000003);
        ch = brand_a & 0xFF;        if (ch > 31) { __store16(0xB8000, 352, 0x0F00 | ch); }
        ch = (brand_a >> 8) & 0xFF;  if (ch > 31) { __store16(0xB8000, 354, 0x0F00 | ch); }
        ch = (brand_a >> 16) & 0xFF; if (ch > 31) { __store16(0xB8000, 356, 0x0F00 | ch); }
        ch = (brand_a >> 24) & 0xFF; if (ch > 31) { __store16(0xB8000, 358, 0x0F00 | ch); }
        ch = brand_b & 0xFF;        if (ch > 31) { __store16(0xB8000, 360, 0x0F00 | ch); }
        ch = (brand_b >> 8) & 0xFF;  if (ch > 31) { __store16(0xB8000, 362, 0x0F00 | ch); }
        ch = (brand_b >> 16) & 0xFF; if (ch > 31) { __store16(0xB8000, 364, 0x0F00 | ch); }
        ch = (brand_b >> 24) & 0xFF; if (ch > 31) { __store16(0xB8000, 366, 0x0F00 | ch); }
        ch = brand_c & 0xFF;        if (ch > 31) { __store16(0xB8000, 368, 0x0F00 | ch); }
        ch = (brand_c >> 8) & 0xFF;  if (ch > 31) { __store16(0xB8000, 370, 0x0F00 | ch); }
        ch = (brand_c >> 16) & 0xFF; if (ch > 31) { __store16(0xB8000, 372, 0x0F00 | ch); }
        ch = (brand_c >> 24) & 0xFF; if (ch > 31) { __store16(0xB8000, 374, 0x0F00 | ch); }
        ch = brand_d & 0xFF;        if (ch > 31) { __store16(0xB8000, 376, 0x0F00 | ch); }
        ch = (brand_d >> 8) & 0xFF;  if (ch > 31) { __store16(0xB8000, 378, 0x0F00 | ch); }
        ch = (brand_d >> 16) & 0xFF; if (ch > 31) { __store16(0xB8000, 380, 0x0F00 | ch); }
        ch = (brand_d >> 24) & 0xFF; if (ch > 31) { __store16(0xB8000, 382, 0x0F00 | ch); }
        brand_a = __cpuid_eax(0x80000004);
        brand_b = __cpuid_ebx(0x80000004);
        brand_c = __cpuid_ecx(0x80000004);
        brand_d = __cpuid_edx(0x80000004);
        ch = brand_a & 0xFF;        if (ch > 31) { __store16(0xB8000, 384, 0x0F00 | ch); }
        ch = (brand_a >> 8) & 0xFF;  if (ch > 31) { __store16(0xB8000, 386, 0x0F00 | ch); }
        ch = (brand_a >> 16) & 0xFF; if (ch > 31) { __store16(0xB8000, 388, 0x0F00 | ch); }
        ch = (brand_a >> 24) & 0xFF; if (ch > 31) { __store16(0xB8000, 390, 0x0F00 | ch); }
        ch = brand_b & 0xFF;        if (ch > 31) { __store16(0xB8000, 392, 0x0F00 | ch); }
        ch = (brand_b >> 8) & 0xFF;  if (ch > 31) { __store16(0xB8000, 394, 0x0F00 | ch); }
        ch = (brand_b >> 16) & 0xFF; if (ch > 31) { __store16(0xB8000, 396, 0x0F00 | ch); }
        ch = (brand_b >> 24) & 0xFF; if (ch > 31) { __store16(0xB8000, 398, 0x0F00 | ch); }
        ch = brand_c & 0xFF;        if (ch > 31) { __store16(0xB8000, 400, 0x0F00 | ch); }
        ch = (brand_c >> 8) & 0xFF;  if (ch > 31) { __store16(0xB8000, 402, 0x0F00 | ch); }
        ch = (brand_c >> 16) & 0xFF; if (ch > 31) { __store16(0xB8000, 404, 0x0F00 | ch); }
        ch = (brand_c >> 24) & 0xFF; if (ch > 31) { __store16(0xB8000, 406, 0x0F00 | ch); }
        ch = brand_d & 0xFF;        if (ch > 31) { __store16(0xB8000, 408, 0x0F00 | ch); }
        ch = (brand_d >> 8) & 0xFF;  if (ch > 31) { __store16(0xB8000, 410, 0x0F00 | ch); }
        ch = (brand_d >> 16) & 0xFF; if (ch > 31) { __store16(0xB8000, 412, 0x0F00 | ch); }
        ch = (brand_d >> 24) & 0xFF; if (ch > 31) { __store16(0xB8000, 414, 0x0F00 | ch); }
    }

    /* Row 4: Features - AVX2 SSE4.2 AES */
    if (avx2 > 0) {
        __store16(0xB8000, 640, 0x0A41);
        __store16(0xB8000, 642, 0x0A56);
        __store16(0xB8000, 644, 0x0A58);
        __store16(0xB8000, 646, 0x0A32);
        __store16(0xB8000, 648, 0x1F20);
    }
    if (avx2 == 0) {
        __store16(0xB8000, 640, 0x0C53);
        __store16(0xB8000, 642, 0x0C53);
        __store16(0xB8000, 644, 0x0C45);
        __store16(0xB8000, 646, 0x1F20);
        __store16(0xB8000, 648, 0x1F20);
    }
    tmp = __cpuid_ecx(1);
    if (tmp & 0x100000) {
        __store16(0xB8000, 650, 0x0A53);
        __store16(0xB8000, 652, 0x0A53);
        __store16(0xB8000, 654, 0x0A45);
        __store16(0xB8000, 656, 0x0A34);
        __store16(0xB8000, 658, 0x0A2E);
        __store16(0xB8000, 660, 0x0A32);
        __store16(0xB8000, 662, 0x1F20);
    }
    if (tmp & 0x2000000) {
        __store16(0xB8000, 664, 0x0A41);
        __store16(0xB8000, 666, 0x0A45);
        __store16(0xB8000, 668, 0x0A53);
    }

    /* Row 5: "Kernel OK  ADead-BIB C" */
    __store16(0xB8000, 800, 0x0A4B);
    __store16(0xB8000, 802, 0x0A65);
    __store16(0xB8000, 804, 0x0A72);
    __store16(0xB8000, 806, 0x0A6E);
    __store16(0xB8000, 808, 0x0A65);
    __store16(0xB8000, 810, 0x0A6C);
    __store16(0xB8000, 812, 0x1F20);
    __store16(0xB8000, 814, 0x0A4F);
    __store16(0xB8000, 816, 0x0A4B);
    __store16(0xB8000, 820, 0x0B41);
    __store16(0xB8000, 822, 0x0B44);
    __store16(0xB8000, 824, 0x0B65);
    __store16(0xB8000, 826, 0x0B61);
    __store16(0xB8000, 828, 0x0B64);
    __store16(0xB8000, 830, 0x0B2D);
    __store16(0xB8000, 832, 0x0B42);
    __store16(0xB8000, 834, 0x0B49);
    __store16(0xB8000, 836, 0x0B42);
    __store16(0xB8000, 838, 0x1F20);
    __store16(0xB8000, 840, 0x0B43);

    /* Row 7: Shell prompt "> " */
    srow = 7;
    __store16(0xB8000, 1120, 0x0F3E);
    __store16(0xB8000, 1122, 0x0F20);
    cursor = 1124;
    clen = 0;
    c0 = 0; c1 = 0; c2 = 0; c3 = 0; c4 = 0;

    /* Keyboard polling loop */
    while (1) {
        sc = __inb(0x64);
        if (sc & 1) {
            key = __inb(0x60);
            if (key > 0) {
                if (key < 128) {
                    /* ENTER */
                    if (key == 0x1C) {
                        orow = srow + 1;

                        /* help (104,101,108,112) */
                        if (c0 == 104) { if (c1 == 101) { if (c2 == 108) { if (c3 == 112) {
                            p = orow * 160;
                            __store16(0xB8000, p, 0x0E43); __store16(0xB8000, p+2, 0x0E6F);
                            __store16(0xB8000, p+4, 0x0E6D); __store16(0xB8000, p+6, 0x0E6D);
                            __store16(0xB8000, p+8, 0x0E61); __store16(0xB8000, p+10, 0x0E6E);
                            __store16(0xB8000, p+12, 0x0E64); __store16(0xB8000, p+14, 0x0E73);
                            __store16(0xB8000, p+16, 0x0E3A);
                            p = (orow + 1) * 160 + 4;
                            __store16(0xB8000, p, 0x0F68); __store16(0xB8000, p+2, 0x0F65);
                            __store16(0xB8000, p+4, 0x0F6C); __store16(0xB8000, p+6, 0x0F70);
                            __store16(0xB8000, p+16, 0x0773); __store16(0xB8000, p+18, 0x0768);
                            __store16(0xB8000, p+20, 0x076F); __store16(0xB8000, p+22, 0x0777);
                            p = (orow + 2) * 160 + 4;
                            __store16(0xB8000, p, 0x0F63); __store16(0xB8000, p+2, 0x0F70);
                            __store16(0xB8000, p+4, 0x0F75);
                            __store16(0xB8000, p+16, 0x0743); __store16(0xB8000, p+18, 0x0750);
                            __store16(0xB8000, p+20, 0x0755); __store16(0xB8000, p+22, 0x1F20);
                            __store16(0xB8000, p+24, 0x0769); __store16(0xB8000, p+26, 0x076E);
                            __store16(0xB8000, p+28, 0x0766); __store16(0xB8000, p+30, 0x076F);
                            p = (orow + 3) * 160 + 4;
                            __store16(0xB8000, p, 0x0F70); __store16(0xB8000, p+2, 0x0F63);
                            __store16(0xB8000, p+4, 0x0F69);
                            __store16(0xB8000, p+16, 0x0750); __store16(0xB8000, p+18, 0x0743);
                            __store16(0xB8000, p+20, 0x0749); __store16(0xB8000, p+22, 0x1F20);
                            __store16(0xB8000, p+24, 0x0773); __store16(0xB8000, p+26, 0x0763);
                            __store16(0xB8000, p+28, 0x0761); __store16(0xB8000, p+30, 0x076E);
                            p = (orow + 4) * 160 + 4;
                            __store16(0xB8000, p, 0x0F63); __store16(0xB8000, p+2, 0x0F6C);
                            __store16(0xB8000, p+4, 0x0F65); __store16(0xB8000, p+6, 0x0F61);
                            __store16(0xB8000, p+8, 0x0F72);
                            __store16(0xB8000, p+16, 0x0763); __store16(0xB8000, p+18, 0x076C);
                            __store16(0xB8000, p+20, 0x0765); __store16(0xB8000, p+22, 0x0761);
                            __store16(0xB8000, p+24, 0x0772);
                            p = (orow + 5) * 160 + 4;
                            __store16(0xB8000, p, 0x0F76); __store16(0xB8000, p+2, 0x0F65);
                            __store16(0xB8000, p+4, 0x0F72);
                            __store16(0xB8000, p+16, 0x0776); __store16(0xB8000, p+18, 0x0765);
                            __store16(0xB8000, p+20, 0x0772); __store16(0xB8000, p+22, 0x0773);
                            __store16(0xB8000, p+24, 0x0769); __store16(0xB8000, p+26, 0x076F);
                            __store16(0xB8000, p+28, 0x076E);
                            srow = orow + 7;
                        }}}}

                        /* cpu (99,112,117) */
                        if (c0 == 99) { if (c1 == 112) { if (c2 == 117) { if (c3 == 0) {
                            p = orow * 160;
                            __store16(0xB8000, p, 0x0E43); __store16(0xB8000, p+2, 0x0E50);
                            __store16(0xB8000, p+4, 0x0E55); __store16(0xB8000, p+6, 0x0E3A);
                            __store16(0xB8000, p+8, 0x1F20);
                            ch = vb & 0xFF; __store16(0xB8000, p+10, 0x0F00|ch);
                            ch = (vb>>8)&0xFF; __store16(0xB8000, p+12, 0x0F00|ch);
                            ch = (vb>>16)&0xFF; __store16(0xB8000, p+14, 0x0F00|ch);
                            ch = (vb>>24)&0xFF; __store16(0xB8000, p+16, 0x0F00|ch);
                            ch = vd & 0xFF; __store16(0xB8000, p+18, 0x0F00|ch);
                            ch = (vd>>8)&0xFF; __store16(0xB8000, p+20, 0x0F00|ch);
                            ch = (vd>>16)&0xFF; __store16(0xB8000, p+22, 0x0F00|ch);
                            ch = (vd>>24)&0xFF; __store16(0xB8000, p+24, 0x0F00|ch);
                            ch = vc & 0xFF; __store16(0xB8000, p+26, 0x0F00|ch);
                            ch = (vc>>8)&0xFF; __store16(0xB8000, p+28, 0x0F00|ch);
                            ch = (vc>>16)&0xFF; __store16(0xB8000, p+30, 0x0F00|ch);
                            ch = (vc>>24)&0xFF; __store16(0xB8000, p+32, 0x0F00|ch);
                            p = (orow+1)*160;
                            if (avx2 > 0) {
                                __store16(0xB8000, p, 0x0A41); __store16(0xB8000, p+2, 0x0A56);
                                __store16(0xB8000, p+4, 0x0A58); __store16(0xB8000, p+6, 0x0A32);
                                __store16(0xB8000, p+8, 0x1F20);
                            }
                            __store16(0xB8000, p+10, 0x0A53); __store16(0xB8000, p+12, 0x0A53);
                            __store16(0xB8000, p+14, 0x0A45); __store16(0xB8000, p+16, 0x0A34);
                            __store16(0xB8000, p+18, 0x0A2E); __store16(0xB8000, p+20, 0x0A32);
                            __store16(0xB8000, p+22, 0x1F20);
                            __store16(0xB8000, p+24, 0x0A41); __store16(0xB8000, p+26, 0x0A45);
                            __store16(0xB8000, p+28, 0x0A53);
                            srow = orow + 3;
                        }}}}

                        /* pci (112,99,105) */
                        if (c0 == 112) { if (c1 == 99) { if (c2 == 105) { if (c3 == 0) {
                            p = orow * 160;
                            __store16(0xB8000, p, 0x0E50); __store16(0xB8000, p+2, 0x0E43);
                            __store16(0xB8000, p+4, 0x0E49); __store16(0xB8000, p+6, 0x1F20);
                            __store16(0xB8000, p+8, 0x0E44); __store16(0xB8000, p+10, 0x0E65);
                            __store16(0xB8000, p+12, 0x0E76); __store16(0xB8000, p+14, 0x0E69);
                            __store16(0xB8000, p+16, 0x0E63); __store16(0xB8000, p+18, 0x0E65);
                            __store16(0xB8000, p+20, 0x0E73); __store16(0xB8000, p+22, 0x0E3A);
                            n = 1;
                            pci_dev = 0;
                            while (pci_dev < 32) {
                                pci_addr = 0x80000000 | (pci_dev << 11);
                                __outl(0xCF8, pci_addr);
                                pci_val = __inl(0xCFC);
                                pci_vendor = pci_val & 0xFFFF;
                                pci_device = (pci_val >> 16) & 0xFFFF;
                                if (pci_vendor != 0xFFFF) { if (pci_vendor > 0) {
                                    p = (orow + n) * 160;
                                    /* dev:vendor:device in hex */
                                    nibble = (pci_dev>>4)&0xF;
                                    if (nibble < 10) { ch = 48+nibble; } if (nibble > 9) { ch = 55+nibble; }
                                    __store16(0xB8000, p, 0x0700|ch);
                                    nibble = pci_dev & 0xF;
                                    if (nibble < 10) { ch = 48+nibble; } if (nibble > 9) { ch = 55+nibble; }
                                    __store16(0xB8000, p+2, 0x0700|ch);
                                    __store16(0xB8000, p+4, 0x0F3A);
                                    /* vendor 4 hex */
                                    hexval = pci_vendor;
                                    nibble=(hexval>>12)&0xF; if(nibble<10){ch=48+nibble;}if(nibble>9){ch=55+nibble;}
                                    __store16(0xB8000, p+6, 0x0A00|ch);
                                    nibble=(hexval>>8)&0xF; if(nibble<10){ch=48+nibble;}if(nibble>9){ch=55+nibble;}
                                    __store16(0xB8000, p+8, 0x0A00|ch);
                                    nibble=(hexval>>4)&0xF; if(nibble<10){ch=48+nibble;}if(nibble>9){ch=55+nibble;}
                                    __store16(0xB8000, p+10, 0x0A00|ch);
                                    nibble=hexval&0xF; if(nibble<10){ch=48+nibble;}if(nibble>9){ch=55+nibble;}
                                    __store16(0xB8000, p+12, 0x0A00|ch);
                                    __store16(0xB8000, p+14, 0x0F3A);
                                    /* device 4 hex */
                                    hexval = pci_device;
                                    nibble=(hexval>>12)&0xF; if(nibble<10){ch=48+nibble;}if(nibble>9){ch=55+nibble;}
                                    __store16(0xB8000, p+16, 0x0B00|ch);
                                    nibble=(hexval>>8)&0xF; if(nibble<10){ch=48+nibble;}if(nibble>9){ch=55+nibble;}
                                    __store16(0xB8000, p+18, 0x0B00|ch);
                                    nibble=(hexval>>4)&0xF; if(nibble<10){ch=48+nibble;}if(nibble>9){ch=55+nibble;}
                                    __store16(0xB8000, p+20, 0x0B00|ch);
                                    nibble=hexval&0xF; if(nibble<10){ch=48+nibble;}if(nibble>9){ch=55+nibble;}
                                    __store16(0xB8000, p+22, 0x0B00|ch);
                                    /* Vendor names */
                                    if (pci_vendor == 0x10DE) {
                                        __store16(0xB8000,p+26,0x0A4E); __store16(0xB8000,p+28,0x0A56);
                                        __store16(0xB8000,p+30,0x0A49); __store16(0xB8000,p+32,0x0A44);
                                        __store16(0xB8000,p+34,0x0A49); __store16(0xB8000,p+36,0x0A41);
                                    }
                                    if (pci_vendor == 0x8086) {
                                        __store16(0xB8000,p+26,0x0B49); __store16(0xB8000,p+28,0x0B6E);
                                        __store16(0xB8000,p+30,0x0B74); __store16(0xB8000,p+32,0x0B65);
                                        __store16(0xB8000,p+34,0x0B6C);
                                    }
                                    if (pci_vendor == 0x1022) {
                                        __store16(0xB8000,p+26,0x0C41); __store16(0xB8000,p+28,0x0C4D);
                                        __store16(0xB8000,p+30,0x0C44);
                                    }
                                    if (pci_vendor == 0x1234) {
                                        __store16(0xB8000,p+26,0x0D56); __store16(0xB8000,p+28,0x0D47);
                                        __store16(0xB8000,p+30,0x0D41);
                                    }
                                    if (pci_vendor == 0x1AF4) {
                                        __store16(0xB8000,p+26,0x0D51); __store16(0xB8000,p+28,0x0D45);
                                        __store16(0xB8000,p+30,0x0D4D); __store16(0xB8000,p+32,0x0D55);
                                    }
                                    n = n + 1;
                                    if (n > 14) { pci_dev = 32; }
                                }}
                                pci_dev = pci_dev + 1;
                            }
                            srow = orow + n + 1;
                        }}}}

                        /* clear (99,108,101,97,114) */
                        if (c0 == 99) { if (c1 == 108) { if (c2 == 101) { if (c3 == 97) { if (c4 == 114) {
                            i = 0;
                            while (i < 4000) { __store32(0xB8000, i, 0x1F201F20); i = i + 4; }
                            __store16(0xB8000, 0, 0x1E46); __store16(0xB8000, 2, 0x1E61);
                            __store16(0xB8000, 4, 0x1E73); __store16(0xB8000, 6, 0x1E74);
                            __store16(0xB8000, 8, 0x1E4F); __store16(0xB8000, 10, 0x1E53);
                            __store16(0xB8000, 12, 0x1F20); __store16(0xB8000, 14, 0x1F76);
                            __store16(0xB8000, 16, 0x1F32); __store16(0xB8000, 18, 0x1F2E);
                            __store16(0xB8000, 20, 0x1F31);
                            srow = 3;
                        }}}}}

                        /* ver (118,101,114) */
                        if (c0 == 118) { if (c1 == 101) { if (c2 == 114) { if (c3 == 0) {
                            p = orow * 160;
                            __store16(0xB8000, p, 0x0E46); __store16(0xB8000, p+2, 0x0E61);
                            __store16(0xB8000, p+4, 0x0E73); __store16(0xB8000, p+6, 0x0E74);
                            __store16(0xB8000, p+8, 0x0E4F); __store16(0xB8000, p+10, 0x0E53);
                            __store16(0xB8000, p+12, 0x1F20);
                            __store16(0xB8000, p+14, 0x0F76); __store16(0xB8000, p+16, 0x0F32);
                            __store16(0xB8000, p+18, 0x0F2E); __store16(0xB8000, p+20, 0x0F31);
                            srow = orow + 2;
                        }}}}

                        /* Scroll reset */
                        if (srow > 23) {
                            i = 0;
                            while (i < 4000) { __store32(0xB8000, i, 0x1F201F20); i = i + 4; }
                            __store16(0xB8000, 0, 0x1E46); __store16(0xB8000, 2, 0x1E61);
                            __store16(0xB8000, 4, 0x1E73); __store16(0xB8000, 6, 0x1E74);
                            __store16(0xB8000, 8, 0x1E4F); __store16(0xB8000, 10, 0x1E53);
                            __store16(0xB8000, 12, 0x1F20); __store16(0xB8000, 14, 0x1F76);
                            __store16(0xB8000, 16, 0x1F32); __store16(0xB8000, 18, 0x1F2E);
                            __store16(0xB8000, 20, 0x1F31);
                            srow = 3;
                        }

                        /* New prompt */
                        p = srow * 160;
                        __store16(0xB8000, p, 0x0F3E);
                        __store16(0xB8000, p + 2, 0x0F20);
                        cursor = p + 4;
                        clen = 0;
                        c0 = 0; c1 = 0; c2 = 0; c3 = 0; c4 = 0;
                    }

                    /* BACKSPACE */
                    if (key == 0x0E) {
                        if (clen > 0) {
                            clen = clen - 1;
                            cursor = cursor - 2;
                            __store16(0xB8000, cursor, 0x1F20);
                            if (clen == 0) { c0 = 0; }
                            if (clen == 1) { c1 = 0; }
                            if (clen == 2) { c2 = 0; }
                            if (clen == 3) { c3 = 0; }
                            if (clen == 4) { c4 = 0; }
                        }
                    }

                    /* Regular keys */
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
                            __store16(0xB8000, cursor, 0x0F00 | ascii);
                            cursor = cursor + 2;
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
