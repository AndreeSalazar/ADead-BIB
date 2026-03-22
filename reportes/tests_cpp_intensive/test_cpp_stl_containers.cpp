// ============================================================
// ADead-BIB C++ STL Containers Intensive Test
// "Respetar Bits" — Type Strictness ULTRA
// ============================================================
// Ejecutar: adb cpp reportes/tests_cpp_intensive/test_cpp_stl_containers.cpp
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// ============================================================
// Simplified Vector implementation (simulating std::vector)
// ============================================================
template<typename T>
class Vector {
private:
    T* data;
    int size_;
    int capacity_;
    
public:
    Vector() : data(nullptr), size_(0), capacity_(0) {
        printf("[Vector] Default constructor\n");
    }
    
    Vector(int initial_capacity) : size_(0), capacity_(initial_capacity) {
        data = (T*)malloc(sizeof(T) * capacity_);
        printf("[Vector] Constructor with capacity %d\n", capacity_);
    }
    
    ~Vector() {
        if (data) {
            printf("[Vector] Destructor, freeing %d elements\n", size_);
            free(data);
        }
    }
    
    void push_back(const T& value) {
        if (size_ >= capacity_) {
            int new_capacity = (capacity_ == 0) ? 4 : capacity_ * 2;
            T* new_data = (T*)malloc(sizeof(T) * new_capacity);
            for (int i = 0; i < size_; i++) {
                new_data[i] = data[i];
            }
            if (data) free(data);
            data = new_data;
            capacity_ = new_capacity;
            printf("  [Vector] Resized to capacity %d\n", capacity_);
        }
        data[size_] = value;
        size_++;
    }
    
    void pop_back() {
        if (size_ > 0) size_--;
    }
    
    T& operator[](int index) {
        return data[index];
    }
    
    int size() const { return size_; }
    int capacity() const { return capacity_; }
    bool empty() const { return size_ == 0; }
    
    T& front() { return data[0]; }
    T& back() { return data[size_ - 1]; }
    
    void clear() { size_ = 0; }
    
    void print() {
        printf("  Vector[%d/%d]: ", size_, capacity_);
        for (int i = 0; i < size_; i++) {
            printf("%d ", (int)data[i]);
        }
        printf("\n");
    }
};

void test_vector() {
    printf("\n=== TEST 1: Vector (std::vector simulation) ===\n");
    
    Vector<int> v;
    v.push_back(10);
    v.push_back(20);
    v.push_back(30);
    v.push_back(40);
    v.push_back(50);
    v.print();
    
    printf("  v[2] = %d\n", v[2]);
    printf("  front() = %d, back() = %d\n", v.front(), v.back());
    printf("  size() = %d, capacity() = %d\n", v.size(), v.capacity());
    
    v.pop_back();
    v.print();
}

// ============================================================
// Simplified Stack implementation (simulating std::stack)
// ============================================================
template<typename T>
class Stack {
private:
    T* data;
    int top_;
    int capacity_;
    
public:
    Stack(int cap = 100) : top_(-1), capacity_(cap) {
        data = (T*)malloc(sizeof(T) * capacity_);
        printf("[Stack] Constructor with capacity %d\n", capacity_);
    }
    
    ~Stack() {
        if (data) free(data);
    }
    
    void push(const T& value) {
        if (top_ < capacity_ - 1) {
            top_++;
            data[top_] = value;
        }
    }
    
    void pop() {
        if (top_ >= 0) top_--;
    }
    
    T& top() { return data[top_]; }
    bool empty() const { return top_ < 0; }
    int size() const { return top_ + 1; }
};

void test_stack() {
    printf("\n=== TEST 2: Stack (std::stack simulation) ===\n");
    
    Stack<int> s;
    
    printf("  Pushing: 10, 20, 30, 40, 50\n");
    s.push(10);
    s.push(20);
    s.push(30);
    s.push(40);
    s.push(50);
    
    printf("  Stack size: %d\n", s.size());
    printf("  Top: %d\n", s.top());
    
    printf("  Popping all elements: ");
    while (!s.empty()) {
        printf("%d ", s.top());
        s.pop();
    }
    printf("\n");
    printf("  Stack empty: %s\n", s.empty() ? "true" : "false");
}

// ============================================================
// Simplified Queue implementation (simulating std::queue)
// ============================================================
template<typename T>
class Queue {
private:
    T* data;
    int front_;
    int rear_;
    int size_;
    int capacity_;
    
public:
    Queue(int cap = 100) : front_(0), rear_(-1), size_(0), capacity_(cap) {
        data = (T*)malloc(sizeof(T) * capacity_);
        printf("[Queue] Constructor with capacity %d\n", capacity_);
    }
    
