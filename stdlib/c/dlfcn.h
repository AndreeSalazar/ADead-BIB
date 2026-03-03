/*
 * ADead-BIB Standard Library
 * dlfcn.h - Dynamic Linking
 * 
 * Based on: POSIX, musl libc
 */

#ifndef _ADEAD_DLFCN_H
#define _ADEAD_DLFCN_H

/* dlopen flags */
#define RTLD_LAZY     0x00001
#define RTLD_NOW      0x00002
#define RTLD_NOLOAD   0x00004
#define RTLD_DEEPBIND 0x00008
#define RTLD_GLOBAL   0x00100
#define RTLD_LOCAL    0x00000
#define RTLD_NODELETE 0x01000

/* Special handles */
#define RTLD_DEFAULT  ((void*)0)
#define RTLD_NEXT     ((void*)-1)

/* Dl_info for dladdr */
typedef struct {
    const char* dli_fname;
    void* dli_fbase;
    const char* dli_sname;
    void* dli_saddr;
} Dl_info;

/* Functions */
void* dlopen(const char* filename, int flags);
int dlclose(void* handle);
void* dlsym(void* handle, const char* symbol);
char* dlerror(void);
int dladdr(const void* addr, Dl_info* info);

/* GNU extensions */
void* dlvsym(void* handle, const char* symbol, const char* version);

#endif /* _ADEAD_DLFCN_H */
