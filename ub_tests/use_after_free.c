void free(void* ptr);
void* malloc(unsigned long size);

int main() {
    int* ptr = (int*)malloc(4);
    free(ptr);
    *ptr = 20; // UB: Use After Free
    return 0;
}
