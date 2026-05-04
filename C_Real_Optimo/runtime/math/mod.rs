//! ADead Runtime - MATH Module
//!
//! Funciones generadas automáticamente desde knowledge.json
//! Categoría: math

#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

/// GetExplicitEntriesFromAclA - from wine/aclapi.h
#[no_mangle]
pub unsafe extern "C" fn GetExplicitEntriesFromAclA(arg0: usize, arg1: usize, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// GetExplicitEntriesFromAclW - from wine/aclapi.h
#[no_mangle]
pub unsafe extern "C" fn GetExplicitEntriesFromAclW(arg0: usize, arg1: usize, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// SetEntriesInAclA - from wine/aclapi.h
#[no_mangle]
pub unsafe extern "C" fn SetEntriesInAclA(arg0: u32, arg1: usize, arg2: usize, arg3: *mut core::ffi::c_void) -> usize {
    0
}

/// SetEntriesInAclW - from wine/aclapi.h
#[no_mangle]
pub unsafe extern "C" fn SetEntriesInAclW(arg0: u32, arg1: usize, arg2: usize, arg3: *mut core::ffi::c_void) -> usize {
    0
}

/// BuildExplicitAccessWithNameA - from wine/aclapi.h
#[no_mangle]
pub unsafe extern "C" fn BuildExplicitAccessWithNameA(arg0: usize, arg1: *mut i8, arg2: u32, arg3: usize, arg4: u32) -> usize {
    0
}

/// BuildExplicitAccessWithNameW - from wine/aclapi.h
#[no_mangle]
pub unsafe extern "C" fn BuildExplicitAccessWithNameW(arg0: usize, arg1: *mut u16, arg2: u32, arg3: usize, arg4: u32) -> usize {
    0
}

/// AtlModuleRegisterWndClassInfoA - from wine/atlwin.h
#[no_mangle]
pub unsafe extern "C" fn AtlModuleRegisterWndClassInfoA(pm: *mut core::ffi::c_void, wci: *mut core::ffi::c_void, pProc: *mut core::ffi::c_void) -> u16 {
    0
}

/// AtlModuleRegisterWndClassInfoW - from wine/atlwin.h
#[no_mangle]
pub unsafe extern "C" fn AtlModuleRegisterWndClassInfoW(pm: *mut core::ffi::c_void, wci: *mut core::ffi::c_void, pProc: *mut core::ffi::c_void) -> u16 {
    0
}

/// BCryptExportKey - from wine/bcrypt.h
#[no_mangle]
pub unsafe extern "C" fn BCryptExportKey(arg0: usize, arg1: usize, arg2: *const u16, arg3: usize, arg4: u32, arg5: *mut u32, arg6: u32) -> i32 {
    0
}

/// CM_Add_Empty_Log_Conf - from wine/cfgmgr32.h
#[no_mangle]
pub unsafe extern "C" fn CM_Add_Empty_Log_Conf(conf: *mut core::ffi::c_void, node: usize, priority: usize, flags: u32) -> usize {
    0
}

/// CM_Add_Empty_Log_Conf_Ex - from wine/cfgmgr32.h
#[no_mangle]
pub unsafe extern "C" fn CM_Add_Empty_Log_Conf_Ex(conf: *mut core::ffi::c_void, node: usize, priority: usize, flags: u32, machine: usize) -> usize {
    0
}

/// CM_Get_First_Log_Conf - from wine/cfgmgr32.h
#[no_mangle]
pub unsafe extern "C" fn CM_Get_First_Log_Conf(conf: *mut core::ffi::c_void, node: usize, flags: u32) -> usize {
    0
}

/// CM_Get_First_Log_Conf_Ex - from wine/cfgmgr32.h
#[no_mangle]
pub unsafe extern "C" fn CM_Get_First_Log_Conf_Ex(conf: *mut core::ffi::c_void, node: usize, flags: u32, machine: usize) -> usize {
    0
}

/// CM_Get_Log_Conf_Priority - from wine/cfgmgr32.h
#[no_mangle]
pub unsafe extern "C" fn CM_Get_Log_Conf_Priority(conf: usize, priority: *mut core::ffi::c_void, flags: u32) -> usize {
    0
}

/// CM_Get_Log_Conf_Priority_Ex - from wine/cfgmgr32.h
#[no_mangle]
pub unsafe extern "C" fn CM_Get_Log_Conf_Priority_Ex(conf: usize, priority: *mut core::ffi::c_void, flags: u32, machine: usize) -> usize {
    0
}

/// CM_Get_Next_Log_Conf - from wine/cfgmgr32.h
#[no_mangle]
pub unsafe extern "C" fn CM_Get_Next_Log_Conf(next: *mut core::ffi::c_void, conf: usize, flags: u32) -> usize {
    0
}

/// CM_Get_Next_Log_Conf_Ex - from wine/cfgmgr32.h
#[no_mangle]
pub unsafe extern "C" fn CM_Get_Next_Log_Conf_Ex(next: *mut core::ffi::c_void, conf: usize, flags: u32, machine: usize) -> usize {
    0
}

/// TaskDialog - from wine/commctrl.h
#[no_mangle]
pub unsafe extern "C" fn TaskDialog(owner: *mut core::ffi::c_void, hinst: *mut core::ffi::c_void, title: *mut u16, main_instruction: *mut u16, content: *mut u16, common_buttons: usize, icon: *mut u16, button: *mut i32) -> usize {
    0
}

/// TaskDialogIndirect - from wine/commctrl.h
#[no_mangle]
pub unsafe extern "C" fn TaskDialogIndirect(arg0: *mut core::ffi::c_void, arg1: *mut i32, arg2: *mut i32, arg3: *mut i32) -> usize {
    0
}

/// CryptUIWizExport - from wine/cryptuiapi.h
#[no_mangle]
pub unsafe extern "C" fn CryptUIWizExport(dwFlags: u32, hwndParent: *mut core::ffi::c_void, pwszWizardTitle: *const u16, pExportInfo: usize, pvoid: *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DGetInputAndOutputSignatureBlob - from wine/d3dcompiler.h
#[no_mangle]
pub unsafe extern "C" fn D3DGetInputAndOutputSignatureBlob(data: *mut core::ffi::c_void, data_size: usize, blob: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXQuaternionExp - from wine/d3dx9math.h
#[no_mangle]
pub unsafe extern "C" fn D3DXQuaternionExp(pout: *mut core::ffi::c_void, pq: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// D3DXComputeTangentFrameEx - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXComputeTangentFrameEx(mesh_in: *mut core::ffi::c_void, texture_in_semantic: u32, texture_in_idx: u32, u_partial_out_semantic: u32, u_partial_out_idx: u32, v_partial_out_semantic: u32, v_partial_out_idx: u32, normal_out_semantic: u32, normal_out_idx: u32, flags: u32, adjacency: *mut u32, partial_edge_threshold: f32, singular_point_threshold: f32, normal_edge_threshold: f32, mesh_out: *mut *mut core::ffi::c_void, buffer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXComputeTangent - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXComputeTangent(mesh: *mut core::ffi::c_void, stage: u32, tangent_idx: u32, binorm_idx: u32, wrap: u32, adjacency: *mut u32) -> i32 {
    0
}

/// D3DXConvertMeshSubsetToSingleStrip - from wine/d3dx9mesh.h
#[no_mangle]
pub unsafe extern "C" fn D3DXConvertMeshSubsetToSingleStrip(mesh_in: *mut core::ffi::c_void, attribute_id: u32, ib_flags: u32, index_buffer: *mut *mut core::ffi::c_void, index_count: *mut u32) -> i32 {
    0
}

/// D3DXGetShaderConstantTableEx - from wine/d3dx9shader.h
#[no_mangle]
pub unsafe extern "C" fn D3DXGetShaderConstantTableEx(byte_code: *mut u32, flags: u32, constant_table: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DXGetShaderConstantTable - from wine/d3dx9shader.h
#[no_mangle]
pub unsafe extern "C" fn D3DXGetShaderConstantTable(byte_code: *mut u32, constant_table: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// DIFXAPISetLogCallbackA - from wine/difxapi.h
#[no_mangle]
pub unsafe extern "C" fn DIFXAPISetLogCallbackA(arg0: usize, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// DIFXAPISetLogCallbackW - from wine/difxapi.h
#[no_mangle]
pub unsafe extern "C" fn DIFXAPISetLogCallbackW(arg0: usize, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// SetDifxLogCallbackA - from wine/difxapi.h
#[no_mangle]
pub unsafe extern "C" fn SetDifxLogCallbackA(arg0: usize, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// SetDifxLogCallbackW - from wine/difxapi.h
#[no_mangle]
pub unsafe extern "C" fn SetDifxLogCallbackW(arg0: usize, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// GetTraceLoggerHandle - from wine/evntrace.h
#[no_mangle]
pub unsafe extern "C" fn GetTraceLoggerHandle(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// PfMakeLog - from wine/fltdefs.h
#[no_mangle]
pub unsafe extern "C" fn PfMakeLog(hEvent: *mut core::ffi::c_void) -> usize {
    0
}

/// PfSetLogBuffer - from wine/fltdefs.h
#[no_mangle]
pub unsafe extern "C" fn PfSetLogBuffer(pbBuffer: usize, dwSize: u32, dwThreshold: u32, dwEntries: u32, pdwLoggedEntries: usize, pdwLostEntries: usize, pdwSizeUsed: usize) -> usize {
    0
}

/// PfDeleteLog - from wine/fltdefs.h
#[no_mangle]
pub unsafe extern "C" fn PfDeleteLog(arg0: usize) -> usize {
    0
}

/// GdipCreateFontFromLogfontA - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFontFromLogfontA(arg0: *mut core::ffi::c_void, LOGFONTA: usize, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipCreateFontFromLogfontW - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFontFromLogfontW(arg0: *mut core::ffi::c_void, LOGFONTW: usize, arg2: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GdipGetLogFontA - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipGetLogFontA(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// GdipGetLogFontW - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipGetLogFontW(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// GdipDrawRectangle - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipDrawRectangle(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> usize {
    0
}

/// GdipDrawRectangleI - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipDrawRectangleI(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: i32, arg3: i32, arg4: i32, arg5: i32) -> usize {
    0
}

/// GdipDrawRectangles - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipDrawRectangles(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, GpRectF: usize, arg3: i32) -> usize {
    0
}

/// GdipDrawRectanglesI - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipDrawRectanglesI(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, GpRect: usize, arg3: i32) -> usize {
    0
}

/// GdipFillRectangle - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipFillRectangle(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> usize {
    0
}

/// GdipFillRectangleI - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipFillRectangleI(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: i32, arg3: i32, arg4: i32, arg5: i32) -> usize {
    0
}

/// GdipFillRectangles - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipFillRectangles(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, GpRectF: usize, arg3: i32) -> usize {
    0
}

/// GdipFillRectanglesI - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipFillRectanglesI(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, GpRect: usize, arg3: i32) -> usize {
    0
}

/// GdipAddPathRectangle - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipAddPathRectangle(arg0: *mut core::ffi::c_void, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// GdipAddPathRectangleI - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipAddPathRectangleI(arg0: *mut core::ffi::c_void, arg1: i32, arg2: i32, arg3: i32, arg4: i32) -> usize {
    0
}

/// GdipAddPathRectangles - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipAddPathRectangles(arg0: *mut core::ffi::c_void, GpRectF: usize, arg2: i32) -> usize {
    0
}

/// GdipAddPathRectanglesI - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipAddPathRectanglesI(arg0: *mut core::ffi::c_void, GpRect: usize, arg2: i32) -> usize {
    0
}

/// GdipIsInfiniteRegion - from wine/gdiplusflat.h
#[no_mangle]
pub unsafe extern "C" fn GdipIsInfiniteRegion(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut i32) -> usize {
    0
}

/// IsIndexedPixelFormat - from wine/gdipluspixelformats.h
#[no_mangle]
pub unsafe extern "C" fn IsIndexedPixelFormat(format: usize) -> i32 {
    0
}

/// CreateProfileFromLogColorSpaceA - from wine/icm.h
#[no_mangle]
pub unsafe extern "C" fn CreateProfileFromLogColorSpaceA(arg0: usize, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// CreateProfileFromLogColorSpaceW - from wine/icm.h
#[no_mangle]
pub unsafe extern "C" fn CreateProfileFromLogColorSpaceW(arg0: usize, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// GetStandardColorSpaceProfileA - from wine/icm.h
#[no_mangle]
pub unsafe extern "C" fn GetStandardColorSpaceProfileA(arg0: usize, arg1: u32, arg2: usize, arg3: usize) -> i32 {
    0
}

/// GetStandardColorSpaceProfileW - from wine/icm.h
#[no_mangle]
pub unsafe extern "C" fn GetStandardColorSpaceProfileW(arg0: usize, arg1: u32, arg2: usize, arg3: usize) -> i32 {
    0
}

/// SetStandardColorSpaceProfileA - from wine/icm.h
#[no_mangle]
pub unsafe extern "C" fn SetStandardColorSpaceProfileA(arg0: usize, arg1: u32, arg2: usize) -> i32 {
    0
}

/// SetStandardColorSpaceProfileW - from wine/icm.h
#[no_mangle]
pub unsafe extern "C" fn SetStandardColorSpaceProfileW(arg0: usize, arg1: u32, arg2: usize) -> i32 {
    0
}

/// ubidi_getLogicalIndex - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ubidi_getLogicalIndex(pBiDi: *mut core::ffi::c_void, visualIndex: i32, pErrorCode: *mut core::ffi::c_void) -> i32 {
    0
}

/// ubidi_getLogicalMap - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ubidi_getLogicalMap(pBiDi: *mut core::ffi::c_void, indexMap: *mut i32, pErrorCode: *mut core::ffi::c_void) {

}

/// ubidi_getLogicalRun - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ubidi_getLogicalRun(pBiDi: *mut core::ffi::c_void, logicalPosition: i32, pLogicalLimit: *mut i32, pLevel: *mut core::ffi::c_void) {

}

/// ubidi_isInverse - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ubidi_isInverse(pBiDi: *mut core::ffi::c_void) -> usize {
    0
}

/// ubidi_reorderLogical - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ubidi_reorderLogical(levels: *mut core::ffi::c_void, length: i32, indexMap: *mut i32) {

}

/// ucnv_countStandards - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ucnv_countStandards() -> u16 {
    0
}

/// ucnv_getStandard - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ucnv_getStandard(n: u16, pErrorCode: *mut core::ffi::c_void) -> *mut i8 {
    core::ptr::null_mut()
}

/// ucnv_getStandardName - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ucnv_getStandardName(name: *mut i8, standard: *mut i8, pErrorCode: *mut core::ffi::c_void) -> *mut i8 {
    core::ptr::null_mut()
}

/// ucnv_openStandardNames - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ucnv_openStandardNames(convName: *mut i8, standard: *mut i8, pErrorCode: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// ucol_getContractionsAndExpansions - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ucol_getContractionsAndExpansions(coll: *mut core::ffi::c_void, contractions: *mut core::ffi::c_void, expansions: *mut core::ffi::c_void, addPrefixes: usize, status: *mut core::ffi::c_void) {

}

/// ucol_getMaxExpansion - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ucol_getMaxExpansion(elems: *mut core::ffi::c_void, order: i32) -> i32 {
    0
}

/// ucsdet_isInputFilterEnabled - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ucsdet_isInputFilterEnabled(ucsd: *mut core::ffi::c_void) -> usize {
    0
}

/// ugender_getInstance - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ugender_getInstance(locale: *mut i8, status: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// unorm2_getInstance - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn unorm2_getInstance(packageName: *mut i8, name: *mut i8, mode: usize, pErrorCode: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// unorm2_getNFCInstance - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn unorm2_getNFCInstance(pErrorCode: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// unorm2_getNFDInstance - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn unorm2_getNFDInstance(pErrorCode: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// unorm2_getNFKCCasefoldInstance - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn unorm2_getNFKCCasefoldInstance(pErrorCode: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// unorm2_getNFKCInstance - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn unorm2_getNFKCInstance(pErrorCode: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// unorm2_getNFKDInstance - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn unorm2_getNFKDInstance(pErrorCode: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// unorm2_isInert - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn unorm2_isInert(norm2: *mut core::ffi::c_void, c: usize) -> usize {
    0
}

/// utext_isLengthExpensive - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn utext_isLengthExpensive(ut: *mut core::ffi::c_void) -> usize {
    0
}

/// utrans_transIncremental - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn utrans_transIncremental(trans: *mut core::ffi::c_void, rep: *mut core::ffi::c_void, repFunc: *mut core::ffi::c_void, pos: *mut core::ffi::c_void, status: *mut core::ffi::c_void) {

}

/// utrans_transIncrementalUChars - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn utrans_transIncrementalUChars(trans: *mut core::ffi::c_void, text: *mut core::ffi::c_void, textLength: *mut i32, textCapacity: i32, pos: *mut core::ffi::c_void, status: *mut core::ffi::c_void) {

}

/// BufferPointerPacketsInteractionContext - from wine/interactioncontext.h
#[no_mangle]
pub unsafe extern "C" fn BufferPointerPacketsInteractionContext(context: usize, entries_count: usize, pointer_info: *mut core::ffi::c_void) -> i32 {
    0
}

/// ProcessBufferedPacketsInteractionContext - from wine/interactioncontext.h
#[no_mangle]
pub unsafe extern "C" fn ProcessBufferedPacketsInteractionContext(context: usize) -> i32 {
    0
}

/// ProcessInertiaInteractionContext - from wine/interactioncontext.h
#[no_mangle]
pub unsafe extern "C" fn ProcessInertiaInteractionContext(context: usize) -> i32 {
    0
}

/// GetAdaptersInfo - from wine/iphlpapi.h
#[no_mangle]
pub unsafe extern "C" fn GetAdaptersInfo(pAdapterInfo: usize, pOutBufLen: usize) -> usize {
    0
}

/// GetRTTAndHopCount - from wine/iphlpapi.h
#[no_mangle]
pub unsafe extern "C" fn GetRTTAndHopCount(DestIpAddress: usize, HopCount: usize, MaxHops: u32, RTT: usize) -> usize {
    0
}

/// I_BrowserSetNetlogonState - from wine/lmbrowsr.h
#[no_mangle]
pub unsafe extern "C" fn I_BrowserSetNetlogonState(ServerName: *mut u16, DomainName: *mut u16, EmulatedServerName: *mut u16, Role: u32) -> usize {
    0
}

/// GetExpandedNameA - from wine/lzexpand.h
#[no_mangle]
pub unsafe extern "C" fn GetExpandedNameA(arg0: *mut i8, arg1: *mut i8) -> usize {
    0
}

/// GetExpandedNameW - from wine/lzexpand.h
#[no_mangle]
pub unsafe extern "C" fn GetExpandedNameW(arg0: *mut u16, arg1: *mut u16) -> usize {
    0
}

/// GetInstance - from wine/mapiutil.h
#[no_mangle]
pub unsafe extern "C" fn GetInstance(arg0: usize, arg1: usize, arg2: u32) -> usize {
    0
}

/// MesIncrementalHandleReset - from wine/midles.h
#[no_mangle]
pub unsafe extern "C" fn MesIncrementalHandleReset(arg0: usize, arg1: *mut core::ffi::c_void, arg2: usize, arg3: usize, arg4: usize, arg5: usize) -> usize {
    0
}

/// MesInqProcEncodingId - from wine/midles.h
#[no_mangle]
pub unsafe extern "C" fn MesInqProcEncodingId(arg0: usize, arg1: usize, arg2: *mut u32) -> usize {
    0
}

/// CryptCATAdminAddCatalog - from wine/mscat.h
#[no_mangle]
pub unsafe extern "C" fn CryptCATAdminAddCatalog(arg0: usize, arg1: usize, arg2: usize, arg3: u32) -> usize {
    0
}

/// CryptCATAdminEnumCatalogFromHash - from wine/mscat.h
#[no_mangle]
pub unsafe extern "C" fn CryptCATAdminEnumCatalogFromHash(arg0: usize, arg1: *mut u8, arg2: u32, arg3: u32, arg4: *mut core::ffi::c_void) -> usize {
    0
}

/// CryptCATAdminReleaseCatalogContext - from wine/mscat.h
#[no_mangle]
pub unsafe extern "C" fn CryptCATAdminReleaseCatalogContext(arg0: usize, arg1: usize, arg2: u32) -> i32 {
    0
}

/// CryptCATAdminRemoveCatalog - from wine/mscat.h
#[no_mangle]
pub unsafe extern "C" fn CryptCATAdminRemoveCatalog(arg0: usize, arg1: *const u16, arg2: u32) -> i32 {
    0
}

/// CryptCATAdminResolveCatalogPath - from wine/mscat.h
#[no_mangle]
pub unsafe extern "C" fn CryptCATAdminResolveCatalogPath(arg0: usize, arg1: *mut u16, arg2: *mut core::ffi::c_void, arg3: u32) -> i32 {
    0
}

/// CryptCATCatalogInfoFromContext - from wine/mscat.h
#[no_mangle]
pub unsafe extern "C" fn CryptCATCatalogInfoFromContext(arg0: usize, arg1: *mut core::ffi::c_void, arg2: u32) -> i32 {
    0
}

/// MsiEnumComponentCostsA - from wine/msi.h
#[no_mangle]
pub unsafe extern "C" fn MsiEnumComponentCostsA(arg0: usize, arg1: *const i8, arg2: u32, arg3: usize, arg4: *mut i8, arg5: usize, arg6: usize, arg7: usize) -> u32 {
    0
}

/// MsiEnumComponentCostsW - from wine/msi.h
#[no_mangle]
pub unsafe extern "C" fn MsiEnumComponentCostsW(arg0: usize, arg1: *const u16, arg2: u32, arg3: usize, arg4: *mut u16, arg5: usize, arg6: usize, arg7: usize) -> u32 {
    0
}

/// MsiEnableLogA - from wine/msi.h
#[no_mangle]
pub unsafe extern "C" fn MsiEnableLogA(arg0: u32, arg1: *const i8, arg2: u32) -> u32 {
    0
}

/// MsiEnableLogW - from wine/msi.h
#[no_mangle]
pub unsafe extern "C" fn MsiEnableLogW(arg0: u32, arg1: *const u16, arg2: u32) -> u32 {
    0
}

/// MsiInstallMissingComponentA - from wine/msi.h
#[no_mangle]
pub unsafe extern "C" fn MsiInstallMissingComponentA(arg0: *const i8, arg1: *const i8, arg2: usize) -> u32 {
    0
}

/// MsiInstallMissingComponentW - from wine/msi.h
#[no_mangle]
pub unsafe extern "C" fn MsiInstallMissingComponentW(arg0: *const u16, arg1: *const u16, arg2: usize) -> u32 {
    0
}

/// MsiPreviewDialogA - from wine/msiquery.h
#[no_mangle]
pub unsafe extern "C" fn MsiPreviewDialogA(arg0: usize, arg1: *const i8) -> u32 {
    0
}

/// MsiPreviewDialogW - from wine/msiquery.h
#[no_mangle]
pub unsafe extern "C" fn MsiPreviewDialogW(arg0: usize, arg1: *const u16) -> u32 {
    0
}

/// MsiDatabaseExportA - from wine/msiquery.h
#[no_mangle]
pub unsafe extern "C" fn MsiDatabaseExportA(arg0: usize, arg1: *const i8, arg2: *const i8, arg3: *const i8) -> u32 {
    0
}

/// MsiDatabaseExportW - from wine/msiquery.h
#[no_mangle]
pub unsafe extern "C" fn MsiDatabaseExportW(arg0: usize, arg1: *const u16, arg2: *const u16, arg3: *const u16) -> u32 {
    0
}

/// MsiGetFeatureCostA - from wine/msiquery.h
#[no_mangle]
pub unsafe extern "C" fn MsiGetFeatureCostA(arg0: usize, arg1: *const i8, arg2: usize, arg3: usize, arg4: usize) -> u32 {
    0
}

/// MsiGetFeatureCostW - from wine/msiquery.h
#[no_mangle]
pub unsafe extern "C" fn MsiGetFeatureCostW(arg0: usize, arg1: *const u16, arg2: usize, arg3: usize, arg4: usize) -> u32 {
    0
}

