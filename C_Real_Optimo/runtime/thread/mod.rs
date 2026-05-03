//! ADead Runtime - THREAD Module
//!
//! Funciones generadas automáticamente desde knowledge.json
//! Categoría: thread

#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

/// CreateSecurityPage - from wine/aclui.h
#[no_mangle]
pub unsafe extern "C" fn CreateSecurityPage(psi: usize) -> usize {
    0
}

/// AtlAxCreateControl - from wine/atlbase.h
#[no_mangle]
pub unsafe extern "C" fn AtlAxCreateControl(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// AtlAxCreateControlEx - from wine/atlbase.h
#[no_mangle]
pub unsafe extern "C" fn AtlAxCreateControlEx(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut *mut core::ffi::c_void, arg4: *mut *mut core::ffi::c_void, arg5: usize, arg6: *mut core::ffi::c_void) -> i32 {
    0
}

/// AtlModuleAddCreateWndData - from wine/atlbase.h
#[no_mangle]
pub unsafe extern "C" fn AtlModuleAddCreateWndData(pM: *mut core::ffi::c_void, pData: *mut core::ffi::c_void, pvObject: *mut core::ffi::c_void) {

}

/// AtlWinModuleAddCreateWndData - from wine/atlbase.h
#[no_mangle]
pub unsafe extern "C" fn AtlWinModuleAddCreateWndData(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) {

}

/// AtlWinModuleExtractCreateWndData - from wine/atlbase.h
#[no_mangle]
pub unsafe extern "C" fn AtlWinModuleExtractCreateWndData(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// AtlModuleExtractCreateWndData - from wine/atlbase.h
#[no_mangle]
pub unsafe extern "C" fn AtlModuleExtractCreateWndData(pM: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// AtlCreateRegistrar - from wine/atlbase.h
#[no_mangle]
pub unsafe extern "C" fn AtlCreateRegistrar(arg0: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// AtlCreateTargetDC - from wine/atlwin.h
#[no_mangle]
pub unsafe extern "C" fn AtlCreateTargetDC(hdc: *mut core::ffi::c_void, ptd: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// BCryptCreateHash - from wine/bcrypt.h
#[no_mangle]
pub unsafe extern "C" fn BCryptCreateHash(arg0: usize, arg1: *mut core::ffi::c_void, arg2: usize, arg3: u32, arg4: usize, arg5: u32, arg6: u32) -> i32 {
    0
}

/// CM_Create_DevNode_ExA - from wine/cfgmgr32.h
#[no_mangle]
pub unsafe extern "C" fn CM_Create_DevNode_ExA(node: *mut core::ffi::c_void, instance_id: usize, parent: usize, flags: u32, machine: usize) -> usize {
    0
}

/// CM_Create_DevNode_ExW - from wine/cfgmgr32.h
#[no_mangle]
pub unsafe extern "C" fn CM_Create_DevNode_ExW(node: *mut core::ffi::c_void, instance_id: usize, parent: usize, flags: u32, machine: usize) -> usize {
    0
}

/// CM_Create_DevNodeA - from wine/cfgmgr32.h
#[no_mangle]
pub unsafe extern "C" fn CM_Create_DevNodeA(node: *mut core::ffi::c_void, instance_id: usize, parent: usize, flags: u32) -> usize {
    0
}

/// CM_Create_DevNodeW - from wine/cfgmgr32.h
#[no_mangle]
pub unsafe extern "C" fn CM_Create_DevNodeW(node: *mut core::ffi::c_void, instance_id: usize, parent: usize, flags: u32) -> usize {
    0
}

/// CM_Create_Range_List - from wine/cfgmgr32.h
#[no_mangle]
pub unsafe extern "C" fn CM_Create_Range_List(ranges: *mut core::ffi::c_void, flags: u32) -> usize {
    0
}

/// CreateStatusWindowA - from wine/commctrl.h
#[no_mangle]
pub unsafe extern "C" fn CreateStatusWindowA(arg0: i32, arg1: *const i8, arg2: *mut core::ffi::c_void, arg3: u32) -> usize {
    0
}

/// CreateStatusWindowW - from wine/commctrl.h
#[no_mangle]
pub unsafe extern "C" fn CreateStatusWindowW(arg0: i32, arg1: *const u16, arg2: *mut core::ffi::c_void, arg3: u32) -> usize {
    0
}

/// CreateUpDownControl - from wine/commctrl.h
#[no_mangle]
pub unsafe extern "C" fn CreateUpDownControl(arg0: u32, arg1: i32, arg2: i32, arg3: i32, arg4: i32, arg5: *mut core::ffi::c_void, arg6: i32, arg7: *mut core::ffi::c_void, arg8: *mut core::ffi::c_void, arg9: i32, arg10: i32, arg11: i32) -> usize {
    0
}

/// ImageList_Create - from wine/commctrl.h
#[no_mangle]
pub unsafe extern "C" fn ImageList_Create(arg0: i32, arg1: i32, arg2: u32, arg3: i32, arg4: i32) -> usize {
    0
}

/// ImageList_DragShowNolock - from wine/commctrl.h
#[no_mangle]
pub unsafe extern "C" fn ImageList_DragShowNolock(arg0: i32) -> usize {
    0
}

/// CreateToolbar - from reactos/appview.h
#[no_mangle]
pub unsafe extern "C" fn CreateToolbar() -> i32 {
    0
}

/// CreateToolbarEx - from wine/commctrl.h
#[no_mangle]
pub unsafe extern "C" fn CreateToolbarEx(arg0: *mut core::ffi::c_void, arg1: u32, arg2: u32, arg3: i32, arg4: *mut core::ffi::c_void, arg5: usize, arg6: usize, arg7: i32, arg8: i32, arg9: i32, arg10: i32, arg11: i32, arg12: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateMappedBitmap - from wine/commctrl.h
#[no_mangle]
pub unsafe extern "C" fn CreateMappedBitmap(arg0: *mut core::ffi::c_void, arg1: usize, arg2: u32, arg3: usize, arg4: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// DSA_Create - from wine/commctrl.h
#[no_mangle]
pub unsafe extern "C" fn DSA_Create(arg0: i32, arg1: i32) -> usize {
    0
}

/// DPA_Create - from wine/commctrl.h
#[no_mangle]
pub unsafe extern "C" fn DPA_Create(arg0: i32) -> usize {
    0
}

/// CreateCompressor - from wine/compressapi.h
#[no_mangle]
pub unsafe extern "C" fn CreateCompressor(arg0: u32, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// CreateDecompressor - from wine/compressapi.h
#[no_mangle]
pub unsafe extern "C" fn CreateDecompressor(arg0: u32, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// CreatePseudoConsole - from wine/consoleapi.h
#[no_mangle]
pub unsafe extern "C" fn CreatePseudoConsole(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: u32, arg4: *mut core::ffi::c_void) -> usize {
    0
}

/// D3D10CreateDevice - from wine/d3d10misc.h
#[no_mangle]
pub unsafe extern "C" fn D3D10CreateDevice(adapter: *mut core::ffi::c_void, driver_type: usize, swrast: *mut core::ffi::c_void, flags: u32, sdk_version: u32, device: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3D10CreateDeviceAndSwapChain - from wine/d3d10misc.h
#[no_mangle]
pub unsafe extern "C" fn D3D10CreateDeviceAndSwapChain(adapter: *mut core::ffi::c_void, driver_type: usize, swrast: *mut core::ffi::c_void, flags: u32, sdk_version: u32, swapchain_desc: *mut core::ffi::c_void, swapchain: *mut *mut core::ffi::c_void, device: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3D10CreateBlob - from wine/d3d10misc.h
#[no_mangle]
pub unsafe extern "C" fn D3D10CreateBlob(data_size: usize, blob: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// Direct3DCreate8 - from wine/d3d8.h
#[no_mangle]
pub unsafe extern "C" fn Direct3DCreate8(SDKVersion: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// D3DCreateBlob - from wine/d3dcompiler.h
#[no_mangle]
pub unsafe extern "C" fn D3DCreateBlob(data_size: usize, blob: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DCreateFunctionLinkingGraph - from wine/d3dcompiler.h
#[no_mangle]
pub unsafe extern "C" fn D3DCreateFunctionLinkingGraph(flags: u32, graph: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DCreateLinker - from wine/d3dcompiler.h
#[no_mangle]
pub unsafe extern "C" fn D3DCreateLinker(linker: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// Direct3DRMCreate - from wine/d3drm.h
#[no_mangle]
pub unsafe extern "C" fn Direct3DRMCreate(d3drm: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DRMCreateColorRGB - from wine/d3drmdef.h
#[no_mangle]
pub unsafe extern "C" fn D3DRMCreateColorRGB(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// D3DRMCreateColorRGBA - from wine/d3drmdef.h
#[no_mangle]
pub unsafe extern "C" fn D3DRMCreateColorRGBA(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// D3DX10CreateEffectFromFileA - from wine/d3dx10async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectFromFileA(filename: *mut i8, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, profile: *mut i8, hlslflags: u32, fxflags: u32, device: *mut core::ffi::c_void, effectpool: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, effect: *mut *mut core::ffi::c_void, errors: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX10CreateEffectFromFileW - from wine/d3dx10async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectFromFileW(filename: *mut u16, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, profile: *mut i8, hlslflags: u32, fxflags: u32, device: *mut core::ffi::c_void, effectpool: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, effect: *mut *mut core::ffi::c_void, errors: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX10CreateEffectFromMemory - from wine/d3dx10async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectFromMemory(data: *mut core::ffi::c_void, datasize: usize, filename: *mut i8, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, profile: *mut i8, hlslflags: u32, fxflags: u32, device: *mut core::ffi::c_void, effectpool: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, effect: *mut *mut core::ffi::c_void, errors: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX10CreateEffectPoolFromFileA - from wine/d3dx10async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectPoolFromFileA(filename: *mut i8, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, profile: *mut i8, hlslflags: u32, fxflags: u32, device: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, effectpool: *mut *mut core::ffi::c_void, errors: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX10CreateEffectPoolFromFileW - from wine/d3dx10async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectPoolFromFileW(filename: *mut u16, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, profile: *mut i8, hlslflags: u32, fxflags: u32, device: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, effectpool: *mut *mut core::ffi::c_void, errors: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX10CreateEffectPoolFromMemory - from wine/d3dx10async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectPoolFromMemory(data: *mut core::ffi::c_void, datasize: usize, filename: *mut i8, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, profile: *mut i8, hlslflags: u32, fxflags: u32, device: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, effectpool: *mut *mut core::ffi::c_void, errors: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX10CreateEffectFromResourceA - from wine/d3dx10async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectFromResourceA(module: *mut core::ffi::c_void, resource_name: *mut i8, filename: *mut i8, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, profile: *mut i8, shader_flags: u32, effect_flags: u32, device: *mut core::ffi::c_void, effect_pool: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, effect: *mut *mut core::ffi::c_void, errors: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX10CreateEffectFromResourceW - from wine/d3dx10async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateEffectFromResourceW(module: *mut core::ffi::c_void, resource_name: *mut u16, filename: *mut u16, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, profile: *mut i8, shader_flags: u32, effect_flags: u32, device: *mut core::ffi::c_void, effect_pool: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, effect: *mut *mut core::ffi::c_void, errors: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX10CreateAsyncFileLoaderW - from wine/d3dx10async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateAsyncFileLoaderW(filename: *mut u16, loader: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX10CreateAsyncFileLoaderA - from wine/d3dx10async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateAsyncFileLoaderA(filename: *mut i8, loader: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX10CreateAsyncMemoryLoader - from wine/d3dx10async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateAsyncMemoryLoader(data: *mut core::ffi::c_void, datasize: usize, loader: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX10CreateAsyncResourceLoaderA - from wine/d3dx10async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateAsyncResourceLoaderA(module: *mut core::ffi::c_void, resource: *mut i8, loader: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX10CreateAsyncResourceLoaderW - from wine/d3dx10async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateAsyncResourceLoaderW(module: *mut core::ffi::c_void, resource: *mut u16, loader: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX10CreateAsyncTextureProcessor - from wine/d3dx10async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateAsyncTextureProcessor(device: *mut core::ffi::c_void, info: *mut core::ffi::c_void, processor: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX10CreateAsyncTextureInfoProcessor - from wine/d3dx10async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateAsyncTextureInfoProcessor(info: *mut core::ffi::c_void, processor: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX10CreateDevice - from wine/d3dx10core.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateDevice(adapter: *mut core::ffi::c_void, driver_type: usize, swrast: *mut core::ffi::c_void, flags: u32, device: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX10CreateDeviceAndSwapChain - from wine/d3dx10core.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateDeviceAndSwapChain(adapter: *mut core::ffi::c_void, driver_type: usize, swrast: *mut core::ffi::c_void, flags: u32, desc: *mut core::ffi::c_void, swapchain: *mut *mut core::ffi::c_void, device: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX10CreateFontIndirectA - from wine/d3dx10core.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateFontIndirectA(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, font: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX10CreateFontIndirectW - from wine/d3dx10core.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateFontIndirectW(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, font: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX10CreateFontA - from wine/d3dx10core.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateFontA(device: *mut core::ffi::c_void, height: i32, width: u32, weight: u32, miplevels: u32, italic: i32, charset: u32, precision: u32, quality: u32, pitchandfamily: u32, facename: *mut i8, font: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX10CreateFontW - from wine/d3dx10core.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateFontW(device: *mut core::ffi::c_void, height: i32, width: u32, weight: u32, miplevels: u32, italic: i32, charset: u32, precision: u32, quality: u32, pitchandfamily: u32, facename: *mut u16, font: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX10CreateSprite - from wine/d3dx10core.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateSprite(device: *mut core::ffi::c_void, size: u32, sprite: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX10CreateTextureFromMemory - from wine/d3dx10tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateTextureFromMemory(device: *mut core::ffi::c_void, src_data: *mut core::ffi::c_void, src_data_size: usize, loadinfo: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, texture: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX10CreateTextureFromFileA - from wine/d3dx10tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateTextureFromFileA(device: *mut core::ffi::c_void, src_file: *mut i8, load_info: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, texture: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX10CreateTextureFromFileW - from wine/d3dx10tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateTextureFromFileW(device: *mut core::ffi::c_void, src_file: *mut u16, load_info: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, texture: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX10CreateTextureFromResourceA - from wine/d3dx10tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateTextureFromResourceA(device: *mut core::ffi::c_void, module: *mut core::ffi::c_void, resource: *mut i8, load_info: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, texture: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX10CreateTextureFromResourceW - from wine/d3dx10tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateTextureFromResourceW(device: *mut core::ffi::c_void, module: *mut core::ffi::c_void, resource: *mut u16, load_info: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, texture: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX11CreateAsyncFileLoaderA - from wine/d3dx11async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateAsyncFileLoaderA(file_name: *mut i8, loader: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX11CreateAsyncFileLoaderW - from wine/d3dx11async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateAsyncFileLoaderW(file_name: *mut u16, loader: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX11CreateAsyncResourceLoaderA - from wine/d3dx11async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateAsyncResourceLoaderA(module: *mut core::ffi::c_void, resource: *mut i8, loader: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX11CreateAsyncResourceLoaderW - from wine/d3dx11async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateAsyncResourceLoaderW(module: *mut core::ffi::c_void, resource: *mut u16, loader: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX11CreateAsyncMemoryLoader - from wine/d3dx11async.h
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateAsyncMemoryLoader(data: *mut core::ffi::c_void, data_size: usize, loader: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DX11CreateShaderResourceViewFromMemory - from wine/d3dx11tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateShaderResourceViewFromMemory(device: *mut core::ffi::c_void, data: *mut core::ffi::c_void, data_size: usize, load_info: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, view: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX11CreateTextureFromFileA - from wine/d3dx11tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateTextureFromFileA(device: *mut core::ffi::c_void, filename: *mut i8, load_info: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, texture: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX11CreateTextureFromFileW - from wine/d3dx11tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateTextureFromFileW(device: *mut core::ffi::c_void, filename: *mut u16, load_info: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, texture: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX11CreateTextureFromResourceA - from wine/d3dx11tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateTextureFromResourceA(device: *mut core::ffi::c_void, module: *mut core::ffi::c_void, resource: *mut i8, load_info: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, texture: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX11CreateTextureFromResourceW - from wine/d3dx11tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateTextureFromResourceW(device: *mut core::ffi::c_void, module: *mut core::ffi::c_void, resource: *mut u16, load_info: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, texture: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DX11CreateTextureFromMemory - from wine/d3dx11tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DX11CreateTextureFromMemory(device: *mut core::ffi::c_void, src_data: *mut core::ffi::c_void, src_data_size: usize, loadinfo: *mut core::ffi::c_void, pump: *mut core::ffi::c_void, texture: *mut *mut core::ffi::c_void, hresult: *mut i32) -> i32 {
    0
}

/// D3DXCreateKeyframedAnimationSet - from wine/d3dx9anim.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateKeyframedAnimationSet(name: *mut i8, ticks_per_second: f64, playback_type: usize, animation_count: u32, callback_key_count: u32, callback_keys: *mut core::ffi::c_void, animation_set: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateCompressedAnimationSet - from wine/d3dx9anim.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCompressedAnimationSet(name: *mut i8, ticks_per_second: f64, playback_type: usize, compressed_data: *mut core::ffi::c_void, callback_key_count: u32, callback_keys: *mut core::ffi::c_void, animation_set: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateAnimationController - from wine/d3dx9anim.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateAnimationController(max_animation_output_count: u32, max_animation_set_count: u32, max_track_count: u32, max_event_count: u32, animation_controller: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateFontA - from wine/d3dx9core.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateFontA(device: *mut core::ffi::c_void, height: i32, width: u32, weight: u32, miplevels: u32, italic: i32, charset: u32, precision: u32, quality: u32, pitchandfamily: u32, facename: *mut i8, font: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateFontW - from wine/d3dx9core.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateFontW(device: *mut core::ffi::c_void, height: i32, width: u32, weight: u32, miplevels: u32, italic: i32, charset: u32, precision: u32, quality: u32, pitchandfamily: u32, facename: *mut u16, font: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateFontIndirectA - from wine/d3dx9core.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateFontIndirectA(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, font: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateFontIndirectW - from wine/d3dx9core.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateFontIndirectW(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, font: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateLine - from wine/d3dx9core.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateLine(device: *mut core::ffi::c_void, line: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateRenderToEnvMap - from wine/d3dx9core.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateRenderToEnvMap(device: *mut core::ffi::c_void, size: u32, miplevels: u32, format: usize, stencil: i32, stencil_format: usize, rtem: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateRenderToSurface - from wine/d3dx9core.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateRenderToSurface(device: *mut core::ffi::c_void, width: u32, height: u32, format: usize, stencil: i32, stencil_format: usize, rts: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateSprite - from wine/d3dx9core.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateSprite(device: *mut core::ffi::c_void, sprite: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffectPool - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectPool(pool: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffect - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffect(device: *mut core::ffi::c_void, srcdata: *mut core::ffi::c_void, srcdatalen: u32, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, flags: u32, pool: *mut core::ffi::c_void, effect: *mut *mut core::ffi::c_void, compilation_errors: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffectEx - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectEx(device: *mut core::ffi::c_void, srcdata: *mut core::ffi::c_void, srcdatalen: u32, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, skip_constants: *mut i8, flags: u32, pool: *mut core::ffi::c_void, effect: *mut *mut core::ffi::c_void, compilation_errors: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffectCompiler - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectCompiler(srcdata: *mut i8, srcdatalen: u32, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, flags: u32, compiler: *mut *mut core::ffi::c_void, parse_errors: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffectFromFileExA - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromFileExA(device: *mut core::ffi::c_void, srcfile: *mut i8, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, skip_constants: *mut i8, flags: u32, pool: *mut core::ffi::c_void, effect: *mut *mut core::ffi::c_void, compilation_errors: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffectFromFileExW - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromFileExW(device: *mut core::ffi::c_void, srcfile: *mut u16, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, skip_constants: *mut i8, flags: u32, pool: *mut core::ffi::c_void, effect: *mut *mut core::ffi::c_void, compilation_errors: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffectFromFileA - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromFileA(device: *mut core::ffi::c_void, srcfile: *mut i8, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, flags: u32, pool: *mut core::ffi::c_void, effect: *mut *mut core::ffi::c_void, compilation_errors: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffectFromFileW - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromFileW(device: *mut core::ffi::c_void, srcfile: *mut u16, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, flags: u32, pool: *mut core::ffi::c_void, effect: *mut *mut core::ffi::c_void, compilation_errors: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffectFromResourceExA - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromResourceExA(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, srcresource: *mut i8, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, skip_constants: *mut i8, flags: u32, pool: *mut core::ffi::c_void, effect: *mut *mut core::ffi::c_void, compilation_errors: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffectFromResourceExW - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromResourceExW(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, srcresource: *mut u16, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, skip_constants: *mut i8, flags: u32, pool: *mut core::ffi::c_void, effect: *mut *mut core::ffi::c_void, compilation_errors: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffectFromResourceA - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromResourceA(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, srcresource: *mut i8, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, flags: u32, pool: *mut core::ffi::c_void, effect: *mut *mut core::ffi::c_void, compilation_errors: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffectFromResourceW - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectFromResourceW(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, srcresource: *mut u16, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, flags: u32, pool: *mut core::ffi::c_void, effect: *mut *mut core::ffi::c_void, compilation_errors: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffectCompilerFromFileA - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectCompilerFromFileA(srcfile: *mut i8, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, flags: u32, effectcompiler: *mut *mut core::ffi::c_void, parseerrors: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffectCompilerFromFileW - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectCompilerFromFileW(srcfile: *mut u16, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, flags: u32, effectcompiler: *mut *mut core::ffi::c_void, parseerrors: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffectCompilerFromResourceA - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectCompilerFromResourceA(srcmodule: *mut core::ffi::c_void, srcresource: *mut i8, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, flags: u32, effectcompiler: *mut *mut core::ffi::c_void, parseerrors: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateEffectCompilerFromResourceW - from wine/d3dx9effect.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateEffectCompilerFromResourceW(srcmodule: *mut core::ffi::c_void, srcresource: *mut u16, defines: *mut core::ffi::c_void, include: *mut core::ffi::c_void, flags: u32, effectcompiler: *mut *mut core::ffi::c_void, parseerrors: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateMatrixStack - from wine/d3dx9math.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateMatrixStack(flags: u32, stack: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateMesh - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateMesh(face_count: u32, vertex_count: u32, flags: u32, declaration: *mut core::ffi::c_void, device: *mut core::ffi::c_void, mesh: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateMeshFVF - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateMeshFVF(face_count: u32, vertex_count: u32, flags: u32, fvf: u32, device: *mut core::ffi::c_void, mesh: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateBuffer - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateBuffer(size: u32, buffer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateSPMesh - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateSPMesh(mesh: *mut core::ffi::c_void, adjacency: *mut u32, attribute_weights: *mut core::ffi::c_void, vertex_weights: *mut f32, spmesh: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreatePMeshFromStream - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreatePMeshFromStream(stream: *mut core::ffi::c_void, flags: u32, device: *mut core::ffi::c_void, materials: *mut *mut core::ffi::c_void, effect_instances: *mut *mut core::ffi::c_void, material_count: *mut u32, mesh: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateSkinInfo - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateSkinInfo(vertex_count: u32, declaration: *mut core::ffi::c_void, bone_count: u32, skin_info: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateSkinInfoFVF - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateSkinInfoFVF(vertex_count: u32, fvf: u32, bone_count: u32, skin_info: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateSkinInfoFromBlendedMesh - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateSkinInfoFromBlendedMesh(mesh: *mut core::ffi::c_void, bone_count: u32, bone_combination_table: *mut core::ffi::c_void, skin_info: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreatePatchMesh - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreatePatchMesh(patch_info: *mut core::ffi::c_void, patch_count: u32, vertex_count: u32, flags: u32, declaration: *mut core::ffi::c_void, device: *mut core::ffi::c_void, mesh: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreatePRTBuffer - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreatePRTBuffer(sample_count: u32, coeff_count: u32, channel_count: u32, buffer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreatePRTBufferTex - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreatePRTBufferTex(width: u32, height: u32, coeff_count: u32, channel_count: u32, buffer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreatePRTCompBuffer - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreatePRTCompBuffer(quality: usize, cluster_count: u32, pca_count: u32, cb: usize, ctx: *mut core::ffi::c_void, input: *mut core::ffi::c_void, buffer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTextureGutterHelper - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureGutterHelper(width: u32, height: u32, mesh: *mut core::ffi::c_void, gutter_size: f32, gh: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreatePRTEngine - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreatePRTEngine(mesh: *mut core::ffi::c_void, adjacency: *mut u32, extract_uv: i32, blocker_mesh: *mut core::ffi::c_void, engine: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXUVAtlasCreate - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXUVAtlasCreate(mesh_in: *mut core::ffi::c_void, max_chart_count: u32, max_stretch_in: f32, width: u32, height: u32, gutter: f32, texture_idx: u32, adjacency: *mut u32, false_edges: *mut u32, imt_array: *mut f32, cb: usize, cb_freq: f32, ctx: *mut core::ffi::c_void, flags: u32, mesh_out: *mut *mut core::ffi::c_void, face_partitioning_out: *mut *mut core::ffi::c_void, vertex_remap_out: *mut *mut core::ffi::c_void, max_stretch_out: *mut f32, chart_count: *mut u32) -> i32 {
    0
}

/// D3DXCreateTextureShader - from wine/d3dx9shader.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureShader(pFunction: *mut u32, ppTextureShader: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateFragmentLinker - from wine/d3dx9shader.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateFragmentLinker(device: *mut core::ffi::c_void, size: u32, linker: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateFragmentLinkerEx - from wine/d3dx9shader.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateFragmentLinkerEx(device: *mut core::ffi::c_void, size: u32, flags: u32, linker: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateBox - from wine/d3dx9shape.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateBox(device: *mut core::ffi::c_void, width: f32, height: f32, depth: f32, mesh: *mut *mut core::ffi::c_void, adjacency: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateCylinder - from wine/d3dx9shape.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCylinder(device: *mut core::ffi::c_void, radius1: f32, radius2: f32, length: f32, slices: u32, stacks: u32, mesh: *mut *mut core::ffi::c_void, adjacency: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreatePolygon - from wine/d3dx9shape.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreatePolygon(device: *mut core::ffi::c_void, length: f32, sides: u32, mesh: *mut *mut core::ffi::c_void, adjacency: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateSphere - from wine/d3dx9shape.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateSphere(device: *mut core::ffi::c_void, radius: f32, slices: u32, stacks: u32, mesh: *mut *mut core::ffi::c_void, adjacency: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTeapot - from wine/d3dx9shape.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTeapot(device: *mut core::ffi::c_void, mesh: *mut *mut core::ffi::c_void, adjacency: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTextA - from wine/d3dx9shape.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextA(device: *mut core::ffi::c_void, hdc: *mut core::ffi::c_void, text: *mut i8, deviation: f32, extrusion: f32, mesh: *mut *mut core::ffi::c_void, adjacency: *mut *mut core::ffi::c_void, glyphmetrics: *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTextW - from wine/d3dx9shape.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextW(device: *mut core::ffi::c_void, hdc: *mut core::ffi::c_void, text: *mut u16, deviation: f32, extrusion: f32, mesh: *mut *mut core::ffi::c_void, adjacency: *mut *mut core::ffi::c_void, glyphmetrics: *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTorus - from wine/d3dx9shape.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTorus(device: *mut core::ffi::c_void, innerradius: f32, outerradius: f32, sides: u32, rings: u32, mesh: *mut *mut core::ffi::c_void, adjacency: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTexture - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTexture(device: *mut core::ffi::c_void, width: u32, height: u32, miplevels: u32, usage: u32, format: usize, pool: usize, texture: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateCubeTexture - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTexture(device: *mut core::ffi::c_void, size: u32, miplevels: u32, usage: u32, format: usize, pool: usize, cube: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateVolumeTexture - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTexture(device: *mut core::ffi::c_void, width: u32, height: u32, depth: u32, miplevels: u32, usage: u32, format: usize, pool: usize, volume: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTextureFromFileA - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromFileA(device: *mut core::ffi::c_void, srcfile: *mut i8, texture: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTextureFromFileW - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromFileW(device: *mut core::ffi::c_void, srcfile: *mut u16, texture: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateCubeTextureFromFileA - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromFileA(device: *mut core::ffi::c_void, srcfile: *mut i8, cube: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateCubeTextureFromFileW - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromFileW(device: *mut core::ffi::c_void, srcfile: *mut u16, cube: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateVolumeTextureFromFileA - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromFileA(device: *mut core::ffi::c_void, srcfile: *mut i8, volume: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateVolumeTextureFromFileW - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromFileW(device: *mut core::ffi::c_void, srcfile: *mut u16, volume: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTextureFromResourceA - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromResourceA(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, resource: *mut i8, texture: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTextureFromResourceW - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromResourceW(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, resource: *mut u16, texture: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateCubeTextureFromResourceA - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromResourceA(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, resource: *mut i8, cube: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateCubeTextureFromResourceW - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromResourceW(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, resource: *mut u16, cube: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateVolumeTextureFromResourceA - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromResourceA(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, resource: *mut i8, volume: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateVolumeTextureFromResourceW - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromResourceW(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, resource: *mut u16, volume: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTextureFromFileExA - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromFileExA(device: *mut core::ffi::c_void, srcfile: *mut i8, width: u32, height: u32, miplevels: u32, usage: u32, format: usize, pool: usize, filter: u32, mipfilter: u32, colorkey: usize, srcinfo: *mut core::ffi::c_void, palette: *mut core::ffi::c_void, texture: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTextureFromFileExW - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromFileExW(device: *mut core::ffi::c_void, srcfile: *mut u16, width: u32, height: u32, miplevels: u32, usage: u32, format: usize, pool: usize, filter: u32, mipfilter: u32, colorkey: usize, srcinfo: *mut core::ffi::c_void, palette: *mut core::ffi::c_void, texture: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateCubeTextureFromFileExA - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromFileExA(device: *mut core::ffi::c_void, srcfile: *mut i8, size: u32, miplevels: u32, usage: u32, format: usize, pool: usize, filter: u32, mipfilter: u32, colorkey: usize, srcinfo: *mut core::ffi::c_void, palette: *mut core::ffi::c_void, cube: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateCubeTextureFromFileExW - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromFileExW(device: *mut core::ffi::c_void, srcfile: *mut u16, size: u32, miplevels: u32, usage: u32, format: usize, pool: usize, filter: u32, mipfilter: u32, colorkey: usize, srcinfo: *mut core::ffi::c_void, palette: *mut core::ffi::c_void, cube: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateVolumeTextureFromFileExA - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromFileExA(device: *mut core::ffi::c_void, srcfile: *mut i8, width: u32, height: u32, depth: u32, miplevels: u32, usage: u32, format: usize, pool: usize, filter: u32, mipfilter: u32, colorkey: usize, srcinfo: *mut core::ffi::c_void, palette: *mut core::ffi::c_void, volume: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateVolumeTextureFromFileExW - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromFileExW(device: *mut core::ffi::c_void, srcfile: *mut u16, width: u32, height: u32, depth: u32, miplevels: u32, usage: u32, format: usize, pool: usize, filter: u32, mipfilter: u32, colorkey: usize, srcinfo: *mut core::ffi::c_void, palette: *mut core::ffi::c_void, volume: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTextureFromResourceExA - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromResourceExA(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, resource: *mut i8, width: u32, height: u32, miplevels: u32, usage: u32, format: usize, pool: usize, filter: u32, mipfilter: u32, colorkey: usize, srcinfo: *mut core::ffi::c_void, palette: *mut core::ffi::c_void, texture: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTextureFromResourceExW - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromResourceExW(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, resource: *mut u16, width: u32, height: u32, miplevels: u32, usage: u32, format: usize, pool: usize, filter: u32, mipfilter: u32, colorkey: usize, srcinfo: *mut core::ffi::c_void, palette: *mut core::ffi::c_void, texture: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateCubeTextureFromResourceExA - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromResourceExA(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, resource: *mut i8, size: u32, miplevels: u32, usage: u32, format: usize, pool: usize, filter: u32, mipfilter: u32, colorkey: usize, srcinfo: *mut core::ffi::c_void, palette: *mut core::ffi::c_void, cube: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateCubeTextureFromResourceExW - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromResourceExW(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, resource: *mut u16, size: u32, miplevels: u32, usage: u32, format: usize, pool: usize, filter: u32, mipfilter: u32, colorkey: usize, srcinfo: *mut core::ffi::c_void, palette: *mut core::ffi::c_void, cube: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateVolumeTextureFromResourceExA - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromResourceExA(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, resource: *mut i8, width: u32, height: u32, depth: u32, miplevels: u32, usage: u32, format: usize, pool: usize, filter: u32, mipfilter: u32, colorkey: usize, srcinfo: *mut core::ffi::c_void, palette: *mut core::ffi::c_void, volume: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateVolumeTextureFromResourceExW - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromResourceExW(device: *mut core::ffi::c_void, srcmodule: *mut core::ffi::c_void, resource: *mut u16, width: u32, height: u32, depth: u32, miplevels: u32, usage: u32, format: usize, pool: usize, filter: u32, mipfilter: u32, colorkey: usize, srcinfo: *mut core::ffi::c_void, palette: *mut core::ffi::c_void, volume: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTextureFromFileInMemory - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromFileInMemory(device: *mut core::ffi::c_void, srcdata: *mut core::ffi::c_void, srcdatasize: u32, texture: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateCubeTextureFromFileInMemory - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromFileInMemory(device: *mut core::ffi::c_void, srcdata: *mut core::ffi::c_void, srcdatasize: u32, cube: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateVolumeTextureFromFileInMemory - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromFileInMemory(device: *mut core::ffi::c_void, srcdata: *mut core::ffi::c_void, srcdatasize: u32, volume: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateTextureFromFileInMemoryEx - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateTextureFromFileInMemoryEx(device: *mut core::ffi::c_void, srcdata: *mut core::ffi::c_void, srcdatasize: u32, width: u32, height: u32, miplevels: u32, usage: u32, format: usize, pool: usize, filter: u32, mipfilter: u32, colorkey: usize, srcinfo: *mut core::ffi::c_void, palette: *mut core::ffi::c_void, texture: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateCubeTextureFromFileInMemoryEx - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateCubeTextureFromFileInMemoryEx(device: *mut core::ffi::c_void, srcdata: *mut core::ffi::c_void, srcdatasize: u32, size: u32, miplevels: u32, usage: u32, format: usize, pool: usize, filter: u32, mipfilter: u32, colorkey: usize, srcinfo: *mut core::ffi::c_void, palette: *mut core::ffi::c_void, cube: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXCreateVolumeTextureFromFileInMemoryEx - from wine/d3dx9tex.h
#[no_mangle]
pub unsafe extern "C" fn D3DXCreateVolumeTextureFromFileInMemoryEx(device: *mut core::ffi::c_void, srcdata: *mut core::ffi::c_void, srcdatasize: u32, width: u32, height: u32, depth: u32, miplevels: u32, usage: u32, format: usize, pool: usize, filter: u32, mipfilter: u32, colorkey: usize, srcinfo: *mut core::ffi::c_void, palette: *mut core::ffi::c_void, volume: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXFileCreate - from wine/d3dx9xof.h
#[no_mangle]
pub unsafe extern "C" fn D3DXFileCreate(file: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// DebugCreate - from wine/dbgeng.h
#[no_mangle]
pub unsafe extern "C" fn DebugCreate(riid: usize, out: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// DCICreatePrimary - from wine/dciman.h
#[no_mangle]
pub unsafe extern "C" fn DCICreatePrimary(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// DdeCreateDataHandle - from wine/ddeml.h
#[no_mangle]
pub unsafe extern "C" fn DdeCreateDataHandle(arg0: u32, arg1: usize, arg2: u32, arg3: u32, arg4: usize, arg5: u32, arg6: u32) -> usize {
    0
}

/// DdeCreateStringHandleA - from wine/ddeml.h
#[no_mangle]
pub unsafe extern "C" fn DdeCreateStringHandleA(arg0: u32, arg1: *const i8, arg2: i32) -> usize {
    0
}

/// DdeCreateStringHandleW - from wine/ddeml.h
#[no_mangle]
pub unsafe extern "C" fn DdeCreateStringHandleW(arg0: u32, arg1: *const u16, arg2: i32) -> usize {
    0
}

/// DirectDrawCreate - from wine/ddraw.h
#[no_mangle]
pub unsafe extern "C" fn DirectDrawCreate(driver_guid: *mut core::ffi::c_void, ddraw: *mut *mut core::ffi::c_void, outer: *mut core::ffi::c_void) -> i32 {
    0
}

/// DirectDrawCreateEx - from wine/ddraw.h
#[no_mangle]
pub unsafe extern "C" fn DirectDrawCreateEx(driver_guid: *mut core::ffi::c_void, ddraw: *mut *mut core::ffi::c_void, interface_iid: usize, outer: *mut core::ffi::c_void) -> i32 {
    0
}

/// DirectDrawCreateClipper - from wine/ddraw.h
#[no_mangle]
pub unsafe extern "C" fn DirectDrawCreateClipper(flags: u32, clipper: *mut *mut core::ffi::c_void, outer: *mut core::ffi::c_void) -> i32 {
    0
}

/// DevCreateObjectQuery - from wine/devquery.h
#[no_mangle]
pub unsafe extern "C" fn DevCreateObjectQuery(arg0: usize, flags: u32, props_len: u32, props: *mut core::ffi::c_void, filters_len: u32, filters: *mut core::ffi::c_void, callback: usize, user_data: *mut core::ffi::c_void, devquery: *mut core::ffi::c_void) -> i32 {
    0
}

/// DevCreateObjectQueryEx - from wine/devquery.h
#[no_mangle]
pub unsafe extern "C" fn DevCreateObjectQueryEx(arg0: usize, flags: u32, props_len: u32, props: *mut core::ffi::c_void, filters_len: u32, filters: *mut core::ffi::c_void, params_len: u32, params: *mut core::ffi::c_void, callback: usize, user_data: *mut core::ffi::c_void, devquery: *mut core::ffi::c_void) -> i32 {
    0
}

/// DevCreateObjectQueryFromId - from wine/devquery.h
#[no_mangle]
pub unsafe extern "C" fn DevCreateObjectQueryFromId(arg0: usize, id: *mut u16, flags: u32, props_len: u32, props: *mut core::ffi::c_void, filters_len: u32, filters: *mut core::ffi::c_void, callback: usize, user_data: *mut core::ffi::c_void, devquery: *mut core::ffi::c_void) -> i32 {
    0
}

/// DevCreateObjectQueryFromIdEx - from wine/devquery.h
#[no_mangle]
pub unsafe extern "C" fn DevCreateObjectQueryFromIdEx(arg0: usize, id: *mut u16, flags: u32, props_len: u32, props: *mut core::ffi::c_void, filters_len: u32, filters: *mut core::ffi::c_void, params_len: u32, params: *mut core::ffi::c_void, callback: usize, user_data: *mut core::ffi::c_void, devquery: *mut core::ffi::c_void) -> i32 {
    0
}

/// DevCreateObjectQueryFromIds - from wine/devquery.h
#[no_mangle]
pub unsafe extern "C" fn DevCreateObjectQueryFromIds(arg0: usize, id_sz: *mut u16, flags: u32, props_len: u32, props: *mut core::ffi::c_void, filters_len: u32, filters: *mut core::ffi::c_void, callback: usize, user_data: *mut core::ffi::c_void, devquery: *mut core::ffi::c_void) -> i32 {
    0
}

/// DevCreateObjectQueryFromIdsEx - from wine/devquery.h
#[no_mangle]
pub unsafe extern "C" fn DevCreateObjectQueryFromIdsEx(arg0: usize, id_sz: *mut u16, flags: u32, props_len: u32, props: *mut core::ffi::c_void, filters_len: u32, filters: *mut core::ffi::c_void, params_len: u32, params: *mut core::ffi::c_void, callback: usize, user_data: *mut core::ffi::c_void, devquery: *mut core::ffi::c_void) -> i32 {
    0
}

/// DirectInput8Create - from wine/dinput.h
#[no_mangle]
pub unsafe extern "C" fn DirectInput8Create(arg0: *mut core::ffi::c_void, arg1: u32, arg2: usize, arg3: *mut *mut core::ffi::c_void, arg4: usize) -> i32 {
    0
}

/// DirectInputCreateA - from wine/dinput.h
#[no_mangle]
pub unsafe extern "C" fn DirectInputCreateA(arg0: *mut core::ffi::c_void, arg1: u32, arg2: *mut core::ffi::c_void, arg3: usize) -> i32 {
    0
}

/// DirectInputCreateW - from wine/dinput.h
#[no_mangle]
pub unsafe extern "C" fn DirectInputCreateW(arg0: *mut core::ffi::c_void, arg1: u32, arg2: *mut core::ffi::c_void, arg3: usize) -> i32 {
    0
}

/// DirectInputCreateEx - from wine/dinput.h
#[no_mangle]
pub unsafe extern "C" fn DirectInputCreateEx(arg0: *mut core::ffi::c_void, arg1: u32, arg2: usize, arg3: *mut *mut core::ffi::c_void, arg4: usize) -> i32 {
    0
}

/// MoCreateMediaType - from wine/dmort.h
#[no_mangle]
pub unsafe extern "C" fn MoCreateMediaType(arg0: *mut *mut core::ffi::c_void, arg1: u32) -> i32 {
    0
}

/// DirectPlay8AddressCreate - from wine/dpaddr.h
#[no_mangle]
pub unsafe extern "C" fn DirectPlay8AddressCreate(pcIID: *mut core::ffi::c_void, ppvInterface: *mut *mut core::ffi::c_void, pUnknown: *mut core::ffi::c_void) -> i32 {
    0
}

/// DirectPlayCreate - from wine/dplay.h
#[no_mangle]
pub unsafe extern "C" fn DirectPlayCreate(lpGUID: usize, lplpDP: *mut core::ffi::c_void, pUnk: *mut core::ffi::c_void) -> i32 {
    0
}

/// DirectPlay8Create - from wine/dplay8.h
#[no_mangle]
pub unsafe extern "C" fn DirectPlay8Create(pcIID: *mut core::ffi::c_void, ppvInterface: *mut *mut core::ffi::c_void, pUnknown: *mut core::ffi::c_void) -> i32 {
    0
}

/// DirectPlayLobbyCreateW - from wine/dplobby.h
#[no_mangle]
pub unsafe extern "C" fn DirectPlayLobbyCreateW(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void, arg4: u32) -> i32 {
    0
}

/// DirectPlayLobbyCreateA - from wine/dplobby.h
#[no_mangle]
pub unsafe extern "C" fn DirectPlayLobbyCreateA(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void, arg4: u32) -> i32 {
    0
}

/// DirectPlay8LobbyCreate - from wine/dplobby8.h
#[no_mangle]
pub unsafe extern "C" fn DirectPlay8LobbyCreate(pcIID: *mut core::ffi::c_void, ppvInterface: *mut *mut core::ffi::c_void, pUnknown: *mut core::ffi::c_void) -> i32 {
    0
}

/// DirectPlayNATHelpCreate - from wine/dpnathlp.h
#[no_mangle]
pub unsafe extern "C" fn DirectPlayNATHelpCreate(pIID: usize, ppvInterface: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// DirectSoundCreate - from wine/dsound.h
#[no_mangle]
pub unsafe extern "C" fn DirectSoundCreate(lpGUID: usize, ppDS: *mut core::ffi::c_void, pUnkOuter: usize) -> i32 {
    0
}

/// DirectSoundCaptureCreate - from wine/dsound.h
#[no_mangle]
pub unsafe extern "C" fn DirectSoundCaptureCreate(lpGUID: usize, ppDSC: *mut core::ffi::c_void, pUnkOuter: usize) -> i32 {
    0
}

/// DirectSoundCreate8 - from wine/dsound.h
#[no_mangle]
pub unsafe extern "C" fn DirectSoundCreate8(lpGUID: usize, ppDS8: *mut core::ffi::c_void, pUnkOuter: usize) -> i32 {
    0
}

/// DirectSoundCaptureCreate8 - from wine/dsound.h
#[no_mangle]
pub unsafe extern "C" fn DirectSoundCaptureCreate8(lpGUID: usize, ppDSC8: *mut core::ffi::c_void, pUnkOuter: usize) -> i32 {
    0
}

/// DirectSoundFullDuplexCreate - from wine/dsound.h
#[no_mangle]
pub unsafe extern "C" fn DirectSoundFullDuplexCreate(pcGuidCaptureDevice: usize, pcGuidRenderDevice: usize, pcDSCBufferDesc: usize, pcDSBufferDesc: usize, hWnd: *mut core::ffi::c_void, dwLevel: u32, ppDSFD: *mut core::ffi::c_void, ppDSCBuffer8: *mut core::ffi::c_void, ppDSBuffer8: *mut core::ffi::c_void, pUnkOuter: usize) -> i32 {
    0
}

/// DXCoreCreateAdapterFactory - from wine/dxcore.h
#[no_mangle]
pub unsafe extern "C" fn DXCoreCreateAdapterFactory(arg0: usize, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateAdapterList - from wine/dxcore_interface.h
#[no_mangle]
pub unsafe extern "C" fn CreateAdapterList(num_attributes: u32, filter_attributes: *mut core::ffi::c_void, ppv: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// DirectXFileCreate - from wine/dxfile.h
#[no_mangle]
pub unsafe extern "C" fn DirectXFileCreate(lplpDirectXFile: *mut core::ffi::c_void) -> usize {
    0
}

/// FCICreate - from wine/fci.h
#[no_mangle]
pub unsafe extern "C" fn FCICreate(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize, arg7: usize, arg8: usize, arg9: usize, arg10: usize, arg11: usize, arg12: *mut core::ffi::c_void) -> usize {
    0
}

/// FDICreate - from wine/fdi.h
#[no_mangle]
pub unsafe extern "C" fn FDICreate(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize, arg7: i32, arg8: usize) -> usize {
    0
}

/// CreateFile2 - from wine/fileapi.h
#[no_mangle]
pub unsafe extern "C" fn CreateFile2(arg0: *const u16, arg1: u32, arg2: u32, arg3: u32, arg4: usize) -> usize {
    0
}

/// PfCreateInterface - from wine/fltdefs.h
#[no_mangle]
pub unsafe extern "C" fn PfCreateInterface(dwName: u32, inAction: usize, outAction: usize, bUseLog: i32, bMustBeUnique: i32, ppInterface: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateFontPackage - from wine/fontsub.h
#[no_mangle]
pub unsafe extern "C" fn CreateFontPackage(src: *mut u8, src_len: u32, dest: *mut *mut u8, dest_len: *mut u32, written: *mut u32, flags: u16, face_index: u16, format: u16, lang: u16, platform: u16, encoding: u16, keep_list: *mut u16, keep_len: u16, allocproc: usize, reallocproc: usize, freeproc: usize, reserved: *mut core::ffi::c_void) -> u32 {
    0
}

/// GdipCreateEffect - from wine/gdipluseffects.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateEffect(guid: usize, effect: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateAdjustableArrowCap - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateAdjustableArrowCap(arg0: usize, arg1: usize, arg2: i32, arg3: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipBitmapCreateApplyEffect - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipBitmapCreateApplyEffect(arg0: *mut *mut core::ffi::c_void, arg1: i32, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void, arg5: *mut *mut core::ffi::c_void, arg6: i32, arg7: *mut *mut core::ffi::c_void, arg8: *mut i32) -> usize {
    0
}

/// GdipBitmapLockBits - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipBitmapLockBits(arg0: *mut core::ffi::c_void, GpRect: usize, arg2: u32, arg3: usize, arg4: *mut core::ffi::c_void) -> usize {
    0
}

/// GdipBitmapUnlockBits - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipBitmapUnlockBits(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateBitmapFromFile - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromFile(WCHAR: usize, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateBitmapFromFileICM - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromFileICM(WCHAR: usize, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateBitmapFromGdiDib - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromGdiDib(BITMAPINFO: usize, arg1: *mut core::ffi::c_void, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateBitmapFromGraphics - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromGraphics(arg0: i32, arg1: i32, arg2: *mut core::ffi::c_void, arg3: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateBitmapFromHBITMAP - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromHBITMAP(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateBitmapFromHICON - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromHICON(arg0: *mut core::ffi::c_void, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateBitmapFromResource - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromResource(arg0: *mut core::ffi::c_void, WCHAR: usize, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateBitmapFromScan0 - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromScan0(arg0: i32, arg1: i32, arg2: i32, arg3: usize, arg4: *mut u8, arg5: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateBitmapFromStream - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromStream(arg0: *mut core::ffi::c_void, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateBitmapFromStreamICM - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateBitmapFromStreamICM(arg0: *mut core::ffi::c_void, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateHBITMAPFromBitmap - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateHBITMAPFromBitmap(arg0: *mut core::ffi::c_void, arg1: *mut *mut core::ffi::c_void, arg2: usize) -> usize {
    0
}

/// GdipCreateHICONFromBitmap - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateHICONFromBitmap(arg0: *mut core::ffi::c_void, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateCachedBitmap - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateCachedBitmap(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateCustomLineCap - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateCustomLineCap(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize, arg3: usize, arg4: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateFont - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFont(GpFontFamily: usize, arg1: usize, arg2: i32, arg3: usize, arg4: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateFontFromDC - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFontFromDC(arg0: *mut core::ffi::c_void, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateFontFamilyFromName - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFontFamilyFromName(WCHAR: usize, arg1: *mut core::ffi::c_void, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateFromHDC - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFromHDC(arg0: *mut core::ffi::c_void, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateFromHDC2 - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFromHDC2(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateFromHWND - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFromHWND(arg0: *mut core::ffi::c_void, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateFromHWNDICM - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFromHWNDICM(arg0: *mut core::ffi::c_void, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateHalftonePalette - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateHalftonePalette() -> usize {
    0
}

/// GdipCreatePath - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePath(arg0: usize, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreatePath2 - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePath2(GpPointF: usize, BYTE: usize, arg2: i32, arg3: usize, arg4: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreatePath2I - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePath2I(GpPoint: usize, BYTE: usize, arg2: i32, arg3: usize, arg4: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateHatchBrush - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateHatchBrush(arg0: usize, arg1: usize, arg2: usize, arg3: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateImageAttributes - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateImageAttributes(arg0: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateLineBrush - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateLineBrush(GpPointF: usize, GpPointF_1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateLineBrushI - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateLineBrushI(GpPoint: usize, GpPoint_1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateLineBrushFromRect - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateLineBrushFromRect(GpRectF: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateLineBrushFromRectI - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateLineBrushFromRectI(GpRect: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateLineBrushFromRectWithAngle - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateLineBrushFromRectWithAngle(GpRectF: usize, arg1: usize, arg2: usize, arg3: usize, arg4: i32, arg5: usize, arg6: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateLineBrushFromRectWithAngleI - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateLineBrushFromRectWithAngleI(GpRect: usize, arg1: usize, arg2: usize, arg3: usize, arg4: i32, arg5: usize, arg6: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateMatrix - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMatrix(arg0: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateMatrix2 - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMatrix2(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateMatrix3 - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMatrix3(arg0: *mut core::ffi::c_void, GpPointF: usize, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateMatrix3I - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMatrix3I(GpRect: usize, GpPoint: usize, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateMetafileFromEmf - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMetafileFromEmf(arg0: usize, arg1: i32, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateMetafileFromWmf - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMetafileFromWmf(arg0: usize, arg1: i32, WmfPlaceableFileHeader: usize, arg3: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateMetafileFromWmfFile - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMetafileFromWmfFile(WCHAR: usize, WmfPlaceableFileHeader: usize, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateMetafileFromFile - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMetafileFromFile(WCHAR: usize, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateMetafileFromStream - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateMetafileFromStream(arg0: *mut core::ffi::c_void, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreatePathGradient - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePathGradient(GpPointF: usize, arg1: i32, arg2: usize, arg3: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreatePathGradientI - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePathGradientI(GpPoint: usize, arg1: i32, arg2: usize, arg3: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreatePathGradientFromPath - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePathGradientFromPath(GpPath: usize, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreatePathIter - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePathIter(arg0: *mut *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreatePen1 - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePen1(arg0: usize, arg1: usize, arg2: usize, arg3: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreatePen2 - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreatePen2(arg0: *mut core::ffi::c_void, arg1: usize, arg2: usize, arg3: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateRegion - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateRegion(arg0: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateRegionPath - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateRegionPath(arg0: *mut core::ffi::c_void, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateRegionRect - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateRegionRect(arg0: *mut core::ffi::c_void, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateRegionRectI - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateRegionRectI(arg0: *mut core::ffi::c_void, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateRegionRgnData - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateRegionRgnData(arg0: *mut core::ffi::c_void, arg1: i32, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateRegionHrgn - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateRegionHrgn(arg0: *mut core::ffi::c_void, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateSolidFill - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateSolidFill(arg0: usize, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateStringFormat - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateStringFormat(arg0: i32, arg1: usize, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateTexture - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateTexture(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateTexture2 - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateTexture2(arg0: *mut core::ffi::c_void, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateTexture2I - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateTexture2I(arg0: *mut core::ffi::c_void, arg1: usize, arg2: i32, arg3: i32, arg4: i32, arg5: i32, arg6: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateTextureIA - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateTextureIA(arg0: *mut core::ffi::c_void, GpImageAttributes: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateTextureIAI - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateTextureIAI(arg0: *mut core::ffi::c_void, GpImageAttributes: usize, arg2: i32, arg3: i32, arg4: i32, arg5: i32, arg6: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateStreamOnFile - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateStreamOnFile(WCHAR: usize, arg1: u32, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// HttpCreateHttpHandle - from wine/http.h
#[no_mangle]
pub unsafe extern "C" fn HttpCreateHttpHandle(arg0: usize, arg1: u32) -> usize {
    0
}

/// HttpCreateRequestQueue - from wine/http.h
#[no_mangle]
pub unsafe extern "C" fn HttpCreateRequestQueue(version: usize, name: *mut u16, sa: *mut core::ffi::c_void, flags: u32, handle: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// HttpCreateServerSession - from wine/http.h
#[no_mangle]
pub unsafe extern "C" fn HttpCreateServerSession(arg0: usize, arg1: usize, arg2: u32) -> usize {
    0
}

/// HttpCreateUrlGroup - from wine/http.h
#[no_mangle]
pub unsafe extern "C" fn HttpCreateUrlGroup(session_id: usize, group_id: *mut core::ffi::c_void, reserved: u32) -> usize {
    0
}

/// CreateColorTransformA - from wine/icm.h
#[no_mangle]
pub unsafe extern "C" fn CreateColorTransformA(arg0: usize, arg1: usize, arg2: usize, arg3: u32) -> usize {
    0
}

/// CreateColorTransformW - from wine/icm.h
#[no_mangle]
pub unsafe extern "C" fn CreateColorTransformW(arg0: usize, arg1: usize, arg2: usize, arg3: u32) -> usize {
    0
}

/// CreateDeviceLinkProfile - from wine/icm.h
#[no_mangle]
pub unsafe extern "C" fn CreateDeviceLinkProfile(arg0: usize, arg1: u32, arg2: usize, arg3: u32, arg4: u32, arg5: *mut core::ffi::c_void, arg6: u32) -> i32 {
    0
}

/// CreateMultiProfileTransform - from wine/icm.h
#[no_mangle]
pub unsafe extern "C" fn CreateMultiProfileTransform(arg0: usize, arg1: u32, arg2: usize, arg3: u32, arg4: u32, arg5: u32) -> usize {
    0
}

/// IcmpCreateFile - from wine/icmpapi.h
#[no_mangle]
pub unsafe extern "C" fn IcmpCreateFile(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// Icmp6CreateFile - from wine/icmpapi.h
#[no_mangle]
pub unsafe extern "C" fn Icmp6CreateFile(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// ublock_getCode - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ublock_getCode(c: usize) -> usize {
    0
}

/// ImmCreateContext - from wine/imm.h
#[no_mangle]
pub unsafe extern "C" fn ImmCreateContext() -> usize {
    0
}

/// ImmLockIMC - from wine/immdev.h
#[no_mangle]
pub unsafe extern "C" fn ImmLockIMC(arg0: usize) -> usize {
    0
}

/// ImmUnlockIMC - from wine/immdev.h
#[no_mangle]
pub unsafe extern "C" fn ImmUnlockIMC(arg0: usize) -> i32 {
    0
}

/// ImmGetIMCLockCount - from wine/immdev.h
#[no_mangle]
pub unsafe extern "C" fn ImmGetIMCLockCount(arg0: usize) -> u32 {
    0
}

/// ImmCreateIMCC - from wine/immdev.h
#[no_mangle]
pub unsafe extern "C" fn ImmCreateIMCC(arg0: u32) -> usize {
    0
}

/// ImmLockIMCC - from wine/immdev.h
#[no_mangle]
pub unsafe extern "C" fn ImmLockIMCC(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// ImmUnlockIMCC - from wine/immdev.h
#[no_mangle]
pub unsafe extern "C" fn ImmUnlockIMCC(arg0: usize) -> i32 {
    0
}

/// ImmGetIMCCLockCount - from wine/immdev.h
#[no_mangle]
pub unsafe extern "C" fn ImmGetIMCCLockCount(arg0: usize) -> u32 {
    0
}

/// ImmCreateSoftKeyboard - from wine/immdev.h
#[no_mangle]
pub unsafe extern "C" fn ImmCreateSoftKeyboard(arg0: u32, arg1: u32, arg2: i32, arg3: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateInteractionContext - from wine/interactioncontext.h
#[no_mangle]
pub unsafe extern "C" fn CreateInteractionContext(context: *mut core::ffi::c_void) -> i32 {
    0
}

/// CreateIpForwardEntry - from wine/iphlpapi.h
#[no_mangle]
pub unsafe extern "C" fn CreateIpForwardEntry(pRoute: usize) -> usize {
    0
}

/// CreateIpNetEntry - from wine/iphlpapi.h
#[no_mangle]
pub unsafe extern "C" fn CreateIpNetEntry(pArpEntry: usize) -> usize {
    0
}

/// CreateProxyArpEntry - from wine/iphlpapi.h
#[no_mangle]
pub unsafe extern "C" fn CreateProxyArpEntry(dwAddress: u32, dwMask: u32, dwIfIndex: u32) -> usize {
    0
}

/// CreateIProp - from wine/mapiutil.h
#[no_mangle]
pub unsafe extern "C" fn CreateIProp(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void, arg5: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateTable - from wine/mapiutil.h
#[no_mangle]
pub unsafe extern "C" fn CreateTable(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void, arg5: u32, arg6: u32, arg7: usize, arg8: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateFileMapping2 - from wine/memoryapi.h
#[no_mangle]
pub unsafe extern "C" fn CreateFileMapping2(arg0: *mut core::ffi::c_void, arg1: usize, arg2: u32, arg3: u32, arg4: u32, arg5: usize, arg6: *mut u16, arg7: *mut core::ffi::c_void, arg8: u32) -> usize {
    0
}

/// MFBeginCreateFile - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFBeginCreateFile(access_mode: usize, open_mode: usize, flags: usize, path: *mut u16, callback: *mut core::ffi::c_void, state: *mut core::ffi::c_void, cancel_cookie: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCancelCreateFile - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCancelCreateFile(cancel_cookie: *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreate2DMediaBuffer - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreate2DMediaBuffer(width: u32, height: u32, fourcc: u32, bottom_up: i32, buffer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateAlignedMemoryBuffer - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateAlignedMemoryBuffer(max_length: u32, alignment: u32, buffer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateAttributes - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateAttributes(attributes: *mut *mut core::ffi::c_void, size: usize) -> i32 {
    0
}

/// MFCreateAsyncResult - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateAsyncResult(object: *mut core::ffi::c_void, callback: *mut core::ffi::c_void, state: *mut core::ffi::c_void, result: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateAudioMediaType - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateAudioMediaType(audioformat: *mut core::ffi::c_void, mediatype: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateCollection - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateCollection(collection: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateDXGIDeviceManager - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateDXGIDeviceManager(token: *mut u32, manager: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateDXGISurfaceBuffer - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateDXGISurfaceBuffer(riid: usize, surface: *mut core::ffi::c_void, subresource: u32, bottomup: i32, buffer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateDXSurfaceBuffer - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateDXSurfaceBuffer(riid: usize, surface: *mut core::ffi::c_void, bottom_up: i32, buffer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateEventQueue - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateEventQueue(queue: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateFile - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateFile(accessmode: usize, openmode: usize, flags: usize, url: *const u16, bytestream: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateMediaBufferFromMediaType - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateMediaBufferFromMediaType(media_type: *mut core::ffi::c_void, duration: i64, min_length: u32, min_alignment: u32, buffer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateMediaEvent - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateMediaEvent(arg0: usize, extended_type: usize, status: i32, value: *mut core::ffi::c_void, event: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateMediaType - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateMediaType(arg0: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateAMMediaTypeFromMFMediaType - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateAMMediaTypeFromMFMediaType(media_type: *mut core::ffi::c_void, format_type: usize, am_type: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateMFVideoFormatFromMFMediaType - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateMFVideoFormatFromMFMediaType(media_type: *mut core::ffi::c_void, video_format: *mut *mut core::ffi::c_void, size: *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateMediaTypeFromRepresentation - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateMediaTypeFromRepresentation(guid_representation: usize, representation: *mut core::ffi::c_void, media_type: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateSample - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateSample(sample: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateTempFile - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateTempFile(accessmode: usize, openmode: usize, flags: usize, bytestream: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateVideoMediaType - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateVideoMediaType(format: *mut core::ffi::c_void, media_type: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateVideoMediaTypeFromSubtype - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateVideoMediaTypeFromSubtype(subtype: *mut core::ffi::c_void, media_type: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateLegacyMediaBufferOnMFMediaBuffer - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateLegacyMediaBufferOnMFMediaBuffer(sample: *mut core::ffi::c_void, media_buffer: *mut core::ffi::c_void, offset: u32, obj: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateVideoMediaTypeFromVideoInfoHeader - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateVideoMediaTypeFromVideoInfoHeader(vih: *mut core::ffi::c_void, size: u32, pixel_aspect_ratio_x: u32, pixel_aspect_ratio_y: u32, interlace_mode: usize, video_flags: u64, subtype: *mut core::ffi::c_void, media_type: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateMemoryBuffer - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateMemoryBuffer(max_length: u32, buffer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFCreateWaveFormatExFromMFMediaType - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFCreateWaveFormatExFromMFMediaType(arg0: *mut core::ffi::c_void, format: *mut *mut core::ffi::c_void, size: *mut core::ffi::c_void, flags: usize) -> i32 {
    0
}

/// MFEndCreateFile - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFEndCreateFile(result: *mut core::ffi::c_void, stream: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFLockDXGIDeviceManager - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFLockDXGIDeviceManager(token: *mut u32, manager: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MFLockPlatform - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFLockPlatform() -> i32 {
    0
}

/// MFLockSharedWorkQueue - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFLockSharedWorkQueue(name: *mut u16, base_priority: i32, taskid: *mut u32, queue: *mut u32) -> i32 {
    0
}

/// MFUnlockDXGIDeviceManager - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFUnlockDXGIDeviceManager() -> i32 {
    0
}

/// MFUnlockPlatform - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFUnlockPlatform() -> i32 {
    0
}

/// MFUnlockWorkQueue - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFUnlockWorkQueue(queue: u32) -> i32 {
    0
}

/// MesEncodeIncrementalHandleCreate - from wine/midles.h
#[no_mangle]
pub unsafe extern "C" fn MesEncodeIncrementalHandleCreate(arg0: *mut core::ffi::c_void, arg1: usize, arg2: usize, arg3: *mut core::ffi::c_void) -> usize {
    0
}

/// MesDecodeIncrementalHandleCreate - from wine/midles.h
#[no_mangle]
pub unsafe extern "C" fn MesDecodeIncrementalHandleCreate(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// MesEncodeFixedBufferHandleCreate - from wine/midles.h
#[no_mangle]
pub unsafe extern "C" fn MesEncodeFixedBufferHandleCreate(arg0: *mut i8, arg1: u32, arg2: *mut u32, arg3: *mut core::ffi::c_void) -> usize {
    0
}

/// MesEncodeDynBufferHandleCreate - from wine/midles.h
#[no_mangle]
pub unsafe extern "C" fn MesEncodeDynBufferHandleCreate(arg0: *mut *mut i8, arg1: *mut u32, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// MesDecodeBufferHandleCreate - from wine/midles.h
#[no_mangle]
pub unsafe extern "C" fn MesDecodeBufferHandleCreate(arg0: *mut i8, arg1: u32, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// mmTaskCreate - from wine/mmddk.h
#[no_mangle]
pub unsafe extern "C" fn mmTaskCreate(arg0: usize, arg1: *mut *mut core::ffi::c_void, arg2: usize) -> u32 {
    0
}

/// mmTaskBlock - from wine/mmddk.h
#[no_mangle]
pub unsafe extern "C" fn mmTaskBlock(arg0: u32) -> usize {
    0
}

/// mmioCreateChunk - from wine/mmsystem.h
#[no_mangle]
pub unsafe extern "C" fn mmioCreateChunk(arg0: usize, arg1: *mut core::ffi::c_void, arg2: u32) -> usize {
    0
}

/// ASN1_CreateDecoder - from wine/msasn1.h
#[no_mangle]
pub unsafe extern "C" fn ASN1_CreateDecoder(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: usize, arg4: usize) -> usize {
    0
}

/// ASN1_CreateDecoderEx - from wine/msasn1.h
#[no_mangle]
pub unsafe extern "C" fn ASN1_CreateDecoderEx(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: usize, arg4: usize, arg5: usize) -> usize {
    0
}

/// ASN1_CreateEncoder - from wine/msasn1.h
#[no_mangle]
pub unsafe extern "C" fn ASN1_CreateEncoder(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: usize, arg4: usize) -> usize {
    0
}

/// ASN1_CreateModule - from wine/msasn1.h
#[no_mangle]
pub unsafe extern "C" fn ASN1_CreateModule(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize, arg7: usize, arg8: usize) -> usize {
    0
}

/// MsiCreateRecord - from wine/msiquery.h
#[no_mangle]
pub unsafe extern "C" fn MsiCreateRecord(arg0: u32) -> usize {
    0
}

/// MsiCreateTransformSummaryInfoA - from wine/msiquery.h
#[no_mangle]
pub unsafe extern "C" fn MsiCreateTransformSummaryInfoA(arg0: usize, arg1: usize, arg2: *const i8, arg3: i32, arg4: i32) -> u32 {
    0
}

/// MsiCreateTransformSummaryInfoW - from wine/msiquery.h
#[no_mangle]
pub unsafe extern "C" fn MsiCreateTransformSummaryInfoW(arg0: usize, arg1: usize, arg2: *const u16, arg3: i32, arg4: i32) -> u32 {
    0
}

/// CryptSIPCreateIndirectData - from wine/mssip.h
#[no_mangle]
pub unsafe extern "C" fn CryptSIPCreateIndirectData(arg0: *mut core::ffi::c_void, arg1: *mut u32, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// CreateBoundaryDescriptorW - from wine/namespaceapi.h
#[no_mangle]
pub unsafe extern "C" fn CreateBoundaryDescriptorW(arg0: *const u16, arg1: u32) -> usize {
    0
}

/// CreatePrivateNamespaceW - from wine/namespaceapi.h
#[no_mangle]
pub unsafe extern "C" fn CreatePrivateNamespaceW(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *const u16) -> usize {
    0
}

/// NCryptCreatePersistedKey - from wine/ncrypt.h
#[no_mangle]
pub unsafe extern "C" fn NCryptCreatePersistedKey(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut u16, arg3: *mut u16, arg4: u32, arg5: u32) -> usize {
    0
}

/// NtGdiCreateBitmap - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateBitmap(width: i32, height: i32, planes: u32, bpp: u32, bits: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiCreateClientObj - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateClientObj(arg0: u32) -> usize {
    0
}

/// NtGdiCreateCompatibleBitmap - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateCompatibleBitmap(hdc: *mut core::ffi::c_void, width: i32, height: i32) -> usize {
    0
}

/// NtGdiCreateCompatibleDC - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateCompatibleDC(hdc: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiCreateDIBBrush - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateDIBBrush(data: *mut core::ffi::c_void, coloruse: u32, size: u32, is_8x8: i32, pen: i32, client: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiCreateDIBSection - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateDIBSection(hdc: *mut core::ffi::c_void, section: *mut core::ffi::c_void, offset: u32, bmi: *mut core::ffi::c_void, usage: u32, header_size: u32, flags: u32, color_space: usize, bits: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiCreateDIBitmapInternal - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateDIBitmapInternal(hdc: *mut core::ffi::c_void, width: i32, height: i32, init: u32, bits: *mut core::ffi::c_void, data: *mut core::ffi::c_void, coloruse: u32, max_info: u32, max_bits: u32, flags: u32, xform: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiCreateEllipticRgn - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateEllipticRgn(left: i32, top: i32, right: i32, bottom: i32) -> usize {
    0
}

/// NtGdiCreateHalftonePalette - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateHalftonePalette(hdc: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiCreateHatchBrushInternal - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateHatchBrushInternal(style: i32, color: u32, pen: i32) -> usize {
    0
}

/// NtGdiCreateMetafileDC - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateMetafileDC(hdc: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiCreatePaletteInternal - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreatePaletteInternal(palette: *mut core::ffi::c_void, count: u32) -> usize {
    0
}

/// NtGdiCreatePatternBrushInternal - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreatePatternBrushInternal(hbitmap: *mut core::ffi::c_void, pen: i32, is_8x8: i32) -> usize {
    0
}

/// NtGdiCreatePen - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreatePen(style: i32, width: i32, color: u32, brush: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiCreateRectRgn - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateRectRgn(left: i32, top: i32, right: i32, bottom: i32) -> usize {
    0
}

/// NtGdiCreateRoundRectRgn - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateRoundRectRgn(left: i32, top: i32, right: i32, bottom: i32, ellipse_width: i32, ellipse_height: i32) -> usize {
    0
}

/// NtGdiCreateSolidBrush - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiCreateSolidBrush(color: u32, brush: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiExtCreatePen - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiExtCreatePen(style: u32, width: u32, brush_style: u32, color: u32, client_hatch: usize, hatch: usize, style_count: u32, style_bits: *mut u32, dib_size: u32, old_style: i32, brush: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiExtCreateRegion - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiExtCreateRegion(xform: *mut core::ffi::c_void, count: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiHfontCreate - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiHfontCreate(logfont: *mut core::ffi::c_void, unk2: u32, unk3: u32, unk4: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiDdDDIAcquireKeyedMutex - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIAcquireKeyedMutex(params: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiDdDDIAcquireKeyedMutex2 - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIAcquireKeyedMutex2(params: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiDdDDICreateDCFromMemory - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDICreateDCFromMemory(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiDdDDICreateDevice - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDICreateDevice(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiDdDDICreateKeyedMutex - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDICreateKeyedMutex(params: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiDdDDICreateKeyedMutex2 - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDICreateKeyedMutex2(params: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiDdDDICreateSynchronizationObject - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDICreateSynchronizationObject(params: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiDdDDICreateSynchronizationObject2 - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDICreateSynchronizationObject2(params: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiDdDDIDestroyKeyedMutex - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIDestroyKeyedMutex(params: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiDdDDIOpenKeyedMutex - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIOpenKeyedMutex(params: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiDdDDIOpenKeyedMutex2 - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIOpenKeyedMutex2(params: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiDdDDIOpenKeyedMutexFromNtHandle - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIOpenKeyedMutexFromNtHandle(params: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiDdDDIReleaseKeyedMutex - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIReleaseKeyedMutex(params: *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiDdDDIReleaseKeyedMutex2 - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiDdDDIReleaseKeyedMutex2(params: *mut core::ffi::c_void) -> usize {
    0
}

/// LsaCreateTrustedDomainEx - from wine/ntsecapi.h
#[no_mangle]
pub unsafe extern "C" fn LsaCreateTrustedDomainEx(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// NtUserCreateAcceleratorTable - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateAcceleratorTable(table: *mut core::ffi::c_void, count: i32) -> usize {
    0
}

/// NtUserCreateCaret - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateCaret(hwnd: *mut core::ffi::c_void, bitmap: *mut core::ffi::c_void, width: i32, height: i32) -> usize {
    0
}

/// NtUserCreateDesktopEx - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateDesktopEx(attr: *mut core::ffi::c_void, device: *mut core::ffi::c_void, devmode: *mut core::ffi::c_void, flags: u32, access: usize, heap_size: u32) -> usize {
    0
}

/// NtUserCreateInputContext - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateInputContext(client_ptr: usize) -> usize {
    0
}

/// NtUserCreateMenu - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateMenu() -> usize {
    0
}

/// NtUserCreatePopupMenu - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserCreatePopupMenu() -> usize {
    0
}

/// NtUserCreateWindowEx - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateWindowEx(ex_style: u32, class_name: *mut core::ffi::c_void, version: *mut core::ffi::c_void, window_name: *mut core::ffi::c_void, style: u32, x: i32, y: i32, cx: i32, cy: i32, parent: *mut core::ffi::c_void, menu: *mut core::ffi::c_void, instance: *mut core::ffi::c_void, params: *mut core::ffi::c_void, flags: u32, client_instance: *mut core::ffi::c_void, class: *mut u16, ansi: i32) -> usize {
    0
}

/// NtUserCreateWindowStation - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateWindowStation(attr: *mut core::ffi::c_void, mask: usize, arg3: u32, arg4: u32, arg5: u32, arg6: u32, arg7: u32) -> usize {
    0
}

/// NtUserLockWindowUpdate - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserLockWindowUpdate(hwnd: *mut core::ffi::c_void) -> usize {
    0
}

/// NtUserCreateCursorIcon - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserCreateCursorIcon(is_icon: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// NtUserEnableThunkLock - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserEnableThunkLock(thunk_lock_callback: usize) {

}

/// CoLockObjectExternal - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CoLockObjectExternal(pUnk: usize, fLock: i32, fLastUnlockReleases: i32) -> usize {
    0
}

/// CoCreateGuid - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CoCreateGuid(pguid: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateDataAdviseHolder - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CreateDataAdviseHolder(ppDAHolder: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateDataCache - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CreateDataCache(pUnkOuter: usize, rclsid: usize, iid: usize, ppv: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateAntiMoniker - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CreateAntiMoniker(ppmk: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateBindCtx - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CreateBindCtx(reserved: u32, ppbc: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateClassMoniker - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CreateClassMoniker(rclsid: usize, ppmk: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateFileMoniker - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CreateFileMoniker(lpszPathName: usize, ppmk: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateGenericComposite - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CreateGenericComposite(pmkFirst: usize, pmkRest: usize, ppmkComposite: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateItemMoniker - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CreateItemMoniker(lpszDelim: usize, lpszItem: usize, ppmk: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateObjrefMoniker - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CreateObjrefMoniker(punk: usize, ppmk: *mut core::ffi::c_void) -> usize {
    0
}

/// CreatePointerMoniker - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CreatePointerMoniker(punk: usize, ppmk: *mut core::ffi::c_void) -> usize {
    0
}

/// StgCreateDocfile - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn StgCreateDocfile(pwcsName: usize, grfMode: u32, reserved: u32, ppstgOpen: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// StgCreateStorageEx - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn StgCreateStorageEx(arg0: *mut u16, arg1: u32, arg2: u32, arg3: u32, arg4: *mut core::ffi::c_void, arg5: *mut core::ffi::c_void, arg6: usize, arg7: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// StgIsStorageILockBytes - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn StgIsStorageILockBytes(plkbyt: *mut core::ffi::c_void) -> usize {
    0
}

/// StgCreateDocfileOnILockBytes - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn StgCreateDocfileOnILockBytes(plkbyt: *mut core::ffi::c_void, grfMode: u32, reserved: u32, ppstgOpen: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// StgOpenStorageOnILockBytes - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn StgOpenStorageOnILockBytes(plkbyt: *mut core::ffi::c_void, pstgPriority: *mut core::ffi::c_void, grfMode: u32, snbExclude: usize, reserved: u32, ppstgOpen: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SQLCreateDataSource - from wine/odbcinst.h
#[no_mangle]
pub unsafe extern "C" fn SQLCreateDataSource(arg0: *mut core::ffi::c_void, arg1: *const i8) -> i32 {
    0
}

/// SQLCreateDataSourceW - from wine/odbcinst.h
#[no_mangle]
pub unsafe extern "C" fn SQLCreateDataSourceW(arg0: *mut core::ffi::c_void, arg1: *const u16) -> i32 {
    0
}

/// OleCreateMenuDescriptor - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn OleCreateMenuDescriptor(hmenuCombined: *mut core::ffi::c_void, lpMenuWidths: usize) -> usize {
    0
}

/// CreateStreamOnHGlobal - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn CreateStreamOnHGlobal(hGlobal: *mut core::ffi::c_void, fDeleteOnRelease: i32, ppstm: *mut core::ffi::c_void) -> usize {
    0
}

/// OleCreateLinkFromData - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn OleCreateLinkFromData(pSrcDataObj: usize, riid: usize, renderopt: u32, pFormatEtc: usize, pClientSite: usize, pStg: usize, ppvObj: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OleQueryCreateFromData - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn OleQueryCreateFromData(pSrcDataObject: usize) -> usize {
    0
}

/// OleCreateStaticFromData - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn OleCreateStaticFromData(pSrcDataObj: usize, iid: usize, renderopt: u32, pFormatEtc: usize, pClientSite: usize, pStg: usize, ppvObj: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetHGlobalFromILockBytes - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn GetHGlobalFromILockBytes(plkbyt: usize, phglobal: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateILockBytesOnHGlobal - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn CreateILockBytesOnHGlobal(hGlobal: *mut core::ffi::c_void, fDeleteOnRelease: i32, pplkbyt: *mut core::ffi::c_void) -> usize {
    0
}

/// OleLockRunning - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn OleLockRunning(pUnknown: usize, fLock: i32, fLastUnlockCloses: i32) -> usize {
    0
}

/// OleCreateFromFile - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn OleCreateFromFile(rclsid: usize, lpszFileName: usize, riid: usize, renderopt: u32, lpFormatEtc: usize, pClientSite: usize, pStg: usize, ppvObj: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OleCreateFromFileEx - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn OleCreateFromFileEx(clsid: usize, filename: usize, iid: usize, flags: u32, renderopt: u32, num_fmts: u32, adv_flags: *mut u32, fmts: usize, sink: *mut core::ffi::c_void, conns: *mut u32, client_site: usize, storage: usize, obj: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OleCreateLink - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn OleCreateLink(pmkLinkSrc: usize, riid: usize, renderopt: u32, lpFormatEtc: usize, pClientSite: usize, pStg: usize, ppvObj: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OleCreate - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn OleCreate(rclsid: usize, riid: usize, renderopt: u32, pFormatEtc: usize, pClientSite: usize, pStg: usize, ppvObj: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OleCreateLinkToFile - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn OleCreateLinkToFile(lpszFileName: usize, riid: usize, renderopt: u32, lpFormatEtc: usize, pClientSite: usize, pStg: usize, ppvObj: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OleCreateFromData - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn OleCreateFromData(pSrcDataObj: usize, riid: usize, renderopt: u32, pFormatEtc: usize, pClientSite: usize, pStg: usize, ppvObj: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OleCreateFromDataEx - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn OleCreateFromDataEx(pSrcDataObj: usize, riid: usize, dwFlags: u32, renderopt: u32, num_formats: u32, adv_flags: *mut u32, fmts: usize, sink: *mut core::ffi::c_void, conns: *mut u32, pClientSite: usize, pStg: usize, ppvObj: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OleCreateDefaultHandler - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn OleCreateDefaultHandler(clsid: usize, pUnkOuter: usize, riid: usize, ppvObj: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OleCreateEmbeddingHelper - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn OleCreateEmbeddingHelper(clsid: usize, pUnkOuter: usize, flags: u32, pCF: *mut core::ffi::c_void, riid: usize, ppvObj: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateOleAdviseHolder - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn CreateOleAdviseHolder(ppOAHolder: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateErrorInfo - from wine/oleauto.h
#[no_mangle]
pub unsafe extern "C" fn CreateErrorInfo(arg0: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SafeArrayCreate - from wine/oleauto.h
#[no_mangle]
pub unsafe extern "C" fn SafeArrayCreate(arg0: usize, arg1: u32, arg2: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SafeArrayCreateEx - from wine/oleauto.h
#[no_mangle]
pub unsafe extern "C" fn SafeArrayCreateEx(arg0: usize, arg1: u32, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SafeArrayCreateVector - from wine/oleauto.h
#[no_mangle]
pub unsafe extern "C" fn SafeArrayCreateVector(arg0: usize, arg1: i32, arg2: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SafeArrayCreateVectorEx - from wine/oleauto.h
#[no_mangle]
pub unsafe extern "C" fn SafeArrayCreateVectorEx(arg0: usize, arg1: i32, arg2: u32, arg3: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SafeArrayLock - from wine/oleauto.h
#[no_mangle]
pub unsafe extern "C" fn SafeArrayLock(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// SafeArrayUnlock - from wine/oleauto.h
#[no_mangle]
pub unsafe extern "C" fn SafeArrayUnlock(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateDispTypeInfo - from wine/oleauto.h
#[no_mangle]
pub unsafe extern "C" fn CreateDispTypeInfo(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateStdDispatch - from wine/oleauto.h
#[no_mangle]
pub unsafe extern "C" fn CreateStdDispatch(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateTypeLib - from wine/oleauto.h
#[no_mangle]
pub unsafe extern "C" fn CreateTypeLib(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateTypeLib2 - from wine/oleauto.h
#[no_mangle]
pub unsafe extern "C" fn CreateTypeLib2(arg0: usize, arg1: usize, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OleCreatePropertyFrameIndirect - from wine/olectl.h
#[no_mangle]
pub unsafe extern "C" fn OleCreatePropertyFrameIndirect(lpParams: usize) -> usize {
    0
}

/// OleCreatePropertyFrame - from wine/olectl.h
#[no_mangle]
pub unsafe extern "C" fn OleCreatePropertyFrame(hwndOwner: *mut core::ffi::c_void, x: u32, y: u32, lpszCaption: usize, cObjects: u32, ppUnk: *mut core::ffi::c_void, cPages: u32, pPageClsID: usize, lcid: usize, dwReserved: u32, pvReserved: *mut core::ffi::c_void) -> usize {
    0
}

/// OleCreatePictureIndirect - from wine/olectl.h
#[no_mangle]
pub unsafe extern "C" fn OleCreatePictureIndirect(lpPictDesc: usize, riid: usize, fOwn: i32, lplpvObj: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OleCreateFontIndirect - from wine/olectl.h
#[no_mangle]
pub unsafe extern "C" fn OleCreateFontIndirect(lpFontDesc: usize, riid: usize, lplpvObj: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreatePropertySheetPageA - from wine/prsht.h
#[no_mangle]
pub unsafe extern "C" fn CreatePropertySheetPageA(arg0: usize) -> usize {
    0
}

/// CreatePropertySheetPageW - from wine/prsht.h
#[no_mangle]
pub unsafe extern "C" fn CreatePropertySheetPageW(arg0: usize) -> usize {
    0
}

/// QOSCreateHandle - from wine/qos2.h
#[no_mangle]
pub unsafe extern "C" fn QOSCreateHandle(version: usize, handle: usize) -> i32 {
    0
}

/// RasCreatePhonebookEntryA - from wine/ras.h
#[no_mangle]
pub unsafe extern "C" fn RasCreatePhonebookEntryA(arg0: *mut core::ffi::c_void, arg1: *const i8) -> u32 {
    0
}

/// RasCreatePhonebookEntryW - from wine/ras.h
#[no_mangle]
pub unsafe extern "C" fn RasCreatePhonebookEntryW(arg0: *mut core::ffi::c_void, arg1: *const u16) -> u32 {
    0
}

/// RpcSsContextLockExclusive - from wine/rpcasync.h
#[no_mangle]
pub unsafe extern "C" fn RpcSsContextLockExclusive(arg0: usize, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// RpcSsContextLockShared - from wine/rpcasync.h
#[no_mangle]
pub unsafe extern "C" fn RpcSsContextLockShared(arg0: usize, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// UuidCreate - from wine/rpcdce.h
#[no_mangle]
pub unsafe extern "C" fn UuidCreate(Uuid: *mut core::ffi::c_void) -> usize {
    0
}

/// UuidCreateSequential - from wine/rpcdce.h
#[no_mangle]
pub unsafe extern "C" fn UuidCreateSequential(Uuid: *mut core::ffi::c_void) -> usize {
    0
}

/// UuidCreateNil - from wine/rpcdce.h
#[no_mangle]
pub unsafe extern "C" fn UuidCreateNil(Uuid: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateProxyFromTypeInfo - from wine/rpcproxy.h
#[no_mangle]
pub unsafe extern "C" fn CreateProxyFromTypeInfo(pTypeInfo: usize, pUnkOuter: usize, riid: usize, ppProxy: *mut core::ffi::c_void, ppv: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateStubFromTypeInfo - from wine/rpcproxy.h
#[no_mangle]
pub unsafe extern "C" fn CreateStubFromTypeInfo(pTypeInfo: usize, riid: usize, pUnkServer: usize, ppStub: *mut core::ffi::c_void) -> usize {
    0
}

/// SetupCreateDiskSpaceListA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupCreateDiskSpaceListA(arg0: *mut core::ffi::c_void, arg1: u32, arg2: u32) -> usize {
    0
}

/// SetupCreateDiskSpaceListW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupCreateDiskSpaceListW(arg0: *mut core::ffi::c_void, arg1: u32, arg2: u32) -> usize {
    0
}

/// SetupDiCreateDeviceInfoList - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInfoList(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// SetupDiCreateDeviceInfoListExA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInfoListExA(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize, arg3: *mut core::ffi::c_void) -> usize {
    0
}

/// SetupDiCreateDeviceInfoListExW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInfoListExW(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize, arg3: *mut core::ffi::c_void) -> usize {
    0
}

/// SetupDiCreateDeviceInfoA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInfoA(arg0: usize, arg1: usize, arg2: *mut core::ffi::c_void, arg3: usize, arg4: *mut core::ffi::c_void, arg5: u32, arg6: usize) -> usize {
    0
}

/// SetupDiCreateDeviceInfoW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInfoW(arg0: usize, arg1: usize, arg2: *mut core::ffi::c_void, arg3: usize, arg4: *mut core::ffi::c_void, arg5: u32, arg6: usize) -> usize {
    0
}

/// SetupDiCreateDeviceInterfaceA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInterfaceA(arg0: usize, arg1: usize, arg2: *mut core::ffi::c_void, arg3: usize, arg4: u32, arg5: usize) -> usize {
    0
}

/// SetupDiCreateDeviceInterfaceW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInterfaceW(arg0: usize, arg1: usize, arg2: *mut core::ffi::c_void, arg3: usize, arg4: u32, arg5: usize) -> usize {
    0
}

/// SetupDiCreateDeviceInterfaceRegKeyA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInterfaceRegKeyA(arg0: usize, arg1: usize, arg2: u32, arg3: usize, arg4: usize, arg5: usize) -> usize {
    0
}

/// SetupDiCreateDeviceInterfaceRegKeyW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDeviceInterfaceRegKeyW(arg0: usize, arg1: usize, arg2: u32, arg3: usize, arg4: usize, arg5: usize) -> usize {
    0
}

/// SetupDiCreateDevRegKeyA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDevRegKeyA(arg0: usize, arg1: usize, arg2: u32, arg3: u32, arg4: u32, arg5: usize, arg6: usize) -> usize {
    0
}

/// SetupDiCreateDevRegKeyW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiCreateDevRegKeyW(arg0: usize, arg1: usize, arg2: u32, arg3: u32, arg4: u32, arg5: usize, arg6: usize) -> usize {
    0
}

/// CreateRandomAccessStreamOverStream - from wine/shcore.h
#[no_mangle]
pub unsafe extern "C" fn CreateRandomAccessStreamOverStream(stream: *mut core::ffi::c_void, options: usize, riid: usize, ppv: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// SHCreatePropSheetExtArray - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHCreatePropSheetExtArray(arg0: *mut core::ffi::c_void, arg1: *const u16, arg2: u32) -> usize {
    0
}

/// SHCreatePropSheetExtArrayEx - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHCreatePropSheetExtArrayEx(arg0: *mut core::ffi::c_void, arg1: *const u16, arg2: u32, arg3: *mut core::ffi::c_void) -> usize {
    0
}

/// SHCreateQueryCancelAutoPlayMoniker - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateQueryCancelAutoPlayMoniker(arg0: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SHCreateShellItem - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateShellItem(arg0: usize, arg1: *mut core::ffi::c_void, arg2: usize, arg3: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SHCreateStdEnumFmtEtc - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateStdEnumFmtEtc(arg0: u32, arg1: *mut core::ffi::c_void, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SHILCreateFromPath - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHILCreateFromPath(arg0: *const u16, arg1: *mut core::ffi::c_void, arg2: *mut u32) -> usize {
    0
}

/// SHCreateShellFolderViewEx - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateShellFolderViewEx(pshfvi: usize, ppshv: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SHCreateShellFolderView - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateShellFolderView(pscfv: *mut core::ffi::c_void, ppsv: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SHChangeNotification_Lock - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHChangeNotification_Lock(hChangeNotification: *mut core::ffi::c_void, dwProcessId: u32, pppidl: *mut *mut core::ffi::c_void, plEvent: *mut i32) -> usize {
    0
}

/// SHChangeNotification_Unlock - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHChangeNotification_Unlock(hLock: *mut core::ffi::c_void) -> usize {
    0
}

/// SHCreateDirectory - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateDirectory(arg0: *mut core::ffi::c_void, arg1: *const core::ffi::c_void) -> usize {
    0
}

/// SHCreateDirectoryExA - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateDirectoryExA(arg0: *mut core::ffi::c_void, arg1: *const i8, arg2: usize) -> usize {
    0
}

/// SHCreateDirectoryExW - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateDirectoryExW(arg0: *mut core::ffi::c_void, arg1: *const u16, arg2: usize) -> usize {
    0
}

/// ILCreateFromPathA - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn ILCreateFromPathA(arg0: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// ILCreateFromPathW - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn ILCreateFromPathW(arg0: *mut u16) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SHCreateDefaultContextMenu - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateDefaultContextMenu(pdcm: *mut core::ffi::c_void, riid: usize, ppv: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CDefFolderMenu_Create2 - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn CDefFolderMenu_Create2(pidlFolder: usize, hwnd: *mut core::ffi::c_void, cidl: u32, apidl: *mut core::ffi::c_void, psf: *mut core::ffi::c_void, lpfn: usize, nKeys: u32, ahkeys: *mut *mut core::ffi::c_void, ppcm: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SHRegCreateUSKeyA - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHRegCreateUSKeyA(arg0: *const i8, arg1: usize, arg2: usize, arg3: usize, arg4: u32) -> usize {
    0
}

/// SHRegCreateUSKeyW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHRegCreateUSKeyW(arg0: *const u16, arg1: usize, arg2: usize, arg3: usize, arg4: u32) -> usize {
    0
}

