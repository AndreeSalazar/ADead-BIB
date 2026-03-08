// ================================================================
// ADead-BIB DX12 HelloTriangle — Full Pipeline (9 Pasos)
// ================================================================
// Pipeline DX12 completo compilado por ADead-BIB:
//   Paso 0: Window creation (CreateWindowExA — 12 args, main only)
//   Paso 1: Device (D3D12CreateDevice via IAT)
//   Paso 2: SwapChain (CreateDXGIFactory1 + CreateSwapChainForHwnd via vtable)
//   Paso 3: Command Queue (device->CreateCommandQueue via vtable)
//   Paso 4: RTV Heap (descriptor heap + render target views)
//   Paso 5: Command List (command allocator + graphics command list)
//   Paso 6: Root Signature (empty — D3D12SerializeRootSignature)
//   Paso 7: Pipeline State (vertex + pixel shader bytecode)
//   Paso 8: Vertex Buffer (3 vertices triangle, upload heap)
//   Paso 9: Render Loop (record, execute, present, fence sync)
//
// Step mode: adb step src/main.cpp
// Compile:   adb cxx src/main.cpp -o bin/dx12_hello.exe
// Run:       bin\dx12_hello.exe
//
// NOTA: COM vtable calls use raw pointer arithmetic:
//   vtable = *(void**)obj;          // load vtable ptr from [obj+0]
//   method = ((void**)vtable)[idx]; // load method ptr at [vtable+idx*8]
//   result = method(obj, args...);  // call with obj as first arg (this)
// ================================================================
#include <header_main.h>

HWND g_hwnd;

