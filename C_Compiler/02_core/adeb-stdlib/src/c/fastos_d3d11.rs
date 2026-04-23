// ============================================================
// fastos_d3d11.rs — Direct3D 11 support
// ============================================================
// DLL: d3d11.dll
// The most widely-used modern DirectX API.
// ============================================================

// ── D3D11 Functions (d3d11.dll) ──
pub const D3D11_FUNCTIONS: &[&str] = &[
    "D3D11CreateDevice",
    "D3D11CreateDeviceAndSwapChain",
    "D3D11On12CreateDevice",
];

// ── D3D11 Interfaces ──
pub const D3D11_INTERFACES: &[&str] = &[
    // Device
    "ID3D11Device", "ID3D11Device1", "ID3D11Device2",
    "ID3D11Device3", "ID3D11Device4", "ID3D11Device5",
    // Context
    "ID3D11DeviceContext", "ID3D11DeviceContext1", "ID3D11DeviceContext2",
    "ID3D11DeviceContext3", "ID3D11DeviceContext4",
    // Resources
    "ID3D11Resource", "ID3D11Buffer", "ID3D11Texture1D",
    "ID3D11Texture2D", "ID3D11Texture2D1", "ID3D11Texture3D", "ID3D11Texture3D1",
    // Views
    "ID3D11View",
    "ID3D11RenderTargetView", "ID3D11RenderTargetView1",
    "ID3D11DepthStencilView",
    "ID3D11ShaderResourceView", "ID3D11ShaderResourceView1",
    "ID3D11UnorderedAccessView", "ID3D11UnorderedAccessView1",
    // Shaders
    "ID3D11VertexShader", "ID3D11PixelShader",
    "ID3D11GeometryShader", "ID3D11HullShader", "ID3D11DomainShader",
    "ID3D11ComputeShader",
    // State
    "ID3D11InputLayout",
    "ID3D11BlendState", "ID3D11BlendState1",
    "ID3D11DepthStencilState",
    "ID3D11RasterizerState", "ID3D11RasterizerState1", "ID3D11RasterizerState2",
    "ID3D11SamplerState",
    // Pipeline
    "ID3D11ClassInstance", "ID3D11ClassLinkage",
    "ID3D11CommandList",
    // Query
    "ID3D11Query", "ID3D11Query1", "ID3D11Predicate", "ID3D11Counter",
    "ID3D11Asynchronous",
    // Misc
    "ID3D11Multithread",
    "ID3D11Debug", "ID3D11InfoQueue",
    "ID3D11Fence",
    "ID3DDeviceContextState",
    "ID3DUserDefinedAnnotation",
];

// ── D3D11 Types / Structs ──
pub const D3D11_TYPES: &[&str] = &[
    "D3D11_BUFFER_DESC", "D3D11_TEXTURE1D_DESC",
    "D3D11_TEXTURE2D_DESC", "D3D11_TEXTURE2D_DESC1",
    "D3D11_TEXTURE3D_DESC", "D3D11_TEXTURE3D_DESC1",
    "D3D11_SUBRESOURCE_DATA",
    "D3D11_MAPPED_SUBRESOURCE",
    "D3D11_RENDER_TARGET_VIEW_DESC", "D3D11_RENDER_TARGET_VIEW_DESC1",
    "D3D11_DEPTH_STENCIL_VIEW_DESC",
    "D3D11_SHADER_RESOURCE_VIEW_DESC", "D3D11_SHADER_RESOURCE_VIEW_DESC1",
    "D3D11_UNORDERED_ACCESS_VIEW_DESC", "D3D11_UNORDERED_ACCESS_VIEW_DESC1",
    "D3D11_INPUT_ELEMENT_DESC",
    "D3D11_VIEWPORT",
    "D3D11_RECT",
    "D3D11_BOX",
    "D3D11_BLEND_DESC", "D3D11_BLEND_DESC1",
    "D3D11_DEPTH_STENCIL_DESC",
    "D3D11_RASTERIZER_DESC", "D3D11_RASTERIZER_DESC1", "D3D11_RASTERIZER_DESC2",
    "D3D11_SAMPLER_DESC",
    "D3D11_QUERY_DESC", "D3D11_QUERY_DESC1",
    "D3D11_COUNTER_DESC",
    "D3D11_SO_DECLARATION_ENTRY",
    "D3D11_FEATURE_DATA_THREADING",
    "D3D11_FEATURE_DATA_DOUBLES",
    "D3D11_FEATURE_DATA_D3D11_OPTIONS",
    "D3D_FEATURE_LEVEL",
    "D3D11_USAGE",
    "D3D11_BIND_FLAG",
    "D3D11_CPU_ACCESS_FLAG",
    "D3D11_MAP",
    "D3D11_PRIMITIVE_TOPOLOGY",
];