/// CryptSIPRetrieveSubjectGuidForCatalogFile - from wine/mssip.h
#[no_mangle]
pub unsafe extern "C" fn CryptSIPRetrieveSubjectGuidForCatalogFile(arg0: *const u16, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// NCryptExportKey - from wine/ncrypt.h
#[no_mangle]
pub unsafe extern "C" fn NCryptExportKey(arg0: usize, arg1: usize, arg2: *mut u16, arg3: *mut core::ffi::c_void, arg4: *mut u8, arg5: u32, arg6: *mut u32, arg7: u32) -> usize {
    0
}

/// GetAnycastIpAddressTable - from wine/netioapi.h
#[no_mangle]
pub unsafe extern "C" fn GetAnycastIpAddressTable(arg0: usize, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// NtGdiGetAndSetDCDword - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiGetAndSetDCDword(hdc: *mut core::ffi::c_void, method: u32, value: u32, result: *mut u32) -> usize {
    0
}

/// NtGdiGetDIBitsInternal - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiGetDIBitsInternal(hdc: *mut core::ffi::c_void, hbitmap: *mut core::ffi::c_void, startscan: u32, lines: u32, bits: *mut core::ffi::c_void, info: *mut core::ffi::c_void, coloruse: u32, max_bits: u32, max_info: u32) -> usize {
    0
}

/// NtGdiGetOutlineTextMetricsInternalW - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiGetOutlineTextMetricsInternalW(hdc: *mut core::ffi::c_void, cbData: u32, otm: *mut core::ffi::c_void, opts: u32) -> usize {
    0
}

/// NtGdiRectangle - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiRectangle(hdc: *mut core::ffi::c_void, left: i32, top: i32, right: i32, bottom: i32) -> usize {
    0
}

/// NtGdiStretchDIBitsInternal - from wine/ntgdi.h
#[no_mangle]
pub unsafe extern "C" fn NtGdiStretchDIBitsInternal(hdc: *mut core::ffi::c_void, x_dst: i32, y_dst: i32, width_dst: i32, height_dst: i32, x_src: i32, y_src: i32, width_src: i32, height_src: i32, bits: *mut core::ffi::c_void, bmi: *mut core::ffi::c_void, coloruse: u32, rop: u32, max_info: u32, max_bits: u32, xform: *mut core::ffi::c_void) -> usize {
    0
}

/// LocateCatalogsA - from wine/ntquery.h
#[no_mangle]
pub unsafe extern "C" fn LocateCatalogsA(arg0: *mut i8, arg1: u32, arg2: *mut i8, arg3: *mut u32, arg4: *mut i8, arg5: *mut u32) -> usize {
    0
}

/// LocateCatalogsW - from wine/ntquery.h
#[no_mangle]
pub unsafe extern "C" fn LocateCatalogsW(arg0: *mut u16, arg1: u32, arg2: *mut u16, arg3: *mut u32, arg4: *mut u16, arg5: *mut u32) -> usize {
    0
}

/// LsaDeregisterLogonProcess - from wine/ntsecapi.h
#[no_mangle]
pub unsafe extern "C" fn LsaDeregisterLogonProcess(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// LsaEnumerateLogonSessions - from wine/ntsecapi.h
#[no_mangle]
pub unsafe extern "C" fn LsaEnumerateLogonSessions(arg0: usize, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// LsaGetLogonSessionData - from wine/ntsecapi.h
#[no_mangle]
pub unsafe extern "C" fn LsaGetLogonSessionData(arg0: usize, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// LsaLogonUser - from wine/ntsecapi.h
#[no_mangle]
pub unsafe extern "C" fn LsaLogonUser(arg0: *mut core::ffi::c_void, arg1: usize, arg2: usize, arg3: u32, arg4: *mut core::ffi::c_void, arg5: u32, arg6: usize, arg7: usize, arg8: *mut *mut core::ffi::c_void, arg9: usize, arg10: usize, arg11: usize, arg12: usize, arg13: usize) -> i32 {
    0
}

/// LsaRegisterLogonProcess - from wine/ntsecapi.h
#[no_mangle]
pub unsafe extern "C" fn LsaRegisterLogonProcess(arg0: usize, arg1: usize, arg2: usize) -> i32 {
    0
}

/// NtUserGetAncestor - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserGetAncestor(hwnd: *mut core::ffi::c_void, arg1: u32) -> usize {
    0
}

/// NtUserGetClassInfoEx - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserGetClassInfoEx(instance: *mut core::ffi::c_void, name: *mut core::ffi::c_void, wc: *mut core::ffi::c_void, menu_name: *mut core::ffi::c_void, ansi: i32) -> usize {
    0
}

/// NtUserLogicalToPerMonitorDPIPhysicalPoint - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserLogicalToPerMonitorDPIPhysicalPoint(hwnd: *mut core::ffi::c_void, pt: *mut core::ffi::c_void) -> usize {
    0
}

/// NtUserPerMonitorDPIPhysicalToLogicalPoint - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserPerMonitorDPIPhysicalToLogicalPoint(hwnd: *mut core::ffi::c_void, pt: *mut core::ffi::c_void) -> usize {
    0
}

/// NtUserSystemParametersInfo - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserSystemParametersInfo(action: u32, val: u32, ptr: *mut core::ffi::c_void, winini: u32) -> usize {
    0
}

/// NtUserSystemParametersInfoForDpi - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserSystemParametersInfoForDpi(action: u32, val: u32, ptr: *mut core::ffi::c_void, winini: u32, dpi: u32) -> usize {
    0
}

/// NtUserGetDialogBaseUnits - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserGetDialogBaseUnits() -> u32 {
    0
}

/// NtUserGetDialogProc - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserGetDialogProc(proc: usize, ansi: i32) -> usize {
    0
}

/// NtUserSetDialogInfo - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserSetDialogInfo(hwnd: *mut core::ffi::c_void, info: *mut core::ffi::c_void) {

}

/// NtUserExposeWindowSurface - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserExposeWindowSurface(hwnd: *mut core::ffi::c_void, flags: u32, rect: *mut core::ffi::c_void, dpi: u32) -> i32 {
    0
}

/// CoCreateInstance - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CoCreateInstance(arg0: usize, arg1: usize, arg2: u32, arg3: usize, arg4: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CoCreateInstanceEx - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CoCreateInstanceEx(arg0: usize, arg1: usize, arg2: u32, arg3: *mut core::ffi::c_void, arg4: u32, arg5: *mut core::ffi::c_void) -> usize {
    0
}

/// CoCreateInstanceFromApp - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CoCreateInstanceFromApp(arg0: usize, arg1: *mut core::ffi::c_void, arg2: u32, arg3: *mut core::ffi::c_void, arg4: u32, arg5: *mut core::ffi::c_void) -> usize {
    0
}

/// CoGetInstanceFromFile - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CoGetInstanceFromFile(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: u32, arg4: u32, arg5: *mut core::ffi::c_void, arg6: u32, arg7: *mut core::ffi::c_void) -> usize {
    0
}

/// CoGetInstanceFromIStorage - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CoGetInstanceFromIStorage(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: u32, arg4: *mut core::ffi::c_void, arg5: u32, arg6: *mut core::ffi::c_void) -> usize {
    0
}

/// CoSuspendClassObjects - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CoSuspendClassObjects() -> usize {
    0
}

/// CoGetStandardMarshal - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CoGetStandardMarshal(riid: usize, pUnk: usize, dwDestContext: u32, pvDestContext: *mut core::ffi::c_void, mshlflags: u32, ppMarshal: *mut core::ffi::c_void) -> usize {
    0
}

/// CoSwitchCallContext - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CoSwitchCallContext(pContext: *mut core::ffi::c_void, ppOldContext: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CoSetProxyBlanket - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CoSetProxyBlanket(pProxy: *mut core::ffi::c_void, dwAuthnSvc: u32, dwAuthzSvc: u32, pServerPrincName: *mut core::ffi::c_void, dwAuthnLevel: u32, dwImpLevel: u32, pAuthInfo: usize, dwCapabilities: u32) -> usize {
    0
}

/// VarR8Pow - from wine/oleauto.h
#[no_mangle]
pub unsafe extern "C" fn VarR8Pow(arg0: f64, arg1: f64, arg2: *mut f64) -> usize {
    0
}

/// VarPow - from wine/oleauto.h
#[no_mangle]
pub unsafe extern "C" fn VarPow(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// PerfDeleteInstance - from wine/perflib.h
#[no_mangle]
pub unsafe extern "C" fn PerfDeleteInstance(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> u32 {
    0
}

/// EFFECTIVE_POWER_MODE_CALLBACK - from wine/powersetting.h
#[no_mangle]
pub unsafe extern "C" fn EFFECTIVE_POWER_MODE_CALLBACK(mode: usize, context: *mut core::ffi::c_void) -> usize {
    0
}

/// PowerRegisterForEffectivePowerModeNotifications - from wine/powersetting.h
#[no_mangle]
pub unsafe extern "C" fn PowerRegisterForEffectivePowerModeNotifications(arg0: u32, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// CallNtPowerInformation - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn CallNtPowerInformation(arg0: usize, arg1: *mut core::ffi::c_void, arg2: u32, arg3: *mut core::ffi::c_void, arg4: u32) -> i32 {
    0
}

/// GetCurrentPowerPolicies - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn GetCurrentPowerPolicies(arg0: usize, arg1: usize) -> usize {
    0
}

/// PowerEnumerate - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn PowerEnumerate(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: usize, arg4: u32, arg5: *mut u8, arg6: *mut u32) -> u32 {
    0
}

/// PowerRegisterSuspendResumeNotification - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn PowerRegisterSuspendResumeNotification(arg0: u32, arg1: *mut core::ffi::c_void, arg2: usize) -> u32 {
    0
}

/// PowerUnregisterSuspendResumeNotification - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn PowerUnregisterSuspendResumeNotification(arg0: usize) -> u32 {
    0
}

/// PowerSettingRegisterNotification - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn PowerSettingRegisterNotification(arg0: *mut core::ffi::c_void, arg1: u32, arg2: *mut core::ffi::c_void, arg3: usize) -> u32 {
    0
}

/// PowerSettingUnregisterNotification - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn PowerSettingUnregisterNotification(arg0: usize) -> u32 {
    0
}

/// PowerDeterminePlatformRole - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn PowerDeterminePlatformRole() -> usize {
    0
}

/// RoActivateInstance - from wine/roapi.h
#[no_mangle]
pub unsafe extern "C" fn RoActivateInstance(classid: usize, instance: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// SetupCloseLog - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupCloseLog() -> usize {
    0
}

/// SetupDiBuildClassInfoList - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiBuildClassInfoList(arg0: u32, arg1: usize, arg2: u32, arg3: usize) -> usize {
    0
}

/// SetupDiBuildClassInfoListExA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiBuildClassInfoListExA(arg0: u32, arg1: usize, arg2: u32, arg3: usize, arg4: usize, arg5: *mut core::ffi::c_void) -> usize {
    0
}

/// SetupDiBuildClassInfoListExW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiBuildClassInfoListExW(arg0: u32, arg1: usize, arg2: u32, arg3: usize, arg4: usize, arg5: *mut core::ffi::c_void) -> usize {
    0
}

/// SetupDiCallClassInstaller - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiCallClassInstaller(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// SetupDiGetClassInstallParamsA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiGetClassInstallParamsA(arg0: usize, arg1: usize, arg2: usize, arg3: u32, arg4: usize) -> usize {
    0
}

/// SetupDiGetClassInstallParamsW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiGetClassInstallParamsW(arg0: usize, arg1: usize, arg2: usize, arg3: u32, arg4: usize) -> usize {
    0
}

/// SetupDiGetDeviceInstanceIdA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiGetDeviceInstanceIdA(arg0: usize, arg1: usize, arg2: usize, arg3: u32, arg4: usize) -> usize {
    0
}

/// SetupDiGetDeviceInstanceIdW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiGetDeviceInstanceIdW(arg0: usize, arg1: usize, arg2: usize, arg3: u32, arg4: usize) -> usize {
    0
}

/// SetupDiSetClassInstallParamsA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiSetClassInstallParamsA(arg0: usize, arg1: usize, arg2: usize, arg3: u32) -> usize {
    0
}

/// SetupDiSetClassInstallParamsW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupDiSetClassInstallParamsW(arg0: usize, arg1: usize, arg2: usize, arg3: u32) -> usize {
    0
}

/// SetupInitializeFileLogA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupInitializeFileLogA(arg0: usize, arg1: u32) -> usize {
    0
}

/// SetupInitializeFileLogW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupInitializeFileLogW(arg0: usize, arg1: u32) -> usize {
    0
}

/// SetupLogErrorA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupLogErrorA(arg0: *const i8, arg1: usize) -> usize {
    0
}

/// SetupLogErrorW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupLogErrorW(arg0: *const u16, arg1: usize) -> usize {
    0
}

/// SetupLogFileA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupLogFileA(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: u32, arg5: usize, arg6: usize, arg7: usize, arg8: u32) -> usize {
    0
}

/// SetupLogFileW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupLogFileW(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: u32, arg5: usize, arg6: usize, arg7: usize, arg8: u32) -> usize {
    0
}

/// SetupOpenLog - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupOpenLog(arg0: i32) -> usize {
    0
}

/// SetupQueryDrivesInDiskSpaceListA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupQueryDrivesInDiskSpaceListA(arg0: usize, arg1: usize, arg2: u32, arg3: usize) -> usize {
    0
}

/// SetupQueryDrivesInDiskSpaceListW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupQueryDrivesInDiskSpaceListW(arg0: usize, arg1: usize, arg2: u32, arg3: usize) -> usize {
    0
}

/// SetupQueryFileLogA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupQueryFileLogA(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: u32, arg6: usize) -> usize {
    0
}

/// SetupQueryFileLogW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupQueryFileLogW(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: u32, arg6: usize) -> usize {
    0
}

/// SetupRemoveFileLogEntryA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupRemoveFileLogEntryA(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// SetupRemoveFileLogEntryW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupRemoveFileLogEntryW(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// SetupTerminateFileLog - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupTerminateFileLog(arg0: usize) -> usize {
    0
}

/// SHCoCreateInstance - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHCoCreateInstance(arg0: *const u16, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: usize, arg4: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SHGetInstanceExplorer - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHGetInstanceExplorer(arg0: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SHStartNetConnectionDialog - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHStartNetConnectionDialog(arg0: *mut core::ffi::c_void, arg1: *const i8, arg2: u32) -> usize {
    0
}

/// RestartDialog - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn RestartDialog(arg0: *mut core::ffi::c_void, arg1: *const u16, arg2: u32) -> usize {
    0
}

/// RestartDialogEx - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn RestartDialogEx(arg0: *mut core::ffi::c_void, arg1: *const u16, arg2: u32, arg3: u32) -> usize {
    0
}

/// SHOpenWithDialog - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHOpenWithDialog(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// PathYetAnotherMakeUniqueName - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn PathYetAnotherMakeUniqueName(arg0: *mut u16, arg1: *const u16, arg2: *const u16, arg3: *const u16) -> usize {
    0
}

/// PathUnExpandEnvStringsA - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn PathUnExpandEnvStringsA(arg0: *const i8, arg1: *mut i8, arg2: u32) -> usize {
    0
}

/// PathUnExpandEnvStringsW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn PathUnExpandEnvStringsW(arg0: *const u16, arg1: *mut u16, arg2: u32) -> usize {
    0
}

/// StrIsIntlEqualA - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn StrIsIntlEqualA(arg0: i32, arg1: *const i8, arg2: *const i8, arg3: i32) -> usize {
    0
}

/// StrIsIntlEqualW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn StrIsIntlEqualW(arg0: i32, arg1: *const u16, arg2: *const u16, arg3: i32) -> usize {
    0
}

/// IsInternetESCEnabled - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn IsInternetESCEnabled() -> usize {
    0
}

/// SLGetLicensingStatusInformation - from wine/slpublic.h
#[no_mangle]
pub unsafe extern "C" fn SLGetLicensingStatusInformation(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *const u16, arg4: *mut u32, arg5: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SLGetWindowsInformation - from wine/slpublic.h
#[no_mangle]
pub unsafe extern "C" fn SLGetWindowsInformation(arg0: *const u16, arg1: *mut core::ffi::c_void, arg2: *mut u32, arg3: *mut core::ffi::c_void) -> usize {
    0
}

/// SLGetWindowsInformationDWORD - from wine/slpublic.h
#[no_mangle]
pub unsafe extern "C" fn SLGetWindowsInformationDWORD(arg0: *const u16, arg1: usize) -> usize {
    0
}

/// SnmpSvcSetLogLevel - from wine/snmp.h
#[no_mangle]
pub unsafe extern "C" fn SnmpSvcSetLogLevel(nLogLevel: i32) -> usize {
    0
}

/// SnmpSvcSetLogType - from wine/snmp.h
#[no_mangle]
pub unsafe extern "C" fn SnmpSvcSetLogType(nLogType: i32) -> usize {
    0
}

/// TraceOpenLogFile - from wine/sqlext.h
#[no_mangle]
pub unsafe extern "C" fn TraceOpenLogFile(arg0: *mut u16, arg1: *mut u16, arg2: u32) -> usize {
    0
}

/// TraceCloseLogFile - from wine/sqlext.h
#[no_mangle]
pub unsafe extern "C" fn TraceCloseLogFile() -> usize {
    0
}

/// ExportSecurityContext - from wine/sspi.h
#[no_mangle]
pub unsafe extern "C" fn ExportSecurityContext(phContext: usize, fFlags: u32, pPackedContext: usize, pToken: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// StiCreateInstanceA - from wine/sti.h
#[no_mangle]
pub unsafe extern "C" fn StiCreateInstanceA(hinst: *mut core::ffi::c_void, dwVer: u32, ppSti: *mut core::ffi::c_void, pUnkOuter: usize) -> i32 {
    0
}

/// StiCreateInstanceW - from wine/sti.h
#[no_mangle]
pub unsafe extern "C" fn StiCreateInstanceW(hinst: *mut core::ffi::c_void, dwVer: u32, ppSti: *mut core::ffi::c_void, pUnkOuter: usize) -> i32 {
    0
}

/// WakeByAddressSingle - from wine/synchapi.h
#[no_mangle]
pub unsafe extern "C" fn WakeByAddressSingle(arg0: *mut core::ffi::c_void) {

}

/// GetLogicalProcessorInformation - from wine/sysinfoapi.h
#[no_mangle]
pub unsafe extern "C" fn GetLogicalProcessorInformation(arg0: usize, arg1: usize) -> usize {
    0
}

/// GetLogicalProcessorInformationEx - from wine/sysinfoapi.h
#[no_mangle]
pub unsafe extern "C" fn GetLogicalProcessorInformationEx(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// lineConfigDialog - from wine/tapi.h
#[no_mangle]
pub unsafe extern "C" fn lineConfigDialog(arg0: u32, arg1: *mut core::ffi::c_void, arg2: *const i8) -> u32 {
    0
}

/// lineConfigDialogEdit - from wine/tapi.h
#[no_mangle]
pub unsafe extern "C" fn lineConfigDialogEdit(arg0: u32, arg1: *mut core::ffi::c_void, arg2: *const i8, arg3: *mut core::ffi::c_void, arg4: u32, arg5: usize) -> u32 {
    0
}

/// lineTranslateDialog - from wine/tapi.h
#[no_mangle]
pub unsafe extern "C" fn lineTranslateDialog(arg0: usize, arg1: u32, arg2: u32, arg3: *mut core::ffi::c_void, arg4: *const i8) -> u32 {
    0
}

/// lineConfigDialogA - from wine/tapi.h
#[no_mangle]
pub unsafe extern "C" fn lineConfigDialogA(arg0: u32, arg1: *mut core::ffi::c_void, arg2: *const i8) -> u32 {
    0
}

/// lineConfigDialogEditA - from wine/tapi.h
#[no_mangle]
pub unsafe extern "C" fn lineConfigDialogEditA(arg0: u32, arg1: *mut core::ffi::c_void, arg2: *const i8, arg3: *mut core::ffi::c_void, arg4: u32, arg5: usize) -> u32 {
    0
}

/// lineTranslateDialogA - from wine/tapi.h
#[no_mangle]
pub unsafe extern "C" fn lineTranslateDialogA(arg0: usize, arg1: u32, arg2: u32, arg3: *mut core::ffi::c_void, arg4: *const i8) -> u32 {
    0
}

/// phoneConfigDialog - from wine/tapi.h
#[no_mangle]
pub unsafe extern "C" fn phoneConfigDialog(arg0: u32, arg1: *mut core::ffi::c_void, arg2: *const i8) -> u32 {
    0
}

/// phoneConfigDialogA - from wine/tapi.h
#[no_mangle]
pub unsafe extern "C" fn phoneConfigDialogA(arg0: u32, arg1: *mut core::ffi::c_void, arg2: *const i8) -> u32 {
    0
}

/// ExpandEnvironmentStringsForUserA - from wine/userenv.h
#[no_mangle]
pub unsafe extern "C" fn ExpandEnvironmentStringsForUserA(arg0: *mut core::ffi::c_void, arg1: *const i8, arg2: *mut i8, arg3: u32) -> usize {
    0
}

/// ExpandEnvironmentStringsForUserW - from wine/userenv.h
#[no_mangle]
pub unsafe extern "C" fn ExpandEnvironmentStringsForUserW(arg0: *mut core::ffi::c_void, arg1: *const u16, arg2: *mut u16, arg3: u32) -> usize {
    0
}

/// ScriptApplyLogicalWidth - from wine/usp10.h
#[no_mangle]
pub unsafe extern "C" fn ScriptApplyLogicalWidth(piDx: *mut i32, cChars: i32, cGlyphs: i32, pwLogClust: *mut u16, psva: *mut core::ffi::c_void, piAdvance: *mut i32, psa: *mut core::ffi::c_void, pABC: *mut core::ffi::c_void, piJustify: *mut i32) -> i32 {
    0
}

/// ScriptGetLogicalWidths - from wine/usp10.h
#[no_mangle]
pub unsafe extern "C" fn ScriptGetLogicalWidths(psa: *mut core::ffi::c_void, cChars: i32, cGlyphs: i32, piGlyphWidth: *mut i32, pwLogClust: *mut u16, psva: *mut core::ffi::c_void, piDx: *mut i32) -> i32 {
    0
}

/// ScriptStringGetLogicalWidths - from wine/usp10.h
#[no_mangle]
pub unsafe extern "C" fn ScriptStringGetLogicalWidths(ssa: usize, piDx: *mut i32) -> i32 {
    0
}

/// ScriptString_pLogAttr - from wine/usp10.h
#[no_mangle]
pub unsafe extern "C" fn ScriptString_pLogAttr(ssa: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// EnableThemeDialogTexture - from reactos/uxthemesupp.h
#[no_mangle]
pub unsafe extern "C" fn EnableThemeDialogTexture(hwnd: usize, dwFlags: usize) -> i32 {
    0
}

/// GetThemeSysInt - from wine/uxtheme.h
#[no_mangle]
pub unsafe extern "C" fn GetThemeSysInt(arg0: usize, arg1: i32, arg2: *mut i32) -> usize {
    0
}

/// WsInitializeMessage - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsInitializeMessage(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// CryptExportKey - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CryptExportKey(arg0: usize, arg1: usize, arg2: u32, arg3: u32, arg4: *mut u8, arg5: *mut u32) -> usize {
    0
}

/// CertEnumCertificatesInStore - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CertEnumCertificatesInStore(hCertStore: usize, pPrev: usize) -> usize {
    0
}

/// CertEnumCRLsInStore - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CertEnumCRLsInStore(hCertStore: usize, pPrev: usize) -> usize {
    0
}

/// CertEnumCTLsInStore - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CertEnumCTLsInStore(hCertStore: usize, pPrev: usize) -> usize {
    0
}

/// CertIsRDNAttrsInCertificateName - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CertIsRDNAttrsInCertificateName(dwCertEncodingType: u32, dwFlags: u32, pCertName: usize, pRDN: usize) -> usize {
    0
}

/// CryptExportPublicKeyInfo - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CryptExportPublicKeyInfo(hCryptProv: usize, dwKeySpec: u32, dwCertEncodingType: u32, pInfo: usize, pcbInfo: *mut u32) -> usize {
    0
}

/// CryptExportPublicKeyInfoEx - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CryptExportPublicKeyInfoEx(hCryptProv: usize, dwKeySpec: u32, dwCertEncodingType: u32, pszPublicKeyObjId: *mut i8, dwFlags: u32, pvAuxInfo: *mut core::ffi::c_void, pInfo: usize, pcbInfo: *mut u32) -> usize {
    0
}

/// CryptMsgGetAndVerifySigner - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CryptMsgGetAndVerifySigner(hCryptMsg: usize, cSignerStore: u32, rghSignerStore: *mut core::ffi::c_void, dwFlags: u32, ppSigner: *mut core::ffi::c_void, pdwSignerIndex: *mut u32) -> usize {
    0
}

/// CryptDecryptAndVerifyMessageSignature - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn CryptDecryptAndVerifyMessageSignature(pDecryptPara: usize, pVerifyPara: usize, dwSignerIndex: u32, pbEncryptedBlob: *mut u8, cbEncryptedBlob: u32, pbDecrypted: *mut u8, pcbDecrypted: *mut u32, ppXchgCert: *mut core::ffi::c_void, ppSignerCert: *mut core::ffi::c_void) -> usize {
    0
}

