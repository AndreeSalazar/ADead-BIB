#include <stdio.h>
#include <pthread.h>
int shared = 0;
void *worker(void *arg) { int id = *(int*)arg; shared += id; return NULL; }
int main() { int id = 1; worker(&id); printf("shared=%d\n", shared); return 0; }