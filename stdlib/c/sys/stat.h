/*
 * ADead-BIB Standard Library
 * sys/stat.h - File Status
 * 
 * Based on: POSIX
 */

#ifndef _ADEAD_SYS_STAT_H
#define _ADEAD_SYS_STAT_H

#include "types.h"
#include "../time.h"

/* File type masks */
#define S_IFMT   0170000  /* Type mask */
#define S_IFSOCK 0140000  /* Socket */
#define S_IFLNK  0120000  /* Symbolic link */
#define S_IFREG  0100000  /* Regular file */
#define S_IFBLK  0060000  /* Block device */
#define S_IFDIR  0040000  /* Directory */
#define S_IFCHR  0020000  /* Character device */
#define S_IFIFO  0010000  /* FIFO */

/* File type test macros */
#define S_ISREG(m)  (((m) & S_IFMT) == S_IFREG)
#define S_ISDIR(m)  (((m) & S_IFMT) == S_IFDIR)
#define S_ISCHR(m)  (((m) & S_IFMT) == S_IFCHR)
#define S_ISBLK(m)  (((m) & S_IFMT) == S_IFBLK)
#define S_ISFIFO(m) (((m) & S_IFMT) == S_IFIFO)
#define S_ISLNK(m)  (((m) & S_IFMT) == S_IFLNK)
#define S_ISSOCK(m) (((m) & S_IFMT) == S_IFSOCK)

/* Permission bits */
#define S_ISUID  04000  /* Set UID */
#define S_ISGID  02000  /* Set GID */
#define S_ISVTX  01000  /* Sticky bit */

#define S_IRWXU  00700  /* Owner RWX */
#define S_IRUSR  00400  /* Owner read */
#define S_IWUSR  00200  /* Owner write */
#define S_IXUSR  00100  /* Owner execute */

#define S_IRWXG  00070  /* Group RWX */
#define S_IRGRP  00040  /* Group read */
#define S_IWGRP  00020  /* Group write */
#define S_IXGRP  00010  /* Group execute */

#define S_IRWXO  00007  /* Others RWX */
#define S_IROTH  00004  /* Others read */
#define S_IWOTH  00002  /* Others write */
#define S_IXOTH  00001  /* Others execute */

/* Stat structure */
struct stat {
    dev_t     st_dev;      /* Device ID */
    ino_t     st_ino;      /* Inode number */
    mode_t    st_mode;     /* File mode */
    nlink_t   st_nlink;    /* Number of hard links */
    uid_t     st_uid;      /* Owner UID */
    gid_t     st_gid;      /* Owner GID */
    dev_t     st_rdev;     /* Device ID (if special) */
    off_t     st_size;     /* File size */
    blksize_t st_blksize;  /* Block size */
    blkcnt_t  st_blocks;   /* Number of blocks */
    struct timespec st_atim;  /* Access time */
    struct timespec st_mtim;  /* Modification time */
    struct timespec st_ctim;  /* Status change time */
};

#define st_atime st_atim.tv_sec
#define st_mtime st_mtim.tv_sec
#define st_ctime st_ctim.tv_sec

/* Functions */
int stat(const char* path, struct stat* buf);
int fstat(int fd, struct stat* buf);
int lstat(const char* path, struct stat* buf);
int fstatat(int dirfd, const char* path, struct stat* buf, int flags);

int chmod(const char* path, mode_t mode);
int fchmod(int fd, mode_t mode);
int fchmodat(int dirfd, const char* path, mode_t mode, int flags);

int mkdir(const char* path, mode_t mode);
int mkdirat(int dirfd, const char* path, mode_t mode);

int mknod(const char* path, mode_t mode, dev_t dev);
int mknodat(int dirfd, const char* path, mode_t mode, dev_t dev);

int mkfifo(const char* path, mode_t mode);
int mkfifoat(int dirfd, const char* path, mode_t mode);

mode_t umask(mode_t mask);

#endif /* _ADEAD_SYS_STAT_H */