/// PFXExportCertStoreEx - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn PFXExportCertStoreEx(hStore: usize, pPFX: *mut core::ffi::c_void, szPassword: *const u16, pvReserved: *mut core::ffi::c_void, dwFlags: u32) -> usize {
    0
}

/// PFXExportCertStore - from wine/wincrypt.h
#[no_mangle]
pub unsafe extern "C" fn PFXExportCertStore(hStore: usize, pPFX: *mut core::ffi::c_void, szPassword: *const u16, dwFlags: u32) -> usize {
    0
}

/// DnsModifyRecordsInSet_A - from wine/windns.h
#[no_mangle]
pub unsafe extern "C" fn DnsModifyRecordsInSet_A(arg0: usize, arg1: usize, arg2: u32, arg3: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void, arg5: *mut core::ffi::c_void) -> usize {
    0
}

/// DnsModifyRecordsInSet_W - from wine/windns.h
#[no_mangle]
pub unsafe extern "C" fn DnsModifyRecordsInSet_W(arg0: usize, arg1: usize, arg2: u32, arg3: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void, arg5: *mut core::ffi::c_void) -> usize {
    0
}

/// DnsModifyRecordsInSet_UTF8 - from wine/windns.h
#[no_mangle]
pub unsafe extern "C" fn DnsModifyRecordsInSet_UTF8(arg0: usize, arg1: usize, arg2: u32, arg3: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void, arg5: *mut core::ffi::c_void) -> usize {
    0
}

/// EvtExportLog - from wine/winevt.h
#[no_mangle]
pub unsafe extern "C" fn EvtExportLog(session: usize, path: *mut u16, query: *mut u16, file: *mut u16, flags: u32) -> i32 {
    0
}

/// InternetConfirmZoneCrossingA - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn InternetConfirmZoneCrossingA(arg0: *mut core::ffi::c_void, arg1: *mut i8, arg2: *mut i8, arg3: i32) -> usize {
    0
}

/// InternetConfirmZoneCrossingW - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn InternetConfirmZoneCrossingW(arg0: *mut core::ffi::c_void, arg1: *mut u16, arg2: *mut u16, arg3: i32) -> usize {
    0
}

/// IsUrlCacheEntryExpiredA - from wine/winineti.h
#[no_mangle]
pub unsafe extern "C" fn IsUrlCacheEntryExpiredA(arg0: *const i8, arg1: u32, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// IsUrlCacheEntryExpiredW - from wine/winineti.h
#[no_mangle]
pub unsafe extern "C" fn IsUrlCacheEntryExpiredW(arg0: *const u16, arg1: u32, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// ldap_explode_dnA - from wine/winldap.h
#[no_mangle]
pub unsafe extern "C" fn ldap_explode_dnA(arg0: *mut i8, arg1: u32) -> *mut *mut i8 {
    core::ptr::null_mut()
}

/// ldap_explode_dnW - from wine/winldap.h
#[no_mangle]
pub unsafe extern "C" fn ldap_explode_dnW(arg0: *mut u16, arg1: u32) -> *mut *mut u16 {
    core::ptr::null_mut()
}

/// WNetConnectionDialog - from wine/winnetwk.h
#[no_mangle]
pub unsafe extern "C" fn WNetConnectionDialog(arg0: *mut core::ffi::c_void, arg1: u32) -> u32 {
    0
}

/// WNetDisconnectDialog - from wine/winnetwk.h
#[no_mangle]
pub unsafe extern "C" fn WNetDisconnectDialog(arg0: *mut core::ffi::c_void, arg1: u32) -> u32 {
    0
}

/// WNetConnectionDialog1A - from wine/winnetwk.h
#[no_mangle]
pub unsafe extern "C" fn WNetConnectionDialog1A(arg0: usize) -> u32 {
    0
}

/// WNetConnectionDialog1W - from wine/winnetwk.h
#[no_mangle]
pub unsafe extern "C" fn WNetConnectionDialog1W(arg0: usize) -> u32 {
    0
}

/// WNetDisconnectDialog1A - from wine/winnetwk.h
#[no_mangle]
pub unsafe extern "C" fn WNetDisconnectDialog1A(arg0: usize) -> u32 {
    0
}

/// WNetDisconnectDialog1W - from wine/winnetwk.h
#[no_mangle]
pub unsafe extern "C" fn WNetDisconnectDialog1W(arg0: usize) -> u32 {
    0
}

/// load_export_name - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn load_export_name(ret_buf: *mut *mut i8, data: *mut core::ffi::c_void, align_mask: usize, unix_fd: i32, sec: *mut core::ffi::c_void, nb_sec: u32) -> i32 {
    0
}

/// set_clip_rectangle - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn set_clip_rectangle(desktop: *mut core::ffi::c_void, rect: *mut core::ffi::c_void, flags: u32, reset: i32) {

}

/// validate_rectangles - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn validate_rectangles(rects: *mut core::ffi::c_void, nb_rects: u32) -> i32 {
    0
}

/// get_mach_importance - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn get_mach_importance(effective_priority: i32) -> i32 {
    0
}

/// dump_rectangle - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn dump_rectangle(prefix: *mut i8, rect: *mut core::ffi::c_void) {

}

/// dump_varargs_rectangles - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn dump_varargs_rectangles(prefix: *mut i8, size: usize) {

}

/// is_using_msvcrt - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn is_using_msvcrt(make: *mut core::ffi::c_void) -> usize {
    0
}

/// get_expanded_make_var_array - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn get_expanded_make_var_array(make: *mut core::ffi::c_void, name: *mut i8) -> usize {
    0
}

/// get_expanded_file_local_var - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn get_expanded_file_local_var(make: *mut core::ffi::c_void, file: *mut i8, name: *mut i8) -> usize {
    0
}

/// get_expanded_make_variable - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn get_expanded_make_variable(arg0: usize, s_s: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// factory_CreateInstance - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn factory_CreateInstance(iface: *mut core::ffi::c_void, outer: *mut core::ffi::c_void, riid: usize, obj: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// IParentAndItem_GetParentAndItem_Stub - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IParentAndItem_GetParentAndItem_Stub(This: *mut core::ffi::c_void, parent: *mut core::ffi::c_void, folder: *mut *mut core::ffi::c_void, child: *mut core::ffi::c_void) -> usize {
    0
}

/// IParentAndItem_GetParentAndItem - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IParentAndItem_GetParentAndItem(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// IParentAndItem_GetParentAndItem_Proxy - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IParentAndItem_GetParentAndItem_Proxy(This: *mut core::ffi::c_void, parent: *mut core::ffi::c_void, folder: *mut *mut core::ffi::c_void, child: *mut core::ffi::c_void) -> usize {
    0
}

/// IParentAndItem_RemoteGetParentAndItem_Proxy - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IParentAndItem_RemoteGetParentAndItem_Proxy(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// sysinfo_QueryInterface - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_QueryInterface(iface: *mut core::ffi::c_void, riid: usize, obj: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// sysinfo_AddRef - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_AddRef(iface: *mut core::ffi::c_void) -> u32 {
    0
}

/// sysinfo_Release - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_Release(iface: *mut core::ffi::c_void) -> u32 {
    0
}

/// sysinfo_GetTypeInfoCount - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_GetTypeInfoCount(iface: *mut core::ffi::c_void, count: *mut u32) -> i32 {
    0
}

/// sysinfo_GetTypeInfo - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_GetTypeInfo(iface: *mut core::ffi::c_void, index: u32, lcid: usize, info: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// sysinfo_GetIDsOfNames - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_GetIDsOfNames(iface: *mut core::ffi::c_void, riid: usize, names: *mut core::ffi::c_void, count: u32, lcid: usize, dispid: *mut core::ffi::c_void) -> i32 {
    0
}

/// sysinfo_get_UserName - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_UserName(iface: *mut core::ffi::c_void, retval: *mut core::ffi::c_void) -> i32 {
    0
}

/// sysinfo_get_ComputerName - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_ComputerName(iface: *mut core::ffi::c_void, retval: *mut core::ffi::c_void) -> i32 {
    0
}

/// sysinfo_get_SiteName - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_SiteName(iface: *mut core::ffi::c_void, retval: *mut core::ffi::c_void) -> i32 {
    0
}

/// sysinfo_get_DomainShortName - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_DomainShortName(iface: *mut core::ffi::c_void, retval: *mut core::ffi::c_void) -> i32 {
    0
}

/// sysinfo_get_DomainDNSName - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_DomainDNSName(iface: *mut core::ffi::c_void, retval: *mut core::ffi::c_void) -> i32 {
    0
}

/// sysinfo_get_ForestDNSName - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_ForestDNSName(iface: *mut core::ffi::c_void, retval: *mut core::ffi::c_void) -> i32 {
    0
}

/// sysinfo_get_PDCRoleOwner - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_PDCRoleOwner(iface: *mut core::ffi::c_void, retval: *mut core::ffi::c_void) -> i32 {
    0
}

/// sysinfo_get_SchemaRoleOwner - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_SchemaRoleOwner(iface: *mut core::ffi::c_void, retval: *mut core::ffi::c_void) -> i32 {
    0
}

/// sysinfo_get_IsNativeMode - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_IsNativeMode(iface: *mut core::ffi::c_void, retval: *mut core::ffi::c_void) -> i32 {
    0
}

/// sysinfo_GetAnyDCName - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_GetAnyDCName(iface: *mut core::ffi::c_void, retval: *mut core::ffi::c_void) -> i32 {
    0
}

/// sysinfo_GetDCSiteName - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_GetDCSiteName(iface: *mut core::ffi::c_void, server: usize, retval: *mut core::ffi::c_void) -> i32 {
    0
}

/// sysinfo_RefreshSchemaCache - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_RefreshSchemaCache(iface: *mut core::ffi::c_void) -> i32 {
    0
}

/// sysinfo_GetTrees - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn sysinfo_GetTrees(iface: *mut core::ffi::c_void, retval: *mut core::ffi::c_void) -> i32 {
    0
}

/// WmiSetSingleInstanceA - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn WmiSetSingleInstanceA(handle: usize, name: *mut i8, reserved: u32, size: u32, buffer: *mut core::ffi::c_void) -> usize {
    0
}

/// WmiSetSingleInstanceW - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn WmiSetSingleInstanceW(handle: usize, name: *mut u16, reserved: u32, size: u32, buffer: *mut core::ffi::c_void) -> usize {
    0
}

/// WmiSetSingleItemA - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn WmiSetSingleItemA(handle: usize, name: *mut i8, id: u32, reserved: u32, size: u32, buffer: *mut core::ffi::c_void) -> usize {
    0
}

/// WmiSetSingleItemW - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn WmiSetSingleItemW(handle: usize, name: *mut u16, id: u32, reserved: u32, size: u32, buffer: *mut core::ffi::c_void) -> usize {
    0
}

/// audio_sink_QueryInterface - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, out: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_sink_AddRef - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_AddRef(iface: *mut core::ffi::c_void) -> u32 {
    0
}

/// audio_sink_Release - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_Release(iface: *mut core::ffi::c_void) -> u32 {
    0
}

/// audio_sink_Connect - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_Connect(iface: *mut core::ffi::c_void, peer: *mut core::ffi::c_void, mt: *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_sink_ReceiveConnection - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_ReceiveConnection(iface: *mut core::ffi::c_void, peer: *mut core::ffi::c_void, mt: *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_sink_Disconnect - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_Disconnect(iface: *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_sink_ConnectedTo - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_ConnectedTo(iface: *mut core::ffi::c_void, peer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_sink_ConnectionMediaType - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_ConnectionMediaType(iface: *mut core::ffi::c_void, mt: *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_sink_QueryPinInfo - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_QueryPinInfo(iface: *mut core::ffi::c_void, info: *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_sink_QueryDirection - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_QueryDirection(iface: *mut core::ffi::c_void, dir: *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_sink_QueryId - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_QueryId(iface: *mut core::ffi::c_void, id: *mut *mut u16) -> i32 {
    0
}

/// audio_sink_QueryAccept - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_QueryAccept(iface: *mut core::ffi::c_void, mt: *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_sink_EnumMediaTypes - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_EnumMediaTypes(iface: *mut core::ffi::c_void, enum_media_types: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_sink_QueryInternalConnections - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_QueryInternalConnections(iface: *mut core::ffi::c_void, pins: *mut *mut core::ffi::c_void, count: *mut u32) -> i32 {
    0
}

/// audio_sink_EndOfStream - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_EndOfStream(iface: *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_sink_BeginFlush - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_BeginFlush(iface: *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_sink_EndFlush - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_EndFlush(iface: *mut core::ffi::c_void) -> i32 {
    0
}

/// audio_sink_NewSegment - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn audio_sink_NewSegment(iface: *mut core::ffi::c_void, start: usize, stop: usize, rate: f64) -> i32 {
    0
}

/// ddraw_sink_QueryInterface - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_QueryInterface(iface: *mut core::ffi::c_void, iid: usize, out: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_sink_AddRef - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_AddRef(iface: *mut core::ffi::c_void) -> u32 {
    0
}

/// ddraw_sink_Release - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_Release(iface: *mut core::ffi::c_void) -> u32 {
    0
}

/// ddraw_sink_Connect - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_Connect(iface: *mut core::ffi::c_void, peer: *mut core::ffi::c_void, mt: *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_sink_ReceiveConnection - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_ReceiveConnection(iface: *mut core::ffi::c_void, peer: *mut core::ffi::c_void, mt: *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_sink_Disconnect - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_Disconnect(iface: *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_sink_ConnectedTo - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_ConnectedTo(iface: *mut core::ffi::c_void, peer: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_sink_ConnectionMediaType - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_ConnectionMediaType(iface: *mut core::ffi::c_void, mt: *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_sink_QueryPinInfo - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_QueryPinInfo(iface: *mut core::ffi::c_void, info: *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_sink_QueryDirection - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_QueryDirection(iface: *mut core::ffi::c_void, dir: *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_sink_QueryId - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_QueryId(iface: *mut core::ffi::c_void, id: *mut *mut u16) -> i32 {
    0
}

/// ddraw_sink_QueryAccept - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_QueryAccept(iface: *mut core::ffi::c_void, mt: *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_sink_EnumMediaTypes - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_EnumMediaTypes(iface: *mut core::ffi::c_void, enum_media_types: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_sink_QueryInternalConnections - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_QueryInternalConnections(iface: *mut core::ffi::c_void, pins: *mut *mut core::ffi::c_void, count: *mut u32) -> i32 {
    0
}

/// ddraw_sink_EndOfStream - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_EndOfStream(iface: *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_sink_BeginFlush - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_BeginFlush(iface: *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_sink_EndFlush - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_EndFlush(iface: *mut core::ffi::c_void) -> i32 {
    0
}

/// ddraw_sink_NewSegment - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_NewSegment(iface: *mut core::ffi::c_void, start: usize, stop: usize, rate: f64) -> i32 {
    0
}

/// filter_seeking_IsUsingTimeFormat - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn filter_seeking_IsUsingTimeFormat(iface: *mut core::ffi::c_void, format: *mut core::ffi::c_void) -> i32 {
    0
}

/// AMCF_CreateInstance - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AMCF_CreateInstance(iface: *mut core::ffi::c_void, pOuter: *mut core::ffi::c_void, riid: usize, ppobj: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// SetInfoDialogText - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn SetInfoDialogText(hKey: *mut core::ffi::c_void, lpKeyName: *const u16, lpAltMessage: *const u16, hWnd: *mut core::ffi::c_void, iDlgItem: i32) {

}

/// factory_ActivateInstance - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn factory_ActivateInstance(iface: *mut core::ffi::c_void, instance: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// RegistrarCF_CreateInstance - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn RegistrarCF_CreateInstance(iface: *mut core::ffi::c_void, pUnkOuter: usize, riid: usize, ppv: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// AX_ConvertDialogTemplate - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AX_ConvertDialogTemplate(src_tmpl: usize) -> usize {
    0
}

/// AtlAxCreateDialogA - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AtlAxCreateDialogA(hInst: *mut core::ffi::c_void, name: *const i8, owner: *mut core::ffi::c_void, dlgProc: usize, param: isize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// AtlAxCreateDialogW - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AtlAxCreateDialogW(arg0: usize, arg1: usize) -> usize {
    0
}

/// AtlAxDialogBoxW - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AtlAxDialogBoxW(instance: *mut core::ffi::c_void, name: *mut u16, owner: *mut core::ffi::c_void, proc: usize, param: isize) -> usize {
    0
}

/// AtlAxDialogBoxA - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AtlAxDialogBoxA(instance: *mut core::ffi::c_void, name: *mut i8, owner: *mut core::ffi::c_void, proc: usize, param: isize) -> usize {
    0
}

/// IClassFactory_fnCreateInstance - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IClassFactory_fnCreateInstance(iface: *mut core::ffi::c_void, pOuter: *mut core::ffi::c_void, riid: usize, ppobj: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// logic_dbl2int - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn logic_dbl2int(a: *mut core::ffi::c_void) -> usize {
    0
}

/// logic_int2dbl - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn logic_int2dbl(a: *mut core::ffi::c_void) -> f64 {
    0.0
}

/// rpn_sin - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_sin(c: *mut core::ffi::c_void) {

}

/// rpn_cos - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_cos(c: *mut core::ffi::c_void) {

}

/// rpn_tan - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_tan(c: *mut core::ffi::c_void) {

}

/// rpn_asin - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_asin(c: *mut core::ffi::c_void) {

}

/// rpn_acos - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_acos(c: *mut core::ffi::c_void) {

}

/// rpn_atan - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_atan(c: *mut core::ffi::c_void) {

}

/// rpn_sinh - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_sinh(c: *mut core::ffi::c_void) {

}

/// rpn_cosh - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_cosh(c: *mut core::ffi::c_void) {

}

/// rpn_tanh - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_tanh(c: *mut core::ffi::c_void) {

}

/// rpn_asinh - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_asinh(c: *mut core::ffi::c_void) {

}

/// rpn_acosh - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_acosh(c: *mut core::ffi::c_void) {

}

/// rpn_atanh - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_atanh(c: *mut core::ffi::c_void) {

}

/// rpn_exp2 - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_exp2(c: *mut core::ffi::c_void) {

}

/// rpn_exp3 - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_exp3(c: *mut core::ffi::c_void) {

}

/// rpn_sqrt - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_sqrt(c: *mut core::ffi::c_void) {

}

/// rpn_exp - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_exp(c: *mut core::ffi::c_void) {

}

/// rpn_exp10 - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_exp10(c: *mut core::ffi::c_void) {

}

/// rpn_log - from reactos/calc.h
#[no_mangle]
pub unsafe extern "C" fn rpn_log(c: *mut core::ffi::c_void) {

}

/// DialogProc - from reactos/MainWindow.h
#[no_mangle]
pub unsafe extern "C" fn DialogProc(hwndDlg: usize, uMsg: usize, wParam: usize, lParam: usize) -> usize {
    0
}

/// EnableDialogTheme - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn EnableDialogTheme(hwnd: *mut core::ffi::c_void) -> i32 {
    0
}

/// FileExtractDialogWndProc - from reactos/fileextractdialog.h
#[no_mangle]
pub unsafe extern "C" fn FileExtractDialogWndProc(hDlg: *mut core::ffi::c_void, message: u32, wParam: usize, lParam: isize) -> usize {
    0
}

/// OnInitDialog - from reactos/dialogs.h
#[no_mangle]
pub unsafe extern "C" fn OnInitDialog(nMsg: u32, wParam: usize, lParam: isize, bHandled: usize) -> isize {
    0
}

/// OpenRDPConnectDialog - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn OpenRDPConnectDialog(hInstance: *mut core::ffi::c_void, pRdpSettings: usize) -> i32 {
    0
}

/// mppc_expand - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn mppc_expand(data: *mut core::ffi::c_void, clen: usize, ctype: usize, roff: *mut core::ffi::c_void, rlen: *mut core::ffi::c_void) -> i32 {
    0
}

/// DIALOG_FileNew - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FileNew(arg0: usize) -> usize {
    0
}

/// DIALOG_FileNewWindow - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FileNewWindow(arg0: usize) -> usize {
    0
}

/// DIALOG_FileOpen - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FileOpen(arg0: usize) -> usize {
    0
}

/// DIALOG_FileSave - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FileSave(arg0: usize) -> i32 {
    0
}

/// DIALOG_FileSaveAs - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FileSaveAs(arg0: usize) -> i32 {
    0
}

/// DIALOG_FilePrint - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FilePrint(arg0: usize) -> usize {
    0
}

/// DIALOG_FilePageSetup - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FilePageSetup(arg0: usize) -> usize {
    0
}

/// DIALOG_FileExit - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FileExit(arg0: usize) -> usize {
    0
}

/// DIALOG_EditUndo - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditUndo(arg0: usize) -> usize {
    0
}

/// DIALOG_EditCut - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditCut(arg0: usize) -> usize {
    0
}

/// DIALOG_EditCopy - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditCopy(arg0: usize) -> usize {
    0
}

/// DIALOG_EditPaste - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditPaste(arg0: usize) -> usize {
    0
}

/// DIALOG_EditDelete - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditDelete(arg0: usize) -> usize {
    0
}

/// DIALOG_EditSelectAll - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditSelectAll(arg0: usize) -> usize {
    0
}

/// DIALOG_EditTimeDate - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditTimeDate(arg0: usize) -> usize {
    0
}

/// DIALOG_EditWrap - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditWrap(arg0: usize) -> usize {
    0
}

/// DIALOG_Search - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_Search(arg0: usize) -> usize {
    0
}

/// DIALOG_SearchNext - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_SearchNext(bDown: i32) -> usize {
    0
}

/// DIALOG_Replace - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_Replace(arg0: usize) -> usize {
    0
}

/// DIALOG_GoTo - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_GoTo(arg0: usize) -> usize {
    0
}

/// DIALOG_SelectFont - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_SelectFont(arg0: usize) -> usize {
    0
}

/// DIALOG_ViewStatusBar - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_ViewStatusBar(arg0: usize) -> usize {
    0
}

/// DIALOG_StatusBarAlignParts - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_StatusBarAlignParts(arg0: usize) -> usize {
    0
}

/// DIALOG_StatusBarUpdateCaretPos - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_StatusBarUpdateCaretPos(arg0: usize) -> usize {
    0
}

/// DIALOG_HelpContents - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_HelpContents(arg0: usize) -> usize {
    0
}

/// DIALOG_HelpSearch - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_HelpSearch(arg0: usize) -> usize {
    0
}

/// DIALOG_HelpLicense - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_HelpLicense(arg0: usize) -> usize {
    0
}

/// DIALOG_HelpNoWarranty - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_HelpNoWarranty(arg0: usize) -> usize {
    0
}

/// DIALOG_HelpAboutNotepad - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_HelpAboutNotepad(arg0: usize) -> usize {
    0
}

/// DIALOG_TimeDate - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_TimeDate(arg0: usize) -> usize {
    0
}

/// DIALOG_StringMsgBox - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn DIALOG_StringMsgBox(hParent: *mut core::ffi::c_void, formatId: i32, szString: usize, dwFlags: u32) -> i32 {
    0
}

/// AboutDialogProc - from reactos/dialog.h
#[no_mangle]
pub unsafe extern "C" fn AboutDialogProc(hDlg: *mut core::ffi::c_void, message: u32, wParam: usize, lParam: isize) -> usize {
    0
}

/// FindDialog - from reactos/main.h
#[no_mangle]
pub unsafe extern "C" fn FindDialog(hWnd: *mut core::ffi::c_void) {

}

/// ExportRegistryFile - from reactos/main.h
#[no_mangle]
pub unsafe extern "C" fn ExportRegistryFile(hWnd: *mut core::ffi::c_void) -> i32 {
    0
}

/// export_registry_key - from reactos/main.h
#[no_mangle]
pub unsafe extern "C" fn export_registry_key(file_name: *mut u16, path: *mut u16, format: u32) -> i32 {
    0
}

/// OnTreeExpanding - from reactos/main.h
#[no_mangle]
pub unsafe extern "C" fn OnTreeExpanding(hWnd: *mut core::ffi::c_void, pnmtv: *mut core::ffi::c_void) -> i32 {
    0
}

/// txt_export_registry_key - from reactos/main.h
#[no_mangle]
pub unsafe extern "C" fn txt_export_registry_key(file_name: *const u16, path: *const u16) -> i32 {
    0
}

/// LoadDialogCtrls - from reactos/sndvol32.h
#[no_mangle]
pub unsafe extern "C" fn LoadDialogCtrls(PrefContext: usize) -> usize {
    0
}

