/* DX12_test/03_d3d9_cube.c — D3D9 Simple Cube
 *
 * CODEGEN NEEDED: C-01 (struct), C-02 (float), C-04 (vtable), C-08 (arrays)
 * TESTS: Direct3DCreate9, CreateDevice, Clear, BeginScene/EndScene, Present
 */
#include <stdio.h>
#include <windows.h>
#include <dxgi.h>
#include <d3d9.h>

/* Window procedure */
long __stdcall WndProc(HWND hwnd, unsigned int msg, unsigned long long wp, long long lp) {
    if (msg == 0x0002) { PostQuitMessage(0); return 0; } /* WM_DESTROY */
    return DefWindowProcA(hwnd, msg, wp, lp);
}

int main(void) {
    printf("=== DX Test 03: D3D9 Cube ===\n");
    
    /* Register window class */
    HWND hwnd;
    /* ... window creation code ... */
    /* Simplified: assume hwnd is created via RegisterClassA + CreateWindowExA */
    
    void *pD3D;
    void *pDevice;
    D3DPRESENT_PARAMETERS pp;
    HRESULT hr;
    int i;
    
    /* Zero out present params */
    /* NOTE: needs memset or field-by-field init (C-01 codegen fix) */
    pp.BackBufferWidth = 800;
    pp.BackBufferHeight = 600;
    pp.BackBufferFormat = D3DFMT_X8R8G8B8;
    pp.BackBufferCount = 1;
    pp.MultiSampleType = 0;
    pp.MultiSampleQuality = 0;
    pp.SwapEffect = D3DSWAPEFFECT_DISCARD;
    pp.hDeviceWindow = 0; /* hwnd */
    pp.Windowed = 1;
    pp.EnableAutoDepthStencil = 0;
    pp.AutoDepthStencilFormat = 0;
    pp.Flags = 0;
    pp.FullScreen_RefreshRateInHz = 0;
    pp.PresentationInterval = 0;
    
    /* Create D3D9 */
    pD3D = Direct3DCreate9(D3D_SDK_VERSION);
    if (pD3D == 0) {
        printf("Direct3DCreate9 FAILED\n");
        return 1;
    }
    printf("Direct3DCreate9 OK\n");
    
    /* Create device - NOTE: needs vtable call (C-04 codegen fix) */
    pDevice = 0;
    hr = IDirect3D9_CreateDevice(pD3D, D3DADAPTER_DEFAULT, D3DDEVTYPE_HAL,
        0, D3DCREATE_SOFTWARE_VERTEXPROCESSING, &pp, &pDevice);
    
    if (hr >= 0 && pDevice != 0) {
        printf("CreateDevice OK\n");
        
        /* Render 60 frames */
        for (i = 0; i < 60; i++) {
            IDirect3DDevice9_Clear(pDevice, 0, 0, D3DCLEAR_TARGET, 
                D3DCOLOR_XRGB(0, 0, (i*4)%256), 1, 0);
            IDirect3DDevice9_BeginScene(pDevice);
            /* TODO: Draw cube vertices here (needs C-02 float + C-08 arrays) */
            IDirect3DDevice9_EndScene(pDevice);
            IDirect3DDevice9_Present(pDevice, 0, 0, 0, 0);
        }
        printf("Rendered 60 frames\n");
        
        IDirect3DDevice9_Release(pDevice);
    } else {
        printf("CreateDevice FAILED (hr=0x%08X)\n", hr);
    }
    
    IDirect3D9_Release(pD3D);
    printf("=== Test 03 DONE ===\n");
    return 0;
}
