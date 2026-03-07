// Test 10: operator overloading in class — DirectX wstring pattern
typedef unsigned int UINT;

class wstring {
public:
    wstring() {}
    wstring(const wchar_t* s) {}
    const wchar_t* c_str() const { return L""; }
    wstring operator+(const wchar_t* s) const { return wstring(); }
    wstring operator+(const wstring& s) const { return wstring(); }
};

int main() {
    wstring s;
    return 0;
}
