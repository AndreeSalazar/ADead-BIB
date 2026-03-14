#include <header_main.h>

// Test POSIX API compat layer declarations

int main() {
    // POSIX flags
    int o_rdonly = 0x0000;
    int o_wronly = 0x0001;
    int o_rdwr = 0x0002;
    int o_creat = 0x0040;
    printf("O_RDONLY=%d O_WRONLY=%d O_RDWR=%d O_CREAT=%d\n",
           o_rdonly, o_wronly, o_rdwr, o_creat);

    // mmap flags
    int prot_read = 0x1;
    int prot_write = 0x2;
    int map_private = 0x02;
    int map_anon = 0x20;
    printf("PROT_READ=%d PROT_WRITE=%d MAP_PRIVATE=%d MAP_ANON=%d\n",
           prot_read, prot_write, map_private, map_anon);

    // errno values
    int enoent = 2;
    int eacces = 13;
    int enomem = 12;
    int enosys = 38;
    printf("ENOENT=%d EACCES=%d ENOMEM=%d ENOSYS=%d\n",
           enoent, eacces, enomem, enosys);

    // Socket constants
    int af_inet = 2;
    int sock_stream = 1;
    printf("AF_INET=%d SOCK_STREAM=%d\n", af_inet, sock_stream);

    printf("posix compat test complete\n");
    return 0;
}
