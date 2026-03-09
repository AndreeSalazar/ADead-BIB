#include <ctype.h>
#include <stdio.h>

int main() {
    char c = 'A';
    int a = isalpha(c);
    int b = isdigit('9');
    int d = isalnum('z');
    int e = isspace(' ');
    int f = isupper('A');
    int g = islower('a');
    int h = ispunct('!');
    int i = isprint('x');
    int j = iscntrl('\n');
    int k = isxdigit('f');
    int l = isgraph('x');
    int m = isblank('\t');
    char n = toupper('a');
    char o = tolower('A');
    printf("isalpha('A') = %d\n", a);
    return 0;
}
