// DirectX 12 HelloTriangle — Feature Analysis for ADead-BIB
// This file contains ALL C++ features used by DirectX 12 HelloTriangle sample
// Simplified to test what ADead-BIB's C++ pipeline can handle

// ============================================================
// 1. Windows typedefs (normally from <windows.h>)
// ============================================================
typedef unsigned int UINT;
typedef unsigned long DWORD;
typedef unsigned long long UINT64;
typedef unsigned char UINT8;
typedef unsigned char byte;
typedef long LONG;
typedef long HRESULT;
typedef void* HANDLE;
typedef void* HWND;
typedef void* HINSTANCE;
typedef char* LPSTR;
typedef const wchar_t* LPCWSTR;
typedef unsigned long long LONG_PTR;

#define NULL 0
#define FALSE 0
#define TRUE 1
#define UINT_MAX 0xFFFFFFFF
#define FAILED(hr) ((hr) < 0)
#define SUCCEEDED(hr) ((hr) >= 0)
#define INFINITE 0xFFFFFFFF

// ============================================================
// 2. std::wstring (simplified)
// ============================================================
namespace std {
    class wstring {
    public:
        wstring() {}
        wstring(const wchar_t* s) {}
        const wchar_t* c_str() const { return L""; }
        wstring operator+(const wchar_t* s) const { return wstring(); }
        wstring operator+(const wstring& s) const { return wstring(); }
    };

    class string {
    public:
        string() {}
        string(const char* s) {}
        const char* c_str() const { return ""; }
    };

    class runtime_error {
    public:
        runtime_error(const string& s) {}
    };

    class exception {
    public:
        exception() {}
    };
}

// ============================================================
// 3. COM smart pointer template (ComPtr<T>) — C++11 templates
// ============================================================
namespace Microsoft {
    namespace WRL {
        template<typename T>
        class ComPtr {
        public:
            ComPtr() : ptr(NULL) {}
            ~ComPtr() { if (ptr) ptr->Release(); }
            T* Get() const { return ptr; }
            T** GetAddressOf() { return &ptr; }
            T* operator->() const { return ptr; }
            T** operator&() { return &ptr; }
            void Reset() { if (ptr) { ptr->Release(); ptr = NULL; } }
            T* Detach() { T* tmp = ptr; ptr = NULL; return tmp; }

            // QueryInterface pattern
            template<typename U>
            HRESULT As(ComPtr<U>* other) { return 0; }

        private:
            T* ptr;
        };
    }
}

using Microsoft::WRL::ComPtr;

// ============================================================
// 4. DirectX Math types (XMFLOAT3, XMFLOAT4)
// ============================================================
namespace DirectX {
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
}
using namespace DirectX;

// ============================================================
// 5. D3D12 interface stubs (COM interfaces with virtual methods)
// ============================================================
struct IUnknown {
    virtual UINT AddRef() = 0;
    virtual UINT Release() = 0;
    virtual HRESULT QueryInterface(void* riid, void** ppvObject) = 0;
};

struct ID3D12Object : public IUnknown {
    virtual HRESULT SetName(LPCWSTR name) = 0;
};

struct ID3D12Device : public ID3D12Object {
    // Stubs for key methods
};

struct ID3D12CommandQueue : public ID3D12Object {};
struct ID3D12CommandAllocator : public ID3D12Object {};
struct ID3D12RootSignature : public ID3D12Object {};
struct ID3D12PipelineState : public ID3D12Object {};
struct ID3D12DescriptorHeap : public ID3D12Object {};
struct ID3D12Resource : public ID3D12Object {};
struct ID3D12Fence : public ID3D12Object {};
struct ID3D12GraphicsCommandList : public ID3D12Object {};
struct ID3DBlob : public IUnknown {};
struct IDXGISwapChain3 : public IUnknown {};
struct IDXGIFactory4 : public IUnknown {};

// ============================================================
// 6. D3D12 structs (aggregate initialization)
// ============================================================
struct D3D12_COMMAND_QUEUE_DESC {
    UINT Flags;
    UINT Type;
};

struct D3D12_VERTEX_BUFFER_VIEW {
    UINT64 BufferLocation;
    UINT SizeInBytes;
    UINT StrideInBytes;
};

struct D3D12_INPUT_ELEMENT_DESC {
    const char* SemanticName;
    UINT SemanticIndex;
    UINT Format;
    UINT InputSlot;
    UINT AlignedByteOffset;
    UINT InputSlotClass;
    UINT InstanceDataStepRate;
};

// ============================================================
// 7. Vertex struct with nested DirectX types
// ============================================================
struct Vertex {
    XMFLOAT3 position;
    XMFLOAT4 color;
};

// ============================================================
// 8. Base class with pure virtual methods (DXSample pattern)
// ============================================================
class DXSample {
public:
    DXSample(UINT width, UINT height, std::wstring name)
        : m_width(width), m_height(height), m_title(name), m_useWarpDevice(false) {
        m_aspectRatio = static_cast<float>(width) / static_cast<float>(height);
    }
    virtual ~DXSample() {}

    virtual void OnInit() = 0;
    virtual void OnUpdate() = 0;
    virtual void OnRender() = 0;
    virtual void OnDestroy() = 0;

