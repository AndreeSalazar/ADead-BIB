//! # ADead-BIB OpenGL 4.7 — Universal Shader Bridge
//!
//! Permite a OpenGL leer y consumir shaders de TODAS las APIs:
//! - **GLSL**      → nativo OpenGL (.glsl, .vert, .frag, .geom, .comp, .tesc, .tese)
//! - **SPIR-V**    → Vulkan/OpenGL 4.6+ (.spv)
//! - **HLSL/DXBC** → DirectX 12 (.hlsl, .dxbc)
//! - **PTX**       → CUDA/CUDead-BIB (.ptx)
//!
//! Filosofía: Un solo punto de entrada para cualquier shader → OpenGL lo consume.
//!
//! Autor: Eddi Andreé Salazar Matos — Marzo 2026

use super::types::*;
use super::glsl::{ShaderStage, ShaderSource};

// =========================================================================
// Shader format detection
// =========================================================================

/// All shader formats that OpenGL can consume via ADead-BIB
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShaderFormat {
    /// GLSL source code — native OpenGL
    Glsl,
    /// SPIR-V binary — Vulkan / GL 4.6 (GL_ARB_gl_spirv)
    SpirV,
    /// HLSL source — DirectX, cross-compiled to GLSL/SPIR-V
    Hlsl,
    /// DXBC bytecode — DirectX compiled, cross-compiled to SPIR-V
    Dxbc,
    /// PTX assembly — CUDA/CUDead, mapped to compute shaders
    Ptx,
}

impl ShaderFormat {
    /// Detect format from file extension
    pub fn from_extension(path: &str) -> Option<Self> {
        let ext = path.rsplit('.').next()?.to_ascii_lowercase();
        match ext.as_str() {
            "glsl" | "vert" | "frag" | "geom" | "comp" | "tesc" | "tese" => Some(ShaderFormat::Glsl),
            "spv" | "spirv" => Some(ShaderFormat::SpirV),
            "hlsl" | "fx" => Some(ShaderFormat::Hlsl),
            "dxbc" | "cso" => Some(ShaderFormat::Dxbc),
            "ptx" => Some(ShaderFormat::Ptx),
            _ => None,
        }
    }

    /// Detect format from binary magic bytes
    pub fn from_magic(data: &[u8]) -> Option<Self> {
        if data.len() < 4 {
            return None;
        }
        // SPIR-V magic: 0x07230203
        if data[0] == 0x03 && data[1] == 0x02 && data[2] == 0x23 && data[3] == 0x07 {
            return Some(ShaderFormat::SpirV);
        }
        // DXBC magic: "DXBC"
        if data[0] == b'D' && data[1] == b'X' && data[2] == b'B' && data[3] == b'C' {
            return Some(ShaderFormat::Dxbc);
        }
        // PTX starts with ".version"
        if data.len() >= 8 && &data[..8] == b".version" {
            return Some(ShaderFormat::Ptx);
        }
        // Assume GLSL if it looks like text starting with #version or //
        if data[0] == b'#' || data[0] == b'/' {
            return Some(ShaderFormat::Glsl);
        }
        None
    }
}

// =========================================================================
// SPIR-V → OpenGL bridge (GL 4.6 / GL_ARB_gl_spirv)
// =========================================================================

/// SPIR-V specialization constant
#[derive(Debug, Clone)]
pub struct SpecConstant {
    pub index: u32,
    pub value: u32,
}

/// SPIR-V binary ready for OpenGL consumption
#[derive(Debug, Clone)]
pub struct SpirVShader {
    pub bytecode: Vec<u8>,
    pub stage: ShaderStage,
    pub entry_point: String,
    pub spec_constants: Vec<SpecConstant>,
}

impl SpirVShader {
    pub fn new(bytecode: Vec<u8>, stage: ShaderStage) -> Self {
        Self {
            bytecode,
            stage,
            entry_point: "main".to_string(),
            spec_constants: Vec::new(),
        }
    }

    pub fn with_entry_point(mut self, entry: &str) -> Self {
        self.entry_point = entry.to_string();
        self
    }

    pub fn add_spec_constant(mut self, index: u32, value: u32) -> Self {
        self.spec_constants.push(SpecConstant { index, value });
        self
    }

