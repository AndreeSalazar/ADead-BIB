// ============================================================
// ADead-BIB C++ Smart Pointers Intensive Test
// "Respetar Bits" — Type Strictness ULTRA
// ============================================================
// Ejecutar: adb cpp reportes/tests_cpp_intensive/test_cpp_smart_pointers.cpp
// ============================================================

#include <stdio.h>
#include <stdlib.h>

// ============================================================
// Simplified unique_ptr implementation
// ============================================================
template<typename T>
class UniquePtr {
private:
    T* ptr;
    
public:
    explicit UniquePtr(T* p = nullptr) : ptr(p) {
        printf("[UniquePtr] Constructor: %s\n", ptr ? "owns resource" : "null");
    }
    
    ~UniquePtr() {
        if (ptr) {
            printf("[UniquePtr] Destructor: releasing resource\n");
            delete ptr;
        }
    }
    
    // Delete copy constructor and assignment
    UniquePtr(const UniquePtr&) = delete;
    UniquePtr& operator=(const UniquePtr&) = delete;
    
    // Move constructor
    UniquePtr(UniquePtr&& other) : ptr(other.ptr) {
        other.ptr = nullptr;
        printf("[UniquePtr] Move constructor: took ownership\n");
    }
    
    // Move assignment
    UniquePtr& operator=(UniquePtr&& other) {
        if (this != &other) {
            if (ptr) delete ptr;
            ptr = other.ptr;
            other.ptr = nullptr;
            printf("[UniquePtr] Move assignment: took ownership\n");
        }
        return *this;
    }
    
    T& operator*() { return *ptr; }
    T* operator->() { return ptr; }
    T* get() { return ptr; }
    
    explicit operator bool() const { return ptr != nullptr; }
    
    T* release() {
        T* temp = ptr;
        ptr = nullptr;
        printf("[UniquePtr] Released ownership\n");
        return temp;
    }
    
    void reset(T* p = nullptr) {
        if (ptr) delete ptr;
        ptr = p;
        printf("[UniquePtr] Reset to %s\n", ptr ? "new resource" : "null");
    }
};

template<typename T, typename... Args>
UniquePtr<T> make_unique(Args&&... args) {
    return UniquePtr<T>(new T(args...));
}

void test_unique_ptr() {
    printf("\n=== TEST 1: UniquePtr (std::unique_ptr simulation) ===\n");
    
    // Create unique_ptr
    UniquePtr<int> p1(new int(42));
    printf("  *p1 = %d\n", *p1);
    
    // Move ownership
    UniquePtr<int> p2 = static_cast<UniquePtr<int>&&>(p1);
    printf("  After move, p1 is %s\n", p1 ? "valid" : "null");
    printf("  *p2 = %d\n", *p2);
    
    // Reset
    p2.reset(new int(100));
    printf("  After reset, *p2 = %d\n", *p2);
    
    // Release
    int* raw = p2.release();
    printf("  Released raw pointer: %d\n", *raw);
    delete raw;
    
    // make_unique simulation
    auto p3 = make_unique<int>(999);
    printf("  make_unique<int>(999): *p3 = %d\n", *p3);
}

// ============================================================
// Simplified shared_ptr implementation
// ============================================================
template<typename T>
class SharedPtr {
private:
    T* ptr;
    int* ref_count;
    
    void release() {
        if (ref_count) {
            (*ref_count)--;
            printf("[SharedPtr] Ref count decreased to %d\n", *ref_count);
            if (*ref_count == 0) {
                printf("[SharedPtr] Ref count is 0, deleting resource\n");
                delete ptr;
                delete ref_count;
            }
        }
    }
    
public:
    explicit SharedPtr(T* p = nullptr) : ptr(p), ref_count(nullptr) {
        if (ptr) {
            ref_count = new int(1);
            printf("[SharedPtr] Constructor: ref_count = 1\n");
        }
    }
    
    ~SharedPtr() {
        release();
    }
    
