#include <stdio.h>
int counter = 0;
int step = 1;
void inc() { counter += step; }
int get() { return counter; }
void reset() { counter = 0; }
int main() { inc(); inc(); inc(); printf("c=%d\n",get()); reset(); printf("r=%d\n",get()); return 0; }