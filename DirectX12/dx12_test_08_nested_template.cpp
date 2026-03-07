// Test 08: Nested template member function — ComPtr::As<U>() pattern
typedef unsigned int UINT;
typedef long HRESULT;

template<typename T>
class ComPtr {
public:
    T* ptr;

    T* Get() { return ptr; }
    void Reset() { ptr = 0; }

    // This nested template member function is what DirectX12 uses
    template<typename U>
    HRESULT As(ComPtr<U>* other) { return 0; }
};

struct ID3D12Device {};

int main() {
    ComPtr<ID3D12Device> device;
    device.Reset();
    return 0;
}
