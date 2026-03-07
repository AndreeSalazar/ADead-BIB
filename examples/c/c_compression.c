#include <stdio.h>
#include <string.h>
#include <zlib.h>
int rle_encode(const char *in, int len, char *out) { int j=0; for(int i=0;i<len;){int c=1;while(i+c<len&&in[i]==in[i+c]&&c<255)c++;out[j++]=in[i];out[j++]=c;i+=c;} return j; }
int main() { char buf[64]; int n=rle_encode("AAABBC",6,buf); printf("encoded %d bytes\n",n); return 0; }