# BG — Binary Guardian: Ideas de Evolución
## Hacia un Guardian Determinista Ultra-Robusto de Hardware

> **Principio fundamental:** Todo lo que BG detecta es estructural y matemático.
> Si una instrucción accede a hardware, BG lo sabe por lo que la instrucción **ES**, no por lo que **parece**.
> Mismo binario + misma policy = mismo veredicto. Siempre.

---

## 1. Cobertura Completa de Hardware PC

### 1.1 CPU — Control Total del Procesador
**Estado actual:** ✅ Parcial (rings, CR, MSR, descriptor tables)

**Mejoras:**
- [ ] **CPUID fingerprinting map** — Catalogar qué features del CPU consulta el binario (CPUID hojas/sub-hojas). Determinista: cada `CPUID` con `EAX=leaf` se registra.
- [ ] **TSC access control** — Detectar `RDTSC`/`RDTSCP` (usados para timing attacks y anti-debugging). Clasificar como Restricted.
- [ ] **Debug register access** — Detectar `MOV DR0-DR7` (usados para hardware breakpoints, anti-debug).
- [ ] **XSAVE/XRSTOR state control** — Detectar manipulación del estado extendido del CPU (FPU, SSE, AVX).
- [ ] **SMM (System Management Mode)** — Detectar instrucciones que intentan entrar en SMM (`RSM`, secuencias de SMI).
- [ ] **VMX/SVM detection** — Detectar instrucciones de virtualización (`VMXON`, `VMLAUNCH`, `VMCALL`, `VMSAVE`). Mapear como capacidad hypervisor.

### 1.2 Memoria — Protección Completa de RAM
**Estado actual:** ✅ Parcial (RWX, self-modifying, regiones)

**Mejoras:**
- [ ] **Page table manipulation map** — Detectar escrituras a estructuras de page tables (CR3 write + secuencias de setup de PML4/PDPT/PD/PT).
- [ ] **Stack pivot detection** — Detectar cambios de RSP a direcciones fuera del stack legítimo (mov rsp, reg con valor no-stack).
- [ ] **Heap spray patterns** — Detectar loops que escriben patrones repetitivos a memoria (determinista: loop + write constante detectable en IR).
- [ ] **NX/XD bit enforcement** — Validar que las secciones de datos nunca tienen bit execute en su mapa de memoria.
- [ ] **ASLR compatibility check** — Verificar que el binario usa relocations (PE: reloc table presente, ELF: PIE flag).
- [ ] **Guard page validation** — Verificar presencia de guard pages entre stack frames (PE: IMAGE_SCN_MEM_GUARD).

### 1.3 IO Ports — Control de Puertos de Hardware
**Estado actual:** ✅ Parcial (IN/OUT detección, whitelist de puertos)

**Mejoras:**
- [ ] **Port range classification** — Mapear cada puerto a su dispositivo de hardware:
  - `0x00-0x0F` → DMA Controller 1
  - `0x20-0x21` → PIC Master (8259A)
  - `0xA0-0xA1` → PIC Slave
  - `0x40-0x43` → PIT (Timer 8254)
  - `0x60-0x64` → Keyboard/PS2 Controller (8042)
  - `0x70-0x71` → CMOS/RTC
  - `0x80` → POST Diagnostic
  - `0x92` → Fast A20 Gate
  - `0x170-0x177` → IDE Secondary
  - `0x1F0-0x1F7` → IDE Primary
  - `0x2F8-0x2FF` → COM2
  - `0x3F8-0x3FF` → COM1
  - `0x3B0-0x3DF` → VGA
  - `0xCF8-0xCFF` → PCI Configuration Space
- [ ] **Device-level policy** — Policy que dice "este driver solo puede tocar puertos de teclado (0x60-0x64)".
- [ ] **Dynamic port detection** — Cuando el puerto viene de `DX`, rastrear si DX fue cargado con un valor conocido (propagación de constantes simple).

### 1.4 DMA — Acceso Directo a Memoria
**Mejoras:**
- [ ] **DMA controller IO map** — Detectar acceso a puertos del DMA controller (0x00-0x0F, 0x80-0x8F, 0xC0-0xDF).
- [ ] **IOMMU/VT-d awareness** — Detectar configuración de IOMMU via MSRs y MMIO.
- [ ] **Bus mastering detection** — Detectar configuración de bus mastering en PCI config space (port 0xCF8/0xCFC).

### 1.5 PCI/PCIe — Control de Dispositivos
**Mejoras:**
- [ ] **PCI config space access** — Detectar lecturas/escrituras a `0xCF8` (address) y `0xCFC` (data). Extraer BDF (Bus:Device:Function) del valor escrito.
- [ ] **MMIO region detection** — Catalogar accesos a regiones de memoria mapeadas a dispositivos (detectables via BAR configuration).
- [ ] **PCIe extended config** — Detectar accesos a ECAM (Enhanced Configuration Access Mechanism).

### 1.6 Interrupciones — Control de IRQ
**Estado actual:** ✅ Parcial (CLI/STI, INT vectors)

