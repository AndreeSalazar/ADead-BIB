# ============================================================================
# FastOS Makefile
# ============================================================================
# Build system para FastOS
#
# Author: Eddi Andreé Salazar Matos
# ============================================================================

.PHONY: all clean run debug

# Herramientas
NASM = nasm
CARGO = cargo
QEMU = qemu-system-x86_64

# Archivos
BOOTLOADER = boot/bootloader.asm
BOOTLOADER_BIN = build/bootloader.bin
KERNEL_BIN = build/kernel.bin
IMAGE = build/fastos.img

# Directorios
BUILD_DIR = build

all: $(IMAGE)

# Crear directorio de build
$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

# Compilar bootloader
$(BOOTLOADER_BIN): $(BOOTLOADER) | $(BUILD_DIR)
	$(NASM) -f bin $< -o $@

# Compilar kernel con Cargo
$(KERNEL_BIN): | $(BUILD_DIR)
	$(CARGO) build --release
	cp target/x86_64-fastos/release/fastos-kernel $@

# Crear imagen de disco
$(IMAGE): $(BOOTLOADER_BIN) $(KERNEL_BIN)
	# Crear imagen de 1.44MB (floppy)
	dd if=/dev/zero of=$@ bs=512 count=2880
	# Escribir bootloader en sector 0
	dd if=$(BOOTLOADER_BIN) of=$@ conv=notrunc
	# Escribir kernel a partir del sector 1
	dd if=$(KERNEL_BIN) of=$@ bs=512 seek=1 conv=notrunc

# Ejecutar en QEMU
run: $(IMAGE)
	$(QEMU) -drive format=raw,file=$(IMAGE) -m 128M

# Debug con GDB
debug: $(IMAGE)
	$(QEMU) -drive format=raw,file=$(IMAGE) -m 128M -s -S

# Limpiar
clean:
	rm -rf $(BUILD_DIR)
	$(CARGO) clean
