/*
 * ADead-BIB Standard Library
 * ctype.h - Character Classification
 */

#ifndef _ADEAD_CTYPE_H
#define _ADEAD_CTYPE_H

/* Character classification */
int isalnum(int c);
int isalpha(int c);
int isblank(int c);
int iscntrl(int c);
int isdigit(int c);
int isgraph(int c);
int islower(int c);
int isprint(int c);
int ispunct(int c);
int isspace(int c);
int isupper(int c);
int isxdigit(int c);

/* Character conversion */
int tolower(int c);
int toupper(int c);

/* Inline implementations for performance */
#define _CTYPE_U  0x01  /* Upper */
#define _CTYPE_L  0x02  /* Lower */
#define _CTYPE_D  0x04  /* Digit */
#define _CTYPE_S  0x08  /* Space */
#define _CTYPE_P  0x10  /* Punct */
#define _CTYPE_C  0x20  /* Control */
#define _CTYPE_X  0x40  /* Hex digit */
#define _CTYPE_B  0x80  /* Blank */

extern const unsigned char __ctype_table[256];

#define isalpha(c)  (__ctype_table[(unsigned char)(c)] & (_CTYPE_U | _CTYPE_L))
#define isupper(c)  (__ctype_table[(unsigned char)(c)] & _CTYPE_U)
#define islower(c)  (__ctype_table[(unsigned char)(c)] & _CTYPE_L)
#define isdigit(c)  (__ctype_table[(unsigned char)(c)] & _CTYPE_D)
#define isxdigit(c) (__ctype_table[(unsigned char)(c)] & _CTYPE_X)
#define isspace(c)  (__ctype_table[(unsigned char)(c)] & _CTYPE_S)
#define ispunct(c)  (__ctype_table[(unsigned char)(c)] & _CTYPE_P)
#define isalnum(c)  (__ctype_table[(unsigned char)(c)] & (_CTYPE_U | _CTYPE_L | _CTYPE_D))
#define isprint(c)  (__ctype_table[(unsigned char)(c)] & (_CTYPE_U | _CTYPE_L | _CTYPE_D | _CTYPE_P | _CTYPE_B))
#define isgraph(c)  (__ctype_table[(unsigned char)(c)] & (_CTYPE_U | _CTYPE_L | _CTYPE_D | _CTYPE_P))
#define iscntrl(c)  (__ctype_table[(unsigned char)(c)] & _CTYPE_C)
#define isblank(c)  (__ctype_table[(unsigned char)(c)] & _CTYPE_B)

#define tolower(c)  (isupper(c) ? (c) + 32 : (c))
#define toupper(c)  (islower(c) ? (c) - 32 : (c))

#endif /* _ADEAD_CTYPE_H */
