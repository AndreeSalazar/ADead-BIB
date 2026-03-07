// ADead-BIB C Example - Hello World
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
void greet(const char *name) { printf("Hello, %s!\n", name); }
int add(int a, int b) { return a + b; }
int main() { printf("Hello from ADead-BIB!\n"); greet("World"); int r = add(10, 20); printf("10+20=%d\n", r); return 0; }