/// AssocCreate - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn AssocCreate(arg0: usize, arg1: usize, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// PathCreateFromUrlA - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn PathCreateFromUrlA(arg0: *const i8, arg1: *mut i8, arg2: usize, arg3: u32) -> usize {
    0
}

/// PathCreateFromUrlW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn PathCreateFromUrlW(arg0: *const u16, arg1: *mut u16, arg2: usize, arg3: u32) -> usize {
    0
}

/// UrlCreateFromPathA - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn UrlCreateFromPathA(arg0: *const i8, arg1: *mut i8, arg2: usize, arg3: u32) -> usize {
    0
}

/// UrlCreateFromPathW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn UrlCreateFromPathW(arg0: *const u16, arg1: *mut u16, arg2: usize, arg3: u32) -> usize {
    0
}

/// SHCreateShellPalette - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateShellPalette(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// SHCreateStreamOnFileA - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateStreamOnFileA(arg0: *const i8, arg1: u32, IStream: usize) -> usize {
    0
}

/// SHCreateStreamOnFileW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateStreamOnFileW(arg0: *const u16, arg1: u32, IStream: usize) -> usize {
    0
}

/// SHCreateMemStream - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateMemStream(arg0: *mut u8, arg1: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SHCreateStreamOnFileEx - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateStreamOnFileEx(arg0: *const u16, arg1: u32, arg2: u32, arg3: i32, IStream: usize, IStream_5: usize) -> usize {
    0
}