/// UpdateDialogLineSliderControl - from reactos/sndvol32.h
#[no_mangle]
pub unsafe extern "C" fn UpdateDialogLineSliderControl(PrefContext: usize, Line: usize, DialogID: u32, Position: u32) -> usize {
    0
}

/// UpdateDialogLineSwitchControl - from reactos/sndvol32.h
#[no_mangle]
pub unsafe extern "C" fn UpdateDialogLineSwitchControl(PrefContext: usize, Line: usize, fValue: i32) -> usize {
    0
}

/// PerfDataGetProcessIndex - from reactos/perfdata.h
#[no_mangle]
pub unsafe extern "C" fn PerfDataGetProcessIndex(pid: u32) -> u32 {
    0
}

/// ShutDown_StandBy - from reactos/shutdown.h
#[no_mangle]
pub unsafe extern "C" fn ShutDown_StandBy(arg0: usize) -> usize {
    0
}

/// ShutDown_PowerOff - from reactos/shutdown.h
#[no_mangle]
pub unsafe extern "C" fn ShutDown_PowerOff(arg0: usize) -> usize {
    0
}

/// ShutDown_LogOffUser - from reactos/shutdown.h
#[no_mangle]
pub unsafe extern "C" fn ShutDown_LogOffUser(arg0: usize) -> usize {
    0
}

/// MACRO_CopyDialog - from reactos/macro.h
#[no_mangle]
pub unsafe extern "C" fn MACRO_CopyDialog() {

}

/// Winver_GetOSInfo - from reactos/winver_p.h
#[no_mangle]
pub unsafe extern "C" fn Winver_GetOSInfo(OSInfo: usize) -> i32 {
    0
}

/// dialog_printsetup - from reactos/wordpad.h
#[no_mangle]
pub unsafe extern "C" fn dialog_printsetup(arg0: *mut core::ffi::c_void) {

}

/// dialog_print - from reactos/wordpad.h
#[no_mangle]
pub unsafe extern "C" fn dialog_print(arg0: *mut core::ffi::c_void, arg1: *mut u16) {

}

/// ExpandTabLength - from reactos/text.h
#[no_mangle]
pub unsafe extern "C" fn ExpandTabLength(line: usize) -> i32 {
    0
}

/// ExpandTab - from reactos/text.h
#[no_mangle]
pub unsafe extern "C" fn ExpandTab(line: usize) -> usize {
    0
}

/// reg_export - from reactos/reg.h
#[no_mangle]
pub unsafe extern "C" fn reg_export(argc: i32, argvW: *mut u16) -> i32 {
    0
}

/// CreateDialogProc - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn CreateDialogProc(hDlg: *mut core::ffi::c_void, message: u32, wParam: usize, lParam: isize) -> usize {
    0
}

/// DeleteDialogProc - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn DeleteDialogProc(hDlg: *mut core::ffi::c_void, message: u32, wParam: usize, lParam: isize) -> usize {
    0
}

/// ProgressDialogProc - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn ProgressDialogProc(hDlg: *mut core::ffi::c_void, Message: u32, wParam: usize, lParam: isize) -> usize {
    0
}

/// StopDependsDialogProc - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn StopDependsDialogProc(hDlg: *mut core::ffi::c_void, message: u32, wParam: usize, lParam: isize) -> usize {
    0
}

/// CreateStopDependsDialog - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn CreateStopDependsDialog(hParent: *mut core::ffi::c_void, ServiceName: *mut u16, DisplayName: *mut u16, ServiceList: *mut u16) -> i32 {
    0
}

/// LogonPageProc - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn LogonPageProc(hwndDlg: *mut core::ffi::c_void, uMsg: u32, wParam: usize, lParam: isize) -> usize {
    0
}

/// ExportFile - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn ExportFile(Info: usize) -> usize {
    0
}

/// login - from glibc/utmp.h
#[no_mangle]
pub unsafe extern "C" fn login(__entry: *mut core::ffi::c_void) {

}

/// GetItemsInContainer - from reactos/stl_bids.h
#[no_mangle]
pub unsafe extern "C" fn GetItemsInContainer() -> usize {
    0
}

/// LogErrorConsole - from reactos/tnerror.h
#[no_mangle]
pub unsafe extern "C" fn LogErrorConsole(szError: usize) {

}

/// CreateInstalledAppInstance - from reactos/appdb.h
#[no_mangle]
pub unsafe extern "C" fn CreateInstalledAppInstance(KeyName: *const u16, User: i32, WowSam: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// IsInstalledEnum - from reactos/appinfo.h
#[no_mangle]
pub unsafe extern "C" fn IsInstalledEnum(x: i32) -> i32 {
    0
}

/// ExtractAndRunGeneratedInstaller - from reactos/appinfo.h
#[no_mangle]
pub unsafe extern "C" fn ExtractAndRunGeneratedInstaller(AppInfo: usize, Archive: *const u16, Silent: usize) -> i32 {
    0
}

/// EmulateDialogReposition - from reactos/misc.h
#[no_mangle]
pub unsafe extern "C" fn EmulateDialogReposition(hwnd: *mut core::ffi::c_void) -> usize {
    0
}

/// OpensWithExplorer - from reactos/misc.h
#[no_mangle]
pub unsafe extern "C" fn OpensWithExplorer(Path: usize) -> i32 {
    0
}

/// InitLogs - from reactos/misc.h
#[no_mangle]
pub unsafe extern "C" fn InitLogs() -> usize {
    0
}

/// ExpandEnvStrings - from reactos/misc.h
#[no_mangle]
pub unsafe extern "C" fn ExpandEnvStrings(Str: usize) -> usize {
    0
}

/// GuessInstallerType - from reactos/misc.h
#[no_mangle]
pub unsafe extern "C" fn GuessInstallerType(Installer: *const u16, ExtraInfo: u32) -> usize {
    0
}

/// cicGetOSInfo - from reactos/cicbase.h
#[no_mangle]
pub unsafe extern "C" fn cicGetOSInfo(puACP: usize, pdwOSInfo: usize) -> usize {
    0
}

/// cicRealCoCreateInstance - from reactos/cicbase.h
#[no_mangle]
pub unsafe extern "C" fn cicRealCoCreateInstance(rclsid: usize, pUnkOuter: usize, dwClsContext: usize, iid: usize, ppv: *mut core::ffi::c_void) -> usize {
    0
}

/// cicCoCreateInstance - from reactos/cicbase.h
#[no_mangle]
pub unsafe extern "C" fn cicCoCreateInstance(rclsid: usize, pUnkOuter: usize, dwClsContext: usize, iid: usize, ppv: *mut core::ffi::c_void) -> usize {
    0
}

/// _IsInCollection - from reactos/displayattributemgr.h
#[no_mangle]
pub unsafe extern "C" fn _IsInCollection(rguid: usize) -> i32 {
    0
}

/// CreateInstance - from reactos/documentmgr.h
#[no_mangle]
pub unsafe extern "C" fn CreateInstance(pThreadMgrSink: *mut core::ffi::c_void, ppOut: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// AdviseSink - from reactos/documentmgr.h
#[no_mangle]
pub unsafe extern "C" fn AdviseSink(riid: usize, punk: *mut core::ffi::c_void, pdwCookie: *mut u32) -> usize {
    0
}

/// UnadviseSink - from reactos/documentmgr.h
#[no_mangle]
pub unsafe extern "C" fn UnadviseSink(pdwCookie: u32) -> usize {
    0
}

/// get_textservice_sink - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn get_textservice_sink(tid: usize, iid: usize, sink: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// set_textservice_sink - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn set_textservice_sink(tid: usize, iid: usize, sink: *mut core::ffi::c_void) -> i32 {
    0
}

/// advise_sink - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn advise_sink(sink_list: *mut core::ffi::c_void, riid: usize, cookie_magic: u32, unk: *mut core::ffi::c_void, cookie: *mut u32) -> i32 {
    0
}

/// unadvise_sink - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn unadvise_sink(cookie: u32) -> i32 {
    0
}

/// AdviseMouseSink - from reactos/inputcontext.h
#[no_mangle]
pub unsafe extern "C" fn AdviseMouseSink(range: *mut core::ffi::c_void, pSink: *mut core::ffi::c_void, pdwCookie: *mut u32) -> usize {
    0
}

/// UnadviseMouseSink - from reactos/inputcontext.h
#[no_mangle]
pub unsafe extern "C" fn UnadviseMouseSink(dwCookie: u32) -> usize {
    0
}

/// IsInteractiveUserLogon - from reactos/misc.h
#[no_mangle]
pub unsafe extern "C" fn IsInteractiveUserLogon(arg0: usize) -> i32 {
    0
}

/// ActiveLanguageProfileNotifySinkCallback - from reactos/profile.h
#[no_mangle]
pub unsafe extern "C" fn ActiveLanguageProfileNotifySinkCallback(rguid1: usize, rguid2: usize, fActivated: i32, pUserData: *mut core::ffi::c_void) -> i32 {
    0
}

/// InitProfileInstance - from reactos/profile.h
#[no_mangle]
pub unsafe extern "C" fn InitProfileInstance(pTLS: *mut core::ffi::c_void) -> i32 {
    0
}

/// DnsIntCacheInitialize - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn DnsIntCacheInitialize(arg0: usize) -> usize {
    0
}

/// DnsIntCacheRemoveEntryItem - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn DnsIntCacheRemoveEntryItem(CacheEntry: usize) -> usize {
    0
}

/// DnsIntCacheFlush - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn DnsIntCacheFlush(ulFlags: usize) -> usize {
    0
}

/// DnsIntFlushCacheEntry - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn DnsIntFlushCacheEntry(pszName: usize, wType: usize) -> usize {
    0
}

/// DnsIntCacheGetEntryByName - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn DnsIntCacheGetEntryByName(Name: *const u16, wType: u16, dwFlags: u32, Record: *mut core::ffi::c_void) -> usize {
    0
}

/// DnsIntCacheAddEntry - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn DnsIntCacheAddEntry(Record: usize, bHostsFileEntry: usize) -> usize {
    0
}

/// DnsIntCacheRemoveEntryByName - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn DnsIntCacheRemoveEntryByName(Name: usize) -> i32 {
    0
}

/// DnsIntCacheGetEntries - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn DnsIntCacheGetEntries(ppCacheEntries: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// LogfListInitialize - from reactos/eventlog.h
#[no_mangle]
pub unsafe extern "C" fn LogfListInitialize(arg0: usize) -> usize {
    0
}

/// LogfListItemCount - from reactos/eventlog.h
#[no_mangle]
pub unsafe extern "C" fn LogfListItemCount(arg0: usize) -> u32 {
    0
}

/// LogfListItemByIndex - from reactos/eventlog.h
#[no_mangle]
pub unsafe extern "C" fn LogfListItemByIndex(Index: u32) -> usize {
    0
}

/// LogfListItemByName - from reactos/eventlog.h
#[no_mangle]
pub unsafe extern "C" fn LogfListItemByName(Name: *const u16) -> usize {
    0
}

/// LogfCreate - from reactos/eventlog.h
#[no_mangle]
pub unsafe extern "C" fn LogfCreate(LogFile: *mut core::ffi::c_void, LogName: usize, FileName: usize, MaxSize: u32, Retention: u32, Permanent: usize, Backup: usize) -> i32 {
    0
}

/// LogfClearFile - from reactos/eventlog.h
#[no_mangle]
pub unsafe extern "C" fn LogfClearFile(LogFile: usize, BackupFileName: usize) -> i32 {
    0
}

/// LogfBackupFile - from reactos/eventlog.h
#[no_mangle]
pub unsafe extern "C" fn LogfBackupFile(LogFile: usize, BackupFileName: usize) -> i32 {
    0
}

/// LogfReportEvent - from reactos/eventlog.h
#[no_mangle]
pub unsafe extern "C" fn LogfReportEvent(wType: u16, wCategory: u16, dwEventId: u32, wNumStrings: u16, pStrings: usize, dwDataSize: u32, pRawData: *mut core::ffi::c_void) -> usize {
    0
}

/// InitLogPort - from reactos/eventlog.h
#[no_mangle]
pub unsafe extern "C" fn InitLogPort(arg0: usize) -> i32 {
    0
}

/// open_log_files - from reactos/daemon_debug.h
#[no_mangle]
pub unsafe extern "C" fn open_log_files() {

}

/// close_log_files - from reactos/daemon_debug.h
#[no_mangle]
pub unsafe extern "C" fn close_log_files() {

}

/// AddToMessageLog - from reactos/service.h
#[no_mangle]
pub unsafe extern "C" fn AddToMessageLog(lpszMsg: usize) {

}

/// nfs_to_standard_info - from reactos/util.h
#[no_mangle]
pub unsafe extern "C" fn nfs_to_standard_info(info: *mut core::ffi::c_void, std_out: usize) {

}

/// SvcNetBiosInit - from reactos/svchost.h
#[no_mangle]
pub unsafe extern "C" fn SvcNetBiosInit(arg0: usize) -> usize {
    0
}

/// InitLogging - from reactos/tcpsvcs.h
#[no_mangle]
pub unsafe extern "C" fn InitLogging(arg0: usize) -> i32 {
    0
}

/// UninitLogging - from reactos/tcpsvcs.h
#[no_mangle]
pub unsafe extern "C" fn UninitLogging(arg0: usize) -> usize {
    0
}

/// LogEvent - from reactos/tcpsvcs.h
#[no_mangle]
pub unsafe extern "C" fn LogEvent(lpMsg: *const u16, errNum: u32, exitCode: u32, flags: u32) -> usize {
    0
}

/// closelog - from reactos/syslog.h
#[no_mangle]
pub unsafe extern "C" fn closelog() {

}

/// openlog - from reactos/syslog.h
#[no_mangle]
pub unsafe extern "C" fn openlog(__ident: *mut i8, __option: i32, __facility: i32) {

}

/// setlogmask - from reactos/syslog.h
#[no_mangle]
pub unsafe extern "C" fn setlogmask(__mask: i32) -> i32 {
    0
}

/// syslog - from reactos/syslog.h
#[no_mangle]
pub unsafe extern "C" fn syslog(__pri: i32, __fmt: *mut i8) {

}

/// vsyslog - from reactos/syslog.h
#[no_mangle]
pub unsafe extern "C" fn vsyslog(__pri: i32, __fmt: *mut i8, __ap: usize) {

}

/// set_syslog_conf_dir - from reactos/syslog.h
#[no_mangle]
pub unsafe extern "C" fn set_syslog_conf_dir(dir: *mut i8) -> *mut i8 {
    core::ptr::null_mut()
}

/// UserLogin - from reactos/telnetd.h
#[no_mangle]
pub unsafe extern "C" fn UserLogin(client_socket: i32) {

}

/// logMess - from reactos/tftpd.h
#[no_mangle]
pub unsafe extern "C" fn logMess(arg0: *mut core::ffi::c_void, arg1: usize) {

}

/// ChangeACLsOfFilesInCurDir - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn ChangeACLsOfFilesInCurDir(pszFiles: usize) -> i32 {
    0
}

/// asinh - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn asinh(x: f64) -> f64 {
    0.0
}

/// log - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn log(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// acosh - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn acosh(x: f64) -> f64 {
    0.0
}

/// sqrt - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn sqrt(arg0: usize) -> usize {
    0
}

/// atanh - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn atanh(x: f64) -> f64 {
    0.0
}

/// sqrti - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn sqrti(number: usize) -> usize {
    0
}

/// rpn_pow_f - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn rpn_pow_f(r: *mut core::ffi::c_void, a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) {

}

/// pow - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn pow(arg0: usize, arg1: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// InitInstance - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn InitInstance(hInst: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// InitializeDialog - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn InitializeDialog(hwndDlg: *mut core::ffi::c_void, pDispDevice: usize) -> i32 {
    0
}

/// DestroyTabCtrlDialogs - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn DestroyTabCtrlDialogs(pContext: usize) -> usize {
    0
}

/// InitializeDxDiagDialog - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn InitializeDxDiagDialog(hwndDlg: *mut core::ffi::c_void) -> usize {
    0
}

/// InitializeDirectInputDialog - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn InitializeDirectInputDialog(hwndDlg: *mut core::ffi::c_void) {

}

/// InitializeDirectPlayDialog - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn InitializeDirectPlayDialog(hwndDlg: *mut core::ffi::c_void) {

}

/// register_iexplore - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn register_iexplore(doregister: i32) -> u32 {
    0
}

/// OpenFileDialog - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn OpenFileDialog(hwnd: *mut core::ffi::c_void, dwFilterIndex: u32, lpType: usize) -> usize {
    0
}

/// SetDialogIcon - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn SetDialogIcon(hDlg: *mut core::ffi::c_void) -> usize {
    0
}

/// InitializeSystemDialog - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn InitializeSystemDialog(hDlg: *mut core::ffi::c_void) -> i32 {
    0
}

/// rdssl_mod_exp - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn rdssl_mod_exp(out: *mut i8, out_len: i32, arg2: *mut i8, in_len: i32, arg4: *mut i8, mod_len: i32, exp: *mut i8, exp_len: i32) -> i32 {
    0
}

/// rdp_send_logon_info - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn rdp_send_logon_info(flags: usize, domain: *mut i8, user: *mut i8, password: *mut i8, program: *mut i8, directory: *mut i8) {

}

/// process_pdu_logon - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn process_pdu_logon(s: usize) {

}

/// rdssl_rkey_get_exp_mod - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn rdssl_rkey_get_exp_mod(rkey: *mut core::ffi::c_void, exponent: *mut core::ffi::c_void, max_exp_len: usize, modulus: *mut core::ffi::c_void, max_mod_len: usize) -> i32 {
    0
}

/// DIALOG_StatusBarUpdateLineEndings - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn DIALOG_StatusBarUpdateLineEndings(arg0: usize) -> usize {
    0
}

/// DIALOG_StatusBarUpdateEncoding - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn DIALOG_StatusBarUpdateEncoding(arg0: usize) -> usize {
    0
}

/// DIALOG_StatusBarUpdateAll - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn DIALOG_StatusBarUpdateAll(arg0: usize) -> usize {
    0
}

/// DIALOG_FileSaveAs_Hook - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FileSaveAs_Hook(hDlg: *mut core::ffi::c_void, msg: u32, wParam: usize, lParam: isize) -> usize {
    0
}

/// DIALOG_SearchDialog - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn DIALOG_SearchDialog(pfnProc: usize) -> usize {
    0
}

/// DIALOG_GoTo_DialogProc - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn DIALOG_GoTo_DialogProc(hwndDialog: *mut core::ffi::c_void, uMsg: u32, wParam: usize, lParam: isize) -> usize {
    0
}

/// DIALOG_Printing_DialogProc - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn DIALOG_Printing_DialogProc(hwnd: *mut core::ffi::c_void, uMsg: u32, wParam: usize, lParam: isize) -> usize {
    0
}

/// EndDialog - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn EndDialog(arg0: usize, arg1: usize) -> usize {
    0
}

/// DIALOG_PAGESETUP_Hook - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn DIALOG_PAGESETUP_Hook(hDlg: *mut core::ffi::c_void, uMsg: u32, wParam: usize, lParam: isize) -> usize {
    0
}

/// __assert_single_arg - from glibc/assert.h
#[no_mangle]
pub unsafe extern "C" fn __assert_single_arg(arg0: usize) -> usize {
    0
}

/// __bswap_constant_16 - from glibc/byteswap.h
#[no_mangle]
pub unsafe extern "C" fn __bswap_constant_16(arg0: usize) -> usize {
    0
}

/// __bswap_constant_32 - from glibc/byteswap.h
#[no_mangle]
pub unsafe extern "C" fn __bswap_constant_32(arg0: usize) -> usize {
    0
}

/// __bswap_constant_64 - from glibc/byteswap.h
#[no_mangle]
pub unsafe extern "C" fn __bswap_constant_64(arg0: usize) -> usize {
    0
}

/// __open_catalog - from glibc/catgetsinfo.h
#[no_mangle]
pub unsafe extern "C" fn __open_catalog(cat_name: *mut i8, nlspath: *mut i8, env_var: *mut i8, __catalog: usize) -> i32 {
    0
}

/// set_expected_protected1 - from glibc/tst-protected1mod.h
#[no_mangle]
pub unsafe extern "C" fn set_expected_protected1(arg0: i32) {

}

/// set_expected_protected3a - from glibc/tst-protected1mod.h
#[no_mangle]
pub unsafe extern "C" fn set_expected_protected3a(arg0: i32) {

}

/// set_expected_protected3b - from glibc/tst-protected1mod.h
#[no_mangle]
pub unsafe extern "C" fn set_expected_protected3b(arg0: i32) {

}

/// __kernel_casinhf - from glibc/complex.h
#[no_mangle]
pub unsafe extern "C" fn __kernel_casinhf(z: usize, adj: i32) -> usize {
    0
}

/// __kernel_casinh - from glibc/complex.h
#[no_mangle]
pub unsafe extern "C" fn __kernel_casinh(z: usize, adj: i32) -> usize {
    0
}

/// __kernel_casinhl - from glibc/complex.h
#[no_mangle]
pub unsafe extern "C" fn __kernel_casinhl(z: usize, adj: i32) -> usize {
    0
}

/// __kernel_casinhf128 - from glibc/complex.h
#[no_mangle]
pub unsafe extern "C" fn __kernel_casinhf128(z: usize, adj: i32) -> usize {
    0
}

/// __feclearexcept - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn __feclearexcept(__excepts: i32) -> i32 {
    0
}

/// __fegetexcept - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn __fegetexcept() -> i32 {
    0
}

/// __fegetexceptflag - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn __fegetexceptflag(__flagp: *mut core::ffi::c_void, __excepts: i32) -> i32 {
    0
}

/// __feraiseexcept - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn __feraiseexcept(__excepts: i32) -> i32 {
    0
}

/// __fesetexceptflag - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn __fesetexceptflag(__flagp: *mut core::ffi::c_void, __excepts: i32) -> i32 {
    0
}

/// __fegetenv - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn __fegetenv(__envp: *mut core::ffi::c_void) -> i32 {
    0
}

/// __fesetenv - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn __fesetenv(__envp: *mut core::ffi::c_void) -> i32 {
    0
}

/// __feupdateenv - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn __feupdateenv(__envp: *mut core::ffi::c_void) -> i32 {
    0
}

/// fegetenv - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn fegetenv(__e: *mut core::ffi::c_void) -> i32 {
    0
}

/// feholdexcept - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn feholdexcept(__e: *mut core::ffi::c_void) -> i32 {
    0
}

/// __feholdexcept - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn __feholdexcept(__e: *mut core::ffi::c_void) -> i32 {
    0
}

/// fesetenv - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn fesetenv(__e: *mut core::ffi::c_void) -> i32 {
    0
}

/// feupdateenv - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn feupdateenv(__e: *mut core::ffi::c_void) -> i32 {
    0
}

/// fegetround - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn fegetround() -> i32 {
    0
}

/// __fegetround - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn __fegetround() -> i32 {
    0
}

/// fesetround - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn fesetround(__d: i32) -> i32 {
    0
}

/// __fesetround - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn __fesetround(__d: i32) -> i32 {
    0
}

/// find_empty_slot_for_expand - from glibc/inline-hashtab.h
#[no_mangle]
pub unsafe extern "C" fn find_empty_slot_for_expand(htab: *mut core::ffi::c_void, hash: i32) -> *mut *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// htab_expand - from glibc/inline-hashtab.h
#[no_mangle]
pub unsafe extern "C" fn htab_expand(htab: *mut core::ffi::c_void, hash_fn: i32) -> i32 {
    0
}

/// __issignalingf - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __issignalingf(x: f32) -> i32 {
    0
}

/// __isinff128 - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __isinff128(x: usize) -> i32 {
    0
}

/// fabsf128 - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn fabsf128(x: usize) -> usize {
    0
}

/// __builtin_fabsf128 - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __builtin_fabsf128(arg0: usize) -> usize {
    0
}

/// __mbsinit - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __mbsinit(__ps: *mut core::ffi::c_void) -> i32 {
    0
}

/// _nl_explode_name - from glibc/loadinfo.h
#[no_mangle]
pub unsafe extern "C" fn _nl_explode_name(name: *mut i8, language: *mut *mut i8, modifier: *mut *mut i8, territory: *mut *mut i8, codeset: *mut *mut i8, normalized_codeset: *mut *mut i8) -> i32 {
    0
}

/// EXTRACT_PLURAL_EXPRESSION - from glibc/plural-exp.h
#[no_mangle]
pub unsafe extern "C" fn EXTRACT_PLURAL_EXPRESSION(nullentry: *mut i8, pluralp: *mut *mut core::ffi::c_void, npluralsp: *mut core::ffi::c_void) {

}

