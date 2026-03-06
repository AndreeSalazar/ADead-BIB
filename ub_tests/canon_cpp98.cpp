// C++98 Canon Test

namespace MySpace {
    // Inheritance and Virtual Functions
    class Animal {
    public:
        virtual int speak() { return 0; }
    };

    class Dog : public Animal {
    public:
        int speak() { return 1; }
    };
    
    // Templates
    template<typename T>
    T max_val(T a, T b) {
        return (a > b) ? a : b;
    }
}

int main() {
    int result = 0;

    // Exception Handling (C++98)
    try {
        MySpace::Dog d;
        int s = d.speak();
        
        // Template instantiation
        int m = MySpace::max_val(10, 20);
        
        if (s == 0) {
            throw -1;
        }
        
        result = m + s;
    } catch (int e) {
        return e;
    }
    
    return result;
}
