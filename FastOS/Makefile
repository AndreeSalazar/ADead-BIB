# FastOS v2.0 — Build System
# ADead-BIB Native Operating System
#
# Usage:
#   make all      - Build everything
#   make kernel   - Build kernel only
#   make clean    - Clean build artifacts
#   make image    - Create bootable disk image

# ADead-BIB Compiler
ADEAD = cargo run --manifest-path=../Cargo.toml --

# Directories
BOOT_DIR = boot
KERNEL_DIR = kernel
DRIVERS_DIR = drivers
LIB_DIR = lib
BUILD_DIR = build/bin
ISO_DIR = build/iso

# Output files
STAGE1 = $(BUILD_DIR)/stage1.bin
STAGE2 = $(BUILD_DIR)/stage2.bin
KERNEL = $(BUILD_DIR)/kernel.bin
FASTOS_IMG = $(BUILD_DIR)/fastos.img
FASTOS_ISO = $(BUILD_DIR)/fastos.iso

# Kernel sources
KERNEL_SRCS = \
	$(KERNEL_DIR)/main.c \
	$(LIB_DIR)/string.c \
	$(DRIVERS_DIR)/pci/pci.c \
	$(DRIVERS_DIR)/video/nouveau/nouveau.c

# Flags
CFLAGS = --flat --freestanding

.PHONY: all clean kernel bootloader image iso

all: dirs bootloader kernel image

dirs:
	@mkdir -p $(BUILD_DIR)
	@mkdir -p $(ISO_DIR)/boot

# Build bootloader (stage1 + stage2)
bootloader: $(STAGE1) $(STAGE2)

$(STAGE1): $(BOOT_DIR)/stage1.c
	@echo "[BOOT] Compiling stage1..."
	$(ADEAD) cc $< -o $@ $(CFLAGS) --org=0x7C00

$(STAGE2): $(BOOT_DIR)/stage2.c
	@echo "[BOOT] Compiling stage2..."
	$(ADEAD) cc $< -o $@ $(CFLAGS) --org=0x8000

# Build kernel
kernel: $(KERNEL)

$(KERNEL): $(KERNEL_SRCS)
	@echo "[KERNEL] Compiling kernel..."
	$(ADEAD) cc $(KERNEL_DIR)/main.c -o $@ $(CFLAGS) --org=0x100000

# Create disk image (1.44MB floppy)
image: bootloader kernel
	@echo "[IMAGE] Creating disk image..."
	@dd if=/dev/zero of=$(FASTOS_IMG) bs=512 count=2880 2>/dev/null
	@dd if=$(STAGE1) of=$(FASTOS_IMG) bs=512 count=1 conv=notrunc 2>/dev/null
	@dd if=$(STAGE2) of=$(FASTOS_IMG) bs=512 seek=1 conv=notrunc 2>/dev/null
	@dd if=$(KERNEL) of=$(FASTOS_IMG) bs=512 seek=20 conv=notrunc 2>/dev/null
	@echo "[IMAGE] Created $(FASTOS_IMG)"

# Create ISO image (for CD boot)
iso: image
	@echo "[ISO] Creating ISO image..."
	@cp $(FASTOS_IMG) $(ISO_DIR)/boot/fastos.img
	@# Would use mkisofs/xorriso here
	@echo "[ISO] ISO creation requires external tools"

# Run in QEMU
run: image
	@echo "[QEMU] Starting FastOS..."
	qemu-system-x86_64 -drive format=raw,file=$(FASTOS_IMG) -m 256M

# Run with KVM acceleration
run-kvm: image
	qemu-system-x86_64 -drive format=raw,file=$(FASTOS_IMG) -m 256M -enable-kvm

# Debug with QEMU
debug: image
	qemu-system-x86_64 -drive format=raw,file=$(FASTOS_IMG) -m 256M -s -S

# Clean build artifacts
clean:
	@echo "[CLEAN] Removing build artifacts..."
	@rm -rf $(BUILD_DIR)/*
	@rm -rf $(ISO_DIR)/*
	@echo "[CLEAN] Done"

# Help
help:
	@echo "FastOS v2.0 Build System"
	@echo ""
	@echo "Targets:"
	@echo "  all       - Build everything"
	@echo "  kernel    - Build kernel only"
	@echo "  bootloader- Build bootloader only"
	@echo "  image     - Create disk image"
	@echo "  iso       - Create ISO image"
	@echo "  run       - Run in QEMU"
	@echo "  run-kvm   - Run in QEMU with KVM"
	@echo "  debug     - Run in QEMU debug mode"
	@echo "  clean     - Clean build artifacts"
	@echo "  help      - Show this help"
