// ADead-BIB C++ Example — Object-Oriented Programming
// Compilar: adB cxx cpp_oop.cpp -o oop.exe
//
// Demuestra: classes, herencia, virtual, override, constructors, destructors
// ADead-BIB elimina vtable overhead en compilacion

int printf(const char *format, ...);

// Base class — Animal
class Animal {
public:
    int age;
    int weight;

    Animal(int a, int w) : age(a), weight(w) {}

    virtual void speak() {
        printf("...\n");
    }

    int getAge() {
        return age;
    }
};

// Derived class — Dog
class Dog : public Animal {
public:
    int tricks;

    Dog(int a, int w, int t) : Animal(a, w), tricks(t) {}

    void speak() override {
        printf("Woof! Age: %d, Tricks: %d\n", age, tricks);
    }

    void fetch() {
        printf("Dog fetches the ball!\n");
        tricks = tricks + 1;
    }
};

// Derived class — Cat
class Cat : public Animal {
public:
    int lives;

    Cat(int a, int w) : Animal(a, w), lives(9) {}

    void speak() override {
        printf("Meow! Age: %d, Lives: %d\n", age, lives);
    }
};

// Template class — simple container
template<typename T>
class Box {
public:
    T value;

    Box(T v) : value(v) {}

    T get() {
        return value;
    }
};

// Enum class (C++11)
enum class Color : int {
    Red = 0,
    Green = 1,
    Blue = 2
};

int main() {
    printf("=== ADead-BIB C++ OOP Demo ===\n\n");

    // Stack objects
    Dog rex(5, 30, 10);
    Cat mia(3, 4);

    printf("Rex: ");
    rex.speak();
    rex.fetch();

    printf("Mia: ");
    mia.speak();

    printf("\nRex age: %d\n", rex.getAge());
    printf("Mia age: %d\n", mia.getAge());

    // Template usage
    Box<int> intBox(42);
    printf("\nBox contains: %d\n", intBox.get());

    printf("\n=== Done! ===\n");
    return 0;
}
