#include <windows.h>
#include <stdio.h>

int main() {
    printf("--- Test 01: Win32 Memory Management ---\n");

    // 1. HeapAlloc
    HANDLE hHeap = GetProcessHeap();
    if (hHeap == NULL) {
        printf("GetProcessHeap failed!\n");
        return 1;
    }

    int* heap_array = (int*)HeapAlloc(hHeap, HEAP_ZERO_MEMORY, 10 * sizeof(int));
    if (heap_array != NULL) {
        heap_array[0] = 42;
        heap_array[9] = 100;
        printf("HeapAlloc success: arr[0]=%d, arr[9]=%d\n", heap_array[0], heap_array[9]);
        HeapFree(hHeap, 0, heap_array);
    }

    // 2. VirtualAlloc
    void* virtual_mem = VirtualAlloc(NULL, 4096, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
    if (virtual_mem != NULL) {
        char* str = (char*)virtual_mem;
        strcpy(str, "VirtualAlloc is working natively!");
        printf("VirtualAlloc success: %s\n", str);
        VirtualFree(virtual_mem, 0, MEM_RELEASE);
    }

    // 3. GlobalAlloc
    HGLOBAL hGlobal = GlobalAlloc(GMEM_MOVEABLE | GMEM_ZEROINIT, 128);
    if (hGlobal != NULL) {
        void* ptr = GlobalLock(hGlobal);
        if (ptr != NULL) {
            short* sptr = (short*)ptr;
            sptr[0] = 0xAA55;
            printf("GlobalAlloc & Lock success: %x\n", sptr[0]);
            GlobalUnlock(hGlobal);
        }
        GlobalFree(hGlobal);
    }

    printf("Memory test passed.\n");
    return 0;
}
