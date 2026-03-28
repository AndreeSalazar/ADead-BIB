//! # ADead-BIB OpenGL Module — v4.7 Global Consolidado
//! 
//! ## Estructura consolidada:
//! - `opengl_global.rs` — Todo OpenGL 1.0-4.6 + constantes (reemplaza 21 archivos)
//! - `shader_bridge.rs` — Universal Shader Expeller (PTX/SPIR-V/HLSL → GLSL)
//! - `glsl.rs` — GLSL nativo
//! - `loader.rs` — Carga dinámica de funciones
//! - `optimizer.rs` — Optimizaciones GPU
//! - `types.rs` — Tipos base

pub mod opengl_global;
pub mod types;
pub mod glsl;
pub mod optimizer;
pub mod shader_bridge;

// OpenGL version modules (GL 1.0 — 4.6)
pub mod gl10;
pub mod gl11;
pub mod gl12;
pub mod gl13;
pub mod gl14;
pub mod gl15;
pub mod gl20;
pub mod gl21;
pub mod gl30;
pub mod gl31;
pub mod gl32;
pub mod gl33;
pub mod gl40;
pub mod gl41;
pub mod gl42;
pub mod gl43;
pub mod gl44;
pub mod gl45;
pub mod gl46;

// Constants (per-version)
pub mod constants_gl1x;
pub mod constants_gl2x;
pub mod constants_gl3x;
pub mod constants_gl4x;

pub mod loader;

// Re-export consolidado
pub use opengl_global::*;
pub use types::*;

// GLVersion y configuración mantenidos del archivo original
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum GLVersion {
    #[default]
    GL10, GL11, GL12, GL121, GL13, GL14, GL15,
    GL20, GL21,
    GL30, GL31, GL32, GL33,
    GL40, GL41, GL42, GL43, GL44, GL45, GL46,
}

impl GLVersion {
    pub fn major(&self) -> u32 {
        match self {
            GLVersion::GL10 | GLVersion::GL11 | GLVersion::GL12 | 
            GLVersion::GL121 | GLVersion::GL13 | GLVersion::GL14 | 
            GLVersion::GL15 => 1,
            GLVersion::GL20 | GLVersion::GL21 => 2,
            GLVersion::GL30 | GLVersion::GL31 | GLVersion::GL32 | 
            GLVersion::GL33 => 3,
            GLVersion::GL40 | GLVersion::GL41 | GLVersion::GL42 | 
            GLVersion::GL43 | GLVersion::GL44 | GLVersion::GL45 | 
            GLVersion::GL46 => 4,
        }
    }

    pub fn minor(&self) -> u32 {
        match self {
            GLVersion::GL10 | GLVersion::GL20 | GLVersion::GL30 | GLVersion::GL40 => 0,
            GLVersion::GL11 | GLVersion::GL21 | GLVersion::GL31 | GLVersion::GL41 => 1,
            GLVersion::GL12 | GLVersion::GL32 | GLVersion::GL42 => 2,
            GLVersion::GL121 => 2,
            GLVersion::GL13 | GLVersion::GL33 | GLVersion::GL43 => 3,
            GLVersion::GL14 | GLVersion::GL44 => 4,
            GLVersion::GL15 | GLVersion::GL45 => 5,
            GLVersion::GL46 => 6,
        }
    }

    pub fn is_core_profile_available(&self) -> bool {
        *self >= GLVersion::GL32
    }

    pub fn glsl_version(&self) -> Option<&'static str> {
        match self {
            GLVersion::GL20 => Some("1.10"),
            GLVersion::GL21 => Some("1.20"),
            GLVersion::GL30 => Some("1.30"),
            GLVersion::GL31 => Some("1.40"),
            GLVersion::GL32 => Some("1.50"),
            GLVersion::GL33 => Some("3.30"),
            GLVersion::GL40 => Some("4.00"),
            GLVersion::GL41 => Some("4.10"),
            GLVersion::GL42 => Some("4.20"),
            GLVersion::GL43 => Some("4.30"),
            GLVersion::GL44 => Some("4.40"),
            GLVersion::GL45 => Some("4.50"),
            GLVersion::GL46 => Some("4.60"),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GLProfile {
    Core,
    Compatibility,
}

#[derive(Debug, Clone)]
pub struct GLConfig {
    pub version: GLVersion,
    pub profile: GLProfile,
    pub debug: bool,
    pub forward_compatible: bool,
    pub robust_access: bool,
}

impl Default for GLConfig {
    fn default() -> Self {
        Self {
            version: GLVersion::GL46,
            profile: GLProfile::Core,
            debug: false,
            forward_compatible: true,
            robust_access: false,
        }
    }
}

/// OpenGL Global Expeller — Convierte cualquier shader a OpenGL
pub struct OpenGLExpeller;

impl OpenGLExpeller {
    /// Expulsa cualquier formato de shader a GLSL/OpenGL
    pub fn expel(shader_data: &[u8], format: shader_bridge::ShaderFormat) -> Result<String, String> {
        match format {
            shader_bridge::ShaderFormat::Glsl => {
                // Ya es GLSL, pasar directo
                String::from_utf8(shader_data.to_vec()).map_err(|e| e.to_string())
            }
            shader_bridge::ShaderFormat::SpirV => {
                // SPIR-V → GLSL (requiere traducción)
                shader_bridge::spirv_to_glsl(shader_data)
            }
            shader_bridge::ShaderFormat::Hlsl => {
                // HLSL → GLSL
                shader_bridge::hlsl_to_glsl(shader_data)
            }
            shader_bridge::ShaderFormat::Dxbc => {
                // DXBC → SPIR-V → GLSL
                let spirv = shader_bridge::dxbc_to_spirv(shader_data)?;
                shader_bridge::spirv_to_glsl(&spirv)
            }
            shader_bridge::ShaderFormat::Ptx => {
                // PTX → GLSL Compute Shader
                shader_bridge::ptx_to_glsl(shader_data)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gl_version_ordering() {
        assert!(GLVersion::GL10 < GLVersion::GL46);
        assert!(GLVersion::GL33 < GLVersion::GL40);
        assert!(GLVersion::GL45 < GLVersion::GL46);
    }

    #[test]
    fn test_gl_version_major_minor() {
        assert_eq!(GLVersion::GL46.major(), 4);
        assert_eq!(GLVersion::GL46.minor(), 6);
    }
}
