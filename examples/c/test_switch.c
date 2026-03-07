#include <stdio.h>
int classify(int n) { switch(n) { case 0: return 0; case 1: return 1; case 2: return 4; default: return -1; } }
int main() { printf("%d %d %d\n", classify(0), classify(2), classify(9)); return 0; }