/// login_tty - from glibc/utmp.h
#[no_mangle]
pub unsafe extern "C" fn login_tty(__fd: i32) -> usize {
    0
}

/// logout - from glibc/utmp.h
#[no_mangle]
pub unsafe extern "C" fn logout(__ut_line: *mut i8) -> i32 {
    0
}

/// logwtmp - from glibc/utmp.h
#[no_mangle]
pub unsafe extern "C" fn logwtmp(__ut_line: *mut i8, __ut_name: *mut i8, __ut_host: *mut i8) {

}

/// fromfp_max_exponent - from glibc/compat_fromfp.h
#[no_mangle]
pub unsafe extern "C" fn fromfp_max_exponent(negative: usize, width: i32) -> i32 {
    0
}

/// feclearexcept - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn feclearexcept(__excepts: i32) -> usize {
    0
}

/// fegetexceptflag - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn fegetexceptflag(__flagp: *mut core::ffi::c_void, __excepts: i32) -> i32 {
    0
}

/// feraiseexcept - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn feraiseexcept(__excepts: i32) -> i32 {
    0
}

/// fesetexcept - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn fesetexcept(__excepts: i32) -> i32 {
    0
}

/// fesetexceptflag - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn fesetexceptflag(__flagp: *mut core::ffi::c_void, __excepts: i32) -> i32 {
    0
}

/// fetestexcept - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn fetestexcept(__excepts: i32) -> i32 {
    0
}

/// fetestexceptflag - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn fetestexceptflag(__flagp: *mut core::ffi::c_void, __excepts: i32) -> i32 {
    0
}

/// fegetmode - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn fegetmode(__modep: *mut core::ffi::c_void) -> i32 {
    0
}

/// fesetmode - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn fesetmode(__modep: *mut core::ffi::c_void) -> i32 {
    0
}

/// feenableexcept - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn feenableexcept(__excepts: i32) -> i32 {
    0
}

/// fedisableexcept - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn fedisableexcept(__excepts: i32) -> i32 {
    0
}

/// fegetexcept - from glibc/fenv.h
#[no_mangle]
pub unsafe extern "C" fn fegetexcept() -> i32 {
    0
}

/// CHECK_NARROW_SQRT - from glibc/math-narrow.h
#[no_mangle]
pub unsafe extern "C" fn CHECK_NARROW_SQRT(arg0: usize, arg1: usize) -> usize {
    0
}

/// __kernel_standard - from glibc/math-svid-compat.h
#[no_mangle]
pub unsafe extern "C" fn __kernel_standard(arg0: f64, arg1: f64, arg2: i32) -> f64 {
    0.0
}

/// __kernel_standard_f - from glibc/math-svid-compat.h
#[no_mangle]
pub unsafe extern "C" fn __kernel_standard_f(arg0: f32, arg1: f32, arg2: i32) -> f32 {
    0.0
}

/// __kernel_standard_l - from glibc/math-svid-compat.h
#[no_mangle]
pub unsafe extern "C" fn __kernel_standard_l(arg0: i64, arg1: i64, arg2: i32) -> f64 {
    0.0
}

/// __nldbl_nexttowardf - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __nldbl_nexttowardf(__x: f32, __y: f64) -> usize {
    0
}

/// __MATHCALL_NARROW_NORMAL - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __MATHCALL_NARROW_NORMAL(arg0: usize, arg1: usize) -> usize {
    0
}

/// issignaling - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn issignaling(__val: f32) -> i32 {
    0
}

/// __issignaling - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __issignaling(arg0: usize) -> usize {
    0
}

/// __issignalingl - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __issignalingl(arg0: usize) -> usize {
    0
}

/// __issignalingf128 - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __issignalingf128(arg0: usize) -> usize {
    0
}

/// iszero - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn iszero(__val: f32) -> i32 {
    0
}

/// __fpclassifyf - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __fpclassifyf(arg0: usize) -> usize {
    0
}

/// __fpclassify - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __fpclassify(arg0: usize) -> usize {
    0
}

/// __fpclassifyl - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __fpclassifyl(arg0: usize) -> usize {
    0
}

/// __fpclassifyf128 - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __fpclassifyf128(arg0: usize) -> usize {
    0
}

/// __call - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __call(__x: f32, __y: f32) -> i32 {
    0
}

/// __iseqsigf - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __iseqsigf(arg0: usize, arg1: usize) -> usize {
    0
}

/// __iseqsig - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __iseqsig(arg0: usize, arg1: usize) -> usize {
    0
}

/// __iseqsigl - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __iseqsigl(arg0: usize, arg1: usize) -> usize {
    0
}

/// __iseqsigf128 - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn __iseqsigf128(arg0: usize, arg1: usize) -> usize {
    0
}

/// iseqsig - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn iseqsig(__x: usize, __y: usize) -> i32 {
    0
}

/// decltype - from glibc/math.h
#[no_mangle]
pub unsafe extern "C" fn decltype(__x: usize, arg1: usize) -> usize {
    0
}

/// mul_expansion - from glibc/mul_split.h
#[no_mangle]
pub unsafe extern "C" fn mul_expansion(hi: *mut f64, lo: *mut f64, h1: f64, l1: f64, h2: f64, l2: f64) {

}

/// div_expansion - from glibc/mul_split.h
#[no_mangle]
pub unsafe extern "C" fn div_expansion(hi: *mut f64, lo: *mut f64, h1: f64, l1: f64, h2: f64, l2: f64) {

}

/// hol_cousin_cluster_cmp - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn hol_cousin_cluster_cmp(cl1: *mut core::ffi::c_void, cl2: *mut core::ffi::c_void) -> i32 {
    0
}

/// __isinf_ns2 - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn __isinf_ns2(d: f64) -> usize {
    0
}

/// kernel_standard - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn kernel_standard(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// AnnotateExpectRace - from cpython/dynamic_annotations.h
#[no_mangle]
pub unsafe extern "C" fn AnnotateExpectRace(file: *mut i8, line: i32, address: *mut core::ffi::c_void, description: *mut i8) {

}

/// _PyObjectIndexPair - from cpython/object.h
#[no_mangle]
pub unsafe extern "C" fn _PyObjectIndexPair(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// PyDTrace_INSTANCE_NEW_START_ENABLED - from cpython/pydtrace.h
#[no_mangle]
pub unsafe extern "C" fn PyDTrace_INSTANCE_NEW_START_ENABLED() -> i32 {
    0
}

/// PyDTrace_INSTANCE_NEW_DONE_ENABLED - from cpython/pydtrace.h
#[no_mangle]
pub unsafe extern "C" fn PyDTrace_INSTANCE_NEW_DONE_ENABLED() -> i32 {
    0
}

/// PyDTrace_INSTANCE_DELETE_START_ENABLED - from cpython/pydtrace.h
#[no_mangle]
pub unsafe extern "C" fn PyDTrace_INSTANCE_DELETE_START_ENABLED() -> i32 {
    0
}

/// PyDTrace_INSTANCE_DELETE_DONE_ENABLED - from cpython/pydtrace.h
#[no_mangle]
pub unsafe extern "C" fn PyDTrace_INSTANCE_DELETE_DONE_ENABLED() -> i32 {
    0
}

/// _Py_log1p - from cpython/_math.h
#[no_mangle]
pub unsafe extern "C" fn _Py_log1p(x: f64) -> f64 {
    0.0
}

/// log1p - from cpython/_math.h
#[no_mangle]
pub unsafe extern "C" fn log1p(arg0: usize) -> usize {
    0
}

/// _PyPegen_lookahead_for_expr - from cpython/pegen.h
#[no_mangle]
pub unsafe extern "C" fn _PyPegen_lookahead_for_expr(arg0: i32, func: usize) -> i32 {
    0
}

/// _PyPegen_expect_forced_result - from cpython/pegen.h
#[no_mangle]
pub unsafe extern "C" fn _PyPegen_expect_forced_result(p: *mut core::ffi::c_void, result: *mut core::ffi::c_void, expected: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _PyPegen_expect_soft_keyword - from cpython/pegen.h
#[no_mangle]
pub unsafe extern "C" fn _PyPegen_expect_soft_keyword(p: *mut core::ffi::c_void, keyword: *mut i8) -> usize {
    0
}

/// _PyPegen_set_expr_context - from cpython/pegen.h
#[no_mangle]
pub unsafe extern "C" fn _PyPegen_set_expr_context(arg0: *mut core::ffi::c_void, arg1: usize, arg2: usize) -> usize {
    0
}

/// _PyPegen_constant_from_token - from cpython/pegen.h
#[no_mangle]
pub unsafe extern "C" fn _PyPegen_constant_from_token(p: *mut core::ffi::c_void, tok: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyPegen_decoded_constant_from_token - from cpython/pegen.h
#[no_mangle]
pub unsafe extern "C" fn _PyPegen_decoded_constant_from_token(p: *mut core::ffi::c_void, tok: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyPegen_constant_from_string - from cpython/pegen.h
#[no_mangle]
pub unsafe extern "C" fn _PyPegen_constant_from_string(p: *mut core::ffi::c_void, tok: *mut core::ffi::c_void) -> usize {
    0
}

/// _register_builtins_for_crossinterpreter_data - from cpython/crossinterp_data_lookup.h
#[no_mangle]
pub unsafe extern "C" fn _register_builtins_for_crossinterpreter_data(arg0: *mut core::ffi::c_void) {

}

/// _TAIL_CALL_CALL_ISINSTANCE - from cpython/opcode_targets.h
#[no_mangle]
pub unsafe extern "C" fn _TAIL_CALL_CALL_ISINSTANCE(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _TAIL_CALL_LOAD_ATTR_INSTANCE_VALUE - from cpython/opcode_targets.h
#[no_mangle]
pub unsafe extern "C" fn _TAIL_CALL_LOAD_ATTR_INSTANCE_VALUE(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _TAIL_CALL_LOAD_COMMON_CONSTANT - from cpython/opcode_targets.h
#[no_mangle]
pub unsafe extern "C" fn _TAIL_CALL_LOAD_COMMON_CONSTANT(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _TAIL_CALL_STORE_ATTR_INSTANCE_VALUE - from cpython/opcode_targets.h
#[no_mangle]
pub unsafe extern "C" fn _TAIL_CALL_STORE_ATTR_INSTANCE_VALUE(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyInstanceMethod_GET_FUNCTION - from cpython/classobject.h
#[no_mangle]
pub unsafe extern "C" fn PyInstanceMethod_GET_FUNCTION(meth: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _PyInstanceMethod_CAST - from cpython/classobject.h
#[no_mangle]
pub unsafe extern "C" fn _PyInstanceMethod_CAST(arg0: usize) -> usize {
    0
}

/// atomic_load_explicit - from cpython/pyatomic_std.h
#[no_mangle]
pub unsafe extern "C" fn atomic_load_explicit(_Atomicint: usize) -> usize {
    0
}

/// _PyObject_RealIsInstance - from cpython/pycore_abstract.h
#[no_mangle]
pub unsafe extern "C" fn _PyObject_RealIsInstance(inst: *mut core::ffi::c_void, cls: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyAST_Expression - from cpython/pycore_ast.h
#[no_mangle]
pub unsafe extern "C" fn _PyAST_Expression(body: usize, arena: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyAST_Expr - from cpython/pycore_ast.h
#[no_mangle]
pub unsafe extern "C" fn _PyAST_Expr(value: usize, lineno: i32, col_offset: i32, end_lineno: i32, end_col_offset: i32, arena: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyAST_NamedExpr - from cpython/pycore_ast.h
#[no_mangle]
pub unsafe extern "C" fn _PyAST_NamedExpr(target: usize, value: usize, lineno: i32, col_offset: i32, end_lineno: i32, end_col_offset: i32, arena: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyAST_IfExp - from cpython/pycore_ast.h
#[no_mangle]
pub unsafe extern "C" fn _PyAST_IfExp(test: usize, body: usize, orelse: usize, lineno: i32, col_offset: i32, end_lineno: i32, end_col_offset: i32, arena: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyAST_GeneratorExp - from cpython/pycore_ast.h
#[no_mangle]
pub unsafe extern "C" fn _PyAST_GeneratorExp(elt: usize, generators: *mut core::ffi::c_void, lineno: i32, col_offset: i32, end_lineno: i32, end_col_offset: i32, arena: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyAST_Constant - from cpython/pycore_ast.h
#[no_mangle]
pub unsafe extern "C" fn _PyAST_Constant(value: usize, kind: usize, lineno: i32, col_offset: i32, end_lineno: i32, end_col_offset: i32, arena: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyAST_MatchSingleton - from cpython/pycore_ast.h
#[no_mangle]
pub unsafe extern "C" fn _PyAST_MatchSingleton(value: usize, lineno: i32, col_offset: i32, end_lineno: i32, end_col_offset: i32, arena: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyAST_ExprAsUnicode - from cpython/pycore_ast.h
#[no_mangle]
pub unsafe extern "C" fn _PyAST_ExprAsUnicode(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _BlocksOutputBuffer_InitAndGrow - from cpython/pycore_blocks_output_buffer.h
#[no_mangle]
pub unsafe extern "C" fn _BlocksOutputBuffer_InitAndGrow(buffer: *mut core::ffi::c_void, max_length: usize, next_out: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// _PyCode_ConstantKey - from cpython/pycore_code.h
#[no_mangle]
pub unsafe extern "C" fn _PyCode_ConstantKey(obj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _PyCompile_StartAnnotationSetup - from cpython/pycore_compile.h
#[no_mangle]
pub unsafe extern "C" fn _PyCompile_StartAnnotationSetup(c: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyCompile_IsInteractiveTopLevel - from cpython/pycore_compile.h
#[no_mangle]
pub unsafe extern "C" fn _PyCompile_IsInteractiveTopLevel(c: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyCompile_IsInInlinedComp - from cpython/pycore_compile.h
#[no_mangle]
pub unsafe extern "C" fn _PyCompile_IsInInlinedComp(c: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyCodegen_Expression - from cpython/pycore_compile.h
#[no_mangle]
pub unsafe extern "C" fn _PyCodegen_Expression(c: *mut core::ffi::c_void, e: usize) -> i32 {
    0
}

/// _PyImport_IsInitialized - from cpython/pycore_import.h
#[no_mangle]
pub unsafe extern "C" fn _PyImport_IsInitialized(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyImport_GetModuleExportHooks - from cpython/pycore_importdl.h
#[no_mangle]
pub unsafe extern "C" fn _PyImport_GetModuleExportHooks(info: *mut core::ffi::c_void, fp: *mut core::ffi::c_void, modinit: *mut core::ffi::c_void, modexport: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyIndexPool_Fini - from cpython/pycore_index_pool.h
#[no_mangle]
pub unsafe extern "C" fn _PyIndexPool_Fini(indices: *mut core::ffi::c_void) {

}

/// _PyInstructionSequence_SetAnnotationsCode - from cpython/pycore_instruction_sequence.h
#[no_mangle]
pub unsafe extern "C" fn _PyInstructionSequence_SetAnnotationsCode(seq: *mut core::ffi::c_void, annotations: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyFrame_IsIncomplete - from cpython/pycore_interpframe.h
#[no_mangle]
pub unsafe extern "C" fn _PyFrame_IsIncomplete(frame: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyLong_CheckExactAndCompact - from cpython/pycore_long.h
#[no_mangle]
pub unsafe extern "C" fn _PyLong_CheckExactAndCompact(op: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyModuleSpec_IsInitializing - from cpython/pycore_moduleobject.h
#[no_mangle]
pub unsafe extern "C" fn _PyModuleSpec_IsInitializing(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// _Py_ExplicitMergeRefcount - from cpython/pycore_object.h
#[no_mangle]
pub unsafe extern "C" fn _Py_ExplicitMergeRefcount(op: *mut core::ffi::c_void, extra: usize) -> usize {
    0
}

/// _PyObject_StoreInstanceAttribute - from cpython/pycore_object.h
#[no_mangle]
pub unsafe extern "C" fn _PyObject_StoreInstanceAttribute(obj: *mut core::ffi::c_void, name: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyObject_TryGetInstanceAttribute - from cpython/pycore_object.h
#[no_mangle]
pub unsafe extern "C" fn _PyObject_TryGetInstanceAttribute(obj: *mut core::ffi::c_void, name: *mut core::ffi::c_void, attr: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// _PyObject_IsInstanceDictEmpty - from cpython/pycore_object.h
#[no_mangle]
pub unsafe extern "C" fn _PyObject_IsInstanceDictEmpty(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// _Py_GetConstant_Init - from cpython/pycore_object.h
#[no_mangle]
pub unsafe extern "C" fn _Py_GetConstant_Init() {

}

/// PyJitRef_IsInvalid - from cpython/pycore_optimizer.h
#[no_mangle]
pub unsafe extern "C" fn PyJitRef_IsInvalid(arg0: usize) -> usize {
    0
}

/// _Py_HandleSystemExitAndKeyboardInterrupt - from cpython/pycore_pylifecycle.h
#[no_mangle]
pub unsafe extern "C" fn _Py_HandleSystemExitAndKeyboardInterrupt(exitcode_p: *mut i32) -> i32 {
    0
}

/// _Py_FdIsInteractive - from cpython/pycore_pylifecycle.h
#[no_mangle]
pub unsafe extern "C" fn _Py_FdIsInteractive(fp: *mut core::ffi::c_void, filename: *mut core::ffi::c_void) -> i32 {
    0
}

/// mi_expand - from cpython/mimalloc.h
#[no_mangle]
pub unsafe extern "C" fn mi_expand(p: *mut core::ffi::c_void, newsize: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mi__expand - from cpython/mimalloc.h
#[no_mangle]
pub unsafe extern "C" fn mi__expand(p: *mut core::ffi::c_void, newsize: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mi_atomic_fetch_add_explicit - from cpython/atomic.h
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_fetch_add_explicit(arg0: usize) -> usize {
    0
}

/// mi_atomic_fetch_sub_explicit - from cpython/atomic.h
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_fetch_sub_explicit(arg0: usize) -> usize {
    0
}

/// mi_atomic_fetch_and_explicit - from cpython/atomic.h
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_fetch_and_explicit(arg0: usize) -> usize {
    0
}

/// mi_atomic_fetch_or_explicit - from cpython/atomic.h
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_fetch_or_explicit(arg0: usize) -> usize {
    0
}

/// mi_atomic_compare_exchange_strong_explicit - from cpython/atomic.h
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_compare_exchange_strong_explicit(arg0: usize) -> usize {
    0
}

/// mi_atomic_exchange_explicit - from cpython/atomic.h
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_exchange_explicit(arg0: usize) -> usize {
    0
}

/// mi_atomic_load_explicit - from cpython/atomic.h
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_load_explicit(arg0: usize) -> usize {
    0
}

/// mi_atomic_store_explicit - from cpython/atomic.h
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_store_explicit(arg0: usize) {

}

/// mi_atomic_loadi64_explicit - from cpython/atomic.h
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_loadi64_explicit(arg0: usize) -> i64 {
    0
}

/// mi_atomic_storei64_explicit - from cpython/atomic.h
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_storei64_explicit(arg0: usize) {

}

/// _mi_is_power_of_two - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_is_power_of_two(x: usize) -> usize {
    0
}

/// cmath_acos_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_acos_impl(module: *mut core::ffi::c_void, z: usize) -> usize {
    0
}

/// cmath_acos - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_acos(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_acosh_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_acosh_impl(module: *mut core::ffi::c_void, z: usize) -> usize {
    0
}

/// cmath_acosh - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_acosh(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_asin_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_asin_impl(module: *mut core::ffi::c_void, z: usize) -> usize {
    0
}

/// cmath_asin - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_asin(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_asinh_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_asinh_impl(module: *mut core::ffi::c_void, z: usize) -> usize {
    0
}

/// cmath_asinh - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_asinh(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_atan_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_atan_impl(module: *mut core::ffi::c_void, z: usize) -> usize {
    0
}

/// cmath_atan - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_atan(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_atanh_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_atanh_impl(module: *mut core::ffi::c_void, z: usize) -> usize {
    0
}

/// cmath_atanh - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_atanh(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_cos_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_cos_impl(module: *mut core::ffi::c_void, z: usize) -> usize {
    0
}

/// cmath_cos - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_cos(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_cosh_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_cosh_impl(module: *mut core::ffi::c_void, z: usize) -> usize {
    0
}

/// cmath_cosh - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_cosh(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_exp_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_exp_impl(module: *mut core::ffi::c_void, z: usize) -> usize {
    0
}

/// cmath_exp - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_exp(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_log10_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_log10_impl(module: *mut core::ffi::c_void, z: usize) -> usize {
    0
}

/// cmath_log10 - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_log10(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_sin_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_sin_impl(module: *mut core::ffi::c_void, z: usize) -> usize {
    0
}

/// cmath_sin - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_sin(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_sinh_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_sinh_impl(module: *mut core::ffi::c_void, z: usize) -> usize {
    0
}

/// cmath_sinh - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_sinh(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_sqrt_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_sqrt_impl(module: *mut core::ffi::c_void, z: usize) -> usize {
    0
}

/// cmath_sqrt - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_sqrt(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_tan_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_tan_impl(module: *mut core::ffi::c_void, z: usize) -> usize {
    0
}

/// cmath_tan - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_tan(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_tanh_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_tanh_impl(module: *mut core::ffi::c_void, z: usize) -> usize {
    0
}

/// cmath_tanh - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_tanh(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// logarithm - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn logarithm(e: usize) -> usize {
    0
}

/// cmath_log_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_log_impl(module: *mut core::ffi::c_void, x: usize, y_obj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_log - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_log(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_isinf_impl - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_isinf_impl(module: *mut core::ffi::c_void, z: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cmath_isinf - from cpython/cmathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn cmath_isinf(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// math_frexp_impl - from cpython/mathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn math_frexp_impl(module: *mut core::ffi::c_void, x: f64) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// math_frexp - from cpython/mathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn math_frexp(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// frexp - from cpython/mathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn frexp() -> usize {
    0
}

/// math_ldexp_impl - from cpython/mathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn math_ldexp_impl(module: *mut core::ffi::c_void, x: f64, i: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// math_ldexp - from cpython/mathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn math_ldexp(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// math_pow_impl - from cpython/mathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn math_pow_impl(module: *mut core::ffi::c_void, x: f64, y: f64) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// math_pow - from cpython/mathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn math_pow(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// math_isinf_impl - from cpython/mathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn math_isinf_impl(module: *mut core::ffi::c_void, x: f64) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// math_isinf - from cpython/mathmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn math_isinf(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_login_tty_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_login_tty_impl(module: *mut core::ffi::c_void, fd: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_login_tty - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_login_tty(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_getlogin_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_getlogin_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_getlogin - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_getlogin(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os__emscripten_log_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os__emscripten_log_impl(module: *mut core::ffi::c_void, arg: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os__emscripten_log - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os__emscripten_log(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_SetReparseDeferralEnabled_impl - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetReparseDeferralEnabled_impl(arg0: *mut core::ffi::c_void, enabled: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_SetReparseDeferralEnabled - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetReparseDeferralEnabled(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_GetReparseDeferralEnabled_impl - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_GetReparseDeferralEnabled_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_GetReparseDeferralEnabled - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_GetReparseDeferralEnabled(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_Parse_impl - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_Parse_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, data: *mut core::ffi::c_void, isfinal: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_Parse - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_Parse(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_ParseFile_impl - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_ParseFile_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, file: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_ParseFile - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_ParseFile(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_SetBase_impl - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetBase_impl(arg0: *mut core::ffi::c_void, base: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_SetBase - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetBase(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_GetBase_impl - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_GetBase_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_GetBase - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_GetBase(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_GetInputContext_impl - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_GetInputContext_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_GetInputContext - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_GetInputContext(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_ExternalEntityParserCreate_impl - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_ExternalEntityParserCreate_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, context: *mut i8, encoding: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_ExternalEntityParserCreate - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_ExternalEntityParserCreate(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_SetParamEntityParsing_impl - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetParamEntityParsing_impl(arg0: *mut core::ffi::c_void, flag: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_SetParamEntityParsing - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetParamEntityParsing(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_UseForeignDTD_impl - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_UseForeignDTD_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, flag: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_UseForeignDTD - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_UseForeignDTD(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_SetBillionLaughsAttackProtectionActivationThreshold_impl - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetBillionLaughsAttackProtectionActivationThreshold_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, threshold: u64) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_SetBillionLaughsAttackProtectionActivationThreshold - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetBillionLaughsAttackProtectionActivationThreshold(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_SetBillionLaughsAttackProtectionMaximumAmplification_impl - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetBillionLaughsAttackProtectionMaximumAmplification_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, max_factor: f32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_xmlparser_SetBillionLaughsAttackProtectionMaximumAmplification - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetBillionLaughsAttackProtectionMaximumAmplification(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_ParserCreate_impl - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_ParserCreate_impl(module: *mut core::ffi::c_void, encoding: *mut i8, namespace_separator: *mut i8, intern: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_ParserCreate - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_ParserCreate(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_ErrorString_impl - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_ErrorString_impl(module: *mut core::ffi::c_void, code: i64) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pyexpat_ErrorString - from cpython/pyexpat.c.h
#[no_mangle]
pub unsafe extern "C" fn pyexpat_ErrorString(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// syslog_openlog_impl - from cpython/syslogmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn syslog_openlog_impl(module: *mut core::ffi::c_void, ident: *mut core::ffi::c_void, logopt: i64, facility: i64) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// syslog_openlog - from cpython/syslogmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn syslog_openlog(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// syslog_syslog_impl - from cpython/syslogmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn syslog_syslog_impl(module: *mut core::ffi::c_void, group_left_1: i32, priority: i32, message: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// syslog_syslog - from cpython/syslogmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn syslog_syslog(module: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// syslog_closelog_impl - from cpython/syslogmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn syslog_closelog_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// syslog_closelog - from cpython/syslogmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn syslog_closelog(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// syslog_setlogmask_impl - from cpython/syslogmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn syslog_setlogmask_impl(module: *mut core::ffi::c_void, maskpri: i64) -> i64 {
    0
}

/// syslog_setlogmask - from cpython/syslogmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn syslog_setlogmask(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// syslog_LOG_MASK_impl - from cpython/syslogmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn syslog_LOG_MASK_impl(module: *mut core::ffi::c_void, pri: i64) -> i64 {
    0
}

/// syslog_LOG_MASK - from cpython/syslogmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn syslog_LOG_MASK(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// syslog_LOG_UPTO_impl - from cpython/syslogmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn syslog_LOG_UPTO_impl(module: *mut core::ffi::c_void, pri: i64) -> i64 {
    0
}

/// syslog_LOG_UPTO - from cpython/syslogmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn syslog_LOG_UPTO(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _abc__abc_instancecheck_impl - from cpython/_abc.c.h
#[no_mangle]
pub unsafe extern "C" fn _abc__abc_instancecheck_impl(module: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, instance: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _abc__abc_instancecheck - from cpython/_abc.c.h
#[no_mangle]
pub unsafe extern "C" fn _abc__abc_instancecheck(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _asyncio_Future__log_traceback_get_impl - from cpython/_asynciomodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__log_traceback_get_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _asyncio_Future__log_traceback_get - from cpython/_asynciomodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__log_traceback_get(arg0: *mut core::ffi::c_void, Py_UNUSEDcontext: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _asyncio_Future__log_traceback_set_impl - from cpython/_asynciomodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__log_traceback_set_impl(arg0: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> i32 {
    0
}

/// _asyncio_Future__log_traceback_set - from cpython/_asynciomodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__log_traceback_set(arg0: *mut core::ffi::c_void, value: *mut core::ffi::c_void, Py_UNUSEDcontext: *mut core::ffi::c_void) -> i32 {
    0
}

/// _asyncio_Task__log_destroy_pending_get_impl - from cpython/_asynciomodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Task__log_destroy_pending_get_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _asyncio_Task__log_destroy_pending_get - from cpython/_asynciomodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Task__log_destroy_pending_get(arg0: *mut core::ffi::c_void, Py_UNUSEDcontext: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _asyncio_Task__log_destroy_pending_set_impl - from cpython/_asynciomodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Task__log_destroy_pending_set_impl(arg0: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> i32 {
    0
}

/// _asyncio_Task__log_destroy_pending_set - from cpython/_asynciomodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Task__log_destroy_pending_set(arg0: *mut core::ffi::c_void, value: *mut core::ffi::c_void, Py_UNUSEDcontext: *mut core::ffi::c_void) -> i32 {
    0
}

/// _hashlib_hmac_singleshot_impl - from cpython/_hashopenssl.c.h
#[no_mangle]
pub unsafe extern "C" fn _hashlib_hmac_singleshot_impl(module: *mut core::ffi::c_void, key: *mut core::ffi::c_void, msg: *mut core::ffi::c_void, digest: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _hashlib_hmac_singleshot - from cpython/_hashopenssl.c.h
#[no_mangle]
pub unsafe extern "C" fn _hashlib_hmac_singleshot(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _operator_pow_impl - from cpython/_operator.c.h
#[no_mangle]
pub unsafe extern "C" fn _operator_pow_impl(module: *mut core::ffi::c_void, a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _operator_pow - from cpython/_operator.c.h
#[no_mangle]
pub unsafe extern "C" fn _operator_pow(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _operator_ipow_impl - from cpython/_operator.c.h
#[no_mangle]
pub unsafe extern "C" fn _operator_ipow_impl(module: *mut core::ffi::c_void, a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _operator_ipow - from cpython/_operator.c.h
#[no_mangle]
pub unsafe extern "C" fn _operator_ipow(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_tkapp_exprstring_impl - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprstring_impl(arg0: *mut core::ffi::c_void, s: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_tkapp_exprstring - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprstring(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_tkapp_exprlong_impl - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprlong_impl(arg0: *mut core::ffi::c_void, s: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_tkapp_exprlong - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprlong(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_tkapp_exprdouble_impl - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprdouble_impl(arg0: *mut core::ffi::c_void, s: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_tkapp_exprdouble - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprdouble(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_tkapp_exprboolean_impl - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprboolean_impl(arg0: *mut core::ffi::c_void, s: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _tkinter_tkapp_exprboolean - from cpython/_tkinter.c.h
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprboolean(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_WaitForSingleObject_impl - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_WaitForSingleObject_impl(module: *mut core::ffi::c_void, handle: *mut core::ffi::c_void, milliseconds: u32) -> i64 {
    0
}

/// _winapi_WaitForSingleObject - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_WaitForSingleObject(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// testingAccountingGetCountBytesIndirect - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn testingAccountingGetCountBytesIndirect(parser: usize) -> u64 {
    0
}

/// XmlPrologStateInit - from cpython/xmlrole.h
#[no_mangle]
pub unsafe extern "C" fn XmlPrologStateInit(state: *mut core::ffi::c_void) {

}

/// XmlPrologStateInitExternalEntity - from cpython/xmlrole.h
#[no_mangle]
pub unsafe extern "C" fn XmlPrologStateInitExternalEntity(state: *mut core::ffi::c_void) {

}

/// _ctypes_simple_instance - from cpython/ctypes.h
#[no_mangle]
pub unsafe extern "C" fn _ctypes_simple_instance(st: *mut core::ffi::c_void, obj: *mut core::ffi::c_void) -> i32 {
    0
}

/// process_single_task_node - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn process_single_task_node(unwinder: *mut core::ffi::c_void, task_addr: usize, task_info: *mut *mut core::ffi::c_void, result: *mut core::ffi::c_void) -> i32 {
    0
}

/// _OutputBuffer_InitAndGrow - from cpython/buffer.h
#[no_mangle]
pub unsafe extern "C" fn _OutputBuffer_InitAndGrow(buffer: *mut core::ffi::c_void, ob: *mut core::ffi::c_void, max_length: usize) -> i32 {
    0
}

/// _decimal_Decimal_exp_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_exp_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Decimal_exp - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_exp(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Decimal_log10_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_log10_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Decimal_log10 - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_log10(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Decimal_sqrt_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_sqrt_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Decimal_sqrt - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_sqrt(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// exponent - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn exponent(arg0: usize) -> usize {
    0
}

/// _decimal_Decimal_logical_invert_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_invert_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Decimal_logical_invert - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_invert(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Decimal_logb_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logb_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Decimal_logb - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logb(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Decimal_logical_and_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_and_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, other: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Decimal_logical_and - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_and(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Decimal_logical_or_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_or_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, other: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Decimal_logical_or - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_or(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Decimal_logical_xor_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_xor_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, other: *mut core::ffi::c_void, context: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Decimal_logical_xor - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_xor(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_exp_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_exp_impl(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, x: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_exp - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_exp(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_log10_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_log10_impl(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, x: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_log10 - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_log10(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_sqrt_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_sqrt_impl(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, x: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_sqrt - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_sqrt(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_power_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_power_impl(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, base: *mut core::ffi::c_void, exp: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_power - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_power(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_logb_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logb_impl(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, x: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_logb - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logb(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_logical_invert_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_invert_impl(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, x: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_logical_invert - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_invert(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_logical_and_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_and_impl(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, x: *mut core::ffi::c_void, y: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_logical_and - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_and(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_logical_or_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_or_impl(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, x: *mut core::ffi::c_void, y: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_logical_or - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_or(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_logical_xor_impl - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_xor_impl(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, x: *mut core::ffi::c_void, y: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _decimal_Context_logical_xor - from cpython/_decimal.c.h
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_xor(context: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _mpd_singlemul - from cpython/basearith.h
#[no_mangle]
pub unsafe extern "C" fn _mpd_singlemul(w2: usize, u: usize, v: usize) {

}

/// ispower2 - from cpython/bits.h
#[no_mangle]
pub unsafe extern "C" fn ispower2(n: usize) -> i32 {
    0
}

/// cosh - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn cosh(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// sinh - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn sinh(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// c_log - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn c_log(z: usize) -> usize {
    0
}

/// cos - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn cos(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// sin - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn sin(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// Py_GetConstant - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn Py_GetConstant(arg0: usize) -> usize {
    0
}

/// _approximate_isqrt - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _approximate_isqrt(n: u64) -> u32 {
    0
}

/// math_integer_isqrt - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn math_integer_isqrt(module: *mut core::ffi::c_void, n: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// m_sinpi - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn m_sinpi(x: f64) -> f64 {
    0.0
}

/// exp - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn exp(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// m_log - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn m_log(x: f64) -> f64 {
    0.0
}

/// m_log2 - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn m_log2(x: f64) -> f64 {
    0.0
}

/// log2 - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn log2(arg0: usize) -> usize {
    0
}

/// m_log10 - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn m_log10(x: f64) -> f64 {
    0.0
}

/// log10 - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn log10(arg0: usize) -> usize {
    0
}

/// loghelper_int - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn loghelper_int(arg: *mut core::ffi::c_void, func: f64) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// loghelper - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn loghelper(arg: *mut core::ffi::c_void, func: f64) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// math_log - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn math_log(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// math_log2 - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn math_log2(module: *mut core::ffi::c_void, x: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// math_log10 - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn math_log10(module: *mut core::ffi::c_void, x: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyUnicode_DecodeFSDefaultAndSize - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyUnicode_DecodeFSDefaultAndSize(arg0: usize, arg1: usize) -> usize {
    0
}

/// HINSTANCE - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn HINSTANCE(arg0: *mut core::ffi::c_void) {

}

/// pyexpat_get_state - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn pyexpat_get_state(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// invalid_expat_handler_rv - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn invalid_expat_handler_rv(name: *mut i8) {

}

/// pyexpat_capsule_destructor - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn pyexpat_capsule_destructor(capsule: *mut core::ffi::c_void) {

}

/// pyexpat_exec - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn pyexpat_exec(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// pyexpat_traverse - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn pyexpat_traverse(module: *mut core::ffi::c_void, visit: usize, arg: *mut core::ffi::c_void) -> i32 {
    0
}

/// pyexpat_clear - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn pyexpat_clear(module: *mut core::ffi::c_void) -> i32 {
    0
}

/// PyInit_pyexpat - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit_pyexpat() -> usize {
    0
}

/// signal_add_constants - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn signal_add_constants(module: *mut core::ffi::c_void) -> i32 {
    0
}

/// symtable_init_constants - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn symtable_init_constants(m: *mut core::ffi::c_void) -> i32 {
    0
}

/// syslog_get_argv - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn syslog_get_argv() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// LOG_MASK - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn LOG_MASK(arg0: usize) -> usize {
    0
}

/// LOG_UPTO - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn LOG_UPTO(arg0: usize) -> usize {
    0
}

/// syslog_exec - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn syslog_exec(module: *mut core::ffi::c_void) -> i32 {
    0
}

/// PyInit_syslog - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit_syslog() -> usize {
    0
}

/// Xxo_get_x_exports - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn Xxo_get_x_exports(op: *mut core::ffi::c_void, Py_UNUSEDclosure: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// OutputBuffer_InitAndGrow - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn OutputBuffer_InitAndGrow(buffer: *mut core::ffi::c_void, max_length: usize, next_out: *mut *mut core::ffi::c_void, avail_out: *mut u32) -> usize {
    0
}

/// defdict_missing - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn defdict_missing(op: *mut core::ffi::c_void, key: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// __missing__ - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn __missing__() -> usize {
    0
}

/// expat_set_error - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn expat_set_error(st: *mut core::ffi::c_void, error_code: usize, line: usize, column: usize, message: *mut i8) {

}

/// expat_default_handler - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn expat_default_handler(op: *mut core::ffi::c_void, data_in: *mut core::ffi::c_void, data_len: i32) {

}

/// expat_start_handler - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn expat_start_handler(op: *mut core::ffi::c_void, tag_in: *mut core::ffi::c_void, attrib_in: *mut *mut core::ffi::c_void) {

}

