#include <stdio.h>
struct Pair { int a; int b; };
int main() { printf("int=%lu Pair=%lu\n",(unsigned long)sizeof(int),(unsigned long)sizeof(struct Pair)); return 0; }