; asm_bit — operaciones de bits (Win64 fastcall)

; uint32_t asm_popcount(uint64_t x)  RCX = x
asm_popcount PROC
    popcnt  rax, rcx
    ret
asm_popcount ENDP

; uint32_t asm_bsr64(uint64_t x)  RCX = x; bit position of MSB or -1 if 0
asm_bsr64 PROC
    bsr     rax, rcx
    ret
asm_bsr64 ENDP

; uint32_t asm_bsf64(uint64_t x)  RCX = x; bit position of LSB or -1 if 0
asm_bsf64 PROC
    bsf     rax, rcx
    ret
asm_bsf64 ENDP

; uint32_t asm_bswap32(uint32_t x)  RCX low 32 bits
asm_bswap32 PROC
    mov     eax, ecx
    bswap   eax
    ret
asm_bswap32 ENDP

; uint64_t asm_bswap64(uint64_t x)  RCX
asm_bswap64 PROC
    mov     rax, rcx
    bswap   rax
    ret
asm_bswap64 ENDP

; int asm_is_aligned(void* p, size_t alignment)  RCX, RDX
asm_is_aligned PROC
    mov     rax, rcx
    dec     rdx             ; mask = align - 1
    test    rax, rdx
    setz    al
    movzx   eax, al
    ret
asm_is_aligned ENDP

; size_t asm_align_up(size_t v, size_t align)  RCX, RDX
asm_align_up PROC
    mov     rax, rcx
    add     rax, rdx
    dec     rax
    not     rdx
    and     rax, rdx
    ret
asm_align_up ENDP

; void asm_noop(void)
asm_noop PROC
    nop
    ret
asm_noop ENDP
