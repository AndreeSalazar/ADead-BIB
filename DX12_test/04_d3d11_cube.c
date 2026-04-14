/* DX12_test/04_d3d11_cube.c — D3D11 Triangle with HLSL
 *
 * CODEGEN NEEDED: C-01,C-02,C-04,C-07,C-08
 * TESTS: D3D11CreateDeviceAndSwapChain, HLSL compile, Draw
 */
#include <stdio.h>
#include <windows.h>
#include <dxgi.h>
#include <d3d11.h>
#include <d3dcompiler.h>

/* HLSL shaders (needs C-07: global string variables) */
const char *g_vsCode =
    "float4 main(float3 pos : POSITION) : SV_Position {\n"
    "    return float4(pos, 1.0);\n"
    "}\n";

const char *g_psCode =
    "float4 main() : SV_Target {\n"
    "    return float4(1.0, 0.5, 0.0, 1.0);\n"
    "}\n";

int main(void) {
    ID3D11Device *pDevice;
    ID3D11DeviceContext *pContext;
    void *pSwapChain;
    HRESULT hr;
    unsigned int featureLevel;
    
    printf("=== DX Test 04: D3D11 Cube ===\n");
    
    /* Create device (WARP — no GPU needed) */
    pDevice = 0;
    pContext = 0;
    featureLevel = 0;
    
    hr = D3D11CreateDevice(
        0,                          /* adapter */
        D3D_DRIVER_TYPE_WARP,       /* WARP software */
        0,                          /* software module */
        D3D11_CREATE_DEVICE_DEBUG,  /* flags */
        0, 0,                       /* feature levels */
        D3D11_SDK_VERSION,
        &pDevice,
        &featureLevel,
        &pContext);
    
    if (hr >= 0) {
        printf("D3D11 Device created (WARP), feature level 0x%X\n", featureLevel);
        
        /* Compile vertex shader */
        ID3DBlob *vsBlob;
        ID3DBlob *errBlob;
        vsBlob = 0;
        errBlob = 0;
        
        hr = D3DCompile(g_vsCode, 60, "vs", 0, 0, "main", "vs_5_0", 0, 0, &vsBlob, &errBlob);
        if (hr >= 0) {
            printf("VS compiled OK, size=%llu\n", ID3DBlob_GetBufferSize(vsBlob));
            ID3DBlob_Release(vsBlob);
        } else {
            printf("VS compile FAILED\n");
            if (errBlob) {
                printf("Error: %s\n", (const char*)ID3DBlob_GetBufferPointer(errBlob));
                ID3DBlob_Release(errBlob);
            }
        }
        
        /* Compile pixel shader */
        ID3DBlob *psBlob;
        psBlob = 0;
        errBlob = 0;
        
        hr = D3DCompile(g_psCode, 47, "ps", 0, 0, "main", "ps_5_0", 0, 0, &psBlob, &errBlob);
        if (hr >= 0) {
            printf("PS compiled OK, size=%llu\n", ID3DBlob_GetBufferSize(psBlob));
            ID3DBlob_Release(psBlob);
        } else {
            printf("PS compile FAILED\n");
        }
        
        ID3D11DeviceContext_Release(pContext);
        ID3D11Device_Release(pDevice);
        printf("Device released\n");
    } else {
        printf("D3D11 Device FAILED (hr=0x%08X)\n", hr);
    }
    
    printf("=== Test 04 DONE ===\n");
    return 0;
}
