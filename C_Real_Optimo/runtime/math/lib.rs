//! ADead Runtime - MATH Module
//!
//! Funciones generadas automáticamente desde knowledge.json
//! Total: 1319 funciones

use std::ffi::c_void;

#[no_mangle]
pub unsafe extern "C" fn GetExplicitEntriesFromAclA(param_23304: PACL, param_56625: PULONG, param_7039: *mut PEXPLICIT_ACCESS_A) -> WINADVAPI DWORD {
    // TODO: implementar GetExplicitEntriesFromAclA desde wine/aclapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetExplicitEntriesFromAclW(param_23304: PACL, param_56625: PULONG, param_53544: *mut PEXPLICIT_ACCESS_W) -> WINADVAPI DWORD {
    // TODO: implementar GetExplicitEntriesFromAclW desde wine/aclapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetEntriesInAclA(param_30140: ULONG, param_44454: PEXPLICIT_ACCESSA, param_23304: PACL, param_38267: *mut PACL) -> WINADVAPI DWORD {
    // TODO: implementar SetEntriesInAclA desde wine/aclapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetEntriesInAclW(param_30140: ULONG, param_45046: PEXPLICIT_ACCESSW, param_23304: PACL, param_38267: *mut PACL) -> WINADVAPI DWORD {
    // TODO: implementar SetEntriesInAclW desde wine/aclapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BuildExplicitAccessWithNameA(param_44454: PEXPLICIT_ACCESSA, param_32262: LPSTR, param_54075: u32, param_25689: ACCESS_MODE, param_54075: u32) -> WINADVAPI void {
    // TODO: implementar BuildExplicitAccessWithNameA desde wine/aclapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BuildExplicitAccessWithNameW(param_45046: PEXPLICIT_ACCESSW, param_46598: LPWSTR, param_54075: u32, param_25689: ACCESS_MODE, param_54075: u32) -> WINADVAPI void {
    // TODO: implementar BuildExplicitAccessWithNameW desde wine/aclapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlModuleRegisterWndClassInfoA(pm: *mut _ATL_MODULEA, wci: *mut _ATL_WNDCLASSINFOA, pProc: *mut WNDPROC) -> ATOM {
    // TODO: implementar AtlModuleRegisterWndClassInfoA desde wine/atlwin.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlModuleRegisterWndClassInfoW(pm: *mut _ATL_MODULEW, wci: *mut _ATL_WNDCLASSINFOW, pProc: *mut WNDPROC) -> ATOM {
    // TODO: implementar AtlModuleRegisterWndClassInfoW desde wine/atlwin.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BCryptExportKey(param_34990: BCRYPT_KEY_HANDLE, param_34990: BCRYPT_KEY_HANDLE, param_25711: LPCWSTR, param_12278: PUCHAR, param_30140: ULONG, param_1537: *mut ULONG, param_30140: ULONG) -> NTSTATUS {
    // TODO: implementar BCryptExportKey desde wine/bcrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CM_Add_Empty_Log_Conf(conf: *mut LOG_CONF, node: DEVINST, priority: PRIORITY, flags: ULONG) -> CMAPI CONFIGRET {
    // TODO: implementar CM_Add_Empty_Log_Conf desde wine/cfgmgr32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CM_Add_Empty_Log_Conf_Ex(conf: *mut LOG_CONF, node: DEVINST, priority: PRIORITY, flags: ULONG, machine: HMACHINE) -> CMAPI CONFIGRET {
    // TODO: implementar CM_Add_Empty_Log_Conf_Ex desde wine/cfgmgr32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CM_Get_First_Log_Conf(conf: *mut LOG_CONF, node: DEVINST, flags: ULONG) -> CMAPI CONFIGRET {
    // TODO: implementar CM_Get_First_Log_Conf desde wine/cfgmgr32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CM_Get_First_Log_Conf_Ex(conf: *mut LOG_CONF, node: DEVINST, flags: ULONG, machine: HMACHINE) -> CMAPI CONFIGRET {
    // TODO: implementar CM_Get_First_Log_Conf_Ex desde wine/cfgmgr32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CM_Get_Log_Conf_Priority(conf: LOG_CONF, priority: *mut PRIORITY, flags: ULONG) -> CMAPI CONFIGRET {
    // TODO: implementar CM_Get_Log_Conf_Priority desde wine/cfgmgr32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CM_Get_Log_Conf_Priority_Ex(conf: LOG_CONF, priority: *mut PRIORITY, flags: ULONG, machine: HMACHINE) -> CMAPI CONFIGRET {
    // TODO: implementar CM_Get_Log_Conf_Priority_Ex desde wine/cfgmgr32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CM_Get_Next_Log_Conf(next: *mut LOG_CONF, conf: LOG_CONF, flags: ULONG) -> CMAPI CONFIGRET {
    // TODO: implementar CM_Get_Next_Log_Conf desde wine/cfgmgr32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CM_Get_Next_Log_Conf_Ex(next: *mut LOG_CONF, conf: LOG_CONF, flags: ULONG, machine: HMACHINE) -> CMAPI CONFIGRET {
    // TODO: implementar CM_Get_Next_Log_Conf_Ex desde wine/cfgmgr32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TaskDialog(owner: *mut core::ffi::c_void, hinst: HINSTANCE, title: *mut const WCHAR, main_instruction: *mut const WCHAR, content: *mut const WCHAR, common_buttons: TASKDIALOG_COMMON_BUTTON_FLAGS, icon: *mut const WCHAR, button: *mut i32) -> WINCOMMCTRLAPI HRESULT {
    // TODO: implementar TaskDialog desde wine/commctrl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TaskDialogIndirect(param_27657: *mut const TASKDIALOGCONFIG, param_7496: *mut i32, param_7496: *mut i32, param_56618: *mut i32) -> WINCOMMCTRLAPI HRESULT {
    // TODO: implementar TaskDialogIndirect desde wine/commctrl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptUIWizExport(dwFlags: u32, hwndParent: *mut core::ffi::c_void, pwszWizardTitle: LPCWSTR, pExportInfo: PCCRYPTUI_WIZ_EXPORT_INFO, pvoid: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar CryptUIWizExport desde wine/cryptuiapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DGetInputAndOutputSignatureBlob(data: *mut const void, data_size: SIZE_T, param_52875: *mut ID3DBlob) -> i32 {
    // TODO: implementar D3DGetInputAndOutputSignatureBlob desde wine/d3dcompiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXQuaternionExp(pout: *mut D3DXQUATERNION, pq: *mut const D3DXQUATERNION) -> *mut D3DXQUATERNION {
    // TODO: implementar D3DXQuaternionExp desde wine/d3dx9math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXComputeTangentFrameEx(mesh_in: *mut ID3DXMesh, texture_in_semantic: u32, texture_in_idx: u32, u_partial_out_semantic: u32, u_partial_out_idx: u32, v_partial_out_semantic: u32, v_partial_out_idx: u32, normal_out_semantic: u32, normal_out_idx: u32, flags: u32, adjacency: *mut const DWORD, partial_edge_threshold: f32, singular_point_threshold: f32, normal_edge_threshold: f32, param_27302: *mut ID3DXMesh, param_29258: *mut ID3DXBuffer) -> i32 {
    // TODO: implementar D3DXComputeTangentFrameEx desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXComputeTangent(mesh: *mut ID3DXMesh, stage: u32, tangent_idx: u32, binorm_idx: u32, wrap: u32, adjacency: *mut const DWORD) -> i32 {
    // TODO: implementar D3DXComputeTangent desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXConvertMeshSubsetToSingleStrip(mesh_in: *mut struct ID3DXBaseMesh, attribute_id: u32, ib_flags: u32, param_17231: *mut struct IDirect3DIndexBuffer9, index_count: *mut u32) -> i32 {
    // TODO: implementar D3DXConvertMeshSubsetToSingleStrip desde wine/d3dx9mesh.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXGetShaderConstantTableEx(byte_code: *mut const DWORD, flags: u32, param_32738: *mut ID3DXConstantTable) -> i32 {
    // TODO: implementar D3DXGetShaderConstantTableEx desde wine/d3dx9shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DXGetShaderConstantTable(byte_code: *mut const DWORD, param_32738: *mut ID3DXConstantTable) -> i32 {
    // TODO: implementar D3DXGetShaderConstantTable desde wine/d3dx9shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIFXAPISetLogCallbackA(param_34421: DIFXAPILOG_A, param_9082: *mut VOID) -> VOID {
    // TODO: implementar DIFXAPISetLogCallbackA desde wine/difxapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIFXAPISetLogCallbackW(param_32646: DIFXAPILOG_W, param_9082: *mut VOID) -> VOID {
    // TODO: implementar DIFXAPISetLogCallbackW desde wine/difxapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetDifxLogCallbackA(param_17578: DIFXLOG_A, param_9082: *mut VOID) -> VOID {
    // TODO: implementar SetDifxLogCallbackA desde wine/difxapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetDifxLogCallbackW(param_46053: DIFXLOG_W, param_9082: *mut VOID) -> VOID {
    // TODO: implementar SetDifxLogCallbackW desde wine/difxapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetTraceLoggerHandle(param_10593: PVOID) -> TRACEHANDLE WMIAPI {
    // TODO: implementar GetTraceLoggerHandle desde wine/evntrace.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PfMakeLog(hEvent: *mut core::ffi::c_void) -> PF {
    // TODO: implementar PfMakeLog desde wine/fltdefs.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PfSetLogBuffer(pbBuffer: PBYTE, dwSize: u32, dwThreshold: u32, dwEntries: u32, pdwLoggedEntries: PDWORD, pdwLostEntries: PDWORD, pdwSizeUsed: PDWORD) -> PF {
    // TODO: implementar PfSetLogBuffer desde wine/fltdefs.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PfDeleteLog(param_9080: VOID) -> PF {
    // TODO: implementar PfDeleteLog desde wine/fltdefs.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFontFromLogfontA(param_216: HDC, param_58558: GDIPCONST, param_3631: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateFontFromLogfontA desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipCreateFontFromLogfontW(param_216: HDC, param_58558: GDIPCONST, param_3631: *mut core::ffi::c_void) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipCreateFontFromLogfontW desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipGetLogFontA(param_2952: *mut GpFont, param_59643: *mut GpGraphics, param_24570: *mut LOGFONTA) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipGetLogFontA desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipGetLogFontW(param_2952: *mut GpFont, param_59643: *mut GpGraphics, param_19298: *mut LOGFONTW) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipGetLogFontW desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipDrawRectangle(param_59643: *mut GpGraphics, param_28992: *mut GpPen, param_35563: REAL, param_35563: REAL, param_35563: REAL, param_35563: REAL) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipDrawRectangle desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipDrawRectangleI(param_59643: *mut GpGraphics, param_28992: *mut GpPen, param_18538: INT, param_18538: INT, param_18538: INT, param_18538: INT) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipDrawRectangleI desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipDrawRectangles(param_59643: *mut GpGraphics, param_28992: *mut GpPen, param_58558: GDIPCONST, param_18538: INT) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipDrawRectangles desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipDrawRectanglesI(param_59643: *mut GpGraphics, param_28992: *mut GpPen, param_58558: GDIPCONST, param_18538: INT) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipDrawRectanglesI desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipFillRectangle(param_59643: *mut GpGraphics, param_25650: *mut GpBrush, param_35563: REAL, param_35563: REAL, param_35563: REAL, param_35563: REAL) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipFillRectangle desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipFillRectangleI(param_59643: *mut GpGraphics, param_25650: *mut GpBrush, param_18538: INT, param_18538: INT, param_18538: INT, param_18538: INT) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipFillRectangleI desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipFillRectangles(param_59643: *mut GpGraphics, param_25650: *mut GpBrush, param_58558: GDIPCONST, param_18538: INT) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipFillRectangles desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipFillRectanglesI(param_59643: *mut GpGraphics, param_25650: *mut GpBrush, param_58558: GDIPCONST, param_18538: INT) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipFillRectanglesI desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipAddPathRectangle(param_30906: *mut GpPath, param_35563: REAL, param_35563: REAL, param_35563: REAL, param_35563: REAL) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipAddPathRectangle desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipAddPathRectangleI(param_30906: *mut GpPath, param_18538: INT, param_18538: INT, param_18538: INT, param_18538: INT) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipAddPathRectangleI desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipAddPathRectangles(param_30906: *mut GpPath, param_58558: GDIPCONST, param_18538: INT) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipAddPathRectangles desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipAddPathRectanglesI(param_30906: *mut GpPath, param_58558: GDIPCONST, param_18538: INT) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipAddPathRectanglesI desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GdipIsInfiniteRegion(param_38003: *mut GpRegion, param_59643: *mut GpGraphics, param_56618: *mut i32) -> GpStatus WINGDIPAPI {
    // TODO: implementar GdipIsInfiniteRegion desde wine/gdiplusflat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsIndexedPixelFormat(format: PixelFormat) -> static inline BOOL {
    // TODO: implementar IsIndexedPixelFormat desde wine/gdipluspixelformats.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateProfileFromLogColorSpaceA(param_7377: LPLOGCOLORSPACEA, param_54983: *mut PBYTE) -> i32 {
    // TODO: implementar CreateProfileFromLogColorSpaceA desde wine/icm.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateProfileFromLogColorSpaceW(param_49762: LPLOGCOLORSPACEW, param_54983: *mut PBYTE) -> i32 {
    // TODO: implementar CreateProfileFromLogColorSpaceW desde wine/icm.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetStandardColorSpaceProfileA(param_29633: PCSTR, param_54075: u32, param_29209: PSTR, param_64752: PDWORD) -> i32 {
    // TODO: implementar GetStandardColorSpaceProfileA desde wine/icm.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetStandardColorSpaceProfileW(param_29658: PCWSTR, param_54075: u32, param_31966: PWSTR, param_64752: PDWORD) -> i32 {
    // TODO: implementar GetStandardColorSpaceProfileW desde wine/icm.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetStandardColorSpaceProfileA(param_29633: PCSTR, param_54075: u32, param_29209: PSTR) -> i32 {
    // TODO: implementar SetStandardColorSpaceProfileA desde wine/icm.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetStandardColorSpaceProfileW(param_29658: PCWSTR, param_54075: u32, param_31966: PWSTR) -> i32 {
    // TODO: implementar SetStandardColorSpaceProfileW desde wine/icm.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ubidi_getLogicalIndex(pBiDi: *mut UBiDi, visualIndex: i32, pErrorCode: *mut UErrorCode) -> i32 {
    // TODO: implementar ubidi_getLogicalIndex desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ubidi_getLogicalMap(pBiDi: *mut UBiDi, indexMap: *mut i32, pErrorCode: *mut UErrorCode) -> core::ffi::c_void {
    // TODO: implementar ubidi_getLogicalMap desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ubidi_getLogicalRun(pBiDi: *mut const UBiDi, logicalPosition: i32, pLogicalLimit: *mut i32, pLevel: *mut UBiDiLevel) -> core::ffi::c_void {
    // TODO: implementar ubidi_getLogicalRun desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ubidi_isInverse(pBiDi: *mut UBiDi) -> UBool {
    // TODO: implementar ubidi_isInverse desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ubidi_reorderLogical(levels: *mut const UBiDiLevel, length: i32, indexMap: *mut i32) -> core::ffi::c_void {
    // TODO: implementar ubidi_reorderLogical desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ucnv_countStandards() -> u16 {
    // TODO: implementar ucnv_countStandards desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ucnv_getStandard(n: u16, pErrorCode: *mut UErrorCode) -> *mut const char {
    // TODO: implementar ucnv_getStandard desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ucnv_getStandardName(name: *mut const char, standard: *mut const char, pErrorCode: *mut UErrorCode) -> *mut const char {
    // TODO: implementar ucnv_getStandardName desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ucnv_openStandardNames(convName: *mut const char, standard: *mut const char, pErrorCode: *mut UErrorCode) -> *mut UEnumeration {
    // TODO: implementar ucnv_openStandardNames desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ucol_getContractionsAndExpansions(coll: *mut const UCollator, contractions: *mut USet, expansions: *mut USet, addPrefixes: UBool, status: *mut UErrorCode) -> core::ffi::c_void {
    // TODO: implementar ucol_getContractionsAndExpansions desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ucol_getMaxExpansion(elems: *mut const UCollationElements, order: i32) -> i32 {
    // TODO: implementar ucol_getMaxExpansion desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ucsdet_isInputFilterEnabled(ucsd: *mut const UCharsetDetector) -> UBool {
    // TODO: implementar ucsdet_isInputFilterEnabled desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ugender_getInstance(locale: *mut const char, status: *mut UErrorCode) -> *mut const UGenderInfo {
    // TODO: implementar ugender_getInstance desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unorm2_getInstance(packageName: *mut const char, name: *mut const char, mode: UNormalization2Mode, pErrorCode: *mut UErrorCode) -> *mut const UNormalizer2 {
    // TODO: implementar unorm2_getInstance desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unorm2_getNFCInstance(pErrorCode: *mut UErrorCode) -> *mut const UNormalizer2 {
    // TODO: implementar unorm2_getNFCInstance desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unorm2_getNFDInstance(pErrorCode: *mut UErrorCode) -> *mut const UNormalizer2 {
    // TODO: implementar unorm2_getNFDInstance desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unorm2_getNFKCCasefoldInstance(pErrorCode: *mut UErrorCode) -> *mut const UNormalizer2 {
    // TODO: implementar unorm2_getNFKCCasefoldInstance desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unorm2_getNFKCInstance(pErrorCode: *mut UErrorCode) -> *mut const UNormalizer2 {
    // TODO: implementar unorm2_getNFKCInstance desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unorm2_getNFKDInstance(pErrorCode: *mut UErrorCode) -> *mut const UNormalizer2 {
    // TODO: implementar unorm2_getNFKDInstance desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unorm2_isInert(norm2: *mut const UNormalizer2, c: UChar32) -> UBool {
    // TODO: implementar unorm2_isInert desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn utext_isLengthExpensive(ut: *mut const UText) -> UBool {
    // TODO: implementar utext_isLengthExpensive desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn utrans_transIncremental(trans: *mut const UTransliterator, rep: *mut UReplaceable, repFunc: *mut const UReplaceableCallbacks, pos: *mut UTransPosition, status: *mut UErrorCode) -> core::ffi::c_void {
    // TODO: implementar utrans_transIncremental desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn utrans_transIncrementalUChars(trans: *mut const UTransliterator, text: *mut UChar, textLength: *mut i32, textCapacity: i32, pos: *mut UTransPosition, status: *mut UErrorCode) -> core::ffi::c_void {
    // TODO: implementar utrans_transIncrementalUChars desde wine/icu.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BufferPointerPacketsInteractionContext(context: HINTERACTIONCONTEXT, entries_count: UINT32, pointer_info: *mut const POINTER_INFO) -> i32 {
    // TODO: implementar BufferPointerPacketsInteractionContext desde wine/interactioncontext.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ProcessBufferedPacketsInteractionContext(context: HINTERACTIONCONTEXT) -> i32 {
    // TODO: implementar ProcessBufferedPacketsInteractionContext desde wine/interactioncontext.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ProcessInertiaInteractionContext(context: HINTERACTIONCONTEXT) -> i32 {
    // TODO: implementar ProcessInertiaInteractionContext desde wine/interactioncontext.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetAdaptersInfo(pAdapterInfo: PIP_ADAPTER_INFO, pOutBufLen: PULONG) -> IPHLPAPI_DLL_LINKAGE DWORD {
    // TODO: implementar GetAdaptersInfo desde wine/iphlpapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetRTTAndHopCount(DestIpAddress: IPAddr, HopCount: PULONG, MaxHops: ULONG, RTT: PULONG) -> IPHLPAPI_DLL_LINKAGE BOOL {
    // TODO: implementar GetRTTAndHopCount desde wine/iphlpapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn I_BrowserSetNetlogonState(ServerName: LPWSTR, DomainName: LPWSTR, EmulatedServerName: LPWSTR, Role: u32) -> NET_API_STATUS {
    // TODO: implementar I_BrowserSetNetlogonState desde wine/lmbrowsr.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetExpandedNameA(param_32262: LPSTR, param_32262: LPSTR) -> WINBASEAPI INT {
    // TODO: implementar GetExpandedNameA desde wine/lzexpand.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetExpandedNameW(param_46598: LPWSTR, param_46598: LPWSTR) -> WINBASEAPI INT {
    // TODO: implementar GetExpandedNameW desde wine/lzexpand.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetInstance(param_55138: LPSPropValue, param_55138: LPSPropValue, param_30140: ULONG) -> VOID {
    // TODO: implementar GetInstance desde wine/mapiutil.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MesIncrementalHandleReset(param_53744: handle_t, param_64866: *mut core::ffi::c_void, param_3533: MIDL_ES_ALLOC, param_19743: MIDL_ES_WRITE, param_30283: MIDL_ES_READ, param_26354: MIDL_ES_CODE) -> RPC_STATUS RPC_ENTRY {
    // TODO: implementar MesIncrementalHandleReset desde wine/midles.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MesInqProcEncodingId(param_53744: handle_t, param_57346: PRPC_SYNTAX_IDENTIFIER, param_1537: *mut ULONG) -> RPC_STATUS RPC_ENTRY {
    // TODO: implementar MesInqProcEncodingId desde wine/midles.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptCATAdminAddCatalog(param_43544: HCATADMIN, param_31966: PWSTR, param_31966: PWSTR, param_54075: u32) -> HCATINFO {
    // TODO: implementar CryptCATAdminAddCatalog desde wine/mscat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptCATAdminEnumCatalogFromHash(param_43544: HCATADMIN, param_60692: *mut BYTE, param_54075: u32, param_54075: u32, param_32630: *mut HCATINFO) -> HCATINFO {
    // TODO: implementar CryptCATAdminEnumCatalogFromHash desde wine/mscat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptCATAdminReleaseCatalogContext(param_43544: HCATADMIN, param_42888: HCATINFO, param_54075: u32) -> i32 {
    // TODO: implementar CryptCATAdminReleaseCatalogContext desde wine/mscat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptCATAdminRemoveCatalog(param_43544: HCATADMIN, param_25711: LPCWSTR, param_54075: u32) -> i32 {
    // TODO: implementar CryptCATAdminRemoveCatalog desde wine/mscat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptCATAdminResolveCatalogPath(param_43544: HCATADMIN, param_34987: *mut u16, param_381: *mut CATALOG_INFO, param_54075: u32) -> i32 {
    // TODO: implementar CryptCATAdminResolveCatalogPath desde wine/mscat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptCATCatalogInfoFromContext(param_42888: HCATINFO, param_381: *mut CATALOG_INFO, param_54075: u32) -> i32 {
    // TODO: implementar CryptCATCatalogInfoFromContext desde wine/mscat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MsiEnumComponentCostsA(param_63707: MSIHANDLE, param_15619: LPCSTR, param_54075: u32, param_39298: INSTALLSTATE, param_32262: LPSTR, param_17803: LPDWORD, param_62129: LPINT, param_62129: LPINT) -> UINT {
    // TODO: implementar MsiEnumComponentCostsA desde wine/msi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MsiEnumComponentCostsW(param_63707: MSIHANDLE, param_25711: LPCWSTR, param_54075: u32, param_39298: INSTALLSTATE, param_46598: LPWSTR, param_17803: LPDWORD, param_62129: LPINT, param_62129: LPINT) -> UINT {
    // TODO: implementar MsiEnumComponentCostsW desde wine/msi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MsiEnableLogA(param_54075: u32, param_15619: LPCSTR, param_54075: u32) -> UINT {
    // TODO: implementar MsiEnableLogA desde wine/msi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MsiEnableLogW(param_54075: u32, param_25711: LPCWSTR, param_54075: u32) -> UINT {
    // TODO: implementar MsiEnableLogW desde wine/msi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MsiInstallMissingComponentA(param_15619: LPCSTR, param_15619: LPCSTR, param_39298: INSTALLSTATE) -> UINT {
    // TODO: implementar MsiInstallMissingComponentA desde wine/msi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MsiInstallMissingComponentW(param_25711: LPCWSTR, param_25711: LPCWSTR, param_39298: INSTALLSTATE) -> UINT {
    // TODO: implementar MsiInstallMissingComponentW desde wine/msi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MsiPreviewDialogA(param_63707: MSIHANDLE, param_15619: LPCSTR) -> UINT {
    // TODO: implementar MsiPreviewDialogA desde wine/msiquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MsiPreviewDialogW(param_63707: MSIHANDLE, param_25711: LPCWSTR) -> UINT {
    // TODO: implementar MsiPreviewDialogW desde wine/msiquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MsiDatabaseExportA(param_63707: MSIHANDLE, param_15619: LPCSTR, param_15619: LPCSTR, param_15619: LPCSTR) -> UINT {
    // TODO: implementar MsiDatabaseExportA desde wine/msiquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MsiDatabaseExportW(param_63707: MSIHANDLE, param_25711: LPCWSTR, param_25711: LPCWSTR, param_25711: LPCWSTR) -> UINT {
    // TODO: implementar MsiDatabaseExportW desde wine/msiquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MsiGetFeatureCostA(param_63707: MSIHANDLE, param_15619: LPCSTR, param_20022: MSICOSTTREE, param_39298: INSTALLSTATE, param_62129: LPINT) -> UINT {
    // TODO: implementar MsiGetFeatureCostA desde wine/msiquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MsiGetFeatureCostW(param_63707: MSIHANDLE, param_25711: LPCWSTR, param_20022: MSICOSTTREE, param_39298: INSTALLSTATE, param_62129: LPINT) -> UINT {
    // TODO: implementar MsiGetFeatureCostW desde wine/msiquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptSIPRetrieveSubjectGuidForCatalogFile(param_25711: LPCWSTR, param_31864: *mut core::ffi::c_void, param_18444: *mut GUID) -> i32 {
    // TODO: implementar CryptSIPRetrieveSubjectGuidForCatalogFile desde wine/mssip.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NCryptExportKey(param_42392: NCRYPT_KEY_HANDLE, param_42392: NCRYPT_KEY_HANDLE, param_51335: *mut const WCHAR, param_59575: *mut NCryptBufferDesc, param_60692: *mut BYTE, param_54075: u32, param_19332: *mut u32, param_54075: u32) -> SECURITY_STATUS {
    // TODO: implementar NCryptExportKey desde wine/ncrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetAnycastIpAddressTable(param_63405: ADDRESS_FAMILY, param_21156: *mut core::ffi::c_void) -> IPHLPAPI_DLL_LINKAGE DWORD {
    // TODO: implementar GetAnycastIpAddressTable desde wine/netioapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiGetAndSetDCDword(hdc: HDC, method: UINT, value: u32, result: *mut u32) -> W32KAPI BOOL {
    // TODO: implementar NtGdiGetAndSetDCDword desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiGetDIBitsInternal(hdc: HDC, hbitmap: HBITMAP, startscan: UINT, lines: UINT, bits: *mut core::ffi::c_void, info: *mut BITMAPINFO, coloruse: UINT, max_bits: UINT, max_info: UINT) -> W32KAPI INT {
    // TODO: implementar NtGdiGetDIBitsInternal desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiGetOutlineTextMetricsInternalW(hdc: HDC, cbData: UINT, otm: *mut OUTLINETEXTMETRICW, opts: ULONG) -> W32KAPI UINT {
    // TODO: implementar NtGdiGetOutlineTextMetricsInternalW desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiRectangle(hdc: HDC, left: INT, top: INT, right: INT, bottom: INT) -> W32KAPI BOOL {
    // TODO: implementar NtGdiRectangle desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtGdiStretchDIBitsInternal(hdc: HDC, x_dst: INT, y_dst: INT, width_dst: INT, height_dst: INT, x_src: INT, y_src: INT, width_src: INT, height_src: INT, bits: *mut const void, bmi: *mut const BITMAPINFO, coloruse: UINT, rop: u32, max_info: UINT, max_bits: UINT, xform: *mut core::ffi::c_void) -> W32KAPI INT {
    // TODO: implementar NtGdiStretchDIBitsInternal desde wine/ntgdi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LocateCatalogsA(param_33286: *mut CHAR const, param_30140: ULONG, param_11068: *mut CHAR, param_1537: *mut ULONG, param_11068: *mut CHAR, param_1537: *mut ULONG) -> STDAPI {
    // TODO: implementar LocateCatalogsA desde wine/ntquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LocateCatalogsW(param_64802: *mut WCHAR const, param_30140: ULONG, param_34987: *mut u16, param_1537: *mut ULONG, param_34987: *mut u16, param_1537: *mut ULONG) -> STDAPI {
    // TODO: implementar LocateCatalogsW desde wine/ntquery.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LsaDeregisterLogonProcess(param_31864: *mut core::ffi::c_void) -> NTSTATUS {
    // TODO: implementar LsaDeregisterLogonProcess desde wine/ntsecapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LsaEnumerateLogonSessions(param_56625: PULONG, param_21186: *mut PLUID) -> NTSTATUS {
    // TODO: implementar LsaEnumerateLogonSessions desde wine/ntsecapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LsaGetLogonSessionData(param_42500: PLUID, param_27347: *mut PSECURITY_LOGON_SESSION_DATA) -> NTSTATUS {
    // TODO: implementar LsaGetLogonSessionData desde wine/ntsecapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LsaLogonUser(param_31864: *mut core::ffi::c_void, param_16238: PLSA_STRING, param_38644: SECURITY_LOGON_TYPE, param_30140: ULONG, param_10593: PVOID, param_30140: ULONG, param_29427: PTOKEN_GROUPS, param_9017: PTOKEN_SOURCE, param_35509: *mut PVOID, param_56625: PULONG, param_42500: PLUID, param_10895: PHANDLE, param_2381: PQUOTA_LIMITS, param_31584: PNTSTATUS) -> NTSTATUS {
    // TODO: implementar LsaLogonUser desde wine/ntsecapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LsaRegisterLogonProcess(param_16238: PLSA_STRING, param_10895: PHANDLE, param_18414: PLSA_OPERATIONAL_MODE) -> NTSTATUS {
    // TODO: implementar LsaRegisterLogonProcess desde wine/ntsecapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserGetAncestor(hwnd: *mut core::ffi::c_void, type: UINT) -> W32KAPI HWND {
    // TODO: implementar NtUserGetAncestor desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserGetClassInfoEx(instance: HINSTANCE, name: *mut UNICODE_STRING, wc: *mut WNDCLASSEXW, menu_name: *mut struct client_menu_name, ansi: i32) -> W32KAPI ATOM {
    // TODO: implementar NtUserGetClassInfoEx desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserLogicalToPerMonitorDPIPhysicalPoint(hwnd: *mut core::ffi::c_void, pt: *mut POINT) -> W32KAPI BOOL {
    // TODO: implementar NtUserLogicalToPerMonitorDPIPhysicalPoint desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserPerMonitorDPIPhysicalToLogicalPoint(hwnd: *mut core::ffi::c_void, pt: *mut POINT) -> W32KAPI BOOL {
    // TODO: implementar NtUserPerMonitorDPIPhysicalToLogicalPoint desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserSystemParametersInfo(action: UINT, val: UINT, ptr: *mut core::ffi::c_void, winini: UINT) -> W32KAPI BOOL {
    // TODO: implementar NtUserSystemParametersInfo desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserSystemParametersInfoForDpi(action: UINT, val: UINT, ptr: PVOID, winini: UINT, dpi: UINT) -> W32KAPI BOOL {
    // TODO: implementar NtUserSystemParametersInfoForDpi desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserGetDialogBaseUnits() -> static inline DWORD {
    // TODO: implementar NtUserGetDialogBaseUnits desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserGetDialogProc(proc: DLGPROC, ansi: i32) -> static inline DLGPROC {
    // TODO: implementar NtUserGetDialogProc desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserSetDialogInfo(hwnd: *mut core::ffi::c_void, info: *mut core::ffi::c_void) -> static inline void {
    // TODO: implementar NtUserSetDialogInfo desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn NtUserExposeWindowSurface(hwnd: *mut core::ffi::c_void, flags: UINT, rect: *mut const RECT, dpi: UINT) -> static inline BOOL {
    // TODO: implementar NtUserExposeWindowSurface desde wine/ntuser.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CoCreateInstance(param_2138: REFCLSID, param_4658: LPUNKNOWN, param_54075: u32, param_55479: REFIID, param_34430: *mut LPVOID) -> WINOLE32API HRESULT {
    // TODO: implementar CoCreateInstance desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CoCreateInstanceEx(param_2138: REFCLSID, param_4658: LPUNKNOWN, param_54075: u32, param_32438: *mut COSERVERINFO, param_30140: ULONG, param_606: *mut MULTI_QI) -> WINOLE32API HRESULT {
    // TODO: implementar CoCreateInstanceEx desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CoCreateInstanceFromApp(param_2138: REFCLSID, param_46438: *mut IUnknown, param_54075: u32, param_64866: *mut core::ffi::c_void, param_54075: u32, param_606: *mut MULTI_QI) -> WINOLE32API HRESULT {
    // TODO: implementar CoCreateInstanceFromApp desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CoGetInstanceFromFile(param_32438: *mut COSERVERINFO, param_18947: *mut CLSID, param_46438: *mut IUnknown, param_54075: u32, param_54075: u32, param_50560: *mut OLECHAR, param_54075: u32, param_606: *mut MULTI_QI) -> WINOLE32API HRESULT {
    // TODO: implementar CoGetInstanceFromFile desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CoGetInstanceFromIStorage(param_32438: *mut COSERVERINFO, param_18947: *mut CLSID, param_46438: *mut IUnknown, param_54075: u32, param_54990: *mut IStorage, param_54075: u32, param_606: *mut MULTI_QI) -> WINOLE32API HRESULT {
    // TODO: implementar CoGetInstanceFromIStorage desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CoSuspendClassObjects() -> WINOLE32API HRESULT {
    // TODO: implementar CoSuspendClassObjects desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CoGetStandardMarshal(riid: REFIID, pUnk: LPUNKNOWN, dwDestContext: u32, pvDestContext: LPVOID, mshlflags: u32, ppMarshal: *mut LPMARSHAL) -> WINOLE32API HRESULT {
    // TODO: implementar CoGetStandardMarshal desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CoSwitchCallContext(pContext: *mut IUnknown, param_46438: *mut IUnknown) -> WINOLE32API HRESULT {
    // TODO: implementar CoSwitchCallContext desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CoSetProxyBlanket(pProxy: *mut IUnknown, dwAuthnSvc: u32, dwAuthzSvc: u32, pServerPrincName: *mut OLECHAR, dwAuthnLevel: u32, dwImpLevel: u32, pAuthInfo: RPC_AUTH_IDENTITY_HANDLE, dwCapabilities: u32) -> WINOLE32API HRESULT {
    // TODO: implementar CoSetProxyBlanket desde wine/objbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn VarR8Pow(param_63078: f64, param_63078: f64, param_16407: *mut f64) -> WINOLEAUTAPI HRESULT {
    // TODO: implementar VarR8Pow desde wine/oleauto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn VarPow(param_45128: LPVARIANT, param_45128: LPVARIANT, param_45128: LPVARIANT) -> WINOLEAUTAPI HRESULT {
    // TODO: implementar VarPow desde wine/oleauto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PerfDeleteInstance(param_31864: *mut core::ffi::c_void, param_42302: *mut PERF_COUNTERSET_INSTANCE) -> ULONG {
    // TODO: implementar PerfDeleteInstance desde wine/perflib.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EFFECTIVE_POWER_MODE_CALLBACK(mode: EFFECTIVE_POWER_MODE, context: *mut core::ffi::c_void) -> typedef void {
    // TODO: implementar EFFECTIVE_POWER_MODE_CALLBACK desde wine/powersetting.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PowerRegisterForEffectivePowerModeNotifications(param_30140: ULONG, param_43750: *mut EFFECTIVE_POWER_MODE_, param_64866: *mut core::ffi::c_void, param_47066: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar PowerRegisterForEffectivePowerModeNotifications desde wine/powersetting.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CallNtPowerInformation(param_27546: POWER_INFORMATION_LEVEL, param_10593: PVOID, param_30140: ULONG, param_10593: PVOID, param_30140: ULONG) -> NTSTATUS {
    // TODO: implementar CallNtPowerInformation desde wine/powrprof.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetCurrentPowerPolicies(param_12498: PGLOBAL_POWER_POLICY, param_23754: PPOWER_POLICY) -> BOOLEAN {
    // TODO: implementar GetCurrentPowerPolicies desde wine/powrprof.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PowerEnumerate(param_28092: HKEY, param_4724: *mut const GUID, param_4724: *mut const GUID, param_38019: POWER_DATA_ACCESSOR, param_30140: ULONG, param_2482: *mut UCHAR, param_19332: *mut u32) -> u32 {
    // TODO: implementar PowerEnumerate desde wine/powrprof.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PowerRegisterSuspendResumeNotification(param_54075: u32, param_31864: *mut core::ffi::c_void, param_11834: PHPOWERNOTIFY) -> u32 {
    // TODO: implementar PowerRegisterSuspendResumeNotification desde wine/powrprof.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PowerUnregisterSuspendResumeNotification(param_63746: HPOWERNOTIFY) -> u32 {
    // TODO: implementar PowerUnregisterSuspendResumeNotification desde wine/powrprof.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PowerSettingRegisterNotification(param_4724: *mut const GUID, param_54075: u32, param_31864: *mut core::ffi::c_void, param_11834: PHPOWERNOTIFY) -> u32 {
    // TODO: implementar PowerSettingRegisterNotification desde wine/powrprof.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PowerSettingUnregisterNotification(param_63746: HPOWERNOTIFY) -> u32 {
    // TODO: implementar PowerSettingUnregisterNotification desde wine/powrprof.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PowerDeterminePlatformRole() -> POWER_PLATFORM_ROLE {
    // TODO: implementar PowerDeterminePlatformRole desde wine/powrprof.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RoActivateInstance(classid: HSTRING, param_24441: *mut IInspectable) -> i32 {
    // TODO: implementar RoActivateInstance desde wine/roapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupCloseLog() -> WINSETUPAPI void {
    // TODO: implementar SetupCloseLog desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiBuildClassInfoList(param_54075: u32, param_22847: LPGUID, param_54075: u32, param_64752: PDWORD) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupDiBuildClassInfoList desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiBuildClassInfoListExA(param_54075: u32, param_22847: LPGUID, param_54075: u32, param_64752: PDWORD, param_29633: PCSTR, param_10593: PVOID) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupDiBuildClassInfoListExA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiBuildClassInfoListExW(param_54075: u32, param_22847: LPGUID, param_54075: u32, param_64752: PDWORD, param_29658: PCWSTR, param_10593: PVOID) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupDiBuildClassInfoListExW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiCallClassInstaller(param_13430: DI_FUNCTION, param_44350: HDEVINFO, param_18335: PSP_DEVINFO_DATA) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupDiCallClassInstaller desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiGetClassInstallParamsA(param_44350: HDEVINFO, param_18335: PSP_DEVINFO_DATA, param_27930: PSP_CLASSINSTALL_HEADER, param_54075: u32, param_64752: PDWORD) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupDiGetClassInstallParamsA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiGetClassInstallParamsW(param_44350: HDEVINFO, param_18335: PSP_DEVINFO_DATA, param_27930: PSP_CLASSINSTALL_HEADER, param_54075: u32, param_64752: PDWORD) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupDiGetClassInstallParamsW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiGetDeviceInstanceIdA(param_44350: HDEVINFO, param_18335: PSP_DEVINFO_DATA, param_29209: PSTR, param_54075: u32, param_64752: PDWORD) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupDiGetDeviceInstanceIdA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiGetDeviceInstanceIdW(param_44350: HDEVINFO, param_18335: PSP_DEVINFO_DATA, param_31966: PWSTR, param_54075: u32, param_64752: PDWORD) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupDiGetDeviceInstanceIdW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiSetClassInstallParamsA(param_44350: HDEVINFO, param_18335: PSP_DEVINFO_DATA, param_27930: PSP_CLASSINSTALL_HEADER, param_54075: u32) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupDiSetClassInstallParamsA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupDiSetClassInstallParamsW(param_44350: HDEVINFO, param_18335: PSP_DEVINFO_DATA, param_27930: PSP_CLASSINSTALL_HEADER, param_54075: u32) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupDiSetClassInstallParamsW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupInitializeFileLogA(param_29633: PCSTR, param_54075: u32) -> WINSETUPAPI HSPFILELOG {
    // TODO: implementar SetupInitializeFileLogA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupInitializeFileLogW(param_29658: PCWSTR, param_54075: u32) -> WINSETUPAPI HSPFILELOG {
    // TODO: implementar SetupInitializeFileLogW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupLogErrorA(param_15619: LPCSTR, param_29580: LogSeverity) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupLogErrorA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupLogErrorW(param_25711: LPCWSTR, param_29580: LogSeverity) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupLogErrorW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupLogFileA(param_17501: HSPFILELOG, param_29633: PCSTR, param_29633: PCSTR, param_29633: PCSTR, param_54075: u32, param_29633: PCSTR, param_29633: PCSTR, param_29633: PCSTR, param_54075: u32) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupLogFileA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupLogFileW(param_17501: HSPFILELOG, param_29658: PCWSTR, param_29658: PCWSTR, param_29658: PCWSTR, param_54075: u32, param_29658: PCWSTR, param_29658: PCWSTR, param_29658: PCWSTR, param_54075: u32) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupLogFileW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupOpenLog(param_16716: i32) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupOpenLog desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupQueryDrivesInDiskSpaceListA(param_6941: HDSKSPC, param_29209: PSTR, param_54075: u32, param_64752: PDWORD) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupQueryDrivesInDiskSpaceListA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupQueryDrivesInDiskSpaceListW(param_6941: HDSKSPC, param_31966: PWSTR, param_54075: u32, param_64752: PDWORD) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupQueryDrivesInDiskSpaceListW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupQueryFileLogA(param_17501: HSPFILELOG, param_29633: PCSTR, param_29633: PCSTR, param_51715: SetupFileLogInfo, param_29209: PSTR, param_54075: u32, param_64752: PDWORD) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupQueryFileLogA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupQueryFileLogW(param_17501: HSPFILELOG, param_29658: PCWSTR, param_29658: PCWSTR, param_51715: SetupFileLogInfo, param_31966: PWSTR, param_54075: u32, param_64752: PDWORD) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupQueryFileLogW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupRemoveFileLogEntryA(param_17501: HSPFILELOG, param_29633: PCSTR, param_29633: PCSTR) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupRemoveFileLogEntryA desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupRemoveFileLogEntryW(param_17501: HSPFILELOG, param_29658: PCWSTR, param_29658: PCWSTR) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupRemoveFileLogEntryW desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetupTerminateFileLog(param_17501: HSPFILELOG) -> WINSETUPAPI BOOL {
    // TODO: implementar SetupTerminateFileLog desde wine/setupapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHCoCreateInstance(param_25711: LPCWSTR, param_60164: const, param_46438: *mut IUnknown, param_55479: REFIID, param_34430: *mut LPVOID) -> WINSHELLAPI HRESULT {
    // TODO: implementar SHCoCreateInstance desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHGetInstanceExplorer(param_57333: *mut core::ffi::c_void) -> WINSHELLAPI HRESULT {
    // TODO: implementar SHGetInstanceExplorer desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHStartNetConnectionDialog(param_11550: *mut core::ffi::c_void, param_15619: LPCSTR, param_54075: u32) -> WINSHELLAPI HRESULT {
    // TODO: implementar SHStartNetConnectionDialog desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RestartDialog(param_11550: *mut core::ffi::c_void, param_25711: LPCWSTR, param_54075: u32) -> WINSHELLAPI int {
    // TODO: implementar RestartDialog desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RestartDialogEx(param_11550: *mut core::ffi::c_void, param_25711: LPCWSTR, param_54075: u32, param_54075: u32) -> WINSHELLAPI int {
    // TODO: implementar RestartDialogEx desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SHOpenWithDialog(param_11550: *mut core::ffi::c_void, param_60164: const) -> WINSHELLAPI HRESULT {
    // TODO: implementar SHOpenWithDialog desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PathYetAnotherMakeUniqueName(param_46598: LPWSTR, param_25711: LPCWSTR, param_25711: LPCWSTR, param_25711: LPCWSTR) -> WINSHELLAPI BOOL {
    // TODO: implementar PathYetAnotherMakeUniqueName desde wine/shlobj.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PathUnExpandEnvStringsA(param_15619: LPCSTR, param_32262: LPSTR, param_2971: UINT) -> WINSHLWAPI BOOL {
    // TODO: implementar PathUnExpandEnvStringsA desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PathUnExpandEnvStringsW(param_25711: LPCWSTR, param_46598: LPWSTR, param_2971: UINT) -> WINSHLWAPI BOOL {
    // TODO: implementar PathUnExpandEnvStringsW desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StrIsIntlEqualA(param_16716: i32, param_15619: LPCSTR, param_15619: LPCSTR, param_59621: i32) -> WINSHLWAPI BOOL {
    // TODO: implementar StrIsIntlEqualA desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StrIsIntlEqualW(param_16716: i32, param_25711: LPCWSTR, param_25711: LPCWSTR, param_59621: i32) -> WINSHLWAPI BOOL {
    // TODO: implementar StrIsIntlEqualW desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsInternetESCEnabled() -> WINSHLWAPI BOOL {
    // TODO: implementar IsInternetESCEnabled desde wine/shlwapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SLGetLicensingStatusInformation(param_51369: HSLC, param_60164: const, param_60164: const, param_25711: LPCWSTR, param_47286: *mut UINT, param_44865: *mut core::ffi::c_void) -> SLCAPI HRESULT {
    // TODO: implementar SLGetLicensingStatusInformation desde wine/slpublic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SLGetWindowsInformation(param_25711: LPCWSTR, param_7977: *mut SLDATATYPE, param_47286: *mut UINT, param_34982: *mut LPBYTE) -> SLCAPI HRESULT {
    // TODO: implementar SLGetWindowsInformation desde wine/slpublic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SLGetWindowsInformationDWORD(param_25711: LPCWSTR, param_17803: LPDWORD) -> SLCAPI HRESULT {
    // TODO: implementar SLGetWindowsInformationDWORD desde wine/slpublic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SnmpSvcSetLogLevel(nLogLevel: INT) -> VOID {
    // TODO: implementar SnmpSvcSetLogLevel desde wine/snmp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SnmpSvcSetLogType(nLogType: INT) -> VOID {
    // TODO: implementar SnmpSvcSetLogType desde wine/snmp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TraceOpenLogFile(param_46598: LPWSTR, param_46598: LPWSTR, param_54075: u32) -> RETCODE {
    // TODO: implementar TraceOpenLogFile desde wine/sqlext.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn TraceCloseLogFile() -> RETCODE {
    // TODO: implementar TraceCloseLogFile desde wine/sqlext.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExportSecurityContext(phContext: PCtxtHandle, fFlags: ULONG, pPackedContext: PSecBuffer, param_64866: *mut core::ffi::c_void) -> SECURITY_STATUS SEC_ENTRY {
    // TODO: implementar ExportSecurityContext desde wine/sspi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StiCreateInstanceA(hinst: HINSTANCE, dwVer: u32, ppSti: *mut PSTIA, pUnkOuter: LPUNKNOWN) -> i32 {
    // TODO: implementar StiCreateInstanceA desde wine/sti.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StiCreateInstanceW(hinst: HINSTANCE, dwVer: u32, ppSti: *mut PSTIW, pUnkOuter: LPUNKNOWN) -> i32 {
    // TODO: implementar StiCreateInstanceW desde wine/sti.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WakeByAddressSingle(param_64866: *mut core::ffi::c_void) -> core::ffi::c_void {
    // TODO: implementar WakeByAddressSingle desde wine/synchapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetLogicalProcessorInformation(param_7081: PSYSTEM_LOGICAL_PROCESSOR_INFORMATION, param_64752: PDWORD) -> WINBASEAPI BOOL {
    // TODO: implementar GetLogicalProcessorInformation desde wine/sysinfoapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetLogicalProcessorInformationEx(param_47747: LOGICAL_PROCESSOR_RELATIONSHIP, param_31154: PSYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, param_64752: PDWORD) -> WINBASEAPI BOOL {
    // TODO: implementar GetLogicalProcessorInformationEx desde wine/sysinfoapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lineConfigDialog(param_54075: u32, param_11550: *mut core::ffi::c_void, param_15619: LPCSTR) -> u32 {
    // TODO: implementar lineConfigDialog desde wine/tapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lineConfigDialogEdit(param_54075: u32, param_11550: *mut core::ffi::c_void, param_15619: LPCSTR, const: LPVOID, param_54075: u32, param_64533: LPVARSTRING) -> u32 {
    // TODO: implementar lineConfigDialogEdit desde wine/tapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lineTranslateDialog(param_48049: HLINEAPP, param_54075: u32, param_54075: u32, param_11550: *mut core::ffi::c_void, param_15619: LPCSTR) -> u32 {
    // TODO: implementar lineTranslateDialog desde wine/tapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lineConfigDialogA(param_54075: u32, param_11550: *mut core::ffi::c_void, param_15619: LPCSTR) -> u32 {
    // TODO: implementar lineConfigDialogA desde wine/tapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lineConfigDialogEditA(param_54075: u32, param_11550: *mut core::ffi::c_void, param_15619: LPCSTR, const: LPVOID, param_54075: u32, param_64533: LPVARSTRING) -> u32 {
    // TODO: implementar lineConfigDialogEditA desde wine/tapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn lineTranslateDialogA(param_48049: HLINEAPP, param_54075: u32, param_54075: u32, param_11550: *mut core::ffi::c_void, param_15619: LPCSTR) -> u32 {
    // TODO: implementar lineTranslateDialogA desde wine/tapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn phoneConfigDialog(param_54075: u32, param_11550: *mut core::ffi::c_void, param_15619: LPCSTR) -> u32 {
    // TODO: implementar phoneConfigDialog desde wine/tapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn phoneConfigDialogA(param_54075: u32, param_11550: *mut core::ffi::c_void, param_15619: LPCSTR) -> u32 {
    // TODO: implementar phoneConfigDialogA desde wine/tapi.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExpandEnvironmentStringsForUserA(param_31864: *mut core::ffi::c_void, param_15619: LPCSTR, param_32262: LPSTR, param_54075: u32) -> USERENVAPI BOOL {
    // TODO: implementar ExpandEnvironmentStringsForUserA desde wine/userenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExpandEnvironmentStringsForUserW(param_31864: *mut core::ffi::c_void, param_25711: LPCWSTR, param_46598: LPWSTR, param_54075: u32) -> USERENVAPI BOOL {
    // TODO: implementar ExpandEnvironmentStringsForUserW desde wine/userenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ScriptApplyLogicalWidth(piDx: *mut const int, cChars: i32, cGlyphs: i32, pwLogClust: *mut const WORD, psva: *mut const SCRIPT_VISATTR, piAdvance: *mut const int, psa: *mut const SCRIPT_ANALYSIS, pABC: *mut ABC, piJustify: *mut i32) -> i32 {
    // TODO: implementar ScriptApplyLogicalWidth desde wine/usp10.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ScriptGetLogicalWidths(psa: *mut const SCRIPT_ANALYSIS, cChars: i32, cGlyphs: i32, piGlyphWidth: *mut const int, pwLogClust: *mut const WORD, psva: *mut const SCRIPT_VISATTR, piDx: *mut i32) -> i32 {
    // TODO: implementar ScriptGetLogicalWidths desde wine/usp10.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ScriptStringGetLogicalWidths(ssa: SCRIPT_STRING_ANALYSIS, piDx: *mut i32) -> i32 {
    // TODO: implementar ScriptStringGetLogicalWidths desde wine/usp10.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ScriptString_pLogAttr(ssa: SCRIPT_STRING_ANALYSIS) -> *mut const SCRIPT_LOGATTR {
    // TODO: implementar ScriptString_pLogAttr desde wine/usp10.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EnableThemeDialogTexture(hwnd: _In_ HWND, dwFlags: _In_ DWORD) -> i32 {
    // TODO: implementar EnableThemeDialogTexture desde reactos/uxthemesupp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetThemeSysInt(param_57865: HTHEME, param_59621: i32, param_7496: *mut i32) -> THEMEAPI {
    // TODO: implementar GetThemeSysInt desde wine/uxtheme.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WsInitializeMessage(param_20329: *mut WS_MESSAGE, param_42570: WS_MESSAGE_INITIALIZATION, param_20329: *mut WS_MESSAGE, param_1224: *mut WS_ERROR) -> i32 {
    // TODO: implementar WsInitializeMessage desde wine/webservices.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptExportKey(param_16953: HCRYPTKEY, param_16953: HCRYPTKEY, param_54075: u32, param_54075: u32, param_60692: *mut BYTE, param_19332: *mut u32) -> WINADVAPI BOOL {
    // TODO: implementar CryptExportKey desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CertEnumCertificatesInStore(hCertStore: HCERTSTORE, pPrev: PCCERT_CONTEXT) -> WINCRYPT32API PCCERT_CONTEXT {
    // TODO: implementar CertEnumCertificatesInStore desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CertEnumCRLsInStore(hCertStore: HCERTSTORE, pPrev: PCCRL_CONTEXT) -> WINCRYPT32API PCCRL_CONTEXT {
    // TODO: implementar CertEnumCRLsInStore desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CertEnumCTLsInStore(hCertStore: HCERTSTORE, pPrev: PCCTL_CONTEXT) -> WINCRYPT32API PCCTL_CONTEXT {
    // TODO: implementar CertEnumCTLsInStore desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CertIsRDNAttrsInCertificateName(dwCertEncodingType: u32, dwFlags: u32, pCertName: PCERT_NAME_BLOB, pRDN: PCERT_RDN) -> WINCRYPT32API BOOL {
    // TODO: implementar CertIsRDNAttrsInCertificateName desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptExportPublicKeyInfo(hCryptProv: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwKeySpec: u32, dwCertEncodingType: u32, pInfo: PCERT_PUBLIC_KEY_INFO, pcbInfo: *mut u32) -> WINCRYPT32API BOOL {
    // TODO: implementar CryptExportPublicKeyInfo desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptExportPublicKeyInfoEx(hCryptProv: HCRYPTPROV_OR_NCRYPT_KEY_HANDLE, dwKeySpec: u32, dwCertEncodingType: u32, pszPublicKeyObjId: LPSTR, dwFlags: u32, pvAuxInfo: *mut core::ffi::c_void, pInfo: PCERT_PUBLIC_KEY_INFO, pcbInfo: *mut u32) -> WINCRYPT32API BOOL {
    // TODO: implementar CryptExportPublicKeyInfoEx desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptMsgGetAndVerifySigner(hCryptMsg: HCRYPTMSG, cSignerStore: u32, rghSignerStore: *mut HCERTSTORE, dwFlags: u32, ppSigner: *mut PCCERT_CONTEXT, pdwSignerIndex: *mut u32) -> WINCRYPT32API BOOL {
    // TODO: implementar CryptMsgGetAndVerifySigner desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CryptDecryptAndVerifyMessageSignature(pDecryptPara: PCRYPT_DECRYPT_MESSAGE_PARA, pVerifyPara: PCRYPT_VERIFY_MESSAGE_PARA, dwSignerIndex: u32, pbEncryptedBlob: *mut const BYTE, cbEncryptedBlob: u32, pbDecrypted: *mut BYTE, pcbDecrypted: *mut u32, ppXchgCert: *mut PCCERT_CONTEXT, ppSignerCert: *mut PCCERT_CONTEXT) -> WINCRYPT32API BOOL {
    // TODO: implementar CryptDecryptAndVerifyMessageSignature desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PFXExportCertStoreEx(hStore: HCERTSTORE, pPFX: *mut CRYPT_DATA_BLOB, szPassword: LPCWSTR, pvReserved: *mut core::ffi::c_void, dwFlags: u32) -> WINCRYPT32API BOOL {
    // TODO: implementar PFXExportCertStoreEx desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PFXExportCertStore(hStore: HCERTSTORE, pPFX: *mut CRYPT_DATA_BLOB, szPassword: LPCWSTR, dwFlags: u32) -> WINCRYPT32API BOOL {
    // TODO: implementar PFXExportCertStore desde wine/wincrypt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DnsModifyRecordsInSet_A(param_60634: PDNS_RECORDA, param_60634: PDNS_RECORDA, param_54075: u32, param_31864: *mut core::ffi::c_void, param_10593: PVOID, param_10593: PVOID) -> DNS_STATUS {
    // TODO: implementar DnsModifyRecordsInSet_A desde wine/windns.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DnsModifyRecordsInSet_W(param_31851: PDNS_RECORDW, param_31851: PDNS_RECORDW, param_54075: u32, param_31864: *mut core::ffi::c_void, param_10593: PVOID, param_10593: PVOID) -> DNS_STATUS {
    // TODO: implementar DnsModifyRecordsInSet_W desde wine/windns.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DnsModifyRecordsInSet_UTF8(param_60634: PDNS_RECORDA, param_60634: PDNS_RECORDA, param_54075: u32, param_31864: *mut core::ffi::c_void, param_10593: PVOID, param_10593: PVOID) -> DNS_STATUS {
    // TODO: implementar DnsModifyRecordsInSet_UTF8 desde wine/windns.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EvtExportLog(session: EVT_HANDLE, path: *mut const WCHAR, query: *mut const WCHAR, file: *mut const WCHAR, flags: u32) -> i32 {
    // TODO: implementar EvtExportLog desde wine/winevt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InternetConfirmZoneCrossingA(param_11550: *mut core::ffi::c_void, param_32262: LPSTR, param_32262: LPSTR, param_16716: i32) -> INTERNETAPI DWORD {
    // TODO: implementar InternetConfirmZoneCrossingA desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InternetConfirmZoneCrossingW(param_11550: *mut core::ffi::c_void, param_46598: LPWSTR, param_46598: LPWSTR, param_16716: i32) -> INTERNETAPI DWORD {
    // TODO: implementar InternetConfirmZoneCrossingW desde wine/wininet.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsUrlCacheEntryExpiredA(param_15619: LPCSTR, param_54075: u32, param_48846: *mut FILETIME) -> i32 {
    // TODO: implementar IsUrlCacheEntryExpiredA desde wine/winineti.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsUrlCacheEntryExpiredW(param_25711: LPCWSTR, param_54075: u32, param_48846: *mut FILETIME) -> i32 {
    // TODO: implementar IsUrlCacheEntryExpiredW desde wine/winineti.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ldap_explode_dnA(param_4138: *mut i8, param_30140: ULONG) -> *mut core::ffi::c_void {
    // TODO: implementar ldap_explode_dnA desde wine/winldap.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ldap_explode_dnW(param_34987: *mut u16, param_30140: ULONG) -> *mut core::ffi::c_void {
    // TODO: implementar ldap_explode_dnW desde wine/winldap.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WNetConnectionDialog(param_11550: *mut core::ffi::c_void, param_54075: u32) -> u32 {
    // TODO: implementar WNetConnectionDialog desde wine/winnetwk.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WNetDisconnectDialog(param_11550: *mut core::ffi::c_void, param_54075: u32) -> u32 {
    // TODO: implementar WNetDisconnectDialog desde wine/winnetwk.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WNetConnectionDialog1A(param_64911: LPCONNECTDLGSTRUCTA) -> u32 {
    // TODO: implementar WNetConnectionDialog1A desde wine/winnetwk.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WNetConnectionDialog1W(param_58083: LPCONNECTDLGSTRUCTW) -> u32 {
    // TODO: implementar WNetConnectionDialog1W desde wine/winnetwk.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WNetDisconnectDialog1A(param_29867: LPDISCDLGSTRUCTA) -> u32 {
    // TODO: implementar WNetDisconnectDialog1A desde wine/winnetwk.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WNetDisconnectDialog1W(param_39146: LPDISCDLGSTRUCTW) -> u32 {
    // TODO: implementar WNetDisconnectDialog1W desde wine/winnetwk.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn load_export_name(param_4138: *mut i8, data: *mut IMAGE_DATA_DIRECTORY, align_mask: usize, unix_fd: i32, sec: *mut IMAGE_SECTION_HEADER, nb_sec: u32) -> static int {
    // TODO: implementar load_export_name desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn set_clip_rectangle(desktop: *mut struct desktop, rect: *mut const struct rectangle, flags: u32, reset: i32) -> core::ffi::c_void {
    // TODO: implementar set_clip_rectangle desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn validate_rectangles(rects: *mut const struct rectangle, nb_rects: u32) -> static inline int {
    // TODO: implementar validate_rectangles desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn get_mach_importance(effective_priority: i32) -> static int {
    // TODO: implementar get_mach_importance desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dump_rectangle(prefix: *mut const char, rect: *mut const struct rectangle) -> static void {
    // TODO: implementar dump_rectangle desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dump_varargs_rectangles(prefix: *mut const char, size: data_size_t) -> static void {
    // TODO: implementar dump_varargs_rectangles desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn is_using_msvcrt(make: *mut struct makefile) -> static bool {
    // TODO: implementar is_using_msvcrt desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn get_expanded_make_var_array(make: *mut const struct makefile, name: *mut const char) -> static struct strarray {
    // TODO: implementar get_expanded_make_var_array desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn get_expanded_file_local_var(make: *mut const struct makefile, file: *mut const char, name: *mut const char) -> static struct strarray {
    // TODO: implementar get_expanded_file_local_var desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn get_expanded_make_variable(param_47778: make, param_340: name) -> return {
    // TODO: implementar get_expanded_make_variable desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn factory_CreateInstance(iface: *mut IClassFactory, outer: *mut IUnknown, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT {
    // TODO: implementar factory_CreateInstance desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IParentAndItem_GetParentAndItem_Stub(This: *mut IParentAndItem, parent: *mut PIDLIST_ABSOLUTE, param_21024: *mut IShellFolder, child: *mut PITEMID_CHILD) -> HRESULT __RPC_STUB {
    // TODO: implementar IParentAndItem_GetParentAndItem_Stub desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IParentAndItem_GetParentAndItem(param_18975: This, param_52606: parent, param_4380: folder, param_47545: child) -> return {
    // TODO: implementar IParentAndItem_GetParentAndItem desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IParentAndItem_GetParentAndItem_Proxy(This: *mut IParentAndItem, parent: *mut PIDLIST_ABSOLUTE, param_21024: *mut IShellFolder, child: *mut PITEMID_CHILD) -> HRESULT __RPC_STUB {
    // TODO: implementar IParentAndItem_GetParentAndItem_Proxy desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IParentAndItem_RemoteGetParentAndItem_Proxy(param_18975: This, param_52606: parent, param_4380: folder, param_47545: child) -> return {
    // TODO: implementar IParentAndItem_RemoteGetParentAndItem_Proxy desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_QueryInterface(iface: *mut IADsADSystemInfo, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT {
    // TODO: implementar sysinfo_QueryInterface desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_AddRef(iface: *mut IADsADSystemInfo) -> static ULONG {
    // TODO: implementar sysinfo_AddRef desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_Release(iface: *mut IADsADSystemInfo) -> static ULONG {
    // TODO: implementar sysinfo_Release desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_GetTypeInfoCount(iface: *mut IADsADSystemInfo, count: *mut UINT) -> static HRESULT {
    // TODO: implementar sysinfo_GetTypeInfoCount desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_GetTypeInfo(iface: *mut IADsADSystemInfo, index: UINT, lcid: LCID, param_42131: *mut ITypeInfo) -> static HRESULT {
    // TODO: implementar sysinfo_GetTypeInfo desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_GetIDsOfNames(iface: *mut IADsADSystemInfo, riid: REFIID, names: *mut LPOLESTR, count: UINT, lcid: LCID, dispid: *mut DISPID) -> static HRESULT {
    // TODO: implementar sysinfo_GetIDsOfNames desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_UserName(iface: *mut IADsADSystemInfo, retval: *mut BSTR) -> static HRESULT {
    // TODO: implementar sysinfo_get_UserName desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_ComputerName(iface: *mut IADsADSystemInfo, retval: *mut BSTR) -> static HRESULT {
    // TODO: implementar sysinfo_get_ComputerName desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_SiteName(iface: *mut IADsADSystemInfo, retval: *mut BSTR) -> static HRESULT {
    // TODO: implementar sysinfo_get_SiteName desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_DomainShortName(iface: *mut IADsADSystemInfo, retval: *mut BSTR) -> static HRESULT {
    // TODO: implementar sysinfo_get_DomainShortName desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_DomainDNSName(iface: *mut IADsADSystemInfo, retval: *mut BSTR) -> static HRESULT {
    // TODO: implementar sysinfo_get_DomainDNSName desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_ForestDNSName(iface: *mut IADsADSystemInfo, retval: *mut BSTR) -> static HRESULT {
    // TODO: implementar sysinfo_get_ForestDNSName desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_PDCRoleOwner(iface: *mut IADsADSystemInfo, retval: *mut BSTR) -> static HRESULT {
    // TODO: implementar sysinfo_get_PDCRoleOwner desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_SchemaRoleOwner(iface: *mut IADsADSystemInfo, retval: *mut BSTR) -> static HRESULT {
    // TODO: implementar sysinfo_get_SchemaRoleOwner desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_get_IsNativeMode(iface: *mut IADsADSystemInfo, retval: *mut VARIANT_BOOL) -> static HRESULT {
    // TODO: implementar sysinfo_get_IsNativeMode desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_GetAnyDCName(iface: *mut IADsADSystemInfo, retval: *mut BSTR) -> static HRESULT {
    // TODO: implementar sysinfo_GetAnyDCName desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_GetDCSiteName(iface: *mut IADsADSystemInfo, server: BSTR, retval: *mut BSTR) -> static HRESULT {
    // TODO: implementar sysinfo_GetDCSiteName desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_RefreshSchemaCache(iface: *mut IADsADSystemInfo) -> static HRESULT {
    // TODO: implementar sysinfo_RefreshSchemaCache desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sysinfo_GetTrees(iface: *mut IADsADSystemInfo, retval: *mut VARIANT) -> static HRESULT {
    // TODO: implementar sysinfo_GetTrees desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WmiSetSingleInstanceA(handle: WMIHANDLE, name: *mut const char, reserved: ULONG, size: ULONG, buffer: *mut core::ffi::c_void) -> ULONG WMIAPI {
    // TODO: implementar WmiSetSingleInstanceA desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WmiSetSingleInstanceW(handle: WMIHANDLE, name: *mut const WCHAR, reserved: ULONG, size: ULONG, buffer: *mut core::ffi::c_void) -> ULONG WMIAPI {
    // TODO: implementar WmiSetSingleInstanceW desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WmiSetSingleItemA(handle: WMIHANDLE, name: *mut const char, id: ULONG, reserved: ULONG, size: ULONG, buffer: *mut core::ffi::c_void) -> ULONG WMIAPI {
    // TODO: implementar WmiSetSingleItemA desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn WmiSetSingleItemW(handle: WMIHANDLE, name: *mut const WCHAR, id: ULONG, reserved: ULONG, size: ULONG, buffer: *mut core::ffi::c_void) -> ULONG WMIAPI {
    // TODO: implementar WmiSetSingleItemW desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_QueryInterface(iface: *mut IPin, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT {
    // TODO: implementar audio_sink_QueryInterface desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_AddRef(iface: *mut IPin) -> static ULONG {
    // TODO: implementar audio_sink_AddRef desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_Release(iface: *mut IPin) -> static ULONG {
    // TODO: implementar audio_sink_Release desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_Connect(iface: *mut IPin, peer: *mut IPin, mt: *mut const AM_MEDIA_TYPE) -> static HRESULT {
    // TODO: implementar audio_sink_Connect desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_ReceiveConnection(iface: *mut IPin, peer: *mut IPin, mt: *mut const AM_MEDIA_TYPE) -> static HRESULT {
    // TODO: implementar audio_sink_ReceiveConnection desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_Disconnect(iface: *mut IPin) -> static HRESULT {
    // TODO: implementar audio_sink_Disconnect desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_ConnectedTo(iface: *mut IPin, param_62284: *mut IPin) -> static HRESULT {
    // TODO: implementar audio_sink_ConnectedTo desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_ConnectionMediaType(iface: *mut IPin, mt: *mut AM_MEDIA_TYPE) -> static HRESULT {
    // TODO: implementar audio_sink_ConnectionMediaType desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_QueryPinInfo(iface: *mut IPin, info: *mut PIN_INFO) -> static HRESULT {
    // TODO: implementar audio_sink_QueryPinInfo desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_QueryDirection(iface: *mut IPin, dir: *mut PIN_DIRECTION) -> static HRESULT {
    // TODO: implementar audio_sink_QueryDirection desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_QueryId(iface: *mut IPin, param_34987: *mut u16) -> static HRESULT {
    // TODO: implementar audio_sink_QueryId desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_QueryAccept(iface: *mut IPin, mt: *mut const AM_MEDIA_TYPE) -> static HRESULT {
    // TODO: implementar audio_sink_QueryAccept desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_EnumMediaTypes(iface: *mut IPin, param_25656: *mut IEnumMediaTypes) -> static HRESULT {
    // TODO: implementar audio_sink_EnumMediaTypes desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_QueryInternalConnections(iface: *mut IPin, param_62284: *mut IPin, count: *mut ULONG) -> static HRESULT {
    // TODO: implementar audio_sink_QueryInternalConnections desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_EndOfStream(iface: *mut IPin) -> static HRESULT {
    // TODO: implementar audio_sink_EndOfStream desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_BeginFlush(iface: *mut IPin) -> static HRESULT {
    // TODO: implementar audio_sink_BeginFlush desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_EndFlush(iface: *mut IPin) -> static HRESULT {
    // TODO: implementar audio_sink_EndFlush desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn audio_sink_NewSegment(iface: *mut IPin, start: REFERENCE_TIME, stop: REFERENCE_TIME, rate: f64) -> static HRESULT {
    // TODO: implementar audio_sink_NewSegment desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_QueryInterface(iface: *mut IPin, iid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT {
    // TODO: implementar ddraw_sink_QueryInterface desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_AddRef(iface: *mut IPin) -> static ULONG {
    // TODO: implementar ddraw_sink_AddRef desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_Release(iface: *mut IPin) -> static ULONG {
    // TODO: implementar ddraw_sink_Release desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_Connect(iface: *mut IPin, peer: *mut IPin, mt: *mut const AM_MEDIA_TYPE) -> static HRESULT {
    // TODO: implementar ddraw_sink_Connect desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_ReceiveConnection(iface: *mut IPin, peer: *mut IPin, mt: *mut const AM_MEDIA_TYPE) -> static HRESULT {
    // TODO: implementar ddraw_sink_ReceiveConnection desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_Disconnect(iface: *mut IPin) -> static HRESULT {
    // TODO: implementar ddraw_sink_Disconnect desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_ConnectedTo(iface: *mut IPin, param_62284: *mut IPin) -> static HRESULT {
    // TODO: implementar ddraw_sink_ConnectedTo desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_ConnectionMediaType(iface: *mut IPin, mt: *mut AM_MEDIA_TYPE) -> static HRESULT {
    // TODO: implementar ddraw_sink_ConnectionMediaType desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_QueryPinInfo(iface: *mut IPin, info: *mut PIN_INFO) -> static HRESULT {
    // TODO: implementar ddraw_sink_QueryPinInfo desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_QueryDirection(iface: *mut IPin, dir: *mut PIN_DIRECTION) -> static HRESULT {
    // TODO: implementar ddraw_sink_QueryDirection desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_QueryId(iface: *mut IPin, param_34987: *mut u16) -> static HRESULT {
    // TODO: implementar ddraw_sink_QueryId desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_QueryAccept(iface: *mut IPin, mt: *mut const AM_MEDIA_TYPE) -> static HRESULT {
    // TODO: implementar ddraw_sink_QueryAccept desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_EnumMediaTypes(iface: *mut IPin, param_25656: *mut IEnumMediaTypes) -> static HRESULT {
    // TODO: implementar ddraw_sink_EnumMediaTypes desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_QueryInternalConnections(iface: *mut IPin, param_62284: *mut IPin, count: *mut ULONG) -> static HRESULT {
    // TODO: implementar ddraw_sink_QueryInternalConnections desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_EndOfStream(iface: *mut IPin) -> static HRESULT {
    // TODO: implementar ddraw_sink_EndOfStream desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_BeginFlush(iface: *mut IPin) -> static HRESULT {
    // TODO: implementar ddraw_sink_BeginFlush desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_EndFlush(iface: *mut IPin) -> static HRESULT {
    // TODO: implementar ddraw_sink_EndFlush desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ddraw_sink_NewSegment(iface: *mut IPin, start: REFERENCE_TIME, stop: REFERENCE_TIME, rate: f64) -> static HRESULT {
    // TODO: implementar ddraw_sink_NewSegment desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn filter_seeking_IsUsingTimeFormat(iface: *mut IMediaSeeking, format: *mut const GUID) -> static HRESULT {
    // TODO: implementar filter_seeking_IsUsingTimeFormat desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AMCF_CreateInstance(iface: *mut IClassFactory, pOuter: *mut IUnknown, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT {
    // TODO: implementar AMCF_CreateInstance desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetInfoDialogText(hKey: HKEY, lpKeyName: LPCWSTR, lpAltMessage: LPCWSTR, hWnd: *mut core::ffi::c_void, iDlgItem: i32) -> static void {
    // TODO: implementar SetInfoDialogText desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn factory_ActivateInstance(iface: *mut IActivationFactory, param_24441: *mut IInspectable) -> static HRESULT {
    // TODO: implementar factory_ActivateInstance desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RegistrarCF_CreateInstance(iface: *mut IClassFactory, pUnkOuter: LPUNKNOWN, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT {
    // TODO: implementar RegistrarCF_CreateInstance desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AX_ConvertDialogTemplate(src_tmpl: LPCDLGTEMPLATEW) -> static LPDLGTEMPLATEW {
    // TODO: implementar AX_ConvertDialogTemplate desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlAxCreateDialogA(hInst: HINSTANCE, name: LPCSTR, owner: *mut core::ffi::c_void, dlgProc: DLGPROC, param: LPARAM) -> *mut core::ffi::c_void {
    // TODO: implementar AtlAxCreateDialogA desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlAxCreateDialogW(param_47240: hInst, param_19654: *mut core::ffi::c_void) -> return {
    // TODO: implementar AtlAxCreateDialogW desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlAxDialogBoxW(instance: HINSTANCE, name: *mut const WCHAR, owner: *mut core::ffi::c_void, proc: DLGPROC, param: LPARAM) -> INT_PTR {
    // TODO: implementar AtlAxDialogBoxW desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AtlAxDialogBoxA(instance: HINSTANCE, name: *mut const char, owner: *mut core::ffi::c_void, proc: DLGPROC, param: LPARAM) -> INT_PTR {
    // TODO: implementar AtlAxDialogBoxA desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IClassFactory_fnCreateInstance(iface: *mut IClassFactory, pOuter: *mut IUnknown, riid: REFIID, param_64866: *mut core::ffi::c_void) -> static HRESULT {
    // TODO: implementar IClassFactory_fnCreateInstance desde wine/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logic_dbl2int(a: *mut calc_number_t) -> __int64 {
    // TODO: implementar logic_dbl2int desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logic_int2dbl(a: *mut calc_number_t) -> f64 {
    // TODO: implementar logic_int2dbl desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_sin(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_sin desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_cos(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_cos desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_tan(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_tan desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_asin(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_asin desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_acos(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_acos desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_atan(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_atan desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_sinh(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_sinh desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_cosh(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_cosh desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_tanh(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_tanh desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_asinh(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_asinh desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_acosh(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_acosh desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_atanh(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_atanh desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_exp2(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_exp2 desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_exp3(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_exp3 desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_sqrt(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_sqrt desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_exp(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_exp desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_exp10(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_exp10 desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_log(c: *mut calc_number_t) -> core::ffi::c_void {
    // TODO: implementar rpn_log desde reactos/calc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DialogProc(hwndDlg: _In_ HWND, uMsg: _In_ UINT, wParam: _In_ WPARAM, lParam: _In_ LPARAM) -> static INT_PTR {
    // TODO: implementar DialogProc desde reactos/MainWindow.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EnableDialogTheme(hwnd: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar EnableDialogTheme desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FileExtractDialogWndProc(hDlg: *mut core::ffi::c_void, message: UINT, wParam: WPARAM, lParam: LPARAM) -> INT_PTR {
    // TODO: implementar FileExtractDialogWndProc desde reactos/fileextractdialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OnInitDialog(nMsg: UINT, wParam: WPARAM, lParam: LPARAM, bHandled: BOOL&) -> LRESULT {
    // TODO: implementar OnInitDialog desde reactos/dialogs.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OpenRDPConnectDialog(hInstance: HINSTANCE, pRdpSettings: PRDPSETTINGS) -> i32 {
    // TODO: implementar OpenRDPConnectDialog desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mppc_expand(data: *mut uint8, clen: uint32, ctype: uint8, roff: *mut uint32, rlen: *mut uint32) -> i32 {
    // TODO: implementar mppc_expand desde reactos/proto.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FileNew(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_FileNew desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FileNewWindow(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_FileNewWindow desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FileOpen(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_FileOpen desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FileSave(param_9080: VOID) -> i32 {
    // TODO: implementar DIALOG_FileSave desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FileSaveAs(param_9080: VOID) -> i32 {
    // TODO: implementar DIALOG_FileSaveAs desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FilePrint(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_FilePrint desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FilePageSetup(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_FilePageSetup desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FileExit(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_FileExit desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditUndo(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_EditUndo desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditCut(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_EditCut desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditCopy(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_EditCopy desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditPaste(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_EditPaste desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditDelete(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_EditDelete desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditSelectAll(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_EditSelectAll desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditTimeDate(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_EditTimeDate desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_EditWrap(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_EditWrap desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_Search(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_Search desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_SearchNext(bDown: i32) -> VOID {
    // TODO: implementar DIALOG_SearchNext desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_Replace(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_Replace desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_GoTo(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_GoTo desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_SelectFont(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_SelectFont desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_ViewStatusBar(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_ViewStatusBar desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_StatusBarAlignParts(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_StatusBarAlignParts desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_StatusBarUpdateCaretPos(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_StatusBarUpdateCaretPos desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_HelpContents(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_HelpContents desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_HelpSearch(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_HelpSearch desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_HelpLicense(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_HelpLicense desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_HelpNoWarranty(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_HelpNoWarranty desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_HelpAboutNotepad(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_HelpAboutNotepad desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_TimeDate(param_9080: VOID) -> VOID {
    // TODO: implementar DIALOG_TimeDate desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_StringMsgBox(hParent: *mut core::ffi::c_void, formatId: i32, szString: LPCTSTR, dwFlags: u32) -> i32 {
    // TODO: implementar DIALOG_StringMsgBox desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AboutDialogProc(hDlg: *mut core::ffi::c_void, message: UINT, wParam: WPARAM, lParam: LPARAM) -> INT_PTR {
    // TODO: implementar AboutDialogProc desde reactos/dialog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FindDialog(hWnd: *mut core::ffi::c_void) -> core::ffi::c_void {
    // TODO: implementar FindDialog desde reactos/main.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExportRegistryFile(hWnd: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar ExportRegistryFile desde reactos/main.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn export_registry_key(file_name: *mut u16, path: *mut u16, format: u32) -> i32 {
    // TODO: implementar export_registry_key desde reactos/main.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OnTreeExpanding(hWnd: *mut core::ffi::c_void, pnmtv: *mut NMTREEVIEW) -> i32 {
    // TODO: implementar OnTreeExpanding desde reactos/main.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn txt_export_registry_key(file_name: LPCWSTR, path: LPCWSTR) -> i32 {
    // TODO: implementar txt_export_registry_key desde reactos/main.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LoadDialogCtrls(PrefContext: PPREFERENCES_CONTEXT) -> VOID {
    // TODO: implementar LoadDialogCtrls desde reactos/sndvol32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UpdateDialogLineSliderControl(PrefContext: PPREFERENCES_CONTEXT, Line: LPMIXERLINE, DialogID: u32, Position: u32) -> VOID {
    // TODO: implementar UpdateDialogLineSliderControl desde reactos/sndvol32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UpdateDialogLineSwitchControl(PrefContext: PPREFERENCES_CONTEXT, Line: LPMIXERLINE, fValue: LONG) -> VOID {
    // TODO: implementar UpdateDialogLineSwitchControl desde reactos/sndvol32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PerfDataGetProcessIndex(pid: ULONG) -> ULONG {
    // TODO: implementar PerfDataGetProcessIndex desde reactos/perfdata.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ShutDown_StandBy(param_9080: VOID) -> VOID {
    // TODO: implementar ShutDown_StandBy desde reactos/shutdown.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ShutDown_PowerOff(param_9080: VOID) -> VOID {
    // TODO: implementar ShutDown_PowerOff desde reactos/shutdown.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ShutDown_LogOffUser(param_9080: VOID) -> VOID {
    // TODO: implementar ShutDown_LogOffUser desde reactos/shutdown.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn MACRO_CopyDialog() -> core::ffi::c_void {
    // TODO: implementar MACRO_CopyDialog desde reactos/macro.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Winver_GetOSInfo(OSInfo: _Out_ PWINVER_OS_INFO) -> i32 {
    // TODO: implementar Winver_GetOSInfo desde reactos/winver_p.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dialog_printsetup(param_11550: *mut core::ffi::c_void) -> core::ffi::c_void {
    // TODO: implementar dialog_printsetup desde reactos/wordpad.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dialog_print(param_11550: *mut core::ffi::c_void, param_46598: LPWSTR) -> core::ffi::c_void {
    // TODO: implementar dialog_print desde reactos/wordpad.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExpandTabLength(line: LPCTSTR) -> static INT {
    // TODO: implementar ExpandTabLength desde reactos/text.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExpandTab(line: LPCTSTR) -> static LPTSTR {
    // TODO: implementar ExpandTab desde reactos/text.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn reg_export(argc: i32) -> i32 {
    // TODO: implementar reg_export desde reactos/reg.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateDialogProc(hDlg: *mut core::ffi::c_void, message: UINT, wParam: WPARAM, lParam: LPARAM) -> INT_PTR {
    // TODO: implementar CreateDialogProc desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DeleteDialogProc(hDlg: *mut core::ffi::c_void, message: UINT, wParam: WPARAM, lParam: LPARAM) -> INT_PTR {
    // TODO: implementar DeleteDialogProc desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ProgressDialogProc(hDlg: *mut core::ffi::c_void, Message: UINT, wParam: WPARAM, lParam: LPARAM) -> INT_PTR {
    // TODO: implementar ProgressDialogProc desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StopDependsDialogProc(hDlg: *mut core::ffi::c_void, message: UINT, wParam: WPARAM, lParam: LPARAM) -> INT_PTR {
    // TODO: implementar StopDependsDialogProc desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateStopDependsDialog(hParent: *mut core::ffi::c_void, ServiceName: LPWSTR, DisplayName: LPWSTR, ServiceList: LPWSTR) -> i32 {
    // TODO: implementar CreateStopDependsDialog desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LogonPageProc(hwndDlg: *mut core::ffi::c_void, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> INT_PTR {
    // TODO: implementar LogonPageProc desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExportFile(Info: PMAIN_WND_INFO) -> VOID {
    // TODO: implementar ExportFile desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn login(__entry: *mut const struct utmp) -> extern void {
    // TODO: implementar login desde glibc/utmp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetItemsInContainer() -> size_type {
    // TODO: implementar GetItemsInContainer desde reactos/stl_bids.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LogErrorConsole(szError: LPTSTR) -> core::ffi::c_void {
    // TODO: implementar LogErrorConsole desde reactos/tnerror.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateInstalledAppInstance(KeyName: LPCWSTR, User: i32, WowSam: REGSAM) -> *mut static CInstalledApplicationInfo {
    // TODO: implementar CreateInstalledAppInstance desde reactos/appdb.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsInstalledEnum(x: INT) -> inline BOOL {
    // TODO: implementar IsInstalledEnum desde reactos/appinfo.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExtractAndRunGeneratedInstaller(param_5510: const CAvailableApplicationInfo, Archive: LPCWSTR, Silent: bool) -> i32 {
    // TODO: implementar ExtractAndRunGeneratedInstaller desde reactos/appinfo.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EmulateDialogReposition(hwnd: *mut core::ffi::c_void) -> VOID {
    // TODO: implementar EmulateDialogReposition desde reactos/misc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OpensWithExplorer(Path: PCWSTR) -> i32 {
    // TODO: implementar OpensWithExplorer desde reactos/misc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitLogs() -> VOID {
    // TODO: implementar InitLogs desde reactos/misc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExpandEnvStrings(param_12365: CStringW) -> bool {
    // TODO: implementar ExpandEnvStrings desde reactos/misc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GuessInstallerType(Installer: LPCWSTR, param_2971: UINT) -> InstallerType {
    // TODO: implementar GuessInstallerType desde reactos/misc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cicGetOSInfo(puACP: LPUINT, pdwOSInfo: LPDWORD) -> EXTERN_C void {
    // TODO: implementar cicGetOSInfo desde reactos/cicbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cicRealCoCreateInstance(rclsid: _In_ REFCLSID, pUnkOuter: _In_ LPUNKNOWN, dwClsContext: _In_ DWORD, iid: _In_ REFIID, ppv: *mut _Out_ LPVOID) -> EXTERN_C HRESULT {
    // TODO: implementar cicRealCoCreateInstance desde reactos/cicbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cicCoCreateInstance(rclsid: _In_ REFCLSID, pUnkOuter: _In_ LPUNKNOWN, dwClsContext: _In_ DWORD, iid: _In_ REFIID, ppv: *mut _Out_ LPVOID) -> EXTERN_C HRESULT {
    // TODO: implementar cicCoCreateInstance desde reactos/cicbase.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _IsInCollection(rguid: REFGUID) -> i32 {
    // TODO: implementar _IsInCollection desde reactos/displayattributemgr.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateInstance(pThreadMgrSink: *mut _In_ ITfThreadMgrEventSink, param_806: *mut _Out_ ITfDocumentMgr) -> static HRESULT {
    // TODO: implementar CreateInstance desde reactos/documentmgr.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AdviseSink(riid: REFIID, punk: *mut IUnknown, pdwCookie: *mut u32) -> STDMETHODIMP {
    // TODO: implementar AdviseSink desde reactos/documentmgr.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnadviseSink(pdwCookie: u32) -> STDMETHODIMP {
    // TODO: implementar UnadviseSink desde reactos/documentmgr.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn get_textservice_sink(tid: TfClientId, iid: REFCLSID, sink: *mut core::ffi::c_void) -> i32 {
    // TODO: implementar get_textservice_sink desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn set_textservice_sink(tid: TfClientId, iid: REFCLSID, sink: *mut IUnknown) -> i32 {
    // TODO: implementar set_textservice_sink desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn advise_sink(sink_list: *mut struct list, riid: REFIID, cookie_magic: u32, unk: *mut IUnknown, cookie: *mut u32) -> i32 {
    // TODO: implementar advise_sink desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unadvise_sink(cookie: u32) -> i32 {
    // TODO: implementar unadvise_sink desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AdviseMouseSink(range: *mut ITfRangeACP, pSink: *mut ITfMouseSink, pdwCookie: *mut u32) -> STDMETHODIMP {
    // TODO: implementar AdviseMouseSink desde reactos/inputcontext.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UnadviseMouseSink(dwCookie: u32) -> STDMETHODIMP {
    // TODO: implementar UnadviseMouseSink desde reactos/inputcontext.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IsInteractiveUserLogon(param_9080: VOID) -> i32 {
    // TODO: implementar IsInteractiveUserLogon desde reactos/misc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ActiveLanguageProfileNotifySinkCallback(rguid1: REFGUID, rguid2: REFGUID, fActivated: i32, pUserData: LPVOID) -> static INT {
    // TODO: implementar ActiveLanguageProfileNotifySinkCallback desde reactos/profile.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitProfileInstance(pTLS: *mut _Inout_ TLS) -> i32 {
    // TODO: implementar InitProfileInstance desde reactos/profile.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DnsIntCacheInitialize(param_9080: VOID) -> VOID {
    // TODO: implementar DnsIntCacheInitialize desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DnsIntCacheRemoveEntryItem(CacheEntry: PRESOLVER_CACHE_ENTRY) -> VOID {
    // TODO: implementar DnsIntCacheRemoveEntryItem desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DnsIntCacheFlush(ulFlags: _In_ ULONG) -> DNS_STATUS {
    // TODO: implementar DnsIntCacheFlush desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DnsIntFlushCacheEntry(pszName: _In_ LPCWSTR, wType: _In_ WORD) -> DNS_STATUS {
    // TODO: implementar DnsIntFlushCacheEntry desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DnsIntCacheGetEntryByName(Name: LPCWSTR, wType: WORD, dwFlags: u32, Record: *mut PDNS_RECORDW) -> DNS_STATUS {
    // TODO: implementar DnsIntCacheGetEntryByName desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DnsIntCacheAddEntry(Record: _In_ PDNS_RECORDW, bHostsFileEntry: _In_ BOOL) -> VOID {
    // TODO: implementar DnsIntCacheAddEntry desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DnsIntCacheRemoveEntryByName(Name: _In_ LPCWSTR) -> i32 {
    // TODO: implementar DnsIntCacheRemoveEntryByName desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DnsIntCacheGetEntries(param_44391: *mut _Out_ DNS_CACHE_ENTRY) -> DNS_STATUS {
    // TODO: implementar DnsIntCacheGetEntries desde reactos/precomp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LogfListInitialize(param_9080: VOID) -> VOID {
    // TODO: implementar LogfListInitialize desde reactos/eventlog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LogfListItemCount(param_9080: VOID) -> u32 {
    // TODO: implementar LogfListItemCount desde reactos/eventlog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LogfListItemByIndex(Index: u32) -> PLOGFILE {
    // TODO: implementar LogfListItemByIndex desde reactos/eventlog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LogfListItemByName(Name: LPCWSTR) -> PLOGFILE {
    // TODO: implementar LogfListItemByName desde reactos/eventlog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LogfCreate(LogFile: *mut PLOGFILE, LogName: PCWSTR, FileName: PUNICODE_STRING, MaxSize: ULONG, Retention: ULONG, Permanent: BOOLEAN, Backup: BOOLEAN) -> NTSTATUS {
    // TODO: implementar LogfCreate desde reactos/eventlog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LogfClearFile(LogFile: PLOGFILE, BackupFileName: PUNICODE_STRING) -> NTSTATUS {
    // TODO: implementar LogfClearFile desde reactos/eventlog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LogfBackupFile(LogFile: PLOGFILE, BackupFileName: PUNICODE_STRING) -> NTSTATUS {
    // TODO: implementar LogfBackupFile desde reactos/eventlog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LogfReportEvent(wType: USHORT, wCategory: USHORT, dwEventId: ULONG, wNumStrings: USHORT, pStrings: PWSTR, dwDataSize: ULONG, pRawData: PVOID) -> VOID {
    // TODO: implementar LogfReportEvent desde reactos/eventlog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitLogPort(param_9080: VOID) -> NTSTATUS {
    // TODO: implementar InitLogPort desde reactos/eventlog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn open_log_files() -> core::ffi::c_void {
    // TODO: implementar open_log_files desde reactos/daemon_debug.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn close_log_files() -> core::ffi::c_void {
    // TODO: implementar close_log_files desde reactos/daemon_debug.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AddToMessageLog(lpszMsg: LPTSTR) -> core::ffi::c_void {
    // TODO: implementar AddToMessageLog desde reactos/service.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn nfs_to_standard_info(info: *mut IN const nfs41_file_info, std_out: OUT PFILE_STANDARD_INFO) -> core::ffi::c_void {
    // TODO: implementar nfs_to_standard_info desde reactos/util.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SvcNetBiosInit(param_9080: VOID) -> VOID {
    // TODO: implementar SvcNetBiosInit desde reactos/svchost.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitLogging(param_9080: VOID) -> i32 {
    // TODO: implementar InitLogging desde reactos/tcpsvcs.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UninitLogging(param_9080: VOID) -> VOID {
    // TODO: implementar UninitLogging desde reactos/tcpsvcs.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LogEvent(lpMsg: LPCWSTR, errNum: u32, exitCode: u32, flags: UINT) -> VOID {
    // TODO: implementar LogEvent desde reactos/tcpsvcs.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn closelog() -> extern void {
    // TODO: implementar closelog desde reactos/syslog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn openlog(__ident: *mut i8, __option: i32, __facility: i32) -> extern void {
    // TODO: implementar openlog desde reactos/syslog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn setlogmask(__mask: i32) -> extern int {
    // TODO: implementar setlogmask desde reactos/syslog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn syslog(__pri: i32, __fmt: *mut i8) -> extern void {
    // TODO: implementar syslog desde reactos/syslog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vsyslog(__pri: i32, __fmt: *mut i8, __ap: va_list) -> extern void {
    // TODO: implementar vsyslog desde reactos/syslog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn set_syslog_conf_dir(dir: *mut const char) -> *mut extern const char {
    // TODO: implementar set_syslog_conf_dir desde reactos/syslog.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UserLogin(client_socket: i32) -> static void {
    // TODO: implementar UserLogin desde reactos/telnetd.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logMess(param_20024: *mut request, param_57831: MYBYTE) -> core::ffi::c_void {
    // TODO: implementar logMess desde reactos/tftpd.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ChangeACLsOfFilesInCurDir(pszFiles: LPCTSTR) -> static BOOL {
    // TODO: implementar ChangeACLsOfFilesInCurDir desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn asinh(x: f64) -> f64 {
    // TODO: implementar asinh desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn log(param_33517: *mut core::ffi::c_void) -> return {
    // TODO: implementar log desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn acosh(x: f64) -> f64 {
    // TODO: implementar acosh desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sqrt(param_55558: -1.0) -> return {
    // TODO: implementar sqrt desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn atanh(x: f64) -> f64 {
    // TODO: implementar atanh desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sqrti(number: unsigned __int64) -> static unsigned __int64 {
    // TODO: implementar sqrti desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rpn_pow_f(r: *mut calc_number_t, a: *mut calc_number_t, b: *mut calc_number_t) -> static void {
    // TODO: implementar rpn_pow_f desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pow(param_44987: 2., param_33216: n.f) -> *mut f {
    // TODO: implementar pow desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitInstance(hInst: HINSTANCE) -> static HWND {
    // TODO: implementar InitInstance desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitializeDialog(hwndDlg: *mut core::ffi::c_void, pDispDevice: PDISPLAY_DEVICEW) -> static BOOL {
    // TODO: implementar InitializeDialog desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DestroyTabCtrlDialogs(pContext: PDXDIAG_CONTEXT) -> VOID {
    // TODO: implementar DestroyTabCtrlDialogs desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitializeDxDiagDialog(hwndDlg: *mut core::ffi::c_void) -> VOID {
    // TODO: implementar InitializeDxDiagDialog desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitializeDirectInputDialog(hwndDlg: *mut core::ffi::c_void) -> static void {
    // TODO: implementar InitializeDirectInputDialog desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitializeDirectPlayDialog(hwndDlg: *mut core::ffi::c_void) -> static void {
    // TODO: implementar InitializeDirectPlayDialog desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn register_iexplore(doregister: i32) -> static DWORD {
    // TODO: implementar register_iexplore desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OpenFileDialog(hwnd: *mut core::ffi::c_void, dwFilterIndex: u32, lpType: LPTSTR) -> static VOID {
    // TODO: implementar OpenFileDialog desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetDialogIcon(hDlg: *mut core::ffi::c_void) -> static VOID {
    // TODO: implementar SetDialogIcon desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitializeSystemDialog(hDlg: *mut core::ffi::c_void) -> static BOOL {
    // TODO: implementar InitializeSystemDialog desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rdssl_mod_exp(out: *mut i8, out_len: i32, in: *mut i8, in_len: i32, mod: *mut i8, mod_len: i32, exp: *mut i8, exp_len: i32) -> i32 {
    // TODO: implementar rdssl_mod_exp desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rdp_send_logon_info(flags: uint32, domain: *mut i8, user: *mut i8, password: *mut i8, program: *mut i8, directory: *mut i8) -> static void {
    // TODO: implementar rdp_send_logon_info desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn process_pdu_logon(s: STREAM) -> core::ffi::c_void {
    // TODO: implementar process_pdu_logon desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rdssl_rkey_get_exp_mod(rkey: *mut uint8, exponent: *mut uint8, max_exp_len: uint32, modulus: *mut uint8, max_mod_len: uint32) -> i32 {
    // TODO: implementar rdssl_rkey_get_exp_mod desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_StatusBarUpdateLineEndings(param_9080: VOID) -> static VOID {
    // TODO: implementar DIALOG_StatusBarUpdateLineEndings desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_StatusBarUpdateEncoding(param_9080: VOID) -> static VOID {
    // TODO: implementar DIALOG_StatusBarUpdateEncoding desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_StatusBarUpdateAll(param_9080: VOID) -> static VOID {
    // TODO: implementar DIALOG_StatusBarUpdateAll desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_FileSaveAs_Hook(hDlg: *mut core::ffi::c_void, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> static UINT_PTR {
    // TODO: implementar DIALOG_FileSaveAs_Hook desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_SearchDialog(pfnProc: FINDPROC) -> static VOID {
    // TODO: implementar DIALOG_SearchDialog desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_GoTo_DialogProc(hwndDialog: *mut core::ffi::c_void, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> static INT_PTR {
    // TODO: implementar DIALOG_GoTo_DialogProc desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_Printing_DialogProc(hwnd: *mut core::ffi::c_void, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> static INT_PTR {
    // TODO: implementar DIALOG_Printing_DialogProc desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EndDialog(param_22891: hwnd, param_1184: IDCANCEL) -> else {
    // TODO: implementar EndDialog desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DIALOG_PAGESETUP_Hook(hDlg: *mut core::ffi::c_void, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> static UINT_PTR {
    // TODO: implementar DIALOG_PAGESETUP_Hook desde reactos/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __assert_single_arg(param_46528: _Bool) -> extern _Bool {
    // TODO: implementar __assert_single_arg desde glibc/assert.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __bswap_constant_16(param_8416: __bsx) -> return {
    // TODO: implementar __bswap_constant_16 desde glibc/byteswap.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __bswap_constant_32(param_8416: __bsx) -> return {
    // TODO: implementar __bswap_constant_32 desde glibc/byteswap.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __bswap_constant_64(param_8416: __bsx) -> return {
    // TODO: implementar __bswap_constant_64 desde glibc/byteswap.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __open_catalog(cat_name: *mut const char, nlspath: *mut const char, env_var: *mut const char, __catalog: __nl_catd) -> extern int {
    // TODO: implementar __open_catalog desde glibc/catgetsinfo.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn set_expected_protected1(param_59621: i32) -> extern void {
    // TODO: implementar set_expected_protected1 desde glibc/tst-protected1mod.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn set_expected_protected3a(param_59621: i32) -> extern void {
    // TODO: implementar set_expected_protected3a desde glibc/tst-protected1mod.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn set_expected_protected3b(param_59621: i32) -> extern void {
    // TODO: implementar set_expected_protected3b desde glibc/tst-protected1mod.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __kernel_casinhf(z: complex float, adj: i32) -> extern complex float {
    // TODO: implementar __kernel_casinhf desde glibc/complex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __kernel_casinh(z: complex double, adj: i32) -> extern complex double {
    // TODO: implementar __kernel_casinh desde glibc/complex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __kernel_casinhl(z: complex long double, adj: i32) -> extern complex long double {
    // TODO: implementar __kernel_casinhl desde glibc/complex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __kernel_casinhf128(z: __CFLOAT128, adj: i32) -> extern __CFLOAT128 {
    // TODO: implementar __kernel_casinhf128 desde glibc/complex.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __feclearexcept(__excepts: i32) -> extern int {
    // TODO: implementar __feclearexcept desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __fegetexcept() -> extern int {
    // TODO: implementar __fegetexcept desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __fegetexceptflag(__flagp: *mut fexcept_t, __excepts: i32) -> extern int {
    // TODO: implementar __fegetexceptflag desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __feraiseexcept(__excepts: i32) -> extern int {
    // TODO: implementar __feraiseexcept desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __fesetexceptflag(__flagp: *mut const fexcept_t, __excepts: i32) -> extern int {
    // TODO: implementar __fesetexceptflag desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __fegetenv(__envp: *mut fenv_t) -> extern int {
    // TODO: implementar __fegetenv desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __fesetenv(__envp: *mut const fenv_t) -> extern int {
    // TODO: implementar __fesetenv desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __feupdateenv(__envp: *mut const fenv_t) -> extern int {
    // TODO: implementar __feupdateenv desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fegetenv(__e: *mut fenv_t) -> extern inline int {
    // TODO: implementar fegetenv desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn feholdexcept(__e: *mut fenv_t) -> extern inline int {
    // TODO: implementar feholdexcept desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __feholdexcept(__e: *mut fenv_t) -> extern inline int {
    // TODO: implementar __feholdexcept desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fesetenv(__e: *mut const fenv_t) -> extern inline int {
    // TODO: implementar fesetenv desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn feupdateenv(__e: *mut const fenv_t) -> extern inline int {
    // TODO: implementar feupdateenv desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fegetround() -> extern inline int {
    // TODO: implementar fegetround desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __fegetround() -> extern inline int {
    // TODO: implementar __fegetround desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fesetround(__d: i32) -> extern inline int {
    // TODO: implementar fesetround desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __fesetround(__d: i32) -> extern inline int {
    // TODO: implementar __fesetround desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn find_empty_slot_for_expand(htab: *mut struct hashtab, hash: i32) -> *mut core::ffi::c_void {
    // TODO: implementar find_empty_slot_for_expand desde glibc/inline-hashtab.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn htab_expand(htab: *mut struct hashtab, param_59621: i32) -> inline static int {
    // TODO: implementar htab_expand desde glibc/inline-hashtab.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __issignalingf(x: f32) -> extern inline int {
    // TODO: implementar __issignalingf desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __isinff128(x: _Float128) -> extern inline int {
    // TODO: implementar __isinff128 desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fabsf128(x: _Float128) -> extern inline _Float128 {
    // TODO: implementar fabsf128 desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __builtin_fabsf128(param_52739: x) -> return {
    // TODO: implementar __builtin_fabsf128 desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __mbsinit(__ps: *mut const __mbstate_t) -> extern int {
    // TODO: implementar __mbsinit desde glibc/wchar.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _nl_explode_name(name: *mut i8, param_6043: *mut const char, param_6043: *mut const char, param_6043: *mut const char, param_6043: *mut const char, param_6043: *mut const char) -> extern int {
    // TODO: implementar _nl_explode_name desde glibc/loadinfo.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EXTRACT_PLURAL_EXPRESSION(nullentry: *mut const char, param_48966: *mut const struct expression, npluralsp: *mut unsigned long int) -> extern void {
    // TODO: implementar EXTRACT_PLURAL_EXPRESSION desde glibc/plural-exp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn login_tty(__fd: i32) -> __BEGIN_DECLS extern int {
    // TODO: implementar login_tty desde glibc/utmp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logout(__ut_line: *mut const char) -> extern int {
    // TODO: implementar logout desde glibc/utmp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logwtmp(__ut_line: *mut const char, __ut_name: *mut const char, __ut_host: *mut const char) -> extern void {
    // TODO: implementar logwtmp desde glibc/utmp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fromfp_max_exponent(negative: bool, width: i32) -> static int {
    // TODO: implementar fromfp_max_exponent desde glibc/compat_fromfp.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn feclearexcept(__excepts: i32) -> __BEGIN_DECLS extern int {
    // TODO: implementar feclearexcept desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fegetexceptflag(__flagp: *mut fexcept_t, __excepts: i32) -> extern int {
    // TODO: implementar fegetexceptflag desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn feraiseexcept(__excepts: i32) -> extern int {
    // TODO: implementar feraiseexcept desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fesetexcept(__excepts: i32) -> extern int {
    // TODO: implementar fesetexcept desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fesetexceptflag(__flagp: *mut const fexcept_t, __excepts: i32) -> extern int {
    // TODO: implementar fesetexceptflag desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fetestexcept(__excepts: i32) -> extern int {
    // TODO: implementar fetestexcept desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fetestexceptflag(__flagp: *mut const fexcept_t, __excepts: i32) -> extern int {
    // TODO: implementar fetestexceptflag desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fegetmode(__modep: *mut femode_t) -> extern int {
    // TODO: implementar fegetmode desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fesetmode(__modep: *mut const femode_t) -> extern int {
    // TODO: implementar fesetmode desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn feenableexcept(__excepts: i32) -> extern int {
    // TODO: implementar feenableexcept desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fedisableexcept(__excepts: i32) -> extern int {
    // TODO: implementar fedisableexcept desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fegetexcept() -> extern int {
    // TODO: implementar fegetexcept desde glibc/fenv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CHECK_NARROW_SQRT(param_60907: ret, param_19346: *mut core::ffi::c_void) -> sqrt {
    // TODO: implementar CHECK_NARROW_SQRT desde glibc/math-narrow.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __kernel_standard(param_63078: f64, param_63078: f64, param_59621: i32) -> extern double {
    // TODO: implementar __kernel_standard desde glibc/math-svid-compat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __kernel_standard_f(param_34772: f32, param_34772: f32, param_59621: i32) -> extern float {
    // TODO: implementar __kernel_standard_f desde glibc/math-svid-compat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __kernel_standard_l(double: i64, double: i64, param_59621: i32) -> extern long double {
    // TODO: implementar __kernel_standard_l desde glibc/math-svid-compat.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __nldbl_nexttowardf(__x: f32, __y: long double) -> defined _LIBC_TEST extern float {
    // TODO: implementar __nldbl_nexttowardf desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __MATHCALL_NARROW_NORMAL(param_54956: func, param_34513: nargs) -> 1 {
    // TODO: implementar __MATHCALL_NARROW_NORMAL desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn issignaling(__val: f32) -> inline int {
    // TODO: implementar issignaling desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __issignaling(param_13651: __val) -> return {
    // TODO: implementar __issignaling desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __issignalingl(param_13651: __val) -> return {
    // TODO: implementar __issignalingl desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __issignalingf128(param_13651: __val) -> return {
    // TODO: implementar __issignalingf128 desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn iszero(__val: f32) -> inline int {
    // TODO: implementar iszero desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __fpclassifyf(param_13651: __val) -> return {
    // TODO: implementar __fpclassifyf desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __fpclassify(param_13651: __val) -> return {
    // TODO: implementar __fpclassify desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __fpclassifyl(param_13651: __val) -> return {
    // TODO: implementar __fpclassifyl desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __fpclassifyf128(param_13651: __val) -> return {
    // TODO: implementar __fpclassifyf128 desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __call(__x: f32, __y: f32) -> static int {
    // TODO: implementar __call desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __iseqsigf(param_51278: __x, param_39398: __y) -> return {
    // TODO: implementar __iseqsigf desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __iseqsig(param_51278: __x, param_39398: __y) -> return {
    // TODO: implementar __iseqsig desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __iseqsigl(param_51278: __x, param_39398: __y) -> return {
    // TODO: implementar __iseqsigl desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __iseqsigf128(param_51278: __x, param_39398: __y) -> return {
    // TODO: implementar __iseqsigf128 desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn iseqsig(__x: _T1, __y: _T2) -> inline int {
    // TODO: implementar iseqsig desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn decltype(param_25011: __MATH_EVAL_FMT2, param_39398: __y) -> typedef {
    // TODO: implementar decltype desde glibc/math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mul_expansion(hi: *mut f64, lo: *mut f64, h1: f64, l1: f64, h2: f64, l2: f64) -> static inline void {
    // TODO: implementar mul_expansion desde glibc/mul_split.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn div_expansion(hi: *mut f64, lo: *mut f64, h1: f64, l1: f64, h2: f64, l2: f64) -> static inline void {
    // TODO: implementar div_expansion desde glibc/mul_split.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn hol_cousin_cluster_cmp(cl1: *mut const struct hol_cluster, cl2: *mut const struct hol_cluster) -> static int {
    // TODO: implementar hol_cousin_cluster_cmp desde glibc/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __isinf_ns2(d: f64) -> static __always_inline int {
    // TODO: implementar __isinf_ns2 desde glibc/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn kernel_standard(param_52739: x, param_49451: y, param_61909: 10) -> return {
    // TODO: implementar kernel_standard desde glibc/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AnnotateExpectRace(file: *mut const char, line: i32, address: *mut const volatile void, description: *mut const char) -> core::ffi::c_void {
    // TODO: implementar AnnotateExpectRace desde cpython/dynamic_annotations.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyObjectIndexPair(param_3597: *_Py_iteritemfunc) -> typedef {
    // TODO: implementar _PyObjectIndexPair desde cpython/object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyDTrace_INSTANCE_NEW_START_ENABLED() -> static inline int {
    // TODO: implementar PyDTrace_INSTANCE_NEW_START_ENABLED desde cpython/pydtrace.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyDTrace_INSTANCE_NEW_DONE_ENABLED() -> static inline int {
    // TODO: implementar PyDTrace_INSTANCE_NEW_DONE_ENABLED desde cpython/pydtrace.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyDTrace_INSTANCE_DELETE_START_ENABLED() -> static inline int {
    // TODO: implementar PyDTrace_INSTANCE_DELETE_START_ENABLED desde cpython/pydtrace.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyDTrace_INSTANCE_DELETE_DONE_ENABLED() -> static inline int {
    // TODO: implementar PyDTrace_INSTANCE_DELETE_DONE_ENABLED desde cpython/pydtrace.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _Py_log1p(x: f64) -> static double {
    // TODO: implementar _Py_log1p desde cpython/_math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn log1p(param_52739: x) -> return {
    // TODO: implementar log1p desde cpython/_math.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyPegen_lookahead_for_expr(param_59621: i32, param_43512: expr_ty) -> i32 {
    // TODO: implementar _PyPegen_lookahead_for_expr desde cpython/pegen.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyPegen_expect_forced_result(p: *mut Parser, result: *mut core::ffi::c_void, expected: *mut const char) -> *mut core::ffi::c_void {
    // TODO: implementar _PyPegen_expect_forced_result desde cpython/pegen.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyPegen_expect_soft_keyword(p: *mut Parser, keyword: *mut const char) -> expr_ty {
    // TODO: implementar _PyPegen_expect_soft_keyword desde cpython/pegen.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyPegen_set_expr_context(param_33222: *mut Parser, param_43512: expr_ty, param_32611: expr_context_ty) -> expr_ty {
    // TODO: implementar _PyPegen_set_expr_context desde cpython/pegen.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyPegen_constant_from_token(p: *mut Parser, tok: *mut Token) -> expr_ty {
    // TODO: implementar _PyPegen_constant_from_token desde cpython/pegen.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyPegen_decoded_constant_from_token(p: *mut Parser, tok: *mut Token) -> expr_ty {
    // TODO: implementar _PyPegen_decoded_constant_from_token desde cpython/pegen.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyPegen_constant_from_string(p: *mut Parser, tok: *mut Token) -> expr_ty {
    // TODO: implementar _PyPegen_constant_from_string desde cpython/pegen.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _register_builtins_for_crossinterpreter_data(param_62972: *mut dlregistry_t) -> static void {
    // TODO: implementar _register_builtins_for_crossinterpreter_data desde cpython/crossinterp_data_lookup.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _TAIL_CALL_CALL_ISINSTANCE(param_28619: TAIL_CALL_PARAMS) -> static PyObject *Py_PRESERVE_NONE_CC {
    // TODO: implementar _TAIL_CALL_CALL_ISINSTANCE desde cpython/opcode_targets.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _TAIL_CALL_LOAD_ATTR_INSTANCE_VALUE(param_28619: TAIL_CALL_PARAMS) -> static PyObject *Py_PRESERVE_NONE_CC {
    // TODO: implementar _TAIL_CALL_LOAD_ATTR_INSTANCE_VALUE desde cpython/opcode_targets.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _TAIL_CALL_LOAD_COMMON_CONSTANT(param_28619: TAIL_CALL_PARAMS) -> static PyObject *Py_PRESERVE_NONE_CC {
    // TODO: implementar _TAIL_CALL_LOAD_COMMON_CONSTANT desde cpython/opcode_targets.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _TAIL_CALL_STORE_ATTR_INSTANCE_VALUE(param_28619: TAIL_CALL_PARAMS) -> static PyObject *Py_PRESERVE_NONE_CC {
    // TODO: implementar _TAIL_CALL_STORE_ATTR_INSTANCE_VALUE desde cpython/opcode_targets.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInstanceMethod_GET_FUNCTION(meth: *mut PyObject) -> *mut static inline PyObject {
    // TODO: implementar PyInstanceMethod_GET_FUNCTION desde cpython/classobject.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyInstanceMethod_CAST(param_65376: meth) -> return {
    // TODO: implementar _PyInstanceMethod_CAST desde cpython/classobject.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn atomic_load_explicit(param_4054: *mut core::ffi::c_void) -> return {
    // TODO: implementar atomic_load_explicit desde cpython/pyatomic_std.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyObject_RealIsInstance(inst: *mut PyObject, cls: *mut PyObject) -> extern int {
    // TODO: implementar _PyObject_RealIsInstance desde cpython/pycore_abstract.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyAST_Expression(body: expr_ty, arena: *mut PyArena) -> mod_ty {
    // TODO: implementar _PyAST_Expression desde cpython/pycore_ast.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyAST_Expr(value: expr_ty, lineno: i32, col_offset: i32, end_lineno: i32, end_col_offset: i32, arena: *mut PyArena) -> stmt_ty {
    // TODO: implementar _PyAST_Expr desde cpython/pycore_ast.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyAST_NamedExpr(target: expr_ty, value: expr_ty, lineno: i32, col_offset: i32, end_lineno: i32, end_col_offset: i32, arena: *mut PyArena) -> expr_ty {
    // TODO: implementar _PyAST_NamedExpr desde cpython/pycore_ast.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyAST_IfExp(test: expr_ty, body: expr_ty, orelse: expr_ty, lineno: i32, col_offset: i32, end_lineno: i32, end_col_offset: i32, arena: *mut PyArena) -> expr_ty {
    // TODO: implementar _PyAST_IfExp desde cpython/pycore_ast.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyAST_GeneratorExp(elt: expr_ty, generators: *mut asdl_comprehension_seq, lineno: i32, col_offset: i32, end_lineno: i32, end_col_offset: i32, arena: *mut PyArena) -> expr_ty {
    // TODO: implementar _PyAST_GeneratorExp desde cpython/pycore_ast.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyAST_Constant(value: constant, kind: string, lineno: i32, col_offset: i32, end_lineno: i32, end_col_offset: i32, arena: *mut PyArena) -> expr_ty {
    // TODO: implementar _PyAST_Constant desde cpython/pycore_ast.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyAST_MatchSingleton(value: constant, lineno: i32, col_offset: i32, end_lineno: i32, end_col_offset: i32, arena: *mut PyArena) -> pattern_ty {
    // TODO: implementar _PyAST_MatchSingleton desde cpython/pycore_ast.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyAST_ExprAsUnicode(param_43512: expr_ty) -> *mut extern PyObject {
    // TODO: implementar _PyAST_ExprAsUnicode desde cpython/pycore_ast.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _BlocksOutputBuffer_InitAndGrow(buffer: *mut _BlocksOutputBuffer, max_length: const Py_ssize_t, param_64866: *mut core::ffi::c_void) -> static inline Py_ssize_t {
    // TODO: implementar _BlocksOutputBuffer_InitAndGrow desde cpython/pycore_blocks_output_buffer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyCode_ConstantKey(obj: *mut PyObject) -> *mut extern PyObject {
    // TODO: implementar _PyCode_ConstantKey desde cpython/pycore_code.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyCompile_StartAnnotationSetup(c: *mut struct _PyCompiler) -> i32 {
    // TODO: implementar _PyCompile_StartAnnotationSetup desde cpython/pycore_compile.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyCompile_IsInteractiveTopLevel(c: *mut struct _PyCompiler) -> i32 {
    // TODO: implementar _PyCompile_IsInteractiveTopLevel desde cpython/pycore_compile.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyCompile_IsInInlinedComp(c: *mut struct _PyCompiler) -> i32 {
    // TODO: implementar _PyCompile_IsInInlinedComp desde cpython/pycore_compile.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyCodegen_Expression(c: *mut struct _PyCompiler, e: expr_ty) -> i32 {
    // TODO: implementar _PyCodegen_Expression desde cpython/pycore_compile.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyImport_IsInitialized(param_15054: *mut PyInterpreterState) -> extern int {
    // TODO: implementar _PyImport_IsInitialized desde cpython/pycore_import.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyImport_GetModuleExportHooks(info: *mut struct _Py_ext_module_loader_info, fp: *mut FILE, modinit: *mut PyModInitFunction, modexport: *mut PyModExportFunction) -> extern int {
    // TODO: implementar _PyImport_GetModuleExportHooks desde cpython/pycore_importdl.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyIndexPool_Fini(indices: *mut _PyIndexPool) -> extern void {
    // TODO: implementar _PyIndexPool_Fini desde cpython/pycore_index_pool.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyInstructionSequence_SetAnnotationsCode(seq: *mut _PyInstructionSequence, annotations: *mut _PyInstructionSequence) -> i32 {
    // TODO: implementar _PyInstructionSequence_SetAnnotationsCode desde cpython/pycore_instruction_sequence.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyFrame_IsIncomplete(frame: *mut _PyInterpreterFrame) -> static inline bool _Py_NO_SANITIZE_THREAD {
    // TODO: implementar _PyFrame_IsIncomplete desde cpython/pycore_interpframe.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyLong_CheckExactAndCompact(op: *mut PyObject) -> static inline int {
    // TODO: implementar _PyLong_CheckExactAndCompact desde cpython/pycore_long.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyModuleSpec_IsInitializing(param_45657: *mut PyObject) -> extern int {
    // TODO: implementar _PyModuleSpec_IsInitializing desde cpython/pycore_moduleobject.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _Py_ExplicitMergeRefcount(op: *mut PyObject, extra: Py_ssize_t) -> Py_ssize_t {
    // TODO: implementar _Py_ExplicitMergeRefcount desde cpython/pycore_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyObject_StoreInstanceAttribute(obj: *mut PyObject, name: *mut PyObject, value: *mut PyObject) -> extern int {
    // TODO: implementar _PyObject_StoreInstanceAttribute desde cpython/pycore_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyObject_TryGetInstanceAttribute(obj: *mut PyObject, name: *mut PyObject, param_45657: *mut PyObject) -> extern bool {
    // TODO: implementar _PyObject_TryGetInstanceAttribute desde cpython/pycore_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _PyObject_IsInstanceDictEmpty(param_45657: *mut PyObject) -> extern int {
    // TODO: implementar _PyObject_IsInstanceDictEmpty desde cpython/pycore_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _Py_GetConstant_Init() -> extern void {
    // TODO: implementar _Py_GetConstant_Init desde cpython/pycore_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyJitRef_IsInvalid(ref: JitOptRef) -> static inline bool {
    // TODO: implementar PyJitRef_IsInvalid desde cpython/pycore_optimizer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _Py_HandleSystemExitAndKeyboardInterrupt(exitcode_p: *mut i32) -> extern int {
    // TODO: implementar _Py_HandleSystemExitAndKeyboardInterrupt desde cpython/pycore_pylifecycle.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _Py_FdIsInteractive(fp: *mut FILE, filename: *mut PyObject) -> extern int {
    // TODO: implementar _Py_FdIsInteractive desde cpython/pycore_pylifecycle.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_expand(p: *mut core::ffi::c_void, newsize: usize) -> *mut mi_decl_export void {
    // TODO: implementar mi_expand desde cpython/mimalloc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi__expand(p: *mut core::ffi::c_void, newsize: usize) -> *mut mi_decl_export void {
    // TODO: implementar mi__expand desde cpython/mimalloc.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_fetch_add_explicit(param_15105: *mut core::ffi::c_void) -> static inline uintptr_t {
    // TODO: implementar mi_atomic_fetch_add_explicit desde cpython/atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_fetch_sub_explicit(param_15105: *mut core::ffi::c_void) -> static inline uintptr_t {
    // TODO: implementar mi_atomic_fetch_sub_explicit desde cpython/atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_fetch_and_explicit(param_15105: *mut core::ffi::c_void) -> static inline uintptr_t {
    // TODO: implementar mi_atomic_fetch_and_explicit desde cpython/atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_fetch_or_explicit(param_15105: *mut core::ffi::c_void) -> static inline uintptr_t {
    // TODO: implementar mi_atomic_fetch_or_explicit desde cpython/atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_compare_exchange_strong_explicit(param_15105: *mut core::ffi::c_void) -> static inline bool {
    // TODO: implementar mi_atomic_compare_exchange_strong_explicit desde cpython/atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_exchange_explicit(param_15105: *mut core::ffi::c_void) -> static inline uintptr_t {
    // TODO: implementar mi_atomic_exchange_explicit desde cpython/atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_load_explicit(param_15105: *mut core::ffi::c_void) -> static inline uintptr_t {
    // TODO: implementar mi_atomic_load_explicit desde cpython/atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_store_explicit(param_15105: *mut core::ffi::c_void) -> static inline void {
    // TODO: implementar mi_atomic_store_explicit desde cpython/atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_loadi64_explicit(param_62577: *mut core::ffi::c_void) -> static inline int64_t {
    // TODO: implementar mi_atomic_loadi64_explicit desde cpython/atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_storei64_explicit(param_62577: *mut core::ffi::c_void) -> static inline void {
    // TODO: implementar mi_atomic_storei64_explicit desde cpython/atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _mi_is_power_of_two(x: usize) -> static inline bool {
    // TODO: implementar _mi_is_power_of_two desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_acos_impl(module: *mut PyObject, z: Py_complex) -> static Py_complex {
    // TODO: implementar cmath_acos_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_acos(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_acos desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_acosh_impl(module: *mut PyObject, z: Py_complex) -> static Py_complex {
    // TODO: implementar cmath_acosh_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_acosh(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_acosh desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_asin_impl(module: *mut PyObject, z: Py_complex) -> static Py_complex {
    // TODO: implementar cmath_asin_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_asin(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_asin desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_asinh_impl(module: *mut PyObject, z: Py_complex) -> static Py_complex {
    // TODO: implementar cmath_asinh_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_asinh(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_asinh desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_atan_impl(module: *mut PyObject, z: Py_complex) -> static Py_complex {
    // TODO: implementar cmath_atan_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_atan(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_atan desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_atanh_impl(module: *mut PyObject, z: Py_complex) -> static Py_complex {
    // TODO: implementar cmath_atanh_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_atanh(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_atanh desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_cos_impl(module: *mut PyObject, z: Py_complex) -> static Py_complex {
    // TODO: implementar cmath_cos_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_cos(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_cos desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_cosh_impl(module: *mut PyObject, z: Py_complex) -> static Py_complex {
    // TODO: implementar cmath_cosh_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_cosh(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_cosh desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_exp_impl(module: *mut PyObject, z: Py_complex) -> static Py_complex {
    // TODO: implementar cmath_exp_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_exp(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_exp desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_log10_impl(module: *mut PyObject, z: Py_complex) -> static Py_complex {
    // TODO: implementar cmath_log10_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_log10(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_log10 desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_sin_impl(module: *mut PyObject, z: Py_complex) -> static Py_complex {
    // TODO: implementar cmath_sin_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_sin(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_sin desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_sinh_impl(module: *mut PyObject, z: Py_complex) -> static Py_complex {
    // TODO: implementar cmath_sinh_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_sinh(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_sinh desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_sqrt_impl(module: *mut PyObject, z: Py_complex) -> static Py_complex {
    // TODO: implementar cmath_sqrt_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_sqrt(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_sqrt desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_tan_impl(module: *mut PyObject, z: Py_complex) -> static Py_complex {
    // TODO: implementar cmath_tan_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_tan(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_tan desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_tanh_impl(module: *mut PyObject, z: Py_complex) -> static Py_complex {
    // TODO: implementar cmath_tanh_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_tanh(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_tanh desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logarithm(e: base) -> returns the natural {
    // TODO: implementar logarithm desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_log_impl(module: *mut PyObject, x: Py_complex, y_obj: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_log_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_log(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar cmath_log desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_isinf_impl(module: *mut PyObject, z: Py_complex) -> *mut static PyObject {
    // TODO: implementar cmath_isinf_impl desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmath_isinf(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar cmath_isinf desde cpython/cmathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn math_frexp_impl(module: *mut PyObject, x: f64) -> *mut static PyObject {
    // TODO: implementar math_frexp_impl desde cpython/mathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn math_frexp(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar math_frexp desde cpython/mathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn frexp() -> This is essentially the inverse of {
    // TODO: implementar frexp desde cpython/mathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn math_ldexp_impl(module: *mut PyObject, x: f64, i: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar math_ldexp_impl desde cpython/mathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn math_ldexp(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar math_ldexp desde cpython/mathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn math_pow_impl(module: *mut PyObject, x: f64, y: f64) -> *mut static PyObject {
    // TODO: implementar math_pow_impl desde cpython/mathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn math_pow(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar math_pow desde cpython/mathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn math_isinf_impl(module: *mut PyObject, x: f64) -> *mut static PyObject {
    // TODO: implementar math_isinf_impl desde cpython/mathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn math_isinf(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar math_isinf desde cpython/mathmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_login_tty_impl(module: *mut PyObject, fd: i32) -> *mut static PyObject {
    // TODO: implementar os_login_tty_impl desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_login_tty(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar os_login_tty desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_getlogin_impl(module: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar os_getlogin_impl desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os_getlogin(module: *mut PyObject, param_45657: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar os_getlogin desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os__emscripten_log_impl(module: *mut PyObject, arg: *mut const char) -> *mut static PyObject {
    // TODO: implementar os__emscripten_log_impl desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn os__emscripten_log(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar os__emscripten_log desde cpython/posixmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetReparseDeferralEnabled_impl(self: *mut xmlparseobject, enabled: i32) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_SetReparseDeferralEnabled_impl desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetReparseDeferralEnabled(self: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_SetReparseDeferralEnabled desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_GetReparseDeferralEnabled_impl(self: *mut xmlparseobject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_GetReparseDeferralEnabled_impl desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_GetReparseDeferralEnabled(self: *mut PyObject, param_45657: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_GetReparseDeferralEnabled desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_Parse_impl(self: *mut xmlparseobject, cls: *mut PyTypeObject, data: *mut PyObject, isfinal: i32) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_Parse_impl desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_Parse(self: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_Parse desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_ParseFile_impl(self: *mut xmlparseobject, cls: *mut PyTypeObject, file: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_ParseFile_impl desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_ParseFile(self: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_ParseFile desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetBase_impl(self: *mut xmlparseobject, base: *mut const char) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_SetBase_impl desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetBase(self: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_SetBase desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_GetBase_impl(self: *mut xmlparseobject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_GetBase_impl desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_GetBase(self: *mut PyObject, param_45657: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_GetBase desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_GetInputContext_impl(self: *mut xmlparseobject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_GetInputContext_impl desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_GetInputContext(self: *mut PyObject, param_45657: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_GetInputContext desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_ExternalEntityParserCreate_impl(self: *mut xmlparseobject, cls: *mut PyTypeObject, context: *mut const char, encoding: *mut const char) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_ExternalEntityParserCreate_impl desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_ExternalEntityParserCreate(self: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_ExternalEntityParserCreate desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetParamEntityParsing_impl(self: *mut xmlparseobject, flag: i32) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_SetParamEntityParsing_impl desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetParamEntityParsing(self: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_SetParamEntityParsing desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_UseForeignDTD_impl(self: *mut xmlparseobject, cls: *mut PyTypeObject, flag: i32) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_UseForeignDTD_impl desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_UseForeignDTD(self: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_UseForeignDTD desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetBillionLaughsAttackProtectionActivationThreshold_impl(self: *mut xmlparseobject, cls: *mut PyTypeObject, threshold: u64) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_SetBillionLaughsAttackProtectionActivationThreshold_impl desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetBillionLaughsAttackProtectionActivationThreshold(self: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_SetBillionLaughsAttackProtectionActivationThreshold desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetBillionLaughsAttackProtectionMaximumAmplification_impl(self: *mut xmlparseobject, cls: *mut PyTypeObject, max_factor: f32) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_SetBillionLaughsAttackProtectionMaximumAmplification_impl desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_xmlparser_SetBillionLaughsAttackProtectionMaximumAmplification(self: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_xmlparser_SetBillionLaughsAttackProtectionMaximumAmplification desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_ParserCreate_impl(module: *mut PyObject, encoding: *mut const char, namespace_separator: *mut const char, intern: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_ParserCreate_impl desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_ParserCreate(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_ParserCreate desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_ErrorString_impl(module: *mut PyObject, code: i64) -> *mut static PyObject {
    // TODO: implementar pyexpat_ErrorString_impl desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_ErrorString(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar pyexpat_ErrorString desde cpython/pyexpat.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn syslog_openlog_impl(module: *mut PyObject, ident: *mut PyObject, logopt: i64, facility: i64) -> *mut static PyObject {
    // TODO: implementar syslog_openlog_impl desde cpython/syslogmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn syslog_openlog(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar syslog_openlog desde cpython/syslogmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn syslog_syslog_impl(module: *mut PyObject, group_left_1: i32, priority: i32, message: *mut const char) -> *mut static PyObject {
    // TODO: implementar syslog_syslog_impl desde cpython/syslogmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn syslog_syslog(module: *mut PyObject, args: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar syslog_syslog desde cpython/syslogmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn syslog_closelog_impl(module: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar syslog_closelog_impl desde cpython/syslogmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn syslog_closelog(module: *mut PyObject, param_45657: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar syslog_closelog desde cpython/syslogmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn syslog_setlogmask_impl(module: *mut PyObject, maskpri: i64) -> static long {
    // TODO: implementar syslog_setlogmask_impl desde cpython/syslogmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn syslog_setlogmask(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar syslog_setlogmask desde cpython/syslogmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn syslog_LOG_MASK_impl(module: *mut PyObject, pri: i64) -> static long {
    // TODO: implementar syslog_LOG_MASK_impl desde cpython/syslogmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn syslog_LOG_MASK(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar syslog_LOG_MASK desde cpython/syslogmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn syslog_LOG_UPTO_impl(module: *mut PyObject, pri: i64) -> static long {
    // TODO: implementar syslog_LOG_UPTO_impl desde cpython/syslogmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn syslog_LOG_UPTO(module: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar syslog_LOG_UPTO desde cpython/syslogmodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _abc__abc_instancecheck_impl(module: *mut PyObject, self: *mut PyObject, instance: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _abc__abc_instancecheck_impl desde cpython/_abc.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _abc__abc_instancecheck(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _abc__abc_instancecheck desde cpython/_abc.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__log_traceback_get_impl(self: *mut FutureObj) -> *mut static PyObject {
    // TODO: implementar _asyncio_Future__log_traceback_get_impl desde cpython/_asynciomodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__log_traceback_get(self: *mut PyObject, param_64866: *mut core::ffi::c_void) -> *mut static PyObject {
    // TODO: implementar _asyncio_Future__log_traceback_get desde cpython/_asynciomodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__log_traceback_set_impl(self: *mut FutureObj, value: *mut PyObject) -> static int {
    // TODO: implementar _asyncio_Future__log_traceback_set_impl desde cpython/_asynciomodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Future__log_traceback_set(self: *mut PyObject, value: *mut PyObject, param_64866: *mut core::ffi::c_void) -> static int {
    // TODO: implementar _asyncio_Future__log_traceback_set desde cpython/_asynciomodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Task__log_destroy_pending_get_impl(self: *mut TaskObj) -> *mut static PyObject {
    // TODO: implementar _asyncio_Task__log_destroy_pending_get_impl desde cpython/_asynciomodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Task__log_destroy_pending_get(self: *mut PyObject, param_64866: *mut core::ffi::c_void) -> *mut static PyObject {
    // TODO: implementar _asyncio_Task__log_destroy_pending_get desde cpython/_asynciomodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Task__log_destroy_pending_set_impl(self: *mut TaskObj, value: *mut PyObject) -> static int {
    // TODO: implementar _asyncio_Task__log_destroy_pending_set_impl desde cpython/_asynciomodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _asyncio_Task__log_destroy_pending_set(self: *mut PyObject, value: *mut PyObject, param_64866: *mut core::ffi::c_void) -> static int {
    // TODO: implementar _asyncio_Task__log_destroy_pending_set desde cpython/_asynciomodule.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _hashlib_hmac_singleshot_impl(module: *mut PyObject, key: *mut Py_buffer, msg: *mut Py_buffer, digest: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _hashlib_hmac_singleshot_impl desde cpython/_hashopenssl.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _hashlib_hmac_singleshot(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _hashlib_hmac_singleshot desde cpython/_hashopenssl.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _operator_pow_impl(module: *mut PyObject, a: *mut PyObject, b: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _operator_pow_impl desde cpython/_operator.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _operator_pow(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _operator_pow desde cpython/_operator.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _operator_ipow_impl(module: *mut PyObject, a: *mut PyObject, b: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _operator_ipow_impl desde cpython/_operator.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _operator_ipow(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _operator_ipow desde cpython/_operator.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprstring_impl(self: *mut TkappObject, s: *mut const char) -> *mut static PyObject {
    // TODO: implementar _tkinter_tkapp_exprstring_impl desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprstring(self: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _tkinter_tkapp_exprstring desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprlong_impl(self: *mut TkappObject, s: *mut const char) -> *mut static PyObject {
    // TODO: implementar _tkinter_tkapp_exprlong_impl desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprlong(self: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _tkinter_tkapp_exprlong desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprdouble_impl(self: *mut TkappObject, s: *mut const char) -> *mut static PyObject {
    // TODO: implementar _tkinter_tkapp_exprdouble_impl desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprdouble(self: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _tkinter_tkapp_exprdouble desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprboolean_impl(self: *mut TkappObject, s: *mut const char) -> *mut static PyObject {
    // TODO: implementar _tkinter_tkapp_exprboolean_impl desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _tkinter_tkapp_exprboolean(self: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _tkinter_tkapp_exprboolean desde cpython/_tkinter.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_WaitForSingleObject_impl(module: *mut PyObject, handle: *mut core::ffi::c_void, milliseconds: u32) -> static long {
    // TODO: implementar _winapi_WaitForSingleObject_impl desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _winapi_WaitForSingleObject(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _winapi_WaitForSingleObject desde cpython/_winapi.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn testingAccountingGetCountBytesIndirect(parser: XML_Parser) -> u64 {
    // TODO: implementar testingAccountingGetCountBytesIndirect desde cpython/internal.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn XmlPrologStateInit(state: *mut PROLOG_STATE) -> core::ffi::c_void {
    // TODO: implementar XmlPrologStateInit desde cpython/xmlrole.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn XmlPrologStateInitExternalEntity(state: *mut PROLOG_STATE) -> core::ffi::c_void {
    // TODO: implementar XmlPrologStateInitExternalEntity desde cpython/xmlrole.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _ctypes_simple_instance(st: *mut ctypes_state, obj: *mut PyObject) -> extern int {
    // TODO: implementar _ctypes_simple_instance desde cpython/ctypes.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn process_single_task_node(unwinder: *mut RemoteUnwinderObject, task_addr: usize, param_45657: *mut PyObject, result: *mut PyObject) -> extern int {
    // TODO: implementar process_single_task_node desde cpython/_remote_debugging.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _OutputBuffer_InitAndGrow(buffer: *mut _BlocksOutputBuffer, ob: *mut ZSTD_outBuffer, max_length: Py_ssize_t) -> static inline int {
    // TODO: implementar _OutputBuffer_InitAndGrow desde cpython/buffer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_exp_impl(self: *mut PyObject, cls: *mut PyTypeObject, context: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_exp_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_exp(self: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_exp desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_log10_impl(self: *mut PyObject, cls: *mut PyTypeObject, context: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_log10_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_log10(self: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_log10 desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_sqrt_impl(self: *mut PyObject, cls: *mut PyTypeObject, context: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_sqrt_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_sqrt(self: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_sqrt desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn exponent(param_16844: exp + digits -) -> Return the adjusted {
    // TODO: implementar exponent desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_invert_impl(self: *mut PyObject, cls: *mut PyTypeObject, context: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_logical_invert_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_invert(self: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_logical_invert desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logb_impl(self: *mut PyObject, cls: *mut PyTypeObject, context: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_logb_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logb(self: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_logb desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_and_impl(self: *mut PyObject, cls: *mut PyTypeObject, other: *mut PyObject, context: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_logical_and_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_and(self: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_logical_and desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_or_impl(self: *mut PyObject, cls: *mut PyTypeObject, other: *mut PyObject, context: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_logical_or_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_or(self: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_logical_or desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_xor_impl(self: *mut PyObject, cls: *mut PyTypeObject, other: *mut PyObject, context: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_logical_xor_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Decimal_logical_xor(self: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Decimal_logical_xor desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_exp_impl(context: *mut PyObject, cls: *mut PyTypeObject, x: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_exp_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_exp(context: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_exp desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_log10_impl(context: *mut PyObject, cls: *mut PyTypeObject, x: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_log10_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_log10(context: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_log10 desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_sqrt_impl(context: *mut PyObject, cls: *mut PyTypeObject, x: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_sqrt_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_sqrt(context: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_sqrt desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_power_impl(context: *mut PyObject, cls: *mut PyTypeObject, base: *mut PyObject, exp: *mut PyObject, mod: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_power_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_power(context: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_power desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logb_impl(context: *mut PyObject, cls: *mut PyTypeObject, x: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_logb_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logb(context: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_logb desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_invert_impl(context: *mut PyObject, cls: *mut PyTypeObject, x: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_logical_invert_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_invert(context: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_logical_invert desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_and_impl(context: *mut PyObject, cls: *mut PyTypeObject, x: *mut PyObject, y: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_logical_and_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_and(context: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_logical_and desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_or_impl(context: *mut PyObject, cls: *mut PyTypeObject, x: *mut PyObject, y: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_logical_or_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_or(context: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_logical_or desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_xor_impl(context: *mut PyObject, cls: *mut PyTypeObject, x: *mut PyObject, y: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_logical_xor_impl desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _decimal_Context_logical_xor(context: *mut PyObject, cls: *mut PyTypeObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t, kwnames: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar _decimal_Context_logical_xor desde cpython/_decimal.c.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _mpd_singlemul(u: mpd_uint_t, v: mpd_uint_t) -> static inline void {
    // TODO: implementar _mpd_singlemul desde cpython/basearith.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ispower2(n: mpd_size_t) -> static inline int {
    // TODO: implementar ispower2 desde cpython/bits.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cosh(param_65403: x_minus_one) -> *mut  {
    // TODO: implementar cosh desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sinh(param_65403: x_minus_one) -> *mut  {
    // TODO: implementar sinh desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn c_log(z: Py_complex) -> static Py_complex {
    // TODO: implementar c_log desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cos(param_56674: phi) -> *mut r {
    // TODO: implementar cos desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sin(param_56674: phi) -> *mut r {
    // TODO: implementar sin desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Py_GetConstant(param_52710: Py_CONSTANT_EMPTY_STR) -> return {
    // TODO: implementar Py_GetConstant desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _approximate_isqrt(n: u64) -> static inline uint32_t {
    // TODO: implementar _approximate_isqrt desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn math_integer_isqrt(module: *mut PyObject, n: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar math_integer_isqrt desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn m_sinpi(x: f64) -> static double {
    // TODO: implementar m_sinpi desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn exp(param_49451: y) -> *mut absx {
    // TODO: implementar exp desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn m_log(x: f64) -> static double {
    // TODO: implementar m_log desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn m_log2(x: f64) -> static double {
    // TODO: implementar m_log2 desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn log2(param_52739: x) -> return {
    // TODO: implementar log2 desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn m_log10(x: f64) -> static double {
    // TODO: implementar m_log10 desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn log10(param_52739: x) -> return {
    // TODO: implementar log10 desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn loghelper_int(arg: *mut PyObject, param_63078: f64) -> *mut static PyObject {
    // TODO: implementar loghelper_int desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn loghelper(arg: *mut PyObject, param_63078: f64) -> *mut static PyObject {
    // TODO: implementar loghelper desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn math_log(module: *mut PyObject, args: *mut core::ffi::c_void, nargs: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar math_log desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn math_log2(module: *mut PyObject, x: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar math_log2 desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn math_log10(module: *mut PyObject, x: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar math_log10 desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyUnicode_DecodeFSDefaultAndSize(param_16197: buffer, param_59684: length) -> return {
    // TODO: implementar PyUnicode_DecodeFSDefaultAndSize desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn HINSTANCE(param_59592: *Py_ShellExecuteW) -> static {
    // TODO: implementar HINSTANCE desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_get_state(module: *mut PyObject) -> *mut static inline pyexpat_state {
    // TODO: implementar pyexpat_get_state desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn invalid_expat_handler_rv(name: *mut const char) -> static inline void {
    // TODO: implementar invalid_expat_handler_rv desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_capsule_destructor(capsule: *mut PyObject) -> static void {
    // TODO: implementar pyexpat_capsule_destructor desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_exec(mod: *mut PyObject) -> static int {
    // TODO: implementar pyexpat_exec desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_traverse(module: *mut PyObject, visit: visitproc, arg: *mut core::ffi::c_void) -> static int {
    // TODO: implementar pyexpat_traverse desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pyexpat_clear(module: *mut PyObject) -> static int {
    // TODO: implementar pyexpat_clear desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit_pyexpat() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit_pyexpat desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn signal_add_constants(module: *mut PyObject) -> static int {
    // TODO: implementar signal_add_constants desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn symtable_init_constants(m: *mut PyObject) -> static int {
    // TODO: implementar symtable_init_constants desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn syslog_get_argv() -> *mut static PyObject {
    // TODO: implementar syslog_get_argv desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LOG_MASK(param_10706: pri) -> return {
    // TODO: implementar LOG_MASK desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn LOG_UPTO(param_10706: pri) -> return {
    // TODO: implementar LOG_UPTO desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn syslog_exec(module: *mut PyObject) -> static int {
    // TODO: implementar syslog_exec desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit_syslog() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit_syslog desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Xxo_get_x_exports(op: *mut PyObject, param_64866: *mut core::ffi::c_void) -> *mut static PyObject {
    // TODO: implementar Xxo_get_x_exports desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OutputBuffer_InitAndGrow(buffer: *mut _BlocksOutputBuffer, max_length: Py_ssize_t, param_55199: *mut Bytef, avail_out: *mut u32) -> static inline Py_ssize_t {
    // TODO: implementar OutputBuffer_InitAndGrow desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn defdict_missing(op: *mut PyObject, key: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar defdict_missing desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn __missing__() -> Factory for default value called by {
    // TODO: implementar __missing__ desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn expat_set_error(st: *mut elementtreestate, error_code: enum XML_Error, line: Py_ssize_t, column: Py_ssize_t, message: *mut const char) -> static void {
    // TODO: implementar expat_set_error desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn expat_default_handler(op: *mut core::ffi::c_void, data_in: *mut const XML_Char, data_len: i32) -> static void {
    // TODO: implementar expat_default_handler desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn expat_start_handler(op: *mut core::ffi::c_void, tag_in: *mut const XML_Char, param_11329: *mut const XML_Char) -> static void {
    // TODO: implementar expat_start_handler desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn expat_data_handler(op: *mut core::ffi::c_void, data_in: *mut const XML_Char, data_len: i32) -> static void {
    // TODO: implementar expat_data_handler desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn expat_end_handler(op: *mut core::ffi::c_void, tag_in: *mut const XML_Char) -> static void {
    // TODO: implementar expat_end_handler desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn expat_start_ns_handler(op: *mut core::ffi::c_void, prefix_in: *mut const XML_Char, uri_in: *mut const XML_Char) -> static void {
    // TODO: implementar expat_start_ns_handler desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn expat_end_ns_handler(op: *mut core::ffi::c_void, prefix_in: *mut const XML_Char) -> static void {
    // TODO: implementar expat_end_ns_handler desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn expat_comment_handler(op: *mut core::ffi::c_void, comment_in: *mut const XML_Char) -> static void {
    // TODO: implementar expat_comment_handler desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn expat_start_doctype_handler(op: *mut core::ffi::c_void, doctype_name: *mut const XML_Char, sysid: *mut const XML_Char, pubid: *mut const XML_Char, has_internal_subset: i32) -> static void {
    // TODO: implementar expat_start_doctype_handler desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn expat_pi_handler(op: *mut core::ffi::c_void, target_in: *mut const XML_Char, data_in: *mut const XML_Char) -> static void {
    // TODO: implementar expat_pi_handler desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn expat_parse(param_29947: st, param_2027: self, param_29515: data_ptr, param_15879: *mut core::ffi::c_void) -> return {
    // TODO: implementar expat_parse desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn hashlib_constants(module: *mut PyObject) -> static int {
    // TODO: implementar hashlib_constants desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _waiting_finish_releasing(waiting: *mut _waiting_t) -> static void {
    // TODO: implementar _waiting_finish_releasing desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _channel_clear_closing(param_36873: *mut struct _channel) -> static void {
    // TODO: implementar _channel_clear_closing desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _channel_finish_closing(param_36873: *mut struct _channel) -> static void {
    // TODO: implementar _channel_finish_closing desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _channel_set_closing(param_42765: *mut _channelref, param_27216: PyThread_type_lock) -> static int {
    // TODO: implementar _channel_set_closing desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn _parse_constant(s: *mut PyScannerObject, constant: *mut const char, idx: Py_ssize_t, next_idx_ptr: *mut Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar _parse_constant desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn module_add_int_constant(m: *mut PyObject, name: *mut const char, value: i64) -> static int {
    // TODO: implementar module_add_int_constant desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyNumber_Power(param_35910: a, param_3235: b, param_44205: Py_None) -> return {
    // TODO: implementar PyNumber_Power desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyNumber_InPlacePower(param_35910: a, param_3235: b, param_44205: Py_None) -> return {
    // TODO: implementar PyNumber_InPlacePower desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn save_singleton_type(state: *mut PickleState, self: *mut PicklerObject, obj: *mut PyObject, singleton: *mut PyObject) -> static int {
    // TODO: implementar save_singleton_type desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn instantiate(cls: *mut PyObject, args: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar instantiate desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ADD_AD_CONSTANT(param_59786: CLOSE_NOTIFY) -> SSL_AD_ {
    // TODO: implementar ADD_AD_CONSTANT desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn pack_single(ptr: *mut i8, item: *mut PyObject, fmt: *mut const char, itemsize: Py_ssize_t) -> static int {
    // TODO: implementar pack_single desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn unpack_single(ptr: *mut i8, fmt: *mut const char, itemsize: Py_ssize_t) -> *mut static PyObject {
    // TODO: implementar unpack_single desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_lazy_hash_inheritance(self: *mut PyObject, param_45657: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar test_lazy_hash_inheritance desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn argparsing(o: *mut PyObject, args: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar argparsing desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ipowType_ipow(self: *mut PyObject, other: *mut PyObject, mod: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar ipowType_ipow desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn check_edit_cost(a: *mut const char, b: *mut const char, expected: Py_ssize_t) -> static int {
    // TODO: implementar check_edit_cost desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_edit_cost(self: *mut PyObject, param_45657: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar test_edit_cost desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn get_crossinterp_data(self: *mut PyObject, args: *mut PyObject, kwargs: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar get_crossinterp_data desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn restore_crossinterp_data(self: *mut PyObject, args: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar restore_crossinterp_data desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn testexport_foo(self: *mut PyObject, args: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar testexport_foo desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_export_uninitialized() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testmultiphase_export_uninitialized desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_export_null() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testmultiphase_export_null desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_export_raise() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testmultiphase_export_raise desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testmultiphase_export_unreported_exception() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testmultiphase_export_unreported_exception desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyModule_AddIntConstant(param_57352: m, param_4655: 1) -> return {
    // TODO: implementar PyModule_AddIntConstant desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyModExport__test_from_modexport_null() -> PyMODEXPORT_FUNC {
    // TODO: implementar PyModExport__test_from_modexport_null desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyModInit__test_from_modexport_null() -> PyMODINIT_FUNC {
    // TODO: implementar PyModInit__test_from_modexport_null desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyModExport__test_from_modexport_exception() -> PyMODEXPORT_FUNC {
    // TODO: implementar PyModExport__test_from_modexport_exception desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyModInit__test_from_modexport_exception() -> PyMODINIT_FUNC {
    // TODO: implementar PyModInit__test_from_modexport_exception desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn modexport_create_string(spec: *mut PyObject, def: *mut PyModuleDef) -> *mut static PyObject {
    // TODO: implementar modexport_create_string desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyModExport__test_from_modexport_empty_slots() -> PyMODEXPORT_FUNC {
    // TODO: implementar PyModExport__test_from_modexport_empty_slots desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyModExport__test_from_modexport_minimal_slots() -> PyMODEXPORT_FUNC {
    // TODO: implementar PyModExport__test_from_modexport_minimal_slots desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn modexport_smoke_exec(mod: *mut PyObject) -> static int {
    // TODO: implementar modexport_smoke_exec desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn modexport_smoke_get_state_int(mod: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar modexport_smoke_get_state_int desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn modexport_smoke_get_test_token(mod: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar modexport_smoke_get_test_token desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn modexport_get_minimal_slots(mod: *mut PyObject, arg: *mut PyObject) -> *mut static PyObject {
    // TODO: implementar modexport_get_minimal_slots desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn init__testsinglephase_basic(def: *mut PyModuleDef) -> *mut static PyObject {
    // TODO: implementar init__testsinglephase_basic desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testsinglephase desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase_basic_wrapper() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testsinglephase_basic_wrapper desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase_with_reinit() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testsinglephase_with_reinit desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase_with_state() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testsinglephase_with_state desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase_check_cache_first() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testsinglephase_check_cache_first desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase_with_reinit_check_cache_first() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testsinglephase_with_reinit_check_cache_first desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase_with_state_check_cache_first() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testsinglephase_with_state_check_cache_first desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PyInit__testsinglephase_raise_exception() -> PyMODINIT_FUNC {
    // TODO: implementar PyInit__testsinglephase_raise_exception desde cpython/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IAGetPrimitiveTopology(pTopology: *mut D3D10_PRIMITIVE_TOPOLOGY) -> void STDMETHODCALLTYPE {
    // TODO: implementar IAGetPrimitiveTopology desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn VSSetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void) -> void STDMETHODCALLTYPE {
    // TODO: implementar VSSetConstantBuffers desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn VSGetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void) -> void STDMETHODCALLTYPE {
    // TODO: implementar VSGetConstantBuffers desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GSSetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void) -> void STDMETHODCALLTYPE {
    // TODO: implementar GSSetConstantBuffers desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GSGetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void) -> void STDMETHODCALLTYPE {
    // TODO: implementar GSGetConstantBuffers desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PSSetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void) -> void STDMETHODCALLTYPE {
    // TODO: implementar PSSetConstantBuffers desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PSGetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void) -> void STDMETHODCALLTYPE {
    // TODO: implementar PSGetConstantBuffers desde dxvk/d3d10_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DecodeLogicOp(LogicOp: D3D11_LOGIC_OP) -> static VkLogicOp {
    // TODO: implementar DecodeLogicOp desde dxvk/d3d11_blend.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ValidateLogicOp(LogicOp: D3D11_LOGIC_OP) -> static bool {
    // TODO: implementar ValidateLogicOp desde dxvk/d3d11_blend.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn EncodeInstanceData(SlotId: u32, pInstance: *mut D3D11ClassInstance) -> D3D11InstanceData {
    // TODO: implementar EncodeInstanceData desde dxvk/d3d11_class_linkage.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetInstanceName(pInstanceName: LPSTR, pBufferLength: *mut SIZE_T) -> void STDMETHODCALLTYPE {
    // TODO: implementar GetInstanceName desde dxvk/d3d11_class_linkage.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateClassInstance(pTypeName: LPCSTR, ConstantBufferOffset: UINT, ConstantVectorOffset: UINT, TextureOffset: UINT, SamplerOffset: UINT, param_10621: *mut ID3D11ClassInstance) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateClassInstance desde dxvk/d3d11_class_linkage.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetClassInstance(pInstanceName: LPCSTR, InstanceIndex: UINT, param_10621: *mut ID3D11ClassInstance) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetClassInstance desde dxvk/d3d11_class_linkage.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AddInstance(pDesc: *mut const D3D11_CLASS_INSTANCE_DESC, pTypeName: LPCSTR, pInstanceName: LPCSTR) -> core::ffi::c_void {
    // TODO: implementar AddInstance desde dxvk/d3d11_class_linkage.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DrawIndexedInstancedIndirect(pBufferForArgs: *mut ID3D11Buffer, AlignedByteOffsetForArgs: UINT) -> void STDMETHODCALLTYPE {
    // TODO: implementar DrawIndexedInstancedIndirect desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DrawInstancedIndirect(pBufferForArgs: *mut ID3D11Buffer, AlignedByteOffsetForArgs: UINT) -> void STDMETHODCALLTYPE {
    // TODO: implementar DrawInstancedIndirect desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn VSSetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void, pFirstConstant: *mut const UINT, pNumConstants: *mut const UINT) -> void STDMETHODCALLTYPE {
    // TODO: implementar VSSetConstantBuffers1 desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn VSGetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> void STDMETHODCALLTYPE {
    // TODO: implementar VSGetConstantBuffers1 desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn HSSetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void) -> void STDMETHODCALLTYPE {
    // TODO: implementar HSSetConstantBuffers desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn HSSetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void, pFirstConstant: *mut const UINT, pNumConstants: *mut const UINT) -> void STDMETHODCALLTYPE {
    // TODO: implementar HSSetConstantBuffers1 desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn HSGetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void) -> void STDMETHODCALLTYPE {
    // TODO: implementar HSGetConstantBuffers desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn HSGetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> void STDMETHODCALLTYPE {
    // TODO: implementar HSGetConstantBuffers1 desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DSSetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void) -> void STDMETHODCALLTYPE {
    // TODO: implementar DSSetConstantBuffers desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DSSetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void, pFirstConstant: *mut const UINT, pNumConstants: *mut const UINT) -> void STDMETHODCALLTYPE {
    // TODO: implementar DSSetConstantBuffers1 desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DSGetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void) -> void STDMETHODCALLTYPE {
    // TODO: implementar DSGetConstantBuffers desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DSGetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> void STDMETHODCALLTYPE {
    // TODO: implementar DSGetConstantBuffers1 desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GSSetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void, pFirstConstant: *mut const UINT, pNumConstants: *mut const UINT) -> void STDMETHODCALLTYPE {
    // TODO: implementar GSSetConstantBuffers1 desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GSGetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> void STDMETHODCALLTYPE {
    // TODO: implementar GSGetConstantBuffers1 desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PSSetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void, pFirstConstant: *mut const UINT, pNumConstants: *mut const UINT) -> void STDMETHODCALLTYPE {
    // TODO: implementar PSSetConstantBuffers1 desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PSGetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> void STDMETHODCALLTYPE {
    // TODO: implementar PSGetConstantBuffers1 desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CSSetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void) -> void STDMETHODCALLTYPE {
    // TODO: implementar CSSetConstantBuffers desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CSSetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void, pFirstConstant: *mut const UINT, pNumConstants: *mut const UINT) -> void STDMETHODCALLTYPE {
    // TODO: implementar CSSetConstantBuffers1 desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CSGetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void) -> void STDMETHODCALLTYPE {
    // TODO: implementar CSGetConstantBuffers desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CSGetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> void STDMETHODCALLTYPE {
    // TODO: implementar CSGetConstantBuffers1 desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ApplyDirtyConstantBuffers(Stage: D3D11ShaderType, BoundMask: const D3D11BindingMask&, DirtyMask: D3D11BindingMask&) -> core::ffi::c_void {
    // TODO: implementar ApplyDirtyConstantBuffers desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ApplyPrimitiveTopology() -> core::ffi::c_void {
    // TODO: implementar ApplyPrimitiveTopology desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BindConstantBuffer(ShaderStage: D3D11ShaderType, Slot: UINT, pBuffer: *mut D3D11Buffer, Offset: UINT, Length: UINT) -> core::ffi::c_void {
    // TODO: implementar BindConstantBuffer desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BindConstantBufferRange(ShaderStage: D3D11ShaderType, Slot: UINT, Offset: UINT, Length: UINT) -> core::ffi::c_void {
    // TODO: implementar BindConstantBufferRange desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DirtyConstantBuffer(ShaderStage: D3D11ShaderType, Slot: u32, IsNull: bool) -> bool {
    // TODO: implementar DirtyConstantBuffer desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void, pFirstConstant: *mut UINT, pNumConstants: *mut UINT) -> core::ffi::c_void {
    // TODO: implementar GetConstantBuffers desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RestoreConstantBuffers(Stage: D3D11ShaderType) -> core::ffi::c_void {
    // TODO: implementar RestoreConstantBuffers desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetConstantBuffers(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void) -> core::ffi::c_void {
    // TODO: implementar SetConstantBuffers desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetConstantBuffers1(StartSlot: UINT, NumBuffers: UINT, ppConstantBuffers: *mut core::ffi::c_void, pFirstConstant: *mut const UINT, pNumConstants: *mut const UINT) -> core::ffi::c_void {
    // TODO: implementar SetConstantBuffers1 desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetClassInstances(pShader: *mut const D3D11CommonShader, ppClassInstances: *mut core::ffi::c_void, NumClassInstances: UINT) -> core::ffi::c_void {
    // TODO: implementar SetClassInstances desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetClassInstances(ppClassInstances: *mut core::ffi::c_void, pNumClassInstances: *mut UINT) -> core::ffi::c_void {
    // TODO: implementar GetClassInstances desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AddCost(Value: u64) -> force_inline void {
    // TODO: implementar AddCost desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitDefaultPrimitiveTopology() -> static DxvkInputAssemblyState {
    // TODO: implementar InitDefaultPrimitiveTopology desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitDefaultLogicOpState() -> static DxvkLogicOpState {
    // TODO: implementar InitDefaultLogicOpState desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn constexpr(AllowFlush: !IsDeferred &&) -> if {
    // TODO: implementar constexpr desde dxvk/d3d11_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetInstanceExtensions(pExtensionCount: *mut UINT, ppExtensions: *mut core::ffi::c_void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetInstanceExtensions desde dxvk/d3d11_on_12_interfaces.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetClassInstanceData(Slot: u32, pClassInstance: *mut D3D11ClassInstance) -> D3D11InstanceData {
    // TODO: implementar GetClassInstanceData desde dxvk/d3d11_shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExportImageInfo() -> core::ffi::c_void {
    // TODO: implementar ExportImageInfo desde dxvk/d3d11_texture.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorSetStreamAutoProcessingMode(pVideoProcessor: *mut ID3D11VideoProcessor, StreamIndex: UINT, Enable: i32) -> void STDMETHODCALLTYPE {
    // TODO: implementar VideoProcessorSetStreamAutoProcessingMode desde dxvk/d3d11_video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn VideoProcessorGetStreamAutoProcessingMode(pVideoProcessor: *mut ID3D11VideoProcessor, StreamIndex: UINT, pEnabled: *mut i32) -> void STDMETHODCALLTYPE {
    // TODO: implementar VideoProcessorGetStreamAutoProcessingMode desde dxvk/d3d11_video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetVertexShaderConstant(StartRegister: u32, pConstantData: *mut const void, ConstantCount: u32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetVertexShaderConstant desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetVertexShaderConstant(Register: u32, pConstantData: *mut core::ffi::c_void, ConstantCount: u32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetVertexShaderConstant desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPixelShaderConstant(Register: u32, pConstantData: *mut core::ffi::c_void, ConstantCount: u32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetPixelShaderConstant desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetPixelShaderConstant(StartRegister: u32, pConstantData: *mut const void, ConstantCount: u32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetPixelShaderConstant desde dxvk/d3d8_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetSoftwareVertexProcessing(value: bool) -> inline HRESULT {
    // TODO: implementar SetSoftwareVertexProcessing desde dxvk/d3d8_state_block.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExposedMipLevels() -> u32 {
    // TODO: implementar ExposedMipLevels desde dxvk/d3d9_common_texture.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetDialogBoxMode(bEnableDialogs: i32) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetDialogBoxMode desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetSoftwareVertexProcessing() -> BOOL STDMETHODCALLTYPE {
    // TODO: implementar GetSoftwareVertexProcessing desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetVertexShaderConstantF(StartRegister: UINT, pConstantData: *mut const float, Vector4fCount: UINT) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetVertexShaderConstantF desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetVertexShaderConstantF(StartRegister: UINT, pConstantData: *mut f32, Vector4fCount: UINT) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetVertexShaderConstantF desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetVertexShaderConstantI(StartRegister: UINT, pConstantData: *mut const int, Vector4iCount: UINT) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetVertexShaderConstantI desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetVertexShaderConstantI(StartRegister: UINT, pConstantData: *mut i32, Vector4iCount: UINT) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetVertexShaderConstantI desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetVertexShaderConstantB(StartRegister: UINT, pConstantData: *mut const BOOL, BoolCount: UINT) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetVertexShaderConstantB desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetVertexShaderConstantB(StartRegister: UINT, pConstantData: *mut i32, BoolCount: UINT) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetVertexShaderConstantB desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetPixelShaderConstantF(StartRegister: UINT, pConstantData: *mut const float, Vector4fCount: UINT) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetPixelShaderConstantF desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPixelShaderConstantF(StartRegister: UINT, pConstantData: *mut f32, Vector4fCount: UINT) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetPixelShaderConstantF desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetPixelShaderConstantI(StartRegister: UINT, pConstantData: *mut const int, Vector4iCount: UINT) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetPixelShaderConstantI desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPixelShaderConstantI(StartRegister: UINT, pConstantData: *mut i32, Vector4iCount: UINT) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetPixelShaderConstantI desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetPixelShaderConstantB(StartRegister: UINT, pConstantData: *mut const BOOL, BoolCount: UINT) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar SetPixelShaderConstantB desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPixelShaderConstantB(StartRegister: UINT, pConstantData: *mut i32, BoolCount: UINT) -> HRESULT STDMETHODCALLTYPE {
    // TODO: implementar GetPixelShaderConstantB desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateConstantBuffers() -> core::ffi::c_void {
    // TODO: implementar CreateConstantBuffers desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BindViewportAndScissor() -> core::ffi::c_void {
    // TODO: implementar BindViewportAndScissor desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UploadSoftwareConstantSet(Src: const D3D9ShaderConstantsVSSoftware&, Layout: const D3D9ConstantLayout&) -> inline void {
    // TODO: implementar UploadSoftwareConstantSet desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CopySoftwareConstants(dstBuffer: D3D9ConstantBuffer&, src: *mut const void, size: u32) -> *mut inline void {
    // TODO: implementar CopySoftwareConstants desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UploadConstantSet(Src: const SoftwareLayoutType&, Layout: const D3D9ConstantLayout&, Shader: const ShaderType&) -> inline void {
    // TODO: implementar UploadConstantSet desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UploadConstants() -> core::ffi::c_void {
    // TODO: implementar UploadConstants desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn UpdatePushConstant(pData: *mut const void) -> core::ffi::c_void {
    // TODO: implementar UpdatePushConstant desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetInstanceCount() -> u32 {
    // TODO: implementar GetInstanceCount desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DetermineConstantLayouts(canSWVP: bool) -> core::ffi::c_void {
    // TODO: implementar DetermineConstantLayouts desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetShaderConstants(StartRegister: UINT, pConstantData: *mut const T, Count: UINT) -> i32 {
    // TODO: implementar SetShaderConstants desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BindSpecConstants() -> core::ffi::c_void {
    // TODO: implementar BindSpecConstants desde dxvk/d3d9_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetSharedConstants(spvModule: SpirvModule&) -> u32 {
    // TODO: implementar GetSharedConstants desde dxvk/d3d9_fixed_function.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetInstanceHandle(pInstance: *mut VkInstance) -> virtual void STDMETHODCALLTYPE {
    // TODO: implementar GetInstanceHandle desde dxvk/d3d9_interfaces.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getSpecConstantBufferSlot() -> static constexpr uint32_t {
    // TODO: implementar getSpecConstantBufferSlot desde dxvk/d3d9_shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetMaxDefinedFloatConstant() -> i32 {
    // TODO: implementar GetMaxDefinedFloatConstant desde dxvk/d3d9_shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetMaxDefinedIntConstant() -> i32 {
    // TODO: implementar GetMaxDefinedIntConstant desde dxvk/d3d9_shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetMaxDefinedBoolConstant() -> i32 {
    // TODO: implementar GetMaxDefinedBoolConstant desde dxvk/d3d9_shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetDXVKInstance() -> STDMETHODCALLTYPE {
    // TODO: implementar GetDXVKInstance desde dxvk/dxgi_adapter.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetVulkanInstance(pInstance: *mut VkInstance, ppfnVkGetInstanceProcAddr: *mut PFN_vkGetInstanceProcAddr) -> void STDMETHODCALLTYPE {
    // TODO: implementar GetVulkanInstance desde dxvk/dxgi_factory.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn processInstruction(ctx: const DxsoInstructionContext&) -> core::ffi::c_void {
    // TODO: implementar processInstruction desde dxvk/dxso_analysis.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn maxDefinedFloatConstant() -> i32 {
    // TODO: implementar maxDefinedFloatConstant desde dxvk/dxso_compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn maxDefinedIntConstant() -> i32 {
    // TODO: implementar maxDefinedIntConstant desde dxvk/dxso_compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn maxDefinedBoolConstant() -> i32 {
    // TODO: implementar maxDefinedBoolConstant desde dxvk/dxso_compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn emitDclSwvpConstantBuffer() -> i32 {
    // TODO: implementar emitDclSwvpConstantBuffer desde dxvk/dxso_compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn emitDclConstantBuffer() -> core::ffi::c_void {
    // TODO: implementar emitDclConstantBuffer desde dxvk/dxso_compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn emitVsInit() -> core::ffi::c_void {
    // TODO: implementar emitVsInit desde dxvk/dxso_compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn emitPsSharedConstants() -> core::ffi::c_void {
    // TODO: implementar emitPsSharedConstants desde dxvk/dxso_compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn emitPsInit() -> core::ffi::c_void {
    // TODO: implementar emitPsInit desde dxvk/dxso_compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn emitLoadConstant(reg: const DxsoBaseRegister&, relative: *mut const DxsoBaseRegister) -> DxsoRegisterValue {
    // TODO: implementar emitLoadConstant desde dxvk/dxso_compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn emitPsProcessing() -> core::ffi::c_void {
    // TODO: implementar emitPsProcessing desde dxvk/dxso_compiler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn relativeAddressingUsesToken(type: DxsoInstructionArgumentType) -> bool {
    // TODO: implementar relativeAddressingUsesToken desde dxvk/dxso_decoder.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cmdPushConstants(cmdBuffer: DxvkCmdBuffer, info: *mut const VkPushConstantsInfo) -> core::ffi::c_void {
    // TODO: implementar cmdPushConstants desde dxvk/dxvk_cmdlist.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getSpecConstantMask() -> u32 {
    // TODO: implementar getSpecConstantMask desde dxvk/dxvk_compute.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn createInstance(state: const DxvkComputePipelineStateInfo&) -> *mut DxvkComputePipelineInstance {
    // TODO: implementar createInstance desde dxvk/dxvk_compute.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn findInstance(state: const DxvkComputePipelineStateInfo&) -> *mut DxvkComputePipelineInstance {
    // TODO: implementar findInstance desde dxvk/dxvk_compute.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logPipelineState(level: LogLevel, state: const DxvkComputePipelineStateInfo&) -> core::ffi::c_void {
    // TODO: implementar logPipelineState desde dxvk/dxvk_compute.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn setPrimitiveTopology(topology: VkPrimitiveTopology) -> core::ffi::c_void {
    // TODO: implementar setPrimitiveTopology desde dxvk/dxvk_constant_state.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logicOpEnable() -> bool {
    // TODO: implementar logicOpEnable desde dxvk/dxvk_constant_state.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logicOp() -> VkLogicOp {
    // TODO: implementar logicOp desde dxvk/dxvk_constant_state.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn VkLogicOp(param_38926: m_logicOp) -> return {
    // TODO: implementar VkLogicOp desde dxvk/dxvk_constant_state.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn setLogicOp(enable: bool, op: VkLogicOp) -> core::ffi::c_void {
    // TODO: implementar setLogicOp desde dxvk/dxvk_constant_state.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn setBlendConstants(blendConstants: DxvkBlendConstants) -> core::ffi::c_void {
    // TODO: implementar setBlendConstants desde dxvk/dxvk_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn setLogicOpState(lo: const DxvkLogicOpState&) -> core::ffi::c_void {
    // TODO: implementar setLogicOpState desde dxvk/dxvk_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn flushClearsInline() -> core::ffi::c_void {
    // TODO: implementar flushClearsInline desde dxvk/dxvk_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn resetSpecConstants(newMask: u32) -> core::ffi::c_void {
    // TODO: implementar resetSpecConstants desde dxvk/dxvk_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn updateSpecConstants() -> core::ffi::c_void {
    // TODO: implementar updateSpecConstants desde dxvk/dxvk_context.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn isInUse() -> bool {
    // TODO: implementar isInUse desde dxvk/dxvk_descriptor_heap.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logDescriptorProperties() -> core::ffi::c_void {
    // TODO: implementar logDescriptorProperties desde dxvk/dxvk_descriptor_info.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logBindingModel() -> core::ffi::c_void {
    // TODO: implementar logBindingModel desde dxvk/dxvk_device.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logDeviceInfo() -> core::ffi::c_void {
    // TODO: implementar logDeviceInfo desde dxvk/dxvk_device_info.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getInstanceExtensions() -> virtual DxvkExtensionList {
    // TODO: implementar getInstanceExtensions desde dxvk/dxvk_extension_provider.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn initInstanceExtensions() -> virtual void {
    // TODO: implementar initInstanceExtensions desde dxvk/dxvk_extension_provider.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn addConstant(id: u32, value: u32) -> core::ffi::c_void {
    // TODO: implementar addConstant desde dxvk/dxvk_graphics.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn computeSpecConstantMask() -> u32 {
    // TODO: implementar computeSpecConstantMask desde dxvk/dxvk_graphics.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn enableLogicOp() -> VkBool32 {
    // TODO: implementar enableLogicOp desde dxvk/dxvk_graphics_state.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn useDynamicBlendConstants() -> bool {
    // TODO: implementar useDynamicBlendConstants desde dxvk/dxvk_graphics_state.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn isInitialized(subresource: const VkImageSubresource&) -> bool {
    // TODO: implementar isInitialized desde dxvk/dxvk_image.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DxvkInstance(flags: DxvkInstanceFlags) -> explicit {
    // TODO: implementar DxvkInstance desde dxvk/dxvk_instance.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn initVulkanInstance(args: const DxvkInstanceImportInfo&, flags: DxvkInstanceFlags) -> bool {
    // TODO: implementar initVulkanInstance desde dxvk/dxvk_instance.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn countEmptyChunksInPool(pool: const DxvkMemoryPool&) -> u32 {
    // TODO: implementar countEmptyChunksInPool desde dxvk/dxvk_memory.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logMemoryError(req: const VkMemoryRequirements&) -> core::ffi::c_void {
    // TODO: implementar logMemoryError desde dxvk/dxvk_memory.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logMemoryStats() -> core::ffi::c_void {
    // TODO: implementar logMemoryStats desde dxvk/dxvk_memory.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn queryInstanceExtensions() -> DxvkExtensionList {
    // TODO: implementar queryInstanceExtensions desde dxvk/dxvk_openvr.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getAllDescriptorsInSet(type: DxvkPipelineLayoutType, set: u32) -> DxvkPipelineBindingRange {
    // TODO: implementar getAllDescriptorsInSet desde dxvk/dxvk_pipelayout.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getResourcesInSet(type: DxvkPipelineLayoutType, set: u32) -> DxvkPipelineBindingRange {
    // TODO: implementar getResourcesInSet desde dxvk/dxvk_pipelayout.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getUniformBuffersInSet(type: DxvkPipelineLayoutType, set: u32) -> DxvkPipelineBindingRange {
    // TODO: implementar getUniformBuffersInSet desde dxvk/dxvk_pipelayout.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn setAniso(anisotropy: u32) -> core::ffi::c_void {
    // TODO: implementar setAniso desde dxvk/dxvk_sampler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn samplerIsInLruList(sampler: SamplerEntry&, index: i32) -> bool {
    // TODO: implementar samplerIsInLruList desde dxvk/dxvk_sampler.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn exportBuiltIn(builder: ir::Builder&, builtIn: ir::BuiltIn, value: ir::SsaDef) -> core::ffi::c_void {
    // TODO: implementar exportBuiltIn desde dxvk/dxvk_shader_builtin.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn exportOutput(builder: ir::Builder&, location: u32, value: ir::SsaDef, name: *mut const char) -> core::ffi::c_void {
    // TODO: implementar exportOutput desde dxvk/dxvk_shader_builtin.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn makeConstantVector(builder: ir::Builder&, constant: ir::SsaDef, type: ir::BasicType) -> SsaDef {
    // TODO: implementar makeConstantVector desde dxvk/dxvk_shader_builtin.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn patchInputTopology(code: SpirvCodeBuffer&, topology: VkPrimitiveTopology) -> static void {
    // TODO: implementar patchInputTopology desde dxvk/dxvk_shader_spirv.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logSparseBindingInfo(level: LogLevel, info: *mut const VkBindSparseInfo) -> core::ffi::c_void {
    // TODO: implementar logSparseBindingInfo desde dxvk/dxvk_sparse.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn isBlendConstantBlendFactor(factor: VkBlendFactor) -> bool {
    // TODO: implementar isBlendConstantBlendFactor desde dxvk/dxvk_util.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opShiftLeftLogical(resultType: u32, base: u32, shift: u32) -> u32 {
    // TODO: implementar opShiftLeftLogical desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opShiftRightLogical(resultType: u32, base: u32, shift: u32) -> u32 {
    // TODO: implementar opShiftRightLogical desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opLogicalEqual(resultType: u32, operand1: u32, operand2: u32) -> u32 {
    // TODO: implementar opLogicalEqual desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opLogicalNotEqual(resultType: u32, operand1: u32, operand2: u32) -> u32 {
    // TODO: implementar opLogicalNotEqual desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opLogicalAnd(resultType: u32, operand1: u32, operand2: u32) -> u32 {
    // TODO: implementar opLogicalAnd desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opLogicalOr(resultType: u32, operand1: u32, operand2: u32) -> u32 {
    // TODO: implementar opLogicalOr desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opLogicalNot(resultType: u32, operand: u32) -> u32 {
    // TODO: implementar opLogicalNot desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opSin(resultType: u32, vector: u32) -> u32 {
    // TODO: implementar opSin desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opCos(resultType: u32, vector: u32) -> u32 {
    // TODO: implementar opCos desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opSqrt(resultType: u32, operand: u32) -> u32 {
    // TODO: implementar opSqrt desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opInverseSqrt(resultType: u32, operand: u32) -> u32 {
    // TODO: implementar opInverseSqrt desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opExp2(resultType: u32, operand: u32) -> u32 {
    // TODO: implementar opExp2 desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opExp(resultType: u32, operand: u32) -> u32 {
    // TODO: implementar opExp desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opLog2(resultType: u32, operand: u32) -> u32 {
    // TODO: implementar opLog2 desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opPow(resultType: u32, base: u32, exponent: u32) -> u32 {
    // TODO: implementar opPow desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opIsInf(resultType: u32, operand: u32) -> u32 {
    // TODO: implementar opIsInf desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opImageSampleExplicitLod(resultType: u32, sampledImage: u32, coordinates: u32, operands: const SpirvImageOperands&) -> u32 {
    // TODO: implementar opImageSampleExplicitLod desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opImageSampleProjExplicitLod(resultType: u32, sampledImage: u32, coordinates: u32, operands: const SpirvImageOperands&) -> u32 {
    // TODO: implementar opImageSampleProjExplicitLod desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opImageSampleDrefExplicitLod(resultType: u32, sampledImage: u32, coordinates: u32, reference: u32, operands: const SpirvImageOperands&) -> u32 {
    // TODO: implementar opImageSampleDrefExplicitLod desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opImageSampleProjDrefExplicitLod(resultType: u32, sampledImage: u32, coordinates: u32, reference: u32, operands: const SpirvImageOperands&) -> u32 {
    // TODO: implementar opImageSampleProjDrefExplicitLod desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn opSinCos(x: u32, useBuiltIn: bool) -> u32 {
    // TODO: implementar opSinCos desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn isInterfaceVar(sclass: spv::StorageClass) -> bool {
    // TODO: implementar isInterfaceVar desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sincosTaylorFactor(power: u32) -> static constexpr double {
    // TODO: implementar sincosTaylorFactor desde dxvk/spirv_module.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn isInModuleDetachment() -> bool {
    // TODO: implementar isInModuleDetachment desde dxvk/thread.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn instance() -> VkInstance {
    // TODO: implementar instance desde dxvk/vulkan_loader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getSpecConstants(key: const HudPipelineKey&) -> HudSpecConstants {
    // TODO: implementar getSpecConstants desde dxvk/dxvk_hud_renderer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getPushConstants() -> HudPushConstants {
    // TODO: implementar getPushConstants desde dxvk/dxvk_hud_renderer.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logQueryInterfaceError(objectGuid: REFIID, requestedGuid: REFIID) -> bool {
    // TODO: implementar logQueryInterfaceError desde dxvk/com_guid.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logOptions() -> core::ffi::c_void {
    // TODO: implementar logOptions desde dxvk/config.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn logLevel() -> static LogLevel {
    // TODO: implementar logLevel desde dxvk/log.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn getMinLogLevel() -> static LogLevel {
    // TODO: implementar getMinLogLevel desde dxvk/log.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn demo_window_set_expose_func(window: *mut struct demo_window, param_42367: core::ffi::c_void) -> static inline void {
    // TODO: implementar demo_window_set_expose_func desde vkd3d-proton/demo_win32.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_instance(create_info: *mut const struct vkd3d_instance_create_info, param_58196: *mut struct vkd3d_instance) -> i32 {
    // TODO: implementar vkd3d_create_instance desde vkd3d-proton/vkd3d.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_instance_decref(instance: *mut struct vkd3d_instance) -> ULONG {
    // TODO: implementar vkd3d_instance_decref desde vkd3d-proton/vkd3d.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_instance_get_vk_instance(instance: *mut struct vkd3d_instance) -> VkInstance {
    // TODO: implementar vkd3d_instance_get_vk_instance desde vkd3d-proton/vkd3d.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_instance_incref(instance: *mut struct vkd3d_instance) -> ULONG {
    // TODO: implementar vkd3d_instance_incref desde vkd3d-proton/vkd3d.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_parse_patch_constant_signature(dxbc: *mut const struct vkd3d_shader_code, signature: *mut struct vkd3d_shader_signature) -> i32 {
    // TODO: implementar vkd3d_shader_parse_patch_constant_signature desde vkd3d-proton/vkd3d_shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_compile_dxil_export(dxil: *mut const struct vkd3d_shader_code, export: *mut const char, demangled_export: *mut const char, spirv: *mut struct vkd3d_shader_code, spirv_debug: *mut struct vkd3d_shader_code_debug, shader_interface_info: *mut const struct vkd3d_shader_interface_info, shader_interface_local_info: *mut const struct vkd3d_shader_interface_local_info, compiler_args: *mut const struct vkd3d_shader_compile_arguments) -> i32 {
    // TODO: implementar vkd3d_shader_compile_dxil_export desde vkd3d-proton/vkd3d_shader.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn is_standard_swizzle_64kb_supported(device: *mut ID3D12Device) -> bool {
    // TODO: implementar is_standard_swizzle_64kb_supported desde vkd3d-proton/d3d12_test_utils.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint32_load_explicit(target: *mut u32, order: vkd3d_memory_order) -> FORCEINLINE uint32_t {
    // TODO: implementar vkd3d_atomic_uint32_load_explicit desde vkd3d-proton/vkd3d_atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint32_store_explicit(target: *mut u32, value: u32, order: vkd3d_memory_order) -> FORCEINLINE void {
    // TODO: implementar vkd3d_atomic_uint32_store_explicit desde vkd3d-proton/vkd3d_atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint32_exchange_explicit(target: *mut u32, value: u32, order: vkd3d_memory_order) -> FORCEINLINE uint32_t {
    // TODO: implementar vkd3d_atomic_uint32_exchange_explicit desde vkd3d-proton/vkd3d_atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint64_load_explicit(target: *mut u64, order: vkd3d_memory_order) -> FORCEINLINE uint64_t {
    // TODO: implementar vkd3d_atomic_uint64_load_explicit desde vkd3d-proton/vkd3d_atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint64_store_explicit(target: *mut u64, value: u64, order: vkd3d_memory_order) -> FORCEINLINE void {
    // TODO: implementar vkd3d_atomic_uint64_store_explicit desde vkd3d-proton/vkd3d_atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_atomic_uint64_exchange_explicit(target: *mut u64, value: u64, order: vkd3d_memory_order) -> FORCEINLINE uint64_t {
    // TODO: implementar vkd3d_atomic_uint64_exchange_explicit desde vkd3d-proton/vkd3d_atomic.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_log2i(x: u32) -> static inline unsigned int {
    // TODO: implementar vkd3d_log2i desde vkd3d-proton/vkd3d_common.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_log2i_ceil(x: u32) -> static inline unsigned int {
    // TODO: implementar vkd3d_log2i_ceil desde vkd3d-proton/vkd3d_common.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn is_power_of_two(x: u32) -> static inline bool {
    // TODO: implementar is_power_of_two desde vkd3d-proton/vkd3d_common.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_open_export_kmt(fence: *mut struct d3d12_shared_fence, device: *mut struct d3d12_device) -> extern void {
    // TODO: implementar d3d12_shared_fence_open_export_kmt desde vkd3d-proton/vkd3d_d3dkmt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_close_export_kmt(fence: *mut struct d3d12_shared_fence) -> extern void {
    // TODO: implementar d3d12_shared_fence_close_export_kmt desde vkd3d-proton/vkd3d_d3dkmt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_open_export_kmt(resource: *mut struct d3d12_resource, device: *mut struct d3d12_device, allocation: *mut struct vkd3d_memory_allocation) -> extern void {
    // TODO: implementar d3d12_resource_open_export_kmt desde vkd3d-proton/vkd3d_d3dkmt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_close_export_kmt(resource: *mut struct d3d12_resource, device: *mut struct d3d12_device) -> extern void {
    // TODO: implementar d3d12_resource_close_export_kmt desde vkd3d-proton/vkd3d_d3dkmt.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_export_strequal(a: *mut const WCHAR, b: *mut const WCHAR) -> bool {
    // TODO: implementar vkd3d_export_strequal desde vkd3d-proton/vkd3d_string.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_export_strequal_mixed(a: *mut const WCHAR, b: *mut const char) -> bool {
    // TODO: implementar vkd3d_export_strequal_mixed desde vkd3d-proton/vkd3d_string.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_export_strequal_substr(a: *mut const WCHAR, n: usize, b: *mut const WCHAR) -> bool {
    // TODO: implementar vkd3d_export_strequal_substr desde vkd3d-proton/vkd3d_string.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DEBUG_CHANNEL_INIT_IMPLICIT_INSTANCE(id: uvec3, inst: uint) -> core::ffi::c_void {
    // TODO: implementar DEBUG_CHANNEL_INIT_IMPLICIT_INSTANCE desde vkd3d-proton/debug_channel.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetPrimitiveTopology_profiled(iface: *mut d3d12_command_list_iface, topology: D3D12_PRIMITIVE_TOPOLOGY) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_IASetPrimitiveTopology_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetViewInstanceMask_profiled(iface: *mut d3d12_command_list_iface, mask: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetViewInstanceMask_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetFrontAndBackStencilRef_profiled(iface: *mut d3d12_command_list_iface, FrontStencilRef: UINT, BackStencilRef: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_OMSetFrontAndBackStencilRef_profiled desde vkd3d-proton/command_list_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateConstantBufferView_profiled(iface: *mut d3d12_device_iface, desc: *mut const D3D12_CONSTANT_BUFFER_VIEW_DESC, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateConstantBufferView_profiled desde vkd3d-proton/device_profiled.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_active_log() -> bool {
    // TODO: implementar vkd3d_descriptor_debug_active_log desde vkd3d-proton/vkd3d_descriptor_debug.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_iface_set_native_sync_handle_on_completion_explicit(iface: *mut ID3D12Fence, wait_type: enum vkd3d_waiting_event_type, value: UINT64, handle: vkd3d_native_sync_handle, payload: *mut u32) -> i32 {
    // TODO: implementar d3d12_fence_iface_set_native_sync_handle_on_completion_explicit desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_buffer_explicit_usage(device: *mut struct d3d12_device, vk_usage: VkBufferUsageFlags2KHR, vk_size: VkDeviceSize, tag: *mut const char, vk_buffer: *mut VkBuffer) -> i32 {
    // TODO: implementar vkd3d_create_buffer_explicit_usage desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_copy_single(dst: vkd3d_cpu_descriptor_va_t, src: vkd3d_cpu_descriptor_va_t, device: *mut struct d3d12_device) -> core::ffi::c_void {
    // TODO: implementar d3d12_desc_copy_single desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_desc_copy_embedded_resource_single_32(dst_va: vkd3d_cpu_descriptor_va_t, src_va: vkd3d_cpu_descriptor_va_t) -> static inline void {
    // TODO: implementar d3d12_desc_copy_embedded_resource_single_32 desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_debug_ring_init_spec_constant(device: *mut struct d3d12_device, info: *mut struct vkd3d_shader_spec_info, hash: vkd3d_shader_hash_t) -> core::ffi::c_void {
    // TODO: implementar vkd3d_shader_debug_ring_init_spec_constant desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_register_instantaneous(trace: *mut struct vkd3d_queue_timeline_trace, type: enum vkd3d_queue_timeline_trace_state_type, value: u64) -> core::ffi::c_void {
    // TODO: implementar vkd3d_queue_timeline_trace_register_instantaneous desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_export_equal(export: LPCWSTR, entry: *mut const struct vkd3d_shader_library_entry_point) -> bool {
    // TODO: implementar vkd3d_export_equal desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_load_vk_instance_procs(procs: *mut struct vkd3d_vk_instance_procs, global_procs: *mut const struct vkd3d_vk_global_procs, instance: VkInstance) -> i32 {
    // TODO: implementar vkd3d_load_vk_instance_procs desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vk_topology_from_d3d12_topology(topology: D3D12_PRIMITIVE_TOPOLOGY) -> enum VkPrimitiveTopology {
    // TODO: implementar vk_topology_from_d3d12_topology desde vkd3d-proton/vkd3d_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn shader_parse_patch_constant_signature(dxbc: *mut const void, dxbc_length: usize, signature: *mut struct vkd3d_shader_signature) -> i32 {
    // TODO: implementar shader_parse_patch_constant_signature desde vkd3d-proton/vkd3d_shader_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_dump_spirv_shader_export(hash: vkd3d_shader_hash_t, shader: *mut const struct vkd3d_shader_code, export: *mut const char) -> core::ffi::c_void {
    // TODO: implementar vkd3d_shader_dump_spirv_shader_export desde vkd3d-proton/vkd3d_shader_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_shader_replace_export(hash: vkd3d_shader_hash_t, param_43276: *mut const void, size: *mut usize, export: *mut const char) -> bool {
    // TODO: implementar vkd3d_shader_replace_export desde vkd3d-proton/vkd3d_shader_private.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cosf(param_50329: angle) -> *mut r0 {
    // TODO: implementar cosf desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn sinf(param_50329: angle) -> *mut r0 {
    // TODO: implementar sinf desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn cxg_expose(window: *mut struct demo_window, user_data: *mut core::ffi::c_void) -> static void {
    // TODO: implementar cxg_expose desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn check_clip_distance(context: *mut struct test_context, pso: *mut ID3D12PipelineState, vb: *mut ID3D12Resource, vs_cb: *mut ID3D12Resource, gs_cb: *mut ID3D12Resource) -> static void {
    // TODO: implementar check_clip_distance desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_clip_distance(use_dxil: bool) -> static void {
    // TODO: implementar test_clip_distance desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_clip_distance_dxbc() -> core::ffi::c_void {
    // TODO: implementar test_clip_distance_dxbc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_clip_distance_dxil() -> core::ffi::c_void {
    // TODO: implementar test_clip_distance_dxil desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_combined_clip_and_cull_distances(use_dxil: bool) -> static void {
    // TODO: implementar test_combined_clip_and_cull_distances desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_combined_clip_and_cull_distances_dxbc() -> core::ffi::c_void {
    // TODO: implementar test_combined_clip_and_cull_distances_dxbc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_combined_clip_and_cull_distances_dxil() -> core::ffi::c_void {
    // TODO: implementar test_combined_clip_and_cull_distances_dxil desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_draw_instanced() -> core::ffi::c_void {
    // TODO: implementar test_draw_instanced desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_draw_indexed_instanced() -> core::ffi::c_void {
    // TODO: implementar test_draw_indexed_instanced desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_bundle_state_inheritance() -> core::ffi::c_void {
    // TODO: implementar test_bundle_state_inheritance desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_execute_indirect_multi_dispatch_root_constants() -> core::ffi::c_void {
    // TODO: implementar test_execute_indirect_multi_dispatch_root_constants desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn prepare_instanced_draw(context: *mut struct test_context) -> static void {
    // TODO: implementar prepare_instanced_draw desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_aliasing_barrier() -> core::ffi::c_void {
    // TODO: implementar test_aliasing_barrier desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn R32_SINT(param_6062: UAV) -> for {
    // TODO: implementar R32_SINT desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_stencil_export(use_dxil: bool) -> static void {
    // TODO: implementar test_stencil_export desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_stencil_export_dxbc() -> core::ffi::c_void {
    // TODO: implementar test_stencil_export_dxbc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_stencil_export_dxil() -> core::ffi::c_void {
    // TODO: implementar test_stencil_export_dxil desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_update_descriptor_heap_after_closing_command_list() -> core::ffi::c_void {
    // TODO: implementar test_update_descriptor_heap_after_closing_command_list desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_null_descriptor_resinfo(use_dxil: bool) -> static void {
    // TODO: implementar test_null_descriptor_resinfo desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_null_descriptor_resinfo_dxbc() -> core::ffi::c_void {
    // TODO: implementar test_null_descriptor_resinfo_dxbc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_null_descriptor_resinfo_dxil() -> core::ffi::c_void {
    // TODO: implementar test_null_descriptor_resinfo_dxil desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn powf(param_38899: *mut core::ffi::c_void) -> else return {
    // TODO: implementar powf desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_topology_triangle_fan() -> core::ffi::c_void {
    // TODO: implementar test_topology_triangle_fan desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_coverage_export_atoc(use_dxil: bool) -> core::ffi::c_void {
    // TODO: implementar test_coverage_export_atoc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_coverage_export_atoc_dxbc() -> core::ffi::c_void {
    // TODO: implementar test_coverage_export_atoc_dxbc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_coverage_export_atoc_dxil() -> core::ffi::c_void {
    // TODO: implementar test_coverage_export_atoc_dxil desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_view_instancing() -> core::ffi::c_void {
    // TODO: implementar test_view_instancing desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_view_instancing_indirect_state() -> core::ffi::c_void {
    // TODO: implementar test_view_instancing_indirect_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_gs_topology_mismatch(dxil: bool) -> core::ffi::c_void {
    // TODO: implementar test_gs_topology_mismatch desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_gs_topology_mismatch_dxbc() -> core::ffi::c_void {
    // TODO: implementar test_gs_topology_mismatch_dxbc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_gs_topology_mismatch_dxil() -> core::ffi::c_void {
    // TODO: implementar test_gs_topology_mismatch_dxil desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn instance_index_is_aabb(index: u32) -> static bool {
    // TODO: implementar instance_index_is_aabb desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_add_subobject_to_exports_association(factory: *mut struct rt_pso_factory, subobject_index: u32, num_exports: u32, exports: *mut LPCWSTR) -> static unsigned int {
    // TODO: implementar rt_pso_factory_add_subobject_to_exports_association desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn rt_pso_factory_add_dxil_subobject_to_exports_association(factory: *mut struct rt_pso_factory, object: LPCWSTR, num_exports: u32, exports: *mut LPCWSTR) -> static unsigned int {
    // TODO: implementar rt_pso_factory_add_dxil_subobject_to_exports_association desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_raytracing_missing_required_objects() -> core::ffi::c_void {
    // TODO: implementar test_raytracing_missing_required_objects desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_srgb_unorm_mismatch_usage_aliasing() -> core::ffi::c_void {
    // TODO: implementar test_srgb_unorm_mismatch_usage_aliasing desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn increasing(param_23998: %u <) -> Resource size is not monotonically {
    // TODO: implementar increasing desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_aliasing_barrier_edge_cases() -> core::ffi::c_void {
    // TODO: implementar test_aliasing_barrier_edge_cases desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_missing_bindings_root_signature() -> core::ffi::c_void {
    // TODO: implementar test_missing_bindings_root_signature desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ps_main_single(vout: VSOut) -> core::ffi::c_void {
    // TODO: implementar ps_main_single desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_cs_constant_buffer() -> core::ffi::c_void {
    // TODO: implementar test_cs_constant_buffer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_constant_buffer_relative_addressing() -> core::ffi::c_void {
    // TODO: implementar test_constant_buffer_relative_addressing desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_immediate_constant_buffer(use_dxil: bool) -> static void {
    // TODO: implementar test_immediate_constant_buffer desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_immediate_constant_buffer_dxbc() -> core::ffi::c_void {
    // TODO: implementar test_immediate_constant_buffer_dxbc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_immediate_constant_buffer_dxil() -> core::ffi::c_void {
    // TODO: implementar test_immediate_constant_buffer_dxil desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_root_constants() -> core::ffi::c_void {
    // TODO: implementar test_root_constants desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_resinfo() -> core::ffi::c_void {
    // TODO: implementar test_resinfo desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_instance_id(use_dxil: bool) -> static void {
    // TODO: implementar test_instance_id desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_instance_id_dxbc() -> core::ffi::c_void {
    // TODO: implementar test_instance_id_dxbc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_instance_id_dxil() -> core::ffi::c_void {
    // TODO: implementar test_instance_id_dxil desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_constant_buffers(use_dxil: bool) -> static void {
    // TODO: implementar test_constant_buffers desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_constant_buffer_sm51() -> core::ffi::c_void {
    // TODO: implementar test_constant_buffer_sm51 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_constant_buffer_dxil() -> core::ffi::c_void {
    // TODO: implementar test_constant_buffer_dxil desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_root_constant_indexing(use_dxil: bool) -> static void {
    // TODO: implementar test_root_constant_indexing desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_root_constant_indexing_dxbc() -> core::ffi::c_void {
    // TODO: implementar test_root_constant_indexing_dxbc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_root_constant_indexing_dxil() -> core::ffi::c_void {
    // TODO: implementar test_root_constant_indexing_dxil desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_vs_instance_input_nonuniform_workarounds() -> core::ffi::c_void {
    // TODO: implementar test_vs_instance_input_nonuniform_workarounds desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_primitive_restart_list_topology_stream_output() -> core::ffi::c_void {
    // TODO: implementar test_primitive_restart_list_topology_stream_output desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_quad_tessellation_wrong_pso_topology_dxbc() -> core::ffi::c_void {
    // TODO: implementar test_quad_tessellation_wrong_pso_topology_dxbc desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_quad_tessellation_wrong_pso_topology_dxil() -> core::ffi::c_void {
    // TODO: implementar test_quad_tessellation_wrong_pso_topology_dxil desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_hull_shader_vertex_input_patch_constant_phase() -> core::ffi::c_void {
    // TODO: implementar test_hull_shader_vertex_input_patch_constant_phase desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn export_strequal(a: LPCWSTR, b: LPCWSTR) -> static bool {
    // TODO: implementar export_strequal desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn basic_expected(desc: *mut const struct workgraph_test_desc, value_index: u32) -> static uint32_t {
    // TODO: implementar basic_expected desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn broadcast_input_expected(desc: *mut const struct workgraph_test_desc, value_index: u32) -> static uint32_t {
    // TODO: implementar broadcast_input_expected desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn broadcast_input_uint2_expected(desc: *mut const struct workgraph_test_desc, value_index: u32) -> static uint32_t {
    // TODO: implementar broadcast_input_uint2_expected desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn broadcast_input_uint16x2_expected(desc: *mut const struct workgraph_test_desc, value_index: u32) -> static uint32_t {
    // TODO: implementar broadcast_input_uint16x2_expected desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn broadcast_input_uint_expected(desc: *mut const struct workgraph_test_desc, value_index: u32) -> static uint32_t {
    // TODO: implementar broadcast_input_uint_expected desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn coalesced_input_expected(desc: *mut const struct workgraph_test_desc, value_index: u32) -> static uint32_t {
    // TODO: implementar coalesced_input_expected desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn copy_descriptor_heap_single(device: *mut ID3D12Device, gpu_heap: *mut ID3D12DescriptorHeap, cpu_heap: *mut ID3D12DescriptorHeap, count: u32) -> static void {
    // TODO: implementar copy_descriptor_heap_single desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn log_process_memory() -> static void {
    // TODO: implementar log_process_memory desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn test_create_instance() -> static void {
    // TODO: implementar test_create_instance desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn check_instance_extensions(param_6043: *mut const char, extensions: *mut struct vulkan_extension, extension_count: u32) -> static uint32_t {
    // TODO: implementar check_instance_extensions desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn fake_vkGetInstanceProcAddr(instance: VkInstance, name: *mut const char) -> static PFN_vkVoidFunction VKAPI_CALL {
    // TODO: implementar fake_vkGetInstanceProcAddr desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkGetInstanceProcAddr(param_2462: instance, param_340: name) -> return {
    // TODO: implementar vkGetInstanceProcAddr desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IVKD3DCoreInterface_EnableExperimentalFeatures(param_960: core, param_31201: feature_count, param_30288: iids, param_7573: configurations, param_2423: configurations_sizes) -> return {
    // TODO: implementar IVKD3DCoreInterface_EnableExperimentalFeatures desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_create_instance_global(param_58196: *mut struct vkd3d_instance) -> static HRESULT {
    // TODO: implementar vkd3d_create_instance_global desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_control_SetExplodeOnValidationError(iface: *mut IVKD3DDebugControlInterface, enable: i32) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar vkd3d_debug_control_SetExplodeOnValidationError desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_factory_EnableExperimentalFeatures(iface: *mut ID3D12DeviceFactory, NumFeatures: UINT, pIIDs: *mut const IID, pConfigurationStructs: *mut core::ffi::c_void, pConfigurationStructSizes: *mut UINT) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_factory_EnableExperimentalFeatures desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12core_EnableExperimentalFeatures(param_50043: NULL, param_40494: NumFeatures, param_41806: pIIDs, param_30568: pConfigurationStructs, param_17913: pConfigurationStructSizes) -> return {
    // TODO: implementar d3d12core_EnableExperimentalFeatures desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_configuration_GetEnabledExperimentalFeatures(iface: *mut ID3D12DeviceConfiguration1, pGuids: *mut GUID, NumGuids: UINT) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_configuration_GetEnabledExperimentalFeatures desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_draw_instanced(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_draw_instanced desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_DrawInstanced(iface: *mut d3d12_command_list_iface, vertex_count_per_instance: UINT, instance_count: UINT, start_vertex_location: UINT, start_instance_location: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_DrawInstanced desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_draw_indexed_instanced(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_draw_indexed_instanced desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_DrawIndexedInstanced(iface: *mut d3d12_command_list_iface, index_count_per_instance: UINT, instance_count: UINT, start_vertex_location: UINT, base_vertex_location: INT, start_instance_location: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_DrawIndexedInstanced desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_ia_set_primitive_topology(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_ia_set_primitive_topology desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_IASetPrimitiveTopology(iface: *mut d3d12_command_list_iface, topology: D3D12_PRIMITIVE_TOPOLOGY) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_IASetPrimitiveTopology desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_compute_root_32bit_constant(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_compute_root_32bit_constant desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetComputeRoot32BitConstant(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, data: UINT, dst_offset: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetComputeRoot32BitConstant desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_graphics_root_32bit_constant(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_graphics_root_32bit_constant desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetGraphicsRoot32BitConstant(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, data: UINT, dst_offset: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetGraphicsRoot32BitConstant desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_compute_root_32bit_constants(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_compute_root_32bit_constants desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetComputeRoot32BitConstants(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, constant_count: UINT, data: *mut const void, dst_offset: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetComputeRoot32BitConstants desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_graphics_root_32bit_constants(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_graphics_root_32bit_constants desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetGraphicsRoot32BitConstants(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, constant_count: UINT, data: *mut const void, dst_offset: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetGraphicsRoot32BitConstants desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetComputeRootConstantBufferView(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, address: D3D12_GPU_VIRTUAL_ADDRESS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetComputeRootConstantBufferView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetGraphicsRootConstantBufferView(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, address: D3D12_GPU_VIRTUAL_ADDRESS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetGraphicsRootConstantBufferView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_set_view_instance_mask(list: *mut d3d12_command_list_iface, args_v: *mut const void) -> static void {
    // TODO: implementar d3d12_bundle_exec_set_view_instance_mask desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_SetViewInstanceMask(iface: *mut d3d12_command_list_iface, mask: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_SetViewInstanceMask desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_OMSetFrontAndBackStencilRef(iface: *mut d3d12_command_list_iface, FrontStencilRef: UINT, BackStencilRef: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_bundle_OMSetFrontAndBackStencilRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_fence_set_native_sync_handle_on_completion_explicit(fence: *mut struct d3d12_fence, wait_type: enum vkd3d_waiting_event_type, value: UINT64, handle: vkd3d_native_sync_handle, payload: *mut u32) -> static HRESULT {
    // TODO: implementar d3d12_fence_set_native_sync_handle_on_completion_explicit desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_shared_fence_set_native_sync_handle_on_completion_explicit(fence: *mut struct d3d12_shared_fence, wait_type: enum vkd3d_waiting_event_type, value: u64, handle: vkd3d_native_sync_handle, payload: *mut u32) -> static HRESULT {
    // TODO: implementar d3d12_shared_fence_set_native_sync_handle_on_completion_explicit desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_update_root_constants(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings, layout: VkPipelineLayout, push_stages: VkShaderStageFlags) -> static void {
    // TODO: implementar d3d12_command_list_update_root_constants desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DrawInstanced(iface: *mut d3d12_command_list_iface, vertex_count_per_instance: UINT, instance_count: UINT, start_vertex_location: UINT, start_instance_location: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_DrawInstanced desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_DrawIndexedInstanced(iface: *mut d3d12_command_list_iface, index_count_per_instance: UINT, instance_count: UINT, start_vertex_location: UINT, base_vertex_location: INT, start_instance_location: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_DrawIndexedInstanced desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_IASetPrimitiveTopology(iface: *mut d3d12_command_list_iface, topology: D3D12_PRIMITIVE_TOPOLOGY) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_IASetPrimitiveTopology desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_set_root_constants(list: *mut struct d3d12_command_list, bindings: *mut struct vkd3d_pipeline_bindings, index: u32, offset: u32, count: u32, data: *mut const void) -> static void {
    // TODO: implementar d3d12_command_list_set_root_constants desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRoot32BitConstant(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, data: UINT, dst_offset: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetComputeRoot32BitConstant desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRoot32BitConstant(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, data: UINT, dst_offset: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetGraphicsRoot32BitConstant desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRoot32BitConstants(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, constant_count: UINT, data: *mut const void, dst_offset: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetComputeRoot32BitConstants desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRoot32BitConstants(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, constant_count: UINT, data: *mut const void, dst_offset: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetGraphicsRoot32BitConstants desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetComputeRootConstantBufferView(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, address: D3D12_GPU_VIRTUAL_ADDRESS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetComputeRootConstantBufferView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetGraphicsRootConstantBufferView(iface: *mut d3d12_command_list_iface, root_parameter_index: UINT, address: D3D12_GPU_VIRTUAL_ADDRESS) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetGraphicsRootConstantBufferView desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn clamp_float_to_sint32(value: f32, min_value: i32, max_value: i32) -> static int32_t {
    // TODO: implementar clamp_float_to_sint32 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_SetViewInstanceMask(iface: *mut d3d12_command_list_iface, mask: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_SetViewInstanceMask desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_OMSetFrontAndBackStencilRef(iface: *mut d3d12_command_list_iface, FrontStencilRef: UINT, BackStencilRef: UINT) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_command_list_OMSetFrontAndBackStencilRef desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_instance_caps(instance: *mut struct vkd3d_instance, create_info: *mut const struct vkd3d_instance_create_info, instance_extension_count: *mut u32, user_extension_supported: *mut bool) -> static HRESULT {
    // TODO: implementar vkd3d_init_instance_caps desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_debug_control_explode_on_vvl_error() -> bool {
    // TODO: implementar vkd3d_debug_control_explode_on_vvl_error desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_instance_apply_application_workarounds() -> static void {
    // TODO: implementar vkd3d_instance_apply_application_workarounds desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_instance_deduce_config_flags_from_environment() -> static void {
    // TODO: implementar vkd3d_instance_deduce_config_flags_from_environment desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_instance_apply_global_shader_quirks() -> static void {
    // TODO: implementar vkd3d_instance_apply_global_shader_quirks desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_instance_init(instance: *mut struct vkd3d_instance, create_info: *mut const struct vkd3d_instance_create_info) -> static HRESULT {
    // TODO: implementar vkd3d_instance_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_destroy_instance(instance: *mut struct vkd3d_instance) -> static void {
    // TODO: implementar vkd3d_destroy_instance desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_add_device_singleton(device: *mut struct d3d12_device, luid: LUID) -> static void {
    // TODO: implementar d3d12_add_device_singleton desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_remove_device_singleton(luid: LUID) -> static void {
    // TODO: implementar d3d12_remove_device_singleton desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateConstantBufferView_embedded(iface: *mut d3d12_device_iface, desc: *mut const D3D12_CONSTANT_BUFFER_VIEW_DESC, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateConstantBufferView_embedded desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_CreateConstantBufferView_default(iface: *mut d3d12_device_iface, desc: *mut const D3D12_CONSTANT_BUFFER_VIEW_DESC, descriptor: D3D12_CPU_DESCRIPTOR_HANDLE) -> static void STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_CreateConstantBufferView_default desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_SetStablePowerState(iface: *mut d3d12_device_iface, enable: i32) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_device_SetStablePowerState desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_dxvk_interop_device_GetInstanceExtensions(iface: *mut d3d12_dxvk_interop_device_iface, extension_count: *mut UINT, param_6043: *mut const char) -> static HRESULT STDMETHODCALLTYPE {
    // TODO: implementar d3d12_dxvk_interop_device_GetInstanceExtensions desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_queue_timeline_trace_flush_instantaneous(trace: *mut struct vkd3d_queue_timeline_trace, worker: *mut struct vkd3d_fence_worker) -> static void {
    // TODO: implementar vkd3d_queue_timeline_trace_flush_instantaneous desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_get_export_index(object: *mut struct d3d12_rt_state_object, export_name: *mut const WCHAR, param_51335: *mut const WCHAR) -> static uint32_t {
    // TODO: implementar d3d12_state_object_get_export_index desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_device_requires_explicit_sparse_init(device: *mut struct d3d12_device) -> static bool {
    // TODO: implementar d3d12_device_requires_explicit_sparse_init desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_info_get_topology(topology: *mut struct vkd3d_memory_topology, device: *mut struct d3d12_device) -> static void {
    // TODO: implementar vkd3d_memory_info_get_topology desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_topology_is_uma_like(topology: *mut const struct vkd3d_memory_topology) -> static bool {
    // TODO: implementar vkd3d_memory_topology_is_uma_like desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_shader_record_constants(root_signature: *mut struct d3d12_root_signature, desc: *mut const D3D12_ROOT_SIGNATURE_DESC2, info: *mut const struct d3d12_root_signature_info) -> static HRESULT {
    // TODO: implementar d3d12_root_signature_init_shader_record_constants desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_root_signature_init_push_constants(root_signature: *mut struct d3d12_root_signature, desc: *mut const D3D12_ROOT_SIGNATURE_DESC2, info: *mut const struct d3d12_root_signature_info, push_constant_range: *mut struct VkPushConstantRange) -> static HRESULT {
    // TODO: implementar d3d12_root_signature_init_push_constants desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vk_logic_op_from_d3d12(op: D3D12_LOGIC_OP) -> static VkLogicOp {
    // TODO: implementar vk_logic_op_from_d3d12 desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_validate_gs_input_toplogy(state: *mut struct d3d12_pipeline_state, gs_meta: *mut const struct vkd3d_shader_meta, geometry_meta: u32) -> static bool {
    // TODO: implementar d3d12_pipeline_state_validate_gs_input_toplogy desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_validate_view_instancing(device: *mut struct d3d12_device, graphics: *mut struct d3d12_graphics_pipeline_state, desc: *mut const struct d3d12_pipeline_state_desc) -> static bool {
    // TODO: implementar d3d12_pipeline_state_validate_view_instancing desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_state_log_graphics_state(state: *mut const struct d3d12_pipeline_state) -> static void {
    // TODO: implementar d3d12_pipeline_state_log_graphics_state desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_bindless_find_copy_template_single(descriptor_size: u32) -> static pfn_vkd3d_host_mapping_copy_template_single {
    // TODO: implementar vkd3d_bindless_find_copy_template_single desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_state_object_find_explicit_assignment_override(kind: enum vkd3d_shader_subobject_kind, associations: *mut const struct d3d12_state_object_association, associations_count: usize, association: *mut const struct d3d12_state_object_association) -> static bool {
    // TODO: implementar d3d12_state_object_find_explicit_assignment_override desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_poll_single_calibration(chain: *mut struct dxgi_vk_swap_chain, time_domain: VkTimeDomainEXT, time_domain_id: u64, calibration: *mut u64) -> static bool {
    // TODO: implementar dxgi_vk_swap_chain_poll_single_calibration desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_find_exported_entry_point(data: *mut struct d3d12_wg_state_object_data, shader: LPCWSTR) -> *mut static const struct vkd3d_shader_library_entry_point {
    // TODO: implementar d3d12_wg_state_object_find_exported_entry_point desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn d3d12_wg_state_object_resolve_entry_points_explicit(object: *mut struct d3d12_wg_state_object, data: *mut struct d3d12_wg_state_object_data, program: *mut struct d3d12_wg_state_object_program) -> static HRESULT {
    // TODO: implementar d3d12_wg_state_object_resolve_entry_points_explicit desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn shader_parse_root_constants(context: *mut struct root_signature_parser_context, offset: u32, constants: *mut struct vkd3d_root_constants) -> static int {
    // TODO: implementar shader_parse_root_constants desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn vkd3d_dxil_log_callback(userdata: *mut core::ffi::c_void, level: dxil_spv_log_level, msg: *mut const char) -> static void {
    // TODO: implementar vkd3d_dxil_log_callback desde vkd3d-proton/unknown
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetPow2DownscaleFactor() -> virtual UINT STDMETHODCALLTYPE {
    // TODO: implementar GetPow2DownscaleFactor desde DirectX-Headers/d3d12video.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CreateExplorer(pUnknown: *mut IUnknown, riid: REFIID, param_43676: *mut _COM_Outptr_ void) -> virtual HRESULT STDMETHODCALLTYPE {
    // TODO: implementar CreateExplorer desde DirectX-Headers/d3dshadercacheregistration.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn Aliasing(pResourceBefore: *mut _In_opt_ ID3D12Resource, pResourceAfter: *mut _In_opt_ ID3D12Resource) -> static inline CD3DX12_RESOURCE_BARRIER {
    // TODO: implementar Aliasing desde DirectX-Headers/d3dx12_barriers.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn OutputMergerLogicOp() -> i32 {
    // TODO: implementar OutputMergerLogicOp desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn StandardSwizzle64KBSupported() -> i32 {
    // TODO: implementar StandardSwizzle64KBSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ExpandedComputeResourceStates() -> i32 {
    // TODO: implementar ExpandedComputeResourceStates desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ViewInstancingTier() -> D3D12_VIEW_INSTANCING_TIER {
    // TODO: implementar ViewInstancingTier desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn BackgroundProcessingSupported() -> i32 {
    // TODO: implementar BackgroundProcessingSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DerivativesInMeshAndAmplificationShadersSupported() -> i32 {
    // TODO: implementar DerivativesInMeshAndAmplificationShadersSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn IndependentFrontAndBackStencilRefMaskSupported() -> i32 {
    // TODO: implementar IndependentFrontAndBackStencilRefMaskSupported desde DirectX-Headers/d3dx12_check_feature_support.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn D3DX12ConditionallyExpandAPIDesc(LclDesc: CD3DX12_RESOURCE_DESC1&, pDesc: *mut const CD3DX12_RESOURCE_DESC1, false: const bool tightAlignmentSupported =, false: const bool alignAsCommitted =) -> *mut inline const CD3DX12_RESOURCE_DESC1 {
    // TODO: implementar D3DX12ConditionallyExpandAPIDesc desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn CD3DX12_VIEW_INSTANCING_DESC(param_55590: CD3DX12_DEFAULT) -> explicit {
    // TODO: implementar CD3DX12_VIEW_INSTANCING_DESC desde DirectX-Headers/d3dx12_core.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn PrimitiveTopologyTypeCb(PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE) -> core::ffi::c_void {
    // TODO: implementar PrimitiveTopologyTypeCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn ViewInstancingCb(ViewInstancingDesc: const D3D12_VIEW_INSTANCING_DESC&) -> core::ffi::c_void {
    // TODO: implementar ViewInstancingCb desde DirectX-Headers/d3dx12_pipeline_state_stream.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FormatExistsInHeader(Format: DXGI_FORMAT, true: bool bExternalHeader =) -> static bool {
    // TODO: implementar FormatExistsInHeader desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetNumComponentsInFormat(Format: DXGI_FORMAT) -> static UINT {
    // TODO: implementar GetNumComponentsInFormat desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetMinNumComponentsInFormats(FormatA: DXGI_FORMAT, FormatB: DXGI_FORMAT) -> static UINT {
    // TODO: implementar GetMinNumComponentsInFormats desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn GetAddressingBitsPerAlignedSize(Format: DXGI_FORMAT) -> static UINT8 {
    // TODO: implementar GetAddressingBitsPerAlignedSize desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn FloatAndNotFloatFormats(FormatA: DXGI_FORMAT, FormatB: DXGI_FORMAT) -> static bool {
    // TODO: implementar FloatAndNotFloatFormats desde DirectX-Headers/d3dx12_property_format_table.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitAsConstants(param_20471: _Out_ D3D12_ROOT_PARAMETER, num32BitValues: UINT, shaderRegister: UINT, param_38160: UINT registerSpace =, D3D12_SHADER_VISIBILITY_ALL: D3D12_SHADER_VISIBILITY visibility =) -> static inline void {
    // TODO: implementar InitAsConstants desde DirectX-Headers/d3dx12_root_signature.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn InitAsConstantBufferView(param_20471: _Out_ D3D12_ROOT_PARAMETER, shaderRegister: UINT, param_38160: UINT registerSpace =, D3D12_SHADER_VISIBILITY_ALL: D3D12_SHADER_VISIBILITY visibility =) -> static inline void {
    // TODO: implementar InitAsConstantBufferView desde DirectX-Headers/d3dx12_root_signature.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DefineExport(Name: LPCWSTR, nullptr: LPCWSTR ExportToRename =, D3D12_EXPORT_FLAG_NONE: D3D12_EXPORT_FLAGS Flags =) -> core::ffi::c_void {
    // TODO: implementar DefineExport desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn DefineExports(param_3842: *mut core::ffi::c_void) -> core::ffi::c_void {
    // TODO: implementar DefineExports desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AddExport(Export: LPCWSTR) -> core::ffi::c_void {
    // TODO: implementar AddExport desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AddExports(param_25711: LPCWSTR) -> core::ffi::c_void {
    // TODO: implementar AddExports desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetHitGroupExport(exportName: LPCWSTR) -> core::ffi::c_void {
    // TODO: implementar SetHitGroupExport desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetAnyHitShaderImport(importName: LPCWSTR) -> core::ffi::c_void {
    // TODO: implementar SetAnyHitShaderImport desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn SetPrimitiveTopologyType(primitiveTopologytype: D3D12_PRIMITIVE_TOPOLOGY_TYPE) -> core::ffi::c_void {
    // TODO: implementar SetPrimitiveTopologyType desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn AddViewInstanceLocation(viewInstanceLocation: D3D12_VIEW_INSTANCE_LOCATION) -> core::ffi::c_void {
    // TODO: implementar AddViewInstanceLocation desde DirectX-Headers/d3dx12_state_object.h
    core::ptr::null_mut()
}
#[no_mangle]
pub unsafe extern "C" fn RuntimeClassInitialize() -> i32 {
    // TODO: implementar RuntimeClassInitialize desde DirectX-Headers/wrladapter.h
    core::ptr::null_mut()
}