// ── D3D11 Constants ──
pub const D3D11_CONSTANTS: &[(&str, &str)] = &[
    // Feature levels
    ("D3D_FEATURE_LEVEL_9_1", "0x9100"),
    ("D3D_FEATURE_LEVEL_9_2", "0x9200"),
    ("D3D_FEATURE_LEVEL_9_3", "0x9300"),
    ("D3D_FEATURE_LEVEL_10_0", "0xa000"),
    ("D3D_FEATURE_LEVEL_10_1", "0xa100"),
    ("D3D_FEATURE_LEVEL_11_0", "0xb000"),
    ("D3D_FEATURE_LEVEL_11_1", "0xb100"),
    ("D3D_FEATURE_LEVEL_12_0", "0xc000"),
    ("D3D_FEATURE_LEVEL_12_1", "0xc100"),
    ("D3D_FEATURE_LEVEL_12_2", "0xc200"),
    // Driver types
    ("D3D_DRIVER_TYPE_UNKNOWN", "0"),
    ("D3D_DRIVER_TYPE_HARDWARE", "1"),
    ("D3D_DRIVER_TYPE_REFERENCE", "2"),
    ("D3D_DRIVER_TYPE_NULL", "3"),
    ("D3D_DRIVER_TYPE_SOFTWARE", "4"),
    ("D3D_DRIVER_TYPE_WARP", "5"),
    // Create flags
    ("D3D11_CREATE_DEVICE_SINGLETHREADED", "0x1"),
    ("D3D11_CREATE_DEVICE_DEBUG", "0x2"),
    ("D3D11_CREATE_DEVICE_BGRA_SUPPORT", "0x20"),
    // Bind flags
    ("D3D11_BIND_VERTEX_BUFFER", "0x1"),
    ("D3D11_BIND_INDEX_BUFFER", "0x2"),
    ("D3D11_BIND_CONSTANT_BUFFER", "0x4"),
    ("D3D11_BIND_SHADER_RESOURCE", "0x8"),
    ("D3D11_BIND_STREAM_OUTPUT", "0x10"),
    ("D3D11_BIND_RENDER_TARGET", "0x20"),
    ("D3D11_BIND_DEPTH_STENCIL", "0x40"),
    ("D3D11_BIND_UNORDERED_ACCESS", "0x80"),
    // Usage
    ("D3D11_USAGE_DEFAULT", "0"),
    ("D3D11_USAGE_IMMUTABLE", "1"),
    ("D3D11_USAGE_DYNAMIC", "2"),
    ("D3D11_USAGE_STAGING", "3"),
    // CPU access
    ("D3D11_CPU_ACCESS_WRITE", "0x10000"),
    ("D3D11_CPU_ACCESS_READ", "0x20000"),
    // Map
    ("D3D11_MAP_READ", "1"),
    ("D3D11_MAP_WRITE", "2"),
    ("D3D11_MAP_READ_WRITE", "3"),
    ("D3D11_MAP_WRITE_DISCARD", "4"),
    ("D3D11_MAP_WRITE_NO_OVERWRITE", "5"),
    // Primitive topology
    ("D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST", "4"),
    ("D3D11_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP", "5"),
    ("D3D11_PRIMITIVE_TOPOLOGY_LINELIST", "2"),
    ("D3D11_PRIMITIVE_TOPOLOGY_LINESTRIP", "3"),
    ("D3D11_PRIMITIVE_TOPOLOGY_POINTLIST", "1"),
    // Clear flags
    ("D3D11_CLEAR_DEPTH", "0x1"),
    ("D3D11_CLEAR_STENCIL", "0x2"),
    // Input classification
    ("D3D11_INPUT_PER_VERTEX_DATA", "0"),
    ("D3D11_INPUT_PER_INSTANCE_DATA", "1"),
    // Fill/cull mode
    ("D3D11_FILL_WIREFRAME", "2"),
    ("D3D11_FILL_SOLID", "3"),
    ("D3D11_CULL_NONE", "1"),
    ("D3D11_CULL_FRONT", "2"),
    ("D3D11_CULL_BACK", "3"),
    // Blend
    ("D3D11_BLEND_ZERO", "1"),
    ("D3D11_BLEND_ONE", "2"),
    ("D3D11_BLEND_SRC_ALPHA", "5"),
    ("D3D11_BLEND_INV_SRC_ALPHA", "6"),
    ("D3D11_BLEND_OP_ADD", "1"),
    // Filter
    ("D3D11_FILTER_MIN_MAG_MIP_POINT", "0"),
    ("D3D11_FILTER_MIN_MAG_MIP_LINEAR", "0x15"),
    ("D3D11_FILTER_ANISOTROPIC", "0x55"),
    // Texture address
    ("D3D11_TEXTURE_ADDRESS_WRAP", "1"),
    ("D3D11_TEXTURE_ADDRESS_MIRROR", "2"),
    ("D3D11_TEXTURE_ADDRESS_CLAMP", "3"),
    // Comparison
    ("D3D11_COMPARISON_LESS", "2"),
    ("D3D11_COMPARISON_LESS_EQUAL", "4"),
    ("D3D11_COMPARISON_ALWAYS", "8"),
    // SDK
    ("D3D11_SDK_VERSION", "7"),
    // Append aligned element
    ("D3D11_APPEND_ALIGNED_ELEMENT", "0xFFFFFFFF"),
];

// ── D3D11 Shader Compiler (d3dcompiler_47.dll) ──
pub const D3DCOMPILER_FUNCTIONS: &[&str] = &[
    "D3DCompile", "D3DCompile2",
    "D3DCompileFromFile",
    "D3DCreateBlob",
    "D3DDisassemble",
    "D3DGetBlobPart",
    "D3DReflect",
    "D3DStripShader",
    "D3DReadFileToBlob", "D3DWriteBlobToFile",
    "D3DPreprocess",
    "D3DGetDebugInfo",
    "D3DGetInputSignatureBlob", "D3DGetOutputSignatureBlob",
    "D3DGetInputAndOutputSignatureBlob",
];

pub fn is_d3d11_symbol(name: &str) -> bool {
    D3D11_FUNCTIONS.contains(&name)
        || D3D11_INTERFACES.contains(&name)
        || D3D11_TYPES.contains(&name)
        || D3D11_CONSTANTS.iter().any(|(n, _)| *n == name)
        || D3DCOMPILER_FUNCTIONS.contains(&name)
}
