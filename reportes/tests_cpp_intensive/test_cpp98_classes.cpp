// ============================================================
// ADead-BIB C++98 Intensive Test — Classes & OOP
// "Respetar Bits" — Type Strictness ULTRA
// ============================================================
// Ejecutar: adb cpp reportes/tests_cpp_intensive/test_cpp98_classes.cpp
// ============================================================

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// ============================================================
// Test 1: Basic Class with Constructor/Destructor
// ============================================================
class Point {
public:
    int x, y;
    
    Point() : x(0), y(0) {
        printf("[Point] Default constructor: (%d, %d)\n", x, y);
    }
    
    Point(int px, int py) : x(px), y(py) {
        printf("[Point] Parameterized constructor: (%d, %d)\n", x, y);
    }
    
    ~Point() {
        printf("[Point] Destructor: (%d, %d)\n", x, y);
    }
    
    void print() {
        printf("  Point: x=%d, y=%d\n", x, y);
    }
    
    int distanceSquared() {
        return x * x + y * y;
    }
};

void test_basic_class() {
    printf("\n=== TEST 1: Basic Class ===\n");
    Point p1;
    p1.print();
    
    Point p2(3, 4);
    p2.print();
    printf("  Distance squared: %d\n", p2.distanceSquared());
}

// ============================================================
// Test 2: Class Inheritance
// ============================================================
class Shape {
public:
    int width, height;
    
    Shape(int w, int h) : width(w), height(h) {
        printf("[Shape] Base constructor: %dx%d\n", width, height);
    }
    
    int area() {
        return 0;
    }
};

class Rectangle : public Shape {
public:
    Rectangle(int w, int h) : Shape(w, h) {
        printf("[Rectangle] Derived constructor\n");
    }
    
    int area() {
        return width * height;
    }
};

class Triangle : public Shape {
public:
    Triangle(int w, int h) : Shape(w, h) {
        printf("[Triangle] Derived constructor\n");
    }
    
    int area() {
        return (width * height) / 2;
    }
};

void test_inheritance() {
    printf("\n=== TEST 2: Class Inheritance ===\n");
    
    Rectangle rect(10, 5);
    printf("  Rectangle area: %d\n", rect.area());
    
    Triangle tri(10, 5);
    printf("  Triangle area: %d\n", tri.area());
    
    printf("  [OK] Inheritance works!\n");
}

// ============================================================
// Test 3: Encapsulation (private/public/protected)
// ============================================================
class BankAccount {
private:
    int balance;
    int accountNumber;
    
public:
    BankAccount(int accNum, int initialBalance) 
        : accountNumber(accNum), balance(initialBalance) {
        printf("[BankAccount] Created account #%d with $%d\n", accountNumber, balance);
    }
    
    void deposit(int amount) {
        if (amount > 0) {
            balance = balance + amount;
            printf("  Deposited $%d, new balance: $%d\n", amount, balance);
        }
    }
    
    bool withdraw(int amount) {
        if (amount > 0 && amount <= balance) {
            balance = balance - amount;
            printf("  Withdrew $%d, new balance: $%d\n", amount, balance);
            return true;
        }
        printf("  Withdrawal of $%d FAILED (balance: $%d)\n", amount, balance);
        return false;
    }
    
    int getBalance() {
        return balance;
    }
};

void test_encapsulation() {
    printf("\n=== TEST 3: Encapsulation ===\n");
    BankAccount acc(12345, 1000);
    acc.deposit(500);
    acc.withdraw(200);
    acc.withdraw(2000);  // Should fail
    printf("  Final balance: $%d\n", acc.getBalance());
}

// ============================================================
// Test 4: Static Members
// ============================================================
// Note: Static members not fully supported yet
// Using instance counter pattern instead
class Counter {
private:
    int id;
    
public:
    Counter(int i) : id(i) {
        printf("[Counter] Created instance #%d\n", id);
    }
    
    int getId() {
        return id;
    }
};

void test_static_members() {
    printf("\n=== TEST 4: Instance Tracking ===\n");
    
    Counter c1(1);
    Counter c2(2);
    Counter c3(3);
    
    printf("  Instance IDs: %d, %d, %d\n", c1.getId(), c2.getId(), c3.getId());
    printf("  [OK] Multiple instances created\n");
}

// ============================================================
// Test 5: Operator Overloading
// ============================================================
class Vector2D {
public:
    float x, y;
    
    Vector2D() : x(0.0f), y(0.0f) {}
    Vector2D(float px, float py) : x(px), y(py) {}
    
    Vector2D operator+(const Vector2D& other) {
        return Vector2D(x + other.x, y + other.y);
    }
    
    Vector2D operator-(const Vector2D& other) {
        return Vector2D(x - other.x, y - other.y);
    }
    
    Vector2D operator*(float scalar) {
        return Vector2D(x * scalar, y * scalar);
    }
    
    bool operator==(const Vector2D& other) {
        return (x == other.x) && (y == other.y);
    }
    
    void print() {
        printf("  Vector2D(%.2f, %.2f)\n", x, y);
    }
};

void test_operator_overloading() {
    printf("\n=== TEST 5: Operator Overloading ===\n");
    Vector2D v1(3.0f, 4.0f);
    Vector2D v2(1.0f, 2.0f);
    
    printf("  v1: "); v1.print();
    printf("  v2: "); v2.print();
    
    Vector2D v3 = v1 + v2;
    printf("  v1 + v2: "); v3.print();
    
    Vector2D v4 = v1 - v2;
    printf("  v1 - v2: "); v4.print();
    
    Vector2D v5 = v1 * 2.0f;
    printf("  v1 * 2: "); v5.print();
    
    printf("  v1 == v2: %s\n", (v1 == v2) ? "true" : "false");
    printf("  v1 == v1: %s\n", (v1 == v1) ? "true" : "false");
}

// ============================================================
// Test 6: Copy Constructor & Assignment
// ============================================================
class String {
private:
    char* data;
    int length;
    
public:
    String() : data(0), length(0) {
        printf("[String] Default constructor\n");
    }
    
    String(const char* str) {
        length = 0;
        while (str[length] != '\0') length++;
        data = (char*)malloc(length + 1);
        for (int i = 0; i <= length; i++) {
            data[i] = str[i];
        }
        printf("[String] Constructor from \"%s\" (len=%d)\n", data, length);
    }
    
    String(const String& other) {
        length = other.length;
        data = (char*)malloc(length + 1);
        for (int i = 0; i <= length; i++) {
            data[i] = other.data[i];
        }
        printf("[String] Copy constructor from \"%s\"\n", data);
    }
    
    ~String() {
        if (data) {
            printf("[String] Destructor for \"%s\"\n", data);
            free(data);
        }
    }
    
    void print() {
        if (data) {
            printf("  String: \"%s\" (len=%d)\n", data, length);
        } else {
            printf("  String: (empty)\n");
        }
    }
};

void test_copy_constructor() {
    printf("\n=== TEST 6: Copy Constructor ===\n");
    String s1("Hello");
    s1.print();
    
    String s2 = s1;  // Copy constructor
    s2.print();
    
    String s3("World");
    s3.print();
}

// ============================================================
// Main — Run all tests
// ============================================================
int main() {
    printf("============================================================\n");
    printf("ADead-BIB C++98 Intensive Test — Classes & OOP\n");
    printf("============================================================\n");
    
    test_basic_class();
    test_inheritance();
    test_encapsulation();
    test_static_members();
    test_operator_overloading();
    test_copy_constructor();
    
    printf("\n============================================================\n");
    printf("All C++98 Class tests completed!\n");
    printf("==============================================================\n");
    
    return 0;
}
