# ============================================================
# FastOS v2.0 — Build System
# Target: AMD Ryzen 5 5600X (Zen3, AVX2, CSM Legacy Boot)
# Tools: NASM + x86_64-elf-gcc cross-compiler
# ============================================================

# Cross-compiler toolchain
# On Windows with cross-compiler installed:
CC      = x86_64-elf-gcc
LD      = x86_64-elf-ld
OBJCOPY = x86_64-elf-objcopy
NASM    = nasm

# Directories
LEGACY  = legacy
KDIR    = kernel
BUILD   = build

# Compiler flags — freestanding 64-bit kernel, no red zone, kernel model
CFLAGS  = -ffreestanding -nostdlib -nostdinc -mno-red-zone -mcmodel=kernel \
          -Wall -Wextra -O2 -fno-exceptions -fno-stack-protector \
          -fno-pic -fno-pie -mno-sse -mgeneral-regs-only \
          -I$(KDIR)/include

# NASM flags
NASMFLAGS = -f elf64

# Linker flags
LDFLAGS = -T kernel.ld -nostdlib -z max-page-size=4096

# ── Source Files ──

# Assembly sources
ASM_SRCS = $(KDIR)/kernel_entry.asm \
           $(KDIR)/isr.asm \
           $(KDIR)/syscall.asm \
           $(KDIR)/hal.asm

# C sources
C_SRCS   = $(KDIR)/kernel.c \
           $(KDIR)/vga.c \
           $(KDIR)/gdt.c \
           $(KDIR)/idt.c \
           $(KDIR)/pic.c \
           $(KDIR)/timer.c \
           $(KDIR)/keyboard.c \
           $(KDIR)/pmm.c \
           $(KDIR)/vmm.c \
           $(KDIR)/heap.c \
           $(KDIR)/scheduler.c \
           $(KDIR)/shell.c \
           $(KDIR)/lib/string.c \
           $(KDIR)/lib/printf.c

# Object files
ASM_OBJS = $(ASM_SRCS:%.asm=$(BUILD)/%.o)
C_OBJS   = $(C_SRCS:%.c=$(BUILD)/%.o)
OBJS     = $(ASM_OBJS) $(C_OBJS)

# ── Targets ──

.PHONY: all clean run debug

all: dirs $(BUILD)/fastos.img

dirs:
	@mkdir -p $(BUILD)/$(KDIR)/lib

# ── Boot sector + Stage2 (flat binary, NASM) ──
$(BUILD)/mbr.bin: $(LEGACY)/boot.asm
	$(NASM) -f bin $< -o $@

$(BUILD)/stage2.bin: $(LEGACY)/stage2.asm
	$(NASM) -f bin $< -o $@

# ── Kernel assembly objects ──
$(BUILD)/$(KDIR)/%.o: $(KDIR)/%.asm
	@mkdir -p $(dir $@)
	$(NASM) $(NASMFLAGS) $< -o $@

# ── Kernel C objects ──
$(BUILD)/$(KDIR)/%.o: $(KDIR)/%.c
	@mkdir -p $(dir $@)
	$(CC) $(CFLAGS) -c $< -o $@

# ── Link kernel ELF → flat binary ──
$(BUILD)/kernel.elf: $(OBJS)
	$(LD) $(LDFLAGS) -o $@ $(OBJS)

$(BUILD)/kernel.bin: $(BUILD)/kernel.elf
	$(OBJCOPY) -O binary $< $@

# ── Create bootable disk image ──
# Layout: MBR (512B) + Stage2 (16KB) + Kernel (64KB+)
# Total reserved: 512 + 16384 + padding to sector boundary
$(BUILD)/fastos.img: $(BUILD)/mbr.bin $(BUILD)/stage2.bin $(BUILD)/kernel.bin
	@echo "[IMAGE] Building fastos.img..."
	dd if=/dev/zero of=$@ bs=512 count=2880 2>/dev/null
	dd if=$(BUILD)/mbr.bin of=$@ bs=512 count=1 conv=notrunc 2>/dev/null
	dd if=$(BUILD)/stage2.bin of=$@ bs=512 seek=1 conv=notrunc 2>/dev/null
	dd if=$(BUILD)/kernel.bin of=$@ bs=512 seek=33 conv=notrunc 2>/dev/null
	@echo "[IMAGE] Done: $@"

# ── QEMU testing ──
run: $(BUILD)/fastos.img
	qemu-system-x86_64 \
		-drive file=$<,format=raw \
		-m 512M \
		-serial stdio \
		-no-reboot

debug: $(BUILD)/fastos.img
	qemu-system-x86_64 \
		-drive file=$<,format=raw \
		-m 512M \
		-serial stdio \
		-no-reboot \
		-s -S

clean:
	rm -rf $(BUILD)
