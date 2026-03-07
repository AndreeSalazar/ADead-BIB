#include <header_main.h>
class Counter {
public:
    int val;
    Counter() : val(0) {}
    void inc() { val++; }
    int get() { return val; }
};
int main() {
    printf("C++ libre\n");
    Counter c;
    c.inc();
    c.inc();
    c.inc();
    printf("counter=%d\n", c.get());
    int arr[3];
    arr[0] = 10; arr[1] = 20; arr[2] = 30;
    int sum = 0;
    for (int i = 0; i < 3; i++) sum += arr[i];
    printf("sum=%d\n", sum);
    return 0;
}