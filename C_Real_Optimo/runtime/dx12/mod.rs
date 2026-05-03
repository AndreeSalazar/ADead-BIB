//! ADead Runtime - DX12 Module
//!
//! Funciones generadas automáticamente desde knowledge.json
//! Categoría: dx12

#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

/// MFMapDX9FormatToDXGIFormat - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFMapDX9FormatToDXGIFormat(format: u32) -> usize {
    0
}

/// MFMapDXGIFormatToDX9Format - from wine/mfapi.h
#[no_mangle]
pub unsafe extern "C" fn MFMapDXGIFormatToDX9Format(dxgi_format: usize) -> u32 {
    0
}

/// GetDevice - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetDevice(riid: usize, ppvDevice: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetPrivateData - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetPrivateData(guid: usize, pDataSize: *mut core::ffi::c_void, pDataSize_2: *mut core::ffi::c_void) -> usize {
    0
}

/// SetPrivateData - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetPrivateData(guid: usize, DataSize: usize, DataSize_2: usize) -> usize {
    0
}

/// SetPrivateDataInterface - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetPrivateDataInterface(guid: usize, pData: *mut core::ffi::c_void) -> usize {
    0
}

/// GetDesc1 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetDesc1() -> usize {
    0
}

/// Map - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn Map(Subresource: u32, pReadRange: *mut core::ffi::c_void, resource: usize) -> usize {
    0
}

/// Unmap - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn Unmap(Subresource: u32, pWrittenRange: *mut core::ffi::c_void) -> usize {
    0
}

/// GetDeviceRemovedReason - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetDeviceRemovedReason() -> usize {
    0
}

/// ClearState - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ClearState(pPipelineState: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateShaderResourceView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateShaderResourceView(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, DestDescriptor: usize) -> usize {
    0
}

/// CreateRenderTargetView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateRenderTargetView(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, DestDescriptor: usize) -> usize {
    0
}

/// CreateDepthStencilView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateDepthStencilView(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, DestDescriptor: usize) -> usize {
    0
}

/// GetCreationFlags - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetCreationFlags() -> usize {
    0
}

/// ClearRenderTargetView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ClearRenderTargetView(RenderTargetView: usize, arg1: usize, NumRects: usize, arg3: usize) -> usize {
    0
}

/// ClearDepthStencilView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ClearDepthStencilView(DepthStencilView: usize, ClearFlags: usize, Depth: usize, Stencil: usize, NumRects: usize, arg5: usize) -> usize {
    0
}

/// SetPredication - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetPredication(pBuffer: *mut core::ffi::c_void, AlignedBufferOffset: usize, Operation: usize) -> usize {
    0
}

/// CopyResource - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CopyResource(pDstResource: *mut core::ffi::c_void, pSrcResource: *mut core::ffi::c_void) -> usize {
    0
}

/// ResolveSubresource - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ResolveSubresource(pDstResource: *mut core::ffi::c_void, DstSubresource: usize, pSrcResource: *mut core::ffi::c_void, SrcSubresource: usize, Format: usize) -> usize {
    0
}

/// DrawInstanced - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn DrawInstanced(VertexCountPerInstance: usize, InstanceCount: usize, StartVertexLocation: usize, StartInstanceLocation: usize) -> usize {
    0
}

/// DrawIndexedInstanced - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn DrawIndexedInstanced(IndexCountPerInstance: usize, InstanceCount: usize, StartIndexLocation: usize, BaseVertexLocation: usize, StartInstanceLocation: usize) -> usize {
    0
}

/// IASetPrimitiveTopology - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn IASetPrimitiveTopology(PrimitiveTopology: usize) -> usize {
    0
}

/// IASetVertexBuffers - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn IASetVertexBuffers(StartSlot: usize, NumViews: usize, arg2: usize) -> usize {
    0
}

/// IASetIndexBuffer - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn IASetIndexBuffer(pView: *mut core::ffi::c_void) -> usize {
    0
}

/// OMSetRenderTargets - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn OMSetRenderTargets(NumRenderTargetDescriptors: usize, pRenderTargetDescriptors: *mut core::ffi::c_void, RTsSingleHandleToDescriptorRange: usize, pDepthStencilDescriptor: *mut core::ffi::c_void) -> usize {
    0
}

/// RSSetViewports - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn RSSetViewports(arg0: usize, arg1: usize) -> usize {
    0
}

/// RSSetScissorRects - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn RSSetScissorRects(arg0: usize, arg1: usize) -> usize {
    0
}

/// SOSetTargets - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SOSetTargets(StartSlot: usize, NumViews: usize, arg2: usize) -> usize {
    0
}

/// BeginEvent - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn BeginEvent(Metadata: u32, arg1: usize) -> usize {
    0
}

/// EndEvent - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn EndEvent() -> usize {
    0
}

/// SetMarker - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetMarker(Metadata: u32, arg1: usize) -> usize {
    0
}

/// GetDescFromD3D12 - from dxvk/d3d11_buffer.h
#[no_mangle]
pub unsafe extern "C" fn GetDescFromD3D12(pResource: *mut core::ffi::c_void, pResourceFlags: *mut core::ffi::c_void, pBufferDesc: *mut core::ffi::c_void) -> i32 {
    0
}

/// DiscardResource - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn DiscardResource(pResource: *mut core::ffi::c_void, pRegion: *mut core::ffi::c_void) -> usize {
    0
}

/// ClearUnorderedAccessViewUint - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ClearUnorderedAccessViewUint(ViewGPUHandleInCurrentHeap: usize, ViewCPUHandle: usize, pResource: *mut core::ffi::c_void, arg3: usize, NumRects: usize, arg5: usize) -> usize {
    0
}

/// ClearUnorderedAccessViewFloat - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ClearUnorderedAccessViewFloat(ViewGPUHandleInCurrentHeap: usize, ViewCPUHandle: usize, pResource: *mut core::ffi::c_void, arg3: usize, NumRects: usize, arg5: usize) -> usize {
    0
}

/// Dispatch - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn Dispatch(ThreadGroupCountX: usize, ThreadGroupCountY: usize, ThreadGroupCountZ: usize) -> usize {
    0
}

/// CopyTiles - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CopyTiles(pTiledResource: *mut core::ffi::c_void, pTileRegionStartCoordinate: *mut core::ffi::c_void, pTileRegionSize: *mut core::ffi::c_void, pBuffer: *mut core::ffi::c_void, BufferStartOffsetInBytes: usize, Flags: usize) -> usize {
    0
}

/// CopyTileMappings - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CopyTileMappings(pDstResource: *mut core::ffi::c_void, pDstRegionStartCoordinate: *mut core::ffi::c_void, pSrcResource: *mut core::ffi::c_void, pSrcRegionStartCoordinate: *mut core::ffi::c_void, pRegionSize: *mut core::ffi::c_void, Flags: usize) -> usize {
    0
}

/// Signal - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn Signal(Value: usize) -> usize {
    0
}

/// CreateUnorderedAccessView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateUnorderedAccessView(pResource: *mut core::ffi::c_void, pCounterResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, DestDescriptor: usize) -> usize {
    0
}

/// CreateFence - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateFence(InitialValue: usize, Flags: usize, riid: usize, ppFence: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// ReadFromSubresource - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ReadFromSubresource(pDstData: *mut core::ffi::c_void, DstRowPitch: u32, DstDepthPitch: u32, SrcSubresource: u32, pSrcBox: *mut core::ffi::c_void) -> usize {
    0
}

/// WriteToSubresource - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn WriteToSubresource(DstSubresource: u32, pDstBox: *mut core::ffi::c_void, pSrcData: *mut core::ffi::c_void, SrcRowPitch: u32, SrcDepthPitch: u32) -> usize {
    0
}

/// CheckFeatureSupport - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CheckFeatureSupport(Feature: usize, arg1: usize) -> usize {
    0
}

/// GetResourceTiling - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetResourceTiling(pTiledResource: *mut core::ffi::c_void, pNumTilesForEntireResource: *mut core::ffi::c_void, pPackedMipDesc: *mut core::ffi::c_void, pStandardTileShapeForNonPackedMips: *mut core::ffi::c_void, pNumSubresourceTilings: *mut core::ffi::c_void, FirstSubresourceTilingToGet: usize, arg6: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateSharedHandle - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateSharedHandle(pObject: *mut core::ffi::c_void, pAttributes: *mut core::ffi::c_void, Access: u32, Name: usize, pHandle: *mut core::ffi::c_void) -> usize {
    0
}

/// SetEventOnCompletion - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetEventOnCompletion(Value: usize, hEvent: *mut core::ffi::c_void) -> usize {
    0
}

/// GetCompletedValue - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetCompletedValue() -> usize {
    0
}

/// GetD3D12Device - from dxvk/d3d11_on_12.h
#[no_mangle]
pub unsafe extern "C" fn GetD3D12Device(riid: usize, ppvDevice: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetDXGIAdapter - from dxvk/d3d11_on_12_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn GetDXGIAdapter(iid: usize, ppvObject: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetName - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetName(Format: usize, arg1: usize) -> *const i8 {
    core::ptr::null_mut()
}

/// Present - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn Present(pResource: *mut core::ffi::c_void, Subresource: u32, window: usize) -> usize {
    0
}

/// GetDxgiUsage - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn GetDxgiUsage() -> usize {
    0
}

/// GetPlaneCount - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetPlaneCount(Format: usize) -> usize {
    0
}

/// SetRenderTarget - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetRenderTarget(renderTargetIndex: u32, renderTargetBlendDesc: usize) {

}

/// DECLSPEC_UUID - from DirectX-Headers/d3d12shader.h
#[no_mangle]
pub unsafe extern "C" fn DECLSPEC_UUID(arg0: usize) -> usize {
    0
}

/// GetAdapterCount - from DirectX-Headers/dxcore_interface.h
#[no_mangle]
pub unsafe extern "C" fn GetAdapterCount() -> usize {
    0
}

/// CreateDevice - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateDevice(adapter: *mut core::ffi::c_void, FeatureLevel: usize, riid: usize, ppvDevice: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CalcSubresource - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn CalcSubresource(MipSlice: u32, ArraySlice: u32, PlaneSlice: u32) -> u32 {
    0
}

/// IsValid - from DirectX-Headers/dxcore_interface.h
#[no_mangle]
pub unsafe extern "C" fn IsValid() -> usize {
    0
}

/// T - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn T(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// GetFlags - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetFlags() -> usize {
    0
}

/// RegisterDestructionCallback - from DirectX-Headers/d3dcommon.h
#[no_mangle]
pub unsafe extern "C" fn RegisterDestructionCallback(callbackFn: usize, pData: *mut core::ffi::c_void, pCallbackID: *mut core::ffi::c_void) -> usize {
    0
}

/// UnregisterDestructionCallback - from DirectX-Headers/d3dcommon.h
#[no_mangle]
pub unsafe extern "C" fn UnregisterDestructionCallback(callbackID: usize) -> usize {
    0
}

/// IDXGISwapChain3_GetCurrentBackBufferIndex - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn IDXGISwapChain3_GetCurrentBackBufferIndex(arg0: usize) -> usize {
    0
}

/// ID3D12Fence_SetEventOnCompletion - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn ID3D12Fence_SetEventOnCompletion(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// enable_d3d12_debug_layer - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn enable_d3d12_debug_layer(argc: i32, argv: *mut *mut i8) {

}

/// d3d12_device_open_resource_descriptor - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_open_resource_descriptor(device: *mut core::ffi::c_void, handle: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_open_kmt - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_open_kmt(device: *mut core::ffi::c_void) {

}

/// d3d12_device_close_kmt - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_close_kmt(device: *mut core::ffi::c_void) {

}

/// d3d12_command_list_Dispatch_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_Dispatch_profiled(iface: *mut core::ffi::c_void, x: u32, y: u32, z: u32) -> usize {
    0
}

/// d3d12_command_list_CopyTextureRegion_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyTextureRegion_profiled(iface: *mut core::ffi::c_void, dst: *mut core::ffi::c_void, dst_x: u32, dst_y: u32, dst_z: u32, src: *mut core::ffi::c_void, src_box: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_CopyResource_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyResource_profiled(iface: *mut core::ffi::c_void, dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_RSSetViewports_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetViewports_profiled(iface: *mut core::ffi::c_void, viewport_count: u32, viewports: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_RSSetScissorRects_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetScissorRects_profiled(iface: *mut core::ffi::c_void, rect_count: u32, rects: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_OMSetBlendFactor_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetBlendFactor_profiled(iface: *mut core::ffi::c_void, blend_factor4: f32) -> usize {
    0
}

/// d3d12_command_list_OMSetStencilRef_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetStencilRef_profiled(iface: *mut core::ffi::c_void, stencil_ref: u32) -> usize {
    0
}

/// d3d12_command_list_SetPipelineState_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPipelineState_profiled(iface: *mut core::ffi::c_void, pipeline_state: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_ResourceBarrier_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ResourceBarrier_profiled(iface: *mut core::ffi::c_void, barrier_count: u32, barriers: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_ExecuteBundle_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ExecuteBundle_profiled(iface: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_SetDescriptorHeaps_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetDescriptorHeaps_profiled(iface: *mut core::ffi::c_void, heap_count: u32, heaps: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_SetComputeRootSignature_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootSignature_profiled(iface: *mut core::ffi::c_void, root_signature: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_SetGraphicsRootSignature_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootSignature_profiled(iface: *mut core::ffi::c_void, root_signature: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_IASetIndexBuffer_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetIndexBuffer_profiled(iface: *mut core::ffi::c_void, view: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_IASetVertexBuffers_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetVertexBuffers_profiled(iface: *mut core::ffi::c_void, start_slot: u32, view_count: u32, views: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_SOSetTargets_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SOSetTargets_profiled(iface: *mut core::ffi::c_void, start_slot: u32, view_count: u32, views: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_ClearRenderTargetView_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ClearRenderTargetView_profiled(iface: *mut core::ffi::c_void, rtv: usize, color4: f32, rect_count: u32, rects: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_DiscardResource_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DiscardResource_profiled(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, region: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_BeginQuery_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_BeginQuery_profiled(iface: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, arg2: usize, index: u32) -> usize {
    0
}

/// d3d12_command_list_EndQuery_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_EndQuery_profiled(iface: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, arg2: usize, index: u32) -> usize {
    0
}

/// d3d12_command_list_SetPredication_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPredication_profiled(iface: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, aligned_buffer_offset: usize, operation: usize) -> usize {
    0
}

/// d3d12_command_list_SetMarker_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetMarker_profiled(iface: *mut core::ffi::c_void, metadata: u32, data: *mut core::ffi::c_void, size: u32) -> usize {
    0
}

/// d3d12_command_list_BeginEvent_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_BeginEvent_profiled(iface: *mut core::ffi::c_void, metadata: u32, data: *mut core::ffi::c_void, size: u32) -> usize {
    0
}

/// d3d12_command_list_EndEvent_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_EndEvent_profiled(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_OMSetDepthBounds_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetDepthBounds_profiled(iface: *mut core::ffi::c_void, min: f32, max: f32) -> usize {
    0
}

/// d3d12_command_list_SetProtectedResourceSession_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetProtectedResourceSession_profiled(iface: *mut core::ffi::c_void, protected_session: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_EndRenderPass_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_EndRenderPass_profiled(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_CopyRaytracingAccelerationStructure_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyRaytracingAccelerationStructure_profiled(iface: *mut core::ffi::c_void, dst_data: usize, src_data: usize, mode: usize) -> usize {
    0
}

/// d3d12_command_list_SetPipelineState1_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPipelineState1_profiled(iface: *mut core::ffi::c_void, state_object: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_DispatchRays_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DispatchRays_profiled(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_RSSetShadingRate_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetShadingRate_profiled(iface: *mut core::ffi::c_void, base: usize, combiners: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_RSSetShadingRateImage_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetShadingRateImage_profiled(iface: *mut core::ffi::c_void, image: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_DispatchMesh_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DispatchMesh_profiled(iface: *mut core::ffi::c_void, x: u32, y: u32, z: u32) -> usize {
    0
}

/// d3d12_command_list_Barrier_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_Barrier_profiled(iface: *mut core::ffi::c_void, NumBarrierGroups: usize, pBarrierGroups: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_IASetIndexBufferStripCutValue_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetIndexBufferStripCutValue_profiled(iface: *mut core::ffi::c_void, IBStripCutValue: usize) -> usize {
    0
}

/// d3d12_command_list_SetProgram_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetProgram_profiled(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_DispatchGraph_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DispatchGraph_profiled(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_CopyDescriptors_profiled - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptors_profiled(iface: *mut core::ffi::c_void, dst_descriptor_range_count: u32, dst_descriptor_range_offsets: *mut core::ffi::c_void, dst_descriptor_range_sizes: *mut u32, src_descriptor_range_count: u32, src_descriptor_range_offsets: *mut core::ffi::c_void, src_descriptor_range_sizes: *mut u32, descriptor_heap_type: usize) -> usize {
    0
}

/// d3d12_device_CopyDescriptorsSimple_profiled - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptorsSimple_profiled(iface: *mut core::ffi::c_void, descriptor_count: u32, dst_descriptor_range_offset: usize, src_descriptor_range_offset: usize, descriptor_heap_type: usize) -> usize {
    0
}

/// d3d12_object_SetName - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_object_SetName(iface: *mut core::ffi::c_void, name: *mut u16) -> usize {
    0
}

/// impl_from_ID3D12Fence1 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn impl_from_ID3D12Fence1(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_fence_set_event_on_completion - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_set_event_on_completion(fence: *mut core::ffi::c_void, value: usize, event: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_fence_set_native_sync_handle_on_completion - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_set_native_sync_handle_on_completion(fence: *mut core::ffi::c_void, value: usize, handle: usize) -> i32 {
    0
}

/// shared_impl_from_ID3D12Fence1 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn shared_impl_from_ID3D12Fence1(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// is_shared_ID3D12Fence1 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn is_shared_ID3D12Fence1(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// is_shared_ID3D12Fence - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn is_shared_ID3D12Fence(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_validate_custom_heap_type - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_validate_custom_heap_type(device: *mut core::ffi::c_void, heap_properties: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_heap_incref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_incref(heap: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_heap_decref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_decref(heap: *mut core::ffi::c_void) -> u32 {
    0
}

/// impl_from_ID3D12Heap1 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn impl_from_ID3D12Heap1(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_is_buffer - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_is_buffer(resource: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_is_acceleration_structure - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_is_acceleration_structure(resource: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_is_texture - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_is_texture(resource: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_pick_layout - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_pick_layout(resource: *mut core::ffi::c_void, layout: usize) -> usize {
    0
}

/// d3d12_resource_incref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_incref(resource: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_resource_decref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_decref(resource: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_resource_decref_retained - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_decref_retained(resource: *mut core::ffi::c_void) {

}

/// d3d12_resource_incref_weak - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_incref_weak(resource: *mut core::ffi::c_void) {

}

/// d3d12_resource_decref_weak - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_decref_weak(resource: *mut core::ffi::c_void) {

}

/// d3d12_resource_is_cpu_accessible - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_is_cpu_accessible(resource: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_promote_desc - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_promote_desc(desc: *mut core::ffi::c_void, desc1: *mut core::ffi::c_void) {

}

/// d3d12_resource_validate_desc - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_validate_desc(desc: *mut core::ffi::c_void, num_castable_formats: u32, castable_formats: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// impl_from_ID3D12Resource2 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn impl_from_ID3D12Resource2(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_desc_copy - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_copy(dst: usize, src: usize, count: u32, heap_type: usize, device: *mut core::ffi::c_void) {

}

/// d3d12_rtv_desc_copy - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_rtv_desc_copy(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, count: u32) {

}

/// d3d12_descriptor_heap_cleanup - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_cleanup(descriptor_heap: *mut core::ffi::c_void) {

}

/// d3d12_descriptor_heap_require_padding_descriptors - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_require_padding_descriptors(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_desc_decode_embedded_resource_va - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_decode_embedded_resource_va(va: usize) -> usize {
    0
}

/// d3d12_desc_copy_embedded_resource - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_copy_embedded_resource(dst_va: usize, src_va: usize, size: usize) {

}

/// d3d12_desc_decode_va - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_decode_va(va: usize) -> usize {
    0
}

/// d3d12_desc_heap_offset_from_embedded_gpu_handle - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_heap_offset_from_embedded_gpu_handle(handle: usize, cbv_srv_uav_size_log2: u32, sampler_size_log2: u32) -> u32 {
    0
}

/// d3d12_desc_heap_offset_from_gpu_handle - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_heap_offset_from_gpu_handle(handle: usize) -> u32 {
    0
}

/// d3d12_root_signature_inc_ref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_inc_ref(state: *mut core::ffi::c_void) {

}

/// d3d12_root_signature_dec_ref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_dec_ref(state: *mut core::ffi::c_void) {

}

/// d3d12_root_signature_get_shader_interface_flags - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_get_shader_interface_flags(root_signature: *mut core::ffi::c_void, pipeline_type: usize) -> u32 {
    0
}

/// d3d12_root_signature_is_pipeline_compatible - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_is_pipeline_compatible(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_root_signature_is_layout_compatible - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_is_layout_compatible(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_state_is_compute - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_is_compute(state: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_state_is_graphics - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_is_graphics(state: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_graphics_pipeline_state_has_unknown_dsv_format_with_test - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_graphics_pipeline_state_has_unknown_dsv_format_with_test(graphics: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_state_inc_public_ref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_inc_public_ref(state: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_pipeline_state_inc_ref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_inc_ref(state: *mut core::ffi::c_void) {

}

/// d3d12_pipeline_state_dec_ref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_dec_ref(state: *mut core::ffi::c_void) {

}

/// d3d12_pipeline_state_has_replaced_shaders - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_has_replaced_shaders(state: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_state_get_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_get_pipeline(state: *mut core::ffi::c_void, dyn_state: *mut core::ffi::c_void, dsv_format: *mut core::ffi::c_void, dynamic_state_flags: *mut u32) -> usize {
    0
}

/// d3d12_cached_pipeline_state_validate - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_cached_pipeline_state_validate(device: *mut core::ffi::c_void, state: *mut core::ffi::c_void, compat: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_cached_pipeline_state_is_dummy - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_cached_pipeline_state_is_dummy(state: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_inc_public_ref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_inc_public_ref(state: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_pipeline_library_dec_public_ref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_dec_public_ref(state: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_pipeline_library_inc_ref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_inc_ref(state: *mut core::ffi::c_void) {

}

/// d3d12_pipeline_library_dec_ref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_dec_ref(state: *mut core::ffi::c_void) {

}

/// d3d12_command_list_decay_tracked_state - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_decay_tracked_state(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_reset_query - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_query(list: *mut core::ffi::c_void, vk_pool: usize, index: u32) -> usize {
    0
}

/// d3d12_command_list_end_current_render_pass - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_end_current_render_pass(list: *mut core::ffi::c_void, suspend: usize) {

}

/// d3d12_command_list_invalidate_all_state - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_invalidate_all_state(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_debug_mark_label - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_debug_mark_label(list: *mut core::ffi::c_void, tag: *mut i8, r: f32, g: f32, b: f32, a: f32) {

}

/// d3d12_command_list_debug_mark_begin_region - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_debug_mark_begin_region(list: *mut core::ffi::c_void, tag: *mut i8) {

}

/// d3d12_command_list_debug_mark_end_region - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_debug_mark_end_region(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_invalidate_current_pipeline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_invalidate_current_pipeline(list: *mut core::ffi::c_void, meta_shader: usize) {

}

/// d3d12_command_list_invalidate_root_parameters - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_invalidate_root_parameters(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void, invalidate_descriptor_heaps: usize, sibling_push_domain: *mut core::ffi::c_void) {

}

/// d3d12_command_list_update_descriptor_buffers - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_descriptor_buffers(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_flush_dgc_batch - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_flush_dgc_batch(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_fetch_root_parameter_data - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_fetch_root_parameter_data(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void, dst_data: *mut core::ffi::c_void) {

}

/// d3d12_bundle_execute - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_execute(bundle: *mut core::ffi::c_void, list: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_submit_stop - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_submit_stop(queue: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_signal_inline - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_signal_inline(queue: *mut core::ffi::c_void, fence: *mut core::ffi::c_void, value: u64) {

}

/// d3d12_command_queue_enqueue_callback - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_enqueue_callback(queue: *mut core::ffi::c_void, callback: ()) {

}

/// d3d12_device_get_nv_shader_extn - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_nv_shader_extn(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_add_queue_timeline_deferred_decref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_add_queue_timeline_deferred_decref(device: *mut core::ffi::c_void, inc_call: ()) {

}

/// d3d12_device_is_uma - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_is_uma(device: *mut core::ffi::c_void, coherent: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_mark_as_removed - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_mark_as_removed(device: *mut core::ffi::c_void, reason: i32, message: *mut i8) {

}

/// d3d12_device_report_fault - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_report_fault(device: *mut core::ffi::c_void) {

}

/// d3d12_device_removed_reason - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_removed_reason(device: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_get_max_descriptor_heap_size - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_max_descriptor_heap_size(device: *mut core::ffi::c_void, heap_type: usize) -> u32 {
    0
}

/// d3d12_device_validate_shader_meta - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_validate_shader_meta(device: *mut core::ffi::c_void, meta: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_get_scratch_buffer - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_scratch_buffer(device: *mut core::ffi::c_void, kind: usize, min_size: u64, memory_types: u32, scratch: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_return_scratch_buffer - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_return_scratch_buffer(device: *mut core::ffi::c_void, kind: usize, scratch: *mut core::ffi::c_void) {

}

/// d3d12_device_get_query_pool - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_query_pool(device: *mut core::ffi::c_void, type_index: u32, pool: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_return_query_pool - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_return_query_pool(device: *mut core::ffi::c_void, pool: *mut core::ffi::c_void) {

}

/// d3d12_device_get_descriptor_heap_gpu_va - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_descriptor_heap_gpu_va(device: *mut core::ffi::c_void, arg1: usize) -> u64 {
    0
}

/// d3d12_device_return_descriptor_heap_gpu_va - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_return_descriptor_heap_gpu_va(device: *mut core::ffi::c_void, va: u64) {

}

/// d3d12_device_uses_descriptor_buffers - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_uses_descriptor_buffers(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_query_interface - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_query_interface(device: *mut core::ffi::c_void, iid: usize, object: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// ID3D12Device12_QueryInterface - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device12_QueryInterface(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// d3d12_device_add_ref_common - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_add_ref_common(device: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_device_release_common - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_release_common(device: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_device_add_ref - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_add_ref(device: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_device_release - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_release(device: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_device_use_embedded_mutable_descriptors - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_use_embedded_mutable_descriptors(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_desc_decode_metadata - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_decode_metadata(device: *mut core::ffi::c_void, va: usize) -> usize {
    0
}

/// d3d12_device_use_ssbo_raw_buffer - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_use_ssbo_raw_buffer(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_get_ssbo_alignment - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_ssbo_alignment(device: *mut core::ffi::c_void) -> u64 {
    0
}

/// d3d12_device_use_ssbo_root_descriptors - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_use_ssbo_root_descriptors(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_supports_variable_shading_rate_tier_1 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_variable_shading_rate_tier_1(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_supports_variable_shading_rate_tier_2 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_variable_shading_rate_tier_2(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_supports_ray_tracing_tier_1_0 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_ray_tracing_tier_1_0(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_supports_ray_tracing_tier_1_2 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_ray_tracing_tier_1_2(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_determine_shading_rate_image_tile_size - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_determine_shading_rate_image_tile_size(device: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_device_supports_required_subgroup_size_for_stage - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_required_subgroup_size_for_stage(device: *mut core::ffi::c_void, stage: usize) -> usize {
    0
}