    /// Validate SPIR-V magic number
    pub fn validate_magic(&self) -> bool {
        self.bytecode.len() >= 4
            && self.bytecode[0] == 0x03
            && self.bytecode[1] == 0x02
            && self.bytecode[2] == 0x23
            && self.bytecode[3] == 0x07
    }

    /// Word count (SPIR-V is u32-aligned)
    pub fn word_count(&self) -> usize {
        self.bytecode.len() / 4
    }

    /// GL function pointers needed: glShaderBinary + glSpecializeShader (GL 4.6)
    pub fn gl_shader_binary_format() -> GLenum {
        0x9551 // GL_SHADER_BINARY_FORMAT_SPIR_V
    }

    pub fn gl_spir_v_binary() -> GLenum {
        0x9552 // GL_SPIR_V_BINARY
    }
}

// =========================================================================
// HLSL → GLSL/SPIR-V cross-compilation (DX12 → OpenGL)
// =========================================================================

/// HLSL shader model target
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HlslShaderModel {
    SM5_0,
    SM5_1,
    SM6_0,
    SM6_1,
    SM6_2,
    SM6_3,
    SM6_4,
    SM6_5,
    SM6_6,
}

/// DX12 → OpenGL semantic mapping
#[derive(Debug, Clone)]
pub struct HlslSemantic {
    pub hlsl_name: &'static str,
    pub glsl_equivalent: &'static str,
    pub gl_location: i32,
}

/// Standard DX12 ↔ OpenGL semantic table
pub const HLSL_TO_GLSL_SEMANTICS: &[HlslSemantic] = &[
    HlslSemantic { hlsl_name: "POSITION",    glsl_equivalent: "gl_Position",     gl_location: 0 },
    HlslSemantic { hlsl_name: "SV_Position", glsl_equivalent: "gl_Position",     gl_location: 0 },
    HlslSemantic { hlsl_name: "NORMAL",      glsl_equivalent: "aNormal",         gl_location: 1 },
    HlslSemantic { hlsl_name: "TEXCOORD",    glsl_equivalent: "aTexCoord",       gl_location: 2 },
    HlslSemantic { hlsl_name: "TANGENT",     glsl_equivalent: "aTangent",        gl_location: 3 },
    HlslSemantic { hlsl_name: "COLOR",       glsl_equivalent: "aColor",          gl_location: 4 },
    HlslSemantic { hlsl_name: "SV_Target",   glsl_equivalent: "fragColor",       gl_location: 0 },
    HlslSemantic { hlsl_name: "SV_Depth",    glsl_equivalent: "gl_FragDepth",    gl_location: -1 },
    HlslSemantic { hlsl_name: "SV_VertexID", glsl_equivalent: "gl_VertexID",     gl_location: -1 },
    HlslSemantic { hlsl_name: "SV_InstanceID", glsl_equivalent: "gl_InstanceID", gl_location: -1 },
    HlslSemantic { hlsl_name: "SV_DispatchThreadID", glsl_equivalent: "gl_GlobalInvocationID", gl_location: -1 },
    HlslSemantic { hlsl_name: "SV_GroupID",    glsl_equivalent: "gl_WorkGroupID",     gl_location: -1 },
    HlslSemantic { hlsl_name: "SV_GroupThreadID", glsl_equivalent: "gl_LocalInvocationID", gl_location: -1 },
];

/// DX12 resource type → OpenGL binding mapping
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dx12ResourceType {
    ConstantBuffer,  // cbuffer → Uniform Buffer Object
    Texture2D,       // Texture2D → sampler2D
    Texture3D,       // Texture3D → sampler3D
    TextureCube,     // TextureCube → samplerCube
    RWTexture2D,     // RWTexture2D → image2D
    StructuredBuffer,    // StructuredBuffer → SSBO
    RWStructuredBuffer,  // RWStructuredBuffer → SSBO
    SamplerState,    // SamplerState → sampler
}

