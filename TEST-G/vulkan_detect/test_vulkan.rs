// TEST-G: Test de Detección Vulkan/CUDA
// Verifica disponibilidad de GPU y backends
//
// Autor: Eddi Andreé Salazar Matos

use std::path::Path;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Detecta si Vulkan está disponible
fn detect_vulkan() -> bool {
    #[cfg(windows)]
    {
        let system32 = std::env::var("SystemRoot")
            .map(|r| format!("{}\\System32\\vulkan-1.dll", r))
            .unwrap_or_default();
        Path::new(&system32).exists()
    }
    #[cfg(not(windows))]
    {
        Path::new("/usr/lib/libvulkan.so.1").exists() ||
        Path::new("/usr/lib/x86_64-linux-gnu/libvulkan.so.1").exists()
    }
}

/// Detecta si CUDA está disponible
fn detect_cuda() -> bool {
    #[cfg(windows)]
    {
        let system32 = std::env::var("SystemRoot")
            .map(|r| format!("{}\\System32\\nvcuda.dll", r))
            .unwrap_or_default();
        Path::new(&system32).exists()
    }
    #[cfg(not(windows))]
    {
        Path::new("/usr/lib/libcuda.so").exists() ||
        Path::new("/usr/lib/x86_64-linux-gnu/libcuda.so").exists()
    }
}

/// Detecta vendor de GPU por archivos de driver
fn detect_gpu_vendor() -> &'static str {
    #[cfg(windows)]
    {
        let system32 = std::env::var("SystemRoot").unwrap_or_default();
        
        // NVIDIA
        if Path::new(&format!("{}\\System32\\nvapi64.dll", system32)).exists() ||
           Path::new(&format!("{}\\System32\\nvcuda.dll", system32)).exists() {
            return "NVIDIA";
        }
        
        // AMD
        if Path::new(&format!("{}\\System32\\amdxc64.dll", system32)).exists() ||
           Path::new(&format!("{}\\System32\\atidxx64.dll", system32)).exists() {
            return "AMD";
        }
        
        // Intel
        if Path::new(&format!("{}\\System32\\igdumdim64.dll", system32)).exists() {
            return "Intel";
        }
        
        "Unknown"
    }
    #[cfg(not(windows))]
    {
        if Path::new("/dev/nvidia0").exists() { "NVIDIA" }
        else if Path::new("/dev/dri/renderD128").exists() { "AMD/Intel" }
        else { "Unknown" }
    }
}

/// Estructura de features GPU
#[derive(Debug, Clone, Hash)]
struct GPUInfo {
    vulkan_available: bool,
    cuda_available: bool,
    vendor: String,
}

impl GPUInfo {
    fn detect() -> Self {
        Self {
            vulkan_available: detect_vulkan(),
            cuda_available: detect_cuda(),
            vendor: detect_gpu_vendor().to_string(),
        }
    }
}

fn hash_info(info: &GPUInfo) -> u64 {
    let mut hasher = DefaultHasher::new();
    info.hash(&mut hasher);
    hasher.finish()
}

/// Test 1: Detección de Vulkan
fn test_vulkan_detection() -> bool {
    let available = detect_vulkan();
    
    println!("  Vulkan disponible: {}", if available { "✓" } else { "✗" });
    
    #[cfg(windows)]
    {
        let path = std::env::var("SystemRoot")
            .map(|r| format!("{}\\System32\\vulkan-1.dll", r))
            .unwrap_or_default();
        println!("  Ruta verificada: {}", path);
    }
    
    true // Test pasa si no hay crash
}

/// Test 2: Detección de CUDA
fn test_cuda_detection() -> bool {
    let available = detect_cuda();
    
    println!("  CUDA disponible: {}", if available { "✓" } else { "✗" });
    
    #[cfg(windows)]
    {
        let path = std::env::var("SystemRoot")
            .map(|r| format!("{}\\System32\\nvcuda.dll", r))
            .unwrap_or_default();
        println!("  Ruta verificada: {}", path);
    }
    
    true
}

/// Test 3: Detección de vendor
fn test_vendor_detection() -> bool {
    let vendor = detect_gpu_vendor();
    
    println!("  GPU Vendor detectado: {}", vendor);
    
    true
}

/// Test 4: Detección determinista
fn test_detection_deterministic() -> bool {
    let info1 = GPUInfo::detect();
    let info2 = GPUInfo::detect();
    let info3 = GPUInfo::detect();
    
    let h1 = hash_info(&info1);
    let h2 = hash_info(&info2);
    let h3 = hash_info(&info3);
    
    println!("  Hash 1: {:016x}", h1);
    println!("  Hash 2: {:016x}", h2);
    println!("  Hash 3: {:016x}", h3);
    println!("  Determinista: {}", h1 == h2 && h2 == h3);
    
    h1 == h2 && h2 == h3
}

/// Test 5: Múltiples detecciones
fn test_multiple_detections() -> bool {
    println!("  Ejecutando 100 detecciones...");
    
    let reference = GPUInfo::detect();
    let ref_hash = hash_info(&reference);
    
    for i in 0..100 {
        let info = GPUInfo::detect();
        let hash = hash_info(&info);
        
        if hash != ref_hash {
            println!("  ❌ Detección {} diferente!", i);
            return false;
        }
    }
    
    println!("  100/100 detecciones idénticas");
    true
}

/// Test 6: Selección de backend
fn test_backend_selection() -> bool {
    let info = GPUInfo::detect();
    
    #[derive(Debug)]
    enum Backend {
        CPU,
        Vulkan,
        CUDA,
    }
    
    let backend = if info.cuda_available && info.vendor == "NVIDIA" {
        Backend::CUDA
    } else if info.vulkan_available {
        Backend::Vulkan
    } else {
        Backend::CPU
    };
    
    println!("  Vulkan: {} | CUDA: {} | Vendor: {}",
             if info.vulkan_available { "✓" } else { "✗" },
             if info.cuda_available { "✓" } else { "✗" },
             info.vendor);
    println!("  Backend seleccionado: {:?}", backend);
    
    true
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     TEST-G: Test Detección Vulkan/CUDA                     ║");
    println!("║     Autor: Eddi Andreé Salazar Matos                       ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    
    let mut passed = 0;
    let mut failed = 0;
    
    println!("🧪 Test 1: Detección Vulkan");
    if test_vulkan_detection() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 2: Detección CUDA");
    if test_cuda_detection() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 3: Detección Vendor");
    if test_vendor_detection() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 4: Detección determinista");
    if test_detection_deterministic() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 5: Múltiples detecciones");
    if test_multiple_detections() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 6: Selección de backend");
    if test_backend_selection() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("════════════════════════════════════════════════════════════");
    println!("📊 Resultados: {} passed, {} failed", passed, failed);
    
    if failed == 0 {
        println!("✅ TEST-G VULKAN COMPLETADO - Detección GPU es DETERMINISTA");
    } else {
        println!("❌ TEST-G VULKAN FALLIDO");
    }
}