/// d3d12_device_supports_workgraphs - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_workgraphs(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_supports_unified_layouts - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_unified_layouts(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_register_swapchain - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_register_swapchain(device: *mut core::ffi::c_void, chain: *mut core::ffi::c_void) {

}

/// d3d12_device_remove_swapchain - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_remove_swapchain(device: *mut core::ffi::c_void, chain: *mut core::ffi::c_void) {

}

/// d3d12_state_object_association_data_equal - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_association_data_equal(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_rt_state_object_add - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_rt_state_object_add(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, parent: *mut core::ffi::c_void, object: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_command_list_workgraph_initialize_scratch - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_initialize_scratch(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_workgraph_dispatch - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_dispatch(list: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) {

}

/// d3d12_box_is_empty - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_box_is_empty(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_desc_get_layer_count - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_get_layer_count(desc: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_resource_get_view_subresource_extent - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_get_view_subresource_extent(resource: *mut core::ffi::c_void, view: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_desc_get_active_level_count - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_get_active_level_count(desc: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_resource_desc_is_sampler_feedback - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_is_sampler_feedback(arg0: usize) -> usize {
    0
}

/// d3d12_resource_desc_get_active_feedback_extent - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_get_active_feedback_extent(desc: *mut core::ffi::c_void, mip_level: u32) -> usize {
    0
}

/// d3d12_resource_desc_get_padded_feedback_extent - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_get_padded_feedback_extent(desc: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_desc_get_sub_resource_count_per_plane - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_get_sub_resource_count_per_plane(desc: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_resource_desc_get_sub_resource_count - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_get_sub_resource_count(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> u32 {
    0
}

/// ID3D12GraphicsCommandList_ClearUnorderedAccessViewUint - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12GraphicsCommandList_ClearUnorderedAccessViewUint(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// ID3D12GraphicsCommandList_ExecuteIndirect - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12GraphicsCommandList_ExecuteIndirect(arg0: usize, arg1: usize, arg2: usize, arg3: usize, draw_arguments: *mut core::ffi::c_void) -> usize {
    0
}

/// ID3D12Device2_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device2_Release(arg0: usize) -> usize {
    0
}

/// ID3D12Device_GetDescriptorHandleIncrementSize - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device_GetDescriptorHandleIncrementSize(arg0: usize, arg1: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// ID3D12GraphicsCommandList8_OMSetStencilRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12GraphicsCommandList8_OMSetStencilRef(arg0: usize, arg1: usize) -> usize {
    0
}

/// ID3D12Fence_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12Fence_Release(arg0: usize) -> usize {
    0
}

/// ID3D12DeviceFactory_SetFlags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12DeviceFactory_SetFlags(arg0: usize, arg1: usize) -> usize {
    0
}

/// ID3D12DeviceFactory_ApplyToGlobalState - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12DeviceFactory_ApplyToGlobalState(arg0: usize) -> usize {
    0
}

/// ID3D12Device_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device_Release(arg0: usize) -> usize {
    0
}

/// ID3D12Device5_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device5_Release(arg0: usize) -> usize {
    0
}

/// ID3D12GraphicsCommandList_OMSetRenderTargets - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12GraphicsCommandList_OMSetRenderTargets(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// ID3D12GraphicsCommandList4_Reset - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12GraphicsCommandList4_Reset(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// ID3D12Resource_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12Resource_Release(arg0: usize) -> usize {
    0
}

/// ID3D12Heap_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12Heap_Release(arg0: usize) -> usize {
    0
}

/// ID3D12GraphicsCommandList_SetGraphicsRootSignature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12GraphicsCommandList_SetGraphicsRootSignature(arg0: usize, arg1: usize) -> usize {
    0
}

/// ID3D12Fence_Signal - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12Fence_Signal(arg0: usize, arg1: usize) -> usize {
    0
}

/// ID3D12RootSignature_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12RootSignature_Release(arg0: usize) -> usize {
    0
}

/// ID3D12PipelineLibrary_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12PipelineLibrary_Release(arg0: usize) -> usize {
    0
}

/// D3D12SerializeRootSignature - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn D3D12SerializeRootSignature(pRootSignature: *mut core::ffi::c_void, Version: usize, ppBlob: *mut *mut core::ffi::c_void, arg3: usize) -> i32 {
    0
}

/// have_d3d12_device - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn have_d3d12_device() -> usize {
    0
}

/// load_d3d12core_module - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn load_d3d12core_module(module_name: *mut i8) -> usize {
    0
}

/// load_d3d12core_once - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn load_d3d12core_once() {

}

/// load_d3d12core - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn load_d3d12core() -> usize {
    0
}

/// D3D12SerializeVersionedRootSignature - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn D3D12SerializeVersionedRootSignature(pRootSignature: *mut core::ffi::c_void, ppBlob: *mut *mut core::ffi::c_void, arg2: usize) -> i32 {
    0
}

/// D3D12GetDebugInterface - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn D3D12GetDebugInterface(riid: usize, ppvDebug: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_sdk_configuration_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_sdk_configuration_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_sdk_configuration_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_sdk_configuration_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_sdk_configuration_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_sdk_configuration_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_sdk_configuration_SetSDKVersion - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_sdk_configuration_SetSDKVersion(iface: *mut core::ffi::c_void, SDKVersion: u32, SDKPath: *const i8) -> usize {
    0
}

/// D3D12GetInterface - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn D3D12GetInterface(rclsid: usize, riid: usize, ppvDebug: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_dred_settings_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dred_settings_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_dred_settings_SetPageFaultEnablement - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dred_settings_SetPageFaultEnablement(iface: *mut core::ffi::c_void, enablement: usize) -> usize {
    0
}

/// d3d12_dred_settings_SetWatsonDumpEnablement - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dred_settings_SetWatsonDumpEnablement(iface: *mut core::ffi::c_void, enablement: usize) -> usize {
    0
}

/// d3d12_dred_settings_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dred_settings_init(object: *mut core::ffi::c_void) {

}

/// d3d12_get_adapter - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_get_adapter(dxgi_adapter: *mut *mut core::ffi::c_void, adapter: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_find_physical_device - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_find_physical_device(instance: *mut core::ffi::c_void, pfn_vkGetInstanceProcAddr: usize, adapter_desc: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12core_SerializeRootSignature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12core_SerializeRootSignature(core: *mut core::ffi::c_void, root_signature_desc: *mut core::ffi::c_void, version: usize, blob: *mut *mut core::ffi::c_void, error_blob: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12core_SerializeVersionedRootSignature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12core_SerializeVersionedRootSignature(core: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, blob: *mut *mut core::ffi::c_void, error_blob: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12core_GetDebugInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12core_GetDebugInterface(core: *mut core::ffi::c_void, iid: usize, debug: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12core_D3D12GetInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12core_D3D12GetInterface(rcslid: usize, iid: usize, debug: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12core_GetInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12core_GetInterface(core: *mut core::ffi::c_void, rcslid: usize, iid: usize, debug: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_factory_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_factory_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_factory_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_factory_InitializeFromGlobalState - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_InitializeFromGlobalState(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_factory_ApplyToGlobalState - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_ApplyToGlobalState(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_factory_SetFlags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_SetFlags(iface: *mut core::ffi::c_void, flags: usize) -> usize {
    0
}

/// d3d12_device_factory_GetFlags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_GetFlags(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_factory_GetConfigurationInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_GetConfigurationInterface(iface: *mut core::ffi::c_void, clsid: usize, iid: usize, ppv: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_configuration_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_configuration_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_configuration_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_configuration_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_configuration_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_configuration_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_configuration_GetDesc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_configuration_GetDesc(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_device_configuration_SerializeVersionedRootSignature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_configuration_SerializeVersionedRootSignature(iface: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, ppResult: *mut *mut core::ffi::c_void, ppError: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_GetType - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_GetType(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_Close - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_Close(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_SetPipelineState - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetPipelineState(iface: *mut core::ffi::c_void, pipeline_state: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_Reset - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_Reset(iface: *mut core::ffi::c_void, allocator: *mut core::ffi::c_void, initial_pipeline_state: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_ClearState - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_ClearState(iface: *mut core::ffi::c_void, pipeline_state: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_exec_dispatch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_dispatch(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_Dispatch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_Dispatch(iface: *mut core::ffi::c_void, x: u32, y: u32, z: u32) -> usize {
    0
}

/// d3d12_bundle_CopyResource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_CopyResource(iface: *mut core::ffi::c_void, dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_RSSetViewports - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_RSSetViewports(iface: *mut core::ffi::c_void, viewport_count: u32, viewports: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_RSSetScissorRects - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_RSSetScissorRects(iface: *mut core::ffi::c_void, rect_count: u32, rects: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_exec_om_set_blend_factor - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_om_set_blend_factor(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_OMSetBlendFactor - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_OMSetBlendFactor(iface: *mut core::ffi::c_void, blend_factor4: f32) -> usize {
    0
}

/// d3d12_bundle_exec_om_set_stencil_ref - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_om_set_stencil_ref(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_OMSetStencilRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_OMSetStencilRef(iface: *mut core::ffi::c_void, stencil_ref: u32) -> usize {
    0
}

/// d3d12_bundle_exec_set_pipeline_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_pipeline_state(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_ResourceBarrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_ResourceBarrier(iface: *mut core::ffi::c_void, barrier_count: u32, barriers: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_ExecuteBundle - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_ExecuteBundle(iface: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_SetDescriptorHeaps - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetDescriptorHeaps(iface: *mut core::ffi::c_void, heap_count: u32, heaps: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_exec_set_compute_root_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_compute_root_signature(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetComputeRootSignature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetComputeRootSignature(iface: *mut core::ffi::c_void, root_signature: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_exec_set_graphics_root_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_graphics_root_signature(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetGraphicsRootSignature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetGraphicsRootSignature(iface: *mut core::ffi::c_void, root_signature: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_exec_set_compute_root_descriptor_table - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_compute_root_descriptor_table(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetComputeRootDescriptorTable - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetComputeRootDescriptorTable(iface: *mut core::ffi::c_void, root_parameter_index: u32, base_descriptor: usize) -> usize {
    0
}

/// d3d12_bundle_exec_set_graphics_root_descriptor_table - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_graphics_root_descriptor_table(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetGraphicsRootDescriptorTable - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetGraphicsRootDescriptorTable(iface: *mut core::ffi::c_void, root_parameter_index: u32, base_descriptor: usize) -> usize {
    0
}

/// d3d12_bundle_exec_set_compute_root_cbv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_compute_root_cbv(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_exec_set_graphics_root_cbv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_graphics_root_cbv(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_exec_set_compute_root_srv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_compute_root_srv(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetComputeRootShaderResourceView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetComputeRootShaderResourceView(iface: *mut core::ffi::c_void, root_parameter_index: u32, address: usize) -> usize {
    0
}

/// d3d12_bundle_exec_set_graphics_root_srv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_graphics_root_srv(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetGraphicsRootShaderResourceView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetGraphicsRootShaderResourceView(iface: *mut core::ffi::c_void, root_parameter_index: u32, address: usize) -> usize {
    0
}

/// d3d12_bundle_exec_set_compute_root_uav - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_compute_root_uav(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetComputeRootUnorderedAccessView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetComputeRootUnorderedAccessView(iface: *mut core::ffi::c_void, root_parameter_index: u32, address: usize) -> usize {
    0
}

/// d3d12_bundle_exec_set_graphics_root_uav - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_graphics_root_uav(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetGraphicsRootUnorderedAccessView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetGraphicsRootUnorderedAccessView(iface: *mut core::ffi::c_void, root_parameter_index: u32, address: usize) -> usize {
    0
}

/// d3d12_bundle_exec_ia_set_index_buffer_null - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_ia_set_index_buffer_null(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_exec_ia_set_index_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_ia_set_index_buffer(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_IASetIndexBuffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_IASetIndexBuffer(iface: *mut core::ffi::c_void, view: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_exec_ia_set_vertex_buffers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_ia_set_vertex_buffers(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_IASetVertexBuffers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_IASetVertexBuffers(iface: *mut core::ffi::c_void, start_slot: u32, view_count: u32, views: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_ClearDepthStencilView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_ClearDepthStencilView(iface: *mut core::ffi::c_void, dsv: usize, flags: usize, depth: f32, stencil: usize, rect_count: u32, rects: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_ClearRenderTargetView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_ClearRenderTargetView(iface: *mut core::ffi::c_void, rtv: usize, color4: f32, rect_count: u32, rects: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_DiscardResource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_DiscardResource(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, region: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_exec_set_marker - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_marker(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetMarker - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetMarker(iface: *mut core::ffi::c_void, metadata: u32, data: *mut core::ffi::c_void, size: u32) -> usize {
    0
}

/// d3d12_bundle_exec_begin_event - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_begin_event(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_BeginEvent - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_BeginEvent(iface: *mut core::ffi::c_void, metadata: u32, data: *mut core::ffi::c_void, size: u32) -> usize {
    0
}

/// d3d12_bundle_exec_end_event - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_end_event(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_EndEvent - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_EndEvent(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_exec_execute_indirect - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_execute_indirect(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_ExecuteIndirect - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_ExecuteIndirect(iface: *mut core::ffi::c_void, command_signature: *mut core::ffi::c_void, max_command_count: u32, arg_buffer: *mut core::ffi::c_void, arg_buffer_offset: usize, count_buffer: *mut core::ffi::c_void, count_buffer_offset: usize) -> usize {
    0
}

/// d3d12_bundle_exec_om_set_depth_bounds - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_om_set_depth_bounds(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_OMSetDepthBounds - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_OMSetDepthBounds(iface: *mut core::ffi::c_void, min: f32, max: f32) -> usize {
    0
}

/// d3d12_bundle_exec_set_sample_positions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_sample_positions(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetSamplePositions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetSamplePositions(iface: *mut core::ffi::c_void, sample_count: u32, pixel_count: u32, sample_positions: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_SetProtectedResourceSession - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetProtectedResourceSession(iface: *mut core::ffi::c_void, protected_session: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_EndRenderPass - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_EndRenderPass(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_CopyRaytracingAccelerationStructure - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_CopyRaytracingAccelerationStructure(iface: *mut core::ffi::c_void, dst_data: usize, src_data: usize, mode: usize) -> usize {
    0
}

/// d3d12_bundle_exec_set_pipeline_state1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_pipeline_state1(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetPipelineState1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetPipelineState1(iface: *mut core::ffi::c_void, state_object: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_exec_dispatch_rays - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_dispatch_rays(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_DispatchRays - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_DispatchRays(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_exec_rs_set_shading_rate - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_rs_set_shading_rate(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_exec_rs_set_shading_rate_base - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_rs_set_shading_rate_base(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_RSSetShadingRate - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_RSSetShadingRate(iface: *mut core::ffi::c_void, base: usize, combiners: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_exec_rs_set_shading_rate_image - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_rs_set_shading_rate_image(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_RSSetShadingRateImage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_RSSetShadingRateImage(iface: *mut core::ffi::c_void, image: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_exec_dispatch_mesh - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_dispatch_mesh(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_DispatchMesh - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_DispatchMesh(iface: *mut core::ffi::c_void, x: u32, y: u32, z: u32) -> usize {
    0
}

/// d3d12_bundle_exec_om_set_front_and_back_stencil_ref - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_om_set_front_and_back_stencil_ref(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_exec_rs_set_depth_bias - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_rs_set_depth_bias(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_RSSetDepthBias - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_RSSetDepthBias(iface: *mut core::ffi::c_void, DepthBias: f32, DepthBiasClamp: f32, SlopeScaledDepthBias: f32) -> usize {
    0
}

/// d3d12_bundle_SetProgram - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetProgram(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_DispatchGraph - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_DispatchGraph(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_bundle_exec_ia_set_index_buffer_strip_cut_value - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_ia_set_index_buffer_strip_cut_value(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_IASetIndexBufferStripCutValue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_IASetIndexBufferStripCutValue(iface: *mut core::ffi::c_void, IBStripCutValue: usize) -> usize {
    0
}

/// impl_from_ID3D12GraphicsCommandList - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn impl_from_ID3D12GraphicsCommandList(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_cached_pipeline_state_to_flags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_cached_pipeline_state_to_flags(state: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_pipeline_library_find_internal_blob - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_find_internal_blob(pipeline_library: *mut core::ffi::c_void, map: *mut core::ffi::c_void, hash: u64, data: *mut *mut core::ffi::c_void, size: *mut usize) -> usize {
    0
}

/// d3d12_cached_pipeline_entry_name_table_size - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_cached_pipeline_entry_name_table_size(entry: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_pipeline_library_insert_hash_map_blob_internal - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_insert_hash_map_blob_internal(pipeline_library: *mut core::ffi::c_void, map: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_serialize_entry - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_serialize_entry(entry: *mut core::ffi::c_void, header: *mut core::ffi::c_void, data: *mut u8, name_offset: usize, blob_offset: usize) {

}

/// d3d12_pipeline_library_cleanup_map - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_cleanup_map(map: *mut core::ffi::c_void) {

}

/// d3d12_pipeline_library_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_cleanup(pipeline_library: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// d3d12_pipeline_library_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_StorePipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_StorePipeline(iface: *mut core::ffi::c_void, name: *const u16, pipeline: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_load_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_load_pipeline(pipeline_library: *mut core::ffi::c_void, name: *const u16, bind_point: usize, desc: *mut core::ffi::c_void, state: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_pipeline_library_LoadGraphicsPipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_LoadGraphicsPipeline(iface: *mut core::ffi::c_void, name: *const u16, desc: *mut core::ffi::c_void, iid: usize, pipeline_state: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_LoadComputePipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_LoadComputePipeline(iface: *mut core::ffi::c_void, name: *const u16, desc: *mut core::ffi::c_void, iid: usize, pipeline_state: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_get_aligned_name_table_size - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_get_aligned_name_table_size(pipeline_library: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_get_serialized_size - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_get_serialized_size(pipeline_library: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_GetSerializedSize - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_GetSerializedSize(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_serialize_hash_map - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_serialize_hash_map(map: *mut core::ffi::c_void, inout_toc_entries: *mut *mut core::ffi::c_void, serialized_data: *mut u8, inout_name_offset: *mut usize, inout_blob_offset: *mut usize) {

}

/// d3d12_pipeline_library_Serialize - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_Serialize(iface: *mut core::ffi::c_void, data: *mut core::ffi::c_void, data_size: usize) -> usize {
    0
}

/// d3d12_pipeline_library_LoadPipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_LoadPipeline(iface: *mut core::ffi::c_void, name: *const u16, desc: *mut core::ffi::c_void, iid: usize, pipeline_state: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_unserialize_hash_map - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_unserialize_hash_map(pipeline_library: *mut core::ffi::c_void, entries: *mut core::ffi::c_void, entries_count: usize, map: *mut core::ffi::c_void, serialized_data_base: *mut u8, serialized_data_size: usize, inout_name_table: *mut *mut u8) -> i32 {
    0
}

/// d3d12_pipeline_library_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_init(pipeline_library: *mut core::ffi::c_void, device: *mut core::ffi::c_void, blob: *mut core::ffi::c_void, blob_length: usize, flags: u32) -> i32 {
    0
}

/// d3d12_fence_signal - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_signal(fence: *mut core::ffi::c_void, worker: *mut core::ffi::c_void, value: u64) -> i32 {
    0
}

/// d3d12_command_queue_add_submission - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_add_submission(queue: *mut core::ffi::c_void, sub: *mut core::ffi::c_void) {

}

/// d3d12_fence_inc_ref - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_inc_ref(fence: *mut core::ffi::c_void) {

}

/// d3d12_fence_dec_ref - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_dec_ref(fence: *mut core::ffi::c_void) {

}

/// d3d12_shared_fence_inc_ref - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_inc_ref(fence: *mut core::ffi::c_void) {

}

/// d3d12_shared_fence_dec_ref - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_dec_ref(fence: *mut core::ffi::c_void) {

}

/// d3d12_fence_iface_inc_ref - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_iface_inc_ref(iface: *mut core::ffi::c_void) {

}

/// d3d12_fence_iface_dec_ref - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_iface_dec_ref(iface: *mut core::ffi::c_void) {

}

/// d3d12_fence_signal_cpu_timeline_semaphore - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_signal_cpu_timeline_semaphore(fence: *mut core::ffi::c_void, value: u64) -> i32 {
    0
}

/// d3d12_command_list_barrier_batch_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_barrier_batch_init(batch: *mut core::ffi::c_void) {

}

/// d3d12_command_list_barrier_batch_end - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_barrier_batch_end(list: *mut core::ffi::c_void, batch: *mut core::ffi::c_void) {

}

/// d3d12_command_list_barrier_batch_add_layout_transition - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_barrier_batch_add_layout_transition(list: *mut core::ffi::c_void, batch: *mut core::ffi::c_void, image_barrier: *mut core::ffi::c_void) {

}

/// d3d12_command_list_barrier_batch_add_global_transition - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_barrier_batch_add_global_transition(list: *mut core::ffi::c_void, batch: *mut core::ffi::c_void, srcStageMask: usize, srcAccessMask: usize, dstStageMask: usize, dstAccessMask: usize) {

}

/// d3d12_command_list_promote_dsv_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_promote_dsv_resource(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, plane_optimal_mask: u32) -> u32 {
    0
}

/// d3d12_command_list_notify_decay_dsv_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_notify_decay_dsv_resource(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_command_list_notify_dsv_discard - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_notify_dsv_discard(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, first_subresource: u32, subresource_count: u32, resource_subresource_count: u32) {

}

/// d3d12_command_list_get_depth_stencil_resource_layout - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_get_depth_stencil_resource_layout(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, plane_optimal_mask: *mut u32) -> usize {
    0
}

/// d3d12_command_list_decay_optimal_dsv_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_decay_optimal_dsv_resource(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, plane_optimal_mask: u32, batch: *mut core::ffi::c_void) {

}

/// d3d12_command_list_end_transfer_batch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_end_transfer_batch(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_end_transfer_batch_if_trivial - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_end_transfer_batch_if_trivial(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_end_wbi_batch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_end_wbi_batch(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_ensure_transfer_batch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ensure_transfer_batch(list: *mut core::ffi::c_void, arg1: usize) {

}

/// d3d12_command_list_flush_rtas_batch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_flush_rtas_batch(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_clear_rtas_batch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_clear_rtas_batch(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_flush_query_resolves - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_flush_query_resolves(list: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_ensure_fence_signal_order - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_ensure_fence_signal_order(command_queue: *mut core::ffi::c_void, fence: *mut core::ffi::c_void, update_count: u64) {

}

/// d3d12_fence_register_pending_gpu_wait - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_register_pending_gpu_wait(fence: *mut core::ffi::c_void, value: u64) -> u64 {
    0
}

/// d3d12_fence_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_fence_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_fence_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_fence_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_fence_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_fence_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_fence_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_fence_GetCompletedValue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_GetCompletedValue(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_fence_SetEventOnCompletion - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_SetEventOnCompletion(iface: *mut core::ffi::c_void, value: usize, event: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_fence_Signal - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_Signal(iface: *mut core::ffi::c_void, value: usize) -> usize {
    0
}

/// d3d12_fence_GetCreationFlags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_GetCreationFlags(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_fence_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_init(fence: *mut core::ffi::c_void, device: *mut core::ffi::c_void, initial_value: usize, flags: usize) -> i32 {
    0
}

/// d3d12_shared_fence_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_shared_fence_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_shared_fence_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_shared_fence_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_shared_fence_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_shared_fence_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_shared_fence_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_shared_fence_GetCompletedValue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_GetCompletedValue(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_shared_fence_SetEventOnCompletion - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_SetEventOnCompletion(iface: *mut core::ffi::c_void, value: usize, os_event: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_shared_fence_Signal - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_Signal(iface: *mut core::ffi::c_void, value: usize) -> usize {
    0
}

/// d3d12_shared_fence_GetCreationFlags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_GetCreationFlags(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_mark_as_invalid - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_mark_as_invalid(list: *mut core::ffi::c_void, message: *mut i8) {

}

/// d3d12_command_list_update_conditional_rendering_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_conditional_rendering_state(list: *mut core::ffi::c_void, end: usize) {

}

/// d3d12_command_list_begin_command_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_begin_command_buffer(list: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_command_list_begin_new_sequence - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_begin_new_sequence(list: *mut core::ffi::c_void, fallback: usize) {

}

/// d3d12_command_list_allows_new_sequence - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_allows_new_sequence(list: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_consider_new_sequence - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_consider_new_sequence(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_debug_mark_begin_region_cmd - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_debug_mark_begin_region_cmd(list: *mut core::ffi::c_void, vk_cmd: usize, tag: *mut i8) {

}

/// d3d12_command_list_debug_mark_end_region_cmd - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_debug_mark_end_region_cmd(list: *mut core::ffi::c_void, vk_cmd: usize) {

}

/// d3d12_command_list_register_used_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_register_used_resource(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void) {

}

/// d3d12_command_list_find_attachment_view - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_find_attachment_view(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, subresource: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_command_list_find_attachment - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_find_attachment(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, view: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_command_list_clear_attachment_inline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_clear_attachment_inline(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, view: *mut core::ffi::c_void, attachment_idx: u32, clear_aspects: usize, clear_value: *mut core::ffi::c_void, rect_count: u32, rects: *mut core::ffi::c_void) {

}

/// d3d12_command_list_reset_transfer_waw_tracking - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_transfer_waw_tracking(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_resolve_transfer_waw - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_resolve_transfer_waw(list: *mut core::ffi::c_void) {

}

/// d3d12_barrier_subresource_range_covers_aspects - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_barrier_subresource_range_covers_aspects(resource: *mut core::ffi::c_void, range: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_decay_optimal_dsv_resources - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_decay_optimal_dsv_resources(list: *mut core::ffi::c_void) {

}

/// d3d12_resource_may_alias_other_resources - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_may_alias_other_resources(resource: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_debug_mark_label_cmd - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_debug_mark_label_cmd(list: *mut core::ffi::c_void, vk_cmd: usize, tag: *mut i8, r: f32, g: f32, b: f32, a: f32) {

}

/// d3d12_command_list_load_attachment - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_load_attachment(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, view: *mut core::ffi::c_void, clear_aspects: usize, clear_value: *mut core::ffi::c_void, rect_count: u32, rects: *mut core::ffi::c_void, load_op: usize) {

}

/// d3d12_command_list_discard_attachment - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_discard_attachment(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, subresources: *mut core::ffi::c_void) {

}

/// d3d12_command_list_discard_attachment_barrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_discard_attachment_barrier(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, subresources: *mut core::ffi::c_void, is_bound: usize) {

}

/// d3d12_command_list_track_resource_usage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_track_resource_usage(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, perform_initial_transition: usize) {

}

/// d3d12_command_list_update_subresource_data - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_subresource_data(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, subresource: usize) {

}

/// d3d12_command_list_flush_subresource_updates - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_flush_subresource_updates(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_emit_render_pass_transition - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_emit_render_pass_transition(list: *mut core::ffi::c_void, mode: usize) {

}

/// d3d12_command_list_begin_active_query - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_begin_active_query(list: *mut core::ffi::c_void, query: *mut core::ffi::c_void) {

}