impl Dx12ResourceType {
    pub fn to_glsl_type(&self) -> &'static str {
        match self {
            Dx12ResourceType::ConstantBuffer => "uniform",
            Dx12ResourceType::Texture2D => "sampler2D",
            Dx12ResourceType::Texture3D => "sampler3D",
            Dx12ResourceType::TextureCube => "samplerCube",
            Dx12ResourceType::RWTexture2D => "image2D",
            Dx12ResourceType::StructuredBuffer => "buffer",
            Dx12ResourceType::RWStructuredBuffer => "buffer",
            Dx12ResourceType::SamplerState => "sampler",
        }
    }
}

/// Result of cross-compiling HLSL → GLSL for OpenGL
#[derive(Debug, Clone)]
pub struct HlslCrossCompiled {
    pub original_model: HlslShaderModel,
    pub glsl_source: String,
    pub stage: ShaderStage,
    pub glsl_version: &'static str,
}

// =========================================================================
// PTX → OpenGL compute shader bridge (CUDA → OpenGL)
// =========================================================================

/// PTX register mapped to GLSL compute shader
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PtxToGlMapping {
    /// %tid.x → gl_LocalInvocationID.x
    ThreadIdX,
    /// %tid.y → gl_LocalInvocationID.y
    ThreadIdY,
    /// %tid.z → gl_LocalInvocationID.z
    ThreadIdZ,
    /// %ctaid.x → gl_WorkGroupID.x
    BlockIdX,
    /// %ctaid.y → gl_WorkGroupID.y
    BlockIdY,
    /// %ctaid.z → gl_WorkGroupID.z
    BlockIdZ,
    /// %ntid.x → gl_WorkGroupSize.x
    BlockDimX,
    /// %ntid.y → gl_WorkGroupSize.y
    BlockDimY,
    /// %ntid.z → gl_WorkGroupSize.z
    BlockDimZ,
    /// %nctaid.x → gl_NumWorkGroups.x
    GridDimX,
    /// %nctaid.y → gl_NumWorkGroups.y
    GridDimY,
    /// %nctaid.z → gl_NumWorkGroups.z
    GridDimZ,
}

impl PtxToGlMapping {
    pub fn ptx_register(&self) -> &'static str {
        match self {
            PtxToGlMapping::ThreadIdX => "%tid.x",
            PtxToGlMapping::ThreadIdY => "%tid.y",
            PtxToGlMapping::ThreadIdZ => "%tid.z",
            PtxToGlMapping::BlockIdX  => "%ctaid.x",
            PtxToGlMapping::BlockIdY  => "%ctaid.y",
            PtxToGlMapping::BlockIdZ  => "%ctaid.z",
            PtxToGlMapping::BlockDimX => "%ntid.x",
            PtxToGlMapping::BlockDimY => "%ntid.y",
            PtxToGlMapping::BlockDimZ => "%ntid.z",
            PtxToGlMapping::GridDimX  => "%nctaid.x",
            PtxToGlMapping::GridDimY  => "%nctaid.y",
            PtxToGlMapping::GridDimZ  => "%nctaid.z",
        }
    }

    pub fn glsl_builtin(&self) -> &'static str {
        match self {
            PtxToGlMapping::ThreadIdX => "gl_LocalInvocationID.x",
            PtxToGlMapping::ThreadIdY => "gl_LocalInvocationID.y",
            PtxToGlMapping::ThreadIdZ => "gl_LocalInvocationID.z",
            PtxToGlMapping::BlockIdX  => "gl_WorkGroupID.x",
            PtxToGlMapping::BlockIdY  => "gl_WorkGroupID.y",
            PtxToGlMapping::BlockIdZ  => "gl_WorkGroupID.z",
            PtxToGlMapping::BlockDimX => "gl_WorkGroupSize.x",
            PtxToGlMapping::BlockDimY => "gl_WorkGroupSize.y",
            PtxToGlMapping::BlockDimZ => "gl_WorkGroupSize.z",
            PtxToGlMapping::GridDimX  => "gl_NumWorkGroups.x",
            PtxToGlMapping::GridDimY  => "gl_NumWorkGroups.y",
            PtxToGlMapping::GridDimZ  => "gl_NumWorkGroups.z",
        }
    }
}

/// CUDA memory space → OpenGL equivalent
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CudaMemorySpace {
    Global,   // __global__ → SSBO (Shader Storage Buffer Object)
    Shared,   // __shared__ → shared (workgroup local)
    Constant, // __constant__ → Uniform Buffer Object
    Local,    // __local__ → local variables
}