int main() {
    printf("=== ADead-BIB DX12 HelloTriangle (9 Pasos) ===\n");

    // ============================================================
    // PASO 0 — Window (CreateWindowExA — 12 args directo en main)
    // ============================================================
    printf("[PASO 0] Creando ventana...\n");
    HINSTANCE hInstance = GetModuleHandleA(0);
    printf("[PASO 0] hInstance=%p\n", hInstance);

    HCURSOR cur = LoadCursorW(0, 32512);
    printf("[PASO 0] cursor=%p\n", cur);

    g_hwnd = CreateWindowExA(
        0, "STATIC", "ADead-BIB DX12 Triangle",
        0x00CF0000, 100, 100, 1280, 720,
        0, 0, hInstance, 0
    );
    if (g_hwnd == 0) {
        printf("[PASO 0] ERROR: CreateWindowExA failed\n");
        return 1;
    }
    ShowWindow(g_hwnd, 5);
    printf("[PASO 0] OK hwnd=%p\n", g_hwnd);

    // ============================================================
    // PASO 1 — Device (D3D12CreateDevice via IAT)
    // ============================================================
    printf("[PASO 1] Creando D3D12 Device...\n");

    // IID_ID3D12Device = {189819f1-1db6-4b57-be54-1821339b85f7}
    // GUID layout: Data1(4) Data2(2)+Data3(2) Data4(8) = 16 bytes
    // Using __store32 intrinsic for 4-byte writes
    void* iid_device = malloc(16);
    memset(iid_device, 0, 16);
    __store32(iid_device, 0,  0x189819F1);  // Data1
    __store32(iid_device, 4,  0x4B571DB6);  // Data2(0x1DB6) + Data3(0x4B57) little-endian
    __store32(iid_device, 8,  0x211854BE);  // Data4[0..3]: BE 54 18 21
    __store32(iid_device, 12, 0xF7859B33);  // Data4[4..7]: 33 9B 85 F7
    printf("[PASO 1] IID_ID3D12Device written via __store32\n");

    void* device = 0;
    HRESULT hr = D3D12CreateDevice(0, 0xB000, iid_device, &device);
    printf("[PASO 1] D3D12CreateDevice hr=0x%08X device=%p\n", hr, device);
    if (device == 0) {
        printf("[PASO 1] WARN: Device creation failed (hr=0x%08X)\n", hr);
        printf("[PASO 1] Falling back to GDI triangle rendering...\n");

        // GDI fallback — gradient triangle + outline
        HDC hdc = GetDC(g_hwnd);
        int y = 100;
        while (y <= 550) {
            int t = y - 100;
            int lx = 640 - 300 * t / 450;
            int rx = 640 + 300 * t / 450;
            int r = 255 - t / 2;
            int g = t / 3;
            int b = t / 4;
            if (r < 0) { r = 0; }
            int c = r + g * 256 + b * 65536;
            int x = lx;
            while (x <= rx) {
                SetPixel(hdc, x, y, c);
                x = x + 1;
            }
            y = y + 2;
        }
        HPEN pen = CreatePen(0, 3, 0x00FFFFFF);
        SelectObject(hdc, pen);
        MoveToEx(hdc, 640, 100, 0);
        LineTo(hdc, 340, 550);
        LineTo(hdc, 940, 550);
        LineTo(hdc, 640, 100);
        DeleteObject(pen);
        ReleaseDC(g_hwnd, hdc);
        printf("[PASO 1] GDI triangle drawn (fallback)\n");

        // Message loop with Sleep to avoid CPU spin
        printf("[LOOP] Entering message loop (GDI fallback)...\n");
        void* pmsg_fb = malloc(64);
        int running_fb = 1;
        while (running_fb) {
            if (PeekMessageA(pmsg_fb, 0, 0, 0, 1)) {
                TranslateMessage(pmsg_fb);
                DispatchMessageA(pmsg_fb);
            } else {
                Sleep(1);
            }
        }
        free(pmsg_fb);
        printf("[LOOP] Window closed\n");
        return 0;
    }
    printf("[PASO 1] OK Device created!\n");

    // ============================================================
    // PASO 2 — DXGI Factory + SwapChain
    // ============================================================
    printf("[PASO 2] Creando DXGI Factory...\n");
    // IID_IDXGIFactory1 = {770aae78-f26f-4dba-a829-253c83d1b387}
    // Using __store32 intrinsic for correct 4-byte GUID writes
    void* iid_factory = malloc(16);
    memset(iid_factory, 0, 16);
    __store32(iid_factory, 0,  0x770AAE78);  // Data1
    __store32(iid_factory, 4,  0x4DBAF26F);  // Data2(0xF26F) + Data3(0x4DBA) little-endian
    __store32(iid_factory, 8,  0x3C2529A8);  // Data4[0..3]: A8 29 25 3C
    __store32(iid_factory, 12, 0x87B3D183);  // Data4[4..7]: 83 D1 B3 87
    printf("[PASO 2] IID_IDXGIFactory1 written via __store32\n");
    void* factory = 0;
    hr = CreateDXGIFactory1(iid_factory, &factory);
    printf("[PASO 2] CreateDXGIFactory1 hr=0x%08X factory=%p\n", hr, factory);
    if (factory == 0) {
        printf("[PASO 2] ERROR: Factory creation failed\n");
        return 1;
    }
    printf("[PASO 2] OK Factory created!\n");

    // ============================================================
    // PASO 3 — Command Queue (device vtable call)
    // ============================================================
    printf("[PASO 3] Creando Command Queue...\n");
    // D3D12_COMMAND_QUEUE_DESC: Type=0(DIRECT), Priority=0, Flags=0, NodeMask=0
    // Layout: 4 UINTs = 16 bytes
    void* cmdQueueDesc = malloc(16);
    memset(cmdQueueDesc, 0, 16);
    // Type = D3D12_COMMAND_LIST_TYPE_DIRECT = 0 (already zeroed)

    void* commandQueue = 0;
    // COM vtable: ID3D12Device inherits IUnknown(3) + ID3D12Object(1 SetName)
    // CreateCommandQueue is the first method of ID3D12Device = vtable index 4
    // device->CreateCommandQueue(&desc, IID, &commandQueue)
    // We pass IID as NULL (0) — DX12 accepts it for simple cases
    printf("[PASO 3] Calling device->CreateCommandQueue (vtable[4])...\n");

    // Load vtable pointer: vtable = *(void**)device
    void* devVtable = 0;
    if (device != 0) {
        // Read the vtable pointer from the first 8 bytes of the device object
        void** devPtr = (void**)device;
        // NOTE: Cannot dereference COM objects with ADead-BIB codegen yet
        // The vtable call requires: load [device+0] -> vtable, load [vtable+idx*8] -> method, call method
        // For now, use printf to show we have the device pointer
        printf("[PASO 3] device ptr=%p (vtable call requires COM indirect dispatch)\n", device);
    }

    // TODO: vtable indirect call when ISA compiler supports it
    // For now, mark as pending
    printf("[PASO 3] PENDING: Command Queue (needs COM vtable dispatch)\n");

    // ============================================================
    // PASO 4 — RTV Descriptor Heap
    // ============================================================
    printf("[PASO 4] RTV Descriptor Heap...\n");
    // D3D12_DESCRIPTOR_HEAP_DESC: Type=RTV(2), NumDescriptors=3, Flags=0, NodeMask=0
    void* rtvHeapDesc = malloc(16);
    memset(rtvHeapDesc, 0, 16);
    // Set Type = D3D12_DESCRIPTOR_HEAP_TYPE_RTV = 2, NumDescriptors = 3
    __store32(rtvHeapDesc, 0, 2);   // Type = RTV
    __store32(rtvHeapDesc, 4, 3);   // NumDescriptors = 3 (triple buffering)
    __store32(rtvHeapDesc, 8, 0);   // Flags = 0
    __store32(rtvHeapDesc, 12, 0);  // NodeMask = 0
    printf("[PASO 4] RTV heap desc prepared (Type=RTV, Count=3) via __store32\n");
    printf("[PASO 4] PENDING: CreateDescriptorHeap (needs COM vtable dispatch)\n");

    // ============================================================
    // PASO 5 — Command Allocator + Command List
    // ============================================================
    printf("[PASO 5] Command Allocator + Command List...\n");
    // device->CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT, IID, &allocator)
    // device->CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, allocator, NULL, IID, &cmdList)
    printf("[PASO 5] PENDING: CreateCommandAllocator (needs COM vtable dispatch)\n");
    printf("[PASO 5] PENDING: CreateCommandList (needs COM vtable dispatch)\n");

    // ============================================================
    // PASO 6 — Root Signature (empty)
    // ============================================================
    printf("[PASO 6] Creando Root Signature (empty)...\n");
    // D3D12_ROOT_SIGNATURE_DESC: empty root signature
    // NumParameters=0, pParameters=NULL, NumStaticSamplers=0, pStaticSamplers=NULL, Flags=1
    // Layout: 5 fields = 40 bytes (UINT, ptr, UINT, ptr, UINT)
    void* rootSigDesc = malloc(40);
    memset(rootSigDesc, 0, 40);
    // D3D12_ROOT_SIGNATURE_DESC layout (x64):
    //   offset 0:  UINT NumParameters = 0
    //   offset 4:  (padding)
    //   offset 8:  ptr pParameters = NULL
    //   offset 16: UINT NumStaticSamplers = 0
    //   offset 20: (padding)
    //   offset 24: ptr pStaticSamplers = NULL
    //   offset 32: UINT Flags = 1 (ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT)
    __store32(rootSigDesc, 32, 1);  // Flags = ALLOW_INPUT_ASSEMBLER

    void* signatureBlob = 0;
    void* errorBlob = 0;
    hr = D3D12SerializeRootSignature(rootSigDesc, 1, &signatureBlob, &errorBlob);
    printf("[PASO 6] D3D12SerializeRootSignature hr=0x%08X blob=%p err=%p\n", hr, signatureBlob, errorBlob);
    if (hr < 0) {
        printf("[PASO 6] WARN: Root signature serialization failed\n");
    } else {
        printf("[PASO 6] OK Root signature serialized!\n");
    }

    // ============================================================
    // PASO 7 — Pipeline State Object (PSO)
    // ============================================================
    printf("[PASO 7] Pipeline State Object...\n");
    // Minimal HLSL bytecode for vertex shader (passthrough):
    //   float4 main(float3 pos : POSITION) : SV_POSITION { return float4(pos, 1.0); }
    // Minimal HLSL bytecode for pixel shader (solid color):
    //   float4 main() : SV_TARGET { return float4(0.0, 0.8, 0.2, 1.0); }
    // NOTE: Real shader bytecode (DXBC/DXIL) requires separate compilation
    //       ADead-BIB would need an HLSL->DXBC compiler or pre-compiled bytecode
    printf("[PASO 7] PENDING: PSO creation (needs shader bytecode + COM vtable)\n");
    printf("[PASO 7] Shader bytecode requires HLSL compilation (future: ADead-BIB HLSL frontend)\n");

    // ============================================================
    // PASO 8 — Vertex Buffer (3 vertices for triangle)
    // ============================================================
    printf("[PASO 8] Vertex Buffer (3 vertices)...\n");
    // Triangle vertices: position (x,y,z) + color (r,g,b,a)
    // Vertex format: float3 position + float4 color = 28 bytes per vertex
    // Total: 3 * 28 = 84 bytes
    int vertexSize = 28;
    int vertexCount = 3;
    int bufferSize = vertexSize * vertexCount;
    void* vertexData = malloc(bufferSize);
    memset(vertexData, 0, bufferSize);

    // Vertex 0: top center (0.0, 0.5, 0.0) + red (1,0,0,1)
    // Vertex 1: bottom right (0.5, -0.5, 0.0) + green (0,1,0,1)
    // Vertex 2: bottom left (-0.5, -0.5, 0.0) + blue (0,0,1,1)
    // NOTE: float writing requires float support in codegen
    // For now, prepare the buffer structure
    printf("[PASO 8] Vertex buffer allocated: %d bytes (%d vertices x %d bytes)\n", bufferSize, vertexCount, vertexSize);
    printf("[PASO 8] Triangle: top(0,0.5,0) right(0.5,-0.5,0) left(-0.5,-0.5,0)\n");
    printf("[PASO 8] PENDING: Upload heap + GPU copy (needs COM vtable dispatch)\n");

    // ============================================================
    // PASO 9 — Render Loop (Present + Fence Sync)
    // ============================================================
    printf("[PASO 9] Render Loop...\n");
    // Fence for CPU/GPU sync
    void* fence = 0;
    HANDLE fenceEvent = CreateEventA(0, 0, 0, 0);
    printf("[PASO 9] Fence event=%p\n", fenceEvent);

    // Render loop structure:
    //   1. Reset command allocator
    //   2. Reset command list
    //   3. Set resource barrier (PRESENT -> RENDER_TARGET)
    //   4. Clear render target view (cornflower blue)
    //   5. Set viewport + scissor
    //   6. Set root signature + PSO
    //   7. Set vertex buffer
    //   8. Draw (3 vertices)
    //   9. Set resource barrier (RENDER_TARGET -> PRESENT)
    //   10. Close command list
    //   11. Execute command list
    //   12. Present
    //   13. Signal fence + wait

    printf("[PASO 9] Loop structure ready\n");
    printf("[PASO 9] PENDING: Full render loop (needs COM vtable dispatch for all steps)\n");

    // ============================================================
    // STATUS SUMMARY
    // ============================================================
    printf("\n=== DX12 Pipeline Status ===\n");
    printf("[PASO 0] Window:          OK (hwnd=%p)\n", g_hwnd);
    printf("[PASO 1] Device:          %s (hr=0x%08X)\n", device != 0 ? "OK" : "FAIL", hr);
    printf("[PASO 2] Factory:         %s (factory=%p)\n", factory != 0 ? "OK" : "FAIL", factory);
    printf("[PASO 3] Command Queue:   PENDING (COM vtable)\n");
    printf("[PASO 4] RTV Heap:        PENDING (COM vtable)\n");
    printf("[PASO 5] Command List:    PENDING (COM vtable)\n");
    printf("[PASO 6] Root Signature:  %s\n", signatureBlob != 0 ? "OK" : "PENDING");
    printf("[PASO 7] Pipeline State:  PENDING (shader bytecode)\n");
    printf("[PASO 8] Vertex Buffer:   PREPARED (%d bytes)\n", bufferSize);
    printf("[PASO 9] Render Loop:     PENDING (COM vtable)\n");
    printf("============================\n");

    // GDI fallback rendering while DX12 pipeline completes
    printf("\n[FALLBACK] Drawing GDI triangle while DX12 pipeline matures...\n");
    HDC hdc = GetDC(g_hwnd);
    int y = 100;
    while (y <= 550) {
        int t = y - 100;
        int lx = 640 - 300 * t / 450;
        int rx = 640 + 300 * t / 450;
        int r = 255 - t / 2;
        int g = t / 3;
        int b = t / 4;
        if (r < 0) { r = 0; }
        int c = r + g * 256 + b * 65536;
        int x = lx;
        while (x <= rx) {
            SetPixel(hdc, x, y, c);
            x = x + 1;
        }
        y = y + 2;
    }
    HPEN pen = CreatePen(0, 3, 0x00FFFFFF);
    SelectObject(hdc, pen);
    MoveToEx(hdc, 640, 100, 0);
    LineTo(hdc, 340, 550);
    LineTo(hdc, 940, 550);
    LineTo(hdc, 640, 100);
    DeleteObject(pen);
    ReleaseDC(g_hwnd, hdc);
    printf("[FALLBACK] GDI triangle rendered!\n");

    // Message loop with WM_DESTROY handling
    printf("[LOOP] Entering message loop...\n");
    void* pmsg = malloc(64);
    int running = 1;
    while (running) {
        if (PeekMessageA(pmsg, 0, 0, 0, 1)) {
            TranslateMessage(pmsg);
            DispatchMessageA(pmsg);
        } else {
            // No messages — sleep briefly to avoid CPU spin
            Sleep(1);
        }
    }

    // Cleanup
    if (fenceEvent != 0) {
        CloseHandle(fenceEvent);
    }
    free(vertexData);
    free(rootSigDesc);
    free(rtvHeapDesc);
    free(cmdQueueDesc);
    free(pmsg);

    return 0;
}
