/* DX12_test/02_dxgi_factory.c — DXGI Factory Creation Test
 * 
 * CODEGEN NEEDED: C-01 (struct GUID), C-09 (cast)
 * TESTS: CreateDXGIFactory1 + EnumAdapters
 */
#include <stdio.h>
#include <windows.h>
#include <dxgi.h>

int main(void) {
    void *pFactory;
    HRESULT hr;
    
    printf("=== DX Test 02: DXGI Factory ===\n");
    
    /* Create DXGI factory */
    IID factoryIID;
    factoryIID.Data1 = 0x770aae78;
    factoryIID.Data2 = 0xf26f;
    factoryIID.Data3 = 0x4dba;
    factoryIID.Data4[0] = 0xa8; factoryIID.Data4[1] = 0x29;
    factoryIID.Data4[2] = 0x25; factoryIID.Data4[3] = 0x3c;
    factoryIID.Data4[4] = 0x83; factoryIID.Data4[5] = 0xd1;
    factoryIID.Data4[6] = 0xb3; factoryIID.Data4[7] = 0x87;
    
    pFactory = 0;
    hr = CreateDXGIFactory1(&factoryIID, &pFactory);
    
    if (hr >= 0 && pFactory != 0) {
        printf("DXGI Factory created OK\n");
        
        /* Enumerate adapters */
        void *pAdapter;
        int i;
        for (i = 0; i < 4; i++) {
            pAdapter = 0;
            hr = IDXGIFactory_EnumAdapters(pFactory, i, &pAdapter);
            if (hr < 0) break;
            printf("  Adapter %d found\n", i);
            IUnknown_Release(pAdapter);
        }
        printf("  Total adapters: %d\n", i);
        
        IUnknown_Release(pFactory);
        printf("Factory released\n");
    } else {
        printf("DXGI Factory FAILED (hr=0x%08X)\n", hr);
    }
    
    printf("=== Test 02 DONE ===\n");
    return 0;
}
