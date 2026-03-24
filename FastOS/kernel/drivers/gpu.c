// ============================================================
// FastOS GPU Driver — RTX 3060 via PCI
// ============================================================
// Comunicación directa CPU ↔ GPU sin drivers NVIDIA
// PCI BAR0 = MMIO registers
// PCI BAR1 = VRAM (framebuffer)
// ============================================================

#include "../include/types.h"

// ============================================================
// PCI Configuration Space
// ============================================================
#define PCI_CONFIG_ADDR  0xCF8
#define PCI_CONFIG_DATA  0xCFC

// NVIDIA Vendor ID
#define NVIDIA_VENDOR_ID 0x10DE

// RTX 3060 Device IDs (GA106)
#define RTX_3060_DEVICE_ID_1 0x2503
#define RTX_3060_DEVICE_ID_2 0x2504

// PCI Configuration Registers
#define PCI_VENDOR_ID    0x00
#define PCI_DEVICE_ID    0x02
#define PCI_COMMAND      0x04
#define PCI_STATUS       0x06
#define PCI_CLASS        0x0A
#define PCI_BAR0         0x10
#define PCI_BAR1         0x14
#define PCI_BAR2         0x18
#define PCI_BAR3         0x1C

// PCI Command bits
#define PCI_CMD_IO       0x01
#define PCI_CMD_MEMORY   0x02
#define PCI_CMD_MASTER   0x04

// ============================================================
// GPU State
// ============================================================
typedef struct {
    u8 bus;
    u8 device;
    u8 function;
    u16 vendor_id;
    u16 device_id;
    u64 bar0_base;      // MMIO registers
    u64 bar1_base;      // VRAM / Framebuffer
    u64 bar0_size;
    u64 bar1_size;
    u32 vram_mb;
    u8 detected;
} GpuDevice;

static GpuDevice gpu = {0};

// ============================================================
// Port I/O
// ============================================================
static inline void outl(u16 port, u32 value) {
    __asm__ volatile ("outl %0, %1" : : "a"(value), "Nd"(port));
}

static inline u32 inl(u16 port) {
    u32 value;
    __asm__ volatile ("inl %1, %0" : "=a"(value) : "Nd"(port));
    return value;
}

// ============================================================
// PCI Functions
// ============================================================
static u32 pci_read32(u8 bus, u8 device, u8 func, u8 offset) {
    u32 address = (1 << 31) |
                  ((u32)bus << 16) |
                  ((u32)device << 11) |
                  ((u32)func << 8) |
                  (offset & 0xFC);
    outl(PCI_CONFIG_ADDR, address);
    return inl(PCI_CONFIG_DATA);
}

static void pci_write32(u8 bus, u8 device, u8 func, u8 offset, u32 value) {
    u32 address = (1 << 31) |
                  ((u32)bus << 16) |
                  ((u32)device << 11) |
                  ((u32)func << 8) |
                  (offset & 0xFC);
    outl(PCI_CONFIG_ADDR, address);
    outl(PCI_CONFIG_DATA, value);
}

static u16 pci_read16(u8 bus, u8 device, u8 func, u8 offset) {
    u32 val = pci_read32(bus, device, func, offset & 0xFC);
    return (val >> ((offset & 2) * 8)) & 0xFFFF;
}

