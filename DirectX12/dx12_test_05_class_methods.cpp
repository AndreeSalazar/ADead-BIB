// Test 05: Class with inline method bodies returning values
typedef unsigned int UINT;

class MyClass {
public:
    int x;
    
    int GetX() { return x; }
    void SetX(int val) { x = val; }
    UINT GetWidth() const { return m_width; }
    
private:
    UINT m_width;
};

int main() {
    MyClass obj;
    obj.SetX(42);
    int val = obj.GetX();
    return 0;
}