/// SHCreateStreamWrapper - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateStreamWrapper(arg0: usize, arg1: u32, arg2: u32, IStream: usize) -> usize {
    0
}

/// SHLockShared - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHLockShared(handle: *mut core::ffi::c_void, pid: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SHUnlockShared - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHUnlockShared(data: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateTextServices - from wine/textserv.h
#[no_mangle]
pub unsafe extern "C" fn CreateTextServices(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// ReleaseMutexWhenCallbackReturns - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn ReleaseMutexWhenCallbackReturns(arg0: usize, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateToolhelp32Snapshot - from wine/tlhelp32.h
#[no_mangle]
pub unsafe extern "C" fn CreateToolhelp32Snapshot(arg0: u32, arg1: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateEnvironmentBlock - from wine/userenv.h
#[no_mangle]
pub unsafe extern "C" fn CreateEnvironmentBlock(arg0: *mut *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: i32) -> usize {
    0
}

/// DestroyEnvironmentBlock - from wine/userenv.h
#[no_mangle]
pub unsafe extern "C" fn DestroyEnvironmentBlock(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// AVIStreamCreate - from wine/vfw.h
#[no_mangle]
pub unsafe extern "C" fn AVIStreamCreate(arg0: *mut core::ffi::c_void, arg1: i32, arg2: i32, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// CreateEditableStream - from wine/vfw.h
#[no_mangle]
pub unsafe extern "C" fn CreateEditableStream(ppEditable: *mut core::ffi::c_void, pSource: usize) -> i32 {
    0
}

/// AVIFileCreateStreamA - from wine/vfw.h
#[no_mangle]
pub unsafe extern "C" fn AVIFileCreateStreamA(pfile: usize, ppavi: *mut core::ffi::c_void, psi: *mut core::ffi::c_void) -> i32 {
    0
}

/// AVIFileCreateStreamW - from wine/vfw.h
#[no_mangle]
pub unsafe extern "C" fn AVIFileCreateStreamW(pfile: usize, ppavi: *mut core::ffi::c_void, psi: *mut core::ffi::c_void) -> i32 {
    0
}

/// MCIWndCreateA - from wine/vfw.h
#[no_mangle]
pub unsafe extern "C" fn MCIWndCreateA(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: u32, arg3: *const i8) -> usize {
    0
}

/// MCIWndCreateW - from wine/vfw.h
#[no_mangle]
pub unsafe extern "C" fn MCIWndCreateW(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: u32, arg3: *const u16) -> usize {
    0
}

/// capCreateCaptureWindowA - from wine/vfw.h
#[no_mangle]
pub unsafe extern "C" fn capCreateCaptureWindowA(arg0: *const i8, arg1: u32, arg2: i32, arg3: i32, arg4: i32, arg5: i32, arg6: *mut core::ffi::c_void, arg7: i32) -> usize {
    0
}

/// capCreateCaptureWindowW - from wine/vfw.h
#[no_mangle]
pub unsafe extern "C" fn capCreateCaptureWindowW(arg0: *const u16, arg1: u32, arg2: i32, arg3: i32, arg4: i32, arg5: i32, arg6: *mut core::ffi::c_void, arg7: i32) -> usize {
    0
}

/// WsCreateChannel - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsCreateChannel(arg0: usize, arg1: usize, arg2: *mut core::ffi::c_void, arg3: u32, arg4: *mut core::ffi::c_void, arg5: *mut *mut core::ffi::c_void, arg6: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsCreateChannelForListener - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsCreateChannelForListener(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: u32, arg3: *mut *mut core::ffi::c_void, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsCreateError - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsCreateError(arg0: *mut core::ffi::c_void, arg1: u32, arg2: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// WsCreateHeap - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsCreateHeap(arg0: usize, arg1: usize, arg2: *mut core::ffi::c_void, arg3: u32, arg4: *mut *mut core::ffi::c_void, arg5: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsCreateListener - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsCreateListener(arg0: usize, arg1: usize, arg2: *mut core::ffi::c_void, arg3: u32, arg4: *mut core::ffi::c_void, arg5: *mut *mut core::ffi::c_void, arg6: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsCreateMessage - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsCreateMessage(arg0: usize, arg1: usize, arg2: *mut core::ffi::c_void, arg3: u32, arg4: *mut *mut core::ffi::c_void, arg5: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsCreateMessageForChannel - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsCreateMessageForChannel(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: u32, arg3: *mut *mut core::ffi::c_void, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsCreateServiceProxy - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsCreateServiceProxy(arg0: usize, arg1: usize, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void, arg4: u32, arg5: *mut core::ffi::c_void, arg6: u32, arg7: *mut *mut core::ffi::c_void, arg8: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsCreateServiceProxyFromTemplate - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsCreateServiceProxyFromTemplate(arg0: usize, arg1: *mut core::ffi::c_void, arg2: u32, arg3: usize, arg4: *mut core::ffi::c_void, arg5: u32, arg6: *mut core::ffi::c_void, arg7: u32, arg8: *mut *mut core::ffi::c_void, arg9: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsCreateXmlBuffer - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsCreateXmlBuffer(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: u32, arg3: *mut *mut core::ffi::c_void, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// WebSocketCreateClientHandle - from wine/websocket.h
#[no_mangle]
pub unsafe extern "C" fn WebSocketCreateClientHandle(arg0: usize, arg1: u32, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// WerRegisterMemoryBlock - from wine/werapi.h
#[no_mangle]
pub unsafe extern "C" fn WerRegisterMemoryBlock(block: *mut core::ffi::c_void, size: u32) -> i32 {
    0
}

/// WerReportCreate - from wine/werapi.h
#[no_mangle]
pub unsafe extern "C" fn WerReportCreate(arg0: usize, arg1: usize, arg2: usize, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// WerUnregisterMemoryBlock - from wine/werapi.h
#[no_mangle]
pub unsafe extern "C" fn WerUnregisterMemoryBlock(block: *mut core::ffi::c_void) -> i32 {
    0
}

/// CreateConsoleScreenBuffer - from wine/wincon.h
#[no_mangle]
pub unsafe extern "C" fn CreateConsoleScreenBuffer(arg0: u32, arg1: u32, arg2: usize, arg3: u32, arg4: *mut core::ffi::c_void) -> usize {
    0
}

/// CryptCreateHash - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CryptCreateHash(arg0: usize, arg1: usize, arg2: usize, arg3: u32, arg4: *mut core::ffi::c_void) -> usize {
    0
}

/// CryptCreateAsyncHandle - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CryptCreateAsyncHandle(dwFlags: u32, phAsync: usize) -> usize {
    0
}

/// CertCreateCertificateChainEngine - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CertCreateCertificateChainEngine(pConfig: usize, phChainEngine: *mut core::ffi::c_void) -> usize {
    0
}

/// CertCreateContext - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CertCreateContext(dwContextType: u32, dwEncodingType: u32, pbEncoded: *mut u8, cbEncoded: u32, dwFlags: u32, pCreatePara: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CertCreateCertificateContext - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CertCreateCertificateContext(dwCertEncodingType: u32, pbCertEncoded: *mut u8, cbCertEncoded: u32) -> usize {
    0
}

/// CertCreateCRLContext - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CertCreateCRLContext(dwCertEncodingType: u32, pbCrlEncoded: *mut u8, cbCrlEncoded: u32) -> usize {
    0
}

/// CertCreateCTLContext - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CertCreateCTLContext(dwMsgAndCertEncodingType: u32, pbCtlEncoded: *mut u8, cbCtlEncoded: u32) -> usize {
    0
}

