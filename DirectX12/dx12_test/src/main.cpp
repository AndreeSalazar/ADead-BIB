// DirectX 12 HelloTriangle — ADead-BIB Test Project
// Compiled with: adb step src/main.cpp or adb run
#include <header_main.h>

// ============================================================
// Vertex structure
// ============================================================
struct Vertex {
    XMFLOAT3 position;
    XMFLOAT4 color;
};

// ============================================================
// DXSample base class
// ============================================================
class DXSample {
public:
    UINT m_width;
    UINT m_height;
    float m_aspectRatio;
    LPCWSTR m_title;

    DXSample(UINT width, UINT height, LPCWSTR title) :
        m_width(width),
        m_height(height),
        m_title(title)
    {
        m_aspectRatio = static_cast<float>(width) / static_cast<float>(height);
    }

    virtual ~DXSample() {}
    virtual void OnInit() = 0;
    virtual void OnUpdate() = 0;
    virtual void OnRender() = 0;
    virtual void OnDestroy() = 0;

    UINT GetWidth() const { return m_width; }
    UINT GetHeight() const { return m_height; }
};

// ============================================================
// D3D12HelloTriangle class
// ============================================================
class D3D12HelloTriangle : public DXSample {
public:
    D3D12HelloTriangle(UINT width, UINT height, LPCWSTR title);

    virtual void OnInit();
    virtual void OnUpdate();
    virtual void OnRender();
    virtual void OnDestroy();

private:
    static const UINT FrameCount = 2;

    // Pipeline objects
    D3D12_VIEWPORT m_viewport;
    D3D12_RECT m_scissorRect;
    ComPtr<IDXGISwapChain3> m_swapChain;
    ComPtr<ID3D12Device> m_device;
    ComPtr<ID3D12Resource> m_renderTargets[2];
    ComPtr<ID3D12CommandAllocator> m_commandAllocator;
    ComPtr<ID3D12CommandQueue> m_commandQueue;
    ComPtr<ID3D12DescriptorHeap> m_rtvHeap;
    ComPtr<ID3D12PipelineState> m_pipelineState;
    ComPtr<ID3D12GraphicsCommandList> m_commandList;
    ComPtr<ID3D12RootSignature> m_rootSignature;
    UINT m_rtvDescriptorSize;

    // App resources
    ComPtr<ID3D12Resource> m_vertexBuffer;
    D3D12_VERTEX_BUFFER_VIEW m_vertexBufferView;

    // Synchronization
    UINT m_frameIndex;
    HANDLE m_fenceEvent;
    ComPtr<ID3D12Fence> m_fence;
    UINT64 m_fenceValue;

    void LoadPipeline();
    void LoadAssets();
    void WaitForPreviousFrame();
};

// ============================================================
// Implementation
// ============================================================
D3D12HelloTriangle::D3D12HelloTriangle(UINT width, UINT height, LPCWSTR title) :
    DXSample(width, height, title),
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

void D3D12HelloTriangle::LoadPipeline() {
    D3D12_COMMAND_QUEUE_DESC queueDesc;
    queueDesc.Type = D3D12_COMMAND_LIST_TYPE_DIRECT;
    queueDesc.Flags = D3D12_COMMAND_QUEUE_FLAG_NONE;
    queueDesc.NodeMask = 0;
    queueDesc.Priority = 0;

    m_viewport.TopLeftX = 0.0f;
    m_viewport.TopLeftY = 0.0f;
    m_viewport.Width = static_cast<float>(m_width);
    m_viewport.Height = static_cast<float>(m_height);
    m_viewport.MinDepth = 0.0f;
    m_viewport.MaxDepth = 1.0f;

    m_scissorRect.left = 0;
    m_scissorRect.top = 0;
    m_scissorRect.right = static_cast<LONG>(m_width);
    m_scissorRect.bottom = static_cast<LONG>(m_height);
}

void D3D12HelloTriangle::LoadAssets() {
    Vertex triangleVertices[] = {
        { XMFLOAT3(0.0f, 0.25f, 0.0f), XMFLOAT4(1.0f, 0.0f, 0.0f, 1.0f) },
        { XMFLOAT3(0.25f, -0.25f, 0.0f), XMFLOAT4(0.0f, 1.0f, 0.0f, 1.0f) },
        { XMFLOAT3(-0.25f, -0.25f, 0.0f), XMFLOAT4(0.0f, 0.0f, 1.0f, 1.0f) }
    };

    const UINT vertexBufferSize = sizeof(triangleVertices);

    D3D12_INPUT_ELEMENT_DESC inputElementDescs[] = {
        { "POSITION", 0, DXGI_FORMAT_R32G32B32_FLOAT, 0, 0, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, 0 },
        { "COLOR", 0, DXGI_FORMAT_R32G32B32A32_FLOAT, 0, 12, D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA, 0 }
    };
}

void D3D12HelloTriangle::WaitForPreviousFrame() {
    m_fenceValue = m_fenceValue + 1;
}

void D3D12HelloTriangle::OnUpdate() {}

void D3D12HelloTriangle::OnRender() {
    WaitForPreviousFrame();
}

void D3D12HelloTriangle::OnDestroy() {
    WaitForPreviousFrame();
    m_swapChain.Reset();
    m_device.Reset();
    m_commandAllocator.Reset();
    m_commandQueue.Reset();
}

// ============================================================
// Entry point
// ============================================================
int main() {
    D3D12HelloTriangle sample(1280, 720, L"D3D12 Hello Triangle - ADead-BIB");

    printf("ADead-BIB DirectX 12 HelloTriangle\n");
    printf("Window: %dx%d\n", sample.GetWidth(), sample.GetHeight());

    sample.OnInit();
    sample.OnUpdate();
    sample.OnRender();
    sample.OnDestroy();

    printf("DirectX 12 pipeline complete.\n");
    return 0;
}