/// expat_data_handler - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn expat_data_handler(op: *mut core::ffi::c_void, data_in: *mut core::ffi::c_void, data_len: i32) {

}

/// expat_end_handler - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn expat_end_handler(op: *mut core::ffi::c_void, tag_in: *mut core::ffi::c_void) {

}

/// expat_start_ns_handler - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn expat_start_ns_handler(op: *mut core::ffi::c_void, prefix_in: *mut core::ffi::c_void, uri_in: *mut core::ffi::c_void) {

}

/// expat_end_ns_handler - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn expat_end_ns_handler(op: *mut core::ffi::c_void, prefix_in: *mut core::ffi::c_void) {

}

/// expat_comment_handler - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn expat_comment_handler(op: *mut core::ffi::c_void, comment_in: *mut core::ffi::c_void) {

}

/// expat_start_doctype_handler - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn expat_start_doctype_handler(op: *mut core::ffi::c_void, doctype_name: *mut core::ffi::c_void, sysid: *mut core::ffi::c_void, pubid: *mut core::ffi::c_void, has_internal_subset: i32) {

}

/// expat_pi_handler - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn expat_pi_handler(op: *mut core::ffi::c_void, target_in: *mut core::ffi::c_void, data_in: *mut core::ffi::c_void) {

}

/// expat_parse - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn expat_parse(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// hashlib_constants - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn hashlib_constants(module: *mut core::ffi::c_void) -> i32 {
    0
}

/// _waiting_finish_releasing - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _waiting_finish_releasing(waiting: *mut core::ffi::c_void) {

}

/// _channel_clear_closing - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _channel_clear_closing(arg0: *mut core::ffi::c_void) {

}

/// _channel_finish_closing - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _channel_finish_closing(arg0: *mut core::ffi::c_void) {

}

/// _channel_set_closing - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _channel_set_closing(arg0: *mut core::ffi::c_void, arg1: usize) -> i32 {
    0
}

/// _parse_constant - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _parse_constant(s: *mut core::ffi::c_void, constant: *mut i8, idx: usize, next_idx_ptr: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// module_add_int_constant - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn module_add_int_constant(m: *mut core::ffi::c_void, name: *mut i8, value: i64) -> i32 {
    0
}

/// PyNumber_Power - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyNumber_Power(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// PyNumber_InPlacePower - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyNumber_InPlacePower(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// save_singleton_type - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn save_singleton_type(state: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, obj: *mut core::ffi::c_void, singleton: *mut core::ffi::c_void) -> i32 {
    0
}

/// instantiate - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn instantiate(cls: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// ADD_AD_CONSTANT - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn ADD_AD_CONSTANT(arg0: usize) -> usize {
    0
}

/// pack_single - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn pack_single(ptr: *mut i8, item: *mut core::ffi::c_void, fmt: *mut i8, itemsize: usize) -> i32 {
    0
}

/// unpack_single - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn unpack_single(ptr: *mut i8, fmt: *mut i8, itemsize: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// test_lazy_hash_inheritance - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn test_lazy_hash_inheritance(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// argparsing - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn argparsing(o: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// ipowType_ipow - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn ipowType_ipow(arg0: *mut core::ffi::c_void, other: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// check_edit_cost - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn check_edit_cost(a: *mut i8, b: *mut i8, expected: usize) -> i32 {
    0
}

/// test_edit_cost - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn test_edit_cost(arg0: *mut core::ffi::c_void, Py_UNUSEDargs: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// get_crossinterp_data - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn get_crossinterp_data(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void, kwargs: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// restore_crossinterp_data - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn restore_crossinterp_data(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// testexport_foo - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn testexport_foo(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyInit__testmultiphase_export_uninitialized - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_export_uninitialized() -> usize {
    0
}

/// PyInit__testmultiphase_export_null - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_export_null() -> usize {
    0
}

/// PyInit__testmultiphase_export_raise - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_export_raise() -> usize {
    0
}

/// PyInit__testmultiphase_export_unreported_exception - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_export_unreported_exception() -> usize {
    0
}

/// PyModule_AddIntConstant - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyModule_AddIntConstant(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// PyModExport__test_from_modexport_null - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyModExport__test_from_modexport_null() -> usize {
    0
}

/// PyModInit__test_from_modexport_null - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyModInit__test_from_modexport_null() -> usize {
    0
}

/// PyModExport__test_from_modexport_exception - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyModExport__test_from_modexport_exception() -> usize {
    0
}

/// PyModInit__test_from_modexport_exception - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyModInit__test_from_modexport_exception() -> usize {
    0
}

/// modexport_create_string - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn modexport_create_string(spec: *mut core::ffi::c_void, def: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyModExport__test_from_modexport_empty_slots - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyModExport__test_from_modexport_empty_slots() -> usize {
    0
}

/// PyModExport__test_from_modexport_minimal_slots - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyModExport__test_from_modexport_minimal_slots() -> usize {
    0
}

/// modexport_smoke_exec - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn modexport_smoke_exec(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// modexport_smoke_get_state_int - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn modexport_smoke_get_state_int(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// modexport_smoke_get_test_token - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn modexport_smoke_get_test_token(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// modexport_get_minimal_slots - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn modexport_get_minimal_slots(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// init__testsinglephase_basic - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn init__testsinglephase_basic(def: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyInit__testsinglephase - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase() -> usize {
    0
}

/// PyInit__testsinglephase_basic_wrapper - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase_basic_wrapper() -> usize {
    0
}

/// PyInit__testsinglephase_with_reinit - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase_with_reinit() -> usize {
    0
}

/// PyInit__testsinglephase_with_state - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase_with_state() -> usize {
    0
}

/// PyInit__testsinglephase_check_cache_first - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase_check_cache_first() -> usize {
    0
}

/// PyInit__testsinglephase_with_reinit_check_cache_first - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase_with_reinit_check_cache_first() -> usize {
    0
}

/// PyInit__testsinglephase_with_state_check_cache_first - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase_with_state_check_cache_first() -> usize {
    0
}

/// PyInit__testsinglephase_raise_exception - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase_raise_exception() -> usize {
    0
}

/// IAGetPrimitiveTopology - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn IAGetPrimitiveTopology(pTopology: *mut core::ffi::c_void) -> usize {
    0
}

/// VSSetConstantBuffers - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn VSSetConstantBuffers(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// VSGetConstantBuffers - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn VSGetConstantBuffers(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GSSetConstantBuffers - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn GSSetConstantBuffers(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GSGetConstantBuffers - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn GSGetConstantBuffers(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// PSSetConstantBuffers - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn PSSetConstantBuffers(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// PSGetConstantBuffers - from dxvk/d3d10_device.h
#[no_mangle]
pub unsafe extern "C" fn PSGetConstantBuffers(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// DecodeLogicOp - from dxvk/d3d11_blend.h
#[no_mangle]
pub unsafe extern "C" fn DecodeLogicOp(LogicOp: usize) -> usize {
    0
}

/// ValidateLogicOp - from dxvk/d3d11_blend.h
#[no_mangle]
pub unsafe extern "C" fn ValidateLogicOp(LogicOp: usize) -> usize {
    0
}

/// EncodeInstanceData - from dxvk/d3d11_class_linkage.h
#[no_mangle]
pub unsafe extern "C" fn EncodeInstanceData(SlotId: u32, pInstance: *mut core::ffi::c_void) -> usize {
    0
}

/// GetInstanceName - from dxvk/d3d11_class_linkage.h
#[no_mangle]
pub unsafe extern "C" fn GetInstanceName(pInstanceName: *mut i8, pBufferLength: *mut core::ffi::c_void) -> usize {
    0
}

/// CreateClassInstance - from dxvk/d3d11_class_linkage.h
#[no_mangle]
pub unsafe extern "C" fn CreateClassInstance(pTypeName: *const i8, ConstantBufferOffset: u32, ConstantVectorOffset: u32, TextureOffset: u32, SamplerOffset: u32, ppInstance: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// GetClassInstance - from dxvk/d3d11_class_linkage.h
#[no_mangle]
pub unsafe extern "C" fn GetClassInstance(pInstanceName: *const i8, InstanceIndex: u32, ppInstance: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// AddInstance - from dxvk/d3d11_class_linkage.h
#[no_mangle]
pub unsafe extern "C" fn AddInstance(pDesc: *mut core::ffi::c_void, pTypeName: *const i8, pInstanceName: *const i8) {

}

/// DrawIndexedInstancedIndirect - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DrawIndexedInstancedIndirect(pBufferForArgs: *mut core::ffi::c_void, AlignedByteOffsetForArgs: u32) -> usize {
    0
}

/// DrawInstancedIndirect - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DrawInstancedIndirect(pBufferForArgs: *mut core::ffi::c_void, AlignedByteOffsetForArgs: u32) -> usize {
    0
}

/// VSSetConstantBuffers1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn VSSetConstantBuffers1(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) -> usize {
    0
}

/// VSGetConstantBuffers1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn VSGetConstantBuffers1(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) -> usize {
    0
}

/// HSSetConstantBuffers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn HSSetConstantBuffers(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// HSSetConstantBuffers1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn HSSetConstantBuffers1(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) -> usize {
    0
}

/// HSGetConstantBuffers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn HSGetConstantBuffers(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// HSGetConstantBuffers1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn HSGetConstantBuffers1(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) -> usize {
    0
}

/// DSSetConstantBuffers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DSSetConstantBuffers(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// DSSetConstantBuffers1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DSSetConstantBuffers1(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) -> usize {
    0
}

/// DSGetConstantBuffers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DSGetConstantBuffers(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// DSGetConstantBuffers1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DSGetConstantBuffers1(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) -> usize {
    0
}

/// GSSetConstantBuffers1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn GSSetConstantBuffers1(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) -> usize {
    0
}

/// GSGetConstantBuffers1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn GSGetConstantBuffers1(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) -> usize {
    0
}

/// PSSetConstantBuffers1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn PSSetConstantBuffers1(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) -> usize {
    0
}

/// PSGetConstantBuffers1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn PSGetConstantBuffers1(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) -> usize {
    0
}

/// CSSetConstantBuffers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CSSetConstantBuffers(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CSSetConstantBuffers1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CSSetConstantBuffers1(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) -> usize {
    0
}

/// CSGetConstantBuffers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CSGetConstantBuffers(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CSGetConstantBuffers1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn CSGetConstantBuffers1(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) -> usize {
    0
}

/// ApplyDirtyConstantBuffers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ApplyDirtyConstantBuffers(Stage: usize, BoundMask: usize, DirtyMask: usize) {

}

/// ApplyPrimitiveTopology - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn ApplyPrimitiveTopology() {

}

/// BindConstantBuffer - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BindConstantBuffer(ShaderStage: usize, Slot: u32, pBuffer: *mut core::ffi::c_void, Offset: u32, Length: u32) {

}

/// BindConstantBufferRange - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn BindConstantBufferRange(ShaderStage: usize, Slot: u32, Offset: u32, Length: u32) {

}

/// DirtyConstantBuffer - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn DirtyConstantBuffer(ShaderStage: usize, Slot: u32, IsNull: usize) -> usize {
    0
}

/// GetConstantBuffers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn GetConstantBuffers(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) {

}

/// RestoreConstantBuffers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn RestoreConstantBuffers(Stage: usize) {

}

/// SetConstantBuffers - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn SetConstantBuffers(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void) {

}

/// SetConstantBuffers1 - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn SetConstantBuffers1(StartSlot: u32, NumBuffers: u32, ppConstantBuffers: *mut *mut core::ffi::c_void, pFirstConstant: *mut u32, pNumConstants: *mut u32) {

}

/// SetClassInstances - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn SetClassInstances(pShader: *mut core::ffi::c_void, ppClassInstances: *mut *mut core::ffi::c_void, NumClassInstances: u32) {

}

/// GetClassInstances - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn GetClassInstances(ppClassInstances: *mut *mut core::ffi::c_void, pNumClassInstances: *mut u32) {

}

