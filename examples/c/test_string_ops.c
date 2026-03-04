#include <stdio.h>

int my_strlen(const char *s) {
    int len = 0;
    while (s[len] != '\0') {
        len++;
    }
    return len;
}

int main() {
    const char *hello = "Hello, World!";
    printf("str=%s\n", hello);
    printf("len=%d\n", my_strlen(hello));
    
    const char *msg = "ADead" "-BIB";
    printf("concat=%s\n", msg);
    
    char ch = 'A';
    printf("char=%c code=%d\n", ch, ch);
    
    printf("escape: tab=[\t] newline follows\n");
    return 0;
}
