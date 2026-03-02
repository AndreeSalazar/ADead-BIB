// ============================================================
// ADead-BIB C Example — Operaciones Bitwise & Hardware
// ============================================================
// Bitwise operations, bit manipulation, flags, register-style
// operations — lo que necesita FastOS para hardware directo.
// ============================================================

#include <stdio.h>

// ==================== Bit Manipulation ====================

unsigned int rotate_left(unsigned int val, int bits) {
    return (val << bits) | (val >> (32 - bits));
}

unsigned int rotate_right(unsigned int val, int bits) {
    return (val >> bits) | (val << (32 - bits));
}

int count_leading_zeros(unsigned int val) {
    if (val == 0) return 32;
    int count = 0;
    while (!(val & 0x80000000)) {
        count++;
        val <<= 1;
    }
    return count;
}

int count_trailing_zeros(unsigned int val) {
    if (val == 0) return 32;
    int count = 0;
    while (!(val & 1)) {
        count++;
        val >>= 1;
    }
    return count;
}

unsigned int popcount(unsigned int val) {
    unsigned int count = 0;
    while (val) {
        count += val & 1;
        val >>= 1;
    }
    return count;
}

unsigned int next_power_of_2(unsigned int val) {
    if (val == 0) return 1;
    val--;
    val |= val >> 1;
    val |= val >> 2;
    val |= val >> 4;
    val |= val >> 8;
    val |= val >> 16;
    return val + 1;
}

int is_power_of_2(unsigned int val) {
    return val != 0 && (val & (val - 1)) == 0;
}

unsigned int reverse_bits(unsigned int val) {
    unsigned int result = 0;
    for (int i = 0; i < 32; i++) {
        result = (result << 1) | (val & 1);
        val >>= 1;
    }
    return result;
}

unsigned int extract_bits(unsigned int val, int start, int len) {
    unsigned int mask = ((1 << len) - 1) << start;
    return (val & mask) >> start;
}

unsigned int insert_bits(unsigned int val, unsigned int bits, int start, int len) {
    unsigned int mask = ((1 << len) - 1) << start;
    val &= ~mask;
    val |= (bits << start) & mask;
    return val;
}

// ==================== CRC-32 ====================

unsigned int crc32_byte(unsigned int crc, unsigned char byte) {
    crc ^= byte;
    for (int i = 0; i < 8; i++) {
        if (crc & 1) {
            crc = (crc >> 1) ^ 0xEDB88320;
        } else {
            crc >>= 1;
        }
    }
    return crc;
}

unsigned int crc32(const unsigned char *data, int len) {
    unsigned int crc = 0xFFFFFFFF;
    for (int i = 0; i < len; i++) {
        crc = crc32_byte(crc, data[i]);
    }
    return ~crc;
}

// ==================== Flags Register ====================

unsigned int flags = 0;

void flag_set(int bit) { flags |= (1 << bit); }
void flag_clear(int bit) { flags &= ~(1 << bit); }
void flag_toggle(int bit) { flags ^= (1 << bit); }
int flag_test(int bit) { return (flags >> bit) & 1; }

void print_flags(unsigned int f, int bits) {
    printf("  Flags: 0b");
    for (int i = bits - 1; i >= 0; i--) {
        printf("%d", (f >> i) & 1);
    }
    printf(" (0x%08x)\n", f);
}

// ==================== Byte Operations ====================

unsigned int swap_bytes_32(unsigned int val) {
    return ((val & 0xFF000000) >> 24) |
           ((val & 0x00FF0000) >> 8)  |
           ((val & 0x0000FF00) << 8)  |
           ((val & 0x000000FF) << 24);
}

unsigned short swap_bytes_16(unsigned short val) {
    return (val >> 8) | (val << 8);
}

unsigned char high_nibble(unsigned char val) {
    return (val >> 4) & 0x0F;
}

unsigned char low_nibble(unsigned char val) {
    return val & 0x0F;
}

// ==================== Main ====================

