#include <header_main.h>

// Test Win32 API compat layer declarations
// These test that the types and constants are recognized

int main() {
    // Win32 types
    printf("Win32 type test\n");

    // Memory constants
    int mem_commit = 0x1000;
    int mem_reserve = 0x2000;
    int page_rw = 0x04;
    printf("MEM_COMMIT=%d MEM_RESERVE=%d PAGE_RW=%d\n",
           mem_commit, mem_reserve, page_rw);

    // Handle constants
    int invalid = -1;
    printf("INVALID_HANDLE=%d\n", invalid);

    // Error codes
    int err_success = 0;
    int err_not_found = 2;
    printf("ERROR_SUCCESS=%d FILE_NOT_FOUND=%d\n", err_success, err_not_found);

    // Console handle IDs
    int std_in = -10;
    int std_out = -11;
    int std_err = -12;
    printf("STD_IN=%d STD_OUT=%d STD_ERR=%d\n", std_in, std_out, std_err);

    printf("win32 compat test complete\n");
    return 0;
}
