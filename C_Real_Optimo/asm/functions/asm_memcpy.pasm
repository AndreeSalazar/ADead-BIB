; asm_memcpy — Win64 fastcall
; void* asm_memcpy(void* dst, const void* src, size_t n)
; RCX = dst, RDX = src, R8 = n; preserva dst en RAX.

asm_memcpy PROC
    mov     rax, rcx        ; return value = dst
    mov     r9, r8          ; counter = n
.loop:
    test    r9, r9
    jz      .done
    mov     r10b, BYTE PTR [rdx]
    mov     BYTE PTR [rcx], r10b
    inc     rcx
    inc     rdx
    dec     r9
    jmp     .loop
.done:
    ret
asm_memcpy ENDP