/// CertCreateSelfSignCertificate - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CertCreateSelfSignCertificate(hProv: usize, pSubjectIssuerBlob: usize, dwFlags: u32, pKeyProvInfo: usize, pSignatureAlgorithm: usize, pStartTime: usize, pEndTime: usize, pExtensions: usize) -> usize {
    0
}

/// EvtCreateBookmark - from wine/winevt.h
#[no_mangle]
pub unsafe extern "C" fn EvtCreateBookmark(arg0: *const u16) -> usize {
    0
}

/// EvtCreateRenderContext - from wine/winevt.h
#[no_mangle]
pub unsafe extern "C" fn EvtCreateRenderContext(arg0: u32, arg1: *mut *const u16, arg2: u32) -> usize {
    0
}

/// WinHttpCreateProxyResolver - from wine/winhttp.h
#[no_mangle]
pub unsafe extern "C" fn WinHttpCreateProxyResolver(arg0: usize, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// WinHttpCreateUrl - from wine/winhttp.h
#[no_mangle]
pub unsafe extern "C" fn WinHttpCreateUrl(arg0: usize, arg1: u32, arg2: *mut u16, arg3: usize) -> usize {
    0
}

/// InternetCreateUrlA - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn InternetCreateUrlA(arg0: usize, arg1: u32, arg2: *mut i8, arg3: usize) -> usize {
    0
}

/// InternetCreateUrlW - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn InternetCreateUrlW(arg0: usize, arg1: u32, arg2: *mut u16, arg3: usize) -> usize {
    0
}

/// InternetLockRequestFile - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn InternetLockRequestFile(arg0: usize, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// InternetUnlockRequestFile - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn InternetUnlockRequestFile(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// FtpCreateDirectoryA - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn FtpCreateDirectoryA(arg0: usize, arg1: *const i8) -> usize {
    0
}

/// FtpCreateDirectoryW - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn FtpCreateDirectoryW(arg0: usize, arg1: *const u16) -> usize {
    0
}

/// GopherCreateLocatorA - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn GopherCreateLocatorA(arg0: *const i8, arg1: usize, arg2: *const i8, arg3: *const i8, arg4: u32, arg5: *mut i8, arg6: usize) -> usize {
    0
}

/// GopherCreateLocatorW - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn GopherCreateLocatorW(arg0: *const u16, arg1: usize, arg2: *const u16, arg3: *const u16, arg4: u32, arg5: *mut u16, arg6: usize) -> usize {
    0
}

/// CreateUrlCacheEntryA - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn CreateUrlCacheEntryA(arg0: *const i8, arg1: u32, arg2: *const i8, arg3: *mut i8, arg4: u32) -> usize {
    0
}

/// CreateUrlCacheEntryW - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn CreateUrlCacheEntryW(arg0: *const u16, arg1: u32, arg2: *const u16, arg3: *mut u16, arg4: u32) -> usize {
    0
}

/// UnlockUrlCacheEntryFileA - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn UnlockUrlCacheEntryFileA(arg0: *const i8, arg1: u32) -> usize {
    0
}

/// UnlockUrlCacheEntryFileW - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn UnlockUrlCacheEntryFileW(arg0: *const u16, arg1: u32) -> usize {
    0
}

/// UnlockUrlCacheEntryStream - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn UnlockUrlCacheEntryStream(arg0: *mut core::ffi::c_void, arg1: u32) -> usize {
    0
}

/// CreateUrlCacheGroup - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn CreateUrlCacheGroup(arg0: u32, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateMD5SSOHash - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn CreateMD5SSOHash(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// ldap_create_page_controlA - from wine/winldap.h
#[no_mangle]
pub unsafe extern "C" fn ldap_create_page_controlA(arg0: usize, arg1: u32, berval: usize, arg3: u8, arg4: *mut core::ffi::c_void) -> u32 {
    0
}

/// ldap_create_page_controlW - from wine/winldap.h
#[no_mangle]
pub unsafe extern "C" fn ldap_create_page_controlW(arg0: usize, arg1: u32, berval: usize, arg3: u8, arg4: *mut core::ffi::c_void) -> u32 {
    0
}

/// ldap_create_sort_controlA - from wine/winldap.h
#[no_mangle]
pub unsafe extern "C" fn ldap_create_sort_controlA(arg0: usize, arg1: *mut core::ffi::c_void, arg2: u8, arg3: *mut core::ffi::c_void) -> u32 {
    0
}

/// ldap_create_sort_controlW - from wine/winldap.h
#[no_mangle]
pub unsafe extern "C" fn ldap_create_sort_controlW(arg0: usize, arg1: *mut core::ffi::c_void, arg2: u8, arg3: *mut core::ffi::c_void) -> u32 {
    0
}

/// ldap_create_vlv_controlA - from wine/winldap.h
#[no_mangle]
pub unsafe extern "C" fn ldap_create_vlv_controlA(arg0: usize, arg1: usize, arg2: u8, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// ldap_create_vlv_controlW - from wine/winldap.h
#[no_mangle]
pub unsafe extern "C" fn ldap_create_vlv_controlW(arg0: usize, arg1: usize, arg2: u8, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// RtlInterlockedFlushSList - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn RtlInterlockedFlushSList(arg0: usize) -> usize {
    0
}

/// RtlInterlockedPopEntrySList - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn RtlInterlockedPopEntrySList(arg0: usize) -> usize {
    0
}

/// RtlInterlockedPushEntrySList - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn RtlInterlockedPushEntrySList(arg0: usize, arg1: usize) -> usize {
    0
}

/// _InterlockedAnd - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedAnd(arg0: *mut i64, arg1: i64) -> i64 {
    0
}

/// _InterlockedCompareExchange - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedCompareExchange(arg0: *mut i64, arg1: i64, arg2: i64) -> i64 {
    0
}

/// _InterlockedCompareExchange64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedCompareExchange64(arg0: *mut i64, arg1: i64, arg2: i64) -> i64 {
    0
}

/// _InterlockedCompareExchangePointer - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedCompareExchangePointer(arg0: *mut *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _InterlockedDecrement - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedDecrement(arg0: *mut i64) -> i64 {
    0
}

/// _InterlockedDecrement16 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedDecrement16(arg0: *mut i16) -> i16 {
    0
}

/// _InterlockedExchange - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedExchange(arg0: *mut i64, arg1: i64) -> i64 {
    0
}

/// _InterlockedExchangeAdd - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedExchangeAdd(arg0: *mut i64, arg1: i64) -> i64 {
    0
}

/// _InterlockedExchangeAdd16 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedExchangeAdd16(arg0: *mut i16, arg1: i16) -> i16 {
    0
}

/// _InterlockedExchangePointer - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedExchangePointer(arg0: *mut *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _InterlockedIncrement - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedIncrement(arg0: *mut i64) -> i64 {
    0
}

/// _InterlockedIncrement16 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedIncrement16(arg0: *mut i16) -> i16 {
    0
}

/// _InterlockedOr - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedOr(arg0: *mut i64, arg1: i64) -> i64 {
    0
}

/// _InterlockedXor - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedXor(arg0: *mut i64, arg1: i64) -> i64 {
    0
}

/// _InterlockedAnd64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedAnd64(arg0: *mut core::ffi::c_void, arg1: usize) -> usize {
    0
}

/// InterlockedAnd64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedAnd64(dest: *mut core::ffi::c_void, val: usize) -> usize {
    0
}

/// _InterlockedExchangeAdd64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedExchangeAdd64(arg0: *mut core::ffi::c_void, arg1: usize) -> usize {
    0
}

/// InterlockedExchangeAdd64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedExchangeAdd64(dest: *mut core::ffi::c_void, val: usize) -> usize {
    0
}

/// _InterlockedDecrement64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedDecrement64(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// InterlockedDecrement64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedDecrement64(dest: *mut core::ffi::c_void) -> usize {
    0
}

/// _InterlockedIncrement64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedIncrement64(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// InterlockedIncrement64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedIncrement64(dest: *mut core::ffi::c_void) -> usize {
    0
}

/// _InterlockedOr64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedOr64(arg0: *mut core::ffi::c_void, arg1: usize) -> usize {
    0
}

/// InterlockedOr64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedOr64(dest: *mut core::ffi::c_void, val: usize) -> usize {
    0
}

/// _InterlockedXor64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedXor64(arg0: *mut core::ffi::c_void, arg1: usize) -> usize {
    0
}

/// InterlockedXor64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedXor64(dest: *mut core::ffi::c_void, val: usize) -> usize {
    0
}

/// InterlockedAdd - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedAdd(dest: *mut i64, val: i64) -> usize {
    0
}

/// InterlockedExchangeAdd - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedExchangeAdd(arg0: usize, arg1: usize) -> usize {
    0
}

/// InterlockedAdd64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedAdd64(dest: *mut core::ffi::c_void, val: usize) -> usize {
    0
}

/// InterlockedAnd - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedAnd(dest: *mut i32, val: i32) -> usize {
    0
}

/// InterlockedCompareExchange - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedCompareExchange(dest: *mut i32, xchg: i32, compare: i32) -> usize {
    0
}

/// InterlockedCompareExchangePointer - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedCompareExchangePointer(dest: *mut *mut core::ffi::c_void, xchg: *mut core::ffi::c_void, compare: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// InterlockedCompareExchange64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedCompareExchange64(dest: *mut i64, xchg: i64, compare: i64) -> usize {
    0
}

/// InterlockedExchange - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedExchange(dest: *mut i32, val: i32) -> usize {
    0
}

/// InterlockedExchangeAdd16 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedExchangeAdd16(dest: *mut i16, incr: i16) -> usize {
    0
}

/// InterlockedIncrement - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedIncrement(dest: *mut i32) -> usize {
    0
}

/// InterlockedIncrement16 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedIncrement16(dest: *mut i16) -> usize {
    0
}

/// InterlockedDecrement - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedDecrement(dest: *mut i32) -> usize {
    0
}

/// InterlockedDecrement16 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedDecrement16(dest: *mut i16) -> usize {
    0
}

/// InterlockedExchangePointer - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedExchangePointer(dest: *mut *mut core::ffi::c_void, val: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// InterlockedOr - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedOr(dest: *mut i32, val: i32) -> usize {
    0
}

/// InterlockedXor - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedXor(dest: *mut i32, val: i32) -> usize {
    0
}

/// _InterlockedCompareExchange128 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _InterlockedCompareExchange128(arg0: *mut core::ffi::c_void, arg1: usize, arg2: usize, arg3: *mut core::ffi::c_void) -> u8 {
    0
}

/// InterlockedCompareExchange128 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn InterlockedCompareExchange128(dest: *mut core::ffi::c_void, xchg_high: usize, xchg_low: usize, compare: *mut core::ffi::c_void) -> usize {
    0
}

/// cancel_blocking - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn cancel_blocking(process: *mut core::ffi::c_void, thread: *mut core::ffi::c_void, iosb: usize) -> i32 {
    0
}

/// create_inode - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_inode(arg0: usize, arg1: usize) -> usize {
    0
}

/// create_anonymous_fd - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_anonymous_fd(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// create_console_connection - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_console_connection(arg0: usize) -> usize {
    0
}

/// create_screen_buffer - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_screen_buffer(arg0: usize) -> usize {
    0
}

/// create_console_server - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_console_server() -> usize {
    0
}

/// create_named_object - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_named_object(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// d3dkmt_mutex_dump - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn d3dkmt_mutex_dump(obj: *mut core::ffi::c_void, verbose: i32) {

}

/// d3dkmt_mutex_destroy - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn d3dkmt_mutex_destroy(obj: *mut core::ffi::c_void) {

}

/// mutex_grab - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mutex_grab(mutex: *mut core::ffi::c_void) {

}

/// mutex_release - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mutex_release(mutex: *mut core::ffi::c_void, abandon: usize) {

}

/// abandon_d3dkmt_mutexes - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn abandon_d3dkmt_mutexes(thread: *mut core::ffi::c_void) {

}

/// fill_create_process_event - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn fill_create_process_event(event: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) {

}

/// create_session - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_session(id: u32) {

}

/// file_lock_dump - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn file_lock_dump(obj: *mut core::ffi::c_void, verbose: i32) {

}

/// file_lock_destroy - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn file_lock_destroy(obj: *mut core::ffi::c_void) {

}

/// set_unix_lock - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn set_unix_lock(fd: *mut core::ffi::c_void, start: usize, end: usize, arg3: i32) -> i32 {
    0
}

/// lock_overlaps - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn lock_overlaps(lock: *mut core::ffi::c_void, start: usize, end: usize) -> i32 {
    0
}

/// remove_unix_locks - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn remove_unix_locks(fd: *mut core::ffi::c_void, start: usize, end: usize) {

}

/// remove_lock - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn remove_lock(lock: *mut core::ffi::c_void, remove_unix: i32) {

}

/// remove_process_locks - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn remove_process_locks(process: *mut core::ffi::c_void) {

}

/// remove_fd_locks - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn remove_fd_locks(fd: *mut core::ffi::c_void) {

}

/// lock_fd - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn lock_fd(fd: *mut core::ffi::c_void, start: usize, count: usize, shared: i32, wait: i32) -> usize {
    0
}

/// unlock_fd - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn unlock_fd(fd: *mut core::ffi::c_void, start: usize, count: usize) {

}

/// abandon_inproc_mutexes - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn abandon_inproc_mutexes(tid: usize) {

}

/// __pthread_kill - from glibc/pthread.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_kill(threadid: usize, signo: i32) -> i32 {
    0
}

/// create_temp_file - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_temp_file(size: usize) -> i32 {
    0
}

/// create_file_for_fd_obj - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_file_for_fd_obj(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// create_mapping - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_mapping(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize, arg7: usize) -> usize {
    0
}

/// mutex_sync_dump - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mutex_sync_dump(obj: *mut core::ffi::c_void, verbose: i32) {

}

/// mutex_sync_signaled - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mutex_sync_signaled(obj: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) -> i32 {
    0
}

/// mutex_sync_satisfied - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mutex_sync_satisfied(obj: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) {

}

/// mutex_sync_destroy - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mutex_sync_destroy(obj: *mut core::ffi::c_void) {

}

/// mutex_dump - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mutex_dump(obj: *mut core::ffi::c_void, verbose: i32) {

}

/// mutex_signal - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mutex_signal(obj: *mut core::ffi::c_void, access: u32, signal: i32) -> i32 {
    0
}

/// mutex_destroy - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mutex_destroy(obj: *mut core::ffi::c_void) {

}

/// abandon_mutexes - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn abandon_mutexes(thread: *mut core::ffi::c_void) {

}

/// mark_block_noaccess - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mark_block_noaccess(ptr: *mut core::ffi::c_void, size: usize) {

}

/// mark_block_uninitialized - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mark_block_uninitialized(ptr: *mut core::ffi::c_void, size: usize) {

}

/// lock_input_keystate - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn lock_input_keystate(input: *mut core::ffi::c_void) {

}

/// unlock_input_keystate - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn unlock_input_keystate(input: *mut core::ffi::c_void) {

}

/// create_key_recursive - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_key_recursive(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// create_server_lock - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_server_lock() -> i32 {
    0
}

/// wait_for_lock - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn wait_for_lock() -> i32 {
    0
}

/// kill_lock_owner - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn kill_lock_owner(sig: i32) -> i32 {
    0
}

/// acquire_lock - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn acquire_lock() {

}

/// create_file_directories - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_file_directories(make: *mut core::ffi::c_void, files: usize) {

}

/// create_image_list - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_image_list(resource: u32, width: u32, height: u32, count: u32, mask_color: u32) -> usize {
    0
}

/// ADsDNWithBinary_create - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ADsDNWithBinary_create(riid: usize, obj: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// Pathname_create - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn Pathname_create(riid: usize, obj: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// factory_LockServer - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn factory_LockServer(iface: *mut core::ffi::c_void, lock: i32) -> i32 {
    0
}

/// LDAPNamespace_create - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn LDAPNamespace_create(riid: usize, obj: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// LDAP_create - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn LDAP_create(riid: usize, obj: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// ADSystemInfo_create - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ADSystemInfo_create(riid: usize, obj: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// dirobj_CreateDSObject - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn dirobj_CreateDSObject(iface: *mut core::ffi::c_void, name: *mut u16, attrs: usize, count: u32, obj: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// RegCreateKeyExW - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn RegCreateKeyExW(arg0: usize, Manager: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> usize {
    0
}

/// CRYPT_CreateMachineGuid - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn CRYPT_CreateMachineGuid() {

}

/// RegCreateKeyW - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn RegCreateKeyW(hkey: *mut core::ffi::c_void, lpSubKey: *const u16, phkResult: usize) -> i32 {
    0
}

/// RegCreateKeyA - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn RegCreateKeyA(hkey: *mut core::ffi::c_void, lpSubKey: *const i8, phkResult: usize) -> i32 {
    0
}

/// RegCreateKeyExA - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn RegCreateKeyExA(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize, arg7: usize, arg8: usize) -> usize {
    0
}

/// SaferCreateLevel - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn SaferCreateLevel(ScopeId: u32, LevelId: u32, OpenFlags: u32, LevelHandle: *mut core::ffi::c_void, lpReserved: *mut core::ffi::c_void) -> i32 {
    0
}

/// LockServiceDatabase - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn LockServiceDatabase(manager: usize) -> usize {
    0
}

/// UnlockServiceDatabase - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn UnlockServiceDatabase(lock: usize) -> i32 {
    0
}

/// QueryServiceLockStatusA - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn QueryServiceLockStatusA(hSCManager: usize, lpLockStatus: usize, cbBufSize: u32, pcbBytesNeeded: usize) -> i32 {
    0
}

/// QueryServiceLockStatusW - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn QueryServiceLockStatusW(hSCManager: usize, lpLockStatus: usize, cbBufSize: u32, pcbBytesNeeded: usize) -> i32 {
    0
}

/// WmiOpenBlock - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn WmiOpenBlock(guid: *mut core::ffi::c_void, access: u32, handle: *mut core::ffi::c_void) -> u32 {
    0
}

/// create_tmp_ini_file - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_tmp_ini_file(hm: *mut core::ffi::c_void, ini_file: *mut u16) -> i32 {
    0
}

/// AMAudioData_create - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AMAudioData_create(pUnkOuter: *mut core::ffi::c_void, ppObj: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// audiostreamsample_create - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audiostreamsample_create(parent: *mut core::ffi::c_void, audio_data: *mut core::ffi::c_void, audio_stream_sample: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_IAMMediaStream_CreateSharedSample - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_IAMMediaStream_CreateSharedSample(iface: *mut core::ffi::c_void, existing_sample: *mut core::ffi::c_void, flags: u32, sample: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_IAudioMediaStream_CreateSharedSample - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_IAudioMediaStream_CreateSharedSample(iface: *mut core::ffi::c_void, existing_sample: *mut core::ffi::c_void, flags: u32, sample: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// IAMMediaStream_CreateSharedSample - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAMMediaStream_CreateSharedSample(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// audio_IAudioMediaStream_CreateSample - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_IAudioMediaStream_CreateSample(iface: *mut core::ffi::c_void, audio_data: *mut core::ffi::c_void, flags: u32, sample: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_meminput_ReceiveCanBlock - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_meminput_ReceiveCanBlock(iface: *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_stream_create - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_stream_create(outer: *mut core::ffi::c_void, out: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// ddrawstreamsample_create - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddrawstreamsample_create(parent: *mut core::ffi::c_void, surface: *mut core::ffi::c_void, rect: *mut core::ffi::c_void, ddraw_stream_sample: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_IAMMediaStream_CreateSharedSample - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_IAMMediaStream_CreateSharedSample(iface: *mut core::ffi::c_void, existing_sample: *mut core::ffi::c_void, flags: u32, sample: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_IDirectDrawMediaStream_CreateSharedSample - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_IDirectDrawMediaStream_CreateSharedSample(iface: *mut core::ffi::c_void, existing_sample: *mut core::ffi::c_void, flags: u32, sample: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_IDirectDrawMediaStream_CreateSample - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_IDirectDrawMediaStream_CreateSample(iface: *mut core::ffi::c_void, surface: *mut core::ffi::c_void, rect: *mut core::ffi::c_void, flags: u32, sample: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_meminput_ReceiveCanBlock - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_meminput_ReceiveCanBlock(iface: *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_stream_create - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_stream_create(outer: *mut core::ffi::c_void, out: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// filter_create - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn filter_create(outer: *mut core::ffi::c_void, out: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// AMCF_LockServer - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AMCF_LockServer(iface: *mut core::ffi::c_void, dolock: i32) -> i32 {
    0
}

/// create_graph - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create_graph(mmstream: *mut core::ffi::c_void, graph: *mut core::ffi::c_void) -> i32 {
    0
}

/// multimedia_stream_create - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn multimedia_stream_create(outer: *mut core::ffi::c_void, out: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// SdbCreateDatabase - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn SdbCreateDatabase(path: *const u16, arg1: usize) -> usize {
    0
}

/// created - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn created(arg0: usize) -> usize {
    0
}

/// RegistrarCF_LockServer - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn RegistrarCF_LockServer(iface: *mut core::ffi::c_void, lock: i32) -> i32 {
    0
}

/// OleContainer_LockContainer - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn OleContainer_LockContainer(iface: *mut core::ffi::c_void, fLock: i32) -> i32 {
    0
}

/// OleControlSite_LockInPlaceActive - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn OleControlSite_LockInPlaceActive(This: *mut core::ffi::c_void, fLock: i32) -> i32 {
    0
}

/// IOCS_Create - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IOCS_Create(hWnd: *mut core::ffi::c_void, pUnkControl: *mut core::ffi::c_void, container: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// AtlAxCreateControlLicEx - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AtlAxCreateControlLicEx(lpszName: usize, hWnd: *mut core::ffi::c_void, pStream: *mut core::ffi::c_void, ppUnkContainer: *mut *mut core::ffi::c_void, ppUnkControl: *mut *mut core::ffi::c_void, iidSink: usize, punkSink: *mut core::ffi::c_void, lic: usize) -> i32 {
    0
}

/// PUT_BLOCK - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn PUT_BLOCK(arg0: usize, arg1: usize) -> usize {
    0
}

/// AtlAxCreateControlLic - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AtlAxCreateControlLic(lpTricsData: *mut u16, hwnd: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, container: *mut *mut core::ffi::c_void, lic: usize) -> i32 {
    0
}

/// create - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn create(arg0: usize) -> usize {
    0
}

/// CreateWindowW - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn CreateWindowW(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize, arg7: usize, arg8: usize, arg9: usize, arg10: usize) -> usize {
    0
}

/// ACMStream_fnCreate - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ACMStream_fnCreate(iface: *mut core::ffi::c_void, lParam1: isize, lParam2: isize) -> i32 {
    0
}

/// AVIFILE_CreateACMStream - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AVIFILE_CreateACMStream(riid: usize, ppv: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// IAVIFile_CreateStream - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIFile_CreateStream(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// AVIFILE_SamplesToBlock - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AVIFILE_SamplesToBlock(This: *mut core::ffi::c_void, pos: usize, offset: usize) {

}

/// IAVIFile_fnCreateStream - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIFile_fnCreateStream(iface: *mut core::ffi::c_void, avis: *mut *mut core::ffi::c_void, asi: *mut core::ffi::c_void) -> i32 {
    0
}

/// AVIFILE_CreateAVIFile - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AVIFILE_CreateAVIFile(pUnkOuter: *mut core::ffi::c_void, riid: usize, ppv: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// IAVIStream_fnCreate - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIStream_fnCreate(iface: *mut core::ffi::c_void, lParam1: isize, lParam2: isize) -> i32 {
    0
}

/// IEditAVIStream_fnCreate - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IEditAVIStream_fnCreate(arg0: *mut core::ffi::c_void, lParam1: isize, lParam2: isize) -> i32 {
    0
}

/// AVIFILE_CreateWAVFile - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AVIFILE_CreateWAVFile(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// AVIFILE_CreateICMStream - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AVIFILE_CreateICMStream(arg0: usize, arg1: usize) -> usize {
    0
}

/// IClassFactory_fnLockServer - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IClassFactory_fnLockServer(iface: *mut core::ffi::c_void, dolock: i32) -> i32 {
    0
}

/// AVIFILE_CreateClassFactory - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AVIFILE_CreateClassFactory(clsid: *mut core::ffi::c_void, riid: *mut core::ffi::c_void, ppv: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// Create - from reactos/GridView.h
#[no_mangle]
pub unsafe extern "C" fn Create(hParent: usize) -> usize {
    0
}

/// OnCreate - from reactos/GridView.h
#[no_mangle]
pub unsafe extern "C" fn OnCreate(hwnd: usize, hParent: usize) -> isize {
    0
}

/// CreateStatusBar - from reactos/MainWindow.h
#[no_mangle]
pub unsafe extern "C" fn CreateStatusBar() -> usize {
    0
}

/// CreateFontComboBox - from reactos/MainWindow.h
#[no_mangle]
pub unsafe extern "C" fn CreateFontComboBox() -> usize {
    0
}

/// CreateImeMenu - from reactos/imemenu.h
#[no_mangle]
pub unsafe extern "C" fn CreateImeMenu(hIMC: usize, lpImeParentMenu: usize, bRightMenu: usize) -> usize {
    0
}

/// CreateConsoleWindow - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn CreateConsoleWindow(OPTIONAL: usize, nCmdShow: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateDIBWithProperties - from reactos/dib.h
#[no_mangle]
pub unsafe extern "C" fn CreateDIBWithProperties(width: i32, height: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateMonoBitmap - from reactos/dib.h
#[no_mangle]
pub unsafe extern "C" fn CreateMonoBitmap(width: i32, height: i32, bWhite: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateColorDIB - from reactos/dib.h
#[no_mangle]
pub unsafe extern "C" fn CreateColorDIB(width: i32, height: i32, rgb: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// DoCreate - from reactos/fullscreen.h
#[no_mangle]
pub unsafe extern "C" fn DoCreate() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// LockBitmap - from reactos/history.h
#[no_mangle]
pub unsafe extern "C" fn LockBitmap() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// UnlockBitmap - from reactos/history.h
#[no_mangle]
pub unsafe extern "C" fn UnlockBitmap(hbmLocked: *mut core::ffi::c_void) {

}

/// createToolObject - from reactos/toolsmodel.h
#[no_mangle]
pub unsafe extern "C" fn createToolObject(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// disk_create_notify - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn disk_create_notify(handle: usize, info_class: usize) -> usize {
    0
}

/// rd_create_ui - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn rd_create_ui() {

}

/// rd_lock_file - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn rd_lock_file(fd: i32, start: i32, len: i32) -> usize {
    0
}

/// ui_get_numlock_state - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn ui_get_numlock_state(state: u32) -> usize {
    0
}

/// ui_create_window - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn ui_create_window() -> usize {
    0
}

/// ui_create_bitmap - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn ui_create_bitmap(width: i32, height: i32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// ui_create_glyph - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn ui_create_glyph(width: i32, height: i32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// ui_create_cursor - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn ui_create_cursor(x: u32, y: u32, width: i32, height: i32, andmask: *mut core::ffi::c_void, xormask: *mut core::ffi::c_void, bpp: i32) -> usize {
    0
}

/// ui_create_colourmap - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn ui_create_colourmap(colours: *mut core::ffi::c_void) -> usize {
    0
}

/// ui_seamless_create_window - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn ui_seamless_create_window(id: u64, group: u64, parent: u64, flags: u64) {

}

/// scard_lock - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn scard_lock(lock: i32) {

}

/// scard_unlock - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn scard_unlock(lock: i32) {

}

/// DoCreateEditWindow - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DoCreateEditWindow(arg0: usize) -> usize {
    0
}

/// OSK_Create - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn OSK_Create(hwnd: *mut core::ffi::c_void) -> isize {
    0
}

