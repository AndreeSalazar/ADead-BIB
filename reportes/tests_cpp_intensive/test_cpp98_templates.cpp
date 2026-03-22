// ============================================================
// ADead-BIB C++98 Intensive Test — Templates
// "Respetar Bits" — Type Strictness ULTRA
// ============================================================
// Ejecutar: adb cpp reportes/tests_cpp_intensive/test_cpp98_templates.cpp
// ============================================================

#include <stdio.h>

// ============================================================
// Test 1: Function Templates
// ============================================================
template<typename T>
T max_value(T a, T b) {
    printf("  max_value<%s>(%d, %d)\n", "T", (int)a, (int)b);
    return (a > b) ? a : b;
}

template<typename T>
T min_value(T a, T b) {
    return (a < b) ? a : b;
}

template<typename T>
void swap_values(T& a, T& b) {
    T temp = a;
    a = b;
    b = temp;
}

void test_function_templates() {
    printf("\n=== TEST 1: Function Templates ===\n");
    
    int i1 = 10, i2 = 20;
    printf("  max(int): %d\n", max_value(i1, i2));
    printf("  min(int): %d\n", min_value(i1, i2));
    
    float f1 = 3.14f, f2 = 2.71f;
    printf("  max(float): %.2f\n", max_value(f1, f2));
    printf("  min(float): %.2f\n", min_value(f1, f2));
    
    printf("  Before swap: i1=%d, i2=%d\n", i1, i2);
    swap_values(i1, i2);
    printf("  After swap: i1=%d, i2=%d\n", i1, i2);
}

// ============================================================
// Test 2: Class Templates
// ============================================================
template<typename T>
class Box {
private:
    T value;
public:
    Box(T v) : value(v) {
        printf("[Box<%s>] Constructor\n", "T");
    }
    
    T get() { return value; }
    void set(T v) { value = v; }
    
    void print() {
        printf("  Box contains: %d\n", (int)value);
    }
};

template<typename T, typename U>
class Pair {
public:
    T first;
    U second;
    
    Pair(T f, U s) : first(f), second(s) {
        printf("[Pair<%s,%s>] Constructor\n", "T", "U");
    }
    
    void print() {
        printf("  Pair: (%d, %d)\n", (int)first, (int)second);
    }
};

void test_class_templates() {
    printf("\n=== TEST 2: Class Templates ===\n");
    
    Box<int> intBox(42);
    intBox.print();
    
    Box<float> floatBox(3.14f);
    floatBox.print();
    
    Pair<int, float> p1(10, 20.5f);
    p1.print();
    
    Pair<int, int> p2(100, 200);
    p2.print();
}

// ============================================================
// Test 3: Template Specialization
// ============================================================
template<typename T>
class TypeInfo {
public:
    static void print() {
        printf("  TypeInfo: Unknown type\n");
    }
};

template<>
class TypeInfo<int> {
public:
    static void print() {
        printf("  TypeInfo<int>: 32-bit signed integer\n");
    }
};

template<>
class TypeInfo<float> {
public:
    static void print() {
        printf("  TypeInfo<float>: 32-bit IEEE 754 floating point\n");
    }
};

template<>
class TypeInfo<double> {
public:
    static void print() {
        printf("  TypeInfo<double>: 64-bit IEEE 754 floating point\n");
    }
};

void test_template_specialization() {
    printf("\n=== TEST 3: Template Specialization ===\n");
    
    TypeInfo<int>::print();
    TypeInfo<float>::print();
    TypeInfo<double>::print();
    TypeInfo<char>::print();  // Uses generic version
}

// ============================================================
// Test 4: Non-Type Template Parameters
// ============================================================
template<typename T, int SIZE>
class StaticArray {
private:
    T data[SIZE];
    int count;
    
public:
    StaticArray() : count(0) {
        printf("[StaticArray<%s, %d>] Constructor\n", "T", SIZE);
        for (int i = 0; i < SIZE; i++) {
            data[i] = T();
        }
    }
    
