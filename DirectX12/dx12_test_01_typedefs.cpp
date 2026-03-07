// Test 01: Windows typedefs only — most basic C++ features
typedef unsigned int UINT;
typedef unsigned long DWORD;
typedef unsigned long long UINT64;
typedef unsigned char UINT8;
typedef long LONG;
typedef long HRESULT;
typedef void* HANDLE;
typedef void* HWND;
typedef const char* LPCSTR;

int main() {
    UINT width = 1280;
    UINT height = 720;
    UINT64 fence = 0;
    HRESULT hr = 0;
    float aspect = static_cast<float>(width) / static_cast<float>(height);
    
    UINT8* ptr = reinterpret_cast<UINT8*>(0);
    
    return 0;
}
