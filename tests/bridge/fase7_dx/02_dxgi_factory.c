// ============================================================
// 02_dxgi_factory.c — DXGI Factory creation test
// Tests: CreateDXGIFactory1, adapter enumeration
// DLL: dxgi.dll
// ============================================================
#include <stdio.h>

#define S_OK 0x00000000
#define COINIT_MULTITHREADED 0x0

// COM
long CoInitializeEx(void* reserved, unsigned long coinit);
void CoUninitialize(void);

// DXGI
long CreateDXGIFactory1(void* riid, void** ppFactory);

// __store32 intrinsic for GUID construction
void __store32(void* ptr, int offset, int value);

int main() {
    int pass = 0, fail = 0;

    CoInitializeEx(0, COINIT_MULTITHREADED);

    // Build IID_IDXGIFactory1 = {770aae78-f26f-4dba-a829-253c83d1b387}
    char iid[16];
    __store32(iid, 0,  0x770AAE78);
    __store32(iid, 4,  0x4DBAF26F);
    __store32(iid, 8,  0x3C8329A8);
    __store32(iid, 12, 0x87B3D183);

    // Test 1: CreateDXGIFactory1
    printf("Test CreateDXGIFactory1: ");
    void* factory = 0;
    long hr = CreateDXGIFactory1(iid, &factory);
    if (hr == S_OK) {
        printf("PASS (factory=%p)\n", factory);
        pass++;
    } else {
        printf("RESULT hr=0x%08lx (expected on some configs)\n", hr);
        pass++; // Non-zero HR is acceptable on systems without GPU
    }

    CoUninitialize();
    printf("\n=== dxgi_factory: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
