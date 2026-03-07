#include <stdio.h>
#include <string.h>
void xor_cipher(char *data, int len, char key) { for(int i=0;i<len;i++) data[i]^=key; }
unsigned int simple_hash(const char *s) { unsigned int h=0; while(*s){h=h*31+(*s);s++;} return h; }
int main() { char msg[]="Hello"; xor_cipher(msg,5,0x42); xor_cipher(msg,5,0x42); printf("hash=%u\n",simple_hash("test")); return 0; }