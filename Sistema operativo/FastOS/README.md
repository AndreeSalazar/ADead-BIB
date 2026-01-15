# FastOS

**GPU-First / Binary-First Operating System (64-bit)**

> Stack: **ADead-BIB + Rust + wgpu**

---

## 🎯 Características

| Módulo | Estado | Descripción |
|--------|--------|-------------|
| **GPU Driver** | ✅ | Acceso directo al framebuffer |
| **ADead-BIB Loader** | ✅ | Carga y ejecuta binarios .adB |
| **Syscall API** | ✅ | API para programas ADead-BIB |
| **Framebuffer** | ✅ | 1280x720 gráficos |

---

## 📁 Estructura

```
FastOS/
├── kernel/
│   ├── main.rs        # Entry point
│   ├── gpu.rs         # Driver GPU (framebuffer)
│   ├── loader.rs      # Cargador de binarios ADead-BIB
│   ├── syscall.rs     # API de sistema (syscalls)
│   └── adead_bib.rs   # Definiciones ADead-BIB
├── Cargo.toml
├── make_boot.ps1      # Crear imagen booteable
└── target/
    └── fastos-bios.img
```

---

## 🚀 Compilar y Ejecutar

```powershell
cd "Sistema operativo\FastOS"

# Compilar y crear imagen
.\make_boot.ps1

# Ejecutar en QEMU
& "C:\Program Files\qemu\qemu-system-x86_64.exe" -drive format=raw,file=target\fastos-bios.img -m 128M
```

---

## 🔧 Módulos Implementados

### 1. GPU Driver (`gpu.rs`)
```rust
// Acceso directo al framebuffer
GpuDriver::init(buffer, width, height, pitch, bpp);
gpu.put_pixel(x, y, color);
gpu.draw_rect(x, y, w, h, color);
gpu.clear(color);
```

### 2. ADead-BIB Loader (`loader.rs`)
```rust
// Cargar y ejecutar binarios .adB
let loader = ADeadLoader::new(syscall_table);
let program = loader.load(binary_data)?;
loader.execute(&program)?;
```

### 3. Syscall API (`syscall.rs`)
```rust
// API para programas ADead-BIB
pub struct SyscallTable {
    pub gpu_clear: fn(color: u32),
    pub gpu_put_pixel: fn(x: u32, y: u32, color: u32),
    pub gpu_draw_rect: fn(x: u32, y: u32, w: u32, h: u32, color: u32),
    pub gpu_get_width: fn() -> u32,
    pub gpu_get_height: fn() -> u32,
    pub sys_exit: fn(code: i32) -> !,
}
```

---

## 📝 Formato Binario ADead-BIB

```
+0x00: Magic (4 bytes)     = 0xADB1B000
+0x04: Version (2 bytes)
+0x06: Flags (2 bytes)     = CPU(0x01) | GPU(0x02) | HYBRID(0x03)
+0x08: Entry Point (8 bytes)
+0x10: Code Size (8 bytes)
+0x18: Data Size (8 bytes)
+0x20: Code...
+0x20+code_size: Data...
```

---

## 🎨 Ejemplo Programa ADead-BIB

```adB
// programa.adB - Para FastOS
#![target(fastos)]

fn main(sys: *SyscallTable) -> i32 {
    // Limpiar pantalla
    sys.gpu_clear(0x000000);
    
    // Dibujar rectángulo rojo
    sys.gpu_draw_rect(100, 100, 200, 150, 0xFF0000);
    
    // Pixel verde en el centro
    let w = sys.gpu_get_width();
    let h = sys.gpu_get_height();
    sys.gpu_put_pixel(w/2, h/2, 0x00FF00);
    
    return 0;
}
```

---

## 👤 Autor

**Eddi Andreé Salazar Matos** 🇵🇪  
📧 eddi.salazar.dev@gmail.com

---

**FastOS: GPU-First. Binary-First. Sin mentiras.**