impl CudaMemorySpace {
    pub fn to_glsl_qualifier(&self) -> &'static str {
        match self {
            CudaMemorySpace::Global   => "buffer",   // SSBO layout
            CudaMemorySpace::Shared   => "shared",   // workgroup shared
            CudaMemorySpace::Constant => "uniform",  // UBO
            CudaMemorySpace::Local    => "",          // no qualifier needed
        }
    }
}

// =========================================================================
// Universal Shader Loader — the v4.7 entry point
// =========================================================================

/// A shader from any API, ready for OpenGL consumption
#[derive(Debug, Clone)]
pub enum UniversalShader {
    /// Native GLSL — pass through directly
    Glsl(ShaderSource),
    /// SPIR-V binary — use glShaderBinary + glSpecializeShader (GL 4.6)
    SpirV(SpirVShader),
    /// Cross-compiled from HLSL → GLSL
    FromHlsl(HlslCrossCompiled),
    /// Cross-compiled from PTX → GLSL compute shader
    FromPtx {
        glsl_compute: ShaderSource,
        work_group_size: [u32; 3],
    },
}

impl UniversalShader {
    /// Get the shader stage regardless of source format
    pub fn stage(&self) -> ShaderStage {
        match self {
            UniversalShader::Glsl(src) => src.stage,
            UniversalShader::SpirV(spv) => spv.stage,
            UniversalShader::FromHlsl(hlsl) => hlsl.stage,
            UniversalShader::FromPtx { .. } => ShaderStage::Compute,
        }
    }

    /// Get the source format
    pub fn format(&self) -> ShaderFormat {
        match self {
            UniversalShader::Glsl(_) => ShaderFormat::Glsl,
            UniversalShader::SpirV(_) => ShaderFormat::SpirV,
            UniversalShader::FromHlsl(_) => ShaderFormat::Hlsl,
            UniversalShader::FromPtx { .. } => ShaderFormat::Ptx,
        }
    }

    /// Is this shader ready for GL without cross-compilation?
    pub fn is_native_gl(&self) -> bool {
        matches!(self, UniversalShader::Glsl(_) | UniversalShader::SpirV(_))
    }

    /// Get GLSL source (for GLSL, cross-compiled HLSL, and PTX)
    pub fn to_glsl_source(&self) -> Option<&str> {
        match self {
            UniversalShader::Glsl(src) => Some(&src.source),
            UniversalShader::FromHlsl(hlsl) => Some(&hlsl.glsl_source),
            UniversalShader::FromPtx { glsl_compute, .. } => Some(&glsl_compute.source),
            UniversalShader::SpirV(_) => None, // binary, no GLSL
        }
    }
}

/// Load a shader from raw bytes, auto-detecting the format
pub fn load_shader_auto(data: &[u8], stage: ShaderStage) -> Option<UniversalShader> {
    let format = ShaderFormat::from_magic(data)?;
    match format {
        ShaderFormat::Glsl => {
            let source = core::str::from_utf8(data).ok()?;
            Some(UniversalShader::Glsl(ShaderSource::new(stage, source)))
        }
        ShaderFormat::SpirV => {
            Some(UniversalShader::SpirV(SpirVShader::new(data.to_vec(), stage)))
        }
        ShaderFormat::Ptx => {
            let ptx_src = core::str::from_utf8(data).ok()?;
            let glsl = ptx_to_glsl_stub(ptx_src);
            Some(UniversalShader::FromPtx {
                glsl_compute: ShaderSource::compute(&glsl),
                work_group_size: [256, 1, 1],
            })
        }
        ShaderFormat::Hlsl => {
            let hlsl_src = core::str::from_utf8(data).ok()?;
            let cross = hlsl_to_glsl_stub(hlsl_src, stage);
            Some(UniversalShader::FromHlsl(cross))
        }
        ShaderFormat::Dxbc => {
            // DXBC → SPIR-V path (binary cross-compile)
            Some(UniversalShader::SpirV(SpirVShader::new(
                dxbc_to_spirv_stub(data),
                stage,
            )))
        }
    }
}

