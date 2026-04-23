// ============================================================
// fastos_dxgi.rs — DXGI (DirectX Graphics Infrastructure)
// ============================================================
// DXGI is the foundation layer shared by DX10, DX11, and DX12.
// Manages swap chains, adapters, outputs, and surface formats.
// DLL: dxgi.dll
// ============================================================

// ── DXGI Functions (dxgi.dll) ──
pub const DXGI_FUNCTIONS: &[&str] = &[
    // Factory creation
    "CreateDXGIFactory", "CreateDXGIFactory1", "CreateDXGIFactory2",
    // Debug
    "DXGIGetDebugInterface1",
];

// ── DXGI Interfaces ──
pub const DXGI_INTERFACES: &[&str] = &[
    // Core interfaces
    "IDXGIObject", "IDXGIDeviceSubObject",
    "IDXGIFactory", "IDXGIFactory1", "IDXGIFactory2",
    "IDXGIFactory3", "IDXGIFactory4", "IDXGIFactory5", "IDXGIFactory6",
    "IDXGIAdapter", "IDXGIAdapter1", "IDXGIAdapter2", "IDXGIAdapter3", "IDXGIAdapter4",
    "IDXGIOutput", "IDXGIOutput1", "IDXGIOutput2", "IDXGIOutput3",
    "IDXGIOutput4", "IDXGIOutput5", "IDXGIOutput6",
    "IDXGISwapChain", "IDXGISwapChain1", "IDXGISwapChain2",
    "IDXGISwapChain3", "IDXGISwapChain4",
    "IDXGIDevice", "IDXGIDevice1", "IDXGIDevice2", "IDXGIDevice3", "IDXGIDevice4",
    "IDXGISurface", "IDXGISurface1", "IDXGISurface2",
    "IDXGIResource", "IDXGIResource1",
    "IDXGIKeyedMutex",
    "IDXGIInfoQueue", "IDXGIDebug", "IDXGIDebug1",
];

// ── DXGI Types / Structs ──
pub const DXGI_TYPES: &[&str] = &[
    "DXGI_SWAP_CHAIN_DESC", "DXGI_SWAP_CHAIN_DESC1",
    "DXGI_SWAP_CHAIN_FULLSCREEN_DESC",
    "DXGI_MODE_DESC", "DXGI_MODE_DESC1",
    "DXGI_SAMPLE_DESC", "DXGI_RATIONAL",
    "DXGI_ADAPTER_DESC", "DXGI_ADAPTER_DESC1", "DXGI_ADAPTER_DESC2", "DXGI_ADAPTER_DESC3",
    "DXGI_OUTPUT_DESC", "DXGI_OUTPUT_DESC1",
    "DXGI_SURFACE_DESC",
    "DXGI_MAPPED_RECT",
    "DXGI_FRAME_STATISTICS",
    "DXGI_SWAP_EFFECT",
    "DXGI_USAGE",
    "DXGI_FORMAT",
    "DXGI_SCALING", "DXGI_ALPHA_MODE",
    "DXGI_PRESENT_PARAMETERS",
    "DXGI_GPU_PREFERENCE",
];

