// fastos_d3d12.h — ADead-BIB minimal D3D12 interfaces
// Only what HelloTriangle needs — not full d3d12.h
#pragma once

// ============================================================
// D3D12 Enums
// ============================================================
typedef UINT D3D12_COMMAND_LIST_TYPE;
#define D3D12_COMMAND_LIST_TYPE_DIRECT  0
#define D3D12_COMMAND_LIST_TYPE_BUNDLE  1
#define D3D12_COMMAND_LIST_TYPE_COMPUTE 2
#define D3D12_COMMAND_LIST_TYPE_COPY    3

typedef UINT D3D12_COMMAND_QUEUE_FLAGS;
#define D3D12_COMMAND_QUEUE_FLAG_NONE 0

typedef UINT D3D12_DESCRIPTOR_HEAP_TYPE;
#define D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV 0
#define D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER      1
#define D3D12_DESCRIPTOR_HEAP_TYPE_RTV          2
#define D3D12_DESCRIPTOR_HEAP_TYPE_DSV          3

typedef UINT D3D12_DESCRIPTOR_HEAP_FLAGS;
#define D3D12_DESCRIPTOR_HEAP_FLAG_NONE           0
#define D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE 1

typedef UINT D3D12_FENCE_FLAGS;
#define D3D12_FENCE_FLAG_NONE 0

typedef UINT D3D12_RESOURCE_STATES;
#define D3D12_RESOURCE_STATE_PRESENT       0
#define D3D12_RESOURCE_STATE_RENDER_TARGET 4

typedef UINT D3D12_HEAP_TYPE;
#define D3D12_HEAP_TYPE_DEFAULT  1
#define D3D12_HEAP_TYPE_UPLOAD   2
#define D3D12_HEAP_TYPE_READBACK 3

typedef UINT D3D12_INPUT_CLASSIFICATION;
#define D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA   0
#define D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA 1

typedef UINT D3D12_PRIMITIVE_TOPOLOGY_TYPE;
#define D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE 3

typedef UINT D3D12_FILL_MODE;
#define D3D12_FILL_MODE_SOLID 3

typedef UINT D3D12_CULL_MODE;
#define D3D12_CULL_MODE_NONE 1
#define D3D12_CULL_MODE_BACK 3

// ============================================================
// D3D12 Structures
// ============================================================
struct D3D12_COMMAND_QUEUE_DESC {
    D3D12_COMMAND_LIST_TYPE Type;
    INT Priority;
    D3D12_COMMAND_QUEUE_FLAGS Flags;
    UINT NodeMask;
};

struct D3D12_DESCRIPTOR_HEAP_DESC {
    D3D12_DESCRIPTOR_HEAP_TYPE Type;
    UINT NumDescriptors;
    D3D12_DESCRIPTOR_HEAP_FLAGS Flags;
    UINT NodeMask;
};

struct D3D12_CPU_DESCRIPTOR_HANDLE {
    UINT64 ptr;
};

struct D3D12_GPU_DESCRIPTOR_HANDLE {
    UINT64 ptr;
};

struct D3D12_VERTEX_BUFFER_VIEW {
    UINT64 BufferLocation;
    UINT SizeInBytes;
    UINT StrideInBytes;
};

struct D3D12_INDEX_BUFFER_VIEW {
    UINT64 BufferLocation;
    UINT SizeInBytes;
    UINT Format;
};

struct D3D12_INPUT_ELEMENT_DESC {
    LPCSTR SemanticName;
    UINT SemanticIndex;
    UINT Format;
    UINT InputSlot;
    UINT AlignedByteOffset;
    D3D12_INPUT_CLASSIFICATION InputSlotClass;
    UINT InstanceDataStepRate;
};

struct D3D12_VIEWPORT {
    FLOAT TopLeftX;
    FLOAT TopLeftY;
    FLOAT Width;
    FLOAT Height;
    FLOAT MinDepth;
    FLOAT MaxDepth;
};