    bool push(T value) {
        if (count < SIZE) {
            data[count] = value;
            count = count + 1;
            return true;
        }
        return false;
    }
    
    T get(int index) {
        if (index >= 0 && index < count) {
            return data[index];
        }
        return T();
    }
    
    int size() { return count; }
    int capacity() { return SIZE; }
    
    void print() {
        printf("  StaticArray[%d/%d]: ", count, SIZE);
        for (int i = 0; i < count; i++) {
            printf("%d ", (int)data[i]);
        }
        printf("\n");
    }
};

void test_nontype_template_params() {
    printf("\n=== TEST 4: Non-Type Template Parameters ===\n");
    
    StaticArray<int, 5> arr5;
    arr5.push(10);
    arr5.push(20);
    arr5.push(30);
    arr5.print();
    
    StaticArray<int, 10> arr10;
    for (int i = 0; i < 7; i++) {
        arr10.push(i * 10);
    }
    arr10.print();
}

// ============================================================
// Test 5: Template with Multiple Parameters
// ============================================================
template<typename K, typename V>
class KeyValue {
public:
    K key;
    V value;
    
    KeyValue(K k, V v) : key(k), value(v) {}
    
    void print() {
        printf("  KeyValue: key=%d, value=%d\n", (int)key, (int)value);
    }
};

template<typename T>
class Stack {
private:
    T items[100];
    int top;
    
public:
    Stack() : top(-1) {
        printf("[Stack<%s>] Constructor\n", "T");
    }
    
    void push(T item) {
        if (top < 99) {
            top = top + 1;
            items[top] = item;
            printf("  Pushed: %d (top=%d)\n", (int)item, top);
        }
    }
    
    T pop() {
        if (top >= 0) {
            T item = items[top];
            top = top - 1;
            printf("  Popped: %d (top=%d)\n", (int)item, top);
            return item;
        }
        return T();
    }
    
    bool isEmpty() {
        return top < 0;
    }
    
    int size() {
        return top + 1;
    }
};

void test_template_containers() {
    printf("\n=== TEST 5: Template Containers ===\n");
    
    KeyValue<int, int> kv1(1, 100);
    kv1.print();
    
    KeyValue<int, float> kv2(2, 3.14f);
    kv2.print();
    
    Stack<int> stack;
    stack.push(10);
    stack.push(20);
    stack.push(30);
    printf("  Stack size: %d\n", stack.size());
    stack.pop();
    stack.pop();
    printf("  Stack size after pops: %d\n", stack.size());
}

// ============================================================
// Test 6: Template Inheritance
// ============================================================
template<typename T>
class Base {
protected:
    T data;
public:
    Base(T d) : data(d) {
        printf("[Base<%s>] Constructor with data=%d\n", "T", (int)d);
    }
    
    virtual void display() {
        printf("  Base data: %d\n", (int)data);
    }
};

template<typename T>
class Derived : public Base<T> {
private:
    T extra;
public:
    Derived(T d, T e) : Base<T>(d), extra(e) {
        printf("[Derived<%s>] Constructor with extra=%d\n", "T", (int)e);
    }
    
    void display() {
        printf("  Derived data: %d, extra: %d\n", (int)this->data, (int)extra);
    }
};

void test_template_inheritance() {
    printf("\n=== TEST 6: Template Inheritance ===\n");
    
    Base<int> b(100);
    b.display();
    
    Derived<int> d(200, 300);
    d.display();
    
    Base<int>* ptr = &d;
    ptr->display();  // Polymorphic call
}

// ============================================================
// Main — Run all tests
// ============================================================
int main() {
    printf("============================================================\n");
    printf("ADead-BIB C++98 Intensive Test — Templates\n");
    printf("============================================================\n");
    
    test_function_templates();
    test_class_templates();
    test_template_specialization();
    test_nontype_template_params();
    test_template_containers();
    test_template_inheritance();
    
    printf("\n============================================================\n");
    printf("All C++98 Template tests completed!\n");
    printf("==============================================================\n");
    
    return 0;
}
