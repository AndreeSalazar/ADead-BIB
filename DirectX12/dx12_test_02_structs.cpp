// Test 02: Structs + classes + inheritance — DirectX 12 patterns
typedef unsigned int UINT;
typedef unsigned long long UINT64;
typedef unsigned char UINT8;
typedef long HRESULT;

// DirectX Math types
struct XMFLOAT3 {
    float x;
    float y;
    float z;
};

struct XMFLOAT4 {
    float x;
    float y;
    float z;
    float w;
};

// Vertex — nested struct types
struct Vertex {
    XMFLOAT3 position;
    XMFLOAT4 color;
};

// D3D12 descriptor structs
struct D3D12_COMMAND_QUEUE_DESC {
    UINT Flags;
    UINT Type;
};

struct D3D12_VERTEX_BUFFER_VIEW {
    UINT64 BufferLocation;
    UINT SizeInBytes;
    UINT StrideInBytes;
};

// Base class with virtual methods
class DXSample {
public:
    UINT m_width;
    UINT m_height;
    float m_aspectRatio;

    virtual void OnInit() = 0;
    virtual void OnUpdate() = 0;
    virtual void OnRender() = 0;
    virtual void OnDestroy() = 0;
};

// Derived class
class HelloTriangle : public DXSample {
public:
    virtual void OnInit() {}
    virtual void OnUpdate() {}
    virtual void OnRender() {}
    virtual void OnDestroy() {}

private:
    UINT m_frameIndex;
    UINT64 m_fenceValue;
    D3D12_VERTEX_BUFFER_VIEW m_vertexBufferView;
};

int main() {
    Vertex v;
    v.position.x = 0.0f;
    v.position.y = 0.25f;
    v.position.z = 0.0f;
    v.color.x = 1.0f;
    v.color.y = 0.0f;
    v.color.z = 0.0f;
    v.color.w = 1.0f;

    D3D12_COMMAND_QUEUE_DESC queueDesc;
    queueDesc.Flags = 0;
    queueDesc.Type = 0;

    return 0;
}