**Mejoras:**
- [ ] **APIC configuration detection** — Detectar escrituras a MMIO del Local APIC (0xFEE00000+) y IO APIC (0xFEC00000+).
- [ ] **IDT manipulation tracking** — Ya detectamos LIDT; añadir tracking de qué vectores se configuran después.
- [ ] **NMI handling** — Detectar manipulación del puerto NMI (0x61, 0x70 bit 7).
- [ ] **Interrupt routing map** — Construir mapa completo de qué vectores de interrupción configura el binario.

### 1.7 Storage — Protección de Almacenamiento
**Mejoras:**
- [ ] **IDE/ATA direct access** — Detectar acceso a puertos IDE (0x1F0-0x1F7, 0x170-0x177). Un binario user-space NUNCA debería tocar estos puertos.
- [ ] **AHCI MMIO detection** — Detectar accesos a regiones AHCI (HBA MMIO).
- [ ] **NVMe doorbell detection** — Detectar escrituras a NVMe doorbells.
- [ ] **Disk sector policy** — Para drivers, policy que limita qué sectores pueden escribir.

### 1.8 Display/GPU — Control de Video
**Mejoras:**
- [ ] **VGA register access** — Detectar acceso a puertos VGA (0x3B0-0x3DF).
- [ ] **VBE/VESA detection** — Detectar INT 0x10 con funciones VESA (AH=0x4F).
- [ ] **Framebuffer write detection** — Detectar escrituras a regiones de framebuffer (0xA0000-0xBFFFF para legacy VGA).
- [ ] **GPU command buffer analysis** — Para drivers que escriben command buffers de GPU.

### 1.9 Red — Control de Hardware de Red
**Mejoras:**
- [ ] **NIC register detection** — Detectar acceso a puertos/MMIO de NICs comunes (Intel e1000, Realtek RTL8139).
- [ ] **Packet injection detection** — Detectar patrones de escritura directa a buffers de transmisión de NIC.
- [ ] **Network import categorization** — Ya implementado en `ImportExportMap`. Expandir con más APIs específicas.

### 1.10 USB — Control de Bus Serie Universal
**Mejoras:**
- [ ] **UHCI/OHCI/EHCI/xHCI detection** — Detectar acceso a registros de controladores USB.
- [ ] **USB descriptor manipulation** — Detectar construcción de descriptores USB (potencial BadUSB).

### 1.11 Audio — Control de Hardware de Audio
**Mejoras:**
- [ ] **HD Audio MMIO** — Detectar acceso a registros de HD Audio controller.
- [ ] **AC'97 IO ports** — Detectar acceso a puertos AC'97.

### 1.12 Firmware/BIOS — Protección de Firmware
**Mejoras:**
- [ ] **SPI flash access** — Detectar secuencias de acceso a SPI flash controller (escritura a firmware).
- [ ] **SMBios access** — Detectar lectura de tablas SMBIOS.
- [ ] **ACPI table manipulation** — Detectar acceso a regiones de tablas ACPI.

---

## 2. Mejoras al Motor de Análisis

### 2.1 Análisis de Flujo de Datos (Determinista)
- [ ] **Constant propagation** — Propagar valores constantes a través de la IR para resolver puertos IO dinámicos, vectores de interrupción dinámicos, y direcciones de memoria fijas.
  ```
  mov dx, 0x3F8    ; DX = 0x3F8 (COM1)
  out dx, al       ; BG ahora sabe: port = 0x3F8
  ```
- [ ] **Register state tracking** — Mantener estado abstracto de registros durante el análisis para resolver valores.
- [ ] **Dead code elimination** — Identificar código inalcanzable (sin predecessors en el CFG) y excluirlo del análisis.

### 2.2 Control Flow Graph (CFG)
- [ ] **Basic block construction** — Dividir el stream de instrucciones en basic blocks para análisis más preciso.
- [ ] **Call graph** — Construir grafo de llamadas para identificar funciones y sus relaciones.
- [ ] **Reachability analysis** — Determinar qué código es realmente alcanzable desde el entry point.
- [ ] **Loop detection** — Identificar loops naturales para detectar patrones de spray/scan.

### 2.3 Binary Format Validation Profunda
- [ ] **PE/ELF header integrity** — Verificar consistencia de todos los campos del header (no solo entry point).
- [ ] **Certificate validation** — Para PE, verificar tabla de certificados (Authenticode).
- [ ] **Resource section analysis** — Analizar sección de recursos (.rsrc) para detectar ejecutables embebidos.
- [ ] **Overlay detection** — Detectar datos después del último section end (overlay = potencial payload oculto).
- [ ] **Debug directory validation** — Verificar que la sección debug no contenga payloads.
- [ ] **TLS callback detection** — Detectar TLS callbacks (código que se ejecuta antes del entry point).
- [ ] **Delay-load import detection** — Catalogar imports de carga diferida.

---

## 3. Mejoras al Policy Engine

