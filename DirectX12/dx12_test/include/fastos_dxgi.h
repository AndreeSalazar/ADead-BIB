// fastos_dxgi.h — ADead-BIB minimal DXGI interfaces
// Only what HelloTriangle needs
#pragma once

// ============================================================
// DXGI Enums / Constants
// ============================================================
#define DXGI_FORMAT_R8G8B8A8_UNORM 28
#define DXGI_FORMAT_R32G32B32_FLOAT 6
#define DXGI_FORMAT_R32G32B32A32_FLOAT 2

#define DXGI_USAGE_RENDER_TARGET_OUTPUT 0x00000020

#define DXGI_SWAP_EFFECT_FLIP_DISCARD 4

#define DXGI_ADAPTER_FLAG_SOFTWARE 2

typedef UINT DXGI_FORMAT;

// ============================================================
// DXGI Structures
// ============================================================
struct DXGI_SAMPLE_DESC {
    UINT Count;
    UINT Quality;
};

struct DXGI_RATIONAL {
    UINT Numerator;
    UINT Denominator;
};

struct DXGI_MODE_DESC {
    UINT Width;
    UINT Height;
    DXGI_RATIONAL RefreshRate;
    DXGI_FORMAT Format;
    UINT ScanlineOrdering;
    UINT Scaling;
};

struct DXGI_SWAP_CHAIN_DESC {
    DXGI_MODE_DESC BufferDesc;
    DXGI_SAMPLE_DESC SampleDesc;
    UINT BufferUsage;
    UINT BufferCount;
    HWND OutputWindow;
    BOOL Windowed;
    UINT SwapEffect;
    UINT Flags;
};

struct DXGI_SWAP_CHAIN_DESC1 {
    UINT Width;
    UINT Height;
    DXGI_FORMAT Format;
    BOOL Stereo;
    DXGI_SAMPLE_DESC SampleDesc;
    UINT BufferUsage;
    UINT BufferCount;
    UINT Scaling;
    UINT SwapEffect;
    UINT AlphaMode;
    UINT Flags;
};

struct DXGI_ADAPTER_DESC1 {
    WCHAR Description[128];
    UINT VendorId;
    UINT DeviceId;
    UINT SubSysId;
    UINT Revision;
    UINT64 DedicatedVideoMemory;
    UINT64 DedicatedSystemMemory;
    UINT64 SharedSystemMemory;
    LARGE_INTEGER AdapterLuid;
    UINT Flags;
};

// ============================================================
// DXGI COM Interfaces
// ============================================================
struct IDXGIObject : public IUnknown {};

struct IDXGIAdapter : public IDXGIObject {};

struct IDXGIAdapter1 : public IDXGIAdapter {
    virtual HRESULT GetDesc1(DXGI_ADAPTER_DESC1* pDesc) = 0;
};

struct IDXGIOutput : public IDXGIObject {};

struct IDXGISwapChain : public IDXGIObject {
    virtual HRESULT Present(UINT SyncInterval, UINT Flags) = 0;
    virtual HRESULT GetBuffer(UINT Buffer, REFIID riid, void** ppSurface) = 0;
    virtual HRESULT ResizeBuffers(UINT BufferCount, UINT Width, UINT Height, DXGI_FORMAT NewFormat, UINT SwapChainFlags) = 0;
};

struct IDXGISwapChain1 : public IDXGISwapChain {};

struct IDXGISwapChain3 : public IDXGISwapChain1 {
    virtual UINT GetCurrentBackBufferIndex() = 0;
};

struct IDXGIFactory : public IDXGIObject {};

struct IDXGIFactory1 : public IDXGIFactory {
    virtual HRESULT EnumAdapters1(UINT Adapter, IDXGIAdapter1** ppAdapter) = 0;
};

struct IDXGIFactory4 : public IDXGIFactory1 {
    virtual HRESULT CreateSwapChainForHwnd(
        IUnknown* pDevice,
        HWND hWnd,
        const DXGI_SWAP_CHAIN_DESC1* pDesc,
        const void* pFullscreenDesc,
        IDXGIOutput* pRestrictToOutput,
        IDXGISwapChain1** ppSwapChain) = 0;
};

// ============================================================
// DXGI API functions
// ============================================================
extern "C" {
    HRESULT CreateDXGIFactory1(REFIID riid, void** ppFactory);
    HRESULT CreateDXGIFactory2(UINT Flags, REFIID riid, void** ppFactory);
}
