#include <iostream>
class Animal {
public:
    int weight;
    Animal(int w) : weight(w) {}
    int get_weight() { return weight; }
};
class Dog : public Animal {
public:
    int speed;
    Dog(int w, int s) : Animal(w), speed(s) {}
    int get_speed() { return speed; }
};
int main() { Dog d(30, 10); printf("w=%d s=%d\n", d.get_weight(), d.get_speed()); return 0; }