### 3.1 Policies por Dispositivo de Hardware
```
SecurityPolicy {
    name: "keyboard_driver",
    level: Driver,
    allowed_io_ports: [0x60, 0x64],     // Solo teclado
    allowed_irq_vectors: [1],            // Solo IRQ1
    allowed_mmio_regions: [],            // Sin MMIO
    allowed_dma_channels: [],            // Sin DMA
    allowed_pci_devices: [],             // Sin PCI directo
}
```

### 3.2 Nuevas Violaciones Propuestas
- [ ] `UnauthorizedDMAAccess` — Binario intenta configurar DMA sin permiso.
- [ ] `UnauthorizedPCIAccess` — Acceso a PCI config space no permitido.
- [ ] `TimingAttackCapability` — Uso de RDTSC sin justificación (en user-space).
- [ ] `DebugRegisterManipulation` — Acceso a DR0-DR7 (anti-debug potencial).
- [ ] `VirtualizationInstructions` — Uso de VMX/SVM sin ser hypervisor.
- [ ] `FirmwareAccess` — Intento de acceso a SPI flash / firmware regions.
- [ ] `HiddenEntryPoint` — TLS callbacks o código pre-entry point.
- [ ] `EmbeddedExecutable` — Ejecutable embebido en recursos.
- [ ] `OverlayPayload` — Datos después del fin de las secciones.
- [ ] `StackPivot` — Cambio de stack pointer a región no-stack.

### 3.3 Reportes Detallados para el Administrador
- [ ] **Hardware access report** — Resumen de exactamente qué hardware toca el binario.
- [ ] **Risk matrix** — Matriz dispositivo × acción con niveles de riesgo.
- [ ] **Diff report** — Comparar dos versiones de un binario para detectar cambios en capabilities.
- [ ] **Policy suggestion** — Dada una architecture map, sugerir la policy mínima que aprobaría el binario.

---

## 4. Integración con FastOS

### 4.1 Loader Gate
- [ ] **Pre-load gate** — El loader de FastOS llama a BG antes de mapear el binario en memoria. Si DENIED, no se carga.
- [ ] **Runtime policy enforcement** — BG genera una máscara de capabilities que el kernel aplica en runtime (ej: IOPL, CR4.TSD).
- [ ] **Capability token** — Generar un token firmado con las capabilities aprobadas que el kernel verifica en cada syscall.

### 4.2 Driver Certification
- [ ] **Driver manifest** — Cada driver declara qué hardware necesita. BG verifica que el binario NO excede lo declarado.
- [ ] **Automatic ring assignment** — BG determina automáticamente en qué ring debe ejecutarse un binario.

### 4.3 Live Update Protection
- [ ] **Binary delta analysis** — Cuando se actualiza un driver/servicio, analizar solo el delta para detectar nuevas capabilities.
- [ ] **Rollback on violation** — Si una actualización introduce capabilities no aprobadas, bloquear y notificar.

---

## 5. Verificación y Testing

### 5.1 Test Suite Expandida
- [ ] **Fixture binaries** — Crear binarios de prueba (PE, ELF) con capabilities conocidas para test end-to-end.
- [ ] **Fuzzing del loader** — Fuzz testing con binarios malformados para verificar robustez del parser.
- [ ] **Property-based testing** — Para cada ADeadOp, verificar que `classify()` es determinista y total.
- [ ] **Benchmark suite** — Medir tiempo de análisis para binarios de 1KB, 100KB, 1MB, 10MB, 100MB.

### 5.2 Validación Formal
- [ ] **Completeness proof** — Demostrar que BG cubre el 100% de las instrucciones privilegiadas x86-64.
- [ ] **Soundness proof** — Demostrar que si BG dice APPROVED, el binario no puede violar la policy.
- [ ] **Enumeration test** — Verificar que cada variante de ADeadOp tiene una clasificación explícita (no depende del catch-all `_`).

---

## 6. Priorización

### Impacto Inmediato (Alta prioridad)
1. Port range classification (§1.3) — Mapear puertos a dispositivos
2. Constant propagation (§2.1) — Resolver puertos dinámicos
3. TLS callback detection (§2.3) — Código pre-entry point
4. RDTSC/debug register detection (§1.1) — Instrucciones anti-debug
5. Overlay detection (§2.3) — Payloads ocultos

### Medio Plazo
6. Basic block / CFG construction (§2.2)
7. PCI config space tracking (§1.5)
8. Policies por dispositivo (§3.1)
9. Policy suggestion engine (§3.3)
10. Hardware access report (§3.3)

### Largo Plazo
11. VMX/SVM detection (§1.1)
12. DMA/IOMMU awareness (§1.4)
13. Firmware access detection (§1.12)
14. FastOS loader integration (§4.1)
15. Formal verification (§5.2)

---

> **Recordatorio arquitectónico:**
> BG NO es un antivirus. NO es un sandbox. NO usa heurísticas.
> BG es una **arquitectura de control estructural** que responde una pregunta simple:
> *"¿Qué puede hacer este binario con el hardware?"*
> La respuesta es matemática, determinista, y completa.
>
> — Eddi Andreé Salazar Matos
