// Test 23: Void function
int counter = 0;

void increment(void) {
    counter = counter + 1;
}

int main() {
    increment();
    increment();
    increment();
    return counter - 3;
}
