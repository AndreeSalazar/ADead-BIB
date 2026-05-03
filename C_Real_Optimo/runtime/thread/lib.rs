//! ADead Runtime - THREAD Module
//!
//! Funciones generadas automáticamente desde knowledge.json
//! Total: 1718 funciones

use std::ffi::c_void;

#[no_mangle]
pub unsafe extern "C" fn CreateSecurityPage(psi: LPSECURITYINFO) -> HPROPSHEETPAGE {
    // TODO: implementar CreateSecurityPage desde wine/aclui.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlAxCreateControl(param_21225: LPCOLESTR, param_11550: *mut core::ffi::c_void, param_39451: *mut IStream, param_57333: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar AtlAxCreateControl desde wine/atlbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlAxCreateControlEx(param_21225: LPCOLESTR, param_11550: *mut core::ffi::c_void, param_39451: *mut IStream, param_57333: *mut core::ffi::c_void, param_57333: *mut core::ffi::c_void, param_55479: REFIID, param_46438: *mut IUnknown) -> i32 {
    // TODO: implementar AtlAxCreateControlEx desde wine/atlbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlModuleAddCreateWndData(pM: *mut _ATL_MODULEW, pData: *mut _AtlCreateWndData, pvObject: *mut core::ffi::c_void) -> core::ffi::c_void {
    // TODO: implementar AtlModuleAddCreateWndData desde wine/atlbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlWinModuleAddCreateWndData(param_39283: *mut _ATL_WIN_MODULE, param_7265: *mut _AtlCreateWndData, param_64866: *mut core::ffi::c_void) -> core::ffi::c_void {
    // TODO: implementar AtlWinModuleAddCreateWndData desde wine/atlbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlWinModuleExtractCreateWndData(param_39283: *mut _ATL_WIN_MODULE) -> *mut core::ffi::c_void {
    // TODO: implementar AtlWinModuleExtractCreateWndData desde wine/atlbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlModuleExtractCreateWndData(pM: *mut _ATL_MODULEW) -> *mut core::ffi::c_void {
    // TODO: implementar AtlModuleExtractCreateWndData desde wine/atlbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlCreateRegistrar(param_31579: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar AtlCreateRegistrar desde wine/atlbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlCreateTargetDC(hdc: HDC, ptd: *mut DVTARGETDEVICE) -> HDC {
    // TODO: implementar AtlCreateTargetDC desde wine/atlwin.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BCryptCreateHash(param_63923: BCRYPT_ALG_HANDLE, param_42316: *mut BCRYPT_HASH_HANDLE, param_12278: PUCHAR, param_30140: ULONG, param_12278: PUCHAR, param_30140: ULONG, param_30140: ULONG) -> NTSTATUS {
    // TODO: implementar BCryptCreateHash desde wine/bcrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CM_Create_DevNode_ExA(node: *mut DEVINST, instance_id: DEVINSTID_A, parent: DEVINST, flags: ULONG, machine: HMACHINE) -> CMAPI CONFIGRET {
    // TODO: implementar CM_Create_DevNode_ExA desde wine/cfgmgr32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CM_Create_DevNode_ExW(node: *mut DEVINST, instance_id: DEVINSTID_W, parent: DEVINST, flags: ULONG, machine: HMACHINE) -> CMAPI CONFIGRET {
    // TODO: implementar CM_Create_DevNode_ExW desde wine/cfgmgr32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CM_Create_DevNodeA(node: *mut DEVINST, instance_id: DEVINSTID_A, parent: DEVINST, flags: ULONG) -> CMAPI CONFIGRET {
    // TODO: implementar CM_Create_DevNodeA desde wine/cfgmgr32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CM_Create_DevNodeW(node: *mut DEVINST, instance_id: DEVINSTID_W, parent: DEVINST, flags: ULONG) -> CMAPI CONFIGRET {
    // TODO: implementar CM_Create_DevNodeW desde wine/cfgmgr32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CM_Create_Range_List(ranges: *mut RANGE_LIST, flags: ULONG) -> CMAPI CONFIGRET {
    // TODO: implementar CM_Create_Range_List desde wine/cfgmgr32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateStatusWindowA(param_9910: LONG, param_15619: LPCSTR, param_11550: *mut core::ffi::c_void, param_2971: UINT) -> WINCOMMCTRLAPI HWND {
    // TODO: implementar CreateStatusWindowA desde wine/commctrl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateStatusWindowW(param_9910: LONG, param_25711: LPCWSTR, param_11550: *mut core::ffi::c_void, param_2971: UINT) -> WINCOMMCTRLAPI HWND {
    // TODO: implementar CreateStatusWindowW desde wine/commctrl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateUpDownControl(param_54075: u32, param_18538: INT, param_18538: INT, param_18538: INT, param_18538: INT, param_11550: *mut core::ffi::c_void, param_18538: INT, param_49462: HINSTANCE, param_11550: *mut core::ffi::c_void, param_18538: INT, param_18538: INT, param_18538: INT) -> WINCOMMCTRLAPI HWND {
    // TODO: implementar CreateUpDownControl desde wine/commctrl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ImageList_Create(param_18538: INT, param_18538: INT, param_2971: UINT, param_18538: INT, param_18538: INT) -> WINCOMMCTRLAPI HIMAGELIST {
    // TODO: implementar ImageList_Create desde wine/commctrl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ImageList_DragShowNolock(param_16716: i32) -> WINCOMMCTRLAPI BOOL {
    // TODO: implementar ImageList_DragShowNolock desde wine/commctrl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateToolbar() -> i32 {
    // TODO: implementar CreateToolbar desde reactos/appview.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateToolbarEx(param_11550: *mut core::ffi::c_void, param_54075: u32, param_2971: UINT, param_18538: INT, param_49462: HINSTANCE, param_49034: UINT_PTR, param_57843: LPCTBBUTTON, param_18538: INT, param_18538: INT, param_18538: INT, param_18538: INT, param_18538: INT, param_2971: UINT) -> *mut core::ffi::c_void {
    // TODO: implementar CreateToolbarEx desde wine/commctrl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateMappedBitmap(param_49462: HINSTANCE, param_64027: INT_PTR, param_2971: UINT, param_62990: LPCOLORMAP, param_18538: INT) -> HBITMAP {
    // TODO: implementar CreateMappedBitmap desde wine/commctrl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DSA_Create(param_18538: INT, param_18538: INT) -> WINCOMMCTRLAPI HDSA {
    // TODO: implementar DSA_Create desde wine/commctrl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DPA_Create(param_18538: INT) -> WINCOMMCTRLAPI HDPA {
    // TODO: implementar DPA_Create desde wine/commctrl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCompressor(param_54075: u32, param_36291: *mut COMPRESS_ALLOCATION_ROUTINES, param_57076: *mut COMPRESSOR_HANDLE) -> i32 {
    // TODO: implementar CreateCompressor desde wine/compressapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDecompressor(param_54075: u32, param_36291: *mut COMPRESS_ALLOCATION_ROUTINES, param_44215: *mut DECOMPRESSOR_HANDLE) -> i32 {
    // TODO: implementar CreateDecompressor desde wine/compressapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreatePseudoConsole(param_14752: COORD, param_31864: *mut core::ffi::c_void, param_31864: *mut core::ffi::c_void, param_54075: u32, param_57203: *mut HPCON) -> WINBASEAPI HRESULT {
    // TODO: implementar CreatePseudoConsole desde wine/consoleapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3D10CreateDevice(adapter: *mut IDXGIAdapter, driver_type: D3D10_DRIVER_TYPE, swrast: HMODULE, flags: UINT, sdk_version: UINT, param_7394: *mut ID3D10Device) -> i32 {
    // TODO: implementar D3D10CreateDevice desde wine/d3d10misc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3D10CreateDeviceAndSwapChain(adapter: *mut IDXGIAdapter, driver_type: D3D10_DRIVER_TYPE, swrast: HMODULE, flags: UINT, sdk_version: UINT, swapchain_desc: *mut DXGI_SWAP_CHAIN_DESC, param_37226: *mut IDXGISwapChain, param_7394: *mut ID3D10Device) -> i32 {
    // TODO: implementar D3D10CreateDeviceAndSwapChain desde wine/d3d10misc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3D10CreateBlob(data_size: SIZE_T, param_51872: *mut ID3D10Blob) -> i32 {
    // TODO: implementar D3D10CreateBlob desde wine/d3d10misc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Direct3DCreate8(SDKVersion: UINT) -> *mut IDirect3D8 {
    // TODO: implementar Direct3DCreate8 desde wine/d3d8.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DCreateBlob(data_size: SIZE_T, param_52875: *mut ID3DBlob) -> i32 {
    // TODO: implementar D3DCreateBlob desde wine/d3dcompiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DCreateFunctionLinkingGraph(flags: UINT, param_32848: *mut ID3D11FunctionLinkingGraph) -> i32 {
    // TODO: implementar D3DCreateFunctionLinkingGraph desde wine/d3dcompiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DCreateLinker(param_43912: *mut ID3D11Linker) -> i32 {
    // TODO: implementar D3DCreateLinker desde wine/d3dcompiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Direct3DRMCreate(param_51324: *mut struct IDirect3DRM) -> i32 {
    // TODO: implementar Direct3DRMCreate desde wine/d3drm.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DRMCreateColorRGB(param_4271: D3DVALUE, param_4271: D3DVALUE, param_4271: D3DVALUE) -> D3DCOLOR {
    // TODO: implementar D3DRMCreateColorRGB desde wine/d3drmdef.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DRMCreateColorRGBA(param_4271: D3DVALUE, param_4271: D3DVALUE, param_4271: D3DVALUE, param_4271: D3DVALUE) -> D3DCOLOR {
    // TODO: implementar D3DRMCreateColorRGBA desde wine/d3drmdef.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectFromFileA(filename: *mut const char, defines: *mut const D3D10_SHADER_MACRO, include: *mut ID3D10Include, profile: *mut const char, hlslflags: UINT, fxflags: UINT, device: *mut ID3D10Device, effectpool: *mut ID3D10EffectPool, pump: *mut ID3DX10ThreadPump, param_51274: *mut ID3D10Effect, param_51872: *mut ID3D10Blob, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX10CreateEffectFromFileA desde wine/d3dx10async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectFromFileW(filename: *mut const WCHAR, defines: *mut const D3D10_SHADER_MACRO, include: *mut ID3D10Include, profile: *mut const char, hlslflags: UINT, fxflags: UINT, device: *mut ID3D10Device, effectpool: *mut ID3D10EffectPool, pump: *mut ID3DX10ThreadPump, param_51274: *mut ID3D10Effect, param_51872: *mut ID3D10Blob, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX10CreateEffectFromFileW desde wine/d3dx10async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectFromMemory(data: *mut const void, datasize: SIZE_T, filename: *mut const char, defines: *mut const D3D10_SHADER_MACRO, include: *mut ID3D10Include, profile: *mut const char, hlslflags: UINT, fxflags: UINT, device: *mut ID3D10Device, effectpool: *mut ID3D10EffectPool, pump: *mut ID3DX10ThreadPump, param_51274: *mut ID3D10Effect, param_51872: *mut ID3D10Blob, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX10CreateEffectFromMemory desde wine/d3dx10async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectPoolFromFileA(filename: *mut const char, defines: *mut const D3D10_SHADER_MACRO, include: *mut ID3D10Include, profile: *mut const char, hlslflags: UINT, fxflags: UINT, device: *mut ID3D10Device, pump: *mut ID3DX10ThreadPump, param_19338: *mut ID3D10EffectPool, param_51872: *mut ID3D10Blob, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX10CreateEffectPoolFromFileA desde wine/d3dx10async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectPoolFromFileW(filename: *mut const WCHAR, defines: *mut const D3D10_SHADER_MACRO, include: *mut ID3D10Include, profile: *mut const char, hlslflags: UINT, fxflags: UINT, device: *mut ID3D10Device, pump: *mut ID3DX10ThreadPump, param_19338: *mut ID3D10EffectPool, param_51872: *mut ID3D10Blob, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX10CreateEffectPoolFromFileW desde wine/d3dx10async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectPoolFromMemory(data: *mut const void, datasize: SIZE_T, filename: *mut const char, defines: *mut const D3D10_SHADER_MACRO, include: *mut ID3D10Include, profile: *mut const char, hlslflags: UINT, fxflags: UINT, device: *mut ID3D10Device, pump: *mut ID3DX10ThreadPump, param_19338: *mut ID3D10EffectPool, param_51872: *mut ID3D10Blob, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX10CreateEffectPoolFromMemory desde wine/d3dx10async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectFromResourceA(module: HMODULE, resource_name: *mut const char, filename: *mut const char, defines: *mut const D3D10_SHADER_MACRO, include: *mut ID3D10Include, profile: *mut const char, shader_flags: UINT, effect_flags: UINT, device: *mut ID3D10Device, effect_pool: *mut ID3D10EffectPool, pump: *mut ID3DX10ThreadPump, param_51274: *mut ID3D10Effect, param_51872: *mut ID3D10Blob, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX10CreateEffectFromResourceA desde wine/d3dx10async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectFromResourceW(module: HMODULE, resource_name: *mut const WCHAR, filename: *mut const WCHAR, defines: *mut const D3D10_SHADER_MACRO, include: *mut ID3D10Include, profile: *mut const char, shader_flags: UINT, effect_flags: UINT, device: *mut ID3D10Device, effect_pool: *mut ID3D10EffectPool, pump: *mut ID3DX10ThreadPump, param_51274: *mut ID3D10Effect, param_51872: *mut ID3D10Blob, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX10CreateEffectFromResourceW desde wine/d3dx10async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateAsyncFileLoaderW(filename: *mut const WCHAR, param_35221: *mut ID3DX10DataLoader) -> i32 {
    // TODO: implementar D3DX10CreateAsyncFileLoaderW desde wine/d3dx10async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateAsyncFileLoaderA(filename: *mut const char, param_35221: *mut ID3DX10DataLoader) -> i32 {
    // TODO: implementar D3DX10CreateAsyncFileLoaderA desde wine/d3dx10async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateAsyncMemoryLoader(data: *mut const void, datasize: SIZE_T, param_35221: *mut ID3DX10DataLoader) -> i32 {
    // TODO: implementar D3DX10CreateAsyncMemoryLoader desde wine/d3dx10async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateAsyncResourceLoaderA(module: HMODULE, resource: *mut const char, param_35221: *mut ID3DX10DataLoader) -> i32 {
    // TODO: implementar D3DX10CreateAsyncResourceLoaderA desde wine/d3dx10async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateAsyncResourceLoaderW(module: HMODULE, resource: *mut const WCHAR, param_35221: *mut ID3DX10DataLoader) -> i32 {
    // TODO: implementar D3DX10CreateAsyncResourceLoaderW desde wine/d3dx10async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateAsyncTextureProcessor(device: *mut ID3D10Device, info: *mut D3DX10_IMAGE_LOAD_INFO, param_9369: *mut ID3DX10DataProcessor) -> i32 {
    // TODO: implementar D3DX10CreateAsyncTextureProcessor desde wine/d3dx10async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateAsyncTextureInfoProcessor(info: *mut D3DX10_IMAGE_INFO, param_9369: *mut ID3DX10DataProcessor) -> i32 {
    // TODO: implementar D3DX10CreateAsyncTextureInfoProcessor desde wine/d3dx10async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateDevice(adapter: *mut IDXGIAdapter, driver_type: D3D10_DRIVER_TYPE, swrast: HMODULE, flags: u32, param_7394: *mut ID3D10Device) -> i32 {
    // TODO: implementar D3DX10CreateDevice desde wine/d3dx10core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateDeviceAndSwapChain(adapter: *mut IDXGIAdapter, driver_type: D3D10_DRIVER_TYPE, swrast: HMODULE, flags: u32, desc: *mut DXGI_SWAP_CHAIN_DESC, param_37226: *mut IDXGISwapChain, param_7394: *mut ID3D10Device) -> i32 {
    // TODO: implementar D3DX10CreateDeviceAndSwapChain desde wine/d3dx10core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateFontIndirectA(device: *mut ID3D10Device, desc: *mut const D3DX10_FONT_DESCA, param_61792: *mut ID3DX10Font) -> i32 {
    // TODO: implementar D3DX10CreateFontIndirectA desde wine/d3dx10core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateFontIndirectW(device: *mut ID3D10Device, desc: *mut const D3DX10_FONT_DESCW, param_61792: *mut ID3DX10Font) -> i32 {
    // TODO: implementar D3DX10CreateFontIndirectW desde wine/d3dx10core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateFontA(device: *mut ID3D10Device, height: INT, width: UINT, weight: UINT, miplevels: UINT, italic: i32, charset: UINT, precision: UINT, quality: UINT, pitchandfamily: UINT, facename: *mut const char, param_61792: *mut ID3DX10Font) -> i32 {
    // TODO: implementar D3DX10CreateFontA desde wine/d3dx10core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateFontW(device: *mut ID3D10Device, height: INT, width: UINT, weight: UINT, miplevels: UINT, italic: i32, charset: UINT, precision: UINT, quality: UINT, pitchandfamily: UINT, facename: *mut const WCHAR, param_61792: *mut ID3DX10Font) -> i32 {
    // TODO: implementar D3DX10CreateFontW desde wine/d3dx10core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateSprite(device: *mut ID3D10Device, size: UINT, param_4218: *mut ID3DX10Sprite) -> i32 {
    // TODO: implementar D3DX10CreateSprite desde wine/d3dx10core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateTextureFromMemory(device: *mut ID3D10Device, src_data: *mut const void, src_data_size: SIZE_T, loadinfo: *mut D3DX10_IMAGE_LOAD_INFO, pump: *mut ID3DX10ThreadPump, param_38046: *mut ID3D10Resource, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX10CreateTextureFromMemory desde wine/d3dx10tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateTextureFromFileA(device: *mut ID3D10Device, src_file: *mut const char, load_info: *mut D3DX10_IMAGE_LOAD_INFO, pump: *mut ID3DX10ThreadPump, param_38046: *mut ID3D10Resource, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX10CreateTextureFromFileA desde wine/d3dx10tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateTextureFromFileW(device: *mut ID3D10Device, src_file: *mut const WCHAR, load_info: *mut D3DX10_IMAGE_LOAD_INFO, pump: *mut ID3DX10ThreadPump, param_38046: *mut ID3D10Resource, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX10CreateTextureFromFileW desde wine/d3dx10tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateTextureFromResourceA(device: *mut ID3D10Device, module: HMODULE, resource: *mut const char, load_info: *mut D3DX10_IMAGE_LOAD_INFO, pump: *mut ID3DX10ThreadPump, param_38046: *mut ID3D10Resource, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX10CreateTextureFromResourceA desde wine/d3dx10tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateTextureFromResourceW(device: *mut ID3D10Device, module: HMODULE, resource: *mut const WCHAR, load_info: *mut D3DX10_IMAGE_LOAD_INFO, pump: *mut ID3DX10ThreadPump, param_38046: *mut ID3D10Resource, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX10CreateTextureFromResourceW desde wine/d3dx10tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateAsyncFileLoaderA(file_name: *mut const char, param_37622: *mut ID3DX11DataLoader) -> i32 {
    // TODO: implementar D3DX11CreateAsyncFileLoaderA desde wine/d3dx11async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateAsyncFileLoaderW(file_name: *mut const WCHAR, param_37622: *mut ID3DX11DataLoader) -> i32 {
    // TODO: implementar D3DX11CreateAsyncFileLoaderW desde wine/d3dx11async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateAsyncResourceLoaderA(module: HMODULE, resource: *mut const char, param_37622: *mut ID3DX11DataLoader) -> i32 {
    // TODO: implementar D3DX11CreateAsyncResourceLoaderA desde wine/d3dx11async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateAsyncResourceLoaderW(module: HMODULE, resource: *mut const WCHAR, param_37622: *mut ID3DX11DataLoader) -> i32 {
    // TODO: implementar D3DX11CreateAsyncResourceLoaderW desde wine/d3dx11async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateAsyncMemoryLoader(data: *mut const void, data_size: SIZE_T, param_37622: *mut ID3DX11DataLoader) -> i32 {
    // TODO: implementar D3DX11CreateAsyncMemoryLoader desde wine/d3dx11async.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateShaderResourceViewFromMemory(device: *mut ID3D11Device, data: *mut const void, data_size: SIZE_T, load_info: *mut D3DX11_IMAGE_LOAD_INFO, pump: *mut ID3DX11ThreadPump, param_60949: *mut ID3D11ShaderResourceView, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX11CreateShaderResourceViewFromMemory desde wine/d3dx11tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateTextureFromFileA(device: *mut ID3D11Device, filename: *mut const char, load_info: *mut D3DX11_IMAGE_LOAD_INFO, pump: *mut ID3DX11ThreadPump, param_42474: *mut ID3D11Resource, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX11CreateTextureFromFileA desde wine/d3dx11tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateTextureFromFileW(device: *mut ID3D11Device, filename: *mut const WCHAR, load_info: *mut D3DX11_IMAGE_LOAD_INFO, pump: *mut ID3DX11ThreadPump, param_42474: *mut ID3D11Resource, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX11CreateTextureFromFileW desde wine/d3dx11tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateTextureFromResourceA(device: *mut ID3D11Device, module: HMODULE, resource: *mut const char, load_info: *mut D3DX11_IMAGE_LOAD_INFO, pump: *mut ID3DX11ThreadPump, param_42474: *mut ID3D11Resource, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX11CreateTextureFromResourceA desde wine/d3dx11tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateTextureFromResourceW(device: *mut ID3D11Device, module: HMODULE, resource: *mut const WCHAR, load_info: *mut D3DX11_IMAGE_LOAD_INFO, pump: *mut ID3DX11ThreadPump, param_42474: *mut ID3D11Resource, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX11CreateTextureFromResourceW desde wine/d3dx11tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateTextureFromMemory(device: *mut ID3D11Device, src_data: *mut const void, src_data_size: SIZE_T, loadinfo: *mut D3DX11_IMAGE_LOAD_INFO, pump: *mut ID3DX11ThreadPump, param_42474: *mut ID3D11Resource, hresult: *mut i32) -> i32 {
    // TODO: implementar D3DX11CreateTextureFromMemory desde wine/d3dx11tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateKeyframedAnimationSet(name: *mut const char, ticks_per_second: f64, playback_type: D3DXPLAYBACK_TYPE, animation_count: UINT, callback_key_count: UINT, callback_keys: *mut const D3DXKEY_, param_51471: *mut ID3DXKeyframedAnimationSet) -> i32 {
    // TODO: implementar D3DXCreateKeyframedAnimationSet desde wine/d3dx9anim.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCompressedAnimationSet(name: *mut const char, ticks_per_second: f64, playback_type: D3DXPLAYBACK_TYPE, compressed_data: *mut ID3DXBuffer, callback_key_count: UINT, callback_keys: *mut const D3DXKEY_, param_49530: *mut ID3DXCompressedAnimationSet) -> i32 {
    // TODO: implementar D3DXCreateCompressedAnimationSet desde wine/d3dx9anim.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateAnimationController(max_animation_output_count: UINT, max_animation_set_count: UINT, max_track_count: UINT, max_event_count: UINT, param_38673: *mut ID3DXAnimationController) -> i32 {
    // TODO: implementar D3DXCreateAnimationController desde wine/d3dx9anim.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateFontA(device: *mut struct IDirect3DDevice9, height: INT, width: UINT, weight: UINT, miplevels: UINT, italic: i32, charset: u32, precision: u32, quality: u32, pitchandfamily: u32, facename: *mut const char, param_34174: *mut struct ID3DXFont) -> i32 {
    // TODO: implementar D3DXCreateFontA desde wine/d3dx9core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateFontW(device: *mut struct IDirect3DDevice9, height: INT, width: UINT, weight: UINT, miplevels: UINT, italic: i32, charset: u32, precision: u32, quality: u32, pitchandfamily: u32, facename: *mut const WCHAR, param_34174: *mut struct ID3DXFont) -> i32 {
    // TODO: implementar D3DXCreateFontW desde wine/d3dx9core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateFontIndirectA(device: *mut struct IDirect3DDevice9, desc: *mut const D3DXFONT_DESCA, param_34174: *mut struct ID3DXFont) -> i32 {
    // TODO: implementar D3DXCreateFontIndirectA desde wine/d3dx9core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateFontIndirectW(device: *mut struct IDirect3DDevice9, desc: *mut const D3DXFONT_DESCW, param_34174: *mut struct ID3DXFont) -> i32 {
    // TODO: implementar D3DXCreateFontIndirectW desde wine/d3dx9core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateLine(device: *mut struct IDirect3DDevice9, param_7220: *mut struct ID3DXLine) -> i32 {
    // TODO: implementar D3DXCreateLine desde wine/d3dx9core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateRenderToEnvMap(device: *mut struct IDirect3DDevice9, size: UINT, miplevels: UINT, format: D3DFORMAT, stencil: i32, stencil_format: D3DFORMAT, param_58958: *mut struct ID3DXRenderToEnvMap) -> i32 {
    // TODO: implementar D3DXCreateRenderToEnvMap desde wine/d3dx9core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateRenderToSurface(device: *mut struct IDirect3DDevice9, width: UINT, height: UINT, format: D3DFORMAT, stencil: i32, stencil_format: D3DFORMAT, param_19689: *mut struct ID3DXRenderToSurface) -> i32 {
    // TODO: implementar D3DXCreateRenderToSurface desde wine/d3dx9core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateSprite(device: *mut struct IDirect3DDevice9, param_2641: *mut struct ID3DXSprite) -> i32 {
    // TODO: implementar D3DXCreateSprite desde wine/d3dx9core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectPool(param_42577: *mut ID3DXEffectPool) -> i32 {
    // TODO: implementar D3DXCreateEffectPool desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffect(device: *mut struct IDirect3DDevice9, srcdata: *mut const void, srcdatalen: UINT, defines: *mut const D3DXMACRO, include: *mut struct ID3DXInclude, flags: u32, pool: *mut struct ID3DXEffectPool, param_49510: *mut struct ID3DXEffect, param_17027: *mut struct ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateEffect desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectEx(device: *mut struct IDirect3DDevice9, srcdata: *mut const void, srcdatalen: UINT, defines: *mut const D3DXMACRO, include: *mut struct ID3DXInclude, skip_constants: *mut const char, flags: u32, pool: *mut struct ID3DXEffectPool, param_49510: *mut struct ID3DXEffect, param_17027: *mut struct ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateEffectEx desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectCompiler(srcdata: *mut const char, srcdatalen: UINT, defines: *mut const D3DXMACRO, include: *mut ID3DXInclude, flags: u32, param_47589: *mut ID3DXEffectCompiler, param_29258: *mut ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateEffectCompiler desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromFileExA(device: *mut struct IDirect3DDevice9, srcfile: *mut const char, defines: *mut const D3DXMACRO, include: *mut struct ID3DXInclude, skip_constants: *mut const char, flags: u32, pool: *mut struct ID3DXEffectPool, param_49510: *mut struct ID3DXEffect, param_17027: *mut struct ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateEffectFromFileExA desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromFileExW(device: *mut struct IDirect3DDevice9, srcfile: *mut const WCHAR, defines: *mut const D3DXMACRO, include: *mut struct ID3DXInclude, skip_constants: *mut const char, flags: u32, pool: *mut struct ID3DXEffectPool, param_49510: *mut struct ID3DXEffect, param_17027: *mut struct ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateEffectFromFileExW desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromFileA(device: *mut struct IDirect3DDevice9, srcfile: *mut const char, defines: *mut const D3DXMACRO, include: *mut struct ID3DXInclude, flags: u32, pool: *mut struct ID3DXEffectPool, param_49510: *mut struct ID3DXEffect, param_17027: *mut struct ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateEffectFromFileA desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromFileW(device: *mut struct IDirect3DDevice9, srcfile: *mut const WCHAR, defines: *mut const D3DXMACRO, include: *mut struct ID3DXInclude, flags: u32, pool: *mut struct ID3DXEffectPool, param_49510: *mut struct ID3DXEffect, param_17027: *mut struct ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateEffectFromFileW desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromResourceExA(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, srcresource: *mut const char, defines: *mut const D3DXMACRO, include: *mut struct ID3DXInclude, skip_constants: *mut const char, flags: u32, pool: *mut struct ID3DXEffectPool, param_49510: *mut struct ID3DXEffect, param_17027: *mut struct ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateEffectFromResourceExA desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromResourceExW(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, srcresource: *mut const WCHAR, defines: *mut const D3DXMACRO, include: *mut struct ID3DXInclude, skip_constants: *mut const char, flags: u32, pool: *mut struct ID3DXEffectPool, param_49510: *mut struct ID3DXEffect, param_17027: *mut struct ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateEffectFromResourceExW desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromResourceA(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, srcresource: *mut const char, defines: *mut const D3DXMACRO, include: *mut struct ID3DXInclude, flags: u32, pool: *mut struct ID3DXEffectPool, param_49510: *mut struct ID3DXEffect, param_17027: *mut struct ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateEffectFromResourceA desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromResourceW(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, srcresource: *mut const WCHAR, defines: *mut const D3DXMACRO, include: *mut struct ID3DXInclude, flags: u32, pool: *mut struct ID3DXEffectPool, param_49510: *mut struct ID3DXEffect, param_17027: *mut struct ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateEffectFromResourceW desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectCompilerFromFileA(srcfile: *mut const char, defines: *mut const D3DXMACRO, include: *mut ID3DXInclude, flags: u32, param_47589: *mut ID3DXEffectCompiler, param_29258: *mut ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateEffectCompilerFromFileA desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectCompilerFromFileW(srcfile: *mut const WCHAR, defines: *mut const D3DXMACRO, include: *mut ID3DXInclude, flags: u32, param_47589: *mut ID3DXEffectCompiler, param_29258: *mut ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateEffectCompilerFromFileW desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectCompilerFromResourceA(srcmodule: HMODULE, srcresource: *mut const char, defines: *mut const D3DXMACRO, include: *mut ID3DXInclude, flags: u32, param_47589: *mut ID3DXEffectCompiler, param_29258: *mut ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateEffectCompilerFromResourceA desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectCompilerFromResourceW(srcmodule: HMODULE, srcresource: *mut const WCHAR, defines: *mut const D3DXMACRO, include: *mut ID3DXInclude, flags: u32, param_47589: *mut ID3DXEffectCompiler, param_29258: *mut ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateEffectCompilerFromResourceW desde wine/d3dx9effect.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateMatrixStack(flags: u32, param_5447: *mut ID3DXMatrixStack) -> i32 {
    // TODO: implementar D3DXCreateMatrixStack desde wine/d3dx9math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateMesh(face_count: u32, vertex_count: u32, flags: u32, declaration: *mut const D3DVERTEXELEMENT9, device: *mut struct IDirect3DDevice9, param_60763: *mut struct ID3DXMesh) -> i32 {
    // TODO: implementar D3DXCreateMesh desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateMeshFVF(face_count: u32, vertex_count: u32, flags: u32, fvf: u32, device: *mut struct IDirect3DDevice9, param_60763: *mut struct ID3DXMesh) -> i32 {
    // TODO: implementar D3DXCreateMeshFVF desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateBuffer(size: u32, param_29258: *mut ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateBuffer desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateSPMesh(mesh: *mut ID3DXMesh, adjacency: *mut const DWORD, attribute_weights: *mut const D3DXATTRIBUTEWEIGHTS, vertex_weights: *mut const float, param_9418: *mut ID3DXSPMesh) -> i32 {
    // TODO: implementar D3DXCreateSPMesh desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreatePMeshFromStream(stream: *mut struct IStream, flags: u32, device: *mut struct IDirect3DDevice9, param_17027: *mut struct ID3DXBuffer, param_17027: *mut struct ID3DXBuffer, material_count: *mut u32, param_49302: *mut struct ID3DXPMesh) -> i32 {
    // TODO: implementar D3DXCreatePMeshFromStream desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateSkinInfo(vertex_count: u32, declaration: *mut const D3DVERTEXELEMENT9, bone_count: u32, param_30171: *mut ID3DXSkinInfo) -> i32 {
    // TODO: implementar D3DXCreateSkinInfo desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateSkinInfoFVF(vertex_count: u32, fvf: u32, bone_count: u32, param_30171: *mut ID3DXSkinInfo) -> i32 {
    // TODO: implementar D3DXCreateSkinInfoFVF desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateSkinInfoFromBlendedMesh(mesh: *mut ID3DXBaseMesh, bone_count: u32, bone_combination_table: *mut const D3DXBONECOMBINATION, param_30171: *mut ID3DXSkinInfo) -> i32 {
    // TODO: implementar D3DXCreateSkinInfoFromBlendedMesh desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreatePatchMesh(patch_info: *mut const D3DXPATCHINFO, patch_count: u32, vertex_count: u32, flags: u32, declaration: *mut const D3DVERTEXELEMENT9, device: *mut struct IDirect3DDevice9, param_13286: *mut struct ID3DXPatchMesh) -> i32 {
    // TODO: implementar D3DXCreatePatchMesh desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreatePRTBuffer(sample_count: UINT, coeff_count: UINT, channel_count: UINT, param_20247: *mut ID3DXPRTBuffer) -> i32 {
    // TODO: implementar D3DXCreatePRTBuffer desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreatePRTBufferTex(width: UINT, height: UINT, coeff_count: UINT, channel_count: UINT, param_20247: *mut ID3DXPRTBuffer) -> i32 {
    // TODO: implementar D3DXCreatePRTBufferTex desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreatePRTCompBuffer(quality: D3DXSHCOMPRESSQUALITYTYPE, cluster_count: UINT, pca_count: UINT, cb: LPD3DXSHPRTSIMCB, ctx: *mut core::ffi::c_void, input: *mut ID3DXPRTBuffer, param_54633: *mut ID3DXPRTCompBuffer) -> i32 {
    // TODO: implementar D3DXCreatePRTCompBuffer desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureGutterHelper(width: UINT, height: UINT, mesh: *mut ID3DXMesh, gutter_size: f32, param_30318: *mut ID3DXTextureGutterHelper) -> i32 {
    // TODO: implementar D3DXCreateTextureGutterHelper desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreatePRTEngine(mesh: *mut ID3DXMesh, adjacency: *mut u32, extract_uv: i32, blocker_mesh: *mut ID3DXMesh, param_47702: *mut ID3DXPRTEngine) -> i32 {
    // TODO: implementar D3DXCreatePRTEngine desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXUVAtlasCreate(mesh_in: *mut ID3DXMesh, max_chart_count: UINT, max_stretch_in: f32, width: UINT, height: UINT, gutter: f32, texture_idx: u32, adjacency: *mut const DWORD, false_edges: *mut const DWORD, imt_array: *mut const float, cb: LPD3DXUVATLASCB, cb_freq: f32, ctx: *mut core::ffi::c_void, flags: u32, param_27302: *mut ID3DXMesh, param_29258: *mut ID3DXBuffer, param_29258: *mut ID3DXBuffer, max_stretch_out: *mut f32, chart_count: *mut UINT) -> i32 {
    // TODO: implementar D3DXUVAtlasCreate desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureShader(pFunction: *mut const DWORD, param_61693: *mut ID3DXTextureShader) -> i32 {
    // TODO: implementar D3DXCreateTextureShader desde wine/d3dx9shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateFragmentLinker(device: *mut IDirect3DDevice9, size: UINT, param_61817: *mut ID3DXFragmentLinker) -> i32 {
    // TODO: implementar D3DXCreateFragmentLinker desde wine/d3dx9shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateFragmentLinkerEx(device: *mut IDirect3DDevice9, size: UINT, flags: u32, param_61817: *mut ID3DXFragmentLinker) -> i32 {
    // TODO: implementar D3DXCreateFragmentLinkerEx desde wine/d3dx9shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateBox(device: *mut struct IDirect3DDevice9, width: f32, height: f32, depth: f32, param_60763: *mut struct ID3DXMesh, param_17027: *mut struct ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateBox desde wine/d3dx9shape.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCylinder(device: *mut struct IDirect3DDevice9, radius1: f32, radius2: f32, length: f32, slices: UINT, stacks: UINT, param_60763: *mut struct ID3DXMesh, param_17027: *mut struct ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateCylinder desde wine/d3dx9shape.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreatePolygon(device: *mut struct IDirect3DDevice9, length: f32, sides: UINT, param_60763: *mut struct ID3DXMesh, param_29258: *mut ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreatePolygon desde wine/d3dx9shape.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateSphere(device: *mut struct IDirect3DDevice9, radius: f32, slices: UINT, stacks: UINT, param_60763: *mut struct ID3DXMesh, param_17027: *mut struct ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateSphere desde wine/d3dx9shape.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTeapot(device: *mut struct IDirect3DDevice9, param_60763: *mut struct ID3DXMesh, param_17027: *mut struct ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateTeapot desde wine/d3dx9shape.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextA(device: *mut struct IDirect3DDevice9, hdc: HDC, text: *mut const char, deviation: f32, extrusion: f32, param_60763: *mut struct ID3DXMesh, param_17027: *mut struct ID3DXBuffer, glyphmetrics: *mut GLYPHMETRICSFLOAT) -> i32 {
    // TODO: implementar D3DXCreateTextA desde wine/d3dx9shape.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextW(device: *mut struct IDirect3DDevice9, hdc: HDC, text: *mut const WCHAR, deviation: f32, extrusion: FLOAT, param_60763: *mut struct ID3DXMesh, param_17027: *mut struct ID3DXBuffer, glyphmetrics: *mut GLYPHMETRICSFLOAT) -> i32 {
    // TODO: implementar D3DXCreateTextW desde wine/d3dx9shape.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTorus(device: *mut struct IDirect3DDevice9, innerradius: f32, outerradius: f32, sides: UINT, rings: UINT, param_60763: *mut struct ID3DXMesh, param_29258: *mut ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXCreateTorus desde wine/d3dx9shape.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTexture(device: *mut struct IDirect3DDevice9, width: UINT, height: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, param_55564: *mut struct IDirect3DTexture9) -> i32 {
    // TODO: implementar D3DXCreateTexture desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTexture(device: *mut struct IDirect3DDevice9, size: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, param_18749: *mut struct IDirect3DCubeTexture9) -> i32 {
    // TODO: implementar D3DXCreateCubeTexture desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTexture(device: *mut struct IDirect3DDevice9, width: UINT, height: UINT, depth: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, param_42121: *mut struct IDirect3DVolumeTexture9) -> i32 {
    // TODO: implementar D3DXCreateVolumeTexture desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromFileA(device: *mut struct IDirect3DDevice9, srcfile: *mut const char, param_55564: *mut struct IDirect3DTexture9) -> i32 {
    // TODO: implementar D3DXCreateTextureFromFileA desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromFileW(device: *mut struct IDirect3DDevice9, srcfile: *mut const WCHAR, param_55564: *mut struct IDirect3DTexture9) -> i32 {
    // TODO: implementar D3DXCreateTextureFromFileW desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromFileA(device: *mut struct IDirect3DDevice9, srcfile: *mut const char, param_18749: *mut struct IDirect3DCubeTexture9) -> i32 {
    // TODO: implementar D3DXCreateCubeTextureFromFileA desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromFileW(device: *mut struct IDirect3DDevice9, srcfile: *mut const WCHAR, param_18749: *mut struct IDirect3DCubeTexture9) -> i32 {
    // TODO: implementar D3DXCreateCubeTextureFromFileW desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromFileA(device: *mut struct IDirect3DDevice9, srcfile: *mut const char, param_42121: *mut struct IDirect3DVolumeTexture9) -> i32 {
    // TODO: implementar D3DXCreateVolumeTextureFromFileA desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromFileW(device: *mut struct IDirect3DDevice9, srcfile: *mut const WCHAR, param_42121: *mut struct IDirect3DVolumeTexture9) -> i32 {
    // TODO: implementar D3DXCreateVolumeTextureFromFileW desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromResourceA(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, resource: *mut const char, param_55564: *mut struct IDirect3DTexture9) -> i32 {
    // TODO: implementar D3DXCreateTextureFromResourceA desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromResourceW(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, resource: *mut const WCHAR, param_55564: *mut struct IDirect3DTexture9) -> i32 {
    // TODO: implementar D3DXCreateTextureFromResourceW desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromResourceA(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, resource: *mut const char, param_18749: *mut struct IDirect3DCubeTexture9) -> i32 {
    // TODO: implementar D3DXCreateCubeTextureFromResourceA desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromResourceW(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, resource: *mut const WCHAR, param_18749: *mut struct IDirect3DCubeTexture9) -> i32 {
    // TODO: implementar D3DXCreateCubeTextureFromResourceW desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromResourceA(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, resource: *mut const char, param_42121: *mut struct IDirect3DVolumeTexture9) -> i32 {
    // TODO: implementar D3DXCreateVolumeTextureFromResourceA desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromResourceW(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, resource: *mut const WCHAR, param_42121: *mut struct IDirect3DVolumeTexture9) -> i32 {
    // TODO: implementar D3DXCreateVolumeTextureFromResourceW desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromFileExA(device: *mut struct IDirect3DDevice9, srcfile: *mut const char, width: UINT, height: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, filter: u32, mipfilter: u32, colorkey: D3DCOLOR, srcinfo: *mut D3DXIMAGE_INFO, palette: *mut PALETTEENTRY, param_55564: *mut struct IDirect3DTexture9) -> i32 {
    // TODO: implementar D3DXCreateTextureFromFileExA desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromFileExW(device: *mut struct IDirect3DDevice9, srcfile: *mut const WCHAR, width: UINT, height: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, filter: u32, mipfilter: u32, colorkey: D3DCOLOR, srcinfo: *mut D3DXIMAGE_INFO, palette: *mut PALETTEENTRY, param_55564: *mut struct IDirect3DTexture9) -> i32 {
    // TODO: implementar D3DXCreateTextureFromFileExW desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromFileExA(device: *mut struct IDirect3DDevice9, srcfile: *mut const char, size: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, filter: u32, mipfilter: u32, colorkey: D3DCOLOR, srcinfo: *mut D3DXIMAGE_INFO, palette: *mut PALETTEENTRY, param_18749: *mut struct IDirect3DCubeTexture9) -> i32 {
    // TODO: implementar D3DXCreateCubeTextureFromFileExA desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromFileExW(device: *mut struct IDirect3DDevice9, srcfile: *mut const WCHAR, size: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, filter: u32, mipfilter: u32, colorkey: D3DCOLOR, srcinfo: *mut D3DXIMAGE_INFO, palette: *mut PALETTEENTRY, param_18749: *mut struct IDirect3DCubeTexture9) -> i32 {
    // TODO: implementar D3DXCreateCubeTextureFromFileExW desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromFileExA(device: *mut struct IDirect3DDevice9, srcfile: *mut const char, width: UINT, height: UINT, depth: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, filter: u32, mipfilter: u32, colorkey: D3DCOLOR, srcinfo: *mut D3DXIMAGE_INFO, palette: *mut PALETTEENTRY, param_42121: *mut struct IDirect3DVolumeTexture9) -> i32 {
    // TODO: implementar D3DXCreateVolumeTextureFromFileExA desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromFileExW(device: *mut struct IDirect3DDevice9, srcfile: *mut const WCHAR, width: UINT, height: UINT, depth: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, filter: u32, mipfilter: u32, colorkey: D3DCOLOR, srcinfo: *mut D3DXIMAGE_INFO, palette: *mut PALETTEENTRY, param_42121: *mut struct IDirect3DVolumeTexture9) -> i32 {
    // TODO: implementar D3DXCreateVolumeTextureFromFileExW desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromResourceExA(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, resource: *mut const char, width: UINT, height: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, filter: u32, mipfilter: u32, colorkey: D3DCOLOR, srcinfo: *mut D3DXIMAGE_INFO, palette: *mut PALETTEENTRY, param_55564: *mut struct IDirect3DTexture9) -> i32 {
    // TODO: implementar D3DXCreateTextureFromResourceExA desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromResourceExW(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, resource: *mut const WCHAR, width: UINT, height: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, filter: u32, mipfilter: u32, colorkey: D3DCOLOR, srcinfo: *mut D3DXIMAGE_INFO, palette: *mut PALETTEENTRY, param_55564: *mut struct IDirect3DTexture9) -> i32 {
    // TODO: implementar D3DXCreateTextureFromResourceExW desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromResourceExA(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, resource: *mut const char, size: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, filter: u32, mipfilter: u32, colorkey: D3DCOLOR, srcinfo: *mut D3DXIMAGE_INFO, palette: *mut PALETTEENTRY, param_18749: *mut struct IDirect3DCubeTexture9) -> i32 {
    // TODO: implementar D3DXCreateCubeTextureFromResourceExA desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromResourceExW(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, resource: *mut const WCHAR, size: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, filter: u32, mipfilter: u32, colorkey: D3DCOLOR, srcinfo: *mut D3DXIMAGE_INFO, palette: *mut PALETTEENTRY, param_18749: *mut struct IDirect3DCubeTexture9) -> i32 {
    // TODO: implementar D3DXCreateCubeTextureFromResourceExW desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromResourceExA(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, resource: *mut const char, width: UINT, height: UINT, depth: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, filter: u32, mipfilter: u32, colorkey: D3DCOLOR, srcinfo: *mut D3DXIMAGE_INFO, palette: *mut PALETTEENTRY, param_42121: *mut struct IDirect3DVolumeTexture9) -> i32 {
    // TODO: implementar D3DXCreateVolumeTextureFromResourceExA desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromResourceExW(device: *mut struct IDirect3DDevice9, srcmodule: HMODULE, resource: *mut const WCHAR, width: UINT, height: UINT, depth: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, filter: u32, mipfilter: u32, colorkey: D3DCOLOR, srcinfo: *mut D3DXIMAGE_INFO, palette: *mut PALETTEENTRY, param_42121: *mut struct IDirect3DVolumeTexture9) -> i32 {
    // TODO: implementar D3DXCreateVolumeTextureFromResourceExW desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromFileInMemory(device: *mut struct IDirect3DDevice9, srcdata: *mut const void, srcdatasize: UINT, param_55564: *mut struct IDirect3DTexture9) -> i32 {
    // TODO: implementar D3DXCreateTextureFromFileInMemory desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromFileInMemory(device: *mut struct IDirect3DDevice9, srcdata: *mut const void, srcdatasize: UINT, param_18749: *mut struct IDirect3DCubeTexture9) -> i32 {
    // TODO: implementar D3DXCreateCubeTextureFromFileInMemory desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromFileInMemory(device: *mut struct IDirect3DDevice9, srcdata: *mut const void, srcdatasize: UINT, param_42121: *mut struct IDirect3DVolumeTexture9) -> i32 {
    // TODO: implementar D3DXCreateVolumeTextureFromFileInMemory desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromFileInMemoryEx(device: *mut struct IDirect3DDevice9, srcdata: *mut const void, srcdatasize: UINT, width: UINT, height: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, filter: u32, mipfilter: u32, colorkey: D3DCOLOR, srcinfo: *mut D3DXIMAGE_INFO, palette: *mut PALETTEENTRY, param_55564: *mut struct IDirect3DTexture9) -> i32 {
    // TODO: implementar D3DXCreateTextureFromFileInMemoryEx desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromFileInMemoryEx(device: *mut struct IDirect3DDevice9, srcdata: *mut const void, srcdatasize: UINT, size: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, filter: u32, mipfilter: u32, colorkey: D3DCOLOR, srcinfo: *mut D3DXIMAGE_INFO, palette: *mut PALETTEENTRY, param_18749: *mut struct IDirect3DCubeTexture9) -> i32 {
    // TODO: implementar D3DXCreateCubeTextureFromFileInMemoryEx desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromFileInMemoryEx(device: *mut struct IDirect3DDevice9, srcdata: *mut const void, srcdatasize: UINT, width: UINT, height: UINT, depth: UINT, miplevels: UINT, usage: u32, format: D3DFORMAT, pool: D3DPOOL, filter: u32, mipfilter: u32, colorkey: D3DCOLOR, srcinfo: *mut D3DXIMAGE_INFO, palette: *mut PALETTEENTRY, param_42121: *mut struct IDirect3DVolumeTexture9) -> i32 {
    // TODO: implementar D3DXCreateVolumeTextureFromFileInMemoryEx desde wine/d3dx9tex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXFileCreate(param_16979: *mut struct ID3DXFile) -> STDAPI {
    // TODO: implementar D3DXFileCreate desde wine/d3dx9xof.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DebugCreate(riid: REFIID, param_64866: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar DebugCreate desde wine/dbgeng.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DCICreatePrimary(param_216: HDC, param_24951: *mut LPDCISURFACEINFO) -> i32 {
    // TODO: implementar DCICreatePrimary desde wine/dciman.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DdeCreateDataHandle(param_54075: u32, param_15972: LPBYTE, param_54075: u32, param_54075: u32, param_29914: HSZ, param_2971: UINT, param_2971: UINT) -> WINUSERAPI HDDEDATA {
    // TODO: implementar DdeCreateDataHandle desde wine/ddeml.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DdeCreateStringHandleA(param_54075: u32, param_15619: LPCSTR, param_18538: INT) -> WINUSERAPI HSZ {
    // TODO: implementar DdeCreateStringHandleA desde wine/ddeml.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DdeCreateStringHandleW(param_54075: u32, param_25711: LPCWSTR, param_18538: INT) -> WINUSERAPI HSZ {
    // TODO: implementar DdeCreateStringHandleW desde wine/ddeml.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectDrawCreate(driver_guid: *mut GUID, param_45876: *mut IDirectDraw, outer: *mut IUnknown) -> i32 {
    // TODO: implementar DirectDrawCreate desde wine/ddraw.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectDrawCreateEx(driver_guid: *mut GUID, param_64866: *mut core::ffi::c_void, interface_iid: REFIID, outer: *mut IUnknown) -> i32 {
    // TODO: implementar DirectDrawCreateEx desde wine/ddraw.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectDrawCreateClipper(flags: u32, param_61484: *mut IDirectDrawClipper, outer: *mut IUnknown) -> i32 {
    // TODO: implementar DirectDrawCreateClipper desde wine/ddraw.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DevCreateObjectQuery(type: DEV_OBJECT_TYPE, flags: ULONG, props_len: ULONG, props: *mut const DEVPROPCOMPKEY, filters_len: ULONG, filters: *mut const DEVPROP_FILTER_EXPRESSION, callback: PDEV_QUERY_RESULT_, user_data: *mut core::ffi::c_void, devquery: *mut HDEVQUERY) -> i32 {
    // TODO: implementar DevCreateObjectQuery desde wine/devquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DevCreateObjectQueryEx(type: DEV_OBJECT_TYPE, flags: ULONG, props_len: ULONG, props: *mut const DEVPROPCOMPKEY, filters_len: ULONG, filters: *mut const DEVPROP_FILTER_EXPRESSION, params_len: ULONG, params: *mut const DEV_QUERY_PARAMETER, callback: PDEV_QUERY_RESULT_, user_data: *mut core::ffi::c_void, devquery: *mut HDEVQUERY) -> i32 {
    // TODO: implementar DevCreateObjectQueryEx desde wine/devquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DevCreateObjectQueryFromId(type: DEV_OBJECT_TYPE, id: *mut const WCHAR, flags: ULONG, props_len: ULONG, props: *mut const DEVPROPCOMPKEY, filters_len: ULONG, filters: *mut const DEVPROP_FILTER_EXPRESSION, callback: PDEV_QUERY_RESULT_, user_data: *mut core::ffi::c_void, devquery: *mut HDEVQUERY) -> i32 {
    // TODO: implementar DevCreateObjectQueryFromId desde wine/devquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DevCreateObjectQueryFromIdEx(type: DEV_OBJECT_TYPE, id: *mut const WCHAR, flags: ULONG, props_len: ULONG, props: *mut const DEVPROPCOMPKEY, filters_len: ULONG, filters: *mut const DEVPROP_FILTER_EXPRESSION, params_len: ULONG, params: *mut const DEV_QUERY_PARAMETER, callback: PDEV_QUERY_RESULT_, user_data: *mut core::ffi::c_void, devquery: *mut HDEVQUERY) -> i32 {
    // TODO: implementar DevCreateObjectQueryFromIdEx desde wine/devquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DevCreateObjectQueryFromIds(type: DEV_OBJECT_TYPE, id_sz: *mut const WCHAR, flags: ULONG, props_len: ULONG, props: *mut const DEVPROPCOMPKEY, filters_len: ULONG, filters: *mut const DEVPROP_FILTER_EXPRESSION, callback: PDEV_QUERY_RESULT_, user_data: *mut core::ffi::c_void, devquery: *mut HDEVQUERY) -> i32 {
    // TODO: implementar DevCreateObjectQueryFromIds desde wine/devquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DevCreateObjectQueryFromIdsEx(type: DEV_OBJECT_TYPE, id_sz: *mut const WCHAR, flags: ULONG, props_len: ULONG, props: *mut const DEVPROPCOMPKEY, filters_len: ULONG, filters: *mut const DEVPROP_FILTER_EXPRESSION, params_len: ULONG, params: *mut const DEV_QUERY_PARAMETER, callback: PDEV_QUERY_RESULT_, user_data: *mut core::ffi::c_void, devquery: *mut HDEVQUERY) -> i32 {
    // TODO: implementar DevCreateObjectQueryFromIdsEx desde wine/devquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectInput8Create(param_49462: HINSTANCE, param_54075: u32, param_55479: REFIID, param_34430: *mut LPVOID, param_4658: LPUNKNOWN) -> i32 {
    // TODO: implementar DirectInput8Create desde wine/dinput.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectInputCreateA(param_49462: HINSTANCE, param_54075: u32, param_36491: *mut LPDIRECTINPUTA, param_4658: LPUNKNOWN) -> i32 {
    // TODO: implementar DirectInputCreateA desde wine/dinput.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectInputCreateW(param_49462: HINSTANCE, param_54075: u32, param_21461: *mut LPDIRECTINPUTW, param_4658: LPUNKNOWN) -> i32 {
    // TODO: implementar DirectInputCreateW desde wine/dinput.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectInputCreateEx(param_49462: HINSTANCE, param_54075: u32, param_55479: REFIID, param_34430: *mut LPVOID, param_4658: LPUNKNOWN) -> i32 {
    // TODO: implementar DirectInputCreateEx desde wine/dinput.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MoCreateMediaType(param_34346: *mut core::ffi::c_void, param_54075: u32) -> i32 {
    // TODO: implementar MoCreateMediaType desde wine/dmort.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectPlay8AddressCreate(pcIID: *mut const GUID, ppvInterface: *mut LPVOID, pUnknown: *mut IUnknown) -> i32 {
    // TODO: implementar DirectPlay8AddressCreate desde wine/dpaddr.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectPlayCreate(lpGUID: LPGUID, lplpDP: *mut LPDIRECTPLAY, pUnk: *mut IUnknown) -> extern HRESULT {
    // TODO: implementar DirectPlayCreate desde wine/dplay.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectPlay8Create(pcIID: *mut const CLSID, ppvInterface: *mut LPVOID, pUnknown: *mut IUnknown) -> i32 {
    // TODO: implementar DirectPlay8Create desde wine/dplay8.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectPlayLobbyCreateW(param_22847: LPGUID, param_61454: *mut LPDIRECTPLAYLOBBY, param_46438: *mut IUnknown, param_29262: LPVOID, param_54075: u32) -> extern HRESULT {
    // TODO: implementar DirectPlayLobbyCreateW desde wine/dplobby.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectPlayLobbyCreateA(param_22847: LPGUID, param_38439: *mut LPDIRECTPLAYLOBBYA, param_46438: *mut IUnknown, param_29262: LPVOID, param_54075: u32) -> extern HRESULT {
    // TODO: implementar DirectPlayLobbyCreateA desde wine/dplobby.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectPlay8LobbyCreate(pcIID: *mut const GUID, ppvInterface: *mut LPVOID, pUnknown: *mut IUnknown) -> i32 {
    // TODO: implementar DirectPlay8LobbyCreate desde wine/dplobby8.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectPlayNATHelpCreate(pIID: LPCGUID, ppvInterface: *mut LPVOID) -> i32 {
    // TODO: implementar DirectPlayNATHelpCreate desde wine/dpnathlp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectSoundCreate(lpGUID: LPCGUID, ppDS: *mut LPDIRECTSOUND, pUnkOuter: LPUNKNOWN) -> extern HRESULT {
    // TODO: implementar DirectSoundCreate desde wine/dsound.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectSoundCaptureCreate(lpGUID: LPCGUID, ppDSC: *mut LPDIRECTSOUNDCAPTURE, pUnkOuter: LPUNKNOWN) -> extern HRESULT {
    // TODO: implementar DirectSoundCaptureCreate desde wine/dsound.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectSoundCreate8(lpGUID: LPCGUID, ppDS8: *mut LPDIRECTSOUND8, pUnkOuter: LPUNKNOWN) -> extern HRESULT {
    // TODO: implementar DirectSoundCreate8 desde wine/dsound.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectSoundCaptureCreate8(lpGUID: LPCGUID, ppDSC8: *mut LPDIRECTSOUNDCAPTURE8, pUnkOuter: LPUNKNOWN) -> extern HRESULT {
    // TODO: implementar DirectSoundCaptureCreate8 desde wine/dsound.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectSoundFullDuplexCreate(pcGuidCaptureDevice: LPCGUID, pcGuidRenderDevice: LPCGUID, pcDSCBufferDesc: LPCDSCBUFFERDESC, pcDSBufferDesc: LPCDSBUFFERDESC, hWnd: *mut core::ffi::c_void, dwLevel: u32, ppDSFD: *mut LPDIRECTSOUNDFULLDUPLEX, ppDSCBuffer8: *mut LPDIRECTSOUNDCAPTUREBUFFER8, ppDSBuffer8: *mut LPDIRECTSOUNDBUFFER8, pUnkOuter: LPUNKNOWN) -> extern HRESULT {
    // TODO: implementar DirectSoundFullDuplexCreate desde wine/dsound.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DXCoreCreateAdapterFactory(param_55479: REFIID, param_64866: *mut core::ffi::c_void) -> STDAPI {
    // TODO: implementar DXCoreCreateAdapterFactory desde wine/dxcore.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateAdapterList(num_attributes: u32, filter_attributes: *mut const GUID, param_9205: *mut T) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateAdapterList desde wine/dxcore_interface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirectXFileCreate(lplpDirectXFile: *mut LPDIRECTXFILE) -> STDAPI {
    // TODO: implementar DirectXFileCreate desde wine/dxfile.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FCICreate(param_31471: PERF, param_4391: PFNFCIFILEPLACED, param_26212: PFNFCIALLOC, param_43375: PFNFCIFREE, param_12935: PFNFCIOPEN, param_1628: PFNFCIREAD, param_20090: PFNFCIWRITE, param_5375: PFNFCICLOSE, param_49364: PFNFCISEEK, param_4652: PFNFCIDELETE, param_64892: PFNFCIGETTEMPFILE, param_2232: PCCAB, param_64866: *mut core::ffi::c_void) -> HFCI {
    // TODO: implementar FCICreate desde wine/fci.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FDICreate(param_65037: PFNALLOC, param_28119: PFNFREE, param_8272: PFNOPEN, param_41544: PFNREAD, param_55237: PFNWRITE, param_38303: PFNCLOSE, param_35552: PFNSEEK, param_59621: i32, param_31471: PERF) -> HFDI {
    // TODO: implementar FDICreate desde wine/fdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateFile2(param_25711: LPCWSTR, param_54075: u32, param_54075: u32, param_54075: u32, param_35709: LPCREATEFILE2_EXTENDED_PARAMETERS) -> WINBASEAPI HANDLE {
    // TODO: implementar CreateFile2 desde wine/fileapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PfCreateInterface(dwName: u32, inAction: PFFORWARD_ACTION, outAction: PFFORWARD_ACTION, bUseLog: i32, bMustBeUnique: i32, ppInterface: *mut INTERFACE_HANDLE) -> PF {
    // TODO: implementar PfCreateInterface desde wine/fltdefs.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateFontPackage(src: *mut const unsigned char, src_len: const ULONG, param_48908: *mut u8, dest_len: *mut ULONG, written: *mut ULONG, flags: const unsigned short, face_index: const unsigned short, format: const unsigned short, lang: const unsigned short, platform: const unsigned short, encoding: const unsigned short, keep_list: *mut const unsigned short, keep_len: const unsigned short, allocproc: CFP_ALLOCPROC, reallocproc: CFP_REALLOCPROC, freeproc: CFP_FREEPROC, reserved: *mut core::ffi::c_void) -> ULONG {
    // TODO: implementar CreateFontPackage desde wine/fontsub.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateEffect(guid: const GUID, param_48530: *mut CGpEffect) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateEffect desde wine/gdipluseffects.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateAdjustableArrowCap(param_35563: REAL, param_35563: REAL, param_16716: i32, param_42630: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateAdjustableArrowCap desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipBitmapCreateApplyEffect(param_30005: *mut core::ffi::c_void, param_18538: INT, param_48530: *mut CGpEffect, param_11076: *mut RECT, param_11076: *mut RECT, param_30005: *mut core::ffi::c_void, param_16716: i32, param_61331: *mut core::ffi::c_void, param_48043: *mut INT) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipBitmapCreateApplyEffect desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipBitmapLockBits(param_60761: *mut GpBitmap, param_58558: GDIPCONST, param_2971: UINT, param_35960: PixelFormat, param_64884: *mut BitmapData) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipBitmapLockBits desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipBitmapUnlockBits(param_60761: *mut GpBitmap, param_64884: *mut BitmapData) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipBitmapUnlockBits desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromFile(param_58558: GDIPCONST, param_30005: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateBitmapFromFile desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromFileICM(param_58558: GDIPCONST, param_30005: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateBitmapFromFileICM desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromGdiDib(param_58558: GDIPCONST, param_9082: *mut VOID, param_30005: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateBitmapFromGdiDib desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromGraphics(param_18538: INT, param_18538: INT, param_59643: *mut GpGraphics, param_30005: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateBitmapFromGraphics desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromHBITMAP(param_46717: HBITMAP, param_64326: HPALETTE, param_30005: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateBitmapFromHBITMAP desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromHICON(param_183: HICON, param_30005: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateBitmapFromHICON desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromResource(param_49462: HINSTANCE, param_58558: GDIPCONST, param_30005: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateBitmapFromResource desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromScan0(param_18538: INT, param_18538: INT, param_18538: INT, param_35960: PixelFormat, param_60692: *mut BYTE, param_30005: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateBitmapFromScan0 desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromStream(param_39451: *mut IStream, param_30005: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateBitmapFromStream desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromStreamICM(param_39451: *mut IStream, param_30005: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateBitmapFromStreamICM desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateHBITMAPFromBitmap(param_60761: *mut GpBitmap, param_25788: *mut HBITMAP, param_59036: ARGB) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateHBITMAPFromBitmap desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateHICONFromBitmap(param_60761: *mut GpBitmap, param_58126: *mut HICON) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateHICONFromBitmap desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateCachedBitmap(param_60761: *mut GpBitmap, param_59643: *mut GpGraphics, param_13896: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateCachedBitmap desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateCustomLineCap(param_30906: *mut GpPath, param_30906: *mut GpPath, param_63979: GpLineCap, param_35563: REAL, param_22217: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateCustomLineCap desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFont(param_58558: GDIPCONST, param_35563: REAL, param_18538: INT, param_8983: Unit, param_3631: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateFont desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFontFromDC(param_216: HDC, param_3631: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateFontFromDC desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFontFamilyFromName(param_58558: GDIPCONST, param_55787: *mut GpFontCollection, param_32932: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateFontFamilyFromName desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFromHDC(param_216: HDC, param_62342: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateFromHDC desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFromHDC2(param_216: HDC, param_31864: *mut core::ffi::c_void, param_62342: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateFromHDC2 desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFromHWND(param_11550: *mut core::ffi::c_void, param_62342: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateFromHWND desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFromHWNDICM(param_11550: *mut core::ffi::c_void, param_62342: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateFromHWNDICM desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateHalftonePalette() -> HPALETTE WINGDIPAPI {
    // TODO: implementar GdipCreateHalftonePalette desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePath(param_11948: GpFillMode, param_20606: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreatePath desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePath2(param_58558: GDIPCONST, param_58558: GDIPCONST, param_18538: INT, param_11948: GpFillMode, param_20606: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreatePath2 desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePath2I(param_58558: GDIPCONST, param_58558: GDIPCONST, param_18538: INT, param_11948: GpFillMode, param_20606: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreatePath2I desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateHatchBrush(param_33367: GpHatchStyle, param_59036: ARGB, param_59036: ARGB, param_23712: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateHatchBrush desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateImageAttributes(param_13092: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateImageAttributes desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateLineBrush(param_58558: GDIPCONST, param_58558: GDIPCONST, param_59036: ARGB, param_59036: ARGB, param_29378: GpWrapMode, param_32335: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateLineBrush desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateLineBrushI(param_58558: GDIPCONST, param_58558: GDIPCONST, param_59036: ARGB, param_59036: ARGB, param_29378: GpWrapMode, param_32335: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateLineBrushI desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateLineBrushFromRect(param_58558: GDIPCONST, param_59036: ARGB, param_59036: ARGB, param_10936: LinearGradientMode, param_29378: GpWrapMode, param_32335: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateLineBrushFromRect desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateLineBrushFromRectI(param_58558: GDIPCONST, param_59036: ARGB, param_59036: ARGB, param_10936: LinearGradientMode, param_29378: GpWrapMode, param_32335: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateLineBrushFromRectI desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateLineBrushFromRectWithAngle(param_58558: GDIPCONST, param_59036: ARGB, param_59036: ARGB, param_35563: REAL, param_16716: i32, param_29378: GpWrapMode, param_32335: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateLineBrushFromRectWithAngle desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateLineBrushFromRectWithAngleI(param_58558: GDIPCONST, param_59036: ARGB, param_59036: ARGB, param_35563: REAL, param_16716: i32, param_29378: GpWrapMode, param_32335: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateLineBrushFromRectWithAngleI desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMatrix(param_3267: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateMatrix desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMatrix2(param_35563: REAL, param_35563: REAL, param_35563: REAL, param_35563: REAL, param_35563: REAL, param_35563: REAL, param_3267: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateMatrix2 desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMatrix3(param_2677: *mut GDIPCONST GpRectF, param_58558: GDIPCONST, param_3267: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateMatrix3 desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMatrix3I(param_58558: GDIPCONST, param_58558: GDIPCONST, param_3267: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateMatrix3I desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMetafileFromEmf(param_9755: HENHMETAFILE, param_16716: i32, param_46095: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateMetafileFromEmf desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMetafileFromWmf(param_246: HMETAFILE, param_16716: i32, param_58558: GDIPCONST, param_46095: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateMetafileFromWmf desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMetafileFromWmfFile(param_58558: GDIPCONST, param_58558: GDIPCONST, param_46095: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateMetafileFromWmfFile desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMetafileFromFile(param_58558: GDIPCONST, param_46095: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateMetafileFromFile desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMetafileFromStream(param_39451: *mut IStream, param_46095: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateMetafileFromStream desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePathGradient(param_58558: GDIPCONST, param_18538: INT, param_29378: GpWrapMode, param_21747: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreatePathGradient desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePathGradientI(param_58558: GDIPCONST, param_18538: INT, param_29378: GpWrapMode, param_21747: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreatePathGradientI desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePathGradientFromPath(param_58558: GDIPCONST, param_21747: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreatePathGradientFromPath desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePathIter(param_54610: *mut core::ffi::c_void, param_30906: *mut GpPath) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreatePathIter desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePen1(param_59036: ARGB, param_35563: REAL, param_13111: GpUnit, param_39954: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreatePen1 desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePen2(param_25650: *mut GpBrush, param_35563: REAL, param_13111: GpUnit, param_39954: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreatePen2 desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateRegion(param_38003: *mut GpRegion) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateRegion desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateRegionPath(param_30906: *mut GpPath, param_38003: *mut GpRegion) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateRegionPath desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateRegionRect(param_2677: *mut GDIPCONST GpRectF, param_38003: *mut GpRegion) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateRegionRect desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateRegionRectI(param_11075: *mut GDIPCONST GpRect, param_38003: *mut GpRegion) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateRegionRectI desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateRegionRgnData(param_41396: *mut GDIPCONST BYTE, param_18538: INT, param_38003: *mut GpRegion) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateRegionRgnData desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateRegionHrgn(param_5592: HRGN, param_38003: *mut GpRegion) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateRegionHrgn desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateSolidFill(param_59036: ARGB, param_63853: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateSolidFill desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateStringFormat(param_18538: INT, param_23558: LANGID, param_25140: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateStringFormat desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateTexture(param_63808: *mut GpImage, param_29378: GpWrapMode, param_20747: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateTexture desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateTexture2(param_63808: *mut GpImage, param_29378: GpWrapMode, param_35563: REAL, param_35563: REAL, param_35563: REAL, param_35563: REAL, param_20747: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateTexture2 desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateTexture2I(param_63808: *mut GpImage, param_29378: GpWrapMode, param_18538: INT, param_18538: INT, param_18538: INT, param_18538: INT, param_20747: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateTexture2I desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateTextureIA(param_63808: *mut GpImage, param_58558: GDIPCONST, param_35563: REAL, param_35563: REAL, param_35563: REAL, param_35563: REAL, param_20747: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateTextureIA desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateTextureIAI(param_63808: *mut GpImage, param_58558: GDIPCONST, param_18538: INT, param_18538: INT, param_18538: INT, param_18538: INT, param_20747: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateTextureIAI desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateStreamOnFile(param_58558: GDIPCONST, param_2971: UINT, param_11530: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateStreamOnFile desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn HttpCreateHttpHandle(param_10895: PHANDLE, param_30140: ULONG) -> HTTPAPI_LINKAGE ULONG {
    // TODO: implementar HttpCreateHttpHandle desde wine/http.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn HttpCreateRequestQueue(version: HTTPAPI_VERSION, name: *mut const WCHAR, sa: *mut SECURITY_ATTRIBUTES, flags: ULONG, handle: *mut *mut core::ffi::c_void) -> HTTPAPI_LINKAGE ULONG {
    // TODO: implementar HttpCreateRequestQueue desde wine/http.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn HttpCreateServerSession(param_14289: HTTPAPI_VERSION, param_32066: PHTTP_SERVER_SESSION_ID, param_30140: ULONG) -> HTTPAPI_LINKAGE ULONG {
    // TODO: implementar HttpCreateServerSession desde wine/http.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn HttpCreateUrlGroup(session_id: HTTP_SERVER_SESSION_ID, group_id: *mut HTTP_URL_GROUP_ID, reserved: ULONG) -> HTTPAPI_LINKAGE ULONG {
    // TODO: implementar HttpCreateUrlGroup desde wine/http.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateColorTransformA(param_7377: LPLOGCOLORSPACEA, param_7957: HPROFILE, param_7957: HPROFILE, param_54075: u32) -> HTRANSFORM {
    // TODO: implementar CreateColorTransformA desde wine/icm.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateColorTransformW(param_49762: LPLOGCOLORSPACEW, param_7957: HPROFILE, param_7957: HPROFILE, param_54075: u32) -> HTRANSFORM {
    // TODO: implementar CreateColorTransformW desde wine/icm.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDeviceLinkProfile(param_23837: PHPROFILE, param_54075: u32, param_64752: PDWORD, param_54075: u32, param_54075: u32, param_54983: *mut PBYTE, param_54075: u32) -> i32 {
    // TODO: implementar CreateDeviceLinkProfile desde wine/icm.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateMultiProfileTransform(param_23837: PHPROFILE, param_54075: u32, param_64752: PDWORD, param_54075: u32, param_54075: u32, param_54075: u32) -> HTRANSFORM {
    // TODO: implementar CreateMultiProfileTransform desde wine/icm.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IcmpCreateFile(param_9080: VOID) -> *mut core::ffi::c_void {
    // TODO: implementar IcmpCreateFile desde wine/icmpapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Icmp6CreateFile(param_9080: VOID) -> *mut core::ffi::c_void {
    // TODO: implementar Icmp6CreateFile desde wine/icmpapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ublock_getCode(c: UChar32) -> UBlockCode {
    // TODO: implementar ublock_getCode desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ImmCreateContext() -> HIMC {
    // TODO: implementar ImmCreateContext desde wine/imm.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ImmLockIMC(param_6687: HIMC) -> LPINPUTCONTEXT {
    // TODO: implementar ImmLockIMC desde wine/immdev.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ImmUnlockIMC(param_6687: HIMC) -> i32 {
    // TODO: implementar ImmUnlockIMC desde wine/immdev.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ImmGetIMCLockCount(param_6687: HIMC) -> u32 {
    // TODO: implementar ImmGetIMCLockCount desde wine/immdev.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ImmCreateIMCC(param_54075: u32) -> HIMCC {
    // TODO: implementar ImmCreateIMCC desde wine/immdev.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ImmLockIMCC(param_35724: HIMCC) -> LPVOID {
    // TODO: implementar ImmLockIMCC desde wine/immdev.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ImmUnlockIMCC(param_35724: HIMCC) -> i32 {
    // TODO: implementar ImmUnlockIMCC desde wine/immdev.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ImmGetIMCCLockCount(param_35724: HIMCC) -> u32 {
    // TODO: implementar ImmGetIMCCLockCount desde wine/immdev.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ImmCreateSoftKeyboard(param_2971: UINT, param_2971: UINT, param_59621: i32, param_59621: i32) -> *mut core::ffi::c_void {
    // TODO: implementar ImmCreateSoftKeyboard desde wine/immdev.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateInteractionContext(context: *mut HINTERACTIONCONTEXT) -> i32 {
    // TODO: implementar CreateInteractionContext desde wine/interactioncontext.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateIpForwardEntry(pRoute: PMIB_IPFORWARDROW) -> IPHLPAPI_DLL_LINKAGE DWORD {
    // TODO: implementar CreateIpForwardEntry desde wine/iphlpapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateIpNetEntry(pArpEntry: PMIB_IPNETROW) -> IPHLPAPI_DLL_LINKAGE DWORD {
    // TODO: implementar CreateIpNetEntry desde wine/iphlpapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateProxyArpEntry(dwAddress: u32, dwMask: u32, dwIfIndex: u32) -> IPHLPAPI_DLL_LINKAGE DWORD {
    // TODO: implementar CreateProxyArpEntry desde wine/iphlpapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateIProp(param_39047: LPCIID, param_9969: *mut ALLOCATEBUFFER, param_18282: *mut ALLOCATEMORE, param_55716: *mut FREEBUFFER, param_29262: LPVOID, param_13456: *mut LPPROPDATA) -> SCODE {
    // TODO: implementar CreateIProp desde wine/mapiutil.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTable(param_39047: LPCIID, param_9969: *mut ALLOCATEBUFFER, param_18282: *mut ALLOCATEMORE, param_55716: *mut FREEBUFFER, param_29262: LPVOID, param_30140: ULONG, param_30140: ULONG, param_37693: LPSPropTagArray, param_5417: *mut LPTABLEDATA) -> SCODE {
    // TODO: implementar CreateTable desde wine/mapiutil.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateFileMapping2(param_31864: *mut core::ffi::c_void, param_16669: LPSECURITY_ATTRIBUTES, param_30140: ULONG, param_30140: ULONG, param_30140: ULONG, param_52756: ULONG64, param_60164: const, param_30268: *mut MEM_EXTENDED_PARAMETER, param_30140: ULONG) -> WINBASEAPI HANDLE {
    // TODO: implementar CreateFileMapping2 desde wine/memoryapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFBeginCreateFile(access_mode: MF_FILE_ACCESSMODE, open_mode: MF_FILE_OPENMODE, flags: MF_FILE_FLAGS, path: *mut const WCHAR, callback: *mut IMFAsyncCallback, state: *mut IUnknown, param_46438: *mut IUnknown) -> i32 {
    // TODO: implementar MFBeginCreateFile desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCancelCreateFile(cancel_cookie: *mut IUnknown) -> i32 {
    // TODO: implementar MFCancelCreateFile desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreate2DMediaBuffer(width: u32, height: u32, fourcc: u32, bottom_up: i32, param_1236: *mut IMFMediaBuffer) -> i32 {
    // TODO: implementar MFCreate2DMediaBuffer desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateAlignedMemoryBuffer(max_length: u32, alignment: u32, param_1236: *mut IMFMediaBuffer) -> i32 {
    // TODO: implementar MFCreateAlignedMemoryBuffer desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateAttributes(param_22124: *mut IMFAttributes, size: UINT32) -> i32 {
    // TODO: implementar MFCreateAttributes desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateAsyncResult(object: *mut IUnknown, callback: *mut IMFAsyncCallback, state: *mut IUnknown, param_15266: *mut IMFAsyncResult) -> i32 {
    // TODO: implementar MFCreateAsyncResult desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateAudioMediaType(audioformat: *mut const WAVEFORMATEX, param_53108: *mut IMFAudioMediaType) -> i32 {
    // TODO: implementar MFCreateAudioMediaType desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateCollection(param_24213: *mut IMFCollection) -> i32 {
    // TODO: implementar MFCreateCollection desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateDXGIDeviceManager(token: *mut UINT, param_12001: *mut IMFDXGIDeviceManager) -> i32 {
    // TODO: implementar MFCreateDXGIDeviceManager desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateDXGISurfaceBuffer(riid: REFIID, surface: *mut IUnknown, subresource: UINT, bottomup: i32, param_1236: *mut IMFMediaBuffer) -> i32 {
    // TODO: implementar MFCreateDXGISurfaceBuffer desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateDXSurfaceBuffer(riid: REFIID, surface: *mut IUnknown, bottom_up: i32, param_1236: *mut IMFMediaBuffer) -> i32 {
    // TODO: implementar MFCreateDXSurfaceBuffer desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateEventQueue(param_8977: *mut IMFMediaEventQueue) -> i32 {
    // TODO: implementar MFCreateEventQueue desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateFile(accessmode: MF_FILE_ACCESSMODE, openmode: MF_FILE_OPENMODE, flags: MF_FILE_FLAGS, url: LPCWSTR, param_40487: *mut IMFByteStream) -> i32 {
    // TODO: implementar MFCreateFile desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateMediaBufferFromMediaType(media_type: *mut IMFMediaType, duration: LONGLONG, min_length: u32, min_alignment: u32, param_1236: *mut IMFMediaBuffer) -> i32 {
    // TODO: implementar MFCreateMediaBufferFromMediaType desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateMediaEvent(type: MediaEventType, extended_type: REFGUID, status: i32, value: *mut const PROPVARIANT, param_33865: *mut IMFMediaEvent) -> i32 {
    // TODO: implementar MFCreateMediaEvent desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateMediaType(param_29547: *mut IMFMediaType) -> i32 {
    // TODO: implementar MFCreateMediaType desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateAMMediaTypeFromMFMediaType(media_type: *mut IMFMediaType, format_type: GUID, param_15857: *mut AM_MEDIA_TYPE) -> i32 {
    // TODO: implementar MFCreateAMMediaTypeFromMFMediaType desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateMFVideoFormatFromMFMediaType(media_type: *mut IMFMediaType, param_31186: *mut MFVIDEOFORMAT, size: *mut UINT32) -> i32 {
    // TODO: implementar MFCreateMFVideoFormatFromMFMediaType desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateMediaTypeFromRepresentation(guid_representation: GUID, representation: *mut core::ffi::c_void, param_29547: *mut IMFMediaType) -> i32 {
    // TODO: implementar MFCreateMediaTypeFromRepresentation desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateSample(param_51841: *mut IMFSample) -> i32 {
    // TODO: implementar MFCreateSample desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateTempFile(accessmode: MF_FILE_ACCESSMODE, openmode: MF_FILE_OPENMODE, flags: MF_FILE_FLAGS, param_40487: *mut IMFByteStream) -> i32 {
    // TODO: implementar MFCreateTempFile desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateVideoMediaType(format: *mut const MFVIDEOFORMAT, param_14029: *mut IMFVideoMediaType) -> i32 {
    // TODO: implementar MFCreateVideoMediaType desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateVideoMediaTypeFromSubtype(subtype: *mut const GUID, param_14029: *mut IMFVideoMediaType) -> i32 {
    // TODO: implementar MFCreateVideoMediaTypeFromSubtype desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateLegacyMediaBufferOnMFMediaBuffer(sample: *mut IMFSample, media_buffer: *mut IMFMediaBuffer, offset: u32, param_55261: *mut IMediaBuffer) -> i32 {
    // TODO: implementar MFCreateLegacyMediaBufferOnMFMediaBuffer desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateVideoMediaTypeFromVideoInfoHeader(vih: *mut const KS_VIDEOINFOHEADER, size: u32, pixel_aspect_ratio_x: u32, pixel_aspect_ratio_y: u32, interlace_mode: MFVideoInterlaceMode, video_flags: QWORD, subtype: *mut const GUID, param_14029: *mut IMFVideoMediaType) -> i32 {
    // TODO: implementar MFCreateVideoMediaTypeFromVideoInfoHeader desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateMemoryBuffer(max_length: u32, param_1236: *mut IMFMediaBuffer) -> i32 {
    // TODO: implementar MFCreateMemoryBuffer desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFCreateWaveFormatExFromMFMediaType(type: *mut IMFMediaType, param_11923: *mut WAVEFORMATEX, size: *mut UINT32, flags: UINT32) -> i32 {
    // TODO: implementar MFCreateWaveFormatExFromMFMediaType desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFEndCreateFile(result: *mut IMFAsyncResult, param_40487: *mut IMFByteStream) -> i32 {
    // TODO: implementar MFEndCreateFile desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFLockDXGIDeviceManager(token: *mut UINT, param_12001: *mut IMFDXGIDeviceManager) -> i32 {
    // TODO: implementar MFLockDXGIDeviceManager desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFLockPlatform() -> i32 {
    // TODO: implementar MFLockPlatform desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFLockSharedWorkQueue(name: *mut const WCHAR, base_priority: LONG, taskid: *mut u32, queue: *mut u32) -> i32 {
    // TODO: implementar MFLockSharedWorkQueue desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFUnlockDXGIDeviceManager() -> i32 {
    // TODO: implementar MFUnlockDXGIDeviceManager desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFUnlockPlatform() -> i32 {
    // TODO: implementar MFUnlockPlatform desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFUnlockWorkQueue(queue: u32) -> i32 {
    // TODO: implementar MFUnlockWorkQueue desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MesEncodeIncrementalHandleCreate(param_64866: *mut core::ffi::c_void, param_3533: MIDL_ES_ALLOC, param_19743: MIDL_ES_WRITE, param_17426: *mut handle_t) -> RPC_STATUS RPC_ENTRY {
    // TODO: implementar MesEncodeIncrementalHandleCreate desde wine/midles.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MesDecodeIncrementalHandleCreate(param_64866: *mut core::ffi::c_void, param_30283: MIDL_ES_READ, param_17426: *mut handle_t) -> RPC_STATUS RPC_ENTRY {
    // TODO: implementar MesDecodeIncrementalHandleCreate desde wine/midles.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MesEncodeFixedBufferHandleCreate(param_4138: *mut i8, param_30140: ULONG, param_1537: *mut ULONG, param_17426: *mut handle_t) -> RPC_STATUS RPC_ENTRY {
    // TODO: implementar MesEncodeFixedBufferHandleCreate desde wine/midles.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MesEncodeDynBufferHandleCreate(param_4138: *mut i8, param_1537: *mut ULONG, param_17426: *mut handle_t) -> RPC_STATUS RPC_ENTRY {
    // TODO: implementar MesEncodeDynBufferHandleCreate desde wine/midles.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MesDecodeBufferHandleCreate(param_4138: *mut i8, param_30140: ULONG, param_17426: *mut handle_t) -> RPC_STATUS RPC_ENTRY {
    // TODO: implementar MesDecodeBufferHandleCreate desde wine/midles.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mmTaskCreate(param_2940: LPTASK, param_41017: *mut *mut core::ffi::c_void, param_56347: DWORD_PTR) -> UINT {
    // TODO: implementar mmTaskCreate desde wine/mmddk.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mmTaskBlock(param_54075: u32) -> VOID {
    // TODO: implementar mmTaskBlock desde wine/mmddk.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mmioCreateChunk(param_52564: HMMIO, param_15334: *mut MMCKINFO, param_2971: UINT) -> WINMMAPI MMRESULT {
    // TODO: implementar mmioCreateChunk desde wine/mmsystem.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ASN1_CreateDecoder(param_43058: ASN1module_t, param_22868: *mut ASN1decoding_t, param_62120: *mut ASN1octet_t, param_38864: ASN1uint32_t, param_29687: ASN1decoding_t) -> ASN1error_e {
    // TODO: implementar ASN1_CreateDecoder desde wine/msasn1.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ASN1_CreateDecoderEx(param_43058: ASN1module_t, param_22868: *mut ASN1decoding_t, param_62120: *mut ASN1octet_t, param_38864: ASN1uint32_t, param_29687: ASN1decoding_t, param_38864: ASN1uint32_t) -> ASN1error_e {
    // TODO: implementar ASN1_CreateDecoderEx desde wine/msasn1.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ASN1_CreateEncoder(param_43058: ASN1module_t, param_64611: *mut ASN1encoding_t, param_62120: *mut ASN1octet_t, param_38864: ASN1uint32_t, param_44115: ASN1encoding_t) -> ASN1error_e {
    // TODO: implementar ASN1_CreateEncoder desde wine/msasn1.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ASN1_CreateModule(param_38864: ASN1uint32_t, param_48291: ASN1encodingrule_e, param_38864: ASN1uint32_t, param_38864: ASN1uint32_t, param_63959: ASN1magic_t) -> ASN1module_t {
    // TODO: implementar ASN1_CreateModule desde wine/msasn1.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MsiCreateRecord(param_2971: UINT) -> MSIHANDLE {
    // TODO: implementar MsiCreateRecord desde wine/msiquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MsiCreateTransformSummaryInfoA(param_63707: MSIHANDLE, param_63707: MSIHANDLE, param_15619: LPCSTR, param_59621: i32, param_59621: i32) -> UINT {
    // TODO: implementar MsiCreateTransformSummaryInfoA desde wine/msiquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MsiCreateTransformSummaryInfoW(param_63707: MSIHANDLE, param_63707: MSIHANDLE, param_25711: LPCWSTR, param_59621: i32, param_59621: i32) -> UINT {
    // TODO: implementar MsiCreateTransformSummaryInfoW desde wine/msiquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptSIPCreateIndirectData(param_47481: *mut SIP_SUBJECTINFO, param_19332: *mut u32, param_42634: *mut SIP_INDIRECT_DATA) -> i32 {
    // TODO: implementar CryptSIPCreateIndirectData desde wine/mssip.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateBoundaryDescriptorW(param_25711: LPCWSTR, param_30140: ULONG) -> WINBASEAPI HANDLE {
    // TODO: implementar CreateBoundaryDescriptorW desde wine/namespaceapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreatePrivateNamespaceW(param_16669: LPSECURITY_ATTRIBUTES, param_29262: LPVOID, param_25711: LPCWSTR) -> WINBASEAPI HANDLE {
    // TODO: implementar CreatePrivateNamespaceW desde wine/namespaceapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NCryptCreatePersistedKey(param_36590: NCRYPT_PROV_HANDLE, param_4898: *mut NCRYPT_KEY_HANDLE, param_51335: *mut const WCHAR, param_51335: *mut const WCHAR, param_54075: u32, param_54075: u32) -> SECURITY_STATUS {
    // TODO: implementar NCryptCreatePersistedKey desde wine/ncrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateBitmap(width: INT, height: INT, planes: UINT, bpp: UINT, bits: *mut const void) -> W32KAPI HBITMAP {
    // TODO: implementar NtGdiCreateBitmap desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateClientObj(type: ULONG) -> W32KAPI HANDLE {
    // TODO: implementar NtGdiCreateClientObj desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateCompatibleBitmap(hdc: HDC, width: INT, height: INT) -> W32KAPI HBITMAP {
    // TODO: implementar NtGdiCreateCompatibleBitmap desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateCompatibleDC(hdc: HDC) -> W32KAPI HDC {
    // TODO: implementar NtGdiCreateCompatibleDC desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateDIBBrush(data: *mut const void, coloruse: UINT, size: UINT, is_8x8: i32, pen: i32, client: *mut const void) -> W32KAPI HBRUSH {
    // TODO: implementar NtGdiCreateDIBBrush desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateDIBSection(hdc: HDC, section: *mut core::ffi::c_void, offset: u32, bmi: *mut const BITMAPINFO, usage: UINT, header_size: UINT, flags: ULONG, color_space: ULONG_PTR, param_64866: *mut core::ffi::c_void) -> W32KAPI HBITMAP {
    // TODO: implementar NtGdiCreateDIBSection desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateDIBitmapInternal(hdc: HDC, width: INT, height: INT, init: u32, bits: *mut const void, data: *mut const BITMAPINFO, coloruse: UINT, max_info: UINT, max_bits: UINT, flags: ULONG, xform: *mut core::ffi::c_void) -> W32KAPI HBITMAP {
    // TODO: implementar NtGdiCreateDIBitmapInternal desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateEllipticRgn(left: INT, top: INT, right: INT, bottom: INT) -> W32KAPI HRGN {
    // TODO: implementar NtGdiCreateEllipticRgn desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateHalftonePalette(hdc: HDC) -> W32KAPI HPALETTE {
    // TODO: implementar NtGdiCreateHalftonePalette desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateHatchBrushInternal(style: INT, color: COLORREF, pen: i32) -> W32KAPI HBRUSH {
    // TODO: implementar NtGdiCreateHatchBrushInternal desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateMetafileDC(hdc: HDC) -> W32KAPI HDC {
    // TODO: implementar NtGdiCreateMetafileDC desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreatePaletteInternal(palette: *mut const LOGPALETTE, count: UINT) -> W32KAPI HPALETTE {
    // TODO: implementar NtGdiCreatePaletteInternal desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreatePatternBrushInternal(hbitmap: HBITMAP, pen: i32, is_8x8: i32) -> W32KAPI HBRUSH {
    // TODO: implementar NtGdiCreatePatternBrushInternal desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreatePen(style: INT, width: INT, color: COLORREF, brush: HBRUSH) -> W32KAPI HPEN {
    // TODO: implementar NtGdiCreatePen desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateRectRgn(left: INT, top: INT, right: INT, bottom: INT) -> W32KAPI HRGN {
    // TODO: implementar NtGdiCreateRectRgn desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateRoundRectRgn(left: INT, top: INT, right: INT, bottom: INT, ellipse_width: INT, ellipse_height: INT) -> W32KAPI HRGN {
    // TODO: implementar NtGdiCreateRoundRectRgn desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateSolidBrush(color: COLORREF, brush: HBRUSH) -> W32KAPI HBRUSH {
    // TODO: implementar NtGdiCreateSolidBrush desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiExtCreatePen(style: u32, width: u32, brush_style: ULONG, color: ULONG, client_hatch: ULONG_PTR, hatch: ULONG_PTR, style_count: u32, style_bits: *mut const DWORD, dib_size: ULONG, old_style: i32, brush: HBRUSH) -> W32KAPI HPEN {
    // TODO: implementar NtGdiExtCreatePen desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiExtCreateRegion(xform: *mut const XFORM, count: u32, data: *mut const RGNDATA) -> W32KAPI HRGN {
    // TODO: implementar NtGdiExtCreateRegion desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiHfontCreate(logfont: *mut const void, unk2: ULONG, unk3: ULONG, unk4: ULONG, data: *mut core::ffi::c_void) -> W32KAPI HFONT {
    // TODO: implementar NtGdiHfontCreate desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIAcquireKeyedMutex(params: *mut D3DKMT_ACQUIREKEYEDMUTEX) -> W32KAPI NTSTATUS {
    // TODO: implementar NtGdiDdDDIAcquireKeyedMutex desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIAcquireKeyedMutex2(params: *mut D3DKMT_ACQUIREKEYEDMUTEX2) -> W32KAPI NTSTATUS {
    // TODO: implementar NtGdiDdDDIAcquireKeyedMutex2 desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDICreateDCFromMemory(desc: *mut D3DKMT_CREATEDCFROMMEMORY) -> W32KAPI NTSTATUS {
    // TODO: implementar NtGdiDdDDICreateDCFromMemory desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDICreateDevice(desc: *mut D3DKMT_CREATEDEVICE) -> W32KAPI NTSTATUS {
    // TODO: implementar NtGdiDdDDICreateDevice desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDICreateKeyedMutex(params: *mut D3DKMT_CREATEKEYEDMUTEX) -> W32KAPI NTSTATUS {
    // TODO: implementar NtGdiDdDDICreateKeyedMutex desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDICreateKeyedMutex2(params: *mut D3DKMT_CREATEKEYEDMUTEX2) -> W32KAPI NTSTATUS {
    // TODO: implementar NtGdiDdDDICreateKeyedMutex2 desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDICreateSynchronizationObject(params: *mut D3DKMT_CREATESYNCHRONIZATIONOBJECT) -> W32KAPI NTSTATUS {
    // TODO: implementar NtGdiDdDDICreateSynchronizationObject desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDICreateSynchronizationObject2(params: *mut D3DKMT_CREATESYNCHRONIZATIONOBJECT2) -> W32KAPI NTSTATUS {
    // TODO: implementar NtGdiDdDDICreateSynchronizationObject2 desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIDestroyKeyedMutex(params: *mut const D3DKMT_DESTROYKEYEDMUTEX) -> W32KAPI NTSTATUS {
    // TODO: implementar NtGdiDdDDIDestroyKeyedMutex desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIOpenKeyedMutex(params: *mut D3DKMT_OPENKEYEDMUTEX) -> W32KAPI NTSTATUS {
    // TODO: implementar NtGdiDdDDIOpenKeyedMutex desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIOpenKeyedMutex2(params: *mut D3DKMT_OPENKEYEDMUTEX2) -> W32KAPI NTSTATUS {
    // TODO: implementar NtGdiDdDDIOpenKeyedMutex2 desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIOpenKeyedMutexFromNtHandle(params: *mut D3DKMT_OPENKEYEDMUTEXFROMNTHANDLE) -> W32KAPI NTSTATUS {
    // TODO: implementar NtGdiDdDDIOpenKeyedMutexFromNtHandle desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIReleaseKeyedMutex(params: *mut D3DKMT_RELEASEKEYEDMUTEX) -> W32KAPI NTSTATUS {
    // TODO: implementar NtGdiDdDDIReleaseKeyedMutex desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIReleaseKeyedMutex2(params: *mut D3DKMT_RELEASEKEYEDMUTEX2) -> W32KAPI NTSTATUS {
    // TODO: implementar NtGdiDdDDIReleaseKeyedMutex2 desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LsaCreateTrustedDomainEx(param_26730: LSA_HANDLE, param_30917: PTRUSTED_DOMAIN_INFORMATION_EX, param_22080: PTRUSTED_DOMAIN_AUTH_INFORMATION, param_15666: ACCESS_MASK, param_56721: PLSA_HANDLE) -> WINADVAPI NTSTATUS {
    // TODO: implementar LsaCreateTrustedDomainEx desde wine/ntsecapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateAcceleratorTable(table: *mut ACCEL, count: INT) -> W32KAPI HACCEL {
    // TODO: implementar NtUserCreateAcceleratorTable desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateCaret(hwnd: *mut core::ffi::c_void, bitmap: HBITMAP, width: i32, height: i32) -> W32KAPI BOOL {
    // TODO: implementar NtUserCreateCaret desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateDesktopEx(attr: *mut OBJECT_ATTRIBUTES, device: *mut UNICODE_STRING, devmode: *mut DEVMODEW, flags: u32, access: ACCESS_MASK, heap_size: ULONG) -> W32KAPI HDESK {
    // TODO: implementar NtUserCreateDesktopEx desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateInputContext(client_ptr: UINT_PTR) -> W32KAPI HIMC {
    // TODO: implementar NtUserCreateInputContext desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateMenu() -> W32KAPI HMENU {
    // TODO: implementar NtUserCreateMenu desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserCreatePopupMenu() -> W32KAPI HMENU {
    // TODO: implementar NtUserCreatePopupMenu desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateWindowEx(ex_style: u32, class_name: *mut UNICODE_STRING, version: *mut UNICODE_STRING, window_name: *mut UNICODE_STRING, style: u32, x: INT, y: INT, cx: INT, cy: INT, parent: *mut core::ffi::c_void, menu: HMENU, instance: HINSTANCE, params: *mut core::ffi::c_void, flags: u32, client_instance: HINSTANCE, class: *mut const WCHAR, ansi: i32) -> W32KAPI HWND {
    // TODO: implementar NtUserCreateWindowEx desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateWindowStation(attr: *mut OBJECT_ATTRIBUTES, mask: ACCESS_MASK, arg3: ULONG, arg4: ULONG, arg5: ULONG, arg6: ULONG, arg7: ULONG) -> W32KAPI HWINSTA {
    // TODO: implementar NtUserCreateWindowStation desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserLockWindowUpdate(hwnd: *mut core::ffi::c_void) -> W32KAPI BOOL {
    // TODO: implementar NtUserLockWindowUpdate desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateCursorIcon(is_icon: i32) -> static inline HICON {
    // TODO: implementar NtUserCreateCursorIcon desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserEnableThunkLock(thunk_lock_callback: ntuser_callback) -> static inline void {
    // TODO: implementar NtUserEnableThunkLock desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CoLockObjectExternal(pUnk: LPUNKNOWN, fLock: i32, fLastUnlockReleases: i32) -> WINOLE32API HRESULT {
    // TODO: implementar CoLockObjectExternal desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CoCreateGuid(pguid: *mut GUID) -> WINOLE32API HRESULT {
    // TODO: implementar CoCreateGuid desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDataAdviseHolder(ppDAHolder: *mut LPDATAADVISEHOLDER) -> WINOLE32API HRESULT {
    // TODO: implementar CreateDataAdviseHolder desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDataCache(pUnkOuter: LPUNKNOWN, rclsid: REFCLSID, iid: REFIID, ppv: *mut LPVOID) -> WINOLE32API HRESULT {
    // TODO: implementar CreateDataCache desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateAntiMoniker(ppmk: *mut LPMONIKER) -> WINOLE32API HRESULT {
    // TODO: implementar CreateAntiMoniker desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateBindCtx(reserved: u32, ppbc: *mut LPBC) -> WINOLE32API HRESULT {
    // TODO: implementar CreateBindCtx desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateClassMoniker(rclsid: REFCLSID, ppmk: *mut LPMONIKER) -> WINOLE32API HRESULT {
    // TODO: implementar CreateClassMoniker desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateFileMoniker(lpszPathName: LPCOLESTR, ppmk: *mut LPMONIKER) -> WINOLE32API HRESULT {
    // TODO: implementar CreateFileMoniker desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateGenericComposite(pmkFirst: LPMONIKER, pmkRest: LPMONIKER, ppmkComposite: *mut LPMONIKER) -> WINOLE32API HRESULT {
    // TODO: implementar CreateGenericComposite desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateItemMoniker(lpszDelim: LPCOLESTR, lpszItem: LPCOLESTR, ppmk: *mut LPMONIKER) -> WINOLE32API HRESULT {
    // TODO: implementar CreateItemMoniker desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateObjrefMoniker(punk: LPUNKNOWN, ppmk: *mut LPMONIKER) -> WINOLE32API HRESULT {
    // TODO: implementar CreateObjrefMoniker desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreatePointerMoniker(punk: LPUNKNOWN, ppmk: *mut LPMONIKER) -> WINOLE32API HRESULT {
    // TODO: implementar CreatePointerMoniker desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StgCreateDocfile(pwcsName: LPCOLESTR, grfMode: u32, reserved: u32, param_54990: *mut IStorage) -> WINOLE32API HRESULT {
    // TODO: implementar StgCreateDocfile desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StgCreateStorageEx(param_60164: const, param_54075: u32, param_54075: u32, param_54075: u32, param_57664: *mut STGOPTIONS, param_64866: *mut core::ffi::c_void, param_55479: REFIID, param_47066: *mut core::ffi::c_void) -> WINOLE32API HRESULT {
    // TODO: implementar StgCreateStorageEx desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StgIsStorageILockBytes(plkbyt: *mut ILockBytes) -> WINOLE32API HRESULT {
    // TODO: implementar StgIsStorageILockBytes desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StgCreateDocfileOnILockBytes(plkbyt: *mut ILockBytes, grfMode: u32, reserved: u32, ppstgOpen: *mut core::ffi::c_void) -> WINOLE32API HRESULT {
    // TODO: implementar StgCreateDocfileOnILockBytes desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StgOpenStorageOnILockBytes(plkbyt: *mut ILockBytes, pstgPriority: *mut IStorage, grfMode: u32, snbExclude: SNB, reserved: u32, param_54990: *mut IStorage) -> WINOLE32API HRESULT {
    // TODO: implementar StgOpenStorageOnILockBytes desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SQLCreateDataSource(param_11550: *mut core::ffi::c_void, param_15619: LPCSTR) -> i32 {
    // TODO: implementar SQLCreateDataSource desde wine/odbcinst.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SQLCreateDataSourceW(param_11550: *mut core::ffi::c_void, param_25711: LPCWSTR) -> i32 {
    // TODO: implementar SQLCreateDataSourceW desde wine/odbcinst.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreateMenuDescriptor(hmenuCombined: HMENU, lpMenuWidths: LPOLEMENUGROUPWIDTHS) -> WINOLE32API HOLEMENU {
    // TODO: implementar OleCreateMenuDescriptor desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateStreamOnHGlobal(hGlobal: HGLOBAL, fDeleteOnRelease: i32, ppstm: *mut LPSTREAM) -> WINOLE32API HRESULT {
    // TODO: implementar CreateStreamOnHGlobal desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreateLinkFromData(pSrcDataObj: LPDATAOBJECT, riid: REFIID, renderopt: u32, pFormatEtc: LPFORMATETC, pClientSite: LPOLECLIENTSITE, pStg: LPSTORAGE, ppvObj: *mut LPVOID) -> WINOLE32API HRESULT {
    // TODO: implementar OleCreateLinkFromData desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleQueryCreateFromData(pSrcDataObject: LPDATAOBJECT) -> WINOLE32API HRESULT {
    // TODO: implementar OleQueryCreateFromData desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreateStaticFromData(pSrcDataObj: LPDATAOBJECT, iid: REFIID, renderopt: u32, pFormatEtc: LPFORMATETC, pClientSite: LPOLECLIENTSITE, pStg: LPSTORAGE, ppvObj: *mut LPVOID) -> WINOLE32API HRESULT {
    // TODO: implementar OleCreateStaticFromData desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetHGlobalFromILockBytes(plkbyt: LPLOCKBYTES, phglobal: *mut HGLOBAL) -> WINOLE32API HRESULT {
    // TODO: implementar GetHGlobalFromILockBytes desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateILockBytesOnHGlobal(hGlobal: HGLOBAL, fDeleteOnRelease: i32, pplkbyt: *mut LPLOCKBYTES) -> WINOLE32API HRESULT {
    // TODO: implementar CreateILockBytesOnHGlobal desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleLockRunning(pUnknown: LPUNKNOWN, fLock: i32, fLastUnlockCloses: i32) -> WINOLE32API HRESULT {
    // TODO: implementar OleLockRunning desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreateFromFile(rclsid: REFCLSID, lpszFileName: LPCOLESTR, riid: REFIID, renderopt: u32, lpFormatEtc: LPFORMATETC, pClientSite: LPOLECLIENTSITE, pStg: LPSTORAGE, ppvObj: *mut LPVOID) -> WINOLE32API HRESULT {
    // TODO: implementar OleCreateFromFile desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreateFromFileEx(clsid: REFCLSID, filename: LPCOLESTR, iid: REFIID, flags: u32, renderopt: u32, num_fmts: ULONG, adv_flags: *mut u32, fmts: LPFORMATETC, sink: *mut IAdviseSink, conns: *mut u32, client_site: LPOLECLIENTSITE, storage: LPSTORAGE, obj: *mut LPVOID) -> WINOLE32API HRESULT {
    // TODO: implementar OleCreateFromFileEx desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreateLink(pmkLinkSrc: LPMONIKER, riid: REFIID, renderopt: u32, lpFormatEtc: LPFORMATETC, pClientSite: LPOLECLIENTSITE, pStg: LPSTORAGE, ppvObj: *mut LPVOID) -> WINOLE32API HRESULT {
    // TODO: implementar OleCreateLink desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreate(rclsid: REFCLSID, riid: REFIID, renderopt: u32, pFormatEtc: LPFORMATETC, pClientSite: LPOLECLIENTSITE, pStg: LPSTORAGE, ppvObj: *mut LPVOID) -> WINOLE32API HRESULT {
    // TODO: implementar OleCreate desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreateLinkToFile(lpszFileName: LPCOLESTR, riid: REFIID, renderopt: u32, lpFormatEtc: LPFORMATETC, pClientSite: LPOLECLIENTSITE, pStg: LPSTORAGE, ppvObj: *mut LPVOID) -> WINOLE32API HRESULT {
    // TODO: implementar OleCreateLinkToFile desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreateFromData(pSrcDataObj: LPDATAOBJECT, riid: REFIID, renderopt: u32, pFormatEtc: LPFORMATETC, pClientSite: LPOLECLIENTSITE, pStg: LPSTORAGE, ppvObj: *mut LPVOID) -> WINOLE32API HRESULT {
    // TODO: implementar OleCreateFromData desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreateFromDataEx(pSrcDataObj: LPDATAOBJECT, riid: REFIID, dwFlags: u32, renderopt: u32, num_formats: ULONG, adv_flags: *mut u32, fmts: LPFORMATETC, sink: *mut IAdviseSink, conns: *mut u32, pClientSite: LPOLECLIENTSITE, pStg: LPSTORAGE, ppvObj: *mut LPVOID) -> WINOLE32API HRESULT {
    // TODO: implementar OleCreateFromDataEx desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreateDefaultHandler(clsid: REFCLSID, pUnkOuter: LPUNKNOWN, riid: REFIID, ppvObj: *mut LPVOID) -> WINOLE32API HRESULT {
    // TODO: implementar OleCreateDefaultHandler desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreateEmbeddingHelper(clsid: REFCLSID, pUnkOuter: LPUNKNOWN, flags: u32, pCF: *mut IClassFactory, riid: REFIID, ppvObj: *mut LPVOID) -> WINOLE32API HRESULT {
    // TODO: implementar OleCreateEmbeddingHelper desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateOleAdviseHolder(ppOAHolder: *mut LPOLEADVISEHOLDER) -> WINOLE32API HRESULT {
    // TODO: implementar CreateOleAdviseHolder desde wine/ole2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateErrorInfo(param_6807: *mut core::ffi::c_void) -> WINOLEAUTAPI HRESULT {
    // TODO: implementar CreateErrorInfo desde wine/oleauto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SafeArrayCreate(param_28113: VARTYPE, param_2971: UINT, param_22006: *mut SAFEARRAYBOUND) -> *mut WINOLEAUTAPI SAFEARRAY {
    // TODO: implementar SafeArrayCreate desde wine/oleauto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SafeArrayCreateEx(param_28113: VARTYPE, param_2971: UINT, param_22006: *mut SAFEARRAYBOUND, param_29262: LPVOID) -> *mut WINOLEAUTAPI SAFEARRAY {
    // TODO: implementar SafeArrayCreateEx desde wine/oleauto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SafeArrayCreateVector(param_28113: VARTYPE, param_9910: LONG, param_30140: ULONG) -> *mut WINOLEAUTAPI SAFEARRAY {
    // TODO: implementar SafeArrayCreateVector desde wine/oleauto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SafeArrayCreateVectorEx(param_28113: VARTYPE, param_9910: LONG, param_30140: ULONG, param_29262: LPVOID) -> *mut WINOLEAUTAPI SAFEARRAY {
    // TODO: implementar SafeArrayCreateVectorEx desde wine/oleauto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SafeArrayLock(param_5644: *mut SAFEARRAY) -> WINOLEAUTAPI HRESULT {
    // TODO: implementar SafeArrayLock desde wine/oleauto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SafeArrayUnlock(param_5644: *mut SAFEARRAY) -> WINOLEAUTAPI HRESULT {
    // TODO: implementar SafeArrayUnlock desde wine/oleauto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDispTypeInfo(param_10915: *mut INTERFACEDATA, param_58556: LCID, param_38166: *mut core::ffi::c_void) -> WINOLEAUTAPI HRESULT {
    // TODO: implementar CreateDispTypeInfo desde wine/oleauto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateStdDispatch(param_46438: *mut IUnknown, param_64866: *mut core::ffi::c_void, param_42131: *mut ITypeInfo, param_57333: *mut core::ffi::c_void) -> WINOLEAUTAPI HRESULT {
    // TODO: implementar CreateStdDispatch desde wine/oleauto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTypeLib(param_55998: SYSKIND, param_60164: const, param_53089: *mut core::ffi::c_void) -> WINOLEAUTAPI HRESULT {
    // TODO: implementar CreateTypeLib desde wine/oleauto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTypeLib2(param_55998: SYSKIND, param_21225: LPCOLESTR, param_46173: *mut core::ffi::c_void) -> WINOLEAUTAPI HRESULT {
    // TODO: implementar CreateTypeLib2 desde wine/oleauto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreatePropertyFrameIndirect(lpParams: LPOCPFIPARAMS) -> WINOLECTLAPI {
    // TODO: implementar OleCreatePropertyFrameIndirect desde wine/olectl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreatePropertyFrame(hwndOwner: *mut core::ffi::c_void, x: UINT, y: UINT, lpszCaption: LPCOLESTR, cObjects: ULONG, ppUnk: *mut LPUNKNOWN, cPages: ULONG, pPageClsID: LPCLSID, lcid: LCID, dwReserved: u32, pvReserved: LPVOID) -> WINOLECTLAPI {
    // TODO: implementar OleCreatePropertyFrame desde wine/olectl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreatePictureIndirect(lpPictDesc: LPPICTDESC, riid: REFIID, fOwn: i32, lplpvObj: *mut LPVOID) -> WINOLECTLAPI {
    // TODO: implementar OleCreatePictureIndirect desde wine/olectl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleCreateFontIndirect(lpFontDesc: LPFONTDESC, riid: REFIID, lplpvObj: *mut LPVOID) -> WINOLECTLAPI {
    // TODO: implementar OleCreateFontIndirect desde wine/olectl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreatePropertySheetPageA(param_17370: LPCPROPSHEETPAGEA) -> WINCOMMCTRLAPI HPROPSHEETPAGE {
    // TODO: implementar CreatePropertySheetPageA desde wine/prsht.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreatePropertySheetPageW(param_57367: LPCPROPSHEETPAGEW) -> WINCOMMCTRLAPI HPROPSHEETPAGE {
    // TODO: implementar CreatePropertySheetPageW desde wine/prsht.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn QOSCreateHandle(version: PQOS_VERSION, handle: PHANDLE) -> i32 {
    // TODO: implementar QOSCreateHandle desde wine/qos2.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RasCreatePhonebookEntryA(param_11550: *mut core::ffi::c_void, param_15619: LPCSTR) -> u32 {
    // TODO: implementar RasCreatePhonebookEntryA desde wine/ras.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RasCreatePhonebookEntryW(param_11550: *mut core::ffi::c_void, param_25711: LPCWSTR) -> u32 {
    // TODO: implementar RasCreatePhonebookEntryW desde wine/ras.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RpcSsContextLockExclusive(param_57202: RPC_BINDING_HANDLE, param_10593: PVOID) -> RPCRTAPI RPC_STATUS RPC_ENTRY {
    // TODO: implementar RpcSsContextLockExclusive desde wine/rpcasync.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RpcSsContextLockShared(param_57202: RPC_BINDING_HANDLE, param_10593: PVOID) -> RPCRTAPI RPC_STATUS RPC_ENTRY {
    // TODO: implementar RpcSsContextLockShared desde wine/rpcasync.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UuidCreate(Uuid: *mut UUID) -> RPCRTAPI RPC_STATUS RPC_ENTRY {
    // TODO: implementar UuidCreate desde wine/rpcdce.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UuidCreateSequential(Uuid: *mut UUID) -> RPCRTAPI RPC_STATUS RPC_ENTRY {
    // TODO: implementar UuidCreateSequential desde wine/rpcdce.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UuidCreateNil(Uuid: *mut UUID) -> RPCRTAPI RPC_STATUS RPC_ENTRY {
    // TODO: implementar UuidCreateNil desde wine/rpcdce.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateProxyFromTypeInfo(pTypeInfo: LPTYPEINFO, pUnkOuter: LPUNKNOWN, riid: REFIID, ppProxy: *mut LPRPCPROXYBUFFER, ppv: *mut LPVOID) -> RPCRTAPI HRESULT RPC_ENTRY {
    // TODO: implementar CreateProxyFromTypeInfo desde wine/rpcproxy.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateStubFromTypeInfo(pTypeInfo: LPTYPEINFO, riid: REFIID, pUnkServer: LPUNKNOWN, ppStub: *mut LPRPCSTUBBUFFER) -> RPCRTAPI HRESULT RPC_ENTRY {
    // TODO: implementar CreateStubFromTypeInfo desde wine/rpcproxy.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupCreateDiskSpaceListA(param_10593: PVOID, param_54075: u32, param_2971: UINT) -> WINSETUPAPI HDSKSPC {
    // TODO: implementar SetupCreateDiskSpaceListA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupCreateDiskSpaceListW(param_10593: PVOID, param_54075: u32, param_2971: UINT) -> WINSETUPAPI HDSKSPC {
    // TODO: implementar SetupCreateDiskSpaceListW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInfoList(param_4724: *mut const GUID, param_11550: *mut core::ffi::c_void) -> WINSETUPAPI HDEVINFO {
    // TODO: implementar SetupDiCreateDeviceInfoList desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInfoListExA(param_4724: *mut const GUID, param_11550: *mut core::ffi::c_void, param_29633: PCSTR, param_10593: PVOID) -> WINSETUPAPI HDEVINFO {
    // TODO: implementar SetupDiCreateDeviceInfoListExA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInfoListExW(param_4724: *mut const GUID, param_11550: *mut core::ffi::c_void, param_29658: PCWSTR, param_10593: PVOID) -> WINSETUPAPI HDEVINFO {
    // TODO: implementar SetupDiCreateDeviceInfoListExW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInfoA(param_44350: HDEVINFO, param_29633: PCSTR, param_60164: const, param_29633: PCSTR, param_11550: *mut core::ffi::c_void, param_54075: u32, param_18335: PSP_DEVINFO_DATA) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupDiCreateDeviceInfoA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInfoW(param_44350: HDEVINFO, param_29658: PCWSTR, param_60164: const, param_29658: PCWSTR, param_11550: *mut core::ffi::c_void, param_54075: u32, param_18335: PSP_DEVINFO_DATA) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupDiCreateDeviceInfoW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInterfaceA(param_44350: HDEVINFO, param_18335: PSP_DEVINFO_DATA, param_4724: *mut const GUID, param_29633: PCSTR, param_54075: u32, param_62266: PSP_DEVICE_INTERFACE_DATA) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupDiCreateDeviceInterfaceA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInterfaceW(param_44350: HDEVINFO, param_18335: PSP_DEVINFO_DATA, param_4724: *mut const GUID, param_29658: PCWSTR, param_54075: u32, param_62266: PSP_DEVICE_INTERFACE_DATA) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupDiCreateDeviceInterfaceW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInterfaceRegKeyA(param_44350: HDEVINFO, param_62266: PSP_DEVICE_INTERFACE_DATA, param_54075: u32, param_16523: REGSAM, param_41734: HINF, param_29633: PCSTR) -> WINSETUPAPI HKEY {
    // TODO: implementar SetupDiCreateDeviceInterfaceRegKeyA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInterfaceRegKeyW(param_44350: HDEVINFO, param_62266: PSP_DEVICE_INTERFACE_DATA, param_54075: u32, param_16523: REGSAM, param_41734: HINF, param_29658: PCWSTR) -> WINSETUPAPI HKEY {
    // TODO: implementar SetupDiCreateDeviceInterfaceRegKeyW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDevRegKeyA(param_44350: HDEVINFO, param_18335: PSP_DEVINFO_DATA, param_54075: u32, param_54075: u32, param_54075: u32, param_41734: HINF, param_29633: PCSTR) -> WINSETUPAPI HKEY {
    // TODO: implementar SetupDiCreateDevRegKeyA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDevRegKeyW(param_44350: HDEVINFO, param_18335: PSP_DEVINFO_DATA, param_54075: u32, param_54075: u32, param_54075: u32, param_41734: HINF, param_29658: PCWSTR) -> WINSETUPAPI HKEY {
    // TODO: implementar SetupDiCreateDevRegKeyW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateRandomAccessStreamOverStream(stream: *mut IStream, options: BSOS_OPTIONS, riid: REFIID, param_64866: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar CreateRandomAccessStreamOverStream desde wine/shcore.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreatePropSheetExtArray(param_28092: HKEY, param_25711: LPCWSTR, param_2971: UINT) -> WINSHELLAPI HPSXA {
    // TODO: implementar SHCreatePropSheetExtArray desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreatePropSheetExtArrayEx(param_28092: HKEY, param_25711: LPCWSTR, param_2971: UINT, param_10484: *mut IDataObject) -> WINSHELLAPI HPSXA {
    // TODO: implementar SHCreatePropSheetExtArrayEx desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreateQueryCancelAutoPlayMoniker(param_30075: *mut core::ffi::c_void) -> WINSHELLAPI HRESULT {
    // TODO: implementar SHCreateQueryCancelAutoPlayMoniker desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreateShellItem(param_26047: LPCITEMIDLIST, param_21024: *mut IShellFolder, param_26047: LPCITEMIDLIST, param_12459: *mut core::ffi::c_void) -> WINSHELLAPI HRESULT {
    // TODO: implementar SHCreateShellItem desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreateStdEnumFmtEtc(param_54075: u32, param_16348: *mut const FORMATETC, param_17498: *mut core::ffi::c_void) -> WINSHELLAPI HRESULT {
    // TODO: implementar SHCreateStdEnumFmtEtc desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHILCreateFromPath(param_25711: LPCWSTR, param_56896: *mut LPITEMIDLIST, param_19332: *mut u32) -> WINSHELLAPI HRESULT {
    // TODO: implementar SHILCreateFromPath desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreateShellFolderViewEx(pshfvi: LPCSFV, param_36594: *mut IShellView) -> WINSHELLAPI HRESULT {
    // TODO: implementar SHCreateShellFolderViewEx desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreateShellFolderView(pscfv: *mut const SFV_CREATE, param_36594: *mut IShellView) -> WINSHELLAPI HRESULT {
    // TODO: implementar SHCreateShellFolderView desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHChangeNotification_Lock(hChangeNotification: *mut core::ffi::c_void, dwProcessId: u32, param_56896: *mut LPITEMIDLIST, plEvent: *mut LONG) -> WINSHELLAPI HANDLE {
    // TODO: implementar SHChangeNotification_Lock desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHChangeNotification_Unlock(hLock: *mut core::ffi::c_void) -> WINSHELLAPI BOOL {
    // TODO: implementar SHChangeNotification_Unlock desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreateDirectory(param_11550: *mut core::ffi::c_void, param_38073: LPCVOID) -> WINSHELLAPI DWORD {
    // TODO: implementar SHCreateDirectory desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreateDirectoryExA(param_11550: *mut core::ffi::c_void, param_15619: LPCSTR, param_16669: LPSECURITY_ATTRIBUTES) -> WINSHELLAPI int {
    // TODO: implementar SHCreateDirectoryExA desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreateDirectoryExW(param_11550: *mut core::ffi::c_void, param_25711: LPCWSTR, param_16669: LPSECURITY_ATTRIBUTES) -> WINSHELLAPI int {
    // TODO: implementar SHCreateDirectoryExW desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ILCreateFromPathA(param_60164: const) -> *mut WINSHELLAPI ITEMIDLIST {
    // TODO: implementar ILCreateFromPathA desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ILCreateFromPathW(param_60164: const) -> *mut WINSHELLAPI ITEMIDLIST {
    // TODO: implementar ILCreateFromPathW desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreateDefaultContextMenu(pdcm: *mut const DEFCONTEXTMENU, riid: REFIID, param_64866: *mut core::ffi::c_void) -> WINSHELLAPI HRESULT {
    // TODO: implementar SHCreateDefaultContextMenu desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CDefFolderMenu_Create2(pidlFolder: LPCITEMIDLIST, hwnd: *mut core::ffi::c_void, cidl: UINT, apidl: *mut LPCITEMIDLIST, psf: *mut IShellFolder, lpfn: LPFNDFM, nKeys: UINT, ahkeys: *mut const HKEY, param_42008: *mut IContextMenu) -> WINSHELLAPI HRESULT {
    // TODO: implementar CDefFolderMenu_Create2 desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHRegCreateUSKeyA(param_15619: LPCSTR, param_16523: REGSAM, param_51665: HUSKEY, param_423: PHUSKEY, param_54075: u32) -> WINSHLWAPI LONG {
    // TODO: implementar SHRegCreateUSKeyA desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHRegCreateUSKeyW(param_25711: LPCWSTR, param_16523: REGSAM, param_51665: HUSKEY, param_423: PHUSKEY, param_54075: u32) -> WINSHLWAPI LONG {
    // TODO: implementar SHRegCreateUSKeyW desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AssocCreate(param_15004: CLSID, param_55479: REFIID, param_34430: *mut LPVOID) -> WINSHLWAPI HRESULT {
    // TODO: implementar AssocCreate desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PathCreateFromUrlA(param_15619: LPCSTR, param_32262: LPSTR, param_17803: LPDWORD, param_54075: u32) -> WINSHLWAPI HRESULT {
    // TODO: implementar PathCreateFromUrlA desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PathCreateFromUrlW(param_25711: LPCWSTR, param_46598: LPWSTR, param_17803: LPDWORD, param_54075: u32) -> WINSHLWAPI HRESULT {
    // TODO: implementar PathCreateFromUrlW desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UrlCreateFromPathA(param_15619: LPCSTR, param_32262: LPSTR, param_17803: LPDWORD, param_54075: u32) -> WINSHLWAPI HRESULT {
    // TODO: implementar UrlCreateFromPathA desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UrlCreateFromPathW(param_25711: LPCWSTR, param_46598: LPWSTR, param_17803: LPDWORD, param_54075: u32) -> WINSHLWAPI HRESULT {
    // TODO: implementar UrlCreateFromPathW desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreateShellPalette(param_216: HDC) -> WINSHLWAPI HPALETTE {
    // TODO: implementar SHCreateShellPalette desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreateStreamOnFileA(param_15619: LPCSTR, param_54075: u32, param_50104: struct) -> WINSHLWAPI HRESULT {
    // TODO: implementar SHCreateStreamOnFileA desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreateStreamOnFileW(param_25711: LPCWSTR, param_54075: u32, param_50104: struct) -> WINSHLWAPI HRESULT {
    // TODO: implementar SHCreateStreamOnFileW desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreateMemStream(param_60164: const, param_2971: UINT) -> *mut WINSHLWAPI struct IStream {
    // TODO: implementar SHCreateMemStream desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreateStreamOnFileEx(param_25711: LPCWSTR, param_54075: u32, param_54075: u32, param_16716: i32, param_50104: struct, param_50104: struct) -> WINSHLWAPI HRESULT {
    // TODO: implementar SHCreateStreamOnFileEx desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCreateStreamWrapper(param_15972: LPBYTE, param_54075: u32, param_54075: u32, param_50104: struct) -> WINSHLWAPI HRESULT {
    // TODO: implementar SHCreateStreamWrapper desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHLockShared(handle: *mut core::ffi::c_void, pid: u32) -> *mut WINSHLWAPI void {
    // TODO: implementar SHLockShared desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHUnlockShared(data: *mut core::ffi::c_void) -> WINSHLWAPI BOOL {
    // TODO: implementar SHUnlockShared desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTextServices(param_46438: *mut IUnknown, param_61445: *mut ITextHost, param_57333: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar CreateTextServices desde wine/textserv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ReleaseMutexWhenCallbackReturns(param_42707: PTP__INSTANCE, param_31864: *mut core::ffi::c_void) -> WINBASEAPI void {
    // TODO: implementar ReleaseMutexWhenCallbackReturns desde wine/threadpoolapiset.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateToolhelp32Snapshot(param_54075: u32, param_54075: u32) -> *mut core::ffi::c_void {
    // TODO: implementar CreateToolhelp32Snapshot desde wine/tlhelp32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateEnvironmentBlock(param_34430: *mut LPVOID, param_31864: *mut core::ffi::c_void, param_16716: i32) -> USERENVAPI BOOL {
    // TODO: implementar CreateEnvironmentBlock desde wine/userenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DestroyEnvironmentBlock(param_29262: LPVOID) -> USERENVAPI BOOL {
    // TODO: implementar DestroyEnvironmentBlock desde wine/userenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AVIStreamCreate(param_14399: *mut PAVISTREAM, param_9910: LONG, param_9910: LONG, param_18947: *mut CLSID) -> i32 {
    // TODO: implementar AVIStreamCreate desde wine/vfw.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateEditableStream(ppEditable: *mut PAVISTREAM, pSource: PAVISTREAM) -> i32 {
    // TODO: implementar CreateEditableStream desde wine/vfw.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AVIFileCreateStreamA(pfile: PAVIFILE, ppavi: *mut PAVISTREAM, psi: *mut AVISTREAMINFOA) -> i32 {
    // TODO: implementar AVIFileCreateStreamA desde wine/vfw.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AVIFileCreateStreamW(pfile: PAVIFILE, ppavi: *mut PAVISTREAM, psi: *mut AVISTREAMINFOW) -> i32 {
    // TODO: implementar AVIFileCreateStreamW desde wine/vfw.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MCIWndCreateA(param_11550: *mut core::ffi::c_void, param_49462: HINSTANCE, param_54075: u32, param_15619: LPCSTR) -> HWND VFWAPIV {
    // TODO: implementar MCIWndCreateA desde wine/vfw.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MCIWndCreateW(param_11550: *mut core::ffi::c_void, param_49462: HINSTANCE, param_54075: u32, param_25711: LPCWSTR) -> HWND VFWAPIV {
    // TODO: implementar MCIWndCreateW desde wine/vfw.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn capCreateCaptureWindowA(param_15619: LPCSTR, param_54075: u32, param_18538: INT, param_18538: INT, param_18538: INT, param_18538: INT, param_11550: *mut core::ffi::c_void, param_18538: INT) -> HWND VFWAPI {
    // TODO: implementar capCreateCaptureWindowA desde wine/vfw.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn capCreateCaptureWindowW(param_25711: LPCWSTR, param_54075: u32, param_18538: INT, param_18538: INT, param_18538: INT, param_18538: INT, param_11550: *mut core::ffi::c_void, param_18538: INT) -> HWND VFWAPI {
    // TODO: implementar capCreateCaptureWindowW desde wine/vfw.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WsCreateChannel(param_53392: WS_CHANNEL_TYPE, param_7973: WS_CHANNEL_BINDING, param_60164: const, param_30140: ULONG, param_60164: const, param_59782: *mut core::ffi::c_void, param_1224: *mut WS_ERROR) -> i32 {
    // TODO: implementar WsCreateChannel desde wine/webservices.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WsCreateChannelForListener(param_31109: *mut WS_LISTENER, param_60164: const, param_30140: ULONG, param_59782: *mut core::ffi::c_void, param_1224: *mut WS_ERROR) -> i32 {
    // TODO: implementar WsCreateChannelForListener desde wine/webservices.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WsCreateError(param_60164: const, param_30140: ULONG, param_17770: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar WsCreateError desde wine/webservices.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WsCreateHeap(param_14373: SIZE_T, param_14373: SIZE_T, param_60164: const, param_30140: ULONG, param_13066: *mut core::ffi::c_void, param_1224: *mut WS_ERROR) -> i32 {
    // TODO: implementar WsCreateHeap desde wine/webservices.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WsCreateListener(param_53392: WS_CHANNEL_TYPE, param_7973: WS_CHANNEL_BINDING, param_60164: const, param_30140: ULONG, param_60164: const, param_10413: *mut core::ffi::c_void, param_1224: *mut WS_ERROR) -> i32 {
    // TODO: implementar WsCreateListener desde wine/webservices.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WsCreateMessage(param_29459: WS_ENVELOPE_VERSION, param_24708: WS_ADDRESSING_VERSION, param_60164: const, param_30140: ULONG, param_14254: *mut core::ffi::c_void, param_1224: *mut WS_ERROR) -> i32 {
    // TODO: implementar WsCreateMessage desde wine/webservices.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WsCreateMessageForChannel(param_12147: *mut WS_CHANNEL, param_60164: const, param_30140: ULONG, param_14254: *mut core::ffi::c_void, param_1224: *mut WS_ERROR) -> i32 {
    // TODO: implementar WsCreateMessageForChannel desde wine/webservices.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WsCreateServiceProxy(WS_CHANNEL_TYPE: const, WS_CHANNEL_BINDING: const, param_60164: const, param_60164: const, ULONG: const, param_60164: const, ULONG: const, param_54660: *mut core::ffi::c_void, param_1224: *mut WS_ERROR) -> i32 {
    // TODO: implementar WsCreateServiceProxy desde wine/webservices.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WsCreateServiceProxyFromTemplate(param_53392: WS_CHANNEL_TYPE, param_60164: const, ULONG: const, param_40298: WS_BINDING_TEMPLATE_TYPE, param_64866: *mut core::ffi::c_void, param_30140: ULONG, param_60164: const, param_30140: ULONG, param_54660: *mut core::ffi::c_void, param_1224: *mut WS_ERROR) -> i32 {
    // TODO: implementar WsCreateServiceProxyFromTemplate desde wine/webservices.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WsCreateXmlBuffer(param_54159: *mut WS_HEAP, param_60164: const, param_30140: ULONG, param_2794: *mut core::ffi::c_void, param_1224: *mut WS_ERROR) -> i32 {
    // TODO: implementar WsCreateXmlBuffer desde wine/webservices.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WebSocketCreateClientHandle(PWEB_SOCKET_PROPERTY: const, param_30140: ULONG, param_59188: *mut WEB_SOCKET_HANDLE) -> i32 {
    // TODO: implementar WebSocketCreateClientHandle desde wine/websocket.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WerRegisterMemoryBlock(block: *mut core::ffi::c_void, size: u32) -> i32 {
    // TODO: implementar WerRegisterMemoryBlock desde wine/werapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WerReportCreate(param_29658: PCWSTR, param_54653: WER_REPORT_TYPE, param_48709: PWER_REPORT_INFORMATION, param_41806: *mut HREPORT) -> i32 {
    // TODO: implementar WerReportCreate desde wine/werapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WerUnregisterMemoryBlock(block: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar WerUnregisterMemoryBlock desde wine/werapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateConsoleScreenBuffer(param_54075: u32, param_54075: u32, param_16669: LPSECURITY_ATTRIBUTES, param_54075: u32, param_29262: LPVOID) -> WINBASEAPI HANDLE {
    // TODO: implementar CreateConsoleScreenBuffer desde wine/wincon.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptCreateHash(param_42387: HCRYPTPROV, param_63405: ALG_ID, param_16953: HCRYPTKEY, param_54075: u32, param_40800: *mut HCRYPTHASH) -> WINADVAPI BOOL {
    // TODO: implementar CryptCreateHash desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptCreateAsyncHandle(dwFlags: u32, phAsync: PHCRYPTASYNC) -> WINCRYPT32API BOOL {
    // TODO: implementar CryptCreateAsyncHandle desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CertCreateCertificateChainEngine(pConfig: PCERT_CHAIN_ENGINE_CONFIG, phChainEngine: *mut HCERTCHAINENGINE) -> WINCRYPT32API BOOL {
    // TODO: implementar CertCreateCertificateChainEngine desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CertCreateContext(dwContextType: u32, dwEncodingType: u32, pbEncoded: *mut const BYTE, cbEncoded: u32, dwFlags: u32, pCreatePara: PCERT_CREATE_CONTEXT_PARA) -> *mut WINCRYPT32API const void {
    // TODO: implementar CertCreateContext desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CertCreateCertificateContext(dwCertEncodingType: u32, pbCertEncoded: *mut const BYTE, cbCertEncoded: u32) -> WINCRYPT32API PCCERT_CONTEXT {
    // TODO: implementar CertCreateCertificateContext desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CertCreateCRLContext(dwCertEncodingType: u32, pbCrlEncoded: *mut const BYTE, cbCrlEncoded: u32) -> WINCRYPT32API PCCRL_CONTEXT {
    // TODO: implementar CertCreateCRLContext desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CertCreateCTLContext(dwMsgAndCertEncodingType: u32, pbCtlEncoded: *mut const BYTE, cbCtlEncoded: u32) -> WINCRYPT32API PCCTL_CONTEXT {
    // TODO: implementar CertCreateCTLContext desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CertCreateSelfSignCertificate(hProv: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, pSubjectIssuerBlob: PCERT_NAME_BLOB, dwFlags: u32, pKeyProvInfo: PCRYPT_KEY_PROV_INFO, pSignatureAlgorithm: PCRYPT_ALGORITHM_IDENTIFIER, pStartTime: PSYSTEMTIME, pEndTime: PSYSTEMTIME, pExtensions: PCERT_EXTENSIONS) -> WINCRYPT32API PCCERT_CONTEXT {
    // TODO: implementar CertCreateSelfSignCertificate desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EvtCreateBookmark(param_25711: LPCWSTR) -> EVT_HANDLE {
    // TODO: implementar EvtCreateBookmark desde wine/winevt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EvtCreateRenderContext(param_54075: u32, param_35067: *mut LPCWSTR, param_54075: u32) -> EVT_HANDLE {
    // TODO: implementar EvtCreateRenderContext desde wine/winevt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WinHttpCreateProxyResolver(param_19620: HINTERNET, param_36410: *mut HINTERNET) -> WINHTTPAPI DWORD {
    // TODO: implementar WinHttpCreateProxyResolver desde wine/winhttp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WinHttpCreateUrl(param_54772: LPURL_COMPONENTS, param_54075: u32, param_46598: LPWSTR, param_17803: LPDWORD) -> WINHTTPAPI BOOL {
    // TODO: implementar WinHttpCreateUrl desde wine/winhttp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InternetCreateUrlA(param_21253: LPURL_COMPONENTSA, param_54075: u32, param_32262: LPSTR, param_17803: LPDWORD) -> BOOLAPI {
    // TODO: implementar InternetCreateUrlA desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InternetCreateUrlW(param_36902: LPURL_COMPONENTSW, param_54075: u32, param_46598: LPWSTR, param_17803: LPDWORD) -> BOOLAPI {
    // TODO: implementar InternetCreateUrlW desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InternetLockRequestFile(param_19620: HINTERNET, param_41017: *mut *mut core::ffi::c_void) -> BOOLAPI {
    // TODO: implementar InternetLockRequestFile desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InternetUnlockRequestFile(param_31864: *mut core::ffi::c_void) -> BOOLAPI {
    // TODO: implementar InternetUnlockRequestFile desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FtpCreateDirectoryA(param_19620: HINTERNET, param_15619: LPCSTR) -> BOOLAPI {
    // TODO: implementar FtpCreateDirectoryA desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FtpCreateDirectoryW(param_19620: HINTERNET, param_25711: LPCWSTR) -> BOOLAPI {
    // TODO: implementar FtpCreateDirectoryW desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GopherCreateLocatorA(param_15619: LPCSTR, param_50874: INTERNET_PORT, param_15619: LPCSTR, param_15619: LPCSTR, param_54075: u32, param_32262: LPSTR, param_17803: LPDWORD) -> BOOLAPI {
    // TODO: implementar GopherCreateLocatorA desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GopherCreateLocatorW(param_25711: LPCWSTR, param_50874: INTERNET_PORT, param_25711: LPCWSTR, param_25711: LPCWSTR, param_54075: u32, param_46598: LPWSTR, param_17803: LPDWORD) -> BOOLAPI {
    // TODO: implementar GopherCreateLocatorW desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateUrlCacheEntryA(param_15619: LPCSTR, param_54075: u32, param_15619: LPCSTR, param_32262: LPSTR, param_54075: u32) -> BOOLAPI {
    // TODO: implementar CreateUrlCacheEntryA desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateUrlCacheEntryW(param_25711: LPCWSTR, param_54075: u32, param_25711: LPCWSTR, param_46598: LPWSTR, param_54075: u32) -> BOOLAPI {
    // TODO: implementar CreateUrlCacheEntryW desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnlockUrlCacheEntryFileA(param_15619: LPCSTR, param_54075: u32) -> BOOLAPI {
    // TODO: implementar UnlockUrlCacheEntryFileA desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnlockUrlCacheEntryFileW(param_25711: LPCWSTR, param_54075: u32) -> BOOLAPI {
    // TODO: implementar UnlockUrlCacheEntryFileW desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnlockUrlCacheEntryStream(param_31864: *mut core::ffi::c_void, param_54075: u32) -> BOOLAPI {
    // TODO: implementar UnlockUrlCacheEntryStream desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateUrlCacheGroup(param_54075: u32, param_29262: LPVOID) -> INTERNETAPI GROUPID {
    // TODO: implementar CreateUrlCacheGroup desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateMD5SSOHash(param_31966: PWSTR, param_31966: PWSTR, param_31966: PWSTR, param_57672: PBYTE) -> BOOLAPI {
    // TODO: implementar CreateMD5SSOHash desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ldap_create_page_controlA(param_28148: PLDAP, param_30140: ULONG, param_50104: struct, param_14799: UCHAR, param_36909: *mut PLDAPControlA) -> ULONG CDECL {
    // TODO: implementar ldap_create_page_controlA desde wine/winldap.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ldap_create_page_controlW(param_28148: PLDAP, param_30140: ULONG, param_50104: struct, param_14799: UCHAR, param_39748: *mut PLDAPControlW) -> ULONG CDECL {
    // TODO: implementar ldap_create_page_controlW desde wine/winldap.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ldap_create_sort_controlA(param_28148: PLDAP, param_30581: *mut PLDAPSortKeyA, param_14799: UCHAR, param_36909: *mut PLDAPControlA) -> ULONG CDECL {
    // TODO: implementar ldap_create_sort_controlA desde wine/winldap.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ldap_create_sort_controlW(param_28148: PLDAP, param_39543: *mut PLDAPSortKeyW, param_14799: UCHAR, param_39748: *mut PLDAPControlW) -> ULONG CDECL {
    // TODO: implementar ldap_create_sort_controlW desde wine/winldap.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ldap_create_vlv_controlA(param_28148: PLDAP, param_32628: PLDAPVLVInfo, param_14799: UCHAR, param_36909: *mut PLDAPControlA) -> INT CDECL {
    // TODO: implementar ldap_create_vlv_controlA desde wine/winldap.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ldap_create_vlv_controlW(param_28148: PLDAP, param_32628: PLDAPVLVInfo, param_14799: UCHAR, param_39748: *mut PLDAPControlW) -> INT CDECL {
    // TODO: implementar ldap_create_vlv_controlW desde wine/winldap.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RtlInterlockedFlushSList(param_63832: PSLIST_HEADER) -> NTSYSAPI PSLIST_ENTRY {
    // TODO: implementar RtlInterlockedFlushSList desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RtlInterlockedPopEntrySList(param_63832: PSLIST_HEADER) -> NTSYSAPI PSLIST_ENTRY {
    // TODO: implementar RtlInterlockedPopEntrySList desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RtlInterlockedPushEntrySList(param_63832: PSLIST_HEADER, param_11168: PSLIST_ENTRY) -> NTSYSAPI PSLIST_ENTRY {
    // TODO: implementar RtlInterlockedPushEntrySList desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedAnd(param_45680: *mut long volatile, param_22384: i64) -> i64 {
    // TODO: implementar _InterlockedAnd desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedCompareExchange(param_22384: i64, param_22384: i64, param_22384: i64) -> i64 {
    // TODO: implementar _InterlockedCompareExchange desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedCompareExchange64(param_52146: i64, long: i64, long: i64) -> i64 {
    // TODO: implementar _InterlockedCompareExchange64 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedCompareExchangePointer(param_64866: *mut core::ffi::c_void, param_64866: *mut core::ffi::c_void, param_64866: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    // TODO: implementar _InterlockedCompareExchangePointer desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedDecrement(param_22384: i64) -> i64 {
    // TODO: implementar _InterlockedDecrement desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedDecrement16(param_56335: i16) -> i16 {
    // TODO: implementar _InterlockedDecrement16 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedExchange(param_22384: i64, param_22384: i64) -> i64 {
    // TODO: implementar _InterlockedExchange desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedExchangeAdd(param_22384: i64, param_22384: i64) -> i64 {
    // TODO: implementar _InterlockedExchangeAdd desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedExchangeAdd16(param_56335: i16, param_56335: i16) -> i16 {
    // TODO: implementar _InterlockedExchangeAdd16 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedExchangePointer(param_64866: *mut core::ffi::c_void, param_64866: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    // TODO: implementar _InterlockedExchangePointer desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedIncrement(param_22384: i64) -> i64 {
    // TODO: implementar _InterlockedIncrement desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedIncrement16(param_56335: i16) -> i16 {
    // TODO: implementar _InterlockedIncrement16 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedOr(param_45680: *mut long volatile, param_22384: i64) -> i64 {
    // TODO: implementar _InterlockedOr desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedXor(param_45680: *mut long volatile, param_22384: i64) -> i64 {
    // TODO: implementar _InterlockedXor desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedAnd64(param_36418: *mut __int64 volatile, param_22257: __int64) -> __int64 {
    // TODO: implementar _InterlockedAnd64 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedAnd64(dest: *mut __int64 volatile, val: __int64) -> static FORCEINLINE __int64 {
    // TODO: implementar InterlockedAnd64 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedExchangeAdd64(param_36418: *mut __int64 volatile, param_22257: __int64) -> __int64 {
    // TODO: implementar _InterlockedExchangeAdd64 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedExchangeAdd64(dest: *mut __int64 volatile, val: __int64) -> static FORCEINLINE __int64 {
    // TODO: implementar InterlockedExchangeAdd64 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedDecrement64(param_36418: *mut __int64 volatile) -> __int64 {
    // TODO: implementar _InterlockedDecrement64 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedDecrement64(dest: *mut __int64 volatile) -> static FORCEINLINE __int64 {
    // TODO: implementar InterlockedDecrement64 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedIncrement64(param_36418: *mut __int64 volatile) -> __int64 {
    // TODO: implementar _InterlockedIncrement64 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedIncrement64(dest: *mut __int64 volatile) -> static FORCEINLINE __int64 {
    // TODO: implementar InterlockedIncrement64 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedOr64(param_36418: *mut __int64 volatile, param_22257: __int64) -> __int64 {
    // TODO: implementar _InterlockedOr64 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedOr64(dest: *mut __int64 volatile, val: __int64) -> static FORCEINLINE __int64 {
    // TODO: implementar InterlockedOr64 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedXor64(param_36418: *mut __int64 volatile, param_22257: __int64) -> __int64 {
    // TODO: implementar _InterlockedXor64 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedXor64(dest: *mut __int64 volatile, val: __int64) -> static FORCEINLINE __int64 {
    // TODO: implementar InterlockedXor64 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedAdd(dest: *mut long volatile, val: i64) -> static FORCEINLINE long {
    // TODO: implementar InterlockedAdd desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedExchangeAdd(param_48826: dest, param_55674: val) -> return {
    // TODO: implementar InterlockedExchangeAdd desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedAdd64(dest: *mut __int64 volatile, val: __int64) -> static FORCEINLINE __int64 {
    // TODO: implementar InterlockedAdd64 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedAnd(dest: *mut LONG volatile, val: LONG) -> static FORCEINLINE LONG {
    // TODO: implementar InterlockedAnd desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedCompareExchange(dest: *mut LONG volatile, xchg: LONG, compare: LONG) -> static FORCEINLINE LONG {
    // TODO: implementar InterlockedCompareExchange desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedCompareExchangePointer(dest: *mut core::ffi::c_void, xchg: *mut core::ffi::c_void, compare: *mut core::ffi::c_void) -> *mut static FORCEINLINE void {
    // TODO: implementar InterlockedCompareExchangePointer desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedCompareExchange64(dest: *mut LONGLONG volatile, xchg: LONGLONG, compare: LONGLONG) -> static FORCEINLINE LONGLONG {
    // TODO: implementar InterlockedCompareExchange64 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedExchange(dest: *mut LONG volatile, val: LONG) -> static FORCEINLINE LONG {
    // TODO: implementar InterlockedExchange desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedExchangeAdd16(dest: *mut short volatile, incr: i16) -> static FORCEINLINE short {
    // TODO: implementar InterlockedExchangeAdd16 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedIncrement(dest: *mut LONG volatile) -> static FORCEINLINE LONG {
    // TODO: implementar InterlockedIncrement desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedIncrement16(dest: *mut short volatile) -> static FORCEINLINE short {
    // TODO: implementar InterlockedIncrement16 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedDecrement(dest: *mut LONG volatile) -> static FORCEINLINE LONG {
    // TODO: implementar InterlockedDecrement desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedDecrement16(dest: *mut short volatile) -> static FORCEINLINE short {
    // TODO: implementar InterlockedDecrement16 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedExchangePointer(dest: *mut core::ffi::c_void, val: *mut core::ffi::c_void) -> *mut static FORCEINLINE void {
    // TODO: implementar InterlockedExchangePointer desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedOr(dest: *mut LONG volatile, val: LONG) -> static FORCEINLINE LONG {
    // TODO: implementar InterlockedOr desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedXor(dest: *mut LONG volatile, val: LONG) -> static FORCEINLINE LONG {
    // TODO: implementar InterlockedXor desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _InterlockedCompareExchange128(param_40545: *mut volatile __int64, param_22257: __int64, param_22257: __int64, param_54384: *mut __int64) -> u8 {
    // TODO: implementar _InterlockedCompareExchange128 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InterlockedCompareExchange128(dest: *mut volatile __int64, xchg_high: __int64, xchg_low: __int64, compare: *mut __int64) -> static FORCEINLINE unsigned char {
    // TODO: implementar InterlockedCompareExchange128 desde wine/winnt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cancel_blocking(process: *mut struct process, thread: *mut struct thread, iosb: client_ptr_t) -> static int {
    // TODO: implementar cancel_blocking desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_inode(param_20930: dev, param_3105: ino) -> return {
    // TODO: implementar create_inode desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_anonymous_fd(param_45317: &inotify_fd_ops, param_3540: unix_fd, param_50043: NULL, param_6097: 0) -> return {
    // TODO: implementar create_anonymous_fd desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_console_connection(param_32511: console) -> return {
    // TODO: implementar create_console_connection desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_screen_buffer(param_7102: current->process->console) -> return {
    // TODO: implementar create_screen_buffer desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_console_server() -> return {
    // TODO: implementar create_console_server desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_named_object(param_61370: root, param_41003: &console_device_ops, param_340: name, param_41624: attr, param_35035: sd) -> return {
    // TODO: implementar create_named_object desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3dkmt_mutex_dump(obj: *mut struct object, verbose: i32) -> static void {
    // TODO: implementar d3dkmt_mutex_dump desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3dkmt_mutex_destroy(obj: *mut struct object) -> static void {
    // TODO: implementar d3dkmt_mutex_destroy desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mutex_grab(mutex: *mut struct d3dkmt_mutex) -> static void {
    // TODO: implementar mutex_grab desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mutex_release(mutex: *mut struct d3dkmt_mutex, abandon: bool) -> static void {
    // TODO: implementar mutex_release desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn abandon_d3dkmt_mutexes(thread: *mut struct thread) -> core::ffi::c_void {
    // TODO: implementar abandon_d3dkmt_mutexes desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fill_create_process_event(event: *mut struct debug_event, arg: *mut const void) -> static void {
    // TODO: implementar fill_create_process_event desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_session(id: u32) -> static void {
    // TODO: implementar create_session desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn file_lock_dump(obj: *mut struct object, verbose: i32) -> static void {
    // TODO: implementar file_lock_dump desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn file_lock_destroy(obj: *mut struct object) -> static void {
    // TODO: implementar file_lock_destroy desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn set_unix_lock(fd: *mut struct fd, start: file_pos_t, end: file_pos_t, type: i32) -> static int {
    // TODO: implementar set_unix_lock desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lock_overlaps(lock: *mut struct file_lock, start: file_pos_t, end: file_pos_t) -> static inline int {
    // TODO: implementar lock_overlaps desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn remove_unix_locks(fd: *mut struct fd, start: file_pos_t, end: file_pos_t) -> static void {
    // TODO: implementar remove_unix_locks desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn remove_lock(lock: *mut struct file_lock, remove_unix: i32) -> static void {
    // TODO: implementar remove_lock desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn remove_process_locks(process: *mut struct process) -> core::ffi::c_void {
    // TODO: implementar remove_process_locks desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn remove_fd_locks(fd: *mut struct fd) -> static void {
    // TODO: implementar remove_fd_locks desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lock_fd(fd: *mut struct fd, start: file_pos_t, count: file_pos_t, shared: i32, wait: i32) -> obj_handle_t {
    // TODO: implementar lock_fd desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unlock_fd(fd: *mut struct fd, start: file_pos_t, count: file_pos_t) -> core::ffi::c_void {
    // TODO: implementar unlock_fd desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn abandon_inproc_mutexes(tid: thread_id_t) -> core::ffi::c_void {
    // TODO: implementar abandon_inproc_mutexes desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __pthread_kill(threadid: pthread_t, signo: i32) -> extern int {
    // TODO: implementar __pthread_kill desde glibc/pthread.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_temp_file(size: file_pos_t) -> static int {
    // TODO: implementar create_temp_file desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_file_for_fd_obj(param_16902: view->fd, param_49089: access, param_41213: sharing) -> return {
    // TODO: implementar create_file_for_fd_obj desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_mapping(param_61370: root, param_340: name, param_41624: attr, param_44127: size, param_58380: SEC_COMMIT, param_6097: 0, param_49089: access, param_35035: sd) -> return {
    // TODO: implementar create_mapping desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mutex_sync_dump(obj: *mut struct object, verbose: i32) -> static void {
    // TODO: implementar mutex_sync_dump desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mutex_sync_signaled(obj: *mut struct object, entry: *mut struct wait_queue_entry) -> static int {
    // TODO: implementar mutex_sync_signaled desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mutex_sync_satisfied(obj: *mut struct object, entry: *mut struct wait_queue_entry) -> static void {
    // TODO: implementar mutex_sync_satisfied desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mutex_sync_destroy(obj: *mut struct object) -> static void {
    // TODO: implementar mutex_sync_destroy desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mutex_dump(obj: *mut struct object, verbose: i32) -> static void {
    // TODO: implementar mutex_dump desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mutex_signal(obj: *mut struct object, access: u32, signal: i32) -> static int {
    // TODO: implementar mutex_signal desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mutex_destroy(obj: *mut struct object) -> static void {
    // TODO: implementar mutex_destroy desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn abandon_mutexes(thread: *mut struct thread) -> core::ffi::c_void {
    // TODO: implementar abandon_mutexes desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mark_block_noaccess(ptr: *mut core::ffi::c_void, size: usize) -> core::ffi::c_void {
    // TODO: implementar mark_block_noaccess desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mark_block_uninitialized(ptr: *mut core::ffi::c_void, size: usize) -> core::ffi::c_void {
    // TODO: implementar mark_block_uninitialized desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lock_input_keystate(input: *mut struct thread_input) -> static void {
    // TODO: implementar lock_input_keystate desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unlock_input_keystate(input: *mut struct thread_input) -> static void {
    // TODO: implementar unlock_input_keystate desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_key_recursive(param_14745: base, param_22277: &name, param_6097: 0) -> return {
    // TODO: implementar create_key_recursive desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_server_lock() -> static int {
    // TODO: implementar create_server_lock desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn wait_for_lock() -> i32 {
    // TODO: implementar wait_for_lock desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn kill_lock_owner(sig: i32) -> i32 {
    // TODO: implementar kill_lock_owner desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn acquire_lock() -> static void {
    // TODO: implementar acquire_lock desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_file_directories(make: *mut const struct makefile, files: struct strarray) -> static void {
    // TODO: implementar create_file_directories desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_image_list(resource: UINT, width: UINT, height: UINT, count: UINT, mask_color: COLORREF) -> static HIMAGELIST {
    // TODO: implementar create_image_list desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ADsDNWithBinary_create(riid: REFIID, param_64866: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar ADsDNWithBinary_create desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Pathname_create(riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT {
    // TODO: implementar Pathname_create desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn factory_LockServer(iface: *mut IClassFactory, lock: i32) -> static HRESULT {
    // TODO: implementar factory_LockServer desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LDAPNamespace_create(riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT {
    // TODO: implementar LDAPNamespace_create desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LDAP_create(riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT {
    // TODO: implementar LDAP_create desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ADSystemInfo_create(riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT {
    // TODO: implementar ADSystemInfo_create desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dirobj_CreateDSObject(iface: *mut IDirectoryObject, name: LPWSTR, attrs: PADS_ATTR_INFO, count: u32, param_13120: *mut IDispatch) -> static HRESULT {
    // TODO: implementar dirobj_CreateDSObject desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RegCreateKeyExW(param_38135: HKEY_CURRENT_USER, param_6097: 0, param_50043: NULL, param_65142: REG_OPTION_NON_VOLATILE, param_34215: *mut core::ffi::c_void) -> return {
    // TODO: implementar RegCreateKeyExW desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CRYPT_CreateMachineGuid() -> static void {
    // TODO: implementar CRYPT_CreateMachineGuid desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RegCreateKeyW(hkey: HKEY, lpSubKey: LPCWSTR, phkResult: PHKEY) -> LSTATUS {
    // TODO: implementar RegCreateKeyW desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RegCreateKeyA(hkey: HKEY, lpSubKey: LPCSTR, phkResult: PHKEY) -> LSTATUS {
    // TODO: implementar RegCreateKeyA desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RegCreateKeyExA(param_3024: hkey, param_23537: lpSubKey, param_6097: 0, param_50043: NULL, param_65142: REG_OPTION_NON_VOLATILE, param_6600: MAXIMUM_ALLOWED, param_50043: NULL, param_26509: phkResult, param_50043: NULL) -> return {
    // TODO: implementar RegCreateKeyExA desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SaferCreateLevel(ScopeId: u32, LevelId: u32, OpenFlags: u32, LevelHandle: *mut SAFER_LEVEL_HANDLE, lpReserved: LPVOID) -> i32 {
    // TODO: implementar SaferCreateLevel desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LockServiceDatabase(manager: SC_HANDLE) -> SC_LOCK {
    // TODO: implementar LockServiceDatabase desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnlockServiceDatabase(lock: SC_LOCK) -> i32 {
    // TODO: implementar UnlockServiceDatabase desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn QueryServiceLockStatusA(hSCManager: SC_HANDLE, lpLockStatus: LPQUERY_SERVICE_LOCK_STATUSA, cbBufSize: u32, pcbBytesNeeded: LPDWORD) -> i32 {
    // TODO: implementar QueryServiceLockStatusA desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn QueryServiceLockStatusW(hSCManager: SC_HANDLE, lpLockStatus: LPQUERY_SERVICE_LOCK_STATUSW, cbBufSize: u32, pcbBytesNeeded: LPDWORD) -> i32 {
    // TODO: implementar QueryServiceLockStatusW desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WmiOpenBlock(guid: *mut GUID, access: ULONG, handle: *mut WMIHANDLE) -> ULONG {
    // TODO: implementar WmiOpenBlock desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_tmp_ini_file(hm: HMODULE, ini_file: *mut u16) -> static BOOL {
    // TODO: implementar create_tmp_ini_file desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AMAudioData_create(pUnkOuter: *mut IUnknown, ppObj: *mut LPVOID) -> i32 {
    // TODO: implementar AMAudioData_create desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audiostreamsample_create(parent: *mut struct audio_stream, audio_data: *mut IAudioData, param_14730: *mut IAudioStreamSample) -> static HRESULT {
    // TODO: implementar audiostreamsample_create desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_IAMMediaStream_CreateSharedSample(iface: *mut IAMMediaStream, existing_sample: *mut IStreamSample, flags: u32, param_4884: *mut IStreamSample) -> static HRESULT {
    // TODO: implementar audio_IAMMediaStream_CreateSharedSample desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_IAudioMediaStream_CreateSharedSample(iface: *mut IAudioMediaStream, existing_sample: *mut IStreamSample, flags: u32, param_4884: *mut IStreamSample) -> static HRESULT {
    // TODO: implementar audio_IAudioMediaStream_CreateSharedSample desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IAMMediaStream_CreateSharedSample(param_40126: &stream->IAMMediaStream_iface, param_54879: existing_sample, param_51689: flags, param_13291: sample) -> return {
    // TODO: implementar IAMMediaStream_CreateSharedSample desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_IAudioMediaStream_CreateSample(iface: *mut IAudioMediaStream, audio_data: *mut IAudioData, flags: u32, param_14730: *mut IAudioStreamSample) -> static HRESULT {
    // TODO: implementar audio_IAudioMediaStream_CreateSample desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_meminput_ReceiveCanBlock(iface: *mut IMemInputPin) -> static HRESULT {
    // TODO: implementar audio_meminput_ReceiveCanBlock desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_stream_create(outer: *mut IUnknown, param_64866: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar audio_stream_create desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddrawstreamsample_create(parent: *mut struct ddraw_stream, surface: *mut IDirectDrawSurface, rect: *mut const RECT, param_17300: *mut IDirectDrawStreamSample) -> static HRESULT {
    // TODO: implementar ddrawstreamsample_create desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_IAMMediaStream_CreateSharedSample(iface: *mut IAMMediaStream, existing_sample: *mut IStreamSample, flags: u32, param_4884: *mut IStreamSample) -> static HRESULT {
    // TODO: implementar ddraw_IAMMediaStream_CreateSharedSample desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_IDirectDrawMediaStream_CreateSharedSample(iface: *mut IDirectDrawMediaStream, existing_sample: *mut IStreamSample, flags: u32, param_4884: *mut IStreamSample) -> static HRESULT {
    // TODO: implementar ddraw_IDirectDrawMediaStream_CreateSharedSample desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_IDirectDrawMediaStream_CreateSample(iface: *mut IDirectDrawMediaStream, surface: *mut IDirectDrawSurface, rect: *mut const RECT, flags: u32, param_17300: *mut IDirectDrawStreamSample) -> static HRESULT {
    // TODO: implementar ddraw_IDirectDrawMediaStream_CreateSample desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_meminput_ReceiveCanBlock(iface: *mut IMemInputPin) -> static HRESULT {
    // TODO: implementar ddraw_meminput_ReceiveCanBlock desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_stream_create(outer: *mut IUnknown, param_64866: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar ddraw_stream_create desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn filter_create(outer: *mut IUnknown, param_64866: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar filter_create desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AMCF_LockServer(iface: *mut IClassFactory, dolock: i32) -> static HRESULT {
    // TODO: implementar AMCF_LockServer desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_graph(mmstream: *mut struct multimedia_stream, graph: *mut IGraphBuilder) -> static HRESULT {
    // TODO: implementar create_graph desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn multimedia_stream_create(outer: *mut IUnknown, param_64866: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar multimedia_stream_create desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SdbCreateDatabase(path: LPCWSTR, type: PATH_TYPE) -> PDB {
    // TODO: implementar SdbCreateDatabase desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn created(param_36496: %lu) -> s does not exist and could not be {
    // TODO: implementar created desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RegistrarCF_LockServer(iface: *mut IClassFactory, lock: i32) -> static HRESULT {
    // TODO: implementar RegistrarCF_LockServer desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleContainer_LockContainer(iface: *mut IOleContainer, fLock: i32) -> static HRESULT {
    // TODO: implementar OleContainer_LockContainer desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OleControlSite_LockInPlaceActive(This: *mut IOleControlSite, fLock: i32) -> static HRESULT {
    // TODO: implementar OleControlSite_LockInPlaceActive desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IOCS_Create(hWnd: *mut core::ffi::c_void, pUnkControl: *mut IUnknown, param_46438: *mut IUnknown) -> static HRESULT {
    // TODO: implementar IOCS_Create desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlAxCreateControlLicEx(lpszName: LPCOLESTR, hWnd: *mut core::ffi::c_void, pStream: *mut IStream, param_46438: *mut IUnknown, param_46438: *mut IUnknown, iidSink: REFIID, punkSink: *mut IUnknown, lic: BSTR) -> i32 {
    // TODO: implementar AtlAxCreateControlLicEx desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PUT_BLOCK(param_19644: tmp, param_47631: src-tmp) -> else {
    // TODO: implementar PUT_BLOCK desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlAxCreateControlLic(lpTricsData: *mut const WCHAR, hwnd: *mut core::ffi::c_void, stream: *mut IStream, param_46438: *mut IUnknown, lic: BSTR) -> i32 {
    // TODO: implementar AtlAxCreateControlLic desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create(param_1124: open) -> Could not {
    // TODO: implementar create desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateWindowW(param_11787: class_name, param_50393: window_name, param_6124: style, param_52739: x, param_49451: y, param_57551: width, param_41294: height, param_52606: parent, param_50043: NULL, param_13450: avicap_instance, param_50043: NULL) -> return {
    // TODO: implementar CreateWindowW desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ACMStream_fnCreate(iface: *mut IAVIStream, lParam1: LPARAM, lParam2: LPARAM) -> static HRESULT {
    // TODO: implementar ACMStream_fnCreate desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AVIFILE_CreateACMStream(riid: REFIID, ppv: *mut LPVOID) -> i32 {
    // TODO: implementar AVIFILE_CreateACMStream desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IAVIFile_CreateStream(param_24701: pfile, param_41793: ppavi, param_60342: &psiw) -> return {
    // TODO: implementar IAVIFile_CreateStream desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AVIFILE_SamplesToBlock(This: *mut const IAVIStreamImpl, pos: LPLONG, offset: LPLONG) -> static void {
    // TODO: implementar AVIFILE_SamplesToBlock desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IAVIFile_fnCreateStream(iface: *mut IAVIFile, param_44472: *mut IAVIStream, asi: *mut AVISTREAMINFOW) -> static HRESULT {
    // TODO: implementar IAVIFile_fnCreateStream desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AVIFILE_CreateAVIFile(pUnkOuter: *mut IUnknown, riid: REFIID, param_64866: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar AVIFILE_CreateAVIFile desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IAVIStream_fnCreate(iface: *mut IAVIStream, lParam1: LPARAM, lParam2: LPARAM) -> static HRESULT {
    // TODO: implementar IAVIStream_fnCreate desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IEditAVIStream_fnCreate(param_24703: IAVIStream*iface, lParam1: LPARAM, lParam2: LPARAM) -> static HRESULT {
    // TODO: implementar IEditAVIStream_fnCreate desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AVIFILE_CreateWAVFile(param_35847: pOuter, param_63178: riid, param_40564: ppobj) -> return {
    // TODO: implementar AVIFILE_CreateWAVFile desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AVIFILE_CreateICMStream(param_63178: riid, param_40564: ppobj) -> return {
    // TODO: implementar AVIFILE_CreateICMStream desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IClassFactory_fnLockServer(iface: *mut IClassFactory, dolock: i32) -> static HRESULT {
    // TODO: implementar IClassFactory_fnLockServer desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AVIFILE_CreateClassFactory(clsid: *mut const CLSID, riid: *mut const IID, param_64866: *mut core::ffi::c_void) -> static HRESULT {
    // TODO: implementar AVIFILE_CreateClassFactory desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Create(hParent: _In_ HWND) -> bool {
    // TODO: implementar Create desde reactos/GridView.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OnCreate(hwnd: _In_ HWND, hParent: _In_ HWND) -> LRESULT {
    // TODO: implementar OnCreate desde reactos/GridView.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateStatusBar() -> bool {
    // TODO: implementar CreateStatusBar desde reactos/MainWindow.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateFontComboBox() -> bool {
    // TODO: implementar CreateFontComboBox desde reactos/MainWindow.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateImeMenu(hIMC: _In_ HIMC, lpImeParentMenu: _Inout_opt_ PIMEMENUITEMINFO, bRightMenu: _In_ BOOL) -> PIMEMENUNODE {
    // TODO: implementar CreateImeMenu desde reactos/imemenu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateConsoleWindow(OPTIONAL: IN LPCTSTR lpFileName, nCmdShow: i32) -> *mut core::ffi::c_void {
    // TODO: implementar CreateConsoleWindow desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDIBWithProperties(width: i32, height: i32) -> HBITMAP {
    // TODO: implementar CreateDIBWithProperties desde reactos/dib.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateMonoBitmap(width: i32, height: i32, bWhite: i32) -> HBITMAP {
    // TODO: implementar CreateMonoBitmap desde reactos/dib.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateColorDIB(width: i32, height: i32, rgb: COLORREF) -> HBITMAP {
    // TODO: implementar CreateColorDIB desde reactos/dib.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DoCreate() -> *mut core::ffi::c_void {
    // TODO: implementar DoCreate desde reactos/fullscreen.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LockBitmap() -> HBITMAP {
    // TODO: implementar LockBitmap desde reactos/history.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnlockBitmap(hbmLocked: HBITMAP) -> core::ffi::c_void {
    // TODO: implementar UnlockBitmap desde reactos/history.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createToolObject(type: TOOLTYPE) -> *mut static ToolBase {
    // TODO: implementar createToolObject desde reactos/toolsmodel.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn disk_create_notify(handle: RD_NTHANDLE, info_class: uint32) -> RD_NTSTATUS {
    // TODO: implementar disk_create_notify desde reactos/proto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rd_create_ui() -> core::ffi::c_void {
    // TODO: implementar rd_create_ui desde reactos/proto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rd_lock_file(fd: i32, start: i32, len: i32) -> RD_BOOL {
    // TODO: implementar rd_lock_file desde reactos/proto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ui_get_numlock_state(state: u32) -> uint16 {
    // TODO: implementar ui_get_numlock_state desde reactos/proto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ui_create_window() -> RD_BOOL {
    // TODO: implementar ui_create_window desde reactos/proto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ui_create_bitmap(width: i32, height: i32, data: *mut uint8) -> RD_HBITMAP {
    // TODO: implementar ui_create_bitmap desde reactos/proto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ui_create_glyph(width: i32, height: i32, data: *mut uint8) -> RD_HGLYPH {
    // TODO: implementar ui_create_glyph desde reactos/proto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ui_create_cursor(x: u32, y: u32, width: i32, height: i32, andmask: *mut uint8, xormask: *mut uint8, bpp: i32) -> RD_HCURSOR {
    // TODO: implementar ui_create_cursor desde reactos/proto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ui_create_colourmap(colours: *mut COLOURMAP) -> RD_HCOLOURMAP {
    // TODO: implementar ui_create_colourmap desde reactos/proto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ui_seamless_create_window(id: u64, group: u64, parent: u64, flags: u64) -> core::ffi::c_void {
    // TODO: implementar ui_seamless_create_window desde reactos/proto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn scard_lock(lock: i32) -> core::ffi::c_void {
    // TODO: implementar scard_lock desde reactos/proto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn scard_unlock(lock: i32) -> core::ffi::c_void {
    // TODO: implementar scard_unlock desde reactos/proto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DoCreateEditWindow(param_9080: VOID) -> VOID {
    // TODO: implementar DoCreateEditWindow desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OSK_Create(hwnd: *mut core::ffi::c_void) -> LRESULT {
    // TODO: implementar OSK_Create desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateListView(hwndParent: *mut core::ffi::c_void, id: HMENU, cx: INT) -> *mut core::ffi::c_void {
    // TODO: implementar CreateListView desde reactos/main.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTreeView(hwndParent: *mut core::ffi::c_void, pHostName: LPWSTR, id: HMENU) -> *mut core::ffi::c_void {
    // TODO: implementar CreateTreeView desde reactos/main.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateNewKey(hwndTV: *mut core::ffi::c_void, hItem: HTREEITEM) -> i32 {
    // TODO: implementar CreateNewKey desde reactos/main.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ParseCreateConfigArguments(ServiceArgs: *mut LPCTSTR, ArgCount: INT, bChangeService: i32, lpServiceInfo: OUT LPSERVICE_CREATE_INFO) -> i32 {
    // TODO: implementar ParseCreateConfigArguments desde reactos/sc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateUsage(param_9080: VOID) -> VOID {
    // TODO: implementar CreateUsage desde reactos/sc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SndMixerCreate(hWndNotification: *mut core::ffi::c_void, MixerId: UINT) -> PSND_MIXER {
    // TODO: implementar SndMixerCreate desde reactos/sndvol32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GraphCtrl_Create(inst: PTM_GRAPH_CONTROL, hWnd: *mut core::ffi::c_void, hParentWnd: *mut core::ffi::c_void, fmt: PTM_FORMAT) -> i32 {
    // TODO: implementar GraphCtrl_Create desde reactos/graphctl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ShutDown_LockComputer(param_9080: VOID) -> VOID {
    // TODO: implementar ShutDown_LockComputer desde reactos/shutdown.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MACRO_CreateButton(param_15619: LPCSTR, param_15619: LPCSTR, param_15619: LPCSTR) -> core::ffi::c_void {
    // TODO: implementar MACRO_CreateButton desde reactos/macro.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WINHELP_CreateHelpWindow(param_58250: *mut WINHELP_WNDPAGE, param_59621: i32, param_16716: i32) -> i32 {
    // TODO: implementar WINHELP_CreateHelpWindow desde reactos/winhelp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WINHELP_CreateIndexWindow(param_16716: i32) -> i32 {
    // TODO: implementar WINHELP_CreateIndexWindow desde reactos/winhelp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSol() -> core::ffi::c_void {
    // TODO: implementar CreateSol desde reactos/solitaire.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSpider() -> core::ffi::c_void {
    // TODO: implementar CreateSpider desde reactos/spider.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateEventDetailsCtrl(hInstance: HINSTANCE, hParentWnd: *mut core::ffi::c_void, lParam: LPARAM) -> *mut core::ffi::c_void {
    // TODO: implementar CreateEventDetailsCtrl desde reactos/evtdetctl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateMainWindow(lpCaption: LPCTSTR, nCmdShow: i32) -> *mut core::ffi::c_void {
    // TODO: implementar CreateMainWindow desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateRootContext(param_9080: VOID) -> i32 {
    // TODO: implementar CreateRootContext desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateRootHelper(param_9080: VOID) -> u32 {
    // TODO: implementar CreateRootHelper desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn get_lock_linewrap() -> bool {
    // TODO: implementar get_lock_linewrap desde reactos/tnconfig.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateHiddenConsoleProcess(szChildName: LPCTSTR, ppi: *mut PROCESS_INFORMATION, phInWrite: LPHANDLE, phOutRead: LPHANDLE, phErrRead: LPHANDLE) -> i32 {
    // TODO: implementar CreateHiddenConsoleProcess desde reactos/tnmisc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateInstalledAppByRegistryKey(KeyName: LPCWSTR, hKeyParent: HKEY, KeyIndex: UINT) -> *mut static CInstalledApplicationInfo {
    // TODO: implementar CreateInstalledAppByRegistryKey desde reactos/appdb.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSearchBar() -> i32 {
    // TODO: implementar CreateSearchBar desde reactos/appview.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateComboBox() -> i32 {
    // TODO: implementar CreateComboBox desde reactos/appview.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateHSplitter() -> i32 {
    // TODO: implementar CreateHSplitter desde reactos/appview.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateAppInfoDisplay() -> i32 {
    // TODO: implementar CreateAppInfoDisplay desde reactos/appview.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSettingsDlg(hwnd: *mut core::ffi::c_void) -> VOID {
    // TODO: implementar CreateSettingsDlg desde reactos/dialogs.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateApplicationView() -> i32 {
    // TODO: implementar CreateApplicationView desde reactos/gui.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVSplitter() -> i32 {
    // TODO: implementar CreateVSplitter desde reactos/gui.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateLayout() -> i32 {
    // TODO: implementar CreateLayout desde reactos/gui.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDirectoryTree(Dir: LPCWSTR) -> UINT {
    // TODO: implementar CreateDirectoryTree desde reactos/misc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCaret(hWnd: *mut core::ffi::c_void, size: SIZE) -> core::ffi::c_void {
    // TODO: implementar CreateCaret desde reactos/ciccaret.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unlock() -> core::ffi::c_void {
    // TODO: implementar unlock desde reactos/cicimc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _LockIMCC(hIMCC: HIMCC, param_31776: *mut T_DATA) -> i32 {
    // TODO: implementar _LockIMCC desde reactos/cicimc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _UnlockIMCC(hIMCC: HIMCC) -> i32 {
    // TODO: implementar _UnlockIMCC desde reactos/cicimc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _LockIMC(hIMC: HIMC, ppIC: *mut LPINPUTCONTEXTDX) -> i32 {
    // TODO: implementar _LockIMC desde reactos/cicimc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _UnlockIMC(hIMC: HIMC) -> i32 {
    // TODO: implementar _UnlockIMC desde reactos/cicimc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _cicRegKey_Create(self: CicRegKey&, hKey: HKEY, lpSubKey: LPCTSTR) -> EXTERN_C LSTATUS {
    // TODO: implementar _cicRegKey_Create desde reactos/cicreg.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cicCreateDitherBrush(param_9080: VOID) -> HBRUSH {
    // TODO: implementar cicCreateDitherBrush desde reactos/cicuif.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cicCreateDisabledBitmap(prc: LPCRECT, hbmMask: HBITMAP, hbr1: HBRUSH, hbr2: HBRUSH, bPressed: i32) -> HBITMAP {
    // TODO: implementar cicCreateDisabledBitmap desde reactos/cicuif.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cicCreateShadowMaskBmp(prc: LPRECT, hbm1: HBITMAP, hbm2: HBITMAP, hbr1: HBRUSH, hbr2: HBRUSH) -> HBITMAP {
    // TODO: implementar cicCreateShadowMaskBmp desde reactos/cicuif.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cicCreateMaskBmp(prc: LPCRECT, hbm1: HBITMAP, hbm2: HBITMAP, hbr: HBRUSH, rgbColor: COLORREF, rgbBack: COLORREF) -> HBITMAP {
    // TODO: implementar cicCreateMaskBmp desde reactos/cicuif.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateScheme() -> core::ffi::c_void {
    // TODO: implementar CreateScheme desde reactos/cicuif.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateRegion(prc: LPCRECT) -> HRGN {
    // TODO: implementar CreateRegion desde reactos/cicuif.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateWnd() -> *mut core::ffi::c_void {
    // TODO: implementar CreateWnd desde reactos/CLoaderWnd.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateContext(tidOwner: TfClientId, dwFlags: u32, punk: *mut IUnknown, param_49624: *mut ITfContext, pecTextStore: *mut TfEditCookie) -> STDMETHODIMP {
    // TODO: implementar CreateContext desde reactos/documentmgr.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InatCreateIcon(LangID: _In_ LANGID) -> HICON {
    // TODO: implementar InatCreateIcon desde reactos/mlng.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InatCreateIconBySize(LangID: _In_ LANGID, nWidth: _In_ INT, nHeight: _In_ INT, plf: *mut _In_ const LOGFONTW) -> HICON {
    // TODO: implementar InatCreateIconBySize desde reactos/mlng.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EnumCreateInputContextCallback(hIMC: HIMC, lParam: LPARAM) -> static BOOL {
    // TODO: implementar EnumCreateInputContextCallback desde reactos/bridge.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateInputContext(pTLS: *mut TLS, hIMC: HIMC) -> i32 {
    // TODO: implementar CreateInputContext desde reactos/bridge.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDefFrameWnd(hwndParent: *mut core::ffi::c_void, hIMC: HIMC) -> i32 {
    // TODO: implementar CreateDefFrameWnd desde reactos/ui.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCompButtonWnd(hwndParent: *mut core::ffi::c_void, hIMC: HIMC) -> i32 {
    // TODO: implementar CreateCompButtonWnd desde reactos/ui.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCompositionWindow(imcLock: CicIMCLock&, hwndParent: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar CreateCompositionWindow desde reactos/ui.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _Create() -> i32 {
    // TODO: implementar _Create desde reactos/ui.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDeviceDescriptor(path: *mut u16, is_enabled: i32) -> *mut VOID {
    // TODO: implementar CreateDeviceDescriptor desde reactos/audiosrv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateAudioDeviceList(max_size: u32) -> i32 {
    // TODO: implementar CreateAudioDeviceList desde reactos/audiosrv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn print_create_attributes(level: i32, create_opts: u32) -> core::ffi::c_void {
    // TODO: implementar print_create_attributes desde reactos/daemon_debug.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_idmap_create(param_57573: *mut nfs41_idmapper) -> i32 {
    // TODO: implementar nfs41_idmap_create desde reactos/idmap.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_name_cache_create(param_14265: *mut OUT struct nfs41_name_cache) -> i32 {
    // TODO: implementar nfs41_name_cache_create desde reactos/name_cache.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_root_create(name: *mut IN const char, sec_flavor: IN uint32_t, wsize: IN uint32_t, rsize: IN uint32_t, param_41962: *mut OUT nfs41_root) -> i32 {
    // TODO: implementar nfs41_root_create desde reactos/nfs41.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_session_create(client: *mut IN nfs41_client, param_29131: *mut IN nfs41_session) -> i32 {
    // TODO: implementar nfs41_session_create desde reactos/nfs41.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_server_find_or_create(server_owner_major_id: *mut IN const char, server_scope: *mut IN const char, addr: *mut IN const netaddr4, param_43984: *mut OUT nfs41_server) -> i32 {
    // TODO: implementar nfs41_server_find_or_create desde reactos/nfs41.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_client_create(rpc: *mut IN nfs41_rpc_clnt, owner: *mut IN const client_owner4, is_data: IN bool_t, exchangeid: *mut IN const struct __nfs41_exchange_id_res, param_62372: *mut OUT nfs41_client) -> i32 {
    // TODO: implementar nfs41_client_create desde reactos/nfs41.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_for_fh(session: *mut IN nfs41_session, fsid: *mut IN const nfs41_fsid, OPTIONAL: IN const nfs41_fh *parent, file: *mut OUT nfs41_path_fh) -> i32 {
    // TODO: implementar nfs41_superblock_for_fh desde reactos/nfs41.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_getattr_mask(superblock: *mut IN const nfs41_superblock, attrs: *mut OUT bitmap4) -> static __inline void {
    // TODO: implementar nfs41_superblock_getattr_mask desde reactos/nfs41.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_supported_attrs(superblock: *mut IN const nfs41_superblock, attrs: *mut IN OUT bitmap4) -> static __inline void {
    // TODO: implementar nfs41_superblock_supported_attrs desde reactos/nfs41.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_supported_attrs_exclcreat(superblock: *mut IN const nfs41_superblock, attrs: *mut IN OUT bitmap4) -> static __inline void {
    // TODO: implementar nfs41_superblock_supported_attrs_exclcreat desde reactos/nfs41.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_fs_attributes(superblock: *mut IN const nfs41_superblock, FsAttrs: *mut OUT struct _FILE_FS_ATTRIBUTE_INFORMATION) -> core::ffi::c_void {
    // TODO: implementar nfs41_superblock_fs_attributes desde reactos/nfs41.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_space_changed(superblock: *mut IN nfs41_superblock) -> core::ffi::c_void {
    // TODO: implementar nfs41_superblock_space_changed desde reactos/nfs41.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_list_init(superblocks: *mut IN nfs41_superblock_list) -> core::ffi::c_void {
    // TODO: implementar nfs41_superblock_list_init desde reactos/nfs41.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_rpc_clnt_create(addrs: *mut IN const multi_addr4, wsize: IN uint32_t, rsize: IN uint32_t, uid: IN uint32_t, gid: IN uint32_t, sec_flavor: IN uint32_t, param_60754: *mut OUT nfs41_rpc_clnt) -> i32 {
    // TODO: implementar nfs41_rpc_clnt_create desde reactos/nfs41.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_create_session(clnt: *mut IN nfs41_client, session: *mut OUT nfs41_session, try_recovery: IN bool_t) -> i32 {
    // TODO: implementar nfs41_create_session desde reactos/nfs41_ops.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_create(session: *mut IN nfs41_session, type: IN uint32_t, createattrs: *mut IN nfs41_file_info, symlink: *mut IN OPTIONAL const char, parent: *mut IN nfs41_path_fh, file: *mut OUT nfs41_path_fh, info: *mut OUT nfs41_file_info) -> i32 {
    // TODO: implementar nfs41_create desde reactos/nfs41_ops.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_lock(session: *mut IN nfs41_session, file: *mut IN nfs41_path_fh, owner: *mut IN state_owner4, type: IN uint32_t, offset: IN uint64_t, length: IN uint64_t, reclaim: IN bool_t, try_recovery: IN bool_t, stateid: *mut IN OUT stateid_arg) -> i32 {
    // TODO: implementar nfs41_lock desde reactos/nfs41_ops.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_unlock(session: *mut IN nfs41_session, file: *mut IN nfs41_path_fh, offset: IN uint64_t, length: IN uint64_t, stateid: *mut IN OUT stateid_arg) -> i32 {
    // TODO: implementar nfs41_unlock desde reactos/nfs41_ops.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_lock_stateid_copy(lock_state: *mut IN nfs41_lock_state, dest: *mut IN OUT stateid4) -> *mut stateid4 {
    // TODO: implementar nfs41_lock_stateid_copy desde reactos/nfs41_ops.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_getattr(session: *mut IN nfs41_session, file: *mut IN nfs41_path_fh, attr_request: *mut IN bitmap4, info: *mut OUT nfs41_file_info, supports_named_attrs: *mut OUT bool_t) -> i32 {
    // TODO: implementar nfs41_superblock_getattr desde reactos/nfs41_ops.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pnfs_layout_list_create(param_8016: *mut OUT struct pnfs_layout_list) -> enum pnfs_status {
    // TODO: implementar pnfs_layout_list_create desde reactos/pnfs.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pnfs_file_device_list_create(param_20224: *mut OUT struct pnfs_file_device_list) -> enum pnfs_status {
    // TODO: implementar pnfs_file_device_list_create desde reactos/pnfs.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_silly_rename(path: *mut IN nfs41_abs_path, fh: *mut IN const nfs41_fh, silly: *mut OUT nfs41_component) -> i32 {
    // TODO: implementar create_silly_rename desde reactos/util.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ScCreateWellKnownSids(param_9080: VOID) -> NTSTATUS NTAPI {
    // TODO: implementar ScCreateWellKnownSids desde reactos/svchost.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSocket() -> static void {
    // TODO: implementar CreateSocket desde reactos/telnetd.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ApiLock(param_9080: VOID) -> extern VOID {
    // TODO: implementar ApiLock desde reactos/rosdhcp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ApiUnlock(param_9080: VOID) -> extern VOID {
    // TODO: implementar ApiUnlock desde reactos/rosdhcp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateComputerTypeList(InfFile: IN HINF) -> PGENERIC_LIST {
    // TODO: implementar CreateComputerTypeList desde reactos/settings.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDisplayDriverList(InfFile: IN HINF) -> PGENERIC_LIST {
    // TODO: implementar CreateDisplayDriverList desde reactos/settings.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateKeyboardDriverList(InfFile: IN HINF) -> PGENERIC_LIST {
    // TODO: implementar CreateKeyboardDriverList desde reactos/settings.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateKeyboardLayoutList(InfFile: IN HINF, LanguageId: IN PCWSTR, DefaultKBLayout: OUT PWSTR) -> PGENERIC_LIST {
    // TODO: implementar CreateKeyboardLayoutList desde reactos/settings.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateLanguageList(InfFile: IN HINF, DefaultLanguage: OUT PWSTR) -> PGENERIC_LIST {
    // TODO: implementar CreateLanguageList desde reactos/settings.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FindVolCreateInTreeByVolume(hTreeList: _In_ HWND, Volume: _In_ PVOLENTRY) -> PVOL_CREATE_INFO {
    // TODO: implementar FindVolCreateInTreeByVolume desde reactos/reactos.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateListViewColumns(hInstance: IN HINSTANCE, hWndListView: IN HWND, pIDs: *mut IN const UINT, pColsWidth: *mut IN const INT, pColsAlign: *mut IN const INT, nNumOfColumns: IN UINT) -> i32 {
    // TODO: implementar CreateListViewColumns desde reactos/reactos.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateFileSystemList(Left: IN SHORT, Top: IN SHORT, ForceFormat: IN BOOLEAN, SelectFileSystem: IN PCWSTR) -> PFILE_SYSTEM_LIST {
    // TODO: implementar CreateFileSystemList desde reactos/fslist.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateProgressBarEx(Left: IN SHORT, Top: IN SHORT, Right: IN SHORT, Bottom: IN SHORT, TextTop: IN SHORT, TextRight: IN SHORT, DoubleEdge: IN BOOLEAN, ProgressColour: IN SHORT, StepCount: IN ULONG, OPTIONAL: IN PCSTR DescriptionText, OPTIONAL: IN PCSTR ProgressFormatText, OPTIONAL: IN PUPDATE_PROGRESS UpdateProgressProc) -> PPROGRESSBAR {
    // TODO: implementar CreateProgressBarEx desde reactos/progress.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateProgressBar(Left: IN SHORT, Top: IN SHORT, Right: IN SHORT, Bottom: IN SHORT, TextTop: IN SHORT, TextRight: IN SHORT, DoubleEdge: IN BOOLEAN, OPTIONAL: IN PCSTR DescriptionText) -> PPROGRESSBAR {
    // TODO: implementar CreateProgressBar desde reactos/progress.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PanelOnCreate(hWnd: *mut core::ffi::c_void, wParam: WPARAM, lParam: LPARAM) -> static int {
    // TODO: implementar PanelOnCreate desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateLargeCell(infoPtr: PMAP) -> static BOOL {
    // TODO: implementar CreateLargeCell desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MapOnCreate(infoPtr: PMAP, hwnd: *mut core::ffi::c_void, hParent: *mut core::ffi::c_void) -> static BOOL {
    // TODO: implementar MapOnCreate desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_target_directory(Target: LPWSTR) -> static void {
    // TODO: implementar create_target_directory desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Display_OnCreate(hwnd: *mut core::ffi::c_void) -> static LRESULT {
    // TODO: implementar Display_OnCreate desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MainWnd_OnCreate(hwnd: *mut core::ffi::c_void) -> static LRESULT {
    // TODO: implementar MainWnd_OnCreate desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTrayIcon(szKLID: LPTSTR, OPTIONAL: LPCTSTR szImeFile) -> static HICON {
    // TODO: implementar CreateTrayIcon desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn KbSwitch_OnCreate(hwnd: *mut core::ffi::c_void) -> static INT {
    // TODO: implementar KbSwitch_OnCreate desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateNewConsoleTitle(param_9080: VOID) -> static LPTSTR {
    // TODO: implementar CreateNewConsoleTitle desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateNewMDIChild(Info: PCONSOLE_MAINFRAME_WND, hwndMDIClient: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    // TODO: implementar CreateNewMDIChild desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FrameOnCreate(hwnd: *mut core::ffi::c_void, lParam: LPARAM) -> static LRESULT {
    // TODO: implementar FrameOnCreate desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreatePropSheet(hInstance: HINSTANCE, hwndOwner: *mut core::ffi::c_void, lpszTitle: LPCTSTR) -> *mut core::ffi::c_void {
    // TODO: implementar CreatePropSheet desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OnMainCreate(hwnd: *mut core::ffi::c_void, pRdpSettings: PRDPSETTINGS) -> static BOOL {
    // TODO: implementar OnMainCreate desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rdssl_rc4_info_create() -> *mut core::ffi::c_void {
    // TODO: implementar rdssl_rc4_info_create desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rdssl_sha1_info_create() -> *mut core::ffi::c_void {
    // TODO: implementar rdssl_sha1_info_create desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rdssl_md5_info_create() -> *mut core::ffi::c_void {
    // TODO: implementar rdssl_md5_info_create desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rdssl_hash_info_create(id: ALG_ID) -> *mut core::ffi::c_void {
    // TODO: implementar rdssl_hash_info_create desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_create_window() -> i32 {
    // TODO: implementar mi_create_window desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_create_cursor(x: u32, y: u32, width: i32, height: i32, andmask: *mut u8, xormask: *mut u8) -> *mut core::ffi::c_void {
    // TODO: implementar mi_create_cursor desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _dl_tls_block_size_with_pre() -> static size_t {
    // TODO: implementar _dl_tls_block_size_with_pre desde glibc/dl-tls_block_align.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _dl_tls_block_align(size: usize, allocated: *mut core::ffi::c_void) -> *mut static void {
    // TODO: implementar _dl_tls_block_align desde glibc/dl-tls_block_align.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __lll_abstimed_lock(__ptr: *mut core::ffi::c_void, __tsp: *mut const struct timespec, __flags: i32, __clk: i32) -> extern int {
    // TODO: implementar __lll_abstimed_lock desde glibc/hurdlock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __lll_robust_lock(__ptr: *mut core::ffi::c_void, __flags: i32) -> extern int {
    // TODO: implementar __lll_robust_lock desde glibc/hurdlock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __lll_robust_abstimed_lock(__ptr: *mut core::ffi::c_void, __tsp: *mut const struct timespec, __flags: i32, __clk: i32) -> extern int {
    // TODO: implementar __lll_robust_abstimed_lock desde glibc/hurdlock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __lll_robust_trylock(__ptr: *mut core::ffi::c_void) -> extern int {
    // TODO: implementar __lll_robust_trylock desde glibc/hurdlock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __lll_robust_unlock(__ptr: *mut core::ffi::c_void, __flags: i32) -> extern void {
    // TODO: implementar __lll_robust_unlock desde glibc/hurdlock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __gconv_create_spec(conv_spec: *mut struct gconv_spec, fromcode: *mut const char, tocode: *mut const char) -> *mut extern struct gconv_spec {
    // TODO: implementar __gconv_create_spec desde glibc/gconv_int.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __argz_create_sep(__string: const char *__restrict, __sep: i32, __argz: *mut core::ffi::c_void, __len: size_t *__restrict) -> extern error_t {
    // TODO: implementar __argz_create_sep desde glibc/argz.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn htab_create() -> *mut inline static struct hashtab {
    // TODO: implementar htab_create desde glibc/inline-hashtab.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __pthread_barrier_init(__barrier: pthread_barrier_t *__restrict, __attr: const pthread_barrierattr_t *__restrict, __count: u32) -> extern int {
    // TODO: implementar __pthread_barrier_init desde glibc/pthread.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __pthread_barrier_wait(__barrier: *mut pthread_barrier_t) -> extern int {
    // TODO: implementar __pthread_barrier_wait desde glibc/pthread.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __pthread_initialize() -> extern void {
    // TODO: implementar __pthread_initialize desde glibc/pthread.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __pthread_self() -> extern pthread_t {
    // TODO: implementar __pthread_self desde glibc/pthread.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __sigblock(__mask: i32) -> extern int {
    // TODO: implementar __sigblock desde glibc/signal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __clock_settime64(clock_id: clockid_t, tp: *mut const struct __timespec64) -> extern int {
    // TODO: implementar __clock_settime64 desde glibc/time.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __clock_getres64(clock_id: clockid_t, tp: *mut struct __timespec64) -> extern int {
    // TODO: implementar __clock_getres64 desde glibc/time.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __clock_nanosleep_time64(clock_id: clockid_t, flags: i32, req: *mut const struct __timespec64, rem: *mut struct __timespec64) -> extern int {
    // TODO: implementar __clock_nanosleep_time64 desde glibc/time.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __clock_gettime64(clock_id: clockid_t, tp: *mut struct __timespec64) -> extern int {
    // TODO: implementar __clock_gettime64 desde glibc/time.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn clock_from_timebase(timebase: i32) -> static inline clockid_t {
    // TODO: implementar clock_from_timebase desde glibc/time.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _IO_peekc_locked(__fp: *mut FILE) -> extern int {
    // TODO: implementar _IO_peekc_locked desde glibc/libio.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _IO_flockfile(param_63061: *mut FILE) -> extern void {
    // TODO: implementar _IO_flockfile desde glibc/libio.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _IO_funlockfile(param_63061: *mut FILE) -> extern void {
    // TODO: implementar _IO_funlockfile desde glibc/libio.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _IO_ftrylockfile(param_63061: *mut FILE) -> extern int {
    // TODO: implementar _IO_ftrylockfile desde glibc/libio.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _IO_list_lock() -> extern void {
    // TODO: implementar _IO_list_lock desde glibc/libioP.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _IO_list_unlock() -> extern void {
    // TODO: implementar _IO_list_unlock desde glibc/libioP.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _IO_list_resetlock() -> extern void {
    // TODO: implementar _IO_list_resetlock desde glibc/libioP.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _IO_enable_locks() -> extern void {
    // TODO: implementar _IO_enable_locks desde glibc/libioP.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _IO_proc_file_chain_lock() -> extern void {
    // TODO: implementar _IO_proc_file_chain_lock desde glibc/libioP.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _IO_proc_file_chain_unlock() -> extern void {
    // TODO: implementar _IO_proc_file_chain_unlock desde glibc/libioP.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _IO_proc_file_chain_resetlock() -> extern void {
    // TODO: implementar _IO_proc_file_chain_resetlock desde glibc/libioP.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _IO_seekoff_unlocked(param_63061: *mut FILE, param_39906: off64_t, param_59621: i32, param_59621: i32) -> extern off64_t {
    // TODO: implementar _IO_seekoff_unlocked desde glibc/libioP.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _IO_seekpos_unlocked(param_63061: *mut FILE, param_39906: off64_t, param_59621: i32) -> extern off64_t {
    // TODO: implementar _IO_seekpos_unlocked desde glibc/libioP.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __spin_lock_init(__lock: *mut __spin_lock_t) -> extern void {
    // TODO: implementar __spin_lock_init desde glibc/lock-intern.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __spin_lock_solid(__lock: *mut __spin_lock_t) -> extern void {
    // TODO: implementar __spin_lock_solid desde glibc/lock-intern.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __spin_lock(__lock: *mut __spin_lock_t) -> extern void {
    // TODO: implementar __spin_lock desde glibc/lock-intern.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __spin_unlock(__lock: *mut __spin_lock_t) -> extern void {
    // TODO: implementar __spin_unlock desde glibc/lock-intern.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __spin_try_lock(__lock: *mut __spin_lock_t) -> extern int {
    // TODO: implementar __spin_try_lock desde glibc/lock-intern.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __spin_lock_locked(__lock: *mut __spin_lock_t) -> extern int {
    // TODO: implementar __spin_lock_locked desde glibc/lock-intern.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __mutex_init(__lock: *mut core::ffi::c_void) -> extern void {
    // TODO: implementar __mutex_init desde glibc/lock-intern.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __mutex_lock(__lock: *mut core::ffi::c_void) -> extern void {
    // TODO: implementar __mutex_lock desde glibc/lock-intern.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __mutex_unlock(__lock: *mut core::ffi::c_void) -> extern void {
    // TODO: implementar __mutex_unlock desde glibc/lock-intern.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __mutex_trylock(__lock: *mut core::ffi::c_void) -> extern int {
    // TODO: implementar __mutex_trylock desde glibc/lock-intern.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn get_block_size(rand_data: u32) -> static unsigned int {
    // TODO: implementar get_block_size desde glibc/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn get_random_block_size(state: *mut u32) -> static unsigned int {
    // TODO: implementar get_random_block_size desde glibc/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_mutex(iters: i64, filler: i32) -> static timing_t {
    // TODO: implementar test_mutex desde glibc/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_mutex_trylock(iters: i64, filler: i32) -> static timing_t {
    // TODO: implementar test_mutex_trylock desde glibc/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_spin_lock(iters: i64, filler: i32) -> static timing_t {
    // TODO: implementar test_spin_lock desde glibc/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_spin_trylock(iters: i64, filler: i32) -> static timing_t {
    // TODO: implementar test_spin_trylock desde glibc/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn bench_random_lock(json_ctx: *mut json_ctx_t, iters: usize) -> static void {
    // TODO: implementar bench_random_lock desde glibc/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AnnotateRWLockCreate(file: *mut const char, line: i32, lock: *mut const volatile void) -> core::ffi::c_void {
    // TODO: implementar AnnotateRWLockCreate desde cpython/dynamic_annotations.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AnnotateRWLockDestroy(file: *mut const char, line: i32, lock: *mut const volatile void) -> core::ffi::c_void {
    // TODO: implementar AnnotateRWLockDestroy desde cpython/dynamic_annotations.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AnnotateRWLockAcquired(file: *mut const char, line: i32, lock: *mut const volatile void, is_w: i64) -> core::ffi::c_void {
    // TODO: implementar AnnotateRWLockAcquired desde cpython/dynamic_annotations.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AnnotateRWLockReleased(file: *mut const char, line: i32, lock: *mut const volatile void, is_w: i64) -> core::ffi::c_void {
    // TODO: implementar AnnotateRWLockReleased desde cpython/dynamic_annotations.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AnnotatePCQCreate(file: *mut const char, line: i32, pcq: *mut const volatile void) -> core::ffi::c_void {
    // TODO: implementar AnnotatePCQCreate desde cpython/dynamic_annotations.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AnnotateMutexIsUsedAsCondVar(file: *mut const char, line: i32, mu: *mut const volatile void) -> core::ffi::c_void {
    // TODO: implementar AnnotateMutexIsUsedAsCondVar desde cpython/dynamic_annotations.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _xidregistry_lock(registry: *mut dlregistry_t) -> static void {
    // TODO: implementar _xidregistry_lock desde cpython/crossinterp_data_lookup.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _xidregistry_unlock(registry: *mut dlregistry_t) -> static void {
    // TODO: implementar _xidregistry_unlock desde cpython/crossinterp_data_lookup.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EnterNonRecursiveMutex(mutex: PNRMUTEX, milliseconds: u32) -> static DWORD {
    // TODO: implementar EnterNonRecursiveMutex desde cpython/thread_nt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LeaveNonRecursiveMutex(mutex: PNRMUTEX) -> static BOOL {
    // TODO: implementar LeaveNonRecursiveMutex desde cpython/thread_nt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSemaphore(param_50043: NULL, param_4655: 1, param_4655: 1, param_50043: NULL) -> return {
    // TODO: implementar CreateSemaphore desde cpython/thread_nt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyMutex_Lock(m: *mut PyMutex) -> static inline void {
    // TODO: implementar _PyMutex_Lock desde cpython/pylock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyMutex_Unlock(m: *mut PyMutex) -> static inline void {
    // TODO: implementar _PyMutex_Unlock desde cpython/pylock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyMutex_IsLocked(m: *mut PyMutex) -> static inline int {
    // TODO: implementar _PyMutex_IsLocked desde cpython/pylock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _BlocksOutputBuffer_InitWithSize(buffer: *mut _BlocksOutputBuffer, init_size: const Py_ssize_t, param_64866: *mut core::ffi::c_void) -> static inline Py_ssize_t {
    // TODO: implementar _BlocksOutputBuffer_InitWithSize desde cpython/pycore_blocks_output_buffer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _BlocksOutputBuffer_Grow(buffer: *mut _BlocksOutputBuffer, param_64866: *mut core::ffi::c_void, avail_out: const Py_ssize_t) -> static inline Py_ssize_t {
    // TODO: implementar _BlocksOutputBuffer_Grow desde cpython/pycore_blocks_output_buffer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _BlocksOutputBuffer_GetDataSize(buffer: *mut _BlocksOutputBuffer, avail_out: const Py_ssize_t) -> static inline Py_ssize_t {
    // TODO: implementar _BlocksOutputBuffer_GetDataSize desde cpython/pycore_blocks_output_buffer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _BlocksOutputBuffer_Finish(buffer: *mut _BlocksOutputBuffer, avail_out: const Py_ssize_t) -> *mut static inline PyObject {
    // TODO: implementar _BlocksOutputBuffer_Finish desde cpython/pycore_blocks_output_buffer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _BlocksOutputBuffer_OnError(buffer: *mut _BlocksOutputBuffer) -> static inline void {
    // TODO: implementar _BlocksOutputBuffer_OnError desde cpython/pycore_blocks_output_buffer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyEval_AcquireLock(tstate: *mut PyThreadState) -> extern void {
    // TODO: implementar _PyEval_AcquireLock desde cpython/pycore_ceval.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyEval_ReleaseLock(param_15054: *mut PyInterpreterState, param_6068: *mut PyThreadState, final_release: i32) -> extern void {
    // TODO: implementar _PyEval_ReleaseLock desde cpython/pycore_ceval.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyCompile_PushFBlock(c: *mut struct _PyCompiler, loc: _Py_SourceLocation, t: enum _PyCompile_FBlockType, block_label: _PyJumpTargetLabel, exit: _PyJumpTargetLabel, datum: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar _PyCompile_PushFBlock desde cpython/pycore_compile.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyCompile_PopFBlock(c: *mut struct _PyCompiler, t: enum _PyCompile_FBlockType, block_label: _PyJumpTargetLabel) -> core::ffi::c_void {
    // TODO: implementar _PyCompile_PopFBlock desde cpython/pycore_compile.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyCompile_EnterConditionalBlock(c: *mut struct _PyCompiler) -> core::ffi::c_void {
    // TODO: implementar _PyCompile_EnterConditionalBlock desde cpython/pycore_compile.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyCompile_LeaveConditionalBlock(c: *mut struct _PyCompiler) -> core::ffi::c_void {
    // TODO: implementar _PyCompile_LeaveConditionalBlock desde cpython/pycore_compile.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyCriticalSection_BeginMutex(tstate: *mut PyThreadState, c: *mut PyCriticalSection, m: *mut PyMutex) -> static inline void {
    // TODO: implementar _PyCriticalSection_BeginMutex desde cpython/pycore_critical_section.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyCriticalSection2_BeginMutex(tstate: *mut PyThreadState, c: *mut PyCriticalSection2, m1: *mut PyMutex, m2: *mut PyMutex) -> static inline void {
    // TODO: implementar _PyCriticalSection2_BeginMutex desde cpython/pycore_critical_section.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyDict_ClearKeysVersionLockHeld(mp: *mut PyObject) -> extern void {
    // TODO: implementar _PyDict_ClearKeysVersionLockHeld desde cpython/pycore_dict.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyDict_SizeOf_LockHeld(param_18877: *mut PyDictObject) -> extern Py_ssize_t {
    // TODO: implementar _PyDict_SizeOf_LockHeld desde cpython/pycore_dict.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyDict_GetItemRef_Unicode_LockHeld(op: *mut PyDictObject, key: *mut PyObject, param_45657: *mut PyObject) -> extern int {
    // TODO: implementar _PyDict_GetItemRef_Unicode_LockHeld desde cpython/pycore_dict.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyDict_Clear_LockHeld(op: *mut PyObject) -> extern void {
    // TODO: implementar _PyDict_Clear_LockHeld desde cpython/pycore_dict.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _Py_get_blocking(fd: i32) -> extern int {
    // TODO: implementar _Py_get_blocking desde cpython/pycore_fileutils.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _Py_set_blocking(fd: i32, blocking: i32) -> extern int {
    // TODO: implementar _Py_set_blocking desde cpython/pycore_fileutils.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyImport_AcquireLock(interp: *mut PyInterpreterState) -> extern void {
    // TODO: implementar _PyImport_AcquireLock desde cpython/pycore_import.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyImport_ReleaseLock(interp: *mut PyInterpreterState) -> extern void {
    // TODO: implementar _PyImport_ReleaseLock desde cpython/pycore_import.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyImport_ReInitLock(interp: *mut PyInterpreterState) -> extern void {
    // TODO: implementar _PyImport_ReInitLock desde cpython/pycore_import.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyConfig_CreateXOptionsDict(config: *mut const PyConfig) -> *mut extern PyObject {
    // TODO: implementar _PyConfig_CreateXOptionsDict desde cpython/pycore_initconfig.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyMutex_LockFast(m: *mut PyMutex) -> static inline int {
    // TODO: implementar PyMutex_LockFast desde cpython/pycore_lock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyMutex_at_fork_reinit(m: *mut PyMutex) -> static inline void {
    // TODO: implementar _PyMutex_at_fork_reinit desde cpython/pycore_lock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyMutex_LockFlags(m: *mut PyMutex, flags: _PyLockFlags) -> static inline void {
    // TODO: implementar PyMutex_LockFlags desde cpython/pycore_lock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyMutex_TryUnlock(m: *mut PyMutex) -> extern int {
    // TODO: implementar _PyMutex_TryUnlock desde cpython/pycore_lock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyRawMutex_LockSlow(m: *mut _PyRawMutex) -> extern void {
    // TODO: implementar _PyRawMutex_LockSlow desde cpython/pycore_lock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyRawMutex_UnlockSlow(m: *mut _PyRawMutex) -> extern void {
    // TODO: implementar _PyRawMutex_UnlockSlow desde cpython/pycore_lock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyRawMutex_Lock(m: *mut _PyRawMutex) -> static inline void {
    // TODO: implementar _PyRawMutex_Lock desde cpython/pycore_lock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyRawMutex_Unlock(m: *mut _PyRawMutex) -> static inline void {
    // TODO: implementar _PyRawMutex_Unlock desde cpython/pycore_lock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyRecursiveMutex_LockTimed(m: *mut _PyRecursiveMutex, timeout: PyTime_t, flags: _PyLockFlags) -> extern PyLockStatus {
    // TODO: implementar _PyRecursiveMutex_LockTimed desde cpython/pycore_lock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyRecursiveMutex_TryUnlock(m: *mut _PyRecursiveMutex) -> extern int {
    // TODO: implementar _PyRecursiveMutex_TryUnlock desde cpython/pycore_lock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyModule_CreateInitialized(param_40462: *mut PyModuleDef, apiver: i32) -> *mut extern PyObject {
    // TODO: implementar _PyModule_CreateInitialized desde cpython/pycore_modsupport.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _Py_NewRefWithLock(op: *mut PyObject) -> *mut static inline PyObject {
    // TODO: implementar _Py_NewRefWithLock desde cpython/pycore_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _Py_XNewRefWithLock(obj: *mut PyObject) -> *mut static inline PyObject {
    // TODO: implementar _Py_XNewRefWithLock desde cpython/pycore_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyUnicodeTranslateError_Create(object: *mut PyObject, start: Py_ssize_t, end: Py_ssize_t, reason: *mut const char) -> *mut extern PyObject {
    // TODO: implementar _PyUnicodeTranslateError_Create desde cpython/pycore_pyerrors.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PySys_Create(tstate: *mut PyThreadState, param_45657: *mut PyObject) -> extern PyStatus {
    // TODO: implementar _PySys_Create desde cpython/pycore_pylifecycle.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _Py_stackref_create(param_14166: obj, param_51689: flags, param_19420: filename, param_20609: linenumber) -> return {
    // TODO: implementar _Py_stackref_create desde cpython/pycore_stackref.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_heap_contains_block(heap: *mut mi_heap_t, p: *mut const void) -> mi_decl_export bool {
    // TODO: implementar mi_heap_contains_block desde cpython/mimalloc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_heap_visit_blocks(heap: *mut const mi_heap_t, visit_all_blocks: bool, visitor: *mut mi_block_visit_fun, arg: *mut core::ffi::c_void) -> mi_decl_export bool {
    // TODO: implementar mi_heap_visit_blocks desde cpython/mimalloc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _mi_abandoned_pool_visit_blocks(pool: *mut mi_abandoned_pool_t, page_tag: u8, visit_blocks: bool, visitor: *mut mi_block_visit_fun, arg: *mut core::ffi::c_void) -> bool {
    // TODO: implementar _mi_abandoned_pool_visit_blocks desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _mi_heap_area_visit_blocks(area: *mut const mi_heap_area_t, page: *mut mi_page_t, visitor: *mut mi_block_visit_fun, arg: *mut core::ffi::c_void) -> bool {
    // TODO: implementar _mi_heap_area_visit_blocks desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _mi_clock_now() -> mi_msecs_t {
    // TODO: implementar _mi_clock_now desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _mi_clock_end(start: mi_msecs_t) -> mi_msecs_t {
    // TODO: implementar _mi_clock_end desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _mi_clock_start() -> mi_msecs_t {
    // TODO: implementar _mi_clock_start desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_page_block_size(page: *mut const mi_page_t) -> static inline size_t {
    // TODO: implementar mi_page_block_size desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_page_usable_block_size(page: *mut const mi_page_t) -> static inline size_t {
    // TODO: implementar mi_page_usable_block_size desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_tf_block(tf: mi_thread_free_t) -> *mut static inline mi_block_t {
    // TODO: implementar mi_tf_block desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_tf_set_block(tf: mi_thread_free_t, block: *mut mi_block_t) -> static inline mi_thread_free_t {
    // TODO: implementar mi_tf_set_block desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_block_nextx(null: *mut const void, block: *mut const mi_block_t, keys: *mut const uintptr_t) -> *mut static inline mi_block_t {
    // TODO: implementar mi_block_nextx desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_block_set_nextx(null: *mut const void, block: *mut mi_block_t, next: *mut const mi_block_t, keys: *mut const uintptr_t) -> static inline void {
    // TODO: implementar mi_block_set_nextx desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_block_next(page: *mut const mi_page_t, block: *mut const mi_block_t) -> *mut static inline mi_block_t {
    // TODO: implementar mi_block_next desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_block_set_next(page: *mut const mi_page_t, block: *mut mi_block_t, next: *mut const mi_block_t) -> static inline void {
    // TODO: implementar mi_block_set_next desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_commit_mask_create_empty(cm: *mut mi_commit_mask_t) -> static inline void {
    // TODO: implementar mi_commit_mask_create_empty desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_commit_mask_create_full(cm: *mut mi_commit_mask_t) -> static inline void {
    // TODO: implementar mi_commit_mask_create_full desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _mi_memid_create(memkind: mi_memkind_t) -> static inline mi_memid_t {
    // TODO: implementar _mi_memid_create desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _mi_memid_create_os(committed: bool, is_zero: bool, is_large: bool) -> static inline mi_memid_t {
    // TODO: implementar _mi_memid_create_os desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _mi_prim_clock_now() -> mi_msecs_t {
    // TODO: implementar _mi_prim_clock_now desde cpython/prim.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn flock(param_61892: 2) -> See the Unix manual page for {
    // TODO: implementar flock desde cpython/fcntlmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fcntl_flock_impl(module: *mut PyObject, fd: i32, code: i32) -> *mut static PyObject {
    // TODO: implementar fcntl_flock_impl desde cpython/fcntlmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fcntl_flock(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar fcntl_flock desde cpython/fcntlmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fcntl_lockf_impl(module: *mut PyObject, fd: i32, code: i32, lenobj: *mut PyObject, startobj: *mut PyObject, whence: i32) -> *mut static PyObject {
    // TODO: implementar fcntl_lockf_impl desde cpython/fcntlmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fcntl_lockf(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar fcntl_lockf desde cpython/fcntlmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _hmac_HMAC_block_size_get_impl(self: *mut HMACObject) -> *mut static PyObject {
    // TODO: implementar _hmac_HMAC_block_size_get_impl desde cpython/hmacmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _hmac_HMAC_block_size_get(self: *mut PyObject, param_64866: *mut core::ffi::c_void) -> *mut static PyObject {
    // TODO: implementar _hmac_HMAC_block_size_get desde cpython/hmacmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _overlapped_CreateIoCompletionPort_impl(module: *mut PyObject, FileHandle: *mut core::ffi::c_void, ExistingCompletionPort: *mut core::ffi::c_void, CompletionKey: ULONG_PTR, NumberOfConcurrentThreads: u32) -> *mut static PyObject {
    // TODO: implementar _overlapped_CreateIoCompletionPort_impl desde cpython/overlapped.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _overlapped_CreateIoCompletionPort(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _overlapped_CreateIoCompletionPort desde cpython/overlapped.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _overlapped_CreateEvent_impl(module: *mut PyObject, EventAttributes: *mut PyObject, ManualReset: i32, InitialState: i32, Name: *mut const wchar_t) -> *mut static PyObject {
    // TODO: implementar _overlapped_CreateEvent_impl desde cpython/overlapped.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _overlapped_CreateEvent(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _overlapped_CreateEvent desde cpython/overlapped.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unlockpt() -> Performs an {
    // TODO: implementar unlockpt desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_unlockpt_impl(module: *mut PyObject, fd: i32) -> *mut static PyObject {
    // TODO: implementar os_unlockpt_impl desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_unlockpt(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar os_unlockpt desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_plock_impl(module: *mut PyObject, op: i32) -> *mut static PyObject {
    // TODO: implementar os_plock_impl desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_plock(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar os_plock desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_timerfd_create_impl(module: *mut PyObject, clockid: i32, flags: i32) -> *mut static PyObject {
    // TODO: implementar os_timerfd_create_impl desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_timerfd_create(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar os_timerfd_create desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_lockf_impl(module: *mut PyObject, fd: i32, command: i32, length: Py_off_t) -> *mut static PyObject {
    // TODO: implementar os_lockf_impl desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_lockf(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar os_lockf desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_memfd_create_impl(module: *mut PyObject, name: *mut PyObject, flags: u32) -> *mut static PyObject {
    // TODO: implementar os_memfd_create_impl desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_memfd_create(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar os_memfd_create desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_get_blocking_impl(module: *mut PyObject, fd: i32) -> static int {
    // TODO: implementar os_get_blocking_impl desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_get_blocking(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar os_get_blocking desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_set_blocking_impl(module: *mut PyObject, fd: i32, blocking: i32) -> *mut static PyObject {
    // TODO: implementar os_set_blocking_impl desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_set_blocking(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar os_set_blocking desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os__create_environ_impl(module: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar os__create_environ_impl desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os__create_environ(module: *mut PyObject, param_45657: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar os__create_environ desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn time_clock_gettime_impl(module: *mut PyObject, clk_id: clockid_t) -> *mut static PyObject {
    // TODO: implementar time_clock_gettime_impl desde cpython/timemodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn time_clock_gettime(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar time_clock_gettime desde cpython/timemodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn time_clock_gettime_ns_impl(module: *mut PyObject, clk_id: clockid_t) -> *mut static PyObject {
    // TODO: implementar time_clock_gettime_ns_impl desde cpython/timemodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn time_clock_gettime_ns(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar time_clock_gettime_ns desde cpython/timemodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unicodedata_block_impl(module: *mut PyObject, chr: i32) -> *mut static PyObject {
    // TODO: implementar unicodedata_block_impl desde cpython/unicodedata.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unicodedata_block(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar unicodedata_block desde cpython/unicodedata.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__asyncio_future_blocking_get_impl(self: *mut FutureObj) -> *mut static PyObject {
    // TODO: implementar _asyncio_Future__asyncio_future_blocking_get_impl desde cpython/_asynciomodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__asyncio_future_blocking_get(self: *mut PyObject, param_64866: *mut core::ffi::c_void) -> *mut static PyObject {
    // TODO: implementar _asyncio_Future__asyncio_future_blocking_get desde cpython/_asynciomodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__asyncio_future_blocking_set_impl(self: *mut FutureObj, value: *mut PyObject) -> static int {
    // TODO: implementar _asyncio_Future__asyncio_future_blocking_set_impl desde cpython/_asynciomodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__asyncio_future_blocking_set(self: *mut PyObject, value: *mut PyObject, param_64866: *mut core::ffi::c_void) -> static int {
    // TODO: implementar _asyncio_Future__asyncio_future_blocking_set desde cpython/_asynciomodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _interpqueues_create_impl(module: *mut PyObject, maxsize: Py_ssize_t, unboundarg: i32, fallbackarg: i32) -> *mut static PyObject {
    // TODO: implementar _interpqueues_create_impl desde cpython/_interpqueuesmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _interpqueues_create(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _interpqueues_create desde cpython/_interpqueuesmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _interpreters_create_impl(module: *mut PyObject, configobj: *mut PyObject, reqrefs: i32) -> *mut static PyObject {
    // TODO: implementar _interpreters_create_impl desde cpython/_interpretersmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _interpreters_create(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _interpreters_create desde cpython/_interpretersmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn locked() -> An obsolete synonym of {
    // TODO: implementar locked desde cpython/_threadmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lock_new_impl(type: *mut PyTypeObject) -> *mut static PyObject {
    // TODO: implementar lock_new_impl desde cpython/_threadmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lock_new(type: *mut PyTypeObject, args: *mut PyObject, kwargs: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar lock_new desde cpython/_threadmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rlock_new_impl(type: *mut PyTypeObject) -> *mut static PyObject {
    // TODO: implementar rlock_new_impl desde cpython/_threadmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rlock_new(type: *mut PyTypeObject, args: *mut PyObject, kwargs: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar rlock_new desde cpython/_threadmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_createcommand_impl(self: *mut TkappObject, name: *mut const char, func: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _tkinter_tkapp_createcommand_impl desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_createcommand(self: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _tkinter_tkapp_createcommand desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_createfilehandler_impl(self: *mut TkappObject, file: *mut PyObject, mask: i32, func: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _tkinter_tkapp_createfilehandler_impl desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_createfilehandler(self: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _tkinter_tkapp_createfilehandler desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_createtimerhandler_impl(self: *mut TkappObject, milliseconds: i32, func: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _tkinter_tkapp_createtimerhandler_impl desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_createtimerhandler(self: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _tkinter_tkapp_createtimerhandler desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_create_impl(module: *mut PyObject, screenName: *mut const char, baseName: *mut const char, className: *mut const char, interactive: i32, wantobjects: i32, wantTk: i32, sync: i32, use: *mut const char) -> *mut static PyObject {
    // TODO: implementar _tkinter_create_impl desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_create(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _tkinter_create desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateEventW_impl(module: *mut PyObject, security_attributes: LPSECURITY_ATTRIBUTES, manual_reset: i32, initial_state: i32, name: LPCWSTR) -> static HANDLE {
    // TODO: implementar _winapi_CreateEventW_impl desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateEventW(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _winapi_CreateEventW desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateFile_impl(module: *mut PyObject, file_name: LPCWSTR, desired_access: u32, share_mode: u32, security_attributes: LPSECURITY_ATTRIBUTES, creation_disposition: u32, flags_and_attributes: u32, template_file: *mut core::ffi::c_void) -> static HANDLE {
    // TODO: implementar _winapi_CreateFile_impl desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateFile(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _winapi_CreateFile desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateFileMapping_impl(module: *mut PyObject, file_handle: *mut core::ffi::c_void, security_attributes: LPSECURITY_ATTRIBUTES, protect: u32, max_size_high: u32, max_size_low: u32, name: LPCWSTR) -> static HANDLE {
    // TODO: implementar _winapi_CreateFileMapping_impl desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateFileMapping(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _winapi_CreateFileMapping desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateJunction_impl(module: *mut PyObject, src_path: LPCWSTR, dst_path: LPCWSTR) -> *mut static PyObject {
    // TODO: implementar _winapi_CreateJunction_impl desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateJunction(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _winapi_CreateJunction desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateMutexW_impl(module: *mut PyObject, security_attributes: LPSECURITY_ATTRIBUTES, initial_owner: i32, name: LPCWSTR) -> static HANDLE {
    // TODO: implementar _winapi_CreateMutexW_impl desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateMutexW(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _winapi_CreateMutexW desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateNamedPipe_impl(module: *mut PyObject, name: LPCWSTR, open_mode: u32, pipe_mode: u32, max_instances: u32, out_buffer_size: u32, in_buffer_size: u32, default_timeout: u32, security_attributes: LPSECURITY_ATTRIBUTES) -> static HANDLE {
    // TODO: implementar _winapi_CreateNamedPipe_impl desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateNamedPipe(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _winapi_CreateNamedPipe desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreatePipe_impl(module: *mut PyObject, pipe_attrs: *mut PyObject, size: u32) -> *mut static PyObject {
    // TODO: implementar _winapi_CreatePipe_impl desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreatePipe(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _winapi_CreatePipe desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateProcess_impl(module: *mut PyObject, application_name: *mut const wchar_t, command_line: *mut PyObject, proc_attrs: *mut PyObject, thread_attrs: *mut PyObject, inherit_handles: i32, creation_flags: u32, env_mapping: *mut PyObject, current_directory: *mut const wchar_t, startup_info: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _winapi_CreateProcess_impl desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateProcess(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _winapi_CreateProcess desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_OpenMutexW_impl(module: *mut PyObject, desired_access: u32, inherit_handle: i32, name: LPCWSTR) -> static HANDLE {
    // TODO: implementar _winapi_OpenMutexW_impl desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_OpenMutexW(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _winapi_OpenMutexW desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_ReleaseMutex_impl(module: *mut PyObject, mutex: *mut core::ffi::c_void) -> *mut static PyObject {
    // TODO: implementar _winapi_ReleaseMutex_impl desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_ReleaseMutex(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _winapi_ReleaseMutex desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn stginfo_set_dict_final_lock_held(info: *mut StgInfo) -> static inline void {
    // TODO: implementar stginfo_set_dict_final_lock_held desde cpython/ctypes.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Hacl_Hash_SHA3_block_len(s: *mut Hacl_Hash_SHA3_state_t) -> u32 {
    // TODO: implementar Hacl_Hash_SHA3_block_len desde cpython/Hacl_Hash_SHA3.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Hacl_Hash_SHA3_shake128_absorb_nblocks(state: *mut u64, input: *mut u8, inputByteLen: u32) -> core::ffi::c_void {
    // TODO: implementar Hacl_Hash_SHA3_shake128_absorb_nblocks desde cpython/Hacl_Hash_SHA3.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Hacl_Hash_SHA3_shake128_squeeze_nblocks(state: *mut u64, output: *mut u8, outputByteLen: u32) -> core::ffi::c_void {
    // TODO: implementar Hacl_Hash_SHA3_shake128_squeeze_nblocks desde cpython/Hacl_Hash_SHA3.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyTestInternalCapi_Init_Lock(module: *mut PyObject) -> i32 {
    // TODO: implementar _PyTestInternalCapi_Init_Lock desde cpython/parts.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_create_decimal_from_float_impl(context: *mut PyObject, cls: *mut PyTypeObject, f: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_create_decimal_from_float_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_create_decimal_from_float(context: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_create_decimal_from_float desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_create_decimal_impl(context: *mut PyObject, num: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_create_decimal_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_create_decimal(context: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_create_decimal desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn atexit_unregister_locked(callbacks: *mut PyObject, func: *mut PyObject) -> static int {
    // TODO: implementar atexit_unregister_locked desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn blake2_blake2b_copy_unlocked(self: *mut Blake2Object, cpy: *mut Blake2Object) -> static int {
    // TODO: implementar blake2_blake2b_copy_unlocked desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn py_blake2b_get_block_size(op: *mut PyObject, param_64866: *mut core::ffi::c_void) -> *mut static PyObject {
    // TODO: implementar py_blake2b_get_block_size desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyMutex_Lock(param_49502: &group_db_mutex) -> Py_END_ALLOW_THREADS {
    // TODO: implementar PyMutex_Lock desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn hmac_digest_compute_locked(self: *mut HMACObject, digest: *mut u8) -> static int {
    // TODO: implementar hmac_digest_compute_locked desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _grouper_create(param_27975: *mut core::ffi::c_void) -> return {
    // TODO: implementar _grouper_create desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn chain_next_lock_held(op: *mut PyObject) -> *mut static inline PyObject {
    // TODO: implementar chain_next_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn product_next_lock_held(op: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar product_next_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn combinations_next_lock_held(op: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar combinations_next_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cwr_next_lock_held(op: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cwr_next_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn permutations_next_lock_held(op: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar permutations_next_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn accumulate_next_lock_held(op: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar accumulate_next_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn zip_longest_next_lock_held(op: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar zip_longest_next_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MD5_get_block_size(param_45657: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar MD5_get_block_size desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mmap_gfind_lock_held(self: *mut mmap_object, view: *mut Py_buffer, start_obj: *mut PyObject, end_obj: *mut PyObject, reverse: i32) -> *mut static PyObject {
    // TODO: implementar mmap_gfind_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mmap__repr__method_lock_held(op: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar mmap__repr__method_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mmap_buffer_getbuf_lock_held(op: *mut PyObject, view: *mut Py_buffer, flags: i32) -> static int {
    // TODO: implementar mmap_buffer_getbuf_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mmap_length_lock_held(op: *mut PyObject) -> static Py_ssize_t {
    // TODO: implementar mmap_length_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mmap_item_lock_held(op: *mut PyObject, i: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar mmap_item_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mmap_subscript_lock_held(op: *mut PyObject, item: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar mmap_subscript_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mmap_ass_item_lock_held(op: *mut PyObject, i: Py_ssize_t, v: *mut PyObject) -> static int {
    // TODO: implementar mmap_ass_item_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mmap_ass_subscript_lock_held(op: *mut PyObject, item: *mut PyObject, value: *mut PyObject) -> static int {
    // TODO: implementar mmap_ass_subscript_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _py_get_history_length_lock_held() -> static int {
    // TODO: implementar _py_get_history_length_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn kqueue_tracking_add_lock_held(state: *mut _selectstate, self: *mut kqueue_queue_Object) -> static int {
    // TODO: implementar kqueue_tracking_add_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn kqueue_tracking_remove_lock_held(state: *mut _selectstate, self: *mut kqueue_queue_Object) -> static void {
    // TODO: implementar kqueue_tracking_remove_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHA1_get_block_size(param_45657: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar SHA1_get_block_size desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHA256_get_block_size(param_45657: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar SHA256_get_block_size desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHA512_get_block_size(param_45657: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar SHA512_get_block_size desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHA3_get_block_size(op: *mut PyObject, param_64866: *mut core::ffi::c_void) -> *mut static PyObject {
    // TODO: implementar SHA3_get_block_size desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn internal_setblocking(s: *mut PySocketSockObject, block: i32) -> static int {
    // TODO: implementar internal_setblocking desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sock_setblocking(self: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar sock_setblocking desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn blocking(true: flag is) -> Set the socket to {
    // TODO: implementar blocking desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sock_getblocking(self: *mut PyObject, param_45657: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar sock_getblocking desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn setblocking(param_4655: 1) -> the timeout feature and is equivalent to {
    // TODO: implementar setblocking desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn py_clock(state: *mut time_module_state, tp: *mut PyTime_t, info: *mut _Py_clock_info_t) -> static int {
    // TODO: implementar py_clock desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn time_clockid_converter(obj: *mut PyObject, p: *mut clockid_t) -> static int {
    // TODO: implementar time_clockid_converter desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn time_clock_settime(self: *mut PyObject, args: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar time_clock_settime desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn time_clock_settime_ns(self: *mut PyObject, args: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar time_clock_settime_ns desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn time_clock_getres(self: *mut PyObject, args: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar time_clock_getres desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn time_get_clock_info(module: *mut PyObject, args: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar time_get_clock_info desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unicodedata_create_capi() -> *mut static PyObject {
    // TODO: implementar unicodedata_create_capi desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_cancelled_error(state: *mut asyncio_state, fut: *mut FutureObj) -> *mut static PyObject {
    // TODO: implementar create_cancelled_error desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FutureIter_am_send_lock_held(it: *mut futureiterobject, param_45657: *mut PyObject) -> static PySendResult {
    // TODO: implementar FutureIter_am_send_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn task_wakeup_lock_held(task: *mut TaskObj, o: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar task_wakeup_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn newblock(deque: *mut dequeobject) -> *mut static inline block {
    // TODO: implementar newblock desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn deque_append_lock_held(deque: *mut dequeobject, item: *mut PyObject, maxlen: Py_ssize_t) -> static inline int {
    // TODO: implementar deque_append_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn deque_appendleft_lock_held(deque: *mut dequeobject, item: *mut PyObject, maxlen: Py_ssize_t) -> static inline int {
    // TODO: implementar deque_appendleft_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn deque_concat_lock_held(deque: *mut dequeobject, other: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar deque_concat_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn deque_inplace_repeat_lock_held(deque: *mut dequeobject, n: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar deque_inplace_repeat_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn deque_contains_lock_held(deque: *mut dequeobject, v: *mut PyObject) -> static int {
    // TODO: implementar deque_contains_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn deque_item_lock_held(deque: *mut dequeobject, i: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar deque_item_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn deque_ass_item_lock_held(deque: *mut dequeobject, i: Py_ssize_t, v: *mut PyObject) -> static int {
    // TODO: implementar deque_ass_item_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dequeiter_next_lock_held(it: *mut dequeiterobject, deque: *mut dequeobject) -> *mut static PyObject {
    // TODO: implementar dequeiter_next_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dequereviter_next_lock_held(it: *mut dequeiterobject, deque: *mut dequeobject) -> *mut static PyObject {
    // TODO: implementar dequereviter_next_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_timezone(offset: *mut PyObject, name: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar create_timezone desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_timezone_from_delta(days: i32, sec: i32, ms: i32, normalize: i32) -> *mut static PyObject {
    // TODO: implementar create_timezone_from_delta desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dbm_length_lock_held(self: *mut PyObject) -> static Py_ssize_t {
    // TODO: implementar dbm_length_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dbm_bool_lock_held(self: *mut PyObject) -> static int {
    // TODO: implementar dbm_bool_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dbm_subscript_lock_held(self: *mut PyObject, key: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar dbm_subscript_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dbm_ass_sub_lock_held(self: *mut PyObject, v: *mut PyObject, w: *mut PyObject) -> static int {
    // TODO: implementar dbm_ass_sub_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dbm_contains_lock_held(self: *mut PyObject, arg: *mut PyObject) -> static int {
    // TODO: implementar dbm_contains_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_elementiter(st: *mut elementtreestate, self: *mut ElementObject, tag: *mut PyObject, gettext: i32) -> *mut static PyObject {
    // TODO: implementar create_elementiter desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn bounded_lru_cache_get_lock_held(self: *mut lru_cache_object, args: *mut PyObject, kwds: *mut PyObject, param_45657: *mut PyObject, param_45657: *mut PyObject, hash: *mut Py_hash_t) -> static int {
    // TODO: implementar bounded_lru_cache_get_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn bounded_lru_cache_update_lock_held(self: *mut lru_cache_object, result: *mut PyObject, key: *mut PyObject, hash: Py_hash_t) -> *mut static PyObject {
    // TODO: implementar bounded_lru_cache_update_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_length_lock_held(op: *mut PyObject) -> static Py_ssize_t {
    // TODO: implementar gdbm_length_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_bool_lock_held(op: *mut PyObject) -> static int {
    // TODO: implementar gdbm_bool_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_subscript_lock_held(op: *mut PyObject, key: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar gdbm_subscript_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_ass_sub_lock_held(op: *mut PyObject, v: *mut PyObject, w: *mut PyObject) -> static int {
    // TODO: implementar gdbm_ass_sub_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn gdbm_contains_lock_held(self: *mut PyObject, arg: *mut PyObject) -> static int {
    // TODO: implementar gdbm_contains_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _hashlib_HASH_copy_locked(self: *mut HASHobject, new_ctx_p: *mut EVP_MD_CTX) -> static int {
    // TODO: implementar _hashlib_HASH_copy_locked desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _hashlib_HASH_get_blocksize(op: *mut PyObject, param_64866: *mut core::ffi::c_void) -> *mut static PyObject {
    // TODO: implementar _hashlib_HASH_get_blocksize desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn hashlib_openssl_HMAC_ctx_copy_with_lock(self: *mut HMACobject) -> *mut static PY_HMAC_CTX_TYPE {
    // TODO: implementar hashlib_openssl_HMAC_ctx_copy_with_lock desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _hashlib_hmac_get_block_size(op: *mut PyObject, param_64866: *mut core::ffi::c_void) -> *mut static PyObject {
    // TODO: implementar _hashlib_hmac_get_block_size desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn channel_create(channels: *mut _channels, defaults: struct _channeldefaults) -> static int64_t {
    // TODO: implementar channel_create desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn channelsmod_create(self: *mut PyObject, args: *mut PyObject, kwds: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar channelsmod_create desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _queue_lock(queue: *mut _queue) -> static int {
    // TODO: implementar _queue_lock desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _queue_unlock(queue: *mut _queue) -> static void {
    // TODO: implementar _queue_unlock desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn queue_create(queues: *mut _queues, maxsize: Py_ssize_t, defaults: struct _queuedefaults) -> static int64_t {
    // TODO: implementar queue_create desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_indent_cache(s: *mut PyEncoderObject, indent_level: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar create_indent_cache desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _encoder_iterate_mapping_lock_held(s: *mut PyEncoderObject, writer: *mut PyUnicodeWriter, first: *mut bool, dct: *mut PyObject, items: *mut PyObject, indent_level: Py_ssize_t, indent_cache: *mut PyObject, separator: *mut PyObject) -> static inline int {
    // TODO: implementar _encoder_iterate_mapping_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _encoder_iterate_dict_lock_held(s: *mut PyEncoderObject, writer: *mut PyUnicodeWriter, first: *mut bool, dct: *mut PyObject, indent_level: Py_ssize_t, indent_cache: *mut PyObject, separator: *mut PyObject) -> static inline int {
    // TODO: implementar _encoder_iterate_dict_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _encoder_iterate_fast_seq_lock_held(s: *mut PyEncoderObject, writer: *mut PyUnicodeWriter, seq: *mut PyObject, s_fast: *mut PyObject, indent_level: Py_ssize_t, indent_cache: *mut PyObject, separator: *mut PyObject) -> static inline int {
    // TODO: implementar _encoder_iterate_fast_seq_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _create_tuple_for_attribute(state: *mut _sslmodulestate, name: *mut const ASN1_OBJECT, value: *mut const ASN1_STRING) -> *mut static PyObject {
    // TODO: implementar _create_tuple_for_attribute desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _create_tuple_for_X509_NAME(state: *mut _sslmodulestate, xname: *mut const X509_NAME) -> *mut static PyObject {
    // TODO: implementar _create_tuple_for_X509_NAME desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn load_cert_chain_lock_held(self: *mut PySSLContext, pw_info: *mut _PySSLPasswordInfo, certfile_bytes: *mut PyObject, keyfile_bytes: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar load_cert_chain_lock_held desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sslmodule_init_lock(module: *mut PyObject) -> static int {
    // TODO: implementar sslmodule_init_lock desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_cfunction(self: *mut PyObject, args: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar create_cfunction desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_managed_weakref_nogc_type(self: *mut PyObject, param_45657: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar create_managed_weakref_nogc_type desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_manual_heap_type() -> *mut static PyObject {
    // TODO: implementar create_manual_heap_type desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_managed_dict_type() -> *mut static PyObject {
    // TODO: implementar create_managed_dict_type desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_interpreter(self: *mut PyObject, args: *mut PyObject, kwargs: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar create_interpreter desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createfunc_nonmodule(spec: *mut PyObject, def: *mut PyModuleDef) -> *mut static PyObject {
    // TODO: implementar createfunc_nonmodule desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_create_int_with_state() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testmultiphase_create_int_with_state desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createfunc_noop(spec: *mut PyObject, def: *mut PyModuleDef) -> *mut static PyObject {
    // TODO: implementar createfunc_noop desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_multiple_create_slots() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testmultiphase_multiple_create_slots desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createfunc_null(spec: *mut PyObject, def: *mut PyModuleDef) -> *mut static PyObject {
    // TODO: implementar createfunc_null desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_create_null() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testmultiphase_create_null desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createfunc_raise(spec: *mut PyObject, def: *mut PyModuleDef) -> *mut static PyObject {
    // TODO: implementar createfunc_raise desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_create_raise() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testmultiphase_create_raise desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createfunc_unreported_exception(spec: *mut PyObject, def: *mut PyModuleDef) -> *mut static PyObject {
    // TODO: implementar createfunc_unreported_exception desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_create_unreported_exception() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testmultiphase_create_unreported_exception desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyModule_Create(param_11926: &_testsinglephase_check_cache_first) -> return {
    // TODO: implementar PyModule_Create desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lock_acquire_parse_timeout(timeout_obj: *mut PyObject, blocking: i32, timeout: *mut PyTime_t) -> static int {
    // TODO: implementar lock_acquire_parse_timeout desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lock_repr(op: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar lock_repr desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rlock_locked_impl(self: *mut rlockobject) -> static int {
    // TODO: implementar rlock_locked_impl desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyMutex_IsLocked(param_2669: &self->lock.mutex) -> return {
    // TODO: implementar PyMutex_IsLocked desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rlock_repr(op: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar rlock_repr desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_localsdict(self: *mut localobject, state: *mut thread_module_state, param_45657: *mut PyObject, param_45657: *mut PyObject) -> static int {
    // TODO: implementar create_localsdict desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_sentinel_wr(self: *mut localobject) -> *mut static PyObject {
    // TODO: implementar create_sentinel_wr desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_localdummies(state: *mut thread_module_state) -> static int {
    // TODO: implementar create_localdummies desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Tcl_CreateFileHandler(param_34326: tfile, param_42206: mask, param_12518: FileHandler, param_22174: *mut core::ffi::c_void) -> ENTER_TCL {
    // TODO: implementar Tcl_CreateFileHandler desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn py_UuidCreate(param_45657: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar py_UuidCreate desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateBuffer(pDesc: *mut const D3D10_BUFFER_DESC, pInitialData: *mut const D3D10_SUBRESOURCE_DATA, ppBuffer: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateBuffer desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTexture1D(pDesc: *mut const D3D10_TEXTURE1D_DESC, pInitialData: *mut const D3D10_SUBRESOURCE_DATA, ppTexture1D: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateTexture1D desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTexture2D(pDesc: *mut const D3D10_TEXTURE2D_DESC, pInitialData: *mut const D3D10_SUBRESOURCE_DATA, ppTexture2D: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateTexture2D desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTexture3D(pDesc: *mut const D3D10_TEXTURE3D_DESC, pInitialData: *mut const D3D10_SUBRESOURCE_DATA, ppTexture3D: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateTexture3D desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateShaderResourceView1(pResource: *mut ID3D10Resource, pDesc: *mut const D3D10_SHADER_RESOURCE_VIEW_DESC1, ppSRView: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateShaderResourceView1 desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateInputLayout(pInputElementDescs: *mut const D3D10_INPUT_ELEMENT_DESC, NumElements: UINT, pShaderBytecodeWithInputSignature: *mut const void, BytecodeLength: SIZE_T, ppInputLayout: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateInputLayout desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVertexShader(pShaderBytecode: *mut const void, BytecodeLength: SIZE_T, ppVertexShader: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVertexShader desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateGeometryShader(pShaderBytecode: *mut const void, BytecodeLength: SIZE_T, ppGeometryShader: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateGeometryShader desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateGeometryShaderWithStreamOutput(pShaderBytecode: *mut const void, BytecodeLength: SIZE_T, pSODeclaration: *mut const D3D10_SO_DECLARATION_ENTRY, NumEntries: UINT, OutputStreamStride: UINT, ppGeometryShader: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateGeometryShaderWithStreamOutput desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreatePixelShader(pShaderBytecode: *mut const void, BytecodeLength: SIZE_T, ppPixelShader: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreatePixelShader desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateBlendState(pBlendStateDesc: *mut const D3D10_BLEND_DESC, ppBlendState: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateBlendState desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateBlendState1(pBlendStateDesc: *mut const D3D10_BLEND_DESC1, ppBlendState: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateBlendState1 desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDepthStencilState(pDepthStencilDesc: *mut const D3D10_DEPTH_STENCIL_DESC, ppDepthStencilState: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateDepthStencilState desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateRasterizerState(pRasterizerDesc: *mut const D3D10_RASTERIZER_DESC, ppRasterizerState: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateRasterizerState desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSamplerState(pSamplerDesc: *mut const D3D10_SAMPLER_DESC, ppSamplerState: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateSamplerState desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateQuery(pQueryDesc: *mut const D3D10_QUERY_DESC, ppQuery: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateQuery desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreatePredicate(pPredicateDesc: *mut const D3D10_QUERY_DESC, ppPredicate: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreatePredicate desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCounter(pCounterDesc: *mut const D3D10_COUNTER_DESC, ppCounter: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateCounter desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AcquireLock() -> D3D10DeviceLock {
    // TODO: implementar AcquireLock desde dxvk/d3d10_multithread.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LockContext() -> D3D10DeviceLock {
    // TODO: implementar LockContext desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3D10DeviceLock() -> return {
    // TODO: implementar D3D10DeviceLock desde dxvk/d3d11_context_def.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn blockDim() -> VkExtent3D {
    // TODO: implementar blockDim desde dxvk/d3d11_cuda.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTexture2D1(pDesc: *mut const D3D11_TEXTURE2D_DESC1, pInitialData: *mut const D3D11_SUBRESOURCE_DATA, ppTexture2D: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateTexture2D1 desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTexture2DBase(pDesc: *mut const D3D11_TEXTURE2D_DESC1, pInitialData: *mut const D3D11_SUBRESOURCE_DATA, ppTexture2D: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateTexture2DBase desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTexture3D1(pDesc: *mut const D3D11_TEXTURE3D_DESC1, pInitialData: *mut const D3D11_SUBRESOURCE_DATA, ppTexture3D: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateTexture3D1 desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTexture3DBase(pDesc: *mut const D3D11_TEXTURE3D_DESC1, pInitialData: *mut const D3D11_SUBRESOURCE_DATA, ppTexture3D: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateTexture3DBase desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateShaderResourceViewBase(pResource: *mut ID3D11Resource, pDesc: *mut const D3D11_SHADER_RESOURCE_VIEW_DESC1, ppSRView: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateShaderResourceViewBase desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateUnorderedAccessView1(pResource: *mut ID3D11Resource, pDesc: *mut const D3D11_UNORDERED_ACCESS_VIEW_DESC1, ppUAView: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateUnorderedAccessView1 desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateUnorderedAccessViewBase(pResource: *mut ID3D11Resource, pDesc: *mut const D3D11_UNORDERED_ACCESS_VIEW_DESC1, ppUAView: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateUnorderedAccessViewBase desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateRenderTargetView1(pResource: *mut ID3D11Resource, pDesc: *mut const D3D11_RENDER_TARGET_VIEW_DESC1, ppRTView: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateRenderTargetView1 desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateRenderTargetViewBase(pResource: *mut ID3D11Resource, pDesc: *mut const D3D11_RENDER_TARGET_VIEW_DESC1, ppRTView: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateRenderTargetViewBase desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateHullShader(pShaderBytecode: *mut const void, BytecodeLength: SIZE_T, pClassLinkage: *mut ID3D11ClassLinkage, ppHullShader: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateHullShader desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDomainShader(pShaderBytecode: *mut const void, BytecodeLength: SIZE_T, pClassLinkage: *mut ID3D11ClassLinkage, ppDomainShader: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateDomainShader desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateComputeShader(pShaderBytecode: *mut const void, BytecodeLength: SIZE_T, pClassLinkage: *mut ID3D11ClassLinkage, ppComputeShader: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateComputeShader desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateClassLinkage(ppLinkage: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateClassLinkage desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateRasterizerState1(pRasterizerDesc: *mut const D3D11_RASTERIZER_DESC1, ppRasterizerState: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateRasterizerState1 desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateRasterizerState2(pRasterizerDesc: *mut const D3D11_RASTERIZER_DESC2, ppRasterizerState: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateRasterizerState2 desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateQuery1(pQueryDesc: *mut const D3D11_QUERY_DESC1, ppQuery: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateQuery1 desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateQueryBase(pQueryDesc: *mut const D3D11_QUERY_DESC1, ppQuery: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateQueryBase desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDeferredContext(ContextFlags: UINT, ppDeferredContext: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateDeferredContext desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDeferredContext1(ContextFlags: UINT, ppDeferredContext: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateDeferredContext1 desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDeferredContext2(ContextFlags: UINT, ppDeferredContext: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateDeferredContext2 desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDeferredContext3(ContextFlags: UINT, ppDeferredContext: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateDeferredContext3 desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDeviceContextState(Flags: UINT, pFeatureLevels: *mut const D3D_FEATURE_LEVEL, FeatureLevels: UINT, SDKVersion: UINT, EmulatedInterface: REFIID, pChosenFeatureLevel: *mut D3D_FEATURE_LEVEL, ppContextState: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateDeviceContextState desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LockImage(Image: const Rc<DxvkImage>&, Usage: VkImageUsageFlags) -> bool {
    // TODO: implementar LockImage desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateShaderModule(pShaderModule: *mut D3D11CommonShader, pLinkage: *mut ID3D11ClassLinkage, ShaderKey: const DxvkShaderHash&, pShaderBytecode: *mut const void, BytecodeLength: usize, ModuleInfo: const DxvkIrShaderCreateInfo&) -> i32 {
    // TODO: implementar CreateShaderModule desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCubinComputeShaderWithNameNVX(pCubin: *mut const void, size: u32, blockX: u32, blockY: u32, blockZ: u32, pShaderName: *mut const char, phShader: *mut core::ffi::c_void) -> bool STDMETHODCALLTYPE {
    // TODO: implementar CreateCubinComputeShaderWithNameNVX desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateUnorderedAccessViewAndGetDriverHandleNVX(pResource: *mut ID3D11Resource, pDesc: *mut const D3D11_UNORDERED_ACCESS_VIEW_DESC, ppUAV: *mut core::ffi::c_void, pDriverHandle: *mut u32) -> bool STDMETHODCALLTYPE {
    // TODO: implementar CreateUnorderedAccessViewAndGetDriverHandleNVX desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateShaderResourceViewAndGetDriverHandleNVX(pResource: *mut ID3D11Resource, pDesc: *mut const D3D11_SHADER_RESOURCE_VIEW_DESC, ppSRV: *mut core::ffi::c_void, pDriverHandle: *mut u32) -> bool STDMETHODCALLTYPE {
    // TODO: implementar CreateShaderResourceViewAndGetDriverHandleNVX desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSamplerStateAndGetDriverHandleNVX(pSamplerDesc: *mut const D3D11_SAMPLER_DESC, ppSamplerState: *mut core::ffi::c_void, pDriverHandle: *mut u32) -> bool STDMETHODCALLTYPE {
    // TODO: implementar CreateSamplerStateAndGetDriverHandleNVX desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LockBuffer(Buffer: const Rc<DxvkBuffer>&) -> core::ffi::c_void {
    // TODO: implementar LockBuffer desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVideoDecoder(pDesc: *mut _In_ const D3D12_VIDEO_DECODER_DESC, riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVideoDecoder desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVideoProcessor(pEnum: *mut ID3D11VideoProcessorEnumerator, RateConversionIndex: UINT, ppVideoProcessor: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVideoProcessor desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateAuthenticatedChannel(ChannelType: D3D11_AUTHENTICATED_CHANNEL_TYPE, ppAuthenticatedChannel: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateAuthenticatedChannel desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCryptoSession(pCryptoType: *mut const GUID, pDecoderProfile: *mut const GUID, pKeyExchangeType: *mut const GUID, ppCryptoSession: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateCryptoSession desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVideoDecoderOutputView(pResource: *mut ID3D11Resource, pDesc: *mut const D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC, ppVDOVView: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVideoDecoderOutputView desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVideoProcessorInputView(pResource: *mut ID3D11Resource, pEnum: *mut ID3D11VideoProcessorEnumerator, pDesc: *mut const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC, ppVPIView: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVideoProcessorInputView desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVideoProcessorOutputView(pResource: *mut ID3D11Resource, pEnum: *mut ID3D11VideoProcessorEnumerator, pDesc: *mut const D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC, ppVPOView: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVideoProcessorOutputView desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVideoProcessorEnumerator(pDesc: *mut const D3D11_VIDEO_PROCESSOR_CONTENT_DESC, ppEnum: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVideoProcessorEnumerator desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSwapChain(pSurfaceFactory: *mut IDXGIVkSurfaceFactory, pDesc: *mut const DXGI_SWAP_CHAIN_DESC1, ppSwapChain: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateSwapChain desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSurface(pDesc: *mut const DXGI_SURFACE_DESC, NumSurfaces: UINT, Usage: DXGI_USAGE, pSharedResource: *mut const DXGI_SHARED_RESOURCE, ppSurface: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateSurface desde dxvk/d3d11_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExecuteFlushLocked() -> core::ffi::c_void {
    // TODO: implementar ExecuteFlushLocked desde dxvk/d3d11_initializer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FlushCsChunkLocked() -> core::ffi::c_void {
    // TODO: implementar FlushCsChunkLocked desde dxvk/d3d11_initializer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NotifyContextFlushLocked() -> core::ffi::c_void {
    // TODO: implementar NotifyContextFlushLocked desde dxvk/d3d11_initializer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LockSubmissionQueue() -> void STDMETHODCALLTYPE {
    // TODO: implementar LockSubmissionQueue desde dxvk/d3d11_interop.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTexture2DFromVkImage(pDesc: *mut const D3D11_TEXTURE2D_DESC1, vkImage: VkImage, ppTexture2D: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateTexture2DFromVkImage desde dxvk/d3d11_interop.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateWrappedResource(pResource12: *mut IUnknown, pResourceFlags: *mut const D3D11_RESOURCE_FLAGS, InputState: D3D12_RESOURCE_STATES, OutputState: D3D12_RESOURCE_STATES, riid: REFIID, ppResource11: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateWrappedResource desde dxvk/d3d11_on_12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LockCommandQueue(pCommandQueue: *mut ID3D12CommandQueue) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar LockCommandQueue desde dxvk/d3d11_on_12_interfaces.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnlockCommandQueue(pCommandQueue: *mut ID3D12CommandQueue) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar UnlockCommandQueue desde dxvk/d3d11_on_12_interfaces.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSubresourceSurface(index: UINT, ppSurface: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateSubresourceSurface desde dxvk/d3d11_resource.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetKeyedMutex(param_64866: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar GetKeyedMutex desde dxvk/d3d11_resource.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateIrShader(pDevice: *mut D3D11Device, ShaderKey: const DxvkShaderHash&, ModuleInfo: const DxvkIrShaderCreateInfo&, pShaderBytecode: *mut const void, BytecodeLength: usize, Icb: const D3D11ShaderIcbInfo&) -> core::ffi::c_void {
    // TODO: implementar CreateIrShader desde dxvk/d3d11_shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateFrameLatencyEvent() -> core::ffi::c_void {
    // TODO: implementar CreateFrameLatencyEvent desde dxvk/d3d11_swapchain.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreatePresenter() -> core::ffi::c_void {
    // TODO: implementar CreatePresenter desde dxvk/d3d11_swapchain.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateBackBuffers() -> core::ffi::c_void {
    // TODO: implementar CreateBackBuffers desde dxvk/d3d11_swapchain.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateBlitter() -> core::ffi::c_void {
    // TODO: implementar CreateBlitter desde dxvk/d3d11_swapchain.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateMappedBuffer(Subresource: UINT) -> core::ffi::c_void {
    // TODO: implementar CreateMappedBuffer desde dxvk/d3d11_texture.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateViewInfo(Desc: const D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC&) -> static DxvkImageViewKey {
    // TODO: implementar CreateViewInfo desde dxvk/d3d11_video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateUniformBuffer() -> core::ffi::c_void {
    // TODO: implementar CreateUniformBuffer desde dxvk/d3d11_video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateShaders() -> core::ffi::c_void {
    // TODO: implementar CreateShaders desde dxvk/d3d11_video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateResources() -> core::ffi::c_void {
    // TODO: implementar CreateResources desde dxvk/d3d11_video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Lock(OffsetToLock: UINT, SizeToLock: UINT, ppbData: *mut core::ffi::c_void, Flags: u32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar Lock desde dxvk/d3d8_batch.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Unlock() -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar Unlock desde dxvk/d3d8_batch.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVertexBuffer(Length: UINT, Usage: u32, FVF: u32, Pool: D3DPOOL) -> *mut inline D3D8BatchBuffer {
    // TODO: implementar CreateVertexBuffer desde dxvk/d3d8_batch.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateAdditionalSwapChain(pPresentationParameters: *mut D3DPRESENT_PARAMETERS, ppSwapChain: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateAdditionalSwapChain desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTexture(Width: UINT, Height: UINT, Levels: UINT, Usage: u32, Format: D3DFORMAT, Pool: D3DPOOL, ppTexture: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateTexture desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVolumeTexture(Width: UINT, Height: UINT, Depth: UINT, Levels: UINT, Usage: u32, Format: D3DFORMAT, Pool: D3DPOOL, ppVolumeTexture: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVolumeTexture desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCubeTexture(EdgeLength: UINT, Levels: UINT, Usage: u32, Format: D3DFORMAT, Pool: D3DPOOL, ppCubeTexture: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateCubeTexture desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateIndexBuffer(Length: UINT, Usage: u32, Format: D3DFORMAT, Pool: D3DPOOL, ppIndexBuffer: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateIndexBuffer desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateRenderTarget(Width: UINT, Height: UINT, Format: D3DFORMAT, MultiSample: D3DMULTISAMPLE_TYPE, Lockable: i32, ppSurface: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateRenderTarget desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDepthStencilSurface(Width: UINT, Height: UINT, Format: D3DFORMAT, MultiSample: D3DMULTISAMPLE_TYPE, ppSurface: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateDepthStencilSurface desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateImageSurface(Width: UINT, Height: UINT, Format: D3DFORMAT, ppSurface: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateImageSurface desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateStateBlock(Type: D3DSTATEBLOCKTYPE, pToken: *mut u32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateStateBlock desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CaptureStateBlock(Token: u32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CaptureStateBlock desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ApplyStateBlock(Token: u32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar ApplyStateBlock desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DeleteStateBlock(Token: u32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar DeleteStateBlock desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BeginStateBlock() -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar BeginStateBlock desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EndStateBlock(pToken: *mut u32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar EndStateBlock desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LockDevice() -> D3D8DeviceLock {
    // TODO: implementar LockDevice desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RecreateBackBuffersAndAutoDepthStencil() -> inline void {
    // TODO: implementar RecreateBackBuffersAndAutoDepthStencil desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ConvertStateBlockType(type: D3DSTATEBLOCKTYPE) -> inline D3D8StateBlockType {
    // TODO: implementar ConvertStateBlockType desde dxvk/d3d8_state_block.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LockRect(pLockedRect: *mut D3DLOCKED_RECT, pRect: *mut CONST RECT, Flags: u32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar LockRect desde dxvk/d3d8_surface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnlockRect() -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar UnlockRect desde dxvk/d3d8_surface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LockBox(Level: UINT, pLockedBox: *mut D3DLOCKED_BOX, pBox: *mut CONST D3DBOX, Flags: u32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar LockBox desde dxvk/d3d8_texture.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnlockBox(Level: UINT) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar UnlockBox desde dxvk/d3d8_texture.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IncrementLockCount() -> inline uint32_t {
    // TODO: implementar IncrementLockCount desde dxvk/d3d9_common_buffer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DecrementLockCount() -> inline uint32_t {
    // TODO: implementar DecrementLockCount desde dxvk/d3d9_common_buffer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetLockCount() -> inline uint32_t {
    // TODO: implementar GetLockCount desde dxvk/d3d9_common_buffer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSampleView(Lod: UINT) -> core::ffi::c_void {
    // TODO: implementar CreateSampleView desde dxvk/d3d9_common_texture.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetLocked(Subresource: UINT, value: bool) -> core::ffi::c_void {
    // TODO: implementar SetLocked desde dxvk/d3d9_common_texture.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetLocked(Subresource: UINT) -> bool {
    // TODO: implementar GetLocked desde dxvk/d3d9_common_texture.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsAnySubresourceLocked() -> bool {
    // TODO: implementar IsAnySubresourceLocked desde dxvk/d3d9_common_texture.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateOffscreenPlainSurface(Width: UINT, Height: UINT, Format: D3DFORMAT, Pool: D3DPOOL, ppSurface: *mut core::ffi::c_void, pSharedHandle: *mut *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateOffscreenPlainSurface desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVertexDeclaration(pVertexElements: *mut const D3DVERTEXELEMENT9, ppDecl: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVertexDeclaration desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateRenderTargetEx(Width: UINT, Height: UINT, Format: D3DFORMAT, MultiSample: D3DMULTISAMPLE_TYPE, MultisampleQuality: u32, Lockable: i32, ppSurface: *mut core::ffi::c_void, pSharedHandle: *mut *mut core::ffi::c_void, Usage: u32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateRenderTargetEx desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateOffscreenPlainSurfaceEx(Width: UINT, Height: UINT, Format: D3DFORMAT, Pool: D3DPOOL, ppSurface: *mut core::ffi::c_void, pSharedHandle: *mut *mut core::ffi::c_void, Usage: u32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateOffscreenPlainSurfaceEx desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDepthStencilSurfaceEx(Width: UINT, Height: UINT, Format: D3DFORMAT, MultiSample: D3DMULTISAMPLE_TYPE, MultisampleQuality: u32, Discard: i32, ppSurface: *mut core::ffi::c_void, pSharedHandle: *mut *mut core::ffi::c_void, Usage: u32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateDepthStencilSurfaceEx desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateAdditionalSwapChainEx(pPresentationParameters: *mut D3DPRESENT_PARAMETERS, pFullscreenDisplayMode: *mut const D3DDISPLAYMODEEX, ppSwapChain: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateAdditionalSwapChainEx desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CalcImageLockOffset(SlicePitch: u32, RowPitch: u32, FormatInfo: *mut const DxvkFormatInfo, pBox: *mut const D3DBOX) -> u32 {
    // TODO: implementar CalcImageLockOffset desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnlockImage(pResource: *mut D3D9CommonTexture, Face: UINT, MipLevel: UINT) -> i32 {
    // TODO: implementar UnlockImage desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnlockBuffer(pResource: *mut D3D9CommonBuffer) -> i32 {
    // TODO: implementar UnlockBuffer desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn HasFormatsUnlocked() -> bool {
    // TODO: implementar HasFormatsUnlocked desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetFormatAlignedBlockSize(Format: D3D9Format) -> D3D9_FORMAT_BLOCK_SIZE {
    // TODO: implementar GetFormatAlignedBlockSize desde dxvk/d3d9_format.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreatePipelineLayout() -> *mut const DxvkPipelineLayout {
    // TODO: implementar CreatePipelineLayout desde dxvk/d3d9_format_helpers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreatePipeline(size: usize, code: *mut const uint32_t, specConstant: u32) -> VkPipeline {
    // TODO: implementar CreatePipeline desde dxvk/d3d9_format_helpers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Direct3DCreate9On12(sdk_version: UINT, override_list: *mut D3D9ON12_ARGS, override_entry_count: UINT) -> *mut IDirect3D9 {
    // TODO: implementar Direct3DCreate9On12 desde dxvk/d3d9_include.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Direct3DCreate9On12Ex(sdk_version: UINT, override_list: *mut D3D9ON12_ARGS, override_entry_count: UINT, output: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar Direct3DCreate9On12Ex desde dxvk/d3d9_include.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDeviceEx(Adapter: UINT, DeviceType: D3DDEVTYPE, hFocusWindow: *mut core::ffi::c_void, BehaviorFlags: u32, pPresentationParameters: *mut D3DPRESENT_PARAMETERS, pFullscreenDisplayMode: *mut D3DDISPLAYMODEEX, ppReturnedDeviceInterface: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateDeviceEx desde dxvk/d3d9_interface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnlockDevice() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar UnlockDevice desde dxvk/d3d9_interfaces.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateImage(desc: *mut const D3D9VkExtImageDesc, ppResult: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateImage desde dxvk/d3d9_interfaces.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnlockAdditionalFormats() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar UnlockAdditionalFormats desde dxvk/d3d9_interfaces.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateTextureResource(desc: const D3D9_COMMON_TEXTURE_DESC&, ppResult: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar CreateTextureResource desde dxvk/d3d9_interop.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MapLocked(memory: *mut D3D9Memory, mappedSize: uint32_t&) -> *mut core::ffi::c_void {
    // TODO: implementar MapLocked desde dxvk/d3d9_mem.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnmapLocked(memory: *mut D3D9Memory) -> u32 {
    // TODO: implementar UnmapLocked desde dxvk/d3d9_mem.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateLegacyShader(pDevice: *mut D3D9DeviceEx, ShaderKey: const DxvkShaderHash&, ModuleInfo: const D3D9ShaderCreateInfo&, pShaderBytecode: *mut const void) -> core::ffi::c_void {
    // TODO: implementar CreateLegacyShader desde dxvk/d3d9_shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSwapChainForHwnd(pDevice: *mut IUnknown, hWnd: *mut core::ffi::c_void, pDesc: *mut const DXGI_SWAP_CHAIN_DESC1, pFullscreenDesc: *mut const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, pRestrictToOutput: *mut IDXGIOutput, ppSwapChain: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateSwapChainForHwnd desde dxvk/dxgi_factory.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSwapChainForCoreWindow(pDevice: *mut IUnknown, pWindow: *mut IUnknown, pDesc: *mut const DXGI_SWAP_CHAIN_DESC1, pRestrictToOutput: *mut IDXGIOutput, ppSwapChain: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateSwapChainForCoreWindow desde dxvk/dxgi_factory.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSwapChainForComposition(pDevice: *mut IUnknown, pDesc: *mut const DXGI_SWAP_CHAIN_DESC1, pRestrictToOutput: *mut IDXGIOutput, ppSwapChain: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateSwapChainForComposition desde dxvk/dxgi_factory.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSwapChainBase(pDevice: *mut IUnknown, hWnd: *mut core::ffi::c_void, pDesc: *mut const DXGI_SWAP_CHAIN_DESC1, pFullscreenDesc: *mut const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, pRestrictToOutput: *mut IDXGIOutput, ppSwapChain: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateSwapChainBase desde dxvk/dxgi_factory.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDummyWindow() -> *mut core::ffi::c_void {
    // TODO: implementar CreateDummyWindow desde dxvk/dxgi_surface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cfgFindBlock(types: const std::initializer_list<DxsoCfgBlockType>&) -> *mut DxsoCfgBlock {
    // TODO: implementar cfgFindBlock desde dxvk/dxso_compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createDebugName(name: *mut const char) -> string {
    // TODO: implementar createDebugName desde dxvk/dxvk_buffer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createDescriptorRange() -> bool {
    // TODO: implementar createDescriptorRange desde dxvk/dxvk_cmdlist.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createPipeline(state: const DxvkComputePipelineStateInfo&) -> VkPipeline {
    // TODO: implementar createPipeline desde dxvk/dxvk_compute.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn computePushDataBlockOffset(index: u32) -> static uint32_t {
    // TODO: implementar computePushDataBlockOffset desde dxvk/dxvk_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createDescriptorPool() -> VkDescriptorPool {
    // TODO: implementar createDescriptorPool desde dxvk/dxvk_descriptor_pool.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getBlock() -> *mut Block {
    // TODO: implementar getBlock desde dxvk/dxvk_descriptor_worker.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn flushBlock() -> *mut Block {
    // TODO: implementar flushBlock desde dxvk/dxvk_descriptor_worker.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn processBlock(block: Block&) -> core::ffi::c_void {
    // TODO: implementar processBlock desde dxvk/dxvk_descriptor_worker.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createBuiltInPipelineLayout(flags: DxvkPipelineLayoutFlags, pushDataStages: VkShaderStageFlags, pushDataSize: VkDeviceSize, bindingCount: u32, bindings: *mut const DxvkDescriptorSetLayoutBinding) -> *mut const DxvkPipelineLayout {
    // TODO: implementar createBuiltInPipelineLayout desde dxvk/dxvk_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createBuiltInComputePipeline(layout: *mut const DxvkPipelineLayout, stage: const util::DxvkBuiltInShaderStage&) -> VkPipeline {
    // TODO: implementar createBuiltInComputePipeline desde dxvk/dxvk_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createBuiltInGraphicsPipeline(layout: *mut const DxvkPipelineLayout, state: const util::DxvkBuiltInGraphicsState&) -> VkPipeline {
    // TODO: implementar createBuiltInGraphicsPipeline desde dxvk/dxvk_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lockSubmission() -> core::ffi::c_void {
    // TODO: implementar lockSubmission desde dxvk/dxvk_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unlockSubmission() -> core::ffi::c_void {
    // TODO: implementar unlockSubmission desde dxvk/dxvk_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn accumulateQueryDataForGpuQueryLocked(query: const Rc<DxvkGpuQuery>&) -> DxvkGpuQueryStatus {
    // TODO: implementar accumulateQueryDataForGpuQueryLocked desde dxvk/dxvk_gpu_query.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn accumulateQueryDataLocked() -> DxvkGpuQueryStatus {
    // TODO: implementar accumulateQueryDataLocked desde dxvk/dxvk_gpu_query.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createQueryPool() -> core::ffi::c_void {
    // TODO: implementar createQueryPool desde dxvk/dxvk_gpu_query.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn canCreateBasePipeline(state: const DxvkGraphicsPipelineStateInfo&) -> bool {
    // TODO: implementar canCreateBasePipeline desde dxvk/dxvk_graphics.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createBasePipeline(key: const DxvkGraphicsPipelineBaseInstanceKey&) -> VkPipeline {
    // TODO: implementar createBasePipeline desde dxvk/dxvk_graphics.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createOptimizedPipeline(key: const DxvkGraphicsPipelineFastInstanceKey&) -> VkPipeline {
    // TODO: implementar createOptimizedPipeline desde dxvk/dxvk_graphics.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createView(type: VkImageViewType) -> *mut const DxvkDescriptor {
    // TODO: implementar createView desde dxvk/dxvk_image.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn setKeyedMutex(mutex: Rc<DxvkKeyedMutex>&&) -> core::ffi::c_void {
    // TODO: implementar setKeyedMutex desde dxvk/dxvk_image.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getImageCreateInfo(usageInfo: const DxvkImageUsageInfo&) -> VkImageCreateInfo {
    // TODO: implementar getImageCreateInfo desde dxvk/dxvk_image.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createBufferView(key: const DxvkBufferViewKey&, baseOffset: VkDeviceSize) -> *mut const DxvkDescriptor {
    // TODO: implementar createBufferView desde dxvk/dxvk_memory.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createImageView(key: const DxvkImageViewKey&) -> *mut const DxvkDescriptor {
    // TODO: implementar createImageView desde dxvk/dxvk_memory.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createPool() -> core::ffi::c_void {
    // TODO: implementar createPool desde dxvk/dxvk_memory.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lockResourceGpuAddress(allocation: const Rc<DxvkResourceAllocation>&) -> core::ffi::c_void {
    // TODO: implementar lockResourceGpuAddress desde dxvk/dxvk_memory.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn performTimedTasksLocked(currentTime: high_resolution_clock::time_point) -> core::ffi::c_void {
    // TODO: implementar performTimedTasksLocked desde dxvk/dxvk_memory.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createPipelineLayout() -> *mut const DxvkPipelineLayout {
    // TODO: implementar createPipelineLayout desde dxvk/dxvk_meta_blit.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createCopyToImagePipeline(layout: *mut const DxvkPipelineLayout, vs: const util::DxvkBuiltInShaderStage&, ps: const util::DxvkBuiltInShaderStage&, dstFormat: VkFormat, dstAspects: VkImageAspectFlags, samples: VkSampleCountFlagBits, bitwiseStencil: bool) -> VkPipeline {
    // TODO: implementar createCopyToImagePipeline desde dxvk/dxvk_meta_copy.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createImageCopyPipeline(key: const DxvkMetaImageCopy::Key&) -> DxvkMetaImageCopy {
    // TODO: implementar createImageCopyPipeline desde dxvk/dxvk_meta_copy.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createInputAttachmentCopyPipeline(key: const DxvkMetaInputAttachmentImageCopy::Key&) -> DxvkMetaInputAttachmentImageCopy {
    // TODO: implementar createInputAttachmentCopyPipeline desde dxvk/dxvk_meta_copy.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createBufferToImageCopyPipeline(key: const DxvkMetaBufferToImageCopy::Key&) -> DxvkMetaBufferToImageCopy {
    // TODO: implementar createBufferToImageCopyPipeline desde dxvk/dxvk_meta_copy.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createImageToBufferCopyPipeline(key: const DxvkMetaImageToBufferCopy::Key&) -> DxvkMetaImageToBufferCopy {
    // TODO: implementar createImageToBufferCopyPipeline desde dxvk/dxvk_meta_copy.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createPackedImageBufferCopyPipeline(key: const DxvkMetaPackedBufferImageCopy::Key&) -> DxvkMetaPackedBufferImageCopy {
    // TODO: implementar createPackedImageBufferCopyPipeline desde dxvk/dxvk_meta_copy.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createViews(pass: u32) -> PassViews {
    // TODO: implementar createViews desde dxvk/dxvk_meta_mipgen.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getBlockOffset() -> u32 {
    // TODO: implementar getBlockOffset desde dxvk/dxvk_pipelayout.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn setBlockOffset(offset: u32) -> core::ffi::c_void {
    // TODO: implementar setBlockOffset desde dxvk/dxvk_pipelayout.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getBlockEntrySize() -> u32 {
    // TODO: implementar getBlockEntrySize desde dxvk/dxvk_pipelayout.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getPushDataBlock(index: u32) -> DxvkPushDataBlock {
    // TODO: implementar getPushDataBlock desde dxvk/dxvk_pipelayout.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createComputePipeline(shaders: const DxvkComputePipelineShaders&) -> *mut DxvkComputePipeline {
    // TODO: implementar createComputePipeline desde dxvk/dxvk_pipemanager.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createGraphicsPipeline(shaders: const DxvkGraphicsPipelineShaders&) -> *mut DxvkGraphicsPipeline {
    // TODO: implementar createGraphicsPipeline desde dxvk/dxvk_pipemanager.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createShaderPipelineLibrary(key: const DxvkShaderPipelineLibraryKey&) -> *mut DxvkShaderPipelineLibrary {
    // TODO: implementar createShaderPipelineLibrary desde dxvk/dxvk_pipemanager.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createVertexInputLibrary(state: const DxvkGraphicsPipelineVertexInputState&) -> *mut DxvkGraphicsPipelineVertexInputLibrary {
    // TODO: implementar createVertexInputLibrary desde dxvk/dxvk_pipemanager.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createFragmentOutputLibrary(state: const DxvkGraphicsPipelineFragmentOutputState&) -> *mut DxvkGraphicsPipelineFragmentOutputLibrary {
    // TODO: implementar createFragmentOutputLibrary desde dxvk/dxvk_pipemanager.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createDescriptorSetLayout(key: const DxvkDescriptorSetLayoutKey&) -> *mut const DxvkDescriptorSetLayout {
    // TODO: implementar createDescriptorSetLayout desde dxvk/dxvk_pipemanager.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createPipelineLibraryLocked(key: const DxvkShaderPipelineLibraryKey&) -> *mut DxvkShaderPipelineLibrary {
    // TODO: implementar createPipelineLibraryLocked desde dxvk/dxvk_pipemanager.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createNullFsPipelineLibrary() -> *mut DxvkShaderPipelineLibrary {
    // TODO: implementar createNullFsPipelineLibrary desde dxvk/dxvk_pipemanager.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn findPipelineLibraryLocked(key: const DxvkShaderPipelineLibraryKey&) -> *mut DxvkShaderPipelineLibrary {
    // TODO: implementar findPipelineLibraryLocked desde dxvk/dxvk_pipemanager.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn recreateSwapChain() -> VkResult {
    // TODO: implementar recreateSwapChain desde dxvk/dxvk_presenter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createSwapChain() -> VkResult {
    // TODO: implementar createSwapChain desde dxvk/dxvk_presenter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createSurface() -> VkResult {
    // TODO: implementar createSurface desde dxvk/dxvk_presenter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createLatencySemaphore() -> VkResult {
    // TODO: implementar createLatencySemaphore desde dxvk/dxvk_presenter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lockDeviceQueue() -> core::ffi::c_void {
    // TODO: implementar lockDeviceQueue desde dxvk/dxvk_queue.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unlockDeviceQueue() -> core::ffi::c_void {
    // TODO: implementar unlockDeviceQueue desde dxvk/dxvk_queue.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createSampler(index: u16, createInfo: *mut const VkSamplerCreateInfo) -> DxvkSamplerDescriptor {
    // TODO: implementar createSampler desde dxvk/dxvk_sampler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn destroyShaderPipelineLocked() -> core::ffi::c_void {
    // TODO: implementar destroyShaderPipelineLocked desde dxvk/dxvk_shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn compileShaderPipelineLocked() -> DxvkShaderPipelineLibraryHandle {
    // TODO: implementar compileShaderPipelineLocked desde dxvk/dxvk_shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn generateModuleIdentifierLocked(identifier: *mut VkShaderModuleIdentifierEXT, spirvCode: const SpirvCodeBuffer&) -> core::ffi::c_void {
    // TODO: implementar generateModuleIdentifierLocked desde dxvk/dxvk_shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn canCreatePipelineLibrary() -> bool {
    // TODO: implementar canCreatePipelineLibrary desde dxvk/dxvk_shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn canCreatePipelineLibraryForShader(shader: DxvkShader&, needsPosition: bool) -> bool {
    // TODO: implementar canCreatePipelineLibraryForShader desde dxvk/dxvk_shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn emitConditionalBlock(builder: ir::Builder&, cond: ir::SsaDef) -> SsaDef {
    // TODO: implementar emitConditionalBlock desde dxvk/dxvk_shader_builtin.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn tryInitializeLocked() -> Status {
    // TODO: implementar tryInitializeLocked desde dxvk/dxvk_shader_cache.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getShaderCreateInfo() -> DxvkIrShaderCreateInfo {
    // TODO: implementar getShaderCreateInfo desde dxvk/dxvk_shader_ir.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createHudImage(extent: VkExtent3D) -> core::ffi::c_void {
    // TODO: implementar createHudImage desde dxvk/dxvk_swapchain_blitter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createBlitPipelineLayout() -> *mut const DxvkPipelineLayout {
    // TODO: implementar createBlitPipelineLayout desde dxvk/dxvk_swapchain_blitter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createCursorPipelineLayout() -> *mut const DxvkPipelineLayout {
    // TODO: implementar createCursorPipelineLayout desde dxvk/dxvk_swapchain_blitter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createBlitPipeline(key: const DxvkSwapchainPipelineKey&) -> VkPipeline {
    // TODO: implementar createBlitPipeline desde dxvk/dxvk_swapchain_blitter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createCursorPipeline(key: const DxvkCursorPipelineKey&) -> VkPipeline {
    // TODO: implementar createCursorPipeline desde dxvk/dxvk_swapchain_blitter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn encodeClearBlockValue(format: VkFormat, color: const VkClearColorValue&) -> inline VkClearColorValue {
    // TODO: implementar encodeClearBlockValue desde dxvk/dxvk_util.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getBlockId() -> u32 {
    // TODO: implementar getBlockId desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn decorateBlock(object: u32) -> core::ffi::c_void {
    // TODO: implementar decorateBlock desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opBeginInvocationInterlock() -> core::ffi::c_void {
    // TODO: implementar opBeginInvocationInterlock desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opEndInvocationInterlock() -> core::ffi::c_void {
    // TODO: implementar opEndInvocationInterlock desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn classifyBlocks(reachableBlocks: std::unordered_set<uint32_t>&, mergeBlocks: std::unordered_set<uint32_t>&) -> core::ffi::c_void {
    // TODO: implementar classifyBlocks desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lock() -> core::ffi::c_void {
    // TODO: implementar lock desde dxvk/thread.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn try_lock() -> bool {
    // TODO: implementar try_lock desde dxvk/thread.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lock_shared() -> core::ffi::c_void {
    // TODO: implementar lock_shared desde dxvk/thread.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unlock_shared() -> core::ffi::c_void {
    // TODO: implementar unlock_shared desde dxvk/thread.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn try_lock_shared() -> bool {
    // TODO: implementar try_lock_shared desde dxvk/thread.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createDirectory(path: const std::string&) -> bool {
    // TODO: implementar createDirectory desde dxvk/util_env.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DKMTAcquireKeyedMutex(desc: *mut D3DKMT_ACQUIREKEYEDMUTEX) -> NTSTATUS {
    // TODO: implementar D3DKMTAcquireKeyedMutex desde dxvk/util_gdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DKMTCreateDCFromMemory(desc: *mut D3DKMT_CREATEDCFROMMEMORY) -> EXTERN_C WINBASEAPI NTSTATUS {
    // TODO: implementar D3DKMTCreateDCFromMemory desde dxvk/util_gdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DKMTCreateDevice(desc: *mut D3DKMT_CREATEDEVICE) -> EXTERN_C WINBASEAPI NTSTATUS {
    // TODO: implementar D3DKMTCreateDevice desde vkd3d-proton/vkd3d_d3dkmt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DKMTCreateKeyedMutex2(desc: *mut D3DKMT_CREATEKEYEDMUTEX2) -> EXTERN_C WINBASEAPI NTSTATUS {
    // TODO: implementar D3DKMTCreateKeyedMutex2 desde dxvk/util_gdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DKMTDestroyKeyedMutex(desc: *mut const D3DKMT_DESTROYKEYEDMUTEX) -> EXTERN_C WINBASEAPI NTSTATUS {
    // TODO: implementar D3DKMTDestroyKeyedMutex desde vkd3d-proton/vkd3d_d3dkmt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DKMTOpenKeyedMutex(desc: *mut D3DKMT_OPENKEYEDMUTEX) -> EXTERN_C WINBASEAPI NTSTATUS {
    // TODO: implementar D3DKMTOpenKeyedMutex desde dxvk/util_gdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DKMTReleaseKeyedMutex(desc: *mut D3DKMT_RELEASEKEYEDMUTEX) -> NTSTATUS {
    // TODO: implementar D3DKMTReleaseKeyedMutex desde dxvk/util_gdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createResources(param_60164: const) -> core::ffi::c_void {
    // TODO: implementar createResources desde dxvk/dxvk_hud_item.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createFontResources() -> core::ffi::c_void {
    // TODO: implementar createFontResources desde dxvk/dxvk_hud_renderer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn demo_create_root_signature(device: *mut ID3D12Device, desc: *mut const D3D12_ROOT_SIGNATURE_DESC, param_58615: *mut ID3D12RootSignature) -> static inline HRESULT {
    // TODO: implementar demo_create_root_signature desde vkd3d-proton/demo.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn demo_create_event() -> static inline HANDLE {
    // TODO: implementar demo_create_event desde vkd3d-proton/demo_win32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn xcb_surface_factory_create(connection: *mut xcb_connection_t, window: xcb_window_t, param_33325: *mut IDXGIVkSurfaceFactory) -> static HRESULT {
    // TODO: implementar xcb_surface_factory_create desde vkd3d-proton/demo_xcb.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_device(create_info: *mut const struct vkd3d_device_create_info, iid: REFIID, param_64866: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar vkd3d_create_device desde vkd3d-proton/vkd3d.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_lock_vk_queue(queue: *mut ID3D12CommandQueue) -> VkQueue {
    // TODO: implementar vkd3d_lock_vk_queue desde vkd3d-proton/vkd3d.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_unlock_vk_queue(queue: *mut ID3D12CommandQueue) -> core::ffi::c_void {
    // TODO: implementar vkd3d_unlock_vk_queue desde vkd3d-proton/vkd3d.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_root_signature_deserializer(data: *mut const void, data_size: SIZE_T, iid: REFIID, param_64866: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar vkd3d_create_root_signature_deserializer desde vkd3d-proton/vkd3d.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_versioned_root_signature_deserializer(data: *mut const void, data_size: SIZE_T, iid: REFIID, param_64866: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar vkd3d_create_versioned_root_signature_deserializer desde vkd3d-proton/vkd3d.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_versioned_root_signature_deserializer_for_subobject(data: *mut const void, data_size: SIZE_T, subobject_name: LPCWSTR, iid: REFIID, param_64866: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar vkd3d_create_versioned_root_signature_deserializer_for_subobject desde vkd3d-proton/vkd3d.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_event() -> static inline HANDLE {
    // TODO: implementar create_event desde vkd3d-proton/d3d12_crosstest.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_buffer_(param_61275: line, param_47516: device, param_33228: D3D12_HEAP_TYPE_READBACK, param_44127: size, param_61713: D3D12_RESOURCE_FLAG_DENY_SHADER_RESOURCE, param_13382: D3D12_RESOURCE_STATE_COPY_DEST) -> return {
    // TODO: implementar create_buffer_ desde vkd3d-proton/d3d12_test_utils.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_buffer2_(param_61275: line, param_47516: device, param_33228: D3D12_HEAP_TYPE_READBACK, param_44127: size, param_61713: D3D12_RESOURCE_FLAG_DENY_SHADER_RESOURCE) -> return {
    // TODO: implementar create_buffer2_ desde vkd3d-proton/d3d12_test_utils.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_default_texture_(param_61275: line, param_47516: device, param_28632: D3D12_RESOURCE_DIMENSION_TEXTURE2D, param_57551: width, param_41294: height, param_35932: array_size, param_60525: miplevel_count, param_40323: format, param_51689: flags, param_29774: initial_state) -> return {
    // TODO: implementar create_default_texture_ desde vkd3d-proton/d3d12_test_utils.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_default_texture_enhanced_(param_61275: line, param_47516: device, param_28632: D3D12_RESOURCE_DIMENSION_TEXTURE2D, param_57551: width, param_41294: height, param_35932: array_size, param_60525: miplevel_count, param_40323: format, param_51689: flags, param_12423: initial_layout) -> return {
    // TODO: implementar create_default_texture_enhanced_ desde vkd3d-proton/d3d12_test_utils.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_root_signature(device: *mut ID3D12Device, desc: *mut const D3D12_ROOT_SIGNATURE_DESC, param_58615: *mut ID3D12RootSignature) -> static inline HRESULT {
    // TODO: implementar create_root_signature desde vkd3d-proton/d3d12_test_utils.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_versioned_root_signature(device: *mut ID3D12Device, desc: *mut const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, param_58615: *mut ID3D12RootSignature) -> static inline HRESULT {
    // TODO: implementar create_versioned_root_signature desde vkd3d-proton/d3d12_test_utils.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_pipeline_state_from_stream_(device: *mut ID3D12Device2, stream: *mut core::ffi::c_void, size: usize, param_2840: *mut ID3D12PipelineState) -> static inline HRESULT {
    // TODO: implementar create_pipeline_state_from_stream_ desde vkd3d-proton/d3d12_test_utils.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device2_CreatePipelineState(param_47516: device, param_53125: &pipeline_desc, param_35971: &IID_ID3D12PipelineState, param_6306: *mut core::ffi::c_void) -> return {
    // TODO: implementar ID3D12Device2_CreatePipelineState desde vkd3d-proton/d3d12_test_utils.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_render_target_(line: u32, context: *mut struct test_context, desc: *mut const struct test_context_desc, param_41867: *mut ID3D12Resource, rtv: *mut const D3D12_CPU_DESCRIPTOR_HANDLE) -> static inline void {
    // TODO: implementar create_render_target_ desde vkd3d-proton/d3d12_test_utils.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_native_sync_handle_create(initial: UINT, type: enum vkd3d_native_sync_handle_type, handle: *mut vkd3d_native_sync_handle) -> static inline HRESULT {
    // TODO: implementar vkd3d_native_sync_handle_create desde vkd3d-proton/vkd3d_native_sync_handle.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn spinlock_init(lock: *mut spinlock_t) -> static inline void {
    // TODO: implementar spinlock_init desde vkd3d-proton/vkd3d_spinlock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn spinlock_try_acquire(lock: *mut spinlock_t) -> static inline bool {
    // TODO: implementar spinlock_try_acquire desde vkd3d-proton/vkd3d_spinlock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_spinlock_try_lock(param_56816: lock) -> return {
    // TODO: implementar vkd3d_spinlock_try_lock desde vkd3d-proton/vkd3d_spinlock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn spinlock_acquire(lock: *mut spinlock_t) -> static inline void {
    // TODO: implementar spinlock_acquire desde vkd3d-proton/vkd3d_spinlock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn spinlock_release(lock: *mut spinlock_t) -> static inline void {
    // TODO: implementar spinlock_release desde vkd3d-proton/vkd3d_spinlock.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rwlock_init(lock: *mut rwlock_t) -> static inline int {
    // TODO: implementar rwlock_init desde vkd3d-proton/vkd3d_threads.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rwlock_destroy(lock: *mut rwlock_t) -> static inline int {
    // TODO: implementar rwlock_destroy desde vkd3d-proton/vkd3d_threads.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DEBUG_CHANNEL_UNLOCK_MESSAGE(buf: RingBuffer, offset: uint, num_words: uint) -> core::ffi::c_void {
    // TODO: implementar DEBUG_CHANNEL_UNLOCK_MESSAGE desde vkd3d-proton/debug_channel.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_dred_settings_create(param_42613: *mut ID3D12DeviceRemovedExtendedDataSettings) -> i32 {
    // TODO: implementar d3d12_dred_settings_create desde vkd3d-proton/debug.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateGraphicsPipelineState_profiled(iface: *mut d3d12_device_iface, desc: *mut const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateGraphicsPipelineState_profiled desde vkd3d-proton/device_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateComputePipelineState_profiled(iface: *mut d3d12_device_iface, desc: *mut const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateComputePipelineState_profiled desde vkd3d-proton/device_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateDescriptorHeap_profiled(iface: *mut d3d12_device_iface, desc: *mut const D3D12_DESCRIPTOR_HEAP_DESC, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateDescriptorHeap_profiled desde vkd3d-proton/device_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateShaderResourceView_profiled(iface: *mut d3d12_device_iface, resource: *mut ID3D12Resource, desc: *mut const D3D12_SHADER_RESOURCE_VIEW_DESC, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateShaderResourceView_profiled desde vkd3d-proton/device_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateRenderTargetView_profiled(iface: *mut d3d12_device_iface, resource: *mut ID3D12Resource, desc: *mut const D3D12_RENDER_TARGET_VIEW_DESC, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateRenderTargetView_profiled desde vkd3d-proton/device_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateDepthStencilView_profiled(iface: *mut d3d12_device_iface, resource: *mut ID3D12Resource, desc: *mut const D3D12_DEPTH_STENCIL_VIEW_DESC, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateDepthStencilView_profiled desde vkd3d-proton/device_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSampler_profiled(iface: *mut d3d12_device_iface, desc: *mut const D3D12_SAMPLER_DESC, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateSampler_profiled desde vkd3d-proton/device_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSampler2_profiled(iface: *mut d3d12_device_iface, desc: *mut const D3D12_SAMPLER_DESC2, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateSampler2_profiled desde vkd3d-proton/device_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateHeap1_profiled(iface: *mut d3d12_device_iface, desc: *mut const D3D12_HEAP_DESC, protected_session: *mut ID3D12ProtectedResourceSession, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateHeap1_profiled desde vkd3d-proton/device_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateHeap_profiled(iface: *mut d3d12_device_iface, desc: *mut const D3D12_HEAP_DESC, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateHeap_profiled desde vkd3d-proton/device_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreatePipelineState_profiled(iface: *mut d3d12_device_iface, desc: *mut const D3D12_PIPELINE_STATE_STREAM_DESC, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreatePipelineState_profiled desde vkd3d-proton/device_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_timeline_semaphore(device: *mut struct d3d12_device, initial_value: u64, shared: bool, vk_semaphore: *mut VkSemaphore) -> i32 {
    // TODO: implementar vkd3d_create_timeline_semaphore desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_private_data_lock(store: *mut struct vkd3d_private_store) -> static inline HRESULT {
    // TODO: implementar vkd3d_private_data_lock desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_private_data_unlock(store: *mut struct vkd3d_private_store) -> static inline void {
    // TODO: implementar vkd3d_private_data_unlock desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_create(device: *mut struct d3d12_device, initial_value: u64, flags: D3D12_FENCE_FLAGS, param_46589: *mut struct d3d12_fence) -> i32 {
    // TODO: implementar d3d12_fence_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_create(device: *mut struct d3d12_device, initial_value: u64, flags: D3D12_FENCE_FLAGS, param_54756: *mut struct d3d12_shared_fence) -> i32 {
    // TODO: implementar d3d12_shared_fence_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_create(device: *mut struct d3d12_device, desc: *mut const D3D12_HEAP_DESC, host_address: *mut core::ffi::c_void, param_46700: *mut struct d3d12_heap) -> i32 {
    // TODO: implementar d3d12_heap_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_create_borrowed(device: *mut struct d3d12_device, desc: *mut const D3D12_RESOURCE_DESC1, vk_handle: UINT64, param_12684: *mut struct d3d12_resource) -> i32 {
    // TODO: implementar d3d12_resource_create_borrowed desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_create_committed(device: *mut struct d3d12_device, desc: *mut const D3D12_RESOURCE_DESC1, heap_properties: *mut const D3D12_HEAP_PROPERTIES, heap_flags: D3D12_HEAP_FLAGS, initial_state: D3D12_RESOURCE_STATES, optimized_clear_value: *mut const D3D12_CLEAR_VALUE, num_castable_formats: UINT, castable_formats: *mut const DXGI_FORMAT, shared_handle: *mut core::ffi::c_void, param_12684: *mut struct d3d12_resource) -> i32 {
    // TODO: implementar d3d12_resource_create_committed desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_create_placed(device: *mut struct d3d12_device, desc: *mut const D3D12_RESOURCE_DESC1, heap: *mut struct d3d12_heap, heap_offset: u64, initial_state: D3D12_RESOURCE_STATES, optimized_clear_value: *mut const D3D12_CLEAR_VALUE, num_castable_formats: UINT, castable_formats: *mut const DXGI_FORMAT, param_12684: *mut struct d3d12_resource) -> i32 {
    // TODO: implementar d3d12_resource_create_placed desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_create_reserved(device: *mut struct d3d12_device, desc: *mut const D3D12_RESOURCE_DESC1, initial_state: D3D12_RESOURCE_STATES, optimized_clear_value: *mut const D3D12_CLEAR_VALUE, num_castable_formats: UINT, castable_formats: *mut const DXGI_FORMAT, param_12684: *mut struct d3d12_resource) -> i32 {
    // TODO: implementar d3d12_resource_create_reserved desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer(device: *mut struct d3d12_device, heap_properties: *mut const D3D12_HEAP_PROPERTIES, heap_flags: D3D12_HEAP_FLAGS, desc: *mut const D3D12_RESOURCE_DESC1, tag: *mut const char, vk_buffer: *mut VkBuffer) -> i32 {
    // TODO: implementar vkd3d_create_buffer desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer_view(device: *mut struct d3d12_device, desc: *mut const struct vkd3d_buffer_view_desc, param_53268: *mut struct vkd3d_view) -> bool {
    // TODO: implementar vkd3d_create_buffer_view desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_raw_r32ui_vk_buffer_view(device: *mut struct d3d12_device, vk_buffer: VkBuffer, offset: VkDeviceSize, range: VkDeviceSize, vk_view: *mut VkBufferView) -> bool {
    // TODO: implementar vkd3d_create_raw_r32ui_vk_buffer_view desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_acceleration_structure_view(device: *mut struct d3d12_device, desc: *mut const struct vkd3d_buffer_view_desc, param_53268: *mut struct vkd3d_view) -> bool {
    // TODO: implementar vkd3d_create_acceleration_structure_view desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_opacity_micromap_view(device: *mut struct d3d12_device, desc: *mut const struct vkd3d_buffer_view_desc, param_53268: *mut struct vkd3d_view) -> bool {
    // TODO: implementar vkd3d_create_opacity_micromap_view desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_texture_view(device: *mut struct d3d12_device, desc: *mut const struct vkd3d_texture_view_desc, param_53268: *mut struct vkd3d_view) -> bool {
    // TODO: implementar vkd3d_create_texture_view desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_cbv(descriptor: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, desc: *mut const D3D12_CONSTANT_BUFFER_VIEW_DESC) -> core::ffi::c_void {
    // TODO: implementar d3d12_desc_create_cbv desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_srv(descriptor: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, desc: *mut const D3D12_SHADER_RESOURCE_VIEW_DESC) -> core::ffi::c_void {
    // TODO: implementar d3d12_desc_create_srv desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_uav(descriptor: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, counter_resource: *mut struct d3d12_resource, desc: *mut const D3D12_UNORDERED_ACCESS_VIEW_DESC) -> core::ffi::c_void {
    // TODO: implementar d3d12_desc_create_uav desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_sampler(sampler: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, desc: *mut const D3D12_SAMPLER_DESC2) -> core::ffi::c_void {
    // TODO: implementar d3d12_desc_create_sampler desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_cbv_embedded(descriptor: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, desc: *mut const D3D12_CONSTANT_BUFFER_VIEW_DESC) -> core::ffi::c_void {
    // TODO: implementar d3d12_desc_create_cbv_embedded desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_srv_embedded(descriptor: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, desc: *mut const D3D12_SHADER_RESOURCE_VIEW_DESC) -> core::ffi::c_void {
    // TODO: implementar d3d12_desc_create_srv_embedded desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_uav_embedded(descriptor: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, counter_resource: *mut struct d3d12_resource, desc: *mut const D3D12_UNORDERED_ACCESS_VIEW_DESC) -> core::ffi::c_void {
    // TODO: implementar d3d12_desc_create_uav_embedded desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_sampler_embedded(sampler: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, desc: *mut const D3D12_SAMPLER_DESC2) -> core::ffi::c_void {
    // TODO: implementar d3d12_desc_create_sampler_embedded desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_vk_buffer_view(device: *mut struct d3d12_device, vk_buffer: VkBuffer, format: *mut const struct vkd3d_format, offset: VkDeviceSize, range: VkDeviceSize, vk_view: *mut VkBufferView) -> bool {
    // TODO: implementar vkd3d_create_vk_buffer_view desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_raw_buffer_view(device: *mut struct d3d12_device, gpu_address: D3D12_GPU_VIRTUAL_ADDRESS, vk_buffer_view: *mut VkBufferView) -> bool {
    // TODO: implementar vkd3d_create_raw_buffer_view desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_create_static_sampler(device: *mut struct d3d12_device, desc: *mut const D3D12_STATIC_SAMPLER_DESC1, vk_sampler: *mut VkSampler) -> i32 {
    // TODO: implementar d3d12_create_static_sampler desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_rtv_desc_create_rtv(rtv_desc: *mut struct d3d12_rtv_desc, device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, desc: *mut const D3D12_RENDER_TARGET_VIEW_DESC) -> core::ffi::c_void {
    // TODO: implementar d3d12_rtv_desc_create_rtv desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_rtv_desc_create_dsv(dsv_desc: *mut struct d3d12_rtv_desc, device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, desc: *mut const D3D12_DEPTH_STENCIL_VIEW_DESC) -> core::ffi::c_void {
    // TODO: implementar d3d12_rtv_desc_create_dsv desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_create(device: *mut struct d3d12_device, desc: *mut const D3D12_DESCRIPTOR_HEAP_DESC, param_58976: *mut struct d3d12_descriptor_heap) -> i32 {
    // TODO: implementar d3d12_descriptor_heap_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_create(device: *mut struct d3d12_device, desc: *mut const D3D12_QUERY_HEAP_DESC, param_49628: *mut struct d3d12_query_heap) -> i32 {
    // TODO: implementar d3d12_query_heap_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_create(device: *mut struct d3d12_device, bytecode: *mut const void, bytecode_length: usize, param_32368: *mut struct d3d12_root_signature) -> i32 {
    // TODO: implementar d3d12_root_signature_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_create_raw(device: *mut struct d3d12_device, payload: *mut const void, payload_size: usize, param_32368: *mut struct d3d12_root_signature) -> i32 {
    // TODO: implementar d3d12_root_signature_create_raw desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_create_empty(device: *mut struct d3d12_device, param_32368: *mut struct d3d12_root_signature) -> i32 {
    // TODO: implementar d3d12_root_signature_create_empty desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_create_local_static_samplers_layout(root_signature: *mut struct d3d12_root_signature, vk_set_layout: VkDescriptorSetLayout, vk_pipeline_layout: *mut VkPipelineLayout) -> i32 {
    // TODO: implementar d3d12_root_signature_create_local_static_samplers_layout desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_create_work_graph_layout(root_signature: *mut struct d3d12_root_signature, vk_push_set_layout: *mut VkDescriptorSetLayout, vk_pipeline_layout: *mut VkPipelineLayout) -> i32 {
    // TODO: implementar d3d12_root_signature_create_work_graph_layout desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_pipeline_layout(device: *mut struct d3d12_device, set_layout_count: u32, set_layouts: *mut const VkDescriptorSetLayout, push_constant_count: u32, push_constants: *mut const VkPushConstantRange, pipeline_layout: *mut VkPipelineLayout) -> i32 {
    // TODO: implementar vkd3d_create_pipeline_layout desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_descriptor_set_layout(device: *mut struct d3d12_device, flags: VkDescriptorSetLayoutCreateFlags, binding_count: u32, bindings: *mut const VkDescriptorSetLayoutBinding, descriptor_buffer_flags: VkDescriptorSetLayoutCreateFlags, set_layout: *mut VkDescriptorSetLayout) -> i32 {
    // TODO: implementar vkd3d_create_descriptor_set_layout desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_vertex_input_pipeline_create(device: *mut struct d3d12_device, desc: *mut const struct vkd3d_vertex_input_pipeline_desc) -> VkPipeline {
    // TODO: implementar vkd3d_vertex_input_pipeline_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_fragment_output_pipeline_create(device: *mut struct d3d12_device, desc: *mut const struct vkd3d_fragment_output_pipeline_desc) -> VkPipeline {
    // TODO: implementar vkd3d_fragment_output_pipeline_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_create_shader_module(device: *mut struct d3d12_device, vk_module: *mut VkShaderModule, code: *mut const struct vkd3d_shader_code) -> i32 {
    // TODO: implementar d3d12_pipeline_state_create_shader_module desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_create(device: *mut struct d3d12_device, bind_point: VkPipelineBindPoint, desc: *mut const struct d3d12_pipeline_state_desc, param_22057: *mut struct d3d12_pipeline_state) -> i32 {
    // TODO: implementar d3d12_pipeline_state_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_get_or_create_pipeline(state: *mut struct d3d12_pipeline_state, dyn_state: *mut const struct vkd3d_dynamic_state, dsv_format: *mut const struct vkd3d_format, dynamic_state_flags: *mut u32) -> VkPipeline {
    // TODO: implementar d3d12_pipeline_state_get_or_create_pipeline desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_create_pipeline_variant(state: *mut struct d3d12_pipeline_state, key: *mut const struct vkd3d_pipeline_key, dsv_format: *mut const struct vkd3d_format, vk_cache: VkPipelineCache, library_flags: VkGraphicsPipelineLibraryFlagsEXT, dynamic_state_flags: *mut u32) -> VkPipeline {
    // TODO: implementar d3d12_pipeline_state_create_pipeline_variant desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_create(device: *mut struct d3d12_device, blob: *mut const void, blob_length: usize, flags: u32, param_29866: *mut struct d3d12_pipeline_library) -> i32 {
    // TODO: implementar d3d12_pipeline_library_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_pipeline_cache(device: *mut struct d3d12_device, size: usize, data: *mut const void, cache: *mut VkPipelineCache) -> VkResult {
    // TODO: implementar vkd3d_create_pipeline_cache desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_pipeline_cache_from_d3d12_desc(device: *mut struct d3d12_device, state: *mut const struct d3d12_cached_pipeline_state, cache: *mut VkPipelineCache) -> i32 {
    // TODO: implementar vkd3d_create_pipeline_cache_from_d3d12_desc desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_create(device: *mut struct d3d12_device, node_mask: UINT, type: D3D12_COMMAND_LIST_TYPE, param_49811: *mut struct d3d12_command_list) -> i32 {
    // TODO: implementar d3d12_command_list_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_create(device: *mut struct d3d12_device, node_mask: UINT, type: D3D12_COMMAND_LIST_TYPE, param_1992: *mut struct d3d12_bundle) -> i32 {
    // TODO: implementar d3d12_bundle_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_create(device: *mut struct d3d12_device, family_index: u32, queue_index: u32, properties: *mut const VkQueueFamilyProperties, param_47027: *mut struct vkd3d_queue) -> i32 {
    // TODO: implementar vkd3d_queue_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_get_signal_fence_proxy_locked(queue: *mut struct vkd3d_queue) -> VkFence {
    // TODO: implementar vkd3d_queue_get_signal_fence_proxy_locked desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_create(device: *mut struct d3d12_device, desc: *mut const D3D12_COMMAND_QUEUE_DESC, vk_family_index: u32, param_42301: *mut struct d3d12_command_queue) -> i32 {
    // TODO: implementar d3d12_command_queue_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_create(device: *mut struct d3d12_device, root_signature: *mut struct d3d12_root_signature, desc: *mut const D3D12_COMMAND_SIGNATURE_DESC, param_60073: *mut struct d3d12_command_signature) -> i32 {
    // TODO: implementar d3d12_command_signature_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_sampler_state_create_static_sampler(state: *mut struct vkd3d_sampler_state, device: *mut struct d3d12_device, desc: *mut const D3D12_STATIC_SAMPLER_DESC1, vk_sampler: *mut VkSampler) -> i32 {
    // TODO: implementar vkd3d_sampler_state_create_static_sampler desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_present_block(trace: *mut struct vkd3d_queue_timeline_trace, present_id: u64) -> struct vkd3d_queue_timeline_trace_cookie {
    // TODO: implementar vkd3d_queue_timeline_trace_register_present_block desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_complete_present_block(trace: *mut struct vkd3d_queue_timeline_trace, cookie: struct vkd3d_queue_timeline_trace_cookie) -> core::ffi::c_void {
    // TODO: implementar vkd3d_queue_timeline_trace_complete_present_block desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_create(instance: *mut struct vkd3d_instance, create_info: *mut const struct vkd3d_device_create_info, param_60588: *mut struct d3d12_device) -> i32 {
    // TODO: implementar d3d12_device_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_or_create_vertex_input_pipeline(device: *mut struct d3d12_device, desc: *mut const struct vkd3d_vertex_input_pipeline_desc) -> VkPipeline {
    // TODO: implementar d3d12_device_get_or_create_vertex_input_pipeline desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_or_create_fragment_output_pipeline(device: *mut struct d3d12_device, desc: *mut const struct vkd3d_fragment_output_pipeline_desc) -> VkPipeline {
    // TODO: implementar d3d12_device_get_or_create_fragment_output_pipeline desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d_blob_create(buffer: *mut core::ffi::c_void, size: SIZE_T, param_1031: *mut struct d3d_blob) -> i32 {
    // TODO: implementar d3d_blob_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_rt_state_object_create(device: *mut struct d3d12_device, desc: *mut const D3D12_STATE_OBJECT_DESC, parent: *mut struct d3d12_rt_state_object, param_53274: *mut struct d3d12_rt_state_object) -> i32 {
    // TODO: implementar d3d12_rt_state_object_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_create(device: *mut struct d3d12_device, desc: *mut const D3D12_STATE_OBJECT_DESC, param_39856: *mut struct d3d12_wg_state_object) -> i32 {
    // TODO: implementar d3d12_wg_state_object_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_create(device: *mut struct d3d12_device, guid: REFGUID, parameters: *mut const void, parameter_size: usize, param_56109: *mut struct d3d12_meta_command) -> i32 {
    // TODO: implementar d3d12_meta_command_create desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compute_block_count(extent: VkExtent3D, format: *mut const struct vkd3d_format) -> static inline VkExtent3D {
    // TODO: implementar vkd3d_compute_block_count desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compute_block_offset(offset: VkOffset3D, format: *mut const struct vkd3d_format) -> static inline VkOffset3D {
    // TODO: implementar vkd3d_compute_block_offset desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compute_texel_count_from_blocks(extent: VkExtent3D, format: *mut const struct vkd3d_format) -> static inline VkExtent3D {
    // TODO: implementar vkd3d_compute_texel_count_from_blocks desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compute_texel_offset_from_blocks(offset: VkOffset3D, format: *mut const struct vkd3d_format) -> static inline VkOffset3D {
    // TODO: implementar vkd3d_compute_texel_offset_from_blocks desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_view_map_create_view2(param_52231: view_map, param_47516: device, param_51202: key, param_32715: false) -> return {
    // TODO: implementar vkd3d_view_map_create_view2 desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cxg_fence_create(fence: *mut struct cxg_fence, device: *mut ID3D12Device) -> static void {
    // TODO: implementar cxg_fence_create desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cxg_mesh_create(device: *mut ID3D12Device, inner_radius: f32, outer_radius: f32, width: f32, tooth_count: u32, tooth_depth: f32, mesh: *mut struct cxg_mesh) -> static void {
    // TODO: implementar cxg_mesh_create desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cxt_fence_create(fence: *mut struct cxt_fence, device: *mut ID3D12Device) -> static void {
    // TODO: implementar cxt_fence_create desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_copy_block_compressed_texture() -> core::ffi::c_void {
    // TODO: implementar test_copy_block_compressed_texture desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device_CreateCommandList(param_22552: context->context.device, param_6097: 0, param_28654: type, param_55567: context->copy_allocator, param_50043: NULL, param_11900: &IID_ID3D12GraphicsCommandList, param_6306: *mut core::ffi::c_void) -> hr {
    // TODO: implementar ID3D12Device_CreateCommandList desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_descriptor_heap() -> core::ffi::c_void {
    // TODO: implementar test_create_descriptor_heap desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_null_descriptors() -> core::ffi::c_void {
    // TODO: implementar test_create_null_descriptors desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_sampler() -> core::ffi::c_void {
    // TODO: implementar test_create_sampler desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_sampler2() -> core::ffi::c_void {
    // TODO: implementar test_create_sampler2 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_unordered_access_view() -> core::ffi::c_void {
    // TODO: implementar test_create_unordered_access_view desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_device() -> core::ffi::c_void {
    // TODO: implementar test_create_device desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_command_list() -> core::ffi::c_void {
    // TODO: implementar test_create_command_list desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_command_queue() -> core::ffi::c_void {
    // TODO: implementar test_create_command_queue desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_command_signature() -> core::ffi::c_void {
    // TODO: implementar test_create_command_signature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn check_create_devices(factory: *mut ID3D12DeviceFactory, singleton_reference: *mut ID3D12Device) -> static void {
    // TODO: implementar check_create_devices desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_device_factory_create_device() -> core::ffi::c_void {
    // TODO: implementar test_device_factory_create_device desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn recreate_command_list_(line: u32, device: *mut ID3D12Device, allocator: *mut ID3D12CommandAllocator, param_29696: *mut ID3D12GraphicsCommandList) -> static void {
    // TODO: implementar recreate_command_list_ desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_mesh_shader_create_pipeline() -> core::ffi::c_void {
    // TODO: implementar test_mesh_shader_create_pipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_compute_pipeline_state() -> core::ffi::c_void {
    // TODO: implementar test_create_compute_pipeline_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_graphics_pipeline_state() -> core::ffi::c_void {
    // TODO: implementar test_create_graphics_pipeline_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_pipeline_state() -> core::ffi::c_void {
    // TODO: implementar test_create_pipeline_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_pipeline_with_null_root_signature() -> core::ffi::c_void {
    // TODO: implementar test_create_pipeline_with_null_root_signature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_query_heap() -> core::ffi::c_void {
    // TODO: implementar test_create_query_heap desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_acceleration_structure(context: *mut struct raytracing_test_context, inputs: *mut const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, rtas: *mut struct rt_acceleration_structure, postbuild_va: D3D12_GPU_VIRTUAL_ADDRESS) -> static void {
    // TODO: implementar create_acceleration_structure desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_committed_resource() -> core::ffi::c_void {
    // TODO: implementar test_create_committed_resource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_heap() -> core::ffi::c_void {
    // TODO: implementar test_create_heap desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_placed_resource_size() -> core::ffi::c_void {
    // TODO: implementar test_create_placed_resource_size desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_placed_resource() -> core::ffi::c_void {
    // TODO: implementar test_create_placed_resource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_reserved_resource() -> core::ffi::c_void {
    // TODO: implementar test_create_reserved_resource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device_CreateRenderTargetView(param_2088: context.device, param_50043: NULL, param_9805: h) -> hr {
    // TODO: implementar ID3D12Device_CreateRenderTargetView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_root_signature() -> core::ffi::c_void {
    // TODO: implementar test_create_root_signature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device8_CreateSamplerFeedbackUnorderedAccessView(param_63811: device8, param_55946: resource, param_14056: feedback, param_18063: *mut core::ffi::c_void) -> hr {
    // TODO: implementar ID3D12Device8_CreateSamplerFeedbackUnorderedAccessView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device_CreateShaderResourceView(param_2088: context.device, param_46773: tiled_buffer, param_41852: &srv_desc, param_64830: *mut core::ffi::c_void, param_69: gpu_heap, param_6097: 0) -> else {
    // TODO: implementar ID3D12Device_CreateShaderResourceView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_fence() -> core::ffi::c_void {
    // TODO: implementar test_create_fence desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_fence_ping_pong_deadlock_stress(use_shared: bool) -> static void {
    // TODO: implementar test_fence_ping_pong_deadlock_stress desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_fence_signal_order_deadlock_stress(use_shared: bool) -> static void {
    // TODO: implementar test_fence_signal_order_deadlock_stress desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device_CreateFence(param_2088: context.device, param_6097: 0, D3D12_FENCE_FLAG_NONE: use_shared ? D3D12_FENCE_FLAG_SHARED :, param_62236: &IID_ID3D12Fence, param_6306: *mut core::ffi::c_void) -> hr {
    // TODO: implementar ID3D12Device_CreateFence desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_fence_ping_pong_deadlock_stress_plain() -> core::ffi::c_void {
    // TODO: implementar test_fence_ping_pong_deadlock_stress_plain desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_fence_signal_order_deadlock_stress_plain() -> core::ffi::c_void {
    // TODO: implementar test_fence_signal_order_deadlock_stress_plain desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_fence_ping_pong_deadlock_stress_shared() -> core::ffi::c_void {
    // TODO: implementar test_fence_ping_pong_deadlock_stress_shared desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_fence_signal_order_deadlock_stress_shared() -> core::ffi::c_void {
    // TODO: implementar test_fence_signal_order_deadlock_stress_shared desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_clock_calibration() -> core::ffi::c_void {
    // TODO: implementar test_clock_calibration desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fake_vkCreateDevice(physical_device: VkPhysicalDevice, create_info: *mut const VkDeviceCreateInfo, allocator: *mut const VkAllocationCallbacks, device: *mut VkDevice) -> static VkResult VKAPI_CALL {
    // TODO: implementar fake_vkCreateDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkCreateDevice(param_13286: physical_device, param_49340: create_info, param_41810: allocator, param_47516: device) -> return {
    // TODO: implementar vkCreateDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn create_vulkan_image(device: *mut ID3D12Device, width: u32, height: u32, vk_format: VkFormat, usage: VkImageUsageFlags) -> static VkImage {
    // TODO: implementar create_vulkan_image desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IVKD3DCoreInterface_CreateDevice(param_960: core, param_59552: adapter, param_58854: minimum_feature_level, param_18003: iid, param_47516: device) -> return {
    // TODO: implementar IVKD3DCoreInterface_CreateDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IVKD3DCoreInterface_CreateRootSignatureDeserializer(param_960: core, param_15109: data, param_42564: data_size, param_18003: iid, param_33645: deserializer) -> return {
    // TODO: implementar IVKD3DCoreInterface_CreateRootSignatureDeserializer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IVKD3DCoreInterface_CreateVersionedRootSignatureDeserializer(param_960: core, param_15109: data, param_42564: data_size, param_18003: iid, param_33645: deserializer) -> return {
    // TODO: implementar IVKD3DCoreInterface_CreateVersionedRootSignatureDeserializer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12core_CreateDeviceFromFactory(adapter: *mut IUnknown, minimum_feature_level: D3D_FEATURE_LEVEL, factory: *mut ID3D12DeviceFactory, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12core_CreateDeviceFromFactory desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12core_CreateDevice(core: *mut d3d12core_interface, adapter: *mut IUnknown, minimum_feature_level: D3D_FEATURE_LEVEL, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12core_CreateDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_insert_hash_map_blob_locked(pipeline_library: *mut struct d3d12_pipeline_library, map: *mut struct hash_map, entry: *mut const struct vkd3d_cached_pipeline_entry) -> static bool {
    // TODO: implementar d3d12_pipeline_library_insert_hash_map_blob_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_binary_semaphore(device: *mut struct d3d12_device, vk_semaphore: *mut VkSemaphore) -> static HRESULT {
    // TODO: implementar vkd3d_create_binary_semaphore desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_add_wait_locked(queue: *mut struct vkd3d_queue, semaphore: VkSemaphore, value: u64) -> static void {
    // TODO: implementar vkd3d_queue_add_wait_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_get_or_create_fence_locked(queue: *mut struct vkd3d_queue) -> static VkFence {
    // TODO: implementar vkd3d_queue_get_or_create_fence_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_wait_find_pending_submission_locked(queue: *mut struct vkd3d_queue, timeline: u64) -> *mut static struct vkd3d_queue_pending_fence_submission {
    // TODO: implementar vkd3d_queue_wait_find_pending_submission_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_recycle_completed_fence_locked(queue: *mut struct vkd3d_queue, vk_fence: VkFence) -> static void {
    // TODO: implementar vkd3d_queue_recycle_completed_fence_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_complete_pending_submission_locked(queue: *mut struct vkd3d_queue, timeline: u64) -> static void {
    // TODO: implementar vkd3d_queue_complete_pending_submission_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_garbage_collect_obsolete_waits_locked(queue: *mut struct vkd3d_queue) -> static void {
    // TODO: implementar vkd3d_queue_garbage_collect_obsolete_waits_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_wait_submission_timeline_iterate_locked(queue: *mut struct vkd3d_queue, value: u64, timeout: u64) -> static VkResult {
    // TODO: implementar vkd3d_queue_wait_submission_timeline_iterate_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_signal_external_events_locked(fence: *mut struct d3d12_fence, worker: *mut struct vkd3d_fence_worker) -> static void {
    // TODO: implementar d3d12_fence_signal_external_events_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_block_until_pending_value_reaches_locked(fence: *mut struct d3d12_fence, pending_value: UINT64, ticket: UINT64, command_queue: *mut struct d3d12_command_queue, fence_value: *mut struct d3d12_fence_value) -> static bool {
    // TODO: implementar d3d12_fence_block_until_pending_value_reaches_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_update_pending_value_locked_and_broadcast(fence: *mut struct d3d12_fence) -> static void {
    // TODO: implementar d3d12_fence_update_pending_value_locked_and_broadcast desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_lock(fence: *mut struct d3d12_fence) -> static void {
    // TODO: implementar d3d12_fence_lock desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_unlock(fence: *mut struct d3d12_fence) -> static void {
    // TODO: implementar d3d12_fence_unlock desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_wait_until_signal_count_reaches_locked(fence: *mut struct d3d12_fence, update_count: u64) -> static void {
    // TODO: implementar d3d12_fence_wait_until_signal_count_reaches_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_update_wait_tickets_locked(fence: *mut struct d3d12_fence) -> static void {
    // TODO: implementar d3d12_fence_update_wait_tickets_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_add_pending_signal_locked(fence: *mut struct d3d12_fence, virtual_value: u64, signalling_queue: *mut const struct d3d12_command_queue) -> static uint64_t {
    // TODO: implementar d3d12_fence_add_pending_signal_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_fetch_root_parameter_uniform_block_data(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings, dst_data: *mut union vkd3d_root_parameter_data) -> static void {
    // TODO: implementar d3d12_command_list_fetch_root_parameter_uniform_block_data desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_GetClockCalibration(iface: *mut ID3D12CommandQueue, gpu_timestamp: *mut UINT64, cpu_timestamp: *mut UINT64) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_GetClockCalibration desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_needs_cpu_waits_locked(command_queue: *mut struct d3d12_command_queue) -> static bool {
    // TODO: implementar d3d12_command_queue_needs_cpu_waits_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_gather_wait_semaphores_locked(command_queue: *mut struct d3d12_command_queue, submit_info: *mut VkSubmitInfo2, wait_flags: u32) -> static void {
    // TODO: implementar d3d12_command_queue_gather_wait_semaphores_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_submit_split_locked(device: *mut struct d3d12_device, vk_queue: VkQueue, num_submits: u32, submits: *mut const VkSubmitInfo2, vk_fence: VkFence) -> static VkResult {
    // TODO: implementar d3d12_command_queue_submit_split_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_needs_staggered_submissions_locked(command_queue: *mut struct d3d12_command_queue) -> static bool {
    // TODO: implementar d3d12_command_queue_needs_staggered_submissions_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_create_vkd3d_queues(device: *mut struct d3d12_device, queue_info: *mut const struct vkd3d_device_queue_info) -> static HRESULT {
    // TODO: implementar d3d12_device_create_vkd3d_queues desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_vk_device(device: *mut struct d3d12_device, create_info: *mut const struct vkd3d_device_create_info) -> static HRESULT {
    // TODO: implementar vkd3d_create_vk_device desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_create_scratch_buffer(device: *mut struct d3d12_device, kind: enum vkd3d_scratch_pool_kind, size: VkDeviceSize, memory_types: u32, scratch: *mut struct vkd3d_scratch_buffer) -> static HRESULT {
    // TODO: implementar d3d12_device_create_scratch_buffer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_create_query_pool(device: *mut struct d3d12_device, type_index: u32, pool: *mut struct vkd3d_query_pool) -> static HRESULT {
    // TODO: implementar d3d12_device_create_query_pool desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateCommandQueue(iface: *mut d3d12_device_iface, desc: *mut const D3D12_COMMAND_QUEUE_DESC, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateCommandQueue desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateGraphicsPipelineState(iface: *mut d3d12_device_iface, desc: *mut const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateGraphicsPipelineState desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateComputePipelineState(iface: *mut d3d12_device_iface, desc: *mut const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateComputePipelineState desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateCommandList1(iface: *mut d3d12_device_iface, node_mask: UINT, type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateCommandList1 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateCommandList(iface: *mut d3d12_device_iface, node_mask: UINT, type: D3D12_COMMAND_LIST_TYPE, command_allocator: *mut ID3D12CommandAllocator, initial_pipeline_state: *mut ID3D12PipelineState, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateCommandList desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateDescriptorHeap(iface: *mut d3d12_device_iface, desc: *mut const D3D12_DESCRIPTOR_HEAP_DESC, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateDescriptorHeap desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateRootSignature(iface: *mut d3d12_device_iface, node_mask: UINT, bytecode: *mut const void, bytecode_length: SIZE_T, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateRootSignature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateShaderResourceView_embedded(iface: *mut d3d12_device_iface, resource: *mut ID3D12Resource, desc: *mut const D3D12_SHADER_RESOURCE_VIEW_DESC, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateShaderResourceView_embedded desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateShaderResourceView_default(iface: *mut d3d12_device_iface, resource: *mut ID3D12Resource, desc: *mut const D3D12_SHADER_RESOURCE_VIEW_DESC, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateShaderResourceView_default desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateUnorderedAccessView_embedded(iface: *mut d3d12_device_iface, resource: *mut ID3D12Resource, counter_resource: *mut ID3D12Resource, desc: *mut const D3D12_UNORDERED_ACCESS_VIEW_DESC, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateUnorderedAccessView_embedded desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateUnorderedAccessView_default(iface: *mut d3d12_device_iface, resource: *mut ID3D12Resource, counter_resource: *mut ID3D12Resource, desc: *mut const D3D12_UNORDERED_ACCESS_VIEW_DESC, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateUnorderedAccessView_default desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateRenderTargetView(iface: *mut d3d12_device_iface, resource: *mut ID3D12Resource, desc: *mut const D3D12_RENDER_TARGET_VIEW_DESC, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateRenderTargetView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateDepthStencilView(iface: *mut d3d12_device_iface, resource: *mut ID3D12Resource, desc: *mut const D3D12_DEPTH_STENCIL_VIEW_DESC, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateDepthStencilView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSampler_embedded(iface: *mut d3d12_device_iface, desc: *mut const D3D12_SAMPLER_DESC, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateSampler_embedded desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSampler_default(iface: *mut d3d12_device_iface, desc: *mut const D3D12_SAMPLER_DESC, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateSampler_default desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSampler2_embedded(iface: *mut d3d12_device_iface, desc: *mut const D3D12_SAMPLER_DESC2, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateSampler2_embedded desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSampler2_default(iface: *mut d3d12_device_iface, desc: *mut const D3D12_SAMPLER_DESC2, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateSampler2_default desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateCommittedResource1(iface: *mut d3d12_device_iface, heap_properties: *mut const D3D12_HEAP_PROPERTIES, heap_flags: D3D12_HEAP_FLAGS, desc: *mut const D3D12_RESOURCE_DESC, initial_state: D3D12_RESOURCE_STATES, optimized_clear_value: *mut const D3D12_CLEAR_VALUE, protected_session: *mut ID3D12ProtectedResourceSession, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateCommittedResource1 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateHeap1(iface: *mut d3d12_device_iface, desc: *mut const D3D12_HEAP_DESC, protected_session: *mut ID3D12ProtectedResourceSession, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateHeap1 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateHeap(iface: *mut d3d12_device_iface, desc: *mut const D3D12_HEAP_DESC, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateHeap desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreatePlacedResource1(iface: *mut d3d12_device_iface, heap: *mut ID3D12Heap, heap_offset: UINT64, resource_desc: *mut const D3D12_RESOURCE_DESC1, initial_state: D3D12_RESOURCE_STATES, optimized_clear_value: *mut const D3D12_CLEAR_VALUE, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreatePlacedResource1 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreatePlacedResource(iface: *mut d3d12_device_iface, heap: *mut ID3D12Heap, heap_offset: UINT64, desc: *mut const D3D12_RESOURCE_DESC, initial_state: D3D12_RESOURCE_STATES, optimized_clear_value: *mut const D3D12_CLEAR_VALUE, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreatePlacedResource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateReservedResource1(iface: *mut d3d12_device_iface, desc: *mut const D3D12_RESOURCE_DESC, initial_state: D3D12_RESOURCE_STATES, optimized_clear_value: *mut const D3D12_CLEAR_VALUE, protected_session: *mut ID3D12ProtectedResourceSession, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateReservedResource1 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSharedHandle(iface: *mut d3d12_device_iface, object: *mut ID3D12DeviceChild, attributes: *mut const SECURITY_ATTRIBUTES, access: u32, name: *mut const WCHAR, handle: *mut *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateSharedHandle desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateFence(iface: *mut d3d12_device_iface, initial_value: UINT64, flags: D3D12_FENCE_FLAGS, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateFence desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateQueryHeap(iface: *mut d3d12_device_iface, desc: *mut const D3D12_QUERY_HEAP_DESC, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateQueryHeap desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreatePipelineLibrary(iface: *mut d3d12_device_iface, blob: *mut const void, blob_size: SIZE_T, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreatePipelineLibrary desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreatePipelineState(iface: *mut d3d12_device_iface, desc: *mut const D3D12_PIPELINE_STATE_STREAM_DESC, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreatePipelineState desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateProtectedResourceSession(iface: *mut d3d12_device_iface, desc: *mut const D3D12_PROTECTED_RESOURCE_SESSION_DESC, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateProtectedResourceSession desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateCommittedResource2(iface: *mut d3d12_device_iface, heap_properties: *mut const D3D12_HEAP_PROPERTIES, heap_flags: D3D12_HEAP_FLAGS, desc: *mut const D3D12_RESOURCE_DESC1, initial_state: D3D12_RESOURCE_STATES, optimized_clear_value: *mut const D3D12_CLEAR_VALUE, protected_session: *mut ID3D12ProtectedResourceSession, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateCommittedResource2 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateMetaCommand(iface: *mut d3d12_device_iface, command_id: REFGUID, node_mask: UINT, param_data: *mut const void, param_size: SIZE_T, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateMetaCommand desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateStateObject(iface: *mut d3d12_device_iface, desc: *mut const D3D12_STATE_OBJECT_DESC, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateStateObject desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_create_sampler_feedback_desc(uav_desc: *mut D3D12_UNORDERED_ACCESS_VIEW_DESC, feedback: *mut struct d3d12_resource) -> static void {
    // TODO: implementar d3d12_device_create_sampler_feedback_desc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSamplerFeedbackUnorderedAccessView_default(iface: *mut d3d12_device_iface, target_resource: *mut ID3D12Resource, feedback_resource: *mut ID3D12Resource, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateSamplerFeedbackUnorderedAccessView_default desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSamplerFeedbackUnorderedAccessView_embedded(iface: *mut d3d12_device_iface, target_resource: *mut ID3D12Resource, feedback_resource: *mut ID3D12Resource, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateSamplerFeedbackUnorderedAccessView_embedded desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateShaderCacheSession(iface: *mut d3d12_device_iface, desc: *mut const D3D12_SHADER_CACHE_SESSION_DESC, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateShaderCacheSession desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateCommittedResource3(iface: *mut d3d12_device_iface, heap_properties: *mut const D3D12_HEAP_PROPERTIES, heap_flags: D3D12_HEAP_FLAGS, desc: *mut const D3D12_RESOURCE_DESC1, initial_layout: D3D12_BARRIER_LAYOUT, optimized_clear_value: *mut const D3D12_CLEAR_VALUE, protected_session: *mut ID3D12ProtectedResourceSession, num_castable_formats: UINT32, castable_formats: *mut const DXGI_FORMAT, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateCommittedResource3 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreatePlacedResource2(iface: *mut d3d12_device_iface, heap: *mut ID3D12Heap, heap_offset: UINT64, desc: *mut const D3D12_RESOURCE_DESC1, initial_layout: D3D12_BARRIER_LAYOUT, optimized_clear_value: *mut const D3D12_CLEAR_VALUE, num_castable_formats: UINT32, castable_formats: *mut const DXGI_FORMAT, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreatePlacedResource2 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateReservedResource2(iface: *mut d3d12_device_iface, desc: *mut const D3D12_RESOURCE_DESC, initial_layout: D3D12_BARRIER_LAYOUT, optimized_clear_value: *mut const D3D12_CLEAR_VALUE, protected_session: *mut ID3D12ProtectedResourceSession, num_castable_formats: UINT32, castable_formats: *mut const DXGI_FORMAT, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateReservedResource2 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_create_sparse_init_timeline(device: *mut struct d3d12_device) -> static HRESULT {
    // TODO: implementar d3d12_device_create_sparse_init_timeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_create_cubin_compute_shader(device: *mut struct d3d12_device, cubin_data: *mut const void, cubin_size: UINT32, block_x: UINT32, block_y: UINT32, block_z: UINT32, shader_name: *mut const char, use_64bit_texturing: bool, flags: UINT32, param_55435: *mut D3D12_CUBIN_DATA_HANDLE) -> static HRESULT {
    // TODO: implementar d3d12_device_vkd3d_ext_create_cubin_compute_shader desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_CreateResourceFromBorrowedHandle(iface: *mut d3d12_device_vkd3d_ext_iface, desc: *mut const D3D12_RESOURCE_DESC1, vk_handle: UINT64, param_41867: *mut ID3D12Resource) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_vkd3d_ext_CreateResourceFromBorrowedHandle desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_LockCommandQueue(iface: *mut d3d12_dxvk_interop_device_iface, queue: *mut ID3D12CommandQueue) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_dxvk_interop_device_LockCommandQueue desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_UnlockCommandQueue(iface: *mut d3d12_dxvk_interop_device_iface, queue: *mut ID3D12CommandQueue) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_dxvk_interop_device_UnlockCommandQueue desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_CreateInteropCommandQueue(iface: *mut d3d12_dxvk_interop_device_iface, desc: *mut const D3D12_COMMAND_QUEUE_DESC, vk_family_index: u32, param_4407: *mut ID3D12CommandQueue) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_dxvk_interop_device_CreateInteropCommandQueue desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_LockVulkanQueue(iface: *mut d3d12_dxvk_interop_device_iface, queue: *mut ID3D12CommandQueue) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_dxvk_interop_device_LockVulkanQueue desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_UnlockVulkanQueue(iface: *mut d3d12_dxvk_interop_device_iface, queue: *mut ID3D12CommandQueue) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_dxvk_interop_device_UnlockVulkanQueue desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_transfer_queue_track_resource_locked(queue: *mut struct vkd3d_memory_transfer_queue, resource: *mut struct d3d12_resource, semaphore_value: UINT64) -> static void {
    // TODO: implementar vkd3d_memory_transfer_queue_track_resource_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_transfer_queue_flush_locked(queue: *mut struct vkd3d_memory_transfer_queue) -> static HRESULT {
    // TODO: implementar vkd3d_memory_transfer_queue_flush_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_global_buffer(device: *mut struct d3d12_device, size: VkDeviceSize, heap_properties: *mut const D3D12_HEAP_PROPERTIES, heap_flags: D3D12_HEAP_FLAGS, vk_buffer: *mut VkBuffer) -> static HRESULT {
    // TODO: implementar vkd3d_create_global_buffer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_chunk_create(device: *mut struct d3d12_device, allocator: *mut struct vkd3d_memory_allocator, info: *mut const struct vkd3d_allocate_memory_info, param_24852: *mut struct vkd3d_memory_chunk) -> static HRESULT {
    // TODO: implementar vkd3d_memory_chunk_create desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_shader_module(device: *mut struct d3d12_device, code: *mut const uint32_t, code_size: usize, module: *mut VkShaderModule) -> static VkResult {
    // TODO: implementar vkd3d_meta_create_shader_module desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_descriptor_set_layout(device: *mut struct d3d12_device, binding_count: u32, bindings: *mut const VkDescriptorSetLayoutBinding, descriptor_buffer_compatible: bool, set_layout: *mut VkDescriptorSetLayout) -> static VkResult {
    // TODO: implementar vkd3d_meta_create_descriptor_set_layout desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_sampler(device: *mut struct d3d12_device, filter: VkFilter, vk_sampler: *mut VkSampler) -> static VkResult {
    // TODO: implementar vkd3d_meta_create_sampler desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_pipeline_layout(device: *mut struct d3d12_device, set_layout_count: u32, set_layouts: *mut const VkDescriptorSetLayout, push_constant_range_count: u32, push_constant_ranges: *mut const VkPushConstantRange, pipeline_layout: *mut VkPipelineLayout) -> static VkResult {
    // TODO: implementar vkd3d_meta_create_pipeline_layout desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_compute_pipeline(device: *mut struct d3d12_device, code_size: usize, code: *mut const uint32_t, layout: VkPipelineLayout, specialization_info: *mut const VkSpecializationInfo, descriptor_buffer_compatible: bool, required_size: *mut const VkPipelineShaderStageRequiredSubgroupSizeCreateInfo, pipeline: *mut VkPipeline) -> static VkResult {
    // TODO: implementar vkd3d_meta_create_compute_pipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_graphics_pipeline(meta_ops: *mut struct vkd3d_meta_ops, layout: VkPipelineLayout, color_format: VkFormat, ds_format: VkFormat, vk_aspect_mask: VkImageAspectFlags, vs_module: VkShaderModule, fs_module: VkShaderModule, samples: VkSampleCountFlagBits, ds_state: *mut const VkPipelineDepthStencilStateCreateInfo, dynamic_state_count: u32, dynamic_states: *mut const VkDynamicState, spec_info: *mut const VkSpecializationInfo, descriptor_buffer_compatible: bool, vk_pipeline: *mut VkPipeline) -> static VkResult {
    // TODO: implementar vkd3d_meta_create_graphics_pipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_swapchain_pipeline(meta_ops: *mut struct vkd3d_meta_ops, key: *mut const struct vkd3d_swapchain_pipeline_key, pipeline: *mut struct vkd3d_swapchain_pipeline) -> static HRESULT {
    // TODO: implementar vkd3d_meta_create_swapchain_pipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_copy_image_pipeline(meta_ops: *mut struct vkd3d_meta_ops, key: *mut const struct vkd3d_copy_image_pipeline_key, pipeline: *mut struct vkd3d_copy_image_pipeline) -> static HRESULT {
    // TODO: implementar vkd3d_meta_create_copy_image_pipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_resolve_image_graphics_pipeline(meta_ops: *mut struct vkd3d_meta_ops, key: *mut const struct vkd3d_resolve_image_graphics_pipeline_key, pipeline: *mut struct vkd3d_resolve_image_pipeline) -> static HRESULT {
    // TODO: implementar vkd3d_meta_create_resolve_image_graphics_pipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_resolve_image_compute_pipeline(meta_ops: *mut struct vkd3d_meta_ops, key: *mut const struct vkd3d_resolve_image_compute_pipeline_key, pipeline: *mut struct vkd3d_resolve_image_pipeline) -> static HRESULT {
    // TODO: implementar vkd3d_meta_create_resolve_image_compute_pipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_create_dstorage(meta_command: *mut struct d3d12_meta_command, device: *mut struct d3d12_device, parameter_data: *mut const void, parameter_size: usize) -> static HRESULT {
    // TODO: implementar d3d12_meta_command_create_dstorage desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_complete_blocking(trace: *mut struct vkd3d_queue_timeline_trace, cookie: struct vkd3d_queue_timeline_trace_cookie, pid: *mut const char) -> core::ffi::c_void {
    // TODO: implementar vkd3d_queue_timeline_trace_complete_blocking desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_build_group_create_info(group_create: *mut VkRayTracingShaderGroupCreateInfoKHR, group_type: VkRayTracingShaderGroupTypeKHR, export: *mut const struct d3d12_rt_state_object_identifier) -> static void {
    // TODO: implementar d3d12_state_object_build_group_create_info desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_image(device: *mut struct d3d12_device, heap_properties: *mut const D3D12_HEAP_PROPERTIES, heap_flags: D3D12_HEAP_FLAGS, desc: *mut const D3D12_RESOURCE_DESC1, resource: *mut struct d3d12_resource, num_castable_formats: UINT, castable_formats: *mut const DXGI_FORMAT, vk_image: *mut VkImage) -> static HRESULT {
    // TODO: implementar vkd3d_create_image desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_create_sampler(device: *mut struct d3d12_device, desc: *mut const D3D12_SAMPLER_DESC2, vk_sampler: *mut VkSampler) -> static HRESULT {
    // TODO: implementar d3d12_create_sampler desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_sampler_state_create_descriptor_pool(device: *mut struct d3d12_device, vk_pool: *mut VkDescriptorPool) -> static VkResult {
    // TODO: implementar vkd3d_sampler_state_create_descriptor_pool desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_validate_create_info(desc: *mut const D3D12_RESOURCE_DESC1, heap_properties: *mut const D3D12_HEAP_PROPERTIES, initial_state: D3D12_RESOURCE_STATES, optimized_clear_value: *mut const D3D12_CLEAR_VALUE, num_castable_formats: UINT, castable_formats: *mut const DXGI_FORMAT, device: *mut struct d3d12_device) -> static HRESULT {
    // TODO: implementar d3d12_resource_validate_create_info desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_create_vk_resource(resource: *mut struct d3d12_resource, num_castable_formats: UINT, castable_formats: *mut const DXGI_FORMAT, device: *mut struct d3d12_device) -> static HRESULT {
    // TODO: implementar d3d12_resource_create_vk_resource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_create_reserved_fallback(device: *mut struct d3d12_device, desc: *mut const D3D12_RESOURCE_DESC1, initial_state: D3D12_RESOURCE_STATES, optimized_clear_value: *mut const D3D12_CLEAR_VALUE, num_castable_formats: UINT, castable_formats: *mut const DXGI_FORMAT, param_12684: *mut struct d3d12_resource) -> static HRESULT {
    // TODO: implementar d3d12_resource_create_reserved_fallback desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer_view_for_resource(device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, view_format: DXGI_FORMAT, offset: VkDeviceSize, size: VkDeviceSize, structure_stride: VkDeviceSize, flags: u32, param_53268: *mut struct vkd3d_view) -> static bool {
    // TODO: implementar vkd3d_create_buffer_view_for_resource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer_srv_embedded(desc_va: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, desc: *mut const D3D12_SHADER_RESOURCE_VIEW_DESC) -> static void {
    // TODO: implementar vkd3d_create_buffer_srv_embedded desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer_srv(desc_va: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, desc: *mut const D3D12_SHADER_RESOURCE_VIEW_DESC) -> static void {
    // TODO: implementar vkd3d_create_buffer_srv desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_view_map_create_view(param_51820: &resource->view_map, param_47516: device, param_31292: &key) -> return {
    // TODO: implementar vkd3d_view_map_create_view desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_texture_srv_embedded(desc_va: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, desc: *mut const D3D12_SHADER_RESOURCE_VIEW_DESC) -> static void {
    // TODO: implementar vkd3d_create_texture_srv_embedded desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_texture_srv(desc_va: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, desc: *mut const D3D12_SHADER_RESOURCE_VIEW_DESC) -> static void {
    // TODO: implementar vkd3d_create_texture_srv desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer_uav_embedded(desc_va: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, counter_resource: *mut struct d3d12_resource, desc: *mut const D3D12_UNORDERED_ACCESS_VIEW_DESC) -> static void {
    // TODO: implementar vkd3d_create_buffer_uav_embedded desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer_uav(desc_va: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, counter_resource: *mut struct d3d12_resource, desc: *mut const D3D12_UNORDERED_ACCESS_VIEW_DESC) -> static void {
    // TODO: implementar vkd3d_create_buffer_uav desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_texture_uav_embedded(desc_va: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, desc: *mut const D3D12_UNORDERED_ACCESS_VIEW_DESC) -> static void {
    // TODO: implementar vkd3d_create_texture_uav_embedded desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_texture_uav(desc_va: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, desc: *mut const D3D12_UNORDERED_ACCESS_VIEW_DESC) -> static void {
    // TODO: implementar vkd3d_create_texture_uav desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_create_descriptor_buffer(descriptor_heap: *mut struct d3d12_descriptor_heap) -> static HRESULT {
    // TODO: implementar d3d12_descriptor_heap_create_descriptor_buffer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_create_descriptor_pool(descriptor_heap: *mut struct d3d12_descriptor_heap, vk_descriptor_pool: *mut VkDescriptorPool) -> static HRESULT {
    // TODO: implementar d3d12_descriptor_heap_create_descriptor_pool desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_create_descriptor_set(descriptor_heap: *mut struct d3d12_descriptor_heap, binding: *mut const struct vkd3d_bindless_set_info, vk_descriptor_set: *mut VkDescriptorSet) -> static HRESULT {
    // TODO: implementar d3d12_descriptor_heap_create_descriptor_set desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_pipeline_layout_for_stage_mask(device: *mut struct d3d12_device, set_layout_count: u32, set_layouts: *mut const VkDescriptorSetLayout, push_constants: *mut const VkPushConstantRange, stages: VkShaderStageFlags, bind_point_layout: *mut struct d3d12_bind_point_layout) -> static HRESULT {
    // TODO: implementar vkd3d_create_pipeline_layout_for_stage_mask desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_create_from_blob(device: *mut struct d3d12_device, bytecode: *mut const void, bytecode_length: usize, raw_payload: bool, param_32368: *mut struct d3d12_root_signature) -> static HRESULT {
    // TODO: implementar d3d12_root_signature_create_from_blob desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_stages_require_work_locked(state: *mut struct d3d12_pipeline_state) -> static bool {
    // TODO: implementar vkd3d_shader_stages_require_work_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_compute_pipeline(state: *mut struct d3d12_pipeline_state, device: *mut struct d3d12_device, code: *mut const D3D12_SHADER_BYTECODE) -> static HRESULT {
    // TODO: implementar vkd3d_create_compute_pipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_graphics_create_shader_stages(state: *mut struct d3d12_pipeline_state, device: *mut struct d3d12_device, desc: *mut const struct d3d12_pipeline_state_desc) -> static HRESULT {
    // TODO: implementar d3d12_pipeline_state_graphics_create_shader_stages desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_init_graphics_create_info(state: *mut struct d3d12_pipeline_state, device: *mut struct d3d12_device, desc: *mut const struct d3d12_pipeline_state_desc) -> static HRESULT {
    // TODO: implementar d3d12_pipeline_state_init_graphics_create_info desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_create_private_root_signature(device: *mut struct d3d12_device, bind_point: VkPipelineBindPoint, desc: *mut const struct d3d12_pipeline_state_desc, param_32368: *mut struct d3d12_root_signature) -> static HRESULT {
    // TODO: implementar d3d12_pipeline_create_private_root_signature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_update_formats_locked(chain: *mut struct dxgi_vk_swap_chain, force_requery: bool) -> static bool {
    // TODO: implementar dxgi_vk_swap_chain_update_formats_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_create_surface(chain: *mut struct dxgi_vk_swap_chain, pFactory: *mut IDXGIVkSurfaceFactory) -> static HRESULT {
    // TODO: implementar dxgi_vk_swap_chain_create_surface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_find_compatible_unlocked_present_mode(chain: *mut struct dxgi_vk_swap_chain, vk_present_mode: *mut VkPresentModeKHR, vk_min_image_count: *mut u32) -> static bool {
    // TODO: implementar dxgi_vk_swap_chain_find_compatible_unlocked_present_mode desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_recreate_swapchain_in_present_task(chain: *mut struct dxgi_vk_swap_chain) -> static void {
    // TODO: implementar dxgi_vk_swap_chain_recreate_swapchain_in_present_task desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_factory_CreateSwapChain(iface: *mut IDXGIVkSwapChainFactory, pFactory: *mut IDXGIVkSurfaceFactory, pDesc: *mut const DXGI_SWAP_CHAIN_DESC1, param_29194: *mut IDXGIVkSwapChain) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar dxgi_vk_swap_chain_factory_CreateSwapChain desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_wait_available_submit_locked(profiler: *mut struct vkd3d_timestamp_profiler, timeline: u64, num_timestamps: usize) -> static void {
    // TODO: implementar vkd3d_timestamp_profiler_wait_available_submit_locked desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_create_query_pool(device: *mut struct d3d12_device, type: VkQueryType, count: u32, pipeline_statistics: VkQueryPipelineStatisticFlags) -> static VkQueryPool {
    // TODO: implementar vkd3d_timestamp_profiler_create_query_pool desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_va_map_get_block_address(va: VkDeviceAddress) -> static inline VkDeviceAddress {
    // TODO: implementar vkd3d_va_map_get_block_address desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSharedResource(pHeapProperties: *mut _In_ const D3D12_HEAP_PROPERTIES, HeapFlags: D3D12_HEAP_FLAGS, pDesc: *mut _In_ const D3D12_RESOURCE_DESC, InitialResourceState: D3D12_RESOURCE_STATES, pOptimizedClearValue: *mut _In_opt_ const D3D12_CLEAR_VALUE, pFlags11: *mut _In_opt_ const D3D11_RESOURCE_FLAGS, CompatibilityFlags: D3D12_COMPATIBILITY_SHARED_FLAGS, pLifetimeTracker: *mut _In_opt_ ID3D12LifetimeTracker, pOwningSwapchain: *mut _In_opt_ ID3D12SwapChainAssistant, riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateSharedResource desde DirectX-Headers/d3d12compatibility.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSharedHeap(pHeapDesc: *mut _In_ const D3D12_HEAP_DESC, CompatibilityFlags: D3D12_COMPATIBILITY_SHARED_FLAGS, riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateSharedHeap desde DirectX-Headers/d3d12compatibility.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3D12CompilerCreateFactory(pPluginCompilerDllPath: _In_ LPCWSTR, riid: _In_ REFIID, ppFactory: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar D3D12CompilerCreateFactory desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCompiler(pCompilerCacheSession: *mut _In_ ID3D12CompilerCacheSession, riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateCompiler desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVideoDecoderHeap(pVideoDecoderHeapDesc: *mut _In_ const D3D12_VIDEO_DECODER_HEAP_DESC, riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVideoDecoderHeap desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVideoMotionEstimator(pDesc: *mut _In_ const D3D12_VIDEO_MOTION_ESTIMATOR_DESC, pProtectedResourceSession: *mut _In_opt_ ID3D12ProtectedResourceSession, riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVideoMotionEstimator desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVideoMotionVectorHeap(pDesc: *mut _In_ const D3D12_VIDEO_MOTION_VECTOR_HEAP_DESC, pProtectedResourceSession: *mut _In_opt_ ID3D12ProtectedResourceSession, riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVideoMotionVectorHeap desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVideoDecoder1(pDesc: *mut _In_ const D3D12_VIDEO_DECODER_DESC, pProtectedResourceSession: *mut _In_opt_ ID3D12ProtectedResourceSession, riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVideoDecoder1 desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVideoDecoderHeap1(pVideoDecoderHeapDesc: *mut _In_ const D3D12_VIDEO_DECODER_HEAP_DESC, pProtectedResourceSession: *mut _In_opt_ ID3D12ProtectedResourceSession, riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVideoDecoderHeap1 desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVideoEncoder(pDesc: *mut _In_ const D3D12_VIDEO_ENCODER_DESC, riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVideoEncoder desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVideoEncoderHeap(pDesc: *mut _In_ const D3D12_VIDEO_ENCODER_HEAP_DESC, riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVideoEncoderHeap desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVideoEncoderHeap1(pDesc: *mut _In_ const D3D12_VIDEO_ENCODER_HEAP_DESC1, riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVideoEncoderHeap1 desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateInstaller(pClient: *mut _In_ ID3DShaderCacheInstallerClient, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateInstaller desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnalignedBlockTexturesSupported() -> i32 {
    // TODO: implementar UnalignedBlockTexturesSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateByteOffsetViewsSupported() -> i32 {
    // TODO: implementar CreateByteOffsetViewsSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsBlockCompressFormat(Format: DXGI_FORMAT) -> static bool {
    // TODO: implementar IsBlockCompressFormat desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSubobject() -> *mut T {
    // TODO: implementar CreateSubobject desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetFrontCounterClockwise(frontCounterClockwise: i32) -> core::ffi::c_void {
    // TODO: implementar SetFrontCounterClockwise desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateNode() -> *mut T {
    // TODO: implementar CreateNode desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateShaderNode(nullptr: LPCWSTR Shader =) -> *mut CD3DX12_SHADER_NODE {
    // TODO: implementar CreateShaderNode desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateBroadcastingLaunchNodeOverrides(nullptr: LPCWSTR Shader =) -> *mut CD3DX12_BROADCASTING_LAUNCH_NODE_OVERRIDES {
    // TODO: implementar CreateBroadcastingLaunchNodeOverrides desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCoalescingLaunchNodeOverrides(nullptr: LPCWSTR Shader =) -> *mut CD3DX12_COALESCING_LAUNCH_NODE_OVERRIDES {
    // TODO: implementar CreateCoalescingLaunchNodeOverrides desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCommonComputeNodeOverrides(nullptr: LPCWSTR Shader =) -> *mut CD3DX12_COMMON_COMPUTE_NODE_OVERRIDES {
    // TODO: implementar CreateCommonComputeNodeOverrides desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DMLCreateDevice(d3d12Device: *mut ID3D12Device, flags: DML_CREATE_DEVICE_FLAGS, riid: REFIID, ppv: *mut core::ffi::c_void) -> STDAPI {
    // TODO: implementar DMLCreateDevice desde DirectX-Headers/DirectML.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DMLCreateDevice1(d3d12Device: *mut ID3D12Device, flags: DML_CREATE_DEVICE_FLAGS, minimumFeatureLevel: DML_FEATURE_LEVEL, riid: REFIID, ppv: *mut core::ffi::c_void) -> STDAPI {
    // TODO: implementar DMLCreateDevice1 desde DirectX-Headers/DirectML.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateAdapterListByWorkload(workload: DXCoreWorkload, runtimeFilter: DXCoreRuntimeFilterFlags, hardwareTypeFilter: DXCoreHardwareTypeFilterFlags, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateAdapterListByWorkload desde DirectX-Headers/dxcore_interface.h
    core::ptr::null_mut()
}
