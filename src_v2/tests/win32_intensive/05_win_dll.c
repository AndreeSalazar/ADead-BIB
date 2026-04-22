#include <windows.h>
#include <stdio.h>

// Function pointer typedef for MessageBoxA
typedef int (WINAPI *MessageBoxA_Fn)(HWND hWnd, LPCSTR lpText, LPCSTR lpCaption, UINT uType);

int main() {
    printf("--- Test 05: Win32 DLL Dynamic Loading ---\n");

    // 1. LoadLibrary
    HMODULE hUser32 = LoadLibraryA("user32.dll");
    if (hUser32 == NULL) {
        printf("LoadLibraryA failed!\n");
        return 1;
    }

    // 2. GetProcAddress
    MessageBoxA_Fn pMessageBoxA = (MessageBoxA_Fn)GetProcAddress(hUser32, "MessageBoxA");
    if (pMessageBoxA == NULL) {
        printf("GetProcAddress failed!\n");
        FreeLibrary(hUser32);
        return 1;
    }

    // 3. Indirect call!
    printf("Calling MessageBoxA dynamically...\n");
    int result = pMessageBoxA(NULL, "Hello from dynamically loaded user32.dll!\nCalled via function pointer.", "ADead-BIB DLL Test", MB_OK | MB_ICONINFORMATION);
    
    printf("MessageBoxA returned: %d\n", result);

    // 4. FreeLibrary
    FreeLibrary(hUser32);
    
    printf("DLL loading test passed.\n");
    return 0;
}
