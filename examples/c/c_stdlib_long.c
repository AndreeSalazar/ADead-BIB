#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <ctype.h>
#include <time.h>
#include <errno.h>
#include <assert.h>
#include <limits.h>
int f1(){return 1;} int f2(){return 2;} int f3(){return 3;} int f4(){return 4;}
int f5(){return 5;} int f6(){return 6;} int f7(){return 7;} int f8(){return 8;}
int f9(){return 9;} int f10(){return 10;} int f11(){return 11;} int f12(){return 12;}
int f13(){return 13;} int f14(){return 14;} int f15(){return 15;} int f16(){return 16;}
int f17(){return 17;} int f18(){return 18;} int f19(){return 19;} int f20(){return 20;}
int f21(){return 21;}
int main() { int s = f1()+f2()+f3()+f4()+f5()+f6()+f7()+f8()+f9()+f10()+f11()+f12()+f13()+f14()+f15()+f16()+f17()+f18()+f19()+f20()+f21(); printf("sum=%d\n", s); return 0; }