/// d3d12_command_list_end_active_query - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_end_active_query(list: *mut core::ffi::c_void, query: *mut core::ffi::c_void) {

}

/// d3d12_command_list_reset_active_query - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_active_query(list: *mut core::ffi::c_void, query: *mut core::ffi::c_void) {

}

/// d3d12_command_list_enable_query - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_enable_query(list: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, index: u32, arg3: usize) -> usize {
    0
}

/// d3d12_command_list_disable_query - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_disable_query(list: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, index: u32) -> usize {
    0
}

/// d3d12_command_list_handle_active_queries - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_handle_active_queries(list: *mut core::ffi::c_void, end: usize) {

}

/// d3d12_command_list_gather_pending_queries - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_gather_pending_queries(list: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_fuse_attachment_clear - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_fuse_attachment_clear(list: *mut core::ffi::c_void, attachment: *mut core::ffi::c_void, stencil_attachment: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, view: *mut core::ffi::c_void, aspect_mask: usize, batch: *mut core::ffi::c_void) {

}

/// d3d12_command_list_resource_overlaps_attachment - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_resource_overlaps_attachment(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, subresources: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_flush_discards - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_flush_discards(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, subresources: *mut core::ffi::c_void) {

}

/// d3d12_command_list_flush_clears - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_flush_clears(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, subresources: *mut core::ffi::c_void) {

}

/// d3d12_command_list_flush_clears_for_rendering - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_flush_clears_for_rendering(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_copy_render_pass_suspend_resume_compat - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_copy_render_pass_suspend_resume_compat(list: *mut core::ffi::c_void, compat: *mut core::ffi::c_void) {

}

/// d3d12_command_list_reset_render_pass_load - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_render_pass_load(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_reset_render_pass_store_resolve - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_render_pass_store_resolve(list: *mut core::ffi::c_void, complete: usize) {

}

/// d3d12_command_list_end_rendering - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_end_rendering(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_add_transition - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_add_transition(list: *mut core::ffi::c_void, transition: *mut core::ffi::c_void) {

}

/// d3d12_command_list_track_query_heap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_track_query_heap(list: *mut core::ffi::c_void, heap: *mut core::ffi::c_void) {

}

/// d3d12_command_list_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_GetType - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_GetType(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_batch_reset_query_pools - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_batch_reset_query_pools(list: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_command_list_build_init_commands - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_build_init_commands(list: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_command_list_Close - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_Close(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_insert_query_range - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_insert_query_range(list: *mut core::ffi::c_void, arg1: *mut usize, vk_pool: usize, index: u32, count: u32, flags: u32) {

}

/// d3d12_command_list_reset_rtv_resolves - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_rtv_resolves(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_reset_api_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_api_state(list: *mut core::ffi::c_void, initial_pipeline_state: *mut core::ffi::c_void) {

}

/// d3d12_command_list_reset_internal_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_internal_state(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_reset_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_state(list: *mut core::ffi::c_void, initial_pipeline_state: *mut core::ffi::c_void) {

}

/// d3d12_command_list_Reset - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_Reset(iface: *mut core::ffi::c_void, allocator: *mut core::ffi::c_void, initial_pipeline_state: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_ClearState - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ClearState(iface: *mut core::ffi::c_void, pipeline_state: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_has_depth_stencil_view - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_has_depth_stencil_view(list: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_update_compute_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_compute_pipeline(list: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_set_root_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_root_signature(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void, root_signature: *mut core::ffi::c_void) {

}

/// d3d12_command_list_update_raygen_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_raygen_pipeline(list: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_check_vbo_alignment - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_check_vbo_alignment(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_update_graphics_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_graphics_pipeline(list: *mut core::ffi::c_void, pipeline_type: usize) -> usize {
    0
}

/// d3d12_command_list_update_descriptor_table_offsets - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_descriptor_table_offsets(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void, layout: usize, push_stages: usize) {

}

/// d3d12_command_list_update_descriptor_heaps - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_descriptor_heaps(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void, vk_bind_point: usize, layout: usize) {

}

/// d3d12_command_list_update_static_samplers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_static_samplers(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void, vk_bind_point: usize, layout: usize) {

}

/// d3d12_command_list_fetch_root_descriptor_vas - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_fetch_root_descriptor_vas(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void, dst_data: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_command_list_update_root_descriptors - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_root_descriptors(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void, vk_bind_point: usize, layout: usize, push_stages: usize, root_signature_flags: u32) {

}

/// d3d12_command_list_update_hoisted_descriptors - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_hoisted_descriptors(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void) {

}

/// d3d12_command_list_check_pre_compute_barrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_check_pre_compute_barrier(list: *mut core::ffi::c_void, vk_dst_stage: usize) {

}

/// d3d12_command_list_update_compute_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_compute_state(list: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_update_raygen_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_raygen_state(list: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_update_dynamic_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_dynamic_state(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_promote_dsv_layout - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_promote_dsv_layout(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_fixup_null_xfb_buffers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_fixup_null_xfb_buffers(list: *mut core::ffi::c_void, count: u32) -> usize {
    0
}

/// d3d12_command_list_render_pass_suspend_resume_avoids_fixup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_render_pass_suspend_resume_avoids_fixup(first: *mut core::ffi::c_void, second: *mut core::ffi::c_void, hazard_queries: usize, hoistable_post_indirect: usize) -> usize {
    0
}

/// d3d12_command_list_begin_rendering - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_begin_rendering(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_begin_render_pass - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_begin_render_pass(list: *mut core::ffi::c_void, pipeline_type: usize) -> usize {
    0
}

/// d3d12_command_list_check_index_buffer_strip_cut_value - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_check_index_buffer_strip_cut_value(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_emit_multi_dispatch_indirect_count - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_emit_multi_dispatch_indirect_count(list: *mut core::ffi::c_void, indirect_args: usize, stride: u32, max_commands: u32, count_arg: usize, scratch: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_emit_predicated_command - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_emit_predicated_command(list: *mut core::ffi::c_void, command_type: usize, indirect_args: usize, direct_args: *mut core::ffi::c_void, scratch: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_update_index_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_index_buffer(list: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_Dispatch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_Dispatch(iface: *mut core::ffi::c_void, x: u32, y: u32, z: u32) -> usize {
    0
}

/// d3d12_command_list_CopyBufferRegion - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyBufferRegion(iface: *mut core::ffi::c_void, dst: *mut core::ffi::c_void, dst_offset: usize, src: *mut core::ffi::c_void, src_offset: usize, byte_count: usize) -> usize {
    0
}

/// d3d12_command_list_transition_image_layout_with_global_memory_barrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_transition_image_layout_with_global_memory_barrier(list: *mut core::ffi::c_void, batch: *mut core::ffi::c_void, vk_image: usize, vk_subresource: *mut core::ffi::c_void, src_stages: usize, src_access: usize, old_layout: usize, dst_stages: usize, dst_access: usize, new_layout: usize, global_src_access: usize, global_dst_access: usize) {

}

/// d3d12_command_list_check_ds_color_copy_compatibility - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_check_ds_color_copy_compatibility(list: *mut core::ffi::c_void, dst_format: *mut core::ffi::c_void, src_format: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_copy_image_transition_images - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_copy_image_transition_images(list: *mut core::ffi::c_void, batch: *mut core::ffi::c_void, dst_resource: *mut core::ffi::c_void, dst_format: *mut core::ffi::c_void, src_resource: *mut core::ffi::c_void, src_format: *mut core::ffi::c_void, region: *mut core::ffi::c_void, writes_full_subresource: usize, overlapping_subresource: usize, vk_availability_stages: usize) {

}

/// d3d12_command_list_copy_image - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_copy_image(list: *mut core::ffi::c_void, batch: *mut core::ffi::c_void, dst_resource: *mut core::ffi::c_void, dst_format: *mut core::ffi::c_void, src_resource: *mut core::ffi::c_void, src_format: *mut core::ffi::c_void, region: *mut core::ffi::c_void, overlapping_subresource: usize, outside_vk_stages: usize, outside_vk_access: usize) {

}

/// validate_d3d12_box - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn validate_d3d12_box(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_init_copy_texture_region - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_init_copy_texture_region(list: *mut core::ffi::c_void, dst: *mut core::ffi::c_void, dst_x: u32, dst_y: u32, dst_z: u32, src: *mut core::ffi::c_void, src_box: *mut core::ffi::c_void, out: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_merge_copy_tracking - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_merge_copy_tracking(list: *mut core::ffi::c_void, batch: *mut core::ffi::c_void) {

}

/// d3d12_command_list_copy_requires_complex_barrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_copy_requires_complex_barrier(list: *mut core::ffi::c_void, info: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_before_copy_texture_region - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_before_copy_texture_region(list: *mut core::ffi::c_void, batch: *mut core::ffi::c_void, info: *mut core::ffi::c_void) {

}

/// d3d12_command_list_copy_texture_region - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_copy_texture_region(list: *mut core::ffi::c_void, batch: *mut core::ffi::c_void, info: *mut core::ffi::c_void) {

}

/// d3d12_command_list_CopyTextureRegion - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyTextureRegion(iface: *mut core::ffi::c_void, dst: *mut core::ffi::c_void, dst_x: u32, dst_y: u32, dst_z: u32, src: *mut core::ffi::c_void, src_box: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_CopyResource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyResource(iface: *mut core::ffi::c_void, dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_CopyTiles - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyTiles(iface: *mut core::ffi::c_void, tiled_resource: *mut core::ffi::c_void, region_coord: *mut core::ffi::c_void, region_size: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, buffer_offset: usize, flags: usize) -> usize {
    0
}

/// d3d12_resource_view_format_is_compatible - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_view_format_is_compatible(resource: *mut core::ffi::c_void, format: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_get_resolve_barrier_for_dst_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_get_resolve_barrier_for_dst_resource(resource: *mut core::ffi::c_void, region: *mut core::ffi::c_void, path: usize, post_resolve: usize, outside_layout: usize, outside_stages: usize, outside_access: usize, barrier: *mut core::ffi::c_void) {

}

/// d3d12_get_resolve_barrier_for_src_resource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_get_resolve_barrier_for_src_resource(resource: *mut core::ffi::c_void, region: *mut core::ffi::c_void, path: usize, post_resolve: usize, outside_layout: usize, outside_stages: usize, outside_access: usize, barrier: *mut core::ffi::c_void) {

}

/// d3d12_command_list_execute_resolve - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_execute_resolve(list: *mut core::ffi::c_void, dst_resource: *mut core::ffi::c_void, src_resource: *mut core::ffi::c_void, region_count: u32, regions: *mut core::ffi::c_void, format: usize, mode: usize, path: usize) {

}

/// d3d12_command_list_resolve_subresource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_resolve_subresource(list: *mut core::ffi::c_void, dst_resource: *mut core::ffi::c_void, dst_subresource_idx: u32, src_resource: *mut core::ffi::c_void, resolve: *mut core::ffi::c_void, format: usize, mode: usize) {

}

/// d3d12_command_list_ResolveSubresource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ResolveSubresource(iface: *mut core::ffi::c_void, dst: *mut core::ffi::c_void, dst_sub_resource_idx: u32, src: *mut core::ffi::c_void, src_sub_resource_idx: u32, format: usize) -> usize {
    0
}

/// d3d12_command_list_RSSetViewports - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetViewports(iface: *mut core::ffi::c_void, viewport_count: u32, viewports: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_RSSetScissorRects - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetScissorRects(iface: *mut core::ffi::c_void, rect_count: u32, rects: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_OMSetBlendFactor - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetBlendFactor(iface: *mut core::ffi::c_void, blend_factor4: f32) -> usize {
    0
}

/// d3d12_command_list_OMSetStencilRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetStencilRef(iface: *mut core::ffi::c_void, stencil_ref: u32) -> usize {
    0
}

/// d3d12_command_list_SetPipelineState - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPipelineState(iface: *mut core::ffi::c_void, pipeline_state: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_merge_copy_tracking_transition - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_merge_copy_tracking_transition(list: *mut core::ffi::c_void, transition: *mut core::ffi::c_void, batch: *mut core::ffi::c_void) {

}

/// d3d12_command_list_ResourceBarrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ResourceBarrier(iface: *mut core::ffi::c_void, barrier_count: u32, barriers: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_ExecuteBundle - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ExecuteBundle(iface: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_set_descriptor_heaps_buffers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_descriptor_heaps_buffers(list: *mut core::ffi::c_void, heap_count: u32, heaps: *mut *mut core::ffi::c_void) {

}

/// d3d12_command_list_set_descriptor_heaps_sets - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_descriptor_heaps_sets(list: *mut core::ffi::c_void, heap_count: u32, heaps: *mut *mut core::ffi::c_void) {

}

/// d3d12_command_list_SetDescriptorHeaps - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetDescriptorHeaps(iface: *mut core::ffi::c_void, heap_count: u32, heaps: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_SetComputeRootSignature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootSignature(iface: *mut core::ffi::c_void, root_signature: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_SetGraphicsRootSignature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootSignature(iface: *mut core::ffi::c_void, root_signature: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_set_descriptor_table_embedded - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_descriptor_table_embedded(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void, index: u32, base_descriptor: usize, cbv_srv_uav_size_log2: u32, sampler_size_log2: u32) {

}

/// d3d12_command_list_set_descriptor_table - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_descriptor_table(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void, index: u32, base_descriptor: usize) {

}

/// d3d12_command_list_SetComputeRootDescriptorTable_embedded_64_16 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootDescriptorTable_embedded_64_16(iface: *mut core::ffi::c_void, root_parameter_index: u32, base_descriptor: usize) -> usize {
    0
}

/// d3d12_command_list_SetGraphicsRootDescriptorTable_embedded_64_16 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootDescriptorTable_embedded_64_16(iface: *mut core::ffi::c_void, root_parameter_index: u32, base_descriptor: usize) -> usize {
    0
}

/// d3d12_command_list_SetComputeRootDescriptorTable_embedded_32_16 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootDescriptorTable_embedded_32_16(iface: *mut core::ffi::c_void, root_parameter_index: u32, base_descriptor: usize) -> usize {
    0
}

/// d3d12_command_list_SetGraphicsRootDescriptorTable_embedded_32_16 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootDescriptorTable_embedded_32_16(iface: *mut core::ffi::c_void, root_parameter_index: u32, base_descriptor: usize) -> usize {
    0
}

/// d3d12_command_list_SetComputeRootDescriptorTable_embedded_default - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootDescriptorTable_embedded_default(iface: *mut core::ffi::c_void, root_parameter_index: u32, base_descriptor: usize) -> usize {
    0
}

/// d3d12_command_list_SetGraphicsRootDescriptorTable_embedded_default - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootDescriptorTable_embedded_default(iface: *mut core::ffi::c_void, root_parameter_index: u32, base_descriptor: usize) -> usize {
    0
}

/// d3d12_command_list_SetComputeRootDescriptorTable_default - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootDescriptorTable_default(iface: *mut core::ffi::c_void, root_parameter_index: u32, base_descriptor: usize) -> usize {
    0
}

/// d3d12_command_list_SetGraphicsRootDescriptorTable_default - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootDescriptorTable_default(iface: *mut core::ffi::c_void, root_parameter_index: u32, base_descriptor: usize) -> usize {
    0
}

/// d3d12_command_list_set_push_descriptor_info - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_push_descriptor_info(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void, index: u32, gpu_address: usize) {

}

/// d3d12_command_list_set_root_descriptor_va - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_root_descriptor_va(list: *mut core::ffi::c_void, descriptor: *mut core::ffi::c_void, gpu_address: usize) {

}

/// d3d12_command_list_set_root_descriptor - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_root_descriptor(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void, index: u32, gpu_address: usize) {

}

/// d3d12_command_list_SetComputeRootShaderResourceView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootShaderResourceView(iface: *mut core::ffi::c_void, root_parameter_index: u32, address: usize) -> usize {
    0
}

/// d3d12_command_list_SetGraphicsRootShaderResourceView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootShaderResourceView(iface: *mut core::ffi::c_void, root_parameter_index: u32, address: usize) -> usize {
    0
}

/// d3d12_command_list_SetComputeRootUnorderedAccessView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootUnorderedAccessView(iface: *mut core::ffi::c_void, root_parameter_index: u32, address: usize) -> usize {
    0
}

/// d3d12_command_list_SetGraphicsRootUnorderedAccessView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootUnorderedAccessView(iface: *mut core::ffi::c_void, root_parameter_index: u32, address: usize) -> usize {
    0
}

/// d3d12_command_list_IASetIndexBuffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetIndexBuffer(iface: *mut core::ffi::c_void, view: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_IASetVertexBuffers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetVertexBuffers(iface: *mut core::ffi::c_void, start_slot: u32, view_count: u32, views: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_SOSetTargets - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SOSetTargets(iface: *mut core::ffi::c_void, start_slot: u32, view_count: u32, views: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_recompute_fb_size - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_recompute_fb_size(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_invalidate_ds_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_invalidate_ds_state(list: *mut core::ffi::c_void, prev_dsv_format: usize) {

}

/// d3d12_command_list_filter_set_render_targets - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_filter_set_render_targets(list: *mut core::ffi::c_void, render_target_descriptor_count: u32, render_target_descriptors: *mut core::ffi::c_void, single_descriptor_handle: i32, depth_stencil_descriptor: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_OMSetRenderTargets - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetRenderTargets(iface: *mut core::ffi::c_void, render_target_descriptor_count: u32, render_target_descriptors: *mut core::ffi::c_void, single_descriptor_handle: i32, depth_stencil_descriptor: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_rect_fully_covers_region - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_rect_fully_covers_region(a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_defer_attachment_clear - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_defer_attachment_clear(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, view: *mut core::ffi::c_void, clear_aspects: usize, clear_value: *mut core::ffi::c_void) {

}

/// d3d12_command_list_clear_attachment - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_clear_attachment(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, view: *mut core::ffi::c_void, clear_aspects: usize, clear_value: *mut core::ffi::c_void, rect_count: u32, rects: *mut core::ffi::c_void) {

}

/// d3d12_command_list_ClearDepthStencilView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ClearDepthStencilView(iface: *mut core::ffi::c_void, dsv: usize, flags: usize, depth: f32, stencil: usize, rect_count: u32, rects: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_ClearRenderTargetView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ClearRenderTargetView(iface: *mut core::ffi::c_void, rtv: usize, color4: f32, rect_count: u32, rects: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_clear_uav - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_clear_uav(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, args: *mut core::ffi::c_void, clear_color: *mut core::ffi::c_void, rect_count: u32, rects: *mut core::ffi::c_void) {

}

/// d3d12_command_list_clear_uav_with_copy - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_clear_uav_with_copy(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, args: *mut core::ffi::c_void, clear_value: *mut core::ffi::c_void, format: *mut core::ffi::c_void, rect_count: u32, rects: *mut core::ffi::c_void) {

}

/// d3d12_command_list_ClearUnorderedAccessViewUint - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ClearUnorderedAccessViewUint(iface: *mut core::ffi::c_void, gpu_handle: usize, cpu_handle: usize, resource: *mut core::ffi::c_void, values4: u32, rect_count: u32, rects: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_ClearUnorderedAccessViewFloat - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ClearUnorderedAccessViewFloat(iface: *mut core::ffi::c_void, gpu_handle: usize, cpu_handle: usize, resource: *mut core::ffi::c_void, values4: f32, rect_count: u32, rects: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_is_subresource_bound_as_rtv_dsv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_is_subresource_bound_as_rtv_dsv(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, subresource: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_DiscardResource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DiscardResource(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, region: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_resolve_binary_occlusion_queries - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_resolve_binary_occlusion_queries(list: *mut core::ffi::c_void, src_va: usize, dst_va: usize, count: u32) {

}

/// d3d12_command_list_is_query_resolve_pending - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_is_query_resolve_pending(list: *mut core::ffi::c_void, query_heap: *mut core::ffi::c_void, query_index: u32) -> usize {
    0
}

/// d3d12_command_list_add_query_lookup_mask - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_add_query_lookup_mask(list: *mut core::ffi::c_void, query_heap: *mut core::ffi::c_void, bucket: u32, query_mask: u64) {

}

/// d3d12_command_list_add_query_lookup_range - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_add_query_lookup_range(list: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) {

}

/// d3d12_command_list_execute_query_resolve - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_execute_query_resolve(list: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) {

}

/// d3d12_command_list_add_query_resolve - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_add_query_resolve(list: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) {

}

/// d3d12_query_type_is_scoped - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_type_is_scoped(arg0: usize) -> usize {
    0
}

/// d3d12_command_list_BeginQuery - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_BeginQuery(iface: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, arg2: usize, index: u32) -> usize {
    0
}

/// d3d12_command_list_EndQuery - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_EndQuery(iface: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, arg2: usize, index: u32) -> usize {
    0
}

/// d3d12_command_list_ResolveQueryData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ResolveQueryData(iface: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, arg2: usize, start_index: u32, query_count: u32, dst_buffer: *mut core::ffi::c_void, aligned_dst_buffer_offset: usize) -> usize {
    0
}

/// d3d12_command_list_SetPredication - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPredication(iface: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, aligned_buffer_offset: usize, operation: usize) -> usize {
    0
}

/// d3d12_command_list_SetMarker - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetMarker(iface: *mut core::ffi::c_void, metadata: u32, data: *mut core::ffi::c_void, size: u32) -> usize {
    0
}

/// d3d12_command_list_BeginEvent - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_BeginEvent(iface: *mut core::ffi::c_void, metadata: u32, data: *mut core::ffi::c_void, size: u32) -> usize {
    0
}

/// d3d12_command_list_EndEvent - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_EndEvent(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_clear_signature_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_clear_signature_state(list: *mut core::ffi::c_void, signature: *mut core::ffi::c_void) {

}

/// d3d12_command_list_execute_indirect_state_template_dgc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_execute_indirect_state_template_dgc(list: *mut core::ffi::c_void, signature: *mut core::ffi::c_void, max_command_count: u32, arg_buffer: *mut core::ffi::c_void, arg_buffer_offset: usize, count_buffer: *mut core::ffi::c_void, count_buffer_offset: usize, dgc_mode: usize, preprocess_va: usize, preprocess_size: u64) {

}

/// d3d12_command_list_ExecuteIndirect - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ExecuteIndirect(iface: *mut core::ffi::c_void, command_signature: *mut core::ffi::c_void, max_command_count: u32, arg_buffer: *mut core::ffi::c_void, arg_buffer_offset: usize, count_buffer: *mut core::ffi::c_void, count_buffer_offset: usize) -> usize {
    0
}

/// d3d12_command_list_OMSetDepthBounds - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetDepthBounds(iface: *mut core::ffi::c_void, min: f32, max: f32) -> usize {
    0
}

/// d3d12_command_list_encode_sampler_feedback - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_encode_sampler_feedback(list: *mut core::ffi::c_void, dst: *mut core::ffi::c_void, dst_subresource_index: u32, dst_x: u32, dst_y: u32, src: *mut core::ffi::c_void, src_subresource_index: u32, src_rect: *mut core::ffi::c_void) {

}

/// d3d12_command_list_decode_sampler_feedback - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_decode_sampler_feedback(list: *mut core::ffi::c_void, dst: *mut core::ffi::c_void, dst_subresource_index: u32, dst_x: u32, dst_y: u32, src: *mut core::ffi::c_void, src_subresource_index: u32, src_rect: *mut core::ffi::c_void) {

}

/// d3d12_command_list_ResolveSubresourceRegion - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ResolveSubresourceRegion(iface: *mut core::ffi::c_void, dst: *mut core::ffi::c_void, dst_sub_resource_idx: u32, dst_x: u32, dst_y: u32, src: *mut core::ffi::c_void, src_sub_resource_idx: u32, src_rect: *mut core::ffi::c_void, format: usize, mode: usize) -> usize {
    0
}

/// d3d12_command_list_SetProtectedResourceSession - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetProtectedResourceSession(iface: *mut core::ffi::c_void, protected_session: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_load_render_pass_rtv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_load_render_pass_rtv(list: *mut core::ffi::c_void, rtv_info: *mut core::ffi::c_void, rt: *mut core::ffi::c_void) {

}

/// d3d12_command_list_load_render_pass_dsv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_load_render_pass_dsv(list: *mut core::ffi::c_void, dsv_info: *mut core::ffi::c_void, ds: *mut core::ffi::c_void) {

}

/// d3d12_command_list_resolve_render_pass_attachments - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_resolve_render_pass_attachments(list: *mut core::ffi::c_void) {

}

/// d3d12_command_list_deduce_attachment_resolve - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_deduce_attachment_resolve(list: *mut core::ffi::c_void, rtv: *mut core::ffi::c_void, args: *mut core::ffi::c_void, aspect: usize, requires_independent_none: usize, requires_independent: usize) -> usize {
    0
}

/// d3d12_render_pass_beginning_access_binds_to_rasterizer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_render_pass_beginning_access_binds_to_rasterizer(access: usize, flags: usize, aspect: usize) -> usize {
    0
}

/// d3d12_command_list_add_render_pass_resolve - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_add_render_pass_resolve(list: *mut core::ffi::c_void, rtv: *mut core::ffi::c_void, beginning: *mut core::ffi::c_void, args: *mut core::ffi::c_void, aspect: usize, flags: usize, requires_independent_none: usize, requires_independent: usize) -> usize {
    0
}

/// d3d12_command_list_setup_render_pass_attachment_resolve - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_setup_render_pass_attachment_resolve(list: *mut core::ffi::c_void, attachment: *mut core::ffi::c_void, resolve: *mut core::ffi::c_void, aspect_mask: usize) {

}

/// d3d12_command_list_BeginRenderPass - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_BeginRenderPass(iface: *mut core::ffi::c_void, rt_count: u32, render_targets: *mut core::ffi::c_void, depth_stencil: *mut core::ffi::c_void, flags: usize) -> usize {
    0
}

/// d3d12_command_list_EndRenderPass - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_EndRenderPass(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_InitializeMetaCommand - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_InitializeMetaCommand(iface: *mut core::ffi::c_void, meta_command: *mut core::ffi::c_void, parameter_data: *mut core::ffi::c_void, parameter_size: usize) -> usize {
    0
}

/// d3d12_command_list_ExecuteMetaCommand - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ExecuteMetaCommand(iface: *mut core::ffi::c_void, meta_command: *mut core::ffi::c_void, parameter_data: *mut core::ffi::c_void, parameter_size: usize) -> usize {
    0
}

/// d3d12_command_list_build_raytracing_opacity_micromap_array - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_build_raytracing_opacity_micromap_array(list: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, num_postbuild_info_descs: u32, postbuild_info_descs: *mut core::ffi::c_void) {

}

/// d3d12_command_list_BuildRaytracingAccelerationStructure - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_BuildRaytracingAccelerationStructure(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, num_postbuild_info_descs: u32, postbuild_info_descs: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_EmitRaytracingAccelerationStructurePostbuildInfo - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_EmitRaytracingAccelerationStructurePostbuildInfo(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, num_acceleration_structures: u32, src_data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_CopyRaytracingAccelerationStructure - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyRaytracingAccelerationStructure(iface: *mut core::ffi::c_void, dst_data: usize, src_data: usize, mode: usize) -> usize {
    0
}

