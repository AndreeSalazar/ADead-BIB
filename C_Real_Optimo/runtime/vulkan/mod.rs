//! ADead Runtime - VULKAN Module
//!
//! Funciones generadas automáticamente desde knowledge.json
//! Categoría: vulkan

#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

/// TYPEDEF_VK_TO_WCHARS - from wine/kbd.h
#[no_mangle]
pub unsafe extern "C" fn TYPEDEF_VK_TO_WCHARS(arg0: usize) -> usize {
    0
}

/// NtUserVkKeyScanEx - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserVkKeyScanEx(chr: u16, layout: usize) -> usize {
    0
}

/// CRYPT_GetProvKeyName - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn CRYPT_GetProvKeyName(pProvName: usize) -> usize {
    0
}

/// IsVKDBEKey - from reactos/misc.h
#[no_mangle]
pub unsafe extern "C" fn IsVKDBEKey(uVirtKey: usize) -> i32 {
    0
}

/// vk2ascii - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn vk2ascii(vk: u32) -> i32 {
    0
}

/// process_vk_key - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn process_vk_key(wParam: usize, lParam: isize) -> i32 {
    0
}

/// __uuidof_helper - from dxvk/windows_base.h
#[no_mangle]
pub unsafe extern "C" fn __uuidof_helper() -> usize {
    0
}

/// fromHwnd - from dxvk/native_glfw.h
#[no_mangle]
pub unsafe extern "C" fn fromHwnd(hWindow: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// toHwnd - from dxvk/native_glfw.h
#[no_mangle]
pub unsafe extern "C" fn toHwnd(pWindow: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// fromHmonitor - from dxvk/native_glfw.h
#[no_mangle]
pub unsafe extern "C" fn fromHmonitor(hMonitor: usize) -> i32 {
    0
}

/// toHmonitor - from dxvk/native_glfw.h
#[no_mangle]
pub unsafe extern "C" fn toHmonitor(displayId: i32) -> usize {
    0
}

/// SDL_DisplayID - from dxvk/native_sdl3.h
#[no_mangle]
pub unsafe extern "C" fn SDL_DisplayID(arg0: usize) -> usize {
    0
}

/// SetEvictionPriority - from dxvk/d3d10_buffer.h
#[no_mangle]
pub unsafe extern "C" fn SetEvictionPriority(EvictionPriority: u32) -> usize {
    0
}

/// GetEvictionPriority - from dxvk/d3d10_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetEvictionPriority() -> usize {
    0
}

/// SetExceptionMode - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn SetExceptionMode(RaiseFlags: u32) -> usize {
    0
}

/// GetExceptionMode - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn GetExceptionMode() -> usize {
    0
}

/// GetFeatureLevel - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn GetFeatureLevel() -> usize {
    0
}

/// CheckFormatSupport - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CheckFormatSupport(Format: usize, pFormatSupport: *mut u32) -> usize {
    0
}

/// CheckMultisampleQualityLevels - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CheckMultisampleQualityLevels(Format: usize, SampleCount: u32, pNumQualityLevels: *mut u32) -> usize {
    0
}

/// CheckCounterInfo - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CheckCounterInfo(pCounterInfo: *mut core::ffi::c_void) -> usize {
    0
}

/// CheckCounter - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CheckCounter(pDesc: *mut core::ffi::c_void, pType: *mut core::ffi::c_void, pActiveCounters: *mut u32, name: *mut i8, pNameLength: *mut u32, units: *mut i8, pUnitsLength: *mut u32, description: *mut i8, pDescriptionLength: *mut u32) -> usize {
    0
}

/// OpenSharedResource - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn OpenSharedResource(hResource: *mut core::ffi::c_void, ReturnedInterface: usize, ppResource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetPredication - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn GetPredication(ppPredicate: *mut *mut core::ffi::c_void, pPredicateValue: *mut i32) -> usize {
    0
}

/// CopySubresourceRegion - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn CopySubresourceRegion(pDstResource: *mut core::ffi::c_void, DstSubresource: u32, DstX: u32, DstY: u32, DstZ: u32, pSrcResource: *mut core::ffi::c_void, SrcSubresource: u32, pSrcBox: *mut core::ffi::c_void) -> usize {
    0
}

/// UpdateSubresource - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateSubresource(pDstResource: *mut core::ffi::c_void, DstSubresource: u32, pDstBox: *mut core::ffi::c_void, pSrcData: *mut core::ffi::c_void, SrcRowPitch: u32, SrcDepthPitch: u32) -> usize {
    0
}

/// GenerateMips - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn GenerateMips(pShaderResourceView: *mut core::ffi::c_void) -> usize {
    0
}

/// DrawIndexed - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn DrawIndexed(IndexCount: u32, StartIndexLocation: u32, BaseVertexLocation: i32) -> usize {
    0
}

/// DrawAuto - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn DrawAuto() -> usize {
    0
}

/// IASetInputLayout - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn IASetInputLayout(pInputLayout: *mut core::ffi::c_void) -> usize {
    0
}

/// IAGetInputLayout - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn IAGetInputLayout(ppInputLayout: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// IAGetVertexBuffers - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn IAGetVertexBuffers(StartSlot: u32, NumBuffers: u32, ppVertexBuffers: *mut *mut core::ffi::c_void, pStrides: *mut u32, pOffsets: *mut u32) -> usize {
    0
}

/// IAGetIndexBuffer - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn IAGetIndexBuffer(pIndexBuffer: *mut *mut core::ffi::c_void, Format: *mut core::ffi::c_void, Offset: *mut u32) -> usize {
    0
}

/// VSSetShader - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn VSSetShader(pVertexShader: *mut core::ffi::c_void) -> usize {
    0
}

/// VSSetShaderResources - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn VSSetShaderResources(StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// VSSetSamplers - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn VSSetSamplers(StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// VSGetShader - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn VSGetShader(ppVertexShader: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// VSGetShaderResources - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn VSGetShaderResources(StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// VSGetSamplers - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn VSGetSamplers(StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GSSetShader - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn GSSetShader(pShader: *mut core::ffi::c_void) -> usize {
    0
}

/// GSSetShaderResources - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn GSSetShaderResources(StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GSSetSamplers - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn GSSetSamplers(StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GSGetShader - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn GSGetShader(ppGeometryShader: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GSGetShaderResources - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn GSGetShaderResources(StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GSGetSamplers - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn GSGetSamplers(StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// PSSetShader - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn PSSetShader(pPixelShader: *mut core::ffi::c_void) -> usize {
    0
}

/// PSSetShaderResources - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn PSSetShaderResources(StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// PSSetSamplers - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn PSSetSamplers(StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// PSGetShader - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn PSGetShader(ppPixelShader: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// PSGetShaderResources - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn PSGetShaderResources(StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// PSGetSamplers - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn PSGetSamplers(StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OMSetBlendState - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn OMSetBlendState(pBlendState: *mut core::ffi::c_void, BlendFactor4: f32, SampleMask: u32) -> usize {
    0
}

/// OMSetDepthStencilState - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn OMSetDepthStencilState(pDepthStencilState: *mut core::ffi::c_void, StencilRef: u32) -> usize {
    0
}

/// OMGetRenderTargets - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn OMGetRenderTargets(NumViews: u32, ppRenderTargetViews: *mut *mut core::ffi::c_void, ppDepthStencilView: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OMGetBlendState - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn OMGetBlendState(ppBlendState: *mut *mut core::ffi::c_void, BlendFactor4: f32, pSampleMask: *mut u32) -> usize {
    0
}

/// OMGetDepthStencilState - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn OMGetDepthStencilState(ppDepthStencilState: *mut *mut core::ffi::c_void, pStencilRef: *mut u32) -> usize {
    0
}

/// RSSetState - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn RSSetState(pRasterizerState: *mut core::ffi::c_void) -> usize {
    0
}

/// RSGetState - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn RSGetState(ppRasterizerState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// RSGetViewports - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn RSGetViewports(NumViewports: *mut u32, pViewports: *mut core::ffi::c_void) -> usize {
    0
}

/// RSGetScissorRects - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn RSGetScissorRects(NumRects: *mut u32, pRects: *mut core::ffi::c_void) -> usize {
    0
}

/// SOGetTargets - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn SOGetTargets(NumBuffers: u32, ppSOTargets: *mut *mut core::ffi::c_void, pOffsets: *mut u32) -> usize {
    0
}

/// SetTextFilterSize - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn SetTextFilterSize(Width: u32, Height: u32) -> usize {
    0
}

/// GetTextFilterSize - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn GetTextFilterSize(pWidth: *mut u32, pHeight: *mut u32) -> usize {
    0
}

/// unlikely - from dxvk/d3d10_multithread.h
#[no_mangle]
pub unsafe extern "C" fn unlikely(arg0: usize) -> usize {
    0
}

/// Begin - from dxvk/d3d10_query.h
#[no_mangle]
pub unsafe extern "C" fn Begin() -> usize {
    0
}

/// End - from dxvk/d3d10_query.h
#[no_mangle]
pub unsafe extern "C" fn End() -> usize {
    0
}

/// GetDataSize - from dxvk/d3d10_query.h
#[no_mangle]
pub unsafe extern "C" fn GetDataSize() -> usize {
    0
}

/// ConvertD3D10ResourceFlags - from dxvk/d3d10_util.h
#[no_mangle]
pub unsafe extern "C" fn ConvertD3D10ResourceFlags(MiscFlags: u32) -> u32 {
    0
}

/// GetD3D10ResourceFromView - from dxvk/d3d10_util.h
#[no_mangle]
pub unsafe extern "C" fn GetD3D10ResourceFromView(pSrcView: *mut core::ffi::c_void, ppDstResource: *mut *mut core::ffi::c_void) {

}

/// GetD3D10Resource - from dxvk/d3d10_util.h
#[no_mangle]
pub unsafe extern "C" fn GetD3D10Resource(pSrcResource: *mut core::ffi::c_void, ppDstResource: *mut *mut core::ffi::c_void) {

}

/// GetD3D10Device - from dxvk/d3d10_util.h
#[no_mangle]
pub unsafe extern "C" fn GetD3D10Device(pObject: *mut core::ffi::c_void, ppDevice: *mut *mut core::ffi::c_void) {

}

/// GetResource - from dxvk/d3d10_view_dsv.h
#[no_mangle]
pub unsafe extern "C" fn GetResource(ppResource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetMsState - from dxvk/d3d11_blend.h
#[no_mangle]
pub unsafe extern "C" fn GetMsState(SampleMask: u32) -> usize {
    0
}

/// GetLoState - from dxvk/d3d11_blend.h
#[no_mangle]
pub unsafe extern "C" fn GetLoState() -> usize {
    0
}

/// GetD3D10Iface - from dxvk/d3d11_blend.h
#[no_mangle]
pub unsafe extern "C" fn GetD3D10Iface() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PromoteDesc - from dxvk/d3d11_blend.h
#[no_mangle]
pub unsafe extern "C" fn PromoteDesc(pSrcDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// NormalizeDesc - from dxvk/d3d11_blend.h
#[no_mangle]
pub unsafe extern "C" fn NormalizeDesc(pDesc: *mut core::ffi::c_void) -> i32 {
    0
}

/// DecodeBlendMode - from dxvk/d3d11_blend.h
#[no_mangle]
pub unsafe extern "C" fn DecodeBlendMode(BlendDesc: usize) -> usize {
    0
}

/// DecodeBlendFactor - from dxvk/d3d11_blend.h
#[no_mangle]
pub unsafe extern "C" fn DecodeBlendFactor(BlendFactor: usize, IsAlpha: usize) -> usize {
    0
}

/// DecodeBlendOp - from dxvk/d3d11_blend.h
#[no_mangle]
pub unsafe extern "C" fn DecodeBlendOp(BlendOp: usize) -> usize {
    0
}

/// ValidateBlendFactor - from dxvk/d3d11_blend.h
#[no_mangle]
pub unsafe extern "C" fn ValidateBlendFactor(Blend: usize) -> usize {
    0
}

/// ValidateBlendFactorAlpha - from dxvk/d3d11_blend.h
#[no_mangle]
pub unsafe extern "C" fn ValidateBlendFactorAlpha(BlendAlpha: usize) -> usize {
    0
}

/// ValidateBlendOp - from dxvk/d3d11_blend.h
#[no_mangle]
pub unsafe extern "C" fn ValidateBlendOp(BlendOp: usize) -> usize {
    0
}

/// ValidateBlendOperations - from dxvk/d3d11_blend.h
#[no_mangle]
pub unsafe extern "C" fn ValidateBlendOperations(SrcBlend: usize, SrcBlendAlpha: usize, SestBlend: usize, DestBlendAlpha: usize, BlendOp: usize, BlendOpAlpha: usize) -> usize {
    0
}

/// SetDebugName - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn SetDebugName(pName: *mut i8) -> usize {
    0
}

/// CheckViewCompatibility - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn CheckViewCompatibility(BindFlags: u32, Format: usize) -> usize {
    0
}

/// Desc - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn Desc() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// IsTilePool - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn IsTilePool() -> i32 {
    0
}

/// GetCookie - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetCookie() -> u64 {
    0
}

/// GetBufferSlice - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetBufferSlice() -> usize {
    0
}

/// DxvkBufferSlice - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn DxvkBufferSlice(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// GetRemainingSize - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetRemainingSize(offset: u64) -> u64 {
    0
}

/// GetSOCounter - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetSOCounter() -> usize {
    0
}

/// GetMapPtr - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetMapPtr() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// TrackSequenceNumber - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn TrackSequenceNumber(Seq: u64) {

}

/// GetSequenceNumber - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetSequenceNumber() -> u64 {
    0
}

/// HasSequenceNumber - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn HasSequenceNumber() -> usize {
    0
}

/// Get11on12Info - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn Get11on12Info() -> usize {
    0
}

/// NormalizeBufferProperties - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn NormalizeBufferProperties(pDesc: *mut core::ffi::c_void) -> i32 {
    0
}

/// CheckFormatFeatureSupport - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn CheckFormatFeatureSupport(Format: usize, Features: usize) -> i32 {
    0
}

/// GetMemoryFlags - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetMemoryFlags() -> usize {
    0
}

/// DetermineMapMode - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn DetermineMapMode(MemFlags: usize) -> usize {
    0
}

/// GetCommonBuffer - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetCommonBuffer(pResource: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// AddType - from dxvk/d3d11_class_linkage.h
#[no_mangle]
pub unsafe extern "C" fn AddType(TypeId: u32, pTypeName: *mut i8) {

}

/// AddSlotInfo - from dxvk/d3d11_class_linkage.h
#[no_mangle]
pub unsafe extern "C" fn AddSlotInfo(FirstSlot: u32, SlotCount: u32, TypeId: u32, FunctionTable: u32) {

}

/// GetTypeName - from dxvk/d3d11_class_linkage.h
#[no_mangle]
pub unsafe extern "C" fn GetTypeName(TypeId: u32) -> *mut i8 {
    core::ptr::null_mut()
}

/// GetClassLinkage - from dxvk/d3d11_class_linkage.h
#[no_mangle]
pub unsafe extern "C" fn GetClassLinkage(ppLinkage: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// MatchesTypeName - from dxvk/d3d11_class_linkage.h
#[no_mangle]
pub unsafe extern "C" fn MatchesTypeName(pName: *mut i8) -> usize {
    0
}

/// AddRefPrivate - from dxvk/d3d11_class_linkage.h
#[no_mangle]
pub unsafe extern "C" fn AddRefPrivate() {

}

/// ReleasePrivate - from dxvk/d3d11_class_linkage.h
#[no_mangle]
pub unsafe extern "C" fn ReleasePrivate() {

}

/// ReturnName - from dxvk/d3d11_class_linkage.h
#[no_mangle]
pub unsafe extern "C" fn ReturnName(pName: *mut i8, pLength: *mut core::ffi::c_void, SrcName: usize) {

}

/// GetContextFlags - from dxvk/d3d11_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn GetContextFlags() -> usize {
    0
}

/// AddQuery - from dxvk/d3d11_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn AddQuery(pQuery: *mut core::ffi::c_void) {

}

/// AddChunk - from dxvk/d3d11_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn AddChunk(Chunk: usize, Cost: u64) -> u64 {
    0
}

/// AddCommandList - from dxvk/d3d11_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn AddCommandList(pCommandList: *mut core::ffi::c_void) -> u64 {
    0
}

/// TrackResourceUsage - from dxvk/d3d11_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn TrackResourceUsage(pResource: *mut core::ffi::c_void, ResourceType: usize, Subresource: u32, ChunkId: u64) {

}

/// TrackResourceSequenceNumber - from dxvk/d3d11_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn TrackResourceSequenceNumber(Resource: usize, Seq: u64) {

}

/// DiscardView - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DiscardView(pResourceView: *mut core::ffi::c_void) -> usize {
    0
}

/// DiscardView1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DiscardView1(pResourceView: *mut core::ffi::c_void, pRects: *mut core::ffi::c_void, NumRects: u32) -> usize {
    0
}

/// DiscardViewBase - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DiscardViewBase(pResourceView: *mut core::ffi::c_void, pRects: *mut core::ffi::c_void, NumRects: u32) -> usize {
    0
}

/// CopySubresourceRegion1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CopySubresourceRegion1(pDstResource: *mut core::ffi::c_void, DstSubresource: u32, DstX: u32, DstY: u32, DstZ: u32, pSrcResource: *mut core::ffi::c_void, SrcSubresource: u32, pSrcBox: *mut core::ffi::c_void, CopyFlags: u32) -> usize {
    0
}

/// CopySubresourceRegionBase - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CopySubresourceRegionBase(pDstResource: *mut core::ffi::c_void, DstSubresource: u32, DstX: u32, DstY: u32, DstZ: u32, pSrcResource: *mut core::ffi::c_void, SrcSubresource: u32, pSrcBox: *mut core::ffi::c_void, CopyFlags: u32) -> usize {
    0
}

/// CopyStructureCount - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CopyStructureCount(pDstBuffer: *mut core::ffi::c_void, DstAlignedByteOffset: u32, pSrcView: *mut core::ffi::c_void) -> usize {
    0
}

/// ClearView - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ClearView(pView: *mut core::ffi::c_void, Color4: f32, pRect: *mut core::ffi::c_void, NumRects: u32) -> usize {
    0
}

/// UpdateSubresource1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn UpdateSubresource1(pDstResource: *mut core::ffi::c_void, DstSubresource: u32, pDstBox: *mut core::ffi::c_void, pSrcData: *mut core::ffi::c_void, SrcRowPitch: u32, SrcDepthPitch: u32, CopyFlags: u32) -> usize {
    0
}

/// DispatchIndirect - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DispatchIndirect(pBufferForArgs: *mut core::ffi::c_void, AlignedByteOffsetForArgs: u32) -> usize {
    0
}

/// HSSetShader - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn HSSetShader(pHullShader: *mut core::ffi::c_void, ppClassInstances: *mut *mut core::ffi::c_void, NumClassInstances: u32) -> usize {
    0
}

/// HSSetShaderResources - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn HSSetShaderResources(StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// HSSetSamplers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn HSSetSamplers(StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// HSGetShader - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn HSGetShader(ppHullShader: *mut *mut core::ffi::c_void, ppClassInstances: *mut *mut core::ffi::c_void, pNumClassInstances: *mut u32) -> usize {
    0
}

/// HSGetShaderResources - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn HSGetShaderResources(StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// HSGetSamplers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn HSGetSamplers(StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// DSSetShader - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DSSetShader(pDomainShader: *mut core::ffi::c_void, ppClassInstances: *mut *mut core::ffi::c_void, NumClassInstances: u32) -> usize {
    0
}

/// DSSetShaderResources - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DSSetShaderResources(StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// DSSetSamplers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DSSetSamplers(StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// DSGetShader - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DSGetShader(ppDomainShader: *mut *mut core::ffi::c_void, ppClassInstances: *mut *mut core::ffi::c_void, pNumClassInstances: *mut u32) -> usize {
    0
}

/// DSGetShaderResources - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DSGetShaderResources(StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// DSGetSamplers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DSGetSamplers(StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CSSetShader - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CSSetShader(pComputeShader: *mut core::ffi::c_void, ppClassInstances: *mut *mut core::ffi::c_void, NumClassInstances: u32) -> usize {
    0
}

/// CSSetShaderResources - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CSSetShaderResources(StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CSSetSamplers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CSSetSamplers(StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CSSetUnorderedAccessViews - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CSSetUnorderedAccessViews(StartSlot: u32, NumUAVs: u32, ppUnorderedAccessViews: *mut *mut core::ffi::c_void, pUAVInitialCounts: *mut u32) -> usize {
    0
}

/// CSGetShader - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CSGetShader(ppComputeShader: *mut *mut core::ffi::c_void, ppClassInstances: *mut *mut core::ffi::c_void, pNumClassInstances: *mut u32) -> usize {
    0
}

/// CSGetShaderResources - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CSGetShaderResources(StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CSGetSamplers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CSGetSamplers(StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CSGetUnorderedAccessViews - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CSGetUnorderedAccessViews(StartSlot: u32, NumUAVs: u32, ppUnorderedAccessViews: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OMSetRenderTargetsAndUnorderedAccessViews - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn OMSetRenderTargetsAndUnorderedAccessViews(NumRTVs: u32, ppRenderTargetViews: *mut *mut core::ffi::c_void, pDepthStencilView: *mut core::ffi::c_void, UAVStartSlot: u32, NumUAVs: u32, ppUnorderedAccessViews: *mut *mut core::ffi::c_void, pUAVInitialCounts: *mut u32) -> usize {
    0
}

/// OMGetRenderTargetsAndUnorderedAccessViews - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn OMGetRenderTargetsAndUnorderedAccessViews(NumRTVs: u32, ppRenderTargetViews: *mut *mut core::ffi::c_void, ppDepthStencilView: *mut *mut core::ffi::c_void, UAVStartSlot: u32, NumUAVs: u32, ppUnorderedAccessViews: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SOGetTargetsWithOffsets - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn SOGetTargetsWithOffsets(NumBuffers: u32, ppSOTargets: *mut *mut core::ffi::c_void, pOffsets: *mut u32) -> usize {
    0
}

/// SetResourceMinLOD - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn SetResourceMinLOD(pResource: *mut core::ffi::c_void, MinLOD: f32) -> usize {
    0
}

/// GetResourceMinLOD - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn GetResourceMinLOD(pResource: *mut core::ffi::c_void) -> usize {
    0
}

/// ResizeTilePool - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ResizeTilePool(pTilePool: *mut core::ffi::c_void, NewSizeInBytes: usize) -> usize {
    0
}

/// TiledResourceBarrier - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn TiledResourceBarrier(pTiledResourceOrViewAccessBeforeBarrier: *mut core::ffi::c_void, pTiledResourceOrViewAccessAfterBarrier: *mut core::ffi::c_void) -> usize {
    0
}

/// UpdateTileMappings - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn UpdateTileMappings(pTiledResource: *mut core::ffi::c_void, NumRegions: u32, pRegionCoordinates: *mut core::ffi::c_void, pRegionSizes: *mut core::ffi::c_void, pTilePool: *mut core::ffi::c_void, NumRanges: u32, pRangeFlags: *mut u32, pRangeTileOffsets: *mut u32, pRangeTileCounts: *mut u32, Flags: u32) -> usize {
    0
}

/// UpdateTiles - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn UpdateTiles(pDestTiledResource: *mut core::ffi::c_void, pDestTileRegionStartCoordinate: *mut core::ffi::c_void, pDestTileRegionSize: *mut core::ffi::c_void, pSourceTileData: *mut core::ffi::c_void, Flags: u32) -> usize {
    0
}

/// IsAnnotationEnabled - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn IsAnnotationEnabled() -> usize {
    0
}

/// SetMarkerInt - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn SetMarkerInt(pLabel: *const u16, Data: i32) -> usize {
    0
}

/// BeginEventInt - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BeginEventInt(pLabel: *const u16, Data: i32) -> usize {
    0
}

/// GetHardwareProtectionState - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn GetHardwareProtectionState(pHwProtectionEnable: *mut i32) -> usize {
    0
}

/// SetHardwareProtectionState - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn SetHardwareProtectionState(HwProtectionEnable: i32) -> usize {
    0
}

/// TransitionSurfaceLayout - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn TransitionSurfaceLayout(pSurface: *mut core::ffi::c_void, pSubresources: *mut core::ffi::c_void, OldLayout: usize, NewLayout: usize) -> usize {
    0
}

/// ApplyDirtySamplers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ApplyDirtySamplers(Stage: usize, BoundMask: usize, DirtyMask: usize) {

}

/// ApplyDirtyShaderResources - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ApplyDirtyShaderResources(Stage: usize, BoundMask: usize, DirtyMask: usize) {

}

/// ApplyDirtyUnorderedAccessViews - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ApplyDirtyUnorderedAccessViews(Stage: usize, BoundMask: usize, DirtyMask: usize) {

}

/// ApplyDirtyGraphicsBindings - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ApplyDirtyGraphicsBindings() {

}

/// ApplyDirtyComputeBindings - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ApplyDirtyComputeBindings() {

}

/// ApplyInputLayout - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ApplyInputLayout() {

}

/// ApplyBlendState - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ApplyBlendState() {

}

/// ApplyBlendFactor - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ApplyBlendFactor() {

}

/// ApplyDepthStencilState - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ApplyDepthStencilState() {

}

/// ApplyStencilRef - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ApplyStencilRef() {

}

/// ApplyRasterizerState - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ApplyRasterizerState() {

}

/// ApplyRasterizerSampleCount - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ApplyRasterizerSampleCount() {

}

/// ApplyViewportState - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ApplyViewportState() {

}

/// BatchDraw - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BatchDraw(draw: usize) {

}

/// BatchDrawIndexed - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BatchDrawIndexed(draw: usize) {

}

/// BindShader - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BindShader(pShaderModule: *mut core::ffi::c_void) {

}

/// BindFramebuffer - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BindFramebuffer() {

}

/// BindDrawBuffers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BindDrawBuffers(pBufferForArgs: *mut core::ffi::c_void, pBufferForCount: *mut core::ffi::c_void) {

}

/// BindVertexBuffer - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BindVertexBuffer(Slot: u32, pBuffer: *mut core::ffi::c_void, Offset: u32, Stride: u32) {

}

/// BindVertexBufferRange - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BindVertexBufferRange(Slot: u32, pBuffer: *mut core::ffi::c_void, Offset: u32, Stride: u32) {

}

/// BindIndexBuffer - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BindIndexBuffer(pBuffer: *mut core::ffi::c_void, Offset: u32, Format: usize) {

}

/// BindIndexBufferRange - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BindIndexBufferRange(pBuffer: *mut core::ffi::c_void, Offset: u32, Format: usize) {

}

/// BindXfbBuffer - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BindXfbBuffer(Slot: u32, pBuffer: *mut core::ffi::c_void, Offset: u32) {

}

/// BindSampler - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BindSampler(ShaderStage: usize, Slot: u32, pSampler: *mut core::ffi::c_void) {

}

/// BindShaderResource - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BindShaderResource(ShaderStage: usize, Slot: u32, pResource: *mut core::ffi::c_void) {

}

/// BindUnorderedAccessView - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BindUnorderedAccessView(ShaderStage: usize, Slot: u32, pUav: *mut core::ffi::c_void) {

}

/// ClearImageView - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ClearImageView(View: usize, Color4: f32, pRects: *mut core::ffi::c_void, NumRects: u32) {

}

/// ClearBufferView - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ClearBufferView(View: usize, Color4: f32, pRects: *mut core::ffi::c_void, NumRects: u32) {

}

/// ConvertColorValue - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ConvertColorValue(Color4: f32, pFormatInfo: *mut core::ffi::c_void) -> usize {
    0
}

/// ConvertRect - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ConvertRect(Rect: usize, Extent: usize) -> usize {
    0
}

/// CopyBuffer - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CopyBuffer(pDstBuffer: *mut core::ffi::c_void, DstOffset: u64, pSrcBuffer: *mut core::ffi::c_void, SrcOffset: u64, ByteCount: u64) {

}

/// CopyImage - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CopyImage(pDstTexture: *mut core::ffi::c_void, pDstLayers: *mut core::ffi::c_void, DstOffset: usize, pSrcTexture: *mut core::ffi::c_void, pSrcLayers: *mut core::ffi::c_void, SrcOffset: usize, SrcExtent: usize) {

}

/// CopyTiledResourceData - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CopyTiledResourceData(pResource: *mut core::ffi::c_void, pRegionCoordinate: *mut core::ffi::c_void, pRegionSize: *mut core::ffi::c_void, BufferSlice: usize, Flags: u32) {

}

/// DirtyBindingGeneric - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DirtyBindingGeneric(ShaderStage: usize, BoundMask: usize, DirtyMask: usize, DirtyBit: usize, IsNull: usize) -> usize {
    0
}

/// DirtySampler - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DirtySampler(ShaderStage: usize, Slot: u32, IsNull: usize) -> usize {
    0
}

/// DirtyShaderResource - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DirtyShaderResource(ShaderStage: usize, Slot: u32, IsNull: usize) -> usize {
    0
}

/// DirtyComputeUnorderedAccessView - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DirtyComputeUnorderedAccessView(Slot: u32, IsNull: usize) -> usize {
    0
}

/// DirtyGraphicsUnorderedAccessView - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DirtyGraphicsUnorderedAccessView(Slot: u32) -> usize {
    0
}

/// DiscardBuffer - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DiscardBuffer(pResource: *mut core::ffi::c_void) {

}

/// DiscardTexture - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DiscardTexture(pResource: *mut core::ffi::c_void, Subresource: u32) {

}

/// GetShaderResources - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn GetShaderResources(StartSlot: u32, NumViews: u32, ppShaderResourceViews: *mut *mut core::ffi::c_void) {

}

/// GetSamplers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn GetSamplers(StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut core::ffi::c_void) {

}

/// GetTiledResourceDependency - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn GetTiledResourceDependency(pObject: *mut core::ffi::c_void) -> usize {
    0
}

/// GetMaxUsedBindings - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn GetMaxUsedBindings() -> usize {
    0
}

/// HasDirtyComputeBindings - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn HasDirtyComputeBindings() -> usize {
    0
}

/// HasDirtyGraphicsBindings - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn HasDirtyGraphicsBindings() -> usize {
    0
}

/// ResetCommandListState - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ResetCommandListState() {

}

/// ResetContextState - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ResetContextState() {

}

/// ResetDirtyTracking - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ResetDirtyTracking() {

}

/// ResetStagingBuffer - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ResetStagingBuffer() {

}

/// ResolveSrvHazards - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ResolveSrvHazards(pView: *mut core::ffi::c_void) {

}

/// ResolveCsSrvHazards - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ResolveCsSrvHazards(pView: *mut core::ffi::c_void) {

}

/// ResolveOmSrvHazards - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ResolveOmSrvHazards(pView: *mut core::ffi::c_void) {

}

/// ResolveOmRtvHazards - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ResolveOmRtvHazards(pView: *mut core::ffi::c_void) -> usize {
    0
}

/// ResolveOmUavHazards - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ResolveOmUavHazards(pView: *mut core::ffi::c_void) {

}

/// RestoreCommandListState - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn RestoreCommandListState() {

}

/// RestoreSamplers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn RestoreSamplers(Stage: usize) {

}

/// RestoreShaderResources - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn RestoreShaderResources(Stage: usize) {

}

/// RestoreUnorderedAccessViews - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn RestoreUnorderedAccessViews(Stage: usize) {

}

/// SetShaderResources - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn SetShaderResources(StartSlot: u32, NumResources: u32, ppResources: *mut *mut core::ffi::c_void) {

}

/// SetSamplers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn SetSamplers(StartSlot: u32, NumSamplers: u32, ppSamplers: *mut *mut core::ffi::c_void) {

}

/// SetRenderTargetsAndUnorderedAccessViews - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn SetRenderTargetsAndUnorderedAccessViews(NumRTVs: u32, ppRenderTargetViews: *mut *mut core::ffi::c_void, pDepthStencilView: *mut core::ffi::c_void, UAVStartSlot: u32, NumUAVs: u32, ppUnorderedAccessViews: *mut *mut core::ffi::c_void, pUAVInitialCounts: *mut u32) {

}

/// SetDrawBuffers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn SetDrawBuffers(pBufferForArgs: *mut core::ffi::c_void, pBufferForCount: *mut core::ffi::c_void) {

}

/// SyncImage - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn SyncImage(DstImage: usize, DstLayers: usize, SrcImage: usize, SrcLayers: usize) {

}

/// TestRtvUavHazards - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn TestRtvUavHazards(NumRTVs: u32, ppRTVs: *mut *mut core::ffi::c_void, NumUAVs: u32, ppUAVs: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// TestSrvHazards - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn TestSrvHazards(pView: *mut core::ffi::c_void) -> usize {
    0
}

/// UpdateBuffer - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn UpdateBuffer(pDstBuffer: *mut core::ffi::c_void, Offset: u32, Length: u32, pSrcData: *mut core::ffi::c_void) {

}

/// UpdateTexture - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn UpdateTexture(pDstTexture: *mut core::ffi::c_void, DstSubresource: u32, pDstBox: *mut core::ffi::c_void, pSrcData: *mut core::ffi::c_void, SrcRowPitch: u32, SrcDepthPitch: u32) {

}

/// UpdateImage - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn UpdateImage(pDstTexture: *mut core::ffi::c_void, pDstSubresource: *mut core::ffi::c_void, DstOffset: usize, DstExtent: usize, StagingBuffer: usize) {

}

/// UpdateResource - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn UpdateResource(pDstResource: *mut core::ffi::c_void, DstSubresource: u32, pDstBox: *mut core::ffi::c_void, pSrcData: *mut core::ffi::c_void, SrcRowPitch: u32, SrcDepthPitch: u32, CopyFlags: u32) {

}

/// UpdateUnorderedAccessViewCounter - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn UpdateUnorderedAccessViewCounter(pUav: *mut core::ffi::c_void, CounterValue: u32) {

}

/// ValidateRenderTargets - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ValidateRenderTargets(NumViews: u32, ppRenderTargetViews: *mut *mut core::ffi::c_void, pDepthStencilView: *mut core::ffi::c_void) -> usize {
    0
}

/// InitDefaultRasterizerState - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn InitDefaultRasterizerState() -> usize {
    0
}

/// InitDefaultDepthStencilState - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn InitDefaultDepthStencilState() -> usize {
    0
}

/// InitDefaultMultisampleState - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn InitDefaultMultisampleState(SampleMask: u32) -> usize {
    0
}

/// InitDefaultBlendState - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn InitDefaultBlendState() -> usize {
    0
}

/// EmitCs - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn EmitCs(command: usize) {

}

/// EmitCsCmd - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn EmitCsCmd(arg0: usize, count: usize, command: usize) {

}

/// FlushCsChunk - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn FlushCsChunk() {

}

/// GetCommonShader - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn GetCommonShader(pShader: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// ValidateDrawBufferSize - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ValidateDrawBufferSize(pBuffer: *mut core::ffi::c_void, Offset: u32, Size: u32) -> usize {
    0
}

/// uint64_t - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn uint64_t(arg0: usize) -> usize {
    0
}

/// GetTypedContext - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn GetTypedContext() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// Flush1 - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn Flush1(ContextType: usize, hEvent: *mut core::ffi::c_void) -> usize {
    0
}

/// ExecuteCommandList - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn ExecuteCommandList(pCommandList: *mut core::ffi::c_void, RestoreContextState: i32) -> usize {
    0
}

/// FinishCommandList - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn FinishCommandList(RestoreDeferredContextState: i32, ppCommandList: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SwapDeviceContextState - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn SwapDeviceContextState(pState: *mut core::ffi::c_void, ppPreviousState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// MapBuffer - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn MapBuffer(pResource: *mut core::ffi::c_void, pMappedResource: *mut core::ffi::c_void) -> i32 {
    0
}

/// MapImage - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn MapImage(pResource: *mut core::ffi::c_void, Subresource: u32, pMappedResource: *mut core::ffi::c_void) -> i32 {
    0
}

/// UpdateMappedBuffer - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn UpdateMappedBuffer(pDstBuffer: *mut core::ffi::c_void, Offset: u32, Length: u32, pSrcData: *mut core::ffi::c_void, CopyFlags: u32) {

}

/// FinalizeQueries - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn FinalizeQueries() {

}

/// EmitCsChunk - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn EmitCsChunk(chunk: usize) {

}

/// GetCurrentChunkId - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn GetCurrentChunkId() -> u64 {
    0
}

/// TrackTextureSequenceNumber - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn TrackTextureSequenceNumber(pResource: *mut core::ffi::c_void, Subresource: u32) {

}

/// TrackBufferSequenceNumber - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn TrackBufferSequenceNumber(pResource: *mut core::ffi::c_void) {

}

/// FindMapEntry - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn FindMapEntry(Coookie: u64) -> usize {
    0
}

/// AddMapEntry - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn AddMapEntry(Cookie: u64, MapInfo: usize) {

}

/// GetCsChunkFlags - from dxvk/d3d11_context_def.h
#[no_mangle]
pub unsafe extern "C" fn GetCsChunkFlags(pDevice: *mut core::ffi::c_void) -> usize {
    0
}

/// MultiDrawIndirect - from dxvk/d3d11_context_ext.h
#[no_mangle]
pub unsafe extern "C" fn MultiDrawIndirect(DrawCount: u32, pBufferForArgs: *mut core::ffi::c_void, ByteOffsetForArgs: u32, ByteStrideForArgs: u32) -> usize {
    0
}

/// MultiDrawIndexedIndirect - from dxvk/d3d11_context_ext.h
#[no_mangle]
pub unsafe extern "C" fn MultiDrawIndexedIndirect(DrawCount: u32, pBufferForArgs: *mut core::ffi::c_void, ByteOffsetForArgs: u32, ByteStrideForArgs: u32) -> usize {
    0
}

/// MultiDrawIndirectCount - from dxvk/d3d11_context_ext.h
#[no_mangle]
pub unsafe extern "C" fn MultiDrawIndirectCount(MaxDrawCount: u32, pBufferForCount: *mut core::ffi::c_void, ByteOffsetForCount: u32, pBufferForArgs: *mut core::ffi::c_void, ByteOffsetForArgs: u32, ByteStrideForArgs: u32) -> usize {
    0
}

/// MultiDrawIndexedIndirectCount - from dxvk/d3d11_context_ext.h
#[no_mangle]
pub unsafe extern "C" fn MultiDrawIndexedIndirectCount(MaxDrawCount: u32, pBufferForCount: *mut core::ffi::c_void, ByteOffsetForCount: u32, pBufferForArgs: *mut core::ffi::c_void, ByteOffsetForArgs: u32, ByteStrideForArgs: u32) -> usize {
    0
}

/// SetDepthBoundsTest - from dxvk/d3d11_context_ext.h
#[no_mangle]
pub unsafe extern "C" fn SetDepthBoundsTest(Enable: i32, MinDepthBounds: f32, MaxDepthBounds: f32) -> usize {
    0
}

/// SetBarrierControl - from dxvk/d3d11_context_ext.h
#[no_mangle]
pub unsafe extern "C" fn SetBarrierControl(ControlFlags: u32) -> usize {
    0
}

/// LaunchCubinShaderNVX - from dxvk/d3d11_context_ext.h
#[no_mangle]
pub unsafe extern "C" fn LaunchCubinShaderNVX(hShader: *mut core::ffi::c_void, GridX: u32, GridY: u32, GridZ: u32, pParams: *mut core::ffi::c_void, paramSize: u32, pReadResources: *mut *mut core::ffi::c_void, NumReadResources: u32, pWriteResources: *mut *mut core::ffi::c_void, NumWriteResources: u32) -> usize {
    0
}

/// Acquire11on12Resource - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn Acquire11on12Resource(pResource: *mut core::ffi::c_void, SrcLayout: usize) {

}

/// Release11on12Resource - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn Release11on12Resource(pResource: *mut core::ffi::c_void, DstLayout: usize) {

}

/// InjectCsChunk - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn InjectCsChunk(Queue: usize, Chunk: usize, Synchronize: usize) {

}

/// InjectCs - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn InjectCs(Queue: usize, Command: usize) {

}

/// UnmapImage - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn UnmapImage(pResource: *mut core::ffi::c_void, Subresource: u32) {

}

/// UpdateDirtyImageRegion - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn UpdateDirtyImageRegion(pResource: *mut core::ffi::c_void, Subresource: u32, pRegion: *mut core::ffi::c_void) {

}

/// SynchronizeDevice - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn SynchronizeDevice() {

}

/// EndFrame - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn EndFrame(LatencyTracker: usize) {

}

/// WaitForResource - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn WaitForResource(Resource: usize, SequenceNumber: u64, MapType: usize, MapFlags: u32) -> usize {
    0
}

/// GetCurrentSequenceNumber - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn GetCurrentSequenceNumber() -> u64 {
    0
}

/// GetPendingCsChunks - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn GetPendingCsChunks() -> u64 {
    0
}

/// ApplyDirtyNullBindings - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn ApplyDirtyNullBindings() {

}

/// ConsiderFlush - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn ConsiderFlush(FlushType: usize) {

}

/// ExecuteFlush - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn ExecuteFlush(FlushType: usize, hEvent: *mut core::ffi::c_void, Synchronize: i32) {

}

/// ThrottleDiscard - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn ThrottleDiscard(Size: u64) {

}

/// NotifyRenderPassBoundary - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn NotifyRenderPassBoundary(IsMultisampled: usize) {

}

/// NotifyResolve - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn NotifyResolve() {

}

/// RequestFlush - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn RequestFlush(ContextType: usize, hEvent: *mut core::ffi::c_void) {

}

/// GetStagingMemoryStatistics - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn GetStagingMemoryStatistics() -> usize {
    0
}

/// GetMaxFlushType - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn GetMaxFlushType(pParent: *mut core::ffi::c_void, Device: usize) -> usize {
    0
}

/// cuModule - from dxvk/d3d11_cuda.h
#[no_mangle]
pub unsafe extern "C" fn cuModule() -> usize {
    0
}

/// cuFunction - from dxvk/d3d11_cuda.h
#[no_mangle]
pub unsafe extern "C" fn cuFunction() -> usize {
    0
}

/// insertResource - from dxvk/d3d11_cuda.h
#[no_mangle]
pub unsafe extern "C" fn insertResource(pResource: *mut core::ffi::c_void, access: usize) {

}

/// insertUniqueResource - from dxvk/d3d11_cuda.h
#[no_mangle]
pub unsafe extern "C" fn insertUniqueResource(arg0: usize, list: usize, resource: usize, access: usize) {

}

/// GetState - from dxvk/d3d11_depth_stencil.h
#[no_mangle]
pub unsafe extern "C" fn GetState() -> usize {
    0
}

/// DecodeStencilOpState - from dxvk/d3d11_depth_stencil.h
#[no_mangle]
pub unsafe extern "C" fn DecodeStencilOpState(StencilDesc: usize, Desc: usize) -> usize {
    0
}

/// DecodeStencilOp - from dxvk/d3d11_depth_stencil.h
#[no_mangle]
pub unsafe extern "C" fn DecodeStencilOp(Op: usize) -> usize {
    0
}

/// ValidateDepthFunc - from dxvk/d3d11_depth_stencil.h
#[no_mangle]
pub unsafe extern "C" fn ValidateDepthFunc(Comparison: usize) -> usize {
    0
}

/// ValidateStencilFunc - from dxvk/d3d11_depth_stencil.h
#[no_mangle]
pub unsafe extern "C" fn ValidateStencilFunc(Comparison: usize) -> usize {
    0
}

/// ValidateStencilOp - from dxvk/d3d11_depth_stencil.h
#[no_mangle]
pub unsafe extern "C" fn ValidateStencilOp(StencilOp: usize) -> usize {
    0
}

/// OpenSharedResource1 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn OpenSharedResource1(hResource: *mut core::ffi::c_void, returnedInterface: usize, ppResource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OpenSharedResourceByName - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn OpenSharedResourceByName(lpName: *const u16, dwDesiredAccess: u32, returnedInterface: usize, ppResource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OpenSharedFence - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn OpenSharedFence(hFence: *mut core::ffi::c_void, ReturnedInterface: usize, ppFence: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CheckMultisampleQualityLevels1 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CheckMultisampleQualityLevels1(Format: usize, SampleCount: u32, Flags: u32, pNumQualityLevels: *mut u32) -> usize {
    0
}

/// GetImmediateContext - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetImmediateContext(ppImmediateContext: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetImmediateContext1 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetImmediateContext1(ppImmediateContext: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetImmediateContext2 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetImmediateContext2(ppImmediateContext: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetImmediateContext3 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetImmediateContext3(ppImmediateContext: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// RegisterDeviceRemovedEvent - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn RegisterDeviceRemovedEvent(hEvent: *mut core::ffi::c_void, pdwCookie: *mut u32) -> usize {
    0
}

/// UnregisterDeviceRemoved - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn UnregisterDeviceRemoved(dwCookie: u32) -> usize {
    0
}

/// FlushInitCommands - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn FlushInitCommands() {

}

/// NotifyContextFlush - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn NotifyContextFlush() {

}

/// InitShaderIcb - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn InitShaderIcb(pShader: *mut core::ffi::c_void, IcbSize: usize, pIcbData: *mut core::ffi::c_void) {

}

/// GetEnabledShaderStages - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetEnabledShaderStages() -> usize {
    0
}

/// LookupFormat - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn LookupFormat(Format: usize, Mode: usize) -> usize {
    0
}

/// LookupPackedFormat - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn LookupPackedFormat(Format: usize, Mode: usize) -> usize {
    0
}

/// LookupFamily - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn LookupFamily(Format: usize, Mode: usize) -> usize {
    0
}

/// DxvkCsChunkRef - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn DxvkCsChunkRef(arg0: usize, arg1: usize) -> usize {
    0
}

/// GetOptions - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetOptions() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// GetD3D10Interface - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetD3D10Interface() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// Is11on12Device - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn Is11on12Device() -> usize {
    0
}

/// GetMaxFeatureLevel - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetMaxFeatureLevel(Device: usize) -> usize {
    0
}

/// GetOptionsBarrierControlFlags - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetOptionsBarrierControlFlags() -> usize {
    0
}

/// ComputeShaderKey - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn ComputeShaderKey(Stage: usize, pShaderBytecode: *mut core::ffi::c_void, BytecodeLength: usize) -> usize {
    0
}

/// GetFormatSupportFlags - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetFormatSupportFlags(Format: usize, pFlags1: *mut u32, pFlags2: *mut u32) -> i32 {
    0
}

/// GetImageTypeSupport - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetImageTypeSupport(Format: usize, Type: usize, Flags: usize) -> i32 {
    0
}

/// OpenSharedResourceGeneric - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn OpenSharedResourceGeneric(hResource: *mut core::ffi::c_void, ReturnedInterface: usize, ppResource: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// GetViewPlaneIndex - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetViewPlaneIndex(pResource: *mut core::ffi::c_void, ViewFormat: usize) -> u32 {
    0
}

/// CopySubresourceData - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CopySubresourceData(pData: *mut core::ffi::c_void, RowPitch: u32, DepthPitch: u32, pTexture: *mut core::ffi::c_void, Subresource: u32, pBox: *mut core::ffi::c_void) {

}

/// GetShaderOptions - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetShaderOptions(Device: usize, Options: usize) -> usize {
    0
}

/// ConvertRuntimeDescriptor - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn ConvertRuntimeDescriptor(size: u32, d3dkmt: usize, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// GetExtensionSupport - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetExtensionSupport(Extension: usize) -> usize {
    0
}

/// GetCudaTextureObjectNVX - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetCudaTextureObjectNVX(srvDriverHandle: u32, samplerDriverHandle: u32, pCudaTextureHandle: *mut u32) -> usize {
    0
}

/// GetResourceHandleGPUVirtualAddressAndSizeNVX - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetResourceHandleGPUVirtualAddressAndSizeNVX(hObject: *mut core::ffi::c_void, gpuVAStart: *mut u64, gpuVASize: *mut u64) -> usize {
    0
}

/// AddSamplerAndHandleNVX - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn AddSamplerAndHandleNVX(pSampler: *mut core::ffi::c_void, Handle: u32) {

}

/// HandleToSamplerNVX - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn HandleToSamplerNVX(Handle: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// AddSrvAndHandleNVX - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn AddSrvAndHandleNVX(pSrv: *mut core::ffi::c_void, Handle: u32) {

}

/// HandleToSrvNVX - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn HandleToSrvNVX(Handle: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// GetVideoDecoderProfileCount - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetVideoDecoderProfileCount() -> usize {
    0
}

/// GetVideoDecoderProfile - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetVideoDecoderProfile(Index: u32, pDecoderProfile: *mut core::ffi::c_void) -> usize {
    0
}

/// CheckVideoDecoderFormat - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CheckVideoDecoderFormat(pDecoderProfile: *mut core::ffi::c_void, Format: usize, pSupported: *mut i32) -> usize {
    0
}

/// GetVideoDecoderConfigCount - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetVideoDecoderConfigCount(pDesc: *mut core::ffi::c_void, pCount: *mut u32) -> usize {
    0
}

/// GetVideoDecoderConfig - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetVideoDecoderConfig(pDesc: *mut core::ffi::c_void, Index: u32, pConfig: *mut core::ffi::c_void) -> usize {
    0
}

/// GetContentProtectionCaps - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetContentProtectionCaps(pCryptoType: *mut core::ffi::c_void, pDecoderProfile: *mut core::ffi::c_void, pCaps: *mut core::ffi::c_void) -> usize {
    0
}

/// CheckCryptoKeyExchange - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn CheckCryptoKeyExchange(pCryptoType: *mut core::ffi::c_void, pDecoderProfile: *mut core::ffi::c_void, Index: u32, pKeyExchangeType: *mut core::ffi::c_void) -> usize {
    0
}

/// SupportsLowLatency - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn SupportsLowLatency() -> usize {
    0
}

/// LatencySleep - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn LatencySleep() -> usize {
    0
}

/// SetLatencySleepMode - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn SetLatencySleepMode(LowLatencyEnable: i32, LowLatencyBoost: i32, MinIntervalUs: usize) -> usize {
    0
}

/// SetLatencyMarker - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn SetLatencyMarker(FrameId: usize, MarkerType: usize) -> usize {
    0
}

/// GetLatencyInfo - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetLatencyInfo(pLowLatencyResults: *mut core::ffi::c_void) -> usize {
    0
}

/// RegisterLatencyTracker - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn RegisterLatencyTracker(Tracker: usize) {

}

/// UnregisterLatencyTracker - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn UnregisterLatencyTracker(Tracker: usize) {

}

/// SetAPIVersion - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn SetAPIVersion(Version: u32) -> usize {
    0
}

/// GetAPIVersion - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetAPIVersion() -> usize {
    0
}

/// GetParent - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetParent(riid: usize, ppParent: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// QueryResourceResidency - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn QueryResourceResidency(ppResources: *mut *mut core::ffi::c_void, pResidencyStatus: *mut core::ffi::c_void, NumResources: u32) -> usize {
    0
}

/// GetMaximumFrameLatency - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetMaximumFrameLatency(pMaxLatency: *mut u32) -> usize {
    0
}

/// SetMaximumFrameLatency - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn SetMaximumFrameLatency(MaxLatency: u32) -> usize {
    0
}

/// OfferResources - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn OfferResources(NumResources: u32, ppResources: *mut *mut core::ffi::c_void, Priority: usize) -> usize {
    0
}

/// OfferResources1 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn OfferResources1(NumResources: u32, ppResources: *mut *mut core::ffi::c_void, Priority: usize, Flags: u32) -> usize {
    0
}

/// ReclaimResources - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn ReclaimResources(NumResources: u32, ppResources: *mut *mut core::ffi::c_void, pDiscarded: *mut i32) -> usize {
    0
}

/// ReclaimResources1 - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn ReclaimResources1(NumResources: u32, ppResources: *mut *mut core::ffi::c_void, pResults: *mut core::ffi::c_void) -> usize {
    0
}

/// EnqueueSetEvent - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn EnqueueSetEvent(hEvent: *mut core::ffi::c_void) -> usize {
    0
}

/// Trim - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn Trim() -> usize {
    0
}

/// GetDXVKDevice - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetDXVKDevice() -> usize {
    0
}

/// GetParentInterface - from dxvk/d3d11_device_child.h
#[no_mangle]
pub unsafe extern "C" fn GetParentInterface() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// IsCurrent - from dxvk/d3d11_device_child.h
#[no_mangle]
pub unsafe extern "C" fn IsCurrent(version: u32) -> i32 {
    0
}

/// GetFeatureData - from dxvk/d3d11_features.h
#[no_mangle]
pub unsafe extern "C" fn GetFeatureData(Feature: usize, FeatureDataSize: u32, pFeatureData: *mut core::ffi::c_void) -> i32 {
    0
}

/// GetTiledResourcesTier - from dxvk/d3d11_features.h
#[no_mangle]
pub unsafe extern "C" fn GetTiledResourcesTier() -> usize {
    0
}

/// GetConservativeRasterizationTier - from dxvk/d3d11_features.h
#[no_mangle]
pub unsafe extern "C" fn GetConservativeRasterizationTier() -> usize {
    0
}

/// GetTypedFeatureData - from dxvk/d3d11_features.h
#[no_mangle]
pub unsafe extern "C" fn GetTypedFeatureData(Size: u32, pDstData: *mut core::ffi::c_void, pSrcData: *mut core::ffi::c_void) -> i32 {
    0
}

/// DetermineConservativeRasterizationTier - from dxvk/d3d11_features.h
#[no_mangle]
pub unsafe extern "C" fn DetermineConservativeRasterizationTier(Device: usize, FeatureLevel: usize) -> usize {
    0
}

/// DetermineSharedResourceTier - from dxvk/d3d11_features.h
#[no_mangle]
pub unsafe extern "C" fn DetermineSharedResourceTier(Device: usize, FeatureLevel: usize) -> usize {
    0
}

/// DetermineTiledResourcesTier - from dxvk/d3d11_features.h
#[no_mangle]
pub unsafe extern "C" fn DetermineTiledResourcesTier(Device: usize, FeatureLevel: usize) -> usize {
    0
}

/// DetermineUavExtendedTypedLoadSupport - from dxvk/d3d11_features.h
#[no_mangle]
pub unsafe extern "C" fn DetermineUavExtendedTypedLoadSupport(Device: usize, FeatureLevel: usize) -> i32 {
    0
}

/// CheckFormatSharingSupport - from dxvk/d3d11_features.h
#[no_mangle]
pub unsafe extern "C" fn CheckFormatSharingSupport(Device: usize, Format: usize, HandleType: usize) -> i32 {
    0
}

/// Acquire - from dxvk/d3d11_gdi.h
#[no_mangle]
pub unsafe extern "C" fn Acquire(Discard: i32, phdc: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// InitBuffer - from dxvk/d3d11_initializer.h
#[no_mangle]
pub unsafe extern "C" fn InitBuffer(pBuffer: *mut core::ffi::c_void, pInitialData: *mut core::ffi::c_void) {

}

/// InitTexture - from dxvk/d3d11_initializer.h
#[no_mangle]
pub unsafe extern "C" fn InitTexture(pTexture: *mut core::ffi::c_void, pInitialData: *mut core::ffi::c_void) {

}

/// InitUavCounter - from dxvk/d3d11_initializer.h
#[no_mangle]
pub unsafe extern "C" fn InitUavCounter(pUav: *mut core::ffi::c_void) {

}

/// InitDeviceLocalBuffer - from dxvk/d3d11_initializer.h
#[no_mangle]
pub unsafe extern "C" fn InitDeviceLocalBuffer(pBuffer: *mut core::ffi::c_void, pInitialData: *mut core::ffi::c_void) {

}

/// InitHostVisibleBuffer - from dxvk/d3d11_initializer.h
#[no_mangle]
pub unsafe extern "C" fn InitHostVisibleBuffer(pBuffer: *mut core::ffi::c_void, pInitialData: *mut core::ffi::c_void) {

}

/// InitDeviceLocalTexture - from dxvk/d3d11_initializer.h
#[no_mangle]
pub unsafe extern "C" fn InitDeviceLocalTexture(pTexture: *mut core::ffi::c_void, pInitialData: *mut core::ffi::c_void) {

}

/// InitHostVisibleTexture - from dxvk/d3d11_initializer.h
#[no_mangle]
pub unsafe extern "C" fn InitHostVisibleTexture(pTexture: *mut core::ffi::c_void, pInitialData: *mut core::ffi::c_void) {

}

/// InitTiledTexture - from dxvk/d3d11_initializer.h
#[no_mangle]
pub unsafe extern "C" fn InitTiledTexture(pTexture: *mut core::ffi::c_void) {

}

/// SyncSharedTexture - from dxvk/d3d11_initializer.h
#[no_mangle]
pub unsafe extern "C" fn SyncSharedTexture(pResource: *mut core::ffi::c_void) {

}

/// GetAttributeCount - from dxvk/d3d11_input_layout.h
#[no_mangle]
pub unsafe extern "C" fn GetAttributeCount() -> u32 {
    0
}

/// GetBindingCount - from dxvk/d3d11_input_layout.h
#[no_mangle]
pub unsafe extern "C" fn GetBindingCount() -> u32 {
    0
}

/// GetInput - from dxvk/d3d11_input_layout.h
#[no_mangle]
pub unsafe extern "C" fn GetInput(Index: u32) -> usize {
    0
}

/// Compare - from dxvk/d3d11_input_layout.h
#[no_mangle]
pub unsafe extern "C" fn Compare(pOther: *mut core::ffi::c_void) -> usize {
    0
}

/// GetVulkanHandles - from dxvk/d3d11_interop.h
#[no_mangle]
pub unsafe extern "C" fn GetVulkanHandles(pInstance: *mut core::ffi::c_void, pPhysDev: *mut core::ffi::c_void, pDevice: *mut core::ffi::c_void) -> usize {
    0
}

/// GetSubmissionQueue - from dxvk/d3d11_interop.h
#[no_mangle]
pub unsafe extern "C" fn GetSubmissionQueue(pQueue: *mut core::ffi::c_void, pQueueFamilyIndex: *mut u32) -> usize {
    0
}

/// FlushRenderingCommands - from dxvk/d3d11_interop.h
#[no_mangle]
pub unsafe extern "C" fn FlushRenderingCommands() -> usize {
    0
}

/// ReleaseSubmissionQueue - from dxvk/d3d11_interop.h
#[no_mangle]
pub unsafe extern "C" fn ReleaseSubmissionQueue() -> usize {
    0
}

/// GetSubmissionQueue1 - from dxvk/d3d11_interop.h
#[no_mangle]
pub unsafe extern "C" fn GetSubmissionQueue1(pQueue: *mut core::ffi::c_void, pQueueIndex: *mut u32, pQueueFamilyIndex: *mut u32) -> usize {
    0
}

/// ReleaseWrappedResources - from dxvk/d3d11_on_12.h
#[no_mangle]
pub unsafe extern "C" fn ReleaseWrappedResources(ppResources: *mut *mut core::ffi::c_void, ResourceCount: u32) -> usize {
    0
}

/// AcquireWrappedResources - from dxvk/d3d11_on_12.h
#[no_mangle]
pub unsafe extern "C" fn AcquireWrappedResources(ppResources: *mut *mut core::ffi::c_void, ResourceCount: u32) -> usize {
    0
}

/// GetDeviceExtensions - from dxvk/d3d11_on_12_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn GetDeviceExtensions(pExtensionCount: *mut u32, ppExtensions: *mut *mut i8) -> usize {
    0
}

/// GetDeviceFeatures - from dxvk/d3d11_on_12_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn GetDeviceFeatures(ppFeatures: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetVulkanQueueInfo - from dxvk/d3d11_on_12_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn GetVulkanQueueInfo(pCommandQueue: *mut core::ffi::c_void, pVkQueue: *mut core::ffi::c_void, pVkQueueFamily: *mut core::ffi::c_void) -> usize {
    0
}

/// GetVulkanImageLayout - from dxvk/d3d11_on_12_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn GetVulkanImageLayout(pResource: *mut core::ffi::c_void, State: usize, pVkLayout: *mut core::ffi::c_void) -> usize {
    0
}

/// GetVulkanResourceInfo - from dxvk/d3d11_on_12_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn GetVulkanResourceInfo(pResource: *mut core::ffi::c_void, pVkHandle: *mut core::ffi::c_void, pBufferOffset: *mut core::ffi::c_void) -> usize {
    0
}

/// DoBegin - from dxvk/d3d11_query.h
#[no_mangle]
pub unsafe extern "C" fn DoBegin() -> usize {
    0
}

/// DoEnd - from dxvk/d3d11_query.h
#[no_mangle]
pub unsafe extern "C" fn DoEnd() -> usize {
    0
}

/// DoDeferredEnd - from dxvk/d3d11_query.h
#[no_mangle]
pub unsafe extern "C" fn DoDeferredEnd() {

}

/// IsScoped - from dxvk/d3d11_query.h
#[no_mangle]
pub unsafe extern "C" fn IsScoped() -> usize {
    0
}

/// IsEvent - from dxvk/d3d11_query.h
#[no_mangle]
pub unsafe extern "C" fn IsEvent() -> usize {
    0
}

/// IsStalling - from dxvk/d3d11_query.h
#[no_mangle]
pub unsafe extern "C" fn IsStalling() -> usize {
    0
}

/// NotifyEnd - from dxvk/d3d11_query.h
#[no_mangle]
pub unsafe extern "C" fn NotifyEnd() {

}

/// NotifyStall - from dxvk/d3d11_query.h
#[no_mangle]
pub unsafe extern "C" fn NotifyStall() {

}

/// ValidateDesc - from dxvk/d3d11_query.h
#[no_mangle]
pub unsafe extern "C" fn ValidateDesc(pDesc: *mut core::ffi::c_void) -> i32 {
    0
}

/// AsPredicate - from dxvk/d3d11_query.h
#[no_mangle]
pub unsafe extern "C" fn AsPredicate(pQuery: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// FromPredicate - from dxvk/d3d11_query.h
#[no_mangle]
pub unsafe extern "C" fn FromPredicate(pPredicate: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// GetTimestampQueryFrequency - from dxvk/d3d11_query.h
#[no_mangle]
pub unsafe extern "C" fn GetTimestampQueryFrequency() -> usize {
    0
}

/// GetDesc2 - from dxvk/d3d11_rasterizer.h
#[no_mangle]
pub unsafe extern "C" fn GetDesc2(pDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// GetDepthBias - from dxvk/d3d11_rasterizer.h
#[no_mangle]
pub unsafe extern "C" fn GetDepthBias() -> usize {
    0
}

/// AcquireSync - from dxvk/d3d11_resource.h
#[no_mangle]
pub unsafe extern "C" fn AcquireSync(Key: usize, dwMilliseconds: u32) -> usize {
    0
}

/// ReleaseSync - from dxvk/d3d11_resource.h
#[no_mangle]
pub unsafe extern "C" fn ReleaseSync(Key: usize) -> usize {
    0
}

/// GetSharedHandle - from dxvk/d3d11_resource.h
#[no_mangle]
pub unsafe extern "C" fn GetSharedHandle(pSharedHandle: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetUsage - from dxvk/d3d11_resource.h
#[no_mangle]
pub unsafe extern "C" fn GetUsage(pUsage: *mut core::ffi::c_void) -> usize {
    0
}

/// GetResource11on12Info - from dxvk/d3d11_resource.h
#[no_mangle]
pub unsafe extern "C" fn GetResource11on12Info(pResource: *mut core::ffi::c_void, p11on12Info: *mut core::ffi::c_void) -> i32 {
    0
}

/// GetCommonResourceDesc - from dxvk/d3d11_resource.h
#[no_mangle]
pub unsafe extern "C" fn GetCommonResourceDesc(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void) -> i32 {
    0
}

/// CheckResourceViewCompatibility - from dxvk/d3d11_resource.h
#[no_mangle]
pub unsafe extern "C" fn CheckResourceViewCompatibility(pResource: *mut core::ffi::c_void, BindFlags: u32, Format: usize, Plane: u32) -> i32 {
    0
}

/// ResourceAddRefPrivate - from dxvk/d3d11_resource.h
#[no_mangle]
pub unsafe extern "C" fn ResourceAddRefPrivate(pResource: *mut core::ffi::c_void, Type: usize) -> i32 {
    0
}

/// ResourceReleasePrivate - from dxvk/d3d11_resource.h
#[no_mangle]
pub unsafe extern "C" fn ResourceReleasePrivate(pResource: *mut core::ffi::c_void, Type: usize) -> i32 {
    0
}

/// GetSubresource - from dxvk/d3d11_resource.h
#[no_mangle]
pub unsafe extern "C" fn GetSubresource() -> u32 {
    0
}

/// ValidateAddressMode - from dxvk/d3d11_sampler.h
#[no_mangle]
pub unsafe extern "C" fn ValidateAddressMode(Mode: usize) -> usize {
    0
}

/// ValidateComparisonFunc - from dxvk/d3d11_sampler.h
#[no_mangle]
pub unsafe extern "C" fn ValidateComparisonFunc(Comparison: usize) -> usize {
    0
}

/// uint32_t - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn uint32_t(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// VkShaderStageFlagBits - from dxvk/d3d11_shader.h
#[no_mangle]
pub unsafe extern "C" fn VkShaderStageFlagBits(uint32_tProgramType: *mut core::ffi::c_void) -> usize {
    0
}

/// computeCbvBinding - from dxvk/d3d11_shader.h
#[no_mangle]
pub unsafe extern "C" fn computeCbvBinding(stage: usize, index: u32) -> u32 {
    0
}

/// computeStageIndex - from dxvk/d3d11_shader.h
#[no_mangle]
pub unsafe extern "C" fn computeStageIndex(arg0: usize) -> usize {
    0
}

/// computeSamplerBinding - from dxvk/d3d11_shader.h
#[no_mangle]
pub unsafe extern "C" fn computeSamplerBinding(stage: usize, index: u32) -> u32 {
    0
}

/// computeSrvBinding - from dxvk/d3d11_shader.h
#[no_mangle]
pub unsafe extern "C" fn computeSrvBinding(stage: usize, index: u32) -> u32 {
    0
}

/// computeUavCounterBinding - from dxvk/d3d11_shader.h
#[no_mangle]
pub unsafe extern "C" fn computeUavCounterBinding(stage: usize, index: u32) -> u32 {
    0
}

/// computeUavBinding - from dxvk/d3d11_shader.h
#[no_mangle]
pub unsafe extern "C" fn computeUavBinding(arg0: usize, arg1: usize) -> usize {
    0
}

/// GetIcb - from dxvk/d3d11_shader.h
#[no_mangle]
pub unsafe extern "C" fn GetIcb() -> usize {
    0
}

/// GetBindingMask - from dxvk/d3d11_shader.h
#[no_mangle]
pub unsafe extern "C" fn GetBindingMask() -> usize {
    0
}

/// GatherInterefaceInfo - from dxvk/d3d11_shader.h
#[no_mangle]
pub unsafe extern "C" fn GatherInterefaceInfo(pLinkage: *mut core::ffi::c_void, pShaderBytecode: *mut core::ffi::c_void, BytecodeLength: usize) {

}

/// GetShaderModule - from dxvk/d3d11_shader.h
#[no_mangle]
pub unsafe extern "C" fn GetShaderModule(pDevice: *mut core::ffi::c_void, pLinkage: *mut core::ffi::c_void, ShaderKey: usize, ModuleInfo: usize, pShaderBytecode: *mut core::ffi::c_void, BytecodeLength: usize, Icb: usize, BindingMask: usize, pShader: *mut core::ffi::c_void) -> i32 {
    0
}

/// GetImage - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetImage(BufferId: u32, riid: usize, ppBuffer: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetImageIndex - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetImageIndex() -> usize {
    0
}

/// GetFrameLatency - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetFrameLatency() -> usize {
    0
}

/// GetFrameLatencyEvent - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetFrameLatencyEvent() -> usize {
    0
}

/// ChangeProperties - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn ChangeProperties(pDesc: *mut core::ffi::c_void, pNodeMasks: *mut u32, ppPresentQueues: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SetPresentRegion - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn SetPresentRegion(pRegion: *mut core::ffi::c_void) -> usize {
    0
}

/// SetGammaControl - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn SetGammaControl(NumControlPoints: u32, pControlPoints: *mut core::ffi::c_void) -> usize {
    0
}

/// SetFrameLatency - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn SetFrameLatency(MaxLatency: u32) -> usize {
    0
}

/// CheckColorSpaceSupport - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn CheckColorSpaceSupport(ColorSpace: usize) -> usize {
    0
}

/// SetHDRMetaData - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn SetHDRMetaData(pMetaData: *mut core::ffi::c_void) -> usize {
    0
}

/// GetLastPresentCount - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetLastPresentCount(pLastPresentCount: *mut core::ffi::c_void) -> usize {
    0
}

/// GetFrameStatistics - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetFrameStatistics(pFrameStatistics: *mut core::ffi::c_void) -> usize {
    0
}

/// SetTargetFrameRate - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn SetTargetFrameRate(FrameRate: f64) -> usize {
    0
}

/// PresentImage - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn PresentImage(SyncInterval: u32) -> i32 {
    0
}

/// RotateBackBuffers - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn RotateBackBuffers(ctx: *mut core::ffi::c_void) {

}

/// DestroyFrameLatencyEvent - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn DestroyFrameLatencyEvent() {

}

/// DestroyLatencyTracker - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn DestroyLatencyTracker() {

}

/// SyncFrameLatency - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn SyncFrameLatency() {

}

/// GetActualFrameLatency - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetActualFrameLatency() -> u32 {
    0
}

/// GetSurfaceFormat - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetSurfaceFormat(Format: usize) -> usize {
    0
}

/// GetApiName - from dxvk/d3d11_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetApiName() -> usize {
    0
}

/// GetInterface - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetInterface() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// GetDimension - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetDimension() -> usize {
    0
}

/// GetVkImageType - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetVkImageType() -> usize {
    0
}

/// GetImageTypeFromResourceDim - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetImageTypeFromResourceDim(arg0: usize) -> usize {
    0
}

/// CountSubresources - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn CountSubresources() -> u32 {
    0
}

/// HasImage - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn HasImage() -> usize {
    0
}

/// NotifyMap - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn NotifyMap(Subresource: u32, MapType: usize) {

}

/// GetPackedFormat - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetPackedFormat() -> usize {
    0
}

/// AddDirtyRegion - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn AddDirtyRegion(Subresource: u32, Offset: usize, Extent: usize) {

}

/// GetDirtyRegion - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetDirtyRegion(Subresource: u32, Region: u32) -> usize {
    0
}

/// ComputeMappedOffset - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn ComputeMappedOffset(Subresource: u32, Plane: u32, Offset: usize) -> u64 {
    0
}

/// GetSubresourceFromIndex - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetSubresourceFromIndex(Aspect: usize, Subresource: u32) -> usize {
    0
}

/// GetSubresourceLayout - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetSubresourceLayout(AspectMask: usize, Subresource: u32) -> usize {
    0
}

/// GetFormatMode - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetFormatMode() -> usize {
    0
}

/// NormalizeTextureProperties - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn NormalizeTextureProperties(pDesc: *mut core::ffi::c_void) -> i32 {
    0
}

/// CheckImageSupport - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn CheckImageSupport(pImageInfo: *mut core::ffi::c_void, Tiling: usize) -> i32 {
    0
}

/// DetermineSubresourceLayout - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn DetermineSubresourceLayout(pImageInfo: *mut core::ffi::c_void, subresource: usize) -> usize {
    0
}

/// IsR32UavCompatibleFormat - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn IsR32UavCompatibleFormat(Format: usize) -> i32 {
    0
}

/// OptimizeLayout - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn OptimizeLayout(Usage: usize) -> usize {
    0
}

/// ReleaseDC - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn ReleaseDC(pDirtyRect: *mut core::ffi::c_void) -> usize {
    0
}

/// isSurfaceCompatible - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn isSurfaceCompatible() -> usize {
    0
}

/// GetVulkanImageInfo - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetVulkanImageInfo(pHandle: *mut core::ffi::c_void, pLayout: *mut core::ffi::c_void, pInfo: *mut core::ffi::c_void) -> usize {
    0
}

/// GetCommonTexture - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetCommonTexture() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// setCbv - from dxvk/d3d11_util.h
#[no_mangle]
pub unsafe extern "C" fn setCbv(index: u32) {

}

/// setSampler - from dxvk/d3d11_util.h
#[no_mangle]
pub unsafe extern "C" fn setSampler(index: u32) {

}

/// setUav - from dxvk/d3d11_util.h
#[no_mangle]
pub unsafe extern "C" fn setUav(index: u32) {

}

/// setSrv - from dxvk/d3d11_util.h
#[no_mangle]
pub unsafe extern "C" fn setSrv(index: u32) {

}

/// CompactSparseList - from dxvk/d3d11_util.h
#[no_mangle]
pub unsafe extern "C" fn CompactSparseList(pData: *mut core::ffi::c_void, Mask: u32) -> u32 {
    0
}

/// DecodeSampleCount - from dxvk/d3d11_util.h
#[no_mangle]
pub unsafe extern "C" fn DecodeSampleCount(Count: u32, pCount: *mut core::ffi::c_void) -> i32 {
    0
}

/// DecodeAddressMode - from dxvk/d3d11_util.h
#[no_mangle]
pub unsafe extern "C" fn DecodeAddressMode(mode: usize) -> usize {
    0
}

/// DecodeCompareOp - from dxvk/d3d11_util.h
#[no_mangle]
pub unsafe extern "C" fn DecodeCompareOp(Mode: usize) -> usize {
    0
}

/// DecodeReductionMode - from dxvk/d3d11_util.h
#[no_mangle]
pub unsafe extern "C" fn DecodeReductionMode(Filter: u32) -> usize {
    0
}

/// DecodeConservativeRasterizationMode - from dxvk/d3d11_util.h
#[no_mangle]
pub unsafe extern "C" fn DecodeConservativeRasterizationMode(Mode: usize) -> usize {
    0
}

/// GetBufferFormatFeatures - from dxvk/d3d11_util.h
#[no_mangle]
pub unsafe extern "C" fn GetBufferFormatFeatures(BindFlags: u32) -> usize {
    0
}

/// GetImageFormatFeatures - from dxvk/d3d11_util.h
#[no_mangle]
pub unsafe extern "C" fn GetImageFormatFeatures(BindFlags: u32) -> usize {
    0
}

/// GetPackedDepthStencilFormat - from dxvk/d3d11_util.h
#[no_mangle]
pub unsafe extern "C" fn GetPackedDepthStencilFormat(Format: usize) -> usize {
    0
}

/// IsMinMaxFilter - from dxvk/d3d11_util.h
#[no_mangle]
pub unsafe extern "C" fn IsMinMaxFilter(Filter: usize) -> i32 {
    0
}

/// GetVideoProcessorContentDesc - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn GetVideoProcessorContentDesc(pContentDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// CheckVideoProcessorFormat - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn CheckVideoProcessorFormat(Format: usize, pFlags: *mut u32) -> usize {
    0
}

/// GetVideoProcessorCaps - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn GetVideoProcessorCaps(pCaps: *mut core::ffi::c_void) -> usize {
    0
}

/// GetVideoProcessorRateConversionCaps - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn GetVideoProcessorRateConversionCaps(TypeIndex: u32, pCaps: *mut core::ffi::c_void) -> usize {
    0
}

/// GetVideoProcessorCustomRate - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn GetVideoProcessorCustomRate(TypeIndex: u32, CustomRateIndex: u32, pRate: *mut core::ffi::c_void) -> usize {
    0
}

/// GetVideoProcessorFilterRange - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn GetVideoProcessorFilterRange(Filter: usize, pRange: *mut core::ffi::c_void) -> usize {
    0
}

/// GetContentDesc - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn GetContentDesc(pDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// GetRateConversionCaps - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn GetRateConversionCaps(pCaps: *mut core::ffi::c_void) -> usize {
    0
}

/// GetStreamState - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn GetStreamState(StreamIndex: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// IsYCbCr - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn IsYCbCr() -> usize {
    0
}

/// GetImageSubresource - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn GetImageSubresource() -> usize {
    0
}

/// IsYCbCrFormat - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn IsYCbCrFormat(Format: usize) -> usize {
    0
}

/// GetDecoderBuffer - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn GetDecoderBuffer(pDecoder: *mut core::ffi::c_void, Type: usize, BufferSize: *mut u32, ppBuffer: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// ReleaseDecoderBuffer - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn ReleaseDecoderBuffer(pDecoder: *mut core::ffi::c_void, Type: usize) -> usize {
    0
}

/// DecoderBeginFrame - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn DecoderBeginFrame(pDecoder: *mut core::ffi::c_void, pView: *mut core::ffi::c_void, KeySize: u32, pKey: *mut core::ffi::c_void) -> usize {
    0
}

/// DecoderEndFrame - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn DecoderEndFrame(pDecoder: *mut core::ffi::c_void) -> usize {
    0
}

/// SubmitDecoderBuffers - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn SubmitDecoderBuffers(pDecoder: *mut core::ffi::c_void, BufferCount: u32, pBufferDescs: *mut core::ffi::c_void) -> usize {
    0
}

/// DecoderExtension - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn DecoderExtension(pDecoder: *mut core::ffi::c_void, pExtension: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorSetOutputTargetRect - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetOutputTargetRect(pVideoProcessor: *mut core::ffi::c_void, Enable: i32, pRect: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorSetOutputBackgroundColor - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetOutputBackgroundColor(pVideoProcessor: *mut core::ffi::c_void, YCbCr: i32, pColor: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorSetOutputColorSpace - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetOutputColorSpace(pVideoProcessor: *mut core::ffi::c_void, pColorSpace: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorSetOutputAlphaFillMode - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetOutputAlphaFillMode(pVideoProcessor: *mut core::ffi::c_void, AlphaFillMode: usize, StreamIndex: u32) -> usize {
    0
}

/// VideoProcessorSetOutputConstriction - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetOutputConstriction(pVideoProcessor: *mut core::ffi::c_void, Enable: i32, Size: usize) -> usize {
    0
}

/// VideoProcessorSetOutputStereoMode - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetOutputStereoMode(pVideoProcessor: *mut core::ffi::c_void, Enable: i32) -> usize {
    0
}

/// VideoProcessorSetOutputExtension - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetOutputExtension(pVideoProcessor: *mut core::ffi::c_void, pExtensionGuid: *mut core::ffi::c_void, DataSize: u32, pData: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorSetStreamFrameFormat - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetStreamFrameFormat(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, Format: usize) -> usize {
    0
}

/// VideoProcessorSetStreamColorSpace - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetStreamColorSpace(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, pColorSpace: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorSetStreamOutputRate - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetStreamOutputRate(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, Rate: usize, Repeat: i32, CustomRate: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorSetStreamSourceRect - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetStreamSourceRect(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, Enable: i32, pRect: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorSetStreamDestRect - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetStreamDestRect(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, Enable: i32, pRect: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorSetStreamAlpha - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetStreamAlpha(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, Enable: i32, Alpha: f32) -> usize {
    0
}

/// VideoProcessorSetStreamPalette - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetStreamPalette(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, EntryCount: u32, pEntries: *mut u32) -> usize {
    0
}

/// VideoProcessorSetStreamPixelAspectRatio - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetStreamPixelAspectRatio(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, Enable: i32, pSrcAspectRatio: *mut core::ffi::c_void, pDstAspectRatio: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorSetStreamLumaKey - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetStreamLumaKey(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, Enable: i32, Lower: f32, Upper: f32) -> usize {
    0
}

/// VideoProcessorSetStreamStereoFormat - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetStreamStereoFormat(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, Enable: i32, Format: usize, LeftViewFrame0: i32, BaseViewFrame0: i32, FlipMode: usize, MonoOffset: i32) -> usize {
    0
}

/// VideoProcessorSetStreamFilter - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetStreamFilter(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, Filter: usize, Enable: i32, Level: i32) -> usize {
    0
}

/// VideoProcessorSetStreamExtension - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetStreamExtension(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, pExtensionGuid: *mut core::ffi::c_void, DataSize: u32, pData: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorSetStreamRotation - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetStreamRotation(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, Enable: i32, Rotation: usize) -> usize {
    0
}

/// VideoProcessorGetOutputTargetRect - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetOutputTargetRect(pVideoProcessor: *mut core::ffi::c_void, pEnabled: *mut i32, pRect: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorGetOutputBackgroundColor - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetOutputBackgroundColor(pVideoProcessor: *mut core::ffi::c_void, pYCbCr: *mut i32, pColor: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorGetOutputColorSpace - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetOutputColorSpace(pVideoProcessor: *mut core::ffi::c_void, pColorSpace: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorGetOutputAlphaFillMode - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetOutputAlphaFillMode(pVideoProcessor: *mut core::ffi::c_void, pAlphaFillMode: *mut core::ffi::c_void, pStreamIndex: *mut u32) -> usize {
    0
}

/// VideoProcessorGetOutputConstriction - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetOutputConstriction(pVideoProcessor: *mut core::ffi::c_void, pEnabled: *mut i32, pSize: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorGetOutputStereoMode - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetOutputStereoMode(pVideoProcessor: *mut core::ffi::c_void, pEnabled: *mut i32) -> usize {
    0
}

/// VideoProcessorGetOutputExtension - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetOutputExtension(pVideoProcessor: *mut core::ffi::c_void, pExtensionGuid: *mut core::ffi::c_void, DataSize: u32, pData: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorGetStreamFrameFormat - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetStreamFrameFormat(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, pFormat: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorGetStreamColorSpace - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetStreamColorSpace(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, pColorSpace: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorGetStreamOutputRate - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetStreamOutputRate(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, pRate: *mut core::ffi::c_void, pRepeat: *mut i32, pCustomRate: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorGetStreamSourceRect - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetStreamSourceRect(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, pEnabled: *mut i32, pRect: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorGetStreamDestRect - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetStreamDestRect(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, pEnabled: *mut i32, pRect: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorGetStreamAlpha - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetStreamAlpha(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, pEnabled: *mut i32, pAlpha: *mut f32) -> usize {
    0
}

/// VideoProcessorGetStreamPalette - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetStreamPalette(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, EntryCount: u32, pEntries: *mut u32) -> usize {
    0
}

/// VideoProcessorGetStreamPixelAspectRatio - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetStreamPixelAspectRatio(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, pEnabled: *mut i32, pSrcAspectRatio: *mut core::ffi::c_void, pDstAspectRatio: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorGetStreamLumaKey - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetStreamLumaKey(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, pEnabled: *mut i32, pLower: *mut f32, pUpper: *mut f32) -> usize {
    0
}

/// VideoProcessorGetStreamStereoFormat - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetStreamStereoFormat(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, pEnabled: *mut i32, pFormat: *mut core::ffi::c_void, pLeftViewFrame0: *mut i32, pBaseViewFrame0: *mut i32, pFlipMode: *mut core::ffi::c_void, pMonoOffset: *mut i32) -> usize {
    0
}

/// VideoProcessorGetStreamFilter - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetStreamFilter(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, Filter: usize, pEnabled: *mut i32, pLevel: *mut i32) -> usize {
    0
}

/// VideoProcessorGetStreamExtension - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetStreamExtension(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, pExtensionGuid: *mut core::ffi::c_void, DataSize: u32, pData: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorGetStreamRotation - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetStreamRotation(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, pEnable: *mut i32, pRotation: *mut core::ffi::c_void) -> usize {
    0
}

/// VideoProcessorBlt - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorBlt(pVideoProcessor: *mut core::ffi::c_void, pOutputView: *mut core::ffi::c_void, FrameIdx: u32, StreamCount: u32, pStreams: *mut core::ffi::c_void) -> usize {
    0
}

/// NegotiateCryptoSessionKeyExchange - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn NegotiateCryptoSessionKeyExchange(pSession: *mut core::ffi::c_void, DataSize: u32, pData: *mut core::ffi::c_void) -> usize {
    0
}

/// EncryptionBlt - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn EncryptionBlt(pSession: *mut core::ffi::c_void, pSrcSurface: *mut core::ffi::c_void, pDstSurface: *mut core::ffi::c_void, IVSize: u32, pIV: *mut core::ffi::c_void) -> usize {
    0
}

/// DecryptionBlt - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn DecryptionBlt(pSession: *mut core::ffi::c_void, pSrcSurface: *mut core::ffi::c_void, pDstSurface: *mut core::ffi::c_void, pBlockInfo: *mut core::ffi::c_void, KeySize: u32, pKey: *mut core::ffi::c_void, IVSize: u32, pIV: *mut core::ffi::c_void) -> usize {
    0
}

/// StartSessionKeyRefresh - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn StartSessionKeyRefresh(pSession: *mut core::ffi::c_void, RandomNumberSize: u32, pRandomNumber: *mut core::ffi::c_void) -> usize {
    0
}

/// FinishSessionKeyRefresh - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn FinishSessionKeyRefresh(pSession: *mut core::ffi::c_void) -> usize {
    0
}

/// GetEncryptionBltKey - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn GetEncryptionBltKey(pSession: *mut core::ffi::c_void, KeySize: u32, pKey: *mut core::ffi::c_void) -> usize {
    0
}

/// NegotiateAuthenticatedChannelKeyExchange - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn NegotiateAuthenticatedChannelKeyExchange(pChannel: *mut core::ffi::c_void, DataSize: u32, pData: *mut core::ffi::c_void) -> usize {
    0
}

/// QueryAuthenticatedChannel - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn QueryAuthenticatedChannel(pChannel: *mut core::ffi::c_void, InputSize: u32, pInput: *mut core::ffi::c_void, OutputSize: u32, pOutput: *mut core::ffi::c_void) -> usize {
    0
}

/// ConfigureAuthenticatedChannel - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn ConfigureAuthenticatedChannel(pChannel: *mut core::ffi::c_void, InputSize: u32, pInput: *mut core::ffi::c_void, pOutput: *mut core::ffi::c_void) -> usize {
    0
}

/// alignas - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn alignas(arg0: usize) -> usize {
    0
}

/// ApplyColorMatrix - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn ApplyColorMatrix(pDst34: f32, pSrc34: f32) {

}

/// ApplyYCbCrMatrix - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn ApplyYCbCrMatrix(pColorMatrix34: f32, UseBt709: usize) {

}

/// BindOutputView - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn BindOutputView(View: usize, FirstView: usize) {

}

/// BlitStream - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn BlitStream(pStreamState: *mut core::ffi::c_void, pStream: *mut core::ffi::c_void) {

}

/// CopyBaseImageToShadow - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn CopyBaseImageToShadow(View: usize) {

}

/// CopyShadowToBaseImage - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn CopyShadowToBaseImage(View: usize) {

}

/// UnbindResources - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn UnbindResources() {

}

/// CheckViewOverlap - from dxvk/d3d11_view.h
#[no_mangle]
pub unsafe extern "C" fn CheckViewOverlap(a: usize, b: usize) -> usize {
    0
}

/// GetResourceType - from dxvk/d3d11_view_dsv.h
#[no_mangle]
pub unsafe extern "C" fn GetResourceType() -> usize {
    0
}

/// GetSampleCount - from dxvk/d3d11_view_dsv.h
#[no_mangle]
pub unsafe extern "C" fn GetSampleCount() -> u32 {
    0
}

/// GetWritableAspectMask - from dxvk/d3d11_view_dsv.h
#[no_mangle]
pub unsafe extern "C" fn GetWritableAspectMask() -> usize {
    0
}

/// GetViewFormat - from dxvk/d3d11_view_dsv.h
#[no_mangle]
pub unsafe extern "C" fn GetViewFormat() -> usize {
    0
}

/// GetDescFromResource - from dxvk/d3d11_view_dsv.h
#[no_mangle]
pub unsafe extern "C" fn GetDescFromResource(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void) -> i32 {
    0
}

/// GetViewLayout - from dxvk/d3d11_view_dsv.h
#[no_mangle]
pub unsafe extern "C" fn GetViewLayout() -> usize {
    0
}

/// HasBindFlag - from dxvk/d3d11_view_rtv.h
#[no_mangle]
pub unsafe extern "C" fn HasBindFlag(Flags: u32) -> i32 {
    0
}

/// GetPlaneSlice - from dxvk/d3d11_view_rtv.h
#[no_mangle]
pub unsafe extern "C" fn GetPlaneSlice(pDesc: *mut core::ffi::c_void) -> u32 {
    0
}

/// GetResourceDesc - from dxvk/d3d11_view_srv.h
#[no_mangle]
pub unsafe extern "C" fn GetResourceDesc() -> usize {
    0
}

/// HasCounter - from dxvk/d3d11_view_uav.h
#[no_mangle]
pub unsafe extern "C" fn HasCounter() -> i32 {
    0
}

/// PreLoad - from dxvk/d3d8_batch.h
#[no_mangle]
pub unsafe extern "C" fn PreLoad() -> usize {
    0
}

/// GetPtr - from dxvk/d3d8_batch.h
#[no_mangle]
pub unsafe extern "C" fn GetPtr(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// Size - from dxvk/d3d8_batch.h
#[no_mangle]
pub unsafe extern "C" fn Size() -> u32 {
    0
}

/// D3D8BatchBuffer - from dxvk/d3d8_batch.h
#[no_mangle]
pub unsafe extern "C" fn D3D8BatchBuffer(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// StateChange - from dxvk/d3d8_batch.h
#[no_mangle]
pub unsafe extern "C" fn StateChange() {

}

/// DrawPrimitive - from dxvk/d3d8_batch.h
#[no_mangle]
pub unsafe extern "C" fn DrawPrimitive(PrimitiveType: usize, StartVertex: u32, PrimitiveCount: u32) -> i32 {
    0
}

/// SetStream - from dxvk/d3d8_batch.h
#[no_mangle]
pub unsafe extern "C" fn SetStream(num: u32, stream: *mut core::ffi::c_void, stride: u32) {

}

/// SetIndices - from dxvk/d3d8_batch.h
#[no_mangle]
pub unsafe extern "C" fn SetIndices(indices: *mut core::ffi::c_void, baseVertexIndex: i32) {

}

/// TestCooperativeLevel - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn TestCooperativeLevel() -> usize {
    0
}

/// GetAvailableTextureMem - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetAvailableTextureMem() -> usize {
    0
}

/// ResourceManagerDiscardBytes - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn ResourceManagerDiscardBytes(bytes: u32) -> usize {
    0
}

/// GetDirect3D - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetDirect3D(ppD3D8: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetDisplayMode - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetDisplayMode(pMode: *mut core::ffi::c_void) -> usize {
    0
}

/// GetCreationParameters - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetCreationParameters(pParameters: *mut core::ffi::c_void) -> usize {
    0
}

/// SetCursorProperties - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetCursorProperties(XHotSpot: u32, YHotSpot: u32, pCursorBitmap: *mut core::ffi::c_void) -> usize {
    0
}

/// ShowCursor - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn ShowCursor(bShow: i32) -> usize {
    0
}

/// GetBackBuffer - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetBackBuffer(iBackBuffer: u32, Type: usize, ppBackBuffer: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetRasterStatus - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetRasterStatus(pRasterStatus: *mut core::ffi::c_void) -> usize {
    0
}

/// SetGammaRamp - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetGammaRamp(Flags: u32, pRamp: *mut core::ffi::c_void) -> usize {
    0
}

/// GetGammaRamp - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetGammaRamp(pRamp: *mut core::ffi::c_void) -> usize {
    0
}

/// CopyRects - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn CopyRects(pSourceSurface: *mut core::ffi::c_void, pSourceRectsArray: *mut core::ffi::c_void, cRects: u32, pDestinationSurface: *mut core::ffi::c_void, pDestPointsArray: *mut core::ffi::c_void) -> usize {
    0
}

/// GetFrontBuffer - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetFrontBuffer(pDestSurface: *mut core::ffi::c_void) -> usize {
    0
}

/// GetRenderTarget - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetRenderTarget(ppRenderTarget: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetDepthStencilSurface - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetDepthStencilSurface(ppZStencilSurface: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// BeginScene - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn BeginScene() -> usize {
    0
}

/// EndScene - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn EndScene() -> usize {
    0
}

/// SetTransform - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetTransform(State: usize, pMatrix: *mut core::ffi::c_void) -> usize {
    0
}

/// MultiplyTransform - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn MultiplyTransform(TransformState: usize, pMatrix: *mut core::ffi::c_void) -> usize {
    0
}

/// SetViewport - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetViewport(pViewport: *mut core::ffi::c_void) -> usize {
    0
}

/// GetViewport - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetViewport(pViewport: *mut core::ffi::c_void) -> usize {
    0
}

/// SetMaterial - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetMaterial(pMaterial: *mut core::ffi::c_void) -> usize {
    0
}

/// GetMaterial - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetMaterial(pMaterial: *mut core::ffi::c_void) -> usize {
    0
}

/// SetLight - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetLight(Index: u32, pLight: *mut core::ffi::c_void) -> usize {
    0
}

/// GetLight - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetLight(Index: u32, pLight: *mut core::ffi::c_void) -> usize {
    0
}

/// LightEnable - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn LightEnable(Index: u32, Enable: i32) -> usize {
    0
}

/// GetLightEnable - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetLightEnable(Index: u32, pEnable: *mut i32) -> usize {
    0
}

/// SetClipPlane - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetClipPlane(Index: u32, pPlane: *mut f32) -> usize {
    0
}

/// GetClipPlane - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetClipPlane(Index: u32, pPlane: *mut f32) -> usize {
    0
}

/// SetRenderState - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetRenderState(State: usize, Value: u32) -> usize {
    0
}

/// GetRenderState - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetRenderState(State: usize, pValue: *mut u32) -> usize {
    0
}

/// SetClipStatus - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetClipStatus(pClipStatus: *mut core::ffi::c_void) -> usize {
    0
}

/// GetClipStatus - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetClipStatus(pClipStatus: *mut core::ffi::c_void) -> usize {
    0
}

/// GetTexture - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetTexture(Stage: u32, ppTexture: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SetTexture - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetTexture(Stage: u32, pTexture: *mut core::ffi::c_void) -> usize {
    0
}

/// GetTextureStageState - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetTextureStageState(Stage: u32, Type: usize, pValue: *mut u32) -> usize {
    0
}

/// SetTextureStageState - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetTextureStageState(Stage: u32, Type: usize, Value: u32) -> usize {
    0
}

/// ValidateDevice - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn ValidateDevice(pNumPasses: *mut u32) -> usize {
    0
}

/// GetInfo - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetInfo(DevInfoID: u32, pDevInfoStruct: *mut core::ffi::c_void, DevInfoStructSize: u32) -> usize {
    0
}

/// SetCurrentTexturePalette - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetCurrentTexturePalette(PaletteNumber: u32) -> usize {
    0
}

/// GetCurrentTexturePalette - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetCurrentTexturePalette(PaletteNumber: *mut u32) -> usize {
    0
}

/// DrawIndexedPrimitive - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn DrawIndexedPrimitive(PrimitiveType: usize, MinVertexIndex: u32, NumVertices: u32, StartIndex: u32, PrimitiveCount: u32) -> usize {
    0
}

/// DrawPrimitiveUP - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn DrawPrimitiveUP(PrimitiveType: usize, PrimitiveCount: u32, pVertexStreamZeroData: *mut core::ffi::c_void, VertexStreamZeroStride: u32) -> usize {
    0
}

/// DrawIndexedPrimitiveUP - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn DrawIndexedPrimitiveUP(PrimitiveType: usize, MinVertexIndex: u32, NumVertices: u32, PrimitiveCount: u32, pIndexData: *mut core::ffi::c_void, IndexDataFormat: usize, pVertexStreamZeroData: *mut core::ffi::c_void, VertexStreamZeroStride: u32) -> usize {
    0
}

/// ProcessVertices - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn ProcessVertices(SrcStartIndex: u32, DestIndex: u32, VertexCount: u32, pDestBuffer: *mut core::ffi::c_void, Flags: u32) -> usize {
    0
}

/// SetVertexShader - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetVertexShader(Handle: u32) -> usize {
    0
}

/// GetVertexShader - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetVertexShader(pHandle: *mut u32) -> usize {
    0
}

/// DeleteVertexShader - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn DeleteVertexShader(Handle: u32) -> usize {
    0
}

/// GetVertexShaderDeclaration - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetVertexShaderDeclaration(Handle: u32, pData: *mut core::ffi::c_void, pSizeOfData: *mut u32) -> usize {
    0
}

/// GetVertexShaderFunction - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetVertexShaderFunction(Handle: u32, pData: *mut core::ffi::c_void, pSizeOfData: *mut u32) -> usize {
    0
}

/// SetStreamSource - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetStreamSource(StreamNumber: u32, pStreamData: *mut core::ffi::c_void, Stride: u32) -> usize {
    0
}

/// GetStreamSource - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetStreamSource(StreamNumber: u32, ppStreamData: *mut *mut core::ffi::c_void, pStride: *mut u32) -> usize {
    0
}

/// GetIndices - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetIndices(ppIndexData: *mut *mut core::ffi::c_void, pBaseVertexIndex: *mut u32) -> usize {
    0
}

/// SetPixelShader - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetPixelShader(Handle: u32) -> usize {
    0
}

/// GetPixelShader - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetPixelShader(pHandle: *mut u32) -> usize {
    0
}

/// DeletePixelShader - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn DeletePixelShader(Handle: usize) -> usize {
    0
}

/// GetPixelShaderFunction - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetPixelShaderFunction(Handle: u32, pData: *mut core::ffi::c_void, pSizeOfData: *mut u32) -> usize {
    0
}

/// DrawRectPatch - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn DrawRectPatch(Handle: u32, pNumSegs: *mut f32, pRectPatchInfo: *mut core::ffi::c_void) -> usize {
    0
}

/// DrawTriPatch - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn DrawTriPatch(Handle: u32, pNumSegs: *mut f32, pTriPatchInfo: *mut core::ffi::c_void) -> usize {
    0
}

/// DeletePatch - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn DeletePatch(Handle: u32) -> usize {
    0
}

/// ShouldRecord - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn ShouldRecord() -> usize {
    0
}

/// ShouldBatch - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn ShouldBatch() -> usize {
    0
}

/// ResetState - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn ResetState() {

}

/// D3D8Surface - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn D3D8Surface(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// getPixelShaderPtr - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn getPixelShaderPtr(device: *mut core::ffi::c_void, Handle: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// getVertexShaderInfo - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn getVertexShaderInfo(device: *mut core::ffi::c_void, Handle: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// getFormatStride - from dxvk/d3d8_format.h
#[no_mangle]
pub unsafe extern "C" fn getFormatStride(fmt: usize) -> u32 {
    0
}

/// getSurfaceSize - from dxvk/d3d8_format.h
#[no_mangle]
pub unsafe extern "C" fn getSurfaceSize(Format: usize, Width: u32, Height: u32) -> u32 {
    0
}

/// RegisterSoftwareDevice - from dxvk/d3d8_interface.h
#[no_mangle]
pub unsafe extern "C" fn RegisterSoftwareDevice(pInitializeFunction: *mut core::ffi::c_void) -> usize {
    0
}

/// GetAdapterIdentifier - from dxvk/d3d8_interface.h
#[no_mangle]
pub unsafe extern "C" fn GetAdapterIdentifier(Adapter: u32, Flags: u32, pIdentifier: *mut core::ffi::c_void) -> usize {
    0
}

/// GetAdapterModeCount - from dxvk/d3d8_interface.h
#[no_mangle]
pub unsafe extern "C" fn GetAdapterModeCount(Adapter: u32) -> usize {
    0
}

/// EnumAdapterModes - from dxvk/d3d8_interface.h
#[no_mangle]
pub unsafe extern "C" fn EnumAdapterModes(Adapter: u32, Mode: u32, pMode: *mut core::ffi::c_void) -> usize {
    0
}

/// GetAdapterDisplayMode - from dxvk/d3d8_interface.h
#[no_mangle]
pub unsafe extern "C" fn GetAdapterDisplayMode(Adapter: u32, pMode: *mut core::ffi::c_void) -> usize {
    0
}

/// CheckDeviceFormat - from dxvk/d3d8_interface.h
#[no_mangle]
pub unsafe extern "C" fn CheckDeviceFormat(Adapter: u32, DeviceType: usize, AdapterFormat: usize, Usage: u32, RType: usize, CheckFormat: usize) -> usize {
    0
}

/// CheckDeviceMultiSampleType - from dxvk/d3d8_interface.h
#[no_mangle]
pub unsafe extern "C" fn CheckDeviceMultiSampleType(Adapter: u32, DeviceType: usize, SurfaceFormat: usize, Windowed: i32, MultiSampleType: usize) -> usize {
    0
}

/// GetAdapterMonitor - from dxvk/d3d8_interface.h
#[no_mangle]
pub unsafe extern "C" fn GetAdapterMonitor(Adapter: u32) -> usize {
    0
}

/// ValidatePresentationParameters - from dxvk/d3d8_interface.h
#[no_mangle]
pub unsafe extern "C" fn ValidatePresentationParameters(pPresentationParameters: *mut core::ffi::c_void) -> i32 {
    0
}

/// parseVsDecl - from dxvk/d3d8_options.h
#[no_mangle]
pub unsafe extern "C" fn parseVsDecl(decl: usize) {

}

/// SetPriority - from dxvk/d3d8_resource.h
#[no_mangle]
pub unsafe extern "C" fn SetPriority(PriorityNew: u32) -> usize {
    0
}

/// GetPriority - from dxvk/d3d8_resource.h
#[no_mangle]
pub unsafe extern "C" fn GetPriority() -> usize {
    0
}

/// TranslateVertexShader8 - from dxvk/d3d8_shader.h
#[no_mangle]
pub unsafe extern "C" fn TranslateVertexShader8(pDeclaration: *mut u32, pFunction: *mut u32, overrides: usize, pTranslatedVS: usize) -> i32 {
    0
}

/// Capture - from dxvk/d3d8_state_block.h
#[no_mangle]
pub unsafe extern "C" fn Capture() -> i32 {
    0
}

/// Apply - from dxvk/d3d8_state_block.h
#[no_mangle]
pub unsafe extern "C" fn Apply() -> i32 {
    0
}

/// GetContainer - from dxvk/d3d8_subresource.h
#[no_mangle]
pub unsafe extern "C" fn GetContainer(riid: usize, ppContainer: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetBaseTexture - from dxvk/d3d8_subresource.h
#[no_mangle]
pub unsafe extern "C" fn GetBaseTexture() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SetLOD - from dxvk/d3d8_texture.h
#[no_mangle]
pub unsafe extern "C" fn SetLOD(LODNew: u32) -> usize {
    0
}

/// GetLOD - from dxvk/d3d8_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetLOD() -> usize {
    0
}

/// GetLevelCount - from dxvk/d3d8_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetLevelCount() -> usize {
    0
}

/// SubresourceType - from dxvk/d3d8_texture.h
#[no_mangle]
pub unsafe extern "C" fn SubresourceType(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// DxvkError - from dxvk/d3d8_texture.h
#[no_mangle]
pub unsafe extern "C" fn DxvkError(arg0: usize, arg1: usize) -> usize {
    0
}

/// GetLevelDesc - from dxvk/d3d8_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetLevelDesc(Level: u32, pDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// GetSurfaceLevel - from dxvk/d3d8_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetSurfaceLevel(Level: u32, ppSurfaceLevel: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// AddDirtyRect - from dxvk/d3d8_texture.h
#[no_mangle]
pub unsafe extern "C" fn AddDirtyRect(pDirtyRect: *mut core::ffi::c_void) -> usize {
    0
}

/// GetVolumeLevel - from dxvk/d3d8_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetVolumeLevel(Level: u32, ppVolumeLevel: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// AddDirtyBox - from dxvk/d3d8_texture.h
#[no_mangle]
pub unsafe extern "C" fn AddDirtyBox(pDirtyBox: *mut core::ffi::c_void) -> usize {
    0
}

/// GetCubeMapSurface - from dxvk/d3d8_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetCubeMapSurface(Face: usize, Level: u32, ppSurfaceLevel: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// ConvertCaps8 - from dxvk/d3d8_util.h
#[no_mangle]
pub unsafe extern "C" fn ConvertCaps8(caps9: usize, pCaps8: *mut core::ffi::c_void) {

}

/// ConvertPresentParameters9 - from dxvk/d3d8_util.h
#[no_mangle]
pub unsafe extern "C" fn ConvertPresentParameters9(pParams: *mut core::ffi::c_void) -> usize {
    0
}

/// ConvertSurfaceDesc8 - from dxvk/d3d8_util.h
#[no_mangle]
pub unsafe extern "C" fn ConvertSurfaceDesc8(pSurf9: *mut core::ffi::c_void, pSurf8: *mut core::ffi::c_void) {

}

/// ConvertVolumeDesc8 - from dxvk/d3d8_util.h
#[no_mangle]
pub unsafe extern "C" fn ConvertVolumeDesc8(pVol9: *mut core::ffi::c_void, pVol8: *mut core::ffi::c_void) {

}

/// GetSamplerStateType9 - from dxvk/d3d8_util.h
#[no_mangle]
pub unsafe extern "C" fn GetSamplerStateType9(StageType: usize) -> usize {
    0
}

/// isFVF - from dxvk/d3d8_util.h
#[no_mangle]
pub unsafe extern "C" fn isFVF(Handle: u32) -> u32 {
    0
}

/// getShaderHandle - from dxvk/d3d8_util.h
#[no_mangle]
pub unsafe extern "C" fn getShaderHandle(Index: u32) -> u32 {
    0
}

/// getShaderIndex - from dxvk/d3d8_util.h
#[no_mangle]
pub unsafe extern "C" fn getShaderIndex(Handle: u32) -> u32 {
    0
}

/// CheckDeviceType - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn CheckDeviceType(DevType: usize, AdapterFormat: usize, BackBufferFormat: usize, bWindowed: i32) -> i32 {
    0
}

/// CheckDepthStencilMatch - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn CheckDepthStencilMatch(DeviceType: usize, AdapterFormat: usize, RenderTargetFormat: usize, DepthStencilFormat: usize) -> i32 {
    0
}

/// CheckDeviceFormatConversion - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn CheckDeviceFormatConversion(DeviceType: usize, SourceFormat: usize, TargetFormat: usize) -> i32 {
    0
}

/// GetMonitor - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn GetMonitor() -> usize {
    0
}

/// GetAdapterModeCountEx - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn GetAdapterModeCountEx(pFilter: *mut core::ffi::c_void) -> u32 {
    0
}

/// EnumAdapterModesEx - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn EnumAdapterModesEx(pFilter: *mut core::ffi::c_void, Mode: u32, pMode: *mut core::ffi::c_void) -> i32 {
    0
}

/// GetAdapterDisplayModeEx - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn GetAdapterDisplayModeEx(pMode: *mut core::ffi::c_void, pRotation: *mut core::ffi::c_void) -> i32 {
    0
}

/// GetAdapterLUID - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn GetAdapterLUID(pLUID: *mut core::ffi::c_void) -> i32 {
    0
}

/// GetOrdinal - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn GetOrdinal() -> u32 {
    0
}

/// GetVendorId - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn GetVendorId() -> u32 {
    0
}

/// GetFormatMapping - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn GetFormatMapping(Format: usize) -> usize {
    0
}

/// GetUnsupportedFormatInfo - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn GetUnsupportedFormatInfo(Format: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// Get9On12Args - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn Get9On12Args() -> usize {
    0
}

/// RefreshFormatsTable - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn RefreshFormatsTable() {

}

/// IsExtended - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn IsExtended() -> usize {
    0
}

/// IsD3D8Compatible - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn IsD3D8Compatible() -> usize {
    0
}

/// incRef - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn incRef() -> usize {
    0
}

/// decRef - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn decRef() -> usize {
    0
}

/// IsCountCompatibleMode - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn IsCountCompatibleMode(wsiMode: usize) -> usize {
    0
}

/// CheckDeviceVkFormat - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn CheckDeviceVkFormat(Format: usize, Usage: u32, RType: usize) -> i32 {
    0
}

/// CacheModes - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn CacheModes(Format: usize) {

}

/// FilterModesByFormat - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn FilterModesByFormat(Format: usize, ApplyOptionsFilter: usize) {

}

/// CacheIdentifierInfo - from dxvk/d3d9_adapter.h
#[no_mangle]
pub unsafe extern "C" fn CacheIdentifierInfo() {

}

/// RegisterAnnotator - from dxvk/d3d9_annotation.h
#[no_mangle]
pub unsafe extern "C" fn RegisterAnnotator(annotation: *mut core::ffi::c_void) {

}

/// UnregisterAnnotator - from dxvk/d3d9_annotation.h
#[no_mangle]
pub unsafe extern "C" fn UnregisterAnnotator(annotation: *mut core::ffi::c_void) {

}

/// SetRegion - from dxvk/d3d9_annotation.h
#[no_mangle]
pub unsafe extern "C" fn SetRegion(color: usize, name: *const u16) {

}

/// QueryRepeatFrame - from dxvk/d3d9_annotation.h
#[no_mangle]
pub unsafe extern "C" fn QueryRepeatFrame() -> i32 {
    0
}

/// SetOptions - from dxvk/d3d9_annotation.h
#[no_mangle]
pub unsafe extern "C" fn SetOptions(options: u32) {

}

/// UpdateTextureFromBuffer - from dxvk/d3d9_bridge.h
#[no_mangle]
pub unsafe extern "C" fn UpdateTextureFromBuffer(pDestSurface: *mut core::ffi::c_void, pSrcSurface: *mut core::ffi::c_void, pSrcRect: *mut core::ffi::c_void, pDestPoint: *mut core::ffi::c_void) -> usize {
    0
}

/// IsSupportedSurfaceFormat - from dxvk/d3d9_bridge.h
#[no_mangle]
pub unsafe extern "C" fn IsSupportedSurfaceFormat(Format: usize) -> usize {
    0
}

/// EnableD3D8CompatibilityMode - from dxvk/d3d9_bridge.h
#[no_mangle]
pub unsafe extern "C" fn EnableD3D8CompatibilityMode() -> usize {
    0
}

/// GetConfig - from dxvk/d3d9_bridge.h
#[no_mangle]
pub unsafe extern "C" fn GetConfig() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// IsDegenerate - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn IsDegenerate() -> usize {
    0
}

/// Conjoin - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn Conjoin(range: usize) {

}

/// Overlaps - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn Overlaps(range: usize) -> usize {
    0
}

/// GetMapBuffer - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetMapBuffer() -> usize {
    0
}

/// GetStagingBuffer - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetStagingBuffer() -> usize {
    0
}

/// GetRealBuffer - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetRealBuffer() -> usize {
    0
}

/// GetMapFlags - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetMapFlags() -> u32 {
    0
}

/// SetMapFlags - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn SetMapFlags(Flags: u32) {

}

/// ValidateBufferProperties - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn ValidateBufferProperties(pDesc: *mut core::ffi::c_void, IsExtended: usize) -> i32 {
    0
}

/// TrackMappingBufferSequenceNumber - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn TrackMappingBufferSequenceNumber(Seq: u64) {

}

/// GetMappingBufferSequenceNumber - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetMappingBufferSequenceNumber() -> u64 {
    0
}

/// DoPerDrawUpload - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn DoPerDrawUpload() -> usize {
    0
}

/// Device - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn Device() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// IsShadow - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn IsShadow() -> usize {
    0
}

/// IsCube - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn IsCube() -> usize {
    0
}

/// IsUpgradedToD32f - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn IsUpgradedToD32f() -> usize {
    0
}

/// SupportsFetch4 - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn SupportsFetch4() -> usize {
    0
}

/// IsNull - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn IsNull() -> usize {
    0
}

/// UnmapData - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn UnmapData() {

}

/// DestroyBuffer - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn DestroyBuffer() {

}

/// IsDynamic - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn IsDynamic() -> usize {
    0
}

/// IsManaged - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn IsManaged() -> usize {
    0
}

/// IsPoolManaged - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn IsPoolManaged(arg0: usize) -> usize {
    0
}

/// IsRenderTarget - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn IsRenderTarget() -> usize {
    0
}

/// IsDepthStencil - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn IsDepthStencil() -> usize {
    0
}

/// IsAutomaticMip - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn IsAutomaticMip() -> usize {
    0
}

/// IsSrgbCompatible - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn IsSrgbCompatible() -> usize {
    0
}

/// GetExtentMip - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetExtentMip(Subresource: u32) -> usize {
    0
}

/// MarkTransitionedToHazardLayout - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn MarkTransitionedToHazardLayout() -> usize {
    0
}

/// HasBeenTransitionedToHazardLayout - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn HasBeenTransitionedToHazardLayout() -> usize {
    0
}

/// GetPool - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetPool() -> usize {
    0
}

/// SetAllNeedUpload - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn SetAllNeedUpload() {

}

/// SetNeedsUpload - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn SetNeedsUpload(Subresource: u32, upload: usize) {

}

/// NeedsUpload - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn NeedsUpload(Subresource: u32) -> usize {
    0
}

/// NeedsAnyUpload - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn NeedsAnyUpload() -> usize {
    0
}

/// ClearNeedsUpload - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn ClearNeedsUpload() {

}

/// SetNeedsMipGen - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn SetNeedsMipGen(value: usize) {

}

/// NeedsMipGen - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn NeedsMipGen() -> usize {
    0
}

/// SetMipFilter - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn SetMipFilter(filter: usize) {

}

/// GetMipFilter - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetMipFilter() -> usize {
    0
}

/// PreLoadAll - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn PreLoadAll() {

}

/// PreLoadSubresource - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn PreLoadSubresource(Subresource: u32) {

}

/// ClearDirtyBoxes - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn ClearDirtyBoxes() {

}

/// GetImageTypeFromResourceType - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetImageTypeFromResourceType(Dimension: usize) -> usize {
    0
}

/// GetImageViewTypeFromResourceType - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetImageViewTypeFromResourceType(Dimension: usize, Layer: u32) -> usize {
    0
}

/// GetMipSize - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetMipSize(Subresource: u32) -> u64 {
    0
}

/// GetTotalSize - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetTotalSize() -> u32 {
    0
}

/// GetVkInterop - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetVkInterop() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// DetermineShadowState - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn DetermineShadowState() -> i32 {
    0
}

/// DetermineFetch4Compatibility - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn DetermineFetch4Compatibility() -> i32 {
    0
}

/// GetAlignment - from dxvk/d3d9_constant_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetAlignment() -> u64 {
    0
}

/// getAlignment - from dxvk/d3d9_constant_buffer.h
#[no_mangle]
pub unsafe extern "C" fn getAlignment(device: usize) -> u64 {
    0
}

/// floatSize - from dxvk/d3d9_constant_layout.h
#[no_mangle]
pub unsafe extern "C" fn floatSize() -> u32 {
    0
}

/// intSize - from dxvk/d3d9_constant_layout.h
#[no_mangle]
pub unsafe extern "C" fn intSize() -> u32 {
    0
}

/// bitmaskSize - from dxvk/d3d9_constant_layout.h
#[no_mangle]
pub unsafe extern "C" fn bitmaskSize() -> u32 {
    0
}

/// intOffset - from dxvk/d3d9_constant_layout.h
#[no_mangle]
pub unsafe extern "C" fn intOffset() -> u32 {
    0
}

/// floatOffset - from dxvk/d3d9_constant_layout.h
#[no_mangle]
pub unsafe extern "C" fn floatOffset() -> u32 {
    0
}

/// bitmaskOffset - from dxvk/d3d9_constant_layout.h
#[no_mangle]
pub unsafe extern "C" fn bitmaskOffset() -> u32 {
    0
}

/// totalSize - from dxvk/d3d9_constant_layout.h
#[no_mangle]
pub unsafe extern "C" fn totalSize() -> u32 {
    0
}

/// ResetCursor - from dxvk/d3d9_cursor.h
#[no_mangle]
pub unsafe extern "C" fn ResetCursor() {

}

/// ResetHardwareCursor - from dxvk/d3d9_cursor.h
#[no_mangle]
pub unsafe extern "C" fn ResetHardwareCursor() {

}

/// ResetSoftwareCursor - from dxvk/d3d9_cursor.h
#[no_mangle]
pub unsafe extern "C" fn ResetSoftwareCursor() {

}

/// UpdateCursor - from dxvk/d3d9_cursor.h
#[no_mangle]
pub unsafe extern "C" fn UpdateCursor(X: i32, Y: i32) {

}

/// SetHardwareCursor - from dxvk/d3d9_cursor.h
#[no_mangle]
pub unsafe extern "C" fn SetHardwareCursor(XHotSpot: u32, YHotSpot: u32, bitmap: usize) {

}

/// SetSoftwareCursor - from dxvk/d3d9_cursor.h
#[no_mangle]
pub unsafe extern "C" fn SetSoftwareCursor(XHotSpot: u32, YHotSpot: u32, Width: u32, Height: u32) {

}

/// GetSoftwareCursor - from dxvk/d3d9_cursor.h
#[no_mangle]
pub unsafe extern "C" fn GetSoftwareCursor() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// IsSoftwareCursor - from dxvk/d3d9_cursor.h
#[no_mangle]
pub unsafe extern "C" fn IsSoftwareCursor() -> usize {
    0
}

/// IsActiveSoftwareCursor - from dxvk/d3d9_cursor.h
#[no_mangle]
pub unsafe extern "C" fn IsActiveSoftwareCursor() -> usize {
    0
}

/// IsHardwareCursor - from dxvk/d3d9_cursor.h
#[no_mangle]
pub unsafe extern "C" fn IsHardwareCursor() -> usize {
    0
}

/// EvictManagedResources - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn EvictManagedResources() -> usize {
    0
}

/// GetSwapChain - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetSwapChain(iSwapChain: u32, pSwapChain: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetNumberOfSwapChains - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetNumberOfSwapChains() -> usize {
    0
}

/// UpdateSurface - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateSurface(pSourceSurface: *mut core::ffi::c_void, pSourceRect: *mut core::ffi::c_void, pDestinationSurface: *mut core::ffi::c_void, pDestPoint: *mut core::ffi::c_void) -> usize {
    0
}

/// GetRenderTargetData - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetRenderTargetData(pRenderTarget: *mut core::ffi::c_void, pDestSurface: *mut core::ffi::c_void) -> usize {
    0
}

/// GetFrontBufferData - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetFrontBufferData(iSwapChain: u32, pDestSurface: *mut core::ffi::c_void) -> usize {
    0
}

/// StretchRect - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn StretchRect(pSourceSurface: *mut core::ffi::c_void, pSourceRect: *mut core::ffi::c_void, pDestSurface: *mut core::ffi::c_void, pDestRect: *mut core::ffi::c_void, Filter: usize) -> usize {
    0
}

/// ColorFill - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn ColorFill(pSurface: *mut core::ffi::c_void, pRect: *mut core::ffi::c_void, Color: usize) -> usize {
    0
}

/// SetDepthStencilSurface - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetDepthStencilSurface(pNewZStencil: *mut core::ffi::c_void) -> usize {
    0
}

/// GetSamplerState - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetSamplerState(Sampler: u32, Type: usize, pValue: *mut u32) -> usize {
    0
}

/// SetSamplerState - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetSamplerState(Sampler: u32, Type: usize, Value: u32) -> usize {
    0
}

/// SetScissorRect - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetScissorRect(pRect: *mut core::ffi::c_void) -> usize {
    0
}

/// GetScissorRect - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetScissorRect(pRect: *mut core::ffi::c_void) -> usize {
    0
}

/// SetNPatchMode - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetNPatchMode(nSegments: f32) -> usize {
    0
}

/// GetNPatchMode - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetNPatchMode() -> usize {
    0
}

/// SetVertexDeclaration - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetVertexDeclaration(pDecl: *mut core::ffi::c_void) -> usize {
    0
}

/// GetVertexDeclaration - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetVertexDeclaration(ppDecl: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SetFVF - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetFVF(FVF: u32) -> usize {
    0
}

/// GetFVF - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetFVF(pFVF: *mut u32) -> usize {
    0
}

/// SetStreamSourceFreq - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetStreamSourceFreq(StreamNumber: u32, Setting: u32) -> usize {
    0
}

/// GetStreamSourceFreq - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetStreamSourceFreq(StreamNumber: u32, pSetting: *mut u32) -> usize {
    0
}

/// SetConvolutionMonoKernel - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetConvolutionMonoKernel(width: u32, height: u32, rows: *mut f32, columns: *mut f32) -> usize {
    0
}

/// ComposeRects - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn ComposeRects(pSrc: *mut core::ffi::c_void, pDst: *mut core::ffi::c_void, pSrcRectDescs: *mut core::ffi::c_void, NumRects: u32, pDstRectDescs: *mut core::ffi::c_void, Operation: usize, Xoffset: i32, Yoffset: i32) -> usize {
    0
}

/// WaitForVBlank - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn WaitForVBlank(iSwapChain: u32) -> usize {
    0
}

/// CheckResourceResidency - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn CheckResourceResidency(pResourceArray: *mut *mut core::ffi::c_void, NumResources: usize) -> usize {
    0
}

/// CheckDeviceState - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn CheckDeviceState(hDestinationWindow: *mut core::ffi::c_void) -> usize {
    0
}

/// PresentEx - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn PresentEx(pSourceRect: *mut core::ffi::c_void, pDestRect: *mut core::ffi::c_void, hDestWindowOverride: *mut core::ffi::c_void, pDirtyRegion: *mut core::ffi::c_void, dwFlags: u32) -> usize {
    0
}

/// ResetEx - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn ResetEx(pPresentationParameters: *mut core::ffi::c_void, pFullscreenDisplayMode: *mut core::ffi::c_void) -> usize {
    0
}

/// GetDisplayModeEx - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetDisplayModeEx(iSwapChain: u32, pMode: *mut core::ffi::c_void, pRotation: *mut core::ffi::c_void) -> usize {
    0
}

/// SetStateSamplerState - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetStateSamplerState(StateSampler: u32, Type: usize, Value: u32) -> i32 {
    0
}

/// SetStateTexture - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetStateTexture(StateSampler: u32, pTexture: *mut core::ffi::c_void) -> i32 {
    0
}

/// SetStateTransform - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetStateTransform(idx: u32, pMatrix: *mut core::ffi::c_void) -> i32 {
    0
}

/// SetStateTextureStageState - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetStateTextureStageState(Stage: u32, Type: usize, Value: u32) -> i32 {
    0
}

/// SupportsSWVP - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SupportsSWVP() -> usize {
    0
}

/// SupportsVCacheQuery - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SupportsVCacheQuery() -> usize {
    0
}

/// UnsupportedFormatInfo - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UnsupportedFormatInfo(Format: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// FlushImage - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn FlushImage(pResource: *mut core::ffi::c_void, Subresource: u32) -> i32 {
    0
}

/// EmitGenerateMips - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn EmitGenerateMips(pResource: *mut core::ffi::c_void) {

}

/// FlushBuffer - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn FlushBuffer(pResource: *mut core::ffi::c_void) -> i32 {
    0
}

/// UploadPerDrawData - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UploadPerDrawData(FirstVertexIndex: usize, NumVertices: u32, FirstIndex: usize, NumIndices: u32, BaseVertexIndex: usize, pDynamicVBOs: *mut core::ffi::c_void, pDynamicIBO: *mut core::ffi::c_void) {

}

/// SetupFPU - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetupFPU() {

}

/// DetermineInitialTextureMemory - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn DetermineInitialTextureMemory() -> i64 {
    0
}

/// FlushAndSync9On12 - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn FlushAndSync9On12() {

}

/// BeginFrame - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BeginFrame(LatencyTracker: usize, FrameId: u64) {

}

/// UpdateTextureBitmasks - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateTextureBitmasks(index: u32, combinedUsage: u32) {

}

/// UpdateActiveHazardsRT - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateActiveHazardsRT(texMask: u32) {

}

/// UpdateActiveHazardsDS - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateActiveHazardsDS(texMask: u32) {

}

/// EmitFeedbackLoopBarriers - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn EmitFeedbackLoopBarriers() {

}

/// UpdateActiveFetch4 - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateActiveFetch4(stateSampler: u32) {

}

/// UpdateTextureTypeMismatchesForShader - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateTextureTypeMismatchesForShader(shader: *mut core::ffi::c_void, shaderSamplerMask: u32, shaderSamplerOffset: u32) {

}

/// UpdateTextureTypeMismatchesForTexture - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateTextureTypeMismatchesForTexture(stateSampler: u32) {

}

/// UploadManagedTexture - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UploadManagedTexture(pResource: *mut core::ffi::c_void) {

}

/// UploadManagedTextures - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UploadManagedTextures(mask: u32) {

}

/// GenerateTextureMips - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GenerateTextureMips(mask: u32) {

}

/// MarkTextureMipsDirty - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn MarkTextureMipsDirty(pResource: *mut core::ffi::c_void) {

}

/// MarkTextureMipsUnDirty - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn MarkTextureMipsUnDirty(pResource: *mut core::ffi::c_void) {

}

/// MarkTextureUploaded - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn MarkTextureUploaded(pResource: *mut core::ffi::c_void) {

}

/// UpdatePointMode - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdatePointMode(pointList: usize) {

}

/// UpdateFog - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateFog() {

}

/// IsNVDepthBoundsTestEnabled - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn IsNVDepthBoundsTestEnabled() -> usize {
    0
}

/// UpdateAlphaToCoverangeAndAlphaTest - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateAlphaToCoverangeAndAlphaTest() {

}

/// IsZTestEnabled - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn IsZTestEnabled() -> usize {
    0
}

/// BindMultiSampleState - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BindMultiSampleState() {

}

/// BindBlendState - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BindBlendState() {

}

/// BindBlendFactor - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BindBlendFactor() {

}

/// BindDepthStencilState - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BindDepthStencilState() {

}

/// BindDepthStencilReference - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BindDepthStencilReference() {

}

/// BindRasterizerState - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BindRasterizerState() {

}

/// BindDepthBias - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BindDepthBias() {

}

/// UpdateClipPlanes - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateClipPlanes() {

}

/// BindTexture - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BindTexture(SamplerSampler: u32) {

}

/// UnbindTextures - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UnbindTextures(mask: u32) {

}

/// UndirtySamplers - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UndirtySamplers(mask: u32) {

}

/// UndirtyTextures - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UndirtyTextures(usedMask: u32) {

}

/// MarkTextureBindingDirty - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn MarkTextureBindingDirty(texture: *mut core::ffi::c_void) {

}

/// SamplerUsesBorderColor - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SamplerUsesBorderColor(Sampler: u32) -> usize {
    0
}

/// SetRenderTargetInternal - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetRenderTargetInternal(RenderTargetIndex: u32, pRenderTarget: *mut core::ffi::c_void) -> usize {
    0
}

/// GenerateDrawInfo - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GenerateDrawInfo(PrimitiveType: usize, PrimitiveCount: u32, InstanceCount: u32) -> usize {
    0
}

/// PrepareDraw - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn PrepareDraw(PrimitiveType: usize, UploadVBOs: usize, UploadIBOs: usize) {

}

/// EnsureSamplerLimit - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn EnsureSamplerLimit() {

}

/// BindFFUbershader - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BindFFUbershader() {

}

/// BindInputLayout - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BindInputLayout() {

}

/// BindIndices - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BindIndices() {

}

/// GetRawState - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetRawState() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SetVertexBoolBitfield - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetVertexBoolBitfield(idx: u32, mask: u32, bits: u32) {

}

/// SetPixelBoolBitfield - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetPixelBoolBitfield(idx: u32, mask: u32, bits: u32) {

}

/// ChangeReportedMemory - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn ChangeReportedMemory(delta: i64) -> usize {
    0
}

/// ResolveZ - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn ResolveZ() {

}

/// TransformImage - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn TransformImage(pResource: *mut core::ffi::c_void, pSubresources: *mut core::ffi::c_void, OldLayout: usize, NewLayout: usize) {

}

/// ResetSwapChain - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn ResetSwapChain(pPresentationParameters: *mut core::ffi::c_void, pFullscreenDisplayMode: *mut core::ffi::c_void) -> i32 {
    0
}

/// InitialReset - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn InitialReset(pPresentationParameters: *mut core::ffi::c_void, pFullscreenDisplayMode: *mut core::ffi::c_void) -> i32 {
    0
}

/// MapTexture - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn MapTexture(pTexture: *mut core::ffi::c_void, Subresource: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// TouchMappedTexture - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn TouchMappedTexture(pTexture: *mut core::ffi::c_void) {

}

/// RemoveMappedTexture - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn RemoveMappedTexture(pTexture: *mut core::ffi::c_void) {

}

/// IsDeviceLost - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn IsDeviceLost() -> usize {
    0
}

/// NotifyFullscreen - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn NotifyFullscreen(window: *mut core::ffi::c_void, fullscreen: usize) {

}

/// NotifyWindowActivated - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn NotifyWindowActivated(window: *mut core::ffi::c_void, activated: usize) {

}

/// IncrementLosableCounter - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn IncrementLosableCounter() {

}

/// DecrementLosableCounter - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn DecrementLosableCounter() {

}

/// CanOnlySWVP - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn CanOnlySWVP() -> usize {
    0
}

/// IsSWVP - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn IsSWVP() -> usize {
    0
}

/// GetFixedFunctionVSCount - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetFixedFunctionVSCount() -> u32 {
    0
}

/// GetFixedFunctionFSCount - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetFixedFunctionFSCount() -> u32 {
    0
}

/// GetSWVPShaderCount - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetSWVPShaderCount() -> u32 {
    0
}

/// Is9On12Device - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn Is9On12Device() -> usize {
    0
}

/// GetResetCounter - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetResetCounter() -> u32 {
    0
}

/// GetUPDataSize - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetUPDataSize(vertexCount: u32, stride: u32) -> u32 {
    0
}

/// GetUPBufferSize - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetUPBufferSize(vertexCount: u32, stride: u32) -> u32 {
    0
}

/// FillUPVertexBuffer - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn FillUPVertexBuffer(buffer: *mut core::ffi::c_void, userData: *mut core::ffi::c_void, dataSize: u32, bufferSize: u32) {

}

/// DetermineSoftwareRegCount - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn DetermineSoftwareRegCount() -> usize {
    0
}

/// DetermineHardwareRegCount - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn DetermineHardwareRegCount() -> u32 {
    0
}

/// UpdateFixedFunctionVS - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateFixedFunctionVS() {

}

/// UpdateFixedFunctionPS - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateFixedFunctionPS() {

}

/// ApplyPrimitiveType - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn ApplyPrimitiveType(pContext: *mut core::ffi::c_void, PrimType: usize) {

}

/// UseProgrammableVS - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UseProgrammableVS() -> usize {
    0
}

/// UseProgrammablePS - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UseProgrammablePS() -> usize {
    0
}

/// GetAlphaTestPrecision - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetAlphaTestPrecision() -> u32 {
    0
}

/// BindAlphaTestState - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BindAlphaTestState() {

}

/// UpdateAlphaTestSpec - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateAlphaTestSpec(alphaOp: usize, precision: u32) {

}

/// UpdateVertexBoolSpec - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateVertexBoolSpec(value: u32) {

}

/// UpdatePixelBoolSpec - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdatePixelBoolSpec(value: u32) {

}

/// UpdatePixelShaderSamplerSpec - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdatePixelShaderSamplerSpec(types: u32, fetch4: u32) {

}

/// UpdateCommonSamplerSpec - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateCommonSamplerSpec(boundMask: u32, depthMask: u32, drefMask: u32, projections: u32) {

}

/// UpdatePointModeSpec - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdatePointModeSpec(mode: u32) {

}

/// UpdateFogModeSpec - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateFogModeSpec(fogEnabled: usize, vertexFogMode: usize, pixelFogMode: usize) {

}

/// BuildFFKeyVS - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BuildFFKeyVS(vertexBlendMode: usize, indexedVertexBlend: usize) -> usize {
    0
}

/// BuildFFKeyFS - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BuildFFKeyFS() -> usize {
    0
}

/// TrackBufferMappingBufferSequenceNumber - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn TrackBufferMappingBufferSequenceNumber(pResource: *mut core::ffi::c_void) {

}

/// TrackTextureMappingBufferSequenceNumber - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn TrackTextureMappingBufferSequenceNumber(pResource: *mut core::ffi::c_void, Subresource: u32) {

}

/// UnmapTextures - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UnmapTextures() {

}

/// GetMostRecentlyUsedSwapchain - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetMostRecentlyUsedSwapchain() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SetMostRecentlyUsedSwapchain - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetMostRecentlyUsedSwapchain(swapchain: *mut core::ffi::c_void) {

}

/// ResetMostRecentlyUsedSwapchain - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn ResetMostRecentlyUsedSwapchain() {

}

/// IsTextureBoundAsAttachment - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn IsTextureBoundAsAttachment(pTexture: *mut core::ffi::c_void) -> usize {
    0
}

/// HasRenderTargetBound - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn HasRenderTargetBound(Index: u32) -> usize {
    0
}

/// ValidateSharedTexture - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn ValidateSharedTexture(handle: *mut core::ffi::c_void, arg1: usize, textureDesc: usize) -> usize {
    0
}

/// ValidateSharedBuffer - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn ValidateSharedBuffer(handle: *mut core::ffi::c_void, bufferDesc: usize) -> usize {
    0
}

/// InitShaderOptions - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn InitShaderOptions() {

}

/// GetGlobalSamplerSetIndex - from dxvk/d3d9_fixed_function.h
#[no_mangle]
pub unsafe extern "C" fn GetGlobalSamplerSetIndex() -> usize {
    0
}

/// GetPushSamplerOffset - from dxvk/d3d9_fixed_function.h
#[no_mangle]
pub unsafe extern "C" fn GetPushSamplerOffset(samplerIndex: u32) -> usize {
    0
}

/// DoFixedFunctionFog - from dxvk/d3d9_fixed_function.h
#[no_mangle]
pub unsafe extern "C" fn DoFixedFunctionFog(spec: usize, spvModule: usize, fogCtx: usize) -> u32 {
    0
}

/// DoFixedFunctionAlphaTest - from dxvk/d3d9_fixed_function.h
#[no_mangle]
pub unsafe extern "C" fn DoFixedFunctionAlphaTest(spvModule: usize, ctx: usize) {

}

/// SetupSamplerArray - from dxvk/d3d9_fixed_function.h
#[no_mangle]
pub unsafe extern "C" fn SetupSamplerArray(spvModule: usize) -> u32 {
    0
}

/// LoadSampler - from dxvk/d3d9_fixed_function.h
#[no_mangle]
pub unsafe extern "C" fn LoadSampler(spvModule: usize, descriptorId: u32, pushBlockId: u32, pushMember: u32, samplerIndex: u32) -> u32 {
    0
}

/// GetPointSizeInfoVS - from dxvk/d3d9_fixed_function.h
#[no_mangle]
pub unsafe extern "C" fn GetPointSizeInfoVS(spec: usize, spvModule: usize, vPos: u32, vtx: u32, perVertPointSize: u32, rsBlock: u32, specUbo: u32, isFixedFunction: usize) -> usize {
    0
}

/// GetPointSizeInfoPS - from dxvk/d3d9_fixed_function.h
#[no_mangle]
pub unsafe extern "C" fn GetPointSizeInfoPS(spec: usize, spvModule: usize, rsBlock: u32, specUbo: u32) -> usize {
    0
}

/// GetPointCoord - from dxvk/d3d9_fixed_function.h
#[no_mangle]
pub unsafe extern "C" fn GetPointCoord(spvModule: usize) -> u32 {
    0
}

/// SetupSpecUBO - from dxvk/d3d9_fixed_function.h
#[no_mangle]
pub unsafe extern "C" fn SetupSpecUBO(spvModule: usize, bindings: usize) -> u32 {
    0
}

/// Dump - from dxvk/d3d9_fixed_function.h
#[no_mangle]
pub unsafe extern "C" fn Dump(pDevice: *mut core::ffi::c_void, Key: usize, Name: usize) {

}

/// GetVSCount - from dxvk/d3d9_fixed_function.h
#[no_mangle]
pub unsafe extern "C" fn GetVSCount() -> u32 {
    0
}

/// GetFSCount - from dxvk/d3d9_fixed_function.h
#[no_mangle]
pub unsafe extern "C" fn GetFSCount() -> u32 {
    0
}

/// EnumerateFormat - from dxvk/d3d9_format.h
#[no_mangle]
pub unsafe extern "C" fn EnumerateFormat(format: usize) -> usize {
    0
}

/// ConvertFormatUnfixed - from dxvk/d3d9_format.h
#[no_mangle]
pub unsafe extern "C" fn ConvertFormatUnfixed(Format: usize) -> usize {
    0
}

/// RefreshFormatSupport - from dxvk/d3d9_format.h
#[no_mangle]
pub unsafe extern "C" fn RefreshFormatSupport(pParent: *mut core::ffi::c_void) {

}

/// CheckImageFormatSupport - from dxvk/d3d9_format.h
#[no_mangle]
pub unsafe extern "C" fn CheckImageFormatSupport(Adapter: usize, Format: usize, Features: usize) -> usize {
    0
}

/// IsFourCCFormat - from dxvk/d3d9_format.h
#[no_mangle]
pub unsafe extern "C" fn IsFourCCFormat(format: usize) -> usize {
    0
}

/// IsDepthStencilFormat - from dxvk/d3d9_format.h
#[no_mangle]
pub unsafe extern "C" fn IsDepthStencilFormat(Format: usize) -> usize {
    0
}

/// IsDepthFormat - from dxvk/d3d9_format.h
#[no_mangle]
pub unsafe extern "C" fn IsDepthFormat(arg0: usize) -> usize {
    0
}

/// ConvertFormat - from dxvk/d3d9_format_helpers.h
#[no_mangle]
pub unsafe extern "C" fn ConvertFormat(ctx: usize, conversionFormat: usize, dstImage: usize, dstSubresource: usize, srcSlice: usize) {

}

/// ConvertGenericFormat - from dxvk/d3d9_format_helpers.h
#[no_mangle]
pub unsafe extern "C" fn ConvertGenericFormat(ctx: usize, videoFormat: usize, dstImage: usize, dstSubresource: usize, srcSlice: usize, bufferFormat: usize, macroPixelRun: usize) {

}

/// InitPipelines - from dxvk/d3d9_format_helpers.h
#[no_mangle]
pub unsafe extern "C" fn InitPipelines() {

}

/// render - from dxvk/d3d9_hud.h
#[no_mangle]
pub unsafe extern "C" fn render(arg0: usize, key: usize, options: usize, renderer: usize, position: usize) -> usize {
    0
}

/// UnwrapUnderlyingResource - from dxvk/d3d9_include.h
#[no_mangle]
pub unsafe extern "C" fn UnwrapUnderlyingResource(resource: *mut core::ffi::c_void, command_queue: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// ReturnUnderlyingResource - from dxvk/d3d9_include.h
#[no_mangle]
pub unsafe extern "C" fn ReturnUnderlyingResource(resource: *mut core::ffi::c_void, num_sync: u32, signal_values: *mut core::ffi::c_void, fences: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// ValidatePresentationParametersEx - from dxvk/d3d9_interface.h
#[no_mangle]
pub unsafe extern "C" fn ValidatePresentationParametersEx(pPresentationParameters: *mut core::ffi::c_void, pFullscreenDisplayMode: *mut core::ffi::c_void) -> i32 {
    0
}

/// EnableAdditionalFormats - from dxvk/d3d9_interface.h
#[no_mangle]
pub unsafe extern "C" fn EnableAdditionalFormats() {

}

/// RefreshAdapterFormatTables - from dxvk/d3d9_interface.h
#[no_mangle]
pub unsafe extern "C" fn RefreshAdapterFormatTables() {

}

/// Find9On12Args - from dxvk/d3d9_interface.h
#[no_mangle]
pub unsafe extern "C" fn Find9On12Args(Adapter: usize, pOverrides: *mut core::ffi::c_void, OverrideCount: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// GetPhysicalDeviceHandle - from dxvk/d3d9_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn GetPhysicalDeviceHandle(Adapter: u32, pPhysicalDevice: *mut core::ffi::c_void) -> usize {
    0
}

/// TransitionTextureLayout - from dxvk/d3d9_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn TransitionTextureLayout(pTexture: *mut core::ffi::c_void, pSubresources: *mut core::ffi::c_void, OldLayout: usize, NewLayout: usize) -> usize {
    0
}

/// GetCurrentOutputDesc - from dxvk/d3d9_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn GetCurrentOutputDesc(pOutputDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// Ptr - from dxvk/d3d9_mem.h
#[no_mangle]
pub unsafe extern "C" fn Ptr() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// GetChunk - from dxvk/d3d9_mem.h
#[no_mangle]
pub unsafe extern "C" fn GetChunk() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// GetOffset - from dxvk/d3d9_mem.h
#[no_mangle]
pub unsafe extern "C" fn GetOffset() -> usize {
    0
}

/// MappedMemory - from dxvk/d3d9_mem.h
#[no_mangle]
pub unsafe extern "C" fn MappedMemory() -> u32 {
    0
}

/// UsedMemory - from dxvk/d3d9_mem.h
#[no_mangle]
pub unsafe extern "C" fn UsedMemory() -> u32 {
    0
}

/// MappingGranularity - from dxvk/d3d9_mem.h
#[no_mangle]
pub unsafe extern "C" fn MappingGranularity() -> u32 {
    0
}

/// GetMonitorFormatBpp - from dxvk/d3d9_monitor.h
#[no_mangle]
pub unsafe extern "C" fn GetMonitorFormatBpp(Format: usize) -> u32 {
    0
}

/// IsSupportedAdapterFormat - from dxvk/d3d9_monitor.h
#[no_mangle]
pub unsafe extern "C" fn IsSupportedAdapterFormat(Format: usize) -> usize {
    0
}

/// IsSupportedModeFormat - from dxvk/d3d9_monitor.h
#[no_mangle]
pub unsafe extern "C" fn IsSupportedModeFormat(Format: usize) -> usize {
    0
}

/// IsSupportedBackBufferFormat - from dxvk/d3d9_monitor.h
#[no_mangle]
pub unsafe extern "C" fn IsSupportedBackBufferFormat(AdapterFormat: usize, BackBufferFormat: usize, Windowed: i32) -> usize {
    0
}

/// ConvertDisplayMode - from dxvk/d3d9_monitor.h
#[no_mangle]
pub unsafe extern "C" fn ConvertDisplayMode(mode: usize) -> usize {
    0
}

/// Issue - from dxvk/d3d9_query.h
#[no_mangle]
pub unsafe extern "C" fn Issue(dwIssueFlags: u32) -> usize {
    0
}

/// GetQueryData - from dxvk/d3d9_query.h
#[no_mangle]
pub unsafe extern "C" fn GetQueryData(pData: *mut core::ffi::c_void, dwSize: u32) -> i32 {
    0
}

/// QueryBeginnable - from dxvk/d3d9_query.h
#[no_mangle]
pub unsafe extern "C" fn QueryBeginnable(QueryType: usize) -> usize {
    0
}

/// QueryEndable - from dxvk/d3d9_query.h
#[no_mangle]
pub unsafe extern "C" fn QueryEndable(QueryType: usize) -> usize {
    0
}

/// QuerySupported - from dxvk/d3d9_query.h
#[no_mangle]
pub unsafe extern "C" fn QuerySupported(pDevice: *mut core::ffi::c_void, QueryType: usize) -> i32 {
    0
}

/// computeTextureBinding - from dxvk/d3d9_shader.h
#[no_mangle]
pub unsafe extern "C" fn computeTextureBinding(shaderType: usize, index: u32) -> usize {
    0
}

/// getSWVPBufferSlot - from dxvk/d3d9_shader.h
#[no_mangle]
pub unsafe extern "C" fn getSWVPBufferSlot() -> usize {
    0
}

/// GetShaderMask - from dxvk/d3d9_shader.h
#[no_mangle]
pub unsafe extern "C" fn GetShaderMask() -> usize {
    0
}

/// GetImageViewType - from dxvk/d3d9_shader.h
#[no_mangle]
pub unsafe extern "C" fn GetImageViewType(samplerSlot: u32) -> usize {
    0
}

/// Instruction - from dxvk/d3d9_shader_validator.h
#[no_mangle]
pub unsafe extern "C" fn Instruction(pFile: *mut i8, Line: u32, pdwInst: *mut u32, cdw: u32) -> usize {
    0
}

/// ValidateHeader - from dxvk/d3d9_shader_validator.h
#[no_mangle]
pub unsafe extern "C" fn ValidateHeader(pFile: *mut i8, Line: u32, pdwInst: *mut u32, cdw: u32) -> i32 {
    0
}

/// ValidateEndToken - from dxvk/d3d9_shader_validator.h
#[no_mangle]
pub unsafe extern "C" fn ValidateEndToken(pFile: *mut i8, Line: u32, pdwInst: *mut u32, cdw: u32) -> i32 {
    0
}

/// ErrorCallback - from dxvk/d3d9_shader_validator.h
#[no_mangle]
pub unsafe extern "C" fn ErrorCallback(pFile: *mut i8, Line: u32, Unknown: u32, pInstr: *mut u32, InstrLength: u32, MessageID: usize, Message: usize) -> i32 {
    0
}

/// mask - from dxvk/d3d9_spec_constants.h
#[no_mangle]
pub unsafe extern "C" fn mask() -> usize {
    0
}

/// set - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn set(zu: usize, p: usize) -> usize {
    0
}

/// getSpecUBODword - from dxvk/d3d9_spec_constants.h
#[no_mangle]
pub unsafe extern "C" fn getSpecUBODword(module: usize, specUbo: u32, idx: u32) -> u32 {
    0
}

/// getOptimizedBool - from dxvk/d3d9_spec_constants.h
#[no_mangle]
pub unsafe extern "C" fn getOptimizedBool(module: usize) -> u32 {
    0
}

/// ensure - from dxvk/d3d9_state.h
#[no_mangle]
pub unsafe extern "C" fn ensure() {

}

/// IsLightEnabled - from dxvk/d3d9_state.h
#[no_mangle]
pub unsafe extern "C" fn IsLightEnabled(Index: u32) -> usize {
    0
}

/// SetStreamSourceWithoutOffset - from dxvk/d3d9_stateblock.h
#[no_mangle]
pub unsafe extern "C" fn SetStreamSourceWithoutOffset(StreamNumber: u32, pStreamData: *mut core::ffi::c_void, Stride: u32) -> i32 {
    0
}

/// CapturePixelRenderStates - from dxvk/d3d9_stateblock.h
#[no_mangle]
pub unsafe extern "C" fn CapturePixelRenderStates() {

}

/// CapturePixelSamplerStates - from dxvk/d3d9_stateblock.h
#[no_mangle]
pub unsafe extern "C" fn CapturePixelSamplerStates() {

}

/// CapturePixelShaderStates - from dxvk/d3d9_stateblock.h
#[no_mangle]
pub unsafe extern "C" fn CapturePixelShaderStates() {

}

/// CaptureVertexRenderStates - from dxvk/d3d9_stateblock.h
#[no_mangle]
pub unsafe extern "C" fn CaptureVertexRenderStates() {

}

/// CaptureVertexSamplerStates - from dxvk/d3d9_stateblock.h
#[no_mangle]
pub unsafe extern "C" fn CaptureVertexSamplerStates() {

}

/// CaptureVertexShaderStates - from dxvk/d3d9_stateblock.h
#[no_mangle]
pub unsafe extern "C" fn CaptureVertexShaderStates() {

}

/// CaptureType - from dxvk/d3d9_stateblock.h
#[no_mangle]
pub unsafe extern "C" fn CaptureType(State: usize) {

}

/// GetFace - from dxvk/d3d9_subresource.h
#[no_mangle]
pub unsafe extern "C" fn GetFace() -> u32 {
    0
}

/// GetMipLevel - from dxvk/d3d9_subresource.h
#[no_mangle]
pub unsafe extern "C" fn GetMipLevel() -> u32 {
    0
}

/// GetSurfaceExtent - from dxvk/d3d9_surface.h
#[no_mangle]
pub unsafe extern "C" fn GetSurfaceExtent() -> usize {
    0
}

/// ClearContainer - from dxvk/d3d9_surface.h
#[no_mangle]
pub unsafe extern "C" fn ClearContainer() {

}

/// PresentImageGDI - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn PresentImageGDI(Window: *mut core::ffi::c_void) -> i32 {
    0
}

/// GetPresentParameters - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetPresentParameters(pPresentationParameters: *mut core::ffi::c_void) -> usize {
    0
}

/// GetPresentStats - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetPresentStats(pPresentationStatistics: *mut core::ffi::c_void) -> usize {
    0
}

/// Invalidate - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn Invalidate(hWindow: *mut core::ffi::c_void) {

}

/// SetCursorTexture - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn SetCursorTexture(Width: u32, Height: u32, pCursorBitmap: *mut u8) {

}

/// GetPresentParams - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetPresentParams() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// DestroyBackBuffers - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn DestroyBackBuffers() {

}

/// UpdateWindowCtx - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn UpdateWindowCtx() -> usize {
    0
}

/// InitRamp - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn InitRamp() {

}

/// UpdateTargetFrameRate - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn UpdateTargetFrameRate(SyncInterval: u32) {

}

/// NormalizePresentParameters - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn NormalizePresentParameters(pPresentParams: *mut core::ffi::c_void) {

}

/// UpdateWindowedRefreshRate - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn UpdateWindowedRefreshRate() {

}

/// EnterFullscreenMode - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn EnterFullscreenMode(pPresentParams: *mut core::ffi::c_void, pFullscreenDisplayMode: *mut core::ffi::c_void) -> i32 {
    0
}

/// LeaveFullscreenMode - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn LeaveFullscreenMode() -> i32 {
    0
}

/// ChangeDisplayMode - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn ChangeDisplayMode(pPresentParams: *mut core::ffi::c_void, pFullscreenDisplayMode: *mut core::ffi::c_void) -> i32 {
    0
}

/// RestoreDisplayMode - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn RestoreDisplayMode(hMonitor: usize) -> i32 {
    0
}

/// UpdatePresentRegion - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn UpdatePresentRegion(pSourceRect: *mut core::ffi::c_void, pDestRect: *mut core::ffi::c_void) {

}

/// UpdatePresentParameters - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn UpdatePresentParameters() {

}

/// GetPresentExtent - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetPresentExtent() -> usize {
    0
}

/// IsDeviceReset - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn IsDeviceReset(wctx: *mut core::ffi::c_void) -> usize {
    0
}

/// SwapWithFrontBuffer - from dxvk/d3d9_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn SwapWithFrontBuffer() -> usize {
    0
}

/// GetShaderCount - from dxvk/d3d9_swvp_emu.h
#[no_mangle]
pub unsafe extern "C" fn GetShaderCount() -> u32 {
    0
}

/// SetAutoGenFilterType - from dxvk/d3d9_texture.h
#[no_mangle]
pub unsafe extern "C" fn SetAutoGenFilterType(FilterType: usize) -> usize {
    0
}

/// GetAutoGenFilterType - from dxvk/d3d9_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetAutoGenFilterType() -> usize {
    0
}

/// GenerateMipSubLevels - from dxvk/d3d9_texture.h
#[no_mangle]
pub unsafe extern "C" fn GenerateMipSubLevels() -> usize {
    0
}

/// TextureRefPrivate - from dxvk/d3d9_texture.h
#[no_mangle]
pub unsafe extern "C" fn TextureRefPrivate(tex: *mut core::ffi::c_void, AddRef: usize) {

}

/// TextureChangePrivate - from dxvk/d3d9_texture.h
#[no_mangle]
pub unsafe extern "C" fn TextureChangePrivate(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void) {

}

/// FixupBlendState - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn FixupBlendState(State: usize) {

}

/// InvalidSampler - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn InvalidSampler(Sampler: u32) -> usize {
    0
}

/// RemapSamplerState - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn RemapSamplerState(Sampler: u32) -> u32 {
    0
}

/// IsVSSampler - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn IsVSSampler(Sampler: u32) -> usize {
    0
}

/// IsPSSampler - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn IsPSSampler(Sampler: u32) -> usize {
    0
}

/// RemapStateSamplerShader - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn RemapStateSamplerShader(arg0: usize) -> usize {
    0
}

/// CastRefPrivate - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn CastRefPrivate(ptr: *mut core::ffi::c_void, AddRef: usize) {

}

/// DisassembleShader - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn DisassembleShader(pShader: *mut core::ffi::c_void, EnableColorCode: i32, pComments: *mut i8, ppDisassembly: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// DecodeMultiSampleType - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn DecodeMultiSampleType(MultiSample: usize, MultisampleQuality: u32, pSampleCount: *mut core::ffi::c_void) -> i32 {
    0
}

/// GetImageUsageFlags - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn GetImageUsageFlags(Usage: u32) -> usize {
    0
}

/// PickSRGB - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn PickSRGB(format: usize, srgbFormat: usize, srgb: usize) -> usize {
    0
}

/// GetShaderStage - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn GetShaderStage(ShaderType: usize) -> usize {
    0
}

/// GetTransformIndex - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn GetTransformIndex(Type: usize) -> u32 {
    0
}

/// ConvertMatrix - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn ConvertMatrix(Matrix: *mut core::ffi::c_void) -> usize {
    0
}

/// Matrix4 - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn Matrix4() -> usize {
    0
}

/// GetVertexCount - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn GetVertexCount(arg0: usize, count: u32) -> u32 {
    0
}

/// DecodeInputAssemblyState - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn DecodeInputAssemblyState(arg0: usize) -> usize {
    0
}

/// DecodeFilter - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn DecodeFilter(Filter: usize) -> usize {
    0
}

/// DecodeMipFilter - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn DecodeMipFilter(Filter: usize) -> usize {
    0
}

/// VkSamplerAddressMode - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn VkSamplerAddressMode(arg0: usize) -> usize {
    0
}

/// DecodeCullMode - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn DecodeCullMode(Mode: usize) -> usize {
    0
}

/// DecodeFillMode - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn DecodeFillMode(Mode: usize) -> usize {
    0
}

/// DecodeIndexType - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn DecodeIndexType(Format: usize) -> usize {
    0
}

/// DecodeDecltype - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn DecodeDecltype(Type: usize) -> usize {
    0
}

/// GetDecltypeSize - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn GetDecltypeSize(Type: usize) -> u32 {
    0
}

/// GetDecltypeCount - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn GetDecltypeCount(Type: usize) -> u32 {
    0
}

/// ConvertBox - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn ConvertBox(arg0: usize, offset: usize, extent: usize) {

}

/// D3DRENDERSTATETYPE - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn D3DRENDERSTATETYPE(arg0: usize) -> usize {
    0
}

/// RemapTextureStageStateType - from dxvk/d3d9_util.h
#[no_mangle]
pub unsafe extern "C" fn RemapTextureStageStateType(Type: usize) -> usize {
    0
}

/// GetDeclaration - from dxvk/d3d9_vertex_declaration.h
#[no_mangle]
pub unsafe extern "C" fn GetDeclaration(pElement: *mut core::ffi::c_void, pNumElements: *mut u32) -> usize {
    0
}

/// TestFlag - from dxvk/d3d9_vertex_declaration.h
#[no_mangle]
pub unsafe extern "C" fn TestFlag(flag: usize) -> usize {
    0
}

/// GetTexcoordMask - from dxvk/d3d9_vertex_declaration.h
#[no_mangle]
pub unsafe extern "C" fn GetTexcoordMask() -> u32 {
    0
}

/// GetStreamMask - from dxvk/d3d9_vertex_declaration.h
#[no_mangle]
pub unsafe extern "C" fn GetStreamMask() -> u32 {
    0
}

/// MapD3DDeclToFvf - from dxvk/d3d9_vertex_declaration.h
#[no_mangle]
pub unsafe extern "C" fn MapD3DDeclToFvf(element: usize, fvf: u32, outFvf: usize, texCountPostUpdate: usize) -> usize {
    0
}

/// MapD3DDeclTypeFloatToFvfXYZBn - from dxvk/d3d9_vertex_declaration.h
#[no_mangle]
pub unsafe extern "C" fn MapD3DDeclTypeFloatToFvfXYZBn(arg0: u8) -> u32 {
    0
}

/// MapD3DDeclUsageTexCoordToFvfTexCoordSize - from dxvk/d3d9_vertex_declaration.h
#[no_mangle]
pub unsafe extern "C" fn MapD3DDeclUsageTexCoordToFvfTexCoordSize(element: usize, fvf: u32, outFvf: usize, texCountPostUpdate: usize) -> usize {
    0
}

/// Classify - from dxvk/d3d9_vertex_declaration.h
#[no_mangle]
pub unsafe extern "C" fn Classify() {

}

/// CallCharsetFunction - from dxvk/d3d9_window.h
#[no_mangle]
pub unsafe extern "C" fn CallCharsetFunction(unicode: usize, ascii: usize, isUnicode: usize, args: usize) -> usize {
    0
}

/// ResetWindowProc - from dxvk/d3d9_window.h
#[no_mangle]
pub unsafe extern "C" fn ResetWindowProc(window: *mut core::ffi::c_void) {

}

/// HookWindowProc - from dxvk/d3d9_window.h
#[no_mangle]
pub unsafe extern "C" fn HookWindowProc(window: *mut core::ffi::c_void, swapchain: *mut core::ffi::c_void) {

}

/// SetActivateProcessed - from dxvk/d3d9_window.h
#[no_mangle]
pub unsafe extern "C" fn SetActivateProcessed(window: *mut core::ffi::c_void, processed: usize) {

}

/// ActivateFocusWindow - from dxvk/d3d9_window.h
#[no_mangle]
pub unsafe extern "C" fn ActivateFocusWindow(window: *mut core::ffi::c_void) {

}

/// CheckInterfaceSupport - from dxvk/dxgi_adapter.h
#[no_mangle]
pub unsafe extern "C" fn CheckInterfaceSupport(InterfaceName: usize, pUMDVersion: *mut core::ffi::c_void) -> usize {
    0
}

/// EnumOutputs - from dxvk/dxgi_adapter.h
#[no_mangle]
pub unsafe extern "C" fn EnumOutputs(Output: u32, ppOutput: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetDesc3 - from dxvk/dxgi_adapter.h
#[no_mangle]
pub unsafe extern "C" fn GetDesc3(pDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// QueryVideoMemoryInfo - from dxvk/dxgi_adapter.h
#[no_mangle]
pub unsafe extern "C" fn QueryVideoMemoryInfo(NodeIndex: u32, MemorySegmentGroup: usize, pVideoMemoryInfo: *mut core::ffi::c_void) -> usize {
    0
}

/// SetVideoMemoryReservation - from dxvk/dxgi_adapter.h
#[no_mangle]
pub unsafe extern "C" fn SetVideoMemoryReservation(NodeIndex: u32, MemorySegmentGroup: usize, Reservation: usize) -> usize {
    0
}

/// RegisterHardwareContentProtectionTeardownStatusEvent - from dxvk/dxgi_adapter.h
#[no_mangle]
pub unsafe extern "C" fn RegisterHardwareContentProtectionTeardownStatusEvent(hEvent: *mut core::ffi::c_void, pdwCookie: *mut u32) -> usize {
    0
}

/// RegisterVideoMemoryBudgetChangeNotificationEvent - from dxvk/dxgi_adapter.h
#[no_mangle]
pub unsafe extern "C" fn RegisterVideoMemoryBudgetChangeNotificationEvent(hEvent: *mut core::ffi::c_void, pdwCookie: *mut u32) -> usize {
    0
}

/// UnregisterHardwareContentProtectionTeardownStatus - from dxvk/dxgi_adapter.h
#[no_mangle]
pub unsafe extern "C" fn UnregisterHardwareContentProtectionTeardownStatus(dwCookie: u32) -> usize {
    0
}

/// UnregisterVideoMemoryBudgetChangeNotification - from dxvk/dxgi_adapter.h
#[no_mangle]
pub unsafe extern "C" fn UnregisterVideoMemoryBudgetChangeNotification(dwCookie: u32) -> usize {
    0
}

/// GetDXVKAdapter - from dxvk/dxgi_adapter.h
#[no_mangle]
pub unsafe extern "C" fn GetDXVKAdapter() -> usize {
    0
}

/// GetAdapterDesc - from dxvk/dxgi_adapter.h
#[no_mangle]
pub unsafe extern "C" fn GetAdapterDesc() -> usize {
    0
}

/// GetGlobalHDRState - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn GetGlobalHDRState(pOutColorSpace: *mut core::ffi::c_void, pOutMetadata: *mut core::ffi::c_void) -> usize {
    0
}

/// SetGlobalHDRState - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn SetGlobalHDRState(ColorSpace: usize, pMetadata: *mut core::ffi::c_void) -> usize {
    0
}

/// IsWindowedStereoEnabled - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn IsWindowedStereoEnabled() -> usize {
    0
}

/// EnumAdapters - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn EnumAdapters(Adapter: u32, ppAdapter: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// EnumAdapters1 - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn EnumAdapters1(Adapter: u32, ppAdapter: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// EnumAdapterByLuid - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn EnumAdapterByLuid(AdapterLuid: usize, riid: usize, ppvAdapter: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// EnumAdapterByGpuPreference - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn EnumAdapterByGpuPreference(Adapter: u32, GpuPreference: usize, riid: usize, ppvAdapter: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// EnumWarpAdapter - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn EnumWarpAdapter(riid: usize, ppvAdapter: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetWindowAssociation - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn GetWindowAssociation(pWindowHandle: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetSharedResourceAdapterLuid - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn GetSharedResourceAdapterLuid(hResource: *mut core::ffi::c_void, pLuid: *mut core::ffi::c_void) -> usize {
    0
}

/// MakeWindowAssociation - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn MakeWindowAssociation(WindowHandle: *mut core::ffi::c_void, Flags: u32) -> usize {
    0
}

/// RegisterOcclusionStatusWindow - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn RegisterOcclusionStatusWindow(WindowHandle: *mut core::ffi::c_void, wMsg: u32, pdwCookie: *mut u32) -> usize {
    0
}

/// RegisterStereoStatusEvent - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn RegisterStereoStatusEvent(hEvent: *mut core::ffi::c_void, pdwCookie: *mut u32) -> usize {
    0
}

/// RegisterStereoStatusWindow - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn RegisterStereoStatusWindow(WindowHandle: *mut core::ffi::c_void, wMsg: u32, pdwCookie: *mut u32) -> usize {
    0
}

/// RegisterOcclusionStatusEvent - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn RegisterOcclusionStatusEvent(hEvent: *mut core::ffi::c_void, pdwCookie: *mut u32) -> usize {
    0
}

/// UnregisterStereoStatus - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn UnregisterStereoStatus(dwCookie: u32) -> usize {
    0
}

/// UnregisterOcclusionStatus - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn UnregisterOcclusionStatus(dwCookie: u32) -> usize {
    0
}

/// RegisterAdaptersChangedEvent - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn RegisterAdaptersChangedEvent(hEvent: *mut core::ffi::c_void, pdwCookie: *mut u32) -> usize {
    0
}

/// UnregisterAdaptersChangedEvent - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn UnregisterAdaptersChangedEvent(Cookie: u32) -> usize {
    0
}

/// UseMonitorFallback - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn UseMonitorFallback() -> i32 {
    0
}

/// GetMonitorInfo - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn GetMonitorInfo() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// GlobalHDRState - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn GlobalHDRState() -> usize {
    0
}

/// GetFormatInfo - from dxvk/dxgi_format.h
#[no_mangle]
pub unsafe extern "C" fn GetFormatInfo(Format: usize, Mode: usize) -> usize {
    0
}

/// GetPackedFormatInfo - from dxvk/dxgi_format.h
#[no_mangle]
pub unsafe extern "C" fn GetPackedFormatInfo(Format: usize, Mode: usize) -> usize {
    0
}

/// GetFormatFamily - from dxvk/dxgi_format.h
#[no_mangle]
pub unsafe extern "C" fn GetFormatFamily(Format: usize, Mode: usize) -> usize {
    0
}

/// GetFormatInfoFromMapping - from dxvk/dxgi_format.h
#[no_mangle]
pub unsafe extern "C" fn GetFormatInfoFromMapping(pMapping: *mut core::ffi::c_void, Mode: usize) -> usize {
    0
}

/// GetPackedFormatMapping - from dxvk/dxgi_format.h
#[no_mangle]
pub unsafe extern "C" fn GetPackedFormatMapping(Format: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// RemapDepthFormat - from dxvk/dxgi_format.h
#[no_mangle]
pub unsafe extern "C" fn RemapDepthFormat(Format: usize, Target: usize) {

}

/// RemapColorFormat - from dxvk/dxgi_format.h
#[no_mangle]
pub unsafe extern "C" fn RemapColorFormat(Format: usize, Target: usize, Swizzle: usize) {

}

/// InitMonitorData - from dxvk/dxgi_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn InitMonitorData(hMonitor: usize, pData: *mut core::ffi::c_void) -> usize {
    0
}

/// AcquireMonitorData - from dxvk/dxgi_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn AcquireMonitorData(hMonitor: usize, ppData: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// ReleaseMonitorData - from dxvk/dxgi_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn ReleaseMonitorData() -> usize {
    0
}

/// PuntColorSpace - from dxvk/dxgi_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn PuntColorSpace(ColorSpace: usize) -> usize {
    0
}

/// CurrentColorSpace - from dxvk/dxgi_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn CurrentColorSpace() -> usize {
    0
}

/// DefaultColorSpace - from dxvk/dxgi_monitor.h
#[no_mangle]
pub unsafe extern "C" fn DefaultColorSpace() -> usize {
    0
}

/// GammaControlPointLocation - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn GammaControlPointLocation(CpIndex: u32) -> f32 {
    0.0
}

/// FindClosestMatchingMode - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn FindClosestMatchingMode(pModeToMatch: *mut core::ffi::c_void, pClosestMatch: *mut core::ffi::c_void, pConcernedDevice: *mut core::ffi::c_void) -> usize {
    0
}

/// FindClosestMatchingMode1 - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn FindClosestMatchingMode1(pModeToMatch: *mut core::ffi::c_void, pClosestMatch: *mut core::ffi::c_void, pConcernedDevice: *mut core::ffi::c_void) -> usize {
    0
}

/// GetDisplayModeList - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn GetDisplayModeList(EnumFormat: usize, Flags: u32, pNumModes: *mut u32, pDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// GetDisplayModeList1 - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn GetDisplayModeList1(EnumFormat: usize, Flags: u32, pNumModes: *mut u32, pDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// GetDisplaySurfaceData - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn GetDisplaySurfaceData(pDestination: *mut core::ffi::c_void) -> usize {
    0
}

/// GetDisplaySurfaceData1 - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn GetDisplaySurfaceData1(pDestination: *mut core::ffi::c_void) -> usize {
    0
}

/// GetGammaControl - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn GetGammaControl(pArray: *mut core::ffi::c_void) -> usize {
    0
}

/// GetGammaControlCapabilities - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn GetGammaControlCapabilities(pGammaCaps: *mut core::ffi::c_void) -> usize {
    0
}

/// ReleaseOwnership - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn ReleaseOwnership() -> usize {
    0
}

/// SetDisplaySurface - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn SetDisplaySurface(pScanoutSurface: *mut core::ffi::c_void) -> usize {
    0
}

/// TakeOwnership - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn TakeOwnership(pDevice: *mut core::ffi::c_void, Exclusive: i32) -> usize {
    0
}

/// DuplicateOutput - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn DuplicateOutput(pDevice: *mut core::ffi::c_void, ppOutputDuplication: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// DuplicateOutput1 - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn DuplicateOutput1(pDevice: *mut core::ffi::c_void, Flags: u32, SupportedFormatsCount: u32, pSupportedFormats: *mut core::ffi::c_void, ppOutputDuplication: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SupportsOverlays - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn SupportsOverlays() -> usize {
    0
}

/// CheckOverlaySupport - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn CheckOverlaySupport(EnumFormat: usize, pConcernedDevice: *mut core::ffi::c_void, pFlags: *mut u32) -> usize {
    0
}

/// CheckOverlayColorSpaceSupport - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn CheckOverlayColorSpaceSupport(Format: usize, ColorSpace: usize, pConcernedDevice: *mut core::ffi::c_void, pFlags: *mut u32) -> usize {
    0
}

/// CheckHardwareCompositionSupport - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn CheckHardwareCompositionSupport(pFlags: *mut u32) -> usize {
    0
}

/// FilterModesByDesc - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn FilterModesByDesc(Modes: usize, TargetMode: usize) {

}

/// CacheMonitorData - from dxvk/dxgi_output.h
#[no_mangle]
pub unsafe extern "C" fn CacheMonitorData() {

}

/// DestroyDummyWindow - from dxvk/dxgi_surface.h
#[no_mangle]
pub unsafe extern "C" fn DestroyDummyWindow() {

}

/// GetBuffer - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetBuffer(Buffer: u32, riid: usize, ppSurface: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetCurrentBackBufferIndex - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetCurrentBackBufferIndex() -> usize {
    0
}

/// GetContainingOutput - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetContainingOutput(ppOutput: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetFullscreenState - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetFullscreenState(pFullscreen: *mut i32, ppTarget: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetFullscreenDesc - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetFullscreenDesc(pDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// GetCoreWindow - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetCoreWindow(refiid: usize, ppUnk: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetBackgroundColor - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetBackgroundColor(pColor: *mut core::ffi::c_void) -> usize {
    0
}

/// GetRotation - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetRotation(pRotation: *mut core::ffi::c_void) -> usize {
    0
}

/// GetRestrictToOutput - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetRestrictToOutput(ppRestrictToOutput: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// IsTemporaryMonoSupported - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn IsTemporaryMonoSupported() -> usize {
    0
}

/// Present1 - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn Present1(SyncInterval: u32, PresentFlags: u32, pPresentParameters: *mut core::ffi::c_void) -> usize {
    0
}

/// ResizeBuffers - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn ResizeBuffers(BufferCount: u32, Width: u32, Height: u32, NewFormat: usize, SwapChainFlags: u32) -> usize {
    0
}

/// ResizeBuffers1 - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn ResizeBuffers1(BufferCount: u32, Width: u32, Height: u32, Format: usize, SwapChainFlags: u32, pCreationNodeMask: *mut u32, ppPresentQueue: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// ResizeTarget - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn ResizeTarget(pNewTargetParameters: *mut core::ffi::c_void) -> usize {
    0
}

/// SetFullscreenState - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn SetFullscreenState(Fullscreen: i32, pTarget: *mut core::ffi::c_void) -> usize {
    0
}

/// SetBackgroundColor - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn SetBackgroundColor(pColor: *mut core::ffi::c_void) -> usize {
    0
}

/// SetRotation - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn SetRotation(Rotation: usize) -> usize {
    0
}

/// GetFrameLatencyWaitableObject - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetFrameLatencyWaitableObject() -> usize {
    0
}

/// GetMatrixTransform - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetMatrixTransform(pMatrix: *mut core::ffi::c_void) -> usize {
    0
}

/// GetSourceSize - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetSourceSize(pWidth: *mut u32, pHeight: *mut u32) -> usize {
    0
}

/// SetMatrixTransform - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn SetMatrixTransform(pMatrix: *mut core::ffi::c_void) -> usize {
    0
}

/// SetSourceSize - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn SetSourceSize(Width: u32, Height: u32) -> usize {
    0
}

/// SetColorSpace1 - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn SetColorSpace1(ColorSpace: usize) -> usize {
    0
}

/// GetOutputFromMonitor - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn GetOutputFromMonitor(Monitor: usize, ppOutput: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// UpdateGlobalHDRState - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn UpdateGlobalHDRState() {

}

/// ValidateColorSpaceSupport - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn ValidateColorSpaceSupport(Format: usize, ColorSpace: usize) -> usize {
    0
}

/// UpdateColorSpace - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn UpdateColorSpace(Format: usize, ColorSpace: usize) -> i32 {
    0
}

/// PresentBase - from dxvk/dxgi_swapchain.h
#[no_mangle]
pub unsafe extern "C" fn PresentBase(SyncInterval: u32, PresentFlags: u32, pPresentParameters: *mut core::ffi::c_void) -> usize {
    0
}

/// finalize - from dxvk/dxso_analysis.h
#[no_mangle]
pub unsafe extern "C" fn finalize(tokenCount: usize) {

}

/// ptrAt - from dxvk/dxso_code.h
#[no_mangle]
pub unsafe extern "C" fn ptrAt(id: u32) -> *mut u32 {
    core::ptr::null_mut()
}

/// skip - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn skip(signature: usize, testn: usize) -> usize {
    0
}

/// iter - from dxvk/dxso_code.h
#[no_mangle]
pub unsafe extern "C" fn iter() -> usize {
    0
}

/// DxsoCodeIter - from dxvk/dxso_code.h
#[no_mangle]
pub unsafe extern "C" fn DxsoCodeIter(arg0: usize) -> usize {
    0
}

/// shaderStage - from dxvk/dxso_common.h
#[no_mangle]
pub unsafe extern "C" fn shaderStage() -> usize {
    0
}

/// executionModel - from dxvk/dxso_common.h
#[no_mangle]
pub unsafe extern "C" fn executionModel() -> usize {
    0
}

/// minorVersion - from dxvk/dxso_common.h
#[no_mangle]
pub unsafe extern "C" fn minorVersion() -> u32 {
    0
}

/// majorVersion - from dxvk/dxso_common.h
#[no_mangle]
pub unsafe extern "C" fn majorVersion() -> u32 {
    0
}

/// SamplerTypeFromTextureType - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn SamplerTypeFromTextureType(arg0: usize) -> usize {
    0
}

/// usedSamplers - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn usedSamplers() -> u32 {
    0
}

/// usedRTs - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn usedRTs() -> u32 {
    0
}

/// textureTypes - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn textureTypes() -> u32 {
    0
}

/// emitInit - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitInit() {

}

/// emitDclInputArray - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitDclInputArray() {

}

/// emitDclOutputArray - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitDclOutputArray() {

}

/// emitFunctionBegin - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitFunctionBegin(entryPoint: u32, returnType: u32, funcType: u32) {

}

/// emitFunctionEnd - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitFunctionEnd() {

}

/// emitFunctionLabel - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitFunctionLabel() -> u32 {
    0
}

/// emitMainFunctionBegin - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitMainFunctionBegin() {

}

/// emitNewVariable - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitNewVariable(info: usize) -> u32 {
    0
}

/// emitNewVariableDefault - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitNewVariableDefault(info: usize, value: u32) -> u32 {
    0
}

/// emitNewBuiltinVariable - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitNewBuiltinVariable(info: usize, builtIn: usize, name: *mut i8, value: u32) -> u32 {
    0
}

/// emitDclInterface - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitDclInterface(input: usize, regNumber: u32, semantic: usize, mask: usize, centroid: usize) {

}

/// emitDclSampler - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitDclSampler(idx: u32, arg1: usize) {

}

/// defineInput - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn defineInput(idx: u32) -> usize {
    0
}

/// defineOutput - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn defineOutput(idx: u32) -> usize {
    0
}

/// emitArrayIndex - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitArrayIndex(idx: u32, relative: *mut core::ffi::c_void) -> u32 {
    0
}

/// emitInputPtr - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitInputPtr(texture: usize, reg: usize, relative: *mut core::ffi::c_void) -> usize {
    0
}

/// emitRegisterPtr - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitRegisterPtr(name: *mut i8, ctype: usize, ccount: u32, defaultVal: u32, spvStorageClassPrivate: usize, spvBuiltInMax: usize) -> usize {
    0
}

/// emitOutputPtr - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitOutputPtr(texcrdOut: usize, reg: usize, relative: *mut core::ffi::c_void) -> usize {
    0
}

/// emitGetOperandPtr - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitGetOperandPtr(reg: usize, relative: *mut core::ffi::c_void) -> usize {
    0
}

/// emitBoolComparison - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitBoolComparison(arg0: usize, cmp: usize, a: u32, b: u32) -> u32 {
    0
}

/// emitValueLoad - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitValueLoad(ptr: usize) -> usize {
    0
}

/// emitDstStore - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitDstStore(ptr: usize, value: usize, writeMask: usize, saturate: usize, predicate: usize, shift: i8, regId: usize) {

}

/// applyPredicate - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn applyPredicate(pred: usize, dst: usize, src: usize) -> usize {
    0
}

/// emitValueStore - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitValueStore(ptr: usize, value: usize, writeMask: usize, predicate: usize) {

}

/// emitClampBoundReplicant - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitClampBoundReplicant(srcValue: usize, lb: f32, ub: f32) -> usize {
    0
}

/// emitSaturate - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitSaturate(srcValue: usize) -> usize {
    0
}

/// emitMulOperand - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitMulOperand(operand: usize, other: usize) -> usize {
    0
}

/// emitMul - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitMul(a: usize, b: usize) -> usize {
    0
}

/// emitMad - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitMad(a: usize, b: usize, c: usize) -> usize {
    0
}

/// emitDot - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitDot(a: usize, b: usize) -> usize {
    0
}

/// emitMix - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitMix(x: usize, y: usize, a: usize) -> usize {
    0
}

/// emitCross - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitCross(a: usize, b: usize) -> usize {
    0
}

/// emitRegisterInsert - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitRegisterInsert(dstValue: usize, srcValue: usize, srcMask: usize) -> usize {
    0
}

/// emitRegisterLoadRaw - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitRegisterLoadRaw(reg: usize, relative: *mut core::ffi::c_void) -> usize {
    0
}

/// emitRegisterExtend - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitRegisterExtend(value: usize, size: u32) -> usize {
    0
}

/// emitSrcOperandPreSwizzleModifiers - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitSrcOperandPreSwizzleModifiers(value: usize, modifier: usize) -> usize {
    0
}

/// emitSrcOperandPostSwizzleModifiers - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitSrcOperandPostSwizzleModifiers(value: usize, modifier: usize) -> usize {
    0
}

/// emitRegisterSwizzle - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitRegisterSwizzle(value: usize, swizzle: usize, writeMask: usize) -> usize {
    0
}

/// emitRegisterLoad - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitRegisterLoad(reg: usize, writeMask: usize, relative: *mut core::ffi::c_void) -> usize {
    0
}

/// emitPredicateLoad - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitPredicateLoad(ctx: usize) -> usize {
    0
}

/// DxsoRegisterValue - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn DxsoRegisterValue() -> usize {
    0
}

/// emitRegisterLoadTexcoord - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitRegisterLoadTexcoord(reg: usize, writeMask: usize) -> usize {
    0
}

/// emitDcl - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitDcl(ctx: usize) {

}

/// emitDef - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitDef(ctx: usize) {

}

/// emitDefF - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitDefF(ctx: usize) {

}

/// emitDefI - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitDefI(ctx: usize) {

}

/// emitDefB - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitDefB(ctx: usize) {

}

/// isScalarRegister - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn isScalarRegister(id: usize) -> usize {
    0
}

/// emitMov - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitMov(ctx: usize) {

}

/// emitPredicateOp - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitPredicateOp(ctx: usize) {

}

/// emitVectorAlu - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitVectorAlu(ctx: usize) {

}

/// emitMatrixAlu - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitMatrixAlu(ctx: usize) {

}

/// emitControlFlowGenericLoop - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitControlFlowGenericLoop(count: usize, initialVar: u32, strideVar: u32, iterationCountVar: u32) {

}

/// emitControlFlowGenericLoopEnd - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitControlFlowGenericLoopEnd() {

}

/// emitControlFlowRep - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitControlFlowRep(ctx: usize) {

}

/// emitControlFlowEndRep - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitControlFlowEndRep(ctx: usize) {

}

/// emitControlFlowLoop - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitControlFlowLoop(ctx: usize) {

}

/// emitControlFlowEndLoop - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitControlFlowEndLoop(ctx: usize) {

}

/// emitControlFlowBreak - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitControlFlowBreak(ctx: usize) {

}

/// emitControlFlowBreakC - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitControlFlowBreakC(ctx: usize) {

}

/// emitControlFlowIf - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitControlFlowIf(ctx: usize) {

}

/// emitControlFlowElse - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitControlFlowElse(ctx: usize) {

}

/// emitControlFlowEndIf - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitControlFlowEndIf(ctx: usize) {

}

/// emitTexCoord - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitTexCoord(ctx: usize) {

}

/// emitTextureSample - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitTextureSample(ctx: usize) {

}

/// emitTextureKill - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitTextureKill(ctx: usize) {

}

/// emitTextureDepth - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitTextureDepth(ctx: usize) {

}

/// emitSample - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitSample(resultType: u32, samplerInfo: usize, coordinates: usize, reference: u32, fetch4: u32, operands: usize) -> u32 {
    0
}

/// emitInputSetup - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitInputSetup() {

}

/// emitVsClipping - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitVsClipping() {

}

/// setupRenderStateInfo - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn setupRenderStateInfo(samplerCount: u32) {

}

/// emitFog - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitFog() {

}

/// emitOutputDepthClamp - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitOutputDepthClamp() {

}

/// emitLinkerOutputSetup - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitLinkerOutputSetup() {

}

/// emitVsFinalize - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitVsFinalize() {

}

/// emitPsFinalize - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitPsFinalize() {

}

/// getScalarTypeId - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn getScalarTypeId(arg0: usize) -> u32 {
    0
}

/// getVectorTypeId - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn getVectorTypeId(arg0: usize) -> u32 {
    0
}

/// getArrayTypeId - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn getArrayTypeId(arg0: usize) -> u32 {
    0
}

/// getPointerTypeId - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn getPointerTypeId(arg0: usize) -> u32 {
    0
}

/// isSwvp - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn isSwvp() -> usize {
    0
}

/// popCount - from dxvk/dxso_decoder.h
#[no_mangle]
pub unsafe extern "C" fn popCount() -> u32 {
    0
}

/// firstSet - from dxvk/dxso_decoder.h
#[no_mangle]
pub unsafe extern "C" fn firstSet() -> u32 {
    0
}

/// minComponents - from dxvk/dxso_decoder.h
#[no_mangle]
pub unsafe extern "C" fn minComponents() -> u32 {
    0
}

/// decodeInstruction - from dxvk/dxso_decoder.h
#[no_mangle]
pub unsafe extern "C" fn decodeInstruction(iter: usize) -> usize {
    0
}

/// decodeInstructionLength - from dxvk/dxso_decoder.h
#[no_mangle]
pub unsafe extern "C" fn decodeInstructionLength(token: u32) -> u32 {
    0
}

/// decodeBaseRegister - from dxvk/dxso_decoder.h
#[no_mangle]
pub unsafe extern "C" fn decodeBaseRegister(reg: usize, token: u32) {

}

/// decodeGenericRegister - from dxvk/dxso_decoder.h
#[no_mangle]
pub unsafe extern "C" fn decodeGenericRegister(reg: usize, token: u32) {

}

/// decodeRelativeRegister - from dxvk/dxso_decoder.h
#[no_mangle]
pub unsafe extern "C" fn decodeRelativeRegister(reg: usize, token: u32) {

}

/// decodeDestinationRegister - from dxvk/dxso_decoder.h
#[no_mangle]
pub unsafe extern "C" fn decodeDestinationRegister(iter: usize) -> usize {
    0
}

/// decodeSourceRegister - from dxvk/dxso_decoder.h
#[no_mangle]
pub unsafe extern "C" fn decodeSourceRegister(i: u32, iter: usize) -> usize {
    0
}

/// decodePredicateRegister - from dxvk/dxso_decoder.h
#[no_mangle]
pub unsafe extern "C" fn decodePredicateRegister(iter: usize) {

}

/// decodeDeclaration - from dxvk/dxso_decoder.h
#[no_mangle]
pub unsafe extern "C" fn decodeDeclaration(iter: usize) {

}

/// decodeDefinition - from dxvk/dxso_decoder.h
#[no_mangle]
pub unsafe extern "C" fn decodeDefinition(opcode: usize, iter: usize) {

}

/// analyze - from dxvk/dxso_module.h
#[no_mangle]
pub unsafe extern "C" fn analyze() -> usize {
    0
}

/// runCompiler - from dxvk/dxso_module.h
#[no_mangle]
pub unsafe extern "C" fn runCompiler(compiler: usize, iter: usize) {

}

/// runAnalyzer - from dxvk/dxso_module.h
#[no_mangle]
pub unsafe extern "C" fn runAnalyzer(analyzer: usize, iter: usize) {

}

/// pos - from dxvk/dxso_reader.h
#[no_mangle]
pub unsafe extern "C" fn pos() -> usize {
    0
}

/// store - from dxvk/dxso_reader.h
#[no_mangle]
pub unsafe extern "C" fn store(stream: usize, size: usize) {

}

/// currentPtr - from dxvk/dxso_reader.h
#[no_mangle]
pub unsafe extern "C" fn currentPtr() -> *mut i8 {
    core::ptr::null_mut()
}

/// DxsoGetDefaultOpcodeLength - from dxvk/dxso_tables.h
#[no_mangle]
pub unsafe extern "C" fn DxsoGetDefaultOpcodeLength(opcode: usize) -> u32 {
    0
}

/// RegisterLinkerSlot - from dxvk/dxso_util.h
#[no_mangle]
pub unsafe extern "C" fn RegisterLinkerSlot(semantic: usize) -> u32 {
    0
}

/// DxvkObjectRef - from dxvk/dxvk_access.h
#[no_mangle]
pub unsafe extern "C" fn DxvkObjectRef(object: usize) -> usize {
    0
}

/// track - from dxvk/dxvk_access.h
#[no_mangle]
pub unsafe extern "C" fn track(args: usize) -> usize {
    0
}

/// advanceList - from dxvk/dxvk_access.h
#[no_mangle]
pub unsafe extern "C" fn advanceList() {

}

/// handle - from dxvk/dxvk_adapter.h
#[no_mangle]
pub unsafe extern "C" fn handle() -> usize {
    0
}

/// kmtLocal - from dxvk/dxvk_adapter.h
#[no_mangle]
pub unsafe extern "C" fn kmtLocal() -> usize {
    0
}

/// info - from dxvk/dxvk_adapter.h
#[no_mangle]
pub unsafe extern "C" fn info() -> usize {
    0
}

/// isCompatible - from dxvk/dxvk_adapter.h
#[no_mangle]
pub unsafe extern "C" fn isCompatible(error: usize) -> usize {
    0
}

/// getMemoryHeapInfo - from dxvk/dxvk_adapter.h
#[no_mangle]
pub unsafe extern "C" fn getMemoryHeapInfo() -> usize {
    0
}

/// getFormatFeatures - from dxvk/dxvk_adapter.h
#[no_mangle]
pub unsafe extern "C" fn getFormatFeatures(format: usize) -> usize {
    0
}

/// findQueueFamilies - from dxvk/dxvk_adapter.h
#[no_mangle]
pub unsafe extern "C" fn findQueueFamilies() -> usize {
    0
}

/// checkFeatureSupport - from dxvk/dxvk_adapter.h
#[no_mangle]
pub unsafe extern "C" fn checkFeatureSupport(required: usize) -> usize {
    0
}

/// enableExtensions - from dxvk/dxvk_adapter.h
#[no_mangle]
pub unsafe extern "C" fn enableExtensions(extensions: usize) {

}

/// notifyMemoryStats - from dxvk/dxvk_adapter.h
#[no_mangle]
pub unsafe extern "C" fn notifyMemoryStats(heap: u32, allocated: i64, used: i64) {

}

/// matchesDriver - from dxvk/dxvk_adapter.h
#[no_mangle]
pub unsafe extern "C" fn matchesDriver(driver: usize, minVer: usize, maxVer: usize) -> usize {
    0
}

/// isUnifiedMemoryArchitecture - from dxvk/dxvk_adapter.h
#[no_mangle]
pub unsafe extern "C" fn isUnifiedMemoryArchitecture() -> usize {
    0
}

/// linkToDGPU - from dxvk/dxvk_adapter.h
#[no_mangle]
pub unsafe extern "C" fn linkToDGPU(dgpu: usize) {

}

/// isLinkedToDGPU - from dxvk/dxvk_adapter.h
#[no_mangle]
pub unsafe extern "C" fn isLinkedToDGPU() -> usize {
    0
}

/// chunkCount - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn chunkCount() -> u32 {
    0
}

/// pageCount - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn pageCount(chunkIndex: u32) -> u32 {
    0
}

/// pagesUsed - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn pagesUsed(chunkIndex: u32) -> u32 {
    0
}

/// chunkIsAvailable - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn chunkIsAvailable(chunkIndex: u32) -> usize {
    0
}

/// addChunk - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn addChunk(size: u64) -> u32 {
    0
}

/// removeChunk - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn removeChunk(chunkIndex: u32) {

}

/// killChunk - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn killChunk(chunkIndex: u32) {

}

/// reviveChunk - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn reviveChunk(chunkIndex: u32) {

}

/// reviveChunks - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn reviveChunks() -> u32 {
    0
}

/// addLutEntry - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn addLutEntry(range: usize, index: i32) {

}

/// removeLutEntry - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn removeLutEntry(range: usize) {

}

/// addPageToList - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn addPageToList(pageIndex: u32, listIndex: u32) {

}

/// removePageFromList - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn removePageFromList(pageIndex: u32, listIndex: u32) {

}

/// computeListIndex - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn computeListIndex(size: u64) -> u32 {
    0
}

/// computePoolCapacity - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn computePoolCapacity(index: u32) -> u32 {
    0
}

/// computeByteAddress - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn computeByteAddress(page: u32, index: u32, list: u32) -> u64 {
    0
}

/// computePageIndexFromByteAddress - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn computePageIndexFromByteAddress(address: u64) -> u32 {
    0
}

/// computeItemIndexFromByteAddress - from dxvk/dxvk_allocator.h
#[no_mangle]
pub unsafe extern "C" fn computeItemIndexFromByteAddress(address: u64, list: u32) -> u32 {
    0
}

/// setRed - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn setRed(red: usize) {

}

/// isRed - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn isRed() -> usize {
    0
}

/// setParent - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn setParent(node: u32) {

}

/// setChild - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn setChild(index: u32, node: u32) {

}

/// parent - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn parent() -> u32 {
    0
}

/// child - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn child(index: u32) -> u32 {
    0
}

/// isRoot - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn isRoot() -> usize {
    0
}

/// findRange - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn findRange(range: usize, accessType: usize) -> usize {
    0
}

/// insertRange - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn insertRange(range: usize, accessType: usize) {

}

/// findNode - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn findNode(range: usize, rootIndex: u32) -> u32 {
    0
}

/// insertNode - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn insertNode(range: usize, rootIndex: u32) -> u32 {
    0
}

/// removeNode - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn removeNode(nodeIndex: u32, rootIndex: u32) {

}

/// rebalancePostInsert - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn rebalancePostInsert(nodeIndex: u32, rootIndex: u32) {

}

/// rotateLeft - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn rotateLeft(nodeIndex: u32, rootIndex: u32) {

}

/// rotateRight - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn rotateRight(nodeIndex: u32, rootIndex: u32) {

}

/// computeRootIndex - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn computeRootIndex(range: usize, access: usize) -> u32 {
    0
}

/// addMemoryBarrier - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn addMemoryBarrier(barrier: usize) {

}

/// addImageBarrier - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn addImageBarrier(barrier: usize) {

}

/// hasLayoutTransitions - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn hasLayoutTransitions() -> usize {
    0
}

/// hasPendingStages - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn hasPendingStages(stages: usize) -> usize {
    0
}

/// hasPendingAccess - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn hasPendingAccess(access: usize) -> usize {
    0
}

/// hasTargetAccess - from dxvk/dxvk_barrier.h
#[no_mangle]
pub unsafe extern "C" fn hasTargetAccess(access: usize) -> usize {
    0
}

/// test - from dxvk/dxvk_bind_mask.h
#[no_mangle]
pub unsafe extern "C" fn test(slot: u32) -> usize {
    0
}

/// clr - from dxvk/dxvk_bind_mask.h
#[no_mangle]
pub unsafe extern "C" fn clr(slot: u32) -> usize {
    0
}

/// setRange - from dxvk/dxvk_bind_mask.h
#[no_mangle]
pub unsafe extern "C" fn setRange(first: u32, count: u32) {

}

/// findNext - from dxvk/dxvk_bind_mask.h
#[no_mangle]
pub unsafe extern "C" fn findNext(first: u32) -> i32 {
    0
}

/// computeIntId - from dxvk/dxvk_bind_mask.h
#[no_mangle]
pub unsafe extern "C" fn computeIntId(slot: u32) -> u32 {
    0
}

/// computeBitId - from dxvk/dxvk_bind_mask.h
#[no_mangle]
pub unsafe extern "C" fn computeBitId(slot: u32) -> u32 {
    0
}

/// getDescriptor - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn getDescriptor(raw: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// getSliceInfo - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn getSliceInfo() -> usize {
    0
}

/// elementCount - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn elementCount() -> u64 {
    0
}

/// buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn buffer(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// formatInfo - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn formatInfo() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// lookupFormatInfo - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn lookupFormatInfo(arg0: usize) -> usize {
    0
}

/// updateViews - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn updateViews() {

}

/// memFlags - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn memFlags() -> usize {
    0
}

/// getShaderStages - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn getShaderStages() -> usize {
    0
}

/// getXfbVertexStride - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn getXfbVertexStride() -> u32 {
    0
}

/// setXfbVertexStride - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn setXfbVertexStride(stride: u32) {

}

/// getResourceId - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn getResourceId() -> usize {
    0
}

/// canRelocate - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn canRelocate() -> usize {
    0
}

/// enableStableAddress - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn enableStableAddress() {

}

/// getSparsePageTable - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn getSparsePageTable() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// setDebugName - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn setDebugName(name: *mut i8) {

}

/// getDebugName - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn getDebugName() -> *mut i8 {
    core::ptr::null_mut()
}

/// updateDebugName - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn updateDebugName() {

}

/// offset - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn offset() -> usize {
    0
}

/// length - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn length() -> usize {
    0
}

/// subSlice - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn subSlice(offset: u64, length: u64) -> usize {
    0
}

/// defined - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn defined() -> usize {
    0
}

/// mapPtr - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn mapPtr(offset: u64) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// matchesBuffer - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn matchesBuffer(other: usize) -> usize {
    0
}

/// matchesRange - from dxvk/dxvk_buffer.h
#[no_mangle]
pub unsafe extern "C" fn matchesRange(other: usize) -> usize {
    0
}

/// waitSemaphore - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn waitSemaphore(semaphore: usize, value: u64, stageMask: usize) {

}

/// signalSemaphore - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn signalSemaphore(semaphore: usize, value: u64, stageMask: usize) {

}

/// executeCommandBuffer - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn executeCommandBuffer(commandBuffer: usize) {

}

/// submit - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn submit(device: *mut core::ffi::c_void, queue: usize, frameId: u64) -> i32 {
    0
}

/// isEmpty - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn isEmpty() -> usize {
    0
}

/// getCommandBuffer - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn getCommandBuffer(arg0: usize) -> usize {
    0
}

/// getSecondaryCommandBuffer - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn getSecondaryCommandBuffer(inheritanceInfo: usize) -> usize {
    0
}

/// addStatCtr - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn addStatCtr(ctr: usize, val: u64) {

}

/// next - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn next() {

}

/// trackGraphicsPipeline - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn trackGraphicsPipeline(pipeline: *mut core::ffi::c_void) {

}

/// queueSignal - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn queueSignal(signal: usize, value: u64) {

}

/// notifyObjects - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn notifyObjects() {

}

/// waitFence - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn waitFence(fence: usize, value: u64) {

}

/// signalFence - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn signalFence(fence: usize, value: u64) {

}

/// setWsiSemaphores - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn setWsiSemaphores(wsiSemaphores: usize) {

}

/// setSubmissionBarrier - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn setSubmissionBarrier() {

}

/// bindResources - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn bindResources(cmdBuffer: usize, layout: *mut core::ffi::c_void, descriptorCount: u32, descriptorInfos: *mut core::ffi::c_void, pushDataSize: usize, pushData: *mut core::ffi::c_void) {

}

/// beginSecondaryCommandBuffer - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn beginSecondaryCommandBuffer(inheritanceInfo: usize) {

}

/// endSecondaryCommandBuffer - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn endSecondaryCommandBuffer() -> usize {
    0
}

/// cmdExecuteCommands - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdExecuteCommands(count: u32, commandBuffers: *mut core::ffi::c_void) {

}

/// cmdBeginQuery - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdBeginQuery(queryPool: usize, query: u32, flags: usize) {

}

/// cmdBeginQueryIndexed - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdBeginQueryIndexed(queryPool: usize, query: u32, flags: usize, index: u32) {

}

/// cmdBeginRendering - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdBeginRendering(cmdBuffer: usize, pRenderingInfo: *mut core::ffi::c_void) {

}

/// cmdBindDescriptorSets - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdBindDescriptorSets(cmdBuffer: usize, info: *mut core::ffi::c_void) {

}

/// cmdSetDescriptorBufferOffsetsEXT - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetDescriptorBufferOffsetsEXT(cmdBuffer: usize, info: *mut core::ffi::c_void) {

}

/// cmdBindIndexBuffer2 - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdBindIndexBuffer2(buffer: usize, offset: u64, size: u64, indexType: usize) {

}

/// cmdBindPipeline - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdBindPipeline(cmdBuffer: usize, pipelineBindPoint: usize, pipeline: usize) {

}

/// cmdLaunchCuKernel - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdLaunchCuKernel(launchInfo: usize) {

}

/// cmdBlitImage - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdBlitImage(pBlitInfo: *mut core::ffi::c_void) {

}

/// cmdClearColorImage - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdClearColorImage(cmdBuffer: usize, image: usize, imageLayout: usize, pColor: *mut core::ffi::c_void, rangeCount: u32, pRanges: *mut core::ffi::c_void) {

}

/// cmdClearDepthStencilImage - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdClearDepthStencilImage(cmdBuffer: usize, image: usize, imageLayout: usize, pDepthStencil: *mut core::ffi::c_void, rangeCount: u32, pRanges: *mut core::ffi::c_void) {

}

/// cmdCopyBuffer - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdCopyBuffer(cmdBuffer: usize, copyInfo: *mut core::ffi::c_void) {

}

/// cmdCopyBufferToImage - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdCopyBufferToImage(cmdBuffer: usize, copyInfo: *mut core::ffi::c_void) {

}

/// cmdCopyImage - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdCopyImage(cmdBuffer: usize, copyInfo: *mut core::ffi::c_void) {

}

/// cmdCopyImageToBuffer - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdCopyImageToBuffer(cmdBuffer: usize, copyInfo: *mut core::ffi::c_void) {

}

/// cmdCopyQueryPoolResults - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdCopyQueryPoolResults(cmdBuffer: usize, queryPool: usize, firstQuery: u32, queryCount: u32, dstBuffer: usize, dstOffset: u64, stride: u64, flags: usize) {

}

/// cmdDispatch - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdDispatch(cmdBuffer: usize, x: u32, y: u32, z: u32) {

}

/// cmdDispatchIndirect - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdDispatchIndirect(cmdBuffer: usize, buffer: usize, offset: u64) {

}

/// cmdDraw - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdDraw(vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32) {

}

/// cmdDrawIndirect - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdDrawIndirect(buffer: usize, offset: u64, drawCount: u32, stride: u32) {

}

/// cmdDrawIndexedIndirect - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdDrawIndexedIndirect(buffer: usize, offset: u64, drawCount: u32, stride: u32) {

}

/// cmdEndQuery - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdEndQuery(queryPool: usize, query: u32) {

}

/// cmdEndQueryIndexed - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdEndQueryIndexed(queryPool: usize, query: u32, index: u32) {

}

/// cmdEndRendering - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdEndRendering(cmdBuffer: usize) {

}

/// cmdFillBuffer - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdFillBuffer(cmdBuffer: usize, dstBuffer: usize, dstOffset: u64, size: u64, data: u32) {

}

/// cmdPipelineBarrier - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdPipelineBarrier(cmdBuffer: usize, dependencyInfo: *mut core::ffi::c_void) {

}

/// cmdPushData - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdPushData(cmdBuffer: usize, info: *mut core::ffi::c_void) {

}

/// cmdResetQueryPool - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdResetQueryPool(cmdBuffer: usize, queryPool: usize, firstQuery: u32, queryCount: u32) {

}

/// cmdResolveImage - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdResolveImage(resolveInfo: *mut core::ffi::c_void) {

}

/// cmdUpdateBuffer - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdUpdateBuffer(cmdBuffer: usize, dstBuffer: usize, dstOffset: u64, dataSize: u64, pData: *mut core::ffi::c_void) {

}

/// cmdSetAlphaToCoverageState - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetAlphaToCoverageState(alphaToCoverageEnable: u32) {

}

/// cmdSetDepthClipState - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetDepthClipState(depthClipEnable: u32) {

}

/// cmdSetDepthBias - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetDepthBias(depthBiasConstantFactor: f32, depthBiasClamp: f32, depthBiasSlopeFactor: f32) {

}

/// cmdSetDepthBias2 - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetDepthBias2(depthBiasInfo: *mut core::ffi::c_void) {

}

/// cmdSetDepthBounds - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetDepthBounds(minDepthBounds: f32, maxDepthBounds: f32) {

}

/// cmdSetDepthTest - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetDepthTest(depthTestEnable: u32) {

}

/// cmdSetDepthCompareOp - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetDepthCompareOp(depthCompareOp: usize) {

}

/// cmdSetEvent - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetEvent(event: usize, dependencyInfo: *mut core::ffi::c_void) {

}

/// cmdSetMultisampleState - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetMultisampleState(sampleCount: usize, sampleMask: u32) {

}

/// cmdSetRasterizerState - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetRasterizerState(cullMode: usize, frontFace: usize) {

}

/// cmdSetSampleLocations - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetSampleLocations(enable: u32, sampleLocations: *mut core::ffi::c_void) {

}

/// cmdSetScissor - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetScissor(scissorCount: u32, scissors: *mut core::ffi::c_void) {

}

/// cmdSetStencilTest - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetStencilTest(enableStencilTest: u32) {

}

/// cmdSetStencilCompareMask - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetStencilCompareMask(faceMask: usize, compareMask: u32) {

}

/// cmdSetStencilReference - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetStencilReference(faceMask: usize, reference: u32) {

}

/// cmdSetViewport - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetViewport(viewportCount: u32, viewports: *mut core::ffi::c_void) {

}

/// cmdBeginDebugUtilsLabel - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdBeginDebugUtilsLabel(cmdBuffer: usize, labelInfo: usize) {

}

/// cmdEndDebugUtilsLabel - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdEndDebugUtilsLabel(cmdBuffer: usize) {

}

/// cmdInsertDebugUtilsLabel - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdInsertDebugUtilsLabel(cmdBuffer: usize, labelInfo: usize) {

}

/// resetQuery - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn resetQuery(queryPool: usize, queryId: u32) {

}

/// bindBufferMemory - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn bindBufferMemory(key: usize, memory: usize) {

}

/// bindImageMemory - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn bindImageMemory(key: usize, memory: usize) {

}

/// bindImageOpaqueMemory - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn bindImageOpaqueMemory(key: usize, memory: usize) {

}

/// setDescriptorPool - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn setDescriptorPool(pool: usize) {

}

/// setDescriptorHeap - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn setDescriptorHeap(heap: usize) {

}

/// setTrackingId - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn setTrackingId(id: u64) {

}

/// setDescriptorSyncHandle - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn setDescriptorSyncHandle(syncHandle: usize) {

}

/// ensureDescriptorHeapBinding - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn ensureDescriptorHeapBinding() {

}

/// invalidateDescriptorHeapBinding - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn invalidateDescriptorHeapBinding() {

}

/// getCmdBuffer - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn getCmdBuffer() -> usize {
    0
}

/// bindResourcesLegacy - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn bindResourcesLegacy(cmdBuffer: usize, layout: *mut core::ffi::c_void, descriptorCount: u32, descriptorInfos: *mut core::ffi::c_void, pushDataSize: usize, pushData: *mut core::ffi::c_void) {

}

/// bindResourcesDescriptorHeap - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn bindResourcesDescriptorHeap(cmdBuffer: usize, layout: *mut core::ffi::c_void, descriptorCount: u32, descriptorInfos: *mut core::ffi::c_void, pushDataSize: usize, pushData: *mut core::ffi::c_void) {

}

/// bindResourcesDescriptorBuffer - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn bindResourcesDescriptorBuffer(cmdBuffer: usize, layout: *mut core::ffi::c_void, descriptorCount: u32, descriptorInfos: *mut core::ffi::c_void, pushDataSize: usize, pushData: *mut core::ffi::c_void) {

}

/// rebindSamplerHeap - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn rebindSamplerHeap() {

}

/// rebindResourceHeap - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn rebindResourceHeap() {

}

/// rebindDescriptorBuffers - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn rebindDescriptorBuffers() {

}

/// bindSamplerHeap - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn bindSamplerHeap(cmdBuffer: usize) {

}

/// bindResourceHeap - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn bindResourceHeap(cmdBuffer: usize) {

}

/// bindDescriptorBuffers - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn bindDescriptorBuffers(cmdBuffer: usize) {

}

/// endCommandBuffer - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn endCommandBuffer(cmdBuffer: usize) {

}

/// countDescriptorStats - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn countDescriptorStats(range: usize, baseOffset: u64) {

}

/// getHeapBindInfo - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn getHeapBindInfo(heapInfo: usize) -> usize {
    0
}

/// eq - from dxvk/dxvk_compute.h
#[no_mangle]
pub unsafe extern "C" fn eq(other: usize) -> usize {
    0
}

/// getLayout - from dxvk/dxvk_compute.h
#[no_mangle]
pub unsafe extern "C" fn getLayout() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// getPipelineHandle - from dxvk/dxvk_compute.h
#[no_mangle]
pub unsafe extern "C" fn getPipelineHandle(state: usize) -> usize {
    0
}

/// compilePipeline - from dxvk/dxvk_compute.h
#[no_mangle]
pub unsafe extern "C" fn compilePipeline(state: usize) {

}

/// debugName - from dxvk/dxvk_compute.h
#[no_mangle]
pub unsafe extern "C" fn debugName() -> *mut i8 {
    core::ptr::null_mut()
}

/// destroyPipeline - from dxvk/dxvk_compute.h
#[no_mangle]
pub unsafe extern "C" fn destroyPipeline(pipeline: usize) {

}

/// primitiveRestart - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn primitiveRestart() -> usize {
    0
}

/// patchVertexCount - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn patchVertexCount() -> u32 {
    0
}

/// setPrimitiveRestart - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setPrimitiveRestart(enable: usize) {

}

/// setPatchVertexCount - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setPatchVertexCount(count: u32) {

}

/// polygonMode - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn polygonMode() -> usize {
    0
}

/// VkPolygonMode - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn VkPolygonMode(arg0: usize) -> usize {
    0
}

/// cullMode - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn cullMode() -> usize {
    0
}

/// VkCullModeFlags - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn VkCullModeFlags(arg0: usize) -> usize {
    0
}

/// frontFace - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn frontFace() -> usize {
    0
}

/// VkFrontFace - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn VkFrontFace(arg0: usize) -> usize {
    0
}

/// depthClip - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn depthClip() -> usize {
    0
}

/// conservativeMode - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn conservativeMode() -> usize {
    0
}

/// VkConservativeRasterizationModeEXT - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn VkConservativeRasterizationModeEXT(arg0: usize) -> usize {
    0
}

/// sampleCount - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn sampleCount() -> usize {
    0
}

/// VkSampleCountFlags - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn VkSampleCountFlags(arg0: usize) -> usize {
    0
}

/// flatShading - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn flatShading() -> usize {
    0
}

/// lineMode - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn lineMode() -> usize {
    0
}

/// VkLineRasterizationModeEXT - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn VkLineRasterizationModeEXT(arg0: usize) -> usize {
    0
}

/// setPolygonMode - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setPolygonMode(mode: usize) {

}

/// setCullMode - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setCullMode(mode: usize) {

}

/// setFrontFace - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setFrontFace(face: usize) {

}

/// setDepthClip - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setDepthClip(enable: usize) {

}

/// setConservativeMode - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setConservativeMode(mode: usize) {

}

/// setSampleCount - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setSampleCount(count: usize) {

}

/// setFlatShading - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setFlatShading(enable: usize) {

}

/// setLineMode - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setLineMode(mode: usize) {

}

/// sampleMask - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn sampleMask() -> u16 {
    0
}

/// alphaToCoverage - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn alphaToCoverage() -> usize {
    0
}

/// setSampleMask - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setSampleMask(mask: u16) {

}

/// setAlphaToCoverage - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setAlphaToCoverage(alphaToCoverage: usize) {

}

/// failOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn failOp() -> usize {
    0
}

/// VkStencilOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn VkStencilOp(arg0: usize) -> usize {
    0
}

/// passOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn passOp() -> usize {
    0
}

/// depthFailOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn depthFailOp() -> usize {
    0
}

/// compareOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn compareOp() -> usize {
    0
}

/// VkCompareOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn VkCompareOp(arg0: usize) -> usize {
    0
}

/// compareMask - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn compareMask() -> u8 {
    0
}

/// setFailOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setFailOp(op: usize) {

}

/// setPassOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setPassOp(op: usize) {

}

/// setDepthFailOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setDepthFailOp(op: usize) {

}

/// setCompareOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setCompareOp(op: usize) {

}

/// setCompareMask - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setCompareMask(mask: u8) {

}

/// normalize - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn normalize(depthOp: usize) -> usize {
    0
}

/// depthTest - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn depthTest() -> usize {
    0
}

/// stencilTest - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn stencilTest() -> usize {
    0
}

/// depthCompareOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn depthCompareOp() -> usize {
    0
}

/// stencilOpFront - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn stencilOpFront() -> usize {
    0
}

/// stencilOpBack - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn stencilOpBack() -> usize {
    0
}

/// setDepthTest - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setDepthTest(depthTest: usize) {

}

/// setStencilTest - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setStencilTest(stencilTest: usize) {

}

/// setDepthCompareOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setDepthCompareOp(compareOp: usize) {

}

/// setStencilOpFront - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setStencilOpFront(op: usize) {

}

/// setStencilOpBack - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setStencilOpBack(op: usize) {

}

/// blendEnable - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn blendEnable() -> usize {
    0
}

/// colorSrcFactor - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn colorSrcFactor() -> usize {
    0
}

/// VkBlendFactor - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn VkBlendFactor(arg0: usize) -> usize {
    0
}

/// colorDstFactor - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn colorDstFactor() -> usize {
    0
}

/// colorBlendOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn colorBlendOp() -> usize {
    0
}

/// VkBlendOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn VkBlendOp(arg0: usize) -> usize {
    0
}

/// alphaSrcFactor - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn alphaSrcFactor() -> usize {
    0
}

/// alphaDstFactor - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn alphaDstFactor() -> usize {
    0
}

/// alphaBlendOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn alphaBlendOp() -> usize {
    0
}

/// VkColorComponentFlags - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn VkColorComponentFlags(arg0: usize) -> usize {
    0
}

/// setBlendEnable - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setBlendEnable(enable: usize) {

}

/// setColorOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setColorOp(srcFactor: usize, dstFactor: usize, op: usize) {

}

/// setAlphaOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setAlphaOp(srcFactor: usize, dstFactor: usize, op: usize) {

}

/// attribute - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn attribute() -> usize {
    0
}

/// binding - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn binding() -> usize {
    0
}

/// beginRecording - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn beginRecording(cmdList: usize) {

}

/// endFrame - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn endFrame() {

}

/// beginLatencyTracking - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn beginLatencyTracking(tracker: usize, frameId: u64) {

}

/// endLatencyTracking - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn endLatencyTracking(tracker: usize) {

}

/// flushCommandList - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn flushCommandList(reason: *mut core::ffi::c_void, status: *mut core::ffi::c_void) {

}

/// synchronizeWsi - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn synchronizeWsi(sync: usize) {

}

/// beginQuery - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn beginQuery(query: usize) {

}

/// endQuery - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn endQuery(query: usize) {

}

/// bindRenderTargets - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn bindRenderTargets(targets: usize, feedbackLoop: usize) {

}

/// bindDrawBuffers - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn bindDrawBuffers(argBuffer: usize, cntBuffer: usize) {

}

/// bindIndexBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn bindIndexBuffer(buffer: usize, indexType: usize) {

}

/// bindIndexBufferRange - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn bindIndexBufferRange(offset: u64, length: u64, indexType: usize) {

}

/// bindUniformBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn bindUniformBuffer(stages: usize, slot: u32, buffer: usize) {

}

/// bindUniformBufferRange - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn bindUniformBufferRange(stages: usize, slot: u32, offset: u64, length: u64) {

}

/// bindResourceSampler - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn bindResourceSampler(stages: usize, slot: u32, sampler: usize) {

}

/// bindShader - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn bindShader(shader: usize) {

}

/// bindVertexBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn bindVertexBuffer(binding: u32, buffer: usize, stride: u32) {

}

/// bindVertexBufferRange - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn bindVertexBufferRange(binding: u32, offset: u64, length: u64, stride: u32) {

}

/// bindXfbBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn bindXfbBuffer(binding: u32, buffer: usize, counter: usize) {

}

/// blitImageView - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn blitImageView(dstView: usize, dstOffsets: *mut core::ffi::c_void, srcView: usize, srcOffsets: *mut core::ffi::c_void, filter: usize) {

}

/// clearBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn clearBuffer(buffer: usize, offset: u64, length: u64, value: u32) {

}

/// clearBufferView - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn clearBufferView(bufferView: usize, offset: u64, length: u64, value: usize) {

}

/// clearRenderTarget - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn clearRenderTarget(imageView: usize, clearAspects: usize, clearValue: usize, discardAspects: usize) {

}

/// clearImageView - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn clearImageView(imageView: usize, offset: usize, extent: usize, aspect: usize, value: usize) {

}

/// copyBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyBuffer(dstBuffer: usize, dstOffset: u64, srcBuffer: usize, srcOffset: u64, numBytes: u64) {

}

/// copyBufferRegion - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyBufferRegion(dstBuffer: usize, dstOffset: u64, srcOffset: u64, numBytes: u64) {

}

/// copyBufferToImage - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyBufferToImage(dstImage: usize, dstSubresource: usize, dstOffset: usize, dstExtent: usize, srcBuffer: usize, srcOffset: u64, rowAlignment: u64, sliceAlignment: u64, srcFormat: usize) {

}

/// copyImage - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyImage(dstImage: usize, dstSubresource: usize, dstOffset: usize, srcImage: usize, srcSubresource: usize, srcOffset: usize, extent: usize) {

}

/// copyImageRegion - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyImageRegion(dstImage: usize, dstSubresource: usize, dstOffset: usize, srcOffset: usize, extent: usize) {

}

/// copyImageToBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyImageToBuffer(dstBuffer: usize, dstOffset: u64, rowAlignment: u64, sliceAlignment: u64, dstFormat: usize, srcImage: usize, srcSubresource: usize, srcOffset: usize, srcExtent: usize) {

}

/// copyPackedBufferImage - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyPackedBufferImage(dstBuffer: usize, dstBufferOffset: u64, dstOffset: usize, dstSize: usize, srcBuffer: usize, srcBufferOffset: u64, srcOffset: usize, srcSize: usize, extent: usize, elementSize: u64) {

}

/// copySparsePagesToBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copySparsePagesToBuffer(dstBuffer: usize, dstOffset: u64, srcResource: usize, pageCount: u32, pages: *mut u32) {

}

/// copySparsePagesFromBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copySparsePagesFromBuffer(dstResource: usize, pageCount: u32, pages: *mut u32, srcBuffer: usize, srcOffset: u64) {

}

/// discardImage - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn discardImage(image: usize) {

}

/// dispatchIndirect - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn dispatchIndirect(offset: u64) {

}

/// draw - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn draw(count: u32, draws: *mut core::ffi::c_void) {

}

/// drawIndirect - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn drawIndirect(offset: u64, count: u32, stride: u32, unroll: usize) {

}

/// drawIndirectCount - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn drawIndirectCount(offset: u64, countOffset: u64, maxCount: u32, stride: u32) {

}

/// drawIndexed - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn drawIndexed(count: u32, draws: *mut core::ffi::c_void) {

}

/// drawIndexedIndirect - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn drawIndexedIndirect(offset: u64, count: u32, stride: u32, unroll: usize) {

}

/// drawIndexedIndirectCount - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn drawIndexedIndirectCount(offset: u64, countOffset: u64, maxCount: u32, stride: u32) {

}

/// drawIndirectXfb - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn drawIndirectXfb(counterOffset: u64, counterDivisor: u32, counterBias: u32) {

}

/// emitGraphicsBarrier - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn emitGraphicsBarrier(srcStages: usize, srcAccess: usize, dstStages: usize, dstAccess: usize) {

}

/// acquireExternalResource - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn acquireExternalResource(resource: usize, layout: usize) {

}

/// releaseExternalResource - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn releaseExternalResource(resource: usize, layout: usize) {

}

/// generateMipmaps - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn generateMipmaps(imageView: usize, filter: usize) {

}

/// initBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn initBuffer(buffer: usize) {

}

/// initImage - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn initImage(image: usize, initialLayout: usize) {

}

/// initSparseImage - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn initSparseImage(image: usize) {

}

/// invalidateBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn invalidateBuffer(buffer: usize, slice: usize) {

}

/// ensureBufferAddress - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn ensureBufferAddress(buffer: usize) {

}

/// invalidateImage - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn invalidateImage(image: usize, slice: usize, layout: usize) {

}

/// invalidateImageWithUsage - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn invalidateImageWithUsage(image: usize, slice: usize, usageInfo: usize, layout: usize) {

}

/// ensureImageCompatibility - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn ensureImageCompatibility(image: usize, usageInfo: usize) -> usize {
    0
}

/// pushData - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn pushData(stages: usize, offset: u32, size: u32, data: *mut core::ffi::c_void) {

}

/// resolveImage - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn resolveImage(dstImage: usize, srcImage: usize, region: usize, format: usize, mode: usize, stencilMode: usize) {

}

/// transformImage - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn transformImage(dstImage: usize, dstSubresources: usize, srcLayout: usize, dstLayout: usize) {

}

/// updateBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateBuffer(buffer: usize, offset: u64, size: u64, data: *mut core::ffi::c_void) {

}

/// uploadBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn uploadBuffer(buffer: usize, source: usize, sourceOffset: u64) {

}

/// uploadImage - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn uploadImage(image: usize, source: usize, sourceOffset: u64, subresourceAlignment: u64, format: usize) {

}

/// setViewports - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn setViewports(viewportCount: u32, viewports: *mut core::ffi::c_void) {

}

/// setDepthBias - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn setDepthBias(depthBias: usize) {

}

/// setDepthBiasRepresentation - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn setDepthBiasRepresentation(depthBiasRepresentation: usize) {

}

/// setDepthBounds - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn setDepthBounds(depthBounds: usize) {

}

/// setStencilReference - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn setStencilReference(reference: u32) {

}

/// setInputAssemblyState - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn setInputAssemblyState(ia: usize) {

}

/// setInputLayout - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn setInputLayout(attributeCount: u32, attributes: *mut core::ffi::c_void, bindingCount: u32, bindings: *mut core::ffi::c_void) {

}

/// setRasterizerState - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn setRasterizerState(rs: usize) {

}

/// setMultisampleState - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn setMultisampleState(ms: usize) {

}

/// setDepthStencilState - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn setDepthStencilState(ds: usize) {

}

/// setBlendMode - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn setBlendMode(attachment: u32, blendMode: usize) {

}

/// setBarrierControl - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn setBarrierControl(control: usize) {

}

/// updatePageTable - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updatePageTable(bindInfo: usize, flags: usize) {

}

/// launchCuKernelNVX - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn launchCuKernelNVX(nvxLaunchInfo: usize, arg1: usize, buffers: usize, arg3: usize, images: usize) {

}

/// signalGpuEvent - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn signalGpuEvent(event: usize) {

}

/// signal - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn signal(signal: usize, value: u64) {

}

/// beginDebugLabel - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn beginDebugLabel(label: usize) {

}

/// endDebugLabel - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn endDebugLabel() {

}

/// insertDebugLabel - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn insertDebugLabel(label: usize) {

}

/// blitImageFb - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn blitImageFb(dstView: usize, dstOffsets: *mut core::ffi::c_void, srcView: usize, srcOffsets: *mut core::ffi::c_void, filter: usize) {

}

/// blitImageHw - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn blitImageHw(dstView: usize, dstOffsets: *mut core::ffi::c_void, srcView: usize, srcOffsets: *mut core::ffi::c_void, filter: usize) {

}

/// copyImageBufferData - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyImageBufferData(cmd: usize, image: usize, imageSubresource: usize, imageOffset: usize, imageExtent: usize, imageLayout: usize, bufferSlice: usize, bufferRowAlignment: u64, bufferSliceAlignment: u64) {

}

/// copyBufferToImageHw - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyBufferToImageHw(image: usize, imageSubresource: usize, imageOffset: usize, imageExtent: usize, buffer: usize, bufferOffset: u64, bufferRowAlignment: u64, bufferSliceAlignment: u64) {

}

/// copyBufferToImageFb - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyBufferToImageFb(image: usize, imageSubresource: usize, imageOffset: usize, imageExtent: usize, buffer: usize, bufferOffset: u64, bufferRowAlignment: u64, bufferSliceAlignment: u64, bufferFormat: usize) {

}

/// copyImageToBufferHw - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyImageToBufferHw(buffer: usize, bufferOffset: u64, bufferRowAlignment: u64, bufferSliceAlignment: u64, image: usize, imageSubresource: usize, imageOffset: usize, imageExtent: usize) {

}

/// copyImageToBufferCs - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyImageToBufferCs(buffer: usize, bufferOffset: u64, bufferRowAlignment: u64, bufferSliceAlignment: u64, bufferFormat: usize, image: usize, imageSubresource: usize, imageOffset: usize, imageExtent: usize) {

}

/// clearImageViewFb - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn clearImageViewFb(imageView: usize, offset: usize, extent: usize, aspect: usize, value: usize) {

}

/// clearImageViewCs - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn clearImageViewCs(imageView: usize, offset: usize, extent: usize, value: usize) {

}

/// copyImageHw - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyImageHw(dstImage: usize, dstSubresource: usize, dstOffset: usize, srcImage: usize, srcSubresource: usize, srcOffset: usize, extent: usize) {

}

/// copyImageFb - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyImageFb(dstImage: usize, dstSubresource: usize, dstOffset: usize, srcImage: usize, srcSubresource: usize, srcOffset: usize, extent: usize) {

}

/// copyImageClear - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyImageClear(dstImage: usize, dstSubresource: usize, dstOffset: usize, dstExtent: usize, srcImage: usize, srcSubresource: usize) -> usize {
    0
}

/// copyImageInline - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copyImageInline(dstImage: usize, dstSubresource: usize, dstOffset: usize, srcImage: usize, srcSubresource: usize, srcOffset: usize, extent: usize) -> usize {
    0
}

/// copySparsePages - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copySparsePages(sparse: usize, pageCount: u32, pages: *mut u32, buffer: usize, offset: u64) {

}

/// copySparseBufferPages - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copySparseBufferPages(sparse: usize, pageCount: u32, pages: *mut u32, buffer: usize, offset: u64) {

}

/// copySparseImagePages - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn copySparseImagePages(sparse: usize, pageCount: u32, pages: *mut u32, buffer: usize, offset: u64) {

}

/// drawGeneric - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn drawGeneric(count: u32, draws: *mut core::ffi::c_void) {

}

/// drawIndirectGeneric - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn drawIndirectGeneric(offset: u64, count: u32, stride: u32, unroll: usize) {

}

/// drawIndirectCountGeneric - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn drawIndirectCountGeneric(offset: u64, countOffset: u64, maxCount: u32, stride: u32) {

}

/// generateMipmapsHw - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn generateMipmapsHw(imageView: usize, filter: usize) {

}

/// generateMipmapsFb - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn generateMipmapsFb(imageView: usize, filter: usize) {

}

/// generateMipmapsCs - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn generateMipmapsCs(imageView: usize) {

}

/// resolveImageHw - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn resolveImageHw(dstImage: usize, srcImage: usize, region: usize) {

}

/// resolveImageRp - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn resolveImageRp(dstImage: usize, srcImage: usize, region: usize, format: usize, mode: usize, stencilMode: usize, flushClears: usize) {

}

/// resolveImageFb - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn resolveImageFb(dstImage: usize, srcImage: usize, region: usize, format: usize, depthMode: usize, stencilMode: usize) {

}

/// resolveImageClear - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn resolveImageClear(dstImage: usize, srcImage: usize, region: usize, format: usize) -> usize {
    0
}

/// resolveImageInline - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn resolveImageInline(dstImage: usize, srcImage: usize, region: usize, format: usize, depthMode: usize, stencilMode: usize) -> usize {
    0
}

/// uploadImageFb - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn uploadImageFb(image: usize, source: usize, sourceOffset: u64, subresourceAlignment: u64, format: usize) {

}

/// uploadImageHw - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn uploadImageHw(image: usize, source: usize, subresourceAlignment: u64, sourceOffset: u64) {

}

/// determineClearStoreOp - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn determineClearStoreOp(loadOp: usize) -> usize {
    0
}

/// performClears - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn performClears(batch: usize) {

}

/// deferClear - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn deferClear(imageView: usize, clearAspects: usize, clearValue: usize) {

}

/// deferDiscard - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn deferDiscard(imageView: usize, discardAspects: usize) {

}

/// hoistInlineClear - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn hoistInlineClear(clear: usize, attachment: usize, aspect: usize) {

}

/// flushClears - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn flushClears(useRenderPass: usize) {

}

/// flushRenderPassDiscards - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn flushRenderPassDiscards() {

}

/// flushRenderPassResolves - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn flushRenderPassResolves() {

}

/// flushResolves - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn flushResolves() {

}

/// finalizeLoadStoreOps - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn finalizeLoadStoreOps() {

}

/// adjustAttachmentLoadStoreOps - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn adjustAttachmentLoadStoreOps(attachment: usize, access: usize) {

}

/// adjustRenderArea - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn adjustRenderArea(rect: usize) {

}

/// beginRenderPass - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn beginRenderPass() {

}

/// endRenderPass - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn endRenderPass(suspend: usize) {

}

/// endCurrentPass - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn endCurrentPass(suspend: usize) {

}

/// acquireRenderTargets - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn acquireRenderTargets(framebufferInfo: usize, ops: usize) {

}

/// releaseRenderTargets - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn releaseRenderTargets() {

}

/// renderPassStartUnsynchronized - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn renderPassStartUnsynchronized() -> usize {
    0
}

/// renderPassBindFramebuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn renderPassBindFramebuffer(framebufferInfo: usize, ops: usize) {

}

/// renderPassUnbindFramebuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn renderPassUnbindFramebuffer() {

}

/// resetRenderPassOps - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn resetRenderPassOps(renderTargets: usize, renderPassOps: usize) {

}

/// startTransformFeedback - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn startTransformFeedback() {

}

/// pauseTransformFeedback - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn pauseTransformFeedback() {

}

/// unbindComputePipeline - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn unbindComputePipeline() {

}

/// updateComputePipelineState - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateComputePipelineState() -> usize {
    0
}

/// unbindGraphicsPipeline - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn unbindGraphicsPipeline() {

}

/// updateGraphicsPipeline - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateGraphicsPipeline() -> usize {
    0
}

/// updateGraphicsPipelineState - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateGraphicsPipelineState() -> usize {
    0
}

/// getGraphicsPipelineDebugColor - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn getGraphicsPipelineDebugColor() -> u32 {
    0
}

/// invalidateState - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn invalidateState() {

}

/// updateSamplerSet - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateSamplerSet(layout: *mut core::ffi::c_void) {

}

/// updateResourceBindings - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateResourceBindings(layout: *mut core::ffi::c_void) -> usize {
    0
}

/// updateDescriptorSetsBindings - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateDescriptorSetsBindings(layout: *mut core::ffi::c_void) {

}

/// updateDescriptorHeapBindings - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateDescriptorHeapBindings(layout: *mut core::ffi::c_void) -> usize {
    0
}

/// updatePushDataBindings - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updatePushDataBindings(layout: *mut core::ffi::c_void) {

}

/// updateComputeShaderResources - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateComputeShaderResources() {

}

/// updateGraphicsShaderResources - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateGraphicsShaderResources() -> usize {
    0
}

/// makeFramebufferInfo - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn makeFramebufferInfo(renderTargets: usize) -> usize {
    0
}

/// updateRenderTargets - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateRenderTargets() {

}

/// flushDeferredClear - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn flushDeferredClear(image: usize, subresources: usize) -> usize {
    0
}

/// findDeferredClear - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn findDeferredClear(image: usize, subresources: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// findOverlappingDeferredClear - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn findOverlappingDeferredClear(image: usize, subresources: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// findOverlappingDeferredResolve - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn findOverlappingDeferredResolve(image: usize, subresources: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// isBoundAsRenderTarget - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn isBoundAsRenderTarget(image: usize, subresources: usize) -> usize {
    0
}

/// findColorAttachmentIndex - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn findColorAttachmentIndex(image: usize, subresources: usize) -> i32 {
    0
}

/// updateIndexBufferBinding - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateIndexBufferBinding() {

}

/// updateVertexBufferBindings - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateVertexBufferBindings() {

}

/// updateTransformFeedbackBuffers - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateTransformFeedbackBuffers() {

}

/// updateTransformFeedbackState - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateTransformFeedbackState() {

}

/// updateDynamicState - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateDynamicState() {

}

/// updatePushData - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updatePushData() {

}

/// beginComputePass - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn beginComputePass() {

}

/// endComputePass - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn endComputePass() {

}

/// commitComputeState - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn commitComputeState() -> usize {
    0
}

/// commitGraphicsState - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn commitGraphicsState() -> usize {
    0
}

/// checkResourceHazards - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn checkResourceHazards(layout: *mut core::ffi::c_void) -> usize {
    0
}

/// checkComputeHazards - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn checkComputeHazards() -> usize {
    0
}

/// checkGraphicsHazards - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn checkGraphicsHazards() -> usize {
    0
}

/// checkBufferBarrier - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn checkBufferBarrier(bufferSlice: usize, access: usize, accessOp: usize) -> usize {
    0
}

/// checkBufferViewBarrier - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn checkBufferViewBarrier(bufferView: usize, access: usize, accessOp: usize) -> usize {
    0
}

/// checkImageViewBarrier - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn checkImageViewBarrier(imageView: usize, access: usize, accessOp: usize) -> usize {
    0
}

/// DxvkAccessFlags - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn DxvkAccessFlags() -> usize {
    0
}

/// emitMemoryBarrier - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn emitMemoryBarrier(srcStages: usize, srcAccess: usize, dstStages: usize, dstAccess: usize) {

}

/// trackDrawBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn trackDrawBuffer() {

}

/// tryInvalidateDeviceLocalBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn tryInvalidateDeviceLocalBuffer(buffer: usize, copySize: u64) -> usize {
    0
}

/// relocateResources - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn relocateResources(bufferCount: usize, bufferInfos: *mut core::ffi::c_void, imageCount: usize, imageInfos: *mut core::ffi::c_void) {

}

/// relocateQueuedResources - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn relocateQueuedResources() {

}

/// lookupGraphicsPipeline - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn lookupGraphicsPipeline(shaders: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// lookupComputePipeline - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn lookupComputePipeline(shaders: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// resizeDescriptorArrays - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn resizeDescriptorArrays(bindingCount: u32) {

}

/// flushImplicitResolves - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn flushImplicitResolves() {

}

/// beginCurrentCommands - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn beginCurrentCommands() {

}

/// endCurrentCommands - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn endCurrentCommands() {

}

/// splitCommands - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn splitCommands() {

}

/// discardRenderTarget - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn discardRenderTarget(image: usize, subresources: usize) {

}

/// flushImageLayoutTransitions - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn flushImageLayoutTransitions(cmdBuffer: usize) {

}

/// addImageLayoutTransition - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn addImageLayoutTransition(image: usize, subresources: usize, srcLayout: usize, srcStages: usize, srcAccess: usize, dstLayout: usize, dstStages: usize, dstAccess: usize) {

}

/// addImageInitTransition - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn addImageInitTransition(image: usize, subresources: usize, dstLayout: usize, dstStages: usize, dstAccess: usize) {

}

/// trackNonDefaultImageLayout - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn trackNonDefaultImageLayout(image: usize) {

}

/// overlapsRenderTarget - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn overlapsRenderTarget(image: usize, subresources: usize) -> usize {
    0
}

/// restoreImageLayout - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn restoreImageLayout(image: usize, subresources: usize, keepAttachments: usize) -> usize {
    0
}

/// restoreImageLayouts - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn restoreImageLayouts(pred: usize, keepAttachments: usize) {

}

/// prepareSharedImages - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn prepareSharedImages() {

}

/// transitionImageLayout - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn transitionImageLayout(image: usize, subresources: usize, srcStages: usize, srcAccess: usize, dstLayout: usize, dstStages: usize, dstAccess: usize, discard: usize) -> usize {
    0
}

/// acquireResources - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn acquireResources(cmdBuffer: usize, count: usize, batch: *mut core::ffi::c_void, arg3: usize) {

}

/// releaseResources - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn releaseResources(cmdBuffer: usize, count: usize, batch: *mut core::ffi::c_void) {

}

/// syncResources - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn syncResources(cmdBuffer: usize, count: usize, batch: *mut core::ffi::c_void, arg3: usize) {

}

/// accessMemory - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn accessMemory(cmdBuffer: usize, srcStages: usize, srcAccess: usize, dstStages: usize, dstAccess: usize) {

}

/// accessImage - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn accessImage(cmdBuffer: usize, image: usize, subresources: usize, srcLayout: usize, srcStages: usize, srcAccess: usize, accessOp: usize) {

}

/// accessImageRegion - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn accessImageRegion(cmdBuffer: usize, image: usize, subresources: usize, offset: usize, extent: usize, srcLayout: usize, srcStages: usize, srcAccess: usize, accessOp: usize) {

}

/// accessImageTransfer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn accessImageTransfer(image: usize, subresources: usize, srcLayout: usize, srcStages: usize, srcAccess: usize) {

}

/// accessBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn accessBuffer(cmdBuffer: usize, buffer: usize, offset: u64, size: u64, srcStages: usize, srcAccess: usize, accessOp: usize) {

}

/// accessBufferTransfer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn accessBufferTransfer(buffer: usize, srcStages: usize, srcAccess: usize) {

}

/// accessDrawBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn accessDrawBuffer(offset: u64, count: u32, stride: u32, size: u32) {

}

/// accessDrawCountBuffer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn accessDrawCountBuffer(offset: u64) {

}

/// flushBarriers - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn flushBarriers() {

}

/// resourceHasAccess - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn resourceHasAccess(buffer: usize, offset: u64, size: u64, access: usize, accessOp: usize) -> usize {
    0
}

/// prepareOutOfOrderTransfer - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn prepareOutOfOrderTransfer(cmdBuffer: usize, accessCount: usize, accessBatch: *mut core::ffi::c_void) -> usize {
    0
}

/// prepareOutOfOrderTransition - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn prepareOutOfOrderTransition(image: usize) -> usize {
    0
}

/// checkResourceBarrier - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn checkResourceBarrier(pred: usize, access: usize) -> usize {
    0
}

/// pred - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn pred(arg0: usize) -> usize {
    0
}

/// needsDrawBarriers - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn needsDrawBarriers() -> usize {
    0
}

/// beginRenderPassDebugRegion - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn beginRenderPassDebugRegion() {

}

/// beginBarrierControlDebugRegion - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn beginBarrierControlDebugRegion() {

}

/// pushDebugRegion - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn pushDebugRegion(label: usize, arg1: usize) {

}

/// popDebugRegion - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn popDebugRegion(arg0: usize) {

}

/// hasDebugRegion - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn hasDebugRegion(arg0: usize) -> usize {
    0
}

/// beginActiveDebugRegions - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn beginActiveDebugRegions() {

}

/// endActiveDebugRegions - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn endActiveDebugRegions() {

}

/// trackBufferViewBinding - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn trackBufferViewBinding(binding: usize, view: usize) -> usize {
    0
}

/// trackImageViewBinding - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn trackImageViewBinding(binding: usize, view: usize) -> usize {
    0
}

/// formatsAreImageCopyCompatible - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn formatsAreImageCopyCompatible(dstFormat: usize, srcFormat: usize) -> usize {
    0
}

/// convertStencilOp - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn convertStencilOp(op: usize, writable: usize) -> usize {
    0
}

/// formatsAreBufferCopyCompatible - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn formatsAreBufferCopyCompatible(imageFormat: usize, bufferFormat: usize) -> usize {
    0
}

/// formatsAreResolveCompatible - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn formatsAreResolveCompatible(resolveFormat: usize, viewFormat: usize) -> usize {
    0
}

/// sanitizeTexelBufferFormat - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn sanitizeTexelBufferFormat(srcFormat: usize) -> usize {
    0
}

/// add - from dxvk/dxvk_context_state.h
#[no_mangle]
pub unsafe extern "C" fn add(info: usize) {

}

/// exec - from dxvk/dxvk_cs.h
#[no_mangle]
pub unsafe extern "C" fn exec(ctx: *mut core::ffi::c_void) -> usize {
    0
}

/// first - from dxvk/dxvk_cs.h
#[no_mangle]
pub unsafe extern "C" fn first() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pushCmd - from dxvk/dxvk_cs.h
#[no_mangle]
pub unsafe extern "C" fn pushCmd(command: usize, count: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// executeAll - from dxvk/dxvk_cs.h
#[no_mangle]
pub unsafe extern "C" fn executeAll(ctx: *mut core::ffi::c_void) {

}

/// append - from dxvk/dxvk_cs.h
#[no_mangle]
pub unsafe extern "C" fn append(cmd: *mut core::ffi::c_void) {

}

/// dispatchChunk - from dxvk/dxvk_cs.h
#[no_mangle]
pub unsafe extern "C" fn dispatchChunk(chunk: usize) -> u64 {
    0
}

/// injectChunk - from dxvk/dxvk_cs.h
#[no_mangle]
pub unsafe extern "C" fn injectChunk(queue: usize, chunk: usize, synchronize: usize) {

}

/// synchronize - from dxvk/dxvk_cs.h
#[no_mangle]
pub unsafe extern "C" fn synchronize(seq: u64) {

}

/// lastSequenceNumber - from dxvk/dxvk_cs.h
#[no_mangle]
pub unsafe extern "C" fn lastSequenceNumber() -> u64 {
    0
}

/// getHostAddressRange - from dxvk/dxvk_descriptor.h
#[no_mangle]
pub unsafe extern "C" fn getHostAddressRange() -> usize {
    0
}

/// getHeapInfo - from dxvk/dxvk_descriptor_heap.h
#[no_mangle]
pub unsafe extern "C" fn getHeapInfo() -> usize {
    0
}

/// getRangeInfo - from dxvk/dxvk_descriptor_heap.h
#[no_mangle]
pub unsafe extern "C" fn getRangeInfo() -> usize {
    0
}

/// addRanges - from dxvk/dxvk_descriptor_heap.h
#[no_mangle]
pub unsafe extern "C" fn addRanges() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// addCopy - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn addCopy(range: usize) {

}

/// addPadding - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn addPadding(loOffset: u32, hiOffset: u32) {

}

/// getDescriptorSize - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn getDescriptorSize(arg0: usize) -> u32 {
    0
}

/// getCopyFn - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn getCopyFn(alignment: u32, size: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// getPaddingFn - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn getPaddingFn(alignment: u32, size: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// copyGeneric - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn copyGeneric(dst: *mut core::ffi::c_void, descriptor: *mut *mut core::ffi::c_void, range: usize) {

}

/// padGeneric - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn padGeneric(dst: *mut core::ffi::c_void, descriptor: *mut *mut core::ffi::c_void, range: usize) {

}

/// copyAligned - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn copyAligned(dst: *mut core::ffi::c_void, descriptor: *mut *mut core::ffi::c_void, range: usize) {

}

/// padAligned - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn padAligned(dst: *mut core::ffi::c_void, descriptor: *mut *mut core::ffi::c_void, range: usize) {

}

/// padAlignedAnySize - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn padAlignedAnySize(dst: *mut core::ffi::c_void, descriptor: *mut *mut core::ffi::c_void, range: usize) {

}

/// getDescriptorTypeInfo - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn getDescriptorTypeInfo(arg0: usize) -> usize {
    0
}

/// getNullDescriptor - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn getNullDescriptor(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// getDescriptorSetAlignment - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn getDescriptorSetAlignment() -> u64 {
    0
}

/// getMaxDescriptorSize - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn getMaxDescriptorSize() -> u64 {
    0
}

/// initDescriptorHeapProperties - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn initDescriptorHeapProperties(device: *mut core::ffi::c_void) {

}

/// initDescriptorBufferProperties - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn initDescriptorBufferProperties(device: *mut core::ffi::c_void) {

}

/// notifyCompletion - from dxvk/dxvk_descriptor_pool.h
#[no_mangle]
pub unsafe extern "C" fn notifyCompletion(trackingId: u64) {

}

/// updateStats - from dxvk/dxvk_descriptor_pool.h
#[no_mangle]
pub unsafe extern "C" fn updateStats(counters: usize) {

}

/// getSyncHandle - from dxvk/dxvk_descriptor_worker.h
#[no_mangle]
pub unsafe extern "C" fn getSyncHandle() -> usize {
    0
}

/// runWorker - from dxvk/dxvk_descriptor_worker.h
#[no_mangle]
pub unsafe extern "C" fn runWorker() {

}

/// debugFlags - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn debugFlags() -> usize {
    0
}

/// hasDedicatedTransferQueue - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn hasDedicatedTransferQueue() -> usize {
    0
}

/// getSharingMode - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn getSharingMode() -> usize {
    0
}

/// getShaderCompileOptions - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn getShaderCompileOptions() -> usize {
    0
}

/// getDeviceStatus - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn getDeviceStatus() -> i32 {
    0
}

/// queryImageSubresourceLayout - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn queryImageSubresourceLayout(createInfo: usize, subresource: usize) -> usize {
    0
}

/// canUseGraphicsPipelineLibrary - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn canUseGraphicsPipelineLibrary() -> usize {
    0
}

/// canUsePipelineCacheControl - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn canUsePipelineCacheControl() -> usize {
    0
}

/// canUseSampleLocations - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn canUseSampleLocations(samples: usize) -> usize {
    0
}

/// mustTrackPipelineLifetime - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn mustTrackPipelineLifetime() -> usize {
    0
}

/// canUseDescriptorHeap - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn canUseDescriptorHeap() -> usize {
    0
}

/// canUseDescriptorBuffer - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn canUseDescriptorBuffer() -> usize {
    0
}

/// hasCudaInterop - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn hasCudaInterop() -> usize {
    0
}

/// getDefaultFramebufferSize - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn getDefaultFramebufferSize() -> usize {
    0
}

/// getShaderPipelineStages - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn getShaderPipelineStages() -> usize {
    0
}

/// perfHints - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn perfHints() -> usize {
    0
}

/// getStatCounters - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn getStatCounters() -> usize {
    0
}

/// getMemoryStats - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn getMemoryStats(heap: u32) -> usize {
    0
}

/// getSamplerStats - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn getSamplerStats() -> usize {
    0
}

/// getSamplerDescriptorSet - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn getSamplerDescriptorSet() -> usize {
    0
}

/// getSamplerDescriptorHeap - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn getSamplerDescriptorHeap() -> usize {
    0
}

/// getCurrentFrameId - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn getCurrentFrameId() -> u32 {
    0
}

/// registerShader - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn registerShader(shader: usize) {

}

/// requestCompileShader - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn requestCompileShader(shader: usize) {

}

/// presentImage - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn presentImage(presenter: usize, tracker: usize, frameId: u64, status: *mut core::ffi::c_void) {

}

/// submitCommandList - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn submitCommandList(commandList: usize, tracker: usize, frameId: u64, status: *mut core::ffi::c_void) {

}

/// waitForSubmission - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn waitForSubmission(status: *mut core::ffi::c_void) -> i32 {
    0
}

/// waitForFence - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn waitForFence(fence: usize, value: u64) {

}

/// waitForResource - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn waitForResource(resource: usize, access: usize) {

}

/// waitForIdle - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn waitForIdle() {

}

/// getPerfHints - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn getPerfHints() -> usize {
    0
}

/// recycleCommandList - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn recycleCommandList(cmdList: usize) {

}

/// determineShaderOptions - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn determineShaderOptions() {

}

/// testAdapter - from dxvk/dxvk_device_filter.h
#[no_mangle]
pub unsafe extern "C" fn testAdapter(adapter: usize) -> usize {
    0
}

/// getQueueMapping - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn getQueueMapping() -> usize {
    0
}

/// queryDeviceExtensions - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn queryDeviceExtensions(count: *mut u32, extensions: *mut core::ffi::c_void) -> usize {
    0
}

/// queryDeviceQueues - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn queryDeviceQueues(count: *mut u32, queues: *mut core::ffi::c_void) -> usize {
    0
}

/// queryDeviceFeatures - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn queryDeviceFeatures(size: *mut usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// isSuitable - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn isSuitable(errorSize: usize, error: *mut i8) -> usize {
    0
}

/// decodeDriverVersion - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn decodeDriverVersion(driverId: usize, version: u32) -> usize {
    0
}

/// initSupportedExtensions - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn initSupportedExtensions(instance: usize, adapter: usize, deviceInfo: *mut core::ffi::c_void) {

}

/// initSupportedFeatures - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn initSupportedFeatures(instance: usize, adapter: usize, deviceInfo: *mut core::ffi::c_void) {

}

/// initDeviceProperties - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn initDeviceProperties(instance: usize, adapter: usize, deviceInfo: *mut core::ffi::c_void) {

}

/// initQueueProperties - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn initQueueProperties(instance: usize, adapter: usize, deviceInfo: *mut core::ffi::c_void) {

}

/// initMemoryProperties - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn initMemoryProperties(instance: usize, adapter: usize) {

}

/// disableUnusedFeatures - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn disableUnusedFeatures(instance: usize) {

}

/// enableFeaturesAndExtensions - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn enableFeaturesAndExtensions() {

}

/// enableQueues - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn enableQueues() {

}

/// enableQueue - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn enableQueue(queue: usize) {

}

/// findQueueFamily - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn findQueueFamily(mask: usize, flags: usize) -> u32 {
    0
}

/// chainFeatures - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn chainFeatures(extensions: usize, features: usize) {

}

/// chainProperties - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn chainProperties(extensions: usize, properties: usize) {

}

/// copyFeature - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn copyFeature(chain: *mut core::ffi::c_void, extension: *mut core::ffi::c_void, feature: *mut core::ffi::c_void) {

}

/// chainFeature - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn chainFeature(extension: *mut core::ffi::c_void, chain: *mut core::ffi::c_void, feature: *mut core::ffi::c_void) {

}

/// getName - from dxvk/dxvk_extension_provider.h
#[no_mangle]
pub unsafe extern "C" fn getName() -> usize {
    0
}

/// getDeviceExtensions - from dxvk/dxvk_extension_provider.h
#[no_mangle]
pub unsafe extern "C" fn getDeviceExtensions(adapterId: u32) -> usize {
    0
}

/// initDeviceExtensions - from dxvk/dxvk_extension_provider.h
#[no_mangle]
pub unsafe extern "C" fn initDeviceExtensions(instance: *mut core::ffi::c_void) -> usize {
    0
}

/// kmtGlobal - from dxvk/dxvk_fence.h
#[no_mangle]
pub unsafe extern "C" fn kmtGlobal() -> usize {
    0
}

/// getValue - from dxvk/dxvk_fence.h
#[no_mangle]
pub unsafe extern "C" fn getValue() -> u64 {
    0
}

/// enqueueWait - from dxvk/dxvk_fence.h
#[no_mangle]
pub unsafe extern "C" fn enqueueWait(value: u64, event: usize) {

}

/// sharedHandle - from dxvk/dxvk_fence.h
#[no_mangle]
pub unsafe extern "C" fn sharedHandle() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// run - from dxvk/dxvk_fence.h
#[no_mangle]
pub unsafe extern "C" fn run() {

}

/// initKmtHandles - from dxvk/dxvk_fence.h
#[no_mangle]
pub unsafe extern "C" fn initKmtHandles() {

}

/// lookupFormatInfoSlow - from dxvk/dxvk_format.h
#[no_mangle]
pub unsafe extern "C" fn lookupFormatInfoSlow(format: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// getDefaultResolveMode - from dxvk/dxvk_format.h
#[no_mangle]
pub unsafe extern "C" fn getDefaultResolveMode(format: usize) -> usize {
    0
}

/// getLinearFormat - from dxvk/dxvk_format.h
#[no_mangle]
pub unsafe extern "C" fn getLinearFormat(format: usize) -> usize {
    0
}

/// getSampleCount - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn getSampleCount() -> usize {
    0
}

/// getDepthFormat - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn getDepthFormat() -> usize {
    0
}

/// getDepthTarget - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn getDepthTarget() -> usize {
    0
}

/// getColorTarget - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn getColorTarget(arg0: usize) -> usize {
    0
}

/// numAttachments - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn numAttachments() -> u32 {
    0
}

/// getColorAttachmentIndex - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn getColorAttachmentIndex(id: u32) -> i32 {
    0
}

/// findAttachment - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn findAttachment(view: usize) -> i32 {
    0
}

/// isFullSize - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn isFullSize(view: usize) -> usize {
    0
}

/// isWritable - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn isWritable(attachmentIndex: u32, aspects: usize) -> usize {
    0
}

/// getRtInfo - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn getRtInfo() -> usize {
    0
}

/// computeRenderSize - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn computeRenderSize(defaultSize: usize) -> usize {
    0
}

/// computeRenderTargetSize - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn computeRenderTargetSize(renderTarget: usize) -> usize {
    0
}

/// getColorAccess - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn getColorAccess(index: u32) -> usize {
    0
}

/// getAccess - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn getAccess(index: usize, index_1: usize) -> usize {
    0
}

/// getDepthAccess - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn getDepthAccess() -> usize {
    0
}

/// getStencilAccess - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn getStencilAccess() -> usize {
    0
}

/// unifyDepthStencilAccess - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn unifyDepthStencilAccess() {

}

/// merge - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn merge(other: usize) {

}

/// DxvkGpuEvent - from dxvk/dxvk_gpu_event.h
#[no_mangle]
pub unsafe extern "C" fn DxvkGpuEvent(parent: *mut core::ffi::c_void) -> usize {
    0
}

/// assignGpuEvent - from dxvk/dxvk_gpu_event.h
#[no_mangle]
pub unsafe extern "C" fn assignGpuEvent(event: usize) {

}

/// flags - from dxvk/dxvk_gpu_query.h
#[no_mangle]
pub unsafe extern "C" fn flags() -> usize {
    0
}

/// getData - from dxvk/dxvk_gpu_query.h
#[no_mangle]
pub unsafe extern "C" fn getData(queryData: usize) -> usize {
    0
}

/// begin - from dxvk/dxvk_gpu_query.h
#[no_mangle]
pub unsafe extern "C" fn begin() {

}

/// end - from dxvk/dxvk_gpu_query.h
#[no_mangle]
pub unsafe extern "C" fn end() {

}

/// addGpuQuery - from dxvk/dxvk_gpu_query.h
#[no_mangle]
pub unsafe extern "C" fn addGpuQuery(query: usize) {

}

/// enableQuery - from dxvk/dxvk_gpu_query.h
#[no_mangle]
pub unsafe extern "C" fn enableQuery(cmd: usize, query: usize) {

}

/// disableQuery - from dxvk/dxvk_gpu_query.h
#[no_mangle]
pub unsafe extern "C" fn disableQuery(cmd: usize, query: usize) {

}

/// beginQueries - from dxvk/dxvk_gpu_query.h
#[no_mangle]
pub unsafe extern "C" fn beginQueries(cmd: usize, arg1: usize) {

}

/// endQueries - from dxvk/dxvk_gpu_query.h
#[no_mangle]
pub unsafe extern "C" fn endQueries(cmd: usize, arg1: usize) {

}

/// restartQueries - from dxvk/dxvk_gpu_query.h
#[no_mangle]
pub unsafe extern "C" fn restartQueries(cmd: usize, arg1: usize, index: u32) {

}

/// getQueryTypeBit - from dxvk/dxvk_gpu_query.h
#[no_mangle]
pub unsafe extern "C" fn getQueryTypeBit(arg0: usize) -> u32 {
    0
}

/// getQueryTypeIndex - from dxvk/dxvk_gpu_query.h
#[no_mangle]
pub unsafe extern "C" fn getQueryTypeIndex(arg0: usize, index: u32) -> u32 {
    0
}

/// getHandle - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn getHandle() -> usize {
    0
}

/// isLineRendering - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn isLineRendering(shaders: usize, state: usize) -> usize {
    0
}

/// getLinkage - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn getLinkage(shaders: usize, shader: usize, state: usize) -> usize {
    0
}

/// validateShaderType - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn validateShaderType(shader: usize, stage: usize) -> usize {
    0
}

/// getGlobalBarrier - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn getGlobalBarrier(state: usize) -> usize {
    0
}

/// acquirePipeline - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn acquirePipeline() {

}

/// releasePipeline - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn releasePipeline() {

}

/// getBasePipeline - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn getBasePipeline(state: usize) -> usize {
    0
}

/// getOptimizedPipeline - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn getOptimizedPipeline(state: usize) -> usize {
    0
}

/// destroyBasePipelines - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn destroyBasePipelines() {

}

/// destroyOptimizedPipelines - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn destroyOptimizedPipelines() {

}

/// destroyVulkanPipeline - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn destroyVulkanPipeline(pipeline: usize) {

}

/// getShaderCode - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn getShaderCode(shader: usize, linkage: usize) -> usize {
    0
}

/// computeAttachmentMask - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn computeAttachmentMask(state: usize) -> usize {
    0
}

/// validatePipelineState - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn validatePipelineState(state: usize, trusted: usize) -> usize {
    0
}

/// buildPipelineLayout - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn buildPipelineLayout() -> usize {
    0
}

/// VkBool32 - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn VkBool32(arg0: usize) -> usize {
    0
}

/// attributeCount - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn attributeCount() -> u32 {
    0
}

/// bindingCount - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn bindingCount() -> u32 {
    0
}

/// location - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn location() -> u32 {
    0
}

/// VkFormat - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn VkFormat(arg0: usize) -> usize {
    0
}

/// description - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn description() -> usize {
    0
}

/// stride - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn stride() -> u32 {
    0
}

/// inputRate - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn inputRate() -> usize {
    0
}

/// VkVertexInputRate - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn VkVertexInputRate(arg0: usize) -> usize {
    0
}

/// divisor - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn divisor() -> u32 {
    0
}

/// setStride - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn setStride(stride: u32) {

}

/// depthClipEnable - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn depthClipEnable() -> u32 {
    0
}

/// enableAlphaToCoverage - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn enableAlphaToCoverage() -> u32 {
    0
}

/// feedbackLoop - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn feedbackLoop() -> usize {
    0
}

/// VkImageAspectFlags - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn VkImageAspectFlags(arg0: usize) -> usize {
    0
}

/// setFeedbackLoop - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn setFeedbackLoop(feedbackLoop: usize) {

}

/// getColorFormat - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn getColorFormat(index: u32) -> usize {
    0
}

/// decodeColorFormat - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn decodeColorFormat(arg0: usize, arg1: usize) -> usize {
    0
}

/// getDepthStencilFormat - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn getDepthStencilFormat() -> usize {
    0
}

/// decodeDepthStencilFormat - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn decodeDepthStencilFormat(arg0: usize) -> usize {
    0
}

/// decodeDepthStencilAspects - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn decodeDepthStencilAspects(arg0: usize) -> usize {
    0
}

/// encodeDepthStencilAspects - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn encodeDepthStencilAspects(aspects: usize) -> u64 {
    0
}

/// encodeColorFormat - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn encodeColorFormat(format: usize, index: u32) -> u64 {
    0
}

/// srcColorBlendFactor - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn srcColorBlendFactor() -> usize {
    0
}

/// dstColorBlendFactor - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn dstColorBlendFactor() -> usize {
    0
}

/// srcAlphaBlendFactor - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn srcAlphaBlendFactor() -> usize {
    0
}

/// dstAlphaBlendFactor - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn dstAlphaBlendFactor() -> usize {
    0
}

/// state - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn state() -> usize {
    0
}

/// rIndex - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn rIndex() -> u32 {
    0
}

/// gIndex - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn gIndex() -> u32 {
    0
}

/// bIndex - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn bIndex() -> u32 {
    0
}

/// aIndex - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn aIndex() -> u32 {
    0
}

/// mapping - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn mapping() -> usize {
    0
}

/// decodeSwizzle - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn decodeSwizzle(swizzle: u8) -> usize {
    0
}

/// VkComponentSwizzle - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn VkComponentSwizzle(arg0: usize) -> usize {
    0
}

/// useDynamicDepthTest - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn useDynamicDepthTest() -> usize {
    0
}

/// useDynamicDepthBounds - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn useDynamicDepthBounds() -> usize {
    0
}

/// useDynamicStencilTest - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn useDynamicStencilTest() -> usize {
    0
}

/// useDynamicVertexStrides - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn useDynamicVertexStrides() -> usize {
    0
}

/// find - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn find(k: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// Entry - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn Entry(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// forEach - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn forEach(arg0: usize) {

}

/// likely - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn likely(arg0: usize) -> usize {
    0
}

/// image - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn image() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mipLevelExtent - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn mipLevelExtent(level: u32) -> usize {
    0
}

/// subresources - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn subresources() -> usize {
    0
}

/// imageSubresources - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn imageSubresources() -> usize {
    0
}

/// matchesView - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn matchesView(view: usize) -> usize {
    0
}

/// checkSubresourceOverlap - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn checkSubresourceOverlap(view: usize) -> usize {
    0
}

/// isMultisampled - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn isMultisampled() -> usize {
    0
}

/// hasGfxStores - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn hasGfxStores() -> usize {
    0
}

/// updateProperties - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn updateProperties() {

}

/// pickLayout - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn pickLayout(layout: usize) -> usize {
    0
}

/// setLayout - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn setLayout(layout: usize) {

}

/// hasUnifiedLayout - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn hasUnifiedLayout() -> usize {
    0
}

/// isViewCompatible - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn isViewCompatible(format: usize) -> usize {
    0
}

/// getMemoryInfo - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn getMemoryInfo() -> usize {
    0
}

/// getAvailableSubresources - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn getAvailableSubresources() -> usize {
    0
}

/// getSubresourceStartAddress - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn getSubresourceStartAddress(mip: u32, layer: u32) -> u64 {
    0
}

/// getSubresourceEndAddress - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn getSubresourceEndAddress(mip: u32, layer: u32) -> u64 {
    0
}

/// getSubresourceAddressAt - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn getSubresourceAddressAt(mip: u32, layer: u32, coord: usize) -> u64 {
    0
}

/// queryLayout - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn queryLayout(subresource: usize) -> usize {
    0
}

/// trackLayout - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn trackLayout(subresources: usize, layout: usize) {

}

/// copyFormatList - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn copyFormatList(formatCount: u32, formats: *mut core::ffi::c_void) {

}

/// canShareImage - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn canShareImage(device: *mut core::ffi::c_void, createInfo: usize, sharingInfo: usize) -> usize {
    0
}

/// canUseUnifiedLayout - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn canUseUnifiedLayout(device: usize) -> usize {
    0
}

/// computeSubresourceCount - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn computeSubresourceCount() -> u32 {
    0
}

/// hasPendingResolves - from dxvk/dxvk_implicit_resolve.h
#[no_mangle]
pub unsafe extern "C" fn hasPendingResolves() -> usize {
    0
}

/// extractResolve - from dxvk/dxvk_implicit_resolve.h
#[no_mangle]
pub unsafe extern "C" fn extractResolve(resolve: usize) -> usize {
    0
}

/// invalidate - from dxvk/dxvk_implicit_resolve.h
#[no_mangle]
pub unsafe extern "C" fn invalidate(image: usize, subresources: usize) {

}

/// cleanup - from dxvk/dxvk_implicit_resolve.h
#[no_mangle]
pub unsafe extern "C" fn cleanup(trackingId: u64) {

}

/// addResolveOp - from dxvk/dxvk_implicit_resolve.h
#[no_mangle]
pub unsafe extern "C" fn addResolveOp(view: usize) {

}

/// adapterCount - from dxvk/dxvk_instance.h
#[no_mangle]
pub unsafe extern "C" fn adapterCount() -> usize {
    0
}

/// getExtensionList - from dxvk/dxvk_instance.h
#[no_mangle]
pub unsafe extern "C" fn getExtensionList() -> usize {
    0
}

/// initVulkanLoader - from dxvk/dxvk_instance.h
#[no_mangle]
pub unsafe extern "C" fn initVulkanLoader(args: usize) -> usize {
    0
}

/// initAdapters - from dxvk/dxvk_instance.h
#[no_mangle]
pub unsafe extern "C" fn initAdapters() -> usize {
    0
}

/// debugCallback - from dxvk/dxvk_instance.h
#[no_mangle]
pub unsafe extern "C" fn debugCallback(messageSeverity: usize, messageTypes: usize, pCallbackData: *mut core::ffi::c_void, pUserData: *mut core::ffi::c_void) -> usize {
    0
}

/// needsAutoMarkers - from dxvk/dxvk_latency.h
#[no_mangle]
pub unsafe extern "C" fn needsAutoMarkers() -> usize {
    0
}

/// notifyCpuPresentBegin - from dxvk/dxvk_latency.h
#[no_mangle]
pub unsafe extern "C" fn notifyCpuPresentBegin(frameId: u64) -> usize {
    0
}

/// notifyCsRenderBegin - from dxvk/dxvk_latency.h
#[no_mangle]
pub unsafe extern "C" fn notifyCsRenderBegin(frameId: u64) -> usize {
    0
}

/// notifyCsRenderEnd - from dxvk/dxvk_latency.h
#[no_mangle]
pub unsafe extern "C" fn notifyCsRenderEnd(frameId: u64) -> usize {
    0
}

/// notifyCpuPresentEnd - from dxvk/dxvk_latency.h
#[no_mangle]
pub unsafe extern "C" fn notifyCpuPresentEnd(frameId: u64) -> usize {
    0
}

/// notifyQueueSubmit - from dxvk/dxvk_latency.h
#[no_mangle]
pub unsafe extern "C" fn notifyQueueSubmit(frameId: u64) -> usize {
    0
}

/// notifyQueuePresentBegin - from dxvk/dxvk_latency.h
#[no_mangle]
pub unsafe extern "C" fn notifyQueuePresentBegin(frameId: u64) -> usize {
    0
}

/// notifyQueuePresentEnd - from dxvk/dxvk_latency.h
#[no_mangle]
pub unsafe extern "C" fn notifyQueuePresentEnd(frameId: u64, status: i32) -> usize {
    0
}

/// notifyGpuExecutionBegin - from dxvk/dxvk_latency.h
#[no_mangle]
pub unsafe extern "C" fn notifyGpuExecutionBegin(frameId: u64) -> usize {
    0
}

/// notifyGpuExecutionEnd - from dxvk/dxvk_latency.h
#[no_mangle]
pub unsafe extern "C" fn notifyGpuExecutionEnd(frameId: u64) -> usize {
    0
}

/// notifyGpuPresentEnd - from dxvk/dxvk_latency.h
#[no_mangle]
pub unsafe extern "C" fn notifyGpuPresentEnd(frameId: u64) -> usize {
    0
}

/// sleepAndBeginFrame - from dxvk/dxvk_latency.h
#[no_mangle]
pub unsafe extern "C" fn sleepAndBeginFrame(frameId: u64, maxFrameRate: f64) -> usize {
    0
}

/// discardTimings - from dxvk/dxvk_latency.h
#[no_mangle]
pub unsafe extern "C" fn discardTimings() -> usize {
    0
}

/// getStatistics - from dxvk/dxvk_latency.h
#[no_mangle]
pub unsafe extern "C" fn getStatistics(frameId: u64) -> usize {
    0
}

/// sleepNv - from dxvk/dxvk_latency_builtin.h
#[no_mangle]
pub unsafe extern "C" fn sleepNv(frameId: u64, maxFrameRate: f64) -> usize {
    0
}

/// sleepBuiltin - from dxvk/dxvk_latency_builtin.h
#[no_mangle]
pub unsafe extern "C" fn sleepBuiltin(frameId: u64, maxFrameRate: f64) -> usize {
    0
}

/// initFrame - from dxvk/dxvk_latency_builtin.h
#[no_mangle]
pub unsafe extern "C" fn initFrame(frameId: u64) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// findFrame - from dxvk/dxvk_latency_builtin.h
#[no_mangle]
pub unsafe extern "C" fn findFrame(frameId: u64) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// forwardLatencyMarkerNv - from dxvk/dxvk_latency_builtin.h
#[no_mangle]
pub unsafe extern "C" fn forwardLatencyMarkerNv(frameId: u64) -> usize {
    0
}

/// computeFrameInterval - from dxvk/dxvk_latency_builtin.h
#[no_mangle]
pub unsafe extern "C" fn computeFrameInterval(maxFrameRate: f64) -> usize {
    0
}

/// computeIntervalFromRate - from dxvk/dxvk_latency_builtin.h
#[no_mangle]
pub unsafe extern "C" fn computeIntervalFromRate(frameRate: f64) -> usize {
    0
}

/// estimateTime - from dxvk/dxvk_latency_builtin.h
#[no_mangle]
pub unsafe extern "C" fn estimateTime(frames: *mut core::ffi::c_void, frameCount: usize) -> usize {
    0
}

/// setLatencySleepMode - from dxvk/dxvk_latency_reflex.h
#[no_mangle]
pub unsafe extern "C" fn setLatencySleepMode(enableLowLatency: usize, enableBoost: usize, minIntervalUs: u64) {

}

/// setLatencyMarker - from dxvk/dxvk_latency_reflex.h
#[no_mangle]
pub unsafe extern "C" fn setLatencyMarker(appFrameId: u64, marker: usize) {

}

/// latencySleep - from dxvk/dxvk_latency_reflex.h
#[no_mangle]
pub unsafe extern "C" fn latencySleep() {

}

/// getFrameReports - from dxvk/dxvk_latency_reflex.h
#[no_mangle]
pub unsafe extern "C" fn getFrameReports(maxCount: u32, reports: *mut core::ffi::c_void) -> u32 {
    0
}

/// frameIdFromAppFrameId - from dxvk/dxvk_latency_reflex.h
#[no_mangle]
pub unsafe extern "C" fn frameIdFromAppFrameId(appFrameId: u64) -> u64 {
    0
}

/// lookupFrameId - from dxvk/dxvk_latency_reflex.h
#[no_mangle]
pub unsafe extern "C" fn lookupFrameId(appFrameId: u64) -> u64 {
    0
}

/// mapFrameId - from dxvk/dxvk_latency_reflex.h
#[no_mangle]
pub unsafe extern "C" fn mapFrameId(appFrameId: u64, dxvkFrameId: u64) {

}

/// mapFrameTimestampToReportUs - from dxvk/dxvk_latency_reflex.h
#[no_mangle]
pub unsafe extern "C" fn mapFrameTimestampToReportUs(frame: usize, report: usize, timestamp: usize) -> u64 {
    0
}

/// fill - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn fill(info: usize) {

}

/// getBufferInfo - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn getBufferInfo() -> usize {
    0
}

/// getImageInfo - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn getImageInfo() -> usize {
    0
}

/// getMemoryProperties - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn getMemoryProperties() -> usize {
    0
}

/// destroyBufferViews - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn destroyBufferViews() {

}

/// getIncrement - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn getIncrement(access: usize) -> usize {
    0
}

/// recycle - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn recycle(allocation: *mut core::ffi::c_void) {

}

/// computePoolIndex - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn computePoolIndex(size: u64) -> u32 {
    0
}

/// assignCache - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn assignCache(size: u64, allocation: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// getStats - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn getStats() -> usize {
    0
}

/// addResource - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn addResource(resource: usize, allocation: *mut core::ffi::c_void, mode: usize) {

}

/// device - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn device(FAILEDhr: usize) -> usize {
    0
}

/// getBufferMemoryRequirements - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn getBufferMemoryRequirements(createInfo: usize, memoryRequirements: usize) -> usize {
    0
}

/// getImageMemoryRequirements - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn getImageMemoryRequirements(createInfo: usize, memoryRequirements: usize) -> usize {
    0
}

/// registerResource - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn registerResource(resource: *mut core::ffi::c_void) {

}

/// unregisterResource - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn unregisterResource(resource: *mut core::ffi::c_void) {

}

/// requestMakeResident - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn requestMakeResident(resource: *mut core::ffi::c_void) {

}

/// performTimedTasks - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn performTimedTasks() {

}

/// pollRelocationList - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn pollRelocationList(count: u32, size: u64) -> usize {
    0
}

/// assignMemoryDebugName - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn assignMemoryDebugName(memory: usize, arg1: usize) {

}

/// findEmptyChunkInPool - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn findEmptyChunkInPool(pool: usize, minSize: u64, maxSize: u64) -> i32 {
    0
}

/// mapDeviceMemory - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn mapDeviceMemory(memory: usize, properties: usize) {

}

/// determineMaxChunkSize - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn determineMaxChunkSize(arg0: usize, mappable: usize) -> u64 {
    0
}

/// determineSparseMemoryTypes - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn determineSparseMemoryTypes(device: *mut core::ffi::c_void) -> u32 {
    0
}

/// determineBufferUsageFlagsPerMemoryType - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn determineBufferUsageFlagsPerMemoryType() {

}

/// determineMemoryTypesWithPropertyFlags - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn determineMemoryTypesWithPropertyFlags() {

}

/// getBufferDeviceAddress - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn getBufferDeviceAddress(buffer: usize) -> usize {
    0
}

/// getMemoryTypeMask - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn getMemoryTypeMask(properties: usize) -> u32 {
    0
}

/// findGlobalBufferMemoryTypeMask - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn findGlobalBufferMemoryTypeMask(usage: usize) -> u32 {
    0
}

/// updateMemoryHeapBudgets - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn updateMemoryHeapBudgets() {

}

/// updateMemoryHeapStats - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn updateMemoryHeapStats(heapIndex: u32) {

}

/// moveDefragChunk - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn moveDefragChunk(arg0: usize) {

}

/// pickDefragChunk - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn pickDefragChunk(arg0: usize) {

}

/// evictResources - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn evictResources(arg0: usize) {

}

/// enableDefrag - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn enableDefrag() -> usize {
    0
}

/// getPipeline - from dxvk/dxvk_meta_blit.h
#[no_mangle]
pub unsafe extern "C" fn getPipeline(key: usize) -> usize {
    0
}

/// determineWorkgroupSize - from dxvk/dxvk_meta_clear.h
#[no_mangle]
pub unsafe extern "C" fn determineWorkgroupSize(key: usize) -> usize {
    0
}

/// getCopyImageFormats - from dxvk/dxvk_meta_copy.h
#[no_mangle]
pub unsafe extern "C" fn getCopyImageFormats(dstFormat: usize, dstAspect: usize, srcFormat: usize, srcAspect: usize) -> usize {
    0
}

/// getSrcViewType - from dxvk/dxvk_meta_mipgen.h
#[no_mangle]
pub unsafe extern "C" fn getSrcViewType() -> usize {
    0
}

/// getPassCount - from dxvk/dxvk_meta_mipgen.h
#[no_mangle]
pub unsafe extern "C" fn getPassCount() -> u32 {
    0
}

/// getTopSubresource - from dxvk/dxvk_meta_mipgen.h
#[no_mangle]
pub unsafe extern "C" fn getTopSubresource() -> usize {
    0
}

/// getBottomSubresource - from dxvk/dxvk_meta_mipgen.h
#[no_mangle]
pub unsafe extern "C" fn getBottomSubresource() -> usize {
    0
}

/// getAllTargetSubresources - from dxvk/dxvk_meta_mipgen.h
#[no_mangle]
pub unsafe extern "C" fn getAllTargetSubresources() -> usize {
    0
}

/// getAllSourceSubresources - from dxvk/dxvk_meta_mipgen.h
#[no_mangle]
pub unsafe extern "C" fn getAllSourceSubresources() -> usize {
    0
}

/// getSourceSubresource - from dxvk/dxvk_meta_mipgen.h
#[no_mangle]
pub unsafe extern "C" fn getSourceSubresource(pass: u32) -> usize {
    0
}

/// computePassExtent - from dxvk/dxvk_meta_mipgen.h
#[no_mangle]
pub unsafe extern "C" fn computePassExtent(passId: u32) -> usize {
    0
}

/// checkFormatSupport - from dxvk/dxvk_meta_mipgen.h
#[no_mangle]
pub unsafe extern "C" fn checkFormatSupport(viewFormat: usize) -> usize {
    0
}

/// queryFormatSupport - from dxvk/dxvk_meta_mipgen.h
#[no_mangle]
pub unsafe extern "C" fn queryFormatSupport(viewFormat: usize) -> usize {
    0
}

/// viewType - from dxvk/dxvk_meta_resolve.h
#[no_mangle]
pub unsafe extern "C" fn viewType(image: usize, subresources: usize, usage: usize) -> usize {
    0
}

/// getModeName - from dxvk/dxvk_meta_resolve.h
#[no_mangle]
pub unsafe extern "C" fn getModeName(mode: usize) -> *mut i8 {
    core::ptr::null_mut()
}

/// parseExtensionList - from dxvk/dxvk_openvr.h
#[no_mangle]
pub unsafe extern "C" fn parseExtensionList(str: usize) -> usize {
    0
}

/// getCompositor - from dxvk/dxvk_openvr.h
#[no_mangle]
pub unsafe extern "C" fn getCompositor() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// shutdown - from dxvk/dxvk_openvr.h
#[no_mangle]
pub unsafe extern "C" fn shutdown() {

}

/// loadLibrary - from dxvk/dxvk_openvr.h
#[no_mangle]
pub unsafe extern "C" fn loadLibrary() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// getSym - from dxvk/dxvk_openvr.h
#[no_mangle]
pub unsafe extern "C" fn getSym(sym: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// loadFunctions - from dxvk/dxvk_openxr.h
#[no_mangle]
pub unsafe extern "C" fn loadFunctions() -> usize {
    0
}

/// dirtyBuffers - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn dirtyBuffers(stages: usize) {

}

/// dirtyViews - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn dirtyViews(stages: usize) {

}

/// dirtySamplers - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn dirtySamplers(stages: usize) {

}

/// dirtyStages - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn dirtyStages(stages: usize) {

}

/// clearStages - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn clearStages(stages: usize) {

}

/// hasDirtyResources - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn hasDirtyResources(stages: usize) -> usize {
    0
}

/// hasDirtySamplers - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn hasDirtySamplers(stages: usize) -> usize {
    0
}

/// hasDirtyVas - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn hasDirtyVas(stages: usize) -> usize {
    0
}

/// testDirtyMask - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn testDirtyMask(mask: u32) -> usize {
    0
}

/// getDirtyStageMask - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getDirtyStageMask(classes: u32) -> usize {
    0
}

/// computeMask - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn computeMask(stages: usize, classes: u32) -> usize {
    0
}

/// getDescriptorType - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getDescriptorType() -> usize {
    0
}

/// VkDescriptorType - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn VkDescriptorType(arg0: usize) -> usize {
    0
}

/// getDescriptorCount - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getDescriptorCount() -> u32 {
    0
}

/// getStageMask - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getStageMask() -> usize {
    0
}

/// VkShaderStageFlags - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn VkShaderStageFlags(arg0: usize) -> usize {
    0
}

/// getResourceIndex - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getResourceIndex() -> u32 {
    0
}

/// getViewType - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getViewType() -> usize {
    0
}

/// isUniformBuffer - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn isUniformBuffer() -> usize {
    0
}

/// usesDescriptor - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn usesDescriptor() -> usize {
    0
}

/// VkAccessFlags - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn VkAccessFlags(arg0: usize) -> usize {
    0
}

/// getAccessOp - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getAccessOp() -> usize {
    0
}

/// getSet - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getSet() -> u32 {
    0
}

/// getBinding - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getBinding() -> u32 {
    0
}

/// getArrayIndex - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getArrayIndex() -> u32 {
    0
}

/// getArrayElement - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getArrayElement(index: u32) -> usize {
    0
}

/// remapBinding - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn remapBinding(set: u32, binding: u32) -> usize {
    0
}

/// lt - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn lt(descriptor: usize) -> usize {
    0
}

/// encodeNumeric - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn encodeNumeric() -> usize {
    0
}

/// isShared - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn isShared() -> usize {
    0
}

/// getSize - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getSize() -> u32 {
    0
}

/// getOffset - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getOffset() -> u32 {
    0
}

/// getResourceDwordMask - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getResourceDwordMask() -> u64 {
    0
}

/// rebase - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn rebase(newOffset: u32, newSize: u32) {

}

/// makeAbsolute - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn makeAbsolute() {

}

/// computeIndex - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn computeIndex(stageMask: usize) -> u32 {
    0
}

/// getBindingCount - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getBindingCount() -> u32 {
    0
}

/// getSetLayout - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getSetLayout() -> usize {
    0
}

/// getSetUpdateTemplate - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getSetUpdateTemplate() -> usize {
    0
}

/// getMemorySize - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getMemorySize() -> u64 {
    0
}

/// initSetLayout - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn initSetLayout(key: usize) {

}

/// initDescriptorBufferUpdate - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn initDescriptorBufferUpdate(key: usize) {

}

/// initDescriptorHeapLayout - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn initDescriptorHeapLayout(key: usize) {

}

/// getType - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getType() -> usize {
    0
}

/// getFlags - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getFlags() -> usize {
    0
}

/// addStages - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn addStages(stageMask: usize) {

}

/// addPushData - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn addPushData(block: usize) {

}

/// setFlags - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn setFlags(flags: usize) {

}

/// getPushDataMask - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getPushDataMask() -> u32 {
    0
}

/// setDescriptorSetLayouts - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn setDescriptorSetLayouts(setCount: u32, setLayouts: *mut *mut core::ffi::c_void) {

}

/// getDescriptorSetCount - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getDescriptorSetCount() -> u32 {
    0
}

/// computeDescriptorSetMask - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn computeDescriptorSetMask() -> u32 {
    0
}

/// getDescriptorSetLayout - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getDescriptorSetLayout(setIndex: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// getBindPoint - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getBindPoint() -> usize {
    0
}

/// getShaderStageMask - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getShaderStageMask() -> usize {
    0
}

/// getPipelineLayout - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getPipelineLayout() -> usize {
    0
}

/// usesSamplerHeap - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn usesSamplerHeap() -> usize {
    0
}

/// getDescriptorOffsetShift - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getDescriptorOffsetShift() -> u32 {
    0
}

/// getDescriptorMemorySize - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getDescriptorMemorySize() -> u64 {
    0
}

/// getPushData - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getPushData() -> usize {
    0
}

/// initMetadata - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn initMetadata(key: usize) {

}

/// initPipelineLayout - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn initPipelineLayout(key: usize) {

}

/// initMappings - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn initMappings(key: usize) {

}

/// addBinding - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn addBinding(srcBinding: usize, dstBinding: usize) {

}

/// mapBinding - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn mapBinding(srcBinding: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mapPushData - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn mapPushData(stage: usize, offset: u32) -> u32 {
    0
}

/// getBindings - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getBindings() -> usize {
    0
}

/// getSamplerHeapBindingCount - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getSamplerHeapBindingCount() -> u32 {
    0
}

/// getSamplerHeapBinding - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getSamplerHeapBinding(index: u32) -> usize {
    0
}

/// addBindings - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn addBindings(bindingCount: u32, bindings: *mut core::ffi::c_void) {

}

/// addSamplerHeap - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn addSamplerHeap(binding: usize) {

}

/// addLayout - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn addLayout(layout: usize) {

}

/// getNonemptyStageMask - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getNonemptyStageMask() -> usize {
    0
}

/// getDirtySetMask - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getDirtySetMask(arg0: usize, state: usize) -> u32 {
    0
}

/// makeBindingRange - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn makeBindingRange(arg0: usize) -> usize {
    0
}

/// getSamplers - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getSamplers(arg0: usize) -> usize {
    0
}

/// getVaBindings - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getVaBindings(arg0: usize) -> usize {
    0
}

/// getHazardousStageMask - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getHazardousStageMask() -> usize {
    0
}

/// getBindingMap - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getBindingMap(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// buildMetadata - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn buildMetadata(builder: usize) {

}

/// getPipelineLayoutFlags - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getPipelineLayoutFlags(arg0: usize, builder: usize) -> usize {
    0
}

/// computeStateMask - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn computeStateMask(binding: usize) -> u32 {
    0
}

/// computeSetForBinding - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn computeSetForBinding(arg0: usize, binding: usize) -> u32 {
    0
}

/// computeSetMaskAndCount - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn computeSetMaskAndCount(arg0: usize, stageMask: usize, bindings: usize) -> usize {
    0
}

/// appendDescriptors - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn appendDescriptors(list: usize, binding: usize, mapping: usize) {

}

/// compilePipelineLibrary - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn compilePipelineLibrary(library: *mut core::ffi::c_void, priority: usize) {

}

/// compileGraphicsPipeline - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn compileGraphicsPipeline(pipeline: *mut core::ffi::c_void, state: usize, priority: usize) {

}

/// stopWorkers - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn stopWorkers() {

}

/// notifyWorkers - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn notifyWorkers(priority: usize) {

}

/// startWorkers - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn startWorkers() {

}

/// getPipelineCount - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn getPipelineCount() -> usize {
    0
}

/// getWorkerStats - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn getWorkerStats() -> usize {
    0
}

/// findPipelineLibrary - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn findPipelineLibrary(key: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// checkSwapChainStatus - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn checkSwapChainStatus() -> i32 {
    0
}

/// acquireNextImage - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn acquireNextImage(sync: usize, image: usize) -> i32 {
    0
}

/// signalFrame - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn signalFrame(frameId: u64, tracker: usize) {

}

/// setSyncInterval - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn setSyncInterval(syncInterval: u32) {

}

/// setFrameRateLimit - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn setFrameRateLimit(frameRate: f64, maxLatency: u32) {

}

/// setSurfaceFormat - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn setSurfaceFormat(format: usize) {

}

/// setSurfaceExtent - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn setSurfaceExtent(extent: usize) {

}

/// setHdrMetadata - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn setHdrMetadata(hdrMetadata: usize) {

}

/// supportsColorSpace - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn supportsColorSpace(colorspace: usize) -> usize {
    0
}

/// invalidateSurface - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn invalidateSurface() {

}

/// destroyResources - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn destroyResources() {

}

/// setLatencySleepModeNv - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn setLatencySleepModeNv(sleepMode: usize) {

}

/// setLatencyMarkerNv - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn setLatencyMarkerNv(frameId: u64, marker: usize) -> usize {
    0
}

/// latencySleepNv - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn latencySleepNv() -> usize {
    0
}

/// getLatencyTimingsNv - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn getLatencyTimingsNv(timingCount: u32, timings: *mut core::ffi::c_void) -> u32 {
    0
}

/// updateSwapChain - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn updateSwapChain() {

}

/// getSupportedFormats - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn getSupportedFormats(formats: usize) -> i32 {
    0
}

/// getSupportedPresentModes - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn getSupportedPresentModes(modes: usize) -> i32 {
    0
}

/// getSwapImages - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn getSwapImages(images: usize) -> i32 {
    0
}

/// pickSurfaceFormat - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn pickSurfaceFormat(numSupported: u32, pSupported: *mut core::ffi::c_void, desired: usize) -> usize {
    0
}

/// pickColorSpace - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn pickColorSpace(numSupported: u32, pSupported: *mut core::ffi::c_void, desired: usize) -> usize {
    0
}

/// pickFormat - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn pickFormat(numSupported: u32, pSupported: *mut core::ffi::c_void, colorSpace: usize, format: usize) -> usize {
    0
}

/// pickPresentMode - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn pickPresentMode(numSupported: u32, pSupported: *mut core::ffi::c_void, syncInterval: u32) -> usize {
    0
}

/// pickImageExtent - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn pickImageExtent(caps: usize, desired: usize) -> usize {
    0
}

/// pickImageCount - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn pickImageCount(minImageCount: u32, maxImageCount: u32) -> u32 {
    0
}

/// destroySwapchain - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn destroySwapchain() {

}

/// destroySurface - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn destroySurface() {

}

/// destroyLatencySemaphore - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn destroyLatencySemaphore() {

}

/// waitForSwapchainFence - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn waitForSwapchainFence(sync: usize) {

}

/// softError - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn softError(vr: i32) -> i32 {
    0
}

/// gpuIdleTicks - from dxvk/dxvk_queue.h
#[no_mangle]
pub unsafe extern "C" fn gpuIdleTicks() -> u64 {
    0
}

/// getLastError - from dxvk/dxvk_queue.h
#[no_mangle]
pub unsafe extern "C" fn getLastError() -> i32 {
    0
}

/// present - from dxvk/dxvk_queue.h
#[no_mangle]
pub unsafe extern "C" fn present(presentInfo: usize, latencyInfo: usize, status: *mut core::ffi::c_void) {

}

/// synchronizeSubmission - from dxvk/dxvk_queue.h
#[no_mangle]
pub unsafe extern "C" fn synchronizeSubmission(status: *mut core::ffi::c_void) {

}

/// synchronizeUntil - from dxvk/dxvk_queue.h
#[no_mangle]
pub unsafe extern "C" fn synchronizeUntil(pred: usize) {

}

/// submitCmdLists - from dxvk/dxvk_queue.h
#[no_mangle]
pub unsafe extern "C" fn submitCmdLists() {

}

/// finishCmdLists - from dxvk/dxvk_queue.h
#[no_mangle]
pub unsafe extern "C" fn finishCmdLists() {

}

/// returnObject - from dxvk/dxvk_recycler.h
#[no_mangle]
pub unsafe extern "C" fn returnObject(object: usize) {

}

/// setFilter - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn setFilter(min: usize, mag: usize, mip: usize) {

}

/// setDepthCompare - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn setDepthCompare(enable: usize, op: usize) {

}

/// setReduction - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn setReduction(reduction: usize) {

}

/// setUsePixelCoordinates - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn setUsePixelCoordinates(enable: usize) {

}

/// setLegacyCubeFilter - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn setLegacyCubeFilter(enable: usize) {

}

/// setAddressModes - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn setAddressModes(u_: usize, v_: usize, w_: usize) {

}

/// setLodRange - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn setLodRange(min: f32, max: f32, bias: f32) {

}

/// setBorderColor - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn setBorderColor(color: usize) {

}

/// setViewProperties - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn setViewProperties(mapping: usize, format: usize) {

}

/// trackId - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn trackId(trackingId: u64) -> usize {
    0
}

/// determineBorderColorType - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn determineBorderColorType(info: usize) -> usize {
    0
}

/// swizzleBorderColor - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn swizzleBorderColor(color: usize, mapping: usize) -> usize {
    0
}

/// mapBorderColorComponent - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn mapBorderColorComponent(color: usize, mapping: usize, which: usize) -> f32 {
    0.0
}

/// getDescriptorSetInfo - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn getDescriptorSetInfo() -> usize {
    0
}

/// getDescriptorHeapInfo - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn getDescriptorHeapInfo() -> usize {
    0
}

/// initDescriptorLayout - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn initDescriptorLayout() {

}

/// initDescriptorPool - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn initDescriptorPool() {

}

/// initDescriptorHeap - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn initDescriptorHeap() {

}

/// registerBorderColor - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn registerBorderColor(borderColor: *mut core::ffi::c_void) -> u32 {
    0
}

/// findBorderColorInfo - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn findBorderColorInfo(s: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// releaseSampler - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn releaseSampler(index: i32) {

}

/// appendLru - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn appendLru(sampler: usize, index: i32) {

}

/// removeLru - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn removeLru(sampler: usize, index: i32) {

}

/// getCookie - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn getCookie() -> usize {
    0
}

/// needsCompile - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn needsCompile() -> usize {
    0
}

/// notifyCompile - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn notifyCompile() -> usize {
    0
}

/// getShaderMetadata - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn getShaderMetadata() -> usize {
    0
}

/// compile - from vkd3d-proton/dxbc_library.h
#[no_mangle]
pub unsafe extern "C" fn compile(code: usize, source_name: *const i8, target: *const i8, entry_point: *const i8, arguments: usize, binary: usize) -> usize {
    0
}

/// getCode - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn getCode(bindings: *mut core::ffi::c_void, linkage: *mut core::ffi::c_void) -> usize {
    0
}

/// getStageCount - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn getStageCount() -> u32 {
    0
}

/// getStageInfos - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn getStageInfos() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// addStage - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn addStage(stage: usize, code: usize, specInfo: *mut core::ffi::c_void) {

}

/// getShaderCount - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn getShaderCount() -> u32 {
    0
}

/// getShader - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn getShader(index: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// addShader - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn addShader(shader: usize) {

}

/// getModuleIdentifier - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn getModuleIdentifier(stage: usize) -> usize {
    0
}

/// acquirePipelineHandle - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn acquirePipelineHandle() -> usize {
    0
}

/// releasePipelineHandle - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn releasePipelineHandle() {

}

/// compileShaderPipeline - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn compileShaderPipeline(flags: usize) -> usize {
    0
}

/// compileVertexShaderPipeline - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn compileVertexShaderPipeline(stageInfo: usize, flags: usize) -> usize {
    0
}

/// compileFragmentShaderPipeline - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn compileFragmentShaderPipeline(stageInfo: usize, flags: usize) -> usize {
    0
}

/// compileComputeShaderPipeline - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn compileComputeShaderPipeline(stageInfo: usize, flags: usize) -> usize {
    0
}

/// getShaderIdentifier - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn getShaderIdentifier(stage: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// notifyLibraryCompile - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn notifyLibraryCompile() {

}

/// compileShaders - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn compileShaders() {

}

/// getPipelineLibraryLayout - from dxvk/dxvk_shader.h
#[no_mangle]
pub unsafe extern "C" fn getPipelineLibraryLayout() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// DxvkBuiltInResourceMapping - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn DxvkBuiltInResourceMapping(layout: *mut core::ffi::c_void) -> usize {
    0
}

/// mapDescriptor - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn mapDescriptor(arg0: usize, regSpace: u32, regIndex: u32) -> usize {
    0
}

/// buildComputeShader - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn buildComputeShader(builder: usize, groupSize: usize) -> usize {
    0
}

/// buildPixelShader - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn buildPixelShader(builder: usize) -> usize {
    0
}

/// buildFullscreenVertexShader - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn buildFullscreenVertexShader(builder: usize) -> usize {
    0
}

/// declareImageSrv - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn declareImageSrv(builder: usize, binding: u32, name: *mut i8, viewType: usize, viewFormat: usize, viewAspect: usize, samples: usize) -> usize {
    0
}

/// declareTexelBufferSrv - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn declareTexelBufferSrv(builder: usize, binding: u32, name: *mut i8, viewFormat: usize) -> usize {
    0
}

/// declareBufferSrv - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn declareBufferSrv(builder: usize, binding: u32, name: *mut i8, elementType: usize) -> usize {
    0
}

/// declareImageUav - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn declareImageUav(builder: usize, binding: u32, name: *mut i8, viewType: usize, viewFormat: usize) -> usize {
    0
}

/// declareTexelBufferUav - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn declareTexelBufferUav(builder: usize, binding: u32, name: *mut i8, viewFormat: usize) -> usize {
    0
}

/// declareBufferUav - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn declareBufferUav(builder: usize, binding: u32, name: *mut i8, elementType: usize) -> usize {
    0
}

/// declareSampler - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn declareSampler(builder: usize, pushDataOffset: u32, name: *mut i8) -> usize {
    0
}

/// declareInputTarget - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn declareInputTarget(builder: usize, binding: u32, name: *mut i8, attachment: u32, format: usize, samples: usize) -> usize {
    0
}

/// declarePushData - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn declarePushData(builder: usize, arg1: usize, pushDataOffset: u32, name: *mut i8) -> usize {
    0
}

/// declareBuiltInInput - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn declareBuiltInInput(builder: usize, arg1: usize, builtIn: usize, irInterpolationModes: usize) -> usize {
    0
}

/// declareInput - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn declareInput(builder: usize, arg1: usize, location: u32, name: *mut i8, irInterpolationModes: usize) -> usize {
    0
}

/// emitBoundCheck - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn emitBoundCheck(builder: usize, coord: usize, size: usize, dims: u32) -> usize {
    0
}

/// emitExtractVector - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn emitExtractVector(builder: usize, vector: usize, first: u32, count: u32) -> usize {
    0
}

/// emitConcatVector - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn emitConcatVector(builder: usize, a: usize, b: usize) -> usize {
    0
}

/// emitReplicateScalar - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn emitReplicateScalar(builder: usize, arg1: usize, value: usize) -> usize {
    0
}

/// emitFormatVector - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn emitFormatVector(builder: usize, format: usize, a: usize) -> usize {
    0
}

/// buildLinearToSrgbFn - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn buildLinearToSrgbFn(builder: usize, arg1: usize) -> usize {
    0
}

/// printShader - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn printShader(level: usize, builder: usize) {

}

/// determineSampledType - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn determineSampledType(format: usize, aspect: usize) -> usize {
    0
}

/// determineResourceKind - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn determineResourceKind(viewType: usize, samples: usize) -> usize {
    0
}

/// findEntryPoint - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn findEntryPoint(builder: usize) -> usize {
    0
}

/// findEntryPointFunction - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn findEntryPointFunction(builder: usize) -> usize {
    0
}

/// declareEntryPoint - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn declareEntryPoint(builder: usize, stage: usize) -> usize {
    0
}

/// splitLoad - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn splitLoad(builder: usize, opCode: usize, def: usize) -> usize {
    0
}

/// splitStore - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn splitStore(builder: usize, opCode: usize, var: usize, value: usize) {

}

/// dumpShader - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn dumpShader(size: usize, dwords: *mut u32) {

}

/// getDefaultFilePaths - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn getDefaultFilePaths() -> usize {
    0
}

/// ensureStatus - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn ensureStatus(status: usize) -> usize {
    0
}

/// initialize - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn initialize() -> usize {
    0
}

/// parseLut - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn parseLut() -> usize {
    0
}

/// getVarCount - from dxvk/dxvk_shader_io.h
#[no_mangle]
pub unsafe extern "C" fn getVarCount() -> u32 {
    0
}

/// getVar - from dxvk/dxvk_shader_io.h
#[no_mangle]
pub unsafe extern "C" fn getVar(index: u32) -> usize {
    0
}

/// checkStageCompatibility - from dxvk/dxvk_shader_io.h
#[no_mangle]
pub unsafe extern "C" fn checkStageCompatibility(stage: usize, inputs: usize, prevStage: usize, outputs: usize, matchSemantics: usize) -> usize {
    0
}

/// forVertexBindings - from dxvk/dxvk_shader_io.h
#[no_mangle]
pub unsafe extern "C" fn forVertexBindings(bindingMask: u32) -> usize {
    0
}

/// isBuiltInInputGenerated - from dxvk/dxvk_shader_io.h
#[no_mangle]
pub unsafe extern "C" fn isBuiltInInputGenerated(stage: usize, prevStage: usize, builtIn: usize) -> usize {
    0
}

/// orderBefore - from dxvk/dxvk_shader_io.h
#[no_mangle]
pub unsafe extern "C" fn orderBefore(a: usize, b: usize) -> usize {
    0
}

/// convertShader - from dxvk/dxvk_shader_ir.h
#[no_mangle]
pub unsafe extern "C" fn convertShader(builder: usize) -> usize {
    0
}

/// determineResourceIndex - from dxvk/dxvk_shader_ir.h
#[no_mangle]
pub unsafe extern "C" fn determineResourceIndex(stage: usize, arg1: usize, regSpace: u32, regIndex: u32) -> usize {
    0
}

/// dumpSource - from dxvk/dxvk_shader_ir.h
#[no_mangle]
pub unsafe extern "C" fn dumpSource(path: usize) -> usize {
    0
}

/// convertIr - from dxvk/dxvk_shader_ir.h
#[no_mangle]
pub unsafe extern "C" fn convertIr(reason: *mut i8) {

}

/// serializeIr - from dxvk/dxvk_shader_ir.h
#[no_mangle]
pub unsafe extern "C" fn serializeIr(builder: usize) {

}

/// deserializeIr - from dxvk/dxvk_shader_ir.h
#[no_mangle]
pub unsafe extern "C" fn deserializeIr(builder: usize) {

}

/// dumpSpv - from dxvk/dxvk_shader_ir.h
#[no_mangle]
pub unsafe extern "C" fn dumpSpv(dumpPath: usize) {

}

/// convertPrimitiveType - from dxvk/dxvk_shader_ir.h
#[no_mangle]
pub unsafe extern "C" fn convertPrimitiveType(topology: usize) -> usize {
    0
}

/// convertOutputSwizzle - from dxvk/dxvk_shader_ir.h
#[no_mangle]
pub unsafe extern "C" fn convertOutputSwizzle(mapping: usize) -> usize {
    0
}

/// convertOutputComponent - from dxvk/dxvk_shader_ir.h
#[no_mangle]
pub unsafe extern "C" fn convertOutputComponent(swizzle: usize, identity: usize) -> usize {
    0
}

/// convertShaderStage - from dxvk/dxvk_shader_ir.h
#[no_mangle]
pub unsafe extern "C" fn convertShaderStage(stage: usize) -> usize {
    0
}

/// convertIoMap - from dxvk/dxvk_shader_ir.h
#[no_mangle]
pub unsafe extern "C" fn convertIoMap(io: usize, stage: usize) -> usize {
    0
}

/// stage - from dxvk/dxvk_shader_key.h
#[no_mangle]
pub unsafe extern "C" fn stage() -> usize {
    0
}

/// hasXfb - from dxvk/dxvk_shader_key.h
#[no_mangle]
pub unsafe extern "C" fn hasXfb() -> usize {
    0
}

/// toString - from dxvk/dxvk_shader_key.h
#[no_mangle]
pub unsafe extern "C" fn toString() -> usize {
    0
}

/// getDword - from dxvk/dxvk_shader_key.h
#[no_mangle]
pub unsafe extern "C" fn getDword(dw: *mut u8) -> usize {
    0
}

/// toHex - from dxvk/dxvk_shader_key.h
#[no_mangle]
pub unsafe extern "C" fn toHex(nibble: u8) -> i8 {
    0
}

/// gatherIdOffsets - from dxvk/dxvk_shader_spirv.h
#[no_mangle]
pub unsafe extern "C" fn gatherIdOffsets(code: usize) {

}

/// gatherMetadata - from dxvk/dxvk_shader_spirv.h
#[no_mangle]
pub unsafe extern "C" fn gatherMetadata(code: usize) {

}

/// handleIoVariable - from dxvk/dxvk_shader_spirv.h
#[no_mangle]
pub unsafe extern "C" fn handleIoVariable(code: usize, arg1: usize, storage: usize, varId: u32, member: i32) {

}

/// handleDecoration - from dxvk/dxvk_shader_spirv.h
#[no_mangle]
pub unsafe extern "C" fn handleDecoration(ins: usize, id: u32, member: i32, baseArg: u32) {

}

/// handleDebugName - from dxvk/dxvk_shader_spirv.h
#[no_mangle]
pub unsafe extern "C" fn handleDebugName(code: usize, stringId: u32) {

}

/// patchResourceBindingsAndIoLocations - from dxvk/dxvk_shader_spirv.h
#[no_mangle]
pub unsafe extern "C" fn patchResourceBindingsAndIoLocations(code: usize, bindings: *mut core::ffi::c_void, linkage: *mut core::ffi::c_void) {

}

/// getShaderStage - from dxvk/dxvk_shader_spirv.h
#[no_mangle]
pub unsafe extern "C" fn getShaderStage(code: usize) -> usize {
    0
}

/// eliminateInput - from dxvk/dxvk_shader_spirv.h
#[no_mangle]
pub unsafe extern "C" fn eliminateInput(code: usize, location: u32) {

}

/// emitOutputSwizzles - from dxvk/dxvk_shader_spirv.h
#[no_mangle]
pub unsafe extern "C" fn emitOutputSwizzles(code: usize, outputMask: u32, swizzles: *mut core::ffi::c_void) {

}

/// emitFlatShadingDeclarations - from dxvk/dxvk_shader_spirv.h
#[no_mangle]
pub unsafe extern "C" fn emitFlatShadingDeclarations(code: usize, inputMask: u32) {

}

/// notify - from dxvk/dxvk_signal.h
#[no_mangle]
pub unsafe extern "C" fn notify() {

}

/// DxvkResourceMemoryInfo - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn DxvkResourceMemoryInfo() -> usize {
    0
}

/// acquirePage - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn acquirePage(page: u32) -> usize {
    0
}

/// setCapacity - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn setCapacity(pageCount: u32) {

}

/// releasePage - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn releasePage(page: usize) {

}

/// getBufferHandle - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn getBufferHandle() -> usize {
    0
}

/// getImageHandle - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn getImageHandle() -> usize {
    0
}

/// getPageCount - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn getPageCount() -> u32 {
    0
}

/// getSubresourceCount - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn getSubresourceCount() -> u32 {
    0
}

/// getProperties - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn getProperties() -> usize {
    0
}

/// getPageInfo - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn getPageInfo(page: u32) -> usize {
    0
}

/// computePageIndex - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn computePageIndex(subresource: u32, regionOffset: usize, regionExtent: usize, regionIsLinear: u32, pageIndex: u32) -> u32 {
    0
}

/// getMapping - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn getMapping(page: u32) -> usize {
    0
}

/// updateMapping - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn updateMapping(cmd: *mut core::ffi::c_void, page: u32, mapping: usize) {

}

/// cookie - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn cookie() -> u64 {
    0
}

/// convertRef - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn convertRef(from: usize, to: usize) -> usize {
    0
}

/// getTrackId - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn getTrackId() -> u64 {
    0
}

/// isTracked - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn isTracked(trackingId: u64, access: usize) -> usize {
    0
}

/// resetTracking - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn resetTracking() {

}

/// trackGfxStores - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn trackGfxStores() -> usize {
    0
}

/// requestEviction - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn requestEviction() -> usize {
    0
}

/// requestResidency - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn requestResidency() {

}

/// updateResidencyStatus - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn updateResidencyStatus(residency: usize) {

}

/// makeResourceResident - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn makeResourceResident() {

}

/// DxvkResourceRef - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn DxvkResourceRef(object: *mut core::ffi::c_void, access: usize) -> usize {
    0
}

/// encodeOffset - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn encodeOffset() -> u64 {
    0
}

/// tryMergeMemoryBind - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn tryMergeMemoryBind(oldBind: usize, newBind: usize) -> usize {
    0
}

/// tryMergeImageBind - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn tryMergeImageBind(arg0: usize, oldBind: usize, arg2: usize, newBind: usize) -> usize {
    0
}

/// processBufferBinds - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn processBufferBinds(buffer: usize) {

}

/// processImageBinds - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn processImageBinds(image: usize) {

}

/// processOpaqueBinds - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn processOpaqueBinds(opaque: usize) {

}

/// populateOutputArrays - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn populateOutputArrays(binds: usize, infos: usize, arg2: usize, input: usize) {

}

/// getCtr - from dxvk/dxvk_stats.h
#[no_mangle]
pub unsafe extern "C" fn getCtr(ctr: usize) -> u64 {
    0
}

/// setCtr - from dxvk/dxvk_stats.h
#[no_mangle]
pub unsafe extern "C" fn setCtr(ctr: usize, val: u64) {

}

/// addCtr - from dxvk/dxvk_stats.h
#[no_mangle]
pub unsafe extern "C" fn addCtr(ctr: usize, val: u64) {

}

/// clrCtr - from dxvk/dxvk_stats.h
#[no_mangle]
pub unsafe extern "C" fn clrCtr(ctr: usize) {

}

/// diff - from dxvk/dxvk_stats.h
#[no_mangle]
pub unsafe extern "C" fn diff(other: usize) -> usize {
    0
}

/// setGammaRamp - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn setGammaRamp(cpCount: u32, cpData: *mut core::ffi::c_void) {

}

/// setCursorTexture - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn setCursorTexture(extent: usize, format: usize, data: *mut core::ffi::c_void) {

}

/// setCursorPos - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn setCursorPos(rect: usize) {

}

/// performDraw - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn performDraw(ctx: usize, dstView: usize, dstRect: usize, srcView: usize, srcRect: usize, composite: u32) {

}

/// renderHudImage - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn renderHudImage(ctx: usize, extent: usize) {

}

/// destroyHudImage - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn destroyHudImage() {

}

/// renderCursor - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn renderCursor(ctx: usize, dstView: usize) {

}

/// uploadGammaImage - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn uploadGammaImage(ctx: usize) {

}

/// uploadCursorImage - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn uploadCursorImage(ctx: usize) {

}

/// uploadTexture - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn uploadTexture(ctx: usize, image: usize, buffer: usize) {

}

/// getBlitPipeline - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn getBlitPipeline(key: usize) -> usize {
    0
}

/// getCursorPipeline - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn getCursorPipeline(key: usize) -> usize {
    0
}

/// needsComposition - from dxvk/dxvk_swapchain_blitter.h
#[no_mangle]
pub unsafe extern "C" fn needsComposition(dstView: usize) -> usize {
    0
}

/// bufferInfo - from dxvk/dxvk_unbound.h
#[no_mangle]
pub unsafe extern "C" fn bufferInfo() -> usize {
    0
}

/// samplerInfo - from dxvk/dxvk_unbound.h
#[no_mangle]
pub unsafe extern "C" fn samplerInfo() -> usize {
    0
}

/// computeMipLevelCount - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn computeMipLevelCount(imageSize: usize) -> u32 {
    0
}

/// packImageData - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn packImageData(dstBytes: *mut core::ffi::c_void, srcBytes: *mut core::ffi::c_void, blockCount: usize, blockSize: u64, pitchPerRow: u64, pitchPerLayer: u64) {

}

/// computeMipLevelExtent - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn computeMipLevelExtent(size: usize, level: u32) -> usize {
    0
}

/// computeMipLevelOffset - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn computeMipLevelOffset(offset: usize, level: u32) -> usize {
    0
}

/// flattenImageExtent - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn flattenImageExtent(extent: usize) -> u32 {
    0
}

/// computeImageDataSize - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn computeImageDataSize(format: usize, extent: usize) -> u64 {
    0
}

/// remapComponentMask - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn remapComponentMask(mask: usize, mapping: usize) -> usize {
    0
}

/// invertComponentMapping - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn invertComponentMapping(mapping: usize) -> usize {
    0
}

/// resolveSrcComponentMapping - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn resolveSrcComponentMapping(dstMapping: usize, srcMapping: usize) -> usize {
    0
}

/// remapAlphaToColorBlendFactor - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn remapAlphaToColorBlendFactor(factor: usize) -> usize {
    0
}

/// isIdentityMapping - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn isIdentityMapping(mapping: usize) -> usize {
    0
}

/// getComponentIndex - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn getComponentIndex(component: usize, identity: u32) -> u32 {
    0
}

/// swizzleClearColor - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn swizzleClearColor(color: usize, mapping: usize) -> usize {
    0
}

/// isDualSourceBlendFactor - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn isDualSourceBlendFactor(factor: usize) -> usize {
    0
}

/// setupSampleLocations - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn setupSampleLocations(sampleCount: usize, center: u32) -> usize {
    0
}

/// computeUnorm - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn computeUnorm(f: f32, bits: u32) -> u32 {
    0
}

/// computeSnorm - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn computeSnorm(f: f32, bits: u32) -> u32 {
    0
}

/// int32_t - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn int32_t(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// SpirvCodeBuffer - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn SpirvCodeBuffer(size: u32) -> usize {
    0
}

/// dwords - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn dwords() -> u32 {
    0
}

/// SpirvInstructionIterator - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn SpirvInstructionIterator(arg0: usize) -> usize {
    0
}

/// putWord - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn putWord(word: u32) {

}

/// putIns - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn putIns(opCode: usize, wordCount: u16) {

}

/// putInt32 - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn putInt32(word: u32) {

}

/// putInt64 - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn putInt64(value: u64) {

}

/// putFloat32 - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn putFloat32(value: f32) {

}

/// putFloat64 - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn putFloat64(value: f64) {

}

/// putStr - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn putStr(str: *mut i8) {

}

/// putHeader - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn putHeader(version: u32, boundIds: u32) {

}

/// erase - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn erase(size: usize) {

}

/// getInsertionPtr - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn getInsertionPtr() -> usize {
    0
}

/// beginInsertion - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn beginInsertion(ptr: usize) {

}

/// endInsertion - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn endInsertion() -> usize {
    0
}

/// encodeDword - from dxvk/spirv_compression.h
#[no_mangle]
pub unsafe extern "C" fn encodeDword(dw: u32) {

}

/// decodeDword - from dxvk/spirv_compression.h
#[no_mangle]
pub unsafe extern "C" fn decodeDword(offset: usize) -> u32 {
    0
}

/// opCode - from dxvk/spirv_instruction.h
#[no_mangle]
pub unsafe extern "C" fn opCode() -> usize {
    0
}

/// arg - from dxvk/spirv_instruction.h
#[no_mangle]
pub unsafe extern "C" fn arg(idx: u32) -> u32 {
    0
}

/// chr - from dxvk/spirv_instruction.h
#[no_mangle]
pub unsafe extern "C" fn chr(idx: u32) -> *mut i8 {
    core::ptr::null_mut()
}

/// setArg - from dxvk/spirv_instruction.h
#[no_mangle]
pub unsafe extern "C" fn setArg(idx: u32, word: u32) {

}

/// SpirvInstruction - from dxvk/spirv_instruction.h
#[no_mangle]
pub unsafe extern "C" fn SpirvInstruction(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// advance - from dxvk/spirv_instruction.h
#[no_mangle]
pub unsafe extern "C" fn advance(n: u32) {

}

/// spvVersion - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn spvVersion(major: u32, minor: u32) -> usize {
    0
}

/// SpirvModule - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn SpirvModule(version: u32) -> usize {
    0
}

/// hasCapability - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn hasCapability(capability: usize) -> usize {
    0
}

/// enableCapability - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn enableCapability(capability: usize) {

}

/// enableExtension - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn enableExtension(extensionName: *mut i8) {

}

/// addEntryPoint - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn addEntryPoint(entryPointId: u32, executionModel: usize, name: *mut i8) {

}

/// setMemoryModel - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn setMemoryModel(addressModel: usize, memoryModel: usize) {

}

/// setExecutionMode - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn setExecutionMode(entryPointId: u32, executionMode: usize) {

}

/// setInvocations - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn setInvocations(entryPointId: u32, invocations: u32) {

}

/// setLocalSize - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn setLocalSize(entryPointId: u32, x: u32, y: u32, z: u32) {

}

/// setOutputVertices - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn setOutputVertices(entryPointId: u32, vertexCount: u32) {

}

/// addDebugString - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn addDebugString(string: *mut i8) -> u32 {
    0
}

/// setDebugSource - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn setDebugSource(language: usize, version: u32, file: u32, source: *mut i8) {

}

/// setDebugMemberName - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn setDebugMemberName(structId: u32, memberId: u32, debugName: *mut i8) {

}

/// constBool - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constBool(v: usize) -> u32 {
    0
}

/// consti32 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn consti32(v: i32) -> u32 {
    0
}

/// consti64 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn consti64(v: i64) -> u32 {
    0
}

/// constu32 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constu32(v: u32) -> u32 {
    0
}

/// constu64 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constu64(v: u64) -> u32 {
    0
}

/// constf32 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constf32(v: f32) -> u32 {
    0
}

/// constf64 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constf64(v: f64) -> u32 {
    0
}

/// constvec4i32 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constvec4i32(x: i32, y: i32, z: i32, w: i32) -> u32 {
    0
}

/// constvec4b32 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constvec4b32(x: usize, y: usize, z: usize, w: usize) -> u32 {
    0
}

/// constvec2u32 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constvec2u32(x: u32, y: u32) -> u32 {
    0
}

/// constvec4u32 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constvec4u32(x: u32, y: u32, z: u32, w: u32) -> u32 {
    0
}

/// constvec2f32 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constvec2f32(x: f32, y: f32) -> u32 {
    0
}

/// constvec3f32 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constvec3f32(x: f32, y: f32, z: f32) -> u32 {
    0
}

/// constvec4f32 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constvec4f32(x: f32, y: f32, z: f32, w: f32) -> u32 {
    0
}

/// constfReplicant - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constfReplicant(replicant: f32, count: u32) -> u32 {
    0
}

/// constbReplicant - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constbReplicant(replicant: usize, count: u32) -> u32 {
    0
}

/// constiReplicant - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constiReplicant(replicant: i32, count: u32) -> u32 {
    0
}

/// constuReplicant - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constuReplicant(replicant: i32, count: u32) -> u32 {
    0
}

/// constComposite - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constComposite(typeId: u32, constCount: u32, constIds: *mut u32) -> u32 {
    0
}

/// constUndef - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constUndef(typeId: u32) -> u32 {
    0
}

/// constNull - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn constNull(typeId: u32) -> u32 {
    0
}

/// lateConst32 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn lateConst32(typeId: u32) -> u32 {
    0
}

/// setLateConst - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn setLateConst(constId: u32, argIds: *mut u32) {

}

/// specConstBool - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn specConstBool(v: usize) -> u32 {
    0
}

/// specConst32 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn specConst32(typeId: u32, value: u32) -> u32 {
    0
}

/// decorate - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn decorate(object: u32, decoration: usize) {

}

/// decorateArrayStride - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn decorateArrayStride(object: u32, stride: u32) {

}

/// decorateBinding - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn decorateBinding(object: u32, binding: u32) {

}

/// decorateBuiltIn - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn decorateBuiltIn(object: u32, builtIn: usize) {

}

/// decorateComponent - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn decorateComponent(object: u32, location: u32) {

}

/// decorateDescriptorSet - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn decorateDescriptorSet(object: u32, set: u32) {

}

/// decorateIndex - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn decorateIndex(object: u32, index: u32) {

}

/// decorateLocation - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn decorateLocation(object: u32, location: u32) {

}

/// decorateSpecId - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn decorateSpecId(object: u32, specId: u32) {

}

/// decorateXfb - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn decorateXfb(object: u32, streamId: u32, bufferId: u32, offset: u32, stride: u32) {

}

/// memberDecorateBuiltIn - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn memberDecorateBuiltIn(structId: u32, memberId: u32, builtIn: usize) {

}

/// memberDecorate - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn memberDecorate(structId: u32, memberId: u32, decoration: usize) {

}

/// memberDecorateMatrixStride - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn memberDecorateMatrixStride(structId: u32, memberId: u32, stride: u32) {

}

/// memberDecorateOffset - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn memberDecorateOffset(structId: u32, memberId: u32, offset: u32) {

}

/// defVoidType - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defVoidType() -> u32 {
    0
}

/// defBoolType - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defBoolType() -> u32 {
    0
}

/// defIntType - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defIntType(width: u32, isSigned: u32) -> u32 {
    0
}

/// defFloatType - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defFloatType(width: u32) -> u32 {
    0
}

/// defVectorType - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defVectorType(elementType: u32, elementCount: u32) -> u32 {
    0
}

/// defMatrixType - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defMatrixType(columnType: u32, columnCount: u32) -> u32 {
    0
}

/// defArrayType - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defArrayType(typeId: u32, length: u32) -> u32 {
    0
}

/// defArrayTypeUnique - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defArrayTypeUnique(typeId: u32, length: u32) -> u32 {
    0
}

/// defRuntimeArrayType - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defRuntimeArrayType(typeId: u32) -> u32 {
    0
}

/// defRuntimeArrayTypeUnique - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defRuntimeArrayTypeUnique(typeId: u32) -> u32 {
    0
}

/// defFunctionType - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defFunctionType(returnType: u32, argCount: u32, argTypes: *mut u32) -> u32 {
    0
}

/// defStructType - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defStructType(memberCount: u32, memberTypes: *mut u32) -> u32 {
    0
}

/// defStructTypeUnique - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defStructTypeUnique(memberCount: u32, memberTypes: *mut u32) -> u32 {
    0
}

/// defPointerType - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defPointerType(variableType: u32, storageClass: usize) -> u32 {
    0
}

/// defSamplerType - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defSamplerType() -> u32 {
    0
}

/// defImageType - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defImageType(sampledType: u32, dimensionality: usize, depth: u32, arrayed: u32, multisample: u32, sampled: u32, format: usize) -> u32 {
    0
}

/// defSampledImageType - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defSampledImageType(imageType: u32) -> u32 {
    0
}

/// newVar - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn newVar(pointerType: u32, storageClass: usize) -> u32 {
    0
}

/// newVarInit - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn newVarInit(pointerType: u32, storageClass: usize, initialValue: u32) -> u32 {
    0
}

/// functionBegin - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn functionBegin(returnType: u32, functionId: u32, functionType: u32, functionControl: usize) {

}

/// functionParameter - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn functionParameter(parameterType: u32) -> u32 {
    0
}

/// functionEnd - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn functionEnd() {

}

/// opAccessChain - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAccessChain(resultType: u32, composite: u32, indexCount: u32, indexArray: *mut u32) -> u32 {
    0
}

/// opArrayLength - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opArrayLength(resultType: u32, structure: u32, memberId: u32) -> u32 {
    0
}

/// opAny - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAny(resultType: u32, vector: u32) -> u32 {
    0
}

/// opAll - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAll(resultType: u32, vector: u32) -> u32 {
    0
}

/// opAtomicLoad - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAtomicLoad(resultType: u32, pointer: u32, scope: u32, semantics: u32) -> u32 {
    0
}

/// opAtomicStore - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAtomicStore(pointer: u32, scope: u32, semantics: u32, value: u32) {

}

/// opAtomicExchange - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAtomicExchange(resultType: u32, pointer: u32, scope: u32, semantics: u32, value: u32) -> u32 {
    0
}

/// opAtomicCompareExchange - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAtomicCompareExchange(resultType: u32, pointer: u32, scope: u32, equal: u32, unequal: u32, value: u32, comparator: u32) -> u32 {
    0
}

/// opAtomicIIncrement - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAtomicIIncrement(resultType: u32, pointer: u32, scope: u32, semantics: u32) -> u32 {
    0
}

/// opAtomicIDecrement - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAtomicIDecrement(resultType: u32, pointer: u32, scope: u32, semantics: u32) -> u32 {
    0
}

/// opAtomicIAdd - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAtomicIAdd(resultType: u32, pointer: u32, scope: u32, semantics: u32, value: u32) -> u32 {
    0
}

/// opAtomicISub - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAtomicISub(resultType: u32, pointer: u32, scope: u32, semantics: u32, value: u32) -> u32 {
    0
}

/// opAtomicSMin - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAtomicSMin(resultType: u32, pointer: u32, scope: u32, semantics: u32, value: u32) -> u32 {
    0
}

/// opAtomicSMax - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAtomicSMax(resultType: u32, pointer: u32, scope: u32, semantics: u32, value: u32) -> u32 {
    0
}

/// opAtomicUMin - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAtomicUMin(resultType: u32, pointer: u32, scope: u32, semantics: u32, value: u32) -> u32 {
    0
}

/// opAtomicUMax - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAtomicUMax(resultType: u32, pointer: u32, scope: u32, semantics: u32, value: u32) -> u32 {
    0
}

/// opAtomicAnd - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAtomicAnd(resultType: u32, pointer: u32, scope: u32, semantics: u32, value: u32) -> u32 {
    0
}

/// opAtomicOr - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAtomicOr(resultType: u32, pointer: u32, scope: u32, semantics: u32, value: u32) -> u32 {
    0
}

/// opAtomicXor - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opAtomicXor(resultType: u32, pointer: u32, scope: u32, semantics: u32, value: u32) -> u32 {
    0
}

/// opBitcast - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opBitcast(resultType: u32, operand: u32) -> u32 {
    0
}

/// opBitCount - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opBitCount(resultType: u32, operand: u32) -> u32 {
    0
}

/// opBitReverse - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opBitReverse(resultType: u32, operand: u32) -> u32 {
    0
}

/// opFindILsb - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFindILsb(resultType: u32, operand: u32) -> u32 {
    0
}

/// opFindUMsb - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFindUMsb(resultType: u32, operand: u32) -> u32 {
    0
}

/// opFindSMsb - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFindSMsb(resultType: u32, operand: u32) -> u32 {
    0
}

/// opBitFieldInsert - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opBitFieldInsert(resultType: u32, base: u32, insert: u32, offset: u32, count: u32) -> u32 {
    0
}

/// opBitFieldSExtract - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opBitFieldSExtract(resultType: u32, base: u32, offset: u32, count: u32) -> u32 {
    0
}

/// opBitFieldUExtract - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opBitFieldUExtract(resultType: u32, base: u32, offset: u32, count: u32) -> u32 {
    0
}

/// opBitwiseAnd - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opBitwiseAnd(resultType: u32, operand1: u32, operand2: u32) -> u32 {
    0
}

/// opBitwiseOr - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opBitwiseOr(resultType: u32, operand1: u32, operand2: u32) -> u32 {
    0
}

/// opBitwiseXor - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opBitwiseXor(resultType: u32, operand1: u32, operand2: u32) -> u32 {
    0
}

/// opNot - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opNot(resultType: u32, operand: u32) -> u32 {
    0
}

/// opShiftRightArithmetic - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opShiftRightArithmetic(resultType: u32, base: u32, shift: u32) -> u32 {
    0
}

/// opConvertFtoS - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opConvertFtoS(resultType: u32, operand: u32) -> u32 {
    0
}

/// opConvertFtoU - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opConvertFtoU(resultType: u32, operand: u32) -> u32 {
    0
}

/// opConvertStoF - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opConvertStoF(resultType: u32, operand: u32) -> u32 {
    0
}

/// opConvertUtoF - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opConvertUtoF(resultType: u32, operand: u32) -> u32 {
    0
}

/// opUConvert - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opUConvert(resultType: u32, operand: u32) -> u32 {
    0
}

/// opCompositeConstruct - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opCompositeConstruct(resultType: u32, valueCount: u32, valueArray: *mut u32) -> u32 {
    0
}

/// opCompositeExtract - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opCompositeExtract(resultType: u32, composite: u32, indexCount: u32, indexArray: *mut u32) -> u32 {
    0
}

/// opCompositeInsert - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opCompositeInsert(resultType: u32, object: u32, composite: u32, indexCount: u32, indexArray: *mut u32) -> u32 {
    0
}

/// opDpdx - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opDpdx(resultType: u32, operand: u32) -> u32 {
    0
}

/// opDpdy - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opDpdy(resultType: u32, operand: u32) -> u32 {
    0
}

/// opDpdxCoarse - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opDpdxCoarse(resultType: u32, operand: u32) -> u32 {
    0
}

/// opDpdyCoarse - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opDpdyCoarse(resultType: u32, operand: u32) -> u32 {
    0
}

/// opDpdxFine - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opDpdxFine(resultType: u32, operand: u32) -> u32 {
    0
}

/// opDpdyFine - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opDpdyFine(resultType: u32, operand: u32) -> u32 {
    0
}

/// opVectorExtractDynamic - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opVectorExtractDynamic(resultType: u32, vector: u32, index: u32) -> u32 {
    0
}

/// opVectorShuffle - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opVectorShuffle(resultType: u32, vectorLeft: u32, vectorRight: u32, indexCount: u32, indexArray: *mut u32) -> u32 {
    0
}

/// opSNegate - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSNegate(resultType: u32, operand: u32) -> u32 {
    0
}

/// opFNegate - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFNegate(resultType: u32, operand: u32) -> u32 {
    0
}

/// opSAbs - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSAbs(resultType: u32, operand: u32) -> u32 {
    0
}

/// opFAbs - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFAbs(resultType: u32, operand: u32) -> u32 {
    0
}

/// opFSign - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFSign(resultType: u32, operand: u32) -> u32 {
    0
}

/// opFMix - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFMix(resultType: u32, x: u32, y: u32, a: u32) -> u32 {
    0
}

/// opCross - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opCross(resultType: u32, x: u32, y: u32) -> u32 {
    0
}

/// opIAdd - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opIAdd(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opISub - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opISub(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opFAdd - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFAdd(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opFSub - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFSub(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opSDiv - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSDiv(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opUDiv - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opUDiv(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opSRem - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSRem(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opUMod - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opUMod(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opFDiv - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFDiv(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opIMul - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opIMul(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opFMul - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFMul(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opVectorTimesScalar - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opVectorTimesScalar(resultType: u32, vector: u32, scalar: u32) -> u32 {
    0
}

/// opMatrixTimesMatrix - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opMatrixTimesMatrix(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opMatrixTimesVector - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opMatrixTimesVector(resultType: u32, matrix: u32, vector: u32) -> u32 {
    0
}

/// opVectorTimesMatrix - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opVectorTimesMatrix(resultType: u32, vector: u32, matrix: u32) -> u32 {
    0
}

/// opTranspose - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opTranspose(resultType: u32, matrix: u32) -> u32 {
    0
}

/// opInverse - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opInverse(resultType: u32, matrix: u32) -> u32 {
    0
}

/// opFFma - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFFma(resultType: u32, a: u32, b: u32, c: u32) -> u32 {
    0
}

/// opFMax - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFMax(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opFMin - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFMin(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opNMax - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opNMax(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opNMin - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opNMin(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opSMax - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSMax(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opSMin - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSMin(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opUMax - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opUMax(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opUMin - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opUMin(resultType: u32, a: u32, b: u32) -> u32 {
    0
}

/// opFClamp - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFClamp(resultType: u32, x: u32, minVal: u32, maxVal: u32) -> u32 {
    0
}

/// opNClamp - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opNClamp(resultType: u32, x: u32, minVal: u32, maxVal: u32) -> u32 {
    0
}

/// opIEqual - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opIEqual(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opINotEqual - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opINotEqual(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opSLessThan - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSLessThan(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opSLessThanEqual - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSLessThanEqual(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opSGreaterThan - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSGreaterThan(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opSGreaterThanEqual - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSGreaterThanEqual(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opULessThan - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opULessThan(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opULessThanEqual - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opULessThanEqual(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opUGreaterThan - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opUGreaterThan(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opUGreaterThanEqual - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opUGreaterThanEqual(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opFOrdEqual - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFOrdEqual(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opFUnordNotEqual - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFUnordNotEqual(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opFOrdLessThan - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFOrdLessThan(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opFOrdLessThanEqual - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFOrdLessThanEqual(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opFOrdGreaterThan - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFOrdGreaterThan(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opFOrdGreaterThanEqual - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFOrdGreaterThanEqual(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opDot - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opDot(resultType: u32, vector1: u32, vector2: u32) -> u32 {
    0
}

/// opNormalize - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opNormalize(resultType: u32, operand: u32) -> u32 {
    0
}

/// opRawAccessChain - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opRawAccessChain(resultType: u32, base: u32, stride: u32, index: u32, offset: u32, operand: u32) -> u32 {
    0
}

/// opReflect - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opReflect(resultType: u32, incident: u32, normal: u32) -> u32 {
    0
}

/// opLength - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opLength(resultType: u32, operand: u32) -> u32 {
    0
}

/// opFract - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFract(resultType: u32, operand: u32) -> u32 {
    0
}

/// opCeil - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opCeil(resultType: u32, operand: u32) -> u32 {
    0
}

/// opFloor - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFloor(resultType: u32, operand: u32) -> u32 {
    0
}

/// opRound - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opRound(resultType: u32, operand: u32) -> u32 {
    0
}

/// opRoundEven - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opRoundEven(resultType: u32, operand: u32) -> u32 {
    0
}

/// opTrunc - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opTrunc(resultType: u32, operand: u32) -> u32 {
    0
}

/// opFConvert - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFConvert(resultType: u32, operand: u32) -> u32 {
    0
}

/// opPackHalf2x16 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opPackHalf2x16(resultType: u32, operand: u32) -> u32 {
    0
}

/// opUnpackHalf2x16 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opUnpackHalf2x16(resultType: u32, operand: u32) -> u32 {
    0
}

/// opSelect - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSelect(resultType: u32, condition: u32, operand1: u32, operand2: u32) -> u32 {
    0
}

/// opIsNan - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opIsNan(resultType: u32, operand: u32) -> u32 {
    0
}

/// opFunctionCall - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opFunctionCall(resultType: u32, functionId: u32, argCount: u32, argIds: *mut u32) -> u32 {
    0
}

/// opLabel - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opLabel(labelId: u32) {

}

/// opLoad - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opLoad(typeId: u32, pointerId: u32) -> u32 {
    0
}

/// opStore - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opStore(pointerId: u32, valueId: u32) {

}

/// opInterpolateAtCentroid - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opInterpolateAtCentroid(resultType: u32, interpolant: u32) -> u32 {
    0
}

/// opInterpolateAtSample - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opInterpolateAtSample(resultType: u32, interpolant: u32, sample: u32) -> u32 {
    0
}

/// opInterpolateAtOffset - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opInterpolateAtOffset(resultType: u32, interpolant: u32, offset: u32) -> u32 {
    0
}

/// opImage - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImage(resultType: u32, sampledImage: u32) -> u32 {
    0
}

/// opImageSparseTexelsResident - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageSparseTexelsResident(resultType: u32, residentCode: u32) -> u32 {
    0
}

/// opImageTexelPointer - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageTexelPointer(resultType: u32, image: u32, coordinates: u32, sample: u32) -> u32 {
    0
}

/// opSampledImage - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSampledImage(resultType: u32, image: u32, sampler: u32) -> u32 {
    0
}

/// opImageQuerySizeLod - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageQuerySizeLod(resultType: u32, image: u32, lod: u32) -> u32 {
    0
}

/// opImageQuerySize - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageQuerySize(resultType: u32, image: u32) -> u32 {
    0
}

/// opImageQueryLevels - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageQueryLevels(resultType: u32, image: u32) -> u32 {
    0
}

/// opImageQueryLod - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageQueryLod(resultType: u32, sampledImage: u32, coordinates: u32) -> u32 {
    0
}

/// opImageQuerySamples - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageQuerySamples(resultType: u32, image: u32) -> u32 {
    0
}

/// opImageFetch - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageFetch(resultType: u32, image: u32, coordinates: u32, operands: usize) -> u32 {
    0
}

/// opImageGather - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageGather(resultType: u32, sampledImage: u32, coordinates: u32, component: u32, operands: usize) -> u32 {
    0
}

/// opImageDrefGather - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageDrefGather(resultType: u32, sampledImage: u32, coordinates: u32, reference: u32, operands: usize) -> u32 {
    0
}

/// opImageSampleImplicitLod - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageSampleImplicitLod(resultType: u32, sampledImage: u32, coordinates: u32, operands: usize) -> u32 {
    0
}

/// opImageSampleProjImplicitLod - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageSampleProjImplicitLod(resultType: u32, sampledImage: u32, coordinates: u32, operands: usize) -> u32 {
    0
}

/// opImageSampleDrefImplicitLod - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageSampleDrefImplicitLod(resultType: u32, sampledImage: u32, coordinates: u32, reference: u32, operands: usize) -> u32 {
    0
}

/// opImageSampleProjDrefImplicitLod - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageSampleProjDrefImplicitLod(resultType: u32, sampledImage: u32, coordinates: u32, reference: u32, operands: usize) -> u32 {
    0
}

/// opGroupNonUniformBallot - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opGroupNonUniformBallot(resultType: u32, execution: u32, predicate: u32) -> u32 {
    0
}

/// opGroupNonUniformBallotBitCount - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opGroupNonUniformBallotBitCount(resultType: u32, execution: u32, operation: u32, ballot: u32) -> u32 {
    0
}

/// opGroupNonUniformElect - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opGroupNonUniformElect(resultType: u32, execution: u32) -> u32 {
    0
}

/// opGroupNonUniformBroadcastFirst - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opGroupNonUniformBroadcastFirst(resultType: u32, execution: u32, value: u32) -> u32 {
    0
}

/// opControlBarrier - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opControlBarrier(execution: u32, memory: u32, semantics: u32) {

}

/// opMemoryBarrier - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opMemoryBarrier(memory: u32, semantics: u32) {

}

/// opLoopMerge - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opLoopMerge(mergeBlock: u32, continueTarget: u32, loopControl: u32) {

}

/// opSelectionMerge - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSelectionMerge(mergeBlock: u32, selectionControl: u32) {

}

/// opBranch - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opBranch(label: u32) {

}

/// opBranchConditional - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opBranchConditional(condition: u32, trueLabel: u32, falseLabel: u32) {

}

/// opSwitch - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSwitch(selector: u32, jumpDefault: u32, caseCount: u32, caseLabels: *mut core::ffi::c_void) {

}

/// opPhi - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opPhi(resultType: u32, sourceCount: u32, sourceLabels: *mut core::ffi::c_void) -> u32 {
    0
}

/// opReturn - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opReturn() {

}

/// opDemoteToHelperInvocation - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opDemoteToHelperInvocation() {

}

/// opEmitVertex - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opEmitVertex(streamId: u32) {

}

/// opEndPrimitive - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opEndPrimitive(streamId: u32) {

}

/// defType - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defType(op: usize, argCount: u32, argIds: *mut u32) -> u32 {
    0
}

/// defConst - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn defConst(op: usize, typeId: u32, argCount: u32, argIds: *mut u32) -> u32 {
    0
}

/// instImportGlsl450 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn instImportGlsl450() {

}

/// getMemoryOperandWordCount - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn getMemoryOperandWordCount(op: usize) -> u32 {
    0
}

/// putMemoryOperands - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn putMemoryOperands(op: usize) {

}

/// getImageOperandWordCount - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn getImageOperandWordCount(op: usize) -> u32 {
    0
}

/// putImageOperands - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn putImageOperands(op: usize) {

}

/// detach - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn detach() {

}

/// joinable - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn joinable() -> usize {
    0
}

/// get_id - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn get_id() -> usize {
    0
}

/// native_handle - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn native_handle() -> usize {
    0
}

/// swap - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn swap(other: usize) {

}

/// join - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn join() {

}

/// set_priority - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn set_priority(priority: usize) {

}

/// hardware_concurrency - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn hardware_concurrency() -> u32 {
    0
}

/// notify_one - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn notify_one() {

}

/// notify_all - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn notify_all() {

}

/// wait_until - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn wait_until(lock: usize, arg1: usize, time: usize) -> usize {
    0
}

/// wait_for - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn wait_for(lock: usize, arg1: usize, timeout: usize) -> usize {
    0
}

/// cast - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn cast(src: usize) -> usize {
    0
}

/// extract - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn extract(value: usize, fst: u32, lst: u32) -> usize {
    0
}

/// popcnt - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn popcnt(n: usize) -> usize {
    0
}

/// tzcnt - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn tzcnt(n: u32) -> u32 {
    0
}

/// _tzcnt_u32 - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn _tzcnt_u32(arg0: usize) -> usize {
    0
}

/// __tzcnt_u32 - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn __tzcnt_u32(arg0: usize) -> usize {
    0
}

/// __tzcnt_u64 - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn __tzcnt_u64(arg0: usize) -> usize {
    0
}

/// bsf - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn bsf(n: u32) -> u32 {
    0
}

/// lzcnt - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn lzcnt(n: u32) -> u32 {
    0
}

/// _lzcnt_u32 - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn _lzcnt_u32(arg0: usize) -> usize {
    0
}

/// _lzcnt_u64 - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn _lzcnt_u64(arg0: usize) -> usize {
    0
}

/// bclear - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn bclear(mem: *mut core::ffi::c_void, size: usize) {

}

/// bcmpeq - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn bcmpeq(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> usize {
    0
}

/// bitset - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn bitset() -> usize {
    0
}

/// exchange - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn exchange(idx: u32, value: usize) -> usize {
    0
}

/// flip - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn flip(idx: u32) -> usize {
    0
}

/// setAll - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn setAll() -> usize {
    0
}

/// clearAll - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn clearAll() -> usize {
    0
}

/// any - from vkd3d-proton/sampler_feedback_decode.h
#[no_mangle]
pub unsafe extern "C" fn any(uvec4HORIZ: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// bitCount - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn bitCount() -> usize {
    0
}

/// dwordCount - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn dwordCount() -> usize {
    0
}

/// setN - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn setN(bits: u32) -> usize {
    0
}

/// ensureSize - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn ensureSize(bitCount: u32) {

}

/// iterator - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn iterator(flags: usize) -> usize {
    0
}

/// BitMask - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn BitMask(n: usize) -> usize {
    0
}

/// encodeFixed - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn encodeFixed(n: f32) -> usize {
    0
}

/// decodeFixed - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn decodeFixed(n: usize) -> f32 {
    0.0
}

/// split2 - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn split2(c: u32) -> u32 {
    0
}

/// split3 - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn split3(c: u64) -> u64 {
    0
}

/// interleave - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn interleave(x: u16, y: u16) -> u32 {
    0
}

/// uint48_t - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn uint48_t(n: u64) -> usize {
    0
}

/// fnv1a_init - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn fnv1a_init() -> u64 {
    0
}

/// fnv1a_iter - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn fnv1a_iter(hash: u64, value: usize) -> u64 {
    0
}

/// fnv1a_hash - from dxvk/util_bit.h
#[no_mangle]
pub unsafe extern "C" fn fnv1a_hash(data: *mut u8, size: usize) -> u64 {
    0
}

/// is32BitHostPlatform - from dxvk/util_env.h
#[no_mangle]
pub unsafe extern "C" fn is32BitHostPlatform() -> usize {
    0
}

/// getEnvVar - from dxvk/util_env.h
#[no_mangle]
pub unsafe extern "C" fn getEnvVar(name: *mut i8) -> usize {
    0
}

/// matchFileExtension - from dxvk/util_env.h
#[no_mangle]
pub unsafe extern "C" fn matchFileExtension(name: usize, ext: *mut i8) -> usize {
    0
}

/// getExeName - from dxvk/util_env.h
#[no_mangle]
pub unsafe extern "C" fn getExeName() -> usize {
    0
}

/// getExeBaseName - from dxvk/util_env.h
#[no_mangle]
pub unsafe extern "C" fn getExeBaseName() -> usize {
    0
}

/// getExePath - from dxvk/util_env.h
#[no_mangle]
pub unsafe extern "C" fn getExePath() -> usize {
    0
}

/// all - from dxvk/util_flags.h
#[no_mangle]
pub unsafe extern "C" fn all(fx: usize) -> usize {
    0
}

/// isClear - from dxvk/util_flags.h
#[no_mangle]
pub unsafe extern "C" fn isClear() -> usize {
    0
}

/// clrAll - from dxvk/util_flags.h
#[no_mangle]
pub unsafe extern "C" fn clrAll() {

}

/// Flags - from dxvk/util_flags.h
#[no_mangle]
pub unsafe extern "C" fn Flags(otherm_bits: usize) -> usize {
    0
}

/// bit - from dxvk/util_flags.h
#[no_mangle]
pub unsafe extern "C" fn bit(f: usize) -> usize {
    0
}

/// IntType - from dxvk/util_flags.h
#[no_mangle]
pub unsafe extern "C" fn IntType(arg0: usize) -> usize {
    0
}

/// bits - from dxvk/util_flags.h
#[no_mangle]
pub unsafe extern "C" fn bits(f: usize, fx: usize) -> usize {
    0
}

/// getPendingType - from dxvk/util_flush.h
#[no_mangle]
pub unsafe extern "C" fn getPendingType() -> usize {
    0
}

/// considerFlush - from dxvk/util_flush.h
#[no_mangle]
pub unsafe extern "C" fn considerFlush(flushType: usize, chunkId: u64, lastCompleteSubmissionId: u32, estimatedCost: u64) -> usize {
    0
}

/// notifyFlush - from dxvk/util_flush.h
#[no_mangle]
pub unsafe extern "C" fn notifyFlush(chunkId: u64, submissionId: u64) {

}

/// setTargetFrameRate - from dxvk/util_fps_limiter.h
#[no_mangle]
pub unsafe extern "C" fn setTargetFrameRate(frameRate: f64, maxLatency: u32) {

}

/// delay - from dxvk/util_fps_limiter.h
#[no_mangle]
pub unsafe extern "C" fn delay() {

}

/// testRefreshHeuristic - from dxvk/util_fps_limiter.h
#[no_mangle]
pub unsafe extern "C" fn testRefreshHeuristic(interval: usize, now: usize, maxLatency: u32) -> usize {
    0
}

/// D3DKMTCloseAdapter - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTCloseAdapter(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTDestroyDCFromMemory - from dxvk/util_gdi.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTDestroyDCFromMemory(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTDestroyDevice - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTDestroyDevice(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTDestroySynchronizationObject - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTDestroySynchronizationObject(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTEscape - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTEscape(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTOpenAdapterFromLuid - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTOpenAdapterFromLuid(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTOpenResource2 - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTOpenResource2(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTOpenResourceFromNtHandle - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTOpenResourceFromNtHandle(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTOpenSynchronizationObject - from dxvk/util_gdi.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTOpenSynchronizationObject(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTOpenSyncObjectFromNtHandle - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTOpenSyncObjectFromNtHandle(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTQueryResourceInfo - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTQueryResourceInfo(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTQueryResourceInfoFromNtHandle - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTQueryResourceInfoFromNtHandle(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DKMTShareObjects - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn D3DKMTShareObjects(count: u32, handles: *mut core::ffi::c_void, attr: *mut core::ffi::c_void, access: u32, handle: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// insert - from dxvk/util_lru.h
#[no_mangle]
pub unsafe extern "C" fn insert(value: usize) {

}

/// touch - from dxvk/util_lru.h
#[no_mangle]
pub unsafe extern "C" fn touch(value: usize) {

}

/// leastRecentlyUsedIter - from dxvk/util_lru.h
#[no_mangle]
pub unsafe extern "C" fn leastRecentlyUsedIter() -> usize {
    0
}

/// leastRecentlyUsedEndIter - from dxvk/util_lru.h
#[no_mangle]
pub unsafe extern "C" fn leastRecentlyUsedEndIter() -> usize {
    0
}

/// clamp - from dxvk/util_math.h
#[no_mangle]
pub unsafe extern "C" fn clamp(n: usize, lo: usize, hi: usize) -> usize {
    0
}

/// alignDown - from dxvk/util_math.h
#[no_mangle]
pub unsafe extern "C" fn alignDown(what: usize, to: usize) -> usize {
    0
}

/// fclamp - from dxvk/util_math.h
#[no_mangle]
pub unsafe extern "C" fn fclamp(value: f32, min: f32, max: f32) -> f32 {
    0.0
}

/// divCeil - from dxvk/util_math.h
#[no_mangle]
pub unsafe extern "C" fn divCeil(dividend: usize, divisor: usize) -> usize {
    0
}

/// transpose - from dxvk/util_matrix.h
#[no_mangle]
pub unsafe extern "C" fn transpose(m: usize) -> usize {
    0
}

/// determinant - from dxvk/util_matrix.h
#[no_mangle]
pub unsafe extern "C" fn determinant(m: usize) -> f32 {
    0.0
}

/// inverse - from dxvk/util_matrix.h
#[no_mangle]
pub unsafe extern "C" fn inverse(m: usize) -> usize {
    0
}

/// hadamardProduct - from dxvk/util_matrix.h
#[no_mangle]
pub unsafe extern "C" fn hadamardProduct(a: usize, b: usize) -> usize {
    0
}

/// DecodeD3DCOLOR - from dxvk/util_misc.h
#[no_mangle]
pub unsafe extern "C" fn DecodeD3DCOLOR(color: usize, rgba: *mut f32) {

}

/// computeRefreshPeriod - from dxvk/util_misc.h
#[no_mangle]
pub unsafe extern "C" fn computeRefreshPeriod(numerator: u64, denominator: u64) -> usize {
    0
}

/// unit - from dxvk/util_misc.h
#[no_mangle]
pub unsafe extern "C" fn unit(arg0: usize) -> usize {
    0
}

/// computeRefreshCount - from dxvk/util_misc.h
#[no_mangle]
pub unsafe extern "C" fn computeRefreshCount(t0: usize, t1: usize, refreshPeriod: usize) -> u64 {
    0
}

/// num - from dxvk/util_ratio.h
#[no_mangle]
pub unsafe extern "C" fn num() -> usize {
    0
}

/// denom - from dxvk/util_ratio.h
#[no_mangle]
pub unsafe extern "C" fn denom() -> usize {
    0
}

/// undefined - from dxvk/util_ratio.h
#[no_mangle]
pub unsafe extern "C" fn undefined() -> usize {
    0
}

/// openKmtHandle - from dxvk/util_shared_res.h
#[no_mangle]
pub unsafe extern "C" fn openKmtHandle(kmt_handle: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// setSharedMetadata - from dxvk/util_shared_res.h
#[no_mangle]
pub unsafe extern "C" fn setSharedMetadata(handle: *mut core::ffi::c_void, buf: *mut core::ffi::c_void, bufSize: u32) -> usize {
    0
}

/// getSharedMetadata - from dxvk/util_shared_res.h
#[no_mangle]
pub unsafe extern "C" fn getSharedMetadata(handle: *mut core::ffi::c_void, buf: *mut core::ffi::c_void, bufSize: u32, metadataSize: *mut u32) -> usize {
    0
}

/// sleepFor - from dxvk/util_sleep.h
#[no_mangle]
pub unsafe extern "C" fn sleepFor(t0: usize, arg1: usize, duration: usize) -> usize {
    0
}

/// sleepUntil - from dxvk/util_sleep.h
#[no_mangle]
pub unsafe extern "C" fn sleepUntil(t0: usize, t1: usize) -> usize {
    0
}

/// initializePlatformSpecifics - from dxvk/util_sleep.h
#[no_mangle]
pub unsafe extern "C" fn initializePlatformSpecifics() {

}

/// systemSleep - from dxvk/util_sleep.h
#[no_mangle]
pub unsafe extern "C" fn systemSleep(duration: usize) {

}

/// small_vector - from dxvk/util_small_vector.h
#[no_mangle]
pub unsafe extern "C" fn small_vector(alloc: usize) -> usize {
    0
}

/// ptr - from dxvk/util_small_vector.h
#[no_mangle]
pub unsafe extern "C" fn ptr(arg0: usize) -> usize {
    0
}

/// capacity - from dxvk/util_small_vector.h
#[no_mangle]
pub unsafe extern "C" fn capacity() -> usize {
    0
}

/// is_embedded - from dxvk/util_small_vector.h
#[no_mangle]
pub unsafe extern "C" fn is_embedded() -> usize {
    0
}

/// reserve - from dxvk/util_small_vector.h
#[no_mangle]
pub unsafe extern "C" fn reserve(n: usize) {

}

/// resize - from dxvk/util_small_vector.h
#[no_mangle]
pub unsafe extern "C" fn resize(n: usize, T: usize) {

}

/// push_back - from dxvk/util_small_vector.h
#[no_mangle]
pub unsafe extern "C" fn push_back(object: usize) {

}

/// pop_back - from dxvk/util_small_vector.h
#[no_mangle]
pub unsafe extern "C" fn pop_back() {

}

/// shrink_to_fit - from dxvk/util_small_vector.h
#[no_mangle]
pub unsafe extern "C" fn shrink_to_fit() {

}

/// cbegin - from dxvk/util_small_vector.h
#[no_mangle]
pub unsafe extern "C" fn cbegin() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cend - from dxvk/util_small_vector.h
#[no_mangle]
pub unsafe extern "C" fn cend() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pick_capacity - from dxvk/util_small_vector.h
#[no_mangle]
pub unsafe extern "C" fn pick_capacity(n: usize) -> usize {
    0
}

/// decodeTypedChar - from dxvk/util_string.h
#[no_mangle]
pub unsafe extern "C" fn decodeTypedChar(begin: *mut u8, end: *mut u8, ch: usize) -> *mut u8 {
    core::ptr::null_mut()
}

/// encodeTypedChar - from dxvk/util_string.h
#[no_mangle]
pub unsafe extern "C" fn encodeTypedChar(begin: *mut u8, end: *mut u8, ch: u32) -> usize {
    0
}

/// decodeChar - from dxvk/util_string.h
#[no_mangle]
pub unsafe extern "C" fn decodeChar(begin: *mut core::ffi::c_void, end: *mut core::ffi::c_void, ch: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// encodeChar - from dxvk/util_string.h
#[no_mangle]
pub unsafe extern "C" fn encodeChar(begin: *mut core::ffi::c_void, end: *mut core::ffi::c_void, ch: u32) -> usize {
    0
}

/// transcodeString - from dxvk/util_string.h
#[no_mangle]
pub unsafe extern "C" fn transcodeString(dstBegin: *mut core::ffi::c_void, dstLength: usize, srcBegin: *mut core::ffi::c_void, srcLength: usize) -> usize {
    0
}

/// fromws - from dxvk/util_string.h
#[no_mangle]
pub unsafe extern "C" fn fromws(ws: *mut u16) -> usize {
    0
}

/// tows - from dxvk/util_string.h
#[no_mangle]
pub unsafe extern "C" fn tows(mbs: *mut i8) -> usize {
    0
}

/// topath - from dxvk/util_string.h
#[no_mangle]
pub unsafe extern "C" fn topath(mbs: *mut i8) -> usize {
    0
}

/// format1 - from dxvk/util_string.h
#[no_mangle]
pub unsafe extern "C" fn format1(str: usize, arg: *mut u16, args: usize) {

}

/// strlcpy - from dxvk/util_string.h
#[no_mangle]
pub unsafe extern "C" fn strlcpy(dst: *mut i8, src: *mut i8, count: usize) {

}

/// compareCharsCaseInsensitive - from dxvk/util_string.h
#[no_mangle]
pub unsafe extern "C" fn compareCharsCaseInsensitive(a: i8, b: i8) -> usize {
    0
}

/// compareCaseInsensitive - from dxvk/util_string.h
#[no_mangle]
pub unsafe extern "C" fn compareCaseInsensitive(a: *mut i8, b: *mut i8) -> usize {
    0
}

/// now - from dxvk/util_time.h
#[no_mangle]
pub unsafe extern "C" fn now() -> usize {
    0
}

/// get_time_from_counter - from dxvk/util_time.h
#[no_mangle]
pub unsafe extern "C" fn get_time_from_counter(arg0: usize) -> usize {
    0
}

/// time_point - from dxvk/util_time.h
#[no_mangle]
pub unsafe extern "C" fn time_point(part: usize) -> usize {
    0
}

/// get_frequency - from dxvk/util_time.h
#[no_mangle]
pub unsafe extern "C" fn get_frequency() -> i64 {
    0
}

/// get_counter - from dxvk/util_time.h
#[no_mangle]
pub unsafe extern "C" fn get_counter() -> i64 {
    0
}

/// dot - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dot(arg0: usize, arg1: usize) -> usize {
    0
}

/// lengthSqr - from dxvk/util_vector.h
#[no_mangle]
pub unsafe extern "C" fn lengthSqr(a: usize) -> usize {
    0
}

/// replaceNaN - from dxvk/util_vector.h
#[no_mangle]
pub unsafe extern "C" fn replaceNaN(a: usize) -> usize {
    0
}

/// major - from dxvk/util_version.h
#[no_mangle]
pub unsafe extern "C" fn major() -> u32 {
    0
}

/// minor - from dxvk/util_version.h
#[no_mangle]
pub unsafe extern "C" fn minor() -> u32 {
    0
}

/// patch - from dxvk/util_version.h
#[no_mangle]
pub unsafe extern "C" fn patch() -> u32 {
    0
}

/// dlopen - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dlopen(arg0: usize, arg1: usize) -> usize {
    0
}

/// dlsym - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dlsym(arg0: usize, arg1: usize) -> usize {
    0
}

/// sym - from dxvk/vulkan_loader.h
#[no_mangle]
pub unsafe extern "C" fn sym(instance: usize, name: *mut i8) -> usize {
    0
}

/// getLoaderProc - from dxvk/vulkan_loader.h
#[no_mangle]
pub unsafe extern "C" fn getLoaderProc() -> usize {
    0
}

/// makeSubresourceRange - from dxvk/vulkan_util.h
#[no_mangle]
pub unsafe extern "C" fn makeSubresourceRange(layers: usize) -> usize {
    0
}

/// makeSubresourceLayers - from dxvk/vulkan_util.h
#[no_mangle]
pub unsafe extern "C" fn makeSubresourceLayers(subres: usize) -> usize {
    0
}

/// pickSubresourceLayers - from dxvk/vulkan_util.h
#[no_mangle]
pub unsafe extern "C" fn pickSubresourceLayers(range: usize, level: u32) -> usize {
    0
}

/// pickSubresource - from dxvk/vulkan_util.h
#[no_mangle]
pub unsafe extern "C" fn pickSubresource(range: usize, layer: u32) -> usize {
    0
}

/// getPlaneCount - from dxvk/vulkan_util.h
#[no_mangle]
pub unsafe extern "C" fn getPlaneCount(aspects: usize) -> u32 {
    0
}

/// getPlaneIndex - from dxvk/vulkan_util.h
#[no_mangle]
pub unsafe extern "C" fn getPlaneIndex(aspect: usize) -> u32 {
    0
}

/// getPlaneAspect - from dxvk/vulkan_util.h
#[no_mangle]
pub unsafe extern "C" fn getPlaneAspect(plane: u32) -> usize {
    0
}

/// VkImageAspectFlagBits - from dxvk/vulkan_util.h
#[no_mangle]
pub unsafe extern "C" fn VkImageAspectFlagBits(plane: usize) -> usize {
    0
}

/// getObjectHandle - from dxvk/vulkan_util.h
#[no_mangle]
pub unsafe extern "C" fn getObjectHandle(handle: u64) -> u64 {
    0
}

/// isValidDebugName - from dxvk/vulkan_util.h
#[no_mangle]
pub unsafe extern "C" fn isValidDebugName(name: *mut i8) -> usize {
    0
}

/// makeLabel - from dxvk/vulkan_util.h
#[no_mangle]
pub unsafe extern "C" fn makeLabel(color: u32, text: *mut i8) -> usize {
    0
}

/// scanChain - from dxvk/vulkan_util.h
#[no_mangle]
pub unsafe extern "C" fn scanChain(pNext: *mut core::ffi::c_void, sType: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// iterChain - from dxvk/vulkan_util.h
#[no_mangle]
pub unsafe extern "C" fn iterChain(pNext: *mut core::ffi::c_void, arg1: usize) {

}

/// makeExtension - from dxvk/vulkan_util.h
#[no_mangle]
pub unsafe extern "C" fn makeExtension(name: *mut i8) -> usize {
    0
}

/// NormalizeDisplayMetadata - from dxvk/wsi_edid.h
#[no_mangle]
pub unsafe extern "C" fn NormalizeDisplayMetadata(isHDR: usize, metadata: usize) {

}

/// getDefaultMonitor - from dxvk/wsi_monitor.h
#[no_mangle]
pub unsafe extern "C" fn getDefaultMonitor() -> usize {
    0
}

/// enumMonitors - from dxvk/wsi_monitor.h
#[no_mangle]
pub unsafe extern "C" fn enumMonitors(index: u32) -> usize {
    0
}

/// getDisplayName - from dxvk/wsi_monitor.h
#[no_mangle]
pub unsafe extern "C" fn getDisplayName(hMonitor: usize, Name: u16) -> usize {
    0
}

/// getDesktopCoordinates - from dxvk/wsi_monitor.h
#[no_mangle]
pub unsafe extern "C" fn getDesktopCoordinates(hMonitor: usize, pRect: *mut core::ffi::c_void) -> usize {
    0
}

/// getDisplayMode - from dxvk/wsi_monitor.h
#[no_mangle]
pub unsafe extern "C" fn getDisplayMode(hMonitor: usize, modeNumber: u32, pMode: *mut core::ffi::c_void) -> usize {
    0
}

/// getCurrentDisplayMode - from dxvk/wsi_monitor.h
#[no_mangle]
pub unsafe extern "C" fn getCurrentDisplayMode(hMonitor: usize, pMode: *mut core::ffi::c_void) -> usize {
    0
}

/// getDesktopDisplayMode - from dxvk/wsi_monitor.h
#[no_mangle]
pub unsafe extern "C" fn getDesktopDisplayMode(hMonitor: usize, pMode: *mut core::ffi::c_void) -> usize {
    0
}

/// getMonitorClientSize - from dxvk/wsi_monitor.h
#[no_mangle]
pub unsafe extern "C" fn getMonitorClientSize(hMonitor: usize, pWidth: *mut u32, pHeight: *mut u32) {

}

/// getMonitorEdid - from dxvk/wsi_monitor.h
#[no_mangle]
pub unsafe extern "C" fn getMonitorEdid(hMonitor: usize) -> usize {
    0
}

/// getWindowSize - from dxvk/wsi_platform.h
#[no_mangle]
pub unsafe extern "C" fn getWindowSize(hWindow: *mut core::ffi::c_void, pWidth: *mut u32, pWeight: *mut u32) -> usize {
    0
}

/// resizeWindow - from dxvk/wsi_platform.h
#[no_mangle]
pub unsafe extern "C" fn resizeWindow(hWindow: *mut core::ffi::c_void, pState: *mut core::ffi::c_void, width: u32, weight: u32) -> usize {
    0
}

/// saveWindowState - from dxvk/wsi_platform.h
#[no_mangle]
pub unsafe extern "C" fn saveWindowState(hWindow: *mut core::ffi::c_void, pState: *mut core::ffi::c_void, saveStyle: usize) -> usize {
    0
}

/// restoreWindowState - from dxvk/wsi_platform.h
#[no_mangle]
pub unsafe extern "C" fn restoreWindowState(hWindow: *mut core::ffi::c_void, pState: *mut core::ffi::c_void, restoreCoordinates: usize) -> usize {
    0
}

/// setWindowMode - from dxvk/wsi_platform.h
#[no_mangle]
pub unsafe extern "C" fn setWindowMode(hMonitor: usize, hWindow: *mut core::ffi::c_void, pState: *mut core::ffi::c_void, mode: usize) -> usize {
    0
}

/// enterFullscreenMode - from dxvk/wsi_platform.h
#[no_mangle]
pub unsafe extern "C" fn enterFullscreenMode(hMonitor: usize, hWindow: *mut core::ffi::c_void, pState: *mut core::ffi::c_void, modeSwitch: usize) -> usize {
    0
}

/// leaveFullscreenMode - from dxvk/wsi_platform.h
#[no_mangle]
pub unsafe extern "C" fn leaveFullscreenMode(hWindow: *mut core::ffi::c_void, pState: *mut core::ffi::c_void) -> usize {
    0
}

/// restoreDisplayMode - from dxvk/wsi_platform.h
#[no_mangle]
pub unsafe extern "C" fn restoreDisplayMode() -> usize {
    0
}

/// getWindowMonitor - from dxvk/wsi_platform.h
#[no_mangle]
pub unsafe extern "C" fn getWindowMonitor(hWindow: *mut core::ffi::c_void) -> usize {
    0
}

/// isWindow - from dxvk/wsi_platform.h
#[no_mangle]
pub unsafe extern "C" fn isWindow(hWindow: *mut core::ffi::c_void) -> usize {
    0
}

/// isMinimized - from dxvk/wsi_platform.h
#[no_mangle]
pub unsafe extern "C" fn isMinimized(hWindow: *mut core::ffi::c_void) -> usize {
    0
}

/// isOccluded - from dxvk/wsi_platform.h
#[no_mangle]
pub unsafe extern "C" fn isOccluded(hWindow: *mut core::ffi::c_void) -> usize {
    0
}

/// updateFullscreenWindow - from dxvk/wsi_platform.h
#[no_mangle]
pub unsafe extern "C" fn updateFullscreenWindow(hMonitor: usize, hWindow: *mut core::ffi::c_void, forceTopmost: usize) -> usize {
    0
}

/// unormalize - from dxvk/d3d9_convert_common.h
#[no_mangle]
pub unsafe extern "C" fn unormalize(value: usize, bits: i32) -> f32 {
    0.0
}

/// snormalize - from dxvk/d3d9_convert_common.h
#[no_mangle]
pub unsafe extern "C" fn snormalize(value: i32, bits: i32) -> f32 {
    0.0
}

/// unpackUnorm - from dxvk/d3d9_convert_common.h
#[no_mangle]
pub unsafe extern "C" fn unpackUnorm(p: usize) -> f32 {
    0.0
}

/// unpackUnorm2x8 - from dxvk/d3d9_convert_common.h
#[no_mangle]
pub unsafe extern "C" fn unpackUnorm2x8(p: usize) -> usize {
    0
}

/// vec2 - from dxvk/d3d9_convert_common.h
#[no_mangle]
pub unsafe extern "C" fn vec2(arg0: usize) -> usize {
    0
}

/// convertYUV - from dxvk/d3d9_convert_common.h
#[no_mangle]
pub unsafe extern "C" fn convertYUV(yuv: usize) -> usize {
    0
}

/// vec4 - from dxvk/d3d9_convert_common.h
#[no_mangle]
pub unsafe extern "C" fn vec4(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// convertBT_709 - from dxvk/d3d9_convert_common.h
#[no_mangle]
pub unsafe extern "C" fn convertBT_709(cde: usize) -> usize {
    0
}

/// getOption - from dxvk/dxvk_hud_item.h
#[no_mangle]
pub unsafe extern "C" fn getOption(option: *mut i8, fallback: usize) -> usize {
    0
}

/// parseOption - from dxvk/dxvk_hud_item.h
#[no_mangle]
pub unsafe extern "C" fn parseOption(str: usize, value: usize) {

}

/// processFrameTimes - from dxvk/dxvk_hud_item.h
#[no_mangle]
pub unsafe extern "C" fn processFrameTimes(arg0: usize, key: usize, renderer: usize, dataPoint: u32, minPos: usize, maxPos: usize) {

}

/// drawFrameTimeGraph - from dxvk/dxvk_hud_item.h
#[no_mangle]
pub unsafe extern "C" fn drawFrameTimeGraph(arg0: usize, key: usize, renderer: usize, dataPoint: u32, graphPos: usize, graphSize: usize) {

}

/// computeBufferLayout - from dxvk/dxvk_hud_item.h
#[no_mangle]
pub unsafe extern "C" fn computeBufferLayout() -> usize {
    0
}

/// drawChunk - from dxvk/dxvk_hud_item.h
#[no_mangle]
pub unsafe extern "C" fn drawChunk(pos: usize, size: usize, color: u32, chunk: usize) {

}

/// flushDraws - from dxvk/dxvk_hud_item.h
#[no_mangle]
pub unsafe extern "C" fn flushDraws(arg0: usize, key: usize, options: usize, renderer: usize) {

}

/// updateDataBuffer - from dxvk/dxvk_hud_item.h
#[no_mangle]
pub unsafe extern "C" fn updateDataBuffer(arg0: usize, drawDescriptor: usize, dataDescriptor: usize) {

}

/// computePercentage - from dxvk/dxvk_hud_item.h
#[no_mangle]
pub unsafe extern "C" fn computePercentage() -> u32 {
    0
}

/// accumulateStats - from dxvk/dxvk_hud_item.h
#[no_mangle]
pub unsafe extern "C" fn accumulateStats(stats: usize) {

}

/// beginFrame - from dxvk/dxvk_hud_renderer.h
#[no_mangle]
pub unsafe extern "C" fn beginFrame(arg0: usize, dstView: usize, options: usize) {

}

/// drawText - from dxvk/dxvk_hud_renderer.h
#[no_mangle]
pub unsafe extern "C" fn drawText(size: u32, pos: usize, color: u32, text: usize) {

}

/// drawTextIndirect - from dxvk/dxvk_hud_renderer.h
#[no_mangle]
pub unsafe extern "C" fn drawTextIndirect(arg0: usize, key: usize, drawArgs: usize, drawInfos: usize, textView: usize, drawCount: u32) {

}

/// getPipelineKey - from dxvk/dxvk_hud_renderer.h
#[no_mangle]
pub unsafe extern "C" fn getPipelineKey(dstView: usize) -> usize {
    0
}

/// getSpecInfo - from dxvk/dxvk_hud_renderer.h
#[no_mangle]
pub unsafe extern "C" fn getSpecInfo(constants: *mut core::ffi::c_void) -> usize {
    0
}

/// uploadFontResources - from dxvk/dxvk_hud_renderer.h
#[no_mangle]
pub unsafe extern "C" fn uploadFontResources(arg0: usize) {

}

/// GetPrivateRefCount - from dxvk/com_object.h
#[no_mangle]
pub unsafe extern "C" fn GetPrivateRefCount() -> u32 {
    0
}

/// HasLiveReferences - from dxvk/com_object.h
#[no_mangle]
pub unsafe extern "C" fn HasLiveReferences() -> usize {
    0
}

/// InitReturnPtr - from dxvk/com_object.h
#[no_mangle]
pub unsafe extern "C" fn InitReturnPtr(ptr: *mut *mut core::ffi::c_void) {

}

/// guid - from dxvk/com_private_data.h
#[no_mangle]
pub unsafe extern "C" fn guid() -> usize {
    0
}

/// hasGuid - from dxvk/com_private_data.h
#[no_mangle]
pub unsafe extern "C" fn hasGuid(guid: usize) -> usize {
    0
}

/// setData - from dxvk/com_private_data.h
#[no_mangle]
pub unsafe extern "C" fn setData(guid: usize, size: u32, data: *mut core::ffi::c_void) -> i32 {
    0
}

/// setInterface - from dxvk/com_private_data.h
#[no_mangle]
pub unsafe extern "C" fn setInterface(guid: usize, iface: *mut core::ffi::c_void) -> i32 {
    0
}

/// findEntry - from dxvk/com_private_data.h
#[no_mangle]
pub unsafe extern "C" fn findEntry(guid: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// insertEntry - from dxvk/com_private_data.h
#[no_mangle]
pub unsafe extern "C" fn insertEntry(entry: usize) {

}

/// setOption - from dxvk/config.h
#[no_mangle]
pub unsafe extern "C" fn setOption(key: usize, value: usize) {

}

/// getAppConfig - from dxvk/config.h
#[no_mangle]
pub unsafe extern "C" fn getAppConfig(appName: usize) -> usize {
    0
}

/// getUserConfig - from dxvk/config.h
#[no_mangle]
pub unsafe extern "C" fn getUserConfig() -> usize {
    0
}

/// toLower - from dxvk/config.h
#[no_mangle]
pub unsafe extern "C" fn toLower(str: usize) -> usize {
    0
}

/// getOptionValue - from dxvk/config.h
#[no_mangle]
pub unsafe extern "C" fn getOptionValue(option: *mut i8) -> usize {
    0
}

/// parseOptionValue - from dxvk/config.h
#[no_mangle]
pub unsafe extern "C" fn parseOptionValue(value: usize, result: usize) -> usize {
    0
}

/// parseStringOption - from dxvk/config.h
#[no_mangle]
pub unsafe extern "C" fn parseStringOption(str: usize, begin: usize, end: usize, value: usize) -> usize {
    0
}

/// applyTristate - from dxvk/config.h
#[no_mangle]
pub unsafe extern "C" fn applyTristate(option: usize, state: usize) {

}

/// trace - from dxvk/log.h
#[no_mangle]
pub unsafe extern "C" fn trace(message: usize) {

}

/// err - from dxvk/log.h
#[no_mangle]
pub unsafe extern "C" fn err(message: usize) {

}

/// emitMsg - from dxvk/log.h
#[no_mangle]
pub unsafe extern "C" fn emitMsg(level: usize, message: usize) {

}

/// getFileName - from dxvk/log.h
#[no_mangle]
pub unsafe extern "C" fn getFileName(base: usize) -> usize {
    0
}

/// methodName - from dxvk/log_debug.h
#[no_mangle]
pub unsafe extern "C" fn methodName(prettyName: usize) -> usize {
    0
}

/// traceArgs - from dxvk/log_debug.h
#[no_mangle]
pub unsafe extern "C" fn traceArgs(stream: usize, arg1: usize) {

}

/// unsafeInsert - from dxvk/util_rc_ptr.h
#[no_mangle]
pub unsafe extern "C" fn unsafeInsert(object: *mut core::ffi::c_void) {

}

/// unsafeExtract - from dxvk/util_rc_ptr.h
#[no_mangle]
pub unsafe extern "C" fn unsafeExtract() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SHA1Init - from dxvk/sha1.h
#[no_mangle]
pub unsafe extern "C" fn SHA1Init(arg0: *mut core::ffi::c_void) {

}

/// SHA1Pad - from dxvk/sha1.h
#[no_mangle]
pub unsafe extern "C" fn SHA1Pad(arg0: *mut core::ffi::c_void) {

}

/// SHA1Transform - from dxvk/sha1.h
#[no_mangle]
pub unsafe extern "C" fn SHA1Transform(arg0: u32, arg1: *mut u8) {

}

/// SHA1Update - from dxvk/sha1.h
#[no_mangle]
pub unsafe extern "C" fn SHA1Update(arg0: *mut core::ffi::c_void, arg1: *mut u8, arg2: usize) {

}

/// SHA1Final - from dxvk/sha1.h
#[no_mangle]
pub unsafe extern "C" fn SHA1Final(SHA1_DIGEST_LENGTH: u8, arg1: *mut core::ffi::c_void) {

}

/// compute - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn compute(arg0: usize) -> usize {
    0
}

/// digestLength - from dxvk/sha1_util.h
#[no_mangle]
pub unsafe extern "C" fn digestLength() -> usize {
    0
}

/// Fence - from dxvk/sync_signal.h
#[no_mangle]
pub unsafe extern "C" fn Fence(value: u64) -> usize {
    0
}

/// CallbackFence - from dxvk/sync_signal.h
#[no_mangle]
pub unsafe extern "C" fn CallbackFence(value: u64) -> usize {
    0
}

/// setCallback - from dxvk/sync_signal.h
#[no_mangle]
pub unsafe extern "C" fn setCallback(value: u64, proc: usize) {

}

/// spin - from dxvk/sync_spinlock.h
#[no_mangle]
pub unsafe extern "C" fn spin(spinCount: u32, arg1: usize) {

}

/// ret - from dxvk/wsi_platform_glfw.h
#[no_mangle]
pub unsafe extern "C" fn ret(displayId: *mut core::ffi::c_void) -> usize {
    0
}

/// isDisplayValid - from dxvk/wsi_platform_glfw.h
#[no_mangle]
pub unsafe extern "C" fn isDisplayValid(displayId: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// convertMode - from dxvk/wsi_platform_sdl3.h
#[no_mangle]
pub unsafe extern "C" fn convertMode(mode: usize, pMode: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// demo_vec3_set - from vkd3d-proton/demo.h
#[no_mangle]
pub unsafe extern "C" fn demo_vec3_set(v: *mut core::ffi::c_void, x: f32, y: f32, z: f32) {

}

/// demo_vec4_set - from vkd3d-proton/demo.h
#[no_mangle]
pub unsafe extern "C" fn demo_vec4_set(v: *mut core::ffi::c_void, x: f32, y: f32, z: f32, w: f32) {

}

/// demo_rasterizer_desc_init_default - from vkd3d-proton/demo.h
#[no_mangle]
pub unsafe extern "C" fn demo_rasterizer_desc_init_default(desc: *mut core::ffi::c_void) {

}

/// demo_window_destroy - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn demo_window_destroy(window: *mut core::ffi::c_void) {

}

/// demo_window_destroy_defer - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn demo_window_destroy_defer(window: *mut core::ffi::c_void) {

}

/// demo_key_from_vkey - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn demo_key_from_vkey(vkey: u32) -> usize {
    0
}

/// demo_window_proc - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn demo_window_proc(hwnd: *mut core::ffi::c_void, message: u32, wparam: usize, lparam: isize) -> isize {
    0
}

/// demo_process_events - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn demo_process_events(demo: *mut core::ffi::c_void) {

}

/// demo_init - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn demo_init(demo: *mut core::ffi::c_void, user_data: *mut core::ffi::c_void) -> usize {
    0
}

/// demo_cleanup - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn demo_cleanup(demo: *mut core::ffi::c_void) {

}

/// demo_set_idle_func - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn demo_set_idle_func(demo: *mut core::ffi::c_void, idle_func: ()) {

}

/// demo_swapchain_get_current_back_buffer_index - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn demo_swapchain_get_current_back_buffer_index(swapchain: *mut core::ffi::c_void) -> u32 {
    0
}

/// demo_swapchain_present - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn demo_swapchain_present(swapchain: *mut core::ffi::c_void) {

}

/// demo_swapchain_destroy - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn demo_swapchain_destroy(swapchain: *mut core::ffi::c_void) {

}

/// demo_wait_event - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn demo_wait_event(event: *mut core::ffi::c_void) -> u32 {
    0
}

/// demo_destroy_event - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn demo_destroy_event(event: *mut core::ffi::c_void) {

}

/// demo_get_atom - from vkd3d-proton/demo_xcb.h
#[no_mangle]
pub unsafe extern "C" fn demo_get_atom(c: *mut core::ffi::c_void, name: *mut i8) -> usize {
    0
}

/// demo_add_window - from vkd3d-proton/demo_xcb.h
#[no_mangle]
pub unsafe extern "C" fn demo_add_window(demo: *mut core::ffi::c_void, window: *mut core::ffi::c_void) -> usize {
    0
}

/// demo_remove_window - from vkd3d-proton/demo_xcb.h
#[no_mangle]
pub unsafe extern "C" fn demo_remove_window(demo: *mut core::ffi::c_void, window: *mut core::ffi::c_void) {

}

/// acquire_eventfd - from vkd3d-proton/demo_xcb.h
#[no_mangle]
pub unsafe extern "C" fn acquire_eventfd(fd: i32) {

}

/// IDXGIVkSwapChain_GetImageIndex - from vkd3d-proton/demo_xcb.h
#[no_mangle]
pub unsafe extern "C" fn IDXGIVkSwapChain_GetImageIndex(arg0: usize) -> usize {
    0
}

/// vkd3d_get_vk_device - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_vk_device(device: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_get_vk_physical_device - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_vk_physical_device(device: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_get_vk_queue_family_index - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_vk_queue_family_index(queue: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_get_vk_queue_index - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_vk_queue_index(queue: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_get_vk_queue_flags - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_vk_queue_flags(queue: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_acquire_vk_queue - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_acquire_vk_queue(queue: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_release_vk_queue - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_release_vk_queue(queue: *mut core::ffi::c_void) {

}

/// vkd3d_enqueue_initial_transition - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_enqueue_initial_transition(queue: *mut core::ffi::c_void, resource: *mut core::ffi::c_void) {

}

/// vkd3d_resource_decref - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_resource_decref(resource: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_resource_incref - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_resource_incref(resource: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_serialize_root_signature - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_serialize_root_signature(desc: *mut core::ffi::c_void, version: usize, blob: *mut *mut core::ffi::c_void, error_blob: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_get_vk_format - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_vk_format(format: usize) -> usize {
    0
}

/// vkd3d_get_dxgi_format - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_dxgi_format(format: usize) -> usize {
    0
}

/// vkd3d_serialize_versioned_root_signature - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_serialize_versioned_root_signature(desc: *mut core::ffi::c_void, blob: *mut *mut core::ffi::c_void, error_blob: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_extract_feature_meta - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_extract_feature_meta(code: *mut core::ffi::c_void) {

}

/// vkd3d_shader_hash - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_hash(shader: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_shader_compile_dxbc - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_compile_dxbc(dxbc: *mut core::ffi::c_void, spirv: *mut core::ffi::c_void, spirv_debug: *mut core::ffi::c_void, compiler_options: u32, shader_interface_info: *mut core::ffi::c_void, compile_args: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_contains_root_signature - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_contains_root_signature(code: *mut core::ffi::c_void, size: usize) -> usize {
    0
}

/// vkd3d_shader_parse_root_signature - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_parse_root_signature(dxbc: *mut core::ffi::c_void, root_signature: *mut core::ffi::c_void, compatibility_hash: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_parse_root_signature_raw - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_parse_root_signature_raw(data: *mut i8, data_size: u32, desc: *mut core::ffi::c_void, compatibility_hash: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_serialize_root_signature - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_serialize_root_signature(root_signature: *mut core::ffi::c_void, dxbc: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_convert_root_signature - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_convert_root_signature(dst: *mut core::ffi::c_void, version: usize, src: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_parse_input_signature - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_parse_input_signature(dxbc: *mut core::ffi::c_void, signature: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_parse_output_signature - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_parse_output_signature(dxbc: *mut core::ffi::c_void, signature: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_dxil_append_library_entry_points_and_subobjects - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_dxil_append_library_entry_points_and_subobjects(library_desc: *mut core::ffi::c_void, identifier: u32, entry_points: *mut *mut core::ffi::c_void, entry_point_size: *mut usize, entry_point_count: *mut usize, subobjects: *mut *mut core::ffi::c_void, subobjects_size: *mut usize, subobjects_count: *mut usize) -> i32 {
    0
}

/// vkd3d_shader_dxil_find_global_root_signature_subobject - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_dxil_find_global_root_signature_subobject(dxbc: *mut core::ffi::c_void, size: usize, code: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_compile_arguments_select_quirks - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_compile_arguments_select_quirks(args: *mut core::ffi::c_void, hash: usize, entry: *mut i8) -> usize {
    0
}

/// vkd3d_shader_get_revision - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_get_revision() -> u64 {
    0
}

/// vkd3d_shader_parse_root_signature_v_1_0 - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_parse_root_signature_v_1_0(dxbc: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, compatibility_hash: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_parse_root_signature_v_1_2 - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_parse_root_signature_v_1_2(dxbc: *mut core::ffi::c_void, out_desc: *mut core::ffi::c_void, compatibility_hash: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_parse_root_signature_v_1_2_from_raw_payload - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_parse_root_signature_v_1_2_from_raw_payload(dxbc: *mut core::ffi::c_void, out_desc: *mut core::ffi::c_void, compatibility_hash: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_root_signature_v_1_2_compute_layout_compat_hash - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_root_signature_v_1_2_compute_layout_compat_hash(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_shader_hash_range_parse_line - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_hash_range_parse_line(line: *mut i8, lo: *mut core::ffi::c_void, hi: *mut core::ffi::c_void, trail: *mut *mut i8) -> usize {
    0
}

/// wait_event - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn wait_event(event: *mut core::ffi::c_void, milliseconds: u32) -> u32 {
    0
}

/// signal_event - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn signal_event(event: *mut core::ffi::c_void) {

}

/// destroy_event - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn destroy_event(event: *mut core::ffi::c_void) {

}

/// vkd3d_sleep - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_sleep(ms: u32) {

}

/// enable_feature_level_override - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn enable_feature_level_override(argc: i32, argv: *mut *mut i8) {

}

/// wait_for_fence - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn wait_for_fence(fence: *mut core::ffi::c_void, value: u64) -> i32 {
    0
}

/// wait_for_fence_no_event - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn wait_for_fence_no_event(fence: *mut core::ffi::c_void, value: u64) -> i32 {
    0
}

/// wait_queue_idle_ - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn wait_queue_idle_(line: u32, device: *mut core::ffi::c_void, queue: *mut core::ffi::c_void) {

}

/// wait_queue_idle_no_event_ - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn wait_queue_idle_no_event_(line: u32, device: *mut core::ffi::c_void, queue: *mut core::ffi::c_void) {

}

/// init_vulkan_loader - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn init_vulkan_loader() -> usize {
    0
}

/// init_adapter_info - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn init_adapter_info() {

}

/// get_adapter_desc - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn get_adapter_desc(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// is_amd_windows_device - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn is_amd_windows_device(device: *mut core::ffi::c_void) -> usize {
    0
}

/// is_intel_windows_device - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn is_intel_windows_device(device: *mut core::ffi::c_void) -> usize {
    0
}

/// is_nvidia_windows_device - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn is_nvidia_windows_device(device: *mut core::ffi::c_void) -> usize {
    0
}

/// is_mesa_device - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn is_mesa_device(device: *mut core::ffi::c_void) -> usize {
    0
}

/// is_mesa_intel_device - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn is_mesa_intel_device(device: *mut core::ffi::c_void) -> usize {
    0
}

/// is_nvidia_device - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn is_nvidia_device(device: *mut core::ffi::c_void) -> usize {
    0
}

/// is_radv_device - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn is_radv_device(device: *mut core::ffi::c_void) -> usize {
    0
}

/// is_amd_vulkan_device - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn is_amd_vulkan_device(device: *mut core::ffi::c_void) -> usize {
    0
}

/// is_adreno_device - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn is_adreno_device(device: *mut core::ffi::c_void) -> usize {
    0
}

/// is_integrated_vulkan_device - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn is_integrated_vulkan_device(device: *mut core::ffi::c_void) -> usize {
    0
}

/// is_vk_device_extension_supported - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn is_vk_device_extension_supported(device: *mut core::ffi::c_void, ext: *mut i8) -> usize {
    0
}

/// is_vkd3d_proton_device - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn is_vkd3d_proton_device(device: *mut core::ffi::c_void) -> usize {
    0
}

/// get_driver_vk_features - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn get_driver_vk_features(device: *mut core::ffi::c_void, pnext: *mut core::ffi::c_void) -> usize {
    0
}

/// get_vulkan_device_properties2 - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn get_vulkan_device_properties2(device: *mut core::ffi::c_void, props2: *mut core::ffi::c_void) -> usize {
    0
}

/// get_driver_properties - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn get_driver_properties(device: *mut core::ffi::c_void, driver_properties: *mut core::ffi::c_void) -> usize {
    0
}

/// parse_args - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn parse_args(argc: i32, argv: *mut *mut i8) {

}

/// device_supports_gpu_upload_heap - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn device_supports_gpu_upload_heap(device: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_set_running_in_test_suite - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_set_running_in_test_suite() {

}

/// vkd3d_mute_validation_message - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_mute_validation_message(vuid: *mut i8, explanation: *mut i8) {

}

/// vkd3d_unmute_validation_message - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_unmute_validation_message(vuid: *mut i8) {

}

/// vkd3d_set_out_of_spec_test_behavior - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_set_out_of_spec_test_behavior(behavior: usize, enable: i32) {

}

/// vkd3d_set_behavior_flags - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_set_behavior_flags(flags: usize) {

}

/// set_rect - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn set_rect(rect: *mut core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) {

}

/// set_box - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn set_box(arg0: *mut core::ffi::c_void, left: u32, top: u32, front: u32, right: u32, bottom: u32, back: u32) {

}

/// set_viewport - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn set_viewport(vp: *mut core::ffi::c_void, x: f32, y: f32, width: f32, height: f32, min_depth: f32, max_depth: f32) {

}

/// delta_uint8 - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn delta_uint8(a: u8, b: u8) -> u8 {
    0
}

/// delta_uint16 - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn delta_uint16(a: u16, b: u16) -> u16 {
    0
}

/// delta_uint32 - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn delta_uint32(a: u32, b: u32) -> u32 {
    0
}

/// delta_uint64 - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn delta_uint64(a: u64, b: u64) -> u64 {
    0
}

/// compare_color - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn compare_color(c1: u32, c2: u32, max_diff: u8) -> usize {
    0
}

/// shader_bytecode - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn shader_bytecode(code: *mut core::ffi::c_void, size: usize) -> usize {
    0
}

/// exec_command_list - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn exec_command_list(queue: *mut core::ffi::c_void, list: *mut core::ffi::c_void) {

}

/// reset_command_list_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn reset_command_list_(line: u32, list: *mut core::ffi::c_void, allocator: *mut core::ffi::c_void) {

}

/// queue_signal_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn queue_signal_(line: u32, queue: *mut core::ffi::c_void, fence: *mut core::ffi::c_void, value: u64) {

}

/// queue_wait_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn queue_wait_(line: u32, queue: *mut core::ffi::c_void, fence: *mut core::ffi::c_void, value: u64) {

}

/// update_buffer_data_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn update_buffer_data_(line: u32, buffer: *mut core::ffi::c_void, offset: usize, size: usize, data: *mut core::ffi::c_void) {

}

/// transition_sub_resource_state - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn transition_sub_resource_state(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, sub_resource_idx: u32, state_before: usize, state_after: usize) {

}

/// format_size - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn format_size(arg0: usize) -> usize {
    0
}

/// format_size_planar - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn format_size_planar(arg0: usize, arg1: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// check_sub_resource_uint_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_sub_resource_uint_(line: u32, texture: *mut core::ffi::c_void, sub_resource_idx: u32, queue: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void, expected: u32, max_diff: u32) {

}

/// init_pipeline_state_desc_shaders - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn init_pipeline_state_desc_shaders(desc: *mut core::ffi::c_void, root_signature: *mut core::ffi::c_void, rt_format: usize, input_layout: *mut core::ffi::c_void, vs_code: *mut core::ffi::c_void, vs_size: usize, ps_code: *mut core::ffi::c_void, ps_size: usize) {

}

/// float2 - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn float2(arg0: usize, arg1: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// begin_renderdoc_capturing - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn begin_renderdoc_capturing(device: *mut core::ffi::c_void) {

}

/// end_renderdoc_capturing - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn end_renderdoc_capturing(device: *mut core::ffi::c_void) {

}

/// init_test_context_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn init_test_context_(line: u32, context: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// destroy_test_context_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn destroy_test_context_(line: u32, context: *mut core::ffi::c_void) {

}

/// get_cpu_handle - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_cpu_handle(device: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, heap_type: usize, offset: u32) -> usize {
    0
}

/// get_gpu_handle - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_gpu_handle(device: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, heap_type: usize, offset: u32) -> usize {
    0
}

/// get_cpu_descriptor_handle - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_cpu_descriptor_handle(context: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, offset: u32) -> usize {
    0
}

/// get_cpu_sampler_handle - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_cpu_sampler_handle(context: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, offset: u32) -> usize {
    0
}

/// get_cpu_rtv_handle - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_cpu_rtv_handle(context: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, offset: u32) -> usize {
    0
}

/// get_cpu_dsv_handle - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_cpu_dsv_handle(context: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, offset: u32) -> usize {
    0
}

/// get_gpu_descriptor_handle - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_gpu_descriptor_handle(context: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, offset: u32) -> usize {
    0
}

/// get_gpu_sampler_handle - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_gpu_sampler_handle(context: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, offset: u32) -> usize {
    0
}

/// compare_float - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn compare_float(f: f32, g: f32, ulps: i32) -> usize {
    0
}

/// compare_vec4 - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn compare_vec4(v1: *mut core::ffi::c_void, v2: *mut core::ffi::c_void, ulps: u32) -> usize {
    0
}

/// compare_uvec4 - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn compare_uvec4(v1: *mut core::ffi::c_void, v2: *mut core::ffi::c_void) -> usize {
    0
}

/// compare_uint8 - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn compare_uint8(a: u8, b: u8, max_diff: u32) -> usize {
    0
}

/// compare_uint16 - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn compare_uint16(a: u16, b: u16, max_diff: u32) -> usize {
    0
}

/// compare_uint64 - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn compare_uint64(a: u64, b: u64, max_diff: u32) -> usize {
    0
}

/// get_refcount - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_refcount(iface: *mut core::ffi::c_void) -> u32 {
    0
}

/// check_interface_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_interface_(line: u32, iface: *mut core::ffi::c_void, riid: usize, supported: usize) {

}

/// check_heap_properties_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_heap_properties_(line: u32, properties: *mut core::ffi::c_void, expected_properties: *mut core::ffi::c_void) {

}

/// check_heap_desc_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_heap_desc_(line: u32, desc: *mut core::ffi::c_void, expected_desc: *mut core::ffi::c_void) {

}

/// check_alignment_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_alignment_(line: u32, size: u64, alignment: u64) {

}

/// uav_barrier - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn uav_barrier(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void) {

}

/// copy_sub_resource_data - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn copy_sub_resource_data(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, row_count: u32, slice_count: u32, row_size: usize) {

}

/// upload_buffer_data_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn upload_buffer_data_(line: u32, buffer: *mut core::ffi::c_void, offset: usize, size: usize, data: *mut core::ffi::c_void, queue: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void) {

}

/// upload_texture_data_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn upload_texture_data_(line: u32, texture: *mut core::ffi::c_void, data: *mut core::ffi::c_void, sub_resource_count: u32, queue: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void) {

}

/// upload_texture_data_base_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn upload_texture_data_base_(line: u32, texture: *mut core::ffi::c_void, data: *mut core::ffi::c_void, first_subresource: u32, sub_resource_count: u32, queue: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void) {

}

/// check_sub_resource_float_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_sub_resource_float_(line: u32, texture: *mut core::ffi::c_void, sub_resource_idx: u32, queue: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void, expected: f32, max_diff: u32) {

}

/// check_sub_resource_uint8_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_sub_resource_uint8_(line: u32, texture: *mut core::ffi::c_void, sub_resource_idx: u32, queue: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void, expected: u8, max_diff: u32) {

}

/// check_sub_resource_uint16_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_sub_resource_uint16_(line: u32, texture: *mut core::ffi::c_void, sub_resource_idx: u32, queue: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void, expected: u16, max_diff: u32) {

}

/// check_sub_resource_uint64_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_sub_resource_uint64_(line: u32, texture: *mut core::ffi::c_void, sub_resource_idx: u32, queue: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void, expected: u64, max_diff: u32) {

}

/// check_sub_resource_vec4_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_sub_resource_vec4_(line: u32, texture: *mut core::ffi::c_void, sub_resource_idx: u32, queue: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void, expected: *mut core::ffi::c_void, max_diff: u32) {

}

/// check_sub_resource_uvec4_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_sub_resource_uvec4_(line: u32, texture: *mut core::ffi::c_void, sub_resource_idx: u32, queue: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void, expected_value: *mut core::ffi::c_void) {

}

/// broken_on_warp - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn broken_on_warp(condition: usize) -> usize {
    0
}

/// is_min_max_filtering_supported - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn is_min_max_filtering_supported(device: *mut core::ffi::c_void) -> usize {
    0
}

/// get_tiled_resources_tier - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_tiled_resources_tier(device: *mut core::ffi::c_void) -> usize {
    0
}

/// is_memory_pool_L1_supported - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn is_memory_pool_L1_supported(device: *mut core::ffi::c_void) -> usize {
    0
}

/// is_vrs_tier1_supported - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn is_vrs_tier1_supported(device: *mut core::ffi::c_void, additional_shading_rates: *mut core::ffi::c_void) -> usize {
    0
}

/// is_vrs_tier2_supported - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn is_vrs_tier2_supported(device: *mut core::ffi::c_void) -> usize {
    0
}

/// context_supports_dxil_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn context_supports_dxil_(line: u32, context: *mut core::ffi::c_void) -> usize {
    0
}

/// init_compute_test_context_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn init_compute_test_context_(line: u32, context: *mut core::ffi::c_void) -> usize {
    0
}

/// init_depth_stencil_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn init_depth_stencil_(line: u32, ds: *mut core::ffi::c_void, device: *mut core::ffi::c_void, width: u32, height: u32, array_size: u32, level_count: u32, format: usize, view_format: usize, clear_value: *mut core::ffi::c_void) {

}

/// destroy_depth_stencil_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn destroy_depth_stencil_(line: u32, ds: *mut core::ffi::c_void) {

}

/// half_to_float - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn half_to_float(u16_value: u16) -> f32 {
    0.0
}

/// float_to_half - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn float_to_half(v: f32) -> u16 {
    0
}

/// insert_debug_label - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn insert_debug_label(list: *mut core::ffi::c_void, str: *mut i8) {

}

/// begin_debug_region - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn begin_debug_region(list: *mut core::ffi::c_void, str: *mut i8) {

}

/// end_debug_region - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn end_debug_region(list: *mut core::ffi::c_void) {

}

/// void_ptr_offset - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn void_ptr_offset(arg0: usize, entry_idx: *mut core::ffi::c_void) -> usize {
    0
}

/// hash_map_get_entry_idx - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_map_get_entry_idx(hash_map: *mut core::ffi::c_void, hash_value: u32) -> u32 {
    0
}

/// hash_map_next_entry_idx - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_map_next_entry_idx(hash_map: *mut core::ffi::c_void, entry_idx: u32) -> u32 {
    0
}

/// hash_map_next_size - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_map_next_size(old_size: u32) -> u32 {
    0
}

/// hash_map_grow - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_map_grow(hash_map: *mut core::ffi::c_void) -> usize {
    0
}

/// hash_map_should_grow_before_insert - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_map_should_grow_before_insert(hash_map: *mut core::ffi::c_void) -> usize {
    0
}

/// hash_map_iter - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_map_iter(hash_map: *mut core::ffi::c_void, iterator: usize, userdata: *mut core::ffi::c_void) {

}

/// hash_map_init - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_map_init(hash_map: *mut core::ffi::c_void, hash_func: usize, compare_func: usize, entry_size: usize) {

}

/// hash_map_clear - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_map_clear(hash_map: *mut core::ffi::c_void) {

}

/// hash_combine - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_combine(old_hash: u32, new_hash: u32) -> u32 {
    0
}

/// hash_uint64 - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_uint64(n: u64) -> u32 {
    0
}

/// hash_data - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_data(data: *mut core::ffi::c_void, size: usize) -> u32 {
    0
}

/// hash_fnv1_init - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_fnv1_init() -> u64 {
    0
}

/// hash_fnv1_iterate_u8 - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_fnv1_iterate_u8(h: u64, value: u8) -> u64 {
    0
}

/// hash_fnv1_iterate_u32 - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_fnv1_iterate_u32(h: u64, value: u32) -> u64 {
    0
}

/// hash_fnv1_iterate_f32 - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_fnv1_iterate_f32(h: u64, value: f32) -> u64 {
    0
}

/// hash_fnv1_iterate_u64 - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_fnv1_iterate_u64(h: u64, value: u64) -> u64 {
    0
}

/// hash_fnv1_iterate_string - from vkd3d-proton/hashmap.h
#[no_mangle]
pub unsafe extern "C" fn hash_fnv1_iterate_string(h: u64, str: *mut i8) -> u64 {
    0
}

/// list_add_after - from vkd3d-proton/list.h
#[no_mangle]
pub unsafe extern "C" fn list_add_after(elem: *mut core::ffi::c_void, to_add: *mut core::ffi::c_void) {

}

/// list_add_before - from vkd3d-proton/list.h
#[no_mangle]
pub unsafe extern "C" fn list_add_before(elem: *mut core::ffi::c_void, to_add: *mut core::ffi::c_void) {

}

/// list_next - from vkd3d-proton/list.h
#[no_mangle]
pub unsafe extern "C" fn list_next(arg0: usize, arg1: usize) -> usize {
    0
}

/// list_prev - from vkd3d-proton/list.h
#[no_mangle]
pub unsafe extern "C" fn list_prev(arg0: usize, arg1: usize) -> usize {
    0
}

/// list_count - from vkd3d-proton/list.h
#[no_mangle]
pub unsafe extern "C" fn list_count(list: *mut core::ffi::c_void) -> u32 {
    0
}

/// list_move_tail - from vkd3d-proton/list.h
#[no_mangle]
pub unsafe extern "C" fn list_move_tail(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void) {

}

/// list_move_head - from vkd3d-proton/list.h
#[no_mangle]
pub unsafe extern "C" fn list_move_head(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void) {

}

/// rb_is_red - from vkd3d-proton/rbtree.h
#[no_mangle]
pub unsafe extern "C" fn rb_is_red(entry: *mut core::ffi::c_void) -> i32 {
    0
}

/// rb_rotate_left - from vkd3d-proton/rbtree.h
#[no_mangle]
pub unsafe extern "C" fn rb_rotate_left(tree: *mut core::ffi::c_void, e: *mut core::ffi::c_void) {

}

/// rb_rotate_right - from vkd3d-proton/rbtree.h
#[no_mangle]
pub unsafe extern "C" fn rb_rotate_right(tree: *mut core::ffi::c_void, e: *mut core::ffi::c_void) {

}

/// rb_flip_color - from vkd3d-proton/rbtree.h
#[no_mangle]
pub unsafe extern "C" fn rb_flip_color(entry: *mut core::ffi::c_void) {

}

/// rb_head - from vkd3d-proton/rbtree.h
#[no_mangle]
pub unsafe extern "C" fn rb_head(arg0: usize) -> usize {
    0
}

/// rb_postorder_head - from vkd3d-proton/rbtree.h
#[no_mangle]
pub unsafe extern "C" fn rb_postorder_head(arg0: usize) -> usize {
    0
}

/// rb_postorder - from vkd3d-proton/rbtree.h
#[no_mangle]
pub unsafe extern "C" fn rb_postorder(tree: *mut core::ffi::c_void, callback: *mut core::ffi::c_void, context: *mut core::ffi::c_void) {

}

/// rb_init - from vkd3d-proton/rbtree.h
#[no_mangle]
pub unsafe extern "C" fn rb_init(tree: *mut core::ffi::c_void, compare: usize) {

}

/// rb_for_each_entry - from vkd3d-proton/rbtree.h
#[no_mangle]
pub unsafe extern "C" fn rb_for_each_entry(tree: *mut core::ffi::c_void, callback: *mut core::ffi::c_void, context: *mut core::ffi::c_void) {

}

/// rb_clear - from vkd3d-proton/rbtree.h
#[no_mangle]
pub unsafe extern "C" fn rb_clear(tree: *mut core::ffi::c_void, callback: *mut core::ffi::c_void, context: *mut core::ffi::c_void) {

}

/// rb_destroy - from vkd3d-proton/rbtree.h
#[no_mangle]
pub unsafe extern "C" fn rb_destroy(tree: *mut core::ffi::c_void, callback: *mut core::ffi::c_void, context: *mut core::ffi::c_void) {

}

/// rb_put - from vkd3d-proton/rbtree.h
#[no_mangle]
pub unsafe extern "C" fn rb_put(tree: *mut core::ffi::c_void, key: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) -> i32 {
    0
}

/// rb_remove - from vkd3d-proton/rbtree.h
#[no_mangle]
pub unsafe extern "C" fn rb_remove(tree: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) {

}

/// rb_remove_key - from vkd3d-proton/rbtree.h
#[no_mangle]
pub unsafe extern "C" fn rb_remove_key(tree: *mut core::ffi::c_void, key: *mut core::ffi::c_void) {

}

/// vkd3d_atomic_uint32_increment - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint32_increment(target: *mut u32, order: usize) -> usize {
    0
}

/// vkd3d_atomic_uint32_decrement - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint32_decrement(target: *mut u32, order: usize) -> usize {
    0
}

/// vkd3d_atomic_uint32_add - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint32_add(target: *mut u32, value: u32, order: usize) -> usize {
    0
}

/// vkd3d_atomic_uint32_sub - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint32_sub(target: *mut u32, value: u32, order: usize) -> usize {
    0
}

/// vkd3d_atomic_uint32_and - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint32_and(target: *mut u32, value: u32, order: usize) -> usize {
    0
}

/// vkd3d_atomic_uint32_or - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint32_or(target: *mut u32, value: u32, order: usize) -> usize {
    0
}

/// vkd3d_atomic_uint32_compare_exchange - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint32_compare_exchange(target: *mut u32, expected: u32, desired: u32, success_order: usize, fail_order: usize) -> usize {
    0
}

/// vkd3d_atomic_uint64_increment - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint64_increment(target: *mut u64, order: usize) -> usize {
    0
}

/// vkd3d_atomic_uint64_decrement - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint64_decrement(target: *mut u64, order: usize) -> usize {
    0
}

/// vkd3d_atomic_uint64_compare_exchange - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint64_compare_exchange(target: *mut core::ffi::c_void, expected: u64, desired: u64, success_order: usize, fail_order: usize) -> usize {
    0
}

/// align64 - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn align64(addr: u64, alignment: u64) -> u64 {
    0
}

/// vkd3d_popcount - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_popcount(v: u32) -> u32 {
    0
}

/// __popcnt - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn __popcnt(arg0: usize) -> usize {
    0
}

/// vkd3d_bitmask_is_contiguous - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bitmask_is_contiguous(mask: u32) -> usize {
    0
}

/// vkd3d_bitmask_tzcnt64 - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bitmask_tzcnt64(mask: u64) -> u32 {
    0
}

/// vkd3d_bitmask_tzcnt32 - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bitmask_tzcnt32(mask: u32) -> u32 {
    0
}

/// vkd3d_bitmask_iter64 - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bitmask_iter64(mask: *mut u64) -> u32 {
    0
}

/// vkd3d_bitmask_iter32 - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bitmask_iter32(mask: *mut u32) -> u32 {
    0
}

/// vkd3d_bitmask_iter32_range - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bitmask_iter32_range(mask: *mut u32) -> usize {
    0
}

/// __builtin_clz - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn __builtin_clz(arg0: usize) -> usize {
    0
}

/// ascii_isupper - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn ascii_isupper(c: i32) -> i32 {
    0
}

/// ascii_tolower - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn ascii_tolower(c: i32) -> i32 {
    0
}

/// ascii_strcasecmp - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn ascii_strcasecmp(a: *mut i8, b: *mut i8) -> i32 {
    0
}

/// vkd3d_parse_version - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_parse_version(version: *mut i8, major: *mut i32, minor: *mut i32, patch: *mut i32) {

}

/// float_bits_to_uint32 - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn float_bits_to_uint32(f: f32) -> u32 {
    0
}

/// vkd3d_wcslen - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_wcslen(wstr: *mut u16) -> usize {
    0
}

/// vkd3d_float_to_fixed_24_8 - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_float_to_fixed_24_8(f: f32) -> i32 {
    0
}

/// lroundf - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn lroundf(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_fixed_24_8_to_float - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_fixed_24_8_to_float(i: i32) -> f32 {
    0.0
}

/// vkd3d_get_current_time_ns - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_current_time_ns() -> u64 {
    0
}

/// vkd3d_get_current_time_ticks - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_current_time_ticks() -> u64 {
    0
}

/// __rdtsc - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn __rdtsc() -> usize {
    0
}

/// __builtin_ia32_rdtsc - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn __builtin_ia32_rdtsc() -> usize {
    0
}

/// vkd3d_dbg_disable_debug_file - from vkd3d-proton/vkd3d_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_dbg_disable_debug_file() {

}

/// vkd3d_dbg_get_level - from vkd3d-proton/vkd3d_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_dbg_get_level(channel: usize) -> usize {
    0
}

/// vkd3d_dbg_flush - from vkd3d-proton/vkd3d_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_dbg_flush() {

}

/// VKD3D_DBG_DISABLE_DEBUG_FILE - from vkd3d-proton/vkd3d_debug.h
#[no_mangle]
pub unsafe extern "C" fn VKD3D_DBG_DISABLE_DEBUG_FILE() -> usize {
    0
}

/// vkd3d_env_var_as_uint - from vkd3d-proton/vkd3d_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_env_var_as_uint(name: *mut i8, default_value: u32) -> u32 {
    0
}

/// vkd3d_debug_list_has_member - from vkd3d-proton/vkd3d_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_list_has_member(string: *mut i8, member: *mut i8) -> usize {
    0
}

/// vkd3d_parse_debug_options - from vkd3d-proton/vkd3d_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_parse_debug_options(string: *mut i8, options: *mut core::ffi::c_void, option_count: u32) -> u64 {
    0
}

/// vkd3d_file_unmap - from vkd3d-proton/vkd3d_file_utils.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_file_unmap(file: *mut core::ffi::c_void) {

}

/// vkd3d_file_rename_no_replace - from vkd3d-proton/vkd3d_file_utils.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_file_rename_no_replace(from_path: *mut i8, to_path: *mut i8) -> usize {
    0
}

/// vkd3d_file_delete - from vkd3d-proton/vkd3d_file_utils.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_file_delete(path: *mut i8) -> usize {
    0
}

/// vkd3d_array_reserve - from vkd3d-proton/vkd3d_memory.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_array_reserve(elements: *mut *mut core::ffi::c_void, capacity: *mut usize, element_count: usize, element_size: usize) -> usize {
    0
}

/// vkd3d_native_sync_handle_eq - from vkd3d-proton/vkd3d_native_sync_handle.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_native_sync_handle_eq(a: usize, b: usize) -> usize {
    0
}

/// vkd3d_native_sync_handle_wrap - from vkd3d-proton/vkd3d_native_sync_handle.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_native_sync_handle_wrap(os_handle: *mut core::ffi::c_void, arg1: usize) -> usize {
    0
}

/// vkd3d_native_sync_handle_release - from vkd3d-proton/vkd3d_native_sync_handle.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_native_sync_handle_release(handle: usize, count: u32) -> i32 {
    0
}

/// semaphore - from vkd3d-proton/vkd3d_native_sync_handle.h
#[no_mangle]
pub unsafe extern "C" fn semaphore(VKD3D_NATIVE_SYNC_HANDLE_TYPE_EVENT: usize) -> usize {
    0
}

/// vkd3d_native_sync_handle_signal - from vkd3d-proton/vkd3d_native_sync_handle.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_native_sync_handle_signal(handle: usize) -> i32 {
    0
}

/// vkd3d_native_sync_handle_is_valid - from vkd3d-proton/vkd3d_native_sync_handle.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_native_sync_handle_is_valid(handle: usize) -> usize {
    0
}

/// vkd3d_native_sync_handle_destroy - from vkd3d-proton/vkd3d_native_sync_handle.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_native_sync_handle_destroy(handle: usize) {

}

/// vkd3d_dlopen - from vkd3d-proton/vkd3d_platform.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_dlopen(name: *mut i8) -> usize {
    0
}

/// vkd3d_dlclose - from vkd3d-proton/vkd3d_platform.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_dlclose(handle: usize) -> i32 {
    0
}

/// vkd3d_get_env_var - from vkd3d-proton/vkd3d_platform.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_env_var(name: *mut i8, value: *mut i8, value_size: usize) -> usize {
    0
}

/// vkd3d_get_program_name - from vkd3d-proton/vkd3d_platform.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_program_name(program_nameVKD3D_PATH_MAX: i8) -> usize {
    0
}

/// vkd3d_get_linux_kernel_version - from vkd3d-proton/vkd3d_platform.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_linux_kernel_version(major: *mut u32, minor: *mut u32, patch: *mut u32) -> usize {
    0
}

/// vkd3d_init_profiling - from vkd3d-proton/vkd3d_profiling.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_profiling() {

}

/// vkd3d_uses_profiling - from vkd3d-proton/vkd3d_profiling.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_uses_profiling() -> usize {
    0
}

/// vkd3d_profiling_register_region - from vkd3d-proton/vkd3d_profiling.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_profiling_register_region(name: *mut i8, lock: *mut core::ffi::c_void, latch: *mut u32) -> u32 {
    0
}

/// vkd3d_profiling_notify_work - from vkd3d-proton/vkd3d_profiling.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_profiling_notify_work(index: u32, start_ticks: u64, end_ticks: u64, iteration_count: u32) {

}

/// vkd3d_pause - from vkd3d-proton/vkd3d_spinlock.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pause() {

}

/// vkd3d_string_ends_with_n - from vkd3d-proton/vkd3d_string.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_string_ends_with_n(str: *mut i8, str_len: usize, ending: *mut i8, ending_len: usize) -> usize {
    0
}

/// vkd3d_string_ends_with - from vkd3d-proton/vkd3d_string.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_string_ends_with(str: *mut i8, ending: *mut i8) -> usize {
    0
}

/// vkd3d_string_compare - from vkd3d-proton/vkd3d_string.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_string_compare(mode: usize, string: *mut i8, comparator: *mut i8) -> usize {
    0
}

/// strstr - from vkd3d-proton/vkd3d_string.h
#[no_mangle]
pub unsafe extern "C" fn strstr(arg0: usize, arg1: usize) -> usize {
    0
}

/// vkd3d_strlcpy - from vkd3d-proton/vkd3d_string.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_strlcpy(dst: *mut i8, dst_size: usize, src: *mut i8) {

}

/// vkd3d_strlcat - from vkd3d-proton/vkd3d_string.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_strlcat(dst: *mut i8, dst_size: usize, src: *mut i8) {

}

/// vkd3d_test_main - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_main(argc: i32, argv: *mut *mut i8) {

}

/// vkd3d_test_start_todo - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_start_todo(is_todo: usize) {

}

/// vkd3d_test_loop_todo - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_loop_todo() -> i32 {
    0
}

/// vkd3d_test_end_todo - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_end_todo() {

}

/// vkd3d_test_assert_that - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_assert_that(arg0: usize, arg1: usize) -> usize {
    0
}

/// vkd3d_test_ok - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_ok(arg0: usize, arg1: usize) -> usize {
    0
}

/// vkd3d_test_todo - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_todo(arg0: usize, arg1: usize) -> usize {
    0
}

/// vkd3d_test_skip - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_skip(arg0: usize, arg1: usize) -> usize {
    0
}

/// vkd3d_test_trace - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_trace(arg0: usize, arg1: usize) -> usize {
    0
}

/// vkd3d_test_platform_is_windows - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_platform_is_windows() -> usize {
    0
}

/// broken - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn broken(condition: usize) -> usize {
    0
}

/// vkd3d_test_check_assert_that - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_check_assert_that(line: u32, result: usize, fmt: *mut i8, args: usize) {

}

/// vkd3d_test_check_ok - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_check_ok(line: u32, result: usize, fmt: *mut i8, args: usize) {

}

/// vkd3d_test_debug - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_debug(fmt: *mut i8) -> usize {
    0
}

/// running_under_wine - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn running_under_wine() -> usize {
    0
}

/// vkd3d_run_test - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_run_test(name: *mut i8, test_pfn: usize) {

}

/// vkd3d_test_start_bug - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_start_bug(is_bug: usize) {

}

/// vkd3d_test_loop_bug - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_loop_bug() -> i32 {
    0
}

/// vkd3d_test_end_bug - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_end_bug() {

}

/// vkd3d_test_set_context - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_test_set_context(fmt: *mut i8) {

}

/// condvar_reltime_init - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn condvar_reltime_init(cond: *mut core::ffi::c_void) -> i32 {
    0
}

/// condvar_reltime_destroy - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn condvar_reltime_destroy(cond: *mut core::ffi::c_void) -> i32 {
    0
}

/// condvar_reltime_signal - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn condvar_reltime_signal(cond: *mut core::ffi::c_void) -> i32 {
    0
}

/// condvar_reltime_wait_timeout_seconds - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn condvar_reltime_wait_timeout_seconds(cond: *mut core::ffi::c_void, lock: *mut core::ffi::c_void, seconds: u32) -> i32 {
    0
}

/// syscall - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn syscall(arg0: usize) -> usize {
    0
}

/// DEBUG_CHANNEL_ELECT - from vkd3d-proton/debug_channel.h
#[no_mangle]
pub unsafe extern "C" fn DEBUG_CHANNEL_ELECT() -> usize {
    0
}

/// subgroupElect - from vkd3d-proton/debug_channel.h
#[no_mangle]
pub unsafe extern "C" fn subgroupElect() -> usize {
    0
}

/// DEBUG_CHANNEL_INIT - from vkd3d-proton/debug_channel.h
#[no_mangle]
pub unsafe extern "C" fn DEBUG_CHANNEL_INIT(id: usize) {

}

/// DEBUG_CHANNEL_MSG_ - from vkd3d-proton/debug_channel.h
#[no_mangle]
pub unsafe extern "C" fn DEBUG_CHANNEL_MSG_() {

}

/// DEBUG_CHANNEL_MSG - from vkd3d-proton/debug_channel.h
#[no_mangle]
pub unsafe extern "C" fn DEBUG_CHANNEL_MSG() {

}

/// DEBUG_CHANNEL_MSG_UNIFORM - from vkd3d-proton/debug_channel.h
#[no_mangle]
pub unsafe extern "C" fn DEBUG_CHANNEL_MSG_UNIFORM(v0: usize) {

}

/// VKD3D_REGION_END - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn VKD3D_REGION_END(arg0: usize) -> usize {
    0
}

/// VKD3D_REGION_DECL - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn VKD3D_REGION_DECL(arg0: usize) -> usize {
    0
}

/// vkd3d_descriptor_debug_encode_buffer_va - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_encode_buffer_va(va: usize, elem_size: u32) -> usize {
    0
}

/// vkd3d_descriptor_debug_sync_validation_barrier - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_sync_validation_barrier(global_info: *mut core::ffi::c_void, device: *mut core::ffi::c_void, vk_cmd_buffer: usize) {

}

/// vkd3d_descriptor_debug_clear_bloom_filter - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_clear_bloom_filter(global_info: *mut core::ffi::c_void, device: *mut core::ffi::c_void, vk_cmd_buffer: usize) -> u32 {
    0
}

/// vkd3d_descriptor_debug_update_va_timestamp - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_update_va_timestamp(global_info: *mut core::ffi::c_void, device: *mut core::ffi::c_void, vk_cmd_buffer: usize) -> u32 {
    0
}

/// vkd3d_descriptor_debug_kick_qa_check - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_kick_qa_check(global_info: *mut core::ffi::c_void) {

}

/// vkd3d_descriptor_debug_get_shader_interface_flags - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_get_shader_interface_flags(global_info: *mut core::ffi::c_void, code: *mut core::ffi::c_void, size: usize) -> u32 {
    0
}

/// vkd3d_descriptor_debug_init - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_init() {

}

/// vkd3d_descriptor_debug_active_instruction_qa_checks - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_active_instruction_qa_checks() -> usize {
    0
}

/// vkd3d_descriptor_debug_active_descriptor_qa_checks - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_active_descriptor_qa_checks() -> usize {
    0
}

/// vkd3d_descriptor_debug_register_heap - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_register_heap(heap: *mut core::ffi::c_void, cookie: usize, desc: *mut core::ffi::c_void) {

}

/// vkd3d_descriptor_debug_unregister_heap - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_unregister_heap(cookie: usize) {

}

/// vkd3d_descriptor_debug_register_resource_cookie - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_register_resource_cookie(global_info: *mut core::ffi::c_void, cookie: usize, desc: *mut core::ffi::c_void) {

}

/// vkd3d_descriptor_debug_register_query_heap_cookie - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_register_query_heap_cookie(global_info: *mut core::ffi::c_void, cookie: usize, desc: *mut core::ffi::c_void) {

}

/// vkd3d_descriptor_debug_register_view_cookie - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_register_view_cookie(global_info: *mut core::ffi::c_void, cookie: usize, resource_cookie: usize) {

}

/// vkd3d_descriptor_debug_unregister_cookie - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_unregister_cookie(global_info: *mut core::ffi::c_void, cookie: usize) {

}

/// vkd3d_descriptor_debug_copy_descriptor - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_copy_descriptor(dst_heap: *mut core::ffi::c_void, dst_heap_cookie: usize, dst_offset: u32, src_heap: *mut core::ffi::c_void, src_heap_cookie: usize, src_offset: u32, cookie: usize) {

}

/// vkd3d_descriptor_debug_heap_info_size - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_heap_info_size(num_descriptors: u32) -> u64 {
    0
}

/// hresult_from_errno - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn hresult_from_errno(rc: i32) -> i32 {
    0
}

/// hresult_from_vk_result - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn hresult_from_vk_result(vr: i32) -> i32 {
    0
}

/// hresult_from_vkd3d_result - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn hresult_from_vkd3d_result(vkd3d_result: i32) -> i32 {
    0
}

/// vkd3d_add_wait_to_all_queues - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_add_wait_to_all_queues(device: *mut core::ffi::c_void, vk_semaphore: usize, value: u64) {

}

/// vkd3d_enqueue_timeline_semaphore - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_enqueue_timeline_semaphore(worker: *mut core::ffi::c_void, fence_info: *mut core::ffi::c_void, timeline_cookie: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_va_map_insert - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_va_map_insert(va_map: *mut core::ffi::c_void, resource: *mut core::ffi::c_void) {

}

/// vkd3d_va_map_remove - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_va_map_remove(va_map: *mut core::ffi::c_void, resource: *mut core::ffi::c_void) {

}

/// vkd3d_va_map_place_acceleration_structure - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_va_map_place_acceleration_structure(va_map: *mut core::ffi::c_void, device: *mut core::ffi::c_void, va: usize) -> usize {
    0
}

/// vkd3d_va_map_place_opacity_micromap - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_va_map_place_opacity_micromap(va_map: *mut core::ffi::c_void, device: *mut core::ffi::c_void, va: usize) -> usize {
    0
}

/// vkd3d_va_map_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_va_map_init(va_map: *mut core::ffi::c_void) {

}

/// vkd3d_va_map_cleanup - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_va_map_cleanup(va_map: *mut core::ffi::c_void) {

}

/// vkd3d_private_data_destroy - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_private_data_destroy(data: *mut core::ffi::c_void) {

}

/// vkd3d_private_store_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_private_store_init(store: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_private_store_destroy - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_private_store_destroy(store: *mut core::ffi::c_void) {

}

/// vkd3d_get_private_data - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_private_data(store: *mut core::ffi::c_void, tag: *mut core::ffi::c_void, out_size: *mut u32, out: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_private_store_set_private_data - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_private_store_set_private_data(store: *mut core::ffi::c_void, tag: *mut core::ffi::c_void, data: *mut core::ffi::c_void, data_size: u32, is_object: usize) -> i32 {
    0
}

/// vkd3d_private_data_object_name_ptr - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_private_data_object_name_ptr(guid: usize, data_size: u32, data: *mut core::ffi::c_void, out_name: *mut *mut i8) -> usize {
    0
}

/// vkd3d_set_private_data - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_set_private_data(store: *mut core::ffi::c_void, tag: *mut core::ffi::c_void, data_size: u32, data: *mut core::ffi::c_void, set_name_callback: usize, calling_object: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_set_private_data_interface - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_set_private_data_interface(store: *mut core::ffi::c_void, tag: *mut core::ffi::c_void, object: *mut core::ffi::c_void, set_name_callback: usize, calling_object: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d_destruction_notifier_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d_destruction_notifier_init(notifier: *mut core::ffi::c_void, parent: *mut core::ffi::c_void) {

}

/// d3d_destruction_notifier_notify - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d_destruction_notifier_notify(notifier: *mut core::ffi::c_void) {

}

/// vkd3d_get_priority_adjust - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_priority_adjust(size: u64) -> u32 {
    0
}

/// vkd3d_convert_to_vk_prio - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_convert_to_vk_prio(d3d12prio: usize) -> f32 {
    0.0
}

/// vkd3d_memory_transfer_queue_cleanup - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_transfer_queue_cleanup(queue: *mut core::ffi::c_void) {

}

/// vkd3d_memory_transfer_queue_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_transfer_queue_init(queue: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_memory_transfer_queue_flush - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_transfer_queue_flush(queue: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_memory_transfer_queue_build_empty_rtas - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_transfer_queue_build_empty_rtas(queue: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_view_map_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_view_map_init(view_map: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_view_map_destroy - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_view_map_destroy(view_map: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_null_cookie - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_null_cookie() -> usize {
    0
}

/// d3d12_resource_get_vk_subresource - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_get_vk_subresource(resource: *mut core::ffi::c_void, subresource_idx: u32, all_aspects: usize) -> usize {
    0
}

/// vk_image_aspect_flags_from_d3d12 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_image_aspect_flags_from_d3d12(format: *mut core::ffi::c_void, plane_idx: u32) -> usize {
    0
}

/// vk_image_subresource_from_d3d12 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_image_subresource_from_d3d12(format: *mut core::ffi::c_void, subresource_idx: u32, miplevel_count: u32, layer_count: u32, all_aspects: usize) -> usize {
    0
}

/// vk_image_subresource_layers_from_d3d12 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_image_subresource_layers_from_d3d12(format: *mut core::ffi::c_void, sub_resource_idx: u32, miplevel_count: u32, layer_count: u32) -> usize {
    0
}

/// vk_image_layout_from_d3d12_resource_state - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_image_layout_from_d3d12_resource_state(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, state: usize) -> usize {
    0
}

/// d3d12_plane_index_from_vk_aspect - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_plane_index_from_vk_aspect(aspect: usize) -> u32 {
    0
}

/// vkd3d_view_decref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_view_decref(view: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_view_incref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_view_incref(view: *mut core::ffi::c_void) {

}

/// vkd3d_descriptor_metadata_view_set_qa_cookie - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_metadata_view_set_qa_cookie(view: *mut core::ffi::c_void, cookie: usize) {

}

/// vk_bind_point_from_pipeline_type - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_bind_point_from_pipeline_type(pipeline_type: usize) -> usize {
    0
}

/// vkd3d_vk_stage_flags_from_visibility - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_vk_stage_flags_from_visibility(visibility: usize) -> usize {
    0
}

/// vkd3d_shader_visibility_from_d3d12 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_visibility_from_d3d12(visibility: usize) -> usize {
    0
}

/// vkd3d_vertex_input_pipeline_desc_hash - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_vertex_input_pipeline_desc_hash(key: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_vertex_input_pipeline_desc_compare - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_vertex_input_pipeline_desc_compare(key: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_fragment_output_pipeline_desc_hash - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_fragment_output_pipeline_desc_hash(key: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_fragment_output_pipeline_desc_compare - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_fragment_output_pipeline_desc_compare(key: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) -> usize {
    0
}

/// dsv_attachment_mask - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn dsv_attachment_mask(graphics: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_pipeline_state_desc_from_d3d12_graphics_desc - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_state_desc_from_d3d12_graphics_desc(desc: *mut core::ffi::c_void, d3d12_desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_pipeline_state_desc_from_d3d12_compute_desc - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_state_desc_from_d3d12_compute_desc(desc: *mut core::ffi::c_void, d3d12_desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_pipeline_state_desc_from_d3d12_stream_desc - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_state_desc_from_d3d12_stream_desc(desc: *mut core::ffi::c_void, d3d12_desc: *mut core::ffi::c_void, vk_bind_point: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_get_cached_spirv_code_from_d3d12_desc - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_cached_spirv_code_from_d3d12_desc(state: *mut core::ffi::c_void, stage: usize, spirv_code: *mut core::ffi::c_void, identifier: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_serialize_pipeline_state - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_serialize_pipeline_state(pipeline_library: *mut core::ffi::c_void, state: *mut core::ffi::c_void, size: *mut usize, data: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_pipeline_cache_compat_from_state_desc - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_cache_compat_from_state_desc(compat: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// vkd3d_pipeline_cache_compatibility_condense - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_cache_compatibility_condense(compat: *mut core::ffi::c_void) -> u64 {
    0
}

/// vkd3d_pipeline_library_store_pipeline_to_disk_cache - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_library_store_pipeline_to_disk_cache(pipeline_library: *mut core::ffi::c_void, state: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_pipeline_library_find_cached_blob_from_disk_cache - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_library_find_cached_blob_from_disk_cache(pipeline_library: *mut core::ffi::c_void, compat: *mut core::ffi::c_void, cached_state: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_pipeline_library_disk_cache_notify_blob_insert - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_library_disk_cache_notify_blob_insert(disk_cache: *mut core::ffi::c_void, hash: u64, arg2: u32, data: *mut core::ffi::c_void, size: usize) {

}

/// vkd3d_pipeline_library_init_disk_cache - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_library_init_disk_cache(cache: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_pipeline_library_flush_disk_cache - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_library_flush_disk_cache(cache: *mut core::ffi::c_void) {

}

/// vk_image_memory_barrier_for_initial_transition - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_image_memory_barrier_for_initial_transition(resource: *mut core::ffi::c_void, barrier: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_queue_acquire - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_acquire(queue: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_set_queue_out_of_band - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_set_queue_out_of_band(device: *mut core::ffi::c_void, queue: *mut core::ffi::c_void, arg2: usize) {

}

/// vkd3d_queue_drain - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_drain(queue: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_queue_destroy - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_destroy(queue: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_queue_release - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_release(queue: *mut core::ffi::c_void) {

}

/// vkd3d_queue_add_wait - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_add_wait(queue: *mut core::ffi::c_void, semaphore: usize, value: u64) {

}

/// vkd3d_queue_wait_submission_timeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_wait_submission_timeline(queue: *mut core::ffi::c_void, timeline: u64, timeout: u64) -> i32 {
    0
}

/// dxgi_vk_swap_chain_low_latency_enabled - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_low_latency_enabled(chain: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_latency_sleep - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_latency_sleep(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_set_latency_sleep_mode - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_set_latency_sleep_mode(chain: *mut core::ffi::c_void, low_latency_mode: usize, low_latency_boost: usize, minimum_interval_us: u32) {

}

/// dxgi_vk_swap_chain_set_latency_marker - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_set_latency_marker(chain: *mut core::ffi::c_void, frameID: u64, marker: usize, from_app: usize) {

}

/// dxgi_vk_swap_chain_get_latency_info - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_get_latency_info(chain: *mut core::ffi::c_void, latency_results: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_incref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_incref(chain: *mut core::ffi::c_void) -> u32 {
    0
}

/// dxgi_vk_swap_chain_decref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_decref(chain: *mut core::ffi::c_void) -> u32 {
    0
}

/// dxgi_vk_swap_chain_factory_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_factory_init(queue: *mut core::ffi::c_void, chain: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_sampler_state_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_sampler_state_init(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_sampler_state_cleanup - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_sampler_state_cleanup(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_global_descriptor_buffer_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_global_descriptor_buffer_init(global_descriptor_buffer: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_global_descriptor_buffer_cleanup - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_global_descriptor_buffer_cleanup(global_descriptor_buffer: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_shader_debug_ring_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_debug_ring_init(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_debug_ring_cleanup - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_debug_ring_cleanup(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_shader_debug_ring_kick - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_debug_ring_kick(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void, device_lost: usize) {

}

/// vkd3d_shader_hash_range_parse - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_hash_range_parse(file: *mut core::ffi::c_void, ranges: *mut *mut core::ffi::c_void, range_size: *mut usize, range_count: *mut usize, kind: usize) {

}

/// vkd3d_debug_buffer_memory_properties - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_buffer_memory_properties(device: *mut core::ffi::c_void, flags: usize, high_throughput: usize) -> usize {
    0
}

/// vkd3d_bindless_state_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bindless_state_init(bindless_state: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_bindless_state_cleanup - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bindless_state_cleanup(bindless_state: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_bindless_state_find_binding - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bindless_state_find_binding(bindless_state: *mut core::ffi::c_void, flags: u32, binding: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_bindless_state_find_set - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bindless_state_find_set(bindless_state: *mut core::ffi::c_void, flags: u32) -> usize {
    0
}

/// vkd3d_bindless_state_find_set_info_index - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bindless_state_find_set_info_index(bindless_state: *mut core::ffi::c_void, flags: u32) -> u32 {
    0
}

/// vkd3d_bindless_state_binding_from_info_index - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bindless_state_binding_from_info_index(bindless_state: *mut core::ffi::c_void, index: u32) -> usize {
    0
}

/// vkd3d_format_compatibility_list_add_format - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_format_compatibility_list_add_format(list: *mut core::ffi::c_void, vk_format: usize) {

}

/// vkd3d_memory_info_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_info_init(info: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_memory_info_cleanup - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_info_cleanup(info: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_meta_ops_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_ops_init(meta_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_ops_cleanup - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_ops_cleanup(meta_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_get_clear_buffer_uav_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_clear_buffer_uav_pipeline(meta_ops: *mut core::ffi::c_void, as_uint: usize, raw: usize) -> usize {
    0
}

/// vkd3d_meta_get_clear_image_uav_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_clear_image_uav_pipeline(meta_ops: *mut core::ffi::c_void, image_view_type: usize, as_uint: usize) -> usize {
    0
}

/// vkd3d_meta_get_clear_image_uav_workgroup_size - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_clear_image_uav_workgroup_size(view_type: usize) -> usize {
    0
}

/// vkd3d_meta_get_clear_buffer_uav_workgroup_size - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_clear_buffer_uav_workgroup_size() -> usize {
    0
}

/// vkd3d_meta_get_copy_image_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_copy_image_pipeline(meta_ops: *mut core::ffi::c_void, key: *mut core::ffi::c_void, info: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_get_copy_image_view_type - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_copy_image_view_type(dim: usize) -> usize {
    0
}

/// vkd3d_meta_get_resolve_image_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_resolve_image_pipeline(meta_ops: *mut core::ffi::c_void, key: *mut core::ffi::c_void, info: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_get_swapchain_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_swapchain_pipeline(meta_ops: *mut core::ffi::c_void, key: *mut core::ffi::c_void, info: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_get_query_gather_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_query_gather_pipeline(meta_ops: *mut core::ffi::c_void, heap_type: usize, info: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_meta_get_predicate_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_predicate_pipeline(meta_ops: *mut core::ffi::c_void, command_type: usize, info: *mut core::ffi::c_void) {

}

/// vkd3d_meta_get_multi_dispatch_indirect_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_multi_dispatch_indirect_pipeline(meta_ops: *mut core::ffi::c_void, info: *mut core::ffi::c_void) {

}

/// vkd3d_meta_get_multi_dispatch_indirect_workgroup_size - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_multi_dispatch_indirect_workgroup_size() -> u32 {
    0
}

/// vkd3d_meta_get_execute_indirect_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_execute_indirect_pipeline(meta_ops: *mut core::ffi::c_void, patch_command_count: u32, info: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_get_sampler_feedback_resolve_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_sampler_feedback_resolve_pipeline(meta_ops: *mut core::ffi::c_void, arg1: usize, info: *mut core::ffi::c_void) {

}

/// vkd3d_meta_get_sampler_feedback_workgroup_size - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_sampler_feedback_workgroup_size() -> usize {
    0
}

/// vkd3d_meta_get_workgraph_workgroup_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_workgraph_workgroup_pipeline(meta_ops: *mut core::ffi::c_void, info: *mut core::ffi::c_void, broadcast_compacting: usize) {

}

/// vkd3d_meta_get_workgraph_setup_gpu_input_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_workgraph_setup_gpu_input_pipeline(meta_ops: *mut core::ffi::c_void, info: *mut core::ffi::c_void) {

}

/// vkd3d_meta_get_workgraph_payload_offset_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_workgraph_payload_offset_pipeline(meta_ops: *mut core::ffi::c_void, info: *mut core::ffi::c_void) {

}

/// vkd3d_meta_get_workgraph_complete_compaction_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_workgraph_complete_compaction_pipeline(meta_ops: *mut core::ffi::c_void, info: *mut core::ffi::c_void) {

}

/// vkd3d_meta_get_workgraph_setup_gpu_input_workgroup_size - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_workgraph_setup_gpu_input_workgroup_size() -> u32 {
    0
}

/// vkd3d_meta_get_workgraph_complete_compaction_workgroup_size - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_get_workgraph_complete_compaction_workgroup_size() -> u32 {
    0
}

/// vkd3d_queue_timeline_trace_cookie_is_valid - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_cookie_is_valid(cookie: usize) -> usize {
    0
}

/// vkd3d_queue_timeline_trace_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_init(trace: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_queue_timeline_trace_cleanup - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_cleanup(trace: *mut core::ffi::c_void) {

}

/// vkd3d_queue_timeline_trace_register_event_signal - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_event_signal(trace: *mut core::ffi::c_void, handle: usize, fence: *mut core::ffi::c_void, value: u64) -> usize {
    0
}

/// vkd3d_queue_timeline_trace_register_signal - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_signal(trace: *mut core::ffi::c_void, fence: *mut core::ffi::c_void, value: u64) -> usize {
    0
}

/// vkd3d_queue_timeline_trace_cpu_signal - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_cpu_signal(trace: *mut core::ffi::c_void, fence: *mut core::ffi::c_void, value: u64) {

}

/// vkd3d_queue_timeline_trace_register_wait - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_wait(trace: *mut core::ffi::c_void, fence: *mut core::ffi::c_void, value: u64) -> usize {
    0
}

/// vkd3d_queue_timeline_trace_register_swapchain_blit - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_swapchain_blit(trace: *mut core::ffi::c_void, present_id: u64) -> usize {
    0
}

/// vkd3d_queue_timeline_trace_register_present_wait - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_present_wait(trace: *mut core::ffi::c_void, present_id: u64) -> usize {
    0
}

/// vkd3d_queue_timeline_trace_register_low_latency_sleep - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_low_latency_sleep(trace: *mut core::ffi::c_void, present_id: u64) -> usize {
    0
}

/// vkd3d_queue_timeline_trace_register_pso_compile - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_pso_compile(trace: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_queue_timeline_trace_register_sparse - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_sparse(trace: *mut core::ffi::c_void, num_tiles: u32) -> usize {
    0
}

/// vkd3d_queue_timeline_trace_register_execute - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_execute(trace: *mut core::ffi::c_void, command_lists: *mut *mut core::ffi::c_void, count: u32) -> usize {
    0
}

/// vkd3d_queue_timeline_trace_register_command_list - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_command_list(trace: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_queue_timeline_trace_register_generic_region - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_generic_region(trace: *mut core::ffi::c_void, tag: *mut i8) -> usize {
    0
}

/// vkd3d_queue_timeline_trace_complete_event_signal - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_complete_event_signal(trace: *mut core::ffi::c_void, worker: *mut core::ffi::c_void, cookie: usize) {

}

/// vkd3d_queue_timeline_trace_complete_execute - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_complete_execute(trace: *mut core::ffi::c_void, worker: *mut core::ffi::c_void, cookie: usize) {

}

/// vkd3d_queue_timeline_trace_complete_present_wait - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_complete_present_wait(trace: *mut core::ffi::c_void, cookie: usize) {

}

/// vkd3d_queue_timeline_trace_complete_low_latency_sleep - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_complete_low_latency_sleep(trace: *mut core::ffi::c_void, cookie: usize) {

}

/// vkd3d_queue_timeline_trace_close_command_list - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_close_command_list(trace: *mut core::ffi::c_void, cookie: usize) {

}

/// vkd3d_queue_timeline_trace_begin_execute - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_begin_execute(trace: *mut core::ffi::c_void, cookie: usize) {

}

/// vkd3d_queue_timeline_trace_begin_execute_overhead - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_begin_execute_overhead(trace: *mut core::ffi::c_void, cookie: usize) {

}

/// vkd3d_queue_timeline_trace_end_execute_overhead - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_end_execute_overhead(trace: *mut core::ffi::c_void, cookie: usize) {

}

/// vkd3d_queue_timeline_trace_complete_pso_compile - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_complete_pso_compile(trace: *mut core::ffi::c_void, cookie: usize, pso_hash: u64, completion_kind: *mut i8) {

}

/// vkd3d_address_binding_tracker_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_address_binding_tracker_init(tracker: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_address_binding_tracker_cleanup - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_address_binding_tracker_cleanup(tracker: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_address_binding_tracker_assign_info - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_address_binding_tracker_assign_info(tracker: *mut core::ffi::c_void, arg1: usize, handle: u64, info: *mut core::ffi::c_void) {

}

/// vkd3d_address_binding_tracker_assign_cookie - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_address_binding_tracker_assign_cookie(tracker: *mut core::ffi::c_void, arg1: usize, handle: u64, cookie: u64) {

}

/// vkd3d_address_binding_tracker_check_va - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_address_binding_tracker_check_va(tracker: *mut core::ffi::c_void, address: usize) {

}

/// vkd3d_address_binding_tracker_active - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_address_binding_tracker_active(tracker: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_nv_shader_extn_entry_hash - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_nv_shader_extn_entry_hash(key: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_nv_shader_extn_entry_compare - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_nv_shader_extn_entry_compare(key: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_nv_shader_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_nv_shader_init(device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_nv_shader_cleanup - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_nv_shader_cleanup(device: *mut core::ffi::c_void) {

}

/// d3d12_device_unmap_vkd3d_queue - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_unmap_vkd3d_queue(queue: *mut core::ffi::c_void, command_queue: *mut core::ffi::c_void) {

}

/// is_cpu_accessible_heap - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn is_cpu_accessible_heap(properties: *mut core::ffi::c_void) -> usize {
    0
}

/// is_cpu_accessible_system_memory_heap - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn is_cpu_accessible_system_memory_heap(properties: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_bindless_get_mutable_descriptor_type_size - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bindless_get_mutable_descriptor_type_size(device: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_bindless_supports_embedded_mutable_type - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bindless_supports_embedded_mutable_type(device: *mut core::ffi::c_void, flags: u32) -> usize {
    0
}

/// vkd3d_enumerate_meta_commands - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_enumerate_meta_commands(device: *mut core::ffi::c_void, count: *mut u32, output_descs: *mut core::ffi::c_void) {

}

/// vkd3d_enumerate_meta_command_parameters - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_enumerate_meta_command_parameters(device: *mut core::ffi::c_void, command_id: usize, stage: usize, total_size: *mut u32, param_count: *mut u32, param_descs: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_format_is_compressed - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_format_is_compressed(format: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_format_copy_data - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_format_copy_data(format: *mut core::ffi::c_void, src: *mut u8, src_row_pitch: u32, src_slice_pitch: u32, dst: *mut u8, dst_row_pitch: u32, dst_slice_pitch: u32, w: u32, h: u32, d: u32) {

}

/// vkd3d_internal_get_vk_format - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_internal_get_vk_format(device: *mut core::ffi::c_void, dxgi_format: usize) -> usize {
    0
}

/// vkd3d_format_footprint_for_plane - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_format_footprint_for_plane(format: *mut core::ffi::c_void, plane_idx: u32) -> usize {
    0
}

/// vkd3d_init_format_info - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_format_info(device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_cleanup_format_info - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_cleanup_format_info(device: *mut core::ffi::c_void) {

}

/// vkd3d_get_format - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_format(arg0: usize, descFormat: usize, D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL: usize) -> usize {
    0
}

/// vk_subresource_range_from_subresource - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_subresource_range_from_subresource(subresource: *mut core::ffi::c_void) -> usize {
    0
}

/// vk_subresource_range_from_layers - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_subresource_range_from_layers(layers: *mut core::ffi::c_void) -> usize {
    0
}

/// vk_subresource_layers_from_subresource - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_subresource_layers_from_subresource(subresource: *mut core::ffi::c_void) -> usize {
    0
}

/// vk_subresource_layers_from_view - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_subresource_layers_from_view(view: *mut core::ffi::c_void) -> usize {
    0
}

/// vk_subresource_range_from_view - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_subresource_range_from_view(view: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_desc_get_vk_subresource_extent - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_get_vk_subresource_extent(desc: *mut core::ffi::c_void, format: *mut core::ffi::c_void, subresource: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_get_depth_bias_representation - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_depth_bias_representation(info: *mut core::ffi::c_void, device: *mut core::ffi::c_void, dsv_format: usize) {

}

/// vkd3d_get_buffer_device_address - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_buffer_device_address(device: *mut core::ffi::c_void, vk_buffer: usize) -> usize {
    0
}

/// vkd3d_get_acceleration_structure_device_address - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_acceleration_structure_device_address(device: *mut core::ffi::c_void, vk_acceleration_structure: usize) -> usize {
    0
}

/// vkd3d_get_null_rtas_va - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_null_rtas_va(device: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_build_null_rtas_va - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_build_null_rtas_va(device: *mut core::ffi::c_void, vk_cmd_buffer: usize) {

}

/// vkd3d_compute_workgroup_count - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compute_workgroup_count(thread_count: u32, workgroup_size: u32) -> u32 {
    0
}

/// vk_compare_op_from_d3d12 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_compare_op_from_d3d12(op: usize) -> usize {
    0
}

/// vk_samples_from_dxgi_sample_desc - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_samples_from_dxgi_sample_desc(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// vk_samples_from_sample_count - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_samples_from_sample_count(sample_count: u32) -> usize {
    0
}

/// is_valid_feature_level - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn is_valid_feature_level(feature_level: usize) -> usize {
    0
}

/// is_valid_resource_state - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn is_valid_resource_state(state: usize) -> usize {
    0
}

/// is_valid_format - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn is_valid_format(format: usize) -> usize {
    0
}

/// return_interface - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn return_interface(iface: *mut core::ffi::c_void, iface_iid: usize, requested_iid: usize, object: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// debug_ignored_node_mask - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn debug_ignored_node_mask(mask: u32) {

}

/// vkd3d_load_vk_global_procs - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_load_vk_global_procs(procs: *mut core::ffi::c_void, vkGetInstanceProcAddr: usize) -> i32 {
    0
}

/// vkd3d_load_vk_device_procs - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_load_vk_device_procs(procs: *mut core::ffi::c_void, parent_procs: *mut core::ffi::c_void, device: usize) -> i32 {
    0
}

/// vkd3d_set_vk_object_name - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_set_vk_object_name(device: *mut core::ffi::c_void, vk_object: u64, vk_object_type: usize, name: *mut i8) -> i32 {
    0
}

/// vk_prepend_struct - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_prepend_struct(header: *mut core::ffi::c_void, structure: *mut core::ffi::c_void) {

}

/// vk_remove_struct - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_remove_struct(header: *mut core::ffi::c_void, arg1: usize) {

}

/// vkd3d_acceleration_structure_get_geometry_count - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_acceleration_structure_get_geometry_count(desc: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_acceleration_structure_convert_inputs - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_acceleration_structure_convert_inputs(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, build_info: *mut core::ffi::c_void, geometry_infos: *mut core::ffi::c_void, omm_infos: *mut core::ffi::c_void, range_infos: *mut core::ffi::c_void, primitive_counts: *mut u32) -> usize {
    0
}

/// vkd3d_acceleration_structure_emit_postbuild_info - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_acceleration_structure_emit_postbuild_info(list: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, count: u32, addresses: *mut core::ffi::c_void) {

}

/// vkd3d_acceleration_structure_emit_immediate_postbuild_info - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_acceleration_structure_emit_immediate_postbuild_info(list: *mut core::ffi::c_void, count: u32, desc: *mut core::ffi::c_void, vk_acceleration_structure: usize) {

}

/// vkd3d_acceleration_structure_copy - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_acceleration_structure_copy(list: *mut core::ffi::c_void, dst: usize, src_as: usize, mode: usize) {

}

/// vkd3d_opacity_micromap_convert_inputs - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_opacity_micromap_convert_inputs(device: *mut core::ffi::c_void, inputs: *mut core::ffi::c_void, build_info: *mut core::ffi::c_void, usages: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_opacity_micromap_emit_postbuild_info - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_opacity_micromap_emit_postbuild_info(list: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, count: u32, addresses: *mut core::ffi::c_void) {

}

/// vkd3d_opacity_micromap_emit_immediate_postbuild_info - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_opacity_micromap_emit_immediate_postbuild_info(list: *mut core::ffi::c_void, count: u32, desc: *mut core::ffi::c_void, vk_opacity_micromap: usize) {

}

/// vkd3d_opacity_micromap_copy - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_opacity_micromap_copy(list: *mut core::ffi::c_void, dst: usize, src_omm: usize, mode: usize) {

}

/// vkd3d_set_shared_metadata - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_set_shared_metadata(handle: *mut core::ffi::c_void, buf: *mut core::ffi::c_void, buf_size: u32) -> usize {
    0
}

/// vkd3d_get_shared_metadata - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_shared_metadata(handle: *mut core::ffi::c_void, buf: *mut core::ffi::c_void, buf_size: u32, metadata_size: *mut u32) -> usize {
    0
}

/// vkd3d_open_kmt_handle - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_open_kmt_handle(kmt_handle: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// vkd3d_mapped_memory_range_align - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_mapped_memory_range_align(device: *mut core::ffi::c_void, range: *mut core::ffi::c_void, size: u64) {

}

/// vkd3d_renderdoc_active - from vkd3d-proton/vkd3d_renderdoc.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_renderdoc_active() -> usize {
    0
}

/// vkd3d_renderdoc_loaded_api - from vkd3d-proton/vkd3d_renderdoc.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_renderdoc_loaded_api() -> usize {
    0
}

/// vkd3d_renderdoc_should_capture_shader_hash - from vkd3d-proton/vkd3d_renderdoc.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_renderdoc_should_capture_shader_hash(hash: usize) -> usize {
    0
}

/// vkd3d_renderdoc_global_capture_enabled - from vkd3d-proton/vkd3d_renderdoc.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_renderdoc_global_capture_enabled() -> usize {
    0
}

/// vkd3d_renderdoc_begin_capture - from vkd3d-proton/vkd3d_renderdoc.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_renderdoc_begin_capture(instance: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_renderdoc_end_capture - from vkd3d-proton/vkd3d_renderdoc.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_renderdoc_end_capture(instance: *mut core::ffi::c_void) {

}

/// vkd3d_renderdoc_init - from vkd3d-proton/vkd3d_renderdoc.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_renderdoc_init() {

}

/// vkd3d_renderdoc_command_list_check_capture - from vkd3d-proton/vkd3d_renderdoc.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_renderdoc_command_list_check_capture(list: *mut core::ffi::c_void, state: *mut core::ffi::c_void) {

}

/// vkd3d_renderdoc_command_queue_begin_capture - from vkd3d-proton/vkd3d_renderdoc.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_renderdoc_command_queue_begin_capture(command_queue: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_renderdoc_command_queue_end_capture - from vkd3d-proton/vkd3d_renderdoc.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_renderdoc_command_queue_end_capture(command_queue: *mut core::ffi::c_void) {

}

/// vkd3d_timestamp_profiler_deinit - from vkd3d-proton/vkd3d_timestamp_profiler.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_deinit(profiler: *mut core::ffi::c_void) {

}

/// vkd3d_timestamp_profiler_set_pipeline_state - from vkd3d-proton/vkd3d_timestamp_profiler.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_set_pipeline_state(profiler: *mut core::ffi::c_void, list: *mut core::ffi::c_void, state: *mut core::ffi::c_void) {

}

/// vkd3d_timestamp_profiler_register_pipeline_state - from vkd3d-proton/vkd3d_timestamp_profiler.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_register_pipeline_state(profiler: *mut core::ffi::c_void, state: *mut core::ffi::c_void) {

}

/// vkd3d_timestamp_profiler_mark_pre_command - from vkd3d-proton/vkd3d_timestamp_profiler.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_mark_pre_command(profiler: *mut core::ffi::c_void, list: *mut core::ffi::c_void) {

}

/// vkd3d_timestamp_profiler_end_render_pass - from vkd3d-proton/vkd3d_timestamp_profiler.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_end_render_pass(profiler: *mut core::ffi::c_void, list: *mut core::ffi::c_void) {

}

/// vkd3d_timestamp_profiler_end_command_buffer - from vkd3d-proton/vkd3d_timestamp_profiler.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_end_command_buffer(profiler: *mut core::ffi::c_void, list: *mut core::ffi::c_void) {

}

/// vkd3d_timestamp_profiler_reset_command_list - from vkd3d-proton/vkd3d_timestamp_profiler.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_reset_command_list(profiler: *mut core::ffi::c_void, list: *mut core::ffi::c_void) {

}

/// vkd3d_timestamp_profiler_submit_command_list - from vkd3d-proton/vkd3d_timestamp_profiler.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_submit_command_list(profiler: *mut core::ffi::c_void, list: *mut core::ffi::c_void) {

}

/// vkd3d_timestamp_profiler_mark_frame_boundary - from vkd3d-proton/vkd3d_timestamp_profiler.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_mark_frame_boundary(profiler: *mut core::ffi::c_void) {

}

/// shader_is_dxil - from vkd3d-proton/vkd3d_shader_private.h
#[no_mangle]
pub unsafe extern "C" fn shader_is_dxil(dxbc: *mut core::ffi::c_void, dxbc_length: usize) -> usize {
    0
}

/// shader_parse_input_signature - from vkd3d-proton/vkd3d_shader_private.h
#[no_mangle]
pub unsafe extern "C" fn shader_parse_input_signature(dxbc: *mut core::ffi::c_void, dxbc_length: usize, signature: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_parse_output_signature - from vkd3d-proton/vkd3d_shader_private.h
#[no_mangle]
pub unsafe extern "C" fn shader_parse_output_signature(dxbc: *mut core::ffi::c_void, dxbc_length: usize, signature: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_compute_dxbc_checksum - from vkd3d-proton/vkd3d_shader_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compute_dxbc_checksum(dxbc: *mut core::ffi::c_void, size: usize, checksum4: u32) {

}

/// vkd3d_shader_dump_spirv_shader - from vkd3d-proton/vkd3d_shader_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_dump_spirv_shader(hash: usize, shader: *mut core::ffi::c_void) {

}

/// vkd3d_shader_dump_shader - from vkd3d-proton/vkd3d_shader_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_dump_shader(hash: usize, shader: *mut core::ffi::c_void, ext: *mut i8) {

}

/// vkd3d_shader_replace - from vkd3d-proton/vkd3d_shader_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_replace(hash: usize, data: *mut *mut core::ffi::c_void, size: *mut usize) -> usize {
    0
}

/// vkd3d_shader_quirk_to_tess_factor_limit - from vkd3d-proton/vkd3d_shader_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_quirk_to_tess_factor_limit(quirks: u32) -> u32 {
    0
}

/// vkd3d_shader_compile_dxil - from vkd3d-proton/vkd3d_shader_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_compile_dxil(dxbc: *mut core::ffi::c_void, spirv: *mut core::ffi::c_void, spirv_debug: *mut core::ffi::c_void, shader_interface_info: *mut core::ffi::c_void, compiler_args: *mut core::ffi::c_void, is_dxil: usize) -> i32 {
    0
}

/// fetch_mip - from vkd3d-proton/sampler_feedback_decode.h
#[no_mangle]
pub unsafe extern "C" fn fetch_mip(unnormalized_feedback_coord: usize, mip: i32, layer: f32) -> usize {
    0
}

/// sampler_feedback_decode_min_mip - from vkd3d-proton/sampler_feedback_decode.h
#[no_mangle]
pub unsafe extern "C" fn sampler_feedback_decode_min_mip(icoord: usize, layer: f32) -> i32 {
    0
}

/// sampler_feedback_decode_mip_used - from vkd3d-proton/sampler_feedback_decode.h
#[no_mangle]
pub unsafe extern "C" fn sampler_feedback_decode_mip_used(icoord: usize, mip: i32, layer: f32) -> usize {
    0
}

/// sampler_feedback_encode_min_mip - from vkd3d-proton/sampler_feedback_encode.h
#[no_mangle]
pub unsafe extern "C" fn sampler_feedback_encode_min_mip(coord: usize, layer: i32, value: usize) {

}

/// clear_bit - from vkd3d-proton/sampler_feedback_encode.h
#[no_mangle]
pub unsafe extern "C" fn clear_bit(v: usize, bit: i32) {

}

/// set_bit - from vkd3d-proton/sampler_feedback_encode.h
#[no_mangle]
pub unsafe extern "C" fn set_bit(v: usize, bit: i32) {

}

/// sampler_feedback_encode_mip_used - from vkd3d-proton/sampler_feedback_encode.h
#[no_mangle]
pub unsafe extern "C" fn sampler_feedback_encode_mip_used(coord: usize, layer: i32, accessed: usize) {

}

/// MD5_Init - from vkd3d-proton/md5.h
#[no_mangle]
pub unsafe extern "C" fn MD5_Init(ctx: *mut core::ffi::c_void) {

}

/// MD5_Update - from vkd3d-proton/md5.h
#[no_mangle]
pub unsafe extern "C" fn MD5_Update(ctx: *mut core::ffi::c_void, data: *mut core::ffi::c_void, size: u64) {

}

/// MD5_Final - from vkd3d-proton/md5.h
#[no_mangle]
pub unsafe extern "C" fn MD5_Final(result: *mut u8, ctx: *mut core::ffi::c_void) {

}

/// AGSMagic - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn AGSMagic(code: usize, arg0: usize, arg1: usize) -> usize {
    0
}

/// Float32ToFloat8 - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn Float32ToFloat8(v: f32) -> usize {
    0
}

/// Float32ToBFloat8 - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn Float32ToBFloat8(v: f32) -> usize {
    0
}

/// Float8ToFloat32 - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn Float8ToFloat32(v: usize) -> f32 {
    0.0
}

/// asfloat - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn asfloat(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// BFloat8ToFloat32 - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn BFloat8ToFloat32(v: usize) -> f32 {
    0.0
}

/// WMMA_MatMulAcc - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn WMMA_MatMulAcc(op: usize, A: usize, B: usize, C: usize) -> usize {
    0
}

/// WMMA_MakeType - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn WMMA_MakeType(fmt: usize, mtype: usize, shape: usize, transposed: usize) -> usize {
    0
}

/// WMMA_Load - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn WMMA_Load(arg0: usize, BAB: usize, offset: usize, stride: usize) -> usize {
    0
}

/// WMMA_Store - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn WMMA_Store(arg0: usize, BAB: usize, offset: usize, stride: usize, m: usize) {

}

/// WMMA_LoadLDS - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn WMMA_LoadLDS(arg0: usize, offset: usize, stride: usize) -> usize {
    0
}

/// WMMA_StoreLDS - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn WMMA_StoreLDS(arg0: usize, offset: usize, stride: usize, m: usize) {

}

/// WMMA_Convert - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn WMMA_Convert(intype: usize, outtype: usize, m: usize) -> usize {
    0
}

/// WMMA_ConvertSaturate - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn WMMA_ConvertSaturate(intype: usize, outtype: usize, m: usize) -> usize {
    0
}

/// WMMA_MatrixLength - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn WMMA_MatrixLength(arg0: usize) -> usize {
    0
}

/// WMMA_MatrixElementFill - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn WMMA_MatrixElementFill(arg0: usize, m: usize, index: usize, data: usize) -> usize {
    0
}

/// WMMA_MatrixFill - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn WMMA_MatrixFill(arg0: usize, value: usize) -> usize {
    0
}

/// WMMA_ElementWiseOp - from vkd3d-proton/wmma_ags.h
#[no_mangle]
pub unsafe extern "C" fn WMMA_ElementWiseOp(arg0: usize, A: usize, B: usize, op: usize) -> usize {
    0
}

/// cxg_populate_command_list - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_populate_command_list(cxg: *mut core::ffi::c_void, rt_idx: u32) {

}

/// cxg_wait_for_previous_frame - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_wait_for_previous_frame(cxg: *mut core::ffi::c_void) {

}

/// cxg_update_mvp - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_update_mvp(cxg: *mut core::ffi::c_void) {

}

/// cxg_get_time - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_get_time() -> f64 {
    0.0
}

/// cxg_render_frame - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_render_frame(cxg: *mut core::ffi::c_void) {

}

/// cxg_destroy_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_destroy_pipeline(cxg: *mut core::ffi::c_void) {

}

/// cxg_load_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_load_pipeline(cxg: *mut core::ffi::c_void) {

}

/// cxg_fence_destroy - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_fence_destroy(cxg_fence: *mut core::ffi::c_void) {

}

/// cxg_destroy_assets - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_destroy_assets(cxg: *mut core::ffi::c_void) {

}

/// cxg_vertex_set_position - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_vertex_set_position(v: *mut core::ffi::c_void, x: f32, y: f32, z: f32) {

}

/// cxg_vertex_set_normal - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_vertex_set_normal(v: *mut core::ffi::c_void, x: f32, y: f32, z: f32) {

}

/// cxg_mesh_destroy - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_mesh_destroy(mesh: *mut core::ffi::c_void) {

}

/// cxg_load_gears - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_load_gears(cxg: *mut core::ffi::c_void) {

}

/// cxg_key_press - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_key_press(window: *mut core::ffi::c_void, key: usize, user_data: *mut core::ffi::c_void) {

}

/// cxg_idle - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_idle(demo: *mut core::ffi::c_void, user_data: *mut core::ffi::c_void) {

}

/// cxg_main - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_main() -> i32 {
    0
}

/// cxt_populate_command_list - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxt_populate_command_list(cxt: *mut core::ffi::c_void) {

}

/// cxt_wait_for_previous_frame - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxt_wait_for_previous_frame(cxt: *mut core::ffi::c_void) {

}

/// cxt_render_frame - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxt_render_frame(window: *mut core::ffi::c_void, user_data: *mut core::ffi::c_void) {

}

/// cxt_destroy_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxt_destroy_pipeline(cxt: *mut core::ffi::c_void) {

}

/// cxt_load_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxt_load_pipeline(cxt: *mut core::ffi::c_void) {

}

/// cxt_fence_destroy - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxt_fence_destroy(cxt_fence: *mut core::ffi::c_void) {

}

/// cxt_destroy_assets - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxt_destroy_assets(cxt: *mut core::ffi::c_void) {

}

/// cxt_key_press - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxt_key_press(window: *mut core::ffi::c_void, key: usize, user_data: *mut core::ffi::c_void) {

}

/// cxt_main - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxt_main() -> i32 {
    0
}

/// test_bindless_heap_sm66_uav_counter - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_heap_sm66_uav_counter() {

}

/// test_bindless_heap_sm66 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_heap_sm66() {

}

/// test_bindless_srv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_srv(use_dxil: usize) {

}

/// test_bindless_srv_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_srv_sm51() {

}

/// test_bindless_srv_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_srv_dxil() {

}

/// test_bindless_samplers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_samplers(use_dxil: usize) {

}

/// test_bindless_samplers_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_samplers_sm51() {

}

/// test_bindless_samplers_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_samplers_dxil() {

}

/// test_bindless_full_root_parameters_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_full_root_parameters_sm51() {

}

/// test_bindless_cbv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_cbv(use_dxil: usize) {

}

/// test_bindless_uav - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_uav(use_dxil: usize) {

}

/// test_bindless_cbv_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_cbv_sm51() {

}

/// test_bindless_cbv_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_cbv_dxil() {

}

/// test_bindless_uav_counter - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_uav_counter(use_dxil: usize) {

}

/// test_bindless_uav_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_uav_sm51() {

}

/// test_bindless_uav_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_uav_dxil() {

}

/// test_bindless_uav_counter_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_uav_counter_sm51() {

}

/// test_bindless_uav_counter_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_uav_counter_dxil() {

}

/// test_bindless_bufinfo - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_bufinfo(use_dxil: usize) {

}

/// test_bindless_bufinfo_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_bufinfo_sm51() {

}

/// test_bindless_bufinfo_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bindless_bufinfo_dxil() {

}

/// test_divergent_buffer_index_varying - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_divergent_buffer_index_varying() {

}

/// test_clear_depth_stencil_view - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_clear_depth_stencil_view() {

}

/// test_clear_render_target_view - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_clear_render_target_view() {

}

/// test_clear_view_extreme_values - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_clear_view_extreme_values(uav: usize) {

}

/// test_clear_render_target_view_extreme_values - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_clear_render_target_view_extreme_values() {

}

/// test_clear_uav_extreme_values - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_clear_uav_extreme_values() {

}

/// test_clear_unordered_access_view_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_clear_unordered_access_view_buffer() {

}

/// test_clear_unordered_access_view_image - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_clear_unordered_access_view_image() {

}

/// test_uav_clear_exhaustive_descriptors - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_uav_clear_exhaustive_descriptors() {

}

/// test_clear_uav_mismatch_heap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_clear_uav_mismatch_heap() {

}

/// test_deferred_clears - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_deferred_clears() {

}

/// test_deferred_clears_dsv_layout - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_deferred_clears_dsv_layout() {

}

/// test_set_render_targets - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_set_render_targets() {

}

/// test_draw_no_descriptor_bindings - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_draw_no_descriptor_bindings() {

}

/// test_multiple_render_targets - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_multiple_render_targets() {

}

/// test_fractional_viewports - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fractional_viewports() {

}

/// test_negative_viewports - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_negative_viewports() {

}

/// test_scissor - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_scissor() {

}

/// test_draw_depth_no_ps - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_draw_depth_no_ps() {

}

/// test_draw_depth_only - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_draw_depth_only() {

}

/// test_draw_uav_only - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_draw_uav_only() {

}

/// test_texture_resource_barriers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_texture_resource_barriers() {

}

/// test_null_vbv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_null_vbv() {

}

/// test_vbv_stride_edge_cases - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_vbv_stride_edge_cases() {

}

/// test_execute_indirect_multi_dispatch_root_descriptors - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_execute_indirect_multi_dispatch_root_descriptors() {

}

/// test_execute_indirect_multi_dispatch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_execute_indirect_multi_dispatch() {

}

/// report_predication_timestamps - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn report_predication_timestamps(queue: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, num_timestamps: u32, tag: *mut i8) {

}

/// test_execute_indirect_state_predication - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_execute_indirect_state_predication() {

}

/// test_execute_indirect_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_execute_indirect_state() {

}

/// test_execute_indirect_state_tier_11 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_execute_indirect_state_tier_11() {

}

/// test_execute_indirect_state_vbo_offsets - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_execute_indirect_state_vbo_offsets() {

}

/// test_execute_indirect - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_execute_indirect() {

}

/// test_dispatch_huge_groups - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_dispatch_huge_groups() {

}

/// test_unaligned_vertex_stride - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_unaligned_vertex_stride() {

}

/// test_zero_vertex_stride - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_zero_vertex_stride() {

}

/// test_command_list_initial_pipeline_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_command_list_initial_pipeline_state() {

}

/// test_conditional_rendering - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_conditional_rendering() {

}

/// test_discard_resource_uav_type - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_discard_resource_uav_type(compute_queue: usize) {

}

/// test_discard_resource_uav - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_discard_resource_uav() {

}

/// test_discard_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_discard_resource() {

}

/// test_root_parameter_preservation - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_root_parameter_preservation() {

}

/// test_cbv_hoisting - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_cbv_hoisting(use_dxil: usize) {

}

/// test_cbv_hoisting_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_cbv_hoisting_sm51() {

}

/// test_cbv_hoisting_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_cbv_hoisting_dxil() {

}

/// test_conservative_rasterization - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_conservative_rasterization(use_dxil: usize) {

}

/// test_conservative_rasterization_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_conservative_rasterization_dxbc() {

}

/// test_conservative_rasterization_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_conservative_rasterization_dxil() {

}

/// test_uninit_root_parameters - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_uninit_root_parameters() {

}

/// test_copy_texture - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_texture() {

}

/// check_sub_resource_float - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_sub_resource_float(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> usize {
    0
}

/// test_copy_texture_ds_edge_cases - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_texture_ds_edge_cases() {

}

/// ok - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ok(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// test_copy_texture_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_texture_buffer() {

}

/// test_copy_texture_bc_rgba - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_texture_bc_rgba() {

}

/// test_copy_buffer_to_depth_stencil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_buffer_to_depth_stencil() {

}

/// test_copy_buffer_texture - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_buffer_texture() {

}

/// test_multisample_resolve - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_multisample_resolve() {

}

/// R32_UINT - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn R32_UINT(arg0: usize) -> usize {
    0
}

/// test_multisample_resolve_strongly_typed - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_multisample_resolve_strongly_typed() {

}

/// test_copy_buffer_overlap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_buffer_overlap() {

}

/// test_resolve_image_exhaustive_descriptors - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_resolve_image_exhaustive_descriptors() {

}

/// supported - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn supported(arg0: usize) -> usize {
    0
}

/// test_copy_batch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_batch() {

}

/// test_resolve_subresource_depth - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_resolve_subresource_depth() {

}

/// test_copy_subresource_depth_stencil_batch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_subresource_depth_stencil_batch() {

}

/// init_copy_test_context - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn init_copy_test_context(context: *mut core::ffi::c_void, arg1: usize) -> usize {
    0
}

/// destroy_copy_test_context - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn destroy_copy_test_context(context: *mut core::ffi::c_void) {

}

/// test_queue_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_queue_buffer(arg0: usize) {

}

/// test_copy_queue_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_queue_buffer() {

}

/// test_compute_queue_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_compute_queue_buffer() {

}

/// test_graphics_queue_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_graphics_queue_buffer() {

}

/// test_queue_buffer_image - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_queue_buffer_image(arg0: usize) {

}

/// test_copy_queue_buffer_image - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_queue_buffer_image() {

}

/// test_compute_queue_buffer_image - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_compute_queue_buffer_image() {

}

/// test_graphics_queue_buffer_image - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_graphics_queue_buffer_image() {

}

/// test_queue_render_target_inner - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_queue_render_target_inner(arg0: usize, msaa: usize) {

}

/// test_copy_queue_render_target - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_queue_render_target() {

}

/// test_copy_queue_render_target_msaa - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_queue_render_target_msaa() {

}

/// test_compute_queue_render_target - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_compute_queue_render_target() {

}

/// test_compute_queue_render_target_msaa - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_compute_queue_render_target_msaa() {

}

/// test_graphics_queue_render_target - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_graphics_queue_render_target() {

}

/// test_graphics_queue_render_target_msaa - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_graphics_queue_render_target_msaa() {

}

/// test_queue_depth_stencil_inner - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_queue_depth_stencil_inner(arg0: usize, msaa: usize) {

}

/// test_copy_queue_depth_stencil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_queue_depth_stencil() {

}

/// test_copy_queue_depth_stencil_msaa - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_queue_depth_stencil_msaa() {

}

/// test_compute_queue_depth_stencil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_compute_queue_depth_stencil() {

}

/// test_compute_queue_depth_stencil_msaa - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_compute_queue_depth_stencil_msaa() {

}

/// test_graphics_queue_depth_stencil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_graphics_queue_depth_stencil() {

}

/// test_graphics_queue_depth_stencil_msaa - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_graphics_queue_depth_stencil_msaa() {

}

/// test_depth_clip - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_depth_clip() {

}

/// check_depth_stencil_sampling_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_depth_stencil_sampling_(line: u32, context: *mut core::ffi::c_void, pso: *mut core::ffi::c_void, cb: *mut core::ffi::c_void, texture: *mut core::ffi::c_void, dsv_handle: usize, srv_heap: *mut core::ffi::c_void, expected_value: f32) {

}

/// test_depth_stencil_sampling - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_depth_stencil_sampling() {

}

/// test_depth_load - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_depth_load() {

}

/// test_stencil_load - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_stencil_load() {

}

/// test_early_depth_stencil_tests - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_early_depth_stencil_tests() {

}

/// init_pipeline_state_desc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn init_pipeline_state_desc(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> usize {
    0
}

/// test_depth_stencil_layout_tracking - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_depth_stencil_layout_tracking() {

}

/// test_depth_stencil_front_and_back - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_depth_stencil_front_and_back() {

}

/// test_depth_bias_behaviour - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_depth_bias_behaviour() {

}

/// test_depth_bias_formats - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_depth_bias_formats() {

}

/// test_descriptor_tables - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_descriptor_tables() {

}

/// test_descriptor_tables_overlapping_bindings - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_descriptor_tables_overlapping_bindings() {

}

/// test_update_root_descriptors - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_update_root_descriptors() {

}

/// test_update_descriptor_tables - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_update_descriptor_tables() {

}

/// test_update_compute_descriptor_tables - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_update_compute_descriptor_tables() {

}

/// test_update_descriptor_tables_after_root_signature_change - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_update_descriptor_tables_after_root_signature_change() {

}

/// test_copy_descriptors - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_descriptors() {

}

/// test_copy_descriptors_range_sizes - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_descriptors_range_sizes() {

}

/// test_copy_rtv_descriptors - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_rtv_descriptors() {

}

/// test_descriptors_visibility - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_descriptors_visibility() {

}

/// test_null_cbv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_null_cbv() {

}

/// test_null_srv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_null_srv() {

}

/// test_null_uav - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_null_uav() {

}

/// test_null_rtv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_null_rtv() {

}

/// test_cpu_descriptors_lifetime - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_cpu_descriptors_lifetime() {

}

/// test_sampler_border_color - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_border_color() {

}

/// test_typed_buffers_many_objects - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_typed_buffers_many_objects(use_dxil: usize) {

}

/// test_typed_buffers_many_objects_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_typed_buffers_many_objects_dxbc() {

}

/// test_typed_buffers_many_objects_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_typed_buffers_many_objects_dxil() {

}

/// test_view_min_lod - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_view_min_lod() {

}

/// test_typed_srv_uav_cast - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_typed_srv_uav_cast() {

}

/// test_typed_srv_cast_clear - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_typed_srv_cast_clear() {

}

/// test_uav_3d_sliced_view - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_uav_3d_sliced_view() {

}

/// test_root_descriptor_offset_sign - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_root_descriptor_offset_sign() {

}

/// test_uav_counters_null_behavior - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_uav_counters_null_behavior(use_dxil: usize) {

}

/// test_uav_counter_null_behavior_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_uav_counter_null_behavior_dxbc() {

}

/// test_uav_counter_null_behavior_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_uav_counter_null_behavior_dxil() {

}

/// test_uav_robustness_oob_structure_element - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_uav_robustness_oob_structure_element(use_dxil: usize) {

}

/// test_uav_robustness_oob_structure_element_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_uav_robustness_oob_structure_element_dxbc() {

}

/// test_uav_robustness_oob_structure_element_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_uav_robustness_oob_structure_element_dxil() {

}

/// test_sampler_non_normalized_coordinates - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_non_normalized_coordinates() {

}

/// test_sampler_rounding - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_rounding() {

}

/// test_tex_array_reinterpretation - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_tex_array_reinterpretation(use_dxil: usize, dim: usize, uav: usize) {

}

/// test_tex2d_array_reinterpretation_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_tex2d_array_reinterpretation_sm51() {

}

/// test_tex2d_array_reinterpretation_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_tex2d_array_reinterpretation_dxil() {

}

/// test_tex1d_array_reinterpretation_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_tex1d_array_reinterpretation_sm51() {

}

/// test_tex1d_array_reinterpretation_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_tex1d_array_reinterpretation_dxil() {

}

/// test_rwtex2d_array_reinterpretation_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rwtex2d_array_reinterpretation_sm51() {

}

/// test_rwtex2d_array_reinterpretation_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rwtex2d_array_reinterpretation_dxil() {

}

/// test_rwtex1d_array_reinterpretation_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rwtex1d_array_reinterpretation_sm51() {

}

/// test_rwtex1d_array_reinterpretation_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rwtex1d_array_reinterpretation_dxil() {

}

/// test_custom_border_color_limits - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_custom_border_color_limits() {

}

/// decode_srgb - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn decode_srgb(v: f32) -> f32 {
    0.0
}

/// encode_srgb - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn encode_srgb(v: f32) -> f32 {
    0.0
}

/// test_custom_border_color_srgb - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_custom_border_color_srgb() {

}

/// test_large_buffer_descriptors - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_large_buffer_descriptors() {

}

/// RADV - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn RADV(anyway: usize) -> usize {
    0
}

/// test_static_sampler_dynamic_index - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_static_sampler_dynamic_index() {

}

/// test_node_count - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_node_count() {

}

/// test_check_feature_support - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_check_feature_support() {

}

/// test_format_support - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_format_support() {

}

/// todo_if - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn todo_if(arg0: usize) -> usize {
    0
}

/// test_multisample_quality_levels - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_multisample_quality_levels() {

}

/// test_object_interface_null_cases - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_object_interface_null_cases() {

}

/// test_object_interface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_object_interface() {

}

/// test_device_removed_reason - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_device_removed_reason() {

}

/// test_enumerate_meta_commands - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_enumerate_meta_commands() {

}

/// test_vtable_origins - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_vtable_origins() {

}

/// destruction_notifier_callback - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn destruction_notifier_callback(userdata: *mut core::ffi::c_void) {

}

/// test_destruction_notifier_callback - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_destruction_notifier_callback() {

}

/// test_destruction_notifier_interfaces - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_destruction_notifier_interfaces() {

}

/// test_sdk_configuration_creation - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sdk_configuration_creation() {

}

/// test_device_factory_creation - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_device_factory_creation() {

}

/// test_sdk_configuration_set_sdk_path - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sdk_configuration_set_sdk_path() {

}

/// test_sdk_configuration1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sdk_configuration1() {

}

/// test_device_factory - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_device_factory() {

}

/// test_root_signature_serialization - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_root_signature_serialization(config: *mut core::ffi::c_void) {

}

/// test_root_signature_subobject_serialization - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_root_signature_subobject_serialization(config: *mut core::ffi::c_void) {

}

/// test_device_configuration - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_device_configuration() {

}

/// verify_hash - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn verify_hash(data: *mut u8, offset: usize, size: usize, expected: u64) {

}

/// test_dstorage_decompression - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_dstorage_decompression() {

}

/// test_vkd3d_dxvk_cmdbuf_interop - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_vkd3d_dxvk_cmdbuf_interop() {

}

/// test_enhanced_barrier_castable_formats_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_enhanced_barrier_castable_formats_buffer() {

}

/// test_enhanced_barrier_castable_formats_validation - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_enhanced_barrier_castable_formats_validation() {

}

/// test_enhanced_barrier_castable_dsv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_enhanced_barrier_castable_dsv() {

}

/// test_enhanced_barrier_castable_formats - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_enhanced_barrier_castable_formats() {

}

/// transition_resource_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn transition_resource_state(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// test_enhanced_barrier_buffer_transfer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_enhanced_barrier_buffer_transfer() {

}

/// test_enhanced_barrier_global_direct_queue_smoke - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_enhanced_barrier_global_direct_queue_smoke() {

}

/// test_enhanced_barrier_split_barrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_enhanced_barrier_split_barrier() {

}

/// test_enhanced_barrier_discard_behavior - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_enhanced_barrier_discard_behavior() {

}

/// test_enhanced_barrier_subresource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_enhanced_barrier_subresource() {

}

/// test_geometry_shader - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_geometry_shader(use_dxil: usize) {

}

/// test_geometry_shader_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_geometry_shader_dxbc() {

}

/// test_geometry_shader_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_geometry_shader_dxil() {

}

/// test_layered_rendering - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_layered_rendering(use_dxil: usize) {

}

/// test_layered_rendering_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_layered_rendering_dxbc() {

}

/// test_layered_rendering_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_layered_rendering_dxil() {

}

/// test_ps_layer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_ps_layer(use_dxil: usize) {

}

/// test_ps_layer_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_ps_layer_dxbc() {

}

/// test_ps_layer_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_ps_layer_dxil() {

}

/// test_invalid_resource_barriers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_invalid_resource_barriers() {

}

/// test_invalid_copy_texture_region - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_invalid_copy_texture_region() {

}

/// test_invalid_unordered_access_views - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_invalid_unordered_access_views() {

}

/// test_mesh_shader_rendering - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_mesh_shader_rendering() {

}

/// test_mesh_shader_execute_indirect - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_mesh_shader_execute_indirect() {

}

/// test_mesh_shader_execute_indirect_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_mesh_shader_execute_indirect_state() {

}

/// test_amplification_shader - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_amplification_shader() {

}

/// test_amplification_shader_execute_indirect_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_amplification_shader_execute_indirect_state() {

}

/// test_integer_blending_pipeline_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_integer_blending_pipeline_state() {

}

/// test_shader_interstage_interface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_interstage_interface() {

}

/// test_shader_input_output_components - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_input_output_components() {

}

/// test_append_aligned_element - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_append_aligned_element() {

}

/// test_blend_factor - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_blend_factor() {

}

/// test_dual_source_blending - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_dual_source_blending(use_dxil: usize) {

}

/// test_dual_source_blending_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_dual_source_blending_dxbc() {

}

/// test_dual_source_blending_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_dual_source_blending_dxil() {

}

/// test_primitive_restart - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_primitive_restart() {

}

/// test_mismatching_pso_stages - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_mismatching_pso_stages() {

}

/// test_pipeline_no_ps_nonzero_rts - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_pipeline_no_ps_nonzero_rts() {

}

/// test_dynamic_depth_bias - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_dynamic_depth_bias() {

}

/// test_dynamic_index_strip_cut - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_dynamic_index_strip_cut() {

}

/// AMD - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn AMD(arg0: usize) -> usize {
    0
}

/// test_line_rasterization - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_line_rasterization() {

}

/// test_shader_io_mismatch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_io_mismatch() {

}

/// test_descriptor_range_validation - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_descriptor_range_validation() {

}

/// test_get_cached_blob - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_get_cached_blob() {

}

/// test_pipeline_library - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_pipeline_library() {

}

/// float4 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn float4(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// test_query_timestamp - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_query_timestamp() {

}

/// test_query_pipeline_statistics - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_query_pipeline_statistics() {

}

/// test_query_occlusion - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_query_occlusion() {

}

/// test_resolve_non_issued_query_data - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_resolve_non_issued_query_data() {

}

/// test_resolve_query_data_in_different_command_list - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_resolve_query_data_in_different_command_list() {

}

/// test_resolve_query_data_in_reordered_command_list - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_resolve_query_data_in_reordered_command_list() {

}

/// test_virtual_queries - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_virtual_queries() {

}

/// destroy_raytracing_test_context - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn destroy_raytracing_test_context(context: *mut core::ffi::c_void) {

}

/// init_raytracing_test_context - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn init_raytracing_test_context(context: *mut core::ffi::c_void, tier: usize) -> usize {
    0
}

/// get_rayquery_shader - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_rayquery_shader() -> usize {
    0
}

/// get_static_sampler_rt_lib - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_static_sampler_rt_lib() -> usize {
    0
}

/// get_default_assignment_bindings_rt_lib - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_default_assignment_bindings_rt_lib() -> usize {
    0
}

/// get_dummy_raygen_rt_lib - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_dummy_raygen_rt_lib() -> usize {
    0
}

/// get_embedded_root_signature_subobject_rt_lib - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_embedded_root_signature_subobject_rt_lib() -> usize {
    0
}

/// get_embedded_root_signature_subobject_rt_lib_conflict - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_embedded_root_signature_subobject_rt_lib_conflict() -> usize {
    0
}

/// get_embedded_root_signature_subobject_rt_lib_conflict_mixed - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_embedded_root_signature_subobject_rt_lib_conflict_mixed() -> usize {
    0
}

/// get_embedded_subobject_rt_lib - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_embedded_subobject_rt_lib() -> usize {
    0
}

/// get_embedded_subobject_dupe_rt_lib - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_embedded_subobject_dupe_rt_lib() -> usize {
    0
}

/// get_default_rt_lib - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_default_rt_lib() -> usize {
    0
}

/// get_multi_rs_lib - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_multi_rs_lib() -> usize {
    0
}

/// get_misfire_lib - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_misfire_lib() -> usize {
    0
}

/// get_omm_lib - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_omm_lib() -> usize {
    0
}

/// destroy_test_geometry - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn destroy_test_geometry(geom: *mut core::ffi::c_void) {

}

/// init_test_geometry - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn init_test_geometry(device: *mut core::ffi::c_void, geom: *mut core::ffi::c_void) {

}

/// init_test_omm_geometry - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn init_test_omm_geometry(device: *mut core::ffi::c_void, geom: *mut core::ffi::c_void) {

}

/// update_acceleration_structure - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn update_acceleration_structure(context: *mut core::ffi::c_void, inputs: *mut core::ffi::c_void, rtas: *mut core::ffi::c_void) {

}

/// destroy_acceleration_structure - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn destroy_acceleration_structure(rtas: *mut core::ffi::c_void) {

}

/// destroy_rt_geometry - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn destroy_rt_geometry(rt_geom: *mut core::ffi::c_void) {

}

/// destroy_rt_omm_geometry - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn destroy_rt_omm_geometry(rt_omm_geom: *mut core::ffi::c_void) {

}

/// init_rt_omm_geometry - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn init_rt_omm_geometry(context: *mut core::ffi::c_void, rt_omm_geom: *mut core::ffi::c_void, geom: *mut core::ffi::c_void, micromap_payload: *mut core::ffi::c_void, micromap_payload_size: usize, config: *mut core::ffi::c_void) {

}

/// init_rt_geometry - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn init_rt_geometry(context: *mut core::ffi::c_void, rt_geom: *mut core::ffi::c_void, geom: *mut core::ffi::c_void, num_geom_desc: u32, geom_offset_x: f32, num_unmasked_instances_y: u32, instance_geom_scale: f32, instance_offset_y: f32, postbuild_va: usize) {

}

/// rt_pso_factory_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_init(factory: *mut core::ffi::c_void) {

}

/// rt_pso_factory_add_subobject - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_add_subobject(factory: *mut core::ffi::c_void, object: *mut core::ffi::c_void) -> u32 {
    0
}

/// rt_pso_factory_add_state_object_config - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_add_state_object_config(factory: *mut core::ffi::c_void, flags: usize) -> u32 {
    0
}

/// rt_pso_factory_add_pipeline_config - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_add_pipeline_config(factory: *mut core::ffi::c_void, recursion_depth: u32) -> u32 {
    0
}

/// rt_pso_factory_add_pipeline_config1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_add_pipeline_config1(factory: *mut core::ffi::c_void, recursion_depth: u32, flags: usize) -> u32 {
    0
}

/// rt_pso_factory_add_shader_config - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_add_shader_config(factory: *mut core::ffi::c_void, attrib_size: u32, payload_size: u32) -> u32 {
    0
}

/// rt_pso_factory_add_global_root_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_add_global_root_signature(factory: *mut core::ffi::c_void, rs: *mut core::ffi::c_void) -> u32 {
    0
}

/// rt_pso_factory_add_local_root_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_add_local_root_signature(factory: *mut core::ffi::c_void, rs: *mut core::ffi::c_void) -> u32 {
    0
}

/// rt_pso_factory_add_dxil_library - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_add_dxil_library(factory: *mut core::ffi::c_void, dxil: usize, num_exports: u32, exports: *mut core::ffi::c_void) -> u32 {
    0
}

/// rt_pso_factory_add_hit_group - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_add_hit_group(factory: *mut core::ffi::c_void, hit_group: *mut core::ffi::c_void) -> u32 {
    0
}

/// rt_pso_factory_add_existing_collection - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_add_existing_collection(factory: *mut core::ffi::c_void, collection: *mut core::ffi::c_void, num_exports: u32, exports: *mut core::ffi::c_void) -> u32 {
    0
}

/// rt_pso_factory_add_default_node_mask - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_add_default_node_mask(factory: *mut core::ffi::c_void) -> u32 {
    0
}

/// rt_pso_factory_compile - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_compile(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// test_mode_to_trace_flags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_mode_to_trace_flags(mode: usize) -> u32 {
    0
}

/// test_raytracing_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_pipeline(mode: usize, minimum_tier: usize) {

}

/// mismatch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn mismatch(f: usize) -> usize {
    0
}

/// test_raytracing - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing() {

}

/// test_rayquery_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rayquery_pipeline(mode: usize, root_table: usize) {

}

/// test_rayquery - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rayquery(root_table: usize) {

}

/// test_rayquery_root_desc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rayquery_root_desc() {

}

/// test_rayquery_root_table - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rayquery_root_table() {

}

/// test_raytracing_local_rs_static_sampler_inner - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_local_rs_static_sampler_inner(use_libraries: usize) {

}

/// test_raytracing_local_rs_static_sampler - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_local_rs_static_sampler() {

}

/// test_raytracing_local_rs_static_sampler_collection - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_local_rs_static_sampler_collection() {

}

/// test_raytracing_no_global_root_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_no_global_root_signature() {

}

/// test_raytracing_default_association_tiebreak - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_default_association_tiebreak() {

}

/// test_raytracing_reject_duplicate_objects - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_reject_duplicate_objects() {

}

/// test_raytracing_embedded_subobjects - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_embedded_subobjects() {

}

/// test_raytracing_collection_identifiers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_collection_identifiers() {

}

/// test_raytracing_object_assignment_ignore_default - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_object_assignment_ignore_default() {

}

/// test_raytracing_root_signature_from_subobject - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_root_signature_from_subobject() {

}

/// test_raytracing_multi_global_rs - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_multi_global_rs() {

}

/// test_raytracing_deferred_compilation - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_deferred_compilation() {

}

/// test_raytracing_mismatch_global_rs_link - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_mismatch_global_rs_link() {

}

/// test_raytracing_null_rtas - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_null_rtas() {

}

/// BarycentricsToSpaceFillingCurveIndex - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn BarycentricsToSpaceFillingCurveIndex(u: f32, v: f32, level: u32) -> u32 {
    0
}

/// test_raytracing_opacity_micro_map - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_opacity_micro_map() {

}

/// test_raytracing_acceleration_structure_validation - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_acceleration_structure_validation() {

}

/// test_raytracing_huge_dispatch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_huge_dispatch() {

}

/// test_unbound_rtv_rendering - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_unbound_rtv_rendering() {

}

/// test_unknown_rtv_format - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_unknown_rtv_format() {

}

/// test_unknown_dsv_format - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_unknown_dsv_format() {

}

/// test_depth_stencil_test_no_dsv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_depth_stencil_test_no_dsv() {

}

/// test_render_a8_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_render_a8_dxbc() {

}

/// test_render_a8_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_render_a8_dxil() {

}

/// test_multisample_rendering - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_multisample_rendering() {

}

/// test_rendering_no_attachments_layers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rendering_no_attachments_layers() {

}

/// test_renderpass_validation - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_renderpass_validation() {

}

/// test_renderpass_rendering - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_renderpass_rendering() {

}

/// test_renderpass_resolve_suspend_resume - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_renderpass_resolve_suspend_resume() {

}

/// test_scissor_clamping - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_scissor_clamping() {

}

/// test_mismatching_rtv_dsv_size - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_mismatching_rtv_dsv_size() {

}

/// decode_rgb9e5 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn decode_rgb9e5(packed: u32, color3: f32) {

}

/// test_rgb9e5_rendering - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rgb9e5_rendering() {

}

/// test_unused_attachments_mix_and_match - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_unused_attachments_mix_and_match() {

}

/// test_render_pass_suspend_resume_opts - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_render_pass_suspend_resume_opts(inline_clears: usize) {

}

/// test_render_pass_suspend_resume_opts_disabled - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_render_pass_suspend_resume_opts_disabled() {

}

/// test_map_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_map_resource() {

}

/// test_map_placed_resources - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_map_placed_resources() {

}

/// test_get_copyable_footprints_planar - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_get_copyable_footprints_planar() {

}

/// test_get_copyable_footprints - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_get_copyable_footprints() {

}

/// padded - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn padded(u: usize) -> usize {
    0
}

/// test_placed_image_alignment - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_placed_image_alignment() {

}

/// unxpected - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn unxpected(f: usize) -> usize {
    0
}

/// test_map_texture_validation - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_map_texture_validation() {

}

/// check_video_format_subresource_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_video_format_subresource_(line: u32, rb: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, plane_idx: u32, data: *mut core::ffi::c_void, constant_data: u32) {

}

/// test_planar_video_formats - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_planar_video_formats() {

}

/// test_large_texel_buffer_view - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_large_texel_buffer_view() {

}

/// test_large_heap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_large_heap() {

}

/// test_non_zeroed_behavior - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_non_zeroed_behavior(placed: usize) {

}

/// test_heap_non_zeroed_behavior_stress - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_heap_non_zeroed_behavior_stress() {

}

/// test_committed_non_zeroed_behavior_stress - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_committed_non_zeroed_behavior_stress() {

}

/// test_tight_resource_alignment - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_tight_resource_alignment() {

}

/// test_placed_msaa_alignment_workaround - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_placed_msaa_alignment_workaround() {

}

/// test_placed_msaa_alignments - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_placed_msaa_alignments() {

}

/// test_buffer_rtv_dsv_usage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_buffer_rtv_dsv_usage() {

}

/// destroy_test_context - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn destroy_test_context(arg0: usize) -> usize {
    0
}

/// test_buffers_oob_behavior_vectorized_structured_16bit - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_buffers_oob_behavior_vectorized_structured_16bit() {

}

/// test_buffers_oob_behavior_vectorized_byte_address - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_buffers_oob_behavior_vectorized_byte_address() {

}

/// test_buffers_oob_behavior - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_buffers_oob_behavior(use_dxil: usize) {

}

/// iteration - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn iteration(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// STAMP - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn STAMP(arg0: usize, arg1: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// test_buffers_oob_behavior_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_buffers_oob_behavior_dxbc() {

}

/// test_buffers_oob_behavior_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_buffers_oob_behavior_dxil() {

}

/// test_undefined_structured_raw_alias - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_undefined_structured_raw_alias(use_dxil: usize) {

}

/// test_undefined_structured_raw_alias_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_undefined_structured_raw_alias_dxbc() {

}

/// test_undefined_structured_raw_alias_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_undefined_structured_raw_alias_dxil() {

}

/// test_null_descriptor_mismatch_type - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_null_descriptor_mismatch_type() {

}

/// test_root_signature_limits - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_root_signature_limits() {

}

/// check_root_parameter_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_root_parameter_(line: u32, parameter: *mut core::ffi::c_void, expected_parameter: *mut core::ffi::c_void) {

}

/// check_root_parameter1_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_root_parameter1_(line: u32, parameter: *mut core::ffi::c_void, expected_parameter: *mut core::ffi::c_void, converted_from_v1_0: usize) {

}

/// check_root_signature_desc_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_root_signature_desc_(line: u32, desc: *mut core::ffi::c_void, expected_desc: *mut core::ffi::c_void) {

}

/// check_root_signature_desc1_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_root_signature_desc1_(line: u32, desc: *mut core::ffi::c_void, expected_desc: *mut core::ffi::c_void, converted_from_v1_0: usize) {

}

/// check_root_signature_desc2_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_root_signature_desc2_(line: u32, desc: *mut core::ffi::c_void, expected_desc: *mut core::ffi::c_void, converted_from_v1_0: usize) {

}

/// check_root_signature_deserialization_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_root_signature_deserialization_(line: u32, code: *mut core::ffi::c_void, expected_desc: *mut core::ffi::c_void, expected_desc1: *mut core::ffi::c_void) {

}

/// check_root_signature_serialization_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_root_signature_serialization_(line: u32, bytecode: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// check_root_signature_deserialization1_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_root_signature_deserialization1_(line: u32, code: *mut core::ffi::c_void, expected_desc: *mut core::ffi::c_void, expected_desc1: *mut core::ffi::c_void) {

}

/// check_root_signature_deserialization2_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_root_signature_deserialization2_(line: u32, code: *mut core::ffi::c_void, expected_desc: *mut core::ffi::c_void, expected_desc1: *mut core::ffi::c_void, expected_desc2: *mut core::ffi::c_void) {

}

/// check_root_signature_serialization1_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_root_signature_serialization1_(line: u32, bytecode: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// check_root_signature_serialization2_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_root_signature_serialization2_(line: u32, bytecode: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// test_root_signature_byte_code - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_root_signature_byte_code() {

}

/// test_root_signature_byte_code2 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_root_signature_byte_code2() {

}

/// test_root_signature_priority - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_root_signature_priority() {

}

/// test_root_signature_empty_blob - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_root_signature_empty_blob() {

}

/// test_root_signature_embedded - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_root_signature_embedded() {

}

/// vs_main - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vs_main(SV_VertexID: usize) -> usize {
    0
}

/// ps_main_pixel - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ps_main_pixel(vout: usize, SV_Coverage: usize) {

}

/// ps_main_pixel_unrolled - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ps_main_pixel_unrolled(vout: usize, SV_Coverage: usize) {

}

/// ps_main_sample - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ps_main_sample(vout: usize, SV_SampleIndex: usize) {

}

/// test_rasterizer_ordered_views - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rasterizer_ordered_views(use_dxil: usize) {

}

/// test_rasterizer_ordered_views_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rasterizer_ordered_views_dxbc() {

}

/// test_rasterizer_ordered_views_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rasterizer_ordered_views_dxil() {

}

/// test_sampler_feedback_resource_creation - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_feedback_resource_creation() {

}

/// test_sampler_feedback_format_features - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_feedback_format_features() {

}

/// test_sampler_feedback_min_mip_level_inner - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_feedback_min_mip_level_inner(arrayed: usize) {

}

/// test_sampler_feedback_npot_min_mip_level - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_feedback_npot_min_mip_level() {

}

/// test_sampler_feedback_min_mip_level - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_feedback_min_mip_level() {

}

/// test_sampler_feedback_min_mip_level_array - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_feedback_min_mip_level_array() {

}

/// test_sampler_feedback_decode_encode_mip_used - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_feedback_decode_encode_mip_used() {

}

/// test_sampler_feedback_mip_used_region_level - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_feedback_mip_used_region_level() {

}

/// test_sampler_feedback_npot_used_region - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_feedback_npot_used_region() {

}

/// test_sampler_feedback_grad - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_feedback_grad() {

}

/// test_sampler_feedback_implicit_lod_inner - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_feedback_implicit_lod_inner(biased: usize) {

}

/// test_sampler_feedback_implicit_lod_aniso - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_feedback_implicit_lod_aniso() {

}

/// test_sampler_feedback_implicit_lod - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_feedback_implicit_lod() {

}

/// test_sampler_feedback_implicit_lod_bias - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sampler_feedback_implicit_lod_bias() {

}

/// test_fragment_coords - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fragment_coords() {

}

/// test_shader_instructions_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_instructions_dxil() {

}

/// test_shader_instructions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_instructions() {

}

/// uint4 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn uint4(arg0: usize) -> usize {
    0
}

/// f16tof32 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn f16tof32(arg0: usize) -> usize {
    0
}

/// f32tof16 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn f32tof16(arg0: usize) -> usize {
    0
}

/// test_compute_shader_instructions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_compute_shader_instructions() {

}

/// test_discard_instruction - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_discard_instruction() {

}

/// test_sample_instructions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sample_instructions() {

}

/// test_texture_ld - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_texture_ld() {

}

/// test_gather - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_gather() {

}

/// test_gather_c - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_gather_c() {

}

/// test_sample_c_lz - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sample_c_lz() {

}

/// test_cube_maps - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_cube_maps() {

}

/// test_multisample_array_texture - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_multisample_array_texture() {

}

/// test_srv_component_mapping - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_srv_component_mapping() {

}

/// test_typed_buffer_uav - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_typed_buffer_uav() {

}

/// test_typed_uav_store - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_typed_uav_store() {

}

/// test_compute_shader_registers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_compute_shader_registers() {

}

/// test_tgsm - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_tgsm() {

}

/// test_uav_load - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_uav_load() {

}

/// float3 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn float3(arg0: usize, arg1: usize, arg2: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// test_cs_uav_store - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_cs_uav_store() {

}

/// compare_id - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn compare_id(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> i32 {
    0
}

/// test_uav_counters - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_uav_counters() {

}

/// test_decrement_uav_counter - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_decrement_uav_counter() {

}

/// test_atomic_instructions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_atomic_instructions(use_dxil: usize) {

}

/// test_atomic_instructions_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_atomic_instructions_dxbc() {

}

/// test_atomic_instructions_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_atomic_instructions_dxil() {

}

/// test_buffer_srv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_buffer_srv() {

}

/// test_vertex_id_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_vertex_id_dxbc() {

}

/// test_vertex_id_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_vertex_id_dxil() {

}

/// test_face_culling - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_face_culling(use_dxil: usize) {

}

/// test_face_culling_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_face_culling_dxbc() {

}

/// test_face_culling_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_face_culling_dxil() {

}

/// test_separate_bindings - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_separate_bindings() {

}

/// test_sample_mask - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sample_mask(use_dxil: usize) {

}

/// test_sample_mask_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sample_mask_dxbc() {

}

/// test_sample_mask_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sample_mask_dxil() {

}

/// test_coverage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_coverage(use_dxil: usize) {

}

/// test_coverage_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_coverage_dxbc() {

}

/// test_coverage_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_coverage_dxil() {

}

/// test_shader_get_render_target_sample_count - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_get_render_target_sample_count(use_dxil: usize) {

}

/// GetRenderTargetSampleCount - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn GetRenderTargetSampleCount() -> usize {
    0
}

/// test_shader_get_render_target_sample_count_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_get_render_target_sample_count_dxbc() {

}

/// test_shader_get_render_target_sample_count_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_get_render_target_sample_count_dxil() {

}

/// test_shader_sample_position - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_sample_position(use_dxil: usize) {

}

/// test_shader_sample_position_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_sample_position_dxbc() {

}

/// test_shader_sample_position_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_sample_position_dxil() {

}

/// test_shader_eval_attribute - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_eval_attribute(use_dxil: usize) {

}

/// test_shader_eval_attribute_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_eval_attribute_dxbc() {

}

/// test_shader_eval_attribute_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_eval_attribute_dxil() {

}

/// test_bufinfo_instruction - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bufinfo_instruction(use_dxil: usize) {

}

/// test_bufinfo_instruction_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bufinfo_instruction_dxbc() {

}

/// test_bufinfo_instruction_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bufinfo_instruction_dxil() {

}

/// test_register_space - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_register_space(use_dxil: usize) {

}

/// test_register_space_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_register_space_sm51() {

}

/// test_register_space_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_register_space_dxil() {

}

/// test_instruction_msad - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_instruction_msad(use_dxil: usize) {

}

/// test_instruction_msad_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_instruction_msad_dxbc() {

}

/// test_instruction_msad_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_instruction_msad_dxil() {

}

/// test_varying_nointerpolation_mixed_type - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_varying_nointerpolation_mixed_type(use_dxil: usize) {

}

/// test_varying_nointerpolation_mixed_type_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_varying_nointerpolation_mixed_type_dxbc() {

}

/// test_varying_nointerpolation_mixed_type_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_varying_nointerpolation_mixed_type_dxil() {

}

/// test_derivative_hoisting - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_derivative_hoisting(use_dxil: usize) {

}

/// test_derivative_hoisting_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_derivative_hoisting_dxbc() {

}

/// test_derivative_hoisting_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_derivative_hoisting_dxil() {

}

/// run_64bit_atomics_test - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn run_64bit_atomics_test(context: *mut core::ffi::c_void, cs: usize, use_heap: usize, use_typed: usize) {

}

/// test_shader_sm66_64bit_atomics - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_sm66_64bit_atomics() {

}

/// table - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn table(arg0: usize) -> usize {
    0
}

/// test_shader_sm66_compute_derivatives - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_sm66_compute_derivatives() {

}

/// test_shader_sm66_wave_size - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_sm66_wave_size() {

}

/// test_shader_sm66_quad_op_semantics - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_sm66_quad_op_semantics() {

}

/// test_sv_barycentric - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sv_barycentric() {

}

/// test_shader_fp16 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_fp16() {

}

/// test_shader_sm62_denorm - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_sm62_denorm() {

}

/// test_shader_sm66_packed - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_sm66_packed() {

}

/// test_shader_sm64_packed - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_sm64_packed() {

}

/// test_shader_waveop_maximal_convergence - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_waveop_maximal_convergence() {

}

/// test_shader_sm65_wave_intrinsics - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_sm65_wave_intrinsics() {

}

/// test_shader_sm66_is_helper_lane - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_shader_sm66_is_helper_lane() {

}

/// test_advanced_cbv_layout - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_advanced_cbv_layout() {

}

/// test_denorm_behavior - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_denorm_behavior(use_dxil: usize) {

}

/// test_denorm_behavior_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_denorm_behavior_dxbc() {

}

/// test_denorm_behavior_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_denorm_behavior_dxil() {

}

/// test_sm67_helper_lane_wave_ops - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sm67_helper_lane_wave_ops() {

}

/// test_quad_vote_sm67_compute - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_quad_vote_sm67_compute() {

}

/// test_sm67_multi_sample_uav - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sm67_multi_sample_uav() {

}

/// test_sm67_sample_cmp_level - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sm67_sample_cmp_level() {

}

/// test_sm67_dynamic_texture_offset - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sm67_dynamic_texture_offset() {

}

/// test_sm67_raw_gather - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sm67_raw_gather() {

}

/// test_sm67_integer_sampling - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sm67_integer_sampling() {

}

/// test_sm68_draw_parameters - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sm68_draw_parameters() {

}

/// test_sm68_wave_size_range - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sm68_wave_size_range() {

}

/// test_sm68_sample_cmp_bias_grad - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sm68_sample_cmp_bias_grad() {

}

/// test_sm67_helper_lane_only_wave_ops - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sm67_helper_lane_only_wave_ops() {

}

/// float_to_fp8 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn float_to_fp8(v: f32, saturate: usize) -> u8 {
    0
}

/// float_to_bf8 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn float_to_bf8(v: f32, saturate: usize) -> u8 {
    0
}

/// fp8_to_float - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn fp8_to_float(u8_value: u8) -> f32 {
    0.0
}

/// bf8_to_float - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn bf8_to_float(u8_value: u8) -> f32 {
    0.0
}

/// quant_fp8 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn quant_fp8(value: f32) -> f32 {
    0.0
}

/// quant_fp16 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn quant_fp16(value: f32) -> f32 {
    0.0
}

/// quant_fp16_fp8 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn quant_fp16_fp8(value: f32) -> f32 {
    0.0
}

/// randomize_fp8_range - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn randomize_fp8_range(base: u8, range: u8, sign: usize) -> u8 {
    0
}

/// randomize_fp8_float - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn randomize_fp8_float() -> f32 {
    0.0
}

/// device_supports_wmma_fp8 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn device_supports_wmma_fp8(device: *mut core::ffi::c_void) -> usize {
    0
}

/// device_supports_wmma_fp8_native - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn device_supports_wmma_fp8_native(device: *mut core::ffi::c_void) -> usize {
    0
}

/// test_wmma_matmul - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_wmma_matmul() {

}

/// test_wmma_multi_matmul - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_wmma_multi_matmul() {

}

/// test_wmma_fp8_fp32_conversions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_wmma_fp8_fp32_conversions() {

}

/// test_wmma_fp32_fp8_conversions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_wmma_fp32_fp8_conversions() {

}

/// test_wmma_fp32_fp8_special_conversions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_wmma_fp32_fp8_special_conversions() {

}

/// unclamped - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn unclamped(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// clamped - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn clamped(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// test_wmma_matrix_length - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_wmma_matrix_length() {

}

/// cmp_8bit - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cmp_8bit(a_: *mut core::ffi::c_void, b_: *mut core::ffi::c_void) -> i32 {
    0
}

/// test_wmma_extract_insert - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_wmma_extract_insert() {

}

/// test_wmma_lds_transpose - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_wmma_lds_transpose() {

}

/// test_wmma_lds_layout - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_wmma_lds_layout() {

}

/// test_wmma_copy_transpose - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_wmma_copy_transpose() {

}

/// test_wmma_layout_assumptions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_wmma_layout_assumptions() {

}

/// test_wmma_special_conversions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_wmma_special_conversions() {

}

/// test_wmma_element_wise - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_wmma_element_wise() {

}

/// test_ags_float8_conversion - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_ags_float8_conversion() {

}

/// test_nvx_cubin - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_nvx_cubin() {

}

/// compute_tile_count - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn compute_tile_count(resource_size: u32, mip: u32, tile_size: u32) -> u32 {
    0
}

/// test_get_resource_tiling - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_get_resource_tiling() {

}

/// set_region_offset - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn set_region_offset(region: *mut core::ffi::c_void, x: u32, y: u32, z: u32, subresource: u32) {

}

/// set_region_size - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn set_region_size(region: *mut core::ffi::c_void, num_tiles: u32, use_box: usize, w: u32, h: u32, d: u32) {

}

/// test_update_tile_mappings_remap_inner - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_update_tile_mappings_remap_inner(smem: usize) {

}

/// test_update_tile_mappings_remap_vmem - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_update_tile_mappings_remap_vmem() {

}

/// test_update_tile_mappings_remap_smem - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_update_tile_mappings_remap_smem() {

}

/// test_update_tile_mappings - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_update_tile_mappings() {

}

/// test_copy_tiles - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_copy_tiles() {

}

/// test_buffer_feedback_instructions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_buffer_feedback_instructions(use_dxil: usize) {

}

/// test_buffer_feedback_instructions_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_buffer_feedback_instructions_sm51() {

}

/// test_buffer_feedback_instructions_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_buffer_feedback_instructions_dxil() {

}

/// test_texture_feedback_instructions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_texture_feedback_instructions(use_dxil: usize) {

}

/// test_texture_feedback_instructions_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_texture_feedback_instructions_sm51() {

}

/// test_texture_feedback_instructions_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_texture_feedback_instructions_dxil() {

}

/// test_sparse_buffer_memory_lifetime - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sparse_buffer_memory_lifetime() {

}

/// test_reserved_resource_mapping - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_reserved_resource_mapping() {

}

/// test_sparse_depth_stencil_rendering - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sparse_depth_stencil_rendering() {

}

/// test_sparse_default_mapping - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_sparse_default_mapping() {

}

/// test_vertex_shader_stream_output - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_vertex_shader_stream_output(use_dxil: usize) {

}

/// test_index_buffer_edge_case_stream_output - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_index_buffer_edge_case_stream_output() {

}

/// test_vertex_shader_stream_output_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_vertex_shader_stream_output_dxbc() {

}

/// test_vertex_shader_stream_output_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_vertex_shader_stream_output_dxil() {

}

/// test_queue_wait - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_queue_wait() {

}

/// test_graphics_compute_queue_synchronization - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_graphics_compute_queue_synchronization() {

}

/// test_fence_values - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_values() {

}

/// test_cpu_signal_fence - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_cpu_signal_fence() {

}

/// test_gpu_signal_fence - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_gpu_signal_fence() {

}

/// fence_event_wait_main - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn fence_event_wait_main(untyped_data: *mut core::ffi::c_void) {

}

/// fence_busy_wait_main - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn fence_busy_wait_main(untyped_data: *mut core::ffi::c_void) {

}

/// test_fence_wait_robustness_inner - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_wait_robustness_inner(shared_handles: usize) {

}

/// test_fence_wait_robustness - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_wait_robustness() {

}

/// test_fence_wait_robustness_shared - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_wait_robustness_shared() {

}

/// test_fence_wait_multiple_inner - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_wait_multiple_inner(shared_handles: usize) {

}

/// test_fence_wait_multiple - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_wait_multiple() {

}

/// test_fence_wait_multiple_shared - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_wait_multiple_shared() {

}

/// test_concurrent_signal_stress_inner - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_concurrent_signal_stress_inner() {

}

/// test_concurrent_signal_stress - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_concurrent_signal_stress() {

}

/// test_fence_pending_signal_cpu_rewind - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_pending_signal_cpu_rewind(use_shared: usize) {

}

/// test_fence_signal_availability - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_signal_availability(use_shared: usize) {

}

/// test_fence_pending_signal_cpu_rewind_plain - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_pending_signal_cpu_rewind_plain() {

}

/// test_fence_signal_availability_plain - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_signal_availability_plain() {

}

/// test_fence_pending_signal_cpu_rewind_shared - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_pending_signal_cpu_rewind_shared() {

}

/// test_fence_signal_availability_shared - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_fence_signal_availability_shared() {

}

/// check_triangles_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_triangles_(line: u32, buffer: *mut core::ffi::c_void, queue: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void, triangles: *mut core::ffi::c_void, triangle_count: u32) {

}

/// test_nop_tessellation_shaders - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_nop_tessellation_shaders() {

}

/// test_quad_tessellation_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_quad_tessellation_dxbc() {

}

/// test_quad_tessellation_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_quad_tessellation_dxil() {

}

/// test_quad_tessellation_wrong_input_count_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_quad_tessellation_wrong_input_count_dxbc() {

}

/// test_quad_tessellation_wrong_input_count_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_quad_tessellation_wrong_input_count_dxil() {

}

/// test_tessellation_dcl_index_range_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_tessellation_dcl_index_range_dxbc() {

}

/// test_tessellation_dcl_index_range_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_tessellation_dcl_index_range_dxil() {

}

/// test_tessellation_dcl_index_range_complex_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_tessellation_dcl_index_range_complex_dxbc() {

}

/// test_tessellation_dcl_index_range_complex_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_tessellation_dcl_index_range_complex_dxil() {

}

/// test_hull_shader_control_point_phase - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_hull_shader_control_point_phase(use_dxil: usize) {

}

/// test_hull_shader_control_point_phase_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_hull_shader_control_point_phase_dxbc() {

}

/// test_hull_shader_control_point_phase_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_hull_shader_control_point_phase_dxil() {

}

/// test_hull_shader_fork_phase - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_hull_shader_fork_phase(use_dxil: usize) {

}

/// test_hull_shader_fork_phase_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_hull_shader_fork_phase_dxbc() {

}

/// test_hull_shader_fork_phase_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_hull_shader_fork_phase_dxil() {

}

/// test_line_tessellation - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_line_tessellation(use_dxil: usize) {

}

/// test_line_tessellation_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_line_tessellation_dxbc() {

}

/// test_line_tessellation_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_line_tessellation_dxil() {

}

/// test_tessellation_primitive_id - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_tessellation_primitive_id() {

}

/// test_gpu_virtual_address - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_gpu_virtual_address() {

}

/// test_vrs - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_vrs() {

}

/// test_vrs_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_vrs_dxil() {

}

/// test_vrs_image - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_vrs_image() {

}

/// test_open_heap_from_address - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_open_heap_from_address() {

}

/// init_workgraph_test_context - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn init_workgraph_test_context(context: *mut core::ffi::c_void) -> usize {
    0
}

/// destroy_workgraph_test_context - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn destroy_workgraph_test_context(context: *mut core::ffi::c_void) {

}

/// check_work_graph_properties - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_work_graph_properties(pso: *mut core::ffi::c_void, expected_program_name: *const u16, expected_entry_node: *const u16, expected_leaf_node: *const u16, expected_input_record_size: u32, ident: *mut core::ffi::c_void, reqs: *mut core::ffi::c_void) {

}

/// execute_workgraph_pso_simple - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn execute_workgraph_pso_simple(context: *mut core::ffi::c_void, pso: *mut core::ffi::c_void, ident: *mut core::ffi::c_void, wg_reqs: *mut core::ffi::c_void, node_payload: *mut core::ffi::c_void, node_payload_stride: usize, node_payload_count: usize, local_root_table: *mut core::ffi::c_void, scratch_output: *mut *mut core::ffi::c_void) {

}

/// execute_workgraph_test - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn execute_workgraph_test(context: *mut core::ffi::c_void, pso: *mut core::ffi::c_void, ident: *mut core::ffi::c_void, wg_reqs: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// test_workgraph_basic - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_workgraph_basic() {

}

/// test_workgraph_broadcast_input - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_workgraph_broadcast_input() {

}

/// test_workgraph_coalesced_input - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_workgraph_coalesced_input() {

}

/// test_workgraph_two_level_broadcast_inner - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_workgraph_two_level_broadcast_inner(context: *mut core::ffi::c_void, pso: *mut core::ffi::c_void) {

}

/// test_workgraph_two_level_broadcast - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_workgraph_two_level_broadcast() {

}

/// test_workgraph_two_level_empty - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_workgraph_two_level_empty() {

}

/// test_workgraph_basic_recursion - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_workgraph_basic_recursion() {

}

/// test_workgraph_cross_group_sharing - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_workgraph_cross_group_sharing() {

}

/// test_workgraph_shared_inputs - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_workgraph_shared_inputs() {

}

/// test_workgraph_local_root_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_workgraph_local_root_signature() {

}

/// setup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn setup(argc: i32, argv: *mut *mut i8) {

}

/// get_time - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_time() -> f64 {
    0.0
}

/// fill_descriptor_heap_srv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn fill_descriptor_heap_srv(device: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, count: u32) {

}

/// zero_descriptor_heap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn zero_descriptor_heap(device: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, count: u32) {

}

/// do_benchmark_run - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn do_benchmark_run(device: *mut core::ffi::c_void) {

}

/// SRVs - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn SRVs(arg0: usize) -> usize {
    0
}

/// heap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn heap(arg0: usize) -> usize {
    0
}

/// resource_get_internal_refcount - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn resource_get_internal_refcount(resource: *mut core::ffi::c_void) -> u32 {
    0
}

/// check_extensions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_extensions(enabled_extensions: *mut *mut i8, extensions: *mut core::ffi::c_void, extension_count: u32, properties: *mut core::ffi::c_void, count: u32) -> u32 {
    0
}

/// check_device_extensions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_device_extensions(vk_physical_device: usize, enabled_extensions: *mut *mut i8, extensions: *mut core::ffi::c_void, extension_count: u32) -> u32 {
    0
}

/// fake_vkEnumerateDeviceExtensionProperties - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn fake_vkEnumerateDeviceExtensionProperties(physical_device: usize, layer_name: *mut i8, out_count: *mut u32, out_properties: *mut core::ffi::c_void) -> usize {
    0
}

/// test_required_device_extensions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_required_device_extensions() {

}

/// test_additional_device_extensions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_additional_device_extensions() {

}

/// test_physical_device - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_physical_device() {

}

/// test_adapter_luid - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_adapter_luid() {

}

/// parent_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn parent_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// parent_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn parent_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// parent_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn parent_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// test_device_parent - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_device_parent() {

}

/// test_vkd3d_queue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_vkd3d_queue() {

}

/// test_resource_internal_refcount - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_resource_internal_refcount() {

}

/// select_vulkan_memory_type - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn select_vulkan_memory_type(device: *mut core::ffi::c_void, memory_type_mask: u32, required_flags: usize) -> u32 {
    0
}

/// test_formats - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_formats() {

}

/// check_version - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_version(v: *mut i8, expected_major: i32, expected_minor: i32) {

}

/// test_parse_version - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_parse_version() {

}

/// check_contiguous - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_contiguous(mask: u32, expected_result: usize) {

}

/// test_bitmask_is_contiguous - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bitmask_is_contiguous() {

}

/// IVKD3DCoreInterface_SerializeRootSignature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn IVKD3DCoreInterface_SerializeRootSignature(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// IVKD3DCoreInterface_SerializeVersionedRootSignature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn IVKD3DCoreInterface_SerializeVersionedRootSignature(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// IVKD3DCoreInterface_GetDebugInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn IVKD3DCoreInterface_GetDebugInterface(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// IVKD3DCoreInterface_GetInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn IVKD3DCoreInterface_GetInterface(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// wait_vr_key - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn wait_vr_key(vr_key: *mut core::ffi::c_void) -> i32 {
    0
}

/// parse_extension_list - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn parse_extension_list(extension_str: *mut i8, extension_list: *mut *mut i8) -> u32 {
    0
}

/// passthrough_unix_environment - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn passthrough_unix_environment(env_name: *mut i8) {

}

/// load_modules_once - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn load_modules_once() {

}

/// load_modules - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn load_modules() -> usize {
    0
}

/// vkd3d_debug_control_has_out_of_spec_test_behavior - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_control_has_out_of_spec_test_behavior(behavior: usize) -> usize {
    0
}

/// vkd3d_debug_control_mute_message_id - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_control_mute_message_id(vuid: *mut i8) -> usize {
    0
}

/// vkd3d_debug_control_SetRunningUnderTest - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_control_SetRunningUnderTest(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_debug_control_MuteValidationGlobal - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_control_MuteValidationGlobal(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_debug_control_UnmuteValidationGlobal - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_control_UnmuteValidationGlobal(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_debug_control_MuteValidationMessageID - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_control_MuteValidationMessageID(iface: *mut core::ffi::c_void, vuid: *mut i8, explanation: *mut i8) -> usize {
    0
}

/// vkd3d_debug_control_UnmuteValidationMessageID - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_control_UnmuteValidationMessageID(iface: *mut core::ffi::c_void, vuid: *mut i8) -> usize {
    0
}

/// vkd3d_debug_control_SetOutOfSpecTestBehavior - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_control_SetOutOfSpecTestBehavior(iface: *mut core::ffi::c_void, behavior: usize, enable: i32) -> usize {
    0
}

/// vkd3d_debug_control_SetBehaviorFlags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_control_SetBehaviorFlags(iface: *mut core::ffi::c_void, behavior: usize) -> usize {
    0
}

/// d3d12_build_flags_to_vk - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_build_flags_to_vk(flags: usize) -> usize {
    0
}

/// d3d12_geometry_flags_to_vk - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_geometry_flags_to_vk(flags: usize) -> usize {
    0
}

/// vkd3d_acceleration_structure_convert_triangles - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_acceleration_structure_convert_triangles(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, geometry_info: *mut core::ffi::c_void, primitive_count: *mut u32) {

}

/// vkd3d_acceleration_structure_end_barrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_acceleration_structure_end_barrier(list: *mut core::ffi::c_void) {

}

/// vkd3d_setup_empty_rtas_build - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_setup_empty_rtas_build(device: *mut core::ffi::c_void, info: *mut core::ffi::c_void) {

}

/// vkd3d_address_binding_callback - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_address_binding_callback(message_severity: usize, message_types: usize, callback_data: *mut core::ffi::c_void, userdata: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_address_binding_tracker_report_entry - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_address_binding_tracker_report_entry(report: *mut core::ffi::c_void, address: usize) {

}

/// BEGIN - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn BEGIN(accuracy: usize) -> usize {
    0
}

/// vkd3d_compute_size_varint - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compute_size_varint(words: *mut u32, word_count: usize) -> usize {
    0
}

/// vkd3d_decode_varint - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_decode_varint(words: *mut u32, words_size: usize, buffer: *mut u8, buffer_size: usize) -> usize {
    0
}

/// VK_CALL - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn VK_CALL(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// vkd3d_pipeline_blob_compute_data_checksum - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_blob_compute_data_checksum(data: *mut u8, size: usize) -> u32 {
    0
}

/// vkd3d_serialized_pipeline_stream_entry_compute_checksum - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_serialized_pipeline_stream_entry_compute_checksum(data: *mut u8, entry: *mut core::ffi::c_void) -> u64 {
    0
}

/// vkd3d_serialized_pipeline_stream_entry_validate - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_serialized_pipeline_stream_entry_validate(data: *mut u8, entry: *mut core::ffi::c_void) -> usize {
    0
}

/// find_blob_chunk_masked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn find_blob_chunk_masked(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// vkd3d_shader_code_compute_serialized_size - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_code_compute_serialized_size(code: *mut core::ffi::c_void, out_varint_size: *mut usize, inline_spirv: usize) -> usize {
    0
}

/// vkd3d_shader_code_serialize_inline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_code_serialize_inline(code: *mut core::ffi::c_void, stage: usize, varint_size: usize, inout_chunk: *mut *mut core::ffi::c_void) {

}

/// vkd3d_shader_code_serialize_identifier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_code_serialize_identifier(pipeline_library: *mut core::ffi::c_void, code: *mut core::ffi::c_void, identifier: *mut core::ffi::c_void, stage: usize, inout_chunk: *mut *mut core::ffi::c_void) {

}

/// vkd3d_shader_code_serialize_referenced - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_code_serialize_referenced(pipeline_library: *mut core::ffi::c_void, code: *mut core::ffi::c_void, stage: usize, varint_size: usize, inout_chunk: *mut *mut core::ffi::c_void) {

}

/// vkd3d_serialize_pipeline_state_inline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_serialize_pipeline_state_inline(state: *mut core::ffi::c_void, chunk: *mut core::ffi::c_void, vk_pipeline_cache_size: usize, varint_size: *mut usize) -> i32 {
    0
}

/// vkd3d_serialize_pipeline_state_referenced - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_serialize_pipeline_state_referenced(pipeline_library: *mut core::ffi::c_void, state: *mut core::ffi::c_void, chunk: *mut core::ffi::c_void, vk_pipeline_cache_size: usize, varint_size: *mut usize) -> i32 {
    0
}

/// vkd3d_cached_pipeline_hash_name - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_cached_pipeline_hash_name(key: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_cached_pipeline_hash_internal - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_cached_pipeline_hash_internal(key: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_cached_pipeline_compare_name - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_cached_pipeline_compare_name(key: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_cached_pipeline_compare_internal - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_cached_pipeline_compare_internal(key: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_pipeline_library_disk_cache_save_pipeline_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_library_disk_cache_save_pipeline_state(cache: *mut core::ffi::c_void, item: *mut core::ffi::c_void) -> i32 {
    0
}

/// disk_cache_entry_key_cb - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn disk_cache_entry_key_cb(key_: *mut core::ffi::c_void) -> u32 {
    0
}

/// disk_cache_entry_compare_cb - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn disk_cache_entry_compare_cb(key: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_pipeline_library_disk_cache_merge - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_library_disk_cache_merge(cache: *mut core::ffi::c_void, read_path: *mut i8, write_path: *mut i8) {

}

/// invalid - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn invalid(vkd3d_file_rename_no_replaceread_path: usize, arg1: usize) -> usize {
    0
}

/// INFO - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn INFO(cachen: usize) -> usize {
    0
}

/// vkd3d_pipeline_library_disk_cache_initial_setup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_library_disk_cache_initial_setup(cache: *mut core::ffi::c_void) {

}

/// vkd3d_waiting_fence_release_submission - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_waiting_fence_release_submission(worker: *mut core::ffi::c_void, userdata: *mut core::ffi::c_void, complete: usize) {

}

/// vkd3d_waiting_fence_release_sparse_resources - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_waiting_fence_release_sparse_resources(worker: *mut core::ffi::c_void, userdata: *mut core::ffi::c_void, complete: usize) {

}

/// vkd3d_waiting_fence_signal_fence - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_waiting_fence_signal_fence(worker: *mut core::ffi::c_void, userdata: *mut core::ffi::c_void, complete: usize) {

}

/// vkd3d_waiting_fence_release_fence - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_waiting_fence_release_fence(worker: *mut core::ffi::c_void, userdata: *mut core::ffi::c_void, complete: usize) {

}

/// vkd3d_waiting_fence_complete_submissions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_waiting_fence_complete_submissions(device: *mut core::ffi::c_void, worker: *mut core::ffi::c_void, fence: *mut core::ffi::c_void, complete: usize) {

}

/// vkd3d_wait_for_gpu_timeline_semaphore - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_wait_for_gpu_timeline_semaphore(worker: *mut core::ffi::c_void, fence: *mut core::ffi::c_void) {

}

/// vkd3d_fence_worker_start - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_fence_worker_start(worker: *mut core::ffi::c_void, queue: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_fence_worker_stop - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_fence_worker_stop(worker: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_fence_destroy_vk_objects - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_destroy_vk_objects(fence: *mut core::ffi::c_void) {

}

/// vkd3d_waiting_event_signal - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_waiting_event_signal(device: *mut core::ffi::c_void, worker: *mut core::ffi::c_void, event: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_debug_control_is_test_suite - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_control_is_test_suite() -> usize {
    0
}

/// vk_rect_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_rect_from_d3d12(rect: *mut core::ffi::c_void, vk_rect: *mut core::ffi::c_void, clamp_rect: *mut core::ffi::c_void) -> usize {
    0
}

/// WAW - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn WAW(arg0: usize) -> usize {
    0
}

/// dsv_plane_optimal_mask_to_layout - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dsv_plane_optimal_mask_to_layout(arg0: usize, arg1: usize) -> usize {
    0
}

/// vk_separate_depth_layout - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_separate_depth_layout(combined_layout: usize) -> usize {
    0
}

/// vk_separate_stencil_layout - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_separate_stencil_layout(combined_layout: usize) -> usize {
    0
}

/// vk_queue_shader_stages - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_queue_shader_stages(device: *mut core::ffi::c_void, vk_queue_flags: usize) -> usize {
    0
}

/// vk_render_pass_barrier_from_view - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_render_pass_barrier_from_view(list: *mut core::ffi::c_void, view: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, mode: usize, layout: usize, vk_barrier: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_query_type_get_vk_flags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_type_get_vk_flags(arg0: usize) -> usize {
    0
}

/// vkd3d_compare_pending_query - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compare_pending_query(query_a: *mut core::ffi::c_void, query_b: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_check_subresource_overlap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_check_subresource_overlap(a: *mut core::ffi::c_void, a_subresources: *mut core::ffi::c_void, b: *mut core::ffi::c_void, b_subresources: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_check_rtv_overlap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_check_rtv_overlap(rtv: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, subresources: *mut core::ffi::c_void) -> usize {
    0
}

/// vk_access_and_stage_flags_from_d3d12_resource_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_access_and_stage_flags_from_d3d12_resource_state(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, state_mask: u32, vk_queue_flags: usize, stages: *mut core::ffi::c_void, access: *mut core::ffi::c_void) {

}

/// d3d12_command_list_vkd3d_ext_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_vkd3d_ext_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_find_query - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_find_query(query_ranges: *mut core::ffi::c_void, query_ranges_count: usize, vk_pool: usize, index: u32, out_pos: *mut usize) -> usize {
    0
}

/// vkd3d_insert_query_range - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_insert_query_range(arg0: *mut usize, vk_pool: usize, index: u32, count: u32, flags: u32, out_ranges: *mut core::ffi::c_void) {

}

/// vk_buffer_image_copy_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_buffer_image_copy_from_d3d12(copy: *mut core::ffi::c_void, footprint: *mut core::ffi::c_void, sub_resource_idx: u32, image_desc: *mut core::ffi::c_void, src_format: *mut core::ffi::c_void, dst_format: *mut core::ffi::c_void, src_box: *mut core::ffi::c_void, dst_x: u32, dst_y: u32, dst_z: u32) {

}

/// vk_image_buffer_copy_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_image_buffer_copy_from_d3d12(copy: *mut core::ffi::c_void, footprint_size: *mut u64, footprint: *mut core::ffi::c_void, sub_resource_idx: u32, image_desc: *mut core::ffi::c_void, src_format: *mut core::ffi::c_void, dst_format: *mut core::ffi::c_void, src_box: *mut core::ffi::c_void, dst_x: u32, dst_y: u32, dst_z: u32) {

}

/// vk_image_copy_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_image_copy_from_d3d12(image_copy: *mut core::ffi::c_void, src_sub_resource_idx: u32, dst_sub_resource_idx: u32, src_desc: *mut core::ffi::c_void, dst_desc: *mut core::ffi::c_void, src_format: *mut core::ffi::c_void, dst_format: *mut core::ffi::c_void, src_box: *mut core::ffi::c_void, dst_x: u32, dst_y: u32, dst_z: u32) -> usize {
    0
}

/// vkd3d_get_tile_index_from_region - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_tile_index_from_region(sparse: *mut core::ffi::c_void, coord: *mut core::ffi::c_void, size: *mut core::ffi::c_void, tile_index_in_region: u32) -> u32 {
    0
}

/// vk_resolve_mode_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_resolve_mode_from_d3d12(mode: usize) -> usize {
    0
}

/// vkd3d_format_from_d3d12_resource_desc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_format_from_d3d12_resource_desc(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// vkd3d_get_resolve_dst_outside_access - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_resolve_dst_outside_access(device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, path: usize, vk_stages: *mut core::ffi::c_void, vk_access: *mut core::ffi::c_void) {

}

/// shader - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader(arg0: usize) -> usize {
    0
}

/// vk_image_layout_from_d3d12_barrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_image_layout_from_d3d12_barrier(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, layout: usize) -> usize {
    0
}

/// vk_image_memory_barrier_subresources_from_d3d12_texture_barrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_image_memory_barrier_subresources_from_d3d12_texture_barrier(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, range: *mut core::ffi::c_void, dsv_decay_mask: u32, vk_range: *mut core::ffi::c_void) {

}

/// vk_image_memory_barrier_for_transition - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_image_memory_barrier_for_transition(image_barrier: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, subresource_idx: u32, old_layout: usize, new_layout: usize, src_stages: usize, src_access: usize, dst_stages: usize, dst_access: usize, dsv_decay_mask: u32) {

}

/// vk_subresource_range_overlaps - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_subresource_range_overlaps(base_a: u32, count_a: u32, base_b: u32, count_b: u32) -> usize {
    0
}

/// vk_image_barrier_overlaps_subresource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_image_barrier_overlaps_subresource(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void, exact_match: *mut core::ffi::c_void) -> usize {
    0
}

/// barrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn barrier(p: usize, preserve_resource: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// vkd3d_pipeline_bindings_set_dirty_sets - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_bindings_set_dirty_sets(bindings: *mut core::ffi::c_void, dirty_mask: u64) {

}

/// clamp_float_to_uint32 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn clamp_float_to_uint32(value: f32, max_value: u32) -> u32 {
    0
}

/// clamp_float_to_narrow_float - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn clamp_float_to_narrow_float(value: f32, max_finite_value: f32, ufloat: usize, flush_nan_inf: usize) -> f32 {
    0.0
}

/// copy_and_clamp_clear_color - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn copy_and_clamp_clear_color(vk_format: usize, color4: f32, in_color4: f32) {

}

/// vkd3d_fixup_clear_uav_uint_color - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_fixup_clear_uav_uint_color(device: *mut core::ffi::c_void, dxgi_format: usize, color: usize) -> usize {
    0
}

/// vkd3d_clear_uav_info_from_metadata - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_clear_uav_info_from_metadata(args: *mut core::ffi::c_void, metadata: usize) -> usize {
    0
}

/// vkd3d_mask_uint_clear_color - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_mask_uint_clear_color(color4: u32, vk_format: usize) {

}

/// vkd3d_clear_uav_synthesize_buffer_view - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_clear_uav_synthesize_buffer_view(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, args: *mut core::ffi::c_void, override_format: *mut core::ffi::c_void, inline_view: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_query_lookup_entry_hash - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_query_lookup_entry_hash(key: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_query_lookup_entry_compare - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_query_lookup_entry_compare(key: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) -> usize {
    0
}

/// vk_clear_color_value_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_clear_color_value_from_d3d12(vk_value: *mut core::ffi::c_void, d3d_value: *mut core::ffi::c_void) {

}

/// vk_clear_depth_stencil_value_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_clear_depth_stencil_value_from_d3d12(vk_value: *mut core::ffi::c_void, d3d_depth_value: *mut core::ffi::c_void, d3d_stencil_value: *mut core::ffi::c_void) {

}

/// convert_strided_range - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn convert_strided_range(region: *mut core::ffi::c_void) -> usize {
    0
}

/// vk_fragment_size_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_fragment_size_from_d3d12(axis_rate: usize) -> u32 {
    0
}

/// vk_stage_flags_from_d3d12_barrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_stage_flags_from_d3d12_barrier(list: *mut core::ffi::c_void, sync: usize, access: usize) -> usize {
    0
}

/// vk_access_flags_from_d3d12_barrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_access_flags_from_d3d12_barrier(list: *mut core::ffi::c_void, access: usize) -> usize {
    0
}

/// vk_sanitize_stage_flags_for_access - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_sanitize_stage_flags_for_access(list: *mut core::ffi::c_void, stages: usize, access: usize) -> usize {
    0
}

/// bounds - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn bounds(programs: usize) -> usize {
    0
}

/// d3d12_command_queue_vkd3d_ext_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_vkd3d_ext_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_get_tile_index_from_coordinate - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_tile_index_from_coordinate(sparse: *mut core::ffi::c_void, coord: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_submission_has_query_reset_hazard - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_submission_has_query_reset_hazard(command_list_count: u32, command_lists: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_waiting_fence_ensure_signal_order - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_waiting_fence_ensure_signal_order(worker: *mut core::ffi::c_void, userdata: *mut core::ffi::c_void, complete: usize) {

}

/// vkd3d_compact_sparse_bind_ranges - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compact_sparse_bind_ranges(src_resource: *mut core::ffi::c_void, bind_ranges: *mut core::ffi::c_void, bind_infos: *mut core::ffi::c_void, count: u32, mode: usize) -> u32 {
    0
}

/// removed - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn removed(queuedescNodeMask: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_vkd3d_ext_LaunchCubinShaderEx - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_vkd3d_ext_LaunchCubinShaderEx(iface: *mut core::ffi::c_void, handle: *mut core::ffi::c_void, block_x: usize, block_y: usize, block_z: usize, smem_size: usize, params: *mut core::ffi::c_void, param_size: usize, raw_params: *mut core::ffi::c_void, raw_params_count: usize) -> usize {
    0
}

/// d3d12_command_queue_vkd3d_ext_NotifyOutOfBandCommandQueue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_vkd3d_ext_NotifyOutOfBandCommandQueue(iface: *mut core::ffi::c_void, arg1: usize) -> usize {
    0
}

/// Tasks - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn Tasks(arg0: usize) -> usize {
    0
}

/// vkd3d_shader_debug_ring_print_message - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_debug_ring_print_message(ring: *mut core::ffi::c_void, word_offset: u32, message_word_count: u32) -> usize {
    0
}

/// vkd3d_patch_command_token_str - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_patch_command_token_str(arg0: usize) -> usize {
    0
}

/// small - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn small(iteration: usize) -> usize {
    0
}

/// vkd3d_descriptor_debug_init_once - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_init_once() {

}

/// offsetof - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn offsetof(vkd3d_descriptor_qa_heap_buffer_data: usize, arg1: usize) -> usize {
    0
}

/// vkd3d_descriptor_debug_qa_check_report_fault - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_qa_check_report_fault(global_info: *mut core::ffi::c_void) {

}

/// vkd3d_descriptor_debug_parse_shader_ranges - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_parse_shader_ranges(global_info: *mut core::ffi::c_void, kind: usize) {

}

/// vkd3d_get_vk_version - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_vk_version() -> u32 {
    0
}

/// VK_MAKE_VERSION - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn VK_MAKE_VERSION(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// get_spec_version - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_spec_version(extensions: *mut core::ffi::c_void, count: u32, extension_name: *mut i8) -> u32 {
    0
}

/// is_extension_disabled - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn is_extension_disabled(extension_name: *mut i8) -> usize {
    0
}

/// has_extension - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn has_extension(extensions: *mut core::ffi::c_void, count: u32, extension_name: *mut i8, minimum_spec_version: u32) -> usize {
    0
}

/// vkd3d_check_extensions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_check_extensions(extensions: *mut core::ffi::c_void, count: u32, required_extensions: *mut *mut i8, required_extension_count: u32, optional_extensions: *mut core::ffi::c_void, optional_extension_count: u32, user_extensions: *mut *mut i8, user_extension_count: u32, optional_user_extensions: *mut *mut i8, optional_user_extension_count: u32, user_extension_supported: *mut core::ffi::c_void, vulkan_info: *mut core::ffi::c_void, extension_type: *mut i8) -> u32 {
    0
}

/// vkd3d_append_extension - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_append_extension(extensions: *mut i8, extension_count: u32, extension_name: *mut i8) -> u32 {
    0
}

/// vkd3d_mark_enabled_user_extensions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_mark_enabled_user_extensions(vulkan_info: *mut core::ffi::c_void, optional_user_extensions: *mut *mut i8, optional_user_extension_count: u32, user_extension_supported: *mut core::ffi::c_void) {

}

/// vkd3d_enable_extensions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_enable_extensions(extensions: *mut i8, required_extensions: *mut *mut i8, required_extension_count: u32, optional_extensions: *mut core::ffi::c_void, optional_extension_count: u32, user_extensions: *mut *mut i8, user_extension_count: u32, optional_user_extensions: *mut *mut i8, optional_user_extension_count: u32, user_extension_supported: *mut core::ffi::c_void, vulkan_info: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_remove_extension - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_remove_extension(to_remove: *mut i8, extensions: *mut i8, extension_count: *mut u32) -> usize {
    0
}

/// vkd3d_disable_nvx_extensions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_disable_nvx_extensions(device: *mut core::ffi::c_void, extensions: *mut i8, enabled_extension_count: *mut u32) -> usize {
    0
}

/// vkd3d_init_vk_global_procs - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_vk_global_procs(instance: *mut core::ffi::c_void, vkGetInstanceProcAddr: usize) -> i32 {
    0
}

/// vkd3d_debug_messenger_callback - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_messenger_callback(message_severity: usize, message_types: usize, callback_data: *mut core::ffi::c_void, userdata: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_init_debug_messenger_callback - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_debug_messenger_callback(instance: *mut core::ffi::c_void) {

}

/// vkd3d_config_flags_init_once - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_config_flags_init_once() {

}

/// vkd3d_config_flags_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_config_flags_init() {

}

/// vkd3d_physical_device_get_time_domains - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_physical_device_get_time_domains(device: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_physical_device_info_init_maint9 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_physical_device_info_init_maint9(info: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_physical_device_info_init_maint10 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_physical_device_info_init_maint10(info: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_physical_device_info_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_physical_device_info_init(info: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_trace_physical_device_properties - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_trace_physical_device_properties(properties: *mut core::ffi::c_void) {

}

/// vkd3d_trace_physical_device - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_trace_physical_device(device: usize, info: *mut core::ffi::c_void, vk_procs: *mut core::ffi::c_void) {

}

/// vkd3d_trace_physical_device_limits - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_trace_physical_device_limits(info: *mut core::ffi::c_void) {

}

/// vkd3d_trace_physical_device_features - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_trace_physical_device_features(info: *mut core::ffi::c_void) {

}

/// vkd3d_init_device_extensions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_device_extensions(device: *mut core::ffi::c_void, create_info: *mut core::ffi::c_void, device_extension_count: *mut u32, user_extension_supported: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_supports_minimum_coopmat_caps - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_supports_minimum_coopmat_caps(device: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_init_device_caps - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_device_caps(device: *mut core::ffi::c_void, create_info: *mut core::ffi::c_void, physical_device_info: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_select_physical_device - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_select_physical_device(instance: *mut core::ffi::c_void, device_index: u32, selected_device: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_destroy_vkd3d_queues - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_destroy_vkd3d_queues(device: *mut core::ffi::c_void) {

}

/// vkd3d_find_queue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_find_queue(count: u32, properties: *mut core::ffi::c_void, mask: usize, flags: usize) -> u32 {
    0
}

/// vkd3d_select_queues - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_select_queues(device: *mut core::ffi::c_void, physical_device: usize, info: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_debug_control_get_behavior_flags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_control_get_behavior_flags() -> usize {
    0
}

/// d3d12_device_vkd3d_ext_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dxvk_interop_device_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// handle_is_kmt_style - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn handle_is_kmt_style(handle: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_init_shader_extensions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_shader_extensions(device: *mut core::ffi::c_void) {

}

/// vkd3d_compute_shader_interface_key - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compute_shader_interface_key(device: *mut core::ffi::c_void) {

}

/// vkd3d_scratch_pool_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_scratch_pool_init(device: *mut core::ffi::c_void) {

}

/// lost - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn lost(device: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// d3d12_device_vkd3d_ext_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_vkd3d_ext_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, out: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_vkd3d_ext_GetVulkanHandles - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_GetVulkanHandles(iface: *mut core::ffi::c_void, vk_instance: *mut core::ffi::c_void, vk_physical_device: *mut core::ffi::c_void, vk_device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_vkd3d_ext_GetExtensionSupport - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_GetExtensionSupport(iface: *mut core::ffi::c_void, extension: usize) -> usize {
    0
}

/// d3d12_device_vkd3d_ext_DestroyCubinComputeShader - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_DestroyCubinComputeShader(iface: *mut core::ffi::c_void, handle: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_vkd3d_ext_GetCudaTextureObject - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_GetCudaTextureObject(iface: *mut core::ffi::c_void, srv_handle: usize, sampler_handle: usize, cuda_texture_handle: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_vkd3d_ext_GetCudaSurfaceObject - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_GetCudaSurfaceObject(iface: *mut core::ffi::c_void, uav_handle: usize, cuda_surface_handle: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_vkd3d_ext_CaptureUAVInfo - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_CaptureUAVInfo(iface: *mut core::ffi::c_void, uav_info: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_vkd3d_ext_SupportsCubin64bit - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_SupportsCubin64bit(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_vkd3d_ext_GetCudaMergedTextureSamplerObject - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_GetCudaMergedTextureSamplerObject(iface: *mut core::ffi::c_void, params: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_vkd3d_ext_GetCudaIndependentDescriptorObject - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_GetCudaIndependentDescriptorObject(iface: *mut core::ffi::c_void, params: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_vkd3d_ext_SupportsAGSExtension - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_SupportsAGSExtension(iface: *mut core::ffi::c_void, agsExtension: usize) -> usize {
    0
}

/// d3d12_device_vkd3d_ext_SetAGSUAVSlot - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_SetAGSUAVSlot(iface: *mut core::ffi::c_void, uavSlot: u32) -> usize {
    0
}

/// d3d12_device_vkd3d_ext_IsNvShaderExtnOpCodeSupported - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_IsNvShaderExtnOpCodeSupported(iface: *mut core::ffi::c_void, op_code: usize) -> usize {
    0
}

/// d3d12_device_vkd3d_ext_SetNvShaderExtnSlotSpace - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_vkd3d_ext_SetNvShaderExtnSlotSpace(iface: *mut core::ffi::c_void, uav_slot: usize, uav_space: usize, local_thread: i32) -> usize {
    0
}

/// d3d12_dxvk_interop_device_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dxvk_interop_device_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, out: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dxvk_interop_device_GetDXGIAdapter - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_GetDXGIAdapter(iface: *mut core::ffi::c_void, iid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dxvk_interop_device_GetVulkanHandles - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_GetVulkanHandles(iface: *mut core::ffi::c_void, vk_instance: *mut core::ffi::c_void, vk_physical_device: *mut core::ffi::c_void, vk_device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dxvk_interop_device_GetDeviceExtensions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_GetDeviceExtensions(iface: *mut core::ffi::c_void, extension_count: *mut u32, extensions: *mut *mut i8) -> usize {
    0
}

/// d3d12_dxvk_interop_device_GetDeviceFeatures - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_GetDeviceFeatures(iface: *mut core::ffi::c_void, features: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dxvk_interop_device_GetVulkanImageLayout - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_GetVulkanImageLayout(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, state: usize, vk_layout: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dxvk_interop_device_GetVulkanResourceInfo1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_GetVulkanResourceInfo1(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, vk_handle: *mut core::ffi::c_void, buffer_offset: *mut core::ffi::c_void, format: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dxvk_interop_device_BeginVkCommandBufferInterop - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_BeginVkCommandBufferInterop(iface: *mut core::ffi::c_void, pCmdList: *mut core::ffi::c_void, cmdBuf: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dxvk_interop_device_EndVkCommandBufferInterop - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_EndVkCommandBufferInterop(iface: *mut core::ffi::c_void, pCmdList: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dxvk_interop_device_GetVulkanHeapInfo - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_GetVulkanHeapInfo(iface: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, vk_memory: *mut core::ffi::c_void, heap_offset: *mut core::ffi::c_void, vk_memory_type: *mut core::ffi::c_void) -> usize {
    0
}

/// preference - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn preference(rebar: usize) -> usize {
    0
}

/// validate_heap_desc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn validate_heap_desc(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_memory_transfer_queue_wait_semaphore - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_transfer_queue_wait_semaphore(queue: *mut core::ffi::c_void, wait_value: u64, timeout: u64) -> usize {
    0
}

/// vkd3d_acquire_tracked_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_acquire_tracked_resource(resource: *mut core::ffi::c_void) {

}

/// vkd3d_release_tracked_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_release_tracked_resource(resource: *mut core::ffi::c_void) {

}

/// vkd3d_select_memory_types - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_select_memory_types(device: *mut core::ffi::c_void, heap_properties: *mut core::ffi::c_void, heap_flags: usize, fallback: usize) -> u32 {
    0
}

/// vkd3d_find_memory_types_with_flags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_find_memory_types_with_flags(device: *mut core::ffi::c_void, type_flags: usize) -> u32 {
    0
}

/// vkd3d_normalize_heap_type - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_normalize_heap_type(heap_properties: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_select_memory_flags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_select_memory_flags(device: *mut core::ffi::c_void, heap_properties: *mut core::ffi::c_void, type_flags: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_report_memory_budget - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_report_memory_budget(device: *mut core::ffi::c_void) {

}

/// vkd3d_memory_info_type_mask_covers_multiple_memory_heaps - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_info_type_mask_covers_multiple_memory_heaps(props: *mut core::ffi::c_void, type_mask: u32) -> usize {
    0
}

/// memory - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn memory(PRIu64: usize, size: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// vkd3d_import_host_memory - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_import_host_memory(device: *mut core::ffi::c_void, host_address: *mut core::ffi::c_void, size: u64, type_flags: usize, type_mask: u32, pNext: *mut core::ffi::c_void, allocation: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_memory_chunk_remove_range - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_chunk_remove_range(chunk: *mut core::ffi::c_void, index: usize) {

}

/// vkd3d_memory_chunk_find_range - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_chunk_find_range(chunk: *mut core::ffi::c_void, offset: u64) -> usize {
    0
}

/// vkd3d_memory_chunk_destroy - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_chunk_destroy(chunk: *mut core::ffi::c_void, device: *mut core::ffi::c_void, allocator: *mut core::ffi::c_void) {

}

/// vkd3d_meta_make_shader_stage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_make_shader_stage(info: *mut core::ffi::c_void, stage: usize, module: usize, entry_point: *mut i8, spec_info: *mut core::ffi::c_void) {

}

/// vkd3d_clear_uav_ops_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_clear_uav_ops_cleanup(meta_clear_uav_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_clear_uav_ops_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_clear_uav_ops_init(meta_clear_uav_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_copy_image_ops_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_copy_image_ops_cleanup(meta_copy_image_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_copy_image_ops_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_copy_image_ops_init(meta_copy_image_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_resolve_image_ops_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_resolve_image_ops_cleanup(meta_resolve_image_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_resolve_image_ops_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_resolve_image_ops_init(meta_resolve_image_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_ops_common_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_ops_common_init(meta_ops_common: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_meta_ops_common_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_meta_ops_common_cleanup(meta_ops_common: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_swapchain_ops_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_swapchain_ops_cleanup(meta_swapchain_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_swapchain_ops_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_swapchain_ops_init(meta_swapchain_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_query_ops_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_query_ops_cleanup(meta_query_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_query_ops_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_query_ops_init(meta_query_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_multi_dispatch_indirect_ops_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_multi_dispatch_indirect_ops_cleanup(meta_multi_dispatch_indirect_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_multi_dispatch_indirect_ops_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_multi_dispatch_indirect_ops_init(meta_multi_dispatch_indirect_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_predicate_ops_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_predicate_ops_cleanup(meta_predicate_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_predicate_ops_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_predicate_ops_init(meta_predicate_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_execute_indirect_ops_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_execute_indirect_ops_init(meta_indirect_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_execute_indirect_ops_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_execute_indirect_ops_cleanup(meta_indirect_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_dstorage_ops_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_dstorage_ops_init(dstorage_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_dstorage_ops_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_dstorage_ops_cleanup(dstorage_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_sampler_feedback_ops_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_sampler_feedback_ops_init(sampler_feedback_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_workgraph_ops_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_workgraph_ops_init(workgraph_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_sampler_feedback_ops_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_sampler_feedback_ops_cleanup(sampler_feedback_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_workgraph_ops_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_workgraph_ops_cleanup(workgraph_ops: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_opacity_micromap_end_barrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_opacity_micromap_end_barrier(list: *mut core::ffi::c_void) {

}

/// vkd3d_queue_timeline_trace_register_generic_op - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_generic_op(trace: *mut core::ffi::c_void, arg1: usize, tag: *mut i8) -> usize {
    0
}

/// SLEEP - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn SLEEP(PRIu64: usize) -> usize {
    0
}

/// get_shader_stack_size - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_shader_stack_size(object: *mut core::ffi::c_void, pipeline_variant_index: u32, index: u32, shader: usize) -> u64 {
    0
}

/// vkd3d_renderdoc_init_capture_count_list - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_renderdoc_init_capture_count_list(env: *mut i8) {

}

/// vkd3d_renderdoc_enable_submit_counter - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_renderdoc_enable_submit_counter(counter: u32) -> usize {
    0
}

/// vkd3d_renderdoc_init_once - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_renderdoc_init_once() {

}

/// vk_image_type_from_d3d12_resource_dimension - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_image_type_from_d3d12_resource_dimension(dimension: usize) -> usize {
    0
}

/// adjust_sparse_buffer_size - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn adjust_sparse_buffer_size(size: u64) -> u64 {
    0
}

/// max_miplevel_count - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn max_miplevel_count(desc: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_get_castable_format_compatibility_list - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_castable_format_compatibility_list(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, num_castable_formats: u32, castable_formats: *mut core::ffi::c_void, list: *mut core::ffi::c_void, vk_flags: *mut core::ffi::c_void) -> usize {
    0
}

/// vk_common_image_layout_from_d3d12_desc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_common_image_layout_from_d3d12_desc(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_sparse_image_may_have_mip_tail - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_sparse_image_may_have_mip_tail(desc: *mut core::ffi::c_void, format: *mut core::ffi::c_void, sparse_info: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_resource_make_vrs_view - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_resource_make_vrs_view(device: *mut core::ffi::c_void, image: usize, view: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_format_allows_shader_copies - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_format_allows_shader_copies(dxgi_format: usize) -> usize {
    0
}

/// vkd3d_format_allows_sampler_feedback_resolve - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_format_allows_sampler_feedback_resolve(dxgi_format: usize) -> usize {
    0
}

/// vkd3d_format_needs_extended_usage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_format_needs_extended_usage(format: *mut core::ffi::c_void, usage: usize) -> usize {
    0
}

/// vkd3d_compute_resource_layouts_from_desc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compute_resource_layouts_from_desc(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, layouts: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_view_entry_hash - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_view_entry_hash(key: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_view_entry_compare - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_view_entry_compare(key: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_view_destroy - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_view_destroy(view: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_view_tag_debug_name - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_view_tag_debug_name(view: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_sampler_entry_hash - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_sampler_entry_hash(key: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_sampler_entry_compare - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_sampler_entry_compare(key: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) -> usize {
    0
}

/// bitmask - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn bitmask(COMMON: usize) -> usize {
    0
}

/// Texture - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn Texture(arg0: usize) -> usize {
    0
}

/// possible - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn possible(u: usize) -> usize {
    0
}

/// vkd3d_get_metadata_buffer_view_for_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_metadata_buffer_view_for_resource(device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, view_format: usize, offset: u64, size: u64, structure_stride: u64, view: *mut core::ffi::c_void) {

}

/// vkd3d_structured_srv_to_texel_buffer_dxgi_format - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_structured_srv_to_texel_buffer_dxgi_format(stride: u32) -> usize {
    0
}

/// vkd3d_structured_uav_to_texel_buffer_dxgi_format - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_structured_uav_to_texel_buffer_dxgi_format(stride: u32) -> usize {
    0
}

/// vkd3d_set_view_swizzle_for_format - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_set_view_swizzle_for_format(components: *mut core::ffi::c_void, format: *mut core::ffi::c_void, allowed_swizzle: usize) {

}

/// vk_component_mapping_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_component_mapping_from_d3d12(components: *mut core::ffi::c_void, component_mapping: u32) {

}

/// swizzle_vk_component - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn swizzle_vk_component(components: *mut core::ffi::c_void, component: usize, swizzle: usize) -> usize {
    0
}

/// vk_component_mapping_compose - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_component_mapping_compose(dst: *mut core::ffi::c_void, b: *mut core::ffi::c_void) {

}

/// init_default_texture_view_desc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn init_default_texture_view_desc(desc: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, view_format: usize) -> usize {
    0
}

/// vkd3d_view_flags_from_d3d12_buffer_srv_flags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_view_flags_from_d3d12_buffer_srv_flags(flags: usize) -> u32 {
    0
}

/// vkd3d_buffer_view_get_bound_range_ssbo - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_buffer_view_get_bound_range_ssbo(device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, offset: u64, range: u64, vk_buffer: *mut core::ffi::c_void, bound_range: *mut core::ffi::c_void) {

}

/// vkd3d_buffer_view_get_aligned_view - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_buffer_view_get_aligned_view(device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, format: usize, vk_flags: u32, first_element: u64, num_elements: u64, structured_stride: u64, bound_range: *mut core::ffi::c_void, view: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_view_flags_from_d3d12_buffer_uav_flags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_view_flags_from_d3d12_buffer_uav_flags(flags: usize) -> u32 {
    0
}

/// vk_filter_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_filter_from_d3d12(arg0: usize) -> usize {
    0
}

/// vk_mipmap_mode_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_mipmap_mode_from_d3d12(arg0: usize) -> usize {
    0
}

/// vk_address_mode_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_address_mode_from_d3d12(mode: usize) -> usize {
    0
}

/// vk_border_color_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_border_color_from_d3d12(device: *mut core::ffi::c_void, border_color: *mut u32, flags: usize) -> usize {
    0
}

/// TRACE - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn TRACE(descriptorsn: usize, arg1: usize) -> usize {
    0
}

/// vkd3d_memory_info_decide_hvv_usage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_info_decide_hvv_usage(topology: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_memory_info_decide_gpu_upload_heap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_info_decide_gpu_upload_heap(is_hvv_use_allowed: usize) -> usize {
    0
}

/// vkd3d_memory_info_init_budgets - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_info_init_budgets(info: *mut core::ffi::c_void, topology: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_memory_info_filter_sysmem_memory_types - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_info_filter_sysmem_memory_types(device: *mut core::ffi::c_void, info: *mut core::ffi::c_void, type_mask: u32) -> u32 {
    0
}

/// vk_descriptor_type_from_d3d12_root_parameter - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_descriptor_type_from_d3d12_root_parameter(device: *mut core::ffi::c_void, arg1: usize) -> usize {
    0
}

/// vkd3d_descriptor_type_from_d3d12_range_type - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_type_from_d3d12_range_type(arg0: usize) -> usize {
    0
}

/// vkd3d_descriptor_type_from_d3d12_root_parameter_type - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_type_from_d3d12_root_parameter_type(arg0: usize) -> usize {
    0
}

/// vkd3d_shader_resource_binding_init_global_heap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_resource_binding_init_global_heap(binding: *mut core::ffi::c_void, range_type: usize) {

}

/// vkd3d_pipeline_state_desc_fixup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_state_desc_fixup(desc: *mut core::ffi::c_void) {

}

/// vkd3d_pipeline_state_desc_get_shader_stages - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_pipeline_state_desc_get_shader_stages(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_load_spirv_from_cached_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_load_spirv_from_cached_state(device: *mut core::ffi::c_void, cached_state: *mut core::ffi::c_void, stage: usize, spirv_code: *mut core::ffi::c_void, identifier: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_setup_shader_stage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_setup_shader_stage(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void, stage_desc: *mut core::ffi::c_void, stage: usize, required_subgroup_size_info: *mut core::ffi::c_void, identifier_create_info: *mut core::ffi::c_void, spirv_code: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_compile_shader_stage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_compile_shader_stage(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void, stage: usize, code: *mut core::ffi::c_void, spirv_code: *mut core::ffi::c_void, spirv_code_debug: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_code_init_empty_fs - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_code_init_empty_fs(code: *mut core::ffi::c_void) {

}

/// vkd3d_late_compile_shader_stages - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_late_compile_shader_stages(state: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_report_pipeline_creation_feedback_results - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_report_pipeline_creation_feedback_results(feedback: *mut core::ffi::c_void) {

}

/// vk_polygon_mode_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_polygon_mode_from_d3d12(mode: usize) -> usize {
    0
}

/// vk_cull_mode_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_cull_mode_from_d3d12(mode: usize) -> usize {
    0
}

/// FIXME_ONCE - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn FIXME_ONCE(supported: usize, linesn: usize) -> usize {
    0
}

/// vk_stencil_op_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_stencil_op_from_d3d12(op: usize) -> usize {
    0
}

/// vk_stencil_op_state_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_stencil_op_state_from_d3d12(vk_desc: *mut core::ffi::c_void, d3d12_desc: *mut core::ffi::c_void) {

}

/// vk_blend_factor_from_d3d12_a8 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_blend_factor_from_d3d12_a8(blend: usize) -> usize {
    0
}

/// vk_blend_factor_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_blend_factor_from_d3d12(blend: usize) -> usize {
    0
}

/// vk_blend_op_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_blend_op_from_d3d12(op: usize) -> usize {
    0
}

/// compute_input_layout_offsets - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn compute_input_layout_offsets(device: *mut core::ffi::c_void, input_layout_desc: *mut core::ffi::c_void, offsets: *mut u32) -> i32 {
    0
}

/// VKD3D_SWIZZLE - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn VKD3D_SWIZZLE(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// vkd3d_init_dynamic_state_array - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_dynamic_state_array(dynamic_states: *mut core::ffi::c_void, dynamic_state_flags: u32) -> u32 {
    0
}

/// vkd3d_vertex_input_pipeline_desc_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_vertex_input_pipeline_desc_init(desc: *mut core::ffi::c_void, state: *mut core::ffi::c_void, key: *mut core::ffi::c_void, dynamic_state_flags: u32) {

}

/// vkd3d_view_mask_to_multiview_mask - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_view_mask_to_multiview_mask(graphics: *mut core::ffi::c_void, view_mask: u32) -> u32 {
    0
}

/// vkd3d_fragment_output_pipeline_desc_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_fragment_output_pipeline_desc_init(desc: *mut core::ffi::c_void, state: *mut core::ffi::c_void, dsv_format: *mut core::ffi::c_void, dynamic_view_mask: u32, dynamic_state_flags: u32) {

}

/// vkd3d_fragment_output_pipeline_desc_prepare - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_fragment_output_pipeline_desc_prepare(desc: *mut core::ffi::c_void) {

}

/// vkd3d_validate_shader_io_signatures - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_validate_shader_io_signatures(output_stage: usize, out_sig: *mut core::ffi::c_void, input_stage: usize, in_sig: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_validate_mesh_shader_io_signatures - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_validate_mesh_shader_io_signatures(vert_sig: *mut core::ffi::c_void, prim_sig: *mut core::ffi::c_void, in_sig: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_validate_vertex_input_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_validate_vertex_input_signature(sig: *mut core::ffi::c_void, input_layout: *mut core::ffi::c_void) -> usize {
    0
}

/// failure - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn failure(x: usize) -> usize {
    0
}

/// vkd3d_bindless_build_mutable_type_list - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bindless_build_mutable_type_list(list: *mut core::ffi::c_void, bindless_flags: u32, set_flags: u32) -> u32 {
    0
}

/// vkd3d_bindless_find_copy_template - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bindless_find_copy_template(descriptor_size: u32) -> usize {
    0
}

/// vkd3d_get_descriptor_size_for_binding - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_descriptor_size_for_binding(device: *mut core::ffi::c_void, set_layout_info: *mut core::ffi::c_void, binding_index: u32) -> u32 {
    0
}

/// vkd3d_bindless_supports_mutable_type - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bindless_supports_mutable_type(device: *mut core::ffi::c_void, bindless_flags: u32) -> usize {
    0
}

/// vkd3d_bindless_state_get_bindless_flags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bindless_state_get_bindless_flags(device: *mut core::ffi::c_void) -> u32 {
    0
}

/// VK_EXT_mutable_descriptor_type - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn VK_EXT_mutable_descriptor_type(VALVE: usize) -> usize {
    0
}

/// vkd3d_bindless_state_get_extra_binding_index - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bindless_state_get_extra_binding_index(extra_flag: u32, set_flags: u32) -> u32 {
    0
}

/// vkd3d_swapchain_present_mode_parse - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_swapchain_present_mode_parse(string: *mut i8, present_mode: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_factory_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_factory_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_factory_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_factory_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_factory_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_factory_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_drain_internal_blit_semaphore - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_drain_internal_blit_semaphore(chain: *mut core::ffi::c_void, value: u64) {

}

/// dxgi_vk_swap_chain_wait_acquire_semaphore - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_wait_acquire_semaphore(chain: *mut core::ffi::c_void, vk_semaphore: usize, blocking: usize) {

}

/// dxgi_vk_swap_chain_ensure_unsignaled_swapchain_fence - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_ensure_unsignaled_swapchain_fence(chain: *mut core::ffi::c_void, index: u32) {

}

/// dxgi_vk_swap_chain_drain_swapchain_fences - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_drain_swapchain_fences(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_wait_semaphore - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_wait_semaphore(chain: *mut core::ffi::c_void, vk_timeline: usize, value: u64) {

}

/// dxgi_vk_swap_chain_drain_complete_semaphore - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_drain_complete_semaphore(chain: *mut core::ffi::c_void, value: u64) {

}

/// dxgi_vk_swap_chain_drain_user_images - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_drain_user_images(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_drain_queue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_drain_queue(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_push_present_id - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_push_present_id(chain: *mut core::ffi::c_void, present_count: u64, present_id: u64, begin_frame_time_ns: u64, present_timing_enabled: usize) {

}

/// dxgi_vk_swap_chain_cleanup_frame_rate_limiter - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_cleanup_frame_rate_limiter(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_cleanup_low_latency - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_cleanup_low_latency(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_cleanup_surface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_cleanup_surface(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_cleanup_sync_objects - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_cleanup_sync_objects(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_cleanup_common - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_cleanup_common(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_cleanup(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_GetDesc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_GetDesc(iface: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_GetAdapter - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_GetAdapter(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_GetDevice(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_GetImage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_GetImage(iface: *mut core::ffi::c_void, BufferId: u32, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_GetImageIndex - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_GetImageIndex(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_GetFrameLatency - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_GetFrameLatency(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_GetFrameLatencyEvent - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_GetFrameLatencyEvent(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_ChangeProperties - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_ChangeProperties(iface: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, pNodeMasks: *mut u32, ppPresentQueues: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// swapchain - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn swapchain(u: usize) -> usize {
    0
}

/// dxgi_vk_swap_chain_SetPresentRegion - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_SetPresentRegion(iface: *mut core::ffi::c_void, pRegion: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_SetFrameLatency - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_SetFrameLatency(iface: *mut core::ffi::c_void, MaxLatency: u32) -> usize {
    0
}

/// convert_xy_color - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn convert_xy_color(dxgi_color: *mut core::ffi::c_void) -> usize {
    0
}

/// convert_max_luminance - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn convert_max_luminance(dxgi_luminance: u32) -> f32 {
    0.0
}

/// convert_min_luminance - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn convert_min_luminance(dxgi_luminance: u32) -> f32 {
    0.0
}

/// convert_level - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn convert_level(dxgi_level: usize) -> f32 {
    0.0
}

/// convert_hdr_metadata_hdr10 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn convert_hdr_metadata_hdr10(dxgi_metadata: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_set_hdr_metadata - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_set_hdr_metadata(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_is_occluded - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_is_occluded(chain: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_present_callback - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_present_callback(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_wait_internal_handle - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_wait_internal_handle(chain: *mut core::ffi::c_void, low_latency_enable: usize) {

}

/// dxgi_vk_swap_chain_Present - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_Present(iface: *mut core::ffi::c_void, SyncInterval: u32, PresentFlags: u32, pPresentParameters: *mut core::ffi::c_void) -> usize {
    0
}

/// convert_color_space - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn convert_color_space(dxgi_color_space: usize) -> usize {
    0
}

/// dxgi_vk_swap_chain_supports_color_space - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_supports_color_space(chain: *mut core::ffi::c_void, ColorSpace: usize) -> usize {
    0
}

/// dxgi_vk_swap_chain_CheckColorSpaceSupport - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_CheckColorSpaceSupport(iface: *mut core::ffi::c_void, ColorSpace: usize) -> usize {
    0
}

/// dxgi_vk_swap_chain_SetColorSpace - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_SetColorSpace(iface: *mut core::ffi::c_void, ColorSpace: usize) -> usize {
    0
}

/// dxgi_vk_swap_chain_SetHDRMetaData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_SetHDRMetaData(iface: *mut core::ffi::c_void, pMetaData: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_GetLastPresentCount - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_GetLastPresentCount(iface: *mut core::ffi::c_void, present_count: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_GetFrameStatistics - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_GetFrameStatistics(iface: *mut core::ffi::c_void, frame_statistics: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_SetTargetFrameRate - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_SetTargetFrameRate(iface: *mut core::ffi::c_void, frame_rate: f64) -> usize {
    0
}

/// dxgi_vk_swap_chain_update_wait_timing_capabilities - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_update_wait_timing_capabilities(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_init_sync_objects - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_init_sync_objects(chain: *mut core::ffi::c_void) -> i32 {
    0
}

/// dxgi_vk_swap_chain_drain_waiter - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_drain_waiter(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_destroy_swapchain_in_present_task - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_destroy_swapchain_in_present_task(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_find_surface_format - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_find_surface_format(chain: *mut core::ffi::c_void, vk_format: usize, color_space: usize, format: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_check_present_mode_support - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_check_present_mode_support(chain: *mut core::ffi::c_void, present_mode: usize) -> usize {
    0
}

/// dxgi_vk_swap_chain_init_blit_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_init_blit_pipeline(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_anti_lag_state_update - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_anti_lag_state_update(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_poll_time_properties - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_poll_time_properties(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_poll_time_domains - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_poll_time_domains(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_poll_calibration - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_poll_calibration(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_present_signal_blit_semaphore - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_present_signal_blit_semaphore(chain: *mut core::ffi::c_void, present_count: u64) {

}

/// dxgi_vk_swap_chain_record_render_pass - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_record_render_pass(chain: *mut core::ffi::c_void, vk_cmd: usize, swapchain_index: u32) {

}

/// dxgi_vk_swap_chain_submit_blit - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_submit_blit(chain: *mut core::ffi::c_void, swapchain_index: u32) -> usize {
    0
}

/// dxgi_vk_swap_chain_ensure_unsignaled_acquire_semaphore - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_ensure_unsignaled_acquire_semaphore(chain: *mut core::ffi::c_void, index: u32, blocking: usize) -> i32 {
    0
}

/// dxgi_vk_swap_chain_try_acquire_next_image - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_try_acquire_next_image(chain: *mut core::ffi::c_void) -> i32 {
    0
}

/// dxgi_vk_swap_chain_setup_present_timing_request - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_setup_present_timing_request(chain: *mut core::ffi::c_void, present_count: u64, timing_info: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_present_iteration - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_present_iteration(chain: *mut core::ffi::c_void, present_count: u64, retry_counter: u32) {

}

/// dxgi_vk_swap_chain_signal_waitable_handle - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_signal_waitable_handle(chain: *mut core::ffi::c_void, present_count: u64) {

}

/// dxgi_vk_swap_chain_delay_next_frame - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_delay_next_frame(chain: *mut core::ffi::c_void, current_time_ns: u64) {

}

/// dxgi_vk_swap_chain_update_present_timing - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_update_present_timing(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_update_past_presentation - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_update_past_presentation(chain: *mut core::ffi::c_void, present_id: u64, time: u64, time_domain: usize, time_domain_id: u64, time_domain_counter: u64) {

}

/// dxgi_vk_swap_chain_poll_past_presentation - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_poll_past_presentation(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_update_frame_statistics - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_update_frame_statistics(chain: *mut core::ffi::c_void, present_count: u64, present_id: u64) {

}

/// dxgi_vk_swap_chain_platform_sleep_for_ns - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_platform_sleep_for_ns(sleep_state: *mut core::ffi::c_void, duration_ns: u64) {

}

/// dxgi_vk_swap_chain_init_low_latency - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_init_low_latency(chain: *mut core::ffi::c_void) -> i32 {
    0
}

/// dxgi_vk_swap_chain_init_sleep_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_init_sleep_state(sleep_state: *mut core::ffi::c_void) -> i32 {
    0
}

/// dxgi_vk_swap_chain_init_frame_rate_limiter - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_init_frame_rate_limiter(chain: *mut core::ffi::c_void) -> i32 {
    0
}

/// dxgi_vk_swap_chain_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_init(chain: *mut core::ffi::c_void, pFactory: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, queue: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_timestamp_profiler_decref_timestamp_index - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_decref_timestamp_index(profiler: *mut core::ffi::c_void, timestamp_index: u32) {

}

/// vkd3d_timestamp_profiler_resolve_timestamp - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_resolve_timestamp(profiler: *mut core::ffi::c_void, work: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_timestamp_profiler_flush - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_flush(profiler: *mut core::ffi::c_void) {

}

/// Time - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn Time(arg0: usize) -> usize {
    0
}

/// vkd3d_timestamp_profiler_flush_active_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_flush_active_state(profiler: *mut core::ffi::c_void, list: *mut core::ffi::c_void) {

}

/// vkd3d_timestamp_profiler_incref_timestamp_index - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_timestamp_profiler_incref_timestamp_index(profiler: *mut core::ffi::c_void, timestamp_index: u32) {

}

/// vkd3d_init_format_compatibility_lists - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_format_compatibility_lists(device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_cleanup_format_compatibility_lists - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_cleanup_format_compatibility_lists(device: *mut core::ffi::c_void) {

}

/// vkd3d_get_vk_format_properties - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_vk_format_properties(device: *mut core::ffi::c_void, vk_format: usize, properties3: *mut core::ffi::c_void) {

}

/// vkd3d_init_format_sample_counts - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_format_sample_counts(device: *mut core::ffi::c_void, format: *mut core::ffi::c_void) {

}

/// vkd3d_init_depth_stencil_formats - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_depth_stencil_formats(device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_cleanup_depth_stencil_formats - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_cleanup_depth_stencil_formats(device: *mut core::ffi::c_void) {

}

/// vkd3d_init_formats - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_formats(device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_cleanup_formats - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_cleanup_formats(device: *mut core::ffi::c_void) {

}

/// vkd3d_lerp_u32_to_float - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_lerp_u32_to_float(uval: u32, ustart: u32, uend: u32, fstart: f32, fend: f32) -> f32 {
    0.0
}

/// flag - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn flag(arg0: usize) -> usize {
    0
}

/// d3d_blob_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d_blob_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d_blob_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d_blob_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d_blob_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d_blob_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d_blob_GetBufferPointer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d_blob_GetBufferPointer(iface: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d_blob_GetBufferSize - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d_blob_GetBufferSize(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d_blob_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d_blob_init(blob: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, size: usize) {

}

/// d3d_destruction_notifier_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d_destruction_notifier_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d_destruction_notifier_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d_destruction_notifier_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d_destruction_notifier_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d_destruction_notifier_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d_destruction_notifier_RegisterDestructionCallback - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d_destruction_notifier_RegisterDestructionCallback(iface: *mut core::ffi::c_void, callback: usize, data: *mut core::ffi::c_void, callback_id: *mut u32) -> usize {
    0
}

/// d3d_destruction_notifier_UnregisterDestructionCallback - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d_destruction_notifier_UnregisterDestructionCallback(iface: *mut core::ffi::c_void, callback_id: u32) -> usize {
    0
}

/// vkd3d_va_map_get_next_address - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_va_map_get_next_address(va: usize) -> usize {
    0
}

/// vkd3d_va_map_cleanup_tree - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_va_map_cleanup_tree(tree: *mut core::ffi::c_void) {

}

/// vkd3d_va_map_deref_mutable - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_va_map_deref_mutable(arg0: usize, arg1: usize) -> usize {
    0
}

/// vkd3d_va_map_try_place_rtas - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_va_map_try_place_rtas(va_map: *mut core::ffi::c_void, device: *mut core::ffi::c_void, va: usize, rtas_is_omm: usize, acceleration_structure: *mut core::ffi::c_void, micromap: *mut core::ffi::c_void) {

}

/// vkd3d_root_signature_version_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_root_signature_version_from_d3d12(version: usize) -> usize {
    0
}

/// entry_point_compare_func - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn entry_point_compare_func(a_: *mut core::ffi::c_void, b_: *mut core::ffi::c_void) -> i32 {
    0
}

/// enough - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn enough(PRIu64n: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// vkd3d_dbg_init_once - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_dbg_init_once() {

}

/// get_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_buffer() -> usize {
    0
}

/// is_option_separator - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn is_option_separator(c: i8) -> usize {
    0
}

/// vkd3d_parse_linux_release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_parse_linux_release(release: *mut i8, major: *mut u32, minor: *mut u32, patch: *mut u32) -> usize {
    0
}

/// dlerror - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dlerror() -> usize {
    0
}

/// vkd3d_init_profiling_path - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_profiling_path(path: *mut i8) {

}

/// vkd3d_init_profiling_once - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_profiling_once() {

}

/// vkd3d_dup_entry_point_n - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_dup_entry_point_n(arg0: usize, arg1: usize) -> usize {
    0
}

/// vkd3d_utf8_len - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_utf8_len(c: u32) -> usize {
    0
}

/// vkd3d_utf8_append - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_utf8_append(dst: *mut *mut i8, c: u32) {

}

/// vkd3d_string_should_loop_u16 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_string_should_loop_u16(max_elements: isize, src: *mut u16, wstr: *mut u16) -> usize {
    0
}

/// require_space - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn require_space(offset: usize, count: usize, size: usize, data_size: usize) -> usize {
    0
}

/// skip_dword_unknown - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn skip_dword_unknown(ptr: *mut *mut i8, count: u32) {

}

/// parse_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn parse_dxbc(data: *mut i8, data_size: usize, chunk_handler: i32) -> i32 {
    0
}

/// shader_parse_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_parse_signature(tag: u32, data: *mut i8, data_size: u32, s: *mut core::ffi::c_void) -> i32 {
    0
}

/// isgn_handler - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn isgn_handler(data: *mut i8, data_size: u32, tag: u32, ctx: *mut core::ffi::c_void) -> i32 {
    0
}

/// osgn_handler - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn osgn_handler(data: *mut i8, data_size: u32, tag: u32, ctx: *mut core::ffi::c_void) -> i32 {
    0
}

/// psgn_handler - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn psgn_handler(data: *mut i8, data_size: u32, tag: u32, ctx: *mut core::ffi::c_void) -> i32 {
    0
}

/// dxil_handler - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxil_handler(data: *mut i8, data_size: u32, tag: u32, context: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_parse_descriptor_ranges - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_parse_descriptor_ranges(context: *mut core::ffi::c_void, offset: u32, count: u32, ranges: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_parse_descriptor_ranges1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_parse_descriptor_ranges1(context: *mut core::ffi::c_void, offset: u32, count: u32, ranges: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_parse_descriptor_table - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_parse_descriptor_table(context: *mut core::ffi::c_void, offset: u32, table: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_parse_descriptor_table1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_parse_descriptor_table1(context: *mut core::ffi::c_void, offset: u32, table: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_parse_root_descriptor - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_parse_root_descriptor(context: *mut core::ffi::c_void, offset: u32, descriptor: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_parse_root_descriptor1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_parse_root_descriptor1(context: *mut core::ffi::c_void, offset: u32, descriptor: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_parse_root_parameters - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_parse_root_parameters(context: *mut core::ffi::c_void, offset: u32, count: u32, parameters: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_parse_root_parameters1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_parse_root_parameters1(context: *mut core::ffi::c_void, offset: u32, count: u32, parameters: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_parse_static_sampler_payload_base - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_parse_static_sampler_payload_base(ptr: *mut *mut i8, sampler_desc: *mut core::ffi::c_void) {

}

/// shader_parse_static_samplers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_parse_static_samplers(context: *mut core::ffi::c_void, offset: u32, count: u32, sampler_descs: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_parse_static_samplers1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_parse_static_samplers1(context: *mut core::ffi::c_void, offset: u32, count: u32, sampler_descs: *mut core::ffi::c_void) -> i32 {
    0
}

/// rts0_handler - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rts0_handler(data: *mut i8, data_size: u32, tag: u32, context: *mut core::ffi::c_void) -> i32 {
    0
}

/// versioned_root_signature_get_parameter_count - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn versioned_root_signature_get_parameter_count(desc: *mut core::ffi::c_void) -> u32 {
    0
}

/// versioned_root_signature_get_static_sampler_count - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn versioned_root_signature_get_static_sampler_count(desc: *mut core::ffi::c_void) -> u32 {
    0
}

/// versioned_root_signature_get_flags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn versioned_root_signature_get_flags(desc: *mut core::ffi::c_void) -> u32 {
    0
}

/// get_chunk_offset - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_chunk_offset(context: *mut core::ffi::c_void) -> usize {
    0
}

/// validate_descriptor_table_v_1_0 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn validate_descriptor_table_v_1_0(descriptor_table: *mut core::ffi::c_void) -> i32 {
    0
}

/// validate_descriptor_table_v_1_1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn validate_descriptor_table_v_1_1(descriptor_table: *mut core::ffi::c_void) -> i32 {
    0
}

/// validate_root_signature_desc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn validate_root_signature_desc(desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// convert_root_parameters_to_v_1_0 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn convert_root_parameters_to_v_1_0(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, count: u32) -> i32 {
    0
}

/// convert_static_sampler_to_v_1_0 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn convert_static_sampler_to_v_1_0(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void) {

}

/// convert_static_sampler_to_v_1_2 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn convert_static_sampler_to_v_1_2(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void) {

}

/// convert_root_signature_to_v1_0 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn convert_root_signature_to_v1_0(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void) -> i32 {
    0
}

/// dup_root_parameters_v_1_1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dup_root_parameters_v_1_1(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, count: u32) -> i32 {
    0
}

/// convert_root_parameters_to_v_1_1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn convert_root_parameters_to_v_1_1(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, count: u32) -> i32 {
    0
}

/// convert_root_signature_to_v1_1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn convert_root_signature_to_v1_1(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void) -> i32 {
    0
}

/// convert_root_signature_to_v1_2 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn convert_root_signature_to_v1_2(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void) -> i32 {
    0
}

/// dxil_match_shader_visibility - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxil_match_shader_visibility(visibility: usize, stage: usize) -> usize {
    0
}

/// dxil_resource_is_in_range - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxil_resource_is_in_range(binding: *mut core::ffi::c_void, d3d_binding: *mut core::ffi::c_void) -> usize {
    0
}

/// dxil_remap_inner - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxil_remap_inner(remap: *mut core::ffi::c_void, descriptor_type: usize, d3d_binding: *mut core::ffi::c_void, vk_binding: *mut core::ffi::c_void, resource_flags: u32) -> usize {
    0
}

/// dxil_remap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxil_remap(remap: *mut core::ffi::c_void, descriptor_type: usize, d3d_binding: *mut core::ffi::c_void, vk_binding: *mut core::ffi::c_void, resource_flags: u32) -> usize {
    0
}

/// dxil_srv_remap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxil_srv_remap(userdata: *mut core::ffi::c_void, d3d_binding: *mut core::ffi::c_void, vk_binding: *mut core::ffi::c_void) -> usize {
    0
}

/// dxil_sampler_remap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxil_sampler_remap(userdata: *mut core::ffi::c_void, d3d_binding: *mut core::ffi::c_void, vk_binding: *mut core::ffi::c_void) -> usize {
    0
}

/// dxil_input_remap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxil_input_remap(userdata: *mut core::ffi::c_void, d3d_input: *mut core::ffi::c_void, vk_input: *mut core::ffi::c_void) -> usize {
    0
}

/// dxil_output_remap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxil_output_remap(userdata: *mut core::ffi::c_void, d3d_output: *mut core::ffi::c_void, vk_output: *mut core::ffi::c_void) -> usize {
    0
}

/// dxil_shader_stage_output_capture - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxil_shader_stage_output_capture(userdata: *mut core::ffi::c_void, d3d_input: *mut core::ffi::c_void, vk_input: *mut core::ffi::c_void) -> usize {
    0
}

/// dxil_shader_stage_input_remap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxil_shader_stage_input_remap(userdata: *mut core::ffi::c_void, d3d_input: *mut core::ffi::c_void, vk_input: *mut core::ffi::c_void) -> usize {
    0
}

/// dxil_uav_remap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxil_uav_remap(userdata: *mut core::ffi::c_void, d3d_binding: *mut core::ffi::c_void, vk_binding: *mut core::ffi::c_void) -> usize {
    0
}

/// dxil_cbv_remap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxil_cbv_remap(userdata: *mut core::ffi::c_void, d3d_binding: *mut core::ffi::c_void, vk_binding: *mut core::ffi::c_void) -> usize {
    0
}

/// dxil_match_shader_stage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxil_match_shader_stage(blob_stage: usize, expected: usize) -> usize {
    0
}

/// vkd3d_dxil_converter_set_quirks - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_dxil_converter_set_quirks(converter: usize, shader_interface_info: *mut core::ffi::c_void, quirks: usize) -> usize {
    0
}

/// convert_stage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn convert_stage(stage: usize) -> usize {
    0
}

/// vkd3d_shader_dxil_dup_node_input - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_dxil_dup_node_input(entry: *mut core::ffi::c_void, node_input: *mut core::ffi::c_void) {

}

/// vkd3d_shader_dxil_dup_node_output - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_dxil_dup_node_output(entry: *mut core::ffi::c_void, node_output: *mut core::ffi::c_void) {

}

/// vkd3d_dxil_build_entry - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_dxil_build_entry(entry: *mut core::ffi::c_void, identifier: u32, mangled_name: *mut i8, demangled_name: *mut i8, stage: usize) -> usize {
    0
}

/// vkd3d_shader_dxil_copy_subobject - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_dxil_copy_subobject(identifier: u32, subobject: *mut core::ffi::c_void, dxil_subobject: *mut core::ffi::c_void) {

}

/// vkd3d_shader_dump_blob - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_dump_blob(path: *mut i8, hash: usize, data: *mut core::ffi::c_void, size: usize, ext: *mut i8) {

}

/// vkd3d_shader_replace_path - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_replace_path(filename: *mut i8, hash: usize, data: *mut *mut core::ffi::c_void, size: *mut usize) -> usize {
    0
}

/// vkd3d_shader_validate_compile_args - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_validate_compile_args(compile_args: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_init_quirk_table - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_init_quirk_table() {

}

/// vkd3d_shader_parse_root_signature_for_version - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_parse_root_signature_for_version(dxbc: *mut core::ffi::c_void, out_desc: *mut core::ffi::c_void, target_version: usize, raw_payload: usize, compatibility_hash: *mut core::ffi::c_void) -> i32 {
    0
}

/// print_usage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn print_usage(program_name: *mut i8) {

}

/// parse_command_line - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn parse_command_line(argc: i32, argv: *mut *mut i8, options: *mut core::ffi::c_void) -> usize {
    0
}

/// dump_root_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dump_root_signature(rs: *mut core::ffi::c_void) {

}