/// CreateListView - from reactos/main.h
#[no_mangle]
pub unsafe extern "C" fn CreateListView(hwndParent: *mut core::ffi::c_void, id: *mut core::ffi::c_void, cx: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateTreeView - from reactos/main.h
#[no_mangle]
pub unsafe extern "C" fn CreateTreeView(hwndParent: *mut core::ffi::c_void, pHostName: *mut u16, id: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateNewKey - from reactos/main.h
#[no_mangle]
pub unsafe extern "C" fn CreateNewKey(hwndTV: *mut core::ffi::c_void, hItem: usize) -> i32 {
    0
}

/// ParseCreateConfigArguments - from reactos/sc.h
#[no_mangle]
pub unsafe extern "C" fn ParseCreateConfigArguments(ServiceArgs: *mut core::ffi::c_void, ArgCount: i32, bChangeService: i32, lpServiceInfo: usize) -> i32 {
    0
}

/// CreateUsage - from reactos/sc.h
#[no_mangle]
pub unsafe extern "C" fn CreateUsage(arg0: usize) -> usize {
    0
}

/// SndMixerCreate - from reactos/sndvol32.h
#[no_mangle]
pub unsafe extern "C" fn SndMixerCreate(hWndNotification: *mut core::ffi::c_void, MixerId: u32) -> usize {
    0
}

/// GraphCtrl_Create - from reactos/graphctl.h
#[no_mangle]
pub unsafe extern "C" fn GraphCtrl_Create(inst: usize, hWnd: *mut core::ffi::c_void, hParentWnd: *mut core::ffi::c_void, fmt: usize) -> i32 {
    0
}

/// ShutDown_LockComputer - from reactos/shutdown.h
#[no_mangle]
pub unsafe extern "C" fn ShutDown_LockComputer(arg0: usize) -> usize {
    0
}

/// MACRO_CreateButton - from reactos/macro.h
#[no_mangle]
pub unsafe extern "C" fn MACRO_CreateButton(arg0: *const i8, arg1: *const i8, arg2: *const i8) {

}

/// WINHELP_CreateHelpWindow - from reactos/winhelp.h
#[no_mangle]
pub unsafe extern "C" fn WINHELP_CreateHelpWindow(arg0: *mut core::ffi::c_void, arg1: i32, arg2: i32) -> i32 {
    0
}

/// WINHELP_CreateIndexWindow - from reactos/winhelp.h
#[no_mangle]
pub unsafe extern "C" fn WINHELP_CreateIndexWindow(arg0: i32) -> i32 {
    0
}

/// CreateSol - from reactos/solitaire.h
#[no_mangle]
pub unsafe extern "C" fn CreateSol() {

}

/// CreateSpider - from reactos/spider.h
#[no_mangle]
pub unsafe extern "C" fn CreateSpider() {

}

/// CreateEventDetailsCtrl - from reactos/evtdetctl.h
#[no_mangle]
pub unsafe extern "C" fn CreateEventDetailsCtrl(hInstance: *mut core::ffi::c_void, hParentWnd: *mut core::ffi::c_void, lParam: isize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateMainWindow - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn CreateMainWindow(lpCaption: usize, nCmdShow: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateRootContext - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn CreateRootContext(arg0: usize) -> i32 {
    0
}

/// CreateRootHelper - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn CreateRootHelper(arg0: usize) -> u32 {
    0
}

/// get_lock_linewrap - from reactos/tnconfig.h
#[no_mangle]
pub unsafe extern "C" fn get_lock_linewrap() -> usize {
    0
}

/// CreateHiddenConsoleProcess - from reactos/tnmisc.h
#[no_mangle]
pub unsafe extern "C" fn CreateHiddenConsoleProcess(szChildName: usize, ppi: *mut core::ffi::c_void, phInWrite: usize, phOutRead: usize, phErrRead: usize) -> i32 {
    0
}

/// CreateInstalledAppByRegistryKey - from reactos/appdb.h
#[no_mangle]
pub unsafe extern "C" fn CreateInstalledAppByRegistryKey(KeyName: *const u16, hKeyParent: *mut core::ffi::c_void, KeyIndex: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateSearchBar - from reactos/appview.h
#[no_mangle]
pub unsafe extern "C" fn CreateSearchBar() -> i32 {
    0
}

/// CreateComboBox - from reactos/appview.h
#[no_mangle]
pub unsafe extern "C" fn CreateComboBox() -> i32 {
    0
}

/// CreateHSplitter - from reactos/appview.h
#[no_mangle]
pub unsafe extern "C" fn CreateHSplitter() -> i32 {
    0
}

/// CreateAppInfoDisplay - from reactos/appview.h
#[no_mangle]
pub unsafe extern "C" fn CreateAppInfoDisplay() -> i32 {
    0
}

/// CreateSettingsDlg - from reactos/dialogs.h
#[no_mangle]
pub unsafe extern "C" fn CreateSettingsDlg(hwnd: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateApplicationView - from reactos/gui.h
#[no_mangle]
pub unsafe extern "C" fn CreateApplicationView() -> i32 {
    0
}

/// CreateVSplitter - from reactos/gui.h
#[no_mangle]
pub unsafe extern "C" fn CreateVSplitter() -> i32 {
    0
}

/// CreateLayout - from reactos/gui.h
#[no_mangle]
pub unsafe extern "C" fn CreateLayout() -> i32 {
    0
}

/// CreateDirectoryTree - from reactos/misc.h
#[no_mangle]
pub unsafe extern "C" fn CreateDirectoryTree(Dir: *const u16) -> u32 {
    0
}

/// CreateCaret - from reactos/ciccaret.h
#[no_mangle]
pub unsafe extern "C" fn CreateCaret(hWnd: *mut core::ffi::c_void, size: usize) {

}

/// unlock - from reactos/cicimc.h
#[no_mangle]
pub unsafe extern "C" fn unlock() {

}

/// _LockIMCC - from reactos/cicimc.h
#[no_mangle]
pub unsafe extern "C" fn _LockIMCC(hIMCC: usize, pptr: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// _UnlockIMCC - from reactos/cicimc.h
#[no_mangle]
pub unsafe extern "C" fn _UnlockIMCC(hIMCC: usize) -> i32 {
    0
}

/// _LockIMC - from reactos/cicimc.h
#[no_mangle]
pub unsafe extern "C" fn _LockIMC(hIMC: usize, ppIC: *mut core::ffi::c_void) -> i32 {
    0
}

/// _UnlockIMC - from reactos/cicimc.h
#[no_mangle]
pub unsafe extern "C" fn _UnlockIMC(hIMC: usize) -> i32 {
    0
}

/// _cicRegKey_Create - from reactos/cicreg.h
#[no_mangle]
pub unsafe extern "C" fn _cicRegKey_Create(arg0: usize, hKey: *mut core::ffi::c_void, lpSubKey: usize) -> usize {
    0
}

/// cicCreateDitherBrush - from reactos/cicuif.h
#[no_mangle]
pub unsafe extern "C" fn cicCreateDitherBrush(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cicCreateDisabledBitmap - from reactos/cicuif.h
#[no_mangle]
pub unsafe extern "C" fn cicCreateDisabledBitmap(prc: usize, hbmMask: *mut core::ffi::c_void, hbr1: *mut core::ffi::c_void, hbr2: *mut core::ffi::c_void, bPressed: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cicCreateShadowMaskBmp - from reactos/cicuif.h
#[no_mangle]
pub unsafe extern "C" fn cicCreateShadowMaskBmp(prc: usize, hbm1: *mut core::ffi::c_void, hbm2: *mut core::ffi::c_void, hbr1: *mut core::ffi::c_void, hbr2: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cicCreateMaskBmp - from reactos/cicuif.h
#[no_mangle]
pub unsafe extern "C" fn cicCreateMaskBmp(prc: usize, hbm1: *mut core::ffi::c_void, hbm2: *mut core::ffi::c_void, hbr: *mut core::ffi::c_void, rgbColor: u32, rgbBack: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateScheme - from reactos/cicuif.h
#[no_mangle]
pub unsafe extern "C" fn CreateScheme() {

}

/// CreateRegion - from reactos/cicuif.h
#[no_mangle]
pub unsafe extern "C" fn CreateRegion(prc: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateWnd - from reactos/CLoaderWnd.h
#[no_mangle]
pub unsafe extern "C" fn CreateWnd() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateContext - from reactos/documentmgr.h
#[no_mangle]
pub unsafe extern "C" fn CreateContext(tidOwner: usize, dwFlags: u32, punk: *mut core::ffi::c_void, ppic: *mut *mut core::ffi::c_void, pecTextStore: *mut core::ffi::c_void) -> usize {
    0
}

/// InatCreateIcon - from reactos/mlng.h
#[no_mangle]
pub unsafe extern "C" fn InatCreateIcon(LangID: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// InatCreateIconBySize - from reactos/mlng.h
#[no_mangle]
pub unsafe extern "C" fn InatCreateIconBySize(LangID: usize, nWidth: usize, nHeight: usize, plf: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// EnumCreateInputContextCallback - from reactos/bridge.h
#[no_mangle]
pub unsafe extern "C" fn EnumCreateInputContextCallback(hIMC: usize, lParam: isize) -> i32 {
    0
}

/// CreateInputContext - from reactos/bridge.h
#[no_mangle]
pub unsafe extern "C" fn CreateInputContext(pTLS: *mut core::ffi::c_void, hIMC: usize) -> i32 {
    0
}

/// CreateDefFrameWnd - from reactos/ui.h
#[no_mangle]
pub unsafe extern "C" fn CreateDefFrameWnd(hwndParent: *mut core::ffi::c_void, hIMC: usize) -> i32 {
    0
}

/// CreateCompButtonWnd - from reactos/ui.h
#[no_mangle]
pub unsafe extern "C" fn CreateCompButtonWnd(hwndParent: *mut core::ffi::c_void, hIMC: usize) -> i32 {
    0
}

/// CreateCompositionWindow - from reactos/ui.h
#[no_mangle]
pub unsafe extern "C" fn CreateCompositionWindow(imcLock: usize, hwndParent: *mut core::ffi::c_void) -> i32 {
    0
}

/// _Create - from reactos/ui.h
#[no_mangle]
pub unsafe extern "C" fn _Create() -> i32 {
    0
}

/// CreateDeviceDescriptor - from reactos/audiosrv.h
#[no_mangle]
pub unsafe extern "C" fn CreateDeviceDescriptor(path: *mut u16, is_enabled: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateAudioDeviceList - from reactos/audiosrv.h
#[no_mangle]
pub unsafe extern "C" fn CreateAudioDeviceList(max_size: u32) -> i32 {
    0
}

/// print_create_attributes - from reactos/daemon_debug.h
#[no_mangle]
pub unsafe extern "C" fn print_create_attributes(level: i32, create_opts: u32) {

}

/// nfs41_idmap_create - from reactos/idmap.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_idmap_create(context_out: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// nfs41_name_cache_create - from reactos/name_cache.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_name_cache_create(cache_out: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// nfs41_root_create - from reactos/nfs41.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_root_create(name: *mut core::ffi::c_void, sec_flavor: usize, wsize: usize, rsize: usize, root_out: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// nfs41_session_create - from reactos/nfs41.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_session_create(client: *mut core::ffi::c_void, session_out: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// nfs41_server_find_or_create - from reactos/nfs41.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_server_find_or_create(server_owner_major_id: *mut core::ffi::c_void, server_scope: *mut core::ffi::c_void, addr: *mut core::ffi::c_void, server_out: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// nfs41_client_create - from reactos/nfs41.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_client_create(rpc: *mut core::ffi::c_void, owner: *mut core::ffi::c_void, is_data: usize, exchangeid: *mut core::ffi::c_void, client_out: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// nfs41_superblock_for_fh - from reactos/nfs41.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_for_fh(session: *mut core::ffi::c_void, fsid: *mut core::ffi::c_void, OPTIONAL: *mut core::ffi::c_void, file: *mut core::ffi::c_void) -> i32 {
    0
}

/// nfs41_superblock_getattr_mask - from reactos/nfs41.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_getattr_mask(superblock: *mut core::ffi::c_void, attrs: *mut core::ffi::c_void) {

}

/// nfs41_superblock_supported_attrs - from reactos/nfs41.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_supported_attrs(superblock: *mut core::ffi::c_void, attrs: *mut core::ffi::c_void) {

}

/// nfs41_superblock_supported_attrs_exclcreat - from reactos/nfs41.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_supported_attrs_exclcreat(superblock: *mut core::ffi::c_void, attrs: *mut core::ffi::c_void) {

}

/// nfs41_superblock_fs_attributes - from reactos/nfs41.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_fs_attributes(superblock: *mut core::ffi::c_void, FsAttrs: *mut core::ffi::c_void) {

}

/// nfs41_superblock_space_changed - from reactos/nfs41.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_space_changed(superblock: *mut core::ffi::c_void) {

}

/// nfs41_superblock_list_init - from reactos/nfs41.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_list_init(superblocks: *mut core::ffi::c_void) {

}

/// nfs41_rpc_clnt_create - from reactos/nfs41.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_rpc_clnt_create(addrs: *mut core::ffi::c_void, wsize: usize, rsize: usize, uid: usize, gid: usize, sec_flavor: usize, rpc_out: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// nfs41_create_session - from reactos/nfs41_ops.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_create_session(clnt: *mut core::ffi::c_void, session: *mut core::ffi::c_void, try_recovery: usize) -> i32 {
    0
}

/// nfs41_create - from reactos/nfs41_ops.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_create(session: *mut core::ffi::c_void, arg1: usize, createattrs: *mut core::ffi::c_void, symlink: *mut core::ffi::c_void, parent: *mut core::ffi::c_void, file: *mut core::ffi::c_void, info: *mut core::ffi::c_void) -> i32 {
    0
}

/// nfs41_lock - from reactos/nfs41_ops.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_lock(session: *mut core::ffi::c_void, file: *mut core::ffi::c_void, owner: *mut core::ffi::c_void, arg3: usize, offset: usize, length: usize, reclaim: usize, try_recovery: usize, stateid: *mut core::ffi::c_void) -> i32 {
    0
}

/// nfs41_unlock - from reactos/nfs41_ops.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_unlock(session: *mut core::ffi::c_void, file: *mut core::ffi::c_void, offset: usize, length: usize, stateid: *mut core::ffi::c_void) -> i32 {
    0
}

/// nfs41_lock_stateid_copy - from reactos/nfs41_ops.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_lock_stateid_copy(lock_state: *mut core::ffi::c_void, dest: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// nfs41_superblock_getattr - from reactos/nfs41_ops.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_superblock_getattr(session: *mut core::ffi::c_void, file: *mut core::ffi::c_void, attr_request: *mut core::ffi::c_void, info: *mut core::ffi::c_void, supports_named_attrs: *mut core::ffi::c_void) -> i32 {
    0
}

/// pnfs_layout_list_create - from reactos/pnfs.h
#[no_mangle]
pub unsafe extern "C" fn pnfs_layout_list_create(layouts_out: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// pnfs_file_device_list_create - from reactos/pnfs.h
#[no_mangle]
pub unsafe extern "C" fn pnfs_file_device_list_create(devices_out: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// create_silly_rename - from reactos/util.h
#[no_mangle]
pub unsafe extern "C" fn create_silly_rename(path: *mut core::ffi::c_void, fh: *mut core::ffi::c_void, silly: *mut core::ffi::c_void) -> i32 {
    0
}

/// ScCreateWellKnownSids - from reactos/svchost.h
#[no_mangle]
pub unsafe extern "C" fn ScCreateWellKnownSids(arg0: usize) -> i32 {
    0
}

/// CreateSocket - from reactos/telnetd.h
#[no_mangle]
pub unsafe extern "C" fn CreateSocket() {

}

/// ApiLock - from reactos/rosdhcp.h
#[no_mangle]
pub unsafe extern "C" fn ApiLock(arg0: usize) -> usize {
    0
}

/// ApiUnlock - from reactos/rosdhcp.h
#[no_mangle]
pub unsafe extern "C" fn ApiUnlock(arg0: usize) -> usize {
    0
}

/// CreateComputerTypeList - from reactos/settings.h
#[no_mangle]
pub unsafe extern "C" fn CreateComputerTypeList(InfFile: usize) -> usize {
    0
}

/// CreateDisplayDriverList - from reactos/settings.h
#[no_mangle]
pub unsafe extern "C" fn CreateDisplayDriverList(InfFile: usize) -> usize {
    0
}

/// CreateKeyboardDriverList - from reactos/settings.h
#[no_mangle]
pub unsafe extern "C" fn CreateKeyboardDriverList(InfFile: usize) -> usize {
    0
}

/// CreateKeyboardLayoutList - from reactos/settings.h
#[no_mangle]
pub unsafe extern "C" fn CreateKeyboardLayoutList(InfFile: usize, LanguageId: usize, DefaultKBLayout: usize) -> usize {
    0
}

/// CreateLanguageList - from reactos/settings.h
#[no_mangle]
pub unsafe extern "C" fn CreateLanguageList(InfFile: usize, DefaultLanguage: usize) -> usize {
    0
}

/// FindVolCreateInTreeByVolume - from reactos/reactos.h
#[no_mangle]
pub unsafe extern "C" fn FindVolCreateInTreeByVolume(hTreeList: usize, Volume: usize) -> usize {
    0
}

/// CreateListViewColumns - from reactos/reactos.h
#[no_mangle]
pub unsafe extern "C" fn CreateListViewColumns(hInstance: usize, hWndListView: usize, pIDs: *mut core::ffi::c_void, pColsWidth: *mut core::ffi::c_void, pColsAlign: *mut core::ffi::c_void, nNumOfColumns: usize) -> i32 {
    0
}

/// CreateFileSystemList - from reactos/fslist.h
#[no_mangle]
pub unsafe extern "C" fn CreateFileSystemList(Left: usize, Top: usize, ForceFormat: usize, SelectFileSystem: usize) -> usize {
    0
}

/// CreateProgressBarEx - from reactos/progress.h
#[no_mangle]
pub unsafe extern "C" fn CreateProgressBarEx(Left: usize, Top: usize, Right: usize, Bottom: usize, TextTop: usize, TextRight: usize, DoubleEdge: usize, ProgressColour: usize, StepCount: usize, OPTIONAL: usize, OPTIONAL_10: usize, OPTIONAL_11: usize) -> usize {
    0
}

/// CreateProgressBar - from reactos/progress.h
#[no_mangle]
pub unsafe extern "C" fn CreateProgressBar(Left: usize, Top: usize, Right: usize, Bottom: usize, TextTop: usize, TextRight: usize, DoubleEdge: usize, OPTIONAL: usize) -> usize {
    0
}

/// PanelOnCreate - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn PanelOnCreate(hWnd: *mut core::ffi::c_void, wParam: usize, lParam: isize) -> i32 {
    0
}

/// CreateLargeCell - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn CreateLargeCell(infoPtr: usize) -> i32 {
    0
}

/// MapOnCreate - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn MapOnCreate(infoPtr: usize, hwnd: *mut core::ffi::c_void, hParent: *mut core::ffi::c_void) -> i32 {
    0
}

/// create_target_directory - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn create_target_directory(Target: *mut u16) {

}

/// Display_OnCreate - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn Display_OnCreate(hwnd: *mut core::ffi::c_void) -> isize {
    0
}

/// MainWnd_OnCreate - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn MainWnd_OnCreate(hwnd: *mut core::ffi::c_void) -> isize {
    0
}

/// CreateTrayIcon - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn CreateTrayIcon(szKLID: usize, OPTIONAL: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// KbSwitch_OnCreate - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn KbSwitch_OnCreate(hwnd: *mut core::ffi::c_void) -> i32 {
    0
}

/// CreateNewConsoleTitle - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn CreateNewConsoleTitle(arg0: usize) -> usize {
    0
}

/// CreateNewMDIChild - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn CreateNewMDIChild(Info: usize, hwndMDIClient: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// FrameOnCreate - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn FrameOnCreate(hwnd: *mut core::ffi::c_void, lParam: isize) -> isize {
    0
}

/// CreatePropSheet - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn CreatePropSheet(hInstance: *mut core::ffi::c_void, hwndOwner: *mut core::ffi::c_void, lpszTitle: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// OnMainCreate - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn OnMainCreate(hwnd: *mut core::ffi::c_void, pRdpSettings: usize) -> i32 {
    0
}

/// rdssl_rc4_info_create - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn rdssl_rc4_info_create() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// rdssl_sha1_info_create - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn rdssl_sha1_info_create() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// rdssl_md5_info_create - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn rdssl_md5_info_create() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// rdssl_hash_info_create - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn rdssl_hash_info_create(id: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mi_create_window - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn mi_create_window() -> i32 {
    0
}

/// mi_create_cursor - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn mi_create_cursor(x: u32, y: u32, width: i32, height: i32, andmask: *mut u8, xormask: *mut u8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _dl_tls_block_size_with_pre - from glibc/dl-tls_block_align.h
#[no_mangle]
pub unsafe extern "C" fn _dl_tls_block_size_with_pre() -> usize {
    0
}

/// _dl_tls_block_align - from glibc/dl-tls_block_align.h
#[no_mangle]
pub unsafe extern "C" fn _dl_tls_block_align(size: usize, allocated: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// __lll_abstimed_lock - from glibc/hurdlock.h
#[no_mangle]
pub unsafe extern "C" fn __lll_abstimed_lock(__ptr: *mut core::ffi::c_void, __tsp: *mut core::ffi::c_void, __flags: i32, __clk: i32) -> i32 {
    0
}

/// __lll_robust_lock - from glibc/hurdlock.h
#[no_mangle]
pub unsafe extern "C" fn __lll_robust_lock(__ptr: *mut core::ffi::c_void, __flags: i32) -> i32 {
    0
}

/// __lll_robust_abstimed_lock - from glibc/hurdlock.h
#[no_mangle]
pub unsafe extern "C" fn __lll_robust_abstimed_lock(__ptr: *mut core::ffi::c_void, __tsp: *mut core::ffi::c_void, __flags: i32, __clk: i32) -> i32 {
    0
}

/// __lll_robust_trylock - from glibc/hurdlock.h
#[no_mangle]
pub unsafe extern "C" fn __lll_robust_trylock(__ptr: *mut core::ffi::c_void) -> i32 {
    0
}

/// __lll_robust_unlock - from glibc/hurdlock.h
#[no_mangle]
pub unsafe extern "C" fn __lll_robust_unlock(__ptr: *mut core::ffi::c_void, __flags: i32) {

}

/// __gconv_create_spec - from glibc/gconv_int.h
#[no_mangle]
pub unsafe extern "C" fn __gconv_create_spec(conv_spec: *mut core::ffi::c_void, fromcode: *mut i8, tocode: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// __argz_create_sep - from glibc/argz.h
#[no_mangle]
pub unsafe extern "C" fn __argz_create_sep(__string: *mut core::ffi::c_void, __sep: i32, __argz: *mut *mut core::ffi::c_void, __len: *mut core::ffi::c_void) -> usize {
    0
}

/// htab_create - from glibc/inline-hashtab.h
#[no_mangle]
pub unsafe extern "C" fn htab_create() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// __pthread_barrier_init - from glibc/pthread.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_barrier_init(__barrier: *mut core::ffi::c_void, __attr: *mut core::ffi::c_void, __count: u32) -> i32 {
    0
}

/// __pthread_barrier_wait - from glibc/pthread.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_barrier_wait(__barrier: *mut core::ffi::c_void) -> i32 {
    0
}

/// __pthread_initialize - from glibc/pthread.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_initialize() {

}

/// __pthread_self - from glibc/pthread.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_self() -> usize {
    0
}

/// __sigblock - from glibc/signal.h
#[no_mangle]
pub unsafe extern "C" fn __sigblock(__mask: i32) -> i32 {
    0
}

/// __clock_settime64 - from glibc/time.h
#[no_mangle]
pub unsafe extern "C" fn __clock_settime64(clock_id: usize, tp: *mut core::ffi::c_void) -> i32 {
    0
}

/// __clock_getres64 - from glibc/time.h
#[no_mangle]
pub unsafe extern "C" fn __clock_getres64(clock_id: usize, tp: *mut core::ffi::c_void) -> i32 {
    0
}

/// __clock_nanosleep_time64 - from glibc/time.h
#[no_mangle]
pub unsafe extern "C" fn __clock_nanosleep_time64(clock_id: usize, flags: i32, req: *mut core::ffi::c_void, rem: *mut core::ffi::c_void) -> i32 {
    0
}

/// __clock_gettime64 - from glibc/time.h
#[no_mangle]
pub unsafe extern "C" fn __clock_gettime64(clock_id: usize, tp: *mut core::ffi::c_void) -> i32 {
    0
}

/// clock_from_timebase - from glibc/time.h
#[no_mangle]
pub unsafe extern "C" fn clock_from_timebase(timebase: i32) -> usize {
    0
}

/// _IO_peekc_locked - from glibc/libio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_peekc_locked(__fp: *mut core::ffi::c_void) -> i32 {
    0
}

/// _IO_flockfile - from glibc/libio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_flockfile(arg0: *mut core::ffi::c_void) {

}

/// _IO_funlockfile - from glibc/libio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_funlockfile(arg0: *mut core::ffi::c_void) {

}

/// _IO_ftrylockfile - from glibc/libio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_ftrylockfile(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// _IO_list_lock - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_list_lock() {

}

/// _IO_list_unlock - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_list_unlock() {

}

/// _IO_list_resetlock - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_list_resetlock() {

}

/// _IO_enable_locks - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_enable_locks() {

}

/// _IO_proc_file_chain_lock - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_proc_file_chain_lock() {

}

/// _IO_proc_file_chain_unlock - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_proc_file_chain_unlock() {

}

/// _IO_proc_file_chain_resetlock - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_proc_file_chain_resetlock() {

}

/// _IO_seekoff_unlocked - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_seekoff_unlocked(arg0: *mut core::ffi::c_void, arg1: usize, arg2: i32, arg3: i32) -> usize {
    0
}

/// _IO_seekpos_unlocked - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_seekpos_unlocked(arg0: *mut core::ffi::c_void, arg1: usize, arg2: i32) -> usize {
    0
}

/// __spin_lock_init - from glibc/lock-intern.h
#[no_mangle]
pub unsafe extern "C" fn __spin_lock_init(__lock: *mut core::ffi::c_void) {

}

/// __spin_lock_solid - from glibc/lock-intern.h
#[no_mangle]
pub unsafe extern "C" fn __spin_lock_solid(__lock: *mut core::ffi::c_void) {

}

/// __spin_lock - from glibc/lock-intern.h
#[no_mangle]
pub unsafe extern "C" fn __spin_lock(__lock: *mut core::ffi::c_void) {

}

/// __spin_unlock - from glibc/lock-intern.h
#[no_mangle]
pub unsafe extern "C" fn __spin_unlock(__lock: *mut core::ffi::c_void) {

}

/// __spin_try_lock - from glibc/lock-intern.h
#[no_mangle]
pub unsafe extern "C" fn __spin_try_lock(__lock: *mut core::ffi::c_void) -> i32 {
    0
}

/// __spin_lock_locked - from glibc/lock-intern.h
#[no_mangle]
pub unsafe extern "C" fn __spin_lock_locked(__lock: *mut core::ffi::c_void) -> i32 {
    0
}

/// __mutex_init - from glibc/lock-intern.h
#[no_mangle]
pub unsafe extern "C" fn __mutex_init(__lock: *mut core::ffi::c_void) {

}

/// __mutex_lock - from glibc/lock-intern.h
#[no_mangle]
pub unsafe extern "C" fn __mutex_lock(__lock: *mut core::ffi::c_void) {

}

/// __mutex_unlock - from glibc/lock-intern.h
#[no_mangle]
pub unsafe extern "C" fn __mutex_unlock(__lock: *mut core::ffi::c_void) {

}

/// __mutex_trylock - from glibc/lock-intern.h
#[no_mangle]
pub unsafe extern "C" fn __mutex_trylock(__lock: *mut core::ffi::c_void) -> i32 {
    0
}

/// get_block_size - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn get_block_size(rand_data: u32) -> u32 {
    0
}

/// get_random_block_size - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn get_random_block_size(state: *mut u32) -> u32 {
    0
}

/// test_mutex - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn test_mutex(iters: i64, filler: i32) -> usize {
    0
}

/// test_mutex_trylock - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn test_mutex_trylock(iters: i64, filler: i32) -> usize {
    0
}

/// test_spin_lock - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn test_spin_lock(iters: i64, filler: i32) -> usize {
    0
}

/// test_spin_trylock - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn test_spin_trylock(iters: i64, filler: i32) -> usize {
    0
}

/// bench_random_lock - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn bench_random_lock(json_ctx: *mut core::ffi::c_void, iters: usize) {

}

/// AnnotateRWLockCreate - from cpython/dynamic_annotations.h
#[no_mangle]
pub unsafe extern "C" fn AnnotateRWLockCreate(file: *mut i8, line: i32, lock: *mut core::ffi::c_void) {

}

/// AnnotateRWLockDestroy - from cpython/dynamic_annotations.h
#[no_mangle]
pub unsafe extern "C" fn AnnotateRWLockDestroy(file: *mut i8, line: i32, lock: *mut core::ffi::c_void) {

}

/// AnnotateRWLockAcquired - from cpython/dynamic_annotations.h
#[no_mangle]
pub unsafe extern "C" fn AnnotateRWLockAcquired(file: *mut i8, line: i32, lock: *mut core::ffi::c_void, is_w: i64) {

}

/// AnnotateRWLockReleased - from cpython/dynamic_annotations.h
#[no_mangle]
pub unsafe extern "C" fn AnnotateRWLockReleased(file: *mut i8, line: i32, lock: *mut core::ffi::c_void, is_w: i64) {

}

/// AnnotatePCQCreate - from cpython/dynamic_annotations.h
#[no_mangle]
pub unsafe extern "C" fn AnnotatePCQCreate(file: *mut i8, line: i32, pcq: *mut core::ffi::c_void) {

}

/// AnnotateMutexIsUsedAsCondVar - from cpython/dynamic_annotations.h
#[no_mangle]
pub unsafe extern "C" fn AnnotateMutexIsUsedAsCondVar(file: *mut i8, line: i32, mu: *mut core::ffi::c_void) {

}

/// _xidregistry_lock - from cpython/crossinterp_data_lookup.h
#[no_mangle]
pub unsafe extern "C" fn _xidregistry_lock(registry: *mut core::ffi::c_void) {

}

/// _xidregistry_unlock - from cpython/crossinterp_data_lookup.h
#[no_mangle]
pub unsafe extern "C" fn _xidregistry_unlock(registry: *mut core::ffi::c_void) {

}

/// EnterNonRecursiveMutex - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn EnterNonRecursiveMutex(mutex: usize, milliseconds: u32) -> u32 {
    0
}

/// LeaveNonRecursiveMutex - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn LeaveNonRecursiveMutex(mutex: usize) -> i32 {
    0
}

/// CreateSemaphore - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn CreateSemaphore(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// _PyMutex_Lock - from cpython/pylock.h
#[no_mangle]
pub unsafe extern "C" fn _PyMutex_Lock(m: *mut core::ffi::c_void) {

}

/// _PyMutex_Unlock - from cpython/pylock.h
#[no_mangle]
pub unsafe extern "C" fn _PyMutex_Unlock(m: *mut core::ffi::c_void) {

}

/// _PyMutex_IsLocked - from cpython/pylock.h
#[no_mangle]
pub unsafe extern "C" fn _PyMutex_IsLocked(m: *mut core::ffi::c_void) -> i32 {
    0
}