/// d3d12_command_list_SetPipelineState1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPipelineState1(iface: *mut core::ffi::c_void, state_object: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_DispatchRays - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DispatchRays(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_RSSetShadingRate - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetShadingRate(iface: *mut core::ffi::c_void, base: usize, combiners: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_RSSetShadingRateImage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetShadingRateImage(iface: *mut core::ffi::c_void, image: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_DispatchMesh - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DispatchMesh(iface: *mut core::ffi::c_void, x: u32, y: u32, z: u32) -> usize {
    0
}

/// d3d12_command_list_process_enhanced_barrier_global - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_process_enhanced_barrier_global(list: *mut core::ffi::c_void, batch: *mut core::ffi::c_void, barrier: *mut core::ffi::c_void) {

}

/// d3d12_command_list_process_enhanced_barrier_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_process_enhanced_barrier_buffer(list: *mut core::ffi::c_void, batch: *mut core::ffi::c_void, barrier: *mut core::ffi::c_void) {

}

/// d3d12_command_list_process_enhanced_barrier_texture - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_process_enhanced_barrier_texture(list: *mut core::ffi::c_void, batch: *mut core::ffi::c_void, barrier: *mut core::ffi::c_void) {

}

/// d3d12_command_list_barrier_is_noop - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_barrier_is_noop(list: *mut core::ffi::c_void, NumBarrierGroups: usize, pBarrierGroups: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_Barrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_Barrier(iface: *mut core::ffi::c_void, NumBarrierGroups: usize, pBarrierGroups: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_RSSetDepthBias - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetDepthBias(iface: *mut core::ffi::c_void, DepthBias: f32, DepthBiasClamp: f32, SlopeScaledDepthBias: f32) -> usize {
    0
}

/// d3d12_command_list_IASetIndexBufferStripCutValue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetIndexBufferStripCutValue(iface: *mut core::ffi::c_void, IBStripCutValue: usize) -> usize {
    0
}

/// d3d12_command_list_SetProgram - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetProgram(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_DispatchGraph - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DispatchGraph(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_init_attachment_info - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_init_attachment_info(attachment_info: *mut core::ffi::c_void) {

}

/// d3d12_command_list_init_rendering_info - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_init_rendering_info(device: *mut core::ffi::c_void, rendering_info: *mut core::ffi::c_void) {

}

/// d3d12_command_list_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_init(list: *mut core::ffi::c_void, device: *mut core::ffi::c_void, arg2: usize) -> i32 {
    0
}

/// d3d12_command_queue_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_queue_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_queue_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_queue_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_queue_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_queue_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_queue_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_queue_UpdateTileMappings - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_UpdateTileMappings(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, region_count: u32, region_coords: *mut core::ffi::c_void, region_sizes: *mut core::ffi::c_void, heap: *mut core::ffi::c_void, range_count: u32, range_flags: *mut core::ffi::c_void, heap_range_offsets: *mut u32, range_tile_counts: *mut u32, flags: usize) -> usize {
    0
}

/// d3d12_command_queue_CopyTileMappings - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_CopyTileMappings(iface: *mut core::ffi::c_void, dst_resource: *mut core::ffi::c_void, dst_region_start_coordinate: *mut core::ffi::c_void, src_resource: *mut core::ffi::c_void, src_region_start_coordinate: *mut core::ffi::c_void, region_size: *mut core::ffi::c_void, flags: usize) -> usize {
    0
}

/// d3d12_command_queue_ExecuteCommandLists - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_ExecuteCommandLists(iface: *mut core::ffi::c_void, command_list_count: u32, command_lists: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_queue_SetMarker - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_SetMarker(iface: *mut core::ffi::c_void, metadata: u32, data: *mut core::ffi::c_void, size: u32) -> usize {
    0
}

/// d3d12_command_queue_BeginEvent - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_BeginEvent(iface: *mut core::ffi::c_void, metadata: u32, data: *mut core::ffi::c_void, size: u32) -> usize {
    0
}

/// d3d12_command_queue_EndEvent - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_EndEvent(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_queue_Signal - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_Signal(iface: *mut core::ffi::c_void, fence_iface: *mut core::ffi::c_void, value: usize) -> usize {
    0
}

/// d3d12_command_queue_Wait - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_Wait(iface: *mut core::ffi::c_void, fence_iface: *mut core::ffi::c_void, value: usize) -> usize {
    0
}

/// d3d12_command_queue_GetTimestampFrequency - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_GetTimestampFrequency(iface: *mut core::ffi::c_void, frequency: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_queue_GetDesc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_GetDesc(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_command_queue_destroy_serializing_semaphore - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_destroy_serializing_semaphore(command_queue: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_reset_fence_waits - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_reset_fence_waits(command_queue: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_push_fence_waits_to_worker - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_push_fence_waits_to_worker(command_queue: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_add_wait_semaphores - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_add_wait_semaphores(command_queue: *mut core::ffi::c_void, wait_count: u32, waits: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_add_wait - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_add_wait(command_queue: *mut core::ffi::c_void, fence: *mut core::ffi::c_void, fence_value: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_eliminate_completed_waits - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_eliminate_completed_waits(command_queue: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_finalize_waits - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_finalize_waits(command_queue: *mut core::ffi::c_void, submit_vr: i32) {

}

/// d3d12_command_queue_flush_waiters - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_flush_waiters(command_queue: *mut core::ffi::c_void, wait_flags: u32) {

}

/// d3d12_command_queue_wait_idle - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_wait_idle(command_queue: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_wait - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_wait(command_queue: *mut core::ffi::c_void, fence: *mut core::ffi::c_void, value: usize, ticket: usize) {

}

/// d3d12_command_queue_signal - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_signal(command_queue: *mut core::ffi::c_void, fence: *mut core::ffi::c_void, value: usize) {

}

/// d3d12_command_queue_wait_shared - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_wait_shared(command_queue: *mut core::ffi::c_void, fence: *mut core::ffi::c_void, value: u64) {

}

/// d3d12_command_queue_signal_shared - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_signal_shared(command_queue: *mut core::ffi::c_void, fence: *mut core::ffi::c_void, value: usize) {

}

/// d3d12_command_queue_transition_pool_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_transition_pool_init(pool: *mut core::ffi::c_void, queue: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_command_queue_transition_pool_wait - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_transition_pool_wait(pool: *mut core::ffi::c_void, device: *mut core::ffi::c_void, value: u64) {

}

/// d3d12_command_queue_transition_pool_deinit - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_transition_pool_deinit(pool: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_init_query_heap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_init_query_heap(device: *mut core::ffi::c_void, vk_cmd_buffer: usize, heap: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_transition_pool_build - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_transition_pool_build(pool: *mut core::ffi::c_void, device: *mut core::ffi::c_void, transitions: *mut core::ffi::c_void, count: usize, fallback: usize, vk_cmd_buffer: *mut core::ffi::c_void, timeline_value: *mut u64) {

}

/// d3d12_command_queue_wait_staggered_submission - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_wait_staggered_submission(command_queue: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_execute - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_execute(command_queue: *mut core::ffi::c_void, exec: *mut core::ffi::c_void, transition_cmd: *mut core::ffi::c_void, transition_semaphore: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_flush_bind_sparse - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_flush_bind_sparse(command_queue: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_register_sparse_hazard - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_register_sparse_hazard(command_queue: *mut core::ffi::c_void, dst_resource: *mut core::ffi::c_void, binds: *mut core::ffi::c_void, count: u32) {

}

/// d3d12_command_queue_bind_sparse - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_bind_sparse(command_queue: *mut core::ffi::c_void, mode: usize, dst_resource: *mut core::ffi::c_void, src_resource: *mut core::ffi::c_void, count: u32, bind_infos: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_acquire_serialized - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_acquire_serialized(queue: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_release_serialized - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_release_serialized(queue: *mut core::ffi::c_void) {

}

/// d3d12_command_queue_exec_submit_needs_fallback_queue - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_exec_submit_needs_fallback_queue(queue: *mut core::ffi::c_void, execute: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_queue_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_init(queue: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, family_info: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_command_signature_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, out: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_signature_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_signature_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_cleanup(signature: *mut core::ffi::c_void) {

}

/// d3d12_command_signature_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_signature_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_signature_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_signature_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_signature_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_signature_init_patch_commands_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_init_patch_commands_buffer(signature: *mut core::ffi::c_void, device: *mut core::ffi::c_void, commands: *mut core::ffi::c_void, command_count: usize) -> i32 {
    0
}

/// d3d12_command_signature_init_indirect_commands_layout - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_init_indirect_commands_layout(signature: *mut core::ffi::c_void, root_signature: *mut core::ffi::c_void, device: *mut core::ffi::c_void, tokens: *mut core::ffi::c_void, token_count: u32, stream_stride: u32) -> i32 {
    0
}

/// d3d12_command_signature_init_state_template_dgc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_init_state_template_dgc(signature: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, root_signature: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_determine_additional_shading_rates_supported - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_determine_additional_shading_rates_supported(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_init_vendor_hacks - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_init_vendor_hacks(device: *mut core::ffi::c_void) {

}

/// d3d12_device_cleanup_vendor_hacks - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_cleanup_vendor_hacks(device: *mut core::ffi::c_void) {

}

/// d3d12_device_init_workarounds - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_init_workarounds(device: *mut core::ffi::c_void) {

}

/// d3d12_device_destroy_scratch_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_destroy_scratch_buffer(device: *mut core::ffi::c_void, scratch: *mut core::ffi::c_void) {

}

/// d3d12_device_destroy_query_pool - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_destroy_query_pool(device: *mut core::ffi::c_void, pool: *mut core::ffi::c_void) {

}

/// d3d12_low_latency_device_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_amd_ext_anti_lag_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_amd_ext_anti_lag_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_destroy - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_destroy(device: *mut core::ffi::c_void) {

}

/// d3d12_device_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_GetNodeCount - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetNodeCount(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_check_multisample_quality_levels - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_check_multisample_quality_levels(device: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_get_format_displayable_features - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_format_displayable_features(device: *mut core::ffi::c_void, format: usize) -> u32 {
    0
}

/// d3d12_format_is_streamout_compatible - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_format_is_streamout_compatible(format: usize) -> usize {
    0
}

/// d3d12_device_get_format_support - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_format_support(device: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_CheckFeatureSupport - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CheckFeatureSupport(iface: *mut core::ffi::c_void, feature: usize, feature_data: *mut core::ffi::c_void, feature_data_size: u32) -> usize {
    0
}

/// d3d12_device_GetDescriptorHandleIncrementSize - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetDescriptorHandleIncrementSize(iface: *mut core::ffi::c_void, descriptor_heap_type: usize) -> usize {
    0
}

/// d3d12_device_get_descriptor_handle_increment_size - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_descriptor_handle_increment_size(arg0: usize, arg1: usize) -> usize {
    0
}

/// d3d12_advance_cpu_descriptor_handle - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_advance_cpu_descriptor_handle(handle: usize, increment: u32, units: u32) -> usize {
    0
}

/// d3d12_device_copy_descriptors_cbv_srv_uav_sampler - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_copy_descriptors_cbv_srv_uav_sampler(device: *mut core::ffi::c_void, dst: usize, src: usize, heap_type: usize, descriptor_count: u32) {

}

/// d3d12_device_copy_descriptors - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_copy_descriptors(device: *mut core::ffi::c_void, dst_descriptor_range_count: u32, dst_descriptor_range_offsets: *mut core::ffi::c_void, dst_descriptor_range_sizes: *mut u32, src_descriptor_range_count: u32, src_descriptor_range_offsets: *mut core::ffi::c_void, src_descriptor_range_sizes: *mut u32, descriptor_heap_type: usize) {

}

/// d3d12_device_CopyDescriptorsSimple_descriptor_buffer_16_16_4 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptorsSimple_descriptor_buffer_16_16_4(iface: *mut core::ffi::c_void, descriptor_count: u32, dst_descriptor_range_offset: usize, src_descriptor_range_offset: usize, descriptor_heap_type: usize) -> usize {
    0
}

/// d3d12_device_CopyDescriptorsSimple_descriptor_buffer_64_64_32 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptorsSimple_descriptor_buffer_64_64_32(iface: *mut core::ffi::c_void, descriptor_count: u32, dst_descriptor_range_offset: usize, src_descriptor_range_offset: usize, descriptor_heap_type: usize) -> usize {
    0
}

/// d3d12_device_CopyDescriptorsSimple_embedded_64_16_packed - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptorsSimple_embedded_64_16_packed(iface: *mut core::ffi::c_void, descriptor_count: u32, dst_descriptor_range_offset: usize, src_descriptor_range_offset: usize, descriptor_heap_type: usize) -> usize {
    0
}

/// d3d12_device_CopyDescriptorsSimple_embedded_32_16_planar - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptorsSimple_embedded_32_16_planar(iface: *mut core::ffi::c_void, descriptor_count: u32, dst_descriptor_range_offset: usize, src_descriptor_range_offset: usize, descriptor_heap_type: usize) -> usize {
    0
}

/// d3d12_device_CopyDescriptorsSimple_embedded_generic - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptorsSimple_embedded_generic(iface: *mut core::ffi::c_void, descriptor_count: u32, dst_descriptor_range_offset: usize, src_descriptor_range_offset: usize, descriptor_heap_type: usize) -> usize {
    0
}

/// d3d12_device_CopyDescriptorsSimple_default - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptorsSimple_default(iface: *mut core::ffi::c_void, descriptor_count: u32, dst_descriptor_range_offset: usize, src_descriptor_range_offset: usize, descriptor_heap_type: usize) -> usize {
    0
}

/// d3d12_device_GetCustomHeapProperties - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetCustomHeapProperties(iface: *mut core::ffi::c_void, heap_properties: *mut core::ffi::c_void, node_mask: u32, heap_type: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_device_OpenSharedHandle - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_OpenSharedHandle(iface: *mut core::ffi::c_void, handle: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_OpenSharedHandleByName - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_OpenSharedHandleByName(iface: *mut core::ffi::c_void, name: *mut u16, access: u32, handle: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_MakeResident - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_MakeResident(iface: *mut core::ffi::c_void, object_count: u32, objects: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_Evict - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_Evict(iface: *mut core::ffi::c_void, object_count: u32, objects: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_GetDeviceRemovedReason - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetDeviceRemovedReason(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_GetCopyableFootprints1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetCopyableFootprints1(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, first_sub_resource: u32, sub_resource_count: u32, base_offset: usize, layouts: *mut core::ffi::c_void, row_counts: *mut u32, row_sizes: *mut core::ffi::c_void, total_bytes: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_GetCopyableFootprints - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetCopyableFootprints(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, first_sub_resource: u32, sub_resource_count: u32, base_offset: usize, layouts: *mut core::ffi::c_void, row_counts: *mut u32, row_sizes: *mut core::ffi::c_void, total_bytes: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_GetResourceTiling - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetResourceTiling(iface: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, tile_count: *mut u32, packed_mip_info: *mut core::ffi::c_void, tile_shape: *mut core::ffi::c_void, tiling_count: *mut u32, first_tiling: u32, tilings: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_GetAdapterLuid - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetAdapterLuid(iface: *mut core::ffi::c_void, luid: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_device_SetEventOnMultipleFenceCompletion - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_SetEventOnMultipleFenceCompletion(iface: *mut core::ffi::c_void, fences: *mut *mut core::ffi::c_void, values: *mut core::ffi::c_void, fence_count: u32, flags: usize, event: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_SetResidencyPriority - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_SetResidencyPriority(iface: *mut core::ffi::c_void, object_count: u32, objects: *mut *mut core::ffi::c_void, priorities: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_OpenExistingHeapFromAddress - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_OpenExistingHeapFromAddress(iface: *mut core::ffi::c_void, address: *mut core::ffi::c_void, riid: usize, heap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_OpenExistingHeapFromFileMapping - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_OpenExistingHeapFromFileMapping(iface: *mut core::ffi::c_void, file_mapping: *mut core::ffi::c_void, riid: usize, heap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_RemoveDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_RemoveDevice(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_EnumerateMetaCommands - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_EnumerateMetaCommands(iface: *mut core::ffi::c_void, count: *mut u32, descs: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_EnumerateMetaCommandParameters - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_EnumerateMetaCommandParameters(iface: *mut core::ffi::c_void, command_id: usize, stage: usize, total_size: *mut u32, param_count: *mut u32, param_descs: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_get_raytracing_opacity_micromap_array_prebuild_info - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_raytracing_opacity_micromap_array_prebuild_info(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, info: *mut core::ffi::c_void) {

}

/// d3d12_device_GetRaytracingAccelerationStructurePrebuildInfo - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetRaytracingAccelerationStructurePrebuildInfo(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, info: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_AddToStateObject - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_AddToStateObject(iface: *mut core::ffi::c_void, addition: *mut core::ffi::c_void, parent_state: *mut core::ffi::c_void, riid: usize, new_state_object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_ShaderCacheControl - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_ShaderCacheControl(iface: *mut core::ffi::c_void, kinds: usize, control: usize) -> usize {
    0
}

/// d3d12_device_supports_rtas_formats - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_rtas_formats(device: *mut core::ffi::c_void, format: *mut core::ffi::c_void, count: usize) -> usize {
    0
}

/// d3d12_device_determine_ray_tracing_tier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_determine_ray_tracing_tier(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_determine_heap_tier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_determine_heap_tier(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_determine_additional_typed_uav_support - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_determine_additional_typed_uav_support(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_caps_init_feature_options - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options1(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options2 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options2(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options3 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options3(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options4 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options4(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options5 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options5(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options6 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options6(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options7 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options7(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options8 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options8(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options9 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options9(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options10 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options10(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options11 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options11(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options12(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options13 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options13(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options14 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options14(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options15 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options15(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options16 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options16(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options17 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options17(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options18 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options18(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options19 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options19(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options20 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options20(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_options21 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options21(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_tight_alignment - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_tight_alignment(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_feature_level - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_level(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_shader_model_override - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_shader_model_override(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init_shader_model - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_shader_model(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_override - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_override(device: *mut core::ffi::c_void) {

}

/// d3d12_device_caps_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init(device: *mut core::ffi::c_void) {

}

/// d3d12_device_supports_feature_level - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_feature_level(device: *mut core::ffi::c_void, feature_level: usize) -> usize {
    0
}

/// d3d12_device_replace_vtable - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_replace_vtable(device: *mut core::ffi::c_void) {

}

/// d3d12_device_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_init(device: *mut core::ffi::c_void, instance: *mut core::ffi::c_void, create_info: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_low_latency_device_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_low_latency_device_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, out: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_low_latency_device_SupportsLowLatency - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_SupportsLowLatency(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_low_latency_device_LatencySleep - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_LatencySleep(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_low_latency_device_SetLatencySleepMode - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_SetLatencySleepMode(iface: *mut core::ffi::c_void, low_latency_mode: i32, low_latency_boost: i32, minimum_interval_us: usize) -> usize {
    0
}

/// d3d12_low_latency_device_SetLatencyMarker - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_SetLatencyMarker(iface: *mut core::ffi::c_void, frameID: usize, markerType: usize) -> usize {
    0
}

/// d3d12_low_latency_device_GetLatencyInfo - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_GetLatencyInfo(iface: *mut core::ffi::c_void, latency_results: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_amd_ext_anti_lag_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_amd_ext_anti_lag_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_amd_ext_anti_lag_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_amd_ext_anti_lag_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, out: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_amd_ext_anti_lag_UpdateAntiLagState - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_amd_ext_anti_lag_UpdateAntiLagState(iface: *mut core::ffi::c_void, pData: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_heap_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_heap_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_heap_destroy - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_destroy(heap: *mut core::ffi::c_void) {

}

/// d3d12_heap_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_heap_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_heap_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_heap_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_heap_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_heap_GetDesc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_GetDesc(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_heap_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_init(heap: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, host_address: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_device_use_nv_memory_decompression - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_use_nv_memory_decompression(arg0: usize) -> usize {
    0
}

/// d3d12_meta_command_destroy - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_destroy(meta_command: *mut core::ffi::c_void) {

}

/// d3d12_meta_command_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_meta_command_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_meta_command_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_meta_command_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_meta_command_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_meta_command_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_meta_command_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_meta_command_GetRequiredParameterResourceSize - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_GetRequiredParameterResourceSize(iface: *mut core::ffi::c_void, stage: usize, parameter_index: u32) -> usize {
    0
}

/// d3d12_meta_command_exec_dstorage - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_exec_dstorage(meta_command: *mut core::ffi::c_void, list: *mut core::ffi::c_void, parameter_data: *mut core::ffi::c_void, parameter_size: usize) {

}

/// D3D12_RAYTRACING_OPACITY_MICROMAP_FORMAT - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn D3D12_RAYTRACING_OPACITY_MICROMAP_FORMAT(arg0: usize) -> usize {
    0
}

/// d3d12_state_object_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_state_object_properties_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_state_object_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_state_object_properties_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_state_object_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_cleanup(object: *mut core::ffi::c_void) {

}

/// d3d12_state_object_inc_ref - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_inc_ref(state_object: *mut core::ffi::c_void) {

}

/// d3d12_state_object_dec_ref - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_dec_ref(state_object: *mut core::ffi::c_void) {

}

/// d3d12_state_object_pipeline_data_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_cleanup(data: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// d3d12_state_object_release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_release(state_object: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_state_object_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_state_object_properties_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_state_object_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_state_object_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_state_object_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_state_object_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_state_object_properties_GetShaderIdentifier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_GetShaderIdentifier(iface: *mut core::ffi::c_void, export_name: *const u16) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_state_object_properties_GetShaderStackSize - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_GetShaderStackSize(iface: *mut core::ffi::c_void, export_name: *const u16) -> usize {
    0
}

/// d3d12_state_object_properties_GetPipelineStackSize - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_GetPipelineStackSize(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_state_object_properties_SetPipelineStackSize - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_SetPipelineStackSize(iface: *mut core::ffi::c_void, stack_size_in_bytes: usize) -> usize {
    0
}

/// d3d12_state_object_properties_GetProgramIdentifier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_GetProgramIdentifier(iface: *mut core::ffi::c_void, ret: *mut core::ffi::c_void, pProgramName: *const u16) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_state_object_pipeline_data_cleanup_modules - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_cleanup_modules(data: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// d3d12_state_object_pipeline_data_cleanup_compile_temporaries - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_cleanup_compile_temporaries(data: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// d3d12_state_object_add_collection_library - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_add_collection_library(collection: *mut core::ffi::c_void, data: *mut core::ffi::c_void, exports: *mut core::ffi::c_void, num_exports: u32) -> i32 {
    0
}

/// d3d12_state_object_add_collection_deferred - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_add_collection_deferred(deferred: *mut core::ffi::c_void, data: *mut core::ffi::c_void, exports: *mut core::ffi::c_void, num_exports: u32) -> i32 {
    0
}

/// d3d12_state_object_set_association_data - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_set_association_data(association: *mut core::ffi::c_void, object: *mut core::ffi::c_void) {

}

/// d3d12_state_object_parse_subobject - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_parse_subobject(object: *mut core::ffi::c_void, obj: *mut core::ffi::c_void, data: *mut core::ffi::c_void, association_priority: u32) -> i32 {
    0
}

/// d3d12_state_object_parse_subobjects - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_parse_subobjects(object: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, parent: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_state_object_pipeline_data_find_entry_inner - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_find_entry_inner(entry_points: *mut core::ffi::c_void, count: usize, import: *mut u16) -> u32 {
    0
}

/// d3d12_state_object_find_collection_variant - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_find_collection_variant(variant: *mut core::ffi::c_void, collection: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_state_object_pipeline_data_find_entry - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_find_entry(data: *mut core::ffi::c_void, variant: *mut core::ffi::c_void, pipeline_variant_index: u32, import: *mut u16) -> u32 {
    0
}

/// d3d12_state_object_pipeline_data_compute_default_stack_size - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_compute_default_stack_size(data: *mut core::ffi::c_void, stack_info: *mut core::ffi::c_void, recursion_depth: u32) -> u64 {
    0
}

/// d3d12_state_object_get_group_handles - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_get_group_handles(object: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_state_object_append_local_static_samplers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_append_local_static_samplers(variant: *mut core::ffi::c_void, out_vk_bindings: *mut *mut core::ffi::c_void, out_vk_bindings_size: *mut usize, out_vk_bindings_count: *mut usize, local_bindings: *mut core::ffi::c_void, sampler_desc: *mut core::ffi::c_void, vk_samplers: *mut core::ffi::c_void, sampler_count: u32) {

}

/// d3d12_state_object_pipeline_data_find_global_state_object - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_find_global_state_object(data: *mut core::ffi::c_void, kind: usize, out_association: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_state_object_pipeline_data_find_global_state_objects - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_find_global_state_objects(data: *mut core::ffi::c_void, out_shader_config: *mut core::ffi::c_void, out_pipeline_config: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_state_object_compile_pipeline_variant - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_compile_pipeline_variant(object: *mut core::ffi::c_void, pipeline_variant_index: u32, data: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_state_object_add_global_root_signature_variant - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_add_global_root_signature_variant(object: *mut core::ffi::c_void, rs: *mut core::ffi::c_void) {

}

/// d3d12_state_object_collect_variants - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_collect_variants(object: *mut core::ffi::c_void, data: *mut core::ffi::c_void) {

}

/// d3d12_state_object_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_init(object: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, parent: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_resource_supports_small_resource_alignment - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_supports_small_resource_alignment(desc: *mut core::ffi::c_void, format: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_prefers_general_depth_stencil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_prefers_general_depth_stencil(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_device_supports_universal_color_ds_copy - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_universal_color_ds_copy(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_sampler_needs_border_color - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_sampler_needs_border_color(u: usize, v: usize, w: usize) -> usize {
    0
}

/// d3d12_resource_get_tiling - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_get_tiling(device: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, total_tile_count: *mut u32, packed_mip_info: *mut core::ffi::c_void, tile_shape: *mut core::ffi::c_void, tilings: *mut core::ffi::c_void, vk_info: *mut core::ffi::c_void) {

}

/// d3d12_resource_destroy - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_destroy(resource: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// d3d12_resource_validate_box - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_validate_box(resource: *mut core::ffi::c_void, subresource_idx: u32, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_deferred_incref - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_deferred_incref(userdata: *mut core::ffi::c_void) {

}

/// d3d12_resource_deferred_decref - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_deferred_decref(userdata: *mut core::ffi::c_void) {

}

/// d3d12_resource_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_SetName - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_SetName(iface: *mut core::ffi::c_void, str: *const u16) -> usize {
    0
}

/// d3d12_resource_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_get_mapped_memory_range - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_get_mapped_memory_range(resource: *mut core::ffi::c_void, subresource: u32, range: *mut core::ffi::c_void, vk_mapped_range: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_invalidate_range - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_invalidate_range(resource: *mut core::ffi::c_void, subresource: u32, read_range: *mut core::ffi::c_void) {

}

/// d3d12_resource_flush_range - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_flush_range(resource: *mut core::ffi::c_void, subresource: u32, written_range: *mut core::ffi::c_void) {

}

/// d3d12_resource_get_map_ptr - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_get_map_ptr(resource: *mut core::ffi::c_void, data: *mut *mut core::ffi::c_void) {

}

