/*
 * ADead-BIB Standard Library
 * dirent.h - Directory Entries
 * 
 * Based on: POSIX
 */

#ifndef _ADEAD_DIRENT_H
#define _ADEAD_DIRENT_H

#include "sys/types.h"

/* Directory entry */
struct dirent {
    ino_t d_ino;           /* Inode number */
    off_t d_off;           /* Offset to next entry */
    unsigned short d_reclen; /* Length of this record */
    unsigned char d_type;  /* File type */
    char d_name[256];      /* Filename */
};

/* File types */
#define DT_UNKNOWN  0
#define DT_FIFO     1
#define DT_CHR      2
#define DT_DIR      4
#define DT_BLK      6
#define DT_REG      8
#define DT_LNK      10
#define DT_SOCK     12
#define DT_WHT      14

/* Directory stream (opaque) */
typedef struct __dirstream DIR;

/* Functions */
DIR* opendir(const char* name);
DIR* fdopendir(int fd);
int closedir(DIR* dirp);
struct dirent* readdir(DIR* dirp);
int readdir_r(DIR* dirp, struct dirent* entry, struct dirent** result);
void rewinddir(DIR* dirp);
void seekdir(DIR* dirp, long loc);
long telldir(DIR* dirp);
int dirfd(DIR* dirp);

/* Scanning */
int scandir(const char* dirp, struct dirent*** namelist,
            int (*filter)(const struct dirent*),
            int (*compar)(const struct dirent**, const struct dirent**));
int alphasort(const struct dirent** a, const struct dirent** b);
int versionsort(const struct dirent** a, const struct dirent** b);

#endif /* _ADEAD_DIRENT_H */
