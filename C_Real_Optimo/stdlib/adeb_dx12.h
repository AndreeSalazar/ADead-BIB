/* adeb_dx12.h — DirectX 12 + COM (mínimo) */
#ifndef ADEB_DX12_H
#define ADEB_DX12_H

#include "adeb_types.h"

typedef void* ID3D12Device;
typedef void* ID3D12CommandQueue;
typedef void* ID3D12Resource;
typedef void* ID3D12GraphicsCommandList;
typedef void* ID3D12CommandAllocator;
typedef void* ID3D12Fence;
typedef void* IDXGIAdapter;
typedef void* IDXGIFactory4;
typedef void* IDXGISwapChain3;
typedef int32_t HRESULT;
typedef struct GUID { uint32_t a; uint16_t b, c; uint8_t d[8]; } GUID;
typedef GUID IID;

#define S_OK 0L

extern HRESULT D3D12CreateDevice(IDXGIAdapter* adapter, uint32_t feature_level,
                                 const IID* riid, void** ppDevice);
extern HRESULT CreateDXGIFactory1(const IID* riid, void** ppFactory);
extern HRESULT D3D12GetDebugInterface(const IID* riid, void** ppvDebug);

#endif /* ADEB_DX12_H */
