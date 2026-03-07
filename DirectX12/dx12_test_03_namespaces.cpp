// Test 03: Namespaces — DirectX uses namespace DirectX, Microsoft::WRL
typedef unsigned int UINT;
typedef long HRESULT;

namespace DirectX {
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
}

using namespace DirectX;

int main() {
    XMFLOAT3 pos;
    pos.x = 0.0f;
    pos.y = 0.25f;
    pos.z = 0.0f;

    XMFLOAT4 col;
    col.x = 1.0f;
    col.y = 0.0f;
    col.z = 0.0f;
    col.w = 1.0f;

    return 0;
}