// ============================================================
// GPU Detection
// ============================================================
int gpu_detect(void) {
    // Scan PCI bus for NVIDIA GPU
    for (u8 bus = 0; bus < 255; bus++) {
        for (u8 dev = 0; dev < 32; dev++) {
            u16 vendor = pci_read16(bus, dev, 0, PCI_VENDOR_ID);
            
            if (vendor == NVIDIA_VENDOR_ID) {
                u16 device_id = pci_read16(bus, dev, 0, PCI_DEVICE_ID);
                
                // Found NVIDIA GPU
                gpu.bus = bus;
                gpu.device = dev;
                gpu.function = 0;
                gpu.vendor_id = vendor;
                gpu.device_id = device_id;
                
                // Read BARs
                u32 bar0 = pci_read32(bus, dev, 0, PCI_BAR0);
                u32 bar1 = pci_read32(bus, dev, 0, PCI_BAR1);
                
                // BAR0 is MMIO (memory mapped I/O)
                if ((bar0 & 0x1) == 0) {
                    // Memory BAR
                    if ((bar0 & 0x6) == 0x4) {
                        // 64-bit BAR
                        u32 bar0_high = pci_read32(bus, dev, 0, PCI_BAR0 + 4);
                        gpu.bar0_base = ((u64)bar0_high << 32) | (bar0 & 0xFFFFFFF0);
                    } else {
                        gpu.bar0_base = bar0 & 0xFFFFFFF0;
                    }
                }
                
                // BAR1 is VRAM
                if ((bar1 & 0x1) == 0) {
                    if ((bar1 & 0x6) == 0x4) {
                        u32 bar1_high = pci_read32(bus, dev, 0, PCI_BAR1 + 4);
                        gpu.bar1_base = ((u64)bar1_high << 32) | (bar1 & 0xFFFFFFF0);
                    } else {
                        gpu.bar1_base = bar1 & 0xFFFFFFF0;
                    }
                }
                
                // Enable bus mastering and memory access
                u16 cmd = pci_read16(bus, dev, 0, PCI_COMMAND);
                cmd |= PCI_CMD_MEMORY | PCI_CMD_MASTER;
                pci_write32(bus, dev, 0, PCI_COMMAND, cmd);
                
                gpu.detected = 1;
                
                // Estimate VRAM (RTX 3060 = 12GB)
                if (device_id == RTX_3060_DEVICE_ID_1 || device_id == RTX_3060_DEVICE_ID_2) {
                    gpu.vram_mb = 12288; // 12 GB
                } else {
                    gpu.vram_mb = 8192;  // Default 8 GB
                }
                
                return 1;
            }
        }
    }
    
    return 0;
}

// ============================================================
// GPU Info
// ============================================================
void gpu_print_info(void) {
    if (!gpu.detected) {
        // Print to VGA text mode
        volatile u16* vga = (volatile u16*)0xB8000;
        const char* msg = "[GPU] No NVIDIA GPU detected";
        for (int i = 0; msg[i]; i++) {
            vga[24*80 + i] = 0x0C00 | msg[i]; // Red
        }
        return;
    }
    
    // Print GPU info to VGA
    volatile u16* vga = (volatile u16*)0xB8000;
    int row = 22;
    int col = 0;
    
    // GPU header
    const char* hdr = "[GPU] NVIDIA RTX 3060 12GB";
    for (int i = 0; hdr[i]; i++) {
        vga[row*80 + col++] = 0x0A00 | hdr[i]; // Green
    }
    
    row++;
    col = 0;
    
    // BAR0
    const char* bar0_msg = "[GPU] BAR0 (MMIO): 0x";
    for (int i = 0; bar0_msg[i]; i++) {
        vga[row*80 + col++] = 0x0700 | bar0_msg[i];
    }
    
    // Print hex address
    u64 addr = gpu.bar0_base;
    for (int i = 15; i >= 0; i--) {
        int nibble = (addr >> (i * 4)) & 0xF;
        char c = nibble < 10 ? '0' + nibble : 'A' + nibble - 10;
        vga[row*80 + col++] = 0x0E00 | c; // Yellow
    }
    
    row++;
    col = 0;
    
    // BAR1
    const char* bar1_msg = "[GPU] BAR1 (VRAM): 0x";
    for (int i = 0; bar1_msg[i]; i++) {
        vga[row*80 + col++] = 0x0700 | bar1_msg[i];
    }
    
    addr = gpu.bar1_base;
    for (int i = 15; i >= 0; i--) {
        int nibble = (addr >> (i * 4)) & 0xF;
        char c = nibble < 10 ? '0' + nibble : 'A' + nibble - 10;
        vga[row*80 + col++] = 0x0E00 | c;
    }
}

