; asm_memset — Win64 fastcall
; void* asm_memset(void* dst, int c, size_t n)
; RCX = dst, RDX = c (low 8 bits), R8 = n; preserva dst en RAX.

asm_memset PROC
    mov     rax, rcx        ; return value = dst
    mov     r9, r8          ; counter
.loop:
    test    r9, r9
    jz      .done
    mov     BYTE PTR [rcx], dl
    inc     rcx
    dec     r9
    jmp     .loop
.done:
    ret
asm_memset ENDP