/// d3d12_resource_texture_validate_map - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_texture_validate_map(resource: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_Map - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_Map(iface: *mut core::ffi::c_void, sub_resource: u32, read_range: *mut core::ffi::c_void, data: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_Unmap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_Unmap(iface: *mut core::ffi::c_void, sub_resource: u32, written_range: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_GetDesc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_GetDesc(iface: *mut core::ffi::c_void, resource_desc: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_resource_GetGPUVirtualAddress - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_GetGPUVirtualAddress(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_GetHeapProperties - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_GetHeapProperties(iface: *mut core::ffi::c_void, heap_properties: *mut core::ffi::c_void, flags: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_GetDesc1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_GetDesc1(iface: *mut core::ffi::c_void, resource_desc: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_resource_validate_texture_format - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_validate_texture_format(desc: *mut core::ffi::c_void, format: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_validate_texture_alignment - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_validate_texture_alignment(desc: *mut core::ffi::c_void, format: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_validate_resource_flags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_validate_resource_flags(arg0: usize) -> usize {
    0
}

/// D3D12_RESOURCE_STATE_COPY_DEST - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn D3D12_RESOURCE_STATE_COPY_DEST(COMMON: usize) -> usize {
    0
}

/// d3d12_resource_init_page_table - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_init_page_table(resource: *mut core::ffi::c_void, device: *mut core::ffi::c_void, sparse: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_resource_init_sparse_info - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_init_sparse_info(resource: *mut core::ffi::c_void, device: *mut core::ffi::c_void, sparse: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_resource_wait_for_sparse_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_wait_for_sparse_init(resource: *mut core::ffi::c_void) {

}

/// d3d12_resource_destroy_and_release_device - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_destroy_and_release_device(resource: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// d3d12_resource_init_subresource_layouts - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_init_subresource_layouts(resource: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_determine_alignment - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_determine_alignment(device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, num_castable_formats: u32, castable_formats: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_resource_desc_default_alignment - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_default_alignment(arg0: usize) -> usize {
    0
}

/// d3d12_resource_tag_debug_name - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_tag_debug_name(resource: *mut core::ffi::c_void, device: *mut core::ffi::c_void, tag: *mut i8) {

}

/// d3d12_resource_validate_heap - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_validate_heap(resource_desc: *mut core::ffi::c_void, heap: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_desc_copy_range - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_copy_range(dst_va: usize, src_va: usize, count: u32, heap_type: usize, device: *mut core::ffi::c_void) {

}

/// d3d12_descriptor_heap_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_descriptor_heap_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_descriptor_heap_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_descriptor_heap_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_descriptor_heap_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_descriptor_heap_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_descriptor_heap_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_descriptor_heap_GetDesc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_GetDesc(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_descriptor_heap_GetCPUDescriptorHandleForHeapStart - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_GetCPUDescriptorHandleForHeapStart(iface: *mut core::ffi::c_void, descriptor: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_descriptor_heap_GetGPUDescriptorHandleForHeapStart - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_GetGPUDescriptorHandleForHeapStart(iface: *mut core::ffi::c_void, descriptor: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_descriptor_heap_zero_initialize - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_zero_initialize(descriptor_heap: *mut core::ffi::c_void, vk_descriptor_type: usize, vk_descriptor_set: usize, binding_index: u32, descriptor_count: u32) {

}

/// d3d12_descriptor_heap_get_host_mapping - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_get_host_mapping(descriptor_heap: *mut core::ffi::c_void, binding: *mut core::ffi::c_void, set_index: u32) {

}

/// d3d12_descriptor_heap_get_buffer_range - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_get_buffer_range(descriptor_heap: *mut core::ffi::c_void, offset: *mut u64, size: u64, range: *mut core::ffi::c_void) {

}

/// d3d12_descriptor_heap_init_data_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_init_data_buffer(descriptor_heap: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_descriptor_heap_update_extra_bindings - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_update_extra_bindings(descriptor_heap: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// d3d12_descriptor_heap_add_null_descriptor_template_buffers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_add_null_descriptor_template_buffers(descriptor_heap: *mut core::ffi::c_void, set_info: *mut core::ffi::c_void, set_info_index: u32) {

}

/// d3d12_descriptor_heap_add_null_descriptor_template_descriptors - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_add_null_descriptor_template_descriptors(descriptor_heap: *mut core::ffi::c_void, set_info: *mut core::ffi::c_void, set_info_index: u32) {

}

/// d3d12_descriptor_heap_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_init(descriptor_heap: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_descriptor_heap_init_descriptors - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_init_descriptors(descriptor_heap: *mut core::ffi::c_void) {

}

/// d3d12_query_heap_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, out: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_query_heap_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_query_heap_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_query_heap_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_query_heap_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_query_heap_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_query_heap_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_root_signature_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_root_signature_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_root_signature_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_cleanup(root_signature: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// d3d12_root_signature_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_root_signature_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_root_signature_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_root_signature_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_root_signature_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_root_signature_info_count_srv_uav_table - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_info_count_srv_uav_table(info: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// d3d12_root_signature_info_count_cbv_table - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_info_count_cbv_table(info: *mut core::ffi::c_void) {

}

/// d3d12_root_signature_info_count_sampler_table - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_info_count_sampler_table(info: *mut core::ffi::c_void) {

}

/// d3d12_root_signature_may_require_global_heap_binding - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_may_require_global_heap_binding(device: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_root_signature_info_from_desc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_info_from_desc(info: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_root_signature_init_srv_uav_binding - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_srv_uav_binding(root_signature: *mut core::ffi::c_void, context: *mut core::ffi::c_void, range_type: usize, binding: *mut core::ffi::c_void, out_bindings_base: *mut core::ffi::c_void, out_index: *mut u32) {

}

/// d3d12_root_signature_init_srv_uav_heap_bindings - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_srv_uav_heap_bindings(root_signature: *mut core::ffi::c_void, context: *mut core::ffi::c_void, range_type: usize) {

}

/// d3d12_root_signature_init_cbv_srv_uav_heap_bindings - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_cbv_srv_uav_heap_bindings(root_signature: *mut core::ffi::c_void, context: *mut core::ffi::c_void) {

}

/// d3d12_root_signature_init_sampler_heap_bindings - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_sampler_heap_bindings(root_signature: *mut core::ffi::c_void, context: *mut core::ffi::c_void) {

}

/// d3d12_root_signature_init_root_descriptor_tables - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_root_descriptor_tables(root_signature: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, info: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_root_signature_add_common_flags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_add_common_flags(root_signature: *mut core::ffi::c_void, common_flags: u32) {

}

/// d3d12_root_signature_init_shader_record_descriptors - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_shader_record_descriptors(root_signature: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, info: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_root_signature_init_root_descriptors - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_root_descriptors(root_signature: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, info: *mut core::ffi::c_void, push_constant_range: *mut core::ffi::c_void, context: *mut core::ffi::c_void, vk_set_layout: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_root_signature_init_local_static_samplers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_local_static_samplers(root_signature: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_root_signature_init_static_samplers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_static_samplers(root_signature: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, context: *mut core::ffi::c_void, vk_set_layout: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_root_signature_init_local - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_local(root_signature: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_root_signature_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init(root_signature: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_line_rasteriztion_mode_from_legacy_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_line_rasteriztion_mode_from_legacy_state(MultisampleEnable: i32, AntialiasedLineEnable: i32) -> usize {
    0
}

/// d3d12_promote_rasterizer_desc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_promote_rasterizer_desc(out: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) {

}

/// d3d12_promote_rasterizer_desc1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_promote_rasterizer_desc1(out: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) {

}

/// d3d12_promote_depth_stencil_desc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_promote_depth_stencil_desc(out: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) {

}

/// d3d12_promote_depth_stencil_desc1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_promote_depth_stencil_desc1(out: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) {

}

/// d3d12_init_pipeline_state_desc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_init_pipeline_state_desc(desc: *mut core::ffi::c_void) {

}

/// d3d12_pipeline_state_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_state_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_state_destroy_shader_modules - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_destroy_shader_modules(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// d3d12_pipeline_state_destroy_graphics - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_destroy_graphics(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// d3d12_pipeline_state_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_state_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_state_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_state_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_state_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_state_GetCachedBlob - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_GetCachedBlob(iface: *mut core::ffi::c_void, blob: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_state_init_shader_interface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_init_shader_interface(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void, stage: usize, shader_interface: *mut core::ffi::c_void) {

}

/// d3d12_pipeline_state_init_compile_arguments - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_init_compile_arguments(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void, stage: usize, compile_arguments: *mut core::ffi::c_void) {

}

/// d3d12_pipeline_state_init_compute - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_init_compute(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, cached_pso: *mut core::ffi::c_void) -> i32 {
    0
}

/// rs_desc_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rs_desc_from_d3d12(vk_desc: *mut core::ffi::c_void, d3d12_desc: *mut core::ffi::c_void) {

}

/// rs_conservative_info_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rs_conservative_info_from_d3d12(conservative_info: *mut core::ffi::c_void, vk_rs_desc: *mut core::ffi::c_void, d3d12_desc: *mut core::ffi::c_void) {

}

/// rs_depth_clip_info_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rs_depth_clip_info_from_d3d12(depth_clip_info: *mut core::ffi::c_void, vk_rs_desc: *mut core::ffi::c_void, d3d12_desc: *mut core::ffi::c_void) {

}

/// rs_line_info_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rs_line_info_from_d3d12(device: *mut core::ffi::c_void, vk_line_info: *mut core::ffi::c_void, vk_rs_desc: *mut core::ffi::c_void, d3d12_desc: *mut core::ffi::c_void) {

}

/// ds_desc_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ds_desc_from_d3d12(vk_desc: *mut core::ffi::c_void, d3d12_desc: *mut core::ffi::c_void) {

}

/// blend_desc_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn blend_desc_from_d3d12(vk_desc: *mut core::ffi::c_void, d3d12_desc: *mut core::ffi::c_void, attachment_count: u32, attachments: *mut core::ffi::c_void) {

}

/// d3d12_graphics_pipeline_state_get_plane_optimal_mask - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_graphics_pipeline_state_get_plane_optimal_mask(graphics: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_graphics_pipeline_state_get_dynamic_state_flags - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_graphics_pipeline_state_get_dynamic_state_flags(state: *mut core::ffi::c_void, key: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_graphics_pipeline_state_init_dynamic_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_graphics_pipeline_state_init_dynamic_state(state: *mut core::ffi::c_void, dynamic_desc: *mut core::ffi::c_void, dynamic_state_buffer: *mut core::ffi::c_void, key: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_pipeline_state_validate_blend_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_validate_blend_state(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, sig: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_pipeline_state_graphics_load_spirv_from_cached_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_graphics_load_spirv_from_cached_state(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, cached_pso: *mut core::ffi::c_void) {

}

/// d3d12_pipeline_state_graphics_handle_meta - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_graphics_handle_meta(state: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_pipeline_state_init_graphics_spirv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_init_graphics_spirv(state: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, cached_pso: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_pipeline_state_init_static_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_init_static_pipeline(state: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_pipeline_state_finish_graphics - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_finish_graphics(state: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_pipeline_state_find_compiled_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_find_compiled_pipeline(state: *mut core::ffi::c_void, key: *mut core::ffi::c_void, dynamic_state_flags: *mut u32) -> usize {
    0
}

/// d3d12_pipeline_state_put_pipeline_to_cache - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_put_pipeline_to_cache(state: *mut core::ffi::c_void, key: *mut core::ffi::c_void, vk_pipeline: usize, dynamic_state_flags: u32) -> usize {
    0
}

/// d3d12_pipeline_state_link_pipeline_variant - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_link_pipeline_variant(state: *mut core::ffi::c_void, key: *mut core::ffi::c_void, dsv_format: *mut core::ffi::c_void, vk_cache: usize, dynamic_state_flags: u32, vk_pipeline: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_max_descriptor_count_from_heap_type - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_max_descriptor_count_from_heap_type(device: *mut core::ffi::c_void, heap_type: usize) -> u32 {
    0
}

/// d3d12_max_host_descriptor_count_from_heap_type - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_max_host_descriptor_count_from_heap_type(device: *mut core::ffi::c_void, heap_type: usize) -> u32 {
    0
}

/// ExecuteIndirect - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ExecuteIndirect(pCommandSignature: *mut core::ffi::c_void, MaxCommandCount: usize, pArgumentBuffer: *mut core::ffi::c_void, ArgumentBufferOffset: usize, pCountBuffer: *mut core::ffi::c_void, CountBufferOffset: usize) -> usize {
    0
}

/// ID3D12CommandQueue_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12CommandQueue_AddRef(arg0: usize) -> usize {
    0
}

/// ID3D12CommandQueue_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12CommandQueue_Release(arg0: usize) -> usize {
    0
}

/// ID3D12CommandQueue_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12CommandQueue_QueryInterface(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// ID3D12Resource2_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12Resource2_QueryInterface(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// DXGI_FORMAT - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn DXGI_FORMAT(arg0: usize) -> usize {
    0
}

/// ID3D12Object_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ID3D12Object_SetPrivateData(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// d3d12_root_signature_deserializer_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_deserializer_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_root_signature_deserializer_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_deserializer_init(deserializer: *mut core::ffi::c_void, dxbc: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_versioned_root_signature_deserializer_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_versioned_root_signature_deserializer_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_versioned_root_signature_deserializer_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_versioned_root_signature_deserializer_init(deserializer: *mut core::ffi::c_void, dxbc: *mut core::ffi::c_void, raw_payload: usize) -> i32 {
    0
}

/// d3d12_wg_state_object_cleanup_data - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_cleanup_data(data: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// d3d12_wg_state_object_parse_subobject - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_parse_subobject(data: *mut core::ffi::c_void, device: *mut core::ffi::c_void, obj: *mut core::ffi::c_void, association_priority: u32) -> i32 {
    0
}

/// d3d12_work_graph_find_array_size_by_id - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_find_array_size_by_id(entries: *mut core::ffi::c_void, entry_count: usize, node_id: *mut i8) -> u32 {
    0
}

/// d3d12_work_graph_find_node_by_id - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_find_node_by_id(entries: *mut core::ffi::c_void, entry_count: usize, node_id: *mut i8, node_array_index: u32) -> u32 {
    0
}

/// d3d12_work_graph_find_node_by_id_wchar - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_find_node_by_id_wchar(entries: *mut core::ffi::c_void, entry_count: usize, node_id: *const u16, node_array_index: u32) -> u32 {
    0
}

/// d3d12_wg_state_object_program_add_node_to_level - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_program_add_node_to_level(program: *mut core::ffi::c_void, data: *mut core::ffi::c_void, entry_point_index: u32, level: u32, is_recursing: usize) -> i32 {
    0
}

/// d3d12_wg_state_object_program_add_outputs_to_level - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_program_add_outputs_to_level(program: *mut core::ffi::c_void, data: *mut core::ffi::c_void, entry_point_index: u32, level: u32) -> i32 {
    0
}

/// d3d12_wg_state_object_rearrange_entry_points - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_rearrange_entry_points(data: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_wg_state_object_apply_node_overrides - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_apply_node_overrides(data: *mut core::ffi::c_void, program: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_wg_state_object_resolve_entry_points - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_resolve_entry_points(object: *mut core::ffi::c_void, data: *mut core::ffi::c_void, program: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_wg_state_object_parse_subobjects - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_parse_subobjects(data: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_wg_state_object_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_wg_state_object_properties_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_work_graph_properties_QueryInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, object: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_wg_state_object_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_wg_state_object_properties_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_work_graph_properties_AddRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_AddRef(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_wg_state_object_cleanup - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_cleanup(state_object: *mut core::ffi::c_void) {

}

/// d3d12_wg_state_object_dec_ref - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_dec_ref(state_object: *mut core::ffi::c_void) {

}

/// d3d12_wg_state_object_release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_release(state_object: *mut core::ffi::c_void) -> u32 {
    0
}

/// d3d12_wg_state_object_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_wg_state_object_properties_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_work_graph_properties_Release - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_Release(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_wg_state_object_GetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_GetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: *mut u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_wg_state_object_SetPrivateData - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_SetPrivateData(iface: *mut core::ffi::c_void, guid: usize, data_size: u32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_wg_state_object_SetPrivateDataInterface - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_SetPrivateDataInterface(iface: *mut core::ffi::c_void, guid: usize, data: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_wg_state_object_GetDevice - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_GetDevice(iface: *mut core::ffi::c_void, iid: usize, device: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_wg_state_object_properties_GetShaderIdentifier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_GetShaderIdentifier(iface: *mut core::ffi::c_void, export_name: *const u16) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_wg_state_object_properties_GetShaderStackSize - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_GetShaderStackSize(iface: *mut core::ffi::c_void, export_name: *const u16) -> usize {
    0
}

/// d3d12_wg_state_object_properties_GetPipelineStackSize - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_GetPipelineStackSize(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_wg_state_object_properties_SetPipelineStackSize - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_SetPipelineStackSize(iface: *mut core::ffi::c_void, stack_size_in_bytes: usize) -> usize {
    0
}

/// d3d12_wg_state_object_properties_GetProgramIdentifier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_GetProgramIdentifier(iface: *mut core::ffi::c_void, ret: *mut core::ffi::c_void, pProgramName: *const u16) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_work_graph_properties_GetNumWorkGraphs - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetNumWorkGraphs(iface: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_work_graph_properties_GetProgramName - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetProgramName(iface: *mut core::ffi::c_void, WorkGraphIndex: u32) -> usize {
    0
}

/// d3d12_work_graph_properties_GetWorkGraphIndex - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetWorkGraphIndex(iface: *mut core::ffi::c_void, pProgramName: *const u16) -> usize {
    0
}

/// d3d12_work_graph_properties_GetNumNodes - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetNumNodes(iface: *mut core::ffi::c_void, WorkGraphIndex: u32) -> usize {
    0
}

/// d3d12_work_graph_properties_node_index_to_entry - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_node_index_to_entry(program: *mut core::ffi::c_void, NodeIndex: u32) -> u32 {
    0
}

/// d3d12_work_graph_properties_GetNodeID - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetNodeID(iface: *mut core::ffi::c_void, ret: *mut core::ffi::c_void, WorkGraphIndex: u32, NodeIndex: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_work_graph_properties_GetNodeIndex - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetNodeIndex(iface: *mut core::ffi::c_void, WorkGraphIndex: u32, NodeID: usize) -> usize {
    0
}

/// d3d12_work_graph_properties_GetNodeLocalRootArgumentsTableIndex - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetNodeLocalRootArgumentsTableIndex(iface: *mut core::ffi::c_void, WorkGraphIndex: u32, NodeIndex: u32) -> usize {
    0
}

/// d3d12_work_graph_properties_GetNumEntrypoints - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetNumEntrypoints(iface: *mut core::ffi::c_void, WorkGraphIndex: u32) -> usize {
    0
}

/// d3d12_work_graph_properties_GetEntrypointID - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetEntrypointID(iface: *mut core::ffi::c_void, ret: *mut core::ffi::c_void, WorkGraphIndex: u32, EntrypointIndex: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_work_graph_properties_GetEntrypointIndex - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetEntrypointIndex(iface: *mut core::ffi::c_void, WorkGraphIndex: u32, NodeID: usize) -> usize {
    0
}

/// d3d12_work_graph_properties_GetEntrypointRecordSizeInBytes - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetEntrypointRecordSizeInBytes(iface: *mut core::ffi::c_void, WorkGraphIndex: u32, EntrypointIndex: u32) -> usize {
    0
}

/// d3d12_work_graph_properties_GetWorkGraphMemoryRequirements - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetWorkGraphMemoryRequirements(iface: *mut core::ffi::c_void, WorkGraphIndex: u32, pWorkGraphMemoryRequirements: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_wg_state_object_compile_pipeline - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_compile_pipeline(object: *mut core::ffi::c_void, data: *mut core::ffi::c_void, program: *mut core::ffi::c_void, entry_point_index: u32, tmp: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_wg_state_object_program_can_compact_broadcast_nodes - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_program_can_compact_broadcast_nodes(object: *mut core::ffi::c_void, data: *mut core::ffi::c_void, program: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_wg_state_object_compile_program - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_compile_program(object: *mut core::ffi::c_void, data: *mut core::ffi::c_void, program: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_wg_state_object_convert_entry_point - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_convert_entry_point(object: *mut core::ffi::c_void, data: *mut core::ffi::c_void, module: *mut core::ffi::c_void, entry: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_wg_state_object_compile_programs - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_compile_programs(object: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_wg_state_object_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_init(object: *mut core::ffi::c_void, device: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_command_list_update_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_buffer(list: *mut core::ffi::c_void, vk_buffer: usize, vk_offset: u64, size: u64, data_: *mut core::ffi::c_void) {

}

/// d3d12_command_list_workgraph_bind_resources - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_bind_resources(list: *mut core::ffi::c_void, state: *mut core::ffi::c_void, program: *mut core::ffi::c_void, module: *mut core::ffi::c_void, vk_root_parameter_buffer: usize, vk_root_parameter_buffer_offset: u64) {

}

/// d3d12_command_list_workgraph_remaining_levels - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_remaining_levels(program: *mut core::ffi::c_void, level: u32, node_index: u32) -> u32 {
    0
}

/// d3d12_command_list_workgraph_execute_node_cpu_entry - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_execute_node_cpu_entry(list: *mut core::ffi::c_void, state: *mut core::ffi::c_void, program: *mut core::ffi::c_void, node_index: u32, desc: *mut core::ffi::c_void, output_payload: usize, input_payload: usize, vk_root_parameter_buffer: usize, vk_root_parameter_buffer_offset: u64) {

}

/// d3d12_command_list_workgraph_barrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_barrier(list: *mut core::ffi::c_void, vk_dst_stages: usize, vk_dst_access: usize) {

}

/// d3d12_command_list_emit_distribute_workgroups - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_emit_distribute_workgroups(list: *mut core::ffi::c_void, state: *mut core::ffi::c_void, program: *mut core::ffi::c_void) {

}

/// d3d12_command_list_emit_distribute_payload_offsets - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_emit_distribute_payload_offsets(list: *mut core::ffi::c_void, state: *mut core::ffi::c_void, program: *mut core::ffi::c_void, level: u32, payload_va: usize) {

}

/// d3d12_command_list_workgraph_execute_node_gpu - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_execute_node_gpu(list: *mut core::ffi::c_void, state: *mut core::ffi::c_void, program: *mut core::ffi::c_void, output_va: usize, input_va: usize, level: u32, node_index: u32, indirect_scratch: *mut core::ffi::c_void, vk_root_parameter_buffer: usize, vk_root_parameter_buffer_offset: u64) {

}

/// d3d12_command_list_workgraph_execute_level - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_execute_level(list: *mut core::ffi::c_void, state: *mut core::ffi::c_void, program: *mut core::ffi::c_void, level: u32, output_payload: usize, input_payload: usize, vk_root_parameter_buffer: usize, vk_root_parameter_buffer_offset: u64) {

}

/// d3d12_command_list_workgraph_setup_indirect - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_setup_indirect(list: *mut core::ffi::c_void, wg_state: *mut core::ffi::c_void, program: *mut core::ffi::c_void, va: usize, indirect_scratch: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_command_list_workgraph_execute_entry_gpu - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_execute_entry_gpu(list: *mut core::ffi::c_void, state: *mut core::ffi::c_void, program: *mut core::ffi::c_void, va: usize, vk_root_param_buffer: usize, vk_root_param_offset: u64) {

}

/// d3d12_command_list_workgraph_execute_entry_cpu - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_execute_entry_cpu(list: *mut core::ffi::c_void, wg_state: *mut core::ffi::c_void, program: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, vk_root_param_buffer: usize, vk_root_param_offset: u64) {

}

/// d3d12_command_list_workgraph_execute_entry_level - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_execute_entry_level(list: *mut core::ffi::c_void, wg_state: *mut core::ffi::c_void, program: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, vk_root_param_buffer: usize, vk_root_param_offset: u64) {

}

/// SetName - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetName(Name: usize) -> usize {
    0
}

/// GetSerializedSize - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetSerializedSize() -> usize {
    0
}

/// GetSerializedData - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetSerializedData(arg0: usize) -> usize {
    0
}

/// GetRootSignatureDesc - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetRootSignatureDesc() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// GetRootSignatureDescAtVersion - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetRootSignatureDescAtVersion(convertToVersion: usize, ppDesc: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetUnconvertedRootSignatureDesc - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetUnconvertedRootSignatureDesc() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// GetGPUVirtualAddress - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetGPUVirtualAddress() -> usize {
    0
}

/// GetHeapProperties - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetHeapProperties(pHeapProperties: *mut core::ffi::c_void, pHeapFlags: *mut core::ffi::c_void) -> usize {
    0
}

/// GetCachedBlob - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetCachedBlob(ppBlob: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetRootSignature - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetRootSignature(riid: usize, ppvRootSignature: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetCPUDescriptorHandleForHeapStart - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetCPUDescriptorHandleForHeapStart() -> usize {
    0
}

/// GetGPUDescriptorHandleForHeapStart - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetGPUDescriptorHandleForHeapStart() -> usize {
    0
}

/// CopyBufferRegion - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CopyBufferRegion(pDstBuffer: *mut core::ffi::c_void, DstOffset: usize, pSrcBuffer: *mut core::ffi::c_void, SrcOffset: usize, NumBytes: usize) -> usize {
    0
}

/// CopyTextureRegion - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CopyTextureRegion(pDst: *mut core::ffi::c_void, DstX: u32, DstY: u32, DstZ: u32, pSrc: *mut core::ffi::c_void, pSrcBox: *mut core::ffi::c_void) -> usize {
    0
}

/// OMSetBlendFactor - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn OMSetBlendFactor(arg0: usize) -> usize {
    0
}

/// OMSetStencilRef - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn OMSetStencilRef(StencilRef: usize) -> usize {
    0
}

/// SetPipelineState - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetPipelineState(pPipelineState: *mut core::ffi::c_void) -> usize {
    0
}

/// ResourceBarrier - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ResourceBarrier(NumBarriers: usize, arg1: usize) -> usize {
    0
}

/// ExecuteBundle - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ExecuteBundle(pCommandList: *mut core::ffi::c_void) -> usize {
    0
}

/// SetDescriptorHeaps - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetDescriptorHeaps(NumDescriptorHeaps: usize, arg1: usize) -> usize {
    0
}

/// SetComputeRootSignature - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetComputeRootSignature(pRootSignature: *mut core::ffi::c_void) -> usize {
    0
}

/// SetGraphicsRootSignature - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetGraphicsRootSignature(pRootSignature: *mut core::ffi::c_void) -> usize {
    0
}

/// SetComputeRootDescriptorTable - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetComputeRootDescriptorTable(RootParameterIndex: usize, BaseDescriptor: usize) -> usize {
    0
}

/// SetGraphicsRootDescriptorTable - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetGraphicsRootDescriptorTable(RootParameterIndex: usize, BaseDescriptor: usize) -> usize {
    0
}

/// SetComputeRoot32BitConstant - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetComputeRoot32BitConstant(RootParameterIndex: usize, SrcData: usize, DestOffsetIn32BitValues: usize) -> usize {
    0
}

/// SetGraphicsRoot32BitConstant - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetGraphicsRoot32BitConstant(RootParameterIndex: usize, SrcData: usize, DestOffsetIn32BitValues: usize) -> usize {
    0
}

/// SetComputeRoot32BitConstants - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetComputeRoot32BitConstants(RootParameterIndex: usize, Num32BitValuesToSet: usize, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// SetGraphicsRoot32BitConstants - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetGraphicsRoot32BitConstants(RootParameterIndex: usize, Num32BitValuesToSet: usize, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// SetComputeRootConstantBufferView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetComputeRootConstantBufferView(RootParameterIndex: usize, BufferLocation: usize) -> usize {
    0
}

