// Test 09: using declarations with scope resolution — DirectX 12 pattern
typedef unsigned int UINT;

namespace Microsoft {
    namespace WRL {
        template<typename T>
        class ComPtr {
        public:
            T* ptr;
            T* Get() { return ptr; }
        };
    }
}

// This is the pattern that fails — using with scoped name
using Microsoft::WRL::ComPtr;

struct ID3D12Device {};

int main() {
    ComPtr<ID3D12Device> device;
    return 0;
}
