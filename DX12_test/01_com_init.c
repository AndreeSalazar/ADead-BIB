/* DX12_test/01_com_init.c — COM Initialization Test
 * 
 * CODEGEN NEEDED: printf (already works)
 * TESTS: CoInitializeEx + CoUninitialize
 * EXPECTED: "COM init OK" or "COM init FAILED"
 */
#include <stdio.h>
#include <windows.h>

/* COM constants */
#define COINIT_MULTITHREADED 0x0
#define S_OK 0
#define S_FALSE 1

/* ole32.dll */
long CoInitializeEx(void *pvReserved, unsigned long dwCoInit);
void CoUninitialize(void);

int main(void) {
    long hr;
    
    printf("=== DX Test 01: COM Init ===\n");
    
    hr = CoInitializeEx(0, COINIT_MULTITHREADED);
    if (hr == S_OK || hr == S_FALSE) {
        printf("COM init OK (hr=0x%08X)\n", hr);
        CoUninitialize();
        printf("COM uninit OK\n");
    } else {
        printf("COM init FAILED (hr=0x%08X)\n", hr);
    }
    
    printf("=== Test 01 PASSED ===\n");
    return 0;
}
