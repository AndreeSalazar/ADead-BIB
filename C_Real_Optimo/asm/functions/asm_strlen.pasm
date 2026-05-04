; asm_strlen — Win64 fastcall ABI
; size_t asm_strlen(const char* s)  RCX = s, RAX = length
; Versión simple byte-a-byte; SSE/AVX vendrá luego.

asm_strlen PROC
    xor     rax, rax        ; counter = 0
.loop:
    cmp     BYTE PTR [rcx + rax], 0
    je      .done
    inc     rax
    jmp     .loop
.done:
    ret
asm_strlen ENDP
