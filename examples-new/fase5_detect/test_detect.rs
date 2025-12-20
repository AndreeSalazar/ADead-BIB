// Test Fase 5: Auto-Detección CPU + GPU
// Verifica que la detección es determinista y correcta
//
// Autor: Eddi Andreé Salazar Matos

use std::arch::x86_64::{__cpuid, __cpuid_count};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Características del CPU (copia simplificada para test)
#[derive(Debug, Clone, Hash)]
struct CPUFeatures {
    vendor: String,
    brand: String,
    has_sse2: bool,
    has_sse4_1: bool,
    has_avx: bool,
    has_avx2: bool,
    has_avx512f: bool,
    has_fma: bool,
    cores: u32,
}

impl CPUFeatures {
    fn detect() -> Self {
        unsafe { Self::detect_cpuid() }
    }
    
    unsafe fn detect_cpuid() -> Self {
        let mut features = Self {
            vendor: String::new(),
            brand: String::new(),
            has_sse2: false,
            has_sse4_1: false,
            has_avx: false,
            has_avx2: false,
            has_avx512f: false,
            has_fma: false,
            cores: 1,
        };
        
        // CPUID leaf 0: Vendor
        let cpuid0 = __cpuid(0);
        let max_leaf = cpuid0.eax;
        
        let vendor_bytes: [u8; 12] = [
            (cpuid0.ebx & 0xFF) as u8,
            ((cpuid0.ebx >> 8) & 0xFF) as u8,
            ((cpuid0.ebx >> 16) & 0xFF) as u8,
            ((cpuid0.ebx >> 24) & 0xFF) as u8,
            (cpuid0.edx & 0xFF) as u8,
            ((cpuid0.edx >> 8) & 0xFF) as u8,
            ((cpuid0.edx >> 16) & 0xFF) as u8,
            ((cpuid0.edx >> 24) & 0xFF) as u8,
            (cpuid0.ecx & 0xFF) as u8,
            ((cpuid0.ecx >> 8) & 0xFF) as u8,
            ((cpuid0.ecx >> 16) & 0xFF) as u8,
            ((cpuid0.ecx >> 24) & 0xFF) as u8,
        ];
        features.vendor = String::from_utf8_lossy(&vendor_bytes).to_string();
        
        // CPUID leaf 1: Features
        if max_leaf >= 1 {
            let cpuid1 = __cpuid(1);
            features.has_sse2 = (cpuid1.edx & (1 << 26)) != 0;
            features.has_sse4_1 = (cpuid1.ecx & (1 << 19)) != 0;
            features.has_avx = (cpuid1.ecx & (1 << 28)) != 0;
            features.has_fma = (cpuid1.ecx & (1 << 12)) != 0;
        }
        
        // CPUID leaf 7: Extended features
        if max_leaf >= 7 {
            let cpuid7 = __cpuid_count(7, 0);
            features.has_avx2 = (cpuid7.ebx & (1 << 5)) != 0;
            features.has_avx512f = (cpuid7.ebx & (1 << 16)) != 0;
        }
        
        // Brand string
        let cpuid_ext0 = __cpuid(0x80000000);
        if cpuid_ext0.eax >= 0x80000004 {
            let mut brand_bytes = Vec::with_capacity(48);
            for leaf in 0x80000002..=0x80000004 {
                let cpuid = __cpuid(leaf);
                brand_bytes.extend_from_slice(&cpuid.eax.to_le_bytes());
                brand_bytes.extend_from_slice(&cpuid.ebx.to_le_bytes());
                brand_bytes.extend_from_slice(&cpuid.ecx.to_le_bytes());
                brand_bytes.extend_from_slice(&cpuid.edx.to_le_bytes());
            }
            features.brand = String::from_utf8_lossy(&brand_bytes)
                .trim_matches('\0')
                .trim()
                .to_string();
        }
        
        features.cores = std::thread::available_parallelism()
            .map(|p| p.get() as u32)
            .unwrap_or(1);
        
        features
    }
    
    fn best_simd(&self) -> &'static str {
        if self.has_avx512f { "AVX-512" }
        else if self.has_avx2 { "AVX2" }
        else if self.has_avx { "AVX" }
        else if self.has_sse4_1 { "SSE4.1" }
        else if self.has_sse2 { "SSE2" }
        else { "Scalar" }
    }
    
    fn simd_width(&self) -> u32 {
        if self.has_avx512f { 512 }
        else if self.has_avx2 { 256 }
        else if self.has_avx { 256 }
        else if self.has_sse2 { 128 }
        else { 64 }
    }
}

fn hash_features(f: &CPUFeatures) -> u64 {
    let mut hasher = DefaultHasher::new();
    f.hash(&mut hasher);
    hasher.finish()
}

