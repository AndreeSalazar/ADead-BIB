//! ADead Runtime - OPENGL Module
//!
//! Funciones generadas automáticamente desde knowledge.json
//! Categoría: opengl

#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

/// D3DPERF_BeginEvent - from wine/d3d9.h
#[no_mangle]
pub unsafe extern "C" fn D3DPERF_BeginEvent(color: usize, name: *mut u16) -> i32 {
    0
}

/// D3DPERF_EndEvent - from wine/d3d9.h
#[no_mangle]
pub unsafe extern "C" fn D3DPERF_EndEvent() -> i32 {
    0
}

/// D3DPERF_GetStatus - from wine/d3d9.h
#[no_mangle]
pub unsafe extern "C" fn D3DPERF_GetStatus() -> u32 {
    0
}

/// D3DPERF_QueryRepeatFrame - from wine/d3d9.h
#[no_mangle]
pub unsafe extern "C" fn D3DPERF_QueryRepeatFrame() -> i32 {
    0
}

/// D3DPERF_SetMarker - from wine/d3d9.h
#[no_mangle]
pub unsafe extern "C" fn D3DPERF_SetMarker(color: usize, name: *mut u16) {

}

/// D3DPERF_SetOptions - from wine/d3d9.h
#[no_mangle]
pub unsafe extern "C" fn D3DPERF_SetOptions(options: u32) {

}

/// D3DPERF_SetRegion - from wine/d3d9.h
#[no_mangle]
pub unsafe extern "C" fn D3DPERF_SetRegion(color: usize, name: *mut u16) {

}

/// Direct3DCreate9 - from wine/d3d9.h
#[no_mangle]
pub unsafe extern "C" fn Direct3DCreate9(sdk_version: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// Direct3DCreate9Ex - from wine/d3d9.h
#[no_mangle]
pub unsafe extern "C" fn Direct3DCreate9Ex(sdk_version: u32, d3d9ex: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3D9Test - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn D3D9Test(lpDevice: *mut core::ffi::c_void, hWnd: *mut core::ffi::c_void) -> i32 {
    0
}

/// GetD3D11Iface - from dxvk/d3d10_blend.h
#[no_mangle]
pub unsafe extern "C" fn GetD3D11Iface() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// ConvertD3D11ResourceFlags - from dxvk/d3d10_util.h
#[no_mangle]
pub unsafe extern "C" fn ConvertD3D11ResourceFlags(MiscFlags: u32) -> u32 {
    0
}

/// GetD3D11ResourceFromView - from dxvk/d3d10_util.h
#[no_mangle]
pub unsafe extern "C" fn GetD3D11ResourceFromView(pSrcView: *mut core::ffi::c_void, ppDstResource: *mut *mut core::ffi::c_void) {

}

/// GetD3D11Resource - from dxvk/d3d10_util.h
#[no_mangle]
pub unsafe extern "C" fn GetD3D11Resource(pSrcResource: *mut core::ffi::c_void, ppDstResource: *mut *mut core::ffi::c_void) {

}

/// GetD3D11Device - from dxvk/d3d10_util.h
#[no_mangle]
pub unsafe extern "C" fn GetD3D11Device(pObject: *mut core::ffi::c_void, ppDevice: *mut *mut core::ffi::c_void) {

}

/// GetD3D11Context - from dxvk/d3d10_util.h
#[no_mangle]
pub unsafe extern "C" fn GetD3D11Context(pObject: *mut core::ffi::c_void, ppContext: *mut *mut core::ffi::c_void) {

}

/// isD3D9ExclusiveFormat - from dxvk/d3d8_format.h
#[no_mangle]
pub unsafe extern "C" fn isD3D9ExclusiveFormat(fmt: usize) -> usize {
    0
}

/// SetD3D9 - from dxvk/d3d8_state_block.h
#[no_mangle]
pub unsafe extern "C" fn SetD3D9(pStateBlock: usize) {

}

/// GetD3D9 - from dxvk/d3d8_wrapped_object.h
#[no_mangle]
pub unsafe extern "C" fn GetD3D9() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// GetD3D9Nullable - from dxvk/d3d8_wrapped_object.h
#[no_mangle]
pub unsafe extern "C" fn GetD3D9Nullable(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// D3D9FFShaderModuleSet - from dxvk/d3d9_fixed_function.h
#[no_mangle]
pub unsafe extern "C" fn D3D9FFShaderModuleSet(pDevice: *mut core::ffi::c_void) -> usize {
    0
}

/// D3D9TextureStageStateTypes - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn D3D9TextureStageStateTypes(arg0: usize) -> usize {
    0
}

/// MapD3D9VertexElementsToFvf - from dxvk/d3d9_vertex_declaration.h
#[no_mangle]
pub unsafe extern "C" fn MapD3D9VertexElementsToFvf() -> u32 {
    0
}

