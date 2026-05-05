// Test 11: Bitwise operators
int main() {
    int a = 0xF0;     // 11110000
    int b = 0x0F;     // 00001111
    int and_ = a & b; // 0
    int or_  = a | b; // 0xFF = 255
    int xor_ = a ^ b; // 0xFF = 255
    int not_ = ~0;    // -1
    int shl  = 1 << 4;// 16
    int shr  = 32 >> 2; // 8
    int sum = and_ + or_ + xor_ + shl + shr; // 0+255+255+16+8 = 534
    return sum - 534 + not_ + 1; // 0 + (-1) + 1 = 0
}
