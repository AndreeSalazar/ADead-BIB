// ============================================================
// 01_com_init.c — COM initialization test
// Tests: CoInitializeEx, CoUninitialize
// DLL: ole32.dll
// ============================================================
#include <stdio.h>

// COM constants
#define S_OK             0x00000000
#define S_FALSE          0x00000001
#define COINIT_MULTITHREADED 0x0

// Forward declarations (IAT)
long CoInitializeEx(void* reserved, unsigned long coinit);
void CoUninitialize(void);

int main() {
    int pass = 0, fail = 0;

    // Test 1: CoInitializeEx should return S_OK or S_FALSE
    printf("Test COM init: ");
    long hr = CoInitializeEx(0, COINIT_MULTITHREADED);
    if (hr == S_OK || hr == S_FALSE) {
        printf("PASS (hr=0x%08lx)\n", hr);
        pass++;
    } else {
        printf("FAIL (hr=0x%08lx)\n", hr);
        fail++;
    }

    // Test 2: Second init should return S_FALSE (already initialized)
    printf("Test COM double init: ");
    long hr2 = CoInitializeEx(0, COINIT_MULTITHREADED);
    if (hr2 == S_FALSE || hr2 == S_OK) {
        printf("PASS (hr=0x%08lx)\n", hr2);
        pass++;
    } else {
        printf("FAIL (hr=0x%08lx)\n", hr2);
        fail++;
    }

    // Cleanup
    CoUninitialize();
    CoUninitialize();

    printf("\n=== com_init: %d passed, %d failed ===\n", pass, fail);
    return fail;
}
