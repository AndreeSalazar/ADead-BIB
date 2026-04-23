// ============================================================
// fastos_d3d9.rs — Direct3D 9 support
// ============================================================
// DLL: d3d9.dll
// The legacy but still widely-used DirectX 9 API.
// ============================================================

// ── D3D9 Functions (d3d9.dll) ──
pub const D3D9_FUNCTIONS: &[&str] = &[
    "Direct3DCreate9",
    "Direct3DCreate9Ex",
    "D3DPERF_BeginEvent", "D3DPERF_EndEvent",
    "D3DPERF_SetMarker", "D3DPERF_SetRegion",
    "D3DPERF_QueryRepeatFrame", "D3DPERF_SetOptions",
    "D3DPERF_GetStatus",
];

// ── D3D9 Interfaces ──
pub const D3D9_INTERFACES: &[&str] = &[
    "IDirect3D9", "IDirect3D9Ex",
    "IDirect3DDevice9", "IDirect3DDevice9Ex",
    "IDirect3DSwapChain9", "IDirect3DSwapChain9Ex",
    "IDirect3DTexture9", "IDirect3DCubeTexture9", "IDirect3DVolumeTexture9",
    "IDirect3DSurface9", "IDirect3DVolume9",
    "IDirect3DVertexBuffer9", "IDirect3DIndexBuffer9",
    "IDirect3DVertexDeclaration9", "IDirect3DVertexShader9", "IDirect3DPixelShader9",
    "IDirect3DStateBlock9", "IDirect3DQuery9",
    "IDirect3DResource9", "IDirect3DBaseTexture9",
];

// ── D3D9 Types / Structs ──
pub const D3D9_TYPES: &[&str] = &[
    "D3DPRESENT_PARAMETERS",
    "D3DDEVICE_CREATION_PARAMETERS",
    "D3DDISPLAYMODE", "D3DDISPLAYMODEEX", "D3DDISPLAYMODEFILTER",
    "D3DADAPTER_IDENTIFIER9",
    "D3DCAPS9",
    "D3DVIEWPORT9",
    "D3DMATERIAL9", "D3DLIGHT9",
    "D3DVERTEXELEMENT9",
    "D3DRECT",
    "D3DMATRIX",
    "D3DVECTOR",
    "D3DCOLORVALUE",
    "D3DLOCKED_RECT", "D3DLOCKED_BOX",
    "D3DBOX", "D3DVOLUME_DESC", "D3DSURFACE_DESC",
    "D3DCLIPSTATUS9",
    "D3DGAMMARAMP",
    "D3DINDEXBUFFER_DESC", "D3DVERTEXBUFFER_DESC",
];

// ── D3D9 Constants ──
pub const D3D9_CONSTANTS: &[(&str, &str)] = &[
    ("D3D_SDK_VERSION", "32"),
    ("D3DADAPTER_DEFAULT", "0"),
    ("D3DCREATE_SOFTWARE_VERTEXPROCESSING", "0x00000020"),
    ("D3DCREATE_HARDWARE_VERTEXPROCESSING", "0x00000040"),
    ("D3DCREATE_MIXED_VERTEXPROCESSING", "0x00000080"),
    ("D3DCREATE_MULTITHREADED", "0x00000004"),
    ("D3DCREATE_FPU_PRESERVE", "0x00000002"),
    ("D3DSWAPEFFECT_DISCARD", "1"),
    ("D3DSWAPEFFECT_FLIP", "2"),
    ("D3DSWAPEFFECT_COPY", "3"),
    ("D3DDEVTYPE_HAL", "1"),
    ("D3DDEVTYPE_REF", "2"),
    ("D3DDEVTYPE_SW", "3"),
    ("D3DDEVTYPE_NULLREF", "4"),
    ("D3DFMT_UNKNOWN", "0"),
    ("D3DFMT_R8G8B8", "20"),
    ("D3DFMT_A8R8G8B8", "21"),
    ("D3DFMT_X8R8G8B8", "22"),
    ("D3DFMT_R5G6B5", "23"),
    ("D3DFMT_D16", "80"),
    ("D3DFMT_D24S8", "75"),
    ("D3DFMT_D24X8", "77"),
    ("D3DFMT_D32", "71"),
    ("D3DRS_ZENABLE", "7"),
    ("D3DRS_FILLMODE", "8"),
    ("D3DRS_LIGHTING", "137"),
    ("D3DRS_CULLMODE", "22"),
    ("D3DRS_ALPHABLENDENABLE", "27"),
    ("D3DPT_POINTLIST", "1"),
    ("D3DPT_LINELIST", "2"),
    ("D3DPT_LINESTRIP", "3"),
    ("D3DPT_TRIANGLELIST", "4"),
    ("D3DPT_TRIANGLESTRIP", "5"),
    ("D3DPT_TRIANGLEFAN", "6"),
    ("D3DCLEAR_TARGET", "0x00000001"),
    ("D3DCLEAR_ZBUFFER", "0x00000002"),
    ("D3DCLEAR_STENCIL", "0x00000004"),
    ("D3DCOLOR_XRGB(r,g,b)", "((0xFF<<24)|((r)<<16)|((g)<<8)|(b))"),
    ("D3DCOLOR_ARGB(a,r,g,b)", "(((a)<<24)|((r)<<16)|((g)<<8)|(b))"),
    ("D3DFVF_XYZ", "0x002"),
    ("D3DFVF_XYZRHW", "0x004"),
    ("D3DFVF_DIFFUSE", "0x040"),
    ("D3DFVF_TEX1", "0x100"),
    ("D3DFVF_NORMAL", "0x010"),
    ("D3DTS_WORLD", "256"),
    ("D3DTS_VIEW", "2"),
    ("D3DTS_PROJECTION", "3"),
    ("D3DPOOL_DEFAULT", "0"),
    ("D3DPOOL_MANAGED", "1"),
    ("D3DPOOL_SYSTEMMEM", "2"),
    ("D3DUSAGE_WRITEONLY", "0x00000008"),
    ("D3DUSAGE_DYNAMIC", "0x00000200"),
    ("D3DLOCK_DISCARD", "0x00002000"),
    ("D3DLOCK_NOOVERWRITE", "0x00001000"),
];

pub fn is_d3d9_symbol(name: &str) -> bool {
    D3D9_FUNCTIONS.contains(&name)
        || D3D9_INTERFACES.contains(&name)
        || D3D9_TYPES.contains(&name)
        || D3D9_CONSTANTS.iter().any(|(n, _)| *n == name)
}