/// _BlocksOutputBuffer_InitWithSize - from cpython/pycore_blocks_output_buffer.h
#[no_mangle]
pub unsafe extern "C" fn _BlocksOutputBuffer_InitWithSize(buffer: *mut core::ffi::c_void, init_size: usize, next_out: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// _BlocksOutputBuffer_Grow - from cpython/pycore_blocks_output_buffer.h
#[no_mangle]
pub unsafe extern "C" fn _BlocksOutputBuffer_Grow(buffer: *mut core::ffi::c_void, next_out: *mut *mut core::ffi::c_void, avail_out: usize) -> usize {
    0
}

/// _BlocksOutputBuffer_GetDataSize - from cpython/pycore_blocks_output_buffer.h
#[no_mangle]
pub unsafe extern "C" fn _BlocksOutputBuffer_GetDataSize(buffer: *mut core::ffi::c_void, avail_out: usize) -> usize {
    0
}

/// _BlocksOutputBuffer_Finish - from cpython/pycore_blocks_output_buffer.h
#[no_mangle]
pub unsafe extern "C" fn _BlocksOutputBuffer_Finish(buffer: *mut core::ffi::c_void, avail_out: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _BlocksOutputBuffer_OnError - from cpython/pycore_blocks_output_buffer.h
#[no_mangle]
pub unsafe extern "C" fn _BlocksOutputBuffer_OnError(buffer: *mut core::ffi::c_void) {

}

/// _PyEval_AcquireLock - from cpython/pycore_ceval.h
#[no_mangle]
pub unsafe extern "C" fn _PyEval_AcquireLock(tstate: *mut core::ffi::c_void) {

}

/// _PyEval_ReleaseLock - from cpython/pycore_ceval.h
#[no_mangle]
pub unsafe extern "C" fn _PyEval_ReleaseLock(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, final_release: i32) {

}

/// _PyCompile_PushFBlock - from cpython/pycore_compile.h
#[no_mangle]
pub unsafe extern "C" fn _PyCompile_PushFBlock(c: *mut core::ffi::c_void, loc: usize, t: usize, block_label: usize, exit: usize, datum: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyCompile_PopFBlock - from cpython/pycore_compile.h
#[no_mangle]
pub unsafe extern "C" fn _PyCompile_PopFBlock(c: *mut core::ffi::c_void, t: usize, block_label: usize) {

}

/// _PyCompile_EnterConditionalBlock - from cpython/pycore_compile.h
#[no_mangle]
pub unsafe extern "C" fn _PyCompile_EnterConditionalBlock(c: *mut core::ffi::c_void) {

}

/// _PyCompile_LeaveConditionalBlock - from cpython/pycore_compile.h
#[no_mangle]
pub unsafe extern "C" fn _PyCompile_LeaveConditionalBlock(c: *mut core::ffi::c_void) {

}

/// _PyCriticalSection_BeginMutex - from cpython/pycore_critical_section.h
#[no_mangle]
pub unsafe extern "C" fn _PyCriticalSection_BeginMutex(tstate: *mut core::ffi::c_void, c: *mut core::ffi::c_void, m: *mut core::ffi::c_void) {

}

/// _PyCriticalSection2_BeginMutex - from cpython/pycore_critical_section.h
#[no_mangle]
pub unsafe extern "C" fn _PyCriticalSection2_BeginMutex(tstate: *mut core::ffi::c_void, c: *mut core::ffi::c_void, m1: *mut core::ffi::c_void, m2: *mut core::ffi::c_void) {

}

/// _PyDict_ClearKeysVersionLockHeld - from cpython/pycore_dict.h
#[no_mangle]
pub unsafe extern "C" fn _PyDict_ClearKeysVersionLockHeld(mp: *mut core::ffi::c_void) {

}

/// _PyDict_SizeOf_LockHeld - from cpython/pycore_dict.h
#[no_mangle]
pub unsafe extern "C" fn _PyDict_SizeOf_LockHeld(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyDict_GetItemRef_Unicode_LockHeld - from cpython/pycore_dict.h
#[no_mangle]
pub unsafe extern "C" fn _PyDict_GetItemRef_Unicode_LockHeld(op: *mut core::ffi::c_void, key: *mut core::ffi::c_void, result: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyDict_Clear_LockHeld - from cpython/pycore_dict.h
#[no_mangle]
pub unsafe extern "C" fn _PyDict_Clear_LockHeld(op: *mut core::ffi::c_void) {

}

/// _Py_get_blocking - from cpython/pycore_fileutils.h
#[no_mangle]
pub unsafe extern "C" fn _Py_get_blocking(fd: i32) -> i32 {
    0
}

/// _Py_set_blocking - from cpython/pycore_fileutils.h
#[no_mangle]
pub unsafe extern "C" fn _Py_set_blocking(fd: i32, blocking: i32) -> i32 {
    0
}

/// _PyImport_AcquireLock - from cpython/pycore_import.h
#[no_mangle]
pub unsafe extern "C" fn _PyImport_AcquireLock(interp: *mut core::ffi::c_void) {

}

/// _PyImport_ReleaseLock - from cpython/pycore_import.h
#[no_mangle]
pub unsafe extern "C" fn _PyImport_ReleaseLock(interp: *mut core::ffi::c_void) {

}

/// _PyImport_ReInitLock - from cpython/pycore_import.h
#[no_mangle]
pub unsafe extern "C" fn _PyImport_ReInitLock(interp: *mut core::ffi::c_void) {

}

/// _PyConfig_CreateXOptionsDict - from cpython/pycore_initconfig.h
#[no_mangle]
pub unsafe extern "C" fn _PyConfig_CreateXOptionsDict(config: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyMutex_LockFast - from cpython/pycore_lock.h
#[no_mangle]
pub unsafe extern "C" fn PyMutex_LockFast(m: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyMutex_at_fork_reinit - from cpython/pycore_lock.h
#[no_mangle]
pub unsafe extern "C" fn _PyMutex_at_fork_reinit(m: *mut core::ffi::c_void) {

}

/// PyMutex_LockFlags - from cpython/pycore_lock.h
#[no_mangle]
pub unsafe extern "C" fn PyMutex_LockFlags(m: *mut core::ffi::c_void, flags: usize) {

}

/// _PyMutex_TryUnlock - from cpython/pycore_lock.h
#[no_mangle]
pub unsafe extern "C" fn _PyMutex_TryUnlock(m: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyRawMutex_LockSlow - from cpython/pycore_lock.h
#[no_mangle]
pub unsafe extern "C" fn _PyRawMutex_LockSlow(m: *mut core::ffi::c_void) {

}

/// _PyRawMutex_UnlockSlow - from cpython/pycore_lock.h
#[no_mangle]
pub unsafe extern "C" fn _PyRawMutex_UnlockSlow(m: *mut core::ffi::c_void) {

}

/// _PyRawMutex_Lock - from cpython/pycore_lock.h
#[no_mangle]
pub unsafe extern "C" fn _PyRawMutex_Lock(m: *mut core::ffi::c_void) {

}

/// _PyRawMutex_Unlock - from cpython/pycore_lock.h
#[no_mangle]
pub unsafe extern "C" fn _PyRawMutex_Unlock(m: *mut core::ffi::c_void) {

}

/// _PyRecursiveMutex_LockTimed - from cpython/pycore_lock.h
#[no_mangle]
pub unsafe extern "C" fn _PyRecursiveMutex_LockTimed(m: *mut core::ffi::c_void, timeout: usize, flags: usize) -> usize {
    0
}

/// _PyRecursiveMutex_TryUnlock - from cpython/pycore_lock.h
#[no_mangle]
pub unsafe extern "C" fn _PyRecursiveMutex_TryUnlock(m: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyModule_CreateInitialized - from cpython/pycore_modsupport.h
#[no_mangle]
pub unsafe extern "C" fn _PyModule_CreateInitialized(arg0: *mut core::ffi::c_void, apiver: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _Py_NewRefWithLock - from cpython/pycore_object.h
#[no_mangle]
pub unsafe extern "C" fn _Py_NewRefWithLock(op: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _Py_XNewRefWithLock - from cpython/pycore_object.h
#[no_mangle]
pub unsafe extern "C" fn _Py_XNewRefWithLock(obj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _PyUnicodeTranslateError_Create - from cpython/pycore_pyerrors.h
#[no_mangle]
pub unsafe extern "C" fn _PyUnicodeTranslateError_Create(object: *mut core::ffi::c_void, start: usize, end: usize, reason: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _PySys_Create - from cpython/pycore_pylifecycle.h
#[no_mangle]
pub unsafe extern "C" fn _PySys_Create(tstate: *mut core::ffi::c_void, sysmod_p: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// _Py_stackref_create - from cpython/pycore_stackref.h
#[no_mangle]
pub unsafe extern "C" fn _Py_stackref_create(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// mi_heap_contains_block - from cpython/mimalloc.h
#[no_mangle]
pub unsafe extern "C" fn mi_heap_contains_block(heap: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> usize {
    0
}

/// mi_heap_visit_blocks - from cpython/mimalloc.h
#[no_mangle]
pub unsafe extern "C" fn mi_heap_visit_blocks(heap: *mut core::ffi::c_void, visit_all_blocks: usize, visitor: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> usize {
    0
}

/// _mi_abandoned_pool_visit_blocks - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_abandoned_pool_visit_blocks(pool: *mut core::ffi::c_void, page_tag: u8, visit_blocks: usize, visitor: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> usize {
    0
}

/// _mi_heap_area_visit_blocks - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_heap_area_visit_blocks(area: *mut core::ffi::c_void, page: *mut core::ffi::c_void, visitor: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> usize {
    0
}

/// _mi_clock_now - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_clock_now() -> usize {
    0
}

/// _mi_clock_end - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_clock_end(start: usize) -> usize {
    0
}

/// _mi_clock_start - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_clock_start() -> usize {
    0
}

/// mi_page_block_size - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn mi_page_block_size(page: *mut core::ffi::c_void) -> usize {
    0
}

/// mi_page_usable_block_size - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn mi_page_usable_block_size(page: *mut core::ffi::c_void) -> usize {
    0
}

/// mi_tf_block - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn mi_tf_block(tf: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mi_tf_set_block - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn mi_tf_set_block(tf: usize, block: *mut core::ffi::c_void) -> usize {
    0
}

/// mi_block_nextx - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn mi_block_nextx(null: *mut core::ffi::c_void, block: *mut core::ffi::c_void, keys: *mut usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mi_block_set_nextx - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn mi_block_set_nextx(null: *mut core::ffi::c_void, block: *mut core::ffi::c_void, next: *mut core::ffi::c_void, keys: *mut usize) {

}

/// mi_block_next - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn mi_block_next(page: *mut core::ffi::c_void, block: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mi_block_set_next - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn mi_block_set_next(page: *mut core::ffi::c_void, block: *mut core::ffi::c_void, next: *mut core::ffi::c_void) {

}

/// mi_commit_mask_create_empty - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn mi_commit_mask_create_empty(cm: *mut core::ffi::c_void) {

}

/// mi_commit_mask_create_full - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn mi_commit_mask_create_full(cm: *mut core::ffi::c_void) {

}

/// _mi_memid_create - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_memid_create(memkind: usize) -> usize {
    0
}

/// _mi_memid_create_os - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_memid_create_os(committed: usize, is_zero: usize, is_large: usize) -> usize {
    0
}

/// _mi_prim_clock_now - from cpython/prim.h
#[no_mangle]
pub unsafe extern "C" fn _mi_prim_clock_now() -> usize {
    0
}

/// flock - from cpython/fcntlmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn flock(arg0: usize) -> usize {
    0
}

/// fcntl_flock_impl - from cpython/fcntlmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn fcntl_flock_impl(module: *mut core::ffi::c_void, fd: i32, code: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// fcntl_flock - from cpython/fcntlmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn fcntl_flock(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// fcntl_lockf_impl - from cpython/fcntlmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn fcntl_lockf_impl(module: *mut core::ffi::c_void, fd: i32, code: i32, lenobj: *mut core::ffi::c_void, startobj: *mut core::ffi::c_void, whence: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// fcntl_lockf - from cpython/fcntlmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn fcntl_lockf(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _hmac_HMAC_block_size_get_impl - from cpython/hmacmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _hmac_HMAC_block_size_get_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _hmac_HMAC_block_size_get - from cpython/hmacmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _hmac_HMAC_block_size_get(arg0: *mut core::ffi::c_void, Py_UNUSEDcontext: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _overlapped_CreateIoCompletionPort_impl - from cpython/overlapped.c.h
#[no_mangle]
pub unsafe extern "C" fn _overlapped_CreateIoCompletionPort_impl(module: *mut core::ffi::c_void, FileHandle: *mut core::ffi::c_void, ExistingCompletionPort: *mut core::ffi::c_void, CompletionKey: usize, NumberOfConcurrentThreads: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _overlapped_CreateIoCompletionPort - from cpython/overlapped.c.h
#[no_mangle]
pub unsafe extern "C" fn _overlapped_CreateIoCompletionPort(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _overlapped_CreateEvent_impl - from cpython/overlapped.c.h
#[no_mangle]
pub unsafe extern "C" fn _overlapped_CreateEvent_impl(module: *mut core::ffi::c_void, EventAttributes: *mut core::ffi::c_void, ManualReset: i32, InitialState: i32, Name: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _overlapped_CreateEvent - from cpython/overlapped.c.h
#[no_mangle]
pub unsafe extern "C" fn _overlapped_CreateEvent(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// unlockpt - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn unlockpt() -> usize {
    0
}

/// os_unlockpt_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_unlockpt_impl(module: *mut core::ffi::c_void, fd: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_unlockpt - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_unlockpt(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_plock_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_plock_impl(module: *mut core::ffi::c_void, op: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_plock - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_plock(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_timerfd_create_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_timerfd_create_impl(module: *mut core::ffi::c_void, clockid: i32, flags: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_timerfd_create - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_timerfd_create(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_lockf_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_lockf_impl(module: *mut core::ffi::c_void, fd: i32, command: i32, length: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_lockf - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_lockf(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_memfd_create_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_memfd_create_impl(module: *mut core::ffi::c_void, name: *mut core::ffi::c_void, flags: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_memfd_create - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_memfd_create(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_get_blocking_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_get_blocking_impl(module: *mut core::ffi::c_void, fd: i32) -> i32 {
    0
}

/// os_get_blocking - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_get_blocking(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_set_blocking_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_set_blocking_impl(module: *mut core::ffi::c_void, fd: i32, blocking: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_set_blocking - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_set_blocking(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os__create_environ_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os__create_environ_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os__create_environ - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os__create_environ(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// time_clock_gettime_impl - from cpython/timemodule.c.h
#[no_mangle]
pub unsafe extern "C" fn time_clock_gettime_impl(module: *mut core::ffi::c_void, clk_id: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// time_clock_gettime - from cpython/timemodule.c.h
#[no_mangle]
pub unsafe extern "C" fn time_clock_gettime(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// time_clock_gettime_ns_impl - from cpython/timemodule.c.h
#[no_mangle]
pub unsafe extern "C" fn time_clock_gettime_ns_impl(module: *mut core::ffi::c_void, clk_id: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// time_clock_gettime_ns - from cpython/timemodule.c.h
#[no_mangle]
pub unsafe extern "C" fn time_clock_gettime_ns(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// unicodedata_block_impl - from cpython/unicodedata.c.h
#[no_mangle]
pub unsafe extern "C" fn unicodedata_block_impl(module: *mut core::ffi::c_void, chr: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// unicodedata_block - from cpython/unicodedata.c.h
#[no_mangle]
pub unsafe extern "C" fn unicodedata_block(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _asyncio_Future__asyncio_future_blocking_get_impl - from cpython/_asynciomodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__asyncio_future_blocking_get_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _asyncio_Future__asyncio_future_blocking_get - from cpython/_asynciomodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__asyncio_future_blocking_get(arg0: *mut core::ffi::c_void, Py_UNUSEDcontext: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _asyncio_Future__asyncio_future_blocking_set_impl - from cpython/_asynciomodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__asyncio_future_blocking_set_impl(arg0: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> i32 {
    0
}

/// _asyncio_Future__asyncio_future_blocking_set - from cpython/_asynciomodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__asyncio_future_blocking_set(arg0: *mut core::ffi::c_void, value: *mut core::ffi::c_void, Py_UNUSEDcontext: *mut core::ffi::c_void) -> i32 {
    0
}

/// _interpqueues_create_impl - from cpython/_interpqueuesmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _interpqueues_create_impl(module: *mut core::ffi::c_void, maxsize: usize, unboundarg: i32, fallbackarg: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _interpqueues_create - from cpython/_interpqueuesmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _interpqueues_create(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _interpreters_create_impl - from cpython/_interpretersmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _interpreters_create_impl(module: *mut core::ffi::c_void, configobj: *mut core::ffi::c_void, reqrefs: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _interpreters_create - from cpython/_interpretersmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _interpreters_create(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// locked - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn locked() -> usize {
    0
}

/// lock_new_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn lock_new_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// lock_new - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn lock_new(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void, kwargs: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// rlock_new_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn rlock_new_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// rlock_new - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn rlock_new(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void, kwargs: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_tkapp_createcommand_impl - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_createcommand_impl(arg0: *mut core::ffi::c_void, name: *mut i8, func: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_tkapp_createcommand - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_createcommand(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_tkapp_createfilehandler_impl - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_createfilehandler_impl(arg0: *mut core::ffi::c_void, file: *mut core::ffi::c_void, mask: i32, func: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_tkapp_createfilehandler - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_createfilehandler(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_tkapp_createtimerhandler_impl - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_createtimerhandler_impl(arg0: *mut core::ffi::c_void, milliseconds: i32, func: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_tkapp_createtimerhandler - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_createtimerhandler(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_create_impl - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_create_impl(module: *mut core::ffi::c_void, screenName: *mut i8, baseName: *mut i8, className: *mut i8, interactive: i32, wantobjects: i32, wantTk: i32, sync: i32, arg8: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_create - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_create(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreateEventW_impl - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateEventW_impl(module: *mut core::ffi::c_void, security_attributes: usize, manual_reset: i32, initial_state: i32, name: *const u16) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreateEventW - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateEventW(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreateFile_impl - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateFile_impl(module: *mut core::ffi::c_void, file_name: *const u16, desired_access: u32, share_mode: u32, security_attributes: usize, creation_disposition: u32, flags_and_attributes: u32, template_file: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreateFile - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateFile(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreateFileMapping_impl - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateFileMapping_impl(module: *mut core::ffi::c_void, file_handle: *mut core::ffi::c_void, security_attributes: usize, protect: u32, max_size_high: u32, max_size_low: u32, name: *const u16) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreateFileMapping - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateFileMapping(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreateJunction_impl - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateJunction_impl(module: *mut core::ffi::c_void, src_path: *const u16, dst_path: *const u16) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreateJunction - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateJunction(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreateMutexW_impl - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateMutexW_impl(module: *mut core::ffi::c_void, security_attributes: usize, initial_owner: i32, name: *const u16) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreateMutexW - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateMutexW(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreateNamedPipe_impl - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateNamedPipe_impl(module: *mut core::ffi::c_void, name: *const u16, open_mode: u32, pipe_mode: u32, max_instances: u32, out_buffer_size: u32, in_buffer_size: u32, default_timeout: u32, security_attributes: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreateNamedPipe - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateNamedPipe(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreatePipe_impl - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreatePipe_impl(module: *mut core::ffi::c_void, pipe_attrs: *mut core::ffi::c_void, size: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreatePipe - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreatePipe(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreateProcess_impl - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateProcess_impl(module: *mut core::ffi::c_void, application_name: *mut core::ffi::c_void, command_line: *mut core::ffi::c_void, proc_attrs: *mut core::ffi::c_void, thread_attrs: *mut core::ffi::c_void, inherit_handles: i32, creation_flags: u32, env_mapping: *mut core::ffi::c_void, current_directory: *mut core::ffi::c_void, startup_info: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_CreateProcess - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_CreateProcess(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_OpenMutexW_impl - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_OpenMutexW_impl(module: *mut core::ffi::c_void, desired_access: u32, inherit_handle: i32, name: *const u16) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_OpenMutexW - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_OpenMutexW(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_ReleaseMutex_impl - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_ReleaseMutex_impl(module: *mut core::ffi::c_void, mutex: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_ReleaseMutex - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_ReleaseMutex(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// stginfo_set_dict_final_lock_held - from cpython/ctypes.h
#[no_mangle]
pub unsafe extern "C" fn stginfo_set_dict_final_lock_held(info: *mut core::ffi::c_void) {

}

/// Hacl_Hash_SHA3_block_len - from cpython/Hacl_Hash_SHA3.h
#[no_mangle]
pub unsafe extern "C" fn Hacl_Hash_SHA3_block_len(s: *mut core::ffi::c_void) -> u32 {
    0
}

/// Hacl_Hash_SHA3_shake128_absorb_nblocks - from cpython/Hacl_Hash_SHA3.h
#[no_mangle]
pub unsafe extern "C" fn Hacl_Hash_SHA3_shake128_absorb_nblocks(state: *mut u64, input: *mut u8, inputByteLen: u32) {

}

/// Hacl_Hash_SHA3_shake128_squeeze_nblocks - from cpython/Hacl_Hash_SHA3.h
#[no_mangle]
pub unsafe extern "C" fn Hacl_Hash_SHA3_shake128_squeeze_nblocks(state: *mut u64, output: *mut u8, outputByteLen: u32) {

}

/// _PyTestInternalCapi_Init_Lock - from cpython/parts.h
#[no_mangle]
pub unsafe extern "C" fn _PyTestInternalCapi_Init_Lock(module: *mut core::ffi::c_void) -> i32 {
    0
}

/// _decimal_Context_create_decimal_from_float_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_create_decimal_from_float_impl(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, f: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_create_decimal_from_float - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_create_decimal_from_float(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_create_decimal_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_create_decimal_impl(context: *mut core::ffi::c_void, num: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_create_decimal - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_create_decimal(context: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// atexit_unregister_locked - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn atexit_unregister_locked(callbacks: *mut core::ffi::c_void, func: *mut core::ffi::c_void) -> i32 {
    0
}

/// blake2_blake2b_copy_unlocked - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn blake2_blake2b_copy_unlocked(arg0: *mut core::ffi::c_void, cpy: *mut core::ffi::c_void) -> i32 {
    0
}

/// py_blake2b_get_block_size - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn py_blake2b_get_block_size(op: *mut core::ffi::c_void, Py_UNUSEDclosure: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyMutex_Lock - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyMutex_Lock(arg0: usize) -> usize {
    0
}

/// hmac_digest_compute_locked - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn hmac_digest_compute_locked(arg0: *mut core::ffi::c_void, digest: *mut u8) -> i32 {
    0
}

/// _grouper_create - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _grouper_create(arg0: usize) -> usize {
    0
}

/// chain_next_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn chain_next_lock_held(op: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// product_next_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn product_next_lock_held(op: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// combinations_next_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn combinations_next_lock_held(op: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cwr_next_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn cwr_next_lock_held(op: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// permutations_next_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn permutations_next_lock_held(op: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// accumulate_next_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn accumulate_next_lock_held(op: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// zip_longest_next_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn zip_longest_next_lock_held(op: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// MD5_get_block_size - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn MD5_get_block_size(Py_UNUSEDself: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mmap_gfind_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn mmap_gfind_lock_held(arg0: *mut core::ffi::c_void, view: *mut core::ffi::c_void, start_obj: *mut core::ffi::c_void, end_obj: *mut core::ffi::c_void, reverse: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mmap__repr__method_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn mmap__repr__method_lock_held(op: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mmap_buffer_getbuf_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn mmap_buffer_getbuf_lock_held(op: *mut core::ffi::c_void, view: *mut core::ffi::c_void, flags: i32) -> i32 {
    0
}

/// mmap_length_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn mmap_length_lock_held(op: *mut core::ffi::c_void) -> usize {
    0
}

/// mmap_item_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn mmap_item_lock_held(op: *mut core::ffi::c_void, i: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mmap_subscript_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn mmap_subscript_lock_held(op: *mut core::ffi::c_void, item: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mmap_ass_item_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn mmap_ass_item_lock_held(op: *mut core::ffi::c_void, i: usize, v: *mut core::ffi::c_void) -> i32 {
    0
}

/// mmap_ass_subscript_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn mmap_ass_subscript_lock_held(op: *mut core::ffi::c_void, item: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> i32 {
    0
}

/// _py_get_history_length_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _py_get_history_length_lock_held() -> i32 {
    0
}

/// kqueue_tracking_add_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn kqueue_tracking_add_lock_held(state: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// kqueue_tracking_remove_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn kqueue_tracking_remove_lock_held(state: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) {

}

/// SHA1_get_block_size - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn SHA1_get_block_size(Py_UNUSEDself: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SHA256_get_block_size - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn SHA256_get_block_size(Py_UNUSEDself: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SHA512_get_block_size - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn SHA512_get_block_size(Py_UNUSEDself: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SHA3_get_block_size - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn SHA3_get_block_size(op: *mut core::ffi::c_void, Py_UNUSEDclosure: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// internal_setblocking - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn internal_setblocking(s: *mut core::ffi::c_void, block: i32) -> i32 {
    0
}

/// sock_setblocking - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn sock_setblocking(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// blocking - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn blocking(arg0: usize) -> usize {
    0
}

/// sock_getblocking - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn sock_getblocking(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// setblocking - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn setblocking(arg0: usize) -> usize {
    0
}

/// py_clock - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn py_clock(state: *mut core::ffi::c_void, tp: *mut core::ffi::c_void, info: *mut core::ffi::c_void) -> i32 {
    0
}

/// time_clockid_converter - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn time_clockid_converter(obj: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> i32 {
    0
}

/// time_clock_settime - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn time_clock_settime(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// time_clock_settime_ns - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn time_clock_settime_ns(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// time_clock_getres - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn time_clock_getres(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// time_get_clock_info - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn time_get_clock_info(module: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// unicodedata_create_capi - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn unicodedata_create_capi() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// create_cancelled_error - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn create_cancelled_error(state: *mut core::ffi::c_void, fut: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// FutureIter_am_send_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn FutureIter_am_send_lock_held(it: *mut core::ffi::c_void, result: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// task_wakeup_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn task_wakeup_lock_held(task: *mut core::ffi::c_void, o: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// newblock - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn newblock(deque: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// deque_append_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn deque_append_lock_held(deque: *mut core::ffi::c_void, item: *mut core::ffi::c_void, maxlen: usize) -> i32 {
    0
}

/// deque_appendleft_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn deque_appendleft_lock_held(deque: *mut core::ffi::c_void, item: *mut core::ffi::c_void, maxlen: usize) -> i32 {
    0
}

/// deque_concat_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn deque_concat_lock_held(deque: *mut core::ffi::c_void, other: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// deque_inplace_repeat_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn deque_inplace_repeat_lock_held(deque: *mut core::ffi::c_void, n: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// deque_contains_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn deque_contains_lock_held(deque: *mut core::ffi::c_void, v: *mut core::ffi::c_void) -> i32 {
    0
}

/// deque_item_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn deque_item_lock_held(deque: *mut core::ffi::c_void, i: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// deque_ass_item_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn deque_ass_item_lock_held(deque: *mut core::ffi::c_void, i: usize, v: *mut core::ffi::c_void) -> i32 {
    0
}

/// dequeiter_next_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn dequeiter_next_lock_held(it: *mut core::ffi::c_void, deque: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// dequereviter_next_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn dequereviter_next_lock_held(it: *mut core::ffi::c_void, deque: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// create_timezone - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn create_timezone(offset: *mut core::ffi::c_void, name: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// create_timezone_from_delta - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn create_timezone_from_delta(days: i32, sec: i32, ms: i32, normalize: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// dbm_length_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn dbm_length_lock_held(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// dbm_bool_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn dbm_bool_lock_held(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// dbm_subscript_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn dbm_subscript_lock_held(arg0: *mut core::ffi::c_void, key: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// dbm_ass_sub_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn dbm_ass_sub_lock_held(arg0: *mut core::ffi::c_void, v: *mut core::ffi::c_void, w: *mut core::ffi::c_void) -> i32 {
    0
}

/// dbm_contains_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn dbm_contains_lock_held(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> i32 {
    0
}

/// create_elementiter - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn create_elementiter(st: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, tag: *mut core::ffi::c_void, gettext: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// bounded_lru_cache_get_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn bounded_lru_cache_get_lock_held(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void, kwds: *mut core::ffi::c_void, result: *mut *mut core::ffi::c_void, key: *mut *mut core::ffi::c_void, hash: *mut core::ffi::c_void) -> i32 {
    0
}

/// bounded_lru_cache_update_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn bounded_lru_cache_update_lock_held(arg0: *mut core::ffi::c_void, result: *mut core::ffi::c_void, key: *mut core::ffi::c_void, hash: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// gdbm_length_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn gdbm_length_lock_held(op: *mut core::ffi::c_void) -> usize {
    0
}

/// gdbm_bool_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn gdbm_bool_lock_held(op: *mut core::ffi::c_void) -> i32 {
    0
}

/// gdbm_subscript_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn gdbm_subscript_lock_held(op: *mut core::ffi::c_void, key: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// gdbm_ass_sub_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn gdbm_ass_sub_lock_held(op: *mut core::ffi::c_void, v: *mut core::ffi::c_void, w: *mut core::ffi::c_void) -> i32 {
    0
}

/// gdbm_contains_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn gdbm_contains_lock_held(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> i32 {
    0
}

/// _hashlib_HASH_copy_locked - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _hashlib_HASH_copy_locked(arg0: *mut core::ffi::c_void, new_ctx_p: *mut core::ffi::c_void) -> i32 {
    0
}

/// _hashlib_HASH_get_blocksize - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _hashlib_HASH_get_blocksize(op: *mut core::ffi::c_void, Py_UNUSEDclosure: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// hashlib_openssl_HMAC_ctx_copy_with_lock - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn hashlib_openssl_HMAC_ctx_copy_with_lock(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _hashlib_hmac_get_block_size - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _hashlib_hmac_get_block_size(op: *mut core::ffi::c_void, Py_UNUSEDclosure: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// channel_create - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn channel_create(channels: *mut core::ffi::c_void, defaults: usize) -> i64 {
    0
}

/// channelsmod_create - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn channelsmod_create(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void, kwds: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _queue_lock - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _queue_lock(queue: *mut core::ffi::c_void) -> i32 {
    0
}

/// _queue_unlock - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _queue_unlock(queue: *mut core::ffi::c_void) {

}

/// queue_create - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn queue_create(queues: *mut core::ffi::c_void, maxsize: usize, defaults: usize) -> i64 {
    0
}

/// create_indent_cache - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn create_indent_cache(s: *mut core::ffi::c_void, indent_level: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _encoder_iterate_mapping_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _encoder_iterate_mapping_lock_held(s: *mut core::ffi::c_void, writer: *mut core::ffi::c_void, first: *mut core::ffi::c_void, dct: *mut core::ffi::c_void, items: *mut core::ffi::c_void, indent_level: usize, indent_cache: *mut core::ffi::c_void, separator: *mut core::ffi::c_void) -> i32 {
    0
}

/// _encoder_iterate_dict_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _encoder_iterate_dict_lock_held(s: *mut core::ffi::c_void, writer: *mut core::ffi::c_void, first: *mut core::ffi::c_void, dct: *mut core::ffi::c_void, indent_level: usize, indent_cache: *mut core::ffi::c_void, separator: *mut core::ffi::c_void) -> i32 {
    0
}

/// _encoder_iterate_fast_seq_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _encoder_iterate_fast_seq_lock_held(s: *mut core::ffi::c_void, writer: *mut core::ffi::c_void, seq: *mut core::ffi::c_void, s_fast: *mut core::ffi::c_void, indent_level: usize, indent_cache: *mut core::ffi::c_void, separator: *mut core::ffi::c_void) -> i32 {
    0
}

/// _create_tuple_for_attribute - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _create_tuple_for_attribute(state: *mut core::ffi::c_void, name: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _create_tuple_for_X509_NAME - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _create_tuple_for_X509_NAME(state: *mut core::ffi::c_void, xname: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// load_cert_chain_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn load_cert_chain_lock_held(arg0: *mut core::ffi::c_void, pw_info: *mut core::ffi::c_void, certfile_bytes: *mut core::ffi::c_void, keyfile_bytes: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// sslmodule_init_lock - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn sslmodule_init_lock(module: *mut core::ffi::c_void) -> i32 {
    0
}

/// create_cfunction - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn create_cfunction(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// create_managed_weakref_nogc_type - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn create_managed_weakref_nogc_type(arg0: *mut core::ffi::c_void, Py_UNUSEDargs: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// create_manual_heap_type - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn create_manual_heap_type() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// create_managed_dict_type - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn create_managed_dict_type() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// create_interpreter - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn create_interpreter(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void, kwargs: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// createfunc_nonmodule - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn createfunc_nonmodule(spec: *mut core::ffi::c_void, def: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyInit__testmultiphase_create_int_with_state - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_create_int_with_state() -> usize {
    0
}

/// createfunc_noop - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn createfunc_noop(spec: *mut core::ffi::c_void, def: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyInit__testmultiphase_multiple_create_slots - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_multiple_create_slots() -> usize {
    0
}

/// createfunc_null - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn createfunc_null(spec: *mut core::ffi::c_void, def: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyInit__testmultiphase_create_null - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_create_null() -> usize {
    0
}

/// createfunc_raise - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn createfunc_raise(spec: *mut core::ffi::c_void, def: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyInit__testmultiphase_create_raise - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_create_raise() -> usize {
    0
}

/// createfunc_unreported_exception - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn createfunc_unreported_exception(spec: *mut core::ffi::c_void, def: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyInit__testmultiphase_create_unreported_exception - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_create_unreported_exception() -> usize {
    0
}

/// PyModule_Create - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyModule_Create(arg0: usize) -> usize {
    0
}

/// lock_acquire_parse_timeout - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn lock_acquire_parse_timeout(timeout_obj: *mut core::ffi::c_void, blocking: i32, timeout: *mut core::ffi::c_void) -> i32 {
    0
}

/// lock_repr - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn lock_repr(op: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// rlock_locked_impl - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn rlock_locked_impl(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// PyMutex_IsLocked - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyMutex_IsLocked(arg0: usize) -> usize {
    0
}

/// rlock_repr - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn rlock_repr(op: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// create_localsdict - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn create_localsdict(arg0: *mut core::ffi::c_void, state: *mut core::ffi::c_void, localsdict: *mut *mut core::ffi::c_void, sentinel_wr: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// create_sentinel_wr - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn create_sentinel_wr(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// create_localdummies - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn create_localdummies(state: *mut core::ffi::c_void) -> i32 {
    0
}

/// Tcl_CreateFileHandler - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn Tcl_CreateFileHandler(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// py_UuidCreate - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn py_UuidCreate(Py_UNUSEDcontext: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateBuffer - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateBuffer(pDesc: *mut core::ffi::c_void, pInitialData: *mut core::ffi::c_void, ppBuffer: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateTexture1D - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateTexture1D(pDesc: *mut core::ffi::c_void, pInitialData: *mut core::ffi::c_void, ppTexture1D: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateTexture2D - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateTexture2D(pDesc: *mut core::ffi::c_void, pInitialData: *mut core::ffi::c_void, ppTexture2D: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateTexture3D - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateTexture3D(pDesc: *mut core::ffi::c_void, pInitialData: *mut core::ffi::c_void, ppTexture3D: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateShaderResourceView1 - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateShaderResourceView1(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, ppSRView: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateInputLayout - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateInputLayout(pInputElementDescs: *mut core::ffi::c_void, NumElements: u32, pShaderBytecodeWithInputSignature: *mut core::ffi::c_void, BytecodeLength: usize, ppInputLayout: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVertexShader - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateVertexShader(pShaderBytecode: *mut core::ffi::c_void, BytecodeLength: usize, ppVertexShader: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateGeometryShader - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateGeometryShader(pShaderBytecode: *mut core::ffi::c_void, BytecodeLength: usize, ppGeometryShader: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateGeometryShaderWithStreamOutput - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateGeometryShaderWithStreamOutput(pShaderBytecode: *mut core::ffi::c_void, BytecodeLength: usize, pSODeclaration: *mut core::ffi::c_void, NumEntries: u32, OutputStreamStride: u32, ppGeometryShader: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreatePixelShader - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreatePixelShader(pShaderBytecode: *mut core::ffi::c_void, BytecodeLength: usize, ppPixelShader: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateBlendState - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateBlendState(pBlendStateDesc: *mut core::ffi::c_void, ppBlendState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateBlendState1 - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateBlendState1(pBlendStateDesc: *mut core::ffi::c_void, ppBlendState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateDepthStencilState - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateDepthStencilState(pDepthStencilDesc: *mut core::ffi::c_void, ppDepthStencilState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateRasterizerState - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateRasterizerState(pRasterizerDesc: *mut core::ffi::c_void, ppRasterizerState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateSamplerState - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateSamplerState(pSamplerDesc: *mut core::ffi::c_void, ppSamplerState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateQuery - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateQuery(pQueryDesc: *mut core::ffi::c_void, ppQuery: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreatePredicate - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreatePredicate(pPredicateDesc: *mut core::ffi::c_void, ppPredicate: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateCounter - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateCounter(pCounterDesc: *mut core::ffi::c_void, ppCounter: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// AcquireLock - from dxvk/d3d10_multithread.h
#[no_mangle]
pub unsafe extern "C" fn AcquireLock() -> usize {
    0
}

/// LockContext - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn LockContext() -> usize {
    0
}

/// D3D10DeviceLock - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn D3D10DeviceLock() -> usize {
    0
}

/// blockDim - from dxvk/d3d11_cuda.h
#[no_mangle]
pub unsafe extern "C" fn blockDim() -> usize {
    0
}

/// CreateTexture2D1 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateTexture2D1(pDesc: *mut core::ffi::c_void, pInitialData: *mut core::ffi::c_void, ppTexture2D: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateTexture2DBase - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateTexture2DBase(pDesc: *mut core::ffi::c_void, pInitialData: *mut core::ffi::c_void, ppTexture2D: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateTexture3D1 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateTexture3D1(pDesc: *mut core::ffi::c_void, pInitialData: *mut core::ffi::c_void, ppTexture3D: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateTexture3DBase - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateTexture3DBase(pDesc: *mut core::ffi::c_void, pInitialData: *mut core::ffi::c_void, ppTexture3D: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateShaderResourceViewBase - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateShaderResourceViewBase(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, ppSRView: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateUnorderedAccessView1 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateUnorderedAccessView1(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, ppUAView: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateUnorderedAccessViewBase - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateUnorderedAccessViewBase(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, ppUAView: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateRenderTargetView1 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateRenderTargetView1(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, ppRTView: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateRenderTargetViewBase - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateRenderTargetViewBase(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, ppRTView: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateHullShader - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateHullShader(pShaderBytecode: *mut core::ffi::c_void, BytecodeLength: usize, pClassLinkage: *mut core::ffi::c_void, ppHullShader: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateDomainShader - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateDomainShader(pShaderBytecode: *mut core::ffi::c_void, BytecodeLength: usize, pClassLinkage: *mut core::ffi::c_void, ppDomainShader: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateComputeShader - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateComputeShader(pShaderBytecode: *mut core::ffi::c_void, BytecodeLength: usize, pClassLinkage: *mut core::ffi::c_void, ppComputeShader: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateClassLinkage - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateClassLinkage(ppLinkage: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateRasterizerState1 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateRasterizerState1(pRasterizerDesc: *mut core::ffi::c_void, ppRasterizerState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateRasterizerState2 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateRasterizerState2(pRasterizerDesc: *mut core::ffi::c_void, ppRasterizerState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateQuery1 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateQuery1(pQueryDesc: *mut core::ffi::c_void, ppQuery: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateQueryBase - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateQueryBase(pQueryDesc: *mut core::ffi::c_void, ppQuery: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateDeferredContext - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateDeferredContext(ContextFlags: u32, ppDeferredContext: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateDeferredContext1 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateDeferredContext1(ContextFlags: u32, ppDeferredContext: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateDeferredContext2 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateDeferredContext2(ContextFlags: u32, ppDeferredContext: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateDeferredContext3 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateDeferredContext3(ContextFlags: u32, ppDeferredContext: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateDeviceContextState - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateDeviceContextState(Flags: u32, pFeatureLevels: *mut core::ffi::c_void, FeatureLevels: u32, SDKVersion: u32, EmulatedInterface: usize, pChosenFeatureLevel: *mut core::ffi::c_void, ppContextState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// LockImage - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn LockImage(Image: usize, Usage: usize) -> usize {
    0
}

