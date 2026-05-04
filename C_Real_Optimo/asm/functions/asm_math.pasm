; asm_math — funciones aritméticas básicas (Win64 fastcall)

; int64_t asm_abs(int64_t x)  RCX = x
asm_abs PROC
    mov     rax, rcx
    cqo                     ; sign-extend RAX into RDX
    xor     rax, rdx
    sub     rax, rdx
    ret
asm_abs ENDP

; int64_t asm_min(int64_t a, int64_t b)  RCX, RDX
asm_min PROC
    mov     rax, rcx
    cmp     rax, rdx
    cmovg   rax, rdx
    ret
asm_min ENDP

; int64_t asm_max(int64_t a, int64_t b)  RCX, RDX
asm_max PROC
    mov     rax, rcx
    cmp     rax, rdx
    cmovl   rax, rdx
    ret
asm_max ENDP

; int64_t asm_clamp(int64_t x, int64_t lo, int64_t hi)  RCX, RDX, R8
asm_clamp PROC
    mov     rax, rcx
    cmp     rax, rdx        ; if x < lo
    cmovl   rax, rdx
    cmp     rax, r8         ; if x > hi
    cmovg   rax, r8
    ret
asm_clamp ENDP

; void asm_swap(int64_t* a, int64_t* b)  RCX, RDX
asm_swap PROC
    mov     rax, QWORD PTR [rcx]
    mov     r9,  QWORD PTR [rdx]
    mov     QWORD PTR [rdx], rax
    mov     QWORD PTR [rcx], r9
    ret
asm_swap ENDP
