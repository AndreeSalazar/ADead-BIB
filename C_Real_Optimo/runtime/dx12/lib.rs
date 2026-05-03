//! ADead Runtime - DX12 Module
//!
//! Funciones generadas automáticamente desde knowledge.json
//! Total: 1720 funciones

use std::ffi::c_void;

#[no_mangle]
pub unsafe extern "C" fn MFMapDX9FormatToDXGIFormat(format: u32) -> DXGI_FORMAT {
    // TODO: implementar MFMapDX9FormatToDXGIFormat desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MFMapDXGIFormatToDX9Format(dxgi_format: DXGI_FORMAT) -> u32 {
    // TODO: implementar MFMapDXGIFormatToDX9Format desde wine/mfapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetDevice(riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetDevice desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPrivateData(guid: _In_ REFGUID, pDataSize: *mut _Inout_ UINT, pDataSize: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetPrivateData desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetPrivateData(guid: _In_ REFGUID, DataSize: _In_ UINT, DataSize: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetPrivateData desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetPrivateDataInterface(guid: _In_ REFGUID, pData: *mut _In_opt_ const IUnknown) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetPrivateDataInterface desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetDesc1() -> virtual D3D12_PROTECTED_RESOURCE_SESSION_DESC1 STDMETHODCALLTYPE {
    // TODO: implementar GetDesc1 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Map(Subresource: UINT, pReadRange: *mut _In_opt_ const D3D12_RANGE) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar Map desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Unmap(Subresource: UINT, pWrittenRange: *mut _In_opt_ const D3D12_RANGE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar Unmap desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetDeviceRemovedReason() -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetDeviceRemovedReason desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ClearState(pPipelineState: *mut _In_opt_ ID3D12PipelineState) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ClearState desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateShaderResourceView(pResource: *mut _In_opt_ ID3D12Resource, pDesc: *mut _In_opt_ const D3D12_SHADER_RESOURCE_VIEW_DESC, DestDescriptor: _In_ D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar CreateShaderResourceView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateRenderTargetView(pResource: *mut _In_opt_ ID3D12Resource, pDesc: *mut _In_opt_ const D3D12_RENDER_TARGET_VIEW_DESC, DestDescriptor: _In_ D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar CreateRenderTargetView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDepthStencilView(pResource: *mut _In_opt_ ID3D12Resource, pDesc: *mut _In_opt_ const D3D12_DEPTH_STENCIL_VIEW_DESC, DestDescriptor: _In_ D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar CreateDepthStencilView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetCreationFlags() -> virtual D3D12_FENCE_FLAGS STDMETHODCALLTYPE {
    // TODO: implementar GetCreationFlags desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ClearRenderTargetView(RenderTargetView: _In_ D3D12_CPU_DESCRIPTOR_HANDLE, NumRects: _In_ UINT, param_41960: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ClearRenderTargetView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ClearDepthStencilView(DepthStencilView: _In_ D3D12_CPU_DESCRIPTOR_HANDLE, ClearFlags: _In_ D3D12_CLEAR_FLAGS, Depth: _In_ FLOAT, Stencil: _In_ UINT8, NumRects: _In_ UINT, param_41960: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ClearDepthStencilView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetPredication(pBuffer: *mut _In_opt_ ID3D12Resource, AlignedBufferOffset: _In_ UINT64, Operation: _In_ D3D12_PREDICATION_OP) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetPredication desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CopyResource(pDstResource: *mut _In_ ID3D12Resource, pSrcResource: *mut _In_ ID3D12Resource) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar CopyResource desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ResolveSubresource(pDstResource: *mut _In_ ID3D12Resource, DstSubresource: _In_ UINT, pSrcResource: *mut _In_ ID3D12Resource, SrcSubresource: _In_ UINT, Format: _In_ DXGI_FORMAT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ResolveSubresource desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DrawInstanced(VertexCountPerInstance: _In_ UINT, InstanceCount: _In_ UINT, StartVertexLocation: _In_ UINT, StartInstanceLocation: _In_ UINT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar DrawInstanced desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DrawIndexedInstanced(IndexCountPerInstance: _In_ UINT, InstanceCount: _In_ UINT, StartIndexLocation: _In_ UINT, BaseVertexLocation: _In_ INT, StartInstanceLocation: _In_ UINT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar DrawIndexedInstanced desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IASetPrimitiveTopology(PrimitiveTopology: _In_ D3D12_PRIMITIVE_TOPOLOGY) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar IASetPrimitiveTopology desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IASetVertexBuffers(StartSlot: _In_ UINT, NumViews: _In_ UINT, param_32314: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar IASetVertexBuffers desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IASetIndexBuffer(pView: *mut _In_opt_ const D3D12_INDEX_BUFFER_VIEW) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar IASetIndexBuffer desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OMSetRenderTargets(NumRenderTargetDescriptors: _In_ UINT, pRenderTargetDescriptors: *mut _In_opt_ const D3D12_CPU_DESCRIPTOR_HANDLE, RTsSingleHandleToDescriptorRange: _In_ BOOL, pDepthStencilDescriptor: *mut _In_opt_ const D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar OMSetRenderTargets desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RSSetViewports(param_53423: *mut core::ffi::c_void, param_63334: D3D12_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar RSSetViewports desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RSSetScissorRects(param_53423: *mut core::ffi::c_void, param_63334: D3D12_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar RSSetScissorRects desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SOSetTargets(StartSlot: _In_ UINT, NumViews: _In_ UINT, param_32314: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SOSetTargets desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BeginEvent(Metadata: UINT, param_5470: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar BeginEvent desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EndEvent() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar EndEvent desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetMarker(Metadata: UINT, param_5470: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetMarker desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetDescFromD3D12(pResource: *mut ID3D12Resource, pResourceFlags: *mut const D3D11_RESOURCE_FLAGS, pBufferDesc: *mut D3D11_BUFFER_DESC) -> static HRESULT {
    // TODO: implementar GetDescFromD3D12 desde dxvk/d3d11_buffer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DiscardResource(pResource: *mut _In_ ID3D12Resource, pRegion: *mut _In_opt_ const D3D12_DISCARD_REGION) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar DiscardResource desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ClearUnorderedAccessViewUint(ViewGPUHandleInCurrentHeap: _In_ D3D12_GPU_DESCRIPTOR_HANDLE, ViewCPUHandle: _In_ D3D12_CPU_DESCRIPTOR_HANDLE, pResource: *mut _In_ ID3D12Resource, NumRects: _In_ UINT, param_41960: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ClearUnorderedAccessViewUint desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ClearUnorderedAccessViewFloat(ViewGPUHandleInCurrentHeap: _In_ D3D12_GPU_DESCRIPTOR_HANDLE, ViewCPUHandle: _In_ D3D12_CPU_DESCRIPTOR_HANDLE, pResource: *mut _In_ ID3D12Resource, NumRects: _In_ UINT, param_41960: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ClearUnorderedAccessViewFloat desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Dispatch(ThreadGroupCountX: _In_ UINT, ThreadGroupCountY: _In_ UINT, ThreadGroupCountZ: _In_ UINT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar Dispatch desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CopyTiles(pTiledResource: *mut _In_ ID3D12Resource, pTileRegionStartCoordinate: *mut _In_ const D3D12_TILED_RESOURCE_COORDINATE, pTileRegionSize: *mut _In_ const D3D12_TILE_REGION_SIZE, pBuffer: *mut _In_ ID3D12Resource, BufferStartOffsetInBytes: UINT64, Flags: D3D12_TILE_COPY_FLAGS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar CopyTiles desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CopyTileMappings(pDstResource: *mut _In_ ID3D12Resource, pDstRegionStartCoordinate: *mut _In_ const D3D12_TILED_RESOURCE_COORDINATE, pSrcResource: *mut _In_ ID3D12Resource, pSrcRegionStartCoordinate: *mut _In_ const D3D12_TILED_RESOURCE_COORDINATE, pRegionSize: *mut _In_ const D3D12_TILE_REGION_SIZE, Flags: D3D12_TILE_MAPPING_FLAGS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar CopyTileMappings desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Signal(Value: UINT64) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar Signal desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateUnorderedAccessView(pResource: *mut _In_opt_ ID3D12Resource, pCounterResource: *mut _In_opt_ ID3D12Resource, pDesc: *mut _In_opt_ const D3D12_UNORDERED_ACCESS_VIEW_DESC, DestDescriptor: _In_ D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar CreateUnorderedAccessView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateFence(InitialValue: UINT64, Flags: D3D12_FENCE_FLAGS, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateFence desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ReadFromSubresource(pDstData: *mut _Out_ void, DstRowPitch: UINT, DstDepthPitch: UINT, SrcSubresource: UINT, pSrcBox: *mut _In_opt_ const D3D12_BOX) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar ReadFromSubresource desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WriteToSubresource(DstSubresource: UINT, pDstBox: *mut _In_opt_ const D3D12_BOX, pSrcData: *mut _In_ const void, SrcRowPitch: UINT, SrcDepthPitch: UINT) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar WriteToSubresource desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CheckFeatureSupport(Feature: D3D12_FEATURE, param_43148: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CheckFeatureSupport desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetResourceTiling(pTiledResource: *mut _In_ ID3D12Resource, pNumTilesForEntireResource: *mut _Out_opt_ UINT, pPackedMipDesc: *mut _Out_opt_ D3D12_PACKED_MIP_INFO, pStandardTileShapeForNonPackedMips: *mut _Out_opt_ D3D12_TILE_SHAPE, pNumSubresourceTilings: *mut _Inout_opt_ UINT, FirstSubresourceTilingToGet: _In_ UINT, param_47141: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar GetResourceTiling desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSharedHandle(pObject: *mut _In_ ID3D12DeviceChild, pAttributes: *mut _In_opt_ const SECURITY_ATTRIBUTES, Access: u32, Name: _In_opt_ LPCWSTR, pHandle: *mut _Out_ HANDLE) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateSharedHandle desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetEventOnCompletion(Value: UINT64, hEvent: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetEventOnCompletion desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetCompletedValue() -> virtual UINT64 STDMETHODCALLTYPE {
    // TODO: implementar GetCompletedValue desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetD3D12Device(riid: REFIID, ppvDevice: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetD3D12Device desde dxvk/d3d11_on_12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetDXGIAdapter(iid: REFIID, ppvObject: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetDXGIAdapter desde dxvk/d3d11_on_12_interfaces.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetName(Format: DXGI_FORMAT, true: bool bHideInternalFormats =) -> static LPCSTR {
    // TODO: implementar GetName desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Present(pResource: *mut _In_ ID3D12Resource, Subresource: UINT, window: _In_ HWND) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar Present desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetDxgiUsage() -> DXGI_USAGE {
    // TODO: implementar GetDxgiUsage desde dxvk/d3d11_texture.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPlaneCount(Format: DXGI_FORMAT) -> static UINT8 {
    // TODO: implementar GetPlaneCount desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetRenderTarget(renderTargetIndex: UINT, renderTargetBlendDesc: const D3D12_RENDER_TARGET_BLEND_DESC&) -> core::ffi::c_void {
    // TODO: implementar SetRenderTarget desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetAdapterCount() -> virtual uint32_t STDMETHODCALLTYPE {
    // TODO: implementar GetAdapterCount desde DirectX-Headers/dxcore_interface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDevice(adapter: *mut _In_opt_ IUnknown, FeatureLevel: D3D_FEATURE_LEVEL, riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateDevice desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CalcSubresource(MipSlice: UINT, ArraySlice: UINT, PlaneSlice: UINT) -> inline UINT {
    // TODO: implementar CalcSubresource desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsValid() -> virtual bool STDMETHODCALLTYPE {
    // TODO: implementar IsValid desde DirectX-Headers/dxcore_interface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn T(param_45740: *this) -> new {
    // TODO: implementar T desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetFlags() -> virtual D3D12_DEVICE_FACTORY_FLAGS STDMETHODCALLTYPE {
    // TODO: implementar GetFlags desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RegisterDestructionCallback(callbackFn: _In_ PFN_DESTRUCTION_, pData: *mut _In_ void, pCallbackID: *mut _Out_ UINT) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar RegisterDestructionCallback desde DirectX-Headers/d3dcommon.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnregisterDestructionCallback(callbackID: _In_ UINT) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar UnregisterDestructionCallback desde DirectX-Headers/d3dcommon.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IDXGISwapChain3_GetCurrentBackBufferIndex(param_12004: swapchain->swapchain) -> return {
    // TODO: implementar IDXGISwapChain3_GetCurrentBackBufferIndex desde vkd3d-proton/demo_win32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Fence_SetEventOnCompletion(param_28937: fence, param_56752: value, param_50043: NULL) -> return {
    // TODO: implementar ID3D12Fence_SetEventOnCompletion desde vkd3d-proton/d3d12_crosstest.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn enable_d3d12_debug_layer(argc: i32, param_4138: *mut i8) -> static inline void {
    // TODO: implementar enable_d3d12_debug_layer desde vkd3d-proton/d3d12_crosstest.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_open_resource_descriptor(device: *mut struct d3d12_device, handle: *mut core::ffi::c_void, desc: *mut D3D12_RESOURCE_DESC1) -> extern HRESULT {
    // TODO: implementar d3d12_device_open_resource_descriptor desde vkd3d-proton/vkd3d_d3dkmt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_open_kmt(device: *mut struct d3d12_device) -> extern void {
    // TODO: implementar d3d12_device_open_kmt desde vkd3d-proton/vkd3d_d3dkmt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_close_kmt(device: *mut struct d3d12_device) -> extern void {
    // TODO: implementar d3d12_device_close_kmt desde vkd3d-proton/vkd3d_d3dkmt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_Dispatch_profiled(iface: *mut d3d12_command_list_iface, x: UINT, y: UINT, z: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_Dispatch_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyTextureRegion_profiled(iface: *mut d3d12_command_list_iface, dst: *mut const D3D12_TEXTURE_COPY_LOCATION, dst_x: UINT, dst_y: UINT, dst_z: UINT, src: *mut const D3D12_TEXTURE_COPY_LOCATION, src_box: *mut const D3D12_BOX) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_CopyTextureRegion_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyResource_profiled(iface: *mut d3d12_command_list_iface, dst: *mut ID3D12Resource, src: *mut ID3D12Resource) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_CopyResource_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetViewports_profiled(iface: *mut d3d12_command_list_iface, viewport_count: UINT, viewports: *mut const D3D12_VIEWPORT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_RSSetViewports_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetScissorRects_profiled(iface: *mut d3d12_command_list_iface, rect_count: UINT, rects: *mut const D3D12_RECT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_RSSetScissorRects_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetBlendFactor_profiled(iface: *mut d3d12_command_list_iface) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_OMSetBlendFactor_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetStencilRef_profiled(iface: *mut d3d12_command_list_iface, stencil_ref: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_OMSetStencilRef_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPipelineState_profiled(iface: *mut d3d12_command_list_iface, pipeline_state: *mut ID3D12PipelineState) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetPipelineState_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ResourceBarrier_profiled(iface: *mut d3d12_command_list_iface, barrier_count: UINT, barriers: *mut const D3D12_RESOURCE_BARRIER) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_ResourceBarrier_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ExecuteBundle_profiled(iface: *mut d3d12_command_list_iface, command_list: *mut ID3D12GraphicsCommandList) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_ExecuteBundle_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetDescriptorHeaps_profiled(iface: *mut d3d12_command_list_iface, heap_count: UINT, heaps: *mut core::ffi::c_void) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetDescriptorHeaps_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootSignature_profiled(iface: *mut d3d12_command_list_iface, root_signature: *mut ID3D12RootSignature) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetComputeRootSignature_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootSignature_profiled(iface: *mut d3d12_command_list_iface, root_signature: *mut ID3D12RootSignature) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetGraphicsRootSignature_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetIndexBuffer_profiled(iface: *mut d3d12_command_list_iface, view: *mut const D3D12_INDEX_BUFFER_VIEW) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_IASetIndexBuffer_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetVertexBuffers_profiled(iface: *mut d3d12_command_list_iface, start_slot: UINT, view_count: UINT, views: *mut const D3D12_VERTEX_BUFFER_VIEW) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_IASetVertexBuffers_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SOSetTargets_profiled(iface: *mut d3d12_command_list_iface, start_slot: UINT, view_count: UINT, views: *mut const D3D12_STREAM_OUTPUT_BUFFER_VIEW) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SOSetTargets_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ClearRenderTargetView_profiled(iface: *mut d3d12_command_list_iface, rtv: D3D12_CPU_DESCRIPTOR_HANDLE, rect_count: UINT, rects: *mut const D3D12_RECT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_ClearRenderTargetView_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DiscardResource_profiled(iface: *mut d3d12_command_list_iface, resource: *mut ID3D12Resource, region: *mut const D3D12_DISCARD_REGION) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_DiscardResource_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_BeginQuery_profiled(iface: *mut d3d12_command_list_iface, heap: *mut ID3D12QueryHeap, type: D3D12_QUERY_TYPE, index: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_BeginQuery_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_EndQuery_profiled(iface: *mut d3d12_command_list_iface, heap: *mut ID3D12QueryHeap, type: D3D12_QUERY_TYPE, index: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_EndQuery_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPredication_profiled(iface: *mut d3d12_command_list_iface, buffer: *mut ID3D12Resource, aligned_buffer_offset: UINT64, operation: D3D12_PREDICATION_OP) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetPredication_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetMarker_profiled(iface: *mut d3d12_command_list_iface, metadata: UINT, data: *mut const void, size: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetMarker_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_BeginEvent_profiled(iface: *mut d3d12_command_list_iface, metadata: UINT, data: *mut const void, size: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_BeginEvent_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_EndEvent_profiled(iface: *mut d3d12_command_list_iface) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_EndEvent_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetDepthBounds_profiled(iface: *mut d3d12_command_list_iface, min: FLOAT, max: FLOAT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_OMSetDepthBounds_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetProtectedResourceSession_profiled(iface: *mut d3d12_command_list_iface, protected_session: *mut ID3D12ProtectedResourceSession) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetProtectedResourceSession_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_EndRenderPass_profiled(iface: *mut d3d12_command_list_iface) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_EndRenderPass_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyRaytracingAccelerationStructure_profiled(iface: *mut d3d12_command_list_iface, dst_data: D3D12_GPU_VIRTUAL_ADDRESS, src_data: D3D12_GPU_VIRTUAL_ADDRESS, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_CopyRaytracingAccelerationStructure_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPipelineState1_profiled(iface: *mut d3d12_command_list_iface, state_object: *mut ID3D12StateObject) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetPipelineState1_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DispatchRays_profiled(iface: *mut d3d12_command_list_iface, desc: *mut const D3D12_DISPATCH_RAYS_DESC) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_DispatchRays_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetShadingRate_profiled(iface: *mut d3d12_command_list_iface, base: D3D12_SHADING_RATE, combiners: *mut const D3D12_SHADING_RATE_COMBINER) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_RSSetShadingRate_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetShadingRateImage_profiled(iface: *mut d3d12_command_list_iface, image: *mut ID3D12Resource) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_RSSetShadingRateImage_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DispatchMesh_profiled(iface: *mut d3d12_command_list_iface, x: UINT, y: UINT, z: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_DispatchMesh_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_Barrier_profiled(iface: *mut d3d12_command_list_iface, NumBarrierGroups: UINT32, pBarrierGroups: *mut const D3D12_BARRIER_GROUP) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_Barrier_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetIndexBufferStripCutValue_profiled(iface: *mut d3d12_command_list_iface, IBStripCutValue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_IASetIndexBufferStripCutValue_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetProgram_profiled(iface: *mut d3d12_command_list_iface, desc: *mut const D3D12_SET_PROGRAM_DESC) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetProgram_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DispatchGraph_profiled(iface: *mut d3d12_command_list_iface, desc: *mut const D3D12_DISPATCH_GRAPH_DESC) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_DispatchGraph_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptors_profiled(iface: *mut d3d12_device_iface, dst_descriptor_range_count: UINT, dst_descriptor_range_offsets: *mut const D3D12_CPU_DESCRIPTOR_HANDLE, dst_descriptor_range_sizes: *mut const UINT, src_descriptor_range_count: UINT, src_descriptor_range_offsets: *mut const D3D12_CPU_DESCRIPTOR_HANDLE, src_descriptor_range_sizes: *mut const UINT, descriptor_heap_type: D3D12_DESCRIPTOR_HEAP_TYPE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CopyDescriptors_profiled desde vkd3d-proton/device_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptorsSimple_profiled(iface: *mut d3d12_device_iface, descriptor_count: UINT, dst_descriptor_range_offset: const D3D12_CPU_DESCRIPTOR_HANDLE, src_descriptor_range_offset: const D3D12_CPU_DESCRIPTOR_HANDLE, descriptor_heap_type: D3D12_DESCRIPTOR_HEAP_TYPE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CopyDescriptorsSimple_profiled desde vkd3d-proton/device_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_object_SetName(iface: *mut ID3D12Object, name: *mut const WCHAR) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_object_SetName desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn impl_from_ID3D12Fence1(param_25901: *mut core::ffi::c_void) -> return {
    // TODO: implementar impl_from_ID3D12Fence1 desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_set_event_on_completion(fence: *mut struct d3d12_fence, value: UINT64, event: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar d3d12_fence_set_event_on_completion desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_set_native_sync_handle_on_completion(fence: *mut struct d3d12_fence, value: UINT64, handle: vkd3d_native_sync_handle) -> i32 {
    // TODO: implementar d3d12_fence_set_native_sync_handle_on_completion desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn shared_impl_from_ID3D12Fence1(param_25901: *mut core::ffi::c_void) -> return {
    // TODO: implementar shared_impl_from_ID3D12Fence1 desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn is_shared_ID3D12Fence1(iface: *mut ID3D12Fence1) -> static inline bool {
    // TODO: implementar is_shared_ID3D12Fence1 desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn is_shared_ID3D12Fence(iface: *mut ID3D12Fence) -> static inline bool {
    // TODO: implementar is_shared_ID3D12Fence desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_validate_custom_heap_type(device: *mut struct d3d12_device, heap_properties: *mut const D3D12_HEAP_PROPERTIES) -> i32 {
    // TODO: implementar d3d12_device_validate_custom_heap_type desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_incref(heap: *mut struct d3d12_heap) -> ULONG {
    // TODO: implementar d3d12_heap_incref desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_decref(heap: *mut struct d3d12_heap) -> ULONG {
    // TODO: implementar d3d12_heap_decref desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn impl_from_ID3D12Heap1(param_38427: *mut core::ffi::c_void) -> return {
    // TODO: implementar impl_from_ID3D12Heap1 desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_is_buffer(resource: *mut const struct d3d12_resource) -> static inline bool {
    // TODO: implementar d3d12_resource_is_buffer desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_is_acceleration_structure(resource: *mut const struct d3d12_resource) -> static inline bool {
    // TODO: implementar d3d12_resource_is_acceleration_structure desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_is_texture(resource: *mut const struct d3d12_resource) -> static inline bool {
    // TODO: implementar d3d12_resource_is_texture desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_pick_layout(resource: *mut const struct d3d12_resource, layout: VkImageLayout) -> static inline VkImageLayout {
    // TODO: implementar d3d12_resource_pick_layout desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_incref(resource: *mut struct d3d12_resource) -> ULONG {
    // TODO: implementar d3d12_resource_incref desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_decref(resource: *mut struct d3d12_resource) -> ULONG {
    // TODO: implementar d3d12_resource_decref desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_decref_retained(resource: *mut struct d3d12_resource) -> core::ffi::c_void {
    // TODO: implementar d3d12_resource_decref_retained desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_incref_weak(resource: *mut struct d3d12_resource) -> core::ffi::c_void {
    // TODO: implementar d3d12_resource_incref_weak desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_decref_weak(resource: *mut struct d3d12_resource) -> core::ffi::c_void {
    // TODO: implementar d3d12_resource_decref_weak desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_is_cpu_accessible(resource: *mut const struct d3d12_resource) -> bool {
    // TODO: implementar d3d12_resource_is_cpu_accessible desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_promote_desc(desc: *mut const D3D12_RESOURCE_DESC, desc1: *mut D3D12_RESOURCE_DESC1) -> core::ffi::c_void {
    // TODO: implementar d3d12_resource_promote_desc desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_validate_desc(desc: *mut const D3D12_RESOURCE_DESC1, num_castable_formats: UINT, castable_formats: *mut const DXGI_FORMAT, device: *mut struct d3d12_device) -> i32 {
    // TODO: implementar d3d12_resource_validate_desc desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn impl_from_ID3D12Resource2(param_4609: *mut core::ffi::c_void) -> return {
    // TODO: implementar impl_from_ID3D12Resource2 desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_copy(dst: vkd3d_cpu_descriptor_va_t, src: vkd3d_cpu_descriptor_va_t, count: u32, heap_type: D3D12_DESCRIPTOR_HEAP_TYPE, device: *mut struct d3d12_device) -> core::ffi::c_void {
    // TODO: implementar d3d12_desc_copy desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_rtv_desc_copy(dst: *mut struct d3d12_rtv_desc, src: *mut struct d3d12_rtv_desc, count: u32) -> core::ffi::c_void {
    // TODO: implementar d3d12_rtv_desc_copy desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_cleanup(descriptor_heap: *mut struct d3d12_descriptor_heap) -> core::ffi::c_void {
    // TODO: implementar d3d12_descriptor_heap_cleanup desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_require_padding_descriptors(device: *mut struct d3d12_device) -> bool {
    // TODO: implementar d3d12_descriptor_heap_require_padding_descriptors desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_decode_embedded_resource_va(va: vkd3d_cpu_descriptor_va_t) -> static inline struct d3d12_desc_split_embedded {
    // TODO: implementar d3d12_desc_decode_embedded_resource_va desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_copy_embedded_resource(dst_va: vkd3d_cpu_descriptor_va_t, src_va: vkd3d_cpu_descriptor_va_t, size: usize) -> static inline void {
    // TODO: implementar d3d12_desc_copy_embedded_resource desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_decode_va(va: vkd3d_cpu_descriptor_va_t) -> static inline struct d3d12_desc_split {
    // TODO: implementar d3d12_desc_decode_va desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_heap_offset_from_embedded_gpu_handle(handle: D3D12_GPU_DESCRIPTOR_HANDLE, cbv_srv_uav_size_log2: u32, sampler_size_log2: u32) -> static inline uint32_t {
    // TODO: implementar d3d12_desc_heap_offset_from_embedded_gpu_handle desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_heap_offset_from_gpu_handle(handle: D3D12_GPU_DESCRIPTOR_HANDLE) -> static inline uint32_t {
    // TODO: implementar d3d12_desc_heap_offset_from_gpu_handle desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_inc_ref(state: *mut struct d3d12_root_signature) -> core::ffi::c_void {
    // TODO: implementar d3d12_root_signature_inc_ref desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_dec_ref(state: *mut struct d3d12_root_signature) -> core::ffi::c_void {
    // TODO: implementar d3d12_root_signature_dec_ref desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_get_shader_interface_flags(root_signature: *mut const struct d3d12_root_signature, pipeline_type: enum vkd3d_pipeline_type) -> u32 {
    // TODO: implementar d3d12_root_signature_get_shader_interface_flags desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_is_pipeline_compatible(a: *mut const struct d3d12_root_signature, b: *mut const struct d3d12_root_signature) -> static inline bool {
    // TODO: implementar d3d12_root_signature_is_pipeline_compatible desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_is_layout_compatible(a: *mut const struct d3d12_root_signature, b: *mut const struct d3d12_root_signature) -> static inline bool {
    // TODO: implementar d3d12_root_signature_is_layout_compatible desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_is_compute(state: *mut const struct d3d12_pipeline_state) -> static inline bool {
    // TODO: implementar d3d12_pipeline_state_is_compute desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_is_graphics(state: *mut const struct d3d12_pipeline_state) -> static inline bool {
    // TODO: implementar d3d12_pipeline_state_is_graphics desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_graphics_pipeline_state_has_unknown_dsv_format_with_test(graphics: *mut const struct d3d12_graphics_pipeline_state) -> static inline bool {
    // TODO: implementar d3d12_graphics_pipeline_state_has_unknown_dsv_format_with_test desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_inc_public_ref(state: *mut struct d3d12_pipeline_state) -> ULONG {
    // TODO: implementar d3d12_pipeline_state_inc_public_ref desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_inc_ref(state: *mut struct d3d12_pipeline_state) -> core::ffi::c_void {
    // TODO: implementar d3d12_pipeline_state_inc_ref desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_dec_ref(state: *mut struct d3d12_pipeline_state) -> core::ffi::c_void {
    // TODO: implementar d3d12_pipeline_state_dec_ref desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_has_replaced_shaders(state: *mut struct d3d12_pipeline_state) -> bool {
    // TODO: implementar d3d12_pipeline_state_has_replaced_shaders desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_get_pipeline(state: *mut struct d3d12_pipeline_state, dyn_state: *mut const struct vkd3d_dynamic_state, dsv_format: *mut const struct vkd3d_format, dynamic_state_flags: *mut u32) -> VkPipeline {
    // TODO: implementar d3d12_pipeline_state_get_pipeline desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_cached_pipeline_state_validate(device: *mut struct d3d12_device, state: *mut const struct d3d12_cached_pipeline_state, compat: *mut const struct vkd3d_pipeline_cache_compatibility) -> i32 {
    // TODO: implementar d3d12_cached_pipeline_state_validate desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_cached_pipeline_state_is_dummy(state: *mut const struct d3d12_cached_pipeline_state) -> bool {
    // TODO: implementar d3d12_cached_pipeline_state_is_dummy desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_inc_public_ref(state: *mut struct d3d12_pipeline_library) -> ULONG {
    // TODO: implementar d3d12_pipeline_library_inc_public_ref desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_dec_public_ref(state: *mut struct d3d12_pipeline_library) -> ULONG {
    // TODO: implementar d3d12_pipeline_library_dec_public_ref desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_inc_ref(state: *mut struct d3d12_pipeline_library) -> core::ffi::c_void {
    // TODO: implementar d3d12_pipeline_library_inc_ref desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_dec_ref(state: *mut struct d3d12_pipeline_library) -> core::ffi::c_void {
    // TODO: implementar d3d12_pipeline_library_dec_ref desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_decay_tracked_state(list: *mut struct d3d12_command_list) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_list_decay_tracked_state desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_query(list: *mut struct d3d12_command_list, vk_pool: VkQueryPool, index: u32) -> bool {
    // TODO: implementar d3d12_command_list_reset_query desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_end_current_render_pass(list: *mut struct d3d12_command_list, suspend: bool) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_list_end_current_render_pass desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_invalidate_all_state(list: *mut struct d3d12_command_list) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_list_invalidate_all_state desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_debug_mark_label(list: *mut struct d3d12_command_list, tag: *mut const char, r: f32, g: f32, b: f32, a: f32) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_list_debug_mark_label desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_debug_mark_begin_region(list: *mut struct d3d12_command_list, tag: *mut const char) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_list_debug_mark_begin_region desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_debug_mark_end_region(list: *mut struct d3d12_command_list) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_list_debug_mark_end_region desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_invalidate_current_pipeline(list: *mut struct d3d12_command_list, meta_shader: bool) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_list_invalidate_current_pipeline desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_invalidate_root_parameters(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings, invalidate_descriptor_heaps: bool, sibling_push_domain: *mut struct vkd3d_pipeline_bindings) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_list_invalidate_root_parameters desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_descriptor_buffers(list: *mut struct d3d12_command_list) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_list_update_descriptor_buffers desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_flush_dgc_batch(list: *mut struct d3d12_command_list) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_list_flush_dgc_batch desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_fetch_root_parameter_data(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings, dst_data: *mut union vkd3d_root_parameter_data) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_list_fetch_root_parameter_data desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_execute(bundle: *mut struct d3d12_bundle, list: *mut d3d12_command_list_iface) -> core::ffi::c_void {
    // TODO: implementar d3d12_bundle_execute desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_submit_stop(queue: *mut struct d3d12_command_queue) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_queue_submit_stop desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_signal_inline(queue: *mut struct d3d12_command_queue, fence: *mut d3d12_fence_iface, value: u64) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_queue_signal_inline desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_enqueue_callback(queue: *mut struct d3d12_command_queue, param_42367: core::ffi::c_void) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_queue_enqueue_callback desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_nv_shader_extn(device: *mut struct d3d12_device) -> struct vkd3d_nv_shader_extn {
    // TODO: implementar d3d12_device_get_nv_shader_extn desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_add_queue_timeline_deferred_decref(device: *mut struct d3d12_device, param_42367: core::ffi::c_void) -> core::ffi::c_void {
    // TODO: implementar d3d12_device_add_queue_timeline_deferred_decref desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_is_uma(device: *mut struct d3d12_device, coherent: *mut bool) -> bool {
    // TODO: implementar d3d12_device_is_uma desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_mark_as_removed(device: *mut struct d3d12_device, reason: i32, message: *mut const char) -> core::ffi::c_void {
    // TODO: implementar d3d12_device_mark_as_removed desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_report_fault(device: *mut struct d3d12_device) -> core::ffi::c_void {
    // TODO: implementar d3d12_device_report_fault desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_removed_reason(device: *mut struct d3d12_device) -> i32 {
    // TODO: implementar d3d12_device_removed_reason desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_max_descriptor_heap_size(device: *mut struct d3d12_device, heap_type: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
    // TODO: implementar d3d12_device_get_max_descriptor_heap_size desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_validate_shader_meta(device: *mut struct d3d12_device, meta: *mut const struct vkd3d_shader_meta) -> bool {
    // TODO: implementar d3d12_device_validate_shader_meta desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_scratch_buffer(device: *mut struct d3d12_device, kind: enum vkd3d_scratch_pool_kind, min_size: VkDeviceSize, memory_types: u32, scratch: *mut struct vkd3d_scratch_buffer) -> i32 {
    // TODO: implementar d3d12_device_get_scratch_buffer desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_return_scratch_buffer(device: *mut struct d3d12_device, kind: enum vkd3d_scratch_pool_kind, scratch: *mut const struct vkd3d_scratch_buffer) -> core::ffi::c_void {
    // TODO: implementar d3d12_device_return_scratch_buffer desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_query_pool(device: *mut struct d3d12_device, type_index: u32, pool: *mut struct vkd3d_query_pool) -> i32 {
    // TODO: implementar d3d12_device_get_query_pool desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_return_query_pool(device: *mut struct d3d12_device, pool: *mut const struct vkd3d_query_pool) -> core::ffi::c_void {
    // TODO: implementar d3d12_device_return_query_pool desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_descriptor_heap_gpu_va(device: *mut struct d3d12_device, type: D3D12_DESCRIPTOR_HEAP_TYPE) -> u64 {
    // TODO: implementar d3d12_device_get_descriptor_heap_gpu_va desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_return_descriptor_heap_gpu_va(device: *mut struct d3d12_device, va: u64) -> core::ffi::c_void {
    // TODO: implementar d3d12_device_return_descriptor_heap_gpu_va desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_uses_descriptor_buffers(device: *mut const struct d3d12_device) -> static inline bool {
    // TODO: implementar d3d12_device_uses_descriptor_buffers desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_query_interface(device: *mut struct d3d12_device, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static inline HRESULT {
    // TODO: implementar d3d12_device_query_interface desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device12_QueryInterface(param_21178: &device->ID3D12Device_iface, param_18003: iid, param_39919: object) -> return {
    // TODO: implementar ID3D12Device12_QueryInterface desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_add_ref_common(device: *mut struct d3d12_device) -> ULONG {
    // TODO: implementar d3d12_device_add_ref_common desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_release_common(device: *mut struct d3d12_device) -> ULONG {
    // TODO: implementar d3d12_device_release_common desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_add_ref(device: *mut struct d3d12_device) -> static inline ULONG {
    // TODO: implementar d3d12_device_add_ref desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_release(device: *mut struct d3d12_device) -> static inline ULONG {
    // TODO: implementar d3d12_device_release desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_use_embedded_mutable_descriptors(device: *mut struct d3d12_device) -> static inline bool {
    // TODO: implementar d3d12_device_use_embedded_mutable_descriptors desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_decode_metadata(device: *mut struct d3d12_device, va: vkd3d_cpu_descriptor_va_t) -> static inline struct d3d12_desc_split_metadata {
    // TODO: implementar d3d12_desc_decode_metadata desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_use_ssbo_raw_buffer(device: *mut struct d3d12_device) -> static inline bool {
    // TODO: implementar d3d12_device_use_ssbo_raw_buffer desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_ssbo_alignment(device: *mut struct d3d12_device) -> static inline VkDeviceSize {
    // TODO: implementar d3d12_device_get_ssbo_alignment desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_use_ssbo_root_descriptors(device: *mut struct d3d12_device) -> static inline bool {
    // TODO: implementar d3d12_device_use_ssbo_root_descriptors desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_variable_shading_rate_tier_1(device: *mut struct d3d12_device) -> bool {
    // TODO: implementar d3d12_device_supports_variable_shading_rate_tier_1 desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_variable_shading_rate_tier_2(device: *mut struct d3d12_device) -> bool {
    // TODO: implementar d3d12_device_supports_variable_shading_rate_tier_2 desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_ray_tracing_tier_1_0(device: *mut const struct d3d12_device) -> bool {
    // TODO: implementar d3d12_device_supports_ray_tracing_tier_1_0 desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_ray_tracing_tier_1_2(device: *mut const struct d3d12_device) -> bool {
    // TODO: implementar d3d12_device_supports_ray_tracing_tier_1_2 desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_determine_shading_rate_image_tile_size(device: *mut struct d3d12_device) -> UINT {
    // TODO: implementar d3d12_determine_shading_rate_image_tile_size desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_required_subgroup_size_for_stage(device: *mut struct d3d12_device, stage: VkShaderStageFlagBits) -> bool {
    // TODO: implementar d3d12_device_supports_required_subgroup_size_for_stage desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_workgraphs(device: *mut const struct d3d12_device) -> bool {
    // TODO: implementar d3d12_device_supports_workgraphs desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_unified_layouts(device: *mut const struct d3d12_device) -> static inline bool {
    // TODO: implementar d3d12_device_supports_unified_layouts desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_register_swapchain(device: *mut struct d3d12_device, chain: *mut struct dxgi_vk_swap_chain) -> static inline void {
    // TODO: implementar d3d12_device_register_swapchain desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_remove_swapchain(device: *mut struct d3d12_device, chain: *mut struct dxgi_vk_swap_chain) -> static inline void {
    // TODO: implementar d3d12_device_remove_swapchain desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_association_data_equal(a: *mut const struct d3d12_state_object_association, b: *mut const struct d3d12_state_object_association) -> bool {
    // TODO: implementar d3d12_state_object_association_data_equal desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_rt_state_object_add(device: *mut struct d3d12_device, desc: *mut const D3D12_STATE_OBJECT_DESC, parent: *mut struct d3d12_rt_state_object, param_53274: *mut struct d3d12_rt_state_object) -> i32 {
    // TODO: implementar d3d12_rt_state_object_add desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_initialize_scratch(list: *mut struct d3d12_command_list) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_list_workgraph_initialize_scratch desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_dispatch(list: *mut struct d3d12_command_list, desc: *mut const D3D12_DISPATCH_GRAPH_DESC) -> core::ffi::c_void {
    // TODO: implementar d3d12_command_list_workgraph_dispatch desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_box_is_empty(box: *mut const D3D12_BOX) -> static inline bool {
    // TODO: implementar d3d12_box_is_empty desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_get_layer_count(desc: *mut const D3D12_RESOURCE_DESC1) -> static inline unsigned int {
    // TODO: implementar d3d12_resource_desc_get_layer_count desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_get_view_subresource_extent(resource: *mut const struct d3d12_resource, view: *mut const struct vkd3d_view) -> static inline VkExtent3D {
    // TODO: implementar d3d12_resource_get_view_subresource_extent desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_get_active_level_count(desc: *mut const D3D12_RESOURCE_DESC1) -> static inline unsigned int {
    // TODO: implementar d3d12_resource_desc_get_active_level_count desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_is_sampler_feedback(param_7058: desc) -> return {
    // TODO: implementar d3d12_resource_desc_is_sampler_feedback desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_get_active_feedback_extent(desc: *mut const D3D12_RESOURCE_DESC1, mip_level: u32) -> static inline VkExtent3D {
    // TODO: implementar d3d12_resource_desc_get_active_feedback_extent desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_get_padded_feedback_extent(desc: *mut const D3D12_RESOURCE_DESC1) -> static inline VkExtent3D {
    // TODO: implementar d3d12_resource_desc_get_padded_feedback_extent desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_get_sub_resource_count_per_plane(desc: *mut const D3D12_RESOURCE_DESC1) -> static inline unsigned int {
    // TODO: implementar d3d12_resource_desc_get_sub_resource_count_per_plane desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_get_sub_resource_count(device: *mut const struct d3d12_device, desc: *mut const D3D12_RESOURCE_DESC1) -> static inline unsigned int {
    // TODO: implementar d3d12_resource_desc_get_sub_resource_count desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12GraphicsCommandList_ClearUnorderedAccessViewUint(param_52213: command_list, param_11565: *mut core::ffi::c_void, param_69: gpu_heap, param_6097: 0) -> else {
    // TODO: implementar ID3D12GraphicsCommandList_ClearUnorderedAccessViewUint desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12GraphicsCommandList_ExecuteIndirect(param_51475: context.list, param_47226: sig, param_4655: 1, param_56378: indirect_graphics, draw_arguments: *mut core::ffi::c_void) -> else {
    // TODO: implementar ID3D12GraphicsCommandList_ExecuteIndirect desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device2_Release(param_43501: device2) -> hr {
    // TODO: implementar ID3D12Device2_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device_GetDescriptorHandleIncrementSize(param_2088: context.device, param_22890: D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV) -> *mut i {
    // TODO: implementar ID3D12Device_GetDescriptorHandleIncrementSize desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12GraphicsCommandList8_OMSetStencilRef(param_10049: command_list8) -> else {
    // TODO: implementar ID3D12GraphicsCommandList8_OMSetStencilRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Fence_Release(param_28937: fence) -> Unexpected hr {
    // TODO: implementar ID3D12Fence_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12DeviceFactory_SetFlags(param_41434: factory, param_64115: D3D12_DEVICE_FACTORY_FLAG_DISALLOW_STORING_NEW_DEVICE_AS_SINGLETON) -> Unexpected flags {
    // TODO: implementar ID3D12DeviceFactory_SetFlags desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12DeviceFactory_ApplyToGlobalState(param_41434: factory) -> Unexpected flags {
    // TODO: implementar ID3D12DeviceFactory_ApplyToGlobalState desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device_Release(param_47516: device) -> Unexpected hr {
    // TODO: implementar ID3D12Device_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Device5_Release(param_43548: context->device5) -> Raytracing tier {
    // TODO: implementar ID3D12Device5_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12GraphicsCommandList_OMSetRenderTargets(param_51475: context.list, param_6097: 0, param_50043: NULL, param_23836: FALSE, param_50043: NULL) -> hr {
    // TODO: implementar ID3D12GraphicsCommandList_OMSetRenderTargets desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12GraphicsCommandList4_Reset(param_36393: altlist, param_8618: context.allocator, param_50043: NULL) -> hr {
    // TODO: implementar ID3D12GraphicsCommandList4_Reset desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Resource_Release(param_55946: resource) -> hr {
    // TODO: implementar ID3D12Resource_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Heap_Release(param_60568: heap) -> hr {
    // TODO: implementar ID3D12Heap_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12GraphicsCommandList_SetGraphicsRootSignature(param_51475: context.list, param_37407: context.root_signature) -> hr {
    // TODO: implementar ID3D12GraphicsCommandList_SetGraphicsRootSignature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Fence_Signal(param_28937: fence, param_25910: 8) -> else {
    // TODO: implementar ID3D12Fence_Signal desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12RootSignature_Release(param_9154: desc.pRootSignature) -> hr {
    // TODO: implementar ID3D12RootSignature_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12PipelineLibrary_Release(param_60635: lib) -> hr {
    // TODO: implementar ID3D12PipelineLibrary_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3D12SerializeRootSignature(pRootSignature: *mut _In_ const D3D12_ROOT_SIGNATURE_DESC, Version: _In_ D3D_ROOT_SIGNATURE_VERSION, ppBlob: *mut core::ffi::c_void, param_23427: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar D3D12SerializeRootSignature desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn have_d3d12_device() -> static bool {
    // TODO: implementar have_d3d12_device desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn load_d3d12core_module(module_name: *mut const char) -> static bool {
    // TODO: implementar load_d3d12core_module desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn load_d3d12core_once() -> static void {
    // TODO: implementar load_d3d12core_once desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn load_d3d12core() -> static bool {
    // TODO: implementar load_d3d12core desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3D12SerializeVersionedRootSignature(pRootSignature: *mut _In_ const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppBlob: *mut core::ffi::c_void, param_23427: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar D3D12SerializeVersionedRootSignature desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3D12GetDebugInterface(riid: _In_ REFIID, ppvDebug: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar D3D12GetDebugInterface desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_sdk_configuration_AddRef(iface: *mut ID3D12SDKConfiguration1) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_sdk_configuration_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_sdk_configuration_Release(iface: *mut ID3D12SDKConfiguration1) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_sdk_configuration_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_sdk_configuration_QueryInterface(iface: *mut ID3D12SDKConfiguration1, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_sdk_configuration_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_sdk_configuration_SetSDKVersion(iface: *mut ID3D12SDKConfiguration1, SDKVersion: UINT, SDKPath: LPCSTR) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_sdk_configuration_SetSDKVersion desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3D12GetInterface(rclsid: _In_ REFCLSID, riid: _In_ REFIID, ppvDebug: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar D3D12GetInterface desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_dred_settings_QueryInterface(iface: *mut d3d12_dred_settings_iface, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_dred_settings_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_dred_settings_SetPageFaultEnablement(iface: *mut d3d12_dred_settings_iface, enablement: D3D12_DRED_ENABLEMENT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_dred_settings_SetPageFaultEnablement desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_dred_settings_SetWatsonDumpEnablement(iface: *mut d3d12_dred_settings_iface, enablement: D3D12_DRED_ENABLEMENT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_dred_settings_SetWatsonDumpEnablement desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_dred_settings_init(object: *mut struct d3d12_dred_settings) -> static void {
    // TODO: implementar d3d12_dred_settings_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_get_adapter(param_62061: *mut IDXGIAdapter, adapter: *mut IUnknown) -> static HRESULT {
    // TODO: implementar d3d12_get_adapter desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_find_physical_device(instance: *mut struct vkd3d_instance, pfn_vkGetInstanceProcAddr: PFN_vkGetInstanceProcAddr, adapter_desc: *mut struct DXGI_ADAPTER_DESC) -> static VkPhysicalDevice {
    // TODO: implementar d3d12_find_physical_device desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12core_SerializeRootSignature(core: *mut d3d12core_interface, root_signature_desc: *mut const D3D12_ROOT_SIGNATURE_DESC, version: D3D_ROOT_SIGNATURE_VERSION, param_52875: *mut ID3DBlob, param_52875: *mut ID3DBlob) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12core_SerializeRootSignature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12core_SerializeVersionedRootSignature(core: *mut d3d12core_interface, desc: *mut const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, param_52875: *mut ID3DBlob, param_52875: *mut ID3DBlob) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12core_SerializeVersionedRootSignature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12core_GetDebugInterface(core: *mut d3d12core_interface, iid: REFIID, debug: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12core_GetDebugInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12core_D3D12GetInterface(rcslid: REFCLSID, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT {
    // TODO: implementar d3d12core_D3D12GetInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12core_GetInterface(core: *mut d3d12core_interface, rcslid: REFCLSID, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12core_GetInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_AddRef(iface: *mut ID3D12DeviceFactory) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_factory_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_Release(iface: *mut ID3D12DeviceFactory) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_factory_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_QueryInterface(iface: *mut ID3D12DeviceFactory, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_factory_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_InitializeFromGlobalState(iface: *mut ID3D12DeviceFactory) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_factory_InitializeFromGlobalState desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_ApplyToGlobalState(iface: *mut ID3D12DeviceFactory) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_factory_ApplyToGlobalState desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_SetFlags(iface: *mut ID3D12DeviceFactory, flags: D3D12_DEVICE_FACTORY_FLAGS) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_factory_SetFlags desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_GetFlags(iface: *mut ID3D12DeviceFactory) -> static D3D12_DEVICE_FACTORY_FLAGS STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_factory_GetFlags desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_GetConfigurationInterface(iface: *mut ID3D12DeviceFactory, clsid: REFCLSID, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_factory_GetConfigurationInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_configuration_AddRef(iface: *mut ID3D12DeviceConfiguration1) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_configuration_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_configuration_Release(iface: *mut ID3D12DeviceConfiguration1) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_configuration_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_configuration_QueryInterface(iface: *mut ID3D12DeviceConfiguration1, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_configuration_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_configuration_GetDesc(iface: *mut ID3D12DeviceConfiguration1, desc: *mut D3D12_DEVICE_CONFIGURATION_DESC) -> static D3D12_DEVICE_CONFIGURATION_DESC * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_configuration_GetDesc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_configuration_SerializeVersionedRootSignature(iface: *mut ID3D12DeviceConfiguration1, pDesc: *mut const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, param_52875: *mut ID3DBlob, param_52875: *mut ID3DBlob) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_configuration_SerializeVersionedRootSignature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_QueryInterface(iface: *mut d3d12_command_list_iface, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_AddRef(iface: *mut d3d12_command_list_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_Release(iface: *mut d3d12_command_list_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_GetPrivateData(iface: *mut d3d12_command_list_iface, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetPrivateData(iface: *mut d3d12_command_list_iface, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetPrivateDataInterface(iface: *mut d3d12_command_list_iface, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_GetDevice(iface: *mut d3d12_command_list_iface, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_GetType(iface: *mut d3d12_command_list_iface) -> static D3D12_COMMAND_LIST_TYPE STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_GetType desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_Close(iface: *mut d3d12_command_list_iface) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_Close desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetPipelineState(iface: *mut d3d12_command_list_iface, pipeline_state: *mut ID3D12PipelineState) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetPipelineState desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_Reset(iface: *mut d3d12_command_list_iface, allocator: *mut ID3D12CommandAllocator, initial_pipeline_state: *mut ID3D12PipelineState) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_Reset desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_ClearState(iface: *mut d3d12_command_list_iface, pipeline_state: *mut ID3D12PipelineState) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_ClearState desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_dispatch(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_dispatch desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_Dispatch(iface: *mut d3d12_command_list_iface, x: UINT, y: UINT, z: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_Dispatch desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_CopyResource(iface: *mut d3d12_command_list_iface, dst: *mut ID3D12Resource, src: *mut ID3D12Resource) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_CopyResource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_RSSetViewports(iface: *mut d3d12_command_list_iface, viewport_count: UINT, viewports: *mut const D3D12_VIEWPORT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_RSSetViewports desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_RSSetScissorRects(iface: *mut d3d12_command_list_iface, rect_count: UINT, rects: *mut const D3D12_RECT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_RSSetScissorRects desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_om_set_blend_factor(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_om_set_blend_factor desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_OMSetBlendFactor(iface: *mut d3d12_command_list_iface) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_OMSetBlendFactor desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_om_set_stencil_ref(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_om_set_stencil_ref desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_OMSetStencilRef(iface: *mut d3d12_command_list_iface, stencil_ref: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_OMSetStencilRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_pipeline_state(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_pipeline_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_ResourceBarrier(iface: *mut d3d12_command_list_iface, barrier_count: UINT, barriers: *mut const D3D12_RESOURCE_BARRIER) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_ResourceBarrier desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_ExecuteBundle(iface: *mut d3d12_command_list_iface, command_list: *mut ID3D12GraphicsCommandList) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_ExecuteBundle desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetDescriptorHeaps(iface: *mut d3d12_command_list_iface, heap_count: UINT, heaps: *mut core::ffi::c_void) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetDescriptorHeaps desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_compute_root_signature(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_compute_root_signature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetComputeRootSignature(iface: *mut d3d12_command_list_iface, root_signature: *mut ID3D12RootSignature) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetComputeRootSignature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_graphics_root_signature(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_graphics_root_signature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetGraphicsRootSignature(iface: *mut d3d12_command_list_iface, root_signature: *mut ID3D12RootSignature) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetGraphicsRootSignature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_compute_root_descriptor_table(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_compute_root_descriptor_table desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetComputeRootDescriptorTable(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, base_descriptor: D3D12_GPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetComputeRootDescriptorTable desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_graphics_root_descriptor_table(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_graphics_root_descriptor_table desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetGraphicsRootDescriptorTable(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, base_descriptor: D3D12_GPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetGraphicsRootDescriptorTable desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_compute_root_cbv(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_compute_root_cbv desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_graphics_root_cbv(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_graphics_root_cbv desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_compute_root_srv(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_compute_root_srv desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetComputeRootShaderResourceView(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, address: D3D12_GPU_VIRTUAL_ADDRESS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetComputeRootShaderResourceView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_graphics_root_srv(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_graphics_root_srv desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetGraphicsRootShaderResourceView(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, address: D3D12_GPU_VIRTUAL_ADDRESS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetGraphicsRootShaderResourceView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_compute_root_uav(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_compute_root_uav desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetComputeRootUnorderedAccessView(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, address: D3D12_GPU_VIRTUAL_ADDRESS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetComputeRootUnorderedAccessView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_graphics_root_uav(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_graphics_root_uav desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetGraphicsRootUnorderedAccessView(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, address: D3D12_GPU_VIRTUAL_ADDRESS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetGraphicsRootUnorderedAccessView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_ia_set_index_buffer_null(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_ia_set_index_buffer_null desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_ia_set_index_buffer(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_ia_set_index_buffer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_IASetIndexBuffer(iface: *mut d3d12_command_list_iface, view: *mut const D3D12_INDEX_BUFFER_VIEW) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_IASetIndexBuffer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_ia_set_vertex_buffers(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_ia_set_vertex_buffers desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_IASetVertexBuffers(iface: *mut d3d12_command_list_iface, start_slot: UINT, view_count: UINT, views: *mut const D3D12_VERTEX_BUFFER_VIEW) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_IASetVertexBuffers desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_ClearDepthStencilView(iface: *mut d3d12_command_list_iface, dsv: D3D12_CPU_DESCRIPTOR_HANDLE, flags: D3D12_CLEAR_FLAGS, depth: f32, stencil: UINT8, rect_count: UINT, rects: *mut const D3D12_RECT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_ClearDepthStencilView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_ClearRenderTargetView(iface: *mut d3d12_command_list_iface, rtv: D3D12_CPU_DESCRIPTOR_HANDLE, rect_count: UINT, rects: *mut const D3D12_RECT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_ClearRenderTargetView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_DiscardResource(iface: *mut d3d12_command_list_iface, resource: *mut ID3D12Resource, region: *mut const D3D12_DISCARD_REGION) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_DiscardResource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_marker(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_marker desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetMarker(iface: *mut d3d12_command_list_iface, metadata: UINT, data: *mut const void, size: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetMarker desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_begin_event(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_begin_event desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_BeginEvent(iface: *mut d3d12_command_list_iface, metadata: UINT, data: *mut const void, size: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_BeginEvent desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_end_event(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_end_event desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_EndEvent(iface: *mut d3d12_command_list_iface) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_EndEvent desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_execute_indirect(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_execute_indirect desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_ExecuteIndirect(iface: *mut d3d12_command_list_iface, command_signature: *mut ID3D12CommandSignature, max_command_count: UINT, arg_buffer: *mut ID3D12Resource, arg_buffer_offset: UINT64, count_buffer: *mut ID3D12Resource, count_buffer_offset: UINT64) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_ExecuteIndirect desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_om_set_depth_bounds(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_om_set_depth_bounds desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_OMSetDepthBounds(iface: *mut d3d12_command_list_iface, min: FLOAT, max: FLOAT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_OMSetDepthBounds desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_sample_positions(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_sample_positions desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetSamplePositions(iface: *mut d3d12_command_list_iface, sample_count: UINT, pixel_count: UINT, sample_positions: *mut D3D12_SAMPLE_POSITION) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetSamplePositions desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetProtectedResourceSession(iface: *mut d3d12_command_list_iface, protected_session: *mut ID3D12ProtectedResourceSession) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetProtectedResourceSession desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_EndRenderPass(iface: *mut d3d12_command_list_iface) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_EndRenderPass desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_CopyRaytracingAccelerationStructure(iface: *mut d3d12_command_list_iface, dst_data: D3D12_GPU_VIRTUAL_ADDRESS, src_data: D3D12_GPU_VIRTUAL_ADDRESS, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_CopyRaytracingAccelerationStructure desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_pipeline_state1(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_pipeline_state1 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetPipelineState1(iface: *mut d3d12_command_list_iface, state_object: *mut ID3D12StateObject) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetPipelineState1 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_dispatch_rays(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_dispatch_rays desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_DispatchRays(iface: *mut d3d12_command_list_iface, desc: *mut const D3D12_DISPATCH_RAYS_DESC) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_DispatchRays desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_rs_set_shading_rate(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_rs_set_shading_rate desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_rs_set_shading_rate_base(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_rs_set_shading_rate_base desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_RSSetShadingRate(iface: *mut d3d12_command_list_iface, base: D3D12_SHADING_RATE, combiners: *mut const D3D12_SHADING_RATE_COMBINER) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_RSSetShadingRate desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_rs_set_shading_rate_image(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_rs_set_shading_rate_image desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_RSSetShadingRateImage(iface: *mut d3d12_command_list_iface, image: *mut ID3D12Resource) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_RSSetShadingRateImage desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_dispatch_mesh(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_dispatch_mesh desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_DispatchMesh(iface: *mut d3d12_command_list_iface, x: UINT, y: UINT, z: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_DispatchMesh desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_om_set_front_and_back_stencil_ref(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_om_set_front_and_back_stencil_ref desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_rs_set_depth_bias(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_rs_set_depth_bias desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_RSSetDepthBias(iface: *mut d3d12_command_list_iface, DepthBias: FLOAT, DepthBiasClamp: FLOAT, SlopeScaledDepthBias: FLOAT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_RSSetDepthBias desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetProgram(iface: *mut d3d12_command_list_iface, desc: *mut const D3D12_SET_PROGRAM_DESC) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetProgram desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_DispatchGraph(iface: *mut d3d12_command_list_iface, desc: *mut const D3D12_DISPATCH_GRAPH_DESC) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_DispatchGraph desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_ia_set_index_buffer_strip_cut_value(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_ia_set_index_buffer_strip_cut_value desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_IASetIndexBufferStripCutValue(iface: *mut d3d12_command_list_iface, IBStripCutValue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_IASetIndexBufferStripCutValue desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn impl_from_ID3D12GraphicsCommandList(param_34011: *mut core::ffi::c_void) -> return {
    // TODO: implementar impl_from_ID3D12GraphicsCommandList desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_cached_pipeline_state_to_flags(state: *mut const struct d3d12_cached_pipeline_state) -> static uint32_t {
    // TODO: implementar d3d12_cached_pipeline_state_to_flags desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_find_internal_blob(pipeline_library: *mut struct d3d12_pipeline_library, map: *mut const struct hash_map, hash: u64, param_43276: *mut const void, size: *mut usize) -> static bool {
    // TODO: implementar d3d12_pipeline_library_find_internal_blob desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_cached_pipeline_entry_name_table_size(entry: *mut const struct vkd3d_cached_pipeline_entry) -> static uint32_t {
    // TODO: implementar d3d12_cached_pipeline_entry_name_table_size desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_insert_hash_map_blob_internal(pipeline_library: *mut struct d3d12_pipeline_library, map: *mut struct hash_map, entry: *mut const struct vkd3d_cached_pipeline_entry) -> static bool {
    // TODO: implementar d3d12_pipeline_library_insert_hash_map_blob_internal desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_serialize_entry(entry: *mut const struct vkd3d_cached_pipeline_entry, header: *mut struct vkd3d_serialized_pipeline_toc_entry, data: *mut u8, name_offset: usize, blob_offset: usize) -> static void {
    // TODO: implementar d3d12_pipeline_library_serialize_entry desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_cleanup_map(map: *mut struct hash_map) -> static void {
    // TODO: implementar d3d12_pipeline_library_cleanup_map desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_cleanup(pipeline_library: *mut struct d3d12_pipeline_library, device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_pipeline_library_cleanup desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_QueryInterface(iface: *mut d3d12_pipeline_library_iface, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_library_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_AddRef(iface: *mut d3d12_pipeline_library_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_library_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_Release(iface: *mut d3d12_pipeline_library_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_library_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_GetPrivateData(iface: *mut d3d12_pipeline_library_iface, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_library_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_SetPrivateData(iface: *mut d3d12_pipeline_library_iface, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_library_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_SetPrivateDataInterface(iface: *mut d3d12_pipeline_library_iface, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_library_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_GetDevice(iface: *mut d3d12_pipeline_library_iface, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_library_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_StorePipeline(iface: *mut d3d12_pipeline_library_iface, name: LPCWSTR, pipeline: *mut ID3D12PipelineState) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_library_StorePipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_load_pipeline(pipeline_library: *mut struct d3d12_pipeline_library, name: LPCWSTR, bind_point: VkPipelineBindPoint, desc: *mut struct d3d12_pipeline_state_desc, param_22057: *mut struct d3d12_pipeline_state) -> static HRESULT {
    // TODO: implementar d3d12_pipeline_library_load_pipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_LoadGraphicsPipeline(iface: *mut d3d12_pipeline_library_iface, name: LPCWSTR, desc: *mut const D3D12_GRAPHICS_PIPELINE_STATE_DESC, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_library_LoadGraphicsPipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_LoadComputePipeline(iface: *mut d3d12_pipeline_library_iface, name: LPCWSTR, desc: *mut const D3D12_COMPUTE_PIPELINE_STATE_DESC, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_library_LoadComputePipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_get_aligned_name_table_size(pipeline_library: *mut struct d3d12_pipeline_library) -> static size_t {
    // TODO: implementar d3d12_pipeline_library_get_aligned_name_table_size desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_get_serialized_size(pipeline_library: *mut struct d3d12_pipeline_library) -> static size_t {
    // TODO: implementar d3d12_pipeline_library_get_serialized_size desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_GetSerializedSize(iface: *mut d3d12_pipeline_library_iface) -> static SIZE_T STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_library_GetSerializedSize desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_serialize_hash_map(map: *mut const struct hash_map, param_4141: *mut struct vkd3d_serialized_pipeline_toc_entry, serialized_data: *mut u8, inout_name_offset: *mut usize, inout_blob_offset: *mut usize) -> static void {
    // TODO: implementar d3d12_pipeline_library_serialize_hash_map desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_Serialize(iface: *mut d3d12_pipeline_library_iface, data: *mut core::ffi::c_void, data_size: SIZE_T) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_library_Serialize desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_LoadPipeline(iface: *mut d3d12_pipeline_library_iface, name: LPCWSTR, desc: *mut const D3D12_PIPELINE_STATE_STREAM_DESC, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_library_LoadPipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_unserialize_hash_map(pipeline_library: *mut struct d3d12_pipeline_library, entries: *mut const struct vkd3d_serialized_pipeline_toc_entry, entries_count: usize, map: *mut struct hash_map, serialized_data_base: *mut const uint8_t, serialized_data_size: usize, param_38864: *mut const uint8_t) -> static HRESULT {
    // TODO: implementar d3d12_pipeline_library_unserialize_hash_map desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_init(pipeline_library: *mut struct d3d12_pipeline_library, device: *mut struct d3d12_device, blob: *mut const void, blob_length: usize, flags: u32) -> static HRESULT {
    // TODO: implementar d3d12_pipeline_library_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_signal(fence: *mut struct d3d12_fence, worker: *mut struct vkd3d_fence_worker, value: u64) -> static HRESULT {
    // TODO: implementar d3d12_fence_signal desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_add_submission(queue: *mut struct d3d12_command_queue, sub: *mut const struct d3d12_command_queue_submission) -> static void {
    // TODO: implementar d3d12_command_queue_add_submission desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_inc_ref(fence: *mut struct d3d12_fence) -> static void {
    // TODO: implementar d3d12_fence_inc_ref desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_dec_ref(fence: *mut struct d3d12_fence) -> static void {
    // TODO: implementar d3d12_fence_dec_ref desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_inc_ref(fence: *mut struct d3d12_shared_fence) -> static void {
    // TODO: implementar d3d12_shared_fence_inc_ref desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_dec_ref(fence: *mut struct d3d12_shared_fence) -> static void {
    // TODO: implementar d3d12_shared_fence_dec_ref desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_iface_inc_ref(iface: *mut d3d12_fence_iface) -> static void {
    // TODO: implementar d3d12_fence_iface_inc_ref desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_iface_dec_ref(iface: *mut d3d12_fence_iface) -> static void {
    // TODO: implementar d3d12_fence_iface_dec_ref desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_signal_cpu_timeline_semaphore(fence: *mut struct d3d12_fence, value: u64) -> static HRESULT {
    // TODO: implementar d3d12_fence_signal_cpu_timeline_semaphore desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_barrier_batch_init(batch: *mut struct d3d12_command_list_barrier_batch) -> static void {
    // TODO: implementar d3d12_command_list_barrier_batch_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_barrier_batch_end(list: *mut struct d3d12_command_list, batch: *mut struct d3d12_command_list_barrier_batch) -> static void {
    // TODO: implementar d3d12_command_list_barrier_batch_end desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_barrier_batch_add_layout_transition(list: *mut struct d3d12_command_list, batch: *mut struct d3d12_command_list_barrier_batch, image_barrier: *mut const VkImageMemoryBarrier2) -> static void {
    // TODO: implementar d3d12_command_list_barrier_batch_add_layout_transition desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_barrier_batch_add_global_transition(list: *mut struct d3d12_command_list, batch: *mut struct d3d12_command_list_barrier_batch, srcStageMask: VkPipelineStageFlags2, srcAccessMask: VkAccessFlags2, dstStageMask: VkPipelineStageFlags2, dstAccessMask: VkAccessFlags2) -> static void {
    // TODO: implementar d3d12_command_list_barrier_batch_add_global_transition desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_promote_dsv_resource(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, plane_optimal_mask: u32) -> static uint32_t {
    // TODO: implementar d3d12_command_list_promote_dsv_resource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_notify_decay_dsv_resource(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource) -> static uint32_t {
    // TODO: implementar d3d12_command_list_notify_decay_dsv_resource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_notify_dsv_discard(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, first_subresource: u32, subresource_count: u32, resource_subresource_count: u32) -> static void {
    // TODO: implementar d3d12_command_list_notify_dsv_discard desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_get_depth_stencil_resource_layout(list: *mut const struct d3d12_command_list, resource: *mut const struct d3d12_resource, plane_optimal_mask: *mut u32) -> static VkImageLayout {
    // TODO: implementar d3d12_command_list_get_depth_stencil_resource_layout desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_decay_optimal_dsv_resource(list: *mut struct d3d12_command_list, resource: *mut const struct d3d12_resource, plane_optimal_mask: u32, batch: *mut struct d3d12_command_list_barrier_batch) -> static void {
    // TODO: implementar d3d12_command_list_decay_optimal_dsv_resource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_end_transfer_batch(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_end_transfer_batch desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_end_transfer_batch_if_trivial(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_end_transfer_batch_if_trivial desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_end_wbi_batch(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_end_wbi_batch desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ensure_transfer_batch(list: *mut struct d3d12_command_list, type: enum vkd3d_batch_type) -> static inline void {
    // TODO: implementar d3d12_command_list_ensure_transfer_batch desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_flush_rtas_batch(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_flush_rtas_batch desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_clear_rtas_batch(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_clear_rtas_batch desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_flush_query_resolves(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_flush_query_resolves desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_ensure_fence_signal_order(command_queue: *mut struct d3d12_command_queue, fence: *mut struct d3d12_fence, update_count: u64) -> static void {
    // TODO: implementar d3d12_command_queue_ensure_fence_signal_order desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_register_pending_gpu_wait(fence: *mut struct d3d12_fence, value: u64) -> u64 {
    // TODO: implementar d3d12_fence_register_pending_gpu_wait desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_QueryInterface(iface: *mut d3d12_fence_iface, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_fence_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_AddRef(iface: *mut d3d12_fence_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_fence_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_Release(iface: *mut d3d12_fence_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_fence_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_GetPrivateData(iface: *mut d3d12_fence_iface, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_fence_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_SetPrivateData(iface: *mut d3d12_fence_iface, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_fence_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_SetPrivateDataInterface(iface: *mut d3d12_fence_iface, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_fence_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_GetDevice(iface: *mut d3d12_fence_iface, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_fence_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_GetCompletedValue(iface: *mut d3d12_fence_iface) -> static UINT64 STDMETHODCALLTYPE {
    // TODO: implementar d3d12_fence_GetCompletedValue desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_SetEventOnCompletion(iface: *mut d3d12_fence_iface, value: UINT64, event: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_fence_SetEventOnCompletion desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_Signal(iface: *mut d3d12_fence_iface, value: UINT64) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_fence_Signal desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_GetCreationFlags(iface: *mut d3d12_fence_iface) -> static D3D12_FENCE_FLAGS STDMETHODCALLTYPE {
    // TODO: implementar d3d12_fence_GetCreationFlags desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_init(fence: *mut struct d3d12_fence, device: *mut struct d3d12_device, initial_value: UINT64, flags: D3D12_FENCE_FLAGS) -> static HRESULT {
    // TODO: implementar d3d12_fence_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_QueryInterface(iface: *mut d3d12_fence_iface, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_shared_fence_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_AddRef(iface: *mut d3d12_fence_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_shared_fence_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_Release(iface: *mut d3d12_fence_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_shared_fence_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_GetPrivateData(iface: *mut d3d12_fence_iface, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_shared_fence_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_SetPrivateData(iface: *mut d3d12_fence_iface, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_shared_fence_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_SetPrivateDataInterface(iface: *mut d3d12_fence_iface, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_shared_fence_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_GetDevice(iface: *mut d3d12_fence_iface, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_shared_fence_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_GetCompletedValue(iface: *mut d3d12_fence_iface) -> static UINT64 STDMETHODCALLTYPE {
    // TODO: implementar d3d12_shared_fence_GetCompletedValue desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_SetEventOnCompletion(iface: *mut d3d12_fence_iface, value: UINT64, os_event: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_shared_fence_SetEventOnCompletion desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_Signal(iface: *mut d3d12_fence_iface, value: UINT64) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_shared_fence_Signal desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_GetCreationFlags(iface: *mut d3d12_fence_iface) -> static D3D12_FENCE_FLAGS STDMETHODCALLTYPE {
    // TODO: implementar d3d12_shared_fence_GetCreationFlags desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_mark_as_invalid(list: *mut struct d3d12_command_list, message: *mut const char) -> static void {
    // TODO: implementar d3d12_command_list_mark_as_invalid desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_conditional_rendering_state(list: *mut struct d3d12_command_list, end: bool) -> static void {
    // TODO: implementar d3d12_command_list_update_conditional_rendering_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_begin_command_buffer(list: *mut struct d3d12_command_list) -> static HRESULT {
    // TODO: implementar d3d12_command_list_begin_command_buffer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_begin_new_sequence(list: *mut struct d3d12_command_list, fallback: bool) -> static void {
    // TODO: implementar d3d12_command_list_begin_new_sequence desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_allows_new_sequence(list: *mut struct d3d12_command_list) -> static bool {
    // TODO: implementar d3d12_command_list_allows_new_sequence desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_consider_new_sequence(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_consider_new_sequence desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_debug_mark_begin_region_cmd(list: *mut struct d3d12_command_list, vk_cmd: VkCommandBuffer, tag: *mut const char) -> static void {
    // TODO: implementar d3d12_command_list_debug_mark_begin_region_cmd desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_debug_mark_end_region_cmd(list: *mut struct d3d12_command_list, vk_cmd: VkCommandBuffer) -> static void {
    // TODO: implementar d3d12_command_list_debug_mark_end_region_cmd desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_register_used_resource(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource) -> static void {
    // TODO: implementar d3d12_command_list_register_used_resource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_find_attachment_view(list: *mut struct d3d12_command_list, resource: *mut const struct d3d12_resource, subresource: *mut const VkImageSubresourceLayers) -> static int {
    // TODO: implementar d3d12_command_list_find_attachment_view desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_find_attachment(list: *mut struct d3d12_command_list, resource: *mut const struct d3d12_resource, view: *mut const struct vkd3d_view) -> static int {
    // TODO: implementar d3d12_command_list_find_attachment desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_clear_attachment_inline(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, view: *mut struct vkd3d_view, attachment_idx: u32, clear_aspects: VkImageAspectFlags, clear_value: *mut const VkClearValue, rect_count: UINT, rects: *mut const D3D12_RECT) -> static void {
    // TODO: implementar d3d12_command_list_clear_attachment_inline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_transfer_waw_tracking(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_reset_transfer_waw_tracking desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_resolve_transfer_waw(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_resolve_transfer_waw desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_barrier_subresource_range_covers_aspects(resource: *mut const struct d3d12_resource, range: *mut const D3D12_BARRIER_SUBRESOURCE_RANGE) -> static VkImageAspectFlags {
    // TODO: implementar d3d12_barrier_subresource_range_covers_aspects desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_decay_optimal_dsv_resources(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_decay_optimal_dsv_resources desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_may_alias_other_resources(resource: *mut struct d3d12_resource) -> static bool {
    // TODO: implementar d3d12_resource_may_alias_other_resources desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_debug_mark_label_cmd(list: *mut struct d3d12_command_list, vk_cmd: VkCommandBuffer, tag: *mut const char, r: f32, g: f32, b: f32, a: f32) -> static void {
    // TODO: implementar d3d12_command_list_debug_mark_label_cmd desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_load_attachment(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, view: *mut struct vkd3d_view, clear_aspects: VkImageAspectFlags, clear_value: *mut const VkClearValue, rect_count: UINT, rects: *mut const D3D12_RECT, load_op: VkAttachmentLoadOp) -> static void {
    // TODO: implementar d3d12_command_list_load_attachment desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_discard_attachment(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, subresources: *mut const VkImageSubresourceRange) -> static void {
    // TODO: implementar d3d12_command_list_discard_attachment desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_discard_attachment_barrier(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, subresources: *mut const VkImageSubresourceRange, is_bound: bool) -> static void {
    // TODO: implementar d3d12_command_list_discard_attachment_barrier desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_track_resource_usage(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, perform_initial_transition: bool) -> static void {
    // TODO: implementar d3d12_command_list_track_resource_usage desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_subresource_data(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, subresource: VkImageSubresourceLayers) -> static void {
    // TODO: implementar d3d12_command_list_update_subresource_data desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_flush_subresource_updates(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_flush_subresource_updates desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_emit_render_pass_transition(list: *mut struct d3d12_command_list, mode: enum vkd3d_render_pass_transition_mode) -> static void {
    // TODO: implementar d3d12_command_list_emit_render_pass_transition desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_begin_active_query(list: *mut struct d3d12_command_list, query: *mut struct vkd3d_active_query) -> static void {
    // TODO: implementar d3d12_command_list_begin_active_query desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_end_active_query(list: *mut struct d3d12_command_list, query: *mut struct vkd3d_active_query) -> static void {
    // TODO: implementar d3d12_command_list_end_active_query desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_active_query(list: *mut struct d3d12_command_list, query: *mut struct vkd3d_active_query) -> static void {
    // TODO: implementar d3d12_command_list_reset_active_query desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_enable_query(list: *mut struct d3d12_command_list, heap: *mut struct d3d12_query_heap, index: u32, type: D3D12_QUERY_TYPE) -> static bool {
    // TODO: implementar d3d12_command_list_enable_query desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_disable_query(list: *mut struct d3d12_command_list, heap: *mut struct d3d12_query_heap, index: u32) -> static bool {
    // TODO: implementar d3d12_command_list_disable_query desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_handle_active_queries(list: *mut struct d3d12_command_list, end: bool) -> static void {
    // TODO: implementar d3d12_command_list_handle_active_queries desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_gather_pending_queries(list: *mut struct d3d12_command_list) -> static bool {
    // TODO: implementar d3d12_command_list_gather_pending_queries desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_fuse_attachment_clear(list: *mut struct d3d12_command_list, attachment: *mut VkRenderingAttachmentInfo, stencil_attachment: *mut VkRenderingAttachmentInfo, resource: *mut struct d3d12_resource, view: *mut struct vkd3d_view, aspect_mask: VkImageAspectFlags, batch: *mut struct d3d12_command_list_barrier_batch) -> static void {
    // TODO: implementar d3d12_command_list_fuse_attachment_clear desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_resource_overlaps_attachment(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, subresources: *mut const VkImageSubresourceRange) -> static bool {
    // TODO: implementar d3d12_command_list_resource_overlaps_attachment desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_flush_discards(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, subresources: *mut const VkImageSubresourceRange) -> static void {
    // TODO: implementar d3d12_command_list_flush_discards desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_flush_clears(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, subresources: *mut const VkImageSubresourceRange) -> static void {
    // TODO: implementar d3d12_command_list_flush_clears desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_flush_clears_for_rendering(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_flush_clears_for_rendering desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_copy_render_pass_suspend_resume_compat(list: *mut struct d3d12_command_list, compat: *mut struct d3d12_command_list_render_pass_suspend_resume_compat) -> static void {
    // TODO: implementar d3d12_command_list_copy_render_pass_suspend_resume_compat desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_render_pass_load(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_reset_render_pass_load desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_render_pass_store_resolve(list: *mut struct d3d12_command_list, complete: bool) -> static void {
    // TODO: implementar d3d12_command_list_reset_render_pass_store_resolve desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_end_rendering(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_end_rendering desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_add_transition(list: *mut struct d3d12_command_list, transition: *mut struct vkd3d_initial_transition) -> static void {
    // TODO: implementar d3d12_command_list_add_transition desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_track_query_heap(list: *mut struct d3d12_command_list, heap: *mut struct d3d12_query_heap) -> static void {
    // TODO: implementar d3d12_command_list_track_query_heap desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_QueryInterface(iface: *mut d3d12_command_list_iface, iid: REFIID, param_64866: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_AddRef(iface: *mut d3d12_command_list_iface) -> ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_Release(iface: *mut d3d12_command_list_iface) -> ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_GetPrivateData(iface: *mut d3d12_command_list_iface, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPrivateData(iface: *mut d3d12_command_list_iface, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPrivateDataInterface(iface: *mut d3d12_command_list_iface, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_GetDevice(iface: *mut d3d12_command_list_iface, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_GetType(iface: *mut d3d12_command_list_iface) -> static D3D12_COMMAND_LIST_TYPE STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_GetType desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_batch_reset_query_pools(list: *mut struct d3d12_command_list) -> static HRESULT {
    // TODO: implementar d3d12_command_list_batch_reset_query_pools desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_build_init_commands(list: *mut struct d3d12_command_list) -> static HRESULT {
    // TODO: implementar d3d12_command_list_build_init_commands desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_Close(iface: *mut d3d12_command_list_iface) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_Close desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_insert_query_range(list: *mut struct d3d12_command_list, where: *mut usize, vk_pool: VkQueryPool, index: u32, count: u32, flags: u32) -> static void {
    // TODO: implementar d3d12_command_list_insert_query_range desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_rtv_resolves(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_reset_rtv_resolves desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_api_state(list: *mut struct d3d12_command_list, initial_pipeline_state: *mut ID3D12PipelineState) -> static void {
    // TODO: implementar d3d12_command_list_reset_api_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_internal_state(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_reset_internal_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_reset_state(list: *mut struct d3d12_command_list, initial_pipeline_state: *mut ID3D12PipelineState) -> static void {
    // TODO: implementar d3d12_command_list_reset_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_Reset(iface: *mut d3d12_command_list_iface, allocator: *mut ID3D12CommandAllocator, initial_pipeline_state: *mut ID3D12PipelineState) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_Reset desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ClearState(iface: *mut d3d12_command_list_iface, pipeline_state: *mut ID3D12PipelineState) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_ClearState desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_has_depth_stencil_view(list: *mut struct d3d12_command_list) -> static bool {
    // TODO: implementar d3d12_command_list_has_depth_stencil_view desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_compute_pipeline(list: *mut struct d3d12_command_list) -> static bool {
    // TODO: implementar d3d12_command_list_update_compute_pipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_root_signature(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings, root_signature: *mut const struct d3d12_root_signature) -> static void {
    // TODO: implementar d3d12_command_list_set_root_signature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_raygen_pipeline(list: *mut struct d3d12_command_list) -> static bool {
    // TODO: implementar d3d12_command_list_update_raygen_pipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_check_vbo_alignment(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_check_vbo_alignment desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_graphics_pipeline(list: *mut struct d3d12_command_list, pipeline_type: enum vkd3d_pipeline_type) -> static bool {
    // TODO: implementar d3d12_command_list_update_graphics_pipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_descriptor_table_offsets(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings, layout: VkPipelineLayout, push_stages: VkShaderStageFlags) -> static void {
    // TODO: implementar d3d12_command_list_update_descriptor_table_offsets desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_descriptor_heaps(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings, vk_bind_point: VkPipelineBindPoint, layout: VkPipelineLayout) -> static void {
    // TODO: implementar d3d12_command_list_update_descriptor_heaps desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_static_samplers(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings, vk_bind_point: VkPipelineBindPoint, layout: VkPipelineLayout) -> static void {
    // TODO: implementar d3d12_command_list_update_static_samplers desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_fetch_root_descriptor_vas(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings, dst_data: *mut union vkd3d_root_parameter_data) -> static unsigned int {
    // TODO: implementar d3d12_command_list_fetch_root_descriptor_vas desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_root_descriptors(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings, vk_bind_point: VkPipelineBindPoint, layout: VkPipelineLayout, push_stages: VkShaderStageFlags, root_signature_flags: u32) -> static void {
    // TODO: implementar d3d12_command_list_update_root_descriptors desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_hoisted_descriptors(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings) -> static void {
    // TODO: implementar d3d12_command_list_update_hoisted_descriptors desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_check_pre_compute_barrier(list: *mut struct d3d12_command_list, vk_dst_stage: VkPipelineStageFlagBits2) -> static void {
    // TODO: implementar d3d12_command_list_check_pre_compute_barrier desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_compute_state(list: *mut struct d3d12_command_list) -> static bool {
    // TODO: implementar d3d12_command_list_update_compute_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_raygen_state(list: *mut struct d3d12_command_list) -> static bool {
    // TODO: implementar d3d12_command_list_update_raygen_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_dynamic_state(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_update_dynamic_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_promote_dsv_layout(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_promote_dsv_layout desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_fixup_null_xfb_buffers(list: *mut struct d3d12_command_list, count: u32) -> static bool {
    // TODO: implementar d3d12_command_list_fixup_null_xfb_buffers desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_render_pass_suspend_resume_avoids_fixup(first: *mut struct d3d12_command_list, second: *mut struct d3d12_command_list, hazard_queries: bool, hoistable_post_indirect: bool) -> static bool {
    // TODO: implementar d3d12_command_list_render_pass_suspend_resume_avoids_fixup desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_begin_rendering(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_begin_rendering desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_begin_render_pass(list: *mut struct d3d12_command_list, pipeline_type: enum vkd3d_pipeline_type) -> static bool {
    // TODO: implementar d3d12_command_list_begin_render_pass desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_check_index_buffer_strip_cut_value(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_check_index_buffer_strip_cut_value desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_emit_multi_dispatch_indirect_count(list: *mut struct d3d12_command_list, indirect_args: VkDeviceAddress, stride: u32, max_commands: u32, count_arg: VkDeviceAddress, scratch: *mut struct vkd3d_scratch_allocation) -> static bool {
    // TODO: implementar d3d12_command_list_emit_multi_dispatch_indirect_count desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_emit_predicated_command(list: *mut struct d3d12_command_list, command_type: enum vkd3d_predicate_command_type, indirect_args: VkDeviceAddress, direct_args: *mut const union vkd3d_predicate_command_direct_args, scratch: *mut struct vkd3d_scratch_allocation) -> static bool {
    // TODO: implementar d3d12_command_list_emit_predicated_command desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_index_buffer(list: *mut struct d3d12_command_list) -> static bool {
    // TODO: implementar d3d12_command_list_update_index_buffer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_Dispatch(iface: *mut d3d12_command_list_iface, x: UINT, y: UINT, z: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_Dispatch desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyBufferRegion(iface: *mut d3d12_command_list_iface, dst: *mut ID3D12Resource, dst_offset: UINT64, src: *mut ID3D12Resource, src_offset: UINT64, byte_count: UINT64) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_CopyBufferRegion desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_transition_image_layout_with_global_memory_barrier(list: *mut struct d3d12_command_list, batch: *mut struct d3d12_command_list_barrier_batch, vk_image: VkImage, vk_subresource: *mut const VkImageSubresourceLayers, src_stages: VkPipelineStageFlags2, src_access: VkAccessFlags2, old_layout: VkImageLayout, dst_stages: VkPipelineStageFlags2, dst_access: VkAccessFlags2, new_layout: VkImageLayout, global_src_access: VkAccessFlags2, global_dst_access: VkAccessFlags2) -> static void {
    // TODO: implementar d3d12_command_list_transition_image_layout_with_global_memory_barrier desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_check_ds_color_copy_compatibility(list: *mut struct d3d12_command_list, dst_format: *mut const struct vkd3d_format, src_format: *mut const struct vkd3d_format) -> static bool {
    // TODO: implementar d3d12_command_list_check_ds_color_copy_compatibility desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_copy_image_transition_images(list: *mut struct d3d12_command_list, batch: *mut struct d3d12_command_list_barrier_batch, dst_resource: *mut struct d3d12_resource, dst_format: *mut const struct vkd3d_format, src_resource: *mut struct d3d12_resource, src_format: *mut const struct vkd3d_format, region: *mut const VkImageCopy2, writes_full_subresource: bool, overlapping_subresource: bool, vk_availability_stages: VkPipelineStageFlags2) -> static void {
    // TODO: implementar d3d12_command_list_copy_image_transition_images desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_copy_image(list: *mut struct d3d12_command_list, batch: *mut struct d3d12_command_list_barrier_batch, dst_resource: *mut struct d3d12_resource, dst_format: *mut const struct vkd3d_format, src_resource: *mut struct d3d12_resource, src_format: *mut const struct vkd3d_format, region: *mut const VkImageCopy2, overlapping_subresource: bool, outside_vk_stages: VkPipelineStageFlags2, outside_vk_access: VkAccessFlags2) -> static void {
    // TODO: implementar d3d12_command_list_copy_image desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn validate_d3d12_box(box: *mut const D3D12_BOX) -> static bool {
    // TODO: implementar validate_d3d12_box desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_init_copy_texture_region(list: *mut struct d3d12_command_list, dst: *mut const D3D12_TEXTURE_COPY_LOCATION, dst_x: UINT, dst_y: UINT, dst_z: UINT, src: *mut const D3D12_TEXTURE_COPY_LOCATION, src_box: *mut const D3D12_BOX, out: *mut struct vkd3d_image_copy_info) -> static bool {
    // TODO: implementar d3d12_command_list_init_copy_texture_region desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_merge_copy_tracking(list: *mut struct d3d12_command_list, batch: *mut struct d3d12_command_list_barrier_batch) -> static void {
    // TODO: implementar d3d12_command_list_merge_copy_tracking desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_copy_requires_complex_barrier(list: *mut struct d3d12_command_list, info: *mut struct vkd3d_image_copy_info) -> static bool {
    // TODO: implementar d3d12_command_list_copy_requires_complex_barrier desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_before_copy_texture_region(list: *mut struct d3d12_command_list, batch: *mut struct d3d12_command_list_barrier_batch, info: *mut struct vkd3d_image_copy_info) -> static void {
    // TODO: implementar d3d12_command_list_before_copy_texture_region desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_copy_texture_region(list: *mut struct d3d12_command_list, batch: *mut struct d3d12_command_list_barrier_batch, info: *mut struct vkd3d_image_copy_info) -> static void {
    // TODO: implementar d3d12_command_list_copy_texture_region desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyTextureRegion(iface: *mut d3d12_command_list_iface, dst: *mut const D3D12_TEXTURE_COPY_LOCATION, dst_x: UINT, dst_y: UINT, dst_z: UINT, src: *mut const D3D12_TEXTURE_COPY_LOCATION, src_box: *mut const D3D12_BOX) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_CopyTextureRegion desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyResource(iface: *mut d3d12_command_list_iface, dst: *mut ID3D12Resource, src: *mut ID3D12Resource) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_CopyResource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyTiles(iface: *mut d3d12_command_list_iface, tiled_resource: *mut ID3D12Resource, region_coord: *mut const D3D12_TILED_RESOURCE_COORDINATE, region_size: *mut const D3D12_TILE_REGION_SIZE, buffer: *mut ID3D12Resource, buffer_offset: UINT64, flags: D3D12_TILE_COPY_FLAGS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_CopyTiles desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_view_format_is_compatible(resource: *mut const struct d3d12_resource, format: *mut const struct vkd3d_format) -> static bool {
    // TODO: implementar d3d12_resource_view_format_is_compatible desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_get_resolve_barrier_for_dst_resource(resource: *mut struct d3d12_resource, region: *mut const VkImageResolve2, path: enum vkd3d_resolve_image_path, post_resolve: bool, outside_layout: VkImageLayout, outside_stages: VkPipelineStageFlags2, outside_access: VkAccessFlags2, barrier: *mut VkImageMemoryBarrier2) -> static void {
    // TODO: implementar d3d12_get_resolve_barrier_for_dst_resource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_get_resolve_barrier_for_src_resource(resource: *mut struct d3d12_resource, region: *mut const VkImageResolve2, path: enum vkd3d_resolve_image_path, post_resolve: bool, outside_layout: VkImageLayout, outside_stages: VkPipelineStageFlags2, outside_access: VkAccessFlags2, barrier: *mut VkImageMemoryBarrier2) -> static void {
    // TODO: implementar d3d12_get_resolve_barrier_for_src_resource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_execute_resolve(list: *mut struct d3d12_command_list, dst_resource: *mut struct d3d12_resource, src_resource: *mut struct d3d12_resource, region_count: u32, regions: *mut const VkImageResolve2, format: DXGI_FORMAT, mode: D3D12_RESOLVE_MODE, path: enum vkd3d_resolve_image_path) -> static void {
    // TODO: implementar d3d12_command_list_execute_resolve desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_resolve_subresource(list: *mut struct d3d12_command_list, dst_resource: *mut struct d3d12_resource, dst_subresource_idx: u32, src_resource: *mut struct d3d12_resource, resolve: *mut const VkImageResolve2, format: DXGI_FORMAT, mode: D3D12_RESOLVE_MODE) -> static void {
    // TODO: implementar d3d12_command_list_resolve_subresource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ResolveSubresource(iface: *mut d3d12_command_list_iface, dst: *mut ID3D12Resource, dst_sub_resource_idx: UINT, src: *mut ID3D12Resource, src_sub_resource_idx: UINT, format: DXGI_FORMAT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_ResolveSubresource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetViewports(iface: *mut d3d12_command_list_iface, viewport_count: UINT, viewports: *mut const D3D12_VIEWPORT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_RSSetViewports desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetScissorRects(iface: *mut d3d12_command_list_iface, rect_count: UINT, rects: *mut const D3D12_RECT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_RSSetScissorRects desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetBlendFactor(iface: *mut d3d12_command_list_iface) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_OMSetBlendFactor desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetStencilRef(iface: *mut d3d12_command_list_iface, stencil_ref: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_OMSetStencilRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPipelineState(iface: *mut d3d12_command_list_iface, pipeline_state: *mut ID3D12PipelineState) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetPipelineState desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_merge_copy_tracking_transition(list: *mut struct d3d12_command_list, transition: *mut const D3D12_RESOURCE_TRANSITION_BARRIER, batch: *mut struct d3d12_command_list_barrier_batch) -> static void {
    // TODO: implementar d3d12_command_list_merge_copy_tracking_transition desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ResourceBarrier(iface: *mut d3d12_command_list_iface, barrier_count: UINT, barriers: *mut const D3D12_RESOURCE_BARRIER) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_ResourceBarrier desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ExecuteBundle(iface: *mut d3d12_command_list_iface, command_list: *mut ID3D12GraphicsCommandList) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_ExecuteBundle desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_descriptor_heaps_buffers(list: *mut struct d3d12_command_list, heap_count: u32, heaps: *mut core::ffi::c_void) -> static void {
    // TODO: implementar d3d12_command_list_set_descriptor_heaps_buffers desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_descriptor_heaps_sets(list: *mut struct d3d12_command_list, heap_count: u32, heaps: *mut core::ffi::c_void) -> static void {
    // TODO: implementar d3d12_command_list_set_descriptor_heaps_sets desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetDescriptorHeaps(iface: *mut d3d12_command_list_iface, heap_count: UINT, heaps: *mut core::ffi::c_void) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetDescriptorHeaps desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootSignature(iface: *mut d3d12_command_list_iface, root_signature: *mut ID3D12RootSignature) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetComputeRootSignature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootSignature(iface: *mut d3d12_command_list_iface, root_signature: *mut ID3D12RootSignature) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetGraphicsRootSignature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_descriptor_table_embedded(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings, index: u32, base_descriptor: D3D12_GPU_DESCRIPTOR_HANDLE, cbv_srv_uav_size_log2: u32, sampler_size_log2: u32) -> static inline void {
    // TODO: implementar d3d12_command_list_set_descriptor_table_embedded desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_descriptor_table(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings, index: u32, base_descriptor: D3D12_GPU_DESCRIPTOR_HANDLE) -> static inline void {
    // TODO: implementar d3d12_command_list_set_descriptor_table desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootDescriptorTable_embedded_64_16(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, base_descriptor: D3D12_GPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetComputeRootDescriptorTable_embedded_64_16 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootDescriptorTable_embedded_64_16(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, base_descriptor: D3D12_GPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetGraphicsRootDescriptorTable_embedded_64_16 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootDescriptorTable_embedded_32_16(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, base_descriptor: D3D12_GPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetComputeRootDescriptorTable_embedded_32_16 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootDescriptorTable_embedded_32_16(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, base_descriptor: D3D12_GPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetGraphicsRootDescriptorTable_embedded_32_16 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootDescriptorTable_embedded_default(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, base_descriptor: D3D12_GPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetComputeRootDescriptorTable_embedded_default desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootDescriptorTable_embedded_default(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, base_descriptor: D3D12_GPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetGraphicsRootDescriptorTable_embedded_default desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootDescriptorTable_default(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, base_descriptor: D3D12_GPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetComputeRootDescriptorTable_default desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootDescriptorTable_default(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, base_descriptor: D3D12_GPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetGraphicsRootDescriptorTable_default desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_push_descriptor_info(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings, index: u32, gpu_address: D3D12_GPU_VIRTUAL_ADDRESS) -> static void {
    // TODO: implementar d3d12_command_list_set_push_descriptor_info desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_root_descriptor_va(list: *mut struct d3d12_command_list, descriptor: *mut struct vkd3d_root_descriptor_info, gpu_address: D3D12_GPU_VIRTUAL_ADDRESS) -> static void {
    // TODO: implementar d3d12_command_list_set_root_descriptor_va desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_root_descriptor(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings, index: u32, gpu_address: D3D12_GPU_VIRTUAL_ADDRESS) -> static void {
    // TODO: implementar d3d12_command_list_set_root_descriptor desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootShaderResourceView(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, address: D3D12_GPU_VIRTUAL_ADDRESS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetComputeRootShaderResourceView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootShaderResourceView(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, address: D3D12_GPU_VIRTUAL_ADDRESS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetGraphicsRootShaderResourceView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootUnorderedAccessView(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, address: D3D12_GPU_VIRTUAL_ADDRESS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetComputeRootUnorderedAccessView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootUnorderedAccessView(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, address: D3D12_GPU_VIRTUAL_ADDRESS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetGraphicsRootUnorderedAccessView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetIndexBuffer(iface: *mut d3d12_command_list_iface, view: *mut const D3D12_INDEX_BUFFER_VIEW) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_IASetIndexBuffer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetVertexBuffers(iface: *mut d3d12_command_list_iface, start_slot: UINT, view_count: UINT, views: *mut const D3D12_VERTEX_BUFFER_VIEW) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_IASetVertexBuffers desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SOSetTargets(iface: *mut d3d12_command_list_iface, start_slot: UINT, view_count: UINT, views: *mut const D3D12_STREAM_OUTPUT_BUFFER_VIEW) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SOSetTargets desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_recompute_fb_size(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_recompute_fb_size desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_invalidate_ds_state(list: *mut struct d3d12_command_list, prev_dsv_format: VkFormat) -> static void {
    // TODO: implementar d3d12_command_list_invalidate_ds_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_filter_set_render_targets(list: *mut struct d3d12_command_list, render_target_descriptor_count: UINT, render_target_descriptors: *mut const D3D12_CPU_DESCRIPTOR_HANDLE, single_descriptor_handle: i32, depth_stencil_descriptor: *mut const D3D12_CPU_DESCRIPTOR_HANDLE) -> static bool {
    // TODO: implementar d3d12_command_list_filter_set_render_targets desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetRenderTargets(iface: *mut d3d12_command_list_iface, render_target_descriptor_count: UINT, render_target_descriptors: *mut const D3D12_CPU_DESCRIPTOR_HANDLE, single_descriptor_handle: i32, depth_stencil_descriptor: *mut const D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_OMSetRenderTargets desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_rect_fully_covers_region(a: *mut const D3D12_RECT, b: *mut const D3D12_RECT) -> static bool {
    // TODO: implementar d3d12_rect_fully_covers_region desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_defer_attachment_clear(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, view: *mut struct vkd3d_view, clear_aspects: VkImageAspectFlags, clear_value: *mut const VkClearValue) -> static void {
    // TODO: implementar d3d12_command_list_defer_attachment_clear desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_clear_attachment(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, view: *mut struct vkd3d_view, clear_aspects: VkImageAspectFlags, clear_value: *mut const VkClearValue, rect_count: UINT, rects: *mut const D3D12_RECT) -> static void {
    // TODO: implementar d3d12_command_list_clear_attachment desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ClearDepthStencilView(iface: *mut d3d12_command_list_iface, dsv: D3D12_CPU_DESCRIPTOR_HANDLE, flags: D3D12_CLEAR_FLAGS, depth: f32, stencil: UINT8, rect_count: UINT, rects: *mut const D3D12_RECT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_ClearDepthStencilView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ClearRenderTargetView(iface: *mut d3d12_command_list_iface, rtv: D3D12_CPU_DESCRIPTOR_HANDLE, rect_count: UINT, rects: *mut const D3D12_RECT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_ClearRenderTargetView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_clear_uav(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, args: *mut const struct vkd3d_clear_uav_info, clear_color: *mut const VkClearColorValue, rect_count: UINT, rects: *mut const D3D12_RECT) -> static void {
    // TODO: implementar d3d12_command_list_clear_uav desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_clear_uav_with_copy(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, args: *mut const struct vkd3d_clear_uav_info, clear_value: *mut const VkClearColorValue, format: *mut const struct vkd3d_format, rect_count: UINT, rects: *mut const D3D12_RECT) -> static void {
    // TODO: implementar d3d12_command_list_clear_uav_with_copy desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ClearUnorderedAccessViewUint(iface: *mut d3d12_command_list_iface, gpu_handle: D3D12_GPU_DESCRIPTOR_HANDLE, cpu_handle: D3D12_CPU_DESCRIPTOR_HANDLE, resource: *mut ID3D12Resource, rect_count: UINT, rects: *mut const D3D12_RECT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_ClearUnorderedAccessViewUint desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ClearUnorderedAccessViewFloat(iface: *mut d3d12_command_list_iface, gpu_handle: D3D12_GPU_DESCRIPTOR_HANDLE, cpu_handle: D3D12_CPU_DESCRIPTOR_HANDLE, resource: *mut ID3D12Resource, rect_count: UINT, rects: *mut const D3D12_RECT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_ClearUnorderedAccessViewFloat desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_is_subresource_bound_as_rtv_dsv(list: *mut struct d3d12_command_list, resource: *mut struct d3d12_resource, subresource: *mut const VkImageSubresource) -> static bool {
    // TODO: implementar d3d12_command_list_is_subresource_bound_as_rtv_dsv desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DiscardResource(iface: *mut d3d12_command_list_iface, resource: *mut ID3D12Resource, region: *mut const D3D12_DISCARD_REGION) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_DiscardResource desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_resolve_binary_occlusion_queries(list: *mut struct d3d12_command_list, src_va: VkDeviceAddress, dst_va: VkDeviceAddress, count: u32) -> static void {
    // TODO: implementar d3d12_command_list_resolve_binary_occlusion_queries desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_is_query_resolve_pending(list: *mut struct d3d12_command_list, query_heap: *mut struct d3d12_query_heap, query_index: u32) -> static bool {
    // TODO: implementar d3d12_command_list_is_query_resolve_pending desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_add_query_lookup_mask(list: *mut struct d3d12_command_list, query_heap: *mut struct d3d12_query_heap, bucket: u32, query_mask: u64) -> static void {
    // TODO: implementar d3d12_command_list_add_query_lookup_mask desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_add_query_lookup_range(list: *mut struct d3d12_command_list, entry: *mut const struct vkd3d_query_resolve_entry) -> static void {
    // TODO: implementar d3d12_command_list_add_query_lookup_range desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_execute_query_resolve(list: *mut struct d3d12_command_list, entry: *mut const struct vkd3d_query_resolve_entry) -> static void {
    // TODO: implementar d3d12_command_list_execute_query_resolve desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_add_query_resolve(list: *mut struct d3d12_command_list, entry: *mut const struct vkd3d_query_resolve_entry) -> static void {
    // TODO: implementar d3d12_command_list_add_query_resolve desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_type_is_scoped(type: D3D12_QUERY_TYPE) -> static inline bool {
    // TODO: implementar d3d12_query_type_is_scoped desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_BeginQuery(iface: *mut d3d12_command_list_iface, heap: *mut ID3D12QueryHeap, type: D3D12_QUERY_TYPE, index: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_BeginQuery desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_EndQuery(iface: *mut d3d12_command_list_iface, heap: *mut ID3D12QueryHeap, type: D3D12_QUERY_TYPE, index: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_EndQuery desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ResolveQueryData(iface: *mut d3d12_command_list_iface, heap: *mut ID3D12QueryHeap, type: D3D12_QUERY_TYPE, start_index: UINT, query_count: UINT, dst_buffer: *mut ID3D12Resource, aligned_dst_buffer_offset: UINT64) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_ResolveQueryData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPredication(iface: *mut d3d12_command_list_iface, buffer: *mut ID3D12Resource, aligned_buffer_offset: UINT64, operation: D3D12_PREDICATION_OP) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetPredication desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetMarker(iface: *mut d3d12_command_list_iface, metadata: UINT, data: *mut const void, size: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetMarker desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_BeginEvent(iface: *mut d3d12_command_list_iface, metadata: UINT, data: *mut const void, size: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_BeginEvent desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_EndEvent(iface: *mut d3d12_command_list_iface) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_EndEvent desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_clear_signature_state(list: *mut struct d3d12_command_list, signature: *mut struct d3d12_command_signature) -> static void {
    // TODO: implementar d3d12_command_list_clear_signature_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_execute_indirect_state_template_dgc(list: *mut struct d3d12_command_list, signature: *mut struct d3d12_command_signature, max_command_count: u32, arg_buffer: *mut struct d3d12_resource, arg_buffer_offset: UINT64, count_buffer: *mut struct d3d12_resource, count_buffer_offset: UINT64, dgc_mode: enum vkd3d_dgc_mode, preprocess_va: VkDeviceAddress, preprocess_size: VkDeviceSize) -> static void {
    // TODO: implementar d3d12_command_list_execute_indirect_state_template_dgc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ExecuteIndirect(iface: *mut d3d12_command_list_iface, command_signature: *mut ID3D12CommandSignature, max_command_count: UINT, arg_buffer: *mut ID3D12Resource, arg_buffer_offset: UINT64, count_buffer: *mut ID3D12Resource, count_buffer_offset: UINT64) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_ExecuteIndirect desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetDepthBounds(iface: *mut d3d12_command_list_iface, min: FLOAT, max: FLOAT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_OMSetDepthBounds desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_encode_sampler_feedback(list: *mut struct d3d12_command_list, dst: *mut struct d3d12_resource, dst_subresource_index: UINT, dst_x: UINT, dst_y: UINT, src: *mut struct d3d12_resource, src_subresource_index: UINT, src_rect: *mut const D3D12_RECT) -> static void {
    // TODO: implementar d3d12_command_list_encode_sampler_feedback desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_decode_sampler_feedback(list: *mut struct d3d12_command_list, dst: *mut struct d3d12_resource, dst_subresource_index: UINT, dst_x: UINT, dst_y: UINT, src: *mut struct d3d12_resource, src_subresource_index: UINT, src_rect: *mut const D3D12_RECT) -> static void {
    // TODO: implementar d3d12_command_list_decode_sampler_feedback desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ResolveSubresourceRegion(iface: *mut d3d12_command_list_iface, dst: *mut ID3D12Resource, dst_sub_resource_idx: UINT, dst_x: UINT, dst_y: UINT, src: *mut ID3D12Resource, src_sub_resource_idx: UINT, src_rect: *mut D3D12_RECT, format: DXGI_FORMAT, mode: D3D12_RESOLVE_MODE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_ResolveSubresourceRegion desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetProtectedResourceSession(iface: *mut d3d12_command_list_iface, protected_session: *mut ID3D12ProtectedResourceSession) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetProtectedResourceSession desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_load_render_pass_rtv(list: *mut struct d3d12_command_list, rtv_info: *mut struct d3d12_rtv_desc, rt: *mut const D3D12_RENDER_PASS_RENDER_TARGET_DESC) -> static void {
    // TODO: implementar d3d12_command_list_load_render_pass_rtv desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_load_render_pass_dsv(list: *mut struct d3d12_command_list, dsv_info: *mut struct d3d12_rtv_desc, ds: *mut const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC) -> static void {
    // TODO: implementar d3d12_command_list_load_render_pass_dsv desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_resolve_render_pass_attachments(list: *mut struct d3d12_command_list) -> static void {
    // TODO: implementar d3d12_command_list_resolve_render_pass_attachments desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_deduce_attachment_resolve(list: *mut struct d3d12_command_list, rtv: *mut struct d3d12_rtv_desc, args: *mut const D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS, aspect: VkImageAspectFlagBits, requires_independent_none: bool, requires_independent: bool) -> static bool {
    // TODO: implementar d3d12_command_list_deduce_attachment_resolve desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_render_pass_beginning_access_binds_to_rasterizer(access: D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE, flags: D3D12_RENDER_PASS_FLAGS, aspect: VkImageAspectFlagBits) -> static bool {
    // TODO: implementar d3d12_render_pass_beginning_access_binds_to_rasterizer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_add_render_pass_resolve(list: *mut struct d3d12_command_list, rtv: *mut struct d3d12_rtv_desc, beginning: *mut const D3D12_RENDER_PASS_BEGINNING_ACCESS, args: *mut const D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS, aspect: VkImageAspectFlagBits, flags: D3D12_RENDER_PASS_FLAGS, requires_independent_none: bool, requires_independent: bool) -> static bool {
    // TODO: implementar d3d12_command_list_add_render_pass_resolve desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_setup_render_pass_attachment_resolve(list: *mut struct d3d12_command_list, attachment: *mut VkRenderingAttachmentInfo, resolve: *mut const D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS, aspect_mask: VkImageAspectFlags) -> static void {
    // TODO: implementar d3d12_command_list_setup_render_pass_attachment_resolve desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_BeginRenderPass(iface: *mut d3d12_command_list_iface, rt_count: UINT, render_targets: *mut const D3D12_RENDER_PASS_RENDER_TARGET_DESC, depth_stencil: *mut const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC, flags: D3D12_RENDER_PASS_FLAGS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_BeginRenderPass desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_EndRenderPass(iface: *mut d3d12_command_list_iface) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_EndRenderPass desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_InitializeMetaCommand(iface: *mut d3d12_command_list_iface, meta_command: *mut ID3D12MetaCommand, parameter_data: *mut const void, parameter_size: SIZE_T) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_InitializeMetaCommand desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_ExecuteMetaCommand(iface: *mut d3d12_command_list_iface, meta_command: *mut ID3D12MetaCommand, parameter_data: *mut const void, parameter_size: SIZE_T) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_ExecuteMetaCommand desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_build_raytracing_opacity_micromap_array(list: *mut struct d3d12_command_list, desc: *mut const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, num_postbuild_info_descs: UINT, postbuild_info_descs: *mut const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC) -> static void {
    // TODO: implementar d3d12_command_list_build_raytracing_opacity_micromap_array desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_BuildRaytracingAccelerationStructure(iface: *mut d3d12_command_list_iface, desc: *mut const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, num_postbuild_info_descs: UINT, postbuild_info_descs: *mut const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_BuildRaytracingAccelerationStructure desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_EmitRaytracingAccelerationStructurePostbuildInfo(iface: *mut d3d12_command_list_iface, desc: *mut const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, num_acceleration_structures: UINT, src_data: *mut const D3D12_GPU_VIRTUAL_ADDRESS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_EmitRaytracingAccelerationStructurePostbuildInfo desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_CopyRaytracingAccelerationStructure(iface: *mut d3d12_command_list_iface, dst_data: D3D12_GPU_VIRTUAL_ADDRESS, src_data: D3D12_GPU_VIRTUAL_ADDRESS, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_CopyRaytracingAccelerationStructure desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetPipelineState1(iface: *mut d3d12_command_list_iface, state_object: *mut ID3D12StateObject) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetPipelineState1 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DispatchRays(iface: *mut d3d12_command_list_iface, desc: *mut const D3D12_DISPATCH_RAYS_DESC) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_DispatchRays desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetShadingRate(iface: *mut d3d12_command_list_iface, base: D3D12_SHADING_RATE, combiners: *mut const D3D12_SHADING_RATE_COMBINER) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_RSSetShadingRate desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetShadingRateImage(iface: *mut d3d12_command_list_iface, image: *mut ID3D12Resource) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_RSSetShadingRateImage desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DispatchMesh(iface: *mut d3d12_command_list_iface, x: UINT, y: UINT, z: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_DispatchMesh desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_process_enhanced_barrier_global(list: *mut struct d3d12_command_list, batch: *mut struct d3d12_command_list_barrier_batch, barrier: *mut const D3D12_GLOBAL_BARRIER) -> static void {
    // TODO: implementar d3d12_command_list_process_enhanced_barrier_global desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_process_enhanced_barrier_buffer(list: *mut struct d3d12_command_list, batch: *mut struct d3d12_command_list_barrier_batch, barrier: *mut const D3D12_BUFFER_BARRIER) -> static void {
    // TODO: implementar d3d12_command_list_process_enhanced_barrier_buffer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_process_enhanced_barrier_texture(list: *mut struct d3d12_command_list, batch: *mut struct d3d12_command_list_barrier_batch, barrier: *mut const D3D12_TEXTURE_BARRIER) -> static void {
    // TODO: implementar d3d12_command_list_process_enhanced_barrier_texture desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_barrier_is_noop(list: *mut struct d3d12_command_list, NumBarrierGroups: UINT32, pBarrierGroups: *mut const D3D12_BARRIER_GROUP) -> static bool {
    // TODO: implementar d3d12_command_list_barrier_is_noop desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_Barrier(iface: *mut d3d12_command_list_iface, NumBarrierGroups: UINT32, pBarrierGroups: *mut const D3D12_BARRIER_GROUP) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_Barrier desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_RSSetDepthBias(iface: *mut d3d12_command_list_iface, DepthBias: FLOAT, DepthBiasClamp: FLOAT, SlopeScaledDepthBias: FLOAT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_RSSetDepthBias desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetIndexBufferStripCutValue(iface: *mut d3d12_command_list_iface, IBStripCutValue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_IASetIndexBufferStripCutValue desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetProgram(iface: *mut d3d12_command_list_iface, desc: *mut const D3D12_SET_PROGRAM_DESC) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetProgram desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DispatchGraph(iface: *mut d3d12_command_list_iface, desc: *mut const D3D12_DISPATCH_GRAPH_DESC) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_DispatchGraph desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_init_attachment_info(attachment_info: *mut VkRenderingAttachmentInfo) -> static void {
    // TODO: implementar d3d12_command_list_init_attachment_info desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_init_rendering_info(device: *mut struct d3d12_device, rendering_info: *mut struct vkd3d_rendering_info) -> static void {
    // TODO: implementar d3d12_command_list_init_rendering_info desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_init(list: *mut struct d3d12_command_list, device: *mut struct d3d12_device, type: D3D12_COMMAND_LIST_TYPE) -> static HRESULT {
    // TODO: implementar d3d12_command_list_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_QueryInterface(iface: *mut ID3D12CommandQueue, riid: REFIID, param_64866: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_AddRef(iface: *mut ID3D12CommandQueue) -> ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_Release(iface: *mut ID3D12CommandQueue) -> ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_GetPrivateData(iface: *mut ID3D12CommandQueue, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_SetPrivateData(iface: *mut ID3D12CommandQueue, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_SetPrivateDataInterface(iface: *mut ID3D12CommandQueue, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_GetDevice(iface: *mut ID3D12CommandQueue, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_UpdateTileMappings(iface: *mut ID3D12CommandQueue, resource: *mut ID3D12Resource, region_count: UINT, region_coords: *mut const D3D12_TILED_RESOURCE_COORDINATE, region_sizes: *mut const D3D12_TILE_REGION_SIZE, heap: *mut ID3D12Heap, range_count: UINT, range_flags: *mut const D3D12_TILE_RANGE_FLAGS, heap_range_offsets: *mut const UINT, range_tile_counts: *mut const UINT, flags: D3D12_TILE_MAPPING_FLAGS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_UpdateTileMappings desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_CopyTileMappings(iface: *mut ID3D12CommandQueue, dst_resource: *mut ID3D12Resource, dst_region_start_coordinate: *mut const D3D12_TILED_RESOURCE_COORDINATE, src_resource: *mut ID3D12Resource, src_region_start_coordinate: *mut const D3D12_TILED_RESOURCE_COORDINATE, region_size: *mut const D3D12_TILE_REGION_SIZE, flags: D3D12_TILE_MAPPING_FLAGS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_CopyTileMappings desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_ExecuteCommandLists(iface: *mut ID3D12CommandQueue, command_list_count: UINT, command_lists: *mut core::ffi::c_void) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_ExecuteCommandLists desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_SetMarker(iface: *mut ID3D12CommandQueue, metadata: UINT, data: *mut const void, size: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_SetMarker desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_BeginEvent(iface: *mut ID3D12CommandQueue, metadata: UINT, data: *mut const void, size: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_BeginEvent desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_EndEvent(iface: *mut ID3D12CommandQueue) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_EndEvent desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_Signal(iface: *mut ID3D12CommandQueue, fence_iface: *mut ID3D12Fence, value: UINT64) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_Signal desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_Wait(iface: *mut ID3D12CommandQueue, fence_iface: *mut ID3D12Fence, value: UINT64) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_Wait desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_GetTimestampFrequency(iface: *mut ID3D12CommandQueue, frequency: *mut UINT64) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_GetTimestampFrequency desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_GetDesc(iface: *mut ID3D12CommandQueue, desc: *mut D3D12_COMMAND_QUEUE_DESC) -> static D3D12_COMMAND_QUEUE_DESC * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_queue_GetDesc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_destroy_serializing_semaphore(command_queue: *mut struct d3d12_command_queue) -> static void {
    // TODO: implementar d3d12_command_queue_destroy_serializing_semaphore desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_reset_fence_waits(command_queue: *mut struct d3d12_command_queue) -> static void {
    // TODO: implementar d3d12_command_queue_reset_fence_waits desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_push_fence_waits_to_worker(command_queue: *mut struct d3d12_command_queue) -> static void {
    // TODO: implementar d3d12_command_queue_push_fence_waits_to_worker desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_add_wait_semaphores(command_queue: *mut struct d3d12_command_queue, wait_count: u32, waits: *mut const VkSemaphoreSubmitInfo) -> static void {
    // TODO: implementar d3d12_command_queue_add_wait_semaphores desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_add_wait(command_queue: *mut struct d3d12_command_queue, fence: *mut struct d3d12_fence, fence_value: *mut const struct d3d12_fence_value) -> static void {
    // TODO: implementar d3d12_command_queue_add_wait desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_eliminate_completed_waits(command_queue: *mut struct d3d12_command_queue) -> static void {
    // TODO: implementar d3d12_command_queue_eliminate_completed_waits desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_finalize_waits(command_queue: *mut struct d3d12_command_queue, submit_vr: VkResult) -> static void {
    // TODO: implementar d3d12_command_queue_finalize_waits desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_flush_waiters(command_queue: *mut struct d3d12_command_queue, wait_flags: u32) -> static void {
    // TODO: implementar d3d12_command_queue_flush_waiters desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_wait_idle(command_queue: *mut struct d3d12_command_queue) -> static void {
    // TODO: implementar d3d12_command_queue_wait_idle desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_wait(command_queue: *mut struct d3d12_command_queue, fence: *mut struct d3d12_fence, value: UINT64, ticket: UINT64) -> static void {
    // TODO: implementar d3d12_command_queue_wait desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_signal(command_queue: *mut struct d3d12_command_queue, fence: *mut struct d3d12_fence, value: UINT64) -> static void {
    // TODO: implementar d3d12_command_queue_signal desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_wait_shared(command_queue: *mut struct d3d12_command_queue, fence: *mut struct d3d12_shared_fence, value: u64) -> static void {
    // TODO: implementar d3d12_command_queue_wait_shared desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_signal_shared(command_queue: *mut struct d3d12_command_queue, fence: *mut struct d3d12_shared_fence, value: UINT64) -> static void {
    // TODO: implementar d3d12_command_queue_signal_shared desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_transition_pool_init(pool: *mut struct d3d12_command_queue_transition_pool, queue: *mut struct d3d12_command_queue) -> static HRESULT {
    // TODO: implementar d3d12_command_queue_transition_pool_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_transition_pool_wait(pool: *mut struct d3d12_command_queue_transition_pool, device: *mut struct d3d12_device, value: u64) -> static void {
    // TODO: implementar d3d12_command_queue_transition_pool_wait desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_transition_pool_deinit(pool: *mut struct d3d12_command_queue_transition_pool, device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_command_queue_transition_pool_deinit desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_init_query_heap(device: *mut struct d3d12_device, vk_cmd_buffer: VkCommandBuffer, heap: *mut const struct d3d12_query_heap) -> static void {
    // TODO: implementar d3d12_command_queue_init_query_heap desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_transition_pool_build(pool: *mut struct d3d12_command_queue_transition_pool, device: *mut struct d3d12_device, transitions: *mut const struct vkd3d_initial_transition, count: usize, fallback: bool, vk_cmd_buffer: *mut VkCommandBuffer, timeline_value: *mut u64) -> static void {
    // TODO: implementar d3d12_command_queue_transition_pool_build desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_wait_staggered_submission(command_queue: *mut struct d3d12_command_queue) -> static void {
    // TODO: implementar d3d12_command_queue_wait_staggered_submission desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_execute(command_queue: *mut struct d3d12_command_queue, exec: *mut const struct d3d12_command_queue_submission_execute, transition_cmd: *mut VkCommandBufferSubmitInfo, transition_semaphore: *mut const VkSemaphoreSubmitInfo) -> static void {
    // TODO: implementar d3d12_command_queue_execute desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_flush_bind_sparse(command_queue: *mut struct d3d12_command_queue) -> static void {
    // TODO: implementar d3d12_command_queue_flush_bind_sparse desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_register_sparse_hazard(command_queue: *mut struct d3d12_command_queue, dst_resource: *mut struct d3d12_resource, binds: *mut const struct vkd3d_sparse_memory_bind, count: u32) -> static void {
    // TODO: implementar d3d12_command_queue_register_sparse_hazard desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_bind_sparse(command_queue: *mut struct d3d12_command_queue, mode: enum vkd3d_sparse_memory_bind_mode, dst_resource: *mut struct d3d12_resource, src_resource: *mut struct d3d12_resource, count: u32, bind_infos: *mut struct vkd3d_sparse_memory_bind) -> static void {
    // TODO: implementar d3d12_command_queue_bind_sparse desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_acquire_serialized(queue: *mut struct d3d12_command_queue) -> static void {
    // TODO: implementar d3d12_command_queue_acquire_serialized desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_release_serialized(queue: *mut struct d3d12_command_queue) -> static void {
    // TODO: implementar d3d12_command_queue_release_serialized desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_exec_submit_needs_fallback_queue(queue: *mut struct d3d12_command_queue, execute: *mut struct d3d12_command_queue_submission_execute) -> static bool {
    // TODO: implementar d3d12_command_queue_exec_submit_needs_fallback_queue desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_queue_init(queue: *mut struct d3d12_command_queue, device: *mut struct d3d12_device, desc: *mut const D3D12_COMMAND_QUEUE_DESC, family_info: *mut struct vkd3d_queue_family_info) -> static HRESULT {
    // TODO: implementar d3d12_command_queue_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_QueryInterface(iface: *mut ID3D12CommandSignature, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_signature_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_AddRef(iface: *mut ID3D12CommandSignature) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_signature_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_cleanup(signature: *mut struct d3d12_command_signature) -> static void {
    // TODO: implementar d3d12_command_signature_cleanup desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_Release(iface: *mut ID3D12CommandSignature) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_signature_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_GetPrivateData(iface: *mut ID3D12CommandSignature, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_signature_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_SetPrivateData(iface: *mut ID3D12CommandSignature, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_signature_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_SetPrivateDataInterface(iface: *mut ID3D12CommandSignature, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_signature_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_GetDevice(iface: *mut ID3D12CommandSignature, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_signature_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_init_patch_commands_buffer(signature: *mut struct d3d12_command_signature, device: *mut struct d3d12_device, commands: *mut const struct vkd3d_patch_command, command_count: usize) -> static HRESULT {
    // TODO: implementar d3d12_command_signature_init_patch_commands_buffer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_init_indirect_commands_layout(signature: *mut struct d3d12_command_signature, root_signature: *mut struct d3d12_root_signature, device: *mut struct d3d12_device, tokens: *mut const VkIndirectCommandsLayoutTokenEXT, token_count: u32, stream_stride: u32) -> static HRESULT {
    // TODO: implementar d3d12_command_signature_init_indirect_commands_layout desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_signature_init_state_template_dgc(signature: *mut struct d3d12_command_signature, desc: *mut const D3D12_COMMAND_SIGNATURE_DESC, root_signature: *mut struct d3d12_root_signature, device: *mut struct d3d12_device) -> static HRESULT {
    // TODO: implementar d3d12_command_signature_init_state_template_dgc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_determine_additional_shading_rates_supported(device: *mut struct d3d12_device) -> static bool {
    // TODO: implementar d3d12_device_determine_additional_shading_rates_supported desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_init_vendor_hacks(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_init_vendor_hacks desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_cleanup_vendor_hacks(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_cleanup_vendor_hacks desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_init_workarounds(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_init_workarounds desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_destroy_scratch_buffer(device: *mut struct d3d12_device, scratch: *mut const struct vkd3d_scratch_buffer) -> static void {
    // TODO: implementar d3d12_device_destroy_scratch_buffer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_destroy_query_pool(device: *mut struct d3d12_device, pool: *mut const struct vkd3d_query_pool) -> static void {
    // TODO: implementar d3d12_device_destroy_query_pool desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_AddRef(iface: *mut ID3DLowLatencyDevice) -> extern ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_low_latency_device_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_amd_ext_anti_lag_AddRef(iface: *mut IAmdExtAntiLagApi) -> extern ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_amd_ext_anti_lag_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_QueryInterface(iface: *mut d3d12_device_iface, riid: REFIID, param_64866: *mut core::ffi::c_void) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_destroy(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_destroy desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_AddRef(iface: *mut d3d12_device_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_Release(iface: *mut d3d12_device_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetPrivateData(iface: *mut d3d12_device_iface, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_SetPrivateData(iface: *mut d3d12_device_iface, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_SetPrivateDataInterface(iface: *mut d3d12_device_iface, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetNodeCount(iface: *mut d3d12_device_iface) -> static UINT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_GetNodeCount desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_check_multisample_quality_levels(device: *mut struct d3d12_device, data: *mut D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS) -> static HRESULT {
    // TODO: implementar d3d12_device_check_multisample_quality_levels desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_format_displayable_features(device: *mut struct d3d12_device, format: DXGI_FORMAT) -> static UINT {
    // TODO: implementar d3d12_device_get_format_displayable_features desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_format_is_streamout_compatible(format: DXGI_FORMAT) -> static bool {
    // TODO: implementar d3d12_format_is_streamout_compatible desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_format_support(device: *mut struct d3d12_device, data: *mut D3D12_FEATURE_DATA_FORMAT_SUPPORT) -> static HRESULT {
    // TODO: implementar d3d12_device_get_format_support desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CheckFeatureSupport(iface: *mut d3d12_device_iface, feature: D3D12_FEATURE, feature_data: *mut core::ffi::c_void, feature_data_size: UINT) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CheckFeatureSupport desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetDescriptorHandleIncrementSize(iface: *mut d3d12_device_iface, descriptor_heap_type: D3D12_DESCRIPTOR_HEAP_TYPE) -> static UINT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_GetDescriptorHandleIncrementSize desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_descriptor_handle_increment_size(param_47516: device, param_4686: descriptor_heap_type) -> return {
    // TODO: implementar d3d12_device_get_descriptor_handle_increment_size desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_advance_cpu_descriptor_handle(handle: D3D12_CPU_DESCRIPTOR_HANDLE, increment: u32, units: u32) -> static inline D3D12_CPU_DESCRIPTOR_HANDLE {
    // TODO: implementar d3d12_advance_cpu_descriptor_handle desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_copy_descriptors_cbv_srv_uav_sampler(device: *mut struct d3d12_device, dst: D3D12_CPU_DESCRIPTOR_HANDLE, src: D3D12_CPU_DESCRIPTOR_HANDLE, heap_type: D3D12_DESCRIPTOR_HEAP_TYPE, descriptor_count: UINT) -> static inline void {
    // TODO: implementar d3d12_device_copy_descriptors_cbv_srv_uav_sampler desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_copy_descriptors(device: *mut struct d3d12_device, dst_descriptor_range_count: UINT, dst_descriptor_range_offsets: *mut const D3D12_CPU_DESCRIPTOR_HANDLE, dst_descriptor_range_sizes: *mut const UINT, src_descriptor_range_count: UINT, src_descriptor_range_offsets: *mut const D3D12_CPU_DESCRIPTOR_HANDLE, src_descriptor_range_sizes: *mut const UINT, descriptor_heap_type: D3D12_DESCRIPTOR_HEAP_TYPE) -> static inline void {
    // TODO: implementar d3d12_device_copy_descriptors desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptorsSimple_descriptor_buffer_16_16_4(iface: *mut d3d12_device_iface, descriptor_count: UINT, dst_descriptor_range_offset: const D3D12_CPU_DESCRIPTOR_HANDLE, src_descriptor_range_offset: const D3D12_CPU_DESCRIPTOR_HANDLE, descriptor_heap_type: D3D12_DESCRIPTOR_HEAP_TYPE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CopyDescriptorsSimple_descriptor_buffer_16_16_4 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptorsSimple_descriptor_buffer_64_64_32(iface: *mut d3d12_device_iface, descriptor_count: UINT, dst_descriptor_range_offset: const D3D12_CPU_DESCRIPTOR_HANDLE, src_descriptor_range_offset: const D3D12_CPU_DESCRIPTOR_HANDLE, descriptor_heap_type: D3D12_DESCRIPTOR_HEAP_TYPE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CopyDescriptorsSimple_descriptor_buffer_64_64_32 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptorsSimple_embedded_64_16_packed(iface: *mut d3d12_device_iface, descriptor_count: UINT, dst_descriptor_range_offset: const D3D12_CPU_DESCRIPTOR_HANDLE, src_descriptor_range_offset: const D3D12_CPU_DESCRIPTOR_HANDLE, descriptor_heap_type: D3D12_DESCRIPTOR_HEAP_TYPE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CopyDescriptorsSimple_embedded_64_16_packed desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptorsSimple_embedded_32_16_planar(iface: *mut d3d12_device_iface, descriptor_count: UINT, dst_descriptor_range_offset: const D3D12_CPU_DESCRIPTOR_HANDLE, src_descriptor_range_offset: const D3D12_CPU_DESCRIPTOR_HANDLE, descriptor_heap_type: D3D12_DESCRIPTOR_HEAP_TYPE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CopyDescriptorsSimple_embedded_32_16_planar desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptorsSimple_embedded_generic(iface: *mut d3d12_device_iface, descriptor_count: UINT, dst_descriptor_range_offset: const D3D12_CPU_DESCRIPTOR_HANDLE, src_descriptor_range_offset: const D3D12_CPU_DESCRIPTOR_HANDLE, descriptor_heap_type: D3D12_DESCRIPTOR_HEAP_TYPE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CopyDescriptorsSimple_embedded_generic desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CopyDescriptorsSimple_default(iface: *mut d3d12_device_iface, descriptor_count: UINT, dst_descriptor_range_offset: const D3D12_CPU_DESCRIPTOR_HANDLE, src_descriptor_range_offset: const D3D12_CPU_DESCRIPTOR_HANDLE, descriptor_heap_type: D3D12_DESCRIPTOR_HEAP_TYPE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CopyDescriptorsSimple_default desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetCustomHeapProperties(iface: *mut d3d12_device_iface, heap_properties: *mut D3D12_HEAP_PROPERTIES, node_mask: UINT, heap_type: D3D12_HEAP_TYPE) -> static D3D12_HEAP_PROPERTIES * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_GetCustomHeapProperties desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_OpenSharedHandle(iface: *mut d3d12_device_iface, handle: *mut core::ffi::c_void, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_OpenSharedHandle desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_OpenSharedHandleByName(iface: *mut d3d12_device_iface, name: *mut const WCHAR, access: u32, handle: *mut *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_OpenSharedHandleByName desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_MakeResident(iface: *mut d3d12_device_iface, object_count: UINT, objects: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_MakeResident desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_Evict(iface: *mut d3d12_device_iface, object_count: UINT, objects: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_Evict desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetDeviceRemovedReason(iface: *mut d3d12_device_iface) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_GetDeviceRemovedReason desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetCopyableFootprints1(iface: *mut d3d12_device_iface, desc: *mut const D3D12_RESOURCE_DESC1, first_sub_resource: UINT, sub_resource_count: UINT, base_offset: UINT64, layouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, row_counts: *mut UINT, row_sizes: *mut UINT64, total_bytes: *mut UINT64) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_GetCopyableFootprints1 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetCopyableFootprints(iface: *mut d3d12_device_iface, desc: *mut const D3D12_RESOURCE_DESC, first_sub_resource: UINT, sub_resource_count: UINT, base_offset: UINT64, layouts: *mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT, row_counts: *mut UINT, row_sizes: *mut UINT64, total_bytes: *mut UINT64) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_GetCopyableFootprints desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetResourceTiling(iface: *mut d3d12_device_iface, resource: *mut ID3D12Resource, tile_count: *mut UINT, packed_mip_info: *mut D3D12_PACKED_MIP_INFO, tile_shape: *mut D3D12_TILE_SHAPE, tiling_count: *mut UINT, first_tiling: UINT, tilings: *mut D3D12_SUBRESOURCE_TILING) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_GetResourceTiling desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetAdapterLuid(iface: *mut d3d12_device_iface, luid: *mut LUID) -> static LUID * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_GetAdapterLuid desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_SetEventOnMultipleFenceCompletion(iface: *mut d3d12_device_iface, fences: *mut core::ffi::c_void, values: *mut const UINT64, fence_count: UINT, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, event: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_SetEventOnMultipleFenceCompletion desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_SetResidencyPriority(iface: *mut d3d12_device_iface, object_count: UINT, objects: *mut core::ffi::c_void, priorities: *mut const D3D12_RESIDENCY_PRIORITY) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_SetResidencyPriority desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_OpenExistingHeapFromAddress(iface: *mut d3d12_device_iface, address: *mut core::ffi::c_void, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_OpenExistingHeapFromAddress desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_OpenExistingHeapFromFileMapping(iface: *mut d3d12_device_iface, file_mapping: *mut core::ffi::c_void, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_OpenExistingHeapFromFileMapping desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_RemoveDevice(iface: *mut d3d12_device_iface) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_RemoveDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_EnumerateMetaCommands(iface: *mut d3d12_device_iface, count: *mut UINT, descs: *mut D3D12_META_COMMAND_DESC) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_EnumerateMetaCommands desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_EnumerateMetaCommandParameters(iface: *mut d3d12_device_iface, command_id: REFGUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, total_size: *mut UINT, param_count: *mut UINT, param_descs: *mut D3D12_META_COMMAND_PARAMETER_DESC) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_EnumerateMetaCommandParameters desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_get_raytracing_opacity_micromap_array_prebuild_info(device: *mut struct d3d12_device, desc: *mut const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, info: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO) -> static void {
    // TODO: implementar d3d12_device_get_raytracing_opacity_micromap_array_prebuild_info desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_GetRaytracingAccelerationStructurePrebuildInfo(iface: *mut d3d12_device_iface, desc: *mut const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, info: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_GetRaytracingAccelerationStructurePrebuildInfo desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_AddToStateObject(iface: *mut d3d12_device_iface, addition: *mut const D3D12_STATE_OBJECT_DESC, parent_state: *mut ID3D12StateObject, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_AddToStateObject desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_ShaderCacheControl(iface: *mut d3d12_device_iface, kinds: D3D12_SHADER_CACHE_KIND_FLAGS, control: D3D12_SHADER_CACHE_CONTROL_FLAGS) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_ShaderCacheControl desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_rtas_formats(device: *mut struct d3d12_device, format: *mut const VkFormat, count: usize) -> static bool {
    // TODO: implementar d3d12_device_supports_rtas_formats desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_determine_ray_tracing_tier(device: *mut struct d3d12_device) -> static D3D12_RAYTRACING_TIER {
    // TODO: implementar d3d12_device_determine_ray_tracing_tier desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_determine_heap_tier(device: *mut struct d3d12_device) -> static D3D12_RESOURCE_HEAP_TIER {
    // TODO: implementar d3d12_device_determine_heap_tier desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_determine_additional_typed_uav_support(device: *mut struct d3d12_device) -> static bool {
    // TODO: implementar d3d12_device_determine_additional_typed_uav_support desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options1(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options1 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options2(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options2 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options3(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options3 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options4(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options4 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options5(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options5 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options6(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options6 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options7(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options7 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options8(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options8 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options9(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options9 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options10(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options10 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options11(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options11 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options12(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options12 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options13(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options13 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options14(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options14 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options15(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options15 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options16(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options16 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options17(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options17 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options18(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options18 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options19(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options19 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options20(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options20 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_options21(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_options21 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_tight_alignment(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_tight_alignment desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_feature_level(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_feature_level desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_shader_model_override(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_shader_model_override desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init_shader_model(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init_shader_model desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_override(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_override desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_caps_init(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_caps_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_feature_level(device: *mut struct d3d12_device, feature_level: D3D_FEATURE_LEVEL) -> static bool {
    // TODO: implementar d3d12_device_supports_feature_level desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_replace_vtable(device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_device_replace_vtable desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_init(device: *mut struct d3d12_device, instance: *mut struct vkd3d_instance, create_info: *mut const struct vkd3d_device_create_info) -> static HRESULT {
    // TODO: implementar d3d12_device_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_Release(iface: *mut d3d_low_latency_device_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_low_latency_device_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_QueryInterface(iface: *mut d3d_low_latency_device_iface, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_low_latency_device_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_SupportsLowLatency(iface: *mut d3d_low_latency_device_iface) -> static BOOL STDMETHODCALLTYPE {
    // TODO: implementar d3d12_low_latency_device_SupportsLowLatency desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_LatencySleep(iface: *mut d3d_low_latency_device_iface) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_low_latency_device_LatencySleep desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_SetLatencySleepMode(iface: *mut d3d_low_latency_device_iface, low_latency_mode: i32, low_latency_boost: i32, minimum_interval_us: UINT32) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_low_latency_device_SetLatencySleepMode desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_SetLatencyMarker(iface: *mut d3d_low_latency_device_iface, frameID: UINT64, markerType: UINT32) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_low_latency_device_SetLatencyMarker desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_low_latency_device_GetLatencyInfo(iface: *mut d3d_low_latency_device_iface, latency_results: *mut D3D12_LATENCY_RESULTS) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_low_latency_device_GetLatencyInfo desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_amd_ext_anti_lag_Release(iface: *mut IAmdExtAntiLagApi) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_amd_ext_anti_lag_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_amd_ext_anti_lag_QueryInterface(iface: *mut IAmdExtAntiLagApi, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_amd_ext_anti_lag_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_amd_ext_anti_lag_UpdateAntiLagState(iface: *mut IAmdExtAntiLagApi, pData: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_amd_ext_anti_lag_UpdateAntiLagState desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_QueryInterface(iface: *mut d3d12_heap_iface, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_heap_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_AddRef(iface: *mut d3d12_heap_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_heap_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_destroy(heap: *mut struct d3d12_heap) -> static void {
    // TODO: implementar d3d12_heap_destroy desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_Release(iface: *mut d3d12_heap_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_heap_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_GetPrivateData(iface: *mut d3d12_heap_iface, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_heap_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_SetPrivateData(iface: *mut d3d12_heap_iface, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_heap_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_SetPrivateDataInterface(iface: *mut d3d12_heap_iface, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_heap_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_GetDevice(iface: *mut d3d12_heap_iface, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_heap_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_GetDesc(iface: *mut d3d12_heap_iface, desc: *mut D3D12_HEAP_DESC) -> static D3D12_HEAP_DESC * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_heap_GetDesc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_heap_init(heap: *mut struct d3d12_heap, device: *mut struct d3d12_device, desc: *mut const D3D12_HEAP_DESC, host_address: *mut core::ffi::c_void) -> static HRESULT {
    // TODO: implementar d3d12_heap_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_use_nv_memory_decompression(param_47516: device) -> return {
    // TODO: implementar d3d12_device_use_nv_memory_decompression desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_destroy(meta_command: *mut struct d3d12_meta_command) -> static void {
    // TODO: implementar d3d12_meta_command_destroy desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_QueryInterface(iface: *mut d3d12_meta_command_iface, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_meta_command_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_AddRef(iface: *mut d3d12_meta_command_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_meta_command_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_Release(iface: *mut d3d12_meta_command_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_meta_command_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_GetPrivateData(iface: *mut d3d12_meta_command_iface, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_meta_command_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_SetPrivateData(iface: *mut d3d12_meta_command_iface, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_meta_command_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_SetPrivateDataInterface(iface: *mut d3d12_meta_command_iface, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_meta_command_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_GetDevice(iface: *mut d3d12_meta_command_iface, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_meta_command_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_GetRequiredParameterResourceSize(iface: *mut d3d12_meta_command_iface, stage: D3D12_META_COMMAND_PARAMETER_STAGE, parameter_index: UINT) -> static UINT64 STDMETHODCALLTYPE {
    // TODO: implementar d3d12_meta_command_GetRequiredParameterResourceSize desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_meta_command_exec_dstorage(meta_command: *mut struct d3d12_meta_command, list: *mut struct d3d12_command_list, parameter_data: *mut const void, parameter_size: usize) -> static void {
    // TODO: implementar d3d12_meta_command_exec_dstorage desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3D12_RAYTRACING_OPACITY_MICROMAP_FORMAT(param_60231: %u) -> Unknown {
    // TODO: implementar D3D12_RAYTRACING_OPACITY_MICROMAP_FORMAT desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_QueryInterface(iface: *mut ID3D12StateObject, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_state_object_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_QueryInterface(iface: *mut d3d12_state_object_properties_iface, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_state_object_properties_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_AddRef(iface: *mut ID3D12StateObject) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_state_object_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_AddRef(iface: *mut d3d12_state_object_properties_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_state_object_properties_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_cleanup(object: *mut struct d3d12_rt_state_object) -> static void {
    // TODO: implementar d3d12_state_object_cleanup desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_inc_ref(state_object: *mut struct d3d12_rt_state_object) -> static void {
    // TODO: implementar d3d12_state_object_inc_ref desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_dec_ref(state_object: *mut struct d3d12_rt_state_object) -> static void {
    // TODO: implementar d3d12_state_object_dec_ref desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_cleanup(data: *mut struct d3d12_rt_state_object_pipeline_data, device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_state_object_pipeline_data_cleanup desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_release(state_object: *mut struct d3d12_rt_state_object) -> static ULONG {
    // TODO: implementar d3d12_state_object_release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_Release(iface: *mut ID3D12StateObject) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_state_object_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_Release(iface: *mut d3d12_state_object_properties_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_state_object_properties_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_GetPrivateData(iface: *mut ID3D12StateObject, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_state_object_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_SetPrivateData(iface: *mut ID3D12StateObject, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_state_object_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_SetPrivateDataInterface(iface: *mut ID3D12StateObject, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_state_object_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_GetDevice(iface: *mut ID3D12StateObject, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_state_object_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_GetShaderIdentifier(iface: *mut d3d12_state_object_properties_iface, export_name: LPCWSTR) -> static void * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_state_object_properties_GetShaderIdentifier desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_GetShaderStackSize(iface: *mut d3d12_state_object_properties_iface, export_name: LPCWSTR) -> static UINT64 STDMETHODCALLTYPE {
    // TODO: implementar d3d12_state_object_properties_GetShaderStackSize desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_GetPipelineStackSize(iface: *mut d3d12_state_object_properties_iface) -> static UINT64 STDMETHODCALLTYPE {
    // TODO: implementar d3d12_state_object_properties_GetPipelineStackSize desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_SetPipelineStackSize(iface: *mut d3d12_state_object_properties_iface, stack_size_in_bytes: UINT64) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_state_object_properties_SetPipelineStackSize desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_properties_GetProgramIdentifier(iface: *mut d3d12_state_object_properties_iface, ret: *mut D3D12_PROGRAM_IDENTIFIER, pProgramName: LPCWSTR) -> static D3D12_PROGRAM_IDENTIFIER * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_state_object_properties_GetProgramIdentifier desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_cleanup_modules(data: *mut struct d3d12_rt_state_object_pipeline_data, device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_state_object_pipeline_data_cleanup_modules desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_cleanup_compile_temporaries(data: *mut struct d3d12_rt_state_object_pipeline_data, device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_state_object_pipeline_data_cleanup_compile_temporaries desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_add_collection_library(collection: *mut struct d3d12_rt_state_object, data: *mut struct d3d12_rt_state_object_pipeline_data, exports: *mut const D3D12_EXPORT_DESC, num_exports: u32) -> static HRESULT {
    // TODO: implementar d3d12_state_object_add_collection_library desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_add_collection_deferred(deferred: *mut struct d3d12_rt_state_object_pipeline_data, data: *mut struct d3d12_rt_state_object_pipeline_data, exports: *mut const D3D12_EXPORT_DESC, num_exports: u32) -> static HRESULT {
    // TODO: implementar d3d12_state_object_add_collection_deferred desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_set_association_data(association: *mut struct d3d12_state_object_association, object: *mut const D3D12_STATE_SUBOBJECT) -> static void {
    // TODO: implementar d3d12_state_object_set_association_data desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_parse_subobject(object: *mut struct d3d12_rt_state_object, obj: *mut const D3D12_STATE_SUBOBJECT, data: *mut struct d3d12_rt_state_object_pipeline_data, association_priority: u32) -> static HRESULT {
    // TODO: implementar d3d12_state_object_parse_subobject desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_parse_subobjects(object: *mut struct d3d12_rt_state_object, desc: *mut const D3D12_STATE_OBJECT_DESC, parent: *mut struct d3d12_rt_state_object, data: *mut struct d3d12_rt_state_object_pipeline_data) -> static HRESULT {
    // TODO: implementar d3d12_state_object_parse_subobjects desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_find_entry_inner(entry_points: *mut const struct vkd3d_shader_library_entry_point, count: usize, import: *mut const WCHAR) -> static uint32_t {
    // TODO: implementar d3d12_state_object_pipeline_data_find_entry_inner desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_find_collection_variant(variant: *mut const struct d3d12_rt_state_object_variant, collection: *mut const struct d3d12_rt_state_object) -> static uint32_t {
    // TODO: implementar d3d12_state_object_find_collection_variant desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_find_entry(data: *mut const struct d3d12_rt_state_object_pipeline_data, variant: *mut const struct d3d12_rt_state_object_variant, pipeline_variant_index: u32, import: *mut const WCHAR) -> static uint32_t {
    // TODO: implementar d3d12_state_object_pipeline_data_find_entry desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_compute_default_stack_size(data: *mut const struct d3d12_rt_state_object_pipeline_data, stack_info: *mut struct d3d12_rt_state_object_stack_info, recursion_depth: u32) -> static VkDeviceSize {
    // TODO: implementar d3d12_state_object_pipeline_data_compute_default_stack_size desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_get_group_handles(object: *mut struct d3d12_rt_state_object, data: *mut const struct d3d12_rt_state_object_pipeline_data) -> static HRESULT {
    // TODO: implementar d3d12_state_object_get_group_handles desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_append_local_static_samplers(variant: *mut struct d3d12_rt_state_object_variant, param_51555: *mut VkDescriptorSetLayoutBinding, out_vk_bindings_size: *mut usize, out_vk_bindings_count: *mut usize, local_bindings: *mut struct vkd3d_shader_resource_binding, sampler_desc: *mut const D3D12_STATIC_SAMPLER_DESC1, vk_samplers: *mut const VkSampler, sampler_count: u32) -> static void {
    // TODO: implementar d3d12_state_object_append_local_static_samplers desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_find_global_state_object(data: *mut struct d3d12_rt_state_object_pipeline_data, kind: enum vkd3d_shader_subobject_kind, param_8638: *mut const struct d3d12_state_object_association) -> static bool {
    // TODO: implementar d3d12_state_object_pipeline_data_find_global_state_object desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_pipeline_data_find_global_state_objects(data: *mut struct d3d12_rt_state_object_pipeline_data, out_shader_config: *mut D3D12_RAYTRACING_SHADER_CONFIG, out_pipeline_config: *mut D3D12_RAYTRACING_PIPELINE_CONFIG1) -> static HRESULT {
    // TODO: implementar d3d12_state_object_pipeline_data_find_global_state_objects desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_compile_pipeline_variant(object: *mut struct d3d12_rt_state_object, pipeline_variant_index: unsigned, data: *mut struct d3d12_rt_state_object_pipeline_data) -> static HRESULT {
    // TODO: implementar d3d12_state_object_compile_pipeline_variant desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_add_global_root_signature_variant(object: *mut struct d3d12_rt_state_object, rs: *mut struct d3d12_root_signature) -> static void {
    // TODO: implementar d3d12_state_object_add_global_root_signature_variant desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_collect_variants(object: *mut struct d3d12_rt_state_object, data: *mut const struct d3d12_rt_state_object_pipeline_data) -> static void {
    // TODO: implementar d3d12_state_object_collect_variants desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_init(object: *mut struct d3d12_rt_state_object, device: *mut struct d3d12_device, desc: *mut const D3D12_STATE_OBJECT_DESC, parent: *mut struct d3d12_rt_state_object) -> static HRESULT {
    // TODO: implementar d3d12_state_object_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_supports_small_resource_alignment(desc: *mut const D3D12_RESOURCE_DESC1, format: *mut const struct vkd3d_format) -> static bool {
    // TODO: implementar d3d12_resource_supports_small_resource_alignment desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_prefers_general_depth_stencil(device: *mut const struct d3d12_device) -> static bool {
    // TODO: implementar d3d12_device_prefers_general_depth_stencil desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_supports_universal_color_ds_copy(device: *mut struct d3d12_device) -> static bool {
    // TODO: implementar d3d12_device_supports_universal_color_ds_copy desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_sampler_needs_border_color(u: D3D12_TEXTURE_ADDRESS_MODE, v: D3D12_TEXTURE_ADDRESS_MODE, w: D3D12_TEXTURE_ADDRESS_MODE) -> static bool {
    // TODO: implementar d3d12_sampler_needs_border_color desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_get_tiling(device: *mut struct d3d12_device, resource: *mut struct d3d12_resource, total_tile_count: *mut UINT, packed_mip_info: *mut D3D12_PACKED_MIP_INFO, tile_shape: *mut D3D12_TILE_SHAPE, tilings: *mut D3D12_SUBRESOURCE_TILING, vk_info: *mut VkSparseImageMemoryRequirements) -> static void {
    // TODO: implementar d3d12_resource_get_tiling desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_destroy(resource: *mut struct d3d12_resource, device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_resource_destroy desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_validate_box(resource: *mut const struct d3d12_resource, subresource_idx: u32, box: *mut const D3D12_BOX) -> static bool {
    // TODO: implementar d3d12_resource_validate_box desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_QueryInterface(iface: *mut d3d12_resource_iface, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_resource_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_AddRef(iface: *mut d3d12_resource_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_resource_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_deferred_incref(userdata: *mut core::ffi::c_void) -> static void {
    // TODO: implementar d3d12_resource_deferred_incref desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_deferred_decref(userdata: *mut core::ffi::c_void) -> static void {
    // TODO: implementar d3d12_resource_deferred_decref desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_Release(iface: *mut d3d12_resource_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_resource_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_GetPrivateData(iface: *mut d3d12_resource_iface, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_resource_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_SetPrivateData(iface: *mut d3d12_resource_iface, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_resource_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_SetPrivateDataInterface(iface: *mut d3d12_resource_iface, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_resource_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_SetName(iface: *mut d3d12_resource_iface, str: LPCWSTR) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_resource_SetName desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_GetDevice(iface: *mut d3d12_resource_iface, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_resource_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_get_mapped_memory_range(resource: *mut struct d3d12_resource, subresource: UINT, range: *mut const D3D12_RANGE, vk_mapped_range: *mut VkMappedMemoryRange) -> static bool {
    // TODO: implementar d3d12_resource_get_mapped_memory_range desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_invalidate_range(resource: *mut struct d3d12_resource, subresource: UINT, read_range: *mut const D3D12_RANGE) -> static void {
    // TODO: implementar d3d12_resource_invalidate_range desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_flush_range(resource: *mut struct d3d12_resource, subresource: UINT, written_range: *mut const D3D12_RANGE) -> static void {
    // TODO: implementar d3d12_resource_flush_range desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_get_map_ptr(resource: *mut struct d3d12_resource, param_64866: *mut core::ffi::c_void) -> static void {
    // TODO: implementar d3d12_resource_get_map_ptr desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_texture_validate_map(resource: *mut struct d3d12_resource) -> static bool {
    // TODO: implementar d3d12_resource_texture_validate_map desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_Map(iface: *mut d3d12_resource_iface, sub_resource: UINT, read_range: *mut const D3D12_RANGE, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_resource_Map desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_Unmap(iface: *mut d3d12_resource_iface, sub_resource: UINT, written_range: *mut const D3D12_RANGE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_resource_Unmap desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_GetDesc(iface: *mut d3d12_resource_iface, resource_desc: *mut D3D12_RESOURCE_DESC) -> static D3D12_RESOURCE_DESC * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_resource_GetDesc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_GetGPUVirtualAddress(iface: *mut d3d12_resource_iface) -> static D3D12_GPU_VIRTUAL_ADDRESS STDMETHODCALLTYPE {
    // TODO: implementar d3d12_resource_GetGPUVirtualAddress desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_GetHeapProperties(iface: *mut d3d12_resource_iface, heap_properties: *mut D3D12_HEAP_PROPERTIES, flags: *mut D3D12_HEAP_FLAGS) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_resource_GetHeapProperties desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_GetDesc1(iface: *mut d3d12_resource_iface, resource_desc: *mut D3D12_RESOURCE_DESC1) -> static D3D12_RESOURCE_DESC1 * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_resource_GetDesc1 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_validate_texture_format(desc: *mut const D3D12_RESOURCE_DESC1, format: *mut const struct vkd3d_format) -> static bool {
    // TODO: implementar d3d12_resource_validate_texture_format desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_validate_texture_alignment(desc: *mut const D3D12_RESOURCE_DESC1, format: *mut const struct vkd3d_format) -> static bool {
    // TODO: implementar d3d12_resource_validate_texture_alignment desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_validate_resource_flags(param_15770: desc->Flags) -> return {
    // TODO: implementar d3d12_validate_resource_flags desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3D12_RESOURCE_STATE_COPY_DEST(COMMON: or) -> For D3D12_HEAP_TYPE_READBACK the state must be {
    // TODO: implementar D3D12_RESOURCE_STATE_COPY_DEST desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_init_page_table(resource: *mut struct d3d12_resource, device: *mut struct d3d12_device, sparse: *mut struct d3d12_sparse_info) -> static HRESULT {
    // TODO: implementar d3d12_resource_init_page_table desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_init_sparse_info(resource: *mut struct d3d12_resource, device: *mut struct d3d12_device, sparse: *mut struct d3d12_sparse_info) -> static HRESULT {
    // TODO: implementar d3d12_resource_init_sparse_info desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_wait_for_sparse_init(resource: *mut struct d3d12_resource) -> static void {
    // TODO: implementar d3d12_resource_wait_for_sparse_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_destroy_and_release_device(resource: *mut struct d3d12_resource, device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_resource_destroy_and_release_device desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_init_subresource_layouts(resource: *mut struct d3d12_resource, device: *mut struct d3d12_device) -> static size_t {
    // TODO: implementar d3d12_resource_init_subresource_layouts desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_determine_alignment(device: *mut struct d3d12_device, desc: *mut const D3D12_RESOURCE_DESC1, num_castable_formats: UINT, castable_formats: *mut const DXGI_FORMAT) -> static UINT64 {
    // TODO: implementar d3d12_resource_determine_alignment desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_desc_default_alignment(param_7058: desc) -> return {
    // TODO: implementar d3d12_resource_desc_default_alignment desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_tag_debug_name(resource: *mut struct d3d12_resource, device: *mut struct d3d12_device, tag: *mut const char) -> static void {
    // TODO: implementar d3d12_resource_tag_debug_name desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_validate_heap(resource_desc: *mut const D3D12_RESOURCE_DESC1, heap: *mut struct d3d12_heap) -> static HRESULT {
    // TODO: implementar d3d12_resource_validate_heap desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_copy_range(dst_va: vkd3d_cpu_descriptor_va_t, src_va: vkd3d_cpu_descriptor_va_t, count: u32, heap_type: D3D12_DESCRIPTOR_HEAP_TYPE, device: *mut struct d3d12_device) -> core::ffi::c_void {
    // TODO: implementar d3d12_desc_copy_range desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_QueryInterface(iface: *mut ID3D12DescriptorHeap, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_descriptor_heap_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_AddRef(iface: *mut ID3D12DescriptorHeap) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_descriptor_heap_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_Release(iface: *mut ID3D12DescriptorHeap) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_descriptor_heap_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_GetPrivateData(iface: *mut ID3D12DescriptorHeap, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_descriptor_heap_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_SetPrivateData(iface: *mut ID3D12DescriptorHeap, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_descriptor_heap_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_SetPrivateDataInterface(iface: *mut ID3D12DescriptorHeap, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_descriptor_heap_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_GetDevice(iface: *mut ID3D12DescriptorHeap, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_descriptor_heap_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_GetDesc(iface: *mut ID3D12DescriptorHeap, desc: *mut D3D12_DESCRIPTOR_HEAP_DESC) -> static D3D12_DESCRIPTOR_HEAP_DESC * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_descriptor_heap_GetDesc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_GetCPUDescriptorHandleForHeapStart(iface: *mut ID3D12DescriptorHeap, descriptor: *mut D3D12_CPU_DESCRIPTOR_HANDLE) -> static D3D12_CPU_DESCRIPTOR_HANDLE * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_descriptor_heap_GetCPUDescriptorHandleForHeapStart desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_GetGPUDescriptorHandleForHeapStart(iface: *mut ID3D12DescriptorHeap, descriptor: *mut D3D12_GPU_DESCRIPTOR_HANDLE) -> static D3D12_GPU_DESCRIPTOR_HANDLE * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_descriptor_heap_GetGPUDescriptorHandleForHeapStart desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_zero_initialize(descriptor_heap: *mut struct d3d12_descriptor_heap, vk_descriptor_type: VkDescriptorType, vk_descriptor_set: VkDescriptorSet, binding_index: u32, descriptor_count: u32) -> static void {
    // TODO: implementar d3d12_descriptor_heap_zero_initialize desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_get_host_mapping(descriptor_heap: *mut struct d3d12_descriptor_heap, binding: *mut const struct vkd3d_bindless_set_info, set_index: u32) -> static void {
    // TODO: implementar d3d12_descriptor_heap_get_host_mapping desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_get_buffer_range(descriptor_heap: *mut struct d3d12_descriptor_heap, offset: *mut VkDeviceSize, size: VkDeviceSize, range: *mut struct vkd3d_host_visible_buffer_range) -> static void {
    // TODO: implementar d3d12_descriptor_heap_get_buffer_range desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_init_data_buffer(descriptor_heap: *mut struct d3d12_descriptor_heap, device: *mut struct d3d12_device, desc: *mut const D3D12_DESCRIPTOR_HEAP_DESC) -> static HRESULT {
    // TODO: implementar d3d12_descriptor_heap_init_data_buffer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_update_extra_bindings(descriptor_heap: *mut struct d3d12_descriptor_heap, device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_descriptor_heap_update_extra_bindings desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_add_null_descriptor_template_buffers(descriptor_heap: *mut struct d3d12_descriptor_heap, set_info: *mut const struct vkd3d_bindless_set_info, set_info_index: u32) -> static void {
    // TODO: implementar d3d12_descriptor_heap_add_null_descriptor_template_buffers desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_add_null_descriptor_template_descriptors(descriptor_heap: *mut struct d3d12_descriptor_heap, set_info: *mut const struct vkd3d_bindless_set_info, set_info_index: u32) -> static void {
    // TODO: implementar d3d12_descriptor_heap_add_null_descriptor_template_descriptors desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_init(descriptor_heap: *mut struct d3d12_descriptor_heap, device: *mut struct d3d12_device, desc: *mut const D3D12_DESCRIPTOR_HEAP_DESC) -> static HRESULT {
    // TODO: implementar d3d12_descriptor_heap_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_init_descriptors(descriptor_heap: *mut struct d3d12_descriptor_heap) -> static void {
    // TODO: implementar d3d12_descriptor_heap_init_descriptors desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_QueryInterface(iface: *mut ID3D12QueryHeap, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_query_heap_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_AddRef(iface: *mut ID3D12QueryHeap) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_query_heap_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_Release(iface: *mut ID3D12QueryHeap) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_query_heap_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_GetPrivateData(iface: *mut ID3D12QueryHeap, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_query_heap_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_SetPrivateData(iface: *mut ID3D12QueryHeap, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_query_heap_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_SetPrivateDataInterface(iface: *mut ID3D12QueryHeap, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_query_heap_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_query_heap_GetDevice(iface: *mut ID3D12QueryHeap, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_query_heap_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_QueryInterface(iface: *mut ID3D12RootSignature, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_root_signature_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_AddRef(iface: *mut ID3D12RootSignature) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_root_signature_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_cleanup(root_signature: *mut struct d3d12_root_signature, device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_root_signature_cleanup desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_Release(iface: *mut ID3D12RootSignature) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_root_signature_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_GetPrivateData(iface: *mut ID3D12RootSignature, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_root_signature_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_SetPrivateData(iface: *mut ID3D12RootSignature, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_root_signature_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_SetPrivateDataInterface(iface: *mut ID3D12RootSignature, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_root_signature_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_GetDevice(iface: *mut ID3D12RootSignature, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_root_signature_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_info_count_srv_uav_table(info: *mut struct d3d12_root_signature_info, device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_root_signature_info_count_srv_uav_table desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_info_count_cbv_table(info: *mut struct d3d12_root_signature_info) -> static void {
    // TODO: implementar d3d12_root_signature_info_count_cbv_table desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_info_count_sampler_table(info: *mut struct d3d12_root_signature_info) -> static void {
    // TODO: implementar d3d12_root_signature_info_count_sampler_table desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_may_require_global_heap_binding(device: *mut struct d3d12_device) -> static bool {
    // TODO: implementar d3d12_root_signature_may_require_global_heap_binding desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_info_from_desc(info: *mut struct d3d12_root_signature_info, device: *mut struct d3d12_device, desc: *mut const D3D12_ROOT_SIGNATURE_DESC2) -> static HRESULT {
    // TODO: implementar d3d12_root_signature_info_from_desc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_srv_uav_binding(root_signature: *mut struct d3d12_root_signature, context: *mut struct vkd3d_descriptor_set_context, range_type: D3D12_DESCRIPTOR_RANGE_TYPE, binding: *mut struct vkd3d_shader_resource_binding, out_bindings_base: *mut struct vkd3d_shader_resource_binding, out_index: *mut u32) -> static void {
    // TODO: implementar d3d12_root_signature_init_srv_uav_binding desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_srv_uav_heap_bindings(root_signature: *mut struct d3d12_root_signature, context: *mut struct vkd3d_descriptor_set_context, range_type: D3D12_DESCRIPTOR_RANGE_TYPE) -> static void {
    // TODO: implementar d3d12_root_signature_init_srv_uav_heap_bindings desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_cbv_srv_uav_heap_bindings(root_signature: *mut struct d3d12_root_signature, context: *mut struct vkd3d_descriptor_set_context) -> static void {
    // TODO: implementar d3d12_root_signature_init_cbv_srv_uav_heap_bindings desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_sampler_heap_bindings(root_signature: *mut struct d3d12_root_signature, context: *mut struct vkd3d_descriptor_set_context) -> static void {
    // TODO: implementar d3d12_root_signature_init_sampler_heap_bindings desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_root_descriptor_tables(root_signature: *mut struct d3d12_root_signature, desc: *mut const D3D12_ROOT_SIGNATURE_DESC2, info: *mut const struct d3d12_root_signature_info, context: *mut struct vkd3d_descriptor_set_context) -> static HRESULT {
    // TODO: implementar d3d12_root_signature_init_root_descriptor_tables desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_add_common_flags(root_signature: *mut struct d3d12_root_signature, common_flags: u32) -> static void {
    // TODO: implementar d3d12_root_signature_add_common_flags desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_shader_record_descriptors(root_signature: *mut struct d3d12_root_signature, desc: *mut const D3D12_ROOT_SIGNATURE_DESC2, info: *mut const struct d3d12_root_signature_info, context: *mut struct vkd3d_descriptor_set_context) -> static HRESULT {
    // TODO: implementar d3d12_root_signature_init_shader_record_descriptors desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_root_descriptors(root_signature: *mut struct d3d12_root_signature, desc: *mut const D3D12_ROOT_SIGNATURE_DESC2, info: *mut struct d3d12_root_signature_info, push_constant_range: *mut const VkPushConstantRange, context: *mut struct vkd3d_descriptor_set_context, vk_set_layout: *mut VkDescriptorSetLayout) -> static HRESULT {
    // TODO: implementar d3d12_root_signature_init_root_descriptors desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_local_static_samplers(root_signature: *mut struct d3d12_root_signature, desc: *mut const D3D12_ROOT_SIGNATURE_DESC2) -> static HRESULT {
    // TODO: implementar d3d12_root_signature_init_local_static_samplers desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_static_samplers(root_signature: *mut struct d3d12_root_signature, desc: *mut const D3D12_ROOT_SIGNATURE_DESC2, context: *mut struct vkd3d_descriptor_set_context, vk_set_layout: *mut VkDescriptorSetLayout) -> static HRESULT {
    // TODO: implementar d3d12_root_signature_init_static_samplers desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_local(root_signature: *mut struct d3d12_root_signature, device: *mut struct d3d12_device, desc: *mut const D3D12_ROOT_SIGNATURE_DESC2) -> static HRESULT {
    // TODO: implementar d3d12_root_signature_init_local desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init(root_signature: *mut struct d3d12_root_signature, device: *mut struct d3d12_device, desc: *mut const D3D12_ROOT_SIGNATURE_DESC2) -> static HRESULT {
    // TODO: implementar d3d12_root_signature_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_line_rasteriztion_mode_from_legacy_state(MultisampleEnable: i32, AntialiasedLineEnable: i32) -> static D3D12_LINE_RASTERIZATION_MODE {
    // TODO: implementar d3d12_line_rasteriztion_mode_from_legacy_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_promote_rasterizer_desc(out: *mut D3D12_RASTERIZER_DESC2, in: *mut const D3D12_RASTERIZER_DESC) -> static void {
    // TODO: implementar d3d12_promote_rasterizer_desc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_promote_rasterizer_desc1(out: *mut D3D12_RASTERIZER_DESC2, in: *mut const D3D12_RASTERIZER_DESC1) -> static void {
    // TODO: implementar d3d12_promote_rasterizer_desc1 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_promote_depth_stencil_desc(out: *mut D3D12_DEPTH_STENCIL_DESC2, in: *mut const D3D12_DEPTH_STENCIL_DESC) -> static void {
    // TODO: implementar d3d12_promote_depth_stencil_desc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_promote_depth_stencil_desc1(out: *mut D3D12_DEPTH_STENCIL_DESC2, in: *mut const D3D12_DEPTH_STENCIL_DESC1) -> static void {
    // TODO: implementar d3d12_promote_depth_stencil_desc1 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_init_pipeline_state_desc(desc: *mut struct d3d12_pipeline_state_desc) -> static void {
    // TODO: implementar d3d12_init_pipeline_state_desc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_QueryInterface(iface: *mut ID3D12PipelineState, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_state_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_AddRef(iface: *mut ID3D12PipelineState) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_state_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_destroy_shader_modules(state: *mut struct d3d12_pipeline_state, device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_pipeline_state_destroy_shader_modules desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_destroy_graphics(state: *mut struct d3d12_pipeline_state, device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_pipeline_state_destroy_graphics desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_Release(iface: *mut ID3D12PipelineState) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_state_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_GetPrivateData(iface: *mut ID3D12PipelineState, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_state_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_SetPrivateData(iface: *mut ID3D12PipelineState, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_state_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_SetPrivateDataInterface(iface: *mut ID3D12PipelineState, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_state_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_GetDevice(iface: *mut ID3D12PipelineState, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_state_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_GetCachedBlob(iface: *mut ID3D12PipelineState, param_52875: *mut ID3DBlob) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_pipeline_state_GetCachedBlob desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_init_shader_interface(state: *mut struct d3d12_pipeline_state, device: *mut struct d3d12_device, stage: VkShaderStageFlagBits, shader_interface: *mut struct vkd3d_shader_interface_info) -> static void {
    // TODO: implementar d3d12_pipeline_state_init_shader_interface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_init_compile_arguments(state: *mut struct d3d12_pipeline_state, device: *mut struct d3d12_device, stage: VkShaderStageFlagBits, compile_arguments: *mut struct vkd3d_shader_compile_arguments) -> static void {
    // TODO: implementar d3d12_pipeline_state_init_compile_arguments desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_init_compute(state: *mut struct d3d12_pipeline_state, device: *mut struct d3d12_device, desc: *mut const struct d3d12_pipeline_state_desc, cached_pso: *mut const struct d3d12_cached_pipeline_state) -> static HRESULT {
    // TODO: implementar d3d12_pipeline_state_init_compute desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rs_desc_from_d3d12(vk_desc: *mut VkPipelineRasterizationStateCreateInfo, d3d12_desc: *mut const D3D12_RASTERIZER_DESC2) -> static void {
    // TODO: implementar rs_desc_from_d3d12 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rs_conservative_info_from_d3d12(conservative_info: *mut VkPipelineRasterizationConservativeStateCreateInfoEXT, vk_rs_desc: *mut VkPipelineRasterizationStateCreateInfo, d3d12_desc: *mut const D3D12_RASTERIZER_DESC2) -> static void {
    // TODO: implementar rs_conservative_info_from_d3d12 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rs_depth_clip_info_from_d3d12(depth_clip_info: *mut VkPipelineRasterizationDepthClipStateCreateInfoEXT, vk_rs_desc: *mut VkPipelineRasterizationStateCreateInfo, d3d12_desc: *mut const D3D12_RASTERIZER_DESC2) -> static void {
    // TODO: implementar rs_depth_clip_info_from_d3d12 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rs_line_info_from_d3d12(device: *mut struct d3d12_device, vk_line_info: *mut VkPipelineRasterizationLineStateCreateInfoEXT, vk_rs_desc: *mut VkPipelineRasterizationStateCreateInfo, d3d12_desc: *mut const D3D12_RASTERIZER_DESC2) -> static void {
    // TODO: implementar rs_line_info_from_d3d12 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ds_desc_from_d3d12(vk_desc: *mut struct VkPipelineDepthStencilStateCreateInfo, d3d12_desc: *mut const D3D12_DEPTH_STENCIL_DESC2) -> static void {
    // TODO: implementar ds_desc_from_d3d12 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn blend_desc_from_d3d12(vk_desc: *mut VkPipelineColorBlendStateCreateInfo, d3d12_desc: *mut const D3D12_BLEND_DESC, attachment_count: u32, attachments: *mut const VkPipelineColorBlendAttachmentState) -> static void {
    // TODO: implementar blend_desc_from_d3d12 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_graphics_pipeline_state_get_plane_optimal_mask(graphics: *mut struct d3d12_graphics_pipeline_state) -> static uint32_t {
    // TODO: implementar d3d12_graphics_pipeline_state_get_plane_optimal_mask desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_graphics_pipeline_state_get_dynamic_state_flags(state: *mut struct d3d12_pipeline_state, key: *mut const struct vkd3d_pipeline_key) -> u32 {
    // TODO: implementar d3d12_graphics_pipeline_state_get_dynamic_state_flags desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_graphics_pipeline_state_init_dynamic_state(state: *mut struct d3d12_pipeline_state, dynamic_desc: *mut VkPipelineDynamicStateCreateInfo, dynamic_state_buffer: *mut VkDynamicState, key: *mut const struct vkd3d_pipeline_key) -> static uint32_t {
    // TODO: implementar d3d12_graphics_pipeline_state_init_dynamic_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_validate_blend_state(state: *mut struct d3d12_pipeline_state, device: *mut const struct d3d12_device, desc: *mut const struct d3d12_pipeline_state_desc, sig: *mut const struct vkd3d_shader_signature) -> static HRESULT {
    // TODO: implementar d3d12_pipeline_state_validate_blend_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_graphics_load_spirv_from_cached_state(state: *mut struct d3d12_pipeline_state, device: *mut struct d3d12_device, desc: *mut const struct d3d12_pipeline_state_desc, cached_pso: *mut const struct d3d12_cached_pipeline_state) -> static void {
    // TODO: implementar d3d12_pipeline_state_graphics_load_spirv_from_cached_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_graphics_handle_meta(state: *mut struct d3d12_pipeline_state, device: *mut struct d3d12_device) -> static HRESULT {
    // TODO: implementar d3d12_pipeline_state_graphics_handle_meta desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_init_graphics_spirv(state: *mut struct d3d12_pipeline_state, desc: *mut const struct d3d12_pipeline_state_desc, cached_pso: *mut const struct d3d12_cached_pipeline_state) -> static HRESULT {
    // TODO: implementar d3d12_pipeline_state_init_graphics_spirv desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_init_static_pipeline(state: *mut struct d3d12_pipeline_state, desc: *mut const struct d3d12_pipeline_state_desc) -> static HRESULT {
    // TODO: implementar d3d12_pipeline_state_init_static_pipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_finish_graphics(state: *mut struct d3d12_pipeline_state) -> static HRESULT {
    // TODO: implementar d3d12_pipeline_state_finish_graphics desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_find_compiled_pipeline(state: *mut struct d3d12_pipeline_state, key: *mut const struct vkd3d_pipeline_key, dynamic_state_flags: *mut u32) -> static VkPipeline {
    // TODO: implementar d3d12_pipeline_state_find_compiled_pipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_put_pipeline_to_cache(state: *mut struct d3d12_pipeline_state, key: *mut const struct vkd3d_pipeline_key, vk_pipeline: VkPipeline, dynamic_state_flags: u32) -> static bool {
    // TODO: implementar d3d12_pipeline_state_put_pipeline_to_cache desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_link_pipeline_variant(state: *mut struct d3d12_pipeline_state, key: *mut const struct vkd3d_pipeline_key, dsv_format: *mut const struct vkd3d_format, vk_cache: VkPipelineCache, dynamic_state_flags: u32, vk_pipeline: *mut VkPipeline) -> static VkResult {
    // TODO: implementar d3d12_pipeline_state_link_pipeline_variant desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_max_descriptor_count_from_heap_type(device: *mut struct d3d12_device, heap_type: D3D12_DESCRIPTOR_HEAP_TYPE) -> static uint32_t {
    // TODO: implementar d3d12_max_descriptor_count_from_heap_type desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_max_host_descriptor_count_from_heap_type(device: *mut struct d3d12_device, heap_type: D3D12_DESCRIPTOR_HEAP_TYPE) -> static uint32_t {
    // TODO: implementar d3d12_max_host_descriptor_count_from_heap_type desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExecuteIndirect(pCommandSignature: *mut _In_ ID3D12CommandSignature, MaxCommandCount: _In_ UINT, pArgumentBuffer: *mut _In_ ID3D12Resource, ArgumentBufferOffset: _In_ UINT64, pCountBuffer: *mut _In_opt_ ID3D12Resource, CountBufferOffset: _In_ UINT64) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ExecuteIndirect desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12CommandQueue_AddRef(param_62859: &chain->queue->ID3D12CommandQueue_iface) -> return {
    // TODO: implementar ID3D12CommandQueue_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12CommandQueue_Release(param_62859: &chain->queue->ID3D12CommandQueue_iface) -> return {
    // TODO: implementar ID3D12CommandQueue_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12CommandQueue_QueryInterface(param_62859: &chain->queue->ID3D12CommandQueue_iface, param_63178: riid, param_39919: object) -> return {
    // TODO: implementar ID3D12CommandQueue_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Resource2_QueryInterface(param_63178: riid, param_39919: object) -> return {
    // TODO: implementar ID3D12Resource2_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DXGI_FORMAT(param_60231: %u) -> Unknown {
    // TODO: implementar DXGI_FORMAT desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ID3D12Object_SetPrivateData(param_31668: iface, param_37033: &WKPDID_D3DDebugObjectNameW, param_44127: size, param_340: name) -> return {
    // TODO: implementar ID3D12Object_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_deserializer_QueryInterface(iface: *mut ID3D12RootSignatureDeserializer, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_root_signature_deserializer_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_deserializer_init(deserializer: *mut struct d3d12_root_signature_deserializer, dxbc: *mut const struct vkd3d_shader_code) -> static HRESULT {
    // TODO: implementar d3d12_root_signature_deserializer_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_versioned_root_signature_deserializer_QueryInterface(iface: *mut ID3D12VersionedRootSignatureDeserializer, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_versioned_root_signature_deserializer_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_versioned_root_signature_deserializer_init(deserializer: *mut struct d3d12_versioned_root_signature_deserializer, dxbc: *mut const struct vkd3d_shader_code, raw_payload: bool) -> static HRESULT {
    // TODO: implementar d3d12_versioned_root_signature_deserializer_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_cleanup_data(data: *mut struct d3d12_wg_state_object_data, device: *mut struct d3d12_device) -> static void {
    // TODO: implementar d3d12_wg_state_object_cleanup_data desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_parse_subobject(data: *mut struct d3d12_wg_state_object_data, device: *mut struct d3d12_device, obj: *mut const D3D12_STATE_SUBOBJECT, association_priority: u32) -> static HRESULT {
    // TODO: implementar d3d12_wg_state_object_parse_subobject desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_find_array_size_by_id(entries: *mut const struct vkd3d_shader_library_entry_point, entry_count: usize, node_id: *mut const char) -> static uint32_t {
    // TODO: implementar d3d12_work_graph_find_array_size_by_id desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_find_node_by_id(entries: *mut const struct vkd3d_shader_library_entry_point, entry_count: usize, node_id: *mut const char, node_array_index: UINT) -> static uint32_t {
    // TODO: implementar d3d12_work_graph_find_node_by_id desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_find_node_by_id_wchar(entries: *mut const struct vkd3d_shader_library_entry_point, entry_count: usize, node_id: LPCWSTR, node_array_index: UINT) -> static uint32_t {
    // TODO: implementar d3d12_work_graph_find_node_by_id_wchar desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_program_add_node_to_level(program: *mut struct d3d12_wg_state_object_program, data: *mut struct d3d12_wg_state_object_data, entry_point_index: u32, level: u32, is_recursing: bool) -> static HRESULT {
    // TODO: implementar d3d12_wg_state_object_program_add_node_to_level desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_program_add_outputs_to_level(program: *mut struct d3d12_wg_state_object_program, data: *mut struct d3d12_wg_state_object_data, entry_point_index: u32, level: u32) -> static HRESULT {
    // TODO: implementar d3d12_wg_state_object_program_add_outputs_to_level desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_rearrange_entry_points(data: *mut struct d3d12_wg_state_object_data) -> static HRESULT {
    // TODO: implementar d3d12_wg_state_object_rearrange_entry_points desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_apply_node_overrides(data: *mut struct d3d12_wg_state_object_data, program: *mut struct d3d12_wg_state_object_program) -> static HRESULT {
    // TODO: implementar d3d12_wg_state_object_apply_node_overrides desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_resolve_entry_points(object: *mut struct d3d12_wg_state_object, data: *mut struct d3d12_wg_state_object_data, program: *mut struct d3d12_wg_state_object_program) -> static HRESULT {
    // TODO: implementar d3d12_wg_state_object_resolve_entry_points desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_parse_subobjects(data: *mut struct d3d12_wg_state_object_data, device: *mut struct d3d12_device, desc: *mut const D3D12_STATE_OBJECT_DESC) -> static HRESULT {
    // TODO: implementar d3d12_wg_state_object_parse_subobjects desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_QueryInterface(iface: *mut ID3D12StateObject, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_wg_state_object_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_QueryInterface(iface: *mut d3d12_state_object_properties_iface, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_wg_state_object_properties_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_QueryInterface(iface: *mut d3d12_work_graph_properties_iface, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_work_graph_properties_QueryInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_AddRef(iface: *mut ID3D12StateObject) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_wg_state_object_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_AddRef(iface: *mut d3d12_state_object_properties_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_wg_state_object_properties_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_AddRef(iface: *mut d3d12_work_graph_properties_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_work_graph_properties_AddRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_cleanup(state_object: *mut struct d3d12_wg_state_object) -> static void {
    // TODO: implementar d3d12_wg_state_object_cleanup desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_dec_ref(state_object: *mut struct d3d12_wg_state_object) -> static void {
    // TODO: implementar d3d12_wg_state_object_dec_ref desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_release(state_object: *mut struct d3d12_wg_state_object) -> static ULONG {
    // TODO: implementar d3d12_wg_state_object_release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_Release(iface: *mut ID3D12StateObject) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_wg_state_object_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_Release(iface: *mut d3d12_state_object_properties_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_wg_state_object_properties_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_Release(iface: *mut d3d12_work_graph_properties_iface) -> static ULONG STDMETHODCALLTYPE {
    // TODO: implementar d3d12_work_graph_properties_Release desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_GetPrivateData(iface: *mut ID3D12StateObject, guid: REFGUID, data_size: *mut UINT, data: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_wg_state_object_GetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_SetPrivateData(iface: *mut ID3D12StateObject, guid: REFGUID, data_size: UINT, data: *mut const void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_wg_state_object_SetPrivateData desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_SetPrivateDataInterface(iface: *mut ID3D12StateObject, guid: REFGUID, data: *mut const IUnknown) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_wg_state_object_SetPrivateDataInterface desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_GetDevice(iface: *mut ID3D12StateObject, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_wg_state_object_GetDevice desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_GetShaderIdentifier(iface: *mut d3d12_state_object_properties_iface, export_name: LPCWSTR) -> static void * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_wg_state_object_properties_GetShaderIdentifier desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_GetShaderStackSize(iface: *mut d3d12_state_object_properties_iface, export_name: LPCWSTR) -> static UINT64 STDMETHODCALLTYPE {
    // TODO: implementar d3d12_wg_state_object_properties_GetShaderStackSize desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_GetPipelineStackSize(iface: *mut d3d12_state_object_properties_iface) -> static UINT64 STDMETHODCALLTYPE {
    // TODO: implementar d3d12_wg_state_object_properties_GetPipelineStackSize desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_SetPipelineStackSize(iface: *mut d3d12_state_object_properties_iface, stack_size_in_bytes: UINT64) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_wg_state_object_properties_SetPipelineStackSize desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_properties_GetProgramIdentifier(iface: *mut d3d12_state_object_properties_iface, ret: *mut D3D12_PROGRAM_IDENTIFIER, pProgramName: LPCWSTR) -> static D3D12_PROGRAM_IDENTIFIER * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_wg_state_object_properties_GetProgramIdentifier desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetNumWorkGraphs(iface: *mut d3d12_work_graph_properties_iface) -> static UINT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_work_graph_properties_GetNumWorkGraphs desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetProgramName(iface: *mut d3d12_work_graph_properties_iface, WorkGraphIndex: UINT) -> static LPCWSTR STDMETHODCALLTYPE {
    // TODO: implementar d3d12_work_graph_properties_GetProgramName desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetWorkGraphIndex(iface: *mut d3d12_work_graph_properties_iface, pProgramName: LPCWSTR) -> static UINT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_work_graph_properties_GetWorkGraphIndex desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetNumNodes(iface: *mut d3d12_work_graph_properties_iface, WorkGraphIndex: UINT) -> static UINT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_work_graph_properties_GetNumNodes desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_node_index_to_entry(program: *mut const struct d3d12_wg_state_object_program, NodeIndex: UINT) -> static unsigned int {
    // TODO: implementar d3d12_work_graph_properties_node_index_to_entry desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetNodeID(iface: *mut d3d12_work_graph_properties_iface, ret: *mut D3D12_NODE_ID, WorkGraphIndex: UINT, NodeIndex: UINT) -> static D3D12_NODE_ID * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_work_graph_properties_GetNodeID desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetNodeIndex(iface: *mut d3d12_work_graph_properties_iface, WorkGraphIndex: UINT, NodeID: D3D12_NODE_ID) -> static UINT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_work_graph_properties_GetNodeIndex desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetNodeLocalRootArgumentsTableIndex(iface: *mut d3d12_work_graph_properties_iface, WorkGraphIndex: UINT, NodeIndex: UINT) -> static UINT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_work_graph_properties_GetNodeLocalRootArgumentsTableIndex desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetNumEntrypoints(iface: *mut d3d12_work_graph_properties_iface, WorkGraphIndex: UINT) -> static UINT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_work_graph_properties_GetNumEntrypoints desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetEntrypointID(iface: *mut d3d12_work_graph_properties_iface, ret: *mut D3D12_NODE_ID, WorkGraphIndex: UINT, EntrypointIndex: UINT) -> static D3D12_NODE_ID * STDMETHODCALLTYPE {
    // TODO: implementar d3d12_work_graph_properties_GetEntrypointID desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetEntrypointIndex(iface: *mut d3d12_work_graph_properties_iface, WorkGraphIndex: UINT, NodeID: D3D12_NODE_ID) -> static UINT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_work_graph_properties_GetEntrypointIndex desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetEntrypointRecordSizeInBytes(iface: *mut d3d12_work_graph_properties_iface, WorkGraphIndex: UINT, EntrypointIndex: UINT) -> static UINT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_work_graph_properties_GetEntrypointRecordSizeInBytes desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_work_graph_properties_GetWorkGraphMemoryRequirements(iface: *mut d3d12_work_graph_properties_iface, WorkGraphIndex: UINT, pWorkGraphMemoryRequirements: *mut D3D12_WORK_GRAPH_MEMORY_REQUIREMENTS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_work_graph_properties_GetWorkGraphMemoryRequirements desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_compile_pipeline(object: *mut struct d3d12_wg_state_object, data: *mut struct d3d12_wg_state_object_data, program: *mut struct d3d12_wg_state_object_program, entry_point_index: u32, tmp: *mut struct d3d12_wg_state_object_spec_constant_tmp) -> static HRESULT {
    // TODO: implementar d3d12_wg_state_object_compile_pipeline desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_program_can_compact_broadcast_nodes(object: *mut struct d3d12_wg_state_object, data: *mut struct d3d12_wg_state_object_data, program: *mut struct d3d12_wg_state_object_program) -> static bool {
    // TODO: implementar d3d12_wg_state_object_program_can_compact_broadcast_nodes desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_compile_program(object: *mut struct d3d12_wg_state_object, data: *mut struct d3d12_wg_state_object_data, program: *mut struct d3d12_wg_state_object_program) -> static HRESULT {
    // TODO: implementar d3d12_wg_state_object_compile_program desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_convert_entry_point(object: *mut struct d3d12_wg_state_object, data: *mut struct d3d12_wg_state_object_data, module: *mut struct d3d12_wg_state_object_module, entry: *mut struct vkd3d_shader_library_entry_point) -> static HRESULT {
    // TODO: implementar d3d12_wg_state_object_convert_entry_point desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_compile_programs(object: *mut struct d3d12_wg_state_object, data: *mut struct d3d12_wg_state_object_data) -> static HRESULT {
    // TODO: implementar d3d12_wg_state_object_compile_programs desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_init(object: *mut struct d3d12_wg_state_object, device: *mut struct d3d12_device, desc: *mut const D3D12_STATE_OBJECT_DESC) -> static HRESULT {
    // TODO: implementar d3d12_wg_state_object_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_buffer(list: *mut struct d3d12_command_list, vk_buffer: VkBuffer, vk_offset: VkDeviceSize, size: VkDeviceSize, data_: *mut const void) -> static void {
    // TODO: implementar d3d12_command_list_update_buffer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_bind_resources(list: *mut struct d3d12_command_list, state: *mut const struct d3d12_wg_state_object, program: *mut const struct d3d12_wg_state_object_program, module: *mut const struct d3d12_wg_state_object_module, vk_root_parameter_buffer: VkBuffer, vk_root_parameter_buffer_offset: VkDeviceSize) -> static void {
    // TODO: implementar d3d12_command_list_workgraph_bind_resources desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_remaining_levels(program: *mut const struct d3d12_wg_state_object_program, level: u32, node_index: u32) -> static unsigned int {
    // TODO: implementar d3d12_command_list_workgraph_remaining_levels desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_execute_node_cpu_entry(list: *mut struct d3d12_command_list, state: *mut const struct d3d12_wg_state_object, program: *mut const struct d3d12_wg_state_object_program, node_index: u32, desc: *mut const D3D12_NODE_CPU_INPUT, output_payload: VkDeviceAddress, input_payload: VkDeviceAddress, vk_root_parameter_buffer: VkBuffer, vk_root_parameter_buffer_offset: VkDeviceSize) -> static void {
    // TODO: implementar d3d12_command_list_workgraph_execute_node_cpu_entry desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_barrier(list: *mut struct d3d12_command_list, vk_dst_stages: VkPipelineStageFlags2, vk_dst_access: VkAccessFlags2) -> static void {
    // TODO: implementar d3d12_command_list_workgraph_barrier desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_emit_distribute_workgroups(list: *mut struct d3d12_command_list, state: *mut const struct d3d12_wg_state_object, program: *mut const struct d3d12_wg_state_object_program) -> static void {
    // TODO: implementar d3d12_command_list_emit_distribute_workgroups desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_emit_distribute_payload_offsets(list: *mut struct d3d12_command_list, state: *mut const struct d3d12_wg_state_object, program: *mut const struct d3d12_wg_state_object_program, level: u32, payload_va: D3D12_GPU_VIRTUAL_ADDRESS) -> static void {
    // TODO: implementar d3d12_command_list_emit_distribute_payload_offsets desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_execute_node_gpu(list: *mut struct d3d12_command_list, state: *mut const struct d3d12_wg_state_object, program: *mut const struct d3d12_wg_state_object_program, output_va: D3D12_GPU_VIRTUAL_ADDRESS, input_va: D3D12_GPU_VIRTUAL_ADDRESS, level: u32, node_index: u32, indirect_scratch: *mut const struct vkd3d_scratch_allocation, vk_root_parameter_buffer: VkBuffer, vk_root_parameter_buffer_offset: VkDeviceSize) -> static void {
    // TODO: implementar d3d12_command_list_workgraph_execute_node_gpu desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_execute_level(list: *mut struct d3d12_command_list, state: *mut const struct d3d12_wg_state_object, program: *mut const struct d3d12_wg_state_object_program, level: u32, output_payload: VkDeviceAddress, input_payload: VkDeviceAddress, vk_root_parameter_buffer: VkBuffer, vk_root_parameter_buffer_offset: VkDeviceSize) -> static void {
    // TODO: implementar d3d12_command_list_workgraph_execute_level desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_setup_indirect(list: *mut struct d3d12_command_list, wg_state: *mut struct d3d12_wg_state_object, program: *mut const struct d3d12_wg_state_object_program, va: D3D12_GPU_VIRTUAL_ADDRESS, indirect_scratch: *mut struct vkd3d_scratch_allocation) -> static bool {
    // TODO: implementar d3d12_command_list_workgraph_setup_indirect desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_execute_entry_gpu(list: *mut struct d3d12_command_list, state: *mut struct d3d12_wg_state_object, program: *mut const struct d3d12_wg_state_object_program, va: D3D12_GPU_VIRTUAL_ADDRESS, vk_root_param_buffer: VkBuffer, vk_root_param_offset: VkDeviceSize) -> static void {
    // TODO: implementar d3d12_command_list_workgraph_execute_entry_gpu desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_execute_entry_cpu(list: *mut struct d3d12_command_list, wg_state: *mut struct d3d12_wg_state_object, program: *mut const struct d3d12_wg_state_object_program, desc: *mut const D3D12_NODE_CPU_INPUT, vk_root_param_buffer: VkBuffer, vk_root_param_offset: VkDeviceSize) -> static void {
    // TODO: implementar d3d12_command_list_workgraph_execute_entry_cpu desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_workgraph_execute_entry_level(list: *mut struct d3d12_command_list, wg_state: *mut struct d3d12_wg_state_object, program: *mut const struct d3d12_wg_state_object_program, desc: *mut const D3D12_DISPATCH_GRAPH_DESC, vk_root_param_buffer: VkBuffer, vk_root_param_offset: VkDeviceSize) -> static void {
    // TODO: implementar d3d12_command_list_workgraph_execute_entry_level desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetName(Name: _In_z_ LPCWSTR) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetName desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetSerializedSize() -> virtual SIZE_T STDMETHODCALLTYPE {
    // TODO: implementar GetSerializedSize desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetSerializedData(param_42423: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetSerializedData desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetRootSignatureDesc() -> virtual const D3D12_ROOT_SIGNATURE_DESC *STDMETHODCALLTYPE {
    // TODO: implementar GetRootSignatureDesc desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetRootSignatureDescAtVersion(convertToVersion: D3D_ROOT_SIGNATURE_VERSION, param_35955: *mut _Out_ const D3D12_VERSIONED_ROOT_SIGNATURE_DESC) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetRootSignatureDescAtVersion desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetUnconvertedRootSignatureDesc() -> virtual const D3D12_VERSIONED_ROOT_SIGNATURE_DESC *STDMETHODCALLTYPE {
    // TODO: implementar GetUnconvertedRootSignatureDesc desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetGPUVirtualAddress() -> virtual D3D12_GPU_VIRTUAL_ADDRESS STDMETHODCALLTYPE {
    // TODO: implementar GetGPUVirtualAddress desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetHeapProperties(pHeapProperties: *mut _Out_opt_ D3D12_HEAP_PROPERTIES, pHeapFlags: *mut _Out_opt_ D3D12_HEAP_FLAGS) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetHeapProperties desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetCachedBlob(param_34927: *mut _COM_Outptr_ ID3DBlob) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetCachedBlob desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetRootSignature(riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetRootSignature desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetCPUDescriptorHandleForHeapStart() -> virtual D3D12_CPU_DESCRIPTOR_HANDLE STDMETHODCALLTYPE {
    // TODO: implementar GetCPUDescriptorHandleForHeapStart desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetGPUDescriptorHandleForHeapStart() -> virtual D3D12_GPU_DESCRIPTOR_HANDLE STDMETHODCALLTYPE {
    // TODO: implementar GetGPUDescriptorHandleForHeapStart desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CopyBufferRegion(pDstBuffer: *mut _In_ ID3D12Resource, DstOffset: UINT64, pSrcBuffer: *mut _In_ ID3D12Resource, SrcOffset: UINT64, NumBytes: UINT64) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar CopyBufferRegion desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CopyTextureRegion(pDst: *mut _In_ const D3D12_TEXTURE_COPY_LOCATION, DstX: UINT, DstY: UINT, DstZ: UINT, pSrc: *mut _In_ const D3D12_TEXTURE_COPY_LOCATION, pSrcBox: *mut _In_opt_ const D3D12_BOX) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar CopyTextureRegion desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OMSetBlendFactor(param_31384: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar OMSetBlendFactor desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OMSetStencilRef(StencilRef: _In_ UINT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar OMSetStencilRef desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetPipelineState(pPipelineState: *mut _In_ ID3D12PipelineState) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetPipelineState desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ResourceBarrier(NumBarriers: _In_ UINT, param_52953: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ResourceBarrier desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExecuteBundle(pCommandList: *mut _In_ ID3D12GraphicsCommandList) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ExecuteBundle desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetDescriptorHeaps(NumDescriptorHeaps: _In_ UINT, param_42388: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetDescriptorHeaps desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetComputeRootSignature(pRootSignature: *mut _In_opt_ ID3D12RootSignature) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetComputeRootSignature desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetGraphicsRootSignature(pRootSignature: *mut _In_opt_ ID3D12RootSignature) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetGraphicsRootSignature desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetComputeRootDescriptorTable(RootParameterIndex: _In_ UINT, BaseDescriptor: _In_ D3D12_GPU_DESCRIPTOR_HANDLE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetComputeRootDescriptorTable desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetGraphicsRootDescriptorTable(RootParameterIndex: _In_ UINT, BaseDescriptor: _In_ D3D12_GPU_DESCRIPTOR_HANDLE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetGraphicsRootDescriptorTable desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetComputeRoot32BitConstant(RootParameterIndex: _In_ UINT, SrcData: _In_ UINT, DestOffsetIn32BitValues: _In_ UINT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetComputeRoot32BitConstant desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetGraphicsRoot32BitConstant(RootParameterIndex: _In_ UINT, SrcData: _In_ UINT, DestOffsetIn32BitValues: _In_ UINT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetGraphicsRoot32BitConstant desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetComputeRoot32BitConstants(RootParameterIndex: _In_ UINT, Num32BitValuesToSet: _In_ UINT, param_65393: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetComputeRoot32BitConstants desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetGraphicsRoot32BitConstants(RootParameterIndex: _In_ UINT, Num32BitValuesToSet: _In_ UINT, param_65393: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetGraphicsRoot32BitConstants desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetComputeRootConstantBufferView(RootParameterIndex: _In_ UINT, BufferLocation: _In_ D3D12_GPU_VIRTUAL_ADDRESS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetComputeRootConstantBufferView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetGraphicsRootConstantBufferView(RootParameterIndex: _In_ UINT, BufferLocation: _In_ D3D12_GPU_VIRTUAL_ADDRESS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetGraphicsRootConstantBufferView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetComputeRootShaderResourceView(RootParameterIndex: _In_ UINT, BufferLocation: _In_ D3D12_GPU_VIRTUAL_ADDRESS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetComputeRootShaderResourceView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetGraphicsRootShaderResourceView(RootParameterIndex: _In_ UINT, BufferLocation: _In_ D3D12_GPU_VIRTUAL_ADDRESS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetGraphicsRootShaderResourceView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetComputeRootUnorderedAccessView(RootParameterIndex: _In_ UINT, BufferLocation: _In_ D3D12_GPU_VIRTUAL_ADDRESS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetComputeRootUnorderedAccessView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetGraphicsRootUnorderedAccessView(RootParameterIndex: _In_ UINT, BufferLocation: _In_ D3D12_GPU_VIRTUAL_ADDRESS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetGraphicsRootUnorderedAccessView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BeginQuery(pQueryHeap: *mut _In_ ID3D12QueryHeap, Type: _In_ D3D12_QUERY_TYPE, Index: _In_ UINT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar BeginQuery desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EndQuery(pQueryHeap: *mut _In_ ID3D12QueryHeap, Type: _In_ D3D12_QUERY_TYPE, Index: _In_ UINT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar EndQuery desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ResolveQueryData(pQueryHeap: *mut _In_ ID3D12QueryHeap, Type: _In_ D3D12_QUERY_TYPE, StartIndex: _In_ UINT, NumQueries: _In_ UINT, pDestinationBuffer: *mut _In_ ID3D12Resource, AlignedDestinationBufferOffset: _In_ UINT64) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ResolveQueryData desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OMSetDepthBounds(Min: _In_ FLOAT, Max: _In_ FLOAT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar OMSetDepthBounds desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetSamplePositions(NumSamplesPerPixel: _In_ UINT, NumPixels: _In_ UINT, param_3020: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetSamplePositions desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ResolveSubresourceRegion(pDstResource: *mut _In_ ID3D12Resource, DstSubresource: _In_ UINT, DstX: _In_ UINT, DstY: _In_ UINT, pSrcResource: *mut _In_ ID3D12Resource, SrcSubresource: _In_ UINT, pSrcRect: *mut _In_opt_ D3D12_RECT, Format: _In_ DXGI_FORMAT, ResolveMode: _In_ D3D12_RESOLVE_MODE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ResolveSubresourceRegion desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetViewInstanceMask(Mask: _In_ UINT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetViewInstanceMask desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExecuteCommandLists(NumCommandLists: _In_ UINT, param_34086: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ExecuteCommandLists desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetTimestampFrequency(pFrequency: *mut _Out_ UINT64) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetTimestampFrequency desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetClockCalibration(pGpuTimestamp: *mut _Out_ UINT64, pCpuTimestamp: *mut _Out_ UINT64) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetClockCalibration desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetProcessPriority(Priority: D3D12_COMMAND_QUEUE_PROCESS_PRIORITY) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetProcessPriority desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetProcessPriority(pOutValue: *mut D3D12_COMMAND_QUEUE_PROCESS_PRIORITY) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetProcessPriority desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetGlobalPriority(Priority: D3D12_COMMAND_QUEUE_GLOBAL_PRIORITY) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetGlobalPriority desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetGlobalPriority(pOutValue: *mut D3D12_COMMAND_QUEUE_GLOBAL_PRIORITY) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetGlobalPriority desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNodeCount() -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetNodeCount desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCommandQueue(pDesc: *mut _In_ const D3D12_COMMAND_QUEUE_DESC, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateCommandQueue desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCommandAllocator(type: _In_ D3D12_COMMAND_LIST_TYPE, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateCommandAllocator desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateGraphicsPipelineState(pDesc: *mut _In_ const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateGraphicsPipelineState desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateComputePipelineState(pDesc: *mut _In_ const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateComputePipelineState desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCommandList(nodeMask: _In_ UINT, type: _In_ D3D12_COMMAND_LIST_TYPE, pCommandAllocator: *mut _In_ ID3D12CommandAllocator, pInitialState: *mut _In_opt_ ID3D12PipelineState, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateCommandList desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDescriptorHeap(pDescriptorHeapDesc: *mut _In_ const D3D12_DESCRIPTOR_HEAP_DESC, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateDescriptorHeap desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetDescriptorHandleIncrementSize(DescriptorHeapType: _In_ D3D12_DESCRIPTOR_HEAP_TYPE) -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetDescriptorHandleIncrementSize desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateConstantBufferView(pDesc: *mut _In_opt_ const D3D12_CONSTANT_BUFFER_VIEW_DESC, DestDescriptor: _In_ D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar CreateConstantBufferView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSampler(pDesc: *mut _In_ const D3D12_SAMPLER_DESC, DestDescriptor: _In_ D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar CreateSampler desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CopyDescriptorsSimple(NumDescriptors: _In_ UINT, DestDescriptorRangeStart: _In_ D3D12_CPU_DESCRIPTOR_HANDLE, SrcDescriptorRangeStart: _In_ D3D12_CPU_DESCRIPTOR_HANDLE, DescriptorHeapsType: _In_ D3D12_DESCRIPTOR_HEAP_TYPE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar CopyDescriptorsSimple desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetResourceAllocationInfo(visibleMask: _In_ UINT, numResourceDescs: _In_ UINT, param_12607: *mut core::ffi::c_void) -> virtual D3D12_RESOURCE_ALLOCATION_INFO STDMETHODCALLTYPE {
    // TODO: implementar GetResourceAllocationInfo desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetCustomHeapProperties(nodeMask: _In_ UINT, heapType: D3D12_HEAP_TYPE) -> virtual D3D12_HEAP_PROPERTIES STDMETHODCALLTYPE {
    // TODO: implementar GetCustomHeapProperties desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCommittedResource(pHeapProperties: *mut _In_ const D3D12_HEAP_PROPERTIES, HeapFlags: D3D12_HEAP_FLAGS, pDesc: *mut _In_ const D3D12_RESOURCE_DESC, InitialResourceState: D3D12_RESOURCE_STATES, pOptimizedClearValue: *mut _In_opt_ const D3D12_CLEAR_VALUE, riidResource: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateCommittedResource desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateHeap(pDesc: *mut _In_ const D3D12_HEAP_DESC, riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateHeap desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreatePlacedResource(pHeap: *mut _In_ ID3D12Heap, HeapOffset: UINT64, pDesc: *mut _In_ const D3D12_RESOURCE_DESC, InitialState: D3D12_RESOURCE_STATES, pOptimizedClearValue: *mut _In_opt_ const D3D12_CLEAR_VALUE, riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreatePlacedResource desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateReservedResource(pDesc: *mut _In_ const D3D12_RESOURCE_DESC, InitialState: D3D12_RESOURCE_STATES, pOptimizedClearValue: *mut _In_opt_ const D3D12_CLEAR_VALUE, riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateReservedResource desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OpenSharedHandle(NTHandle: _In_ HANDLE, riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar OpenSharedHandle desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OpenSharedHandleByName(Name: _In_ LPCWSTR, Access: u32, pNTHandle: *mut _Out_ HANDLE) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar OpenSharedHandleByName desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MakeResident(NumObjects: UINT, param_6011: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar MakeResident desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Evict(NumObjects: UINT, param_6011: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar Evict desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateQueryHeap(pDesc: *mut _In_ const D3D12_QUERY_HEAP_DESC, riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateQueryHeap desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetStablePowerState(Enable: i32) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetStablePowerState desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCommandSignature(pDesc: *mut _In_ const D3D12_COMMAND_SIGNATURE_DESC, pRootSignature: *mut _In_opt_ ID3D12RootSignature, riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateCommandSignature desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetAdapterLuid() -> virtual LUID STDMETHODCALLTYPE {
    // TODO: implementar GetAdapterLuid desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StorePipeline(pName: _In_opt_ LPCWSTR, pPipeline: *mut _In_ ID3D12PipelineState) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar StorePipeline desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LoadGraphicsPipeline(pName: _In_ LPCWSTR, pDesc: *mut _In_ const D3D12_GRAPHICS_PIPELINE_STATE_DESC, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar LoadGraphicsPipeline desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LoadComputePipeline(pName: _In_ LPCWSTR, pDesc: *mut _In_ const D3D12_COMPUTE_PIPELINE_STATE_DESC, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar LoadComputePipeline desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Serialize(param_40755: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar Serialize desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LoadPipeline(pName: _In_ LPCWSTR, pDesc: *mut _In_ const D3D12_PIPELINE_STATE_STREAM_DESC, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar LoadPipeline desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreatePipelineState(pDesc: *mut const D3D12_PIPELINE_STATE_STREAM_DESC, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreatePipelineState desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OpenExistingHeapFromAddress(pAddress: *mut _In_ const void, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar OpenExistingHeapFromAddress desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OpenExistingHeapFromFileMapping(hFileMapping: _In_ HANDLE, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar OpenExistingHeapFromFileMapping desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EnqueueMakeResident(Flags: D3D12_RESIDENCY_FLAGS, NumObjects: UINT, param_6011: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar EnqueueMakeResident desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetStatusFence(riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetStatusFence desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetSessionStatus() -> virtual D3D12_PROTECTED_SESSION_STATUS STDMETHODCALLTYPE {
    // TODO: implementar GetSessionStatus desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCommandList1(nodeMask: _In_ UINT, type: _In_ D3D12_COMMAND_LIST_TYPE, flags: _In_ D3D12_COMMAND_LIST_FLAGS, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateCommandList1 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateProtectedResourceSession(pDesc: *mut _In_ const D3D12_PROTECTED_RESOURCE_SESSION_DESC, riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateProtectedResourceSession desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCommittedResource1(pHeapProperties: *mut _In_ const D3D12_HEAP_PROPERTIES, HeapFlags: D3D12_HEAP_FLAGS, pDesc: *mut _In_ const D3D12_RESOURCE_DESC, InitialResourceState: D3D12_RESOURCE_STATES, pOptimizedClearValue: *mut _In_opt_ const D3D12_CLEAR_VALUE, pProtectedSession: *mut _In_opt_ ID3D12ProtectedResourceSession, riidResource: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateCommittedResource1 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateHeap1(pDesc: *mut _In_ const D3D12_HEAP_DESC, pProtectedSession: *mut _In_opt_ ID3D12ProtectedResourceSession, riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateHeap1 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateReservedResource1(pDesc: *mut _In_ const D3D12_RESOURCE_DESC, InitialState: D3D12_RESOURCE_STATES, pOptimizedClearValue: *mut _In_opt_ const D3D12_CLEAR_VALUE, pProtectedSession: *mut _In_opt_ ID3D12ProtectedResourceSession, riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateReservedResource1 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LifetimeStateUpdated(NewState: D3D12_LIFETIME_STATE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar LifetimeStateUpdated desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetLUID() -> virtual LUID STDMETHODCALLTYPE {
    // TODO: implementar GetLUID desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetSwapChainObject(riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetSwapChainObject desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetCurrentResourceAndCommandQueue(riidResource: REFIID, param_43676: *mut _COM_Outptr_ void, riidQueue: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetCurrentResourceAndCommandQueue desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InsertImplicitSync() -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar InsertImplicitSync desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DestroyOwnedObject(pObject: *mut _In_ ID3D12DeviceChild) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar DestroyOwnedObject desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetShaderIdentifier(pExportName: _In_ LPCWSTR) -> virtual void *STDMETHODCALLTYPE {
    // TODO: implementar GetShaderIdentifier desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetShaderStackSize(pExportName: _In_ LPCWSTR) -> virtual UINT64 STDMETHODCALLTYPE {
    // TODO: implementar GetShaderStackSize desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPipelineStackSize() -> virtual UINT64 STDMETHODCALLTYPE {
    // TODO: implementar GetPipelineStackSize desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetPipelineStackSize(PipelineStackSizeInBytes: UINT64) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetPipelineStackSize desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetProgramIdentifier(pProgramName: LPCWSTR) -> virtual D3D12_PROGRAM_IDENTIFIER STDMETHODCALLTYPE {
    // TODO: implementar GetProgramIdentifier desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetGlobalRootSignatureForProgram(pProgramName: LPCWSTR, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetGlobalRootSignatureForProgram desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetGlobalRootSignatureForShader(pExportName: LPCWSTR, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetGlobalRootSignatureForShader desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNumWorkGraphs() -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetNumWorkGraphs desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetProgramName(WorkGraphIndex: UINT) -> virtual LPCWSTR STDMETHODCALLTYPE {
    // TODO: implementar GetProgramName desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetWorkGraphIndex(pProgramName: LPCWSTR) -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetWorkGraphIndex desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNumNodes(WorkGraphIndex: UINT) -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetNumNodes desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNodeID(WorkGraphIndex: UINT, NodeIndex: UINT) -> virtual D3D12_NODE_ID STDMETHODCALLTYPE {
    // TODO: implementar GetNodeID desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNodeIndex(WorkGraphIndex: UINT, NodeID: D3D12_NODE_ID) -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetNodeIndex desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNodeLocalRootArgumentsTableIndex(WorkGraphIndex: UINT, NodeIndex: UINT) -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetNodeLocalRootArgumentsTableIndex desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNumEntrypoints(WorkGraphIndex: UINT) -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetNumEntrypoints desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetEntrypointID(WorkGraphIndex: UINT, EntrypointIndex: UINT) -> virtual D3D12_NODE_ID STDMETHODCALLTYPE {
    // TODO: implementar GetEntrypointID desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetEntrypointIndex(WorkGraphIndex: UINT, NodeID: D3D12_NODE_ID) -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetEntrypointIndex desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetEntrypointRecordSizeInBytes(WorkGraphIndex: UINT, EntrypointIndex: UINT) -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetEntrypointRecordSizeInBytes desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetWorkGraphMemoryRequirements(WorkGraphIndex: UINT, pWorkGraphMemoryRequirements: *mut _Out_ D3D12_WORK_GRAPH_MEMORY_REQUIREMENTS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar GetWorkGraphMemoryRequirements desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetEntrypointRecordAlignmentInBytes(WorkGraphIndex: UINT, EntrypointIndex: UINT) -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetEntrypointRecordAlignmentInBytes desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateLifetimeTracker(pOwner: *mut _In_ ID3D12LifetimeOwner, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateLifetimeTracker desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RemoveDevice() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar RemoveDevice desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EnumerateMetaCommands(pNumMetaCommands: *mut _Inout_ UINT, param_45641: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar EnumerateMetaCommands desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EnumerateMetaCommandParameters(CommandId: _In_ REFGUID, Stage: _In_ D3D12_META_COMMAND_PARAMETER_STAGE, pTotalStructureSizeInBytes: *mut _Out_opt_ UINT, pParameterCount: *mut _Inout_ UINT, param_38147: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar EnumerateMetaCommandParameters desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateStateObject(pDesc: *mut const D3D12_STATE_OBJECT_DESC, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateStateObject desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetRaytracingAccelerationStructurePrebuildInfo(pDesc: *mut _In_ const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pInfo: *mut _Out_ D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar GetRaytracingAccelerationStructurePrebuildInfo desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CheckDriverMatchingIdentifier(SerializedDataType: _In_ D3D12_SERIALIZED_DATA_TYPE, pIdentifierToCheck: *mut _In_ const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> virtual D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS STDMETHODCALLTYPE {
    // TODO: implementar CheckDriverMatchingIdentifier desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetAutoBreadcrumbsEnablement(Enablement: D3D12_DRED_ENABLEMENT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetAutoBreadcrumbsEnablement desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetPageFaultEnablement(Enablement: D3D12_DRED_ENABLEMENT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetPageFaultEnablement desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetWatsonDumpEnablement(Enablement: D3D12_DRED_ENABLEMENT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetWatsonDumpEnablement desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetBreadcrumbContextEnablement(Enablement: D3D12_DRED_ENABLEMENT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetBreadcrumbContextEnablement desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UseMarkersOnlyAutoBreadcrumbs(MarkersOnly: i32) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar UseMarkersOnlyAutoBreadcrumbs desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetAutoBreadcrumbsOutput(pOutput: *mut _Out_ D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetAutoBreadcrumbsOutput desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPageFaultAllocationOutput(pOutput: *mut _Out_ D3D12_DRED_PAGE_FAULT_OUTPUT) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetPageFaultAllocationOutput desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetAutoBreadcrumbsOutput1(pOutput: *mut _Out_ D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetAutoBreadcrumbsOutput1 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPageFaultAllocationOutput1(pOutput: *mut _Out_ D3D12_DRED_PAGE_FAULT_OUTPUT1) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetPageFaultAllocationOutput1 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPageFaultAllocationOutput2(pOutput: *mut _Out_ D3D12_DRED_PAGE_FAULT_OUTPUT2) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetPageFaultAllocationOutput2 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetDeviceState() -> virtual D3D12_DRED_DEVICE_STATE STDMETHODCALLTYPE {
    // TODO: implementar GetDeviceState desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetBackgroundProcessingMode(Mode: D3D12_BACKGROUND_PROCESSING_MODE, MeasurementsAction: D3D12_MEASUREMENTS_ACTION, hEventToSignalUponCompletion: _In_opt_ HANDLE, pbFurtherMeasurementsDesired: *mut _Out_opt_ BOOL) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetBackgroundProcessingMode desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AddToStateObject(pAddition: *mut const D3D12_STATE_OBJECT_DESC, pStateObjectToGrowFrom: *mut ID3D12StateObject, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar AddToStateObject desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateProtectedResourceSession1(pDesc: *mut _In_ const D3D12_PROTECTED_RESOURCE_SESSION_DESC1, riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateProtectedResourceSession1 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCommittedResource2(pHeapProperties: *mut _In_ const D3D12_HEAP_PROPERTIES, HeapFlags: D3D12_HEAP_FLAGS, pDesc: *mut _In_ const D3D12_RESOURCE_DESC1, InitialResourceState: D3D12_RESOURCE_STATES, pOptimizedClearValue: *mut _In_opt_ const D3D12_CLEAR_VALUE, pProtectedSession: *mut _In_opt_ ID3D12ProtectedResourceSession, riidResource: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateCommittedResource2 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreatePlacedResource1(pHeap: *mut _In_ ID3D12Heap, HeapOffset: UINT64, pDesc: *mut _In_ const D3D12_RESOURCE_DESC1, InitialState: D3D12_RESOURCE_STATES, pOptimizedClearValue: *mut _In_opt_ const D3D12_CLEAR_VALUE, riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreatePlacedResource1 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSamplerFeedbackUnorderedAccessView(pTargetedResource: *mut _In_opt_ ID3D12Resource, pFeedbackResource: *mut _In_opt_ ID3D12Resource, DestDescriptor: _In_ D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar CreateSamplerFeedbackUnorderedAccessView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetProtectedResourceSession(riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetProtectedResourceSession desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetProtectedResourceSession(pProtectedResourceSession: *mut _In_opt_ ID3D12ProtectedResourceSession) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetProtectedResourceSession desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetRequiredParameterResourceSize(Stage: _In_ D3D12_META_COMMAND_PARAMETER_STAGE, ParameterIndex: _In_ UINT) -> virtual UINT64 STDMETHODCALLTYPE {
    // TODO: implementar GetRequiredParameterResourceSize desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EndRenderPass() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar EndRenderPass desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExecuteMetaCommand(pMetaCommand: *mut _In_ ID3D12MetaCommand, param_62267: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ExecuteMetaCommand desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BuildRaytracingAccelerationStructure(pDesc: *mut _In_ const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, NumPostbuildInfoDescs: _In_ UINT, param_38757: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar BuildRaytracingAccelerationStructure desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EmitRaytracingAccelerationStructurePostbuildInfo(pDesc: *mut _In_ const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, NumSourceAccelerationStructures: _In_ UINT, NumSourceAccelerationStructures: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar EmitRaytracingAccelerationStructurePostbuildInfo desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CopyRaytracingAccelerationStructure(DestAccelerationStructureData: _In_ D3D12_GPU_VIRTUAL_ADDRESS, SourceAccelerationStructureData: _In_ D3D12_GPU_VIRTUAL_ADDRESS, Mode: _In_ D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar CopyRaytracingAccelerationStructure desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetPipelineState1(pStateObject: *mut _In_ ID3D12StateObject) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetPipelineState1 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DispatchRays(pDesc: *mut _In_ const D3D12_DISPATCH_RAYS_DESC) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar DispatchRays desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetDeleteOnDestroy() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetDeleteOnDestroy desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateShaderCacheSession(pDesc: *mut _In_ const D3D12_SHADER_CACHE_SESSION_DESC, riid: REFIID, param_14394: *mut _COM_Outptr_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateShaderCacheSession desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ShaderCacheControl(Kinds: D3D12_SHADER_CACHE_KIND_FLAGS, Control: D3D12_SHADER_CACHE_CONTROL_FLAGS) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar ShaderCacheControl desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCommandQueue1(pDesc: *mut _In_ const D3D12_COMMAND_QUEUE_DESC, CreatorID: REFIID, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateCommandQueue1 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateCommittedResource3(pHeapProperties: *mut _In_ const D3D12_HEAP_PROPERTIES, HeapFlags: D3D12_HEAP_FLAGS, pDesc: *mut _In_ const D3D12_RESOURCE_DESC1, InitialLayout: D3D12_BARRIER_LAYOUT, pOptimizedClearValue: *mut _In_opt_ const D3D12_CLEAR_VALUE, pProtectedSession: *mut _In_opt_ ID3D12ProtectedResourceSession, NumCastableFormats: UINT32, param_18174: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateCommittedResource3 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreatePlacedResource2(pHeap: *mut _In_ ID3D12Heap, HeapOffset: UINT64, pDesc: *mut _In_ const D3D12_RESOURCE_DESC1, InitialLayout: D3D12_BARRIER_LAYOUT, pOptimizedClearValue: *mut _In_opt_ const D3D12_CLEAR_VALUE, NumCastableFormats: UINT32, param_18174: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreatePlacedResource2 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateReservedResource2(pDesc: *mut _In_ const D3D12_RESOURCE_DESC, InitialLayout: D3D12_BARRIER_LAYOUT, pOptimizedClearValue: *mut _In_opt_ const D3D12_CLEAR_VALUE, pProtectedSession: *mut _In_opt_ ID3D12ProtectedResourceSession, NumCastableFormats: UINT32, param_18174: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateReservedResource2 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateSampler2(pDesc: *mut _In_ const D3D12_SAMPLER_DESC2, DestDescriptor: _In_ D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar CreateSampler2 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OpenExistingHeapFromAddress1(pAddress: *mut _In_ const void, size: SIZE_T, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar OpenExistingHeapFromAddress1 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RegisterTrimNotificationCallback(pData: *mut _Inout_ D3D12_REGISTER_TRIM_NOTIFICATION) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar RegisterTrimNotificationCallback desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnregisterTrimNotificationCallback(CallbackCookie: u32) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar UnregisterTrimNotificationCallback desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TryCreateShaderResourceView(pResource: *mut _In_opt_ ID3D12Resource, pDesc: *mut _In_opt_ const D3D12_SHADER_RESOURCE_VIEW_DESC, DestDescriptor: _In_ D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar TryCreateShaderResourceView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TryCreateUnorderedAccessView(pResource: *mut _In_opt_ ID3D12Resource, pCounterResource: *mut _In_opt_ ID3D12Resource, pDesc: *mut _In_opt_ const D3D12_UNORDERED_ACCESS_VIEW_DESC, DestDescriptor: _In_ D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar TryCreateUnorderedAccessView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TryCreateConstantBufferView(pDesc: *mut _In_opt_ const D3D12_CONSTANT_BUFFER_VIEW_DESC, DestDescriptor: _In_ D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar TryCreateConstantBufferView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TryCreateSampler2(pDesc: *mut _In_ const D3D12_SAMPLER_DESC2, DestDescriptor: _In_ D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar TryCreateSampler2 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TryCreateRenderTargetView(pResource: *mut _In_opt_ ID3D12Resource, pDesc: *mut _In_opt_ const D3D12_RENDER_TARGET_VIEW_DESC, DestDescriptor: _In_ D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar TryCreateRenderTargetView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TryCreateDepthStencilView(pResource: *mut _In_opt_ ID3D12Resource, pDesc: *mut _In_opt_ const D3D12_DEPTH_STENCIL_VIEW_DESC, DestDescriptor: _In_ D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar TryCreateDepthStencilView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TryCreateSamplerFeedbackUnorderedAccessView(pTargetedResource: *mut _In_opt_ ID3D12Resource, pFeedbackResource: *mut _In_opt_ ID3D12Resource, DestDescriptor: _In_ D3D12_CPU_DESCRIPTOR_HANDLE) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar TryCreateSamplerFeedbackUnorderedAccessView desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateQueryHeap1(pDesc: *mut _In_ const D3D12_QUERY_HEAP_DESC, Flags: _In_ D3D12_QUERY_HEAP_FLAGS, riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateQueryHeap1 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetApplicationDesc(pApplicationDesc: *mut _In_ const D3D12_APPLICATION_DESC) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetApplicationDesc desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetApplicationDesc(CallbackFunc: _In_ D3D12ApplicationDescFunc, pContext: *mut _Inout_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetApplicationDesc desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FindStateObjectDesc(param_5011: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar FindStateObjectDesc desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FindObjectVersion(param_5011: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar FindObjectVersion desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ShareWithHost(pObject: *mut _In_ ID3D12DeviceChild, pHandle: *mut _Out_ HANDLE) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar ShareWithHost desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateFenceFd(pFence: *mut _In_ ID3D12Fence, FenceValue: UINT64, pFenceFd: *mut _Out_ int) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateFenceFd desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EnableShaderInstrumentation(bEnable: i32) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar EnableShaderInstrumentation desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ShaderInstrumentationEnabled() -> virtual BOOL STDMETHODCALLTYPE {
    // TODO: implementar ShaderInstrumentationEnabled desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ReserveGPUVARangesAtCreate(param_64360: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar ReserveGPUVARangesAtCreate desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ClearReservedGPUVARangesList() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ClearReservedGPUVARangesList desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetApplicationSpecificDriverState(pAdapter: *mut _In_ IUnknown, pBlob: *mut _In_opt_ ID3DBlob) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetApplicationSpecificDriverState desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DisableFailuresFromStricterValidationInAppLocalRuntime(bDisable: i32) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar DisableFailuresFromStricterValidationInAppLocalRuntime desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FailuresFromStricterValidationInAppLocalRuntimeDisabled() -> virtual BOOL STDMETHODCALLTYPE {
    // TODO: implementar FailuresFromStricterValidationInAppLocalRuntimeDisabled desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetAllocation(pAllocation: *mut _Inout_ D3D12_GPU_VIRTUAL_ADDRESS_RANGE) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetAllocation desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetNextAllocationAddress(nextAllocationVirtualAddress: _In_ D3D12_GPU_VIRTUAL_ADDRESS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetNextAllocationAddress desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetApplicationSpecificDriverState(param_34927: *mut _COM_Outptr_ ID3DBlob) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetApplicationSpecificDriverState desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetApplicationSpecificDriverBlobStatus() -> virtual D3D12_APPLICATION_SPECIFIC_DRIVER_BLOB_STATUS STDMETHODCALLTYPE {
    // TODO: implementar GetApplicationSpecificDriverBlobStatus desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3D12CreateDevice(pAdapter: *mut _In_opt_ IUnknown, MinimumFeatureLevel: D3D_FEATURE_LEVEL, riid: _In_ REFIID, ppDevice: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar D3D12CreateDevice desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetSDKVersion(SDKVersion: UINT, SDKPath: _In_z_ LPCSTR) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetSDKVersion desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDeviceFactory(SDKVersion: UINT, SDKPath: _In_ LPCSTR, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateDeviceFactory desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FreeUnusedSDKs() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar FreeUnusedSDKs desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitializeFromGlobalState() -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar InitializeFromGlobalState desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ApplyToGlobalState() -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar ApplyToGlobalState desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetFlags(flags: D3D12_DEVICE_FACTORY_FLAGS) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetFlags desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetConfigurationInterface(clsid: REFCLSID, iid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetConfigurationInterface desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetEnabledExperimentalFeatures(param_63279: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetEnabledExperimentalFeatures desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SerializeVersionedRootSignature(pDesc: *mut _In_ const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, param_34927: *mut _COM_Outptr_ ID3DBlob, param_23427: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SerializeVersionedRootSignature desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateVersionedRootSignatureDeserializer(param_64661: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateVersionedRootSignatureDeserializer desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateStateObjectDatabaseFromFile(pDatabaseFile: LPCWSTR, flags: D3D12_STATE_OBJECT_DATABASE_FLAGS, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateStateObjectDatabaseFromFile desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetApplicationIdentity(pDesc: *mut const D3D12_APPLICATION_DESC, AppId: REFGUID) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetApplicationIdentity desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RSSetShadingRate(baseShadingRate: _In_ D3D12_SHADING_RATE, param_10725: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar RSSetShadingRate desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RSSetShadingRateImage(shadingRateImage: *mut _In_opt_ ID3D12Resource) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar RSSetShadingRateImage desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DispatchMesh(ThreadGroupCountX: _In_ UINT, ThreadGroupCountY: _In_ UINT, ThreadGroupCountZ: _In_ UINT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar DispatchMesh desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Barrier(NumBarrierGroups: UINT32, param_48171: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar Barrier desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OMSetFrontAndBackStencilRef(FrontStencilRef: _In_ UINT, BackStencilRef: _In_ UINT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar OMSetFrontAndBackStencilRef desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RSSetDepthBias(DepthBias: _In_ FLOAT, DepthBiasClamp: _In_ FLOAT, SlopeScaledDepthBias: _In_ FLOAT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar RSSetDepthBias desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IASetIndexBufferStripCutValue(IBStripCutValue: _In_ D3D12_INDEX_BUFFER_STRIP_CUT_VALUE) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar IASetIndexBufferStripCutValue desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetProgram(pDesc: *mut _In_ const D3D12_SET_PROGRAM_DESC) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetProgram desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DispatchGraph(pDesc: *mut _In_ const D3D12_DISPATCH_GRAPH_DESC) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar DispatchGraph desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDSRDevice(pD3D12Device: *mut ID3D12Device, NodeMask: UINT, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateDSRDevice desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetGBVEntireSubresourceStatesData(pResource: *mut _In_ ID3D12Resource, param_45140: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetGBVEntireSubresourceStatesData desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetGBVSubresourceState(pResource: *mut _In_ ID3D12Resource, Subresource: UINT, pData: *mut _Out_ int) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetGBVSubresourceState desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetGBVResourceUniformState(pResource: *mut _In_ ID3D12Resource, pData: *mut _Out_ int) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetGBVResourceUniformState desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetGBVResourceInfo(pResource: *mut _In_ ID3D12Resource, pResourceDesc: *mut _In_opt_ D3D12_RESOURCE_DESC, pResourceHash: *mut _In_opt_ UINT32, pSubresourceStatesByteOffset: *mut _In_opt_ UINT32) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetGBVResourceInfo desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GBVReserved0() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar GBVReserved0 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GBVReserved1() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar GBVReserved1 desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetStateObjectStatistics(pStatistics: *mut _Out_ D3D12_STATE_OBJECT_STATISTICS) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetStateObjectStatistics desde DirectX-Headers/d3d12.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ReflectSharedProperties(pHeapOrResource: *mut _In_ ID3D12Object, ReflectType: D3D12_REFLECT_SHARED_PROPERTY, param_45140: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar ReflectSharedProperties desde DirectX-Headers/d3d12compatibility.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3D12CompilerSerializeVersionedRootSignature(pRootSignature: *mut _In_ const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, ppBlob: *mut core::ffi::c_void, param_23427: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar D3D12CompilerSerializeVersionedRootSignature desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FindGroup(pGroupKey: *mut _In_ const D3D12_COMPILER_CACHE_GROUP_KEY, pGroupVersion: *mut _Out_opt_ UINT) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar FindGroup desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FindGroupValueKeys(pGroupKey: *mut _In_ const D3D12_COMPILER_CACHE_GROUP_KEY, pExpectedGroupVersion: *mut _In_opt_ const UINT, CallbackFunc: _In_ D3D12CompilerCacheSessionGroupValueKeysFunc, pContext: *mut _Inout_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar FindGroupValueKeys desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FindGroupValues(pGroupKey: *mut _In_ const D3D12_COMPILER_CACHE_GROUP_KEY, pExpectedGroupVersion: *mut _In_opt_ const UINT, ValueTypeFlags: D3D12_COMPILER_VALUE_TYPE_FLAGS, CallbackFunc: _In_opt_ D3D12CompilerCacheSessionGroupValuesFunc, pContext: *mut _Inout_opt_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar FindGroupValues desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetCompilerTarget() -> virtual D3D12_COMPILER_TARGET STDMETHODCALLTYPE {
    // TODO: implementar GetCompilerTarget desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetValueTypes() -> virtual D3D12_COMPILER_VALUE_TYPE_FLAGS STDMETHODCALLTYPE {
    // TODO: implementar GetValueTypes desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StoreGroupValueKeys(pGroupKey: *mut _In_ const D3D12_COMPILER_CACHE_GROUP_KEY, GroupVersion: UINT, param_52518: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar StoreGroupValueKeys desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StoreValue(pValueKey: *mut _In_ const D3D12_COMPILER_CACHE_VALUE_KEY, param_54700: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar StoreValue desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetCompiler(riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetCompiler desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CompilePipelineState(pGroupKey: *mut _In_ const D3D12_COMPILER_CACHE_GROUP_KEY, GroupVersion: UINT, pDesc: *mut _In_ const D3D12_PIPELINE_STATE_STREAM_DESC) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CompilePipelineState desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CompileStateObject(pGroupKey: *mut _In_ const D3D12_COMPILER_CACHE_GROUP_KEY, GroupVersion: UINT, pDesc: *mut _In_ const D3D12_STATE_OBJECT_DESC, riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CompileStateObject desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CompileAddToStateObject(pGroupKey: *mut _In_ const D3D12_COMPILER_CACHE_GROUP_KEY, GroupVersion: UINT, pAddition: *mut _In_ const D3D12_STATE_OBJECT_DESC, pCompilerStateObjectToGrowFrom: *mut _In_ ID3D12CompilerStateObject, riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CompileAddToStateObject desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetCacheSession(riid: _In_ REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetCacheSession desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EnumerateAdapterFamilies(AdapterFamilyIndex: UINT, pAdapterFamily: *mut _Out_ D3D12_ADAPTER_FAMILY) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar EnumerateAdapterFamilies desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EnumerateAdapterFamilyABIVersions(AdapterFamilyIndex: UINT, pNumABIVersions: *mut _Inout_ UINT32, param_49153: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar EnumerateAdapterFamilyABIVersions desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EnumerateAdapterFamilyCompilerVersion(AdapterFamilyIndex: UINT, pCompilerVersion: *mut _Out_ D3D12_VERSION_NUMBER) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar EnumerateAdapterFamilyCompilerVersion desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetApplicationProfileVersion(pTarget: *mut _In_ const D3D12_COMPILER_TARGET, pApplicationDesc: *mut _In_ const D3D12_APPLICATION_DESC, pApplicationProfileVersion: *mut _Out_ D3D12_VERSION_NUMBER) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetApplicationProfileVersion desde DirectX-Headers/d3d12compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EnableDebugLayer() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar EnableDebugLayer desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetEnableGPUBasedValidation(Enable: i32) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetEnableGPUBasedValidation desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetEnableSynchronizedCommandQueueValidation(Enable: i32) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetEnableSynchronizedCommandQueueValidation desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetGPUBasedValidationFlags(Flags: D3D12_GPU_BASED_VALIDATION_FLAGS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetGPUBasedValidationFlags desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DisableDebugLayer() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar DisableDebugLayer desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetEnableAutoName(Enable: i32) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetEnableAutoName desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetForceLegacyBarrierValidation(Enable: i32) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetForceLegacyBarrierValidation desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetDebugParameter(Type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, param_10742: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetDebugParameter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetDebugParameter(Type: D3D12_DEBUG_DEVICE_PARAMETER_TYPE, param_45140: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetDebugParameter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ReportLiveDeviceObjects(Flags: D3D12_RLDO_FLAGS) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar ReportLiveDeviceObjects desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetFeatureMask(Mask: D3D12_DEBUG_FEATURE) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetFeatureMask desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetFeatureMask() -> virtual D3D12_DEBUG_FEATURE STDMETHODCALLTYPE {
    // TODO: implementar GetFeatureMask desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AssertResourceState(pResource: *mut _In_ ID3D12Resource, Subresource: UINT, State: UINT) -> virtual BOOL STDMETHODCALLTYPE {
    // TODO: implementar AssertResourceState desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AssertResourceAccess(pResource: *mut _In_ ID3D12Resource, Subresource: UINT, Access: D3D12_BARRIER_ACCESS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar AssertResourceAccess desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AssertTextureLayout(pResource: *mut _In_ ID3D12Resource, Subresource: UINT, Layout: D3D12_BARRIER_LAYOUT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar AssertTextureLayout desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SharedFenceSignal(pFence: *mut _In_ ID3D12Fence, FenceValue: UINT64) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SharedFenceSignal desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BeginCapturableWork(guid: _In_ REFGUID) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar BeginCapturableWork desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EndCapturableWork(guid: _In_ REFGUID) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar EndCapturableWork desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetMessageCountLimit(MessageCountLimit: _In_ UINT64) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetMessageCountLimit desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ClearStoredMessages() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ClearStoredMessages desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNumMessagesAllowedByStorageFilter() -> virtual UINT64 STDMETHODCALLTYPE {
    // TODO: implementar GetNumMessagesAllowedByStorageFilter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNumMessagesDeniedByStorageFilter() -> virtual UINT64 STDMETHODCALLTYPE {
    // TODO: implementar GetNumMessagesDeniedByStorageFilter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNumStoredMessages() -> virtual UINT64 STDMETHODCALLTYPE {
    // TODO: implementar GetNumStoredMessages desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNumStoredMessagesAllowedByRetrievalFilter() -> virtual UINT64 STDMETHODCALLTYPE {
    // TODO: implementar GetNumStoredMessagesAllowedByRetrievalFilter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNumMessagesDiscardedByMessageCountLimit() -> virtual UINT64 STDMETHODCALLTYPE {
    // TODO: implementar GetNumMessagesDiscardedByMessageCountLimit desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AddStorageFilterEntries(pFilter: *mut _In_ D3D12_INFO_QUEUE_FILTER) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar AddStorageFilterEntries desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetStorageFilter(param_22138: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetStorageFilter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ClearStorageFilter() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ClearStorageFilter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PushEmptyStorageFilter() -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar PushEmptyStorageFilter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PushCopyOfStorageFilter() -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar PushCopyOfStorageFilter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PushStorageFilter(pFilter: *mut _In_ D3D12_INFO_QUEUE_FILTER) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar PushStorageFilter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PopStorageFilter() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar PopStorageFilter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetStorageFilterStackSize() -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetStorageFilterStackSize desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AddRetrievalFilterEntries(pFilter: *mut _In_ D3D12_INFO_QUEUE_FILTER) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar AddRetrievalFilterEntries desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetRetrievalFilter(param_22138: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetRetrievalFilter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ClearRetrievalFilter() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ClearRetrievalFilter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PushEmptyRetrievalFilter() -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar PushEmptyRetrievalFilter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PushCopyOfRetrievalFilter() -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar PushCopyOfRetrievalFilter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PushRetrievalFilter(pFilter: *mut _In_ D3D12_INFO_QUEUE_FILTER) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar PushRetrievalFilter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PopRetrievalFilter() -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar PopRetrievalFilter desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetRetrievalFilterStackSize() -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetRetrievalFilterStackSize desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AddMessage(Category: _In_ D3D12_MESSAGE_CATEGORY, Severity: _In_ D3D12_MESSAGE_SEVERITY, ID: _In_ D3D12_MESSAGE_ID, pDescription: _In_ LPCSTR) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar AddMessage desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AddApplicationMessage(Severity: _In_ D3D12_MESSAGE_SEVERITY, pDescription: _In_ LPCSTR) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar AddApplicationMessage desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetBreakOnCategory(Category: _In_ D3D12_MESSAGE_CATEGORY, bEnable: _In_ BOOL) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetBreakOnCategory desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetBreakOnSeverity(Severity: _In_ D3D12_MESSAGE_SEVERITY, bEnable: _In_ BOOL) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetBreakOnSeverity desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetBreakOnID(ID: _In_ D3D12_MESSAGE_ID, bEnable: _In_ BOOL) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetBreakOnID desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetBreakOnCategory(Category: _In_ D3D12_MESSAGE_CATEGORY) -> virtual BOOL STDMETHODCALLTYPE {
    // TODO: implementar GetBreakOnCategory desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetBreakOnSeverity(Severity: _In_ D3D12_MESSAGE_SEVERITY) -> virtual BOOL STDMETHODCALLTYPE {
    // TODO: implementar GetBreakOnSeverity desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetBreakOnID(ID: _In_ D3D12_MESSAGE_ID) -> virtual BOOL STDMETHODCALLTYPE {
    // TODO: implementar GetBreakOnID desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetMuteDebugOutput(bMute: _In_ BOOL) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar SetMuteDebugOutput desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetMuteDebugOutput() -> virtual BOOL STDMETHODCALLTYPE {
    // TODO: implementar GetMuteDebugOutput desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RegisterMessageCallback(CallbackFunc: _In_ D3D12MessageFunc, CallbackFilterFlags: _In_ D3D12_MESSAGE__FLAGS, pContext: *mut _Inout_ void, pCallbackCookie: *mut _Inout_ DWORD) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar RegisterMessageCallback desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnregisterMessageCallback(CallbackCookie: _In_ DWORD) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar UnregisterMessageCallback desde DirectX-Headers/d3d12sdklayers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNodeMask() -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetNodeMask desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNumInputStreamDescs() -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetNumInputStreamDescs desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetInputStreamDescs(NumInputStreamDescs: UINT, param_37706: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetInputStreamDescs desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetOutputStreamDesc() -> virtual D3D12_VIDEO_PROCESS_OUTPUT_STREAM_DESC STDMETHODCALLTYPE {
    // TODO: implementar GetOutputStreamDesc desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DecodeFrame(pDecoder: *mut _In_ ID3D12VideoDecoder, pOutputArguments: *mut _In_ const D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS, pInputArguments: *mut _In_ const D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar DecodeFrame desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ProcessFrames(pVideoProcessor: *mut _In_ ID3D12VideoProcessor, pOutputArguments: *mut _In_ const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS, NumInputStreams: UINT, param_33674: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ProcessFrames desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DecodeFrame1(pDecoder: *mut _In_ ID3D12VideoDecoder, pOutputArguments: *mut _In_ const D3D12_VIDEO_DECODE_OUTPUT_STREAM_ARGUMENTS1, pInputArguments: *mut _In_ const D3D12_VIDEO_DECODE_INPUT_STREAM_ARGUMENTS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar DecodeFrame1 desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ProcessFrames1(pVideoProcessor: *mut _In_ ID3D12VideoProcessor, pOutputArguments: *mut _In_ const D3D12_VIDEO_PROCESS_OUTPUT_STREAM_ARGUMENTS, NumInputStreams: UINT, param_33674: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ProcessFrames1 desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EstimateMotion(pMotionEstimator: *mut _In_ ID3D12VideoMotionEstimator, pOutputArguments: *mut _In_ const D3D12_VIDEO_MOTION_ESTIMATOR_OUTPUT, pInputArguments: *mut _In_ const D3D12_VIDEO_MOTION_ESTIMATOR_INPUT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar EstimateMotion desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ResolveMotionVectorHeap(pOutputArguments: *mut const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_OUTPUT, pInputArguments: *mut const D3D12_RESOLVE_VIDEO_MOTION_VECTOR_HEAP_INPUT) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ResolveMotionVectorHeap desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitializeExtensionCommand(pExtensionCommand: *mut _In_ ID3D12VideoExtensionCommand, param_55797: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar InitializeExtensionCommand desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExecuteExtensionCommand(pExtensionCommand: *mut _In_ ID3D12VideoExtensionCommand, param_669: *mut core::ffi::c_void) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ExecuteExtensionCommand desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetEncoderFlags() -> virtual D3D12_VIDEO_ENCODER_FLAGS STDMETHODCALLTYPE {
    // TODO: implementar GetEncoderFlags desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetCodec() -> virtual D3D12_VIDEO_ENCODER_CODEC STDMETHODCALLTYPE {
    // TODO: implementar GetCodec desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetCodecProfile(dstProfile: _Inout_ D3D12_VIDEO_ENCODER_PROFILE_DESC) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetCodecProfile desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetCodecConfiguration(dstCodecConfig: _Inout_ D3D12_VIDEO_ENCODER_CODEC_CONFIGURATION) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetCodecConfiguration desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetInputFormat() -> virtual DXGI_FORMAT STDMETHODCALLTYPE {
    // TODO: implementar GetInputFormat desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetMaxMotionEstimationPrecision() -> virtual D3D12_VIDEO_ENCODER_MOTION_ESTIMATION_PRECISION_MODE STDMETHODCALLTYPE {
    // TODO: implementar GetMaxMotionEstimationPrecision desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetEncoderHeapFlags() -> virtual D3D12_VIDEO_ENCODER_HEAP_FLAGS STDMETHODCALLTYPE {
    // TODO: implementar GetEncoderHeapFlags desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetCodecLevel(dstLevel: _Inout_ D3D12_VIDEO_ENCODER_LEVEL_SETTING) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetCodecLevel desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetResolutionListCount() -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetResolutionListCount desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetResolutionList(ResolutionsListCount: const UINT, param_42008: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetResolutionList desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EncodeFrame(pEncoder: *mut _In_ ID3D12VideoEncoder, pHeap: *mut _In_ ID3D12VideoEncoderHeap, pInputArguments: *mut _In_ const D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS, pOutputArguments: *mut _In_ const D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar EncodeFrame desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ResolveEncoderOutputMetadata(pInputArguments: *mut _In_ const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS, pOutputArguments: *mut _In_ const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ResolveEncoderOutputMetadata desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EncodeFrame1(pEncoder: *mut _In_ ID3D12VideoEncoder, pHeap: *mut _In_ ID3D12VideoEncoderHeap1, pInputArguments: *mut _In_ const D3D12_VIDEO_ENCODER_ENCODEFRAME_INPUT_ARGUMENTS1, pOutputArguments: *mut _In_ const D3D12_VIDEO_ENCODER_ENCODEFRAME_OUTPUT_ARGUMENTS1) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar EncodeFrame1 desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ResolveEncoderOutputMetadata1(pInputArguments: *mut _In_ const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_INPUT_ARGUMENTS1, pOutputArguments: *mut _In_ const D3D12_VIDEO_ENCODER_RESOLVE_METADATA_OUTPUT_ARGUMENTS1) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ResolveEncoderOutputMetadata1 desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ResolveInputParamLayout(pInputArguments: *mut _In_ const D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_INPUT_ARGUMENTS, pOutputArguments: *mut _In_ const D3D12_VIDEO_ENCODER_RESOLVE_INPUT_PARAM_LAYOUT_OUTPUT_ARGUMENTS) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar ResolveInputParamLayout desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetBufferPointer() -> virtual LPVOID STDMETHODCALLTYPE {
    // TODO: implementar GetBufferPointer desde DirectX-Headers/d3dcommon.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetBufferSize() -> virtual SIZE_T STDMETHODCALLTYPE {
    // TODO: implementar GetBufferSize desde DirectX-Headers/d3dcommon.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetInstallerName(pNameLength: *mut _Inout_ SIZE_T, param_8901: *mut core::ffi::c_void) -> BEGIN_INTERFACE virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetInstallerName desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetInstallerScope() -> virtual D3D_SHADER_CACHE_APP_REGISTRATION_SCOPE STDMETHODCALLTYPE {
    // TODO: implementar GetInstallerScope desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn HandleDriverUpdate(pInstaller: *mut _In_ ID3DShaderCacheInstaller) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar HandleDriverUpdate desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetComponentName(param_27934: *mut _Out_ const wchar_t) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetComponentName desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetStateObjectDatabasePath(param_27934: *mut _Out_ const wchar_t) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetStateObjectDatabasePath desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPrecompiledCachePath(pAdapterFamily: *mut _In_ const wchar_t, param_13530: *mut _Inout_ const wchar_t) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetPrecompiledCachePath desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPrecompiledShaderDatabaseCount() -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetPrecompiledShaderDatabaseCount desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPrecompiledShaderDatabases(ArraySize: UINT, param_40628: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetPrecompiledShaderDatabases desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetExePath(param_27934: *mut _Out_ const wchar_t) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetExePath desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RegisterComponent(pName: *mut _In_ const wchar_t, pStateObjectDBPath: *mut _In_ const wchar_t, NumPSDB: _In_ UINT, param_52532: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar RegisterComponent desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RemoveComponent(pComponent: *mut _In_ ID3DShaderCacheComponent) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar RemoveComponent desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetComponentCount() -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetComponentCount desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetComponent(index: _In_ UINT, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetComponent desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPrecompileTargetCount(flags: D3D_SHADER_CACHE_TARGET_FLAGS) -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetPrecompileTargetCount desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPrecompileTargets(ArraySize: _In_ UINT, param_1977: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetPrecompileTargets desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RegisterDriverUpdateListener() -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar RegisterDriverUpdateListener desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnregisterDriverUpdateListener() -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar UnregisterDriverUpdateListener desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RegisterServiceDriverUpdateTrigger(hServiceHandle: SC_HANDLE) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar RegisterServiceDriverUpdateTrigger desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnregisterServiceDriverUpdateTrigger(hServiceHandle: SC_HANDLE) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar UnregisterServiceDriverUpdateTrigger desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RegisterApplication(pExePath: *mut _In_ const wchar_t, pApplicationDesc: *mut _In_ const D3D_SHADER_CACHE_APPLICATION_DESC, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar RegisterApplication desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RemoveApplication(pApplication: *mut _In_ ID3DShaderCacheApplication) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar RemoveApplication desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetApplicationCount() -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetApplicationCount desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetApplication(index: _In_ UINT, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetApplication desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ClearAllState() -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar ClearAllState desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetMaxPrecompileTargetCount() -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetMaxPrecompileTargetCount desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetApplicationFromExePath(pFullExePath: *mut _In_ const wchar_t, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetApplicationFromExePath desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Transition(pResource: *mut _In_ ID3D12Resource, stateBefore: D3D12_RESOURCE_STATES, stateAfter: D3D12_RESOURCE_STATES, D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES: UINT subresource =, D3D12_RESOURCE_BARRIER_FLAG_NONE: D3D12_RESOURCE_BARRIER_FLAGS flags =) -> static inline CD3DX12_RESOURCE_BARRIER {
    // TODO: implementar Transition desde DirectX-Headers/d3dx12_barriers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UAV(pResource: *mut _In_opt_ ID3D12Resource) -> static inline CD3DX12_RESOURCE_BARRIER {
    // TODO: implementar UAV desde DirectX-Headers/d3dx12_barriers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DoublePrecisionFloatShaderOps() -> i32 {
    // TODO: implementar DoublePrecisionFloatShaderOps desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MinPrecisionSupport() -> D3D12_SHADER_MIN_PRECISION_SUPPORT {
    // TODO: implementar MinPrecisionSupport desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TiledResourcesTier() -> D3D12_TILED_RESOURCES_TIER {
    // TODO: implementar TiledResourcesTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ResourceBindingTier() -> D3D12_RESOURCE_BINDING_TIER {
    // TODO: implementar ResourceBindingTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PSSpecifiedStencilRefSupported() -> i32 {
    // TODO: implementar PSSpecifiedStencilRefSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TypedUAVLoadAdditionalFormats() -> i32 {
    // TODO: implementar TypedUAVLoadAdditionalFormats desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ROVsSupported() -> i32 {
    // TODO: implementar ROVsSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ConservativeRasterizationTier() -> D3D12_CONSERVATIVE_RASTERIZATION_TIER {
    // TODO: implementar ConservativeRasterizationTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CrossAdapterRowMajorTextureSupported() -> i32 {
    // TODO: implementar CrossAdapterRowMajorTextureSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation() -> i32 {
    // TODO: implementar VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ResourceHeapTier() -> D3D12_RESOURCE_HEAP_TIER {
    // TODO: implementar ResourceHeapTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CrossNodeSharingTier() -> D3D12_CROSS_NODE_SHARING_TIER {
    // TODO: implementar CrossNodeSharingTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MaxGPUVirtualAddressBitsPerResource() -> UINT {
    // TODO: implementar MaxGPUVirtualAddressBitsPerResource desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MaxSupportedFeatureLevel() -> D3D_FEATURE_LEVEL {
    // TODO: implementar MaxSupportedFeatureLevel desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FormatSupport(Format: DXGI_FORMAT, Support1: D3D12_FORMAT_SUPPORT1&, Support2: D3D12_FORMAT_SUPPORT2&) -> i32 {
    // TODO: implementar FormatSupport desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MultisampleQualityLevels(Format: DXGI_FORMAT, SampleCount: UINT, Flags: D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS, NumQualityLevels: UINT&) -> i32 {
    // TODO: implementar MultisampleQualityLevels desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FormatInfo(Format: DXGI_FORMAT, PlaneCount: UINT8&) -> i32 {
    // TODO: implementar FormatInfo desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MaxGPUVirtualAddressBitsPerProcess() -> UINT {
    // TODO: implementar MaxGPUVirtualAddressBitsPerProcess desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn HighestShaderModel() -> D3D_SHADER_MODEL {
    // TODO: implementar HighestShaderModel desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WaveOps() -> i32 {
    // TODO: implementar WaveOps desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WaveLaneCountMin() -> UINT {
    // TODO: implementar WaveLaneCountMin desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WaveLaneCountMax() -> UINT {
    // TODO: implementar WaveLaneCountMax desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TotalLaneCount() -> UINT {
    // TODO: implementar TotalLaneCount desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Int64ShaderOps() -> i32 {
    // TODO: implementar Int64ShaderOps desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ProtectedResourceSessionSupport(param_64659: UINT NodeIndex =) -> D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    // TODO: implementar ProtectedResourceSessionSupport desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn HighestRootSignatureVersion() -> D3D_ROOT_SIGNATURE_VERSION {
    // TODO: implementar HighestRootSignatureVersion desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TileBasedRenderer(param_64659: UINT NodeIndex =) -> i32 {
    // TODO: implementar TileBasedRenderer desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UMA(param_64659: UINT NodeIndex =) -> i32 {
    // TODO: implementar UMA desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CacheCoherentUMA(param_64659: UINT NodeIndex =) -> i32 {
    // TODO: implementar CacheCoherentUMA desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsolatedMMU(param_64659: UINT NodeIndex =) -> i32 {
    // TODO: implementar IsolatedMMU desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DepthBoundsTestSupported() -> i32 {
    // TODO: implementar DepthBoundsTestSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ProgrammableSamplePositionsTier() -> D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER {
    // TODO: implementar ProgrammableSamplePositionsTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ShaderCacheSupportFlags() -> D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    // TODO: implementar ShaderCacheSupportFlags desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CommandQueuePrioritySupported(CommandListType: D3D12_COMMAND_LIST_TYPE, Priority: UINT) -> i32 {
    // TODO: implementar CommandQueuePrioritySupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CopyQueueTimestampQueriesSupported() -> i32 {
    // TODO: implementar CopyQueueTimestampQueriesSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CastingFullyTypedFormatSupported() -> i32 {
    // TODO: implementar CastingFullyTypedFormatSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BarycentricsSupported() -> i32 {
    // TODO: implementar BarycentricsSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExistingHeapsSupported() -> i32 {
    // TODO: implementar ExistingHeapsSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MSAA64KBAlignedTextureSupported() -> i32 {
    // TODO: implementar MSAA64KBAlignedTextureSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SharedResourceCompatibilityTier() -> D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER {
    // TODO: implementar SharedResourceCompatibilityTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Native16BitShaderOpsSupported() -> i32 {
    // TODO: implementar Native16BitShaderOpsSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn HeapSerializationTier(param_64659: UINT NodeIndex =) -> D3D12_HEAP_SERIALIZATION_TIER {
    // TODO: implementar HeapSerializationTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CrossNodeAtomicShaderInstructions() -> i32 {
    // TODO: implementar CrossNodeAtomicShaderInstructions desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SRVOnlyTiledResourceTier3() -> i32 {
    // TODO: implementar SRVOnlyTiledResourceTier3 desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RenderPassesTier() -> D3D12_RENDER_PASS_TIER {
    // TODO: implementar RenderPassesTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RaytracingTier() -> D3D12_RAYTRACING_TIER {
    // TODO: implementar RaytracingTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DisplayableTexture() -> i32 {
    // TODO: implementar DisplayableTexture desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AdditionalShadingRatesSupported() -> i32 {
    // TODO: implementar AdditionalShadingRatesSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PerPrimitiveShadingRateSupportedWithViewportIndexing() -> i32 {
    // TODO: implementar PerPrimitiveShadingRateSupportedWithViewportIndexing desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn VariableShadingRateTier() -> D3D12_VARIABLE_SHADING_RATE_TIER {
    // TODO: implementar VariableShadingRateTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ShadingRateImageTileSize() -> UINT {
    // TODO: implementar ShadingRateImageTileSize desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn QueryMetaCommand(dQueryMetaCommand: D3D12_FEATURE_DATA_QUERY_META_COMMAND&) -> i32 {
    // TODO: implementar QueryMetaCommand desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MeshShaderTier() -> D3D12_MESH_SHADER_TIER {
    // TODO: implementar MeshShaderTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SamplerFeedbackTier() -> D3D12_SAMPLER_FEEDBACK_TIER {
    // TODO: implementar SamplerFeedbackTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ProtectedResourceSessionTypeCount(param_64659: UINT NodeIndex =) -> UINT {
    // TODO: implementar ProtectedResourceSessionTypeCount desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MeshShaderPipelineStatsSupported() -> i32 {
    // TODO: implementar MeshShaderPipelineStatsSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MeshShaderSupportsFullRangeRenderTargetArrayIndex() -> i32 {
    // TODO: implementar MeshShaderSupportsFullRangeRenderTargetArrayIndex desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtomicInt64OnTypedResourceSupported() -> i32 {
    // TODO: implementar AtomicInt64OnTypedResourceSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtomicInt64OnGroupSharedSupported() -> i32 {
    // TODO: implementar AtomicInt64OnGroupSharedSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WaveMMATier() -> D3D12_WAVE_MMA_TIER {
    // TODO: implementar WaveMMATier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn VariableRateShadingSumCombinerSupported() -> i32 {
    // TODO: implementar VariableRateShadingSumCombinerSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MeshShaderPerPrimitiveShadingRateSupported() -> i32 {
    // TODO: implementar MeshShaderPerPrimitiveShadingRateSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtomicInt64OnDescriptorHeapResourceSupported() -> i32 {
    // TODO: implementar AtomicInt64OnDescriptorHeapResourceSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MSPrimitivesPipelineStatisticIncludesCulledPrimitives() -> D3D12_TRI_STATE {
    // TODO: implementar MSPrimitivesPipelineStatisticIncludesCulledPrimitives desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EnhancedBarriersSupported() -> i32 {
    // TODO: implementar EnhancedBarriersSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RelaxedFormatCastingSupported() -> i32 {
    // TODO: implementar RelaxedFormatCastingSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnrestrictedBufferTextureCopyPitchSupported() -> i32 {
    // TODO: implementar UnrestrictedBufferTextureCopyPitchSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnrestrictedVertexElementAlignmentSupported() -> i32 {
    // TODO: implementar UnrestrictedVertexElementAlignmentSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InvertedViewportHeightFlipsYSupported() -> i32 {
    // TODO: implementar InvertedViewportHeightFlipsYSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InvertedViewportDepthFlipsZSupported() -> i32 {
    // TODO: implementar InvertedViewportDepthFlipsZSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TextureCopyBetweenDimensionsSupported() -> i32 {
    // TODO: implementar TextureCopyBetweenDimensionsSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AlphaBlendFactorSupported() -> i32 {
    // TODO: implementar AlphaBlendFactorSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AdvancedTextureOpsSupported() -> i32 {
    // TODO: implementar AdvancedTextureOpsSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TriangleFanSupported() -> i32 {
    // TODO: implementar TriangleFanSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DynamicIndexBufferStripCutSupported() -> i32 {
    // TODO: implementar DynamicIndexBufferStripCutSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DynamicDepthBiasSupported() -> i32 {
    // TODO: implementar DynamicDepthBiasSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GPUUploadHeapSupported() -> i32 {
    // TODO: implementar GPUUploadHeapSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NonNormalizedCoordinateSamplersSupported() -> i32 {
    // TODO: implementar NonNormalizedCoordinateSamplersSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RenderPassesValid() -> i32 {
    // TODO: implementar RenderPassesValid desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MismatchingOutputDimensionsSupported() -> i32 {
    // TODO: implementar MismatchingOutputDimensionsSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SupportedSampleCountsWithNoOutputs() -> UINT {
    // TODO: implementar SupportedSampleCountsWithNoOutputs desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PointSamplingAddressesNeverRoundUp() -> i32 {
    // TODO: implementar PointSamplingAddressesNeverRoundUp desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RasterizerDesc2Supported() -> i32 {
    // TODO: implementar RasterizerDesc2Supported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NarrowQuadrilateralLinesSupported() -> i32 {
    // TODO: implementar NarrowQuadrilateralLinesSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AnisoFilterWithPointMipSupported() -> i32 {
    // TODO: implementar AnisoFilterWithPointMipSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MaxSamplerDescriptorHeapSize() -> UINT {
    // TODO: implementar MaxSamplerDescriptorHeapSize desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MaxSamplerDescriptorHeapSizeWithStaticSamplers() -> UINT {
    // TODO: implementar MaxSamplerDescriptorHeapSizeWithStaticSamplers desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MaxViewDescriptorHeapSize() -> UINT {
    // TODO: implementar MaxViewDescriptorHeapSize desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExecuteIndirectTier() -> D3D12_EXECUTE_INDIRECT_TIER {
    // TODO: implementar ExecuteIndirectTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WorkGraphsTier() -> D3D12_WORK_GRAPHS_TIER {
    // TODO: implementar WorkGraphsTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TightAlignmentSupportTier() -> D3D12_TIGHT_ALIGNMENT_TIER {
    // TODO: implementar TightAlignmentSupportTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ShaderExecutionReorderingActuallyReorders() -> i32 {
    // TODO: implementar ShaderExecutionReorderingActuallyReorders desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Max1DDispatchSize() -> UINT {
    // TODO: implementar Max1DDispatchSize desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Max1DDispatchMeshSize() -> UINT {
    // TODO: implementar Max1DDispatchMeshSize desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn QueryHighestShaderModel() -> i32 {
    // TODO: implementar QueryHighestShaderModel desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn QueryHighestRootSignatureVersion() -> i32 {
    // TODO: implementar QueryHighestRootSignatureVersion desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn QueryHighestFeatureLevel() -> i32 {
    // TODO: implementar QueryHighestFeatureLevel desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn QueryProtectedResourceSessionTypes(NodeIndex: UINT, Count: UINT) -> i32 {
    // TODO: implementar QueryProtectedResourceSessionTypes desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_RANGE(param_20756: const D3D12_RANGE) -> explicit {
    // TODO: implementar CD3DX12_RANGE desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_SUBRESOURCE_FOOTPRINT(resDesc: const D3D12_RESOURCE_DESC&, rowPitch: UINT) -> explicit {
    // TODO: implementar CD3DX12_SUBRESOURCE_FOOTPRINT desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3D12CalcSubresource(MipSlice: UINT, ArraySlice: UINT, PlaneSlice: UINT, MipLevels: UINT, ArraySize: UINT) -> constexpr UINT {
    // TODO: implementar D3D12CalcSubresource desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3D12GetFormatPlaneCount(pDevice: *mut _In_ ID3D12Device, Format: DXGI_FORMAT) -> inline UINT8 {
    // TODO: implementar D3D12GetFormatPlaneCount desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_RESOURCE_DESC(param_11614: D3D12_RESOURCE_DIMENSION_BUFFER, param_54154: resAllocInfo.Alignment, param_15228: resAllocInfo.SizeInBytes, param_4655: 1, param_4655: 1, param_4655: 1, param_29369: DXGI_FORMAT_UNKNOWN, param_4655: 1, param_6097: 0, param_64597: D3D12_TEXTURE_LAYOUT_ROW_MAJOR, param_51689: flags) -> return {
    // TODO: implementar CD3DX12_RESOURCE_DESC desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Depth() -> inline UINT16 {
    // TODO: implementar Depth desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PlaneCount(pDevice: *mut _In_ ID3D12Device) -> inline UINT8 {
    // TODO: implementar PlaneCount desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Subresources(pDevice: *mut _In_ ID3D12Device) -> inline UINT {
    // TODO: implementar Subresources desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_RESOURCE_DESC1(o: const D3D12_RESOURCE_DESC&) -> explicit {
    // TODO: implementar CD3DX12_RESOURCE_DESC1 desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StructuredBuffer(NumElements: UINT, StructureByteStride: UINT, param_13801: UINT64 FirstElement =) -> static inline CD3DX12_SHADER_RESOURCE_VIEW_DESC {
    // TODO: implementar StructuredBuffer desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RawBuffer(NumElements: UINT, param_13801: UINT64 FirstElement =) -> static inline CD3DX12_SHADER_RESOURCE_VIEW_DESC {
    // TODO: implementar RawBuffer desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TypedBuffer(Format: DXGI_FORMAT, NumElements: UINT, param_13801: UINT64 FirstElement =, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: UINT Shader4ComponentMapping =) -> static inline CD3DX12_SHADER_RESOURCE_VIEW_DESC {
    // TODO: implementar TypedBuffer desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Tex1D(Format: DXGI_FORMAT, param_28566: UINT MipLevels =, param_33012: UINT MostDetailedMip =, param_45126: FLOAT ResourceMinLODClamp =, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: UINT Shader4ComponentMapping =) -> static inline CD3DX12_SHADER_RESOURCE_VIEW_DESC {
    // TODO: implementar Tex1D desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Tex1DArray(Format: DXGI_FORMAT, param_1718: UINT ArraySize =, param_28566: UINT MipLevels =, param_65202: UINT FirstArraySlice =, param_33012: UINT MostDetailedMip =, param_45126: FLOAT ResourceMinLODClamp =, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: UINT Shader4ComponentMapping =) -> static inline CD3DX12_SHADER_RESOURCE_VIEW_DESC {
    // TODO: implementar Tex1DArray desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Tex2D(Format: DXGI_FORMAT, param_28566: UINT MipLevels =, param_33012: UINT MostDetailedMip =, param_21259: UINT PlaneSlice =, param_45126: FLOAT ResourceMinLODClamp =, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: UINT Shader4ComponentMapping =) -> static inline CD3DX12_SHADER_RESOURCE_VIEW_DESC {
    // TODO: implementar Tex2D desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Tex2DArray(Format: DXGI_FORMAT, param_1718: UINT ArraySize =, param_28566: UINT MipLevels =, param_65202: UINT FirstArraySlice =, param_33012: UINT MostDetailedMip =, param_21259: UINT PlaneSlice =, param_45126: FLOAT ResourceMinLODClamp =, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: UINT Shader4ComponentMapping =) -> static inline CD3DX12_SHADER_RESOURCE_VIEW_DESC {
    // TODO: implementar Tex2DArray desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Tex2DMS(Format: DXGI_FORMAT, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: UINT Shader4ComponentMapping =) -> static inline CD3DX12_SHADER_RESOURCE_VIEW_DESC {
    // TODO: implementar Tex2DMS desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Tex2DMSArray(Format: DXGI_FORMAT, ArraySize: UINT, param_65202: UINT FirstArraySlice =, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: UINT Shader4ComponentMapping =) -> static inline CD3DX12_SHADER_RESOURCE_VIEW_DESC {
    // TODO: implementar Tex2DMSArray desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Tex3D(Format: DXGI_FORMAT, param_28566: UINT MipLevels =, param_33012: UINT MostDetailedMip =, param_45126: FLOAT ResourceMinLODClamp =, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: UINT Shader4ComponentMapping =) -> static inline CD3DX12_SHADER_RESOURCE_VIEW_DESC {
    // TODO: implementar Tex3D desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TexCube(Format: DXGI_FORMAT, param_28566: UINT MipLevels =, param_33012: UINT MostDetailedMip =, param_45126: FLOAT ResourceMinLODClamp =, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: UINT Shader4ComponentMapping =) -> static inline CD3DX12_SHADER_RESOURCE_VIEW_DESC {
    // TODO: implementar TexCube desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TexCubeArray(Format: DXGI_FORMAT, NumCubes: UINT, param_28566: UINT MipLevels =, param_63529: UINT First2DArrayFace =, param_33012: UINT MostDetailedMip =, param_45126: FLOAT ResourceMinLODClamp =, D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING: UINT Shader4ComponentMapping =) -> static inline CD3DX12_SHADER_RESOURCE_VIEW_DESC {
    // TODO: implementar TexCubeArray desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RaytracingAccelStruct(Location: D3D12_GPU_VIRTUAL_ADDRESS) -> static inline CD3DX12_SHADER_RESOURCE_VIEW_DESC {
    // TODO: implementar RaytracingAccelStruct desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_RT_FORMAT_ARRAY(param_32534: *mut core::ffi::c_void) -> explicit {
    // TODO: implementar CD3DX12_RT_FORMAT_ARRAY desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_SERIALIZED_ROOT_SIGNATURE_DESC(param_55590: CD3DX12_DEFAULT) -> explicit {
    // TODO: implementar CD3DX12_SERIALIZED_ROOT_SIGNATURE_DESC desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ErrorUnknownSubobject(param_2971: UINT) -> virtual void {
    // TODO: implementar ErrorUnknownSubobject desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FinalizeCb() -> virtual void {
    // TODO: implementar FinalizeCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GraphicsDescV0() -> D3D12_GRAPHICS_PIPELINE_STATE_DESC {
    // TODO: implementar GraphicsDescV0 desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ComputeDescV0() -> D3D12_COMPUTE_PIPELINE_STATE_DESC {
    // TODO: implementar ComputeDescV0 desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MeshShaderDescV0() -> D3DX12_MESH_SHADER_PIPELINE_STATE_DESC {
    // TODO: implementar MeshShaderDescV0 desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FlagsCb(Flags: D3D12_PIPELINE_STATE_FLAGS) -> core::ffi::c_void {
    // TODO: implementar FlagsCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NodeMaskCb(NodeMask: UINT) -> core::ffi::c_void {
    // TODO: implementar NodeMaskCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RootSignatureCb(pRootSignature: *mut ID3D12RootSignature) -> core::ffi::c_void {
    // TODO: implementar RootSignatureCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InputLayoutCb(InputLayout: const D3D12_INPUT_LAYOUT_DESC&) -> core::ffi::c_void {
    // TODO: implementar InputLayoutCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IBStripCutValueCb(IBStripCutValue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE) -> core::ffi::c_void {
    // TODO: implementar IBStripCutValueCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn VSCb(VS: const D3D12_SHADER_BYTECODE&) -> core::ffi::c_void {
    // TODO: implementar VSCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GSCb(GS: const D3D12_SHADER_BYTECODE&) -> core::ffi::c_void {
    // TODO: implementar GSCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StreamOutputCb(StreamOutput: const D3D12_STREAM_OUTPUT_DESC&) -> core::ffi::c_void {
    // TODO: implementar StreamOutputCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn HSCb(HS: const D3D12_SHADER_BYTECODE&) -> core::ffi::c_void {
    // TODO: implementar HSCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DSCb(DS: const D3D12_SHADER_BYTECODE&) -> core::ffi::c_void {
    // TODO: implementar DSCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PSCb(PS: const D3D12_SHADER_BYTECODE&) -> core::ffi::c_void {
    // TODO: implementar PSCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CSCb(CS: const D3D12_SHADER_BYTECODE&) -> core::ffi::c_void {
    // TODO: implementar CSCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ASCb(AS: const D3D12_SHADER_BYTECODE&) -> core::ffi::c_void {
    // TODO: implementar ASCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MSCb(MS: const D3D12_SHADER_BYTECODE&) -> core::ffi::c_void {
    // TODO: implementar MSCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BlendStateCb(BlendState: const D3D12_BLEND_DESC&) -> core::ffi::c_void {
    // TODO: implementar BlendStateCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DepthStencilStateCb(DepthStencilState: const D3D12_DEPTH_STENCIL_DESC&) -> core::ffi::c_void {
    // TODO: implementar DepthStencilStateCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DepthStencilState1Cb(DepthStencilState: const D3D12_DEPTH_STENCIL_DESC1&) -> core::ffi::c_void {
    // TODO: implementar DepthStencilState1Cb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DSVFormatCb(DSVFormat: DXGI_FORMAT) -> core::ffi::c_void {
    // TODO: implementar DSVFormatCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RasterizerStateCb(RasterizerState: const D3D12_RASTERIZER_DESC&) -> core::ffi::c_void {
    // TODO: implementar RasterizerStateCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RTVFormatsCb(RTVFormats: const D3D12_RT_FORMAT_ARRAY&) -> core::ffi::c_void {
    // TODO: implementar RTVFormatsCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SampleDescCb(SampleDesc: const DXGI_SAMPLE_DESC&) -> core::ffi::c_void {
    // TODO: implementar SampleDescCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SampleMaskCb(SampleMask: UINT) -> core::ffi::c_void {
    // TODO: implementar SampleMaskCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CachedPSOCb(CachedPSO: const D3D12_CACHED_PIPELINE_STATE&) -> core::ffi::c_void {
    // TODO: implementar CachedPSOCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DepthStencilState2Cb(DepthStencilState: const D3D12_DEPTH_STENCIL_DESC2&) -> core::ffi::c_void {
    // TODO: implementar DepthStencilState2Cb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RasterizerState1Cb(RasterizerState: const D3D12_RASTERIZER_DESC1&) -> core::ffi::c_void {
    // TODO: implementar RasterizerState1Cb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RasterizerState2Cb(RasterizerState: const D3D12_RASTERIZER_DESC2&) -> core::ffi::c_void {
    // TODO: implementar RasterizerState2Cb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX12ParsePipelineStream(Desc: const D3D12_PIPELINE_STATE_STREAM_DESC&, pCallbacks: *mut ID3DX12PipelineParserCallbacks) -> inline HRESULT {
    // TODO: implementar D3DX12ParsePipelineStream desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNumFormats() -> static UINT {
    // TODO: implementar GetNumFormats desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetFormatTable() -> *mut static const FORMAT_DETAIL {
    // TODO: implementar GetFormatTable desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetHighestDefinedFeatureLevel() -> static D3D_FEATURE_LEVEL {
    // TODO: implementar GetHighestDefinedFeatureLevel desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetFormat(Index: SIZE_T) -> static DXGI_FORMAT {
    // TODO: implementar GetFormat desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FormatExists(Format: DXGI_FORMAT) -> static bool {
    // TODO: implementar FormatExists desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetByteAlignment(Format: DXGI_FORMAT) -> static UINT {
    // TODO: implementar GetByteAlignment desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsSRGBFormat(Format: DXGI_FORMAT) -> static bool {
    // TODO: implementar IsSRGBFormat desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetBitsPerStencil(Format: DXGI_FORMAT) -> static UINT {
    // TODO: implementar GetBitsPerStencil desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetBitsPerDepth(Format: DXGI_FORMAT) -> static UINT {
    // TODO: implementar GetBitsPerDepth desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetFormatReturnTypes(Format: DXGI_FORMAT, pInterpretations: *mut D3D_FORMAT_COMPONENT_INTERPRETATION) -> static void {
    // TODO: implementar GetFormatReturnTypes desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Sequential2AbsoluteComponentIndex(Format: DXGI_FORMAT, SequentialComponentIndex: UINT) -> static UINT {
    // TODO: implementar Sequential2AbsoluteComponentIndex desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CanBeCastEvenFullyTyped(Format: DXGI_FORMAT, fl: D3D_FEATURE_LEVEL) -> static bool {
    // TODO: implementar CanBeCastEvenFullyTyped desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetParentFormat(Format: DXGI_FORMAT) -> static DXGI_FORMAT {
    // TODO: implementar GetParentFormat desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetFormatCastSet(Format: DXGI_FORMAT) -> *mut static const DXGI_FORMAT {
    // TODO: implementar GetFormatCastSet desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetTypeLevel(Format: DXGI_FORMAT) -> static D3D_FORMAT_TYPE_LEVEL {
    // TODO: implementar GetTypeLevel desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetBitsPerUnit(Format: DXGI_FORMAT) -> static UINT {
    // TODO: implementar GetBitsPerUnit desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetBitsPerUnitThrow(Format: DXGI_FORMAT) -> static UINT {
    // TODO: implementar GetBitsPerUnitThrow desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetBitsPerElement(Format: DXGI_FORMAT) -> static UINT {
    // TODO: implementar GetBitsPerElement desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetWidthAlignment(Format: DXGI_FORMAT) -> static UINT {
    // TODO: implementar GetWidthAlignment desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetHeightAlignment(Format: DXGI_FORMAT) -> static UINT {
    // TODO: implementar GetHeightAlignment desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetDepthAlignment(Format: DXGI_FORMAT) -> static UINT {
    // TODO: implementar GetDepthAlignment desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Planar(Format: DXGI_FORMAT) -> static BOOL {
    // TODO: implementar Planar desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NonOpaquePlanar(Format: DXGI_FORMAT) -> static BOOL {
    // TODO: implementar NonOpaquePlanar desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn YUV(Format: DXGI_FORMAT) -> static BOOL {
    // TODO: implementar YUV desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Opaque(Format: DXGI_FORMAT) -> static BOOL {
    // TODO: implementar Opaque desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FamilySupportsStencil(Format: DXGI_FORMAT) -> static bool {
    // TODO: implementar FamilySupportsStencil desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NonOpaquePlaneCount(Format: DXGI_FORMAT) -> static UINT {
    // TODO: implementar NonOpaquePlaneCount desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DX9VertexOrIndexFormat(Format: DXGI_FORMAT) -> static BOOL {
    // TODO: implementar DX9VertexOrIndexFormat desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DX9TextureFormat(Format: DXGI_FORMAT) -> static BOOL {
    // TODO: implementar DX9TextureFormat desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FloatNormTextureFormat(Format: DXGI_FORMAT) -> static BOOL {
    // TODO: implementar FloatNormTextureFormat desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DepthOnlyFormat(format: DXGI_FORMAT) -> static bool {
    // TODO: implementar DepthOnlyFormat desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MotionEstimatorAllowedInputFormat(Format: DXGI_FORMAT) -> static bool {
    // TODO: implementar MotionEstimatorAllowedInputFormat desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SupportsSamplerFeedback(Format: DXGI_FORMAT) -> static bool {
    // TODO: implementar SupportsSamplerFeedback desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DecodeHistogramAllowedForOutputFormatSupport(Format: DXGI_FORMAT) -> static bool {
    // TODO: implementar DecodeHistogramAllowedForOutputFormatSupport desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPlaneSliceFromViewFormat(ResourceFormat: DXGI_FORMAT, ViewFormat: DXGI_FORMAT) -> static UINT8 {
    // TODO: implementar GetPlaneSliceFromViewFormat desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SNORMAndUNORMFormats(FormatA: DXGI_FORMAT, FormatB: DXGI_FORMAT) -> static bool {
    // TODO: implementar SNORMAndUNORMFormats desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ValidCastToR32UAV(from: DXGI_FORMAT, to: DXGI_FORMAT) -> static bool {
    // TODO: implementar ValidCastToR32UAV desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsSupportedTextureDisplayableFormat(param_5914: DXGI_FORMAT, bMediaFormatOnly: bool) -> static bool {
    // TODO: implementar IsSupportedTextureDisplayableFormat desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetFormatComponentInterpretation(Format: DXGI_FORMAT, AbsoluteComponentIndex: UINT) -> static D3D_FORMAT_COMPONENT_INTERPRETATION {
    // TODO: implementar GetFormatComponentInterpretation desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetBitsPerComponent(Format: DXGI_FORMAT, AbsoluteComponentIndex: UINT) -> static UINT {
    // TODO: implementar GetBitsPerComponent desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CalculateExtraPlanarRows(format: DXGI_FORMAT, plane0Height: UINT, totalHeight: _Out_ UINT&) -> static HRESULT {
    // TODO: implementar CalculateExtraPlanarRows desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CalculateMinimumRowMajorRowPitch(Format: DXGI_FORMAT, Width: UINT, RowPitch: _Out_ UINT&) -> static HRESULT {
    // TODO: implementar CalculateMinimumRowMajorRowPitch desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CalculateMinimumRowMajorSlicePitch(Format: DXGI_FORMAT, ContextBasedRowPitch: UINT, Height: UINT, SlicePitch: _Out_ UINT&) -> static HRESULT {
    // TODO: implementar CalculateMinimumRowMajorSlicePitch desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetYCbCrChromaSubsampling(Format: DXGI_FORMAT, HorizontalSubsampling: _Out_ UINT&, VerticalSubsampling: _Out_ UINT&) -> static void {
    // TODO: implementar GetYCbCrChromaSubsampling desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CalculateResourceSize(width: UINT, height: UINT, depth: UINT, format: DXGI_FORMAT, mipLevels: UINT, subresources: UINT, totalByteSize: _Out_ SIZE_T&, param_45429: *mut core::ffi::c_void) -> static HRESULT {
    // TODO: implementar CalculateResourceSize desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetTileShape(pTileShape: *mut D3D12_TILE_SHAPE, Format: DXGI_FORMAT, Dimension: D3D12_RESOURCE_DIMENSION, SampleCount: UINT) -> static void {
    // TODO: implementar GetTileShape desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Get4KTileShape(pTileShape: *mut D3D12_TILE_SHAPE, Format: DXGI_FORMAT, Dimension: D3D12_RESOURCE_DIMENSION, SampleCount: UINT) -> static void {
    // TODO: implementar Get4KTileShape desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetMipDimensions(mipSlice: UINT8, pWidth: *mut _Inout_ UINT64, nullptr: _Inout_opt_ UINT64* pHeight =, nullptr: _Inout_opt_ UINT64* pDepth =) -> static void {
    // TODO: implementar GetMipDimensions desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPlaneSubsampledSizeAndFormatForCopyableLayout(PlaneSlice: UINT, Format: DXGI_FORMAT, Width: UINT, Height: UINT, PlaneFormat: _Out_ DXGI_FORMAT&, MinPlanePitchWidth: _Out_ UINT&, PlaneWidth: _Out_ UINT&, PlaneHeight: _Out_ UINT&) -> static void {
    // TODO: implementar GetPlaneSubsampledSizeAndFormatForCopyableLayout desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetDetailTableIndex(Format: DXGI_FORMAT) -> static UINT {
    // TODO: implementar GetDetailTableIndex desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetDetailTableIndexNoThrow(Format: DXGI_FORMAT) -> static UINT {
    // TODO: implementar GetDetailTableIndexNoThrow desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetDetailTableIndexThrow(Format: DXGI_FORMAT) -> static UINT {
    // TODO: implementar GetDetailTableIndexThrow desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SupportsDepth(Format: DXGI_FORMAT) -> static bool {
    // TODO: implementar SupportsDepth desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SupportsStencil(Format: DXGI_FORMAT) -> static bool {
    // TODO: implementar SupportsStencil desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetFormatDetail(Format: DXGI_FORMAT) -> *mut static const FORMAT_DETAIL {
    // TODO: implementar GetFormatDetail desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX12ResourceDesc0ToDesc1(desc0: D3D12_RESOURCE_DESC const&) -> inline D3D12_RESOURCE_DESC1 {
    // TODO: implementar D3DX12ResourceDesc0ToDesc1 desde DirectX-Headers/d3dx12_resource_helpers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX12GetCopyableFootprints(param_37937: *mut core::ffi::c_void) -> return {
    // TODO: implementar D3DX12GetCopyableFootprints desde DirectX-Headers/d3dx12_resource_helpers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitAsShaderResourceView(param_20471: _Out_ D3D12_ROOT_PARAMETER, shaderRegister: UINT, param_38160: UINT registerSpace =, D3D12_SHADER_VISIBILITY_ALL: D3D12_SHADER_VISIBILITY visibility =) -> static inline void {
    // TODO: implementar InitAsShaderResourceView desde DirectX-Headers/d3dx12_root_signature.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitAsUnorderedAccessView(param_20471: _Out_ D3D12_ROOT_PARAMETER, shaderRegister: UINT, param_38160: UINT registerSpace =, D3D12_SHADER_VISIBILITY_ALL: D3D12_SHADER_VISIBILITY visibility =) -> static inline void {
    // TODO: implementar InitAsUnorderedAccessView desde DirectX-Headers/d3dx12_root_signature.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_STATIC_SAMPLER_DESC1(param_62387: const D3D12_STATIC_SAMPLER_DESC) -> explicit {
    // TODO: implementar CD3DX12_STATIC_SAMPLER_DESC1 desde DirectX-Headers/d3dx12_root_signature.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_VERSIONED_ROOT_SIGNATURE_DESC(param_19625: const D3D12_ROOT_SIGNATURE_DESC) -> explicit {
    // TODO: implementar CD3DX12_VERSIONED_ROOT_SIGNATURE_DESC desde DirectX-Headers/d3dx12_root_signature.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn INT64(param_32257: descriptorIncrementSize) -> *mut  {
    // TODO: implementar INT64 desde DirectX-Headers/d3dx12_root_signature.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitOffsetted(param_15773: _In_ const D3D12_CPU_DESCRIPTOR_HANDLE, offsetScaledByIncrementSize: INT) -> inline void {
    // TODO: implementar InitOffsetted desde DirectX-Headers/d3dx12_root_signature.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX12SerializeVersionedRootSignature(pRootSignatureDesc: *mut _In_ const D3D12_VERSIONED_ROOT_SIGNATURE_DESC, MaxVersion: D3D_ROOT_SIGNATURE_VERSION, ppBlob: *mut core::ffi::c_void, param_23427: *mut core::ffi::c_void) -> inline HRESULT {
    // TODO: implementar D3DX12SerializeVersionedRootSignature desde DirectX-Headers/d3dx12_root_signature.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn desc_1_0(param_10422: desc_1_1.NumParameters, param_12043: pParameters_1_0, param_31348: desc_1_1.NumStaticSamplers, pStaticSamplers: pStaticSamplers == nullptr ? desc_1_1.pStaticSamplers :, param_4807: desc_1_1.Flags) -> const CD3DX12_ROOT_SIGNATURE_DESC {
    // TODO: implementar desc_1_0 desde DirectX-Headers/d3dx12_root_signature.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn desc(param_10422: desc_1_1.NumParameters, param_21657: desc_1_1.pParameters, param_31348: desc_1_1.NumStaticSamplers, pStaticSamplers: pStaticSamplers == nullptr ? desc_1_1.pStaticSamplers :, param_4807: desc_1_1.Flags) -> const CD3DX12_VERSIONED_ROOT_SIGNATURE_DESC {
    // TODO: implementar desc desde DirectX-Headers/d3dx12_root_signature.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetStateObjectType(Type: D3D12_STATE_OBJECT_TYPE) -> core::ffi::c_void {
    // TODO: implementar SetStateObjectType desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TrackSubobject(Type: D3D12_STATE_SUBOBJECT_TYPE, pDesc: *mut core::ffi::c_void) -> *mut D3D12_STATE_SUBOBJECT {
    // TODO: implementar TrackSubobject desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LocalCopy(string: CStr, false: bool bSingleString =) -> CStr {
    // TODO: implementar LocalCopy desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Type() -> virtual D3D12_STATE_SUBOBJECT_TYPE {
    // TODO: implementar Type desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Data() -> *mut virtual void {
    // TODO: implementar Data desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetDXILLibrary(pCode: *mut const D3D12_SHADER_BYTECODE) -> core::ffi::c_void {
    // TODO: implementar SetDXILLibrary desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetExistingCollection(param_42870: ID3D12StateObject*pExistingCollection) -> core::ffi::c_void {
    // TODO: implementar SetExistingCollection desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetSubobjectToAssociate(SubobjectToAssociate: const D3D12_STATE_SUBOBJECT&) -> core::ffi::c_void {
    // TODO: implementar SetSubobjectToAssociate desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetSubobjectNameToAssociate(SubobjectToAssociate: LPCWSTR) -> core::ffi::c_void {
    // TODO: implementar SetSubobjectNameToAssociate desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetHitGroupType(Type: D3D12_HIT_GROUP_TYPE) -> core::ffi::c_void {
    // TODO: implementar SetHitGroupType desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetClosestHitShaderImport(importName: LPCWSTR) -> core::ffi::c_void {
    // TODO: implementar SetClosestHitShaderImport desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetIntersectionShaderImport(importName: LPCWSTR) -> core::ffi::c_void {
    // TODO: implementar SetIntersectionShaderImport desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Config(MaxPayloadSizeInBytes: UINT, MaxAttributeSizeInBytes: UINT) -> core::ffi::c_void {
    // TODO: implementar Config desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetRootSignature(pRootSig: *mut ID3D12RootSignature) -> core::ffi::c_void {
    // TODO: implementar SetRootSignature desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX12_COM_PTR_GET(param_522: m_pRootSig) -> return {
    // TODO: implementar D3DX12_COM_PTR_GET desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX12_COM_PTR_ADDRESSOF(param_522: m_pRootSig) -> return {
    // TODO: implementar D3DX12_COM_PTR_ADDRESSOF desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetNodeMask(NodeMask: UINT) -> core::ffi::c_void {
    // TODO: implementar SetNodeMask desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetSODeclEntries(soDeclEntries: *mut const D3D12_SO_DECLARATION_ENTRY, numEntries: UINT) -> core::ffi::c_void {
    // TODO: implementar SetSODeclEntries desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetBufferStrides(bufferStrides: *mut const UINT, numStrides: UINT) -> core::ffi::c_void {
    // TODO: implementar SetBufferStrides desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetRasterizedStream(rasterizedStream: UINT) -> core::ffi::c_void {
    // TODO: implementar SetRasterizedStream desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetAlphaToCoverageEnable(alphaToCoverageEnable: bool) -> core::ffi::c_void {
    // TODO: implementar SetAlphaToCoverageEnable desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetIndependentBlendEnable(independentBlendEnable: bool) -> core::ffi::c_void {
    // TODO: implementar SetIndependentBlendEnable desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetFillMode(fillMode: D3D12_FILL_MODE) -> core::ffi::c_void {
    // TODO: implementar SetFillMode desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetCullMode(cullMode: D3D12_CULL_MODE) -> core::ffi::c_void {
    // TODO: implementar SetCullMode desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetDepthBias(depthBias: FLOAT) -> core::ffi::c_void {
    // TODO: implementar SetDepthBias desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetDepthBiasClamp(depthBiasClamp: FLOAT) -> core::ffi::c_void {
    // TODO: implementar SetDepthBiasClamp desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetSlopeScaledDepthBias(slopeScaledDepthBias: FLOAT) -> core::ffi::c_void {
    // TODO: implementar SetSlopeScaledDepthBias desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetDepthClipEnable(depthClipEnable: i32) -> core::ffi::c_void {
    // TODO: implementar SetDepthClipEnable desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetLineRasterizationMode(lineRasterizationMode: D3D12_LINE_RASTERIZATION_MODE) -> core::ffi::c_void {
    // TODO: implementar SetLineRasterizationMode desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetForcedSampleCount(forcedSampleCount: UINT) -> core::ffi::c_void {
    // TODO: implementar SetForcedSampleCount desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetConservativeRaster(conservativeRaster: D3D12_CONSERVATIVE_RASTERIZATION_MODE) -> core::ffi::c_void {
    // TODO: implementar SetConservativeRaster desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetDepthEnable(depthEnable: i32) -> core::ffi::c_void {
    // TODO: implementar SetDepthEnable desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetDepthFunc(depthFunc: D3D12_COMPARISON_FUNC) -> core::ffi::c_void {
    // TODO: implementar SetDepthFunc desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetStencilEnable(stencilEnable: i32) -> core::ffi::c_void {
    // TODO: implementar SetStencilEnable desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetDepthBoundsTestEnable(depthBoundsTestEnable: i32) -> core::ffi::c_void {
    // TODO: implementar SetDepthBoundsTestEnable desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetIBStripCutValue(ibStripCutValue: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE) -> core::ffi::c_void {
    // TODO: implementar SetIBStripCutValue desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetNumRenderTargets(numRenderTargets: UINT) -> core::ffi::c_void {
    // TODO: implementar SetNumRenderTargets desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetRenderTargetFormat(renderTarget: UINT, renderTargetFormat: DXGI_FORMAT) -> core::ffi::c_void {
    // TODO: implementar SetRenderTargetFormat desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetDepthStencilFormat(depthStencilFormat: DXGI_FORMAT) -> core::ffi::c_void {
    // TODO: implementar SetDepthStencilFormat desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetCount(count: UINT) -> core::ffi::c_void {
    // TODO: implementar SetCount desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetQuality(quality: UINT) -> core::ffi::c_void {
    // TODO: implementar SetQuality desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetSampleMask(sampleMask: UINT) -> core::ffi::c_void {
    // TODO: implementar SetSampleMask desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetProgramName(ProgramName: LPCWSTR) -> core::ffi::c_void {
    // TODO: implementar SetProgramName desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AddSubobject(subobject: const D3D12_STATE_SUBOBJECT&) -> core::ffi::c_void {
    // TODO: implementar AddSubobject desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NewOutputOverride() -> core::ffi::c_void {
    // TODO: implementar NewOutputOverride desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OutputIndex(index: UINT) -> core::ffi::c_void {
    // TODO: implementar OutputIndex desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NewName(Name: LPCWSTR, param_57966: UINT ArrayIndex =) -> core::ffi::c_void {
    // TODO: implementar NewName desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AllowSparseNodes(bAllow: i32) -> core::ffi::c_void {
    // TODO: implementar AllowSparseNodes desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MaxOutputRecords(maxOutputRecords: UINT) -> core::ffi::c_void {
    // TODO: implementar MaxOutputRecords desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MaxOutputRecordsSharedWith(outputIndex: UINT) -> core::ffi::c_void {
    // TODO: implementar MaxOutputRecordsSharedWith desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Shader(_Shader: LPCWSTR) -> core::ffi::c_void {
    // TODO: implementar Shader desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetShaderName() -> LPCWSTR {
    // TODO: implementar GetShaderName desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNode() -> return {
    // TODO: implementar GetNode desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LocalRootArgumentsTableIndex(index: UINT) -> core::ffi::c_void {
    // TODO: implementar LocalRootArgumentsTableIndex desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ProgramEntry(bIsProgramEntry: i32) -> core::ffi::c_void {
    // TODO: implementar ProgramEntry desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ShareInputOf(NodeID: D3D12_NODE_ID) -> core::ffi::c_void {
    // TODO: implementar ShareInputOf desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DispatchGrid(x: UINT, y: UINT, z: UINT) -> core::ffi::c_void {
    // TODO: implementar DispatchGrid desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MaxDispatchGrid(x: UINT, y: UINT, z: UINT) -> core::ffi::c_void {
    // TODO: implementar MaxDispatchGrid desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IncludeAllAvailableNodes() -> core::ffi::c_void {
    // TODO: implementar IncludeAllAvailableNodes desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsAttributeSupported(attributeGUID: REFGUID) -> virtual bool STDMETHODCALLTYPE {
    // TODO: implementar IsAttributeSupported desde DirectX-Headers/dxcore_interface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsPropertySupported(property: DXCoreAdapterProperty) -> virtual bool STDMETHODCALLTYPE {
    // TODO: implementar IsPropertySupported desde DirectX-Headers/dxcore_interface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPropertySize(property: DXCoreAdapterProperty, bufferSize: *mut _Out_ size_t) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetPropertySize desde DirectX-Headers/dxcore_interface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsQueryStateSupported(property: DXCoreAdapterState) -> virtual bool STDMETHODCALLTYPE {
    // TODO: implementar IsQueryStateSupported desde DirectX-Headers/dxcore_interface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsSetStateSupported(property: DXCoreAdapterState) -> virtual bool STDMETHODCALLTYPE {
    // TODO: implementar IsSetStateSupported desde DirectX-Headers/dxcore_interface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPropertyWithInput(param_48407: property, param_54050: *mut core::ffi::c_void) -> return {
    // TODO: implementar GetPropertyWithInput desde DirectX-Headers/dxcore_interface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsStale() -> virtual bool STDMETHODCALLTYPE {
    // TODO: implementar IsStale desde DirectX-Headers/dxcore_interface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsAdapterPreferenceSupported(preference: DXCoreAdapterPreference) -> virtual bool STDMETHODCALLTYPE {
    // TODO: implementar IsAdapterPreferenceSupported desde DirectX-Headers/dxcore_interface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsNotificationTypeSupported(notificationType: DXCoreNotificationType) -> virtual bool STDMETHODCALLTYPE {
    // TODO: implementar IsNotificationTypeSupported desde DirectX-Headers/dxcore_interface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RegisterEventNotification(dxCoreObject: *mut _In_ IUnknown, notificationType: DXCoreNotificationType, callbackFunction: _In_ PFN_DXCORE_NOTIFICATION_, callbackContext: *mut _In_opt_ void, eventCookie: *mut _Out_ uint32_t) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar RegisterEventNotification desde DirectX-Headers/dxcore_interface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnregisterEventNotification(eventCookie: u32) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar UnregisterEventNotification desde DirectX-Headers/dxcore_interface.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn uuidof() -> GUID {
    // TODO: implementar uuidof desde DirectX-Headers/dxguids.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetAddressOf() -> *mut core::ffi::c_void {
    // TODO: implementar GetAddressOf desde DirectX-Headers/wrladapter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ReleaseAndGetAddressOf() -> *mut core::ffi::c_void {
    // TODO: implementar ReleaseAndGetAddressOf desde DirectX-Headers/wrladapter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InternalAddRef() -> core::ffi::c_void {
    // TODO: implementar InternalAddRef desde DirectX-Headers/wrladapter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InternalRelease() -> u64 {
    // TODO: implementar InternalRelease desde DirectX-Headers/wrladapter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CopyTo(ptr: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar CopyTo desde DirectX-Headers/wrladapter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn As(p: _Inout_ Details::ComPtrRef<ComPtr<U>>) -> i32 {
    // TODO: implementar As desde DirectX-Headers/wrladapter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AsIID(implements: *mut _In_ T, riid: REFIID, param_24048: *mut _Outptr_result_nullonfailure_ void) -> static HRESULT {
    // TODO: implementar AsIID desde DirectX-Headers/wrladapter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CastToBase(ptr: *mut _In_ T) -> *mut static Base {
    // TODO: implementar CastToBase desde DirectX-Headers/wrladapter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CastToUnknown(ptr: *mut _In_ T) -> *mut static IUnknown {
    // TODO: implementar CastToUnknown desde DirectX-Headers/wrladapter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CanCastTo(ptr: *mut _In_ T, riid: REFIID, param_3335: *mut _Outptr_ void) -> static bool {
    // TODO: implementar CanCastTo desde DirectX-Headers/wrladapter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetRefCount() -> u64 {
    // TODO: implementar GetRefCount desde DirectX-Headers/wrladapter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CustomQueryInterface(param_55479: REFIID, param_46467: _Outptr_result_nullonfailure_, handled: *mut _Out_ bool) -> i32 {
    // TODO: implementar CustomQueryInterface desde DirectX-Headers/wrladapter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IID_PPV_ARGS_Helper(pp: Microsoft::WRL::Details::ComPtrRef<T>) -> *mut core::ffi::c_void {
    // TODO: implementar IID_PPV_ARGS_Helper desde DirectX-Headers/wrladapter.h
    core::ptr::null_mut()
}
