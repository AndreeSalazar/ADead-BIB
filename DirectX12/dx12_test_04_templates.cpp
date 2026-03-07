// Test 04: Templates — ComPtr<T> pattern from DirectX 12
typedef unsigned int UINT;
typedef long HRESULT;

// Simplified COM interface
struct IUnknown {
    virtual UINT AddRef() = 0;
    virtual UINT Release() = 0;
};

struct ID3D12Device : public IUnknown {};
struct ID3D12CommandQueue : public IUnknown {};

// ComPtr template — the core of DirectX 12 C++ usage
template<typename T>
class ComPtr {
public:
    T* ptr;

    T* Get() { return ptr; }
    void Reset() { ptr = 0; }
};

int main() {
    ComPtr<ID3D12Device> device;
    ComPtr<ID3D12CommandQueue> queue;

    device.Reset();
    queue.Reset();

    return 0;
}