/// Load a shader from file path, auto-detecting by extension
pub fn load_shader_from_path(path: &str, stage: ShaderStage) -> Option<UniversalShader> {
    let format = ShaderFormat::from_extension(path)?;
    // This returns the format; actual file I/O is handled by the caller
    // who passes the bytes to load_shader_auto()
    Some(match format {
        ShaderFormat::Glsl => UniversalShader::Glsl(ShaderSource::new(stage, "")),
        ShaderFormat::SpirV => UniversalShader::SpirV(SpirVShader::new(Vec::new(), stage)),
        _ => return None, // caller must read file and use load_shader_auto
    })
}

// =========================================================================
// Cross-compilation stubs (estructura para futura implementación)
// =========================================================================

/// Stub: PTX → GLSL compute shader
fn ptx_to_glsl_stub(ptx_source: &str) -> String {
    // PTX kernel → GLSL compute shader mapping:
    // .entry kernel_name → layout(local_size_x=256) in; void main()
    // ld.global → SSBO read
    // st.global → SSBO write
    // bar.sync → memoryBarrierShared(); barrier();
    format!(
        "#version 460 core\n\
         layout(local_size_x = 256) in;\n\
         // Auto-translated from PTX by ADead-BIB v4.7\n\
         // Original PTX: {} bytes\n\
         layout(std430, binding = 0) buffer DataIn {{ float data_in[]; }};\n\
         layout(std430, binding = 1) buffer DataOut {{ float data_out[]; }};\n\
         void main() {{\n\
         \tuint gid = gl_GlobalInvocationID.x;\n\
         \tdata_out[gid] = data_in[gid];\n\
         }}\n",
        ptx_source.len()
    )
}

/// Stub: HLSL → GLSL cross-compilation
fn hlsl_to_glsl_stub(hlsl_source: &str, stage: ShaderStage) -> HlslCrossCompiled {
    // HLSL → GLSL mapping skeleton:
    // cbuffer → uniform block (std140)
    // Texture2D + SamplerState → sampler2D
    // float4 → vec4, float3x3 → mat3
    // SV_Position → gl_Position
    // SV_Target → fragColor (layout(location=0) out)
    let glsl_ver = "460 core";
    let glsl = format!(
        "#version {}\n\
         // Auto-translated from HLSL by ADead-BIB v4.7\n\
         // Original HLSL: {} bytes\n\
         void main() {{\n\
         }}\n",
        glsl_ver,
        hlsl_source.len()
    );
    HlslCrossCompiled {
        original_model: HlslShaderModel::SM5_1,
        glsl_source: glsl,
        stage,
        glsl_version: "4.60",
    }
}