// ── DXGI Format Constants ──
pub const DXGI_FORMAT_CONSTANTS: &[(&str, &str)] = &[
    ("DXGI_FORMAT_UNKNOWN", "0"),
    ("DXGI_FORMAT_R32G32B32A32_FLOAT", "2"),
    ("DXGI_FORMAT_R32G32B32A32_UINT", "3"),
    ("DXGI_FORMAT_R32G32B32_FLOAT", "6"),
    ("DXGI_FORMAT_R16G16B16A16_FLOAT", "10"),
    ("DXGI_FORMAT_R32G32_FLOAT", "16"),
    ("DXGI_FORMAT_R8G8B8A8_UNORM", "28"),
    ("DXGI_FORMAT_R8G8B8A8_UNORM_SRGB", "29"),
    ("DXGI_FORMAT_B8G8R8A8_UNORM", "87"),
    ("DXGI_FORMAT_B8G8R8A8_UNORM_SRGB", "91"),
    ("DXGI_FORMAT_R32_FLOAT", "41"),
    ("DXGI_FORMAT_R32_UINT", "42"),
    ("DXGI_FORMAT_D32_FLOAT", "40"),
    ("DXGI_FORMAT_D24_UNORM_S8_UINT", "45"),
    ("DXGI_FORMAT_D16_UNORM", "55"),
    ("DXGI_FORMAT_R16_FLOAT", "54"),
    ("DXGI_FORMAT_R8_UNORM", "61"),
    ("DXGI_FORMAT_BC1_UNORM", "71"),
    ("DXGI_FORMAT_BC3_UNORM", "77"),
    ("DXGI_FORMAT_BC7_UNORM", "98"),
];

// ── DXGI Constants ──
pub const DXGI_CONSTANTS: &[(&str, &str)] = &[
    ("DXGI_SWAP_EFFECT_DISCARD", "0"),
    ("DXGI_SWAP_EFFECT_SEQUENTIAL", "1"),
    ("DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL", "3"),
    ("DXGI_SWAP_EFFECT_FLIP_DISCARD", "4"),
    ("DXGI_USAGE_RENDER_TARGET_OUTPUT", "0x00000020"),
    ("DXGI_USAGE_SHADER_INPUT", "0x00000010"),
    ("DXGI_USAGE_BACK_BUFFER", "0x00000040"),
    ("DXGI_SCALING_STRETCH", "0"),
    ("DXGI_SCALING_NONE", "1"),
    ("DXGI_ALPHA_MODE_UNSPECIFIED", "0"),
    ("DXGI_ALPHA_MODE_PREMULTIPLIED", "1"),
    ("DXGI_ALPHA_MODE_STRAIGHT", "2"),
    ("DXGI_GPU_PREFERENCE_UNSPECIFIED", "0"),
    ("DXGI_GPU_PREFERENCE_MINIMUM_POWER", "1"),
    ("DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE", "2"),
    ("DXGI_MWA_NO_WINDOW_CHANGES", "0x1"),
    ("DXGI_MWA_NO_ALT_ENTER", "0x2"),
    ("DXGI_PRESENT_ALLOW_TEARING", "0x00000200"),
    ("DXGI_CREATE_FACTORY_DEBUG", "0x01"),
];

// ── Well-known DXGI IIDs ──
pub const DXGI_IIDS: &[(&str, &str)] = &[
    ("IID_IDXGIFactory", "{7b7166ec-21c7-44ae-b21a-c9ae321ae369}"),
    ("IID_IDXGIFactory1", "{770aae78-f26f-4dba-a829-253c83d1b387}"),
    ("IID_IDXGIFactory2", "{50c83a1c-e072-4c48-87b0-3630fa36a6d0}"),
    ("IID_IDXGIFactory4", "{1bc6ea02-ef36-464f-bf0c-21ca39e5168a}"),
    ("IID_IDXGIFactory6", "{c1b6694f-ff09-44a9-b03c-77900a0a1d17}"),
    ("IID_IDXGIAdapter1", "{29038f61-3839-4626-91fd-086879011a05}"),
    ("IID_IDXGISwapChain", "{310d36a0-d2e7-4c0a-aa04-6a9d23b8886a}"),
    ("IID_IDXGISwapChain3", "{94d99bdb-f1f8-4ab0-b236-7da0170edab1}"),
];

pub fn is_dxgi_symbol(name: &str) -> bool {
    DXGI_FUNCTIONS.contains(&name)
        || DXGI_INTERFACES.contains(&name)
        || DXGI_TYPES.contains(&name)
        || DXGI_FORMAT_CONSTANTS.iter().any(|(n, _)| *n == name)
        || DXGI_CONSTANTS.iter().any(|(n, _)| *n == name)
}
