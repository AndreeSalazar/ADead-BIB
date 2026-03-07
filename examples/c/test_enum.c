#include <stdio.h>
enum Color { RED, GREEN, BLUE, YELLOW = 10, WHITE };
int main() { enum Color c = GREEN; printf("GREEN=%d YELLOW=%d WHITE=%d\n", c, YELLOW, WHITE); return 0; }