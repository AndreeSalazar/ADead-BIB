#include <windows.h>
#include <stdio.h>

// Global shared state
int counter = 0;
HANDLE hMutex = NULL;

// Thread Callback (Requires __stdcall via WINAPI/DWORD)
DWORD WINAPI ThreadFunc(LPVOID lpParam) {
    int thread_id = (int)(intptr_t)lpParam;
    
    // Acquire Mutex
    WaitForSingleObject(hMutex, INFINITE);
    
    printf("Thread %d acquired mutex. Counter was %d\n", thread_id, counter);
    counter += 10;
    
    // Release Mutex
    ReleaseMutex(hMutex);
    
    return 0;
}

int main() {
    printf("--- Test 03: Win32 Threads & Sync ---\n");

    hMutex = CreateMutex(NULL, FALSE, NULL);
    if (hMutex == NULL) {
        printf("CreateMutex failed.\n");
        return 1;
    }

    HANDLE threads[2];
    DWORD threadId1, threadId2;

    threads[0] = CreateThread(
        NULL,                   // default security
        0,                      // default stack size
        ThreadFunc,             // thread function (requires indirect call support)
        (LPVOID)1,              // argument
        0,                      // default creation flags
        &threadId1              // returns thread id
    );

    threads[1] = CreateThread(
        NULL,
        0,
        ThreadFunc,
        (LPVOID)2,
        0,
        &threadId2
    );

    if (threads[0] != NULL && threads[1] != NULL) {
        // Wait for both threads
        WaitForMultipleObjects(2, threads, TRUE, INFINITE);
        printf("Both threads finished. Final counter: %d\n", counter);
        
        CloseHandle(threads[0]);
        CloseHandle(threads[1]);
    } else {
        printf("CreateThread failed.\n");
    }

    CloseHandle(hMutex);
    printf("Threads test passed.\n");
    return 0;
}
