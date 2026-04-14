/* DX12_test/05_d3d12_init.c — D3D12 Device + CommandQueue Init
 *
 * CODEGEN NEEDED: C-01 (struct), C-09 (cast/GUID)
 * TESTS: D3D12GetDebugInterface, D3D12CreateDevice, CreateCommandQueue
 */
#include <stdio.h>
#include <windows.h>
#include <dxgi.h>
#include <d3d12.h>

int main(void) {
    HRESULT hr;
    ID3D12Debug *pDebug;
    ID3D12Device *pDevice;
    ID3D12CommandQueue *pQueue;
    
    printf("=== DX Test 05: D3D12 Init ===\n");
    
    /* Enable debug layer */
    pDebug = 0;
    hr = D3D12GetDebugInterface(&IID_ID3D12Debug, (void**)&pDebug);
    if (hr >= 0 && pDebug != 0) {
        ID3D12Debug_EnableDebugLayer(pDebug);
        ID3D12Debug_Release(pDebug);
        printf("Debug layer enabled\n");
    } else {
        printf("Debug layer not available (OK for release)\n");
    }
    
    /* Create D3D12 device */
    pDevice = 0;
    hr = D3D12CreateDevice(0, D3D_FEATURE_LEVEL_11_0, &IID_ID3D12Device, (void**)&pDevice);
    
    if (hr >= 0 && pDevice != 0) {
        printf("D3D12 Device created OK\n");
        
        /* Create command queue */
        D3D12_COMMAND_QUEUE_DESC queueDesc;
        queueDesc.Type = D3D12_COMMAND_LIST_TYPE_DIRECT;
        queueDesc.Priority = 0;
        queueDesc.Flags = D3D12_COMMAND_QUEUE_FLAG_NONE;
        queueDesc.NodeMask = 0;
        
        pQueue = 0;
        hr = ID3D12Device_CreateCommandQueue(pDevice, &queueDesc,
            &IID_ID3D12CommandQueue, (void**)&pQueue);
        
        if (hr >= 0) {
            printf("CommandQueue created OK\n");
            ID3D12CommandQueue_Release(pQueue);
        } else {
            printf("CommandQueue FAILED (hr=0x%08X)\n", hr);
        }
        
        /* Create command allocator */
        ID3D12CommandAllocator *pAlloc;
        pAlloc = 0;
        hr = ID3D12Device_CreateCommandAllocator(pDevice,
            D3D12_COMMAND_LIST_TYPE_DIRECT,
            &IID_ID3D12CommandAllocator, (void**)&pAlloc);
        
        if (hr >= 0) {
            printf("CommandAllocator created OK\n");
            ID3D12CommandAllocator_Release(pAlloc);
        } else {
            printf("CommandAllocator FAILED (hr=0x%08X)\n", hr);
        }
        
        /* Create fence */
        ID3D12Fence *pFence;
        pFence = 0;
        hr = ID3D12Device_CreateFence(pDevice, 0, D3D12_FENCE_FLAG_NONE,
            &IID_ID3D12Fence, (void**)&pFence);
        
        if (hr >= 0) {
            printf("Fence created OK\n");
            ID3D12Fence_Release(pFence);
        } else {
            printf("Fence FAILED (hr=0x%08X)\n", hr);
        }
        
        ID3D12Device_Release(pDevice);
        printf("Device released\n");
    } else {
        printf("D3D12 Device FAILED (hr=0x%08X)\n", hr);
    }
    
    printf("=== Test 05 DONE ===\n");
    return 0;
}