/// SetGraphicsRootConstantBufferView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetGraphicsRootConstantBufferView(RootParameterIndex: usize, BufferLocation: usize) -> usize {
    0
}

/// SetComputeRootShaderResourceView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetComputeRootShaderResourceView(RootParameterIndex: usize, BufferLocation: usize) -> usize {
    0
}

/// SetGraphicsRootShaderResourceView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetGraphicsRootShaderResourceView(RootParameterIndex: usize, BufferLocation: usize) -> usize {
    0
}

/// SetComputeRootUnorderedAccessView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetComputeRootUnorderedAccessView(RootParameterIndex: usize, BufferLocation: usize) -> usize {
    0
}

/// SetGraphicsRootUnorderedAccessView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetGraphicsRootUnorderedAccessView(RootParameterIndex: usize, BufferLocation: usize) -> usize {
    0
}

/// BeginQuery - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn BeginQuery(pQueryHeap: *mut core::ffi::c_void, Type: usize, Index: usize) -> usize {
    0
}

/// EndQuery - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn EndQuery(pQueryHeap: *mut core::ffi::c_void, Type: usize, Index: usize) -> usize {
    0
}

/// ResolveQueryData - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ResolveQueryData(pQueryHeap: *mut core::ffi::c_void, Type: usize, StartIndex: usize, NumQueries: usize, pDestinationBuffer: *mut core::ffi::c_void, AlignedDestinationBufferOffset: usize) -> usize {
    0
}

/// OMSetDepthBounds - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn OMSetDepthBounds(Min: usize, Max: usize) -> usize {
    0
}

/// SetSamplePositions - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetSamplePositions(NumSamplesPerPixel: usize, NumPixels: usize, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// ResolveSubresourceRegion - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ResolveSubresourceRegion(pDstResource: *mut core::ffi::c_void, DstSubresource: usize, DstX: usize, DstY: usize, pSrcResource: *mut core::ffi::c_void, SrcSubresource: usize, pSrcRect: *mut core::ffi::c_void, Format: usize, ResolveMode: usize) -> usize {
    0
}

/// SetViewInstanceMask - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetViewInstanceMask(Mask: usize) -> usize {
    0
}

/// ExecuteCommandLists - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ExecuteCommandLists(NumCommandLists: usize, arg1: usize) -> usize {
    0
}

/// GetTimestampFrequency - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetTimestampFrequency(pFrequency: *mut core::ffi::c_void) -> usize {
    0
}

/// GetClockCalibration - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetClockCalibration(pGpuTimestamp: *mut core::ffi::c_void, pCpuTimestamp: *mut core::ffi::c_void) -> usize {
    0
}

/// SetProcessPriority - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetProcessPriority(Priority: usize) -> usize {
    0
}

/// GetProcessPriority - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetProcessPriority(pOutValue: *mut core::ffi::c_void) -> usize {
    0
}

/// SetGlobalPriority - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetGlobalPriority(Priority: usize) -> usize {
    0
}

/// GetGlobalPriority - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetGlobalPriority(pOutValue: *mut core::ffi::c_void) -> usize {
    0
}

/// GetNodeCount - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetNodeCount() -> usize {
    0
}

/// CreateCommandQueue - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateCommandQueue(pDesc: *mut core::ffi::c_void, riid: usize, ppCommandQueue: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateCommandAllocator - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateCommandAllocator(arg0: usize, riid: usize, ppCommandAllocator: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateGraphicsPipelineState - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateGraphicsPipelineState(pDesc: *mut core::ffi::c_void, riid: usize, ppPipelineState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateComputePipelineState - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateComputePipelineState(pDesc: *mut core::ffi::c_void, riid: usize, ppPipelineState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateCommandList - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateCommandList(nodeMask: usize, arg1: usize, pCommandAllocator: *mut core::ffi::c_void, pInitialState: *mut core::ffi::c_void, riid: usize, ppCommandList: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateDescriptorHeap - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateDescriptorHeap(pDescriptorHeapDesc: *mut core::ffi::c_void, riid: usize, ppvHeap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetDescriptorHandleIncrementSize - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetDescriptorHandleIncrementSize(DescriptorHeapType: usize) -> usize {
    0
}

/// CreateConstantBufferView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateConstantBufferView(pDesc: *mut core::ffi::c_void, DestDescriptor: usize) -> usize {
    0
}

/// CreateSampler - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateSampler(pDesc: *mut core::ffi::c_void, DestDescriptor: usize) -> usize {
    0
}

/// CopyDescriptorsSimple - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CopyDescriptorsSimple(NumDescriptors: usize, DestDescriptorRangeStart: usize, SrcDescriptorRangeStart: usize, DescriptorHeapsType: usize) -> usize {
    0
}

/// GetResourceAllocationInfo - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetResourceAllocationInfo(visibleMask: usize, numResourceDescs: usize, arg2: usize) -> usize {
    0
}

/// GetCustomHeapProperties - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetCustomHeapProperties(nodeMask: usize, heapType: usize) -> usize {
    0
}

/// CreateCommittedResource - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateCommittedResource(pHeapProperties: *mut core::ffi::c_void, HeapFlags: usize, pDesc: *mut core::ffi::c_void, InitialResourceState: usize, pOptimizedClearValue: *mut core::ffi::c_void, riidResource: usize, ppvResource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateHeap - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateHeap(pDesc: *mut core::ffi::c_void, riid: usize, ppvHeap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreatePlacedResource - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreatePlacedResource(pHeap: *mut core::ffi::c_void, HeapOffset: usize, pDesc: *mut core::ffi::c_void, InitialState: usize, pOptimizedClearValue: *mut core::ffi::c_void, riid: usize, ppvResource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateReservedResource - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateReservedResource(pDesc: *mut core::ffi::c_void, InitialState: usize, pOptimizedClearValue: *mut core::ffi::c_void, riid: usize, ppvResource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OpenSharedHandle - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn OpenSharedHandle(NTHandle: usize, riid: usize, ppvObj: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OpenSharedHandleByName - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn OpenSharedHandleByName(Name: usize, Access: u32, pNTHandle: *mut core::ffi::c_void) -> usize {
    0
}

/// MakeResident - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn MakeResident(NumObjects: u32, arg1: usize) -> usize {
    0
}

/// Evict - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn Evict(NumObjects: u32, arg1: usize) -> usize {
    0
}

/// CreateQueryHeap - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateQueryHeap(pDesc: *mut core::ffi::c_void, riid: usize, ppvHeap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SetStablePowerState - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetStablePowerState(Enable: i32) -> usize {
    0
}

/// CreateCommandSignature - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateCommandSignature(pDesc: *mut core::ffi::c_void, pRootSignature: *mut core::ffi::c_void, riid: usize, ppvCommandSignature: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetAdapterLuid - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetAdapterLuid() -> usize {
    0
}

/// StorePipeline - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn StorePipeline(pName: usize, pPipeline: *mut core::ffi::c_void) -> usize {
    0
}

/// LoadGraphicsPipeline - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn LoadGraphicsPipeline(pName: usize, pDesc: *mut core::ffi::c_void, riid: usize, ppPipelineState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// LoadComputePipeline - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn LoadComputePipeline(pName: usize, pDesc: *mut core::ffi::c_void, riid: usize, ppPipelineState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// Serialize - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn Serialize(arg0: usize) -> usize {
    0
}

/// LoadPipeline - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn LoadPipeline(pName: usize, pDesc: *mut core::ffi::c_void, riid: usize, ppPipelineState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreatePipelineState - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreatePipelineState(pDesc: *mut core::ffi::c_void, riid: usize, ppPipelineState: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OpenExistingHeapFromAddress - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn OpenExistingHeapFromAddress(pAddress: *mut core::ffi::c_void, riid: usize, ppvHeap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// OpenExistingHeapFromFileMapping - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn OpenExistingHeapFromFileMapping(hFileMapping: usize, riid: usize, ppvHeap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// EnqueueMakeResident - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn EnqueueMakeResident(Flags: usize, NumObjects: u32, arg2: usize) -> usize {
    0
}

/// GetStatusFence - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetStatusFence(riid: usize, ppFence: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetSessionStatus - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetSessionStatus() -> usize {
    0
}

/// CreateCommandList1 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateCommandList1(nodeMask: usize, arg1: usize, flags: usize, riid: usize, ppCommandList: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateProtectedResourceSession - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateProtectedResourceSession(pDesc: *mut core::ffi::c_void, riid: usize, ppSession: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateCommittedResource1 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateCommittedResource1(pHeapProperties: *mut core::ffi::c_void, HeapFlags: usize, pDesc: *mut core::ffi::c_void, InitialResourceState: usize, pOptimizedClearValue: *mut core::ffi::c_void, pProtectedSession: *mut core::ffi::c_void, riidResource: usize, ppvResource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateHeap1 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateHeap1(pDesc: *mut core::ffi::c_void, pProtectedSession: *mut core::ffi::c_void, riid: usize, ppvHeap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateReservedResource1 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateReservedResource1(pDesc: *mut core::ffi::c_void, InitialState: usize, pOptimizedClearValue: *mut core::ffi::c_void, pProtectedSession: *mut core::ffi::c_void, riid: usize, ppvResource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// LifetimeStateUpdated - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn LifetimeStateUpdated(NewState: usize) -> usize {
    0
}

/// GetLUID - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetLUID() -> usize {
    0
}

/// GetSwapChainObject - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetSwapChainObject(riid: usize, ppv: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetCurrentResourceAndCommandQueue - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetCurrentResourceAndCommandQueue(riidResource: usize, ppvResource: *mut *mut core::ffi::c_void, riidQueue: usize, ppvQueue: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// InsertImplicitSync - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn InsertImplicitSync() -> usize {
    0
}

/// DestroyOwnedObject - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn DestroyOwnedObject(pObject: *mut core::ffi::c_void) -> usize {
    0
}

/// GetShaderIdentifier - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetShaderIdentifier(pExportName: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// GetShaderStackSize - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetShaderStackSize(pExportName: usize) -> usize {
    0
}

/// GetPipelineStackSize - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetPipelineStackSize() -> usize {
    0
}

/// SetPipelineStackSize - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetPipelineStackSize(PipelineStackSizeInBytes: usize) -> usize {
    0
}

/// GetProgramIdentifier - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetProgramIdentifier(pProgramName: *const u16) -> usize {
    0
}

/// GetGlobalRootSignatureForProgram - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetGlobalRootSignatureForProgram(pProgramName: *const u16, riid: usize, ppvRootSignature: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetGlobalRootSignatureForShader - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetGlobalRootSignatureForShader(pExportName: *const u16, riid: usize, ppvRootSignature: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetNumWorkGraphs - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetNumWorkGraphs() -> usize {
    0
}

/// GetProgramName - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetProgramName(WorkGraphIndex: u32) -> usize {
    0
}

/// GetWorkGraphIndex - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetWorkGraphIndex(pProgramName: *const u16) -> usize {
    0
}

/// GetNumNodes - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetNumNodes(WorkGraphIndex: u32) -> usize {
    0
}

/// GetNodeID - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetNodeID(WorkGraphIndex: u32, NodeIndex: u32) -> usize {
    0
}

/// GetNodeIndex - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetNodeIndex(WorkGraphIndex: u32, NodeID: usize) -> usize {
    0
}

/// GetNodeLocalRootArgumentsTableIndex - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetNodeLocalRootArgumentsTableIndex(WorkGraphIndex: u32, NodeIndex: u32) -> usize {
    0
}

/// GetNumEntrypoints - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetNumEntrypoints(WorkGraphIndex: u32) -> usize {
    0
}

/// GetEntrypointID - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetEntrypointID(WorkGraphIndex: u32, EntrypointIndex: u32) -> usize {
    0
}

/// GetEntrypointIndex - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetEntrypointIndex(WorkGraphIndex: u32, NodeID: usize) -> usize {
    0
}

/// GetEntrypointRecordSizeInBytes - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetEntrypointRecordSizeInBytes(WorkGraphIndex: u32, EntrypointIndex: u32) -> usize {
    0
}

/// GetWorkGraphMemoryRequirements - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetWorkGraphMemoryRequirements(WorkGraphIndex: u32, pWorkGraphMemoryRequirements: *mut core::ffi::c_void) -> usize {
    0
}

/// GetEntrypointRecordAlignmentInBytes - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetEntrypointRecordAlignmentInBytes(WorkGraphIndex: u32, EntrypointIndex: u32) -> usize {
    0
}

/// CreateLifetimeTracker - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateLifetimeTracker(pOwner: *mut core::ffi::c_void, riid: usize, ppvTracker: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// RemoveDevice - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn RemoveDevice() -> usize {
    0
}

/// EnumerateMetaCommands - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn EnumerateMetaCommands(pNumMetaCommands: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// EnumerateMetaCommandParameters - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn EnumerateMetaCommandParameters(CommandId: usize, Stage: usize, pTotalStructureSizeInBytes: *mut core::ffi::c_void, pParameterCount: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateStateObject - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateStateObject(pDesc: *mut core::ffi::c_void, riid: usize, ppStateObject: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetRaytracingAccelerationStructurePrebuildInfo - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetRaytracingAccelerationStructurePrebuildInfo(pDesc: *mut core::ffi::c_void, pInfo: *mut core::ffi::c_void) -> usize {
    0
}

/// CheckDriverMatchingIdentifier - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CheckDriverMatchingIdentifier(SerializedDataType: usize, pIdentifierToCheck: *mut core::ffi::c_void) -> usize {
    0
}

/// SetAutoBreadcrumbsEnablement - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetAutoBreadcrumbsEnablement(Enablement: usize) -> usize {
    0
}

/// SetPageFaultEnablement - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetPageFaultEnablement(Enablement: usize) -> usize {
    0
}

/// SetWatsonDumpEnablement - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetWatsonDumpEnablement(Enablement: usize) -> usize {
    0
}

/// SetBreadcrumbContextEnablement - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetBreadcrumbContextEnablement(Enablement: usize) -> usize {
    0
}

/// UseMarkersOnlyAutoBreadcrumbs - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn UseMarkersOnlyAutoBreadcrumbs(MarkersOnly: i32) -> usize {
    0
}

/// GetAutoBreadcrumbsOutput - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetAutoBreadcrumbsOutput(pOutput: *mut core::ffi::c_void) -> usize {
    0
}

/// GetPageFaultAllocationOutput - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetPageFaultAllocationOutput(pOutput: *mut core::ffi::c_void) -> usize {
    0
}

/// GetAutoBreadcrumbsOutput1 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetAutoBreadcrumbsOutput1(pOutput: *mut core::ffi::c_void) -> usize {
    0
}

/// GetPageFaultAllocationOutput1 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetPageFaultAllocationOutput1(pOutput: *mut core::ffi::c_void) -> usize {
    0
}

/// GetPageFaultAllocationOutput2 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetPageFaultAllocationOutput2(pOutput: *mut core::ffi::c_void) -> usize {
    0
}

/// GetDeviceState - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetDeviceState() -> usize {
    0
}

/// SetBackgroundProcessingMode - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetBackgroundProcessingMode(Mode: usize, MeasurementsAction: usize, hEventToSignalUponCompletion: usize, pbFurtherMeasurementsDesired: *mut core::ffi::c_void) -> usize {
    0
}

/// AddToStateObject - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn AddToStateObject(pAddition: *mut core::ffi::c_void, pStateObjectToGrowFrom: *mut core::ffi::c_void, riid: usize, ppNewStateObject: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateProtectedResourceSession1 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateProtectedResourceSession1(pDesc: *mut core::ffi::c_void, riid: usize, ppSession: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateCommittedResource2 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateCommittedResource2(pHeapProperties: *mut core::ffi::c_void, HeapFlags: usize, pDesc: *mut core::ffi::c_void, InitialResourceState: usize, pOptimizedClearValue: *mut core::ffi::c_void, pProtectedSession: *mut core::ffi::c_void, riidResource: usize, ppvResource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreatePlacedResource1 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreatePlacedResource1(pHeap: *mut core::ffi::c_void, HeapOffset: usize, pDesc: *mut core::ffi::c_void, InitialState: usize, pOptimizedClearValue: *mut core::ffi::c_void, riid: usize, ppvResource: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateSamplerFeedbackUnorderedAccessView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateSamplerFeedbackUnorderedAccessView(pTargetedResource: *mut core::ffi::c_void, pFeedbackResource: *mut core::ffi::c_void, DestDescriptor: usize) -> usize {
    0
}

/// GetProtectedResourceSession - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetProtectedResourceSession(riid: usize, ppProtectedSession: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SetProtectedResourceSession - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetProtectedResourceSession(pProtectedResourceSession: *mut core::ffi::c_void) -> usize {
    0
}

/// GetRequiredParameterResourceSize - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetRequiredParameterResourceSize(Stage: usize, ParameterIndex: usize) -> usize {
    0
}

/// EndRenderPass - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn EndRenderPass() -> usize {
    0
}

/// ExecuteMetaCommand - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ExecuteMetaCommand(pMetaCommand: *mut core::ffi::c_void, arg1: usize) -> usize {
    0
}

/// BuildRaytracingAccelerationStructure - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn BuildRaytracingAccelerationStructure(pDesc: *mut core::ffi::c_void, NumPostbuildInfoDescs: usize, arg2: usize) -> usize {
    0
}

/// EmitRaytracingAccelerationStructurePostbuildInfo - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn EmitRaytracingAccelerationStructurePostbuildInfo(pDesc: *mut core::ffi::c_void, NumSourceAccelerationStructures: usize, NumSourceAccelerationStructures_2: usize) -> usize {
    0
}

/// CopyRaytracingAccelerationStructure - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CopyRaytracingAccelerationStructure(DestAccelerationStructureData: usize, SourceAccelerationStructureData: usize, Mode: usize) -> usize {
    0
}

/// SetPipelineState1 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetPipelineState1(pStateObject: *mut core::ffi::c_void) -> usize {
    0
}

/// DispatchRays - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn DispatchRays(pDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// SetDeleteOnDestroy - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetDeleteOnDestroy() -> usize {
    0
}

/// CreateShaderCacheSession - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateShaderCacheSession(pDesc: *mut core::ffi::c_void, riid: usize, ppvSession: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// ShaderCacheControl - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ShaderCacheControl(Kinds: usize, Control: usize) -> usize {
    0
}

/// CreateCommandQueue1 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateCommandQueue1(pDesc: *mut core::ffi::c_void, CreatorID: usize, riid: usize, ppCommandQueue: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CreateCommittedResource3 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateCommittedResource3(pHeapProperties: *mut core::ffi::c_void, HeapFlags: usize, pDesc: *mut core::ffi::c_void, InitialLayout: usize, pOptimizedClearValue: *mut core::ffi::c_void, pProtectedSession: *mut core::ffi::c_void, NumCastableFormats: usize, arg7: usize) -> usize {
    0
}

/// CreatePlacedResource2 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreatePlacedResource2(pHeap: *mut core::ffi::c_void, HeapOffset: usize, pDesc: *mut core::ffi::c_void, InitialLayout: usize, pOptimizedClearValue: *mut core::ffi::c_void, NumCastableFormats: usize, arg6: usize) -> usize {
    0
}

/// CreateReservedResource2 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateReservedResource2(pDesc: *mut core::ffi::c_void, InitialLayout: usize, pOptimizedClearValue: *mut core::ffi::c_void, pProtectedSession: *mut core::ffi::c_void, NumCastableFormats: usize, arg5: usize) -> usize {
    0
}

/// CreateSampler2 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateSampler2(pDesc: *mut core::ffi::c_void, DestDescriptor: usize) -> usize {
    0
}

/// OpenExistingHeapFromAddress1 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn OpenExistingHeapFromAddress1(pAddress: *mut core::ffi::c_void, size: usize, riid: usize, ppvHeap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// RegisterTrimNotificationCallback - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn RegisterTrimNotificationCallback(pData: *mut core::ffi::c_void) -> usize {
    0
}

/// UnregisterTrimNotificationCallback - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn UnregisterTrimNotificationCallback(CallbackCookie: u32) -> usize {
    0
}

/// TryCreateShaderResourceView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn TryCreateShaderResourceView(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, DestDescriptor: usize) -> usize {
    0
}

/// TryCreateUnorderedAccessView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn TryCreateUnorderedAccessView(pResource: *mut core::ffi::c_void, pCounterResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, DestDescriptor: usize) -> usize {
    0
}

/// TryCreateConstantBufferView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn TryCreateConstantBufferView(pDesc: *mut core::ffi::c_void, DestDescriptor: usize) -> usize {
    0
}

/// TryCreateSampler2 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn TryCreateSampler2(pDesc: *mut core::ffi::c_void, DestDescriptor: usize) -> usize {
    0
}

/// TryCreateRenderTargetView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn TryCreateRenderTargetView(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, DestDescriptor: usize) -> usize {
    0
}

/// TryCreateDepthStencilView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn TryCreateDepthStencilView(pResource: *mut core::ffi::c_void, pDesc: *mut core::ffi::c_void, DestDescriptor: usize) -> usize {
    0
}

/// TryCreateSamplerFeedbackUnorderedAccessView - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn TryCreateSamplerFeedbackUnorderedAccessView(pTargetedResource: *mut core::ffi::c_void, pFeedbackResource: *mut core::ffi::c_void, DestDescriptor: usize) -> usize {
    0
}

/// CreateQueryHeap1 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateQueryHeap1(pDesc: *mut core::ffi::c_void, Flags: usize, riid: usize, ppvHeap: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SetApplicationDesc - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetApplicationDesc(pApplicationDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// GetApplicationDesc - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetApplicationDesc(CallbackFunc: usize, pContext: *mut core::ffi::c_void) -> usize {
    0
}

/// FindStateObjectDesc - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn FindStateObjectDesc(arg0: usize) -> usize {
    0
}

/// FindObjectVersion - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn FindObjectVersion(arg0: usize) -> usize {
    0
}

/// ShareWithHost - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ShareWithHost(pObject: *mut core::ffi::c_void, pHandle: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateFenceFd - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateFenceFd(pFence: *mut core::ffi::c_void, FenceValue: usize, pFenceFd: *mut core::ffi::c_void) -> usize {
    0
}

/// EnableShaderInstrumentation - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn EnableShaderInstrumentation(bEnable: i32) -> usize {
    0
}

/// ShaderInstrumentationEnabled - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ShaderInstrumentationEnabled() -> usize {
    0
}

/// ReserveGPUVARangesAtCreate - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ReserveGPUVARangesAtCreate(arg0: usize) -> usize {
    0
}

/// ClearReservedGPUVARangesList - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ClearReservedGPUVARangesList() -> usize {
    0
}

/// SetApplicationSpecificDriverState - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetApplicationSpecificDriverState(pAdapter: *mut core::ffi::c_void, pBlob: *mut core::ffi::c_void) -> usize {
    0
}

/// DisableFailuresFromStricterValidationInAppLocalRuntime - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn DisableFailuresFromStricterValidationInAppLocalRuntime(bDisable: i32) -> usize {
    0
}

/// FailuresFromStricterValidationInAppLocalRuntimeDisabled - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn FailuresFromStricterValidationInAppLocalRuntimeDisabled() -> usize {
    0
}

/// GetAllocation - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetAllocation(pAllocation: *mut core::ffi::c_void) -> usize {
    0
}

/// SetNextAllocationAddress - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetNextAllocationAddress(nextAllocationVirtualAddress: usize) -> usize {
    0
}

/// GetApplicationSpecificDriverState - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetApplicationSpecificDriverState(ppBlob: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetApplicationSpecificDriverBlobStatus - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetApplicationSpecificDriverBlobStatus() -> usize {
    0
}

/// D3D12CreateDevice - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn D3D12CreateDevice(pAdapter: *mut core::ffi::c_void, MinimumFeatureLevel: usize, riid: usize, ppDevice: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// SetSDKVersion - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetSDKVersion(SDKVersion: u32, SDKPath: usize) -> usize {
    0
}

/// CreateDeviceFactory - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateDeviceFactory(SDKVersion: u32, SDKPath: usize, riid: usize, ppvFactory: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// FreeUnusedSDKs - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn FreeUnusedSDKs() -> usize {
    0
}

/// InitializeFromGlobalState - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn InitializeFromGlobalState() -> usize {
    0
}

/// ApplyToGlobalState - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn ApplyToGlobalState() -> usize {
    0
}

/// SetFlags - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetFlags(flags: usize) -> usize {
    0
}

/// GetConfigurationInterface - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetConfigurationInterface(clsid: usize, iid: usize, ppv: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetEnabledExperimentalFeatures - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetEnabledExperimentalFeatures(arg0: usize) -> usize {
    0
}

/// SerializeVersionedRootSignature - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SerializeVersionedRootSignature(pDesc: *mut core::ffi::c_void, ppResult: *mut *mut core::ffi::c_void, arg2: usize) -> usize {
    0
}

/// CreateVersionedRootSignatureDeserializer - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateVersionedRootSignatureDeserializer(arg0: usize) -> usize {
    0
}

/// CreateStateObjectDatabaseFromFile - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateStateObjectDatabaseFromFile(pDatabaseFile: *const u16, flags: usize, riid: usize, ppvStateObjectDatabase: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SetApplicationIdentity - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetApplicationIdentity(pDesc: *mut core::ffi::c_void, AppId: usize) -> usize {
    0
}

/// RSSetShadingRate - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn RSSetShadingRate(baseShadingRate: usize, arg1: usize) -> usize {
    0
}

/// RSSetShadingRateImage - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn RSSetShadingRateImage(shadingRateImage: *mut core::ffi::c_void) -> usize {
    0
}

/// DispatchMesh - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn DispatchMesh(ThreadGroupCountX: usize, ThreadGroupCountY: usize, ThreadGroupCountZ: usize) -> usize {
    0
}

/// Barrier - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn Barrier(NumBarrierGroups: usize, arg1: usize) -> usize {
    0
}

/// OMSetFrontAndBackStencilRef - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn OMSetFrontAndBackStencilRef(FrontStencilRef: usize, BackStencilRef: usize) -> usize {
    0
}

/// RSSetDepthBias - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn RSSetDepthBias(DepthBias: usize, DepthBiasClamp: usize, SlopeScaledDepthBias: usize) -> usize {
    0
}

/// IASetIndexBufferStripCutValue - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn IASetIndexBufferStripCutValue(IBStripCutValue: usize) -> usize {
    0
}

/// SetProgram - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn SetProgram(pDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// DispatchGraph - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn DispatchGraph(pDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateDSRDevice - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn CreateDSRDevice(pD3D12Device: *mut core::ffi::c_void, NodeMask: u32, riid: usize, ppvDSRDevice: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetGBVEntireSubresourceStatesData - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetGBVEntireSubresourceStatesData(pResource: *mut core::ffi::c_void, arg1: usize) -> usize {
    0
}

/// GetGBVSubresourceState - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetGBVSubresourceState(pResource: *mut core::ffi::c_void, Subresource: u32, pData: *mut core::ffi::c_void) -> usize {
    0
}

/// GetGBVResourceUniformState - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetGBVResourceUniformState(pResource: *mut core::ffi::c_void, pData: *mut core::ffi::c_void) -> usize {
    0
}

/// GetGBVResourceInfo - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetGBVResourceInfo(pResource: *mut core::ffi::c_void, pResourceDesc: *mut core::ffi::c_void, pResourceHash: *mut core::ffi::c_void, pSubresourceStatesByteOffset: *mut core::ffi::c_void) -> usize {
    0
}