int main() {
    printf("=== ADead-BIB: Bitwise Operations ===\n\n");

    // Basic operations
    unsigned int a = 0xDEADBEEF;
    unsigned int b = 0xCAFEBABE;
    printf("Basic:\n");
    printf("  a = 0x%08x\n", a);
    printf("  b = 0x%08x\n", b);
    printf("  a & b = 0x%08x\n", a & b);
    printf("  a | b = 0x%08x\n", a | b);
    printf("  a ^ b = 0x%08x\n", a ^ b);
    printf("  ~a    = 0x%08x\n", ~a);

    // Shifts
    printf("\nShifts:\n");
    unsigned int val = 0xFF;
    printf("  0xFF << 8  = 0x%08x\n", val << 8);
    printf("  0xFF << 16 = 0x%08x\n", val << 16);
    printf("  0xFF << 24 = 0x%08x\n", val << 24);
    printf("  0x80000000 >> 4 = 0x%08x\n", 0x80000000u >> 4);

    // Rotations
    printf("\nRotations:\n");
    printf("  rotl(0x12345678, 8)  = 0x%08x\n", rotate_left(0x12345678, 8));
    printf("  rotr(0x12345678, 8)  = 0x%08x\n", rotate_right(0x12345678, 8));

    // Bit counting
    printf("\nBit Counting:\n");
    printf("  popcount(0xFF)      = %d\n", popcount(0xFF));
    printf("  popcount(0xAAAAAAAA)= %d\n", popcount(0xAAAAAAAA));
    printf("  clz(0x00100000)     = %d\n", count_leading_zeros(0x00100000));
    printf("  ctz(0x00100000)     = %d\n", count_trailing_zeros(0x00100000));

    // Power of 2
    printf("\nPower of 2:\n");
    for (int i = 0; i <= 10; i++) {
        int v = 1 << i;
        printf("  %4d: is_pow2=%d  next_pow2=%d\n", v, is_power_of_2(v), next_power_of_2(v));
    }
    printf("  %4d: is_pow2=%d  next_pow2=%d\n", 7, is_power_of_2(7), next_power_of_2(7));
    printf("  %4d: is_pow2=%d  next_pow2=%d\n", 100, is_power_of_2(100), next_power_of_2(100));

    // Bit extraction/insertion
    printf("\nBit Fields:\n");
    unsigned int reg = 0xABCD1234;
    printf("  reg = 0x%08x\n", reg);
    printf("  bits[4:8]  = 0x%x\n", extract_bits(reg, 4, 4));
    printf("  bits[8:8]  = 0x%x\n", extract_bits(reg, 8, 8));
    printf("  bits[16:16]= 0x%x\n", extract_bits(reg, 16, 16));
    reg = insert_bits(reg, 0xF, 4, 4);
    printf("  insert 0xF at [4:4] = 0x%08x\n", reg);

    // Byte swap (endianness)
    printf("\nEndianness:\n");
    printf("  bswap32(0x12345678) = 0x%08x\n", swap_bytes_32(0x12345678));
    printf("  bswap16(0xABCD)     = 0x%04x\n", swap_bytes_16(0xABCD));
    printf("  nibbles(0xF3): hi=%x lo=%x\n", high_nibble(0xF3), low_nibble(0xF3));

    // Flags register
    printf("\nFlags Register:\n");
    flags = 0;
    flag_set(0);
    flag_set(3);
    flag_set(7);
    flag_set(15);
    print_flags(flags, 16);
    printf("  test(3)=%d  test(4)=%d\n", flag_test(3), flag_test(4));
    flag_toggle(3);
    printf("  After toggle(3):\n");
    print_flags(flags, 16);

    // CRC-32
    printf("\nCRC-32:\n");
    const char *msg = "ADead-BIB";
    unsigned int checksum = crc32((const unsigned char *)msg, 9);
    printf("  crc32(\"%s\") = 0x%08x\n", msg, checksum);

    printf("\n=== Complete ===\n");
    return 0;
}
