// Test: <uchar.h> + <tgmath.h> — C11 Unicode chars + type-generic math
// Expected: Compile OK — char16_t/char32_t types + tgmath macros

#include <uchar.h>
#include <stdio.h>

int main() {
    printf("=== uchar/tgmath test ===\n");

    // uchar.h types
    char16_t c16 = 0x0041;  // 'A' in UTF-16
    char32_t c32 = 0x0001F600;  // emoji codepoint

    printf("char16_t size=%d char32_t size=%d\n",
           (int)sizeof(char16_t), (int)sizeof(char32_t));
    printf("c16=0x%x c32=0x%x\n", (unsigned)c16, (unsigned)c32);

    printf("=== uchar/tgmath OK ===\n");
    return 0;
}