/// CreateShaderModule - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateShaderModule(pShaderModule: *mut core::ffi::c_void, pLinkage: *mut core::ffi::c_void, ShaderKey: usize, pShaderBytecode: *mut core::ffi::c_void, BytecodeLength: usize, ModuleInfo: usize) -> i32 {
    0
}

/// CreateCubinComputeShaderWithNameNVX - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateCubinComputeShaderWithNameNVX(pCubin: *mut core::ffi::c_void, size: u32, blockX: u32, blockY: u32, blockZ: u32, pShaderName: *mut i8, phShader: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateUnorderedAccessViewAndGetDriverHandleNVX - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateUnorderedAccessViewAndGetDriverHandleNVX(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, ppUAV: *mut *mut core::ffi::c_void, pDriverHandle: *mut u32) -> usize {
    0
}

/// CreateShaderResourceViewAndGetDriverHandleNVX - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateShaderResourceViewAndGetDriverHandleNVX(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, ppSRV: *mut *mut core::ffi::c_void, pDriverHandle: *mut u32) -> usize {
    0
}

/// CreateSamplerStateAndGetDriverHandleNVX - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateSamplerStateAndGetDriverHandleNVX(pSamplerDesc: *mut core::ffi::c_void, ppSamplerState: *mut *mut core::ffi::c_void, pDriverHandle: *mut u32) -> usize {
    0
}

/// LockBuffer - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn LockBuffer(Buffer: usize) {

}

/// CreateVideoDecoder - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn CreateVideoDecoder(pDesc: *mut core::ffi::c_void, riid: usize, ppVideoDecoder: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVideoProcessor - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateVideoProcessor(pEnum: *mut core::ffi::c_void, RateConversionIndex: u32, ppVideoProcessor: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateAuthenticatedChannel - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateAuthenticatedChannel(ChannelType: usize, ppAuthenticatedChannel: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateCryptoSession - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateCryptoSession(pCryptoType: *mut core::ffi::c_void, pDecoderProfile: *mut core::ffi::c_void, pKeyExchangeType: *mut core::ffi::c_void, ppCryptoSession: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVideoDecoderOutputView - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateVideoDecoderOutputView(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, ppVDOVView: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVideoProcessorInputView - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateVideoProcessorInputView(pResource: *mut core::ffi::c_void, pEnum: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, ppVPIView: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVideoProcessorOutputView - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateVideoProcessorOutputView(pResource: *mut core::ffi::c_void, pEnum: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, ppVPOView: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVideoProcessorEnumerator - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateVideoProcessorEnumerator(pDesc: *mut core::ffi::c_void, ppEnum: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateSwapChain - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateSwapChain(pSurfaceFactory: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, ppSwapChain: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateSurface - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateSurface(pDesc: *mut core::ffi::c_void, NumSurfaces: u32, Usage: usize, pSharedResource: *mut core::ffi::c_void, ppSurface: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// ExecuteFlushLocked - from dxvk/d3d11_initializer.h
#[no_mangle]
pub unsafe extern "C" fn ExecuteFlushLocked() {

}

/// FlushCsChunkLocked - from dxvk/d3d11_initializer.h
#[no_mangle]
pub unsafe extern "C" fn FlushCsChunkLocked() {

}

/// NotifyContextFlushLocked - from dxvk/d3d11_initializer.h
#[no_mangle]
pub unsafe extern "C" fn NotifyContextFlushLocked() {

}

/// LockSubmissionQueue - from dxvk/d3d11_interop.h
#[no_mangle]
pub unsafe extern "C" fn LockSubmissionQueue() -> usize {
    0
}

/// CreateTexture2DFromVkImage - from dxvk/d3d11_interop.h
#[no_mangle]
pub unsafe extern "C" fn CreateTexture2DFromVkImage(pDesc: *mut core::ffi::c_void, vkImage: usize, ppTexture2D: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateWrappedResource - from dxvk/d3d11_on_12.h
#[no_mangle]
pub unsafe extern "C" fn CreateWrappedResource(pResource12: *mut core::ffi::c_void, pResourceFlags: *mut core::ffi::c_void, InputState: usize, OutputState: usize, riid: usize, ppResource11: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// LockCommandQueue - from dxvk/d3d11_on_12_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn LockCommandQueue(pCommandQueue: *mut core::ffi::c_void) -> usize {
    0
}

/// UnlockCommandQueue - from dxvk/d3d11_on_12_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn UnlockCommandQueue(pCommandQueue: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateSubresourceSurface - from dxvk/d3d11_resource.h
#[no_mangle]
pub unsafe extern "C" fn CreateSubresourceSurface(index: u32, ppSurface: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetKeyedMutex - from dxvk/d3d11_resource.h
#[no_mangle]
pub unsafe extern "C" fn GetKeyedMutex(ppvObject: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// CreateIrShader - from dxvk/d3d11_shader.h
#[no_mangle]
pub unsafe extern "C" fn CreateIrShader(pDevice: *mut core::ffi::c_void, ShaderKey: usize, ModuleInfo: usize, pShaderBytecode: *mut core::ffi::c_void, BytecodeLength: usize, Icb: usize) {

}

/// CreateFrameLatencyEvent - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn CreateFrameLatencyEvent() {

}

/// CreatePresenter - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn CreatePresenter() {

}

/// CreateBackBuffers - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn CreateBackBuffers() {

}

/// CreateBlitter - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn CreateBlitter() {

}

/// CreateMappedBuffer - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn CreateMappedBuffer(Subresource: u32) {

}

/// CreateViewInfo - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn CreateViewInfo(Desc: usize) -> usize {
    0
}

/// CreateUniformBuffer - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn CreateUniformBuffer() {

}

/// CreateShaders - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn CreateShaders() {

}

/// CreateResources - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn CreateResources() {

}

/// Lock - from dxvk/d3d8_batch.h
#[no_mangle]
pub unsafe extern "C" fn Lock(OffsetToLock: u32, SizeToLock: u32, ppbData: *mut *mut u8, Flags: u32) -> usize {
    0
}

/// Unlock - from dxvk/d3d8_batch.h
#[no_mangle]
pub unsafe extern "C" fn Unlock() -> usize {
    0
}

/// CreateVertexBuffer - from dxvk/d3d8_batch.h
#[no_mangle]
pub unsafe extern "C" fn CreateVertexBuffer(Length: u32, Usage: u32, FVF: u32, Pool: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateAdditionalSwapChain - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateAdditionalSwapChain(pPresentationParameters: *mut core::ffi::c_void, ppSwapChain: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateTexture - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateTexture(Width: u32, Height: u32, Levels: u32, Usage: u32, Format: usize, Pool: usize, ppTexture: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVolumeTexture - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateVolumeTexture(Width: u32, Height: u32, Depth: u32, Levels: u32, Usage: u32, Format: usize, Pool: usize, ppVolumeTexture: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateCubeTexture - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateCubeTexture(EdgeLength: u32, Levels: u32, Usage: u32, Format: usize, Pool: usize, ppCubeTexture: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateIndexBuffer - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateIndexBuffer(Length: u32, Usage: u32, Format: usize, Pool: usize, ppIndexBuffer: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateRenderTarget - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateRenderTarget(Width: u32, Height: u32, Format: usize, MultiSample: usize, Lockable: i32, ppSurface: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateDepthStencilSurface - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateDepthStencilSurface(Width: u32, Height: u32, Format: usize, MultiSample: usize, ppSurface: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateImageSurface - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateImageSurface(Width: u32, Height: u32, Format: usize, ppSurface: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateStateBlock - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateStateBlock(Type: usize, pToken: *mut u32) -> usize {
    0
}

/// CaptureStateBlock - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn CaptureStateBlock(Token: u32) -> usize {
    0
}

/// ApplyStateBlock - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn ApplyStateBlock(Token: u32) -> usize {
    0
}

/// DeleteStateBlock - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn DeleteStateBlock(Token: u32) -> usize {
    0
}

/// BeginStateBlock - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn BeginStateBlock() -> usize {
    0
}

/// EndStateBlock - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn EndStateBlock(pToken: *mut u32) -> usize {
    0
}

/// LockDevice - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn LockDevice() -> usize {
    0
}

/// RecreateBackBuffersAndAutoDepthStencil - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn RecreateBackBuffersAndAutoDepthStencil() {

}

/// ConvertStateBlockType - from dxvk/d3d8_state_block.h
#[no_mangle]
pub unsafe extern "C" fn ConvertStateBlockType(arg0: usize) -> usize {
    0
}

/// LockRect - from dxvk/d3d8_surface.h
#[no_mangle]
pub unsafe extern "C" fn LockRect(pLockedRect: *mut core::ffi::c_void, pRect: *mut core::ffi::c_void, Flags: u32) -> usize {
    0
}

/// UnlockRect - from dxvk/d3d8_surface.h
#[no_mangle]
pub unsafe extern "C" fn UnlockRect() -> usize {
    0
}

/// LockBox - from dxvk/d3d8_texture.h
#[no_mangle]
pub unsafe extern "C" fn LockBox(Level: u32, pLockedBox: *mut core::ffi::c_void, pBox: *mut core::ffi::c_void, Flags: u32) -> usize {
    0
}

/// UnlockBox - from dxvk/d3d8_texture.h
#[no_mangle]
pub unsafe extern "C" fn UnlockBox(Level: u32) -> usize {
    0
}

/// IncrementLockCount - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn IncrementLockCount() -> u32 {
    0
}

/// DecrementLockCount - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn DecrementLockCount() -> u32 {
    0
}

/// GetLockCount - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetLockCount() -> u32 {
    0
}

/// CreateSampleView - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn CreateSampleView(Lod: u32) {

}

/// SetLocked - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn SetLocked(Subresource: u32, value: usize) {

}

/// GetLocked - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetLocked(Subresource: u32) -> usize {
    0
}

/// IsAnySubresourceLocked - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn IsAnySubresourceLocked() -> usize {
    0
}

/// CreateOffscreenPlainSurface - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateOffscreenPlainSurface(Width: u32, Height: u32, Format: usize, Pool: usize, ppSurface: *mut *mut core::ffi::c_void, pSharedHandle: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVertexDeclaration - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateVertexDeclaration(pVertexElements: *mut core::ffi::c_void, ppDecl: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateRenderTargetEx - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateRenderTargetEx(Width: u32, Height: u32, Format: usize, MultiSample: usize, MultisampleQuality: u32, Lockable: i32, ppSurface: *mut *mut core::ffi::c_void, pSharedHandle: *mut *mut core::ffi::c_void, Usage: u32) -> usize {
    0
}

/// CreateOffscreenPlainSurfaceEx - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateOffscreenPlainSurfaceEx(Width: u32, Height: u32, Format: usize, Pool: usize, ppSurface: *mut *mut core::ffi::c_void, pSharedHandle: *mut *mut core::ffi::c_void, Usage: u32) -> usize {
    0
}

/// CreateDepthStencilSurfaceEx - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateDepthStencilSurfaceEx(Width: u32, Height: u32, Format: usize, MultiSample: usize, MultisampleQuality: u32, Discard: i32, ppSurface: *mut *mut core::ffi::c_void, pSharedHandle: *mut *mut core::ffi::c_void, Usage: u32) -> usize {
    0
}

/// CreateAdditionalSwapChainEx - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateAdditionalSwapChainEx(pPresentationParameters: *mut core::ffi::c_void, pFullscreenDisplayMode: *mut core::ffi::c_void, ppSwapChain: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CalcImageLockOffset - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn CalcImageLockOffset(SlicePitch: u32, RowPitch: u32, FormatInfo: *mut core::ffi::c_void, pBox: *mut core::ffi::c_void) -> u32 {
    0
}

/// UnlockImage - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UnlockImage(pResource: *mut core::ffi::c_void, Face: u32, MipLevel: u32) -> i32 {
    0
}

/// UnlockBuffer - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UnlockBuffer(pResource: *mut core::ffi::c_void) -> i32 {
    0
}

/// HasFormatsUnlocked - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn HasFormatsUnlocked() -> usize {
    0
}

/// GetFormatAlignedBlockSize - from dxvk/d3d9_format.h
#[no_mangle]
pub unsafe extern "C" fn GetFormatAlignedBlockSize(Format: usize) -> usize {
    0
}

/// CreatePipelineLayout - from dxvk/d3d9_format_helpers.h
#[no_mangle]
pub unsafe extern "C" fn CreatePipelineLayout() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreatePipeline - from dxvk/d3d9_format_helpers.h
#[no_mangle]
pub unsafe extern "C" fn CreatePipeline(size: usize, code: *mut u32, specConstant: u32) -> usize {
    0
}

/// Direct3DCreate9On12 - from dxvk/d3d9_include.h
#[no_mangle]
pub unsafe extern "C" fn Direct3DCreate9On12(sdk_version: u32, override_list: *mut core::ffi::c_void, override_entry_count: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// Direct3DCreate9On12Ex - from dxvk/d3d9_include.h
#[no_mangle]
pub unsafe extern "C" fn Direct3DCreate9On12Ex(sdk_version: u32, override_list: *mut core::ffi::c_void, override_entry_count: u32, output: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// CreateDeviceEx - from dxvk/d3d9_interface.h
#[no_mangle]
pub unsafe extern "C" fn CreateDeviceEx(Adapter: u32, DeviceType: usize, hFocusWindow: *mut core::ffi::c_void, BehaviorFlags: u32, pPresentationParameters: *mut core::ffi::c_void, pFullscreenDisplayMode: *mut core::ffi::c_void, ppReturnedDeviceInterface: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// UnlockDevice - from dxvk/d3d9_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn UnlockDevice() -> usize {
    0
}

/// CreateImage - from dxvk/d3d9_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn CreateImage(desc: *mut core::ffi::c_void, ppResult: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// UnlockAdditionalFormats - from dxvk/d3d9_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn UnlockAdditionalFormats() -> usize {
    0
}

/// CreateTextureResource - from dxvk/d3d9_interop.h
#[no_mangle]
pub unsafe extern "C" fn CreateTextureResource(desc: usize, ppResult: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// MapLocked - from dxvk/d3d9_mem.h
#[no_mangle]
pub unsafe extern "C" fn MapLocked(memory: *mut core::ffi::c_void, mappedSize: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// UnmapLocked - from dxvk/d3d9_mem.h
#[no_mangle]
pub unsafe extern "C" fn UnmapLocked(memory: *mut core::ffi::c_void) -> u32 {
    0
}

/// CreateLegacyShader - from dxvk/d3d9_shader.h
#[no_mangle]
pub unsafe extern "C" fn CreateLegacyShader(pDevice: *mut core::ffi::c_void, ShaderKey: usize, ModuleInfo: usize, pShaderBytecode: *mut core::ffi::c_void) {

}

/// CreateSwapChainForHwnd - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn CreateSwapChainForHwnd(pDevice: *mut core::ffi::c_void, hWnd: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, pFullscreenDesc: *mut core::ffi::c_void, pRestrictToOutput: *mut core::ffi::c_void, ppSwapChain: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateSwapChainForCoreWindow - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn CreateSwapChainForCoreWindow(pDevice: *mut core::ffi::c_void, pWindow: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, pRestrictToOutput: *mut core::ffi::c_void, ppSwapChain: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateSwapChainForComposition - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn CreateSwapChainForComposition(pDevice: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, pRestrictToOutput: *mut core::ffi::c_void, ppSwapChain: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateSwapChainBase - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn CreateSwapChainBase(pDevice: *mut core::ffi::c_void, hWnd: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, pFullscreenDesc: *mut core::ffi::c_void, pRestrictToOutput: *mut core::ffi::c_void, ppSwapChain: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateDummyWindow - from dxvk/dxgi_surface.h
#[no_mangle]
pub unsafe extern "C" fn CreateDummyWindow() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cfgFindBlock - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn cfgFindBlock(types: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// createDebugName - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn createDebugName(name: *mut i8) -> usize {
    0
}

/// createDescriptorRange - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn createDescriptorRange() -> usize {
    0
}

/// createPipeline - from dxvk/dxvk_compute.h
#[no_mangle]
pub unsafe extern "C" fn createPipeline(state: usize) -> usize {
    0
}

/// computePushDataBlockOffset - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn computePushDataBlockOffset(index: u32) -> u32 {
    0
}

/// createDescriptorPool - from dxvk/dxvk_descriptor_pool.h
#[no_mangle]
pub unsafe extern "C" fn createDescriptorPool() -> usize {
    0
}

/// getBlock - from dxvk/dxvk_descriptor_worker.h
#[no_mangle]
pub unsafe extern "C" fn getBlock() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// flushBlock - from dxvk/dxvk_descriptor_worker.h
#[no_mangle]
pub unsafe extern "C" fn flushBlock() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// processBlock - from dxvk/dxvk_descriptor_worker.h
#[no_mangle]
pub unsafe extern "C" fn processBlock(block: usize) {

}

/// createBuiltInPipelineLayout - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn createBuiltInPipelineLayout(flags: usize, pushDataStages: usize, pushDataSize: u64, bindingCount: u32, bindings: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// createBuiltInComputePipeline - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn createBuiltInComputePipeline(layout: *mut core::ffi::c_void, stage: usize) -> usize {
    0
}

/// createBuiltInGraphicsPipeline - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn createBuiltInGraphicsPipeline(layout: *mut core::ffi::c_void, state: usize) -> usize {
    0
}

/// lockSubmission - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn lockSubmission() {

}

/// unlockSubmission - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn unlockSubmission() {

}

/// accumulateQueryDataForGpuQueryLocked - from dxvk/dxvk_gpu_query.h
#[no_mangle]
pub unsafe extern "C" fn accumulateQueryDataForGpuQueryLocked(query: usize) -> usize {
    0
}

/// accumulateQueryDataLocked - from dxvk/dxvk_gpu_query.h
#[no_mangle]
pub unsafe extern "C" fn accumulateQueryDataLocked() -> usize {
    0
}

/// createQueryPool - from dxvk/dxvk_gpu_query.h
#[no_mangle]
pub unsafe extern "C" fn createQueryPool() {

}

/// canCreateBasePipeline - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn canCreateBasePipeline(state: usize) -> usize {
    0
}

/// createBasePipeline - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn createBasePipeline(key: usize) -> usize {
    0
}

/// createOptimizedPipeline - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn createOptimizedPipeline(key: usize) -> usize {
    0
}

/// createView - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn createView(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// setKeyedMutex - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn setKeyedMutex(mutex: usize) {

}

/// getImageCreateInfo - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn getImageCreateInfo(usageInfo: usize) -> usize {
    0
}

/// createBufferView - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn createBufferView(key: usize, baseOffset: u64) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// createImageView - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn createImageView(key: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// createPool - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn createPool() {

}

/// lockResourceGpuAddress - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn lockResourceGpuAddress(allocation: usize) {

}

/// performTimedTasksLocked - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn performTimedTasksLocked(currentTime: usize) {

}

/// createPipelineLayout - from dxvk/dxvk_meta_blit.h
#[no_mangle]
pub unsafe extern "C" fn createPipelineLayout() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// createCopyToImagePipeline - from dxvk/dxvk_meta_copy.h
#[no_mangle]
pub unsafe extern "C" fn createCopyToImagePipeline(layout: *mut core::ffi::c_void, vs: usize, ps: usize, dstFormat: usize, dstAspects: usize, samples: usize, bitwiseStencil: usize) -> usize {
    0
}

/// createImageCopyPipeline - from dxvk/dxvk_meta_copy.h
#[no_mangle]
pub unsafe extern "C" fn createImageCopyPipeline(key: usize) -> usize {
    0
}

/// createInputAttachmentCopyPipeline - from dxvk/dxvk_meta_copy.h
#[no_mangle]
pub unsafe extern "C" fn createInputAttachmentCopyPipeline(key: usize) -> usize {
    0
}

/// createBufferToImageCopyPipeline - from dxvk/dxvk_meta_copy.h
#[no_mangle]
pub unsafe extern "C" fn createBufferToImageCopyPipeline(key: usize) -> usize {
    0
}

/// createImageToBufferCopyPipeline - from dxvk/dxvk_meta_copy.h
#[no_mangle]
pub unsafe extern "C" fn createImageToBufferCopyPipeline(key: usize) -> usize {
    0
}

/// createPackedImageBufferCopyPipeline - from dxvk/dxvk_meta_copy.h
#[no_mangle]
pub unsafe extern "C" fn createPackedImageBufferCopyPipeline(key: usize) -> usize {
    0
}

/// createViews - from dxvk/dxvk_meta_mipgen.h
#[no_mangle]
pub unsafe extern "C" fn createViews(pass: u32) -> usize {
    0
}

/// getBlockOffset - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getBlockOffset() -> u32 {
    0
}

/// setBlockOffset - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn setBlockOffset(offset: u32) {

}

/// getBlockEntrySize - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getBlockEntrySize() -> u32 {
    0
}

/// getPushDataBlock - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getPushDataBlock(index: u32) -> usize {
    0
}

/// createComputePipeline - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn createComputePipeline(shaders: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// createGraphicsPipeline - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn createGraphicsPipeline(shaders: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// createShaderPipelineLibrary - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn createShaderPipelineLibrary(key: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// createVertexInputLibrary - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn createVertexInputLibrary(state: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// createFragmentOutputLibrary - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn createFragmentOutputLibrary(state: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// createDescriptorSetLayout - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn createDescriptorSetLayout(key: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// createPipelineLibraryLocked - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn createPipelineLibraryLocked(key: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// createNullFsPipelineLibrary - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn createNullFsPipelineLibrary() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// findPipelineLibraryLocked - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn findPipelineLibraryLocked(key: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// recreateSwapChain - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn recreateSwapChain() -> i32 {
    0
}

/// createSwapChain - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn createSwapChain() -> i32 {
    0
}

/// createSurface - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn createSurface() -> i32 {
    0
}

/// createLatencySemaphore - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn createLatencySemaphore() -> i32 {
    0
}

/// lockDeviceQueue - from dxvk/dxvk_queue.h
#[no_mangle]
pub unsafe extern "C" fn lockDeviceQueue() {

}

/// unlockDeviceQueue - from dxvk/dxvk_queue.h
#[no_mangle]
pub unsafe extern "C" fn unlockDeviceQueue() {

}

/// createSampler - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn createSampler(index: u16, createInfo: *mut core::ffi::c_void) -> usize {
    0
}

/// destroyShaderPipelineLocked - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn destroyShaderPipelineLocked() {

}

/// compileShaderPipelineLocked - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn compileShaderPipelineLocked() -> usize {
    0
}

/// generateModuleIdentifierLocked - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn generateModuleIdentifierLocked(identifier: *mut core::ffi::c_void, spirvCode: usize) {

}

/// canCreatePipelineLibrary - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn canCreatePipelineLibrary() -> usize {
    0
}

/// canCreatePipelineLibraryForShader - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn canCreatePipelineLibraryForShader(shader: usize, needsPosition: usize) -> usize {
    0
}

/// emitConditionalBlock - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn emitConditionalBlock(builder: usize, cond: usize) -> usize {
    0
}

/// tryInitializeLocked - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn tryInitializeLocked() -> usize {
    0
}

/// getShaderCreateInfo - from dxvk/dxvk_shader_ir.h
#[no_mangle]
pub unsafe extern "C" fn getShaderCreateInfo() -> usize {
    0
}

/// createHudImage - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn createHudImage(extent: usize) {

}

/// createBlitPipelineLayout - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn createBlitPipelineLayout() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// createCursorPipelineLayout - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn createCursorPipelineLayout() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// createBlitPipeline - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn createBlitPipeline(key: usize) -> usize {
    0
}

/// createCursorPipeline - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn createCursorPipeline(key: usize) -> usize {
    0
}

/// encodeClearBlockValue - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn encodeClearBlockValue(format: usize, color: usize) -> usize {
    0
}

/// getBlockId - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn getBlockId() -> u32 {
    0
}

/// decorateBlock - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn decorateBlock(object: u32) {

}

/// opBeginInvocationInterlock - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opBeginInvocationInterlock() {

}

/// opEndInvocationInterlock - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opEndInvocationInterlock() {

}

/// classifyBlocks - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn classifyBlocks(reachableBlocks: usize, mergeBlocks: usize) {

}

/// lock - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn lock() {

}

/// try_lock - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn try_lock() -> usize {
    0
}

/// lock_shared - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn lock_shared() {

}

/// unlock_shared - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn unlock_shared() {

}

/// try_lock_shared - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn try_lock_shared() -> usize {
    0
}

/// createDirectory - from dxvk/util_env.h
#[no_mangle]
pub unsafe extern "C" fn createDirectory(path: usize) -> usize {
    0
}

/// D3DKMTAcquireKeyedMutex - from dxvk/util_gdi.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTAcquireKeyedMutex(desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DKMTCreateDCFromMemory - from dxvk/util_gdi.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTCreateDCFromMemory(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTCreateDevice - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTCreateDevice(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTCreateKeyedMutex2 - from dxvk/util_gdi.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTCreateKeyedMutex2(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTDestroyKeyedMutex - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTDestroyKeyedMutex(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTOpenKeyedMutex - from dxvk/util_gdi.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTOpenKeyedMutex(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTReleaseKeyedMutex - from dxvk/util_gdi.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTReleaseKeyedMutex(desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// createResources - from dxvk/dxvk_hud_item.h
#[no_mangle]
pub unsafe extern "C" fn createResources(arg0: usize) {

}

/// createFontResources - from dxvk/dxvk_hud_renderer.h
#[no_mangle]
pub unsafe extern "C" fn createFontResources() {

}

/// demo_create_root_signature - from vkd3d-proton/demo.h
#[no_mangle]
pub unsafe extern "C" fn demo_create_root_signature(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, signature: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// demo_create_event - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn demo_create_event() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// xcb_surface_factory_create - from vkd3d-proton/demo_xcb.h
#[no_mangle]
pub unsafe extern "C" fn xcb_surface_factory_create(connection: *mut core::ffi::c_void, window: usize, out_factory: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_create_device - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_device(create_info: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_lock_vk_queue - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_lock_vk_queue(queue: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_unlock_vk_queue - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_unlock_vk_queue(queue: *mut core::ffi::c_void) {

}

/// vkd3d_create_root_signature_deserializer - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_root_signature_deserializer(data: *mut core::ffi::c_void, data_size: usize, iid: usize, deserializer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_create_versioned_root_signature_deserializer - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_versioned_root_signature_deserializer(data: *mut core::ffi::c_void, data_size: usize, iid: usize, deserializer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_create_versioned_root_signature_deserializer_for_subobject - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_versioned_root_signature_deserializer_for_subobject(data: *mut core::ffi::c_void, data_size: usize, subobject_name: *const u16, iid: usize, deserializer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// create_event - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn create_event() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// create_buffer_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn create_buffer_(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> usize {
    0
}

/// create_buffer2_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn create_buffer2_(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// create_default_texture_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn create_default_texture_(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize, arg7: usize, arg8: usize, arg9: usize) -> usize {
    0
}

/// create_default_texture_enhanced_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn create_default_texture_enhanced_(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize, arg7: usize, arg8: usize, arg9: usize) -> usize {
    0
}

/// create_root_signature - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn create_root_signature(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, root_signature: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// create_versioned_root_signature - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn create_versioned_root_signature(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, root_signature: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// create_pipeline_state_from_stream_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn create_pipeline_state_from_stream_(device: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, size: usize, state: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// ID3D12Device2_CreatePipelineState - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device2_CreatePipelineState(arg0: usize, arg1: usize, arg2: usize, arg3: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// create_render_target_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn create_render_target_(line: u32, context: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, render_target: *mut *mut core::ffi::c_void, rtv: *mut core::ffi::c_void) {

}

/// vkd3d_native_sync_handle_create - from vkd3d-proton/vkd3d_native_sync_handle.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_native_sync_handle_create(initial: u32, arg1: usize, handle: *mut core::ffi::c_void) -> i32 {
    0
}

/// spinlock_init - from vkd3d-proton/vkd3d_spinlock.h
#[no_mangle]
pub unsafe extern "C" fn spinlock_init(lock: *mut core::ffi::c_void) {

}

/// spinlock_try_acquire - from vkd3d-proton/vkd3d_spinlock.h
#[no_mangle]
pub unsafe extern "C" fn spinlock_try_acquire(lock: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_spinlock_try_lock - from vkd3d-proton/vkd3d_spinlock.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_spinlock_try_lock(arg0: usize) -> usize {
    0
}

/// spinlock_acquire - from vkd3d-proton/vkd3d_spinlock.h
#[no_mangle]
pub unsafe extern "C" fn spinlock_acquire(lock: *mut core::ffi::c_void) {

}

/// spinlock_release - from vkd3d-proton/vkd3d_spinlock.h
#[no_mangle]
pub unsafe extern "C" fn spinlock_release(lock: *mut core::ffi::c_void) {

}

/// rwlock_init - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn rwlock_init(lock: *mut core::ffi::c_void) -> i32 {
    0
}

/// rwlock_destroy - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn rwlock_destroy(lock: *mut core::ffi::c_void) -> i32 {
    0
}

/// DEBUG_CHANNEL_UNLOCK_MESSAGE - from vkd3d-proton/debug_channel.h
#[no_mangle]
pub unsafe extern "C" fn DEBUG_CHANNEL_UNLOCK_MESSAGE(buf: usize, offset: usize, num_words: usize) {

}

/// d3d12_dred_settings_create - from vkd3d-proton/debug.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_dred_settings_create(dred_settings: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_CreateGraphicsPipelineState_profiled - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateGraphicsPipelineState_profiled(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, riid: usize, pipeline_state: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateComputePipelineState_profiled - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateComputePipelineState_profiled(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, riid: usize, pipeline_state: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateDescriptorHeap_profiled - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateDescriptorHeap_profiled(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, riid: usize, descriptor_heap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateShaderResourceView_profiled - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateShaderResourceView_profiled(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateRenderTargetView_profiled - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateRenderTargetView_profiled(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateDepthStencilView_profiled - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateDepthStencilView_profiled(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateSampler_profiled - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSampler_profiled(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateSampler2_profiled - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSampler2_profiled(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateHeap1_profiled - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateHeap1_profiled(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, protected_session: *mut core::ffi::c_void, iid: usize, heap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateHeap_profiled - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateHeap_profiled(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, iid: usize, heap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreatePipelineState_profiled - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreatePipelineState_profiled(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, riid: usize, pipeline_state: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_create_timeline_semaphore - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_timeline_semaphore(device: *mut core::ffi::c_void, initial_value: u64, shared: usize, vk_semaphore: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_private_data_lock - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_private_data_lock(store: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_private_data_unlock - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_private_data_unlock(store: *mut core::ffi::c_void) {

}

