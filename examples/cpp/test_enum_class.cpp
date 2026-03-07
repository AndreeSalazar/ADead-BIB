#include <iostream>
enum class Color { Red, Green, Blue };
int main() { Color c = Color::Green; int v = (int)c; printf("color=%d\n", v); return 0; }