/// Stub: DXBC → SPIR-V cross-compilation
fn dxbc_to_spirv_stub(dxbc_data: &[u8]) -> Vec<u8> {
    // DXBC bytecode → SPIR-V binary path
    // This would use the ADead-BIB SPIR-V backend to emit valid SPIR-V
    // from decompiled DXBC instructions
    let _ = dxbc_data;
    // Return empty SPIR-V with just magic number as placeholder
    vec![0x03, 0x02, 0x23, 0x07]
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_detection_by_extension() {
        assert_eq!(ShaderFormat::from_extension("shader.vert"), Some(ShaderFormat::Glsl));
        assert_eq!(ShaderFormat::from_extension("shader.frag"), Some(ShaderFormat::Glsl));
        assert_eq!(ShaderFormat::from_extension("shader.glsl"), Some(ShaderFormat::Glsl));
        assert_eq!(ShaderFormat::from_extension("shader.spv"), Some(ShaderFormat::SpirV));
        assert_eq!(ShaderFormat::from_extension("shader.hlsl"), Some(ShaderFormat::Hlsl));
        assert_eq!(ShaderFormat::from_extension("shader.dxbc"), Some(ShaderFormat::Dxbc));
        assert_eq!(ShaderFormat::from_extension("kernel.ptx"), Some(ShaderFormat::Ptx));
        assert_eq!(ShaderFormat::from_extension("readme.txt"), None);
    }

    #[test]
    fn test_format_detection_by_magic() {
        // SPIR-V magic
        assert_eq!(ShaderFormat::from_magic(&[0x03, 0x02, 0x23, 0x07]), Some(ShaderFormat::SpirV));
        // DXBC magic
        assert_eq!(ShaderFormat::from_magic(b"DXBC1234"), Some(ShaderFormat::Dxbc));
        // PTX magic
        assert_eq!(ShaderFormat::from_magic(b".version 7.0"), Some(ShaderFormat::Ptx));
        // GLSL (#version)
        assert_eq!(ShaderFormat::from_magic(b"#version 460"), Some(ShaderFormat::Glsl));
    }

    #[test]
    fn test_spirv_shader_validate() {
        let spv = SpirVShader::new(vec![0x03, 0x02, 0x23, 0x07, 0, 0, 0, 0], ShaderStage::Compute);
        assert!(spv.validate_magic());
        assert_eq!(spv.word_count(), 2);
        assert_eq!(spv.entry_point, "main");
    }

    #[test]
    fn test_spirv_shader_invalid() {
        let spv = SpirVShader::new(vec![0xFF, 0xFF, 0xFF, 0xFF], ShaderStage::Vertex);
        assert!(!spv.validate_magic());
    }

    #[test]
    fn test_universal_shader_stage() {
        let glsl = UniversalShader::Glsl(ShaderSource::vertex("void main() {}"));
        assert_eq!(glsl.stage(), ShaderStage::Vertex);
        assert_eq!(glsl.format(), ShaderFormat::Glsl);
        assert!(glsl.is_native_gl());

        let spv = UniversalShader::SpirV(SpirVShader::new(vec![], ShaderStage::Fragment));
        assert_eq!(spv.stage(), ShaderStage::Fragment);
        assert!(spv.is_native_gl());
    }

    #[test]
    fn test_load_auto_glsl() {
        let data = b"#version 460 core\nvoid main() {}";
        let shader = load_shader_auto(data, ShaderStage::Vertex).unwrap();
        assert_eq!(shader.format(), ShaderFormat::Glsl);
        assert!(shader.to_glsl_source().is_some());
    }

    #[test]
    fn test_load_auto_spirv() {
        let data = &[0x03, 0x02, 0x23, 0x07, 0, 0, 0, 0];
        let shader = load_shader_auto(data, ShaderStage::Compute).unwrap();
        assert_eq!(shader.format(), ShaderFormat::SpirV);
        assert!(shader.to_glsl_source().is_none());
    }

    #[test]
    fn test_ptx_to_gl_mapping() {
        assert_eq!(PtxToGlMapping::ThreadIdX.ptx_register(), "%tid.x");
        assert_eq!(PtxToGlMapping::ThreadIdX.glsl_builtin(), "gl_LocalInvocationID.x");
        assert_eq!(PtxToGlMapping::BlockIdX.ptx_register(), "%ctaid.x");
        assert_eq!(PtxToGlMapping::BlockIdX.glsl_builtin(), "gl_WorkGroupID.x");
    }

    #[test]
    fn test_cuda_memory_space_mapping() {
        assert_eq!(CudaMemorySpace::Global.to_glsl_qualifier(), "buffer");
        assert_eq!(CudaMemorySpace::Shared.to_glsl_qualifier(), "shared");
        assert_eq!(CudaMemorySpace::Constant.to_glsl_qualifier(), "uniform");
    }

    #[test]
    fn test_dx12_resource_mapping() {
        assert_eq!(Dx12ResourceType::ConstantBuffer.to_glsl_type(), "uniform");
        assert_eq!(Dx12ResourceType::Texture2D.to_glsl_type(), "sampler2D");
        assert_eq!(Dx12ResourceType::RWStructuredBuffer.to_glsl_type(), "buffer");
    }

    #[test]
    fn test_hlsl_semantics_table() {
        let sv_pos = HLSL_TO_GLSL_SEMANTICS.iter().find(|s| s.hlsl_name == "SV_Position");
        assert!(sv_pos.is_some());
        assert_eq!(sv_pos.unwrap().glsl_equivalent, "gl_Position");

        let dispatch = HLSL_TO_GLSL_SEMANTICS.iter().find(|s| s.hlsl_name == "SV_DispatchThreadID");
        assert!(dispatch.is_some());
        assert_eq!(dispatch.unwrap().glsl_equivalent, "gl_GlobalInvocationID");
    }
}
