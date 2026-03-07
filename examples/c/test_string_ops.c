#include <stdio.h>
#include <string.h>
int my_strlen(const char *s) { int n=0; while(s[n]) n++; return n; }
void my_reverse(char *s) { int n=my_strlen(s); for(int i=0;i<n/2;i++){char t=s[i];s[i]=s[n-1-i];s[n-1-i]=t;} }
int main() { char buf[32]; strcpy(buf,"Hello"); printf("len=%d\n",my_strlen(buf)); my_reverse(buf); printf("rev=%s\n",buf); return 0; }