    ~Queue() {
        if (data) free(data);
    }
    
    void push(const T& value) {
        if (size_ < capacity_) {
            rear_ = (rear_ + 1) % capacity_;
            data[rear_] = value;
            size_++;
        }
    }
    
    void pop() {
        if (size_ > 0) {
            front_ = (front_ + 1) % capacity_;
            size_--;
        }
    }
    
    T& front() { return data[front_]; }
    T& back() { return data[rear_]; }
    bool empty() const { return size_ == 0; }
    int size() const { return size_; }
};

void test_queue() {
    printf("\n=== TEST 3: Queue (std::queue simulation) ===\n");
    
    Queue<int> q;
    
    printf("  Enqueueing: 10, 20, 30, 40, 50\n");
    q.push(10);
    q.push(20);
    q.push(30);
    q.push(40);
    q.push(50);
    
    printf("  Queue size: %d\n", q.size());
    printf("  Front: %d, Back: %d\n", q.front(), q.back());
    
    printf("  Dequeueing all elements: ");
    while (!q.empty()) {
        printf("%d ", q.front());
        q.pop();
    }
    printf("\n");
}

// ============================================================
// Simplified Map implementation (simulating std::map)
// ============================================================
template<typename K, typename V>
class Map {
private:
    struct Entry {
        K key;
        V value;
        bool used;
    };
    
    Entry* data;
    int size_;
    int capacity_;
    
    int find_index(const K& key) {
        for (int i = 0; i < capacity_; i++) {
            if (data[i].used && data[i].key == key) {
                return i;
            }
        }
        return -1;
    }
    
public:
    Map(int cap = 100) : size_(0), capacity_(cap) {
        data = (Entry*)malloc(sizeof(Entry) * capacity_);
        for (int i = 0; i < capacity_; i++) {
            data[i].used = false;
        }
        printf("[Map] Constructor with capacity %d\n", capacity_);
    }
    
    ~Map() {
        if (data) free(data);
    }
    
    void insert(const K& key, const V& value) {
        int idx = find_index(key);
        if (idx >= 0) {
            data[idx].value = value;
        } else {
            for (int i = 0; i < capacity_; i++) {
                if (!data[i].used) {
                    data[i].key = key;
                    data[i].value = value;
                    data[i].used = true;
                    size_++;
                    break;
                }
            }
        }
    }
    
    V* find(const K& key) {
        int idx = find_index(key);
        if (idx >= 0) {
            return &data[idx].value;
        }
        return nullptr;
    }
    
    bool contains(const K& key) {
        return find_index(key) >= 0;
    }
    
    void erase(const K& key) {
        int idx = find_index(key);
        if (idx >= 0) {
            data[idx].used = false;
            size_--;
        }
    }
    
    int size() const { return size_; }
    bool empty() const { return size_ == 0; }
    
    void print() {
        printf("  Map[%d]: ", size_);
        for (int i = 0; i < capacity_; i++) {
            if (data[i].used) {
                printf("{%d:%d} ", (int)data[i].key, (int)data[i].value);
            }
        }
        printf("\n");
    }
};

void test_map() {
    printf("\n=== TEST 4: Map (std::map simulation) ===\n");
    
    Map<int, int> m;
    
    m.insert(1, 100);
    m.insert(2, 200);
    m.insert(3, 300);
    m.insert(5, 500);
    m.print();
    
    printf("  contains(3): %s\n", m.contains(3) ? "true" : "false");
    printf("  contains(4): %s\n", m.contains(4) ? "true" : "false");
    
    int* val = m.find(2);
    if (val) {
        printf("  find(2) = %d\n", *val);
    }
    
    m.erase(2);
    printf("  After erase(2): ");
    m.print();
}

// ============================================================
// Simplified Set implementation (simulating std::set)
// ============================================================
template<typename T>
class Set {
private:
    T* data;
    int size_;
    int capacity_;
    
    int find_index(const T& value) {
        for (int i = 0; i < size_; i++) {
            if (data[i] == value) return i;
        }
        return -1;
    }
    
public:
    Set(int cap = 100) : size_(0), capacity_(cap) {
        data = (T*)malloc(sizeof(T) * capacity_);
        printf("[Set] Constructor with capacity %d\n", capacity_);
    }
    
    ~Set() {
        if (data) free(data);
    }
    