/// Test 1: Detección básica funciona
fn test_detection_works() -> bool {
    let features = CPUFeatures::detect();
    
    println!("  Vendor: {}", features.vendor);
    println!("  Brand:  {}", features.brand);
    println!("  Cores:  {}", features.cores);
    
    let vendor_ok = !features.vendor.is_empty() && 
                    (features.vendor.contains("Intel") || 
                     features.vendor.contains("AMD") ||
                     features.vendor.contains("Genuine"));
    
    println!("  Vendor válido: {}", vendor_ok);
    
    vendor_ok
}

/// Test 2: Detección es determinista
fn test_detection_deterministic() -> bool {
    let f1 = CPUFeatures::detect();
    let f2 = CPUFeatures::detect();
    let f3 = CPUFeatures::detect();
    
    let h1 = hash_features(&f1);
    let h2 = hash_features(&f2);
    let h3 = hash_features(&f3);
    
    println!("  Hash 1: {:016x}", h1);
    println!("  Hash 2: {:016x}", h2);
    println!("  Hash 3: {:016x}", h3);
    println!("  Determinista: {}", h1 == h2 && h2 == h3);
    
    h1 == h2 && h2 == h3
}

/// Test 3: SSE2 siempre disponible en x86-64
fn test_sse2_available() -> bool {
    let features = CPUFeatures::detect();
    
    println!("  SSE2: {}", features.has_sse2);
    println!("  (Requerido para x86-64)");
    
    features.has_sse2
}

/// Test 4: Detección de SIMD avanzado
fn test_simd_detection() -> bool {
    let features = CPUFeatures::detect();
    
    println!("  SSE2:    {}", if features.has_sse2 { "✓" } else { "✗" });
    println!("  SSE4.1:  {}", if features.has_sse4_1 { "✓" } else { "✗" });
    println!("  AVX:     {}", if features.has_avx { "✓" } else { "✗" });
    println!("  AVX2:    {}", if features.has_avx2 { "✓" } else { "✗" });
    println!("  AVX-512: {}", if features.has_avx512f { "✓" } else { "✗" });
    println!("  FMA:     {}", if features.has_fma { "✓" } else { "✗" });
    println!();
    println!("  Best SIMD: {} ({}-bit)", features.best_simd(), features.simd_width());
    
    // Al menos SSE2 debe estar disponible
    features.has_sse2
}

/// Test 5: Múltiples detecciones son idénticas
fn test_multiple_detections() -> bool {
    let mut all_same = true;
    let reference = CPUFeatures::detect();
    let ref_hash = hash_features(&reference);
    
    println!("  Ejecutando 100 detecciones...");
    
    for i in 0..100 {
        let features = CPUFeatures::detect();
        let hash = hash_features(&features);
        
        if hash != ref_hash {
            println!("  ❌ Detección {} diferente!", i);
            all_same = false;
            break;
        }
    }
    
    if all_same {
        println!("  100/100 detecciones idénticas");
    }
    
    all_same
}

/// Test 6: Selección de backend
fn test_backend_selection() -> bool {
    let features = CPUFeatures::detect();
    
    #[derive(Debug)]
    enum Backend {
        Scalar,
        SSE2,
        AVX,
        AVX2,
        AVX512,
    }
    
    let backend = if features.has_avx512f { Backend::AVX512 }
                  else if features.has_avx2 { Backend::AVX2 }
                  else if features.has_avx { Backend::AVX }
                  else if features.has_sse2 { Backend::SSE2 }
                  else { Backend::Scalar };
    
    println!("  Backend seleccionado: {:?}", backend);
    println!("  SIMD width: {} bits", features.simd_width());
    println!("  Floats por vector: {}", features.simd_width() / 32);
    
    // Verificar consistencia
    let width = features.simd_width();
    let expected_width = match backend {
        Backend::AVX512 => 512,
        Backend::AVX2 | Backend::AVX => 256,
        Backend::SSE2 => 128,
        Backend::Scalar => 64,
    };
    
    println!("  Width consistente: {}", width == expected_width);
    
    width == expected_width
}

fn main() {
    println!("╔════════════════════════════════════════════════════════════╗");
    println!("║     ADead-BIB - Fase 5: Test Auto-Detección CPU            ║");
    println!("║     Autor: Eddi Andreé Salazar Matos                       ║");
    println!("╚════════════════════════════════════════════════════════════╝");
    println!();
    
    let mut passed = 0;
    let mut failed = 0;
    
    println!("🧪 Test 1: Detección básica");
    if test_detection_works() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 2: Detección determinista");
    if test_detection_deterministic() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 3: SSE2 disponible");
    if test_sse2_available() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 4: Detección SIMD avanzado");
    if test_simd_detection() {
        println!("   ✅ PASSED\n");
        passed += 1;
    } else {
        println!("   ❌ FAILED\n");
        failed += 1;
    }
    
    println!("🧪 Test 5: Múltiples detecciones idénticas");
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
        println!("✅ FASE 5 COMPLETADA - Detección CPU es DETERMINISTA");
        println!();
        println!("🎯 Tu CPU puede usar: {}", CPUFeatures::detect().best_simd());
    } else {
        println!("❌ FASE 5 FALLIDA - Revisar implementación");
    }
}
