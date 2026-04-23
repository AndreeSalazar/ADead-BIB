#include <windows.h>
#include <stdio.h>

int main() {
    printf("--- Test 02: Win32 File I/O ---\n");

    const char* filename = "test_adead_win32.txt";
    const char* write_data = "Hello from ADead-BIB Native Windows API!";
    DWORD bytesWritten = 0;
    DWORD bytesRead = 0;
    char read_buffer[100];
    memset(read_buffer, 0, sizeof(read_buffer));

    // 1. CreateFileA (Write)
    HANDLE hFileWrite = CreateFileA(
        filename,
        GENERIC_WRITE,
        0,
        NULL,
        CREATE_ALWAYS,
        FILE_ATTRIBUTE_NORMAL,
        NULL
    );

    if (hFileWrite == INVALID_HANDLE_VALUE) {
        printf("CreateFileA (Write) failed. Error: %d\n", GetLastError());
        return 1;
    }

    // 2. WriteFile
    BOOL wRes = WriteFile(
        hFileWrite,
        write_data,
        strlen(write_data),
        &bytesWritten,
        NULL // OVERLAPPED
    );

    if (wRes) {
        printf("WriteFile success: %d bytes written.\n", bytesWritten);
    } else {
        printf("WriteFile failed.\n");
    }
    CloseHandle(hFileWrite);

    // 3. CreateFileA (Read)
    HANDLE hFileRead = CreateFileA(
        filename,
        GENERIC_READ,
        FILE_SHARE_READ,
        NULL,
        OPEN_EXISTING,
        FILE_ATTRIBUTE_NORMAL,
        NULL
    );

    if (hFileRead != INVALID_HANDLE_VALUE) {
        // 4. ReadFile
        BOOL rRes = ReadFile(
            hFileRead,
            read_buffer,
            sizeof(read_buffer) - 1,
            &bytesRead,
            NULL
        );
        if (rRes) {
            printf("ReadFile success: %d bytes read. Content: '%s'\n", bytesRead, read_buffer);
        } else {
            printf("ReadFile failed.\n");
        }
        CloseHandle(hFileRead);
    }

    // Clean up
    DeleteFileA(filename);
    
    printf("File I/O test passed.\n");
    return 0;
}
