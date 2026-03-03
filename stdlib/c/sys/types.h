/*
 * ADead-BIB Standard Library
 * sys/types.h - System Data Types
 * 
 * Based on: POSIX, x86-64 ABI
 */

#ifndef _ADEAD_SYS_TYPES_H
#define _ADEAD_SYS_TYPES_H

#include "../stdint.h"

/* Process and user IDs */
typedef int32_t pid_t;
typedef uint32_t uid_t;
typedef uint32_t gid_t;

/* File system types */
typedef int64_t off_t;
typedef int64_t loff_t;
typedef uint64_t ino_t;
typedef uint32_t mode_t;
typedef uint32_t nlink_t;
typedef uint64_t dev_t;
typedef int64_t blksize_t;
typedef int64_t blkcnt_t;

/* Size types */
typedef uint64_t size_t;
typedef int64_t ssize_t;
typedef int64_t ptrdiff_t;
typedef uint64_t uintptr_t;
typedef int64_t intptr_t;

/* Time types */
typedef int64_t time_t;
typedef int64_t clock_t;
typedef int32_t suseconds_t;
typedef uint32_t useconds_t;

/* Socket types */
typedef uint32_t socklen_t;
typedef uint16_t sa_family_t;
typedef uint16_t in_port_t;
typedef uint32_t in_addr_t;

/* Key types */
typedef int32_t key_t;

/* Thread types (opaque) */
typedef unsigned long pthread_t;
typedef unsigned int pthread_key_t;
typedef int pthread_once_t;

/* Register type for setjmp/longjmp */
typedef int64_t __jmp_buf[8];

/* File descriptor set for select() */
#define FD_SETSIZE 1024

typedef struct {
    unsigned long fds_bits[FD_SETSIZE / (8 * sizeof(unsigned long))];
} fd_set;

#define FD_ZERO(set)    __builtin_memset((set), 0, sizeof(fd_set))
#define FD_SET(fd, set) ((set)->fds_bits[(fd) / 64] |= (1UL << ((fd) % 64)))
#define FD_CLR(fd, set) ((set)->fds_bits[(fd) / 64] &= ~(1UL << ((fd) % 64)))
#define FD_ISSET(fd, set) ((set)->fds_bits[(fd) / 64] & (1UL << ((fd) % 64)))

#endif /* _ADEAD_SYS_TYPES_H */