/// AddCost - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn AddCost(Value: u64) -> usize {
    0
}

/// InitDefaultPrimitiveTopology - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn InitDefaultPrimitiveTopology() -> usize {
    0
}

/// InitDefaultLogicOpState - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn InitDefaultLogicOpState() -> usize {
    0
}

/// constexpr - from dxvk/d3d11_context.h
#[no_mangle]
pub unsafe extern "C" fn constexpr(AllowFlush: usize) -> usize {
    0
}

/// GetInstanceExtensions - from dxvk/d3d11_on_12_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn GetInstanceExtensions(pExtensionCount: *mut u32, ppExtensions: *mut *mut i8) -> usize {
    0
}

/// GetClassInstanceData - from dxvk/d3d11_shader.h
#[no_mangle]
pub unsafe extern "C" fn GetClassInstanceData(Slot: u32, pClassInstance: *mut core::ffi::c_void) -> usize {
    0
}

/// ExportImageInfo - from dxvk/d3d11_texture.h
#[no_mangle]
pub unsafe extern "C" fn ExportImageInfo() {

}

/// VideoProcessorSetStreamAutoProcessingMode - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetStreamAutoProcessingMode(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, Enable: i32) -> usize {
    0
}

/// VideoProcessorGetStreamAutoProcessingMode - from dxvk/d3d11_video.h
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetStreamAutoProcessingMode(pVideoProcessor: *mut core::ffi::c_void, StreamIndex: u32, pEnabled: *mut i32) -> usize {
    0
}

/// SetVertexShaderConstant - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetVertexShaderConstant(StartRegister: u32, pConstantData: *mut core::ffi::c_void, ConstantCount: u32) -> usize {
    0
}

/// GetVertexShaderConstant - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetVertexShaderConstant(Register: u32, pConstantData: *mut core::ffi::c_void, ConstantCount: u32) -> usize {
    0
}

/// GetPixelShaderConstant - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn GetPixelShaderConstant(Register: u32, pConstantData: *mut core::ffi::c_void, ConstantCount: u32) -> usize {
    0
}

/// SetPixelShaderConstant - from dxvk/d3d8_device.h
#[no_mangle]
pub unsafe extern "C" fn SetPixelShaderConstant(StartRegister: u32, pConstantData: *mut core::ffi::c_void, ConstantCount: u32) -> usize {
    0
}

/// SetSoftwareVertexProcessing - from dxvk/d3d8_state_block.h
#[no_mangle]
pub unsafe extern "C" fn SetSoftwareVertexProcessing(value: usize) -> i32 {
    0
}

/// ExposedMipLevels - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn ExposedMipLevels() -> u32 {
    0
}

/// SetDialogBoxMode - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetDialogBoxMode(bEnableDialogs: i32) -> usize {
    0
}

/// GetSoftwareVertexProcessing - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetSoftwareVertexProcessing() -> usize {
    0
}

/// SetVertexShaderConstantF - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetVertexShaderConstantF(StartRegister: u32, pConstantData: *mut f32, Vector4fCount: u32) -> usize {
    0
}

/// GetVertexShaderConstantF - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetVertexShaderConstantF(StartRegister: u32, pConstantData: *mut f32, Vector4fCount: u32) -> usize {
    0
}

/// SetVertexShaderConstantI - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetVertexShaderConstantI(StartRegister: u32, pConstantData: *mut i32, Vector4iCount: u32) -> usize {
    0
}

/// GetVertexShaderConstantI - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetVertexShaderConstantI(StartRegister: u32, pConstantData: *mut i32, Vector4iCount: u32) -> usize {
    0
}

/// SetVertexShaderConstantB - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetVertexShaderConstantB(StartRegister: u32, pConstantData: *mut i32, BoolCount: u32) -> usize {
    0
}

/// GetVertexShaderConstantB - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetVertexShaderConstantB(StartRegister: u32, pConstantData: *mut i32, BoolCount: u32) -> usize {
    0
}

/// SetPixelShaderConstantF - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetPixelShaderConstantF(StartRegister: u32, pConstantData: *mut f32, Vector4fCount: u32) -> usize {
    0
}

/// GetPixelShaderConstantF - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetPixelShaderConstantF(StartRegister: u32, pConstantData: *mut f32, Vector4fCount: u32) -> usize {
    0
}

/// SetPixelShaderConstantI - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetPixelShaderConstantI(StartRegister: u32, pConstantData: *mut i32, Vector4iCount: u32) -> usize {
    0
}

/// GetPixelShaderConstantI - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetPixelShaderConstantI(StartRegister: u32, pConstantData: *mut i32, Vector4iCount: u32) -> usize {
    0
}

/// SetPixelShaderConstantB - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetPixelShaderConstantB(StartRegister: u32, pConstantData: *mut i32, BoolCount: u32) -> usize {
    0
}

/// GetPixelShaderConstantB - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetPixelShaderConstantB(StartRegister: u32, pConstantData: *mut i32, BoolCount: u32) -> usize {
    0
}

/// CreateConstantBuffers - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn CreateConstantBuffers() {

}

/// BindViewportAndScissor - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BindViewportAndScissor() {

}

/// UploadSoftwareConstantSet - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UploadSoftwareConstantSet(Src: usize, Layout: usize) {

}

/// CopySoftwareConstants - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn CopySoftwareConstants(dstBuffer: usize, src: *mut core::ffi::c_void, size: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// UploadConstantSet - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UploadConstantSet(Src: usize, Layout: usize, Shader: usize) {

}

/// UploadConstants - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UploadConstants() {

}

/// UpdatePushConstant - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdatePushConstant(pData: *mut core::ffi::c_void) {

}

/// GetInstanceCount - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn GetInstanceCount() -> u32 {
    0
}

/// DetermineConstantLayouts - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn DetermineConstantLayouts(canSWVP: usize) {

}

/// SetShaderConstants - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn SetShaderConstants(StartRegister: u32, pConstantData: *mut core::ffi::c_void, Count: u32) -> i32 {
    0
}

/// BindSpecConstants - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn BindSpecConstants() {

}

/// GetSharedConstants - from dxvk/d3d9_fixed_function.h
#[no_mangle]
pub unsafe extern "C" fn GetSharedConstants(spvModule: usize) -> u32 {
    0
}

/// GetInstanceHandle - from dxvk/d3d9_interfaces.h
#[no_mangle]
pub unsafe extern "C" fn GetInstanceHandle(pInstance: *mut core::ffi::c_void) -> usize {
    0
}

/// getSpecConstantBufferSlot - from dxvk/d3d9_shader.h
#[no_mangle]
pub unsafe extern "C" fn getSpecConstantBufferSlot() -> usize {
    0
}

/// GetMaxDefinedFloatConstant - from dxvk/d3d9_shader.h
#[no_mangle]
pub unsafe extern "C" fn GetMaxDefinedFloatConstant() -> i32 {
    0
}

/// GetMaxDefinedIntConstant - from dxvk/d3d9_shader.h
#[no_mangle]
pub unsafe extern "C" fn GetMaxDefinedIntConstant() -> i32 {
    0
}

/// GetMaxDefinedBoolConstant - from dxvk/d3d9_shader.h
#[no_mangle]
pub unsafe extern "C" fn GetMaxDefinedBoolConstant() -> i32 {
    0
}

/// GetDXVKInstance - from dxvk/dxgi_adapter.h
#[no_mangle]
pub unsafe extern "C" fn GetDXVKInstance() -> usize {
    0
}

/// GetVulkanInstance - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn GetVulkanInstance(pInstance: *mut core::ffi::c_void, ppfnVkGetInstanceProcAddr: *mut core::ffi::c_void) -> usize {
    0
}

/// processInstruction - from dxvk/dxso_analysis.h
#[no_mangle]
pub unsafe extern "C" fn processInstruction(ctx: usize) {

}

/// maxDefinedFloatConstant - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn maxDefinedFloatConstant() -> i32 {
    0
}

/// maxDefinedIntConstant - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn maxDefinedIntConstant() -> i32 {
    0
}

/// maxDefinedBoolConstant - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn maxDefinedBoolConstant() -> i32 {
    0
}

/// emitDclSwvpConstantBuffer - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitDclSwvpConstantBuffer() -> i32 {
    0
}

/// emitDclConstantBuffer - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitDclConstantBuffer() {

}

/// emitVsInit - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitVsInit() {

}

/// emitPsSharedConstants - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitPsSharedConstants() {

}

/// emitPsInit - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitPsInit() {

}

/// emitLoadConstant - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitLoadConstant(reg: usize, relative: *mut core::ffi::c_void) -> usize {
    0
}

/// emitPsProcessing - from dxvk/dxso_compiler.h
#[no_mangle]
pub unsafe extern "C" fn emitPsProcessing() {

}

/// relativeAddressingUsesToken - from dxvk/dxso_decoder.h
#[no_mangle]
pub unsafe extern "C" fn relativeAddressingUsesToken(arg0: usize) -> usize {
    0
}

/// cmdPushConstants - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdPushConstants(cmdBuffer: usize, info: *mut core::ffi::c_void) {

}

/// cmdSetBlendConstants - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetBlendConstants(blendConstants4: f32) {

}

/// getSpecConstantMask - from dxvk/dxvk_compute.h
#[no_mangle]
pub unsafe extern "C" fn getSpecConstantMask() -> u32 {
    0
}

/// createInstance - from dxvk/dxvk_compute.h
#[no_mangle]
pub unsafe extern "C" fn createInstance(state: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// findInstance - from dxvk/dxvk_compute.h
#[no_mangle]
pub unsafe extern "C" fn findInstance(state: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// logPipelineState - from dxvk/dxvk_compute.h
#[no_mangle]
pub unsafe extern "C" fn logPipelineState(level: usize, state: usize) {

}

/// setPrimitiveTopology - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setPrimitiveTopology(topology: usize) {

}

/// logicOpEnable - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn logicOpEnable() -> usize {
    0
}

/// logicOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn logicOp() -> usize {
    0
}

/// VkLogicOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn VkLogicOp(arg0: usize) -> usize {
    0
}

/// setLogicOp - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setLogicOp(enable: usize, op: usize) {

}

/// setBlendConstants - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn setBlendConstants(blendConstants: usize) {

}

/// setLogicOpState - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn setLogicOpState(lo: usize) {

}

/// flushClearsInline - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn flushClearsInline() {

}

/// resetSpecConstants - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn resetSpecConstants(newMask: u32) {

}

/// updateSpecConstants - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn updateSpecConstants() {

}

/// isInUse - from dxvk/dxvk_descriptor_heap.h
#[no_mangle]
pub unsafe extern "C" fn isInUse() -> usize {
    0
}

/// logDescriptorProperties - from dxvk/dxvk_descriptor_info.h
#[no_mangle]
pub unsafe extern "C" fn logDescriptorProperties() {

}

/// logBindingModel - from dxvk/dxvk_device.h
#[no_mangle]
pub unsafe extern "C" fn logBindingModel() {

}

/// logDeviceInfo - from dxvk/dxvk_device_info.h
#[no_mangle]
pub unsafe extern "C" fn logDeviceInfo() {

}

/// getInstanceExtensions - from dxvk/dxvk_extension_provider.h
#[no_mangle]
pub unsafe extern "C" fn getInstanceExtensions() -> usize {
    0
}

/// initInstanceExtensions - from dxvk/dxvk_extension_provider.h
#[no_mangle]
pub unsafe extern "C" fn initInstanceExtensions() -> usize {
    0
}

/// addConstant - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn addConstant(id: u32, value: u32) {

}

/// computeSpecConstantMask - from dxvk/dxvk_graphics.h
#[no_mangle]
pub unsafe extern "C" fn computeSpecConstantMask() -> u32 {
    0
}

/// enableLogicOp - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn enableLogicOp() -> u32 {
    0
}

/// useDynamicBlendConstants - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn useDynamicBlendConstants() -> usize {
    0
}

/// isInitialized - from dxvk/dxvk_image.h
#[no_mangle]
pub unsafe extern "C" fn isInitialized(subresource: usize) -> usize {
    0
}

/// DxvkInstance - from dxvk/dxvk_instance.h
#[no_mangle]
pub unsafe extern "C" fn DxvkInstance(flags: usize) -> usize {
    0
}

/// initVulkanInstance - from dxvk/dxvk_instance.h
#[no_mangle]
pub unsafe extern "C" fn initVulkanInstance(args: usize, flags: usize) -> usize {
    0
}

/// countEmptyChunksInPool - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn countEmptyChunksInPool(pool: usize) -> u32 {
    0
}

/// logMemoryError - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn logMemoryError(req: usize) {

}

/// logMemoryStats - from dxvk/dxvk_memory.h
#[no_mangle]
pub unsafe extern "C" fn logMemoryStats() {

}

/// queryInstanceExtensions - from dxvk/dxvk_openvr.h
#[no_mangle]
pub unsafe extern "C" fn queryInstanceExtensions() -> usize {
    0
}

/// getAllDescriptorsInSet - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getAllDescriptorsInSet(arg0: usize, set: u32) -> usize {
    0
}

/// getResourcesInSet - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getResourcesInSet(arg0: usize, set: u32) -> usize {
    0
}

/// getUniformBuffersInSet - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getUniformBuffersInSet(arg0: usize, set: u32) -> usize {
    0
}

/// setAniso - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn setAniso(anisotropy: u32) {

}

/// samplerIsInLruList - from dxvk/dxvk_sampler.h
#[no_mangle]
pub unsafe extern "C" fn samplerIsInLruList(sampler: usize, index: i32) -> usize {
    0
}

/// exportBuiltIn - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn exportBuiltIn(builder: usize, builtIn: usize, value: usize) {

}

/// exportOutput - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn exportOutput(builder: usize, location: u32, value: usize, name: *mut i8) {

}

/// makeConstantVector - from dxvk/dxvk_shader_builtin.h
#[no_mangle]
pub unsafe extern "C" fn makeConstantVector(builder: usize, constant: usize, arg2: usize) -> usize {
    0
}

/// patchInputTopology - from dxvk/dxvk_shader_spirv.h
#[no_mangle]
pub unsafe extern "C" fn patchInputTopology(code: usize, topology: usize) {

}

/// logSparseBindingInfo - from dxvk/dxvk_sparse.h
#[no_mangle]
pub unsafe extern "C" fn logSparseBindingInfo(level: usize, info: *mut core::ffi::c_void) {

}

/// isBlendConstantBlendFactor - from dxvk/dxvk_util.h
#[no_mangle]
pub unsafe extern "C" fn isBlendConstantBlendFactor(factor: usize) -> usize {
    0
}

/// opShiftLeftLogical - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opShiftLeftLogical(resultType: u32, base: u32, shift: u32) -> u32 {
    0
}

/// opShiftRightLogical - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opShiftRightLogical(resultType: u32, base: u32, shift: u32) -> u32 {
    0
}

/// opLogicalEqual - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opLogicalEqual(resultType: u32, operand1: u32, operand2: u32) -> u32 {
    0
}

/// opLogicalNotEqual - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opLogicalNotEqual(resultType: u32, operand1: u32, operand2: u32) -> u32 {
    0
}

/// opLogicalAnd - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opLogicalAnd(resultType: u32, operand1: u32, operand2: u32) -> u32 {
    0
}

/// opLogicalOr - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opLogicalOr(resultType: u32, operand1: u32, operand2: u32) -> u32 {
    0
}

/// opLogicalNot - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opLogicalNot(resultType: u32, operand: u32) -> u32 {
    0
}

/// opSin - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSin(resultType: u32, vector: u32) -> u32 {
    0
}

/// opCos - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opCos(resultType: u32, vector: u32) -> u32 {
    0
}

/// opSqrt - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSqrt(resultType: u32, operand: u32) -> u32 {
    0
}

/// opInverseSqrt - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opInverseSqrt(resultType: u32, operand: u32) -> u32 {
    0
}

/// opExp2 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opExp2(resultType: u32, operand: u32) -> u32 {
    0
}

/// opExp - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opExp(resultType: u32, operand: u32) -> u32 {
    0
}

/// opLog2 - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opLog2(resultType: u32, operand: u32) -> u32 {
    0
}

/// opPow - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opPow(resultType: u32, base: u32, exponent: u32) -> u32 {
    0
}

/// opIsInf - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opIsInf(resultType: u32, operand: u32) -> u32 {
    0
}

/// opImageSampleExplicitLod - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageSampleExplicitLod(resultType: u32, sampledImage: u32, coordinates: u32, operands: usize) -> u32 {
    0
}

/// opImageSampleProjExplicitLod - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageSampleProjExplicitLod(resultType: u32, sampledImage: u32, coordinates: u32, operands: usize) -> u32 {
    0
}

/// opImageSampleDrefExplicitLod - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageSampleDrefExplicitLod(resultType: u32, sampledImage: u32, coordinates: u32, reference: u32, operands: usize) -> u32 {
    0
}

/// opImageSampleProjDrefExplicitLod - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageSampleProjDrefExplicitLod(resultType: u32, sampledImage: u32, coordinates: u32, reference: u32, operands: usize) -> u32 {
    0
}

/// opSinCos - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opSinCos(x: u32, useBuiltIn: usize) -> u32 {
    0
}

/// isInterfaceVar - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn isInterfaceVar(sclass: usize) -> usize {
    0
}

/// sincosTaylorFactor - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn sincosTaylorFactor(power: u32) -> usize {
    0
}

/// isInModuleDetachment - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn isInModuleDetachment() -> usize {
    0
}

/// instance - from dxvk/vulkan_loader.h
#[no_mangle]
pub unsafe extern "C" fn instance() -> usize {
    0
}

/// getSpecConstants - from dxvk/dxvk_hud_renderer.h
#[no_mangle]
pub unsafe extern "C" fn getSpecConstants(key: usize) -> usize {
    0
}

/// getPushConstants - from dxvk/dxvk_hud_renderer.h
#[no_mangle]
pub unsafe extern "C" fn getPushConstants() -> usize {
    0
}

/// logQueryInterfaceError - from dxvk/com_guid.h
#[no_mangle]
pub unsafe extern "C" fn logQueryInterfaceError(objectGuid: usize, requestedGuid: usize) -> usize {
    0
}

/// logOptions - from dxvk/config.h
#[no_mangle]
pub unsafe extern "C" fn logOptions() {

}

/// logLevel - from dxvk/log.h
#[no_mangle]
pub unsafe extern "C" fn logLevel() -> usize {
    0
}

/// getMinLogLevel - from dxvk/log.h
#[no_mangle]
pub unsafe extern "C" fn getMinLogLevel() -> usize {
    0
}

/// demo_window_set_expose_func - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn demo_window_set_expose_func(window: *mut core::ffi::c_void, expose_func: *mut core::ffi::c_void) {

}

/// vkd3d_create_instance - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_instance(create_info: *mut core::ffi::c_void, instance: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_instance_decref - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_instance_decref(instance: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_instance_get_vk_instance - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_instance_get_vk_instance(instance: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_instance_incref - from vkd3d-proton/vkd3d.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_instance_incref(instance: *mut core::ffi::c_void) -> u32 {
    0
}

/// vkd3d_shader_parse_patch_constant_signature - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_parse_patch_constant_signature(dxbc: *mut core::ffi::c_void, signature: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_compile_dxil_export - from vkd3d-proton/vkd3d_shader.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_compile_dxil_export(dxil: *mut core::ffi::c_void, export: *mut i8, demangled_export: *mut i8, spirv: *mut core::ffi::c_void, spirv_debug: *mut core::ffi::c_void, shader_interface_info: *mut core::ffi::c_void, shader_interface_local_info: *mut core::ffi::c_void, compiler_args: *mut core::ffi::c_void) -> i32 {
    0
}

/// is_standard_swizzle_64kb_supported - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn is_standard_swizzle_64kb_supported(device: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_atomic_uint32_load_explicit - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint32_load_explicit(target: *mut u32, order: usize) -> usize {
    0
}

/// vkd3d_atomic_uint32_store_explicit - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint32_store_explicit(target: *mut u32, value: u32, order: usize) -> usize {
    0
}

/// vkd3d_atomic_uint32_exchange_explicit - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint32_exchange_explicit(target: *mut u32, value: u32, order: usize) -> usize {
    0
}

/// vkd3d_atomic_uint64_load_explicit - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint64_load_explicit(target: *mut u64, order: usize) -> usize {
    0
}

/// vkd3d_atomic_uint64_store_explicit - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint64_store_explicit(target: *mut u64, value: u64, order: usize) -> usize {
    0
}

/// vkd3d_atomic_uint64_exchange_explicit - from vkd3d-proton/vkd3d_atomic.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint64_exchange_explicit(target: *mut u64, value: u64, order: usize) -> usize {
    0
}

/// vkd3d_log2i - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_log2i(x: u32) -> u32 {
    0
}

/// vkd3d_log2i_ceil - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_log2i_ceil(x: u32) -> u32 {
    0
}

/// is_power_of_two - from vkd3d-proton/vkd3d_common.h
#[no_mangle]
pub unsafe extern "C" fn is_power_of_two(x: u32) -> usize {
    0
}

/// d3d12_shared_fence_open_export_kmt - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_open_export_kmt(fence: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// d3d12_shared_fence_close_export_kmt - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_close_export_kmt(fence: *mut core::ffi::c_void) {

}

/// d3d12_resource_open_export_kmt - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_open_export_kmt(resource: *mut core::ffi::c_void, device: *mut core::ffi::c_void, allocation: *mut core::ffi::c_void) {

}

/// d3d12_resource_close_export_kmt - from vkd3d-proton/vkd3d_d3dkmt.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_close_export_kmt(resource: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_export_strequal - from vkd3d-proton/vkd3d_string.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_export_strequal(a: *mut u16, b: *mut u16) -> usize {
    0
}

/// vkd3d_export_strequal_mixed - from vkd3d-proton/vkd3d_string.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_export_strequal_mixed(a: *mut u16, b: *mut i8) -> usize {
    0
}

/// vkd3d_export_strequal_substr - from vkd3d-proton/vkd3d_string.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_export_strequal_substr(a: *mut u16, n: usize, b: *mut u16) -> usize {
    0
}

/// DEBUG_CHANNEL_INIT_IMPLICIT_INSTANCE - from vkd3d-proton/debug_channel.h
#[no_mangle]
pub unsafe extern "C" fn DEBUG_CHANNEL_INIT_IMPLICIT_INSTANCE(id: usize, inst: usize) {

}

/// d3d12_command_list_IASetPrimitiveTopology_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetPrimitiveTopology_profiled(iface: *mut core::ffi::c_void, topology: usize) -> usize {
    0
}

/// d3d12_command_list_SetViewInstanceMask_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetViewInstanceMask_profiled(iface: *mut core::ffi::c_void, mask: u32) -> usize {
    0
}

/// d3d12_command_list_OMSetFrontAndBackStencilRef_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetFrontAndBackStencilRef_profiled(iface: *mut core::ffi::c_void, FrontStencilRef: u32, BackStencilRef: u32) -> usize {
    0
}

/// d3d12_device_CreateConstantBufferView_profiled - from vkd3d-proton/device_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateConstantBufferView_profiled(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// vkd3d_descriptor_debug_active_log - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_active_log() -> usize {
    0
}

/// d3d12_fence_iface_set_native_sync_handle_on_completion_explicit - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_iface_set_native_sync_handle_on_completion_explicit(iface: *mut core::ffi::c_void, wait_type: usize, value: usize, handle: usize, payload: *mut u32) -> i32 {
    0
}

/// vkd3d_create_buffer_explicit_usage - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer_explicit_usage(device: *mut core::ffi::c_void, vk_usage: usize, vk_size: u64, tag: *mut i8, vk_buffer: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_desc_copy_single - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_copy_single(dst: usize, src: usize, device: *mut core::ffi::c_void) {

}

/// d3d12_desc_copy_embedded_resource_single_32 - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_copy_embedded_resource_single_32(dst_va: usize, src_va: usize) {

}

/// vkd3d_shader_debug_ring_init_spec_constant - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_debug_ring_init_spec_constant(device: *mut core::ffi::c_void, info: *mut core::ffi::c_void, hash: usize) {

}

/// vkd3d_queue_timeline_trace_register_instantaneous - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_instantaneous(trace: *mut core::ffi::c_void, arg1: usize, value: u64) {

}

/// vkd3d_export_equal - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_export_equal(export: *const u16, entry: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_load_vk_instance_procs - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_load_vk_instance_procs(procs: *mut core::ffi::c_void, global_procs: *mut core::ffi::c_void, instance: usize) -> i32 {
    0
}

/// vk_topology_from_d3d12_topology - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vk_topology_from_d3d12_topology(topology: usize) -> usize {
    0
}

