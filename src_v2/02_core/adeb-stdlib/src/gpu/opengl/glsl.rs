//! GLSL Shader Utilities for ADead-BIB OpenGL
//! 
//! Provides shader source management, stage definitions,
//! and compile-time shader validation helpers.

use super::types::*;

/// GLSL Shader stages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderStage {
    Vertex,
    Fragment,
    Geometry,
    TessControl,
    TessEvaluation,
    Compute,
}

impl ShaderStage {
    /// Get the GL enum for this shader stage
    #[inline]
    pub const fn to_gl_enum(&self) -> GLenum {
        match self {
            ShaderStage::Vertex => 0x8B31,          // GL_VERTEX_SHADER
            ShaderStage::Fragment => 0x8B30,         // GL_FRAGMENT_SHADER
            ShaderStage::Geometry => 0x8DD9,         // GL_GEOMETRY_SHADER
            ShaderStage::TessControl => 0x8E88,      // GL_TESS_CONTROL_SHADER
            ShaderStage::TessEvaluation => 0x8E87,   // GL_TESS_EVALUATION_SHADER
            ShaderStage::Compute => 0x91B9,          // GL_COMPUTE_SHADER
        }
    }

    /// Get the pipeline stage bit for glUseProgramStages
    #[inline]
    pub const fn to_stage_bit(&self) -> GLbitfield {
        match self {
            ShaderStage::Vertex => 0x00000001,          // GL_VERTEX_SHADER_BIT
            ShaderStage::Fragment => 0x00000002,         // GL_FRAGMENT_SHADER_BIT
            ShaderStage::Geometry => 0x00000004,         // GL_GEOMETRY_SHADER_BIT
            ShaderStage::TessControl => 0x00000008,      // GL_TESS_CONTROL_SHADER_BIT
            ShaderStage::TessEvaluation => 0x00000010,   // GL_TESS_EVALUATION_SHADER_BIT
            ShaderStage::Compute => 0x00000020,          // GL_COMPUTE_SHADER_BIT
        }
    }

    /// Minimum OpenGL version required
    #[inline]
    pub const fn min_version_major(&self) -> u32 {
        match self {
            ShaderStage::Vertex | ShaderStage::Fragment => 2,
            ShaderStage::Geometry => 3,
            ShaderStage::TessControl | ShaderStage::TessEvaluation => 4,
            ShaderStage::Compute => 4,
        }
    }

    #[inline]
    pub const fn min_version_minor(&self) -> u32 {
        match self {
            ShaderStage::Vertex | ShaderStage::Fragment => 0,
            ShaderStage::Geometry => 2,
            ShaderStage::TessControl | ShaderStage::TessEvaluation => 0,
            ShaderStage::Compute => 3,
        }
    }
}

/// GLSL shader source with metadata
#[derive(Debug, Clone)]
pub struct ShaderSource {
    pub stage: ShaderStage,
    pub source: String,
    pub entry_point: String,
}

impl ShaderSource {
    pub fn new(stage: ShaderStage, source: &str) -> Self {
        Self {
            stage,
            source: source.to_string(),
            entry_point: "main".to_string(),
        }
    }

    pub fn vertex(source: &str) -> Self {
        Self::new(ShaderStage::Vertex, source)
    }

    pub fn fragment(source: &str) -> Self {
        Self::new(ShaderStage::Fragment, source)
    }

    pub fn geometry(source: &str) -> Self {
        Self::new(ShaderStage::Geometry, source)
    }

    pub fn compute(source: &str) -> Self {
        Self::new(ShaderStage::Compute, source)
    }

    /// Prepend a #version directive if not present
    pub fn with_version(mut self, version: &str, profile: Option<&str>) -> Self {
        if !self.source.starts_with("#version") {
            let profile_str = profile.map(|p| format!(" {}", p)).unwrap_or_default();
            self.source = format!("#version {}{}\n{}", version, profile_str, self.source);
        }
        self
    }
}

/// GLSL Uniform block layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UniformBlockLayout {
    /// std140 — guaranteed layout, cross-platform
    Std140,
    /// std430 — tighter packing (SSBOs only, GL 4.3+)
    Std430,
    /// shared — implementation-dependent
    Shared,
    /// packed — implementation-dependent, tightest
    Packed,
}

impl UniformBlockLayout {
    pub const fn layout_qualifier(&self) -> &'static str {
        match self {
            UniformBlockLayout::Std140 => "std140",
            UniformBlockLayout::Std430 => "std430",
            UniformBlockLayout::Shared => "shared",
            UniformBlockLayout::Packed => "packed",
        }
    }

    /// Base alignment for this layout (in bytes)
    pub const fn vec4_alignment(&self) -> usize {
        match self {
            UniformBlockLayout::Std140 => 16,
            UniformBlockLayout::Std430 => 16,
            _ => 0, // implementation-dependent
        }
    }
}

/// Shader program pipeline helper
#[derive(Debug, Clone)]
pub struct ShaderPipeline {
    pub stages: Vec<ShaderSource>,
}

impl ShaderPipeline {
    pub fn new() -> Self {
        Self { stages: Vec::new() }
    }

    pub fn add_stage(mut self, source: ShaderSource) -> Self {
        self.stages.push(source);
        self
    }

    pub fn has_stage(&self, stage: ShaderStage) -> bool {
        self.stages.iter().any(|s| s.stage == stage)
    }

    pub fn has_vertex(&self) -> bool { self.has_stage(ShaderStage::Vertex) }
    pub fn has_fragment(&self) -> bool { self.has_stage(ShaderStage::Fragment) }
    pub fn has_geometry(&self) -> bool { self.has_stage(ShaderStage::Geometry) }
    pub fn has_compute(&self) -> bool { self.has_stage(ShaderStage::Compute) }

    /// Minimum OpenGL version required for all stages
    pub fn min_gl_version(&self) -> (u32, u32) {
        let mut major = 2u32;
        let mut minor = 0u32;
        for s in &self.stages {
            let smaj = s.stage.min_version_major();
            let smin = s.stage.min_version_minor();
            if smaj > major || (smaj == major && smin > minor) {
                major = smaj;
                minor = smin;
            }
        }
        (major, minor)
    }
}

impl Default for ShaderPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shader_stage_gl_enum() {
        assert_eq!(ShaderStage::Vertex.to_gl_enum(), 0x8B31);
        assert_eq!(ShaderStage::Fragment.to_gl_enum(), 0x8B30);
        assert_eq!(ShaderStage::Compute.to_gl_enum(), 0x91B9);
    }

    #[test]
    fn test_shader_pipeline_min_version() {
        let pipeline = ShaderPipeline::new()
            .add_stage(ShaderSource::vertex("void main() {}"))
            .add_stage(ShaderSource::fragment("void main() {}"));
        assert_eq!(pipeline.min_gl_version(), (2, 0));

        let pipeline_compute = ShaderPipeline::new()
            .add_stage(ShaderSource::compute("void main() {}"));
        assert_eq!(pipeline_compute.min_gl_version(), (4, 3));
    }

    #[test]
    fn test_shader_source_with_version() {
        let src = ShaderSource::vertex("void main() {}")
            .with_version("460", Some("core"));
        assert!(src.source.starts_with("#version 460 core"));
    }
}