/// d3d12_fence_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_create(device: *mut core::ffi::c_void, initial_value: u64, flags: usize, fence: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_shared_fence_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_create(device: *mut core::ffi::c_void, initial_value: u64, flags: usize, fence: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_heap_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_create(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, host_address: *mut core::ffi::c_void, heap: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_resource_create_borrowed - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_create_borrowed(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, vk_handle: usize, resource: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_resource_create_committed - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_create_committed(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, heap_properties: *mut core::ffi::c_void, heap_flags: usize, initial_state: usize, optimized_clear_value: *mut core::ffi::c_void, num_castable_formats: u32, castable_formats: *mut core::ffi::c_void, shared_handle: *mut core::ffi::c_void, resource: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_resource_create_placed - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_create_placed(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, heap_offset: u64, initial_state: usize, optimized_clear_value: *mut core::ffi::c_void, num_castable_formats: u32, castable_formats: *mut core::ffi::c_void, resource: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_resource_create_reserved - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_create_reserved(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, initial_state: usize, optimized_clear_value: *mut core::ffi::c_void, num_castable_formats: u32, castable_formats: *mut core::ffi::c_void, resource: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_create_buffer - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer(device: *mut core::ffi::c_void, heap_properties: *mut core::ffi::c_void, heap_flags: usize, desc: *mut core::ffi::c_void, tag: *mut i8, vk_buffer: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_create_buffer_view - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer_view(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, view: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_create_raw_r32ui_vk_buffer_view - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_raw_r32ui_vk_buffer_view(device: *mut core::ffi::c_void, vk_buffer: usize, offset: u64, range: u64, vk_view: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_create_acceleration_structure_view - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_acceleration_structure_view(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, view: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_create_opacity_micromap_view - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_opacity_micromap_view(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, view: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_create_texture_view - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_texture_view(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, view: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_desc_create_cbv - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_cbv(descriptor: usize, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// d3d12_desc_create_srv - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_srv(descriptor: usize, device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// d3d12_desc_create_uav - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_uav(descriptor: usize, device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, counter_resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// d3d12_desc_create_sampler - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_sampler(sampler: usize, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// d3d12_desc_create_cbv_embedded - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_cbv_embedded(descriptor: usize, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// d3d12_desc_create_srv_embedded - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_srv_embedded(descriptor: usize, device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// d3d12_desc_create_uav_embedded - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_uav_embedded(descriptor: usize, device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, counter_resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// d3d12_desc_create_sampler_embedded - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_create_sampler_embedded(sampler: usize, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// vkd3d_create_vk_buffer_view - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_vk_buffer_view(device: *mut core::ffi::c_void, vk_buffer: usize, format: *mut core::ffi::c_void, offset: u64, range: u64, vk_view: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_create_raw_buffer_view - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_raw_buffer_view(device: *mut core::ffi::c_void, gpu_address: usize, vk_buffer_view: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_create_static_sampler - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_create_static_sampler(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, vk_sampler: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_rtv_desc_create_rtv - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_rtv_desc_create_rtv(rtv_desc: *mut core::ffi::c_void, device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// d3d12_rtv_desc_create_dsv - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_rtv_desc_create_dsv(dsv_desc: *mut core::ffi::c_void, device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// d3d12_descriptor_heap_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_create(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor_heap: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_query_heap_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_create(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, heap: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_root_signature_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_create(device: *mut core::ffi::c_void, bytecode: *mut core::ffi::c_void, bytecode_length: usize, root_signature: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_root_signature_create_raw - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_create_raw(device: *mut core::ffi::c_void, payload: *mut core::ffi::c_void, payload_size: usize, root_signature: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_root_signature_create_empty - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_create_empty(device: *mut core::ffi::c_void, root_signature: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_root_signature_create_local_static_samplers_layout - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_create_local_static_samplers_layout(root_signature: *mut core::ffi::c_void, vk_set_layout: usize, vk_pipeline_layout: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_root_signature_create_work_graph_layout - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_create_work_graph_layout(root_signature: *mut core::ffi::c_void, vk_push_set_layout: *mut core::ffi::c_void, vk_pipeline_layout: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_create_pipeline_layout - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_pipeline_layout(device: *mut core::ffi::c_void, set_layout_count: u32, set_layouts: *mut core::ffi::c_void, push_constant_count: u32, push_constants: *mut core::ffi::c_void, pipeline_layout: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_create_descriptor_set_layout - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_descriptor_set_layout(device: *mut core::ffi::c_void, flags: usize, binding_count: u32, bindings: *mut core::ffi::c_void, descriptor_buffer_flags: usize, set_layout: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_vertex_input_pipeline_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_vertex_input_pipeline_create(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_fragment_output_pipeline_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_fragment_output_pipeline_create(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_state_create_shader_module - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_create_shader_module(device: *mut core::ffi::c_void, vk_module: *mut core::ffi::c_void, code: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_pipeline_state_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_create(device: *mut core::ffi::c_void, bind_point: usize, desc: *mut core::ffi::c_void, state: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_pipeline_state_get_or_create_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_get_or_create_pipeline(state: *mut core::ffi::c_void, dyn_state: *mut core::ffi::c_void, dsv_format: *mut core::ffi::c_void, dynamic_state_flags: *mut u32) -> usize {
    0
}

/// d3d12_pipeline_state_create_pipeline_variant - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_create_pipeline_variant(state: *mut core::ffi::c_void, key: *mut core::ffi::c_void, dsv_format: *mut core::ffi::c_void, vk_cache: usize, library_flags: usize, dynamic_state_flags: *mut u32) -> usize {
    0
}

/// d3d12_pipeline_library_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_create(device: *mut core::ffi::c_void, blob: *mut core::ffi::c_void, blob_length: usize, flags: u32, pipeline_library: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_create_pipeline_cache - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_pipeline_cache(device: *mut core::ffi::c_void, size: usize, data: *mut core::ffi::c_void, cache: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_create_pipeline_cache_from_d3d12_desc - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_pipeline_cache_from_d3d12_desc(device: *mut core::ffi::c_void, state: *mut core::ffi::c_void, cache: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_command_list_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_create(device: *mut core::ffi::c_void, node_mask: u32, arg2: usize, list: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_bundle_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_create(device: *mut core::ffi::c_void, node_mask: u32, arg2: usize, bundle: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_queue_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_create(device: *mut core::ffi::c_void, family_index: u32, queue_index: u32, properties: *mut core::ffi::c_void, queue: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_queue_get_signal_fence_proxy_locked - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_get_signal_fence_proxy_locked(queue: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_queue_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_create(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, vk_family_index: u32, queue: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_command_signature_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_create(device: *mut core::ffi::c_void, root_signature: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, signature: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_sampler_state_create_static_sampler - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_sampler_state_create_static_sampler(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, vk_sampler: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_queue_timeline_trace_register_present_block - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_present_block(trace: *mut core::ffi::c_void, present_id: u64) -> usize {
    0
}

/// vkd3d_queue_timeline_trace_complete_present_block - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_complete_present_block(trace: *mut core::ffi::c_void, cookie: usize) {

}

/// d3d12_device_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_create(instance: *mut core::ffi::c_void, create_info: *mut core::ffi::c_void, device: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_get_or_create_vertex_input_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_or_create_vertex_input_pipeline(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_get_or_create_fragment_output_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_or_create_fragment_output_pipeline(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d_blob_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d_blob_create(buffer: *mut core::ffi::c_void, size: usize, blob: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_rt_state_object_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_rt_state_object_create(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, parent: *mut core::ffi::c_void, object: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_wg_state_object_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_create(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, object: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_meta_command_create - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_create(device: *mut core::ffi::c_void, guid: usize, parameters: *mut core::ffi::c_void, parameter_size: usize, meta_command: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_compute_block_count - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compute_block_count(extent: usize, format: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_compute_block_offset - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compute_block_offset(offset: usize, format: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_compute_texel_count_from_blocks - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compute_texel_count_from_blocks(extent: usize, format: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_compute_texel_offset_from_blocks - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compute_texel_offset_from_blocks(offset: usize, format: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_view_map_create_view2 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_view_map_create_view2(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// cxg_fence_create - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_fence_create(fence: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// cxg_mesh_create - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_mesh_create(device: *mut core::ffi::c_void, inner_radius: f32, outer_radius: f32, width: f32, tooth_count: u32, tooth_depth: f32, mesh: *mut core::ffi::c_void) {

}

/// cxt_fence_create - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxt_fence_create(fence: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// test_copy_block_compressed_texture - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_block_compressed_texture() {

}

/// ID3D12Device_CreateCommandList - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device_CreateCommandList(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// test_create_descriptor_heap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_descriptor_heap() {

}

/// test_create_null_descriptors - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_null_descriptors() {

}

/// test_create_sampler - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_sampler() {

}

/// test_create_sampler2 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_sampler2() {

}

/// test_create_unordered_access_view - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_unordered_access_view() {

}

/// test_create_device - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_device() {

}

/// test_create_command_list - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_command_list() {

}

/// test_create_command_queue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_command_queue() {

}

/// test_create_command_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_command_signature() {

}

/// check_create_devices - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_create_devices(factory: *mut core::ffi::c_void, singleton_reference: *mut core::ffi::c_void) {

}

/// test_device_factory_create_device - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_device_factory_create_device() {

}

/// recreate_command_list_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn recreate_command_list_(line: u32, device: *mut core::ffi::c_void, allocator: *mut core::ffi::c_void, command_list: *mut *mut core::ffi::c_void) {

}

/// test_mesh_shader_create_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_mesh_shader_create_pipeline() {

}

/// test_create_compute_pipeline_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_compute_pipeline_state() {

}

/// test_create_graphics_pipeline_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_graphics_pipeline_state() {

}

/// test_create_pipeline_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_pipeline_state() {

}

/// test_create_pipeline_with_null_root_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_pipeline_with_null_root_signature() {

}

/// test_create_query_heap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_query_heap() {

}

/// create_acceleration_structure - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn create_acceleration_structure(context: *mut core::ffi::c_void, inputs: *mut core::ffi::c_void, rtas: *mut core::ffi::c_void, postbuild_va: usize) {

}

/// test_create_committed_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_committed_resource() {

}

/// test_create_heap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_heap() {

}

/// test_create_placed_resource_size - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_placed_resource_size() {

}

/// test_create_placed_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_placed_resource() {

}

/// test_create_reserved_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_reserved_resource() {

}

/// ID3D12Device_CreateRenderTargetView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device_CreateRenderTargetView(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// test_create_root_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_root_signature() {

}

/// ID3D12Device8_CreateSamplerFeedbackUnorderedAccessView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device8_CreateSamplerFeedbackUnorderedAccessView(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// ID3D12Device_CreateShaderResourceView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device_CreateShaderResourceView(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> usize {
    0
}

/// test_create_fence - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_fence() {

}

/// test_fence_ping_pong_deadlock_stress - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_ping_pong_deadlock_stress(use_shared: usize) {

}

/// test_fence_signal_order_deadlock_stress - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_signal_order_deadlock_stress(use_shared: usize) {

}

/// ID3D12Device_CreateFence - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device_CreateFence(arg0: usize, arg1: usize, D3D12_FENCE_FLAG_NONE: usize, arg3: usize, arg4: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// test_fence_ping_pong_deadlock_stress_plain - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_ping_pong_deadlock_stress_plain() {

}

/// test_fence_signal_order_deadlock_stress_plain - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_signal_order_deadlock_stress_plain() {

}

/// test_fence_ping_pong_deadlock_stress_shared - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_ping_pong_deadlock_stress_shared() {

}

/// test_fence_signal_order_deadlock_stress_shared - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_signal_order_deadlock_stress_shared() {

}

/// test_clock_calibration - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_clock_calibration() {

}

/// fake_vkCreateDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn fake_vkCreateDevice(physical_device: usize, create_info: *mut core::ffi::c_void, allocator: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> usize {
    0
}

/// vkCreateDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkCreateDevice(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// create_vulkan_image - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn create_vulkan_image(device: *mut core::ffi::c_void, width: u32, height: u32, vk_format: usize, usage: usize) -> usize {
    0
}

/// IVKD3DCoreInterface_CreateDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn IVKD3DCoreInterface_CreateDevice(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// IVKD3DCoreInterface_CreateRootSignatureDeserializer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn IVKD3DCoreInterface_CreateRootSignatureDeserializer(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// IVKD3DCoreInterface_CreateVersionedRootSignatureDeserializer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn IVKD3DCoreInterface_CreateVersionedRootSignatureDeserializer(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// d3d12core_CreateDeviceFromFactory - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12core_CreateDeviceFromFactory(adapter: *mut core::ffi::c_void, minimum_feature_level: usize, factory: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12core_CreateDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12core_CreateDevice(core: *mut core::ffi::c_void, adapter: *mut core::ffi::c_void, minimum_feature_level: usize, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_insert_hash_map_blob_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_insert_hash_map_blob_locked(pipeline_library: *mut core::ffi::c_void, map: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_create_binary_semaphore - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_binary_semaphore(device: *mut core::ffi::c_void, vk_semaphore: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_queue_add_wait_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_add_wait_locked(queue: *mut core::ffi::c_void, semaphore: usize, value: u64) {

}

/// vkd3d_queue_get_or_create_fence_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_get_or_create_fence_locked(queue: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_queue_wait_find_pending_submission_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_wait_find_pending_submission_locked(queue: *mut core::ffi::c_void, timeline: u64) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// vkd3d_queue_recycle_completed_fence_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_recycle_completed_fence_locked(queue: *mut core::ffi::c_void, vk_fence: usize) {

}

/// vkd3d_queue_complete_pending_submission_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_complete_pending_submission_locked(queue: *mut core::ffi::c_void, timeline: u64) {

}

/// vkd3d_queue_garbage_collect_obsolete_waits_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_garbage_collect_obsolete_waits_locked(queue: *mut core::ffi::c_void) {

}

/// vkd3d_queue_wait_submission_timeline_iterate_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_wait_submission_timeline_iterate_locked(queue: *mut core::ffi::c_void, value: u64, timeout: u64) -> i32 {
    0
}

/// d3d12_fence_signal_external_events_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_signal_external_events_locked(fence: *mut core::ffi::c_void, worker: *mut core::ffi::c_void) {

}

/// d3d12_fence_block_until_pending_value_reaches_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_block_until_pending_value_reaches_locked(fence: *mut core::ffi::c_void, pending_value: usize, ticket: usize, command_queue: *mut core::ffi::c_void, fence_value: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_fence_update_pending_value_locked_and_broadcast - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_update_pending_value_locked_and_broadcast(fence: *mut core::ffi::c_void) {

}

/// d3d12_fence_lock - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_lock(fence: *mut core::ffi::c_void) {

}

/// d3d12_fence_unlock - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_unlock(fence: *mut core::ffi::c_void) {

}

/// d3d12_fence_wait_until_signal_count_reaches_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_wait_until_signal_count_reaches_locked(fence: *mut core::ffi::c_void, update_count: u64) {

}

/// d3d12_fence_update_wait_tickets_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_update_wait_tickets_locked(fence: *mut core::ffi::c_void) {

}

/// d3d12_fence_add_pending_signal_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_add_pending_signal_locked(fence: *mut core::ffi::c_void, virtual_value: u64, signalling_queue: *mut core::ffi::c_void) -> u64 {
    0
}

/// d3d12_command_list_fetch_root_parameter_uniform_block_data - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_fetch_root_parameter_uniform_block_data(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void, dst_data: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_GetClockCalibration - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_GetClockCalibration(iface: *mut core::ffi::c_void, gpu_timestamp: *mut core::ffi::c_void, cpu_timestamp: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_queue_needs_cpu_waits_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_needs_cpu_waits_locked(command_queue: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_queue_gather_wait_semaphores_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_gather_wait_semaphores_locked(command_queue: *mut core::ffi::c_void, submit_info: *mut core::ffi::c_void, wait_flags: u32) {

}

/// d3d12_command_queue_submit_split_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_submit_split_locked(device: *mut core::ffi::c_void, vk_queue: usize, num_submits: u32, submits: *mut core::ffi::c_void, vk_fence: usize) -> i32 {
    0
}

/// d3d12_command_queue_needs_staggered_submissions_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_needs_staggered_submissions_locked(command_queue: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_create_vkd3d_queues - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_create_vkd3d_queues(device: *mut core::ffi::c_void, queue_info: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_create_vk_device - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_vk_device(device: *mut core::ffi::c_void, create_info: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_create_scratch_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_create_scratch_buffer(device: *mut core::ffi::c_void, kind: usize, size: u64, memory_types: u32, scratch: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_create_query_pool - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_create_query_pool(device: *mut core::ffi::c_void, type_index: u32, pool: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_CreateCommandQueue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateCommandQueue(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, riid: usize, command_queue: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateGraphicsPipelineState - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateGraphicsPipelineState(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, riid: usize, pipeline_state: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateComputePipelineState - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateComputePipelineState(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, riid: usize, pipeline_state: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateCommandList1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateCommandList1(iface: *mut core::ffi::c_void, node_mask: u32, arg2: usize, flags: usize, riid: usize, command_list: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateCommandList - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateCommandList(iface: *mut core::ffi::c_void, node_mask: u32, arg2: usize, command_allocator: *mut core::ffi::c_void, initial_pipeline_state: *mut core::ffi::c_void, riid: usize, command_list: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateDescriptorHeap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateDescriptorHeap(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, riid: usize, descriptor_heap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateRootSignature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateRootSignature(iface: *mut core::ffi::c_void, node_mask: u32, bytecode: *mut core::ffi::c_void, bytecode_length: usize, riid: usize, root_signature: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateShaderResourceView_embedded - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateShaderResourceView_embedded(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateShaderResourceView_default - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateShaderResourceView_default(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateUnorderedAccessView_embedded - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateUnorderedAccessView_embedded(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, counter_resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateUnorderedAccessView_default - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateUnorderedAccessView_default(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, counter_resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateRenderTargetView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateRenderTargetView(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateDepthStencilView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateDepthStencilView(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateSampler_embedded - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSampler_embedded(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateSampler_default - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSampler_default(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateSampler2_embedded - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSampler2_embedded(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateSampler2_default - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSampler2_default(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateCommittedResource1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateCommittedResource1(iface: *mut core::ffi::c_void, heap_properties: *mut core::ffi::c_void, heap_flags: usize, desc: *mut core::ffi::c_void, initial_state: usize, optimized_clear_value: *mut core::ffi::c_void, protected_session: *mut core::ffi::c_void, iid: usize, resource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateHeap1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateHeap1(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, protected_session: *mut core::ffi::c_void, iid: usize, heap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateHeap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateHeap(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, iid: usize, heap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreatePlacedResource1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreatePlacedResource1(iface: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, heap_offset: usize, resource_desc: *mut core::ffi::c_void, initial_state: usize, optimized_clear_value: *mut core::ffi::c_void, riid: usize, resource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreatePlacedResource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreatePlacedResource(iface: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, heap_offset: usize, desc: *mut core::ffi::c_void, initial_state: usize, optimized_clear_value: *mut core::ffi::c_void, iid: usize, resource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateReservedResource1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateReservedResource1(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, initial_state: usize, optimized_clear_value: *mut core::ffi::c_void, protected_session: *mut core::ffi::c_void, iid: usize, resource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateSharedHandle - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSharedHandle(iface: *mut core::ffi::c_void, object: *mut core::ffi::c_void, attributes: *mut core::ffi::c_void, access: u32, name: *mut u16, handle: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateFence - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateFence(iface: *mut core::ffi::c_void, initial_value: usize, flags: usize, riid: usize, fence: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateQueryHeap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateQueryHeap(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, iid: usize, heap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreatePipelineLibrary - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreatePipelineLibrary(iface: *mut core::ffi::c_void, blob: *mut core::ffi::c_void, blob_size: usize, iid: usize, lib: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreatePipelineState - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreatePipelineState(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, riid: usize, pipeline_state: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateProtectedResourceSession - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateProtectedResourceSession(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, iid: usize, session: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateCommittedResource2 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateCommittedResource2(iface: *mut core::ffi::c_void, heap_properties: *mut core::ffi::c_void, heap_flags: usize, desc: *mut core::ffi::c_void, initial_state: usize, optimized_clear_value: *mut core::ffi::c_void, protected_session: *mut core::ffi::c_void, iid: usize, resource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateMetaCommand - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateMetaCommand(iface: *mut core::ffi::c_void, command_id: usize, node_mask: u32, param_data: *mut core::ffi::c_void, param_size: usize, iid: usize, meta_command: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateStateObject - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateStateObject(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, iid: usize, state_object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_create_sampler_feedback_desc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_create_sampler_feedback_desc(uav_desc: *mut core::ffi::c_void, feedback: *mut core::ffi::c_void) {

}

/// d3d12_device_CreateSamplerFeedbackUnorderedAccessView_default - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSamplerFeedbackUnorderedAccessView_default(iface: *mut core::ffi::c_void, target_resource: *mut core::ffi::c_void, feedback_resource: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateSamplerFeedbackUnorderedAccessView_embedded - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateSamplerFeedbackUnorderedAccessView_embedded(iface: *mut core::ffi::c_void, target_resource: *mut core::ffi::c_void, feedback_resource: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateShaderCacheSession - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateShaderCacheSession(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, iid: usize, session: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateCommittedResource3 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateCommittedResource3(iface: *mut core::ffi::c_void, heap_properties: *mut core::ffi::c_void, heap_flags: usize, desc: *mut core::ffi::c_void, initial_layout: usize, optimized_clear_value: *mut core::ffi::c_void, protected_session: *mut core::ffi::c_void, num_castable_formats: usize, castable_formats: *mut core::ffi::c_void, iid: usize, resource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreatePlacedResource2 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreatePlacedResource2(iface: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, heap_offset: usize, desc: *mut core::ffi::c_void, initial_layout: usize, optimized_clear_value: *mut core::ffi::c_void, num_castable_formats: usize, castable_formats: *mut core::ffi::c_void, iid: usize, resource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CreateReservedResource2 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateReservedResource2(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, initial_layout: usize, optimized_clear_value: *mut core::ffi::c_void, protected_session: *mut core::ffi::c_void, num_castable_formats: usize, castable_formats: *mut core::ffi::c_void, iid: usize, resource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_create_sparse_init_timeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_create_sparse_init_timeline(device: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_vkd3d_ext_create_cubin_compute_shader - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_create_cubin_compute_shader(device: *mut core::ffi::c_void, cubin_data: *mut core::ffi::c_void, cubin_size: usize, block_x: usize, block_y: usize, block_z: usize, shader_name: *mut i8, use_64bit_texturing: usize, flags: usize, out_handle: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_vkd3d_ext_CreateResourceFromBorrowedHandle - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_CreateResourceFromBorrowedHandle(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, vk_handle: usize, ppResource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dxvk_interop_device_LockCommandQueue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_LockCommandQueue(iface: *mut core::ffi::c_void, queue: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dxvk_interop_device_UnlockCommandQueue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_UnlockCommandQueue(iface: *mut core::ffi::c_void, queue: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dxvk_interop_device_CreateInteropCommandQueue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_CreateInteropCommandQueue(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, vk_family_index: u32, command_queue: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dxvk_interop_device_LockVulkanQueue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_LockVulkanQueue(iface: *mut core::ffi::c_void, queue: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dxvk_interop_device_UnlockVulkanQueue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_UnlockVulkanQueue(iface: *mut core::ffi::c_void, queue: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_memory_transfer_queue_track_resource_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_transfer_queue_track_resource_locked(queue: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, semaphore_value: usize) {

}

/// vkd3d_memory_transfer_queue_flush_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_transfer_queue_flush_locked(queue: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_create_global_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_global_buffer(device: *mut core::ffi::c_void, size: u64, heap_properties: *mut core::ffi::c_void, heap_flags: usize, vk_buffer: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_memory_chunk_create - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_chunk_create(device: *mut core::ffi::c_void, allocator: *mut core::ffi::c_void, info: *mut core::ffi::c_void, chunk: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_create_shader_module - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_shader_module(device: *mut core::ffi::c_void, code: *mut u32, code_size: usize, module: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_create_descriptor_set_layout - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_descriptor_set_layout(device: *mut core::ffi::c_void, binding_count: u32, bindings: *mut core::ffi::c_void, descriptor_buffer_compatible: usize, set_layout: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_create_sampler - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_sampler(device: *mut core::ffi::c_void, filter: usize, vk_sampler: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_create_pipeline_layout - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_pipeline_layout(device: *mut core::ffi::c_void, set_layout_count: u32, set_layouts: *mut core::ffi::c_void, push_constant_range_count: u32, push_constant_ranges: *mut core::ffi::c_void, pipeline_layout: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_create_compute_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_compute_pipeline(device: *mut core::ffi::c_void, code_size: usize, code: *mut u32, layout: usize, specialization_info: *mut core::ffi::c_void, descriptor_buffer_compatible: usize, required_size: *mut core::ffi::c_void, pipeline: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_create_graphics_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_graphics_pipeline(meta_ops: *mut core::ffi::c_void, layout: usize, color_format: usize, ds_format: usize, vk_aspect_mask: usize, vs_module: usize, fs_module: usize, samples: usize, ds_state: *mut core::ffi::c_void, dynamic_state_count: u32, dynamic_states: *mut core::ffi::c_void, spec_info: *mut core::ffi::c_void, descriptor_buffer_compatible: usize, vk_pipeline: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_create_swapchain_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_swapchain_pipeline(meta_ops: *mut core::ffi::c_void, key: *mut core::ffi::c_void, pipeline: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_create_copy_image_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_copy_image_pipeline(meta_ops: *mut core::ffi::c_void, key: *mut core::ffi::c_void, pipeline: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_create_resolve_image_graphics_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_resolve_image_graphics_pipeline(meta_ops: *mut core::ffi::c_void, key: *mut core::ffi::c_void, pipeline: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_create_resolve_image_compute_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_create_resolve_image_compute_pipeline(meta_ops: *mut core::ffi::c_void, key: *mut core::ffi::c_void, pipeline: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_meta_command_create_dstorage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_create_dstorage(meta_command: *mut core::ffi::c_void, device: *mut core::ffi::c_void, parameter_data: *mut core::ffi::c_void, parameter_size: usize) -> i32 {
    0
}

/// vkd3d_queue_timeline_trace_complete_blocking - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_complete_blocking(trace: *mut core::ffi::c_void, cookie: usize, pid: *mut i8) {

}

/// d3d12_state_object_build_group_create_info - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_build_group_create_info(group_create: *mut core::ffi::c_void, group_type: usize, export: *mut core::ffi::c_void) {

}

/// vkd3d_create_image - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_image(device: *mut core::ffi::c_void, heap_properties: *mut core::ffi::c_void, heap_flags: usize, desc: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, num_castable_formats: u32, castable_formats: *mut core::ffi::c_void, vk_image: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_create_sampler - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_create_sampler(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, vk_sampler: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_sampler_state_create_descriptor_pool - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_sampler_state_create_descriptor_pool(device: *mut core::ffi::c_void, vk_pool: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_resource_validate_create_info - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_validate_create_info(desc: *mut core::ffi::c_void, heap_properties: *mut core::ffi::c_void, initial_state: usize, optimized_clear_value: *mut core::ffi::c_void, num_castable_formats: u32, castable_formats: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_resource_create_vk_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_create_vk_resource(resource: *mut core::ffi::c_void, num_castable_formats: u32, castable_formats: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_resource_create_reserved_fallback - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_create_reserved_fallback(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, initial_state: usize, optimized_clear_value: *mut core::ffi::c_void, num_castable_formats: u32, castable_formats: *mut core::ffi::c_void, resource: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_create_buffer_view_for_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer_view_for_resource(device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, view_format: usize, offset: u64, size: u64, structure_stride: u64, flags: u32, view: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_create_buffer_srv_embedded - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer_srv_embedded(desc_va: usize, device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// vkd3d_create_buffer_srv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer_srv(desc_va: usize, device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// vkd3d_view_map_create_view - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_view_map_create_view(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// vkd3d_create_texture_srv_embedded - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_texture_srv_embedded(desc_va: usize, device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// vkd3d_create_texture_srv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_texture_srv(desc_va: usize, device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// vkd3d_create_buffer_uav_embedded - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer_uav_embedded(desc_va: usize, device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, counter_resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// vkd3d_create_buffer_uav - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer_uav(desc_va: usize, device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, counter_resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// vkd3d_create_texture_uav_embedded - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_texture_uav_embedded(desc_va: usize, device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// vkd3d_create_texture_uav - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_texture_uav(desc_va: usize, device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// d3d12_descriptor_heap_create_descriptor_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_create_descriptor_buffer(descriptor_heap: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_descriptor_heap_create_descriptor_pool - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_create_descriptor_pool(descriptor_heap: *mut core::ffi::c_void, vk_descriptor_pool: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_descriptor_heap_create_descriptor_set - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_create_descriptor_set(descriptor_heap: *mut core::ffi::c_void, binding: *mut core::ffi::c_void, vk_descriptor_set: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_create_pipeline_layout_for_stage_mask - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_pipeline_layout_for_stage_mask(device: *mut core::ffi::c_void, set_layout_count: u32, set_layouts: *mut core::ffi::c_void, push_constants: *mut core::ffi::c_void, stages: usize, bind_point_layout: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_root_signature_create_from_blob - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_create_from_blob(device: *mut core::ffi::c_void, bytecode: *mut core::ffi::c_void, bytecode_length: usize, raw_payload: usize, root_signature: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_stages_require_work_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_stages_require_work_locked(state: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_create_compute_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_compute_pipeline(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void, code: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_pipeline_state_graphics_create_shader_stages - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_graphics_create_shader_stages(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_pipeline_state_init_graphics_create_info - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_init_graphics_create_info(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_pipeline_create_private_root_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_create_private_root_signature(device: *mut core::ffi::c_void, bind_point: usize, desc: *mut core::ffi::c_void, root_signature: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// dxgi_vk_swap_chain_update_formats_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_update_formats_locked(chain: *mut core::ffi::c_void, force_requery: usize) -> usize {
    0
}

/// dxgi_vk_swap_chain_create_surface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_create_surface(chain: *mut core::ffi::c_void, pFactory: *mut core::ffi::c_void) -> i32 {
    0
}

/// dxgi_vk_swap_chain_find_compatible_unlocked_present_mode - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_find_compatible_unlocked_present_mode(chain: *mut core::ffi::c_void, vk_present_mode: *mut core::ffi::c_void, vk_min_image_count: *mut u32) -> usize {
    0
}

/// dxgi_vk_swap_chain_recreate_swapchain_in_present_task - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_recreate_swapchain_in_present_task(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_factory_CreateSwapChain - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_factory_CreateSwapChain(iface: *mut core::ffi::c_void, pFactory: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, ppSwapchain: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_timestamp_profiler_wait_available_submit_locked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_wait_available_submit_locked(profiler: *mut core::ffi::c_void, timeline: u64, num_timestamps: usize) {

}

/// vkd3d_timestamp_profiler_create_query_pool - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_create_query_pool(device: *mut core::ffi::c_void, arg1: usize, count: u32, pipeline_statistics: usize) -> usize {
    0
}

/// vkd3d_va_map_get_block_address - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_va_map_get_block_address(va: usize) -> usize {
    0
}

/// CreateSharedResource - from DirectX-Headers/d3d12compatibility.h
#[no_mangle]
pub unsafe extern "C" fn CreateSharedResource(pHeapProperties: *mut core::ffi::c_void, HeapFlags: usize, pDesc: *mut core::ffi::c_void, InitialResourceState: usize, pOptimizedClearValue: *mut core::ffi::c_void, pFlags11: *mut core::ffi::c_void, CompatibilityFlags: usize, pLifetimeTracker: *mut core::ffi::c_void, pOwningSwapchain: *mut core::ffi::c_void, riid: usize, ppResource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateSharedHeap - from DirectX-Headers/d3d12compatibility.h
#[no_mangle]
pub unsafe extern "C" fn CreateSharedHeap(pHeapDesc: *mut core::ffi::c_void, CompatibilityFlags: usize, riid: usize, ppHeap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// D3D12CompilerCreateFactory - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn D3D12CompilerCreateFactory(pPluginCompilerDllPath: usize, riid: usize, ppFactory: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// CreateCompiler - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn CreateCompiler(pCompilerCacheSession: *mut core::ffi::c_void, riid: usize, ppCompiler: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVideoDecoderHeap - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn CreateVideoDecoderHeap(pVideoDecoderHeapDesc: *mut core::ffi::c_void, riid: usize, ppVideoDecoderHeap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVideoMotionEstimator - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn CreateVideoMotionEstimator(pDesc: *mut core::ffi::c_void, pProtectedResourceSession: *mut core::ffi::c_void, riid: usize, ppVideoMotionEstimator: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVideoMotionVectorHeap - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn CreateVideoMotionVectorHeap(pDesc: *mut core::ffi::c_void, pProtectedResourceSession: *mut core::ffi::c_void, riid: usize, ppVideoMotionVectorHeap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVideoDecoder1 - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn CreateVideoDecoder1(pDesc: *mut core::ffi::c_void, pProtectedResourceSession: *mut core::ffi::c_void, riid: usize, ppVideoDecoder: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVideoDecoderHeap1 - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn CreateVideoDecoderHeap1(pVideoDecoderHeapDesc: *mut core::ffi::c_void, pProtectedResourceSession: *mut core::ffi::c_void, riid: usize, ppVideoDecoderHeap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVideoEncoder - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn CreateVideoEncoder(pDesc: *mut core::ffi::c_void, riid: usize, ppVideoEncoder: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVideoEncoderHeap - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn CreateVideoEncoderHeap(pDesc: *mut core::ffi::c_void, riid: usize, ppVideoEncoderHeap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateVideoEncoderHeap1 - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn CreateVideoEncoderHeap1(pDesc: *mut core::ffi::c_void, riid: usize, ppVideoEncoderHeap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateInstaller - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn CreateInstaller(pClient: *mut core::ffi::c_void, riid: usize, ppvInstaller: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// UnalignedBlockTexturesSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn UnalignedBlockTexturesSupported() -> i32 {
    0
}

/// CreateByteOffsetViewsSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn CreateByteOffsetViewsSupported() -> i32 {
    0
}

/// IsBlockCompressFormat - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn IsBlockCompressFormat(Format: usize) -> usize {
    0
}

/// CreateSubobject - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn CreateSubobject() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SetFrontCounterClockwise - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetFrontCounterClockwise(frontCounterClockwise: i32) {

}

/// CreateNode - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn CreateNode() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateShaderNode - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn CreateShaderNode(nullptr: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateBroadcastingLaunchNodeOverrides - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn CreateBroadcastingLaunchNodeOverrides(nullptr: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateCoalescingLaunchNodeOverrides - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn CreateCoalescingLaunchNodeOverrides(nullptr: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateCommonComputeNodeOverrides - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn CreateCommonComputeNodeOverrides(nullptr: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// DMLCreateDevice - from DirectX-Headers/DirectML.h
#[no_mangle]
pub unsafe extern "C" fn DMLCreateDevice(d3d12Device: *mut core::ffi::c_void, flags: usize, riid: usize, ppv: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// DMLCreateDevice1 - from DirectX-Headers/DirectML.h
#[no_mangle]
pub unsafe extern "C" fn DMLCreateDevice1(d3d12Device: *mut core::ffi::c_void, flags: usize, minimumFeatureLevel: usize, riid: usize, ppv: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateAdapterListByWorkload - from DirectX-Headers/dxcore_interface.h
#[no_mangle]
pub unsafe extern "C" fn CreateAdapterListByWorkload(workload: usize, runtimeFilter: usize, hardwareTypeFilter: usize, riid: usize, ppvAdapterList: *mut *mut core::ffi::c_void) -> usize {
    0
}