/// shader_parse_patch_constant_signature - from vkd3d-proton/vkd3d_shader_private.h
#[no_mangle]
pub unsafe extern "C" fn shader_parse_patch_constant_signature(dxbc: *mut core::ffi::c_void, dxbc_length: usize, signature: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_shader_dump_spirv_shader_export - from vkd3d-proton/vkd3d_shader_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_dump_spirv_shader_export(hash: usize, shader: *mut core::ffi::c_void, export: *mut i8) {

}

/// vkd3d_shader_replace_export - from vkd3d-proton/vkd3d_shader_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_replace_export(hash: usize, data: *mut *mut core::ffi::c_void, size: *mut usize, export: *mut i8) -> usize {
    0
}

/// cosf - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cosf(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// sinf - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn sinf(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// cxg_expose - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn cxg_expose(window: *mut core::ffi::c_void, user_data: *mut core::ffi::c_void) {

}

/// check_clip_distance - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_clip_distance(context: *mut core::ffi::c_void, pso: *mut core::ffi::c_void, vbv2: usize, vb: *mut core::ffi::c_void, vs_cb: *mut core::ffi::c_void, gs_cb: *mut core::ffi::c_void) {

}

/// test_clip_distance - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_clip_distance(use_dxil: usize) {

}

/// test_clip_distance_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_clip_distance_dxbc() {

}

/// test_clip_distance_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_clip_distance_dxil() {

}

/// test_combined_clip_and_cull_distances - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_combined_clip_and_cull_distances(use_dxil: usize) {

}

/// test_combined_clip_and_cull_distances_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_combined_clip_and_cull_distances_dxbc() {

}

/// test_combined_clip_and_cull_distances_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_combined_clip_and_cull_distances_dxil() {

}

/// test_draw_instanced - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_draw_instanced() {

}

/// test_draw_indexed_instanced - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_draw_indexed_instanced() {

}

/// test_bundle_state_inheritance - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_bundle_state_inheritance() {

}

/// test_execute_indirect_multi_dispatch_root_constants - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_execute_indirect_multi_dispatch_root_constants() {

}

/// prepare_instanced_draw - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn prepare_instanced_draw(context: *mut core::ffi::c_void) {

}

/// test_aliasing_barrier - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_aliasing_barrier() {

}

/// R32_SINT - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn R32_SINT(arg0: usize) -> usize {
    0
}

/// test_stencil_export - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_stencil_export(use_dxil: usize) {

}

/// test_stencil_export_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_stencil_export_dxbc() {

}

/// test_stencil_export_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_stencil_export_dxil() {

}

/// test_update_descriptor_heap_after_closing_command_list - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_update_descriptor_heap_after_closing_command_list() {

}

/// test_null_descriptor_resinfo - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_null_descriptor_resinfo(use_dxil: usize) {

}

/// test_null_descriptor_resinfo_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_null_descriptor_resinfo_dxbc() {

}

/// test_null_descriptor_resinfo_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_null_descriptor_resinfo_dxil() {

}

/// powf - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn powf(arg0: usize) -> usize {
    0
}

/// test_topology_triangle_fan - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_topology_triangle_fan() {

}

/// test_coverage_export_atoc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_coverage_export_atoc(use_dxil: usize) {

}

/// test_coverage_export_atoc_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_coverage_export_atoc_dxbc() {

}

/// test_coverage_export_atoc_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_coverage_export_atoc_dxil() {

}

/// test_view_instancing - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_view_instancing() {

}

/// test_view_instancing_indirect_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_view_instancing_indirect_state() {

}

/// test_gs_topology_mismatch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_gs_topology_mismatch(dxil: usize) {

}

/// test_gs_topology_mismatch_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_gs_topology_mismatch_dxbc() {

}

/// test_gs_topology_mismatch_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_gs_topology_mismatch_dxil() {

}

/// instance_index_is_aabb - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn instance_index_is_aabb(index: u32) -> usize {
    0
}

/// rt_pso_factory_add_subobject_to_exports_association - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_add_subobject_to_exports_association(factory: *mut core::ffi::c_void, subobject_index: u32, num_exports: u32, exports: *mut *const u16) -> u32 {
    0
}

/// rt_pso_factory_add_dxil_subobject_to_exports_association - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_add_dxil_subobject_to_exports_association(factory: *mut core::ffi::c_void, object: *const u16, num_exports: u32, exports: *mut *const u16) -> u32 {
    0
}

/// test_raytracing_missing_required_objects - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_missing_required_objects() {

}

/// test_srgb_unorm_mismatch_usage_aliasing - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_srgb_unorm_mismatch_usage_aliasing() {

}

/// increasing - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn increasing(u: usize) -> usize {
    0
}

/// test_aliasing_barrier_edge_cases - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_aliasing_barrier_edge_cases() {

}

/// test_missing_bindings_root_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_missing_bindings_root_signature() {

}

/// ps_main_single - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ps_main_single(vout: usize) {

}

/// test_cs_constant_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_cs_constant_buffer() {

}

/// test_constant_buffer_relative_addressing - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_constant_buffer_relative_addressing() {

}

/// test_immediate_constant_buffer - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_immediate_constant_buffer(use_dxil: usize) {

}

/// test_immediate_constant_buffer_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_immediate_constant_buffer_dxbc() {

}

/// test_immediate_constant_buffer_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_immediate_constant_buffer_dxil() {

}

/// test_root_constants - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_root_constants() {

}

/// test_resinfo - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_resinfo() {

}

/// test_instance_id - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_instance_id(use_dxil: usize) {

}

/// test_instance_id_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_instance_id_dxbc() {

}

/// test_instance_id_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_instance_id_dxil() {

}

/// test_constant_buffers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_constant_buffers(use_dxil: usize) {

}

/// test_constant_buffer_sm51 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_constant_buffer_sm51() {

}

/// test_constant_buffer_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_constant_buffer_dxil() {

}

/// test_root_constant_indexing - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_root_constant_indexing(use_dxil: usize) {

}

/// test_root_constant_indexing_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_root_constant_indexing_dxbc() {

}

/// test_root_constant_indexing_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_root_constant_indexing_dxil() {

}

/// test_vs_instance_input_nonuniform_workarounds - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_vs_instance_input_nonuniform_workarounds() {

}

/// test_primitive_restart_list_topology_stream_output - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_primitive_restart_list_topology_stream_output() {

}

/// test_quad_tessellation_wrong_pso_topology_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_quad_tessellation_wrong_pso_topology_dxbc() {

}

/// test_quad_tessellation_wrong_pso_topology_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_quad_tessellation_wrong_pso_topology_dxil() {

}

/// test_hull_shader_vertex_input_patch_constant_phase - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_hull_shader_vertex_input_patch_constant_phase() {

}

/// export_strequal - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn export_strequal(a: *const u16, b: *const u16) -> usize {
    0
}

/// basic_expected - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn basic_expected(desc: *mut core::ffi::c_void, value_index: u32) -> u32 {
    0
}

/// broadcast_input_expected - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn broadcast_input_expected(desc: *mut core::ffi::c_void, value_index: u32) -> u32 {
    0
}

/// broadcast_input_uint2_expected - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn broadcast_input_uint2_expected(desc: *mut core::ffi::c_void, value_index: u32) -> u32 {
    0
}

/// broadcast_input_uint16x2_expected - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn broadcast_input_uint16x2_expected(desc: *mut core::ffi::c_void, value_index: u32) -> u32 {
    0
}

/// broadcast_input_uint_expected - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn broadcast_input_uint_expected(desc: *mut core::ffi::c_void, value_index: u32) -> u32 {
    0
}

/// coalesced_input_expected - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn coalesced_input_expected(desc: *mut core::ffi::c_void, value_index: u32) -> u32 {
    0
}

/// copy_descriptor_heap_single - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn copy_descriptor_heap_single(device: *mut core::ffi::c_void, gpu_heap: *mut core::ffi::c_void, cpu_heap: *mut core::ffi::c_void, count: u32) {

}

/// log_process_memory - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn log_process_memory() {

}

/// test_create_instance - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_create_instance() {

}

/// check_instance_extensions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_instance_extensions(enabled_extensions: *mut *mut i8, extensions: *mut core::ffi::c_void, extension_count: u32) -> u32 {
    0
}

/// fake_vkGetInstanceProcAddr - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn fake_vkGetInstanceProcAddr(instance: usize, name: *mut i8) -> usize {
    0
}

/// vkGetInstanceProcAddr - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkGetInstanceProcAddr(arg0: usize, arg1: usize) -> usize {
    0
}

/// IVKD3DCoreInterface_EnableExperimentalFeatures - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn IVKD3DCoreInterface_EnableExperimentalFeatures(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// vkd3d_create_instance_global - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_instance_global(out_instance: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_debug_control_SetExplodeOnValidationError - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_control_SetExplodeOnValidationError(iface: *mut core::ffi::c_void, enable: i32) -> usize {
    0
}

/// d3d12_device_factory_EnableExperimentalFeatures - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_EnableExperimentalFeatures(iface: *mut core::ffi::c_void, NumFeatures: u32, pIIDs: *mut core::ffi::c_void, pConfigurationStructs: *mut core::ffi::c_void, pConfigurationStructSizes: *mut u32) -> usize {
    0
}

/// d3d12core_EnableExperimentalFeatures - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12core_EnableExperimentalFeatures(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// d3d12_device_configuration_GetEnabledExperimentalFeatures - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_configuration_GetEnabledExperimentalFeatures(iface: *mut core::ffi::c_void, pGuids: *mut core::ffi::c_void, NumGuids: u32) -> usize {
    0
}

/// d3d12_bundle_exec_draw_instanced - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_draw_instanced(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_DrawInstanced - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_DrawInstanced(iface: *mut core::ffi::c_void, vertex_count_per_instance: u32, instance_count: u32, start_vertex_location: u32, start_instance_location: u32) -> usize {
    0
}

/// d3d12_bundle_exec_draw_indexed_instanced - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_draw_indexed_instanced(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_DrawIndexedInstanced - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_DrawIndexedInstanced(iface: *mut core::ffi::c_void, index_count_per_instance: u32, instance_count: u32, start_vertex_location: u32, base_vertex_location: i32, start_instance_location: u32) -> usize {
    0
}

/// d3d12_bundle_exec_ia_set_primitive_topology - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_ia_set_primitive_topology(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_IASetPrimitiveTopology - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_IASetPrimitiveTopology(iface: *mut core::ffi::c_void, topology: usize) -> usize {
    0
}

/// d3d12_bundle_exec_set_compute_root_32bit_constant - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_compute_root_32bit_constant(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetComputeRoot32BitConstant - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetComputeRoot32BitConstant(iface: *mut core::ffi::c_void, root_parameter_index: u32, data: u32, dst_offset: u32) -> usize {
    0
}

/// d3d12_bundle_exec_set_graphics_root_32bit_constant - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_graphics_root_32bit_constant(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetGraphicsRoot32BitConstant - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetGraphicsRoot32BitConstant(iface: *mut core::ffi::c_void, root_parameter_index: u32, data: u32, dst_offset: u32) -> usize {
    0
}

/// d3d12_bundle_exec_set_compute_root_32bit_constants - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_compute_root_32bit_constants(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetComputeRoot32BitConstants - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetComputeRoot32BitConstants(iface: *mut core::ffi::c_void, root_parameter_index: u32, constant_count: u32, data: *mut core::ffi::c_void, dst_offset: u32) -> usize {
    0
}

/// d3d12_bundle_exec_set_graphics_root_32bit_constants - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_graphics_root_32bit_constants(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetGraphicsRoot32BitConstants - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetGraphicsRoot32BitConstants(iface: *mut core::ffi::c_void, root_parameter_index: u32, constant_count: u32, data: *mut core::ffi::c_void, dst_offset: u32) -> usize {
    0
}

/// d3d12_bundle_SetComputeRootConstantBufferView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetComputeRootConstantBufferView(iface: *mut core::ffi::c_void, root_parameter_index: u32, address: usize) -> usize {
    0
}

/// d3d12_bundle_SetGraphicsRootConstantBufferView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetGraphicsRootConstantBufferView(iface: *mut core::ffi::c_void, root_parameter_index: u32, address: usize) -> usize {
    0
}

/// d3d12_bundle_exec_set_view_instance_mask - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_view_instance_mask(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_SetViewInstanceMask - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetViewInstanceMask(iface: *mut core::ffi::c_void, mask: u32) -> usize {
    0
}

/// d3d12_bundle_OMSetFrontAndBackStencilRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_OMSetFrontAndBackStencilRef(iface: *mut core::ffi::c_void, FrontStencilRef: u32, BackStencilRef: u32) -> usize {
    0
}

/// d3d12_fence_set_native_sync_handle_on_completion_explicit - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_set_native_sync_handle_on_completion_explicit(fence: *mut core::ffi::c_void, wait_type: usize, value: usize, handle: usize, payload: *mut u32) -> i32 {
    0
}

/// d3d12_shared_fence_set_native_sync_handle_on_completion_explicit - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_set_native_sync_handle_on_completion_explicit(fence: *mut core::ffi::c_void, wait_type: usize, value: u64, handle: usize, payload: *mut u32) -> i32 {
    0
}

/// d3d12_command_list_update_root_constants - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_root_constants(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void, layout: usize, push_stages: usize) {

}

/// d3d12_command_list_DrawInstanced - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DrawInstanced(iface: *mut core::ffi::c_void, vertex_count_per_instance: u32, instance_count: u32, start_vertex_location: u32, start_instance_location: u32) -> usize {
    0
}

/// d3d12_command_list_DrawIndexedInstanced - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DrawIndexedInstanced(iface: *mut core::ffi::c_void, index_count_per_instance: u32, instance_count: u32, start_vertex_location: u32, base_vertex_location: i32, start_instance_location: u32) -> usize {
    0
}

/// d3d12_command_list_IASetPrimitiveTopology - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetPrimitiveTopology(iface: *mut core::ffi::c_void, topology: usize) -> usize {
    0
}

/// d3d12_command_list_set_root_constants - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_root_constants(list: *mut core::ffi::c_void, bindings: *mut core::ffi::c_void, index: u32, offset: u32, count: u32, data: *mut core::ffi::c_void) {

}

/// d3d12_command_list_SetComputeRoot32BitConstant - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRoot32BitConstant(iface: *mut core::ffi::c_void, root_parameter_index: u32, data: u32, dst_offset: u32) -> usize {
    0
}

/// d3d12_command_list_SetGraphicsRoot32BitConstant - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRoot32BitConstant(iface: *mut core::ffi::c_void, root_parameter_index: u32, data: u32, dst_offset: u32) -> usize {
    0
}

/// d3d12_command_list_SetComputeRoot32BitConstants - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRoot32BitConstants(iface: *mut core::ffi::c_void, root_parameter_index: u32, constant_count: u32, data: *mut core::ffi::c_void, dst_offset: u32) -> usize {
    0
}

/// d3d12_command_list_SetGraphicsRoot32BitConstants - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRoot32BitConstants(iface: *mut core::ffi::c_void, root_parameter_index: u32, constant_count: u32, data: *mut core::ffi::c_void, dst_offset: u32) -> usize {
    0
}

/// d3d12_command_list_SetComputeRootConstantBufferView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootConstantBufferView(iface: *mut core::ffi::c_void, root_parameter_index: u32, address: usize) -> usize {
    0
}

/// d3d12_command_list_SetGraphicsRootConstantBufferView - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootConstantBufferView(iface: *mut core::ffi::c_void, root_parameter_index: u32, address: usize) -> usize {
    0
}

/// clamp_float_to_sint32 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn clamp_float_to_sint32(value: f32, min_value: i32, max_value: i32) -> i32 {
    0
}

/// d3d12_command_list_SetViewInstanceMask - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetViewInstanceMask(iface: *mut core::ffi::c_void, mask: u32) -> usize {
    0
}

/// d3d12_command_list_OMSetFrontAndBackStencilRef - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetFrontAndBackStencilRef(iface: *mut core::ffi::c_void, FrontStencilRef: u32, BackStencilRef: u32) -> usize {
    0
}

/// vkd3d_init_instance_caps - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_instance_caps(instance: *mut core::ffi::c_void, create_info: *mut core::ffi::c_void, instance_extension_count: *mut u32, user_extension_supported: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_debug_control_explode_on_vvl_error - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_control_explode_on_vvl_error() -> usize {
    0
}

/// vkd3d_instance_apply_application_workarounds - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_instance_apply_application_workarounds() {

}

/// vkd3d_instance_deduce_config_flags_from_environment - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_instance_deduce_config_flags_from_environment() {

}

/// vkd3d_instance_apply_global_shader_quirks - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_instance_apply_global_shader_quirks() {

}

/// vkd3d_instance_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_instance_init(instance: *mut core::ffi::c_void, create_info: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_destroy_instance - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_destroy_instance(instance: *mut core::ffi::c_void) {

}

/// d3d12_add_device_singleton - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_add_device_singleton(device: *mut core::ffi::c_void, luid: usize) {

}

/// d3d12_remove_device_singleton - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_remove_device_singleton(luid: usize) {

}

/// d3d12_device_CreateConstantBufferView_embedded - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateConstantBufferView_embedded(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_CreateConstantBufferView_default - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateConstantBufferView_default(iface: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, descriptor: usize) -> usize {
    0
}

/// d3d12_device_SetStablePowerState - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_SetStablePowerState(iface: *mut core::ffi::c_void, enable: i32) -> usize {
    0
}

/// d3d12_dxvk_interop_device_GetInstanceExtensions - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_GetInstanceExtensions(iface: *mut core::ffi::c_void, extension_count: *mut u32, extensions: *mut *mut i8) -> usize {
    0
}

/// vkd3d_queue_timeline_trace_flush_instantaneous - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_flush_instantaneous(trace: *mut core::ffi::c_void, worker: *mut core::ffi::c_void) {

}

/// d3d12_state_object_get_export_index - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_get_export_index(object: *mut core::ffi::c_void, export_name: *mut u16, out_subtype: *mut *mut u16) -> u32 {
    0
}

/// d3d12_device_requires_explicit_sparse_init - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_requires_explicit_sparse_init(device: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_memory_info_get_topology - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_info_get_topology(topology: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_memory_topology_is_uma_like - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_topology_is_uma_like(topology: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_root_signature_init_shader_record_constants - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_shader_record_constants(root_signature: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, info: *mut core::ffi::c_void) -> i32 {
    0
}

/// d3d12_root_signature_init_push_constants - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_push_constants(root_signature: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, info: *mut core::ffi::c_void, push_constant_range: *mut core::ffi::c_void) -> i32 {
    0
}

/// vk_logic_op_from_d3d12 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_logic_op_from_d3d12(op: usize) -> usize {
    0
}

/// d3d12_pipeline_state_validate_gs_input_toplogy - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_validate_gs_input_toplogy(state: *mut core::ffi::c_void, gs_meta: *mut core::ffi::c_void, geometry_meta: u32) -> usize {
    0
}

/// d3d12_pipeline_state_validate_view_instancing - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_validate_view_instancing(device: *mut core::ffi::c_void, graphics: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_state_log_graphics_state - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_log_graphics_state(state: *mut core::ffi::c_void) {

}

/// vkd3d_bindless_find_copy_template_single - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bindless_find_copy_template_single(descriptor_size: u32) -> usize {
    0
}

/// d3d12_state_object_find_explicit_assignment_override - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_find_explicit_assignment_override(kind: usize, associations: *mut core::ffi::c_void, associations_count: usize, association: *mut core::ffi::c_void) -> usize {
    0
}

/// dxgi_vk_swap_chain_poll_single_calibration - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_poll_single_calibration(chain: *mut core::ffi::c_void, time_domain: usize, time_domain_id: u64, calibration: *mut u64) -> usize {
    0
}

/// d3d12_wg_state_object_find_exported_entry_point - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_find_exported_entry_point(data: *mut core::ffi::c_void, shader: *const u16) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// d3d12_wg_state_object_resolve_entry_points_explicit - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_resolve_entry_points_explicit(object: *mut core::ffi::c_void, data: *mut core::ffi::c_void, program: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_parse_root_constants - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_parse_root_constants(context: *mut core::ffi::c_void, offset: u32, constants: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_dxil_log_callback - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_dxil_log_callback(userdata: *mut core::ffi::c_void, level: usize, msg: *mut i8) {

}

/// GetPow2DownscaleFactor - from DirectX-Headers/d3d12video.h
#[no_mangle]
pub unsafe extern "C" fn GetPow2DownscaleFactor() -> usize {
    0
}

/// CreateExplorer - from DirectX-Headers/d3dshadercacheregistration.h
#[no_mangle]
pub unsafe extern "C" fn CreateExplorer(pUnknown: *mut core::ffi::c_void, riid: usize, ppvExplorer: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// Aliasing - from DirectX-Headers/d3dx12_barriers.h
#[no_mangle]
pub unsafe extern "C" fn Aliasing(pResourceBefore: *mut core::ffi::c_void, pResourceAfter: *mut core::ffi::c_void) -> usize {
    0
}

/// OutputMergerLogicOp - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn OutputMergerLogicOp() -> i32 {
    0
}

/// StandardSwizzle64KBSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn StandardSwizzle64KBSupported() -> i32 {
    0
}

/// ExpandedComputeResourceStates - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ExpandedComputeResourceStates() -> i32 {
    0
}

/// ViewInstancingTier - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ViewInstancingTier() -> usize {
    0
}

/// BackgroundProcessingSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn BackgroundProcessingSupported() -> i32 {
    0
}

/// DerivativesInMeshAndAmplificationShadersSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn DerivativesInMeshAndAmplificationShadersSupported() -> i32 {
    0
}

/// IndependentFrontAndBackStencilRefMaskSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn IndependentFrontAndBackStencilRefMaskSupported() -> i32 {
    0
}

/// D3DX12ConditionallyExpandAPIDesc - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn D3DX12ConditionallyExpandAPIDesc(LclDesc: usize, pDesc: *mut core::ffi::c_void, arg2: usize, arg3: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CD3DX12_VIEW_INSTANCING_DESC - from DirectX-Headers/d3dx12_core.h
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_VIEW_INSTANCING_DESC(arg0: usize) -> usize {
    0
}

/// PrimitiveTopologyTypeCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn PrimitiveTopologyTypeCb(PrimitiveTopologyType: usize) {

}

/// ViewInstancingCb - from DirectX-Headers/d3dx12_pipeline_state_stream.h
#[no_mangle]
pub unsafe extern "C" fn ViewInstancingCb(ViewInstancingDesc: usize) {

}

/// FormatExistsInHeader - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn FormatExistsInHeader(Format: usize, arg1: usize) -> usize {
    0
}

/// GetNumComponentsInFormat - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetNumComponentsInFormat(Format: usize) -> u32 {
    0
}

/// GetMinNumComponentsInFormats - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetMinNumComponentsInFormats(FormatA: usize, FormatB: usize) -> u32 {
    0
}

/// GetAddressingBitsPerAlignedSize - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn GetAddressingBitsPerAlignedSize(Format: usize) -> usize {
    0
}

/// FloatAndNotFloatFormats - from DirectX-Headers/d3dx12_property_format_table.h
#[no_mangle]
pub unsafe extern "C" fn FloatAndNotFloatFormats(FormatA: usize, FormatB: usize) -> usize {
    0
}

/// InitAsConstants - from DirectX-Headers/d3dx12_root_signature.h
#[no_mangle]
pub unsafe extern "C" fn InitAsConstants(rootParam: usize, num32BitValues: u32, shaderRegister: u32, arg3: usize, D3D12_SHADER_VISIBILITY_ALL: usize) {

}

/// InitAsConstantBufferView - from DirectX-Headers/d3dx12_root_signature.h
#[no_mangle]
pub unsafe extern "C" fn InitAsConstantBufferView(rootParam: usize, shaderRegister: u32, arg2: usize, D3D12_SHADER_VISIBILITY_ALL: usize) {

}

/// DefineExport - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn DefineExport(Name: *const u16, nullptr: usize, D3D12_EXPORT_FLAG_NONE: usize) {

}

/// DefineExports - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn DefineExports(arg0: usize) {

}

/// AddExport - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn AddExport(Export: *const u16) {

}

/// AddExports - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn AddExports(Exports: *const u16) {

}

/// SetHitGroupExport - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetHitGroupExport(exportName: *const u16) {

}

/// SetAnyHitShaderImport - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetAnyHitShaderImport(importName: *const u16) {

}

/// SetPrimitiveTopologyType - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetPrimitiveTopologyType(primitiveTopologytype: usize) {

}

/// AddViewInstanceLocation - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn AddViewInstanceLocation(viewInstanceLocation: usize) {

}

/// RuntimeClassInitialize - from DirectX-Headers/wrladapter.h
#[no_mangle]
pub unsafe extern "C" fn RuntimeClassInitialize() -> i32 {
    0
}

