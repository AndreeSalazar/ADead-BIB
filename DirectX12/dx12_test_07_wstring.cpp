// Test 07: Wide string literals L"" — DirectX 12 uses wchar_t extensively
typedef unsigned int UINT;

class wstring {
public:
    wstring() {}
    const wchar_t* c_str() const { return L""; }
};

int main() {
    wstring s;
    return 0;
}