    virtual void OnKeyDown(UINT8 key) {}
    virtual void OnKeyUp(UINT8 key) {}

    UINT GetWidth() const { return m_width; }
    UINT GetHeight() const { return m_height; }
    const wchar_t* GetTitle() const { return m_title.c_str(); }

protected:
    UINT m_width;
    UINT m_height;
    float m_aspectRatio;
    bool m_useWarpDevice;

private:
    std::wstring m_title;
};

// ============================================================
// 9. HelloTriangle class: inheritance, ComPtr<T>, virtual override
// ============================================================
class D3D12HelloTriangle : public DXSample {
public:
    D3D12HelloTriangle(UINT width, UINT height, std::wstring name);

    virtual void OnInit();
    virtual void OnUpdate();
    virtual void OnRender();
    virtual void OnDestroy();

private:
    static const UINT FrameCount = 2;

    // ComPtr templates with D3D12 interfaces
    ComPtr<IDXGISwapChain3> m_swapChain;
    ComPtr<ID3D12Device> m_device;
    ComPtr<ID3D12Resource> m_renderTargets[FrameCount];
    ComPtr<ID3D12CommandAllocator> m_commandAllocator;
    ComPtr<ID3D12CommandQueue> m_commandQueue;
    ComPtr<ID3D12RootSignature> m_rootSignature;
    ComPtr<ID3D12DescriptorHeap> m_rtvHeap;
    ComPtr<ID3D12PipelineState> m_pipelineState;
    ComPtr<ID3D12GraphicsCommandList> m_commandList;
    UINT m_rtvDescriptorSize;

    ComPtr<ID3D12Resource> m_vertexBuffer;
    D3D12_VERTEX_BUFFER_VIEW m_vertexBufferView;

    UINT m_frameIndex;
    HANDLE m_fenceEvent;
    ComPtr<ID3D12Fence> m_fence;
    UINT64 m_fenceValue;

    void LoadPipeline();
    void LoadAssets();
};

// ============================================================
// 10. Member initializer list (C++11 style)
// ============================================================
D3D12HelloTriangle::D3D12HelloTriangle(UINT width, UINT height, std::wstring name) :
    DXSample(width, height, name),
    m_frameIndex(0),
    m_rtvDescriptorSize(0),
    m_fenceValue(0),
    m_fenceEvent(NULL)
{
}

void D3D12HelloTriangle::OnInit() {
    LoadPipeline();
    LoadAssets();
}

void D3D12HelloTriangle::OnUpdate() {}
void D3D12HelloTriangle::OnRender() {}
void D3D12HelloTriangle::OnDestroy() {}

void D3D12HelloTriangle::LoadPipeline() {
    // Aggregate initialization (C++11)
    D3D12_COMMAND_QUEUE_DESC queueDesc = {};

    // static_cast (C++ feature)
    float w = static_cast<float>(m_width);
    float h = static_cast<float>(m_height);

    // reinterpret_cast (C++ feature)
    void* ptr = NULL;
    UINT8* bytePtr = reinterpret_cast<UINT8*>(ptr);
}

void D3D12HelloTriangle::LoadAssets() {
    // Vertex array initialization with nested struct init
    Vertex triangleVertices[] = {
        { { 0.0f, 0.25f, 0.0f }, { 1.0f, 0.0f, 0.0f, 1.0f } },
        { { 0.25f, -0.25f, 0.0f }, { 0.0f, 1.0f, 0.0f, 1.0f } },
        { { -0.25f, -0.25f, 0.0f }, { 0.0f, 0.0f, 1.0f, 1.0f } }
    };

    // sizeof
    const UINT vertexBufferSize = sizeof(triangleVertices);

    // D3D12 Input Element Desc array
    D3D12_INPUT_ELEMENT_DESC inputElementDescs[] = {
        { "POSITION", 0, 0, 0, 0, 0, 0 },
        { "COLOR", 0, 0, 0, 12, 0, 0 }
    };
}

// ============================================================
// 11. Exception class inheritance (std::runtime_error)
// ============================================================
class HrException : public std::runtime_error {
public:
    HrException(HRESULT hr) : std::runtime_error(std::string("HRESULT error")), m_hr(hr) {}
    HRESULT Error() const { return m_hr; }
private:
    const HRESULT m_hr;
};

// ============================================================
// 12. Inline helper functions
// ============================================================
inline void ThrowIfFailed(HRESULT hr) {
    if (FAILED(hr)) {
        throw HrException(hr);
    }
}

// ============================================================
// 13. Template functions (range-for, auto)
// ============================================================
template<typename T>
void ResetComPtrArray(T* comPtrArray) {
    for (auto& i : *comPtrArray) {
        i.Reset();
    }
}

// ============================================================
// 14. WinMain entry point
// ============================================================
int main() {
    D3D12HelloTriangle sample(1280, 720, std::wstring(L"D3D12 Hello Triangle"));

    // Test ComPtr
    ComPtr<ID3D12Device> device;

    // Test vertex
    Vertex v;
    v.position = XMFLOAT3(0.0f, 0.0f, 0.0f);
    v.color = XMFLOAT4(1.0f, 0.0f, 0.0f, 1.0f);

    // Test macros
    HRESULT hr = 0;
    if (SUCCEEDED(hr)) {
        // ok
    }

    return 0;
}