struct D3D12_RECT {
    LONG left;
    LONG top;
    LONG right;
    LONG bottom;
};

struct D3D12_RESOURCE_BARRIER {
    UINT Type;
    UINT Flags;
    // Simplified — real struct has union of transition/aliasing/UAV
    struct {
        void* pResource;
        UINT Subresource;
        D3D12_RESOURCE_STATES StateBefore;
        D3D12_RESOURCE_STATES StateAfter;
    } Transition;
};

struct D3D12_HEAP_PROPERTIES {
    D3D12_HEAP_TYPE Type;
    UINT CPUPageProperty;
    UINT MemoryPoolPreference;
    UINT CreationNodeMask;
    UINT VisibleNodeMask;
};

struct D3D12_RESOURCE_DESC {
    UINT Dimension;
    UINT64 Alignment;
    UINT64 Width;
    UINT Height;
    UINT16 DepthOrArraySize;
    UINT16 MipLevels;
    UINT Format;
    UINT SampleCount;
    UINT SampleQuality;
    UINT Layout;
    UINT Flags;
};

struct D3D12_CLEAR_VALUE {
    UINT Format;
    float Color[4];
};

// ============================================================
// D3D12 COM Interfaces (minimal stubs)
// ============================================================
struct ID3D12Object : public IUnknown {
    virtual HRESULT SetName(LPCWSTR Name) = 0;
};

struct ID3D12DeviceChild : public ID3D12Object {};

struct ID3D12Pageable : public ID3D12DeviceChild {};

struct ID3D12Resource : public ID3D12Pageable {
    virtual HRESULT Map(UINT Subresource, const void* pReadRange, void** ppData) = 0;
    virtual void Unmap(UINT Subresource, const void* pWrittenRange) = 0;
    virtual UINT64 GetGPUVirtualAddress() = 0;
};

struct ID3D12CommandAllocator : public ID3D12Pageable {
    virtual HRESULT Reset() = 0;
};

struct ID3D12Fence : public ID3D12Pageable {
    virtual UINT64 GetCompletedValue() = 0;
    virtual HRESULT SetEventOnCompletion(UINT64 Value, HANDLE hEvent) = 0;
    virtual HRESULT Signal(UINT64 Value) = 0;
};

struct ID3D12DescriptorHeap : public ID3D12Pageable {
    virtual D3D12_CPU_DESCRIPTOR_HANDLE GetCPUDescriptorHandleForHeapStart() = 0;
    virtual D3D12_GPU_DESCRIPTOR_HANDLE GetGPUDescriptorHandleForHeapStart() = 0;
};

struct ID3D12RootSignature : public ID3D12DeviceChild {};

struct ID3D12PipelineState : public ID3D12Pageable {};

struct ID3D12CommandList : public ID3D12DeviceChild {};

struct ID3D12GraphicsCommandList : public ID3D12CommandList {
    virtual HRESULT Close() = 0;
    virtual HRESULT Reset(ID3D12CommandAllocator* pAllocator, ID3D12PipelineState* pInitialState) = 0;
    virtual void SetGraphicsRootSignature(ID3D12RootSignature* pRootSignature) = 0;
    virtual void RSSetViewports(UINT NumViewports, const D3D12_VIEWPORT* pViewports) = 0;
    virtual void RSSetScissorRects(UINT NumRects, const D3D12_RECT* pRects) = 0;
    virtual void ResourceBarrier(UINT NumBarriers, const D3D12_RESOURCE_BARRIER* pBarriers) = 0;
    virtual void OMSetRenderTargets(UINT NumRenderTargetDescriptors, const D3D12_CPU_DESCRIPTOR_HANDLE* pRenderTargetDescriptors, BOOL RTsSingleHandleToDescriptorRange, const D3D12_CPU_DESCRIPTOR_HANDLE* pDepthStencilDescriptor) = 0;
    virtual void ClearRenderTargetView(D3D12_CPU_DESCRIPTOR_HANDLE RenderTargetView, const FLOAT ColorRGBA[4], UINT NumRects, const D3D12_RECT* pRects) = 0;
    virtual void IASetPrimitiveTopology(UINT PrimitiveTopology) = 0;
    virtual void IASetVertexBuffers(UINT StartSlot, UINT NumViews, const D3D12_VERTEX_BUFFER_VIEW* pViews) = 0;
    virtual void DrawInstanced(UINT VertexCountPerInstance, UINT InstanceCount, UINT StartVertexLocation, UINT StartInstanceLocation) = 0;
};