    // Copy constructor
    SharedPtr(const SharedPtr& other) : ptr(other.ptr), ref_count(other.ref_count) {
        if (ref_count) {
            (*ref_count)++;
            printf("[SharedPtr] Copy constructor: ref_count = %d\n", *ref_count);
        }
    }
    
    // Copy assignment
    SharedPtr& operator=(const SharedPtr& other) {
        if (this != &other) {
            release();
            ptr = other.ptr;
            ref_count = other.ref_count;
            if (ref_count) {
                (*ref_count)++;
                printf("[SharedPtr] Copy assignment: ref_count = %d\n", *ref_count);
            }
        }
        return *this;
    }
    
    // Move constructor
    SharedPtr(SharedPtr&& other) : ptr(other.ptr), ref_count(other.ref_count) {
        other.ptr = nullptr;
        other.ref_count = nullptr;
        printf("[SharedPtr] Move constructor\n");
    }
    
    T& operator*() { return *ptr; }
    T* operator->() { return ptr; }
    T* get() { return ptr; }
    
    int use_count() const { return ref_count ? *ref_count : 0; }
    bool unique() const { return use_count() == 1; }
    explicit operator bool() const { return ptr != nullptr; }
    
    void reset(T* p = nullptr) {
        release();
        ptr = p;
        if (ptr) {
            ref_count = new int(1);
        } else {
            ref_count = nullptr;
        }
    }
};

template<typename T, typename... Args>
SharedPtr<T> make_shared(Args&&... args) {
    return SharedPtr<T>(new T(args...));
}

void test_shared_ptr() {
    printf("\n=== TEST 2: SharedPtr (std::shared_ptr simulation) ===\n");
    
    // Create shared_ptr
    SharedPtr<int> p1(new int(42));
    printf("  *p1 = %d, use_count = %d\n", *p1, p1.use_count());
    
    // Copy (share ownership)
    SharedPtr<int> p2 = p1;
    printf("  After copy, p1.use_count = %d, p2.use_count = %d\n", 
           p1.use_count(), p2.use_count());
    
    // Another copy
    SharedPtr<int> p3 = p2;
    printf("  After another copy, use_count = %d\n", p1.use_count());
    
    // Modify through any pointer
    *p2 = 100;
    printf("  After *p2 = 100: *p1 = %d, *p3 = %d\n", *p1, *p3);
    
    // Scope test
    {
        SharedPtr<int> p4 = p1;
        printf("  Inside scope, use_count = %d\n", p1.use_count());
    }
    printf("  After scope exit, use_count = %d\n", p1.use_count());
}

// ============================================================
// Simplified weak_ptr implementation
// ============================================================
template<typename T>
class WeakPtr {
private:
    T* ptr;
    int* ref_count;
    
public:
    WeakPtr() : ptr(nullptr), ref_count(nullptr) {}
    
    WeakPtr(const SharedPtr<T>& shared) : ptr(shared.get()), ref_count(nullptr) {
        // In real implementation, would share control block
        printf("[WeakPtr] Created from SharedPtr\n");
    }
    
    bool expired() const {
        return ptr == nullptr || (ref_count && *ref_count == 0);
    }
    
    SharedPtr<T> lock() {
        if (!expired()) {
            return SharedPtr<T>(ptr);  // Simplified
        }
        return SharedPtr<T>();
    }
};

void test_weak_ptr() {
    printf("\n=== TEST 3: WeakPtr (std::weak_ptr simulation) ===\n");
    
    SharedPtr<int> shared(new int(42));
    WeakPtr<int> weak(shared);
    
    printf("  shared.use_count = %d\n", shared.use_count());
    printf("  weak.expired = %s\n", weak.expired() ? "true" : "false");
    
    // Lock to get shared_ptr
    if (auto locked = weak.lock()) {
        printf("  Locked successfully: *locked = %d\n", *locked);
    }
}