/// GBVReserved0 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GBVReserved0() -> usize {
    0
}

/// GBVReserved1 - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GBVReserved1() -> usize {
    0
}

/// GetStateObjectStatistics - from DirectX-Headers/d3d12.h
#[no_mangle]
pub unsafe extern "C" fn GetStateObjectStatistics(pStatistics: *mut core::ffi::c_void) -> usize {
    0
}

/// ReflectSharedProperties - from DirectX-Headers/d3d12compatibility.h
#[no_mangle]
pub unsafe extern "C" fn ReflectSharedProperties(pHeapOrResource: *mut core::ffi::c_void, ReflectType: usize, arg2: usize) -> usize {
    0
}

/// D3D12CompilerSerializeVersionedRootSignature - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn D3D12CompilerSerializeVersionedRootSignature(pRootSignature: *mut core::ffi::c_void, ppBlob: *mut *mut core::ffi::c_void, arg2: usize) -> i32 {
    0
}

/// FindGroup - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn FindGroup(pGroupKey: *mut core::ffi::c_void, pGroupVersion: *mut core::ffi::c_void) -> usize {
    0
}

/// FindGroupValueKeys - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn FindGroupValueKeys(pGroupKey: *mut core::ffi::c_void, pExpectedGroupVersion: *mut core::ffi::c_void, CallbackFunc: usize, pContext: *mut core::ffi::c_void) -> usize {
    0
}

/// FindGroupValues - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn FindGroupValues(pGroupKey: *mut core::ffi::c_void, pExpectedGroupVersion: *mut core::ffi::c_void, ValueTypeFlags: usize, CallbackFunc: usize, pContext: *mut core::ffi::c_void) -> usize {
    0
}

/// GetCompilerTarget - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn GetCompilerTarget() -> usize {
    0
}

/// GetValueTypes - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn GetValueTypes() -> usize {
    0
}

/// StoreGroupValueKeys - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn StoreGroupValueKeys(pGroupKey: *mut core::ffi::c_void, GroupVersion: u32, arg2: usize) -> usize {
    0
}

/// StoreValue - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn StoreValue(pValueKey: *mut core::ffi::c_void, arg1: usize) -> usize {
    0
}

/// GetCompiler - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn GetCompiler(riid: usize, ppCompiler: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CompilePipelineState - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn CompilePipelineState(pGroupKey: *mut core::ffi::c_void, GroupVersion: u32, pDesc: *mut core::ffi::c_void) -> usize {
    0
}

/// CompileStateObject - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn CompileStateObject(pGroupKey: *mut core::ffi::c_void, GroupVersion: u32, pDesc: *mut core::ffi::c_void, riid: usize, ppCompilerStateObject: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CompileAddToStateObject - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn CompileAddToStateObject(pGroupKey: *mut core::ffi::c_void, GroupVersion: u32, pAddition: *mut core::ffi::c_void, pCompilerStateObjectToGrowFrom: *mut core::ffi::c_void, riid: usize, ppNewCompilerStateObject: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetCacheSession - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn GetCacheSession(riid: usize, ppCompilerCacheSession: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// EnumerateAdapterFamilies - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn EnumerateAdapterFamilies(AdapterFamilyIndex: u32, pAdapterFamily: *mut core::ffi::c_void) -> usize {
    0
}

/// EnumerateAdapterFamilyABIVersions - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn EnumerateAdapterFamilyABIVersions(AdapterFamilyIndex: u32, pNumABIVersions: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// EnumerateAdapterFamilyCompilerVersion - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn EnumerateAdapterFamilyCompilerVersion(AdapterFamilyIndex: u32, pCompilerVersion: *mut core::ffi::c_void) -> usize {
    0
}

/// GetApplicationProfileVersion - from DirectX-Headers/d3d12compiler.h
#[no_mangle]
pub unsafe extern "C" fn GetApplicationProfileVersion(pTarget: *mut core::ffi::c_void, pApplicationDesc: *mut core::ffi::c_void, pApplicationProfileVersion: *mut core::ffi::c_void) -> usize {
    0
}

/// EnableDebugLayer - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn EnableDebugLayer() -> usize {
    0
}

/// SetEnableGPUBasedValidation - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn SetEnableGPUBasedValidation(Enable: i32) -> usize {
    0
}

/// SetEnableSynchronizedCommandQueueValidation - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn SetEnableSynchronizedCommandQueueValidation(Enable: i32) -> usize {
    0
}

/// SetGPUBasedValidationFlags - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn SetGPUBasedValidationFlags(Flags: usize) -> usize {
    0
}

/// DisableDebugLayer - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn DisableDebugLayer() -> usize {
    0
}

/// SetEnableAutoName - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn SetEnableAutoName(Enable: i32) -> usize {
    0
}

/// SetForceLegacyBarrierValidation - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn SetForceLegacyBarrierValidation(Enable: i32) -> usize {
    0
}

/// SetDebugParameter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn SetDebugParameter(Type: usize, arg1: usize) -> usize {
    0
}

/// GetDebugParameter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn GetDebugParameter(Type: usize, arg1: usize) -> usize {
    0
}

/// ReportLiveDeviceObjects - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn ReportLiveDeviceObjects(Flags: usize) -> usize {
    0
}

/// SetFeatureMask - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn SetFeatureMask(Mask: usize) -> usize {
    0
}

/// GetFeatureMask - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn GetFeatureMask() -> usize {
    0
}

/// AssertResourceState - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn AssertResourceState(pResource: *mut core::ffi::c_void, Subresource: u32, State: u32) -> usize {
    0
}

/// AssertResourceAccess - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn AssertResourceAccess(pResource: *mut core::ffi::c_void, Subresource: u32, Access: usize) -> usize {
    0
}

/// AssertTextureLayout - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn AssertTextureLayout(pResource: *mut core::ffi::c_void, Subresource: u32, Layout: usize) -> usize {
    0
}

/// SharedFenceSignal - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn SharedFenceSignal(pFence: *mut core::ffi::c_void, FenceValue: usize) -> usize {
    0
}

/// BeginCapturableWork - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn BeginCapturableWork(guid: usize) -> usize {
    0
}

/// EndCapturableWork - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn EndCapturableWork(guid: usize) -> usize {
    0
}

/// SetMessageCountLimit - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn SetMessageCountLimit(MessageCountLimit: usize) -> usize {
    0
}

/// ClearStoredMessages - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn ClearStoredMessages() -> usize {
    0
}

/// GetNumMessagesAllowedByStorageFilter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn GetNumMessagesAllowedByStorageFilter() -> usize {
    0
}

/// GetNumMessagesDeniedByStorageFilter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn GetNumMessagesDeniedByStorageFilter() -> usize {
    0
}

/// GetNumStoredMessages - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn GetNumStoredMessages() -> usize {
    0
}

/// GetNumStoredMessagesAllowedByRetrievalFilter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn GetNumStoredMessagesAllowedByRetrievalFilter() -> usize {
    0
}

/// GetNumMessagesDiscardedByMessageCountLimit - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn GetNumMessagesDiscardedByMessageCountLimit() -> usize {
    0
}

/// AddStorageFilterEntries - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn AddStorageFilterEntries(pFilter: *mut core::ffi::c_void) -> usize {
    0
}

/// GetStorageFilter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn GetStorageFilter(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// ClearStorageFilter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn ClearStorageFilter() -> usize {
    0
}

/// PushEmptyStorageFilter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn PushEmptyStorageFilter() -> usize {
    0
}

/// PushCopyOfStorageFilter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn PushCopyOfStorageFilter() -> usize {
    0
}

/// PushStorageFilter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn PushStorageFilter(pFilter: *mut core::ffi::c_void) -> usize {
    0
}

/// PopStorageFilter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn PopStorageFilter() -> usize {
    0
}

/// GetStorageFilterStackSize - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn GetStorageFilterStackSize() -> usize {
    0
}

/// AddRetrievalFilterEntries - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn AddRetrievalFilterEntries(pFilter: *mut core::ffi::c_void) -> usize {
    0
}

/// GetRetrievalFilter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn GetRetrievalFilter(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// ClearRetrievalFilter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn ClearRetrievalFilter() -> usize {
    0
}

/// PushEmptyRetrievalFilter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn PushEmptyRetrievalFilter() -> usize {
    0
}

/// PushCopyOfRetrievalFilter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn PushCopyOfRetrievalFilter() -> usize {
    0
}

/// PushRetrievalFilter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn PushRetrievalFilter(pFilter: *mut core::ffi::c_void) -> usize {
    0
}

/// PopRetrievalFilter - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn PopRetrievalFilter() -> usize {
    0
}

/// GetRetrievalFilterStackSize - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn GetRetrievalFilterStackSize() -> usize {
    0
}

/// AddMessage - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn AddMessage(Category: usize, Severity: usize, ID: usize, pDescription: usize) -> usize {
    0
}

/// AddApplicationMessage - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn AddApplicationMessage(Severity: usize, pDescription: usize) -> usize {
    0
}

/// SetBreakOnCategory - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn SetBreakOnCategory(Category: usize, bEnable: usize) -> usize {
    0
}

/// SetBreakOnSeverity - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn SetBreakOnSeverity(Severity: usize, bEnable: usize) -> usize {
    0
}

/// SetBreakOnID - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn SetBreakOnID(ID: usize, bEnable: usize) -> usize {
    0
}

/// GetBreakOnCategory - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn GetBreakOnCategory(Category: usize) -> usize {
    0
}

/// GetBreakOnSeverity - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn GetBreakOnSeverity(Severity: usize) -> usize {
    0
}

/// GetBreakOnID - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn GetBreakOnID(ID: usize) -> usize {
    0
}

/// SetMuteDebugOutput - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn SetMuteDebugOutput(bMute: usize) -> usize {
    0
}

/// GetMuteDebugOutput - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn GetMuteDebugOutput() -> usize {
    0
}

/// RegisterMessageCallback - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn RegisterMessageCallback(CallbackFunc: usize, CallbackFilterFlags: usize, pContext: *mut core::ffi::c_void, pCallbackCookie: *mut core::ffi::c_void) -> usize {
    0
}

/// UnregisterMessageCallback - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn UnregisterMessageCallback(CallbackCookie: usize) -> usize {
    0
}

/// GetNodeMask - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn GetNodeMask() -> usize {
    0
}

/// GetNumInputStreamDescs - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn GetNumInputStreamDescs() -> usize {
    0
}

/// GetInputStreamDescs - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn GetInputStreamDescs(NumInputStreamDescs: u32, arg1: usize) -> usize {
    0
}

/// GetOutputStreamDesc - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn GetOutputStreamDesc() -> usize {
    0
}

/// DecodeFrame - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn DecodeFrame(pDecoder: *mut core::ffi::c_void, pOutputArguments: *mut core::ffi::c_void, pInputArguments: *mut core::ffi::c_void) -> usize {
    0
}

/// ProcessFrames - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn ProcessFrames(pVideoProcessor: *mut core::ffi::c_void, pOutputArguments: *mut core::ffi::c_void, NumInputStreams: u32, arg3: usize) -> usize {
    0
}

/// DecodeFrame1 - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn DecodeFrame1(pDecoder: *mut core::ffi::c_void, pOutputArguments: *mut core::ffi::c_void, pInputArguments: *mut core::ffi::c_void) -> usize {
    0
}

/// ProcessFrames1 - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn ProcessFrames1(pVideoProcessor: *mut core::ffi::c_void, pOutputArguments: *mut core::ffi::c_void, NumInputStreams: u32, arg3: usize) -> usize {
    0
}

/// EstimateMotion - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn EstimateMotion(pMotionEstimator: *mut core::ffi::c_void, pOutputArguments: *mut core::ffi::c_void, pInputArguments: *mut core::ffi::c_void) -> usize {
    0
}

/// ResolveMotionVectorHeap - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn ResolveMotionVectorHeap(pOutputArguments: *mut core::ffi::c_void, pInputArguments: *mut core::ffi::c_void) -> usize {
    0
}

/// InitializeExtensionCommand - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn InitializeExtensionCommand(pExtensionCommand: *mut core::ffi::c_void, arg1: usize) -> usize {
    0
}

/// ExecuteExtensionCommand - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn ExecuteExtensionCommand(pExtensionCommand: *mut core::ffi::c_void, arg1: usize) -> usize {
    0
}

/// GetEncoderFlags - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn GetEncoderFlags() -> usize {
    0
}

/// GetCodec - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn GetCodec() -> usize {
    0
}

/// GetCodecProfile - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn GetCodecProfile(dstProfile: usize) -> usize {
    0
}

/// GetCodecConfiguration - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn GetCodecConfiguration(dstCodecConfig: usize) -> usize {
    0
}

/// GetInputFormat - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn GetInputFormat() -> usize {
    0
}

/// GetMaxMotionEstimationPrecision - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn GetMaxMotionEstimationPrecision() -> usize {
    0
}

/// GetEncoderHeapFlags - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn GetEncoderHeapFlags() -> usize {
    0
}

/// GetCodecLevel - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn GetCodecLevel(dstLevel: usize) -> usize {
    0
}

/// GetResolutionListCount - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn GetResolutionListCount() -> usize {
    0
}

/// GetResolutionList - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn GetResolutionList(ResolutionsListCount: u32, arg1: usize) -> usize {
    0
}

/// EncodeFrame - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn EncodeFrame(pEncoder: *mut core::ffi::c_void, pHeap: *mut core::ffi::c_void, pInputArguments: *mut core::ffi::c_void, pOutputArguments: *mut core::ffi::c_void) -> usize {
    0
}

/// ResolveEncoderOutputMetadata - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn ResolveEncoderOutputMetadata(pInputArguments: *mut core::ffi::c_void, pOutputArguments: *mut core::ffi::c_void) -> usize {
    0
}

/// EncodeFrame1 - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn EncodeFrame1(pEncoder: *mut core::ffi::c_void, pHeap: *mut core::ffi::c_void, pInputArguments: *mut core::ffi::c_void, pOutputArguments: *mut core::ffi::c_void) -> usize {
    0
}

/// ResolveEncoderOutputMetadata1 - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn ResolveEncoderOutputMetadata1(pInputArguments: *mut core::ffi::c_void, pOutputArguments: *mut core::ffi::c_void) -> usize {
    0
}

/// ResolveInputParamLayout - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn ResolveInputParamLayout(pInputArguments: *mut core::ffi::c_void, pOutputArguments: *mut core::ffi::c_void) -> usize {
    0
}

/// GetBufferPointer - from DirectX-Headers/d3dcommon.h
#[no_mangle]
pub unsafe extern "C" fn GetBufferPointer() -> usize {
    0
}

/// GetBufferSize - from DirectX-Headers/d3dcommon.h
#[no_mangle]
pub unsafe extern "C" fn GetBufferSize() -> usize {
    0
}

/// GetInstallerName - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetInstallerName(pNameLength: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// GetInstallerScope - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetInstallerScope() -> usize {
    0
}

/// HandleDriverUpdate - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn HandleDriverUpdate(pInstaller: *mut core::ffi::c_void) -> usize {
    0
}

/// GetComponentName - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetComponentName(pName: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetStateObjectDatabasePath - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetStateObjectDatabasePath(pPath: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetPrecompiledCachePath - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetPrecompiledCachePath(pAdapterFamily: *mut core::ffi::c_void, pPath: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetPrecompiledShaderDatabaseCount - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetPrecompiledShaderDatabaseCount() -> usize {
    0
}

/// GetPrecompiledShaderDatabases - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetPrecompiledShaderDatabases(ArraySize: u32, arg1: usize) -> usize {
    0
}

/// GetExePath - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetExePath(pExePath: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// RegisterComponent - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn RegisterComponent(pName: *mut core::ffi::c_void, pStateObjectDBPath: *mut core::ffi::c_void, NumPSDB: usize, arg3: usize) -> usize {
    0
}

/// RemoveComponent - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn RemoveComponent(pComponent: *mut core::ffi::c_void) -> usize {
    0
}

/// GetComponentCount - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetComponentCount() -> usize {
    0
}

/// GetComponent - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetComponent(index: usize, riid: usize, ppvComponent: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetPrecompileTargetCount - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetPrecompileTargetCount(flags: usize) -> usize {
    0
}

/// GetPrecompileTargets - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetPrecompileTargets(ArraySize: usize, arg1: usize) -> usize {
    0
}

/// RegisterDriverUpdateListener - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn RegisterDriverUpdateListener() -> usize {
    0
}

/// UnregisterDriverUpdateListener - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn UnregisterDriverUpdateListener() -> usize {
    0
}

/// RegisterServiceDriverUpdateTrigger - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn RegisterServiceDriverUpdateTrigger(hServiceHandle: usize) -> usize {
    0
}

/// UnregisterServiceDriverUpdateTrigger - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn UnregisterServiceDriverUpdateTrigger(hServiceHandle: usize) -> usize {
    0
}

/// RegisterApplication - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn RegisterApplication(pExePath: *mut core::ffi::c_void, pApplicationDesc: *mut core::ffi::c_void, riid: usize, ppvApp: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// RemoveApplication - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn RemoveApplication(pApplication: *mut core::ffi::c_void) -> usize {
    0
}

/// GetApplicationCount - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetApplicationCount() -> usize {
    0
}

/// GetApplication - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetApplication(index: usize, riid: usize, ppvApp: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// ClearAllState - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn ClearAllState() -> usize {
    0
}

/// GetMaxPrecompileTargetCount - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetMaxPrecompileTargetCount() -> usize {
    0
}

/// GetApplicationFromExePath - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn GetApplicationFromExePath(pFullExePath: *mut core::ffi::c_void, riid: usize, ppvApp: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// Transition - from DirectX-Headers/d3dx12_barriers.h
#[no_mangle]
pub unsafe extern "C" fn Transition(pResource: *mut core::ffi::c_void, stateBefore: usize, stateAfter: usize, D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES: usize, D3D12_RESOURCE_BARRIER_FLAG_NONE: usize) -> usize {
    0
}

/// UAV - from DirectX-Headers/d3dx12_barriers.h
#[no_mangle]
pub unsafe extern "C" fn UAV(pResource: *mut core::ffi::c_void) -> usize {
    0
}

/// DoublePrecisionFloatShaderOps - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn DoublePrecisionFloatShaderOps() -> i32 {
    0
}

/// MinPrecisionSupport - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn MinPrecisionSupport() -> usize {
    0
}

/// TiledResourcesTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn TiledResourcesTier() -> usize {
    0
}

/// ResourceBindingTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ResourceBindingTier() -> usize {
    0
}

/// PSSpecifiedStencilRefSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn PSSpecifiedStencilRefSupported() -> i32 {
    0
}

/// TypedUAVLoadAdditionalFormats - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn TypedUAVLoadAdditionalFormats() -> i32 {
    0
}

/// ROVsSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ROVsSupported() -> i32 {
    0
}

/// ConservativeRasterizationTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ConservativeRasterizationTier() -> usize {
    0
}

/// CrossAdapterRowMajorTextureSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn CrossAdapterRowMajorTextureSupported() -> i32 {
    0
}

/// VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation() -> i32 {
    0
}

/// ResourceHeapTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ResourceHeapTier() -> usize {
    0
}

/// CrossNodeSharingTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn CrossNodeSharingTier() -> usize {
    0
}

/// MaxGPUVirtualAddressBitsPerResource - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn MaxGPUVirtualAddressBitsPerResource() -> u32 {
    0
}

/// MaxSupportedFeatureLevel - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn MaxSupportedFeatureLevel() -> usize {
    0
}

/// FormatSupport - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn FormatSupport(Format: usize, Support1: usize, Support2: usize) -> i32 {
    0
}

/// MultisampleQualityLevels - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn MultisampleQualityLevels(Format: usize, SampleCount: u32, Flags: usize, NumQualityLevels: usize) -> i32 {
    0
}

/// FormatInfo - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn FormatInfo(Format: usize, PlaneCount: usize) -> i32 {
    0
}

/// MaxGPUVirtualAddressBitsPerProcess - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn MaxGPUVirtualAddressBitsPerProcess() -> u32 {
    0
}

/// HighestShaderModel - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn HighestShaderModel() -> usize {
    0
}

/// WaveOps - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn WaveOps() -> i32 {
    0
}

/// WaveLaneCountMin - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn WaveLaneCountMin() -> u32 {
    0
}

/// WaveLaneCountMax - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn WaveLaneCountMax() -> u32 {
    0
}

/// TotalLaneCount - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn TotalLaneCount() -> u32 {
    0
}

/// Int64ShaderOps - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn Int64ShaderOps() -> i32 {
    0
}

/// ProtectedResourceSessionSupport - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ProtectedResourceSessionSupport(arg0: usize) -> usize {
    0
}

/// HighestRootSignatureVersion - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn HighestRootSignatureVersion() -> usize {
    0
}

/// TileBasedRenderer - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn TileBasedRenderer(arg0: usize) -> i32 {
    0
}

/// UMA - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn UMA(arg0: usize) -> i32 {
    0
}

/// CacheCoherentUMA - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn CacheCoherentUMA(arg0: usize) -> i32 {
    0
}

/// IsolatedMMU - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn IsolatedMMU(arg0: usize) -> i32 {
    0
}

/// DepthBoundsTestSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn DepthBoundsTestSupported() -> i32 {
    0
}

/// ProgrammableSamplePositionsTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ProgrammableSamplePositionsTier() -> usize {
    0
}

/// ShaderCacheSupportFlags - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ShaderCacheSupportFlags() -> usize {
    0
}

/// CommandQueuePrioritySupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn CommandQueuePrioritySupported(CommandListType: usize, Priority: u32) -> i32 {
    0
}

/// CopyQueueTimestampQueriesSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn CopyQueueTimestampQueriesSupported() -> i32 {
    0
}

/// CastingFullyTypedFormatSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn CastingFullyTypedFormatSupported() -> i32 {
    0
}

/// BarycentricsSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn BarycentricsSupported() -> i32 {
    0
}

/// ExistingHeapsSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ExistingHeapsSupported() -> i32 {
    0
}

/// MSAA64KBAlignedTextureSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn MSAA64KBAlignedTextureSupported() -> i32 {
    0
}

/// SharedResourceCompatibilityTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn SharedResourceCompatibilityTier() -> usize {
    0
}

/// Native16BitShaderOpsSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn Native16BitShaderOpsSupported() -> i32 {
    0
}

/// HeapSerializationTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn HeapSerializationTier(arg0: usize) -> usize {
    0
}

/// CrossNodeAtomicShaderInstructions - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn CrossNodeAtomicShaderInstructions() -> i32 {
    0
}

/// SRVOnlyTiledResourceTier3 - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn SRVOnlyTiledResourceTier3() -> i32 {
    0
}

/// RenderPassesTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn RenderPassesTier() -> usize {
    0
}

/// RaytracingTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn RaytracingTier() -> usize {
    0
}

/// DisplayableTexture - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn DisplayableTexture() -> i32 {
    0
}

/// AdditionalShadingRatesSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn AdditionalShadingRatesSupported() -> i32 {
    0
}

/// PerPrimitiveShadingRateSupportedWithViewportIndexing - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn PerPrimitiveShadingRateSupportedWithViewportIndexing() -> i32 {
    0
}

/// VariableShadingRateTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn VariableShadingRateTier() -> usize {
    0
}

/// ShadingRateImageTileSize - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ShadingRateImageTileSize() -> u32 {
    0
}

/// QueryMetaCommand - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn QueryMetaCommand(dQueryMetaCommand: usize) -> i32 {
    0
}

/// MeshShaderTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn MeshShaderTier() -> usize {
    0
}

/// SamplerFeedbackTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn SamplerFeedbackTier() -> usize {
    0
}

/// ProtectedResourceSessionTypeCount - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ProtectedResourceSessionTypeCount(arg0: usize) -> u32 {
    0
}

/// MeshShaderPipelineStatsSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn MeshShaderPipelineStatsSupported() -> i32 {
    0
}

/// MeshShaderSupportsFullRangeRenderTargetArrayIndex - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn MeshShaderSupportsFullRangeRenderTargetArrayIndex() -> i32 {
    0
}

/// AtomicInt64OnTypedResourceSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn AtomicInt64OnTypedResourceSupported() -> i32 {
    0
}

/// AtomicInt64OnGroupSharedSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn AtomicInt64OnGroupSharedSupported() -> i32 {
    0
}

/// WaveMMATier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn WaveMMATier() -> usize {
    0
}

/// VariableRateShadingSumCombinerSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn VariableRateShadingSumCombinerSupported() -> i32 {
    0
}

/// MeshShaderPerPrimitiveShadingRateSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn MeshShaderPerPrimitiveShadingRateSupported() -> i32 {
    0
}

/// AtomicInt64OnDescriptorHeapResourceSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn AtomicInt64OnDescriptorHeapResourceSupported() -> i32 {
    0
}

/// MSPrimitivesPipelineStatisticIncludesCulledPrimitives - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn MSPrimitivesPipelineStatisticIncludesCulledPrimitives() -> usize {
    0
}

/// EnhancedBarriersSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn EnhancedBarriersSupported() -> i32 {
    0
}

/// RelaxedFormatCastingSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn RelaxedFormatCastingSupported() -> i32 {
    0
}

/// UnrestrictedBufferTextureCopyPitchSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn UnrestrictedBufferTextureCopyPitchSupported() -> i32 {
    0
}

/// UnrestrictedVertexElementAlignmentSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn UnrestrictedVertexElementAlignmentSupported() -> i32 {
    0
}

/// InvertedViewportHeightFlipsYSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn InvertedViewportHeightFlipsYSupported() -> i32 {
    0
}

/// InvertedViewportDepthFlipsZSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn InvertedViewportDepthFlipsZSupported() -> i32 {
    0
}

/// TextureCopyBetweenDimensionsSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn TextureCopyBetweenDimensionsSupported() -> i32 {
    0
}

/// AlphaBlendFactorSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn AlphaBlendFactorSupported() -> i32 {
    0
}

/// AdvancedTextureOpsSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn AdvancedTextureOpsSupported() -> i32 {
    0
}

/// TriangleFanSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn TriangleFanSupported() -> i32 {
    0
}

/// DynamicIndexBufferStripCutSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn DynamicIndexBufferStripCutSupported() -> i32 {
    0
}

/// DynamicDepthBiasSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn DynamicDepthBiasSupported() -> i32 {
    0
}

/// GPUUploadHeapSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn GPUUploadHeapSupported() -> i32 {
    0
}

/// NonNormalizedCoordinateSamplersSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn NonNormalizedCoordinateSamplersSupported() -> i32 {
    0
}

/// RenderPassesValid - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn RenderPassesValid() -> i32 {
    0
}

/// MismatchingOutputDimensionsSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn MismatchingOutputDimensionsSupported() -> i32 {
    0
}

/// SupportedSampleCountsWithNoOutputs - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn SupportedSampleCountsWithNoOutputs() -> u32 {
    0
}

/// PointSamplingAddressesNeverRoundUp - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn PointSamplingAddressesNeverRoundUp() -> i32 {
    0
}

/// RasterizerDesc2Supported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn RasterizerDesc2Supported() -> i32 {
    0
}

/// NarrowQuadrilateralLinesSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn NarrowQuadrilateralLinesSupported() -> i32 {
    0
}

/// AnisoFilterWithPointMipSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn AnisoFilterWithPointMipSupported() -> i32 {
    0
}

/// MaxSamplerDescriptorHeapSize - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn MaxSamplerDescriptorHeapSize() -> u32 {
    0
}

