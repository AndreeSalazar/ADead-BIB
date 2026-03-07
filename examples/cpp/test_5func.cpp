#include <iostream>
int f1(int x) { return x + 1; }
int f2(int x) { return x * 2; }
int f3(int x) { return x - 3; }
int f4(int x) { return x / 2; }
int f5(int x) { return x % 7; }
int main() { int r = f1(f2(f3(f4(f5(100))))); printf("r=%d\n", r); return 0; }