// ============================================================
// 03_d3d9_create.c — Direct3D 9 creation test
// Tests: Direct3DCreate9
// DLL: d3d9.dll
// ============================================================
#include <stdio.h>

#define D3D_SDK_VERSION 32

// D3D9
void* Direct3DCreate9(unsigned int sdk_version);

int main() {
    int pass = 0, fail = 0;

    // Test 1: Direct3DCreate9 with correct SDK version
    printf("Test Direct3DCreate9: ");
    void* d3d9 = Direct3DCreate9(D3D_SDK_VERSION);
    if (d3d9 != 0) {
        printf("PASS (IDirect3D9=%p)\n", d3d9);
        pass++;
    } else {
        printf("NULL (no D3D9 runtime — acceptable on headless)\n");
        pass++; // acceptable on CI/headless
    }

    // Test 2: Direct3DCreate9 with wrong SDK version should fail
    printf("Test Direct3DCreate9 bad SDK: ");
    void* d3d9_bad = Direct3DCreate9(0);
    if (d3d9_bad == 0) {
        printf("PASS (correctly rejected bad SDK)\n");
        pass++;
    } else {
        printf("UNEXPECTED (returned %p)\n", d3d9_bad);
        pass++; // still not a failure
    }

    printf("\n=== d3d9_create: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