/// MaxSamplerDescriptorHeapSizeWithStaticSamplers - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn MaxSamplerDescriptorHeapSizeWithStaticSamplers() -> u32 {
    0
}

/// MaxViewDescriptorHeapSize - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn MaxViewDescriptorHeapSize() -> u32 {
    0
}

/// ExecuteIndirectTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ExecuteIndirectTier() -> usize {
    0
}

/// WorkGraphsTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn WorkGraphsTier() -> usize {
    0
}

/// TightAlignmentSupportTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn TightAlignmentSupportTier() -> usize {
    0
}

/// ShaderExecutionReorderingActuallyReorders - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ShaderExecutionReorderingActuallyReorders() -> i32 {
    0
}

/// Max1DDispatchSize - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn Max1DDispatchSize() -> u32 {
    0
}

/// Max1DDispatchMeshSize - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn Max1DDispatchMeshSize() -> u32 {
    0
}

/// QueryHighestShaderModel - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn QueryHighestShaderModel() -> i32 {
    0
}

/// QueryHighestRootSignatureVersion - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn QueryHighestRootSignatureVersion() -> i32 {
    0
}

/// QueryHighestFeatureLevel - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn QueryHighestFeatureLevel() -> i32 {
    0
}

/// QueryProtectedResourceSessionTypes - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn QueryProtectedResourceSessionTypes(NodeIndex: u32, Count: u32) -> i32 {
    0
}

/// CD3DX12_RANGE - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_RANGE(o: usize) -> usize {
    0
}

/// CD3DX12_SUBRESOURCE_FOOTPRINT - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_SUBRESOURCE_FOOTPRINT(resDesc: usize, rowPitch: u32) -> usize {
    0
}

/// D3D12CalcSubresource - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn D3D12CalcSubresource(MipSlice: u32, ArraySlice: u32, PlaneSlice: u32, MipLevels: u32, ArraySize: u32) -> usize {
    0
}

/// D3D12GetFormatPlaneCount - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn D3D12GetFormatPlaneCount(pDevice: *mut core::ffi::c_void, Format: usize) -> usize {
    0
}

/// CD3DX12_RESOURCE_DESC - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_RESOURCE_DESC(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize, arg7: usize, arg8: usize, arg9: usize, arg10: usize) -> usize {
    0
}

/// Depth - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn Depth() -> usize {
    0
}

/// PlaneCount - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn PlaneCount(pDevice: *mut core::ffi::c_void) -> usize {
    0
}

/// Subresources - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn Subresources(pDevice: *mut core::ffi::c_void) -> u32 {
    0
}

/// CD3DX12_RESOURCE_DESC1 - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_RESOURCE_DESC1(o: usize) -> usize {
    0
}

/// StructuredBuffer - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn StructuredBuffer(NumElements: u32, StructureByteStride: u32, arg2: usize) -> usize {
    0
}

/// RawBuffer - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn RawBuffer(NumElements: u32, arg1: usize) -> usize {
    0
}

/// TypedBuffer - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn TypedBuffer(Format: usize, NumElements: u32, arg2: usize, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: usize) -> usize {
    0
}

/// Tex1D - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn Tex1D(Format: usize, arg1: usize, arg2: usize, arg3: usize, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: usize) -> usize {
    0
}

/// Tex1DArray - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn Tex1DArray(Format: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: usize) -> usize {
    0
}

/// Tex2D - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn Tex2D(Format: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: usize) -> usize {
    0
}

/// Tex2DArray - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn Tex2DArray(Format: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: usize) -> usize {
    0
}

/// Tex2DMS - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn Tex2DMS(Format: usize, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: usize) -> usize {
    0
}

/// Tex2DMSArray - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn Tex2DMSArray(Format: usize, ArraySize: u32, arg2: usize, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: usize) -> usize {
    0
}

/// Tex3D - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn Tex3D(Format: usize, arg1: usize, arg2: usize, arg3: usize, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: usize) -> usize {
    0
}

/// TexCube - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn TexCube(Format: usize, arg1: usize, arg2: usize, arg3: usize, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: usize) -> usize {
    0
}

/// TexCubeArray - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn TexCubeArray(Format: usize, NumCubes: u32, arg2: usize, arg3: usize, arg4: usize, arg5: usize, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: usize) -> usize {
    0
}

/// RaytracingAccelStruct - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn RaytracingAccelStruct(Location: usize) -> usize {
    0
}

/// CD3DX12_RT_FORMAT_ARRAY - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_RT_FORMAT_ARRAY(arg0: usize) -> usize {
    0
}

/// CD3DX12_SERIALIZED_ROOT_SIGNATURE_DESC - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_SERIALIZED_ROOT_SIGNATURE_DESC(arg0: usize) -> usize {
    0
}

/// ErrorUnknownSubobject - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn ErrorUnknownSubobject(arg0: u32) -> usize {
    0
}

/// FinalizeCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn FinalizeCb() -> usize {
    0
}

/// GraphicsDescV0 - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn GraphicsDescV0() -> usize {
    0
}

/// ComputeDescV0 - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn ComputeDescV0() -> usize {
    0
}

/// MeshShaderDescV0 - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn MeshShaderDescV0() -> usize {
    0
}

/// FlagsCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn FlagsCb(Flags: usize) {

}

/// NodeMaskCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn NodeMaskCb(NodeMask: u32) {

}

/// RootSignatureCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn RootSignatureCb(pRootSignature: *mut core::ffi::c_void) {

}

/// InputLayoutCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn InputLayoutCb(InputLayout: usize) {

}

/// IBStripCutValueCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn IBStripCutValueCb(IBStripCutValue: usize) {

}

/// VSCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn VSCb(VS: usize) {

}

/// GSCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn GSCb(GS: usize) {

}

/// StreamOutputCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn StreamOutputCb(StreamOutput: usize) {

}

/// HSCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn HSCb(HS: usize) {

}

/// DSCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn DSCb(DS: usize) {

}

/// PSCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn PSCb(PS: usize) {

}

/// CSCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn CSCb(CS: usize) {

}

/// ASCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn ASCb(AS: usize) {

}

/// MSCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn MSCb(MS: usize) {

}

/// BlendStateCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn BlendStateCb(BlendState: usize) {

}

/// DepthStencilStateCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn DepthStencilStateCb(DepthStencilState: usize) {

}

/// DepthStencilState1Cb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn DepthStencilState1Cb(DepthStencilState: usize) {

}

/// DSVFormatCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn DSVFormatCb(DSVFormat: usize) {

}

/// RasterizerStateCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn RasterizerStateCb(RasterizerState: usize) {

}

/// RTVFormatsCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn RTVFormatsCb(RTVFormats: usize) {

}

/// SampleDescCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn SampleDescCb(SampleDesc: usize) {

}

/// SampleMaskCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn SampleMaskCb(SampleMask: u32) {

}

/// CachedPSOCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn CachedPSOCb(CachedPSO: usize) {

}

/// DepthStencilState2Cb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn DepthStencilState2Cb(DepthStencilState: usize) {

}

/// RasterizerState1Cb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn RasterizerState1Cb(RasterizerState: usize) {

}

/// RasterizerState2Cb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn RasterizerState2Cb(RasterizerState: usize) {

}

/// D3DX12ParsePipelineStream - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn D3DX12ParsePipelineStream(Desc: usize, pCallbacks: *mut core::ffi::c_void) -> i32 {
    0
}

/// GetNumFormats - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetNumFormats() -> u32 {
    0
}

/// GetFormatTable - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetFormatTable() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// GetHighestDefinedFeatureLevel - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetHighestDefinedFeatureLevel() -> usize {
    0
}

/// GetFormat - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetFormat(Index: usize) -> usize {
    0
}

/// FormatExists - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn FormatExists(Format: usize) -> usize {
    0
}

/// GetByteAlignment - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetByteAlignment(Format: usize) -> u32 {
    0
}

/// IsSRGBFormat - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn IsSRGBFormat(Format: usize) -> usize {
    0
}

/// GetBitsPerStencil - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetBitsPerStencil(Format: usize) -> u32 {
    0
}

/// GetBitsPerDepth - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetBitsPerDepth(Format: usize) -> u32 {
    0
}

/// GetFormatReturnTypes - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetFormatReturnTypes(Format: usize, pInterpretations: *mut core::ffi::c_void) {

}

/// Sequential2AbsoluteComponentIndex - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn Sequential2AbsoluteComponentIndex(Format: usize, SequentialComponentIndex: u32) -> u32 {
    0
}

/// CanBeCastEvenFullyTyped - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn CanBeCastEvenFullyTyped(Format: usize, fl: usize) -> usize {
    0
}

/// GetParentFormat - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetParentFormat(Format: usize) -> usize {
    0
}

/// GetFormatCastSet - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetFormatCastSet(Format: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// GetTypeLevel - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetTypeLevel(Format: usize) -> usize {
    0
}

/// GetBitsPerUnit - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetBitsPerUnit(Format: usize) -> u32 {
    0
}

/// GetBitsPerUnitThrow - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetBitsPerUnitThrow(Format: usize) -> u32 {
    0
}

/// GetBitsPerElement - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetBitsPerElement(Format: usize) -> u32 {
    0
}

/// GetWidthAlignment - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetWidthAlignment(Format: usize) -> u32 {
    0
}

/// GetHeightAlignment - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetHeightAlignment(Format: usize) -> u32 {
    0
}

/// GetDepthAlignment - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetDepthAlignment(Format: usize) -> u32 {
    0
}

/// Planar - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn Planar(Format: usize) -> i32 {
    0
}

/// NonOpaquePlanar - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn NonOpaquePlanar(Format: usize) -> i32 {
    0
}

/// YUV - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn YUV(Format: usize) -> i32 {
    0
}

/// Opaque - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn Opaque(Format: usize) -> i32 {
    0
}

/// FamilySupportsStencil - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn FamilySupportsStencil(Format: usize) -> usize {
    0
}

/// NonOpaquePlaneCount - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn NonOpaquePlaneCount(Format: usize) -> u32 {
    0
}

/// DX9VertexOrIndexFormat - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn DX9VertexOrIndexFormat(Format: usize) -> i32 {
    0
}

/// DX9TextureFormat - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn DX9TextureFormat(Format: usize) -> i32 {
    0
}

/// FloatNormTextureFormat - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn FloatNormTextureFormat(Format: usize) -> i32 {
    0
}

/// DepthOnlyFormat - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn DepthOnlyFormat(format: usize) -> usize {
    0
}

/// MotionEstimatorAllowedInputFormat - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn MotionEstimatorAllowedInputFormat(Format: usize) -> usize {
    0
}

/// SupportsSamplerFeedback - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn SupportsSamplerFeedback(Format: usize) -> usize {
    0
}

/// DecodeHistogramAllowedForOutputFormatSupport - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn DecodeHistogramAllowedForOutputFormatSupport(Format: usize) -> usize {
    0
}

/// GetPlaneSliceFromViewFormat - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetPlaneSliceFromViewFormat(ResourceFormat: usize, ViewFormat: usize) -> usize {
    0
}

/// SNORMAndUNORMFormats - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn SNORMAndUNORMFormats(FormatA: usize, FormatB: usize) -> usize {
    0
}

/// ValidCastToR32UAV - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn ValidCastToR32UAV(from: usize, to: usize) -> usize {
    0
}

/// IsSupportedTextureDisplayableFormat - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn IsSupportedTextureDisplayableFormat(arg0: usize, bMediaFormatOnly: usize) -> usize {
    0
}

/// GetFormatComponentInterpretation - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetFormatComponentInterpretation(Format: usize, AbsoluteComponentIndex: u32) -> usize {
    0
}

/// GetBitsPerComponent - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetBitsPerComponent(Format: usize, AbsoluteComponentIndex: u32) -> u32 {
    0
}

/// CalculateExtraPlanarRows - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn CalculateExtraPlanarRows(format: usize, plane0Height: u32, totalHeight: usize) -> i32 {
    0
}

/// CalculateMinimumRowMajorRowPitch - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn CalculateMinimumRowMajorRowPitch(Format: usize, Width: u32, RowPitch: usize) -> i32 {
    0
}

/// CalculateMinimumRowMajorSlicePitch - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn CalculateMinimumRowMajorSlicePitch(Format: usize, ContextBasedRowPitch: u32, Height: u32, SlicePitch: usize) -> i32 {
    0
}

/// GetYCbCrChromaSubsampling - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetYCbCrChromaSubsampling(Format: usize, HorizontalSubsampling: usize, VerticalSubsampling: usize) {

}

/// CalculateResourceSize - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn CalculateResourceSize(width: u32, height: u32, depth: u32, format: usize, mipLevels: u32, subresources: u32, totalByteSize: usize, arg7: usize) -> i32 {
    0
}

/// GetTileShape - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetTileShape(pTileShape: *mut core::ffi::c_void, Format: usize, Dimension: usize, SampleCount: u32) {

}

/// Get4KTileShape - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn Get4KTileShape(pTileShape: *mut core::ffi::c_void, Format: usize, Dimension: usize, SampleCount: u32) {

}

/// GetMipDimensions - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetMipDimensions(mipSlice: usize, pWidth: *mut core::ffi::c_void, nullptr: *mut core::ffi::c_void, nullptr_3: *mut core::ffi::c_void) {

}

/// GetPlaneSubsampledSizeAndFormatForCopyableLayout - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetPlaneSubsampledSizeAndFormatForCopyableLayout(PlaneSlice: u32, Format: usize, Width: u32, Height: u32, PlaneFormat: usize, MinPlanePitchWidth: usize, PlaneWidth: usize, PlaneHeight: usize) {

}

/// GetDetailTableIndex - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetDetailTableIndex(Format: usize) -> u32 {
    0
}

/// GetDetailTableIndexNoThrow - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetDetailTableIndexNoThrow(Format: usize) -> u32 {
    0
}

/// GetDetailTableIndexThrow - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetDetailTableIndexThrow(Format: usize) -> u32 {
    0
}

/// SupportsDepth - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn SupportsDepth(Format: usize) -> usize {
    0
}

/// SupportsStencil - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn SupportsStencil(Format: usize) -> usize {
    0
}

/// GetFormatDetail - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetFormatDetail(Format: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// D3DX12ResourceDesc0ToDesc1 - from DirectX-Headers/d3dx12_resource_helpers.h
#[no_mangle]
pub unsafe extern "C" fn D3DX12ResourceDesc0ToDesc1(desc0: usize) -> usize {
    0
}

/// D3DX12GetCopyableFootprints - from DirectX-Headers/d3dx12_resource_helpers.h
#[no_mangle]
pub unsafe extern "C" fn D3DX12GetCopyableFootprints(arg0: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// InitAsShaderResourceView - from DirectX-Headers/d3dx12_root_signature.h
#[no_mangle]
pub unsafe extern "C" fn InitAsShaderResourceView(rootParam: usize, shaderRegister: u32, arg2: usize, D3D12_SHADER_VISIBILITY_ALL: usize) {

}

/// InitAsUnorderedAccessView - from DirectX-Headers/d3dx12_root_signature.h
#[no_mangle]
pub unsafe extern "C" fn InitAsUnorderedAccessView(rootParam: usize, shaderRegister: u32, arg2: usize, D3D12_SHADER_VISIBILITY_ALL: usize) {

}

/// CD3DX12_STATIC_SAMPLER_DESC1 - from DirectX-Headers/d3dx12_root_signature.h
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_STATIC_SAMPLER_DESC1(o: usize) -> usize {
    0
}

/// CD3DX12_VERSIONED_ROOT_SIGNATURE_DESC - from DirectX-Headers/d3dx12_root_signature.h
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_VERSIONED_ROOT_SIGNATURE_DESC(o: usize) -> usize {
    0
}

/// INT64 - from DirectX-Headers/d3dx12_root_signature.h
#[no_mangle]
pub unsafe extern "C" fn INT64(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// InitOffsetted - from DirectX-Headers/d3dx12_root_signature.h
#[no_mangle]
pub unsafe extern "C" fn InitOffsetted(base: usize, offsetScaledByIncrementSize: i32) {

}

/// D3DX12SerializeVersionedRootSignature - from DirectX-Headers/d3dx12_root_signature.h
#[no_mangle]
pub unsafe extern "C" fn D3DX12SerializeVersionedRootSignature(pRootSignatureDesc: *mut core::ffi::c_void, MaxVersion: usize, ppBlob: *mut *mut core::ffi::c_void, arg3: usize) -> i32 {
    0
}

/// desc_1_0 - from DirectX-Headers/d3dx12_root_signature.h
#[no_mangle]
pub unsafe extern "C" fn desc_1_0(arg0: usize, arg1: usize, arg2: usize, pStaticSamplers: usize, arg4: usize) -> usize {
    0
}

/// desc - from DirectX-Headers/d3dx12_root_signature.h
#[no_mangle]
pub unsafe extern "C" fn desc(arg0: usize, arg1: usize, arg2: usize, pStaticSamplers: usize, arg4: usize) -> usize {
    0
}

/// SetStateObjectType - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetStateObjectType(Type: usize) {

}

/// TrackSubobject - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn TrackSubobject(Type: usize, pDesc: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// LocalCopy - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn LocalCopy(string: usize, arg1: usize) -> usize {
    0
}

/// Type - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn Type() -> usize {
    0
}

/// Data - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn Data() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// SetDXILLibrary - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetDXILLibrary(pCode: *mut core::ffi::c_void) {

}

/// SetExistingCollection - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetExistingCollection(arg0: *mut core::ffi::c_void) {

}

/// SetSubobjectToAssociate - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetSubobjectToAssociate(SubobjectToAssociate: usize) {

}

/// SetSubobjectNameToAssociate - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetSubobjectNameToAssociate(SubobjectToAssociate: *const u16) {

}

/// SetHitGroupType - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetHitGroupType(Type: usize) {

}

/// SetClosestHitShaderImport - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetClosestHitShaderImport(importName: *const u16) {

}

/// SetIntersectionShaderImport - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetIntersectionShaderImport(importName: *const u16) {

}

/// Config - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn Config(MaxPayloadSizeInBytes: u32, MaxAttributeSizeInBytes: u32) {

}

/// SetRootSignature - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetRootSignature(pRootSig: *mut core::ffi::c_void) {

}

/// D3DX12_COM_PTR_GET - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn D3DX12_COM_PTR_GET(arg0: usize) -> usize {
    0
}

/// D3DX12_COM_PTR_ADDRESSOF - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn D3DX12_COM_PTR_ADDRESSOF(arg0: usize) -> usize {
    0
}

/// SetNodeMask - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetNodeMask(NodeMask: u32) {

}

/// SetSODeclEntries - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetSODeclEntries(soDeclEntries: *mut core::ffi::c_void, numEntries: u32) {

}

/// SetBufferStrides - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetBufferStrides(bufferStrides: *mut u32, numStrides: u32) {

}

/// SetRasterizedStream - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetRasterizedStream(rasterizedStream: u32) {

}

/// SetAlphaToCoverageEnable - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetAlphaToCoverageEnable(alphaToCoverageEnable: usize) {

}

/// SetIndependentBlendEnable - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetIndependentBlendEnable(independentBlendEnable: usize) {

}

/// SetFillMode - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetFillMode(fillMode: usize) {

}

/// SetCullMode - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetCullMode(cullMode: usize) {

}

/// SetDepthBias - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetDepthBias(depthBias: f32) {

}

/// SetDepthBiasClamp - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetDepthBiasClamp(depthBiasClamp: f32) {

}

/// SetSlopeScaledDepthBias - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetSlopeScaledDepthBias(slopeScaledDepthBias: f32) {

}

/// SetDepthClipEnable - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetDepthClipEnable(depthClipEnable: i32) {

}

/// SetLineRasterizationMode - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetLineRasterizationMode(lineRasterizationMode: usize) {

}

/// SetForcedSampleCount - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetForcedSampleCount(forcedSampleCount: u32) {

}

/// SetConservativeRaster - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetConservativeRaster(conservativeRaster: usize) {

}

/// SetDepthEnable - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetDepthEnable(depthEnable: i32) {

}

/// SetDepthFunc - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetDepthFunc(depthFunc: usize) {

}

/// SetStencilEnable - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetStencilEnable(stencilEnable: i32) {

}

/// SetDepthBoundsTestEnable - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetDepthBoundsTestEnable(depthBoundsTestEnable: i32) {

}

/// SetIBStripCutValue - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetIBStripCutValue(ibStripCutValue: usize) {

}

/// SetNumRenderTargets - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetNumRenderTargets(numRenderTargets: u32) {

}

/// SetRenderTargetFormat - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetRenderTargetFormat(renderTarget: u32, renderTargetFormat: usize) {

}

/// SetDepthStencilFormat - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetDepthStencilFormat(depthStencilFormat: usize) {

}

/// SetCount - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetCount(count: u32) {

}

/// SetQuality - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetQuality(quality: u32) {

}

/// SetSampleMask - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetSampleMask(sampleMask: u32) {

}

/// SetProgramName - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetProgramName(ProgramName: *const u16) {

}

/// AddSubobject - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn AddSubobject(subobject: usize) {

}

/// NewOutputOverride - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn NewOutputOverride() {

}

/// OutputIndex - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn OutputIndex(index: u32) {

}

/// NewName - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn NewName(Name: *const u16, arg1: usize) {

}

/// AllowSparseNodes - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn AllowSparseNodes(bAllow: i32) {

}

/// MaxOutputRecords - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn MaxOutputRecords(maxOutputRecords: u32) {

}

/// MaxOutputRecordsSharedWith - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn MaxOutputRecordsSharedWith(outputIndex: u32) {

}

/// Shader - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn Shader(_Shader: *const u16) {

}

/// GetShaderName - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn GetShaderName() -> *const u16 {
    core::ptr::null_mut()
}

/// GetNode - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn GetNode() -> usize {
    0
}

/// LocalRootArgumentsTableIndex - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn LocalRootArgumentsTableIndex(index: u32) {

}

/// ProgramEntry - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn ProgramEntry(bIsProgramEntry: i32) {

}

/// ShareInputOf - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn ShareInputOf(NodeID: usize) {

}

/// DispatchGrid - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn DispatchGrid(x: u32, y: u32, z: u32) {

}

/// MaxDispatchGrid - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn MaxDispatchGrid(x: u32, y: u32, z: u32) {

}

/// IncludeAllAvailableNodes - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn IncludeAllAvailableNodes() {

}

/// DML_DECLARE_INTERFACE - from DirectX-Headers/DirectML.h
#[no_mangle]
pub unsafe extern "C" fn DML_DECLARE_INTERFACE(arg0: usize) -> usize {
    0
}

/// IsAttributeSupported - from DirectX-Headers/dxcore_interface.h
#[no_mangle]
pub unsafe extern "C" fn IsAttributeSupported(attributeGUID: usize) -> usize {
    0
}

/// IsPropertySupported - from DirectX-Headers/dxcore_interface.h
#[no_mangle]
pub unsafe extern "C" fn IsPropertySupported(property: usize) -> usize {
    0
}

/// GetPropertySize - from DirectX-Headers/dxcore_interface.h
#[no_mangle]
pub unsafe extern "C" fn GetPropertySize(property: usize, bufferSize: *mut core::ffi::c_void) -> usize {
    0
}

/// IsQueryStateSupported - from DirectX-Headers/dxcore_interface.h
#[no_mangle]
pub unsafe extern "C" fn IsQueryStateSupported(property: usize) -> usize {
    0
}

/// IsSetStateSupported - from DirectX-Headers/dxcore_interface.h
#[no_mangle]
pub unsafe extern "C" fn IsSetStateSupported(property: usize) -> usize {
    0
}

/// GetPropertyWithInput - from DirectX-Headers/dxcore_interface.h
#[no_mangle]
pub unsafe extern "C" fn GetPropertyWithInput(arg0: usize, arg1: usize) -> usize {
    0
}

/// IsStale - from DirectX-Headers/dxcore_interface.h
#[no_mangle]
pub unsafe extern "C" fn IsStale() -> usize {
    0
}

/// IsAdapterPreferenceSupported - from DirectX-Headers/dxcore_interface.h
#[no_mangle]
pub unsafe extern "C" fn IsAdapterPreferenceSupported(preference: usize) -> usize {
    0
}

/// IsNotificationTypeSupported - from DirectX-Headers/dxcore_interface.h
#[no_mangle]
pub unsafe extern "C" fn IsNotificationTypeSupported(notificationType: usize) -> usize {
    0
}

/// RegisterEventNotification - from DirectX-Headers/dxcore_interface.h
#[no_mangle]
pub unsafe extern "C" fn RegisterEventNotification(dxCoreObject: *mut core::ffi::c_void, notificationType: usize, callbackFunction: usize, callbackContext: *mut core::ffi::c_void, eventCookie: *mut core::ffi::c_void) -> usize {
    0
}

/// UnregisterEventNotification - from DirectX-Headers/dxcore_interface.h
#[no_mangle]
pub unsafe extern "C" fn UnregisterEventNotification(eventCookie: u32) -> usize {
    0
}

/// uuidof - from DirectX-Headers/dxguids.h
#[no_mangle]
pub unsafe extern "C" fn uuidof() -> usize {
    0
}

/// GetAddressOf - from DirectX-Headers/wrladapter.h
#[no_mangle]
pub unsafe extern "C" fn GetAddressOf() -> *mut *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// ReleaseAndGetAddressOf - from DirectX-Headers/wrladapter.h
#[no_mangle]
pub unsafe extern "C" fn ReleaseAndGetAddressOf() -> *mut *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// InternalAddRef - from DirectX-Headers/wrladapter.h
#[no_mangle]
pub unsafe extern "C" fn InternalAddRef() {

}

/// InternalRelease - from DirectX-Headers/wrladapter.h
#[no_mangle]
pub unsafe extern "C" fn InternalRelease() -> u64 {
    0
}

/// CopyTo - from DirectX-Headers/wrladapter.h
#[no_mangle]
pub unsafe extern "C" fn CopyTo(ptr: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// As - from DirectX-Headers/wrladapter.h
#[no_mangle]
pub unsafe extern "C" fn As(p: usize) -> i32 {
    0
}

/// AsIID - from DirectX-Headers/wrladapter.h
#[no_mangle]
pub unsafe extern "C" fn AsIID(implements: *mut core::ffi::c_void, riid: usize, ppvObject: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// CastToBase - from DirectX-Headers/wrladapter.h
#[no_mangle]
pub unsafe extern "C" fn CastToBase(ptr: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CastToUnknown - from DirectX-Headers/wrladapter.h
#[no_mangle]
pub unsafe extern "C" fn CastToUnknown(ptr: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CanCastTo - from DirectX-Headers/wrladapter.h
#[no_mangle]
pub unsafe extern "C" fn CanCastTo(ptr: *mut core::ffi::c_void, riid: usize, ppv: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetRefCount - from DirectX-Headers/wrladapter.h
#[no_mangle]
pub unsafe extern "C" fn GetRefCount() -> u64 {
    0
}

/// CustomQueryInterface - from DirectX-Headers/wrladapter.h
#[no_mangle]
pub unsafe extern "C" fn CustomQueryInterface(arg0: usize, arg1: usize, handled: *mut core::ffi::c_void) -> i32 {
    0
}

/// IID_PPV_ARGS_Helper - from DirectX-Headers/wrladapter.h
#[no_mangle]
pub unsafe extern "C" fn IID_PPV_ARGS_Helper(pp: usize) -> *mut *mut core::ffi::c_void {
    core::ptr::null_mut()
}

