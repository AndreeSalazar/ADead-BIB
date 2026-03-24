; ============================================================
; FastOS v2.0 — Hardware Abstraction Layer (NASM, 64-bit)
; Implements all hardware intrinsics as callable functions.
; ABI: System V AMD64 (RDI=arg1, RSI=arg2, RDX=arg3, RAX=return)
;
; Linked with C code compiled by ADead-BIB compiler.
; ============================================================

[BITS 64]

section .text

; ============================================================
; Port I/O
; ============================================================

; void hal_outb(uint16_t port, uint8_t val)
;   RDI = port, RSI = val
global hal_outb
hal_outb:
    mov  dx, di        ; port -> DX
    mov  al, sil       ; val  -> AL
    out  dx, al
    ret

; uint8_t hal_inb(uint16_t port)
;   RDI = port, return AL
global hal_inb
hal_inb:
    mov  dx, di
    xor  eax, eax
    in   al, dx
    ret

; void hal_outw(uint16_t port, uint16_t val)
;   RDI = port, RSI = val
global hal_outw
hal_outw:
    mov  dx, di
    mov  ax, si
    out  dx, ax
    ret

; uint16_t hal_inw(uint16_t port)
;   RDI = port, return AX
global hal_inw
hal_inw:
    mov  dx, di
    xor  eax, eax
    in   ax, dx
    ret

; void hal_outl(uint16_t port, uint32_t val)
;   RDI = port, RSI = val
global hal_outl
hal_outl:
    mov  dx, di
    mov  eax, esi
    out  dx, eax
    ret

; uint32_t hal_inl(uint16_t port)
;   RDI = port, return EAX
global hal_inl
hal_inl:
    mov  dx, di
    in   eax, dx
    ret

; void hal_io_wait(void)
;   Write to port 0x80 (POST diagnostic, safe no-op delay)
global hal_io_wait
hal_io_wait:
    mov  al, 0
    out  0x80, al
    ret

; ============================================================
; CPU Control
; ============================================================

; void hal_cli(void)
global hal_cli
hal_cli:
    cli
    ret

; void hal_sti(void)
global hal_sti
hal_sti:
    sti
    ret

; void hal_hlt(void)
global hal_hlt
hal_hlt:
    hlt
    ret

; void hal_pause(void)
global hal_pause
hal_pause:
    pause
    ret

; void hal_int3(void)
global hal_int3
hal_int3:
    int 0x03
    ret

; ============================================================
; Memory Barriers
; ============================================================

; void hal_mfence(void)
global hal_mfence
hal_mfence:
    mfence
    ret

; void hal_lfence(void)
global hal_lfence
hal_lfence:
    lfence
    ret

; void hal_sfence(void)
global hal_sfence
hal_sfence:
    sfence
    ret

; ============================================================
; Timestamp Counter
; ============================================================

; uint64_t hal_rdtsc(void)
;   Returns 64-bit TSC in RAX
global hal_rdtsc
hal_rdtsc:
    rdtsc
    shl  rdx, 32
    or   rax, rdx
    ret

; ============================================================
; Model-Specific Registers
; ============================================================

; uint64_t hal_rdmsr(uint32_t msr)
;   RDI = msr (-> ECX), return RAX = EDX:EAX combined
global hal_rdmsr
hal_rdmsr:
    mov  ecx, edi
    rdmsr
    shl  rdx, 32
    or   rax, rdx
    ret

; void hal_wrmsr(uint32_t msr, uint64_t val)
;   RDI = msr, RSI = val
global hal_wrmsr
hal_wrmsr:
    mov  ecx, edi
    mov  eax, esi       ; low 32 bits
    mov  rdx, rsi
    shr  rdx, 32        ; high 32 bits
    wrmsr
    ret

; ============================================================
; CPUID
; ============================================================

; void hal_cpuid(uint32_t leaf, uint32_t *eax, uint32_t *ebx,
;                uint32_t *ecx, uint32_t *edx)
;   RDI = leaf, RSI = &eax, RDX = &ebx, RCX = &ecx, R8 = &edx
global hal_cpuid
hal_cpuid:
    push rbx            ; cpuid clobbers EBX
    mov  r9, rdx        ; save &ebx (RDX is clobbered by cpuid)
    mov  r10, rcx       ; save &ecx
    mov  eax, edi       ; leaf
    xor  ecx, ecx       ; sub-leaf = 0
    cpuid
    mov  [rsi], eax     ; *eax_out = EAX
    mov  [r9],  ebx     ; *ebx_out = EBX
    mov  [r10], ecx     ; *ecx_out = ECX
    mov  [r8],  edx     ; *edx_out = EDX
    pop  rbx
    ret

; ============================================================
; Control Registers
; ============================================================

; uint64_t hal_read_cr0(void)
global hal_read_cr0
hal_read_cr0:
    mov  rax, cr0
    ret

; uint64_t hal_read_cr2(void)
global hal_read_cr2
hal_read_cr2:
    mov  rax, cr2
    ret

; uint64_t hal_read_cr3(void)
global hal_read_cr3
hal_read_cr3:
    mov  rax, cr3
    ret

; void hal_write_cr3(uint64_t val)
;   RDI = val
global hal_write_cr3
hal_write_cr3:
    mov  cr3, rdi
    ret

; uint64_t hal_read_cr4(void)
global hal_read_cr4
hal_read_cr4:
    mov  rax, cr4
    ret

; void hal_write_cr4(uint64_t val)
;   RDI = val
global hal_write_cr4
hal_write_cr4:
    mov  cr4, rdi
    ret

; ============================================================
; TLB Management
; ============================================================

; void hal_invlpg(void *addr)
;   RDI = addr
global hal_invlpg
hal_invlpg:
    invlpg [rdi]
    ret

; void hal_flush_tlb(void)
;   Reload CR3 to flush entire TLB
global hal_flush_tlb
hal_flush_tlb:
    mov  rax, cr3
    mov  cr3, rax
    ret

; ============================================================
; Descriptor Tables
; ============================================================

; void hal_lgdt(void *gdtr)
;   RDI = pointer to 10-byte GDTR struct
global hal_lgdt
hal_lgdt:
    lgdt [rdi]
    ret

; void hal_lidt(void *idtr)
;   RDI = pointer to 10-byte IDTR struct
global hal_lidt
hal_lidt:
    lidt [rdi]
    ret

; void hal_ltr(uint16_t sel)
;   RDI = selector (low 16 bits)
global hal_ltr
hal_ltr:
    mov  ax, di
    ltr  ax
    ret

; ============================================================
; Segment Reload
; ============================================================

; void hal_reload_segments(void)
;   Reloads DS, ES, FS, GS, SS with 0x10 (kernel data selector)
global hal_reload_segments
hal_reload_segments:
    mov  ax, 0x10
    mov  ds, ax
    mov  es, ax
    mov  fs, ax
    mov  gs, ax
    mov  ss, ax
    ret