// ============================================================
// Custom deleter example
// ============================================================
template<typename T>
struct ArrayDeleter {
    void operator()(T* ptr) {
        printf("[ArrayDeleter] Deleting array\n");
        delete[] ptr;
    }
};

template<typename T, typename Deleter = void>
class UniquePtrWithDeleter {
private:
    T* ptr;
    Deleter deleter;
    
public:
    explicit UniquePtrWithDeleter(T* p = nullptr) : ptr(p) {
        printf("[UniquePtrWithDeleter] Constructor\n");
    }
    
    ~UniquePtrWithDeleter() {
        if (ptr) {
            deleter(ptr);
        }
    }
    
    T& operator[](int index) { return ptr[index]; }
    T* get() { return ptr; }
};

void test_custom_deleter() {
    printf("\n=== TEST 4: Custom Deleter ===\n");
    
    // Array with custom deleter
    UniquePtrWithDeleter<int, ArrayDeleter<int>> arr(new int[5]);
    
    for (int i = 0; i < 5; i++) {
        arr[i] = i * 10;
    }
    
    printf("  Array contents: ");
    for (int i = 0; i < 5; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");
}

// ============================================================
// RAII Pattern demonstration
// ============================================================
class FileHandle {
private:
    const char* filename;
    bool is_open;
    
public:
    FileHandle(const char* name) : filename(name), is_open(true) {
        printf("[FileHandle] Opened file: %s\n", filename);
    }
    
    ~FileHandle() {
        if (is_open) {
            printf("[FileHandle] Closed file: %s\n", filename);
        }
    }
    
    void write(const char* data) {
        printf("  Writing to %s: \"%s\"\n", filename, data);
    }
    
    void read() {
        printf("  Reading from %s\n", filename);
    }
};

class MutexLock {
private:
    const char* name;
    
public:
    MutexLock(const char* n) : name(n) {
        printf("[MutexLock] Acquired lock: %s\n", name);
    }
    
    ~MutexLock() {
        printf("[MutexLock] Released lock: %s\n", name);
    }
};

void test_raii() {
    printf("\n=== TEST 5: RAII Pattern ===\n");
    
    {
        printf("  Entering scope...\n");
        FileHandle file("data.txt");
        file.write("Hello, World!");
        file.read();
        
        MutexLock lock("database_mutex");
        printf("  Critical section...\n");
        
        printf("  Leaving scope...\n");
    }
    printf("  After scope (resources released)\n");
}

// ============================================================
// Ownership transfer patterns
// ============================================================
class Resource {
public:
    int value;
    Resource(int v) : value(v) {
        printf("[Resource] Created with value %d\n", value);
    }
    ~Resource() {
        printf("[Resource] Destroyed with value %d\n", value);
    }
};

UniquePtr<Resource> create_resource(int val) {
    return UniquePtr<Resource>(new Resource(val));
}

void consume_resource(UniquePtr<Resource> res) {
    printf("  Consuming resource with value: %d\n", res->value);
    // Resource automatically deleted when function returns
}

void test_ownership_transfer() {
    printf("\n=== TEST 6: Ownership Transfer ===\n");
    
    // Factory function returns ownership
    auto res1 = create_resource(100);
    printf("  Created resource: %d\n", res1->value);
    
    // Transfer ownership to function
    auto res2 = create_resource(200);
    consume_resource(static_cast<UniquePtr<Resource>&&>(res2));
    printf("  After consume, res2 is %s\n", res2 ? "valid" : "null");
}

// ============================================================
// Main — Run all tests
// ============================================================
int main() {
    printf("============================================================\n");
    printf("ADead-BIB C++ Smart Pointers Intensive Test\n");
    printf("============================================================\n");
    
    test_unique_ptr();
    test_shared_ptr();
    test_weak_ptr();
    test_custom_deleter();
    test_raii();
    test_ownership_transfer();
    
    printf("\n============================================================\n");
    printf("All Smart Pointer tests completed!\n");
    printf("==============================================================\n");
    
    return 0;
}