struct ID3D12CommandQueue : public ID3D12Pageable {
    virtual void ExecuteCommandLists(UINT NumCommandLists, ID3D12CommandList* const* ppCommandLists) = 0;
    virtual HRESULT Signal(ID3D12Fence* pFence, UINT64 Value) = 0;
};

struct ID3D12Device : public ID3D12Object {
    virtual HRESULT CreateCommandQueue(const D3D12_COMMAND_QUEUE_DESC* pDesc, REFIID riid, void** ppCommandQueue) = 0;
    virtual HRESULT CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE type, REFIID riid, void** ppCommandAllocator) = 0;
    virtual HRESULT CreateCommandList(UINT nodeMask, D3D12_COMMAND_LIST_TYPE type, ID3D12CommandAllocator* pCommandAllocator, ID3D12PipelineState* pInitialState, REFIID riid, void** ppCommandList) = 0;
    virtual HRESULT CreateFence(UINT64 InitialValue, D3D12_FENCE_FLAGS Flags, REFIID riid, void** ppFence) = 0;
    virtual HRESULT CreateDescriptorHeap(const D3D12_DESCRIPTOR_HEAP_DESC* pDescriptorHeapDesc, REFIID riid, void** ppvHeap) = 0;
    virtual UINT GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE DescriptorHeapType) = 0;
    virtual HRESULT CreateRenderTargetView(ID3D12Resource* pResource, const void* pDesc, D3D12_CPU_DESCRIPTOR_HANDLE DestDescriptor) = 0;
    virtual HRESULT CreateCommittedResource(const D3D12_HEAP_PROPERTIES* pHeapProperties, UINT HeapFlags, const D3D12_RESOURCE_DESC* pDesc, D3D12_RESOURCE_STATES InitialResourceState, const D3D12_CLEAR_VALUE* pOptimizedClearValue, REFIID riidResource, void** ppvResource) = 0;
    virtual HRESULT CreateRootSignature(UINT nodeMask, const void* pBlobWithRootSignature, UINT64 blobLengthInBytes, REFIID riid, void** ppvRootSignature) = 0;
    virtual HRESULT CreateGraphicsPipelineState(const void* pDesc, REFIID riid, void** ppPipelineState) = 0;
};

// ============================================================
// D3D12 API functions
// ============================================================
extern "C" {
    HRESULT D3D12CreateDevice(IUnknown* pAdapter, UINT MinimumFeatureLevel, REFIID riid, void** ppDevice);
    HRESULT D3D12GetDebugInterface(REFIID riid, void** ppvDebug);
}

// ============================================================
// DirectXMath types
// ============================================================
namespace DirectX {
    struct XMFLOAT2 {
        float x, y;
        XMFLOAT2() : x(0), y(0) {}
        XMFLOAT2(float _x, float _y) : x(_x), y(_y) {}
    };
    struct XMFLOAT3 {
        float x, y, z;
        XMFLOAT3() : x(0), y(0), z(0) {}
        XMFLOAT3(float _x, float _y, float _z) : x(_x), y(_y), z(_z) {}
    };
    struct XMFLOAT4 {
        float x, y, z, w;
        XMFLOAT4() : x(0), y(0), z(0), w(0) {}
        XMFLOAT4(float _x, float _y, float _z, float _w) : x(_x), y(_y), z(_z), w(_w) {}
    };
    struct XMFLOAT4X4 {
        float m[4][4];
    };
}
using namespace DirectX;
