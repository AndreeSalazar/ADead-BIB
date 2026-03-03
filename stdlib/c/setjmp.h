/*
 * ADead-BIB Standard Library
 * setjmp.h - Non-local Jumps
 * 
 * Based on: C99, x86-64 ABI
 * VAX heritage: Stack frame unwinding
 */

#ifndef _ADEAD_SETJMP_H
#define _ADEAD_SETJMP_H

/* Jump buffer for x86-64 */
typedef struct {
    unsigned long __rbx;
    unsigned long __rbp;
    unsigned long __r12;
    unsigned long __r13;
    unsigned long __r14;
    unsigned long __r15;
    unsigned long __rsp;
    unsigned long __rip;
} jmp_buf[1];

/* POSIX sigjmp_buf includes signal mask */
typedef struct {
    jmp_buf __jmpbuf;
    int __mask_was_saved;
    unsigned long __saved_mask;
} sigjmp_buf[1];

/* Functions */
int setjmp(jmp_buf env);
void longjmp(jmp_buf env, int val);

int _setjmp(jmp_buf env);
void _longjmp(jmp_buf env, int val);

int sigsetjmp(sigjmp_buf env, int savemask);
void siglongjmp(sigjmp_buf env, int val);

#endif /* _ADEAD_SETJMP_H */
