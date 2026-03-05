// ============================================================
// ADead-BIB — ESPECIALES/string.h
// String operations — ADead-BIB implementation layer
// Compatible with C89/C99/C11/C++11
// ============================================================

#ifndef _ADEAD_STRING_H
#define _ADEAD_STRING_H

typedef unsigned long long size_t;
#define NULL 0

// ── Memory operations ────────────────────────────────────────
extern void*  memcpy(void* dest, const void* src, size_t n);
extern void*  memmove(void* dest, const void* src, size_t n);
extern void*  memset(void* dest, int c, size_t n);
extern int    memcmp(const void* a, const void* b, size_t n);
extern void*  memchr(const void* s, int c, size_t n);
extern void*  memrchr(const void* s, int c, size_t n);

// ── String length ────────────────────────────────────────────
extern size_t strlen(const char* s);
extern size_t strnlen(const char* s, size_t maxlen);
extern size_t wcslen(const wchar_t* s);

// ── String copy ──────────────────────────────────────────────
extern char*  strcpy(char* dest, const char* src);
extern char*  strncpy(char* dest, const char* src, size_t n);
extern char*  strdup(const char* s);
extern char*  strndup(const char* s, size_t n);

// ── String concatenation ─────────────────────────────────────
extern char*  strcat(char* dest, const char* src);
extern char*  strncat(char* dest, const char* src, size_t n);

// ── String comparison ────────────────────────────────────────
extern int    strcmp(const char* a, const char* b);
extern int    strncmp(const char* a, const char* b, size_t n);
extern int    strcasecmp(const char* a, const char* b);
extern int    strncasecmp(const char* a, const char* b, size_t n);
extern int    strcoll(const char* a, const char* b);

// ── String search ────────────────────────────────────────────
extern char*  strchr(const char* s, int c);
extern char*  strrchr(const char* s, int c);
extern char*  strstr(const char* haystack, const char* needle);
extern char*  strcasestr(const char* haystack, const char* needle);
extern char*  strpbrk(const char* s, const char* accept);
extern size_t strspn(const char* s, const char* accept);
extern size_t strcspn(const char* s, const char* reject);
extern char*  strtok(char* s, const char* delimiters);
extern char*  strtok_r(char* s, const char* delimiters, char** saveptr);

// ── Error strings ────────────────────────────────────────────
extern char*  strerror(int errnum);
extern int    strerror_r(int errnum, char* buf, size_t buflen);

// ── String transform ─────────────────────────────────────────
extern size_t strxfrm(char* dest, const char* src, size_t n);

// ── Bit/byte counting ─────────────────────────────────────────
// (extensions commonly available)
extern int    __builtin_popcount(unsigned int x);
extern int    __builtin_popcountl(unsigned long x);
extern int    __builtin_popcountll(unsigned long long x);
extern int    __builtin_clz(unsigned int x);
extern int    __builtin_ctz(unsigned int x);
extern int    __builtin_parity(unsigned int x);

// ── Useful string macros ──────────────────────────────────────
#define STR_EMPTY(s)    ((s) == NULL || (s)[0] == '\0')
#define STR_EQ(a, b)    (strcmp((a), (b)) == 0)
#define STR_STARTS(s, prefix) (strncmp((s), (prefix), strlen(prefix)) == 0)

#endif // _ADEAD_STRING_H