// ============================================================
// MMIO Access
// ============================================================
static inline void gpu_write32(u64 offset, u32 value) {
    if (!gpu.detected) return;
    volatile u32* reg = (volatile u32*)(gpu.bar0_base + offset);
    *reg = value;
}

static inline u32 gpu_read32(u64 offset) {
    if (!gpu.detected) return 0;
    volatile u32* reg = (volatile u32*)(gpu.bar0_base + offset);
    return *reg;
}

// ============================================================
// VRAM Access (Framebuffer)
// ============================================================
void gpu_vram_write32(u64 offset, u32 value) {
    if (!gpu.detected) return;
    volatile u32* vram = (volatile u32*)(gpu.bar1_base + offset);
    *vram = value;
}

u32 gpu_vram_read32(u64 offset) {
    if (!gpu.detected) return 0;
    volatile u32* vram = (volatile u32*)(gpu.bar1_base + offset);
    return *vram;
}

// ============================================================
// GPU Framebuffer Drawing
// ============================================================

// Set pixel in VRAM (assuming linear framebuffer)
void gpu_set_pixel(u32 x, u32 y, u32 color) {
    if (!gpu.detected) return;
    
    // Assuming 1920x1080 resolution, 32bpp
    u32 pitch = 1920 * 4; // bytes per row
    u64 offset = y * pitch + x * 4;
    
    gpu_vram_write32(offset, color);
}

// Fill rectangle
void gpu_fill_rect(u32 x, u32 y, u32 w, u32 h, u32 color) {
    if (!gpu.detected) return;
    
    for (u32 py = y; py < y + h; py++) {
        for (u32 px = x; px < x + w; px++) {
            gpu_set_pixel(px, py, color);
        }
    }
}

// Draw horizontal line
void gpu_hline(u32 x, u32 y, u32 len, u32 color) {
    for (u32 i = 0; i < len; i++) {
        gpu_set_pixel(x + i, y, color);
    }
}

// Draw vertical line
void gpu_vline(u32 x, u32 y, u32 len, u32 color) {
    for (u32 i = 0; i < len; i++) {
        gpu_set_pixel(x, y + i, color);
    }
}

// Draw rectangle outline
void gpu_rect(u32 x, u32 y, u32 w, u32 h, u32 color) {
    gpu_hline(x, y, w, color);
    gpu_hline(x, y + h - 1, w, color);
    gpu_vline(x, y, h, color);
    gpu_vline(x + w - 1, y, h, color);
}

// ============================================================
// GPU Initialization
// ============================================================
int gpu_init(void) {
    if (!gpu_detect()) {
        return 0;
    }
    
    gpu_print_info();
    
    return 1;
}

// ============================================================
// GPU Test Pattern
// ============================================================
void gpu_test_pattern(void) {
    if (!gpu.detected) return;
    
    // Draw colored rectangles
    gpu_fill_rect(100, 100, 200, 150, 0x00FF0000); // Red
    gpu_fill_rect(350, 100, 200, 150, 0x0000FF00); // Green
    gpu_fill_rect(600, 100, 200, 150, 0x000000FF); // Blue
    
    // Draw white border
    gpu_rect(100, 100, 200, 150, 0x00FFFFFF);
    gpu_rect(350, 100, 200, 150, 0x00FFFFFF);
    gpu_rect(600, 100, 200, 150, 0x00FFFFFF);
}

// ============================================================
// GPU Getters
// ============================================================
u64 gpu_get_bar0(void) { return gpu.bar0_base; }
u64 gpu_get_bar1(void) { return gpu.bar1_base; }
u32 gpu_get_vram_mb(void) { return gpu.vram_mb; }
u8 gpu_is_detected(void) { return gpu.detected; }
u16 gpu_get_device_id(void) { return gpu.device_id; }