    bool insert(const T& value) {
        if (find_index(value) >= 0) return false;  // Already exists
        if (size_ < capacity_) {
            data[size_] = value;
            size_++;
            return true;
        }
        return false;
    }
    
    bool contains(const T& value) {
        return find_index(value) >= 0;
    }
    
    void erase(const T& value) {
        int idx = find_index(value);
        if (idx >= 0) {
            for (int i = idx; i < size_ - 1; i++) {
                data[i] = data[i + 1];
            }
            size_--;
        }
    }
    
    int size() const { return size_; }
    bool empty() const { return size_ == 0; }
    
    void print() {
        printf("  Set[%d]: { ", size_);
        for (int i = 0; i < size_; i++) {
            printf("%d ", (int)data[i]);
        }
        printf("}\n");
    }
};

void test_set() {
    printf("\n=== TEST 5: Set (std::set simulation) ===\n");
    
    Set<int> s;
    
    printf("  Inserting: 5, 3, 8, 1, 9, 3 (duplicate)\n");
    s.insert(5);
    s.insert(3);
    s.insert(8);
    s.insert(1);
    s.insert(9);
    bool dup = s.insert(3);  // Should fail (duplicate)
    printf("  Insert duplicate 3: %s\n", dup ? "success" : "failed (expected)");
    
    s.print();
    
    printf("  contains(5): %s\n", s.contains(5) ? "true" : "false");
    printf("  contains(7): %s\n", s.contains(7) ? "true" : "false");
    
    s.erase(3);
    printf("  After erase(3): ");
    s.print();
}

// ============================================================
// Simplified List implementation (simulating std::list)
// ============================================================
template<typename T>
class List {
private:
    struct Node {
        T value;
        Node* next;
        Node* prev;
    };
    
    Node* head;
    Node* tail;
    int size_;
    
public:
    List() : head(nullptr), tail(nullptr), size_(0) {
        printf("[List] Constructor\n");
    }
    
    ~List() {
        Node* current = head;
        while (current) {
            Node* next = current->next;
            free(current);
            current = next;
        }
        printf("[List] Destructor, freed %d nodes\n", size_);
    }
    
    void push_back(const T& value) {
        Node* node = (Node*)malloc(sizeof(Node));
        node->value = value;
        node->next = nullptr;
        node->prev = tail;
        
        if (tail) {
            tail->next = node;
        } else {
            head = node;
        }
        tail = node;
        size_++;
    }
    
    void push_front(const T& value) {
        Node* node = (Node*)malloc(sizeof(Node));
        node->value = value;
        node->prev = nullptr;
        node->next = head;
        
        if (head) {
            head->prev = node;
        } else {
            tail = node;
        }
        head = node;
        size_++;
    }
    
    void pop_back() {
        if (tail) {
            Node* prev = tail->prev;
            free(tail);
            tail = prev;
            if (tail) {
                tail->next = nullptr;
            } else {
                head = nullptr;
            }
            size_--;
        }
    }
    
    void pop_front() {
        if (head) {
            Node* next = head->next;
            free(head);
            head = next;
            if (head) {
                head->prev = nullptr;
            } else {
                tail = nullptr;
            }
            size_--;
        }
    }
    
    T& front() { return head->value; }
    T& back() { return tail->value; }
    int size() const { return size_; }
    bool empty() const { return size_ == 0; }
    
    void print() {
        printf("  List[%d]: ", size_);
        Node* current = head;
        while (current) {
            printf("%d ", (int)current->value);
            current = current->next;
        }
        printf("\n");
    }
};

void test_list() {
    printf("\n=== TEST 6: List (std::list simulation) ===\n");
    
    List<int> l;
    
    l.push_back(30);
    l.push_back(40);
    l.push_front(20);
    l.push_front(10);
    l.push_back(50);
    
    l.print();
    
    printf("  front() = %d, back() = %d\n", l.front(), l.back());
    
    l.pop_front();
    l.pop_back();
    printf("  After pop_front() and pop_back(): ");
    l.print();
}

// ============================================================
// Main — Run all tests
// ============================================================
int main() {
    printf("============================================================\n");
    printf("ADead-BIB C++ STL Containers Intensive Test\n");
    printf("============================================================\n");
    
    test_vector();
    test_stack();
    test_queue();
    test_map();
    test_set();
    test_list();
    
    printf("\n============================================================\n");
    printf("All STL Container tests completed!\n");
    printf("============================================================\n");
    
    return 0;
}
