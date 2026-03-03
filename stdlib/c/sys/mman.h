/*
 * ADead-BIB Standard Library
 * sys/mman.h - Memory Management
 * 
 * Based on: POSIX
 */

#ifndef _ADEAD_SYS_MMAN_H
#define _ADEAD_SYS_MMAN_H

#include "types.h"

/* Protection flags */
#define PROT_NONE   0x0
#define PROT_READ   0x1
#define PROT_WRITE  0x2
#define PROT_EXEC   0x4

/* Map flags */
#define MAP_SHARED      0x01
#define MAP_PRIVATE     0x02
#define MAP_FIXED       0x10
#define MAP_ANONYMOUS   0x20
#define MAP_ANON        MAP_ANONYMOUS
#define MAP_GROWSDOWN   0x100
#define MAP_DENYWRITE   0x800
#define MAP_EXECUTABLE  0x1000
#define MAP_LOCKED      0x2000
#define MAP_NORESERVE   0x4000
#define MAP_POPULATE    0x8000
#define MAP_NONBLOCK    0x10000
#define MAP_STACK       0x20000
#define MAP_HUGETLB     0x40000

/* Map failed */
#define MAP_FAILED ((void*)-1)

/* msync flags */
#define MS_ASYNC      1
#define MS_SYNC       4
#define MS_INVALIDATE 2

/* madvise advice */
#define MADV_NORMAL     0
#define MADV_RANDOM     1
#define MADV_SEQUENTIAL 2
#define MADV_WILLNEED   3
#define MADV_DONTNEED   4
#define MADV_FREE       8
#define MADV_HUGEPAGE   14
#define MADV_NOHUGEPAGE 15

/* mlock flags */
#define MCL_CURRENT 1
#define MCL_FUTURE  2
#define MCL_ONFAULT 4

/* Functions */
void* mmap(void* addr, size_t length, int prot, int flags, int fd, off_t offset);
int munmap(void* addr, size_t length);
int mprotect(void* addr, size_t len, int prot);
int msync(void* addr, size_t length, int flags);
int madvise(void* addr, size_t length, int advice);
int mlock(const void* addr, size_t len);
int munlock(const void* addr, size_t len);
int mlockall(int flags);
int munlockall(void);
int mincore(void* addr, size_t length, unsigned char* vec);
void* mremap(void* old_address, size_t old_size, size_t new_size, int flags, ...);

/* Shared memory */
int shm_open(const char* name, int oflag, mode_t mode);
int shm_unlink(const char* name);

#endif /* _ADEAD_SYS_MMAN_H */
