// ADead-BIB Runtime - GPU Detection
// Auto-detección de GPU via Vulkan/CUDA
// Determinista y sin dependencias externas
//
// Autor: Eddi Andreé Salazar Matos
// Email: eddi.salazar.dev@gmail.com

use std::path::Path;

/// Vendor de GPU detectado
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GPUVendor {
    NVIDIA,
    AMD,
    Intel,
    Unknown,
}

impl GPUVendor {
    pub fn from_id(vendor_id: u32) -> Self {
        match vendor_id {
            0x10DE => Self::NVIDIA,
            0x1002 => Self::AMD,
            0x8086 => Self::Intel,
            _ => Self::Unknown,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            Self::NVIDIA => "NVIDIA",
            Self::AMD => "AMD",
            Self::Intel => "Intel",
            Self::Unknown => "Unknown",
        }
    }
}

/// Características de la GPU detectada
#[derive(Debug, Clone)]
pub struct GPUFeatures {
    /// GPU disponible
    pub available: bool,
    /// Vulkan disponible
    pub vulkan_available: bool,
    /// CUDA disponible (solo NVIDIA)
    pub cuda_available: bool,
    /// Vendor de la GPU
    pub vendor: GPUVendor,
    /// Nombre del dispositivo
    pub device_name: String,
    /// VRAM total en MB
    pub vram_mb: u32,
    /// Número de compute units/SMs
    pub compute_units: u32,
    /// Tamaño máximo de workgroup
    pub max_workgroup_size: u32,
    /// Soporte FP16
    pub supports_fp16: bool,
    /// Soporte FP64
    pub supports_fp64: bool,
    /// Soporte INT8
    pub supports_int8: bool,
    /// Versión de Vulkan (major.minor)
    pub vulkan_version: (u32, u32),
    /// Versión de CUDA (si aplica)
    pub cuda_version: (u32, u32),
}

impl Default for GPUFeatures {
    fn default() -> Self {
        Self {
            available: false,
            vulkan_available: false,
            cuda_available: false,
            vendor: GPUVendor::Unknown,
            device_name: String::new(),
            vram_mb: 0,
            compute_units: 0,
            max_workgroup_size: 0,
            supports_fp16: false,
            supports_fp64: false,
            supports_int8: false,
            vulkan_version: (0, 0),
            cuda_version: (0, 0),
        }
    }
}

impl GPUFeatures {
    /// Detecta GPU disponible (Vulkan primero, luego CUDA)
    pub fn detect() -> Self {
        let mut features = Self::default();
        
        // Intentar Vulkan primero
        if let Some(vk_features) = Self::detect_vulkan() {
            features = vk_features;
            features.vulkan_available = true;
            features.available = true;
        }
        
        // Si es NVIDIA, intentar CUDA también
        if features.vendor == GPUVendor::NVIDIA {
            if let Some((major, minor)) = Self::detect_cuda_version() {
                features.cuda_available = true;
                features.cuda_version = (major, minor);
            }
        }
        
        features
    }
    
    /// Detecta GPU via Vulkan (verificación de archivos)
    fn detect_vulkan() -> Option<Self> {
        if !detect_vulkan_simple() {
            return None;
        }
        
        // Vulkan está disponible, retornar info básica
        // La detección completa de dispositivo requiere inicializar Vulkan
        Some(Self {
            available: true,
            vulkan_available: true,
            cuda_available: false,
            vendor: GPUVendor::Unknown,
            device_name: "Vulkan Device (detected)".to_string(),
            vram_mb: 0,
            compute_units: 0,
            max_workgroup_size: 1024,
            supports_fp16: true,
            supports_fp64: false,
            supports_int8: true,
            vulkan_version: (1, 0), // Asumimos al menos Vulkan 1.0
            cuda_version: (0, 0),
        })
    }
    
    /// Detecta versión de CUDA (verificación de archivos)
    fn detect_cuda_version() -> Option<(u32, u32)> {
        if detect_cuda_simple() {
            // CUDA está disponible
            Some((12, 0)) // Asumimos versión reciente
        } else {
            None
        }
    }
    
    /// Imprime resumen de la GPU
    pub fn print_summary(&self) {
        println!("╔════════════════════════════════════════════════════════════╗");
        println!("║                    GPU FEATURES                             ║");
        println!("╠════════════════════════════════════════════════════════════╣");
        
        if self.available {
            println!("║ GPU Available: ✓                                           ║");
            println!("║ Device: {:50} ║", &self.device_name[..self.device_name.len().min(50)]);
            println!("║ Vendor: {:50} ║", self.vendor.name());
            println!("║ VRAM: {:5} MB                                             ║", self.vram_mb);
            println!("╠════════════════════════════════════════════════════════════╣");
            println!("║ Vulkan: {} (v{}.{})                                         ║",
                     if self.vulkan_available { "✓" } else { "✗" },
                     self.vulkan_version.0, self.vulkan_version.1);
            println!("║ CUDA:   {} (v{}.{})                                         ║",
                     if self.cuda_available { "✓" } else { "✗" },
                     self.cuda_version.0, self.cuda_version.1);
            println!("╠════════════════════════════════════════════════════════════╣");
            println!("║ FP16: {} | FP64: {} | INT8: {}                              ║",
                     if self.supports_fp16 { "✓" } else { "✗" },
                     if self.supports_fp64 { "✓" } else { "✗" },
                     if self.supports_int8 { "✓" } else { "✗" });
        } else {
            println!("║ GPU Available: ✗                                           ║");
            println!("║ No compatible GPU detected                                 ║");
        }
        
        println!("╚════════════════════════════════════════════════════════════╝");
    }
}

/// Detección simple sin libloading (para tests)
pub fn detect_vulkan_simple() -> bool {
    #[cfg(windows)]
    {
        // Verificar si vulkan-1.dll existe en el sistema
        use std::path::Path;
        let system32 = std::env::var("SystemRoot")
            .map(|r| format!("{}\\System32\\vulkan-1.dll", r))
            .unwrap_or_default();
        Path::new(&system32).exists()
    }
    #[cfg(not(windows))]
    {
        use std::path::Path;
        Path::new("/usr/lib/libvulkan.so.1").exists() ||
        Path::new("/usr/lib/x86_64-linux-gnu/libvulkan.so.1").exists()
    }
}

/// Detección simple de CUDA
pub fn detect_cuda_simple() -> bool {
    #[cfg(windows)]
    {
        use std::path::Path;
        let system32 = std::env::var("SystemRoot")
            .map(|r| format!("{}\\System32\\nvcuda.dll", r))
            .unwrap_or_default();
        Path::new(&system32).exists()
    }
    #[cfg(not(windows))]
    {
        use std::path::Path;
        Path::new("/usr/lib/libcuda.so").exists() ||
        Path::new("/usr/lib/x86_64-linux-gnu/libcuda.so").exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vulkan_detection_deterministic() {
        let result1 = detect_vulkan_simple();
        let result2 = detect_vulkan_simple();
        let result3 = detect_vulkan_simple();
        
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
    
    #[test]
    fn test_cuda_detection_deterministic() {
        let result1 = detect_cuda_simple();
        let result2 = detect_cuda_simple();
        let result3 = detect_cuda_simple();
        
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
    }
    
    #[test]
    fn test_vendor_from_id() {
        assert_eq!(GPUVendor::from_id(0x10DE), GPUVendor::NVIDIA);
        assert_eq!(GPUVendor::from_id(0x1002), GPUVendor::AMD);
        assert_eq!(GPUVendor::from_id(0x8086), GPUVendor::Intel);
    }
}
