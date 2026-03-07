#include <stdio.h>
unsigned int set_bit(unsigned int x, int p) { return x|(1<<p); }
unsigned int clear_bit(unsigned int x, int p) { return x&~(1<<p); }
int count_bits(unsigned int x) { int c=0; while(x){c+=x&1;x>>=1;} return c; }
int main() { unsigned int f=0; f=set_bit(f,0); f=set_bit(f,3); printf("f=0x%x bits=%d\n",f,count_bits(0xFF)); int a=0xAB; printf("%d %d %d %d\n",a<<4,a>>2,a&0x0F,a|0xF0); return 0; }