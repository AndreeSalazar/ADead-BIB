// ADead-BIB Bridge Test 15 — Win32 Threads
// Level: ADVANCED
// Tests: CreateThread, WaitForSingleObject, InterlockedIncrement,
//        CloseHandle, GetExitCodeThread
// Requires: kernel32.dll in IAT

typedef void* HANDLE;
typedef unsigned long DWORD;
typedef unsigned int UINT;
typedef long LONG;
typedef DWORD (*LPTHREAD_START_ROUTINE)(void*);

extern int printf(const char*, ...);

// kernel32
extern HANDLE CreateThread(void* lpThreadAttributes, unsigned long long dwStackSize,
    LPTHREAD_START_ROUTINE lpStartAddress, void* lpParameter,
    DWORD dwCreationFlags, DWORD* lpThreadId);
extern DWORD WaitForSingleObject(HANDLE hHandle, DWORD dwMilliseconds);
extern int CloseHandle(HANDLE hObject);
extern int GetExitCodeThread(HANDLE hThread, DWORD* lpExitCode);
extern LONG InterlockedIncrement(LONG* Addend);
extern void Sleep(DWORD dwMilliseconds);

#define INFINITE 0xFFFFFFFF

// Shared counter for thread increment test
volatile LONG g_counter = 0;

DWORD thread_increment(void* param) {
    int iterations = *(int*)param;
    for (int i = 0; i < iterations; i++) {
        InterlockedIncrement(&g_counter);
    }
    return 42; // known return value for exit code test
}

DWORD thread_simple(void* param) {
    int val = *(int*)param;
    return (DWORD)(val * 2);
}

int main() {
    printf("=== ADead-BIB Bridge Test 15: Win32 Threads ===\n");
    int pass = 0, fail = 0;

    // 1. Create a single thread and verify it runs
    int arg1 = 55;
    DWORD tid1 = 0;
    HANDLE h1 = CreateThread(0, 0, thread_simple, &arg1, 0, &tid1);
    if (h1) { pass++; } else { fail++; printf("FAIL: CreateThread single\n"); }

    // 2. WaitForSingleObject on single thread
    if (h1) {
        DWORD wait = WaitForSingleObject(h1, INFINITE);
        if (wait == 0) { pass++; } else { fail++; printf("FAIL: WaitForSingleObject=%u\n", (unsigned int)wait); }

        // 3. GetExitCodeThread — expect 110 (55*2)
        DWORD exitCode = 0;
        int ok = GetExitCodeThread(h1, &exitCode);
        if (ok && exitCode == 110) { pass++; } else { fail++; printf("FAIL: GetExitCodeThread=%u expected=110\n", (unsigned int)exitCode); }

        CloseHandle(h1);
    }

    // 4. Create 4 threads that each InterlockedIncrement 1000 times
    g_counter = 0;
    int iterations = 1000;
    HANDLE threads[4];
    DWORD tids[4];
    int created = 1;
    for (int i = 0; i < 4; i++) {
        threads[i] = CreateThread(0, 0, thread_increment, &iterations, 0, &tids[i]);
        if (!threads[i]) { created = 0; }
    }
    if (created) { pass++; } else { fail++; printf("FAIL: CreateThread x4\n"); }

    // 5. Wait for all 4 threads
    int all_waited = 1;
    for (int i = 0; i < 4; i++) {
        if (threads[i]) {
            DWORD wait = WaitForSingleObject(threads[i], INFINITE);
            if (wait != 0) all_waited = 0;
        }
    }
    if (all_waited) { pass++; } else { fail++; printf("FAIL: WaitForSingleObject x4\n"); }

    // 6. Verify counter == 4000
    if (g_counter == 4000) { pass++; } else { fail++; printf("FAIL: counter=%d expected=4000\n", (int)g_counter); }

    // Cleanup handles
    for (int i = 0; i < 4; i++) {
        if (threads[i]) CloseHandle(threads[i]);
    }

    printf("Results: %d passed, %d failed\n", pass, fail);
    printf("=== Test 15: %s ===\n", fail == 0 ? "PASS" : "FAIL");
    return fail;
}
