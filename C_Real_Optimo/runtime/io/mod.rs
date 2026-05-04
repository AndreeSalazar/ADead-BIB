//! ADead Runtime - IO Module
//!
//! Funciones generadas automáticamente desde knowledge.json
//! Categoría: io

#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

/// AppPolicyGetThreadInitializationType - from wine/appmodel.h
#[no_mangle]
pub unsafe extern "C" fn AppPolicyGetThreadInitializationType(token: *mut core::ffi::c_void, policy: *mut core::ffi::c_void) -> i32 {
    0
}

/// AvSetMmThreadCharacteristicsA - from wine/avrt.h
#[no_mangle]
pub unsafe extern "C" fn AvSetMmThreadCharacteristicsA(TaskName: *const i8, TaskIndex: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// AvSetMmThreadCharacteristicsW - from wine/avrt.h
#[no_mangle]
pub unsafe extern "C" fn AvSetMmThreadCharacteristicsW(TaskName: *const u16, TaskIndex: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// AvSetMmMaxThreadCharacteristicsA - from wine/avrt.h
#[no_mangle]
pub unsafe extern "C" fn AvSetMmMaxThreadCharacteristicsA(FirstTask: *const i8, SecondTask: *const i8, TaskIndex: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// AvSetMmMaxThreadCharacteristicsW - from wine/avrt.h
#[no_mangle]
pub unsafe extern "C" fn AvSetMmMaxThreadCharacteristicsW(FirstTask: *const u16, SecondTask: *const u16, TaskIndex: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// AvRevertMmThreadCharacteristics - from wine/avrt.h
#[no_mangle]
pub unsafe extern "C" fn AvRevertMmThreadCharacteristics(AvrtHandle: *mut core::ffi::c_void) -> i32 {
    0
}

/// AvSetMmThreadPriority - from wine/avrt.h
#[no_mangle]
pub unsafe extern "C" fn AvSetMmThreadPriority(AvrtHandle: *mut core::ffi::c_void, Priority: usize) -> i32 {
    0
}

/// AvRtCreateThreadOrderingGroup - from wine/avrt.h
#[no_mangle]
pub unsafe extern "C" fn AvRtCreateThreadOrderingGroup(Context: usize, Period: usize, ThreadOrderingGuid: *mut core::ffi::c_void, Timeout: usize) -> i32 {
    0
}

/// AvRtCreateThreadOrderingGroupExA - from wine/avrt.h
#[no_mangle]
pub unsafe extern "C" fn AvRtCreateThreadOrderingGroupExA(Context: usize, Period: usize, ThreadOrderingGuid: *mut core::ffi::c_void, Timeout: usize, TaskName: *const i8) -> i32 {
    0
}

/// AvRtCreateThreadOrderingGroupExW - from wine/avrt.h
#[no_mangle]
pub unsafe extern "C" fn AvRtCreateThreadOrderingGroupExW(Context: usize, Period: usize, ThreadOrderingGuid: *mut core::ffi::c_void, Timeout: usize, TaskName: *const i8) -> i32 {
    0
}

/// AvRtJoinThreadOrderingGroup - from wine/avrt.h
#[no_mangle]
pub unsafe extern "C" fn AvRtJoinThreadOrderingGroup(Context: usize, ThreadOrderingGuid: *mut core::ffi::c_void, Before: i32) -> i32 {
    0
}

/// AvRtWaitOnThreadOrderingGroup - from wine/avrt.h
#[no_mangle]
pub unsafe extern "C" fn AvRtWaitOnThreadOrderingGroup(Context: *mut core::ffi::c_void) -> i32 {
    0
}

/// AvRtLeaveThreadOrderingGroup - from wine/avrt.h
#[no_mangle]
pub unsafe extern "C" fn AvRtLeaveThreadOrderingGroup(Context: *mut core::ffi::c_void) -> i32 {
    0
}

/// AvRtDeleteThreadOrderingGroup - from wine/avrt.h
#[no_mangle]
pub unsafe extern "C" fn AvRtDeleteThreadOrderingGroup(Context: *mut core::ffi::c_void) -> i32 {
    0
}

/// ImageList_Read - from wine/commctrl.h
#[no_mangle]
pub unsafe extern "C" fn ImageList_Read(IStream: usize) -> usize {
    0
}

/// ImageList_Write - from wine/commctrl.h
#[no_mangle]
pub unsafe extern "C" fn ImageList_Write(arg0: usize, IStream: usize) -> usize {
    0
}

/// ImageList_WriteEx - from wine/commctrl.h
#[no_mangle]
pub unsafe extern "C" fn ImageList_WriteEx(arg0: usize, arg1: u32, IStream: usize) -> usize {
    0
}

/// ReadConsoleA - from wine/consoleapi.h
#[no_mangle]
pub unsafe extern "C" fn ReadConsoleA(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: u32, arg3: *mut u32, arg4: *mut core::ffi::c_void) -> usize {
    0
}

/// ReadConsoleW - from wine/consoleapi.h
#[no_mangle]
pub unsafe extern "C" fn ReadConsoleW(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: u32, arg3: *mut u32, arg4: *mut core::ffi::c_void) -> usize {
    0
}

/// ReadConsoleInputA - from wine/consoleapi.h
#[no_mangle]
pub unsafe extern "C" fn ReadConsoleInputA(arg0: *mut core::ffi::c_void, arg1: usize, arg2: u32, arg3: *mut u32) -> usize {
    0
}

/// ReadConsoleInputW - from wine/consoleapi.h
#[no_mangle]
pub unsafe extern "C" fn ReadConsoleInputW(arg0: *mut core::ffi::c_void, arg1: usize, arg2: u32, arg3: *mut u32) -> usize {
    0
}

/// WriteConsoleA - from wine/consoleapi.h
#[no_mangle]
pub unsafe extern "C" fn WriteConsoleA(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: u32, arg3: *mut u32, arg4: *mut core::ffi::c_void) -> usize {
    0
}

/// WriteConsoleW - from wine/consoleapi.h
#[no_mangle]
pub unsafe extern "C" fn WriteConsoleW(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: u32, arg3: *mut u32, arg4: *mut core::ffi::c_void) -> usize {
    0
}

/// D3DReadFileToBlob - from wine/d3dcompiler.h
#[no_mangle]
pub unsafe extern "C" fn D3DReadFileToBlob(filename: *mut u16, contents: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// D3DWriteBlobToFile - from wine/d3dcompiler.h
#[no_mangle]
pub unsafe extern "C" fn D3DWriteBlobToFile(blob: *mut core::ffi::c_void, filename: *mut u16, overwrite: i32) -> i32 {
    0
}

/// D3DX10CreateThreadPump - from wine/d3dx10core.h
#[no_mangle]
pub unsafe extern "C" fn D3DX10CreateThreadPump(io_threads: u32, proc_threads: u32, pump: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// EventWrite - from wine/evntprov.h
#[no_mangle]
pub unsafe extern "C" fn EventWrite(arg0: usize, arg1: usize, arg2: u32, arg3: usize) -> usize {
    0
}

/// EventWriteString - from wine/evntprov.h
#[no_mangle]
pub unsafe extern "C" fn EventWriteString(arg0: usize, arg1: u8, arg2: u64, arg3: *mut u16) -> usize {
    0
}

/// EventWriteTransfer - from wine/evntprov.h
#[no_mangle]
pub unsafe extern "C" fn EventWriteTransfer(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: u32, arg5: usize) -> usize {
    0
}

/// IsThreadAFiber - from wine/fibersapi.h
#[no_mangle]
pub unsafe extern "C" fn IsThreadAFiber() -> usize {
    0
}

/// ubidi_writeReordered - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ubidi_writeReordered(pBiDi: *mut core::ffi::c_void, dest: *mut core::ffi::c_void, destSize: i32, options: u16, pErrorCode: *mut core::ffi::c_void) -> i32 {
    0
}

/// ubidi_writeReverse - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ubidi_writeReverse(src: *mut core::ffi::c_void, srcLength: i32, dest: *mut core::ffi::c_void, destSize: i32, options: u16, pErrorCode: *mut core::ffi::c_void) -> i32 {
    0
}

/// ucnv_cbFromUWriteBytes - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ucnv_cbFromUWriteBytes(args: *mut core::ffi::c_void, source: *mut i8, length: i32, offsetIndex: i32, err: *mut core::ffi::c_void) {

}

/// ucnv_cbFromUWriteSub - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ucnv_cbFromUWriteSub(args: *mut core::ffi::c_void, offsetIndex: i32, err: *mut core::ffi::c_void) {

}

/// ucnv_cbFromUWriteUChars - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ucnv_cbFromUWriteUChars(args: *mut core::ffi::c_void, source: *mut *mut core::ffi::c_void, sourceLimit: *mut core::ffi::c_void, offsetIndex: i32, err: *mut core::ffi::c_void) {

}

/// ucnv_cbToUWriteSub - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ucnv_cbToUWriteSub(args: *mut core::ffi::c_void, offsetIndex: i32, err: *mut core::ffi::c_void) {

}

/// ucnv_cbToUWriteUChars - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn ucnv_cbToUWriteUChars(args: *mut core::ffi::c_void, source: *mut core::ffi::c_void, length: i32, offsetIndex: i32, err: *mut core::ffi::c_void) {

}

/// NetShareAdd - from wine/lmshare.h
#[no_mangle]
pub unsafe extern "C" fn NetShareAdd(arg0: usize, arg1: u32, arg2: usize, arg3: usize) -> usize {
    0
}

/// LZRead - from wine/lzexpand.h
#[no_mangle]
pub unsafe extern "C" fn LZRead(arg0: i32, arg1: *mut i8, arg2: i32) -> usize {
    0
}

/// HrThisThreadAdviseSink - from wine/mapiutil.h
#[no_mangle]
pub unsafe extern "C" fn HrThisThreadAdviseSink(arg0: usize, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// MiniDumpWriteDump - from wine/minidumpapiset.h
#[no_mangle]
pub unsafe extern "C" fn MiniDumpWriteDump(process: *mut core::ffi::c_void, pid: u32, hfile: *mut core::ffi::c_void, dumptype: usize, ExceptionParam: usize, UserStreamParam: usize, CallbackParam: usize) -> i32 {
    0
}

/// MiniDumpReadDumpStream - from wine/minidumpapiset.h
#[no_mangle]
pub unsafe extern "C" fn MiniDumpReadDumpStream(base: *mut core::ffi::c_void, index: u32, dir: *mut core::ffi::c_void, streamptr: *mut *mut core::ffi::c_void, stream_size: *mut u32) -> i32 {
    0
}

/// waveOutWrite - from wine/mmsystem.h
#[no_mangle]
pub unsafe extern "C" fn waveOutWrite(arg0: usize, arg1: *mut core::ffi::c_void, arg2: u32) -> usize {
    0
}

/// mmioRead - from wine/mmsystem.h
#[no_mangle]
pub unsafe extern "C" fn mmioRead(arg0: usize, arg1: usize, arg2: i32) -> usize {
    0
}

/// mmioWrite - from wine/mmsystem.h
#[no_mangle]
pub unsafe extern "C" fn mmioWrite(arg0: usize, arg1: usize, arg2: i32) -> usize {
    0
}

/// CryptCATCDFClose - from wine/mscat.h
#[no_mangle]
pub unsafe extern "C" fn CryptCATCDFClose(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// CryptCATCDFOpen - from wine/mscat.h
#[no_mangle]
pub unsafe extern "C" fn CryptCATCDFOpen(arg0: *mut u16, arg1: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// MsiRecordReadStream - from wine/msiquery.h
#[no_mangle]
pub unsafe extern "C" fn MsiRecordReadStream(arg0: usize, arg1: u32, arg2: *mut i8, arg3: usize) -> u32 {
    0
}

/// GetCurrentThreadCompartmentId - from wine/netioapi.h
#[no_mangle]
pub unsafe extern "C" fn GetCurrentThreadCompartmentId() -> usize {
    0
}

/// SetCurrentThreadCompartmentId - from wine/netioapi.h
#[no_mangle]
pub unsafe extern "C" fn SetCurrentThreadCompartmentId(arg0: usize) -> usize {
    0
}

/// NtUserAttachThreadInput - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserAttachThreadInput(from: u32, to: u32, attach: i32) -> usize {
    0
}

/// NtUserDisableThreadIme - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserDisableThreadIme(thread_id: u32) -> usize {
    0
}

/// NtUserEnableMouseInPointerForThread - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserEnableMouseInPointerForThread() -> usize {
    0
}

/// NtUserGetGUIThreadInfo - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserGetGUIThreadInfo(id: u32, info: *mut core::ffi::c_void) -> usize {
    0
}

/// NtUserGetThreadDesktop - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserGetThreadDesktop(thread: u32) -> usize {
    0
}

/// NtUserGetThreadState - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserGetThreadState(cls: usize) -> usize {
    0
}

/// NtUserPostThreadMessage - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserPostThreadMessage(thread: u32, msg: u32, wparam: usize, lparam: isize) -> usize {
    0
}

/// NtUserSetThreadDesktop - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserSetThreadDesktop(handle: usize) -> usize {
    0
}

/// NtUserSetThreadDpiAwarenessContext - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserSetThreadDpiAwarenessContext(context: u32) -> u32 {
    0
}

/// NtUserGetWindowThread - from wine/ntuser.h
#[no_mangle]
pub unsafe extern "C" fn NtUserGetWindowThread(hwnd: *mut core::ffi::c_void, process: *mut u32) -> u32 {
    0
}

/// CoGetCurrentLogicalThreadId - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CoGetCurrentLogicalThreadId(id: *mut core::ffi::c_void) -> usize {
    0
}

/// CoMarshalInterThreadInterfaceInStream - from wine/objbase.h
#[no_mangle]
pub unsafe extern "C" fn CoMarshalInterThreadInterfaceInStream(riid: usize, pUnk: usize, ppStm: *mut core::ffi::c_void) -> usize {
    0
}

/// SQLReadFileDSN - from wine/odbcinst.h
#[no_mangle]
pub unsafe extern "C" fn SQLReadFileDSN(arg0: *const i8, arg1: *const i8, arg2: *const i8, arg3: *mut i8, arg4: u16, arg5: *mut u16) -> i32 {
    0
}

/// SQLReadFileDSNW - from wine/odbcinst.h
#[no_mangle]
pub unsafe extern "C" fn SQLReadFileDSNW(arg0: *const u16, arg1: *const u16, arg2: *const u16, arg3: *mut u16, arg4: u16, arg5: *mut u16) -> i32 {
    0
}

/// SQLWriteDSNToIni - from wine/odbcinst.h
#[no_mangle]
pub unsafe extern "C" fn SQLWriteDSNToIni(arg0: *const i8, arg1: *const i8) -> i32 {
    0
}

/// SQLWriteDSNToIniW - from wine/odbcinst.h
#[no_mangle]
pub unsafe extern "C" fn SQLWriteDSNToIniW(arg0: *const u16, arg1: *const u16) -> i32 {
    0
}

/// SQLWriteFileDSN - from wine/odbcinst.h
#[no_mangle]
pub unsafe extern "C" fn SQLWriteFileDSN(arg0: *const i8, arg1: *const i8, arg2: *const i8, arg3: *const i8) -> i32 {
    0
}

/// SQLWriteFileDSNW - from wine/odbcinst.h
#[no_mangle]
pub unsafe extern "C" fn SQLWriteFileDSNW(arg0: *const u16, arg1: *const u16, arg2: *const u16, arg3: *const u16) -> i32 {
    0
}

/// SQLWritePrivateProfileString - from wine/odbcinst.h
#[no_mangle]
pub unsafe extern "C" fn SQLWritePrivateProfileString(arg0: *const i8, arg1: *const i8, arg2: *const i8, arg3: *const i8) -> i32 {
    0
}

/// SQLWritePrivateProfileStringW - from wine/odbcinst.h
#[no_mangle]
pub unsafe extern "C" fn SQLWritePrivateProfileStringW(arg0: *const u16, arg1: *const u16, arg2: *const u16, arg3: *const u16) -> i32 {
    0
}

/// WriteClassStg - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn WriteClassStg(pstg: *mut core::ffi::c_void, rclsid: usize) -> usize {
    0
}

/// ReadClassStg - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn ReadClassStg(pstg: *mut core::ffi::c_void, pclsid: *mut core::ffi::c_void) -> usize {
    0
}

/// WriteClassStm - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn WriteClassStm(pStm: *mut core::ffi::c_void, rclsid: usize) -> usize {
    0
}

/// ReadClassStm - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn ReadClassStm(pStm: *mut core::ffi::c_void, pclsid: *mut core::ffi::c_void) -> usize {
    0
}

/// ReadFmtUserTypeStg - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn ReadFmtUserTypeStg(pstg: usize, pcf: *mut core::ffi::c_void, lplpszUserType: *mut core::ffi::c_void) -> usize {
    0
}

/// WriteFmtUserTypeStg - from wine/ole2.h
#[no_mangle]
pub unsafe extern "C" fn WriteFmtUserTypeStg(pstg: usize, cf: usize, lpszUserType: usize) -> usize {
    0
}

/// PerfCloseQueryHandle - from wine/perflib.h
#[no_mangle]
pub unsafe extern "C" fn PerfCloseQueryHandle(arg0: *mut core::ffi::c_void) -> u32 {
    0
}

/// PerfOpenQueryHandle - from wine/perflib.h
#[no_mangle]
pub unsafe extern "C" fn PerfOpenQueryHandle(arg0: *mut u16, arg1: *mut *mut core::ffi::c_void) -> u32 {
    0
}

/// CanUserWritePwrScheme - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn CanUserWritePwrScheme(arg0: usize) -> usize {
    0
}

/// PowerWriteACValueIndex - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn PowerWriteACValueIndex(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void, arg4: u32) -> u32 {
    0
}

/// ReadGlobalPwrPolicy - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn ReadGlobalPwrPolicy(arg0: usize) -> usize {
    0
}

/// ReadProcessorPwrScheme - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn ReadProcessorPwrScheme(arg0: u32, arg1: usize) -> usize {
    0
}

/// ReadPwrScheme - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn ReadPwrScheme(arg0: u32, arg1: usize) -> usize {
    0
}

/// WriteGlobalPwrPolicy - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn WriteGlobalPwrPolicy(arg0: usize) -> usize {
    0
}

/// WriteProcessorPwrScheme - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn WriteProcessorPwrScheme(arg0: u32, arg1: usize) -> usize {
    0
}

/// WritePwrScheme - from wine/powrprof.h
#[no_mangle]
pub unsafe extern "C" fn WritePwrScheme(arg0: usize, arg1: *mut u16, arg2: *mut u16, arg3: usize) -> usize {
    0
}

/// GetCurrentThreadStackLimits - from wine/processthreadsapi.h
#[no_mangle]
pub unsafe extern "C" fn GetCurrentThreadStackLimits(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// GetThreadDescription - from wine/processthreadsapi.h
#[no_mangle]
pub unsafe extern "C" fn GetThreadDescription(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// SetThreadDescription - from wine/processthreadsapi.h
#[no_mangle]
pub unsafe extern "C" fn SetThreadDescription(arg0: *mut core::ffi::c_void, arg1: usize) -> usize {
    0
}

/// SetThreadInformation - from wine/processthreadsapi.h
#[no_mangle]
pub unsafe extern "C" fn SetThreadInformation(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut core::ffi::c_void, arg3: u32) -> usize {
    0
}

/// QueryThreadCycleTime - from wine/realtimeapiset.h
#[no_mangle]
pub unsafe extern "C" fn QueryThreadCycleTime(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// RpcAsyncCleanupThread - from wine/rpcasync.h
#[no_mangle]
pub unsafe extern "C" fn RpcAsyncCleanupThread(arg0: u32) -> usize {
    0
}

/// RpcCancelThread - from wine/rpcdce.h
#[no_mangle]
pub unsafe extern "C" fn RpcCancelThread(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// RpcCancelThreadEx - from wine/rpcdce.h
#[no_mangle]
pub unsafe extern "C" fn RpcCancelThreadEx(arg0: *mut core::ffi::c_void, arg1: i32) -> usize {
    0
}

/// I_GetThreadWindowHandle - from wine/rpcdcep.h
#[no_mangle]
pub unsafe extern "C" fn I_GetThreadWindowHandle(hWnd: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// I_RpcSetThreadParams - from wine/rpcdcep.h
#[no_mangle]
pub unsafe extern "C" fn I_RpcSetThreadParams(fClientFree: i32, Context: *mut core::ffi::c_void, hWndClient: *mut core::ffi::c_void) -> usize {
    0
}

/// SetupScanFileQueueA - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupScanFileQueueA(arg0: usize, arg1: u32, arg2: *mut core::ffi::c_void, arg3: usize, arg4: *mut core::ffi::c_void, arg5: usize) -> usize {
    0
}

/// SetupScanFileQueueW - from wine/setupapi.h
#[no_mangle]
pub unsafe extern "C" fn SetupScanFileQueueW(arg0: usize, arg1: u32, arg2: *mut core::ffi::c_void, arg3: usize, arg4: *mut core::ffi::c_void, arg5: usize) -> usize {
    0
}

/// SHEnumerateUnreadMailAccountsA - from wine/shellapi.h
#[no_mangle]
pub unsafe extern "C" fn SHEnumerateUnreadMailAccountsA(arg0: *mut core::ffi::c_void, arg1: u32, arg2: *mut i8, arg3: i32) -> usize {
    0
}

/// SHEnumerateUnreadMailAccountsW - from wine/shellapi.h
#[no_mangle]
pub unsafe extern "C" fn SHEnumerateUnreadMailAccountsW(arg0: *mut core::ffi::c_void, arg1: u32, arg2: *mut u16, arg3: i32) -> usize {
    0
}

/// SHPathPrepareForWriteA - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHPathPrepareForWriteA(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *const i8, arg3: u32) -> usize {
    0
}

/// SHPathPrepareForWriteW - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn SHPathPrepareForWriteW(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *const u16, arg3: u32) -> usize {
    0
}

/// ReadCabinetState - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn ReadCabinetState(arg0: *mut core::ffi::c_void, arg1: i32) -> usize {
    0
}

/// WriteCabinetState - from wine/shlobj.h
#[no_mangle]
pub unsafe extern "C" fn WriteCabinetState(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// SHRegWriteUSValueA - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHRegWriteUSValueA(arg0: usize, arg1: *const i8, arg2: u32, arg3: *mut core::ffi::c_void, arg4: u32, arg5: u32) -> usize {
    0
}

/// SHRegWriteUSValueW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHRegWriteUSValueW(arg0: usize, arg1: *const u16, arg2: u32, arg3: *mut core::ffi::c_void, arg4: u32, arg5: u32) -> usize {
    0
}

/// wvnsprintfA - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn wvnsprintfA(arg0: *mut i8, arg1: i32, arg2: *const i8, arg3: usize) -> usize {
    0
}

/// wvnsprintfW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn wvnsprintfW(arg0: *mut u16, arg1: i32, arg2: *const u16, arg3: usize) -> usize {
    0
}

/// wnsprintfA - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn wnsprintfA(arg0: *mut i8, arg1: i32, arg2: *const i8) -> usize {
    0
}

/// wnsprintfW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn wnsprintfW(arg0: *mut u16, arg1: i32, arg2: *const u16) -> usize {
    0
}

/// SHCreateThreadRef - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateThreadRef(arg0: *mut i32, arg1: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SHGetThreadRef - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHGetThreadRef(arg0: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// SHSetThreadRef - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHSetThreadRef(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// SHReleaseThreadRef - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHReleaseThreadRef() -> usize {
    0
}

/// SHCreateThread - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn SHCreateThread(arg0: usize, arg1: *mut core::ffi::c_void, arg2: u32, arg3: usize) -> usize {
    0
}

/// SspiPrepareForCredWrite - from wine/sspi.h
#[no_mangle]
pub unsafe extern "C" fn SspiPrepareForCredWrite(arg0: usize, arg1: usize, arg2: usize, arg3: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void, arg5: *mut core::ffi::c_void, arg6: usize) -> usize {
    0
}

/// StringVPrintfWorkerA - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringVPrintfWorkerA(pszDest: usize, cchDest: usize, pszFormat: usize, argList: usize) -> usize {
    0
}

/// StringVPrintfWorkerW - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringVPrintfWorkerW(pszDest: usize, cchDest: usize, pszFormat: usize, argList: usize) -> usize {
    0
}

/// StringVPrintfExWorkerA - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringVPrintfExWorkerA(pszDest: usize, cchDest: usize, cbDest: usize, ppszDestEnd: *mut core::ffi::c_void, pcchRemaining: *mut usize, dwFlags: u32, pszFormat: usize, argList: usize) -> usize {
    0
}

/// StringVPrintfExWorkerW - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringVPrintfExWorkerW(pszDest: usize, cchDest: usize, cbDest: usize, ppszDestEnd: *mut core::ffi::c_void, pcchRemaining: *mut usize, dwFlags: u32, pszFormat: usize, argList: usize) -> usize {
    0
}

/// StringCchVPrintfA - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCchVPrintfA(pszDest: usize, cchDest: usize, pszFormat: usize, argList: usize) -> usize {
    0
}

/// StringCchVPrintfW - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCchVPrintfW(pszDest: usize, cchDest: usize, pszFormat: usize, argList: usize) -> usize {
    0
}

/// StringCbVPrintfA - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCbVPrintfA(pszDest: usize, cbDest: usize, pszFormat: usize, argList: usize) -> usize {
    0
}

/// StringCbVPrintfW - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCbVPrintfW(pszDest: usize, cbDest: usize, pszFormat: usize, argList: usize) -> usize {
    0
}

/// StringCchPrintfA - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCchPrintfA(pszDest: usize, cchDest: usize, pszFormat: usize) -> usize {
    0
}

/// StringCchPrintfW - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCchPrintfW(pszDest: usize, cchDest: usize, pszFormat: usize) -> usize {
    0
}

/// StringCbPrintfA - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCbPrintfA(pszDest: usize, cbDest: usize, pszFormat: usize) -> usize {
    0
}

/// StringCbPrintfW - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCbPrintfW(pszDest: usize, cbDest: usize, pszFormat: usize) -> usize {
    0
}

/// StringCchPrintfExA - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCchPrintfExA(pszDest: usize, cchDest: usize, ppszDestEnd: *mut core::ffi::c_void, pcchRemaining: *mut usize, dwFlags: u32, pszFormat: usize) -> usize {
    0
}

/// StringCchPrintfExW - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCchPrintfExW(pszDest: usize, cchDest: usize, ppszDestEnd: *mut core::ffi::c_void, pcchRemaining: *mut usize, dwFlags: u32, pszFormat: usize) -> usize {
    0
}

/// StringCbPrintfExA - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCbPrintfExA(pszDest: usize, cbDest: usize, ppszDestEnd: *mut core::ffi::c_void, pcbRemaining: *mut usize, dwFlags: u32, pszFormat: usize) -> usize {
    0
}

/// StringCbPrintfExW - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCbPrintfExW(pszDest: usize, cbDest: usize, ppszDestEnd: *mut core::ffi::c_void, pcbRemaining: *mut usize, dwFlags: u32, pszFormat: usize) -> usize {
    0
}

/// StringCchVPrintfExA - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCchVPrintfExA(pszDest: usize, cchDest: usize, ppszDestEnd: *mut core::ffi::c_void, pcchRemaining: *mut usize, dwFlags: u32, pszFormat: usize, argList: usize) -> usize {
    0
}

/// StringCchVPrintfExW - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCchVPrintfExW(pszDest: usize, cchDest: usize, ppszDestEnd: *mut core::ffi::c_void, pcchRemaining: *mut usize, dwFlags: u32, pszFormat: usize, argList: usize) -> usize {
    0
}

/// StringCbVPrintfExA - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCbVPrintfExA(pszDest: usize, cbDest: usize, ppszDestEnd: *mut core::ffi::c_void, pcbRemaining: *mut usize, dwFlags: u32, pszFormat: usize, argList: usize) -> usize {
    0
}

/// StringCbVPrintfExW - from wine/strsafe.h
#[no_mangle]
pub unsafe extern "C" fn StringCbVPrintfExW(pszDest: usize, cbDest: usize, ppszDestEnd: *mut core::ffi::c_void, pcbRemaining: *mut usize, dwFlags: u32, pszFormat: usize, argList: usize) -> usize {
    0
}

/// linePrepareAddToConference - from wine/tapi.h
#[no_mangle]
pub unsafe extern "C" fn linePrepareAddToConference(arg0: usize, arg1: usize, arg2: usize) -> u32 {
    0
}

/// linePrepareAddToConferenceA - from wine/tapi.h
#[no_mangle]
pub unsafe extern "C" fn linePrepareAddToConferenceA(arg0: usize, arg1: usize, arg2: usize) -> u32 {
    0
}

/// CancelThreadpoolIo - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn CancelThreadpoolIo(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// CloseThreadpool - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn CloseThreadpool(arg0: usize) -> usize {
    0
}

/// CloseThreadpoolCleanupGroup - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn CloseThreadpoolCleanupGroup(arg0: usize) -> usize {
    0
}

/// CloseThreadpoolCleanupGroupMembers - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn CloseThreadpoolCleanupGroupMembers(arg0: usize, arg1: i32, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// CloseThreadpoolIo - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn CloseThreadpoolIo(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// CloseThreadpoolTimer - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn CloseThreadpoolTimer(arg0: usize) -> usize {
    0
}

/// CloseThreadpoolWait - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn CloseThreadpoolWait(arg0: usize) -> usize {
    0
}

/// CloseThreadpoolWork - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn CloseThreadpoolWork(arg0: usize) -> usize {
    0
}

/// CreateThreadpool - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn CreateThreadpool(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateThreadpoolCleanupGroup - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn CreateThreadpoolCleanupGroup() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateThreadpoolIo - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn CreateThreadpoolIo(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateThreadpoolTimer - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn CreateThreadpoolTimer(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateThreadpoolWait - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn CreateThreadpoolWait(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// CreateThreadpoolWork - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn CreateThreadpoolWork(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// DisassociateCurrentThreadFromCallback - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn DisassociateCurrentThreadFromCallback(arg0: usize) -> usize {
    0
}

/// IsThreadpoolTimerSet - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn IsThreadpoolTimerSet(arg0: usize) -> usize {
    0
}

/// QueryThreadpoolStackInformation - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn QueryThreadpoolStackInformation(arg0: usize, arg1: usize) -> usize {
    0
}

/// SetThreadpoolStackInformation - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn SetThreadpoolStackInformation(arg0: usize, arg1: usize) -> usize {
    0
}

/// SetThreadpoolThreadMaximum - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn SetThreadpoolThreadMaximum(arg0: usize, arg1: u32) -> usize {
    0
}

/// SetThreadpoolThreadMinimum - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn SetThreadpoolThreadMinimum(arg0: usize, arg1: u32) -> usize {
    0
}

/// SetThreadpoolTimer - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn SetThreadpoolTimer(arg0: usize, arg1: *mut core::ffi::c_void, arg2: u32, arg3: u32) -> usize {
    0
}

/// SetThreadpoolWait - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn SetThreadpoolWait(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// StartThreadpoolIo - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn StartThreadpoolIo(arg0: *mut core::ffi::c_void) -> usize {
    0
}

/// SubmitThreadpoolWork - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn SubmitThreadpoolWork(arg0: usize) -> usize {
    0
}

/// TrySubmitThreadpoolCallback - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn TrySubmitThreadpoolCallback(arg0: usize, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// WaitForThreadpoolIoCallbacks - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn WaitForThreadpoolIoCallbacks(arg0: *mut core::ffi::c_void, arg1: i32) -> usize {
    0
}

/// WaitForThreadpoolTimerCallbacks - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn WaitForThreadpoolTimerCallbacks(arg0: usize, arg1: i32) -> usize {
    0
}

/// WaitForThreadpoolWaitCallbacks - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn WaitForThreadpoolWaitCallbacks(arg0: usize, arg1: i32) -> usize {
    0
}

/// WaitForThreadpoolWorkCallbacks - from wine/threadpoolapiset.h
#[no_mangle]
pub unsafe extern "C" fn WaitForThreadpoolWorkCallbacks(arg0: usize, arg1: i32) -> usize {
    0
}

/// Thread32First - from wine/tlhelp32.h
#[no_mangle]
pub unsafe extern "C" fn Thread32First(arg0: *mut core::ffi::c_void, arg1: usize) -> i32 {
    0
}

/// Thread32Next - from wine/tlhelp32.h
#[no_mangle]
pub unsafe extern "C" fn Thread32Next(arg0: *mut core::ffi::c_void, arg1: usize) -> i32 {
    0
}

/// Toolhelp32ReadProcessMemory - from wine/tlhelp32.h
#[no_mangle]
pub unsafe extern "C" fn Toolhelp32ReadProcessMemory(arg0: u32, arg1: *const core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: usize, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// AVIStreamReadFormat - from wine/vfw.h
#[no_mangle]
pub unsafe extern "C" fn AVIStreamReadFormat(iface: usize, pos: i32, format: *mut core::ffi::c_void, formatsize: *mut i32) -> i32 {
    0
}

/// AVIStreamRead - from wine/vfw.h
#[no_mangle]
pub unsafe extern "C" fn AVIStreamRead(iface: usize, start: i32, samples: i32, buffer: *mut core::ffi::c_void, buffersize: i32, bytesread: *mut i32, samplesread: *mut i32) -> i32 {
    0
}

/// AVIStreamWrite - from wine/vfw.h
#[no_mangle]
pub unsafe extern "C" fn AVIStreamWrite(iface: usize, start: i32, samples: i32, buffer: *mut core::ffi::c_void, buffersize: i32, flags: u32, sampwritten: *mut i32, byteswritten: *mut i32) -> i32 {
    0
}

/// AVIStreamReadData - from wine/vfw.h
#[no_mangle]
pub unsafe extern "C" fn AVIStreamReadData(iface: usize, fcc: u32, lp: *mut core::ffi::c_void, lpread: *mut i32) -> i32 {
    0
}

/// AVIStreamWriteData - from wine/vfw.h
#[no_mangle]
pub unsafe extern "C" fn AVIStreamWriteData(iface: usize, fcc: u32, lp: *mut core::ffi::c_void, size: i32) -> i32 {
    0
}

/// AVIFileWriteData - from wine/vfw.h
#[no_mangle]
pub unsafe extern "C" fn AVIFileWriteData(pfile: usize, fcc: u32, lp: *mut core::ffi::c_void, size: i32) -> i32 {
    0
}

/// AVIFileReadData - from wine/vfw.h
#[no_mangle]
pub unsafe extern "C" fn AVIFileReadData(pfile: usize, fcc: u32, lp: *mut core::ffi::c_void, size: usize) -> i32 {
    0
}

/// CloseThreadWaitChainSession - from wine/wct.h
#[no_mangle]
pub unsafe extern "C" fn CloseThreadWaitChainSession(arg0: usize) -> usize {
    0
}

/// GetThreadWaitChain - from wine/wct.h
#[no_mangle]
pub unsafe extern "C" fn GetThreadWaitChain(arg0: usize, arg1: usize, arg2: u32, arg3: u32, arg4: usize, arg5: usize, arg6: usize) -> i32 {
    0
}

/// OpenThreadWaitChainSession - from wine/wct.h
#[no_mangle]
pub unsafe extern "C" fn OpenThreadWaitChainSession(arg0: u32, arg1: usize) -> usize {
    0
}

/// WsCreateReader - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsCreateReader(arg0: *mut core::ffi::c_void, arg1: u32, arg2: *mut *mut core::ffi::c_void, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsCreateWriter - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsCreateWriter(arg0: *mut core::ffi::c_void, arg1: u32, arg2: *mut *mut core::ffi::c_void, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsFillReader - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsFillReader(arg0: *mut core::ffi::c_void, arg1: u32, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsFlushWriter - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsFlushWriter(arg0: *mut core::ffi::c_void, arg1: u32, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsGetReaderNode - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsGetReaderNode(arg0: *mut core::ffi::c_void, arg1: *mut *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsGetReaderPosition - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsGetReaderPosition(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsGetReaderProperty - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsGetReaderProperty(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut core::ffi::c_void, arg3: u32, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsGetWriterPosition - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsGetWriterPosition(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsGetWriterProperty - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsGetWriterProperty(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut core::ffi::c_void, arg3: u32, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsMoveReader - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsMoveReader(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut i32, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsMoveWriter - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsMoveWriter(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut i32, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadAttribute - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadAttribute(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize, arg3: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void, arg5: u32, arg6: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadBody - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadBody(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize, arg3: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void, arg5: u32, arg6: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadBytes - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadBytes(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: u32, arg3: *mut u32, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadChars - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadChars(arg0: *mut core::ffi::c_void, arg1: *mut u16, arg2: u32, arg3: *mut u32, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadCharsUtf8 - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadCharsUtf8(arg0: *mut core::ffi::c_void, arg1: *mut u8, arg2: u32, arg3: *mut u32, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadElement - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadElement(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize, arg3: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void, arg5: u32, arg6: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadEndAttribute - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadEndAttribute(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadEndElement - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadEndElement(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadEnvelopeEnd - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadEnvelopeEnd(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadEnvelopeStart - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadEnvelopeStart(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize, arg3: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadMessageEnd - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadMessageEnd(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadMessageStart - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadMessageStart(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadNode - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadNode(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadQualifiedName - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadQualifiedName(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void, arg5: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadStartAttribute - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadStartAttribute(arg0: *mut core::ffi::c_void, arg1: u32, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadStartElement - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadStartElement(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadToStartElement - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadToStartElement(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut i32, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadType - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadType(arg0: *mut core::ffi::c_void, arg1: usize, arg2: usize, arg3: *mut core::ffi::c_void, arg4: usize, arg5: *mut core::ffi::c_void, arg6: *mut core::ffi::c_void, arg7: u32, arg8: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadValue - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadValue(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut core::ffi::c_void, arg3: u32, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsReadXmlBuffer - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsReadXmlBuffer(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut *mut core::ffi::c_void, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsSetReaderPosition - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsSetReaderPosition(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsSetWriterPosition - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsSetWriterPosition(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteArray - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteArray(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: usize, arg4: *mut core::ffi::c_void, arg5: u32, arg6: u32, arg7: u32, arg8: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteAttribute - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteAttribute(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize, arg3: *mut core::ffi::c_void, arg4: u32, arg5: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteBody - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteBody(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize, arg3: *mut core::ffi::c_void, arg4: u32, arg5: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteBytes - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteBytes(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: u32, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteChars - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteChars(arg0: *mut core::ffi::c_void, arg1: *mut u16, arg2: u32, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteCharsUtf8 - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteCharsUtf8(arg0: *mut core::ffi::c_void, arg1: *mut u8, arg2: u32, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteElement - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteElement(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize, arg3: *mut core::ffi::c_void, arg4: u32, arg5: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteEndAttribute - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteEndAttribute(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteEndCData - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteEndCData(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteEndElement - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteEndElement(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteEndStartElement - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteEndStartElement(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteEnvelopeEnd - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteEnvelopeEnd(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteEnvelopeStart - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteEnvelopeStart(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize, arg3: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteMessageStart - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteMessageStart(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteMessageEnd - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteMessageEnd(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteNode - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteNode(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteQualifiedName - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteQualifiedName(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteStartAttribute - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteStartAttribute(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void, arg4: i32, arg5: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteStartCData - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteStartCData(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteStartElement - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteStartElement(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteText - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteText(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteType - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteType(arg0: *mut core::ffi::c_void, arg1: usize, arg2: usize, arg3: *mut core::ffi::c_void, arg4: usize, arg5: *mut core::ffi::c_void, arg6: u32, arg7: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteValue - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteValue(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut core::ffi::c_void, arg3: u32, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteXmlBuffer - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteXmlBuffer(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteXmlBufferToBytes - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteXmlBufferToBytes(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void, arg4: u32, arg5: *mut core::ffi::c_void, arg6: *mut *mut core::ffi::c_void, arg7: *mut u32, arg8: *mut core::ffi::c_void) -> i32 {
    0
}

/// WsWriteXmlnsAttribute - from wine/webservices.h
#[no_mangle]
pub unsafe extern "C" fn WsWriteXmlnsAttribute(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, arg3: i32, arg4: *mut core::ffi::c_void) -> i32 {
    0
}

/// ber_printf - from wine/winber.h
#[no_mangle]
pub unsafe extern "C" fn ber_printf(arg0: *mut core::ffi::c_void, arg1: *mut i8) -> i32 {
    0
}

/// ber_scanf - from wine/winber.h
#[no_mangle]
pub unsafe extern "C" fn ber_scanf(arg0: *mut core::ffi::c_void, arg1: *mut i8) -> u32 {
    0
}

/// ReadConsoleOutputA - from wine/wincon.h
#[no_mangle]
pub unsafe extern "C" fn ReadConsoleOutputA(arg0: *mut core::ffi::c_void, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// ReadConsoleOutputW - from wine/wincon.h
#[no_mangle]
pub unsafe extern "C" fn ReadConsoleOutputW(arg0: *mut core::ffi::c_void, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// ReadConsoleOutputAttribute - from wine/wincon.h
#[no_mangle]
pub unsafe extern "C" fn ReadConsoleOutputAttribute(arg0: *mut core::ffi::c_void, arg1: usize, arg2: u32, arg3: usize, arg4: usize) -> usize {
    0
}

/// ReadConsoleOutputCharacterA - from wine/wincon.h
#[no_mangle]
pub unsafe extern "C" fn ReadConsoleOutputCharacterA(arg0: *mut core::ffi::c_void, arg1: *mut i8, arg2: u32, arg3: usize, arg4: usize) -> usize {
    0
}

/// ReadConsoleOutputCharacterW - from wine/wincon.h
#[no_mangle]
pub unsafe extern "C" fn ReadConsoleOutputCharacterW(arg0: *mut core::ffi::c_void, arg1: *mut u16, arg2: u32, arg3: usize, arg4: usize) -> usize {
    0
}

/// WriteConsoleInputA - from wine/wincon.h
#[no_mangle]
pub unsafe extern "C" fn WriteConsoleInputA(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: u32, arg3: usize) -> usize {
    0
}

/// WriteConsoleInputW - from wine/wincon.h
#[no_mangle]
pub unsafe extern "C" fn WriteConsoleInputW(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: u32, arg3: usize) -> usize {
    0
}

/// WriteConsoleOutputA - from wine/wincon.h
#[no_mangle]
pub unsafe extern "C" fn WriteConsoleOutputA(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// WriteConsoleOutputW - from wine/wincon.h
#[no_mangle]
pub unsafe extern "C" fn WriteConsoleOutputW(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize, arg3: usize, arg4: usize) -> usize {
    0
}

/// WriteConsoleOutputAttribute - from wine/wincon.h
#[no_mangle]
pub unsafe extern "C" fn WriteConsoleOutputAttribute(arg0: *mut core::ffi::c_void, arg1: *mut u16, arg2: u32, arg3: usize, arg4: usize) -> usize {
    0
}

/// WriteConsoleOutputCharacterA - from reactos/console.h
#[no_mangle]
pub unsafe extern "C" fn WriteConsoleOutputCharacterA(hConsoleOutput: *mut core::ffi::c_void, lpCharacter: usize, nLength: usize, dwWriteCoord: usize, lpNumberOfCharsWritten: usize) -> i32 {
    0
}

/// WriteConsoleOutputCharacterW - from reactos/console.h
#[no_mangle]
pub unsafe extern "C" fn WriteConsoleOutputCharacterW(hConsoleOutput: *mut core::ffi::c_void, lpCharacter: usize, nLength: usize, dwWriteCoord: usize, lpNumberOfCharsWritten: usize) -> i32 {
    0
}

/// CredReadA - from wine/wincred.h
#[no_mangle]
pub unsafe extern "C" fn CredReadA(arg0: *const i8, arg1: u32, arg2: u32, arg3: *mut core::ffi::c_void) -> usize {
    0
}

/// CredReadW - from wine/wincred.h
#[no_mangle]
pub unsafe extern "C" fn CredReadW(arg0: *const u16, arg1: u32, arg2: u32, arg3: *mut core::ffi::c_void) -> usize {
    0
}

/// CredReadDomainCredentialsA - from wine/wincred.h
#[no_mangle]
pub unsafe extern "C" fn CredReadDomainCredentialsA(arg0: usize, arg1: u32, arg2: *mut u32, arg3: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CredReadDomainCredentialsW - from wine/wincred.h
#[no_mangle]
pub unsafe extern "C" fn CredReadDomainCredentialsW(arg0: usize, arg1: u32, arg2: *mut u32, arg3: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// CredWriteA - from wine/wincred.h
#[no_mangle]
pub unsafe extern "C" fn CredWriteA(arg0: usize, arg1: u32) -> usize {
    0
}

/// CredWriteW - from wine/wincred.h
#[no_mangle]
pub unsafe extern "C" fn CredWriteW(arg0: usize, arg1: u32) -> usize {
    0
}

/// CredUIReadSSOCredW - from wine/wincred.h
#[no_mangle]
pub unsafe extern "C" fn CredUIReadSSOCredW(arg0: usize, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// DnsWriteQuestionToBuffer_W - from wine/windns.h
#[no_mangle]
pub unsafe extern "C" fn DnsWriteQuestionToBuffer_W(arg0: usize, arg1: usize, arg2: usize, arg3: u16, arg4: u16, arg5: i32) -> i32 {
    0
}

/// DnsWriteQuestionToBuffer_UTF8 - from wine/windns.h
#[no_mangle]
pub unsafe extern "C" fn DnsWriteQuestionToBuffer_UTF8(arg0: usize, arg1: usize, arg2: usize, arg3: u16, arg4: u16, arg5: i32) -> i32 {
    0
}

/// WinHttpReadData - from wine/winhttp.h
#[no_mangle]
pub unsafe extern "C" fn WinHttpReadData(arg0: usize, arg1: *mut core::ffi::c_void, arg2: u32, arg3: usize) -> usize {
    0
}

/// WinHttpReadProxySettings - from wine/winhttp.h
#[no_mangle]
pub unsafe extern "C" fn WinHttpReadProxySettings(arg0: usize, arg1: usize, arg2: i32, arg3: i32, arg4: *mut u32, arg5: *mut i32, arg6: *mut core::ffi::c_void) -> usize {
    0
}

/// WinHttpWriteData - from wine/winhttp.h
#[no_mangle]
pub unsafe extern "C" fn WinHttpWriteData(arg0: usize, arg1: *const core::ffi::c_void, arg2: u32, arg3: usize) -> usize {
    0
}

/// WinHttpWriteProxySettings - from wine/winhttp.h
#[no_mangle]
pub unsafe extern "C" fn WinHttpWriteProxySettings(arg0: usize, arg1: i32, arg2: *mut core::ffi::c_void) -> usize {
    0
}

/// InternetReadFile - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn InternetReadFile(arg0: usize, arg1: *mut core::ffi::c_void, arg2: u32, arg3: usize) -> usize {
    0
}

/// InternetReadFileExA - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn InternetReadFileExA(arg0: usize, arg1: usize, arg2: u32, arg3: usize) -> usize {
    0
}

/// InternetReadFileExW - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn InternetReadFileExW(arg0: usize, arg1: usize, arg2: u32, arg3: usize) -> usize {
    0
}

/// InternetWriteFile - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn InternetWriteFile(arg0: usize, arg1: *const core::ffi::c_void, arg2: u32, arg3: usize) -> usize {
    0
}

/// ReadUrlCacheEntryStream - from wine/wininet.h
#[no_mangle]
pub unsafe extern "C" fn ReadUrlCacheEntryStream(arg0: *mut core::ffi::c_void, arg1: u32, arg2: *mut core::ffi::c_void, arg3: usize, arg4: u32) -> usize {
    0
}

/// GetThreadLocale - from wine/winnls.h
#[no_mangle]
pub unsafe extern "C" fn GetThreadLocale() -> usize {
    0
}

/// GetThreadPreferredUILanguages - from wine/winnls.h
#[no_mangle]
pub unsafe extern "C" fn GetThreadPreferredUILanguages(arg0: u32, arg1: *mut u32, arg2: *mut u16, arg3: *mut u32) -> usize {
    0
}

/// GetThreadUILanguage - from wine/winnls.h
#[no_mangle]
pub unsafe extern "C" fn GetThreadUILanguage() -> usize {
    0
}

/// SetThreadLocale - from wine/winnls.h
#[no_mangle]
pub unsafe extern "C" fn SetThreadLocale(arg0: usize) -> usize {
    0
}

/// SetThreadPreferredUILanguages - from wine/winnls.h
#[no_mangle]
pub unsafe extern "C" fn SetThreadPreferredUILanguages(arg0: u32, arg1: usize, arg2: usize) -> usize {
    0
}

/// SetThreadUILanguage - from wine/winnls.h
#[no_mangle]
pub unsafe extern "C" fn SetThreadUILanguage(arg0: usize) -> usize {
    0
}

/// __readfsdword - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn __readfsdword(arg0: u32) -> u32 {
    0
}

/// __readgsqword - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn __readgsqword(arg0: u32) -> usize {
    0
}

/// _BitScanForward - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _BitScanForward(arg0: u32, arg1: u32) -> usize {
    0
}

/// _BitScanForward64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _BitScanForward64(arg0: u32, __int64: u32) -> usize {
    0
}

/// _ReadWriteBarrier - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn _ReadWriteBarrier() {

}

/// ReadAcquire - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn ReadAcquire(src: *mut i32) -> usize {
    0
}

/// ReadAcquire64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn ReadAcquire64(src: *mut core::ffi::c_void) -> usize {
    0
}

/// ReadNoFence - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn ReadNoFence(src: *mut i32) -> usize {
    0
}

/// ReadNoFence64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn ReadNoFence64(src: *mut core::ffi::c_void) -> usize {
    0
}

/// WriteRelease - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn WriteRelease(dest: *mut i32, value: i32) -> usize {
    0
}

/// WriteRelease64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn WriteRelease64(dest: *mut core::ffi::c_void, value: usize) -> usize {
    0
}

/// WriteNoFence - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn WriteNoFence(dest: *mut i32, value: i32) -> usize {
    0
}

/// WriteNoFence64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn WriteNoFence64(dest: *mut core::ffi::c_void, value: usize) -> usize {
    0
}

/// BitScanForward - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn BitScanForward(index: *mut u32, mask: u32) -> usize {
    0
}

/// BitScanForward64 - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn BitScanForward64(index: *mut u32, mask: usize) -> usize {
    0
}

/// WritePointerRelease - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn WritePointerRelease(dest: *mut *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> usize {
    0
}

/// WritePointerNoFence - from wine/winnt.h
#[no_mangle]
pub unsafe extern "C" fn WritePointerNoFence(dest: *mut *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> usize {
    0
}

/// wld_read - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn wld_read(fd: i32, buffer: *mut core::ffi::c_void, len: usize) -> isize {
    0
}

/// wld_write - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn wld_write(fd: i32, buffer: *mut core::ffi::c_void, len: usize) -> isize {
    0
}

/// wld_vsprintf - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn wld_vsprintf(buffer: *mut i8, fmt: *mut i8, args: usize) -> i32 {
    0
}

/// wld_printf - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn wld_printf(fmt: *mut i8) {

}

/// cancel_terminating_thread_asyncs - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn cancel_terminating_thread_asyncs(thread: *mut core::ffi::c_void) {

}

/// cleanup_clipboard_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn cleanup_clipboard_thread(thread: *mut core::ffi::c_void) {

}

/// cleanup_thread_completion - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn cleanup_thread_completion(thread: *mut core::ffi::c_void) {

}

/// console_read - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn console_read(fd: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, pos: usize) {

}

/// screen_buffer_write - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn screen_buffer_write(fd: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, pos: usize) {

}

/// console_input_read - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn console_input_read(fd: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, pos: usize) {

}

/// console_output_write - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn console_output_write(fd: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, pos: usize) {

}

/// fill_create_thread_event - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn fill_create_thread_event(event: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) {

}

/// fill_exit_thread_event - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn fill_exit_thread_event(event: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) {

}

/// device_file_read - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn device_file_read(fd: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, pos: usize) {

}

/// device_file_write - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn device_file_write(fd: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, pos: usize) {

}

/// perror - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn perror(__s: *mut i8) {

}

/// no_fd_read - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn no_fd_read(fd: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, pos: usize) {

}

/// no_fd_write - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn no_fd_write(fd: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, pos: usize) {

}

/// run_hook_in_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn run_hook_in_thread(hook: *mut core::ffi::c_void, thread: *mut core::ffi::c_void) -> i32 {
    0
}

/// run_hook_in_current_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn run_hook_in_current_thread(hook: *mut core::ffi::c_void) -> i32 {
    0
}

/// remove_thread_hooks - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn remove_thread_hooks(thread: *mut core::ffi::c_void) {

}

/// get_thread_context - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn get_thread_context(thread: *mut core::ffi::c_void, context: *mut core::ffi::c_void, flags: u32) {

}

/// set_thread_context - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn set_thread_context(thread: *mut core::ffi::c_void, context: *mut core::ffi::c_void, flags: u32) {

}

/// send_thread_signal - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn send_thread_signal(thread: *mut core::ffi::c_void, sig: i32) -> i32 {
    0
}

/// read_process_memory - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn read_process_memory(process: *mut core::ffi::c_void, ptr: usize, size: usize, dest: *mut i8) -> i32 {
    0
}

/// write_process_memory - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn write_process_memory(process: *mut core::ffi::c_void, ptr: usize, size: usize, src: *mut i8, written: *mut core::ffi::c_void) -> i32 {
    0
}

/// mailslot_read - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mailslot_read(fd: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, pos: usize) {

}

/// mailslot_write - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mailslot_write(fd: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, pos: usize) {

}

/// mail_writer_dump - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mail_writer_dump(obj: *mut core::ffi::c_void, verbose: i32) {

}

/// mail_writer_map_access - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mail_writer_map_access(obj: *mut core::ffi::c_void, access: u32) -> u32 {
    0
}

/// mail_writer_destroy - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mail_writer_destroy(obj: *mut core::ffi::c_void) {

}

/// mail_writer_get_fd_type - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mail_writer_get_fd_type(fd: *mut core::ffi::c_void) -> usize {
    0
}

/// mail_writer_read - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mail_writer_read(fd: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, pos: usize) {

}

/// mail_writer_write - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn mail_writer_write(fd: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, pos: usize) {

}

/// pread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn pread(arg0: usize, arg1: usize, arg2: usize, seciVirtualAddress: usize) -> usize {
    0
}

/// pipe_end_read - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn pipe_end_read(fd: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, pos: usize) {

}

/// pipe_end_write - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn pipe_end_write(fd: *mut core::ffi::c_void, async_data: *mut core::ffi::c_void, pos: usize) {

}

/// message_queue_read - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn message_queue_read(pipe_end: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) {

}

/// reselect_write_queue - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn reselect_write_queue(pipe_end: *mut core::ffi::c_void) {

}

/// reselect_read_queue - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn reselect_read_queue(pipe_end: *mut core::ffi::c_void, reselect_write: i32) {

}

/// add_process_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn add_process_thread(process: *mut core::ffi::c_void, thread: *mut core::ffi::c_void) {

}

/// remove_process_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn remove_process_thread(process: *mut core::ffi::c_void, thread: *mut core::ffi::c_void) {

}

/// fprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fprintf(__stream: *mut core::ffi::c_void, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// waitpid_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn waitpid_thread(thread: *mut core::ffi::c_void, signal: i32) -> i32 {
    0
}

/// read_thread_long - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn read_thread_long(thread: *mut core::ffi::c_void, addr: *mut core::ffi::c_void, data: *mut u64) -> i32 {
    0
}

/// write_thread_long - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn write_thread_long(thread: *mut core::ffi::c_void, addr: *mut core::ffi::c_void, data: u64, mask: u64) -> i64 {
    0
}

/// read_process_memory_vm - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn read_process_memory_vm(thread: *mut core::ffi::c_void, ptr: usize, size: usize, dest: *mut i8) -> i32 {
    0
}

/// write_process_memory_vm - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn write_process_memory_vm(thread: *mut core::ffi::c_void, ptr: usize, size: usize, src: *mut i8, written: *mut core::ffi::c_void) -> i32 {
    0
}

/// read_process_memory_ptrace - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn read_process_memory_ptrace(thread: *mut core::ffi::c_void, ptr: usize, size: usize, dest: *mut i8) -> i32 {
    0
}

/// check_process_write_access - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn check_process_write_access(thread: *mut core::ffi::c_void, addr: *mut i64, len: usize) -> i32 {
    0
}

/// write_process_memory_ptrace - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn write_process_memory_ptrace(thread: *mut core::ffi::c_void, ptr: usize, size: usize, src: *mut i8, written: *mut core::ffi::c_void) -> i32 {
    0
}

/// init_thread_context - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn init_thread_context(thread: *mut core::ffi::c_void) {

}

/// thread_input_dump - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_input_dump(obj: *mut core::ffi::c_void, verbose: i32) {

}

/// thread_input_destroy - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_input_destroy(obj: *mut core::ffi::c_void) {

}

/// assign_thread_input - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn assign_thread_input(thread: *mut core::ffi::c_void, new_input: *mut core::ffi::c_void) -> i32 {
    0
}

/// thread_input_cleanup_window - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_input_cleanup_window(queue: *mut core::ffi::c_void, window: usize) {

}

/// init_thread_queue - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn init_thread_queue(thread: *mut core::ffi::c_void) -> i32 {
    0
}

/// attach_thread_input - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn attach_thread_input(thread_from: *mut core::ffi::c_void, thread_to: *mut core::ffi::c_void) -> i32 {
    0
}

/// detach_thread_input - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn detach_thread_input(thread_from: *mut core::ffi::c_void) {

}

/// update_thread_input_key_state - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn update_thread_input_key_state(input: *mut core::ffi::c_void, msg: u32, wparam: usize) {

}

/// get_window_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn get_window_thread(arg0: usize) -> usize {
    0
}

/// read_next_line - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn read_next_line(info: *mut core::ffi::c_void) -> i32 {
    0
}

/// file_read_error - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn file_read_error(key: usize, arg1: usize) -> usize {
    0
}

/// write_reply - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn write_reply(thread: *mut core::ffi::c_void) {

}

/// read_request - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn read_request(thread: *mut core::ffi::c_void) {

}

/// serial_read_timeout - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn serial_read_timeout(arg: *mut core::ffi::c_void) {

}

/// dump_thread_apc - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn dump_thread_apc(obj: *mut core::ffi::c_void, verbose: i32) {

}

/// thread_apc_destroy - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_apc_destroy(obj: *mut core::ffi::c_void) {

}

/// dump_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn dump_thread(obj: *mut core::ffi::c_void, verbose: i32) {

}

/// thread_map_access - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_map_access(obj: *mut core::ffi::c_void, access: u32) -> u32 {
    0
}

/// thread_poll_event - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_poll_event(fd: *mut core::ffi::c_void, event: i32) {

}

/// destroy_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn destroy_thread(obj: *mut core::ffi::c_void) {

}

/// init_threading - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn init_threading() {

}

/// apply_thread_priority - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn apply_thread_priority(thread: *mut core::ffi::c_void) {

}

/// init_thread_structure - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn init_thread_structure(thread: *mut core::ffi::c_void) {

}

/// is_thread_suspended - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn is_thread_suspended(thread: *mut core::ffi::c_void) -> i32 {
    0
}

/// cleanup_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn cleanup_thread(thread: *mut core::ffi::c_void) {

}

/// set_thread_affinity - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn set_thread_affinity(thread: *mut core::ffi::c_void, affinity: usize) -> i32 {
    0
}

/// get_thread_affinity - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn get_thread_affinity(thread: *mut core::ffi::c_void) -> usize {
    0
}

/// get_effective_thread_priority - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn get_effective_thread_priority(thread: *mut core::ffi::c_void) -> i32 {
    0
}

/// set_thread_priority - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn set_thread_priority(thread: *mut core::ffi::c_void, priority: i32) -> u32 {
    0
}

/// set_thread_base_priority - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn set_thread_base_priority(thread: *mut core::ffi::c_void, base_priority: i32) -> u32 {
    0
}

/// set_thread_disable_boost - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn set_thread_disable_boost(thread: *mut core::ffi::c_void, disable_boost: i32) {

}

/// stop_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn stop_thread(thread: *mut core::ffi::c_void) {

}

/// suspend_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn suspend_thread(thread: *mut core::ffi::c_void) -> i32 {
    0
}

/// resume_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn resume_thread(thread: *mut core::ffi::c_void) -> i32 {
    0
}

/// send_thread_wakeup - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn send_thread_wakeup(thread: *mut core::ffi::c_void, cookie: usize, signaled: i32) -> i32 {
    0
}

/// wake_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn wake_thread(thread: *mut core::ffi::c_void) -> i32 {
    0
}

/// wake_thread_queue_entry - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn wake_thread_queue_entry(entry: *mut core::ffi::c_void) -> i32 {
    0
}

/// thread_timeout - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_timeout(ptr: *mut core::ffi::c_void) {

}

/// thread_queue_apc - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_queue_apc(process: *mut core::ffi::c_void, thread: *mut core::ffi::c_void, owner: *mut core::ffi::c_void, call_data: *mut core::ffi::c_void) -> i32 {
    0
}

/// thread_cancel_apc - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_cancel_apc(thread: *mut core::ffi::c_void, owner: *mut core::ffi::c_void, arg2: usize) {

}

/// thread_add_inflight_fd - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_add_inflight_fd(thread: *mut core::ffi::c_void, client: i32, server: i32) -> i32 {
    0
}

/// thread_get_inflight_fd - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_get_inflight_fd(thread: *mut core::ffi::c_void, client: i32) -> i32 {
    0
}

/// kill_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn kill_thread(thread: *mut core::ffi::c_void, violent_death: i32) {

}

/// security_set_thread_token - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn security_set_thread_token(thread: *mut core::ffi::c_void, handle: usize) {

}

/// detach_window_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn detach_window_thread(win: *mut core::ffi::c_void) {

}

/// destroy_thread_windows - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn destroy_thread_windows(thread: *mut core::ffi::c_void) {

}

/// add_desktop_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn add_desktop_thread(desktop: *mut core::ffi::c_void, thread: *mut core::ffi::c_void) {

}

/// remove_desktop_thread - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn remove_desktop_thread(desktop: *mut core::ffi::c_void, thread: *mut core::ffi::c_void) {

}

/// set_thread_default_desktop - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn set_thread_default_desktop(thread: *mut core::ffi::c_void, desktop: *mut core::ffi::c_void, handle: usize) {

}

/// release_thread_desktop - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn release_thread_desktop(thread: *mut core::ffi::c_void, close: i32) {

}

/// feof - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn feof(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// read_byte - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn read_byte(byte: *mut i8) -> i32 {
    0
}

/// unread_byte - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn unread_byte(last_byte: i8) -> i32 {
    0
}

/// ungetc - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn ungetc(__c: i32, __stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// read_bytes - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn read_bytes(data: *mut core::ffi::c_void, size: u32) -> i32 {
    0
}

/// fread - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fread(__ptr: *mut core::ffi::c_void, __size: usize, __n: usize, __stream: *mut core::ffi::c_void) -> usize {
    0
}

/// write_c_hex_bytes - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn write_c_hex_bytes() -> i32 {
    0
}

/// write_raw_bytes - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn write_raw_bytes() -> i32 {
    0
}

/// fwrite - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fwrite(__ptr: *mut core::ffi::c_void, __size: usize, __n: usize, __s: *mut core::ffi::c_void) -> usize {
    0
}

/// read_credential_blob - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn read_credential_blob(hkey: *mut core::ffi::c_void, key_dataKEY_SIZE: u8, credential_blob: usize, credential_blob_size: *mut u32) -> u32 {
    0
}

/// registry_read_credential - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn registry_read_credential(hkey: *mut core::ffi::c_void, credential: usize, key_dataKEY_SIZE: u8, buffer: *mut i8, len: *mut u32) -> u32 {
    0
}

/// write_credential_blob - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn write_credential_blob(hkey: *mut core::ffi::c_void, target_name: *const u16, arg2: u32, key_dataKEY_SIZE: u8, credential_blob: *mut u8, credential_blob_size: u32) -> u32 {
    0
}

/// registry_write_credential - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn registry_write_credential(hkey: *mut core::ffi::c_void, credential: *mut core::ffi::c_void, key_dataKEY_SIZE: u8, preserve_blob: i32) -> u32 {
    0
}

/// host_write_credential - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn host_write_credential(credential: *mut core::ffi::c_void, preserve_blob: i32) -> u32 {
    0
}

/// host_read_credential - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn host_read_credential(targetname: *mut u16, ret_credential: *mut *mut core::ffi::c_void) -> u32 {
    0
}

/// wine_dbg_sprintf - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn wine_dbg_sprintf(arg0: usize, arg1: usize) -> usize {
    0
}

/// write_predefined_strings - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn write_predefined_strings(hm: *mut core::ffi::c_void, ini_path: *const u16) -> i32 {
    0
}

/// ReadApplicationsFromRegistry - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ReadApplicationsFromRegistry(root: *mut core::ffi::c_void) -> i32 {
    0
}

/// strbuf_write - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn strbuf_write(str: usize, buf: *mut core::ffi::c_void, len: i32) {

}

/// ACMStream_fnReadFormat - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ACMStream_fnReadFormat(iface: *mut core::ffi::c_void, pos: i32, format: *mut core::ffi::c_void, formatsize: *mut i32) -> i32 {
    0
}

/// ACMStream_fnRead - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ACMStream_fnRead(iface: *mut core::ffi::c_void, start: i32, samples: i32, buffer: *mut core::ffi::c_void, buffersize: i32, bytesread: usize, samplesread: usize) -> i32 {
    0
}

/// IAVIStream_Read - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIStream_Read(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize) -> usize {
    0
}

/// ACMStream_fnWrite - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ACMStream_fnWrite(iface: *mut core::ffi::c_void, start: i32, samples: i32, buffer: *mut core::ffi::c_void, buffersize: i32, flags: u32, sampwritten: usize, byteswritten: usize) -> i32 {
    0
}

/// IAVIStream_Write - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIStream_Write(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, arg6: usize, arg7: usize) -> usize {
    0
}

/// ACMStream_fnReadData - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ACMStream_fnReadData(iface: *mut core::ffi::c_void, fcc: u32, lp: *mut core::ffi::c_void, lpread: usize) -> i32 {
    0
}

/// IAVIStream_ReadData - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIStream_ReadData(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// ACMStream_fnWriteData - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ACMStream_fnWriteData(iface: *mut core::ffi::c_void, fcc: u32, lp: *mut core::ffi::c_void, size: i32) -> i32 {
    0
}

/// IAVIStream_WriteData - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIStream_WriteData(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// IAVIFile_WriteData - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIFile_WriteData(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// IAVIFile_ReadData - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIFile_ReadData(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// IAVIStream_ReadFormat - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIStream_ReadFormat(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// AVIFILE_ReadBlock - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AVIFILE_ReadBlock(This: *mut core::ffi::c_void, start: u32, buffer: *mut core::ffi::c_void, size: u32) -> i32 {
    0
}

/// AVIFILE_WriteBlock - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AVIFILE_WriteBlock(This: *mut core::ffi::c_void, block: u32, ckid: usize, flags: u32, buffer: *const core::ffi::c_void, size: i32) -> i32 {
    0
}

/// IAVIFile_fnWriteData - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIFile_fnWriteData(iface: *mut core::ffi::c_void, ckid: u32, lpData: *mut core::ffi::c_void, size: i32) -> i32 {
    0
}

/// WriteExtraChunk - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn WriteExtraChunk(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// IAVIFile_fnReadData - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIFile_fnReadData(iface: *mut core::ffi::c_void, ckid: u32, lpData: *mut core::ffi::c_void, size: *mut i32) -> i32 {
    0
}

/// ReadExtraChunk - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ReadExtraChunk(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// IAVIStream_fnReadFormat - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIStream_fnReadFormat(iface: *mut core::ffi::c_void, pos: i32, format: *mut core::ffi::c_void, formatsize: *mut i32) -> i32 {
    0
}

/// IAVIStream_fnRead - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIStream_fnRead(iface: *mut core::ffi::c_void, start: i32, samples: i32, buffer: *mut core::ffi::c_void, buffersize: i32, bytesread: *mut i32, samplesread: *mut i32) -> i32 {
    0
}

/// IAVIStream_fnWrite - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIStream_fnWrite(iface: *mut core::ffi::c_void, start: i32, samples: i32, buffer: *mut core::ffi::c_void, buffersize: i32, flags: u32, sampwritten: *mut i32, byteswritten: *mut i32) -> i32 {
    0
}

/// IAVIStream_fnReadData - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIStream_fnReadData(iface: *mut core::ffi::c_void, fcc: u32, lp: *mut core::ffi::c_void, lpread: *mut i32) -> i32 {
    0
}

/// IAVIStream_fnWriteData - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IAVIStream_fnWriteData(iface: *mut core::ffi::c_void, fcc: u32, lp: *mut core::ffi::c_void, size: i32) -> i32 {
    0
}

/// wsprintfW - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn wsprintfW(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// AVIFILE_ReadFrame - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn AVIFILE_ReadFrame(This: *mut core::ffi::c_void, pstream: usize, pos: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// IEditAVIStream_fnReadFormat - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IEditAVIStream_fnReadFormat(arg0: *mut core::ffi::c_void, pos: i32, format: *mut core::ffi::c_void, arg3: *mut core::ffi::c_void) -> i32 {
    0
}

/// IEditAVIStream_fnRead - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IEditAVIStream_fnRead(arg0: *mut core::ffi::c_void, start: i32, samples: i32, buffer: *mut core::ffi::c_void, buffersize: i32, arg5: *mut core::ffi::c_void, arg6: *mut core::ffi::c_void) -> i32 {
    0
}

/// IEditAVIStream_fnReadData - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IEditAVIStream_fnReadData(arg0: *mut core::ffi::c_void, fcc: u32, lp: *mut core::ffi::c_void, lpread: *mut i32) -> i32 {
    0
}

/// IEditAVIStream_fnWriteData - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn IEditAVIStream_fnWriteData(arg0: *mut core::ffi::c_void, fcc: u32, lp: *mut core::ffi::c_void, size: i32) -> i32 {
    0
}

/// ReadChunkIntoExtra - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn ReadChunkIntoExtra(extra: usize, hmmio: usize, lpck: *mut core::ffi::c_void) -> i32 {
    0
}

/// ReadClipboardFile - from reactos/fileutils.h
#[no_mangle]
pub unsafe extern "C" fn ReadClipboardFile(lpFileName: *const u16) {

}

/// WriteClipboardFile - from reactos/fileutils.h
#[no_mangle]
pub unsafe extern "C" fn WriteClipboardFile(lpFileName: *const u16, wFileIdentifier: u16) {

}

/// xfprintf - from reactos/drwtsn32.h
#[no_mangle]
pub unsafe extern "C" fn xfprintf(stream: *mut core::ffi::c_void, fmt: *mut i8) {

}

/// rd_read_file - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn rd_read_file(fd: i32, ptr: *mut core::ffi::c_void, len: i32) -> i32 {
    0
}

/// rd_write_file - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn rd_write_file(fd: i32, ptr: *mut core::ffi::c_void, len: i32) -> i32 {
    0
}

/// read_keyboard_state - from reactos/proto.h
#[no_mangle]
pub unsafe extern "C" fn read_keyboard_state() -> u32 {
    0
}

/// ui_read_wire - from reactos/uimain.h
#[no_mangle]
pub unsafe extern "C" fn ui_read_wire() -> i32 {
    0
}

/// ReadText - from reactos/notepad.h
#[no_mangle]
pub unsafe extern "C" fn ReadText(hFile: *mut core::ffi::c_void, phLocal: *mut *mut core::ffi::c_void, pencFile: *mut core::ffi::c_void, piEoln: *mut core::ffi::c_void) -> i32 {
    0
}

/// WriteText - from reactos/notepad.h
#[no_mangle]
pub unsafe extern "C" fn WriteText(hFile: *mut core::ffi::c_void, pszText: *const u16, dwTextLen: u32, encFile: usize, iEoln: usize) -> i32 {
    0
}

/// OSK_WarningDlgThread - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn OSK_WarningDlgThread(lpParameter: *mut core::ffi::c_void) -> u32 {
    0
}

/// write_wav - from reactos/sndrec32.h
#[no_mangle]
pub unsafe extern "C" fn write_wav(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// ReadLineConfig - from reactos/sndvol32.h
#[no_mangle]
pub unsafe extern "C" fn ReadLineConfig(szDeviceName: usize, szLineName: usize, szControlName: usize, Flags: *mut core::ffi::c_void) -> i32 {
    0
}

/// WriteLineConfig - from reactos/sndvol32.h
#[no_mangle]
pub unsafe extern "C" fn WriteLineConfig(szDeviceName: usize, szLineName: usize, LineState: usize, cbSize: usize) -> i32 {
    0
}

/// PerfDataGetThreadCount - from reactos/perfdata.h
#[no_mangle]
pub unsafe extern "C" fn PerfDataGetThreadCount(Index: u32) -> u32 {
    0
}

/// PerfDataGetTotalThreadCount - from reactos/perfdata.h
#[no_mangle]
pub unsafe extern "C" fn PerfDataGetTotalThreadCount() -> u32 {
    0
}

/// EndLocalThread - from reactos/taskmgr.h
#[no_mangle]
pub unsafe extern "C" fn EndLocalThread(hThread: *mut *mut core::ffi::c_void, dwThread: u32) -> u32 {
    0
}

/// HLPFILE_ReadHlpFile - from reactos/hlpfile.h
#[no_mangle]
pub unsafe extern "C" fn HLPFILE_ReadHlpFile(lpszPath: *const i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// registry_read_pagemargins - from reactos/wordpad.h
#[no_mangle]
pub unsafe extern "C" fn registry_read_pagemargins(arg0: *mut core::ffi::c_void) {

}

/// registry_read_previewpages - from reactos/wordpad.h
#[no_mangle]
pub unsafe extern "C" fn registry_read_previewpages(hKey: *mut core::ffi::c_void) {

}

/// registry_read_filelist - from reactos/wordpad.h
#[no_mangle]
pub unsafe extern "C" fn registry_read_filelist(arg0: *mut core::ffi::c_void) {

}

/// registry_read_options - from reactos/wordpad.h
#[no_mangle]
pub unsafe extern "C" fn registry_read_options() {

}

/// registry_read_formatopts_all - from reactos/wordpad.h
#[no_mangle]
pub unsafe extern "C" fn registry_read_formatopts_all(arg0: usize, arg1: usize) {

}

/// registry_read_winrect - from reactos/wordpad.h
#[no_mangle]
pub unsafe extern "C" fn registry_read_winrect(arg0: *mut core::ffi::c_void) {

}

/// registry_read_maximized - from reactos/wordpad.h
#[no_mangle]
pub unsafe extern "C" fn registry_read_maximized(arg0: *mut u32) {

}

/// ConPrintf - from reactos/conutils_noros.h
#[no_mangle]
pub unsafe extern "C" fn ConPrintf(fp: *mut core::ffi::c_void, psz: *const u16) {

}

/// ConResPrintf - from reactos/conutils_noros.h
#[no_mangle]
pub unsafe extern "C" fn ConResPrintf(fp: *mut core::ffi::c_void, nID: u32) {

}

/// CannotRead - from reactos/fc.h
#[no_mangle]
pub unsafe extern "C" fn CannotRead(file: *const u16) -> usize {
    0
}

/// output_writeconsole - from reactos/reg.h
#[no_mangle]
pub unsafe extern "C" fn output_writeconsole(str: *mut u16, wlen: u32) {

}

/// ReadFromConsole - from reactos/net.h
#[no_mangle]
pub unsafe extern "C" fn ReadFromConsole(lpInput: *mut u16, dwLength: u32, bEcho: i32) -> usize {
    0
}

/// WriteString - from reactos/tconsole.h
#[no_mangle]
pub unsafe extern "C" fn WriteString(pszString: *mut i8, cbString: u64) -> u64 {
    0
}

/// WriteStringFast - from reactos/tconsole.h
#[no_mangle]
pub unsafe extern "C" fn WriteStringFast(pszString: *mut i8, cbString: u64) -> u64 {
    0
}

/// WriteCtrlString - from reactos/tconsole.h
#[no_mangle]
pub unsafe extern "C" fn WriteCtrlString(pszString: *mut i8, cbString: u64) -> u64 {
    0
}

/// WriteCtrlChar - from reactos/tconsole.h
#[no_mangle]
pub unsafe extern "C" fn WriteCtrlChar(c: i8) -> u64 {
    0
}

/// NetWriteString - from reactos/tconsole.h
#[no_mangle]
pub unsafe extern "C" fn NetWriteString(pszString: *mut i8, cbString: u64) -> u64 {
    0
}

/// get_fast_write - from reactos/tnconfig.h
#[no_mangle]
pub unsafe extern "C" fn get_fast_write() -> usize {
    0
}

/// ReadString - from reactos/tnetwork.h
#[no_mangle]
pub unsafe extern "C" fn ReadString(str: *mut i8, length: i32) -> i32 {
    0
}

/// ReadSection - from reactos/configparser.h
#[no_mangle]
pub unsafe extern "C" fn ReadSection(Buffer: usize, Section: usize, isArch: i32) {

}

/// ReadIniValue - from reactos/configparser.h
#[no_mangle]
pub unsafe extern "C" fn ReadIniValue(File: *const u16, Section: *const u16, Name: *const u16, Output: usize) -> i32 {
    0
}

/// WriteLogMessage - from reactos/misc.h
#[no_mangle]
pub unsafe extern "C" fn WriteLogMessage(wType: u16, dwEventID: u32, lpMsg: *const u16) -> i32 {
    0
}

/// TFUninitLib_Thread - from reactos/cicutb.h
#[no_mangle]
pub unsafe extern "C" fn TFUninitLib_Thread(pLibThread: usize) {

}

/// ThreadMgr_Constructor - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn ThreadMgr_Constructor(pUnkOuter: *mut core::ffi::c_void, ppOut: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// ThreadMgr_OnDocumentMgrDestruction - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn ThreadMgr_OnDocumentMgrDestruction(tm: *mut core::ffi::c_void, mgr: *mut core::ffi::c_void) {

}

/// RpcThreadRoutine - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn RpcThreadRoutine(lpParameter: *mut core::ffi::c_void) -> u32 {
    0
}

/// ReadHostsFile - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn ReadHostsFile(arg0: usize) -> i32 {
    0
}

/// LogfClose - from reactos/eventlog.h
#[no_mangle]
pub unsafe extern "C" fn LogfClose(LogFile: usize, ForceClose: usize) -> usize {
    0
}

/// LogfCloseAll - from reactos/eventlog.h
#[no_mangle]
pub unsafe extern "C" fn LogfCloseAll(arg0: usize) -> usize {
    0
}

/// LogfReadEvents - from reactos/eventlog.h
#[no_mangle]
pub unsafe extern "C" fn LogfReadEvents(LogFile: usize, Flags: u32, RecordNumber: usize, BufSize: u32, Buffer: usize, BytesRead: usize, BytesNeeded: usize, Ansi: usize) -> i32 {
    0
}

/// LogfWriteRecord - from reactos/eventlog.h
#[no_mangle]
pub unsafe extern "C" fn LogfWriteRecord(LogFile: usize, Record: usize, BufSize: usize) -> i32 {
    0
}

/// PortThreadRoutine - from reactos/eventlog.h
#[no_mangle]
pub unsafe extern "C" fn PortThreadRoutine(Param: *mut core::ffi::c_void) -> i32 {
    0
}

/// dprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn dprintf(__fd: i32, __fmt: *mut core::ffi::c_void) -> i32 {
    0
}

/// eprintf - from reactos/daemon_debug.h
#[no_mangle]
pub unsafe extern "C" fn eprintf(format: *const i8) {

}

/// nfs41_write - from reactos/nfs41_ops.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_write(session: *mut core::ffi::c_void, file: *mut core::ffi::c_void, stateid: *mut core::ffi::c_void, data: *mut core::ffi::c_void, data_len: usize, offset: usize, stable: usize, bytes_written: *mut core::ffi::c_void, verf: *mut core::ffi::c_void, cinfo: *mut core::ffi::c_void) -> i32 {
    0
}

/// nfs41_read - from reactos/nfs41_ops.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_read(session: *mut core::ffi::c_void, file: *mut core::ffi::c_void, stateid: *mut core::ffi::c_void, offset: usize, count: usize, data_out: *mut core::ffi::c_void, data_len_out: *mut core::ffi::c_void, eof_out: *mut core::ffi::c_void) -> i32 {
    0
}

/// nfs41_readdir - from reactos/nfs41_ops.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_readdir(session: *mut core::ffi::c_void, file: *mut core::ffi::c_void, attr_request: *mut core::ffi::c_void, cookie: *mut core::ffi::c_void, entries: *mut core::ffi::c_void, entries_len: *mut core::ffi::c_void, eof_out: *mut core::ffi::c_void) -> i32 {
    0
}

/// nfs41_readlink - from reactos/nfs41_ops.h
#[no_mangle]
pub unsafe extern "C" fn nfs41_readlink(session: *mut core::ffi::c_void, file: *mut core::ffi::c_void, max_len: usize, link_out: *mut core::ffi::c_void, len_out: *mut core::ffi::c_void) -> i32 {
    0
}

/// pnfs_read - from reactos/pnfs.h
#[no_mangle]
pub unsafe extern "C" fn pnfs_read(root: *mut core::ffi::c_void, state: *mut core::ffi::c_void, stateid: *mut core::ffi::c_void, layout: *mut core::ffi::c_void, offset: usize, length: usize, buffer_out: *mut core::ffi::c_void, len_out: *mut core::ffi::c_void) -> usize {
    0
}

/// pnfs_write - from reactos/pnfs.h
#[no_mangle]
pub unsafe extern "C" fn pnfs_write(root: *mut core::ffi::c_void, state: *mut core::ffi::c_void, stateid: *mut core::ffi::c_void, layout: *mut core::ffi::c_void, offset: usize, length: usize, buffer: *mut core::ffi::c_void, len_out: *mut core::ffi::c_void, cinfo: *mut core::ffi::c_void) -> usize {
    0
}

/// safe_read - from reactos/util.h
#[no_mangle]
pub unsafe extern "C" fn safe_read(pos: *mut *mut u8, remaining: *mut u32, dest: *mut core::ffi::c_void, dest_len: u32) -> i32 {
    0
}

/// safe_write - from reactos/util.h
#[no_mangle]
pub unsafe extern "C" fn safe_write(pos: *mut *mut u8, remaining: *mut u32, dest: *mut core::ffi::c_void, dest_len: u32) -> i32 {
    0
}

/// max_read_size - from reactos/util.h
#[no_mangle]
pub unsafe extern "C" fn max_read_size(session: *mut core::ffi::c_void, fh: *mut core::ffi::c_void) -> u32 {
    0
}

/// max_write_size - from reactos/util.h
#[no_mangle]
pub unsafe extern "C" fn max_write_size(session: *mut core::ffi::c_void, fh: *mut core::ffi::c_void) -> u32 {
    0
}

/// verify_write - from reactos/util.h
#[no_mangle]
pub unsafe extern "C" fn verify_write(verf: *mut core::ffi::c_void, stable: *mut core::ffi::c_void) -> usize {
    0
}

/// UserLoginThread - from reactos/telnetd.h
#[no_mangle]
pub unsafe extern "C" fn UserLoginThread(arg0: *mut core::ffi::c_void) -> u32 {
    0
}

/// MonitorChildThread - from reactos/telnetd.h
#[no_mangle]
pub unsafe extern "C" fn MonitorChildThread(arg0: *mut core::ffi::c_void) -> u32 {
    0
}

/// WriteToPipeThread - from reactos/telnetd.h
#[no_mangle]
pub unsafe extern "C" fn WriteToPipeThread(arg0: *mut core::ffi::c_void) -> u32 {
    0
}

/// ReadFromPipeThread - from reactos/telnetd.h
#[no_mangle]
pub unsafe extern "C" fn ReadFromPipeThread(arg0: *mut core::ffi::c_void) -> u32 {
    0
}

/// readSection - from reactos/tftpd.h
#[no_mangle]
pub unsafe extern "C" fn readSection(arg0: *mut i8, arg1: *mut core::ffi::c_void) -> *mut i8 {
    core::ptr::null_mut()
}

/// PnpEventThread - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn PnpEventThread(lpParameter: *mut core::ffi::c_void) -> u32 {
    0
}

/// DeviceInstallThread - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn DeviceInstallThread(lpParameter: *mut core::ffi::c_void) -> u32 {
    0
}

/// RpcServerThread - from reactos/precomp.h
#[no_mangle]
pub unsafe extern "C" fn RpcServerThread(lpParameter: *mut core::ffi::c_void) -> u32 {
    0
}

/// rewrite_client_leases - from reactos/dhcpd.h
#[no_mangle]
pub unsafe extern "C" fn rewrite_client_leases(arg0: *mut core::ffi::c_void) {

}

/// write_client_lease - from reactos/dhcpd.h
#[no_mangle]
pub unsafe extern "C" fn write_client_lease(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: i32) {

}

/// priv_script_write_params - from reactos/dhcpd.h
#[no_mangle]
pub unsafe extern "C" fn priv_script_write_params(arg0: *mut core::ffi::c_void, arg1: *mut i8, arg2: *mut core::ffi::c_void) {

}

/// script_write_params - from reactos/dhcpd.h
#[no_mangle]
pub unsafe extern "C" fn script_write_params(arg0: *mut i8, arg1: *mut core::ffi::c_void) {

}

/// read_client_conf - from reactos/dhcpd.h
#[no_mangle]
pub unsafe extern "C" fn read_client_conf(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// read_client_leases - from reactos/dhcpd.h
#[no_mangle]
pub unsafe extern "C" fn read_client_leases() {

}

/// buf_read - from reactos/dhcpd.h
#[no_mangle]
pub unsafe extern "C" fn buf_read(arg0: i32, arg1: *mut core::ffi::c_void, arg2: usize) -> isize {
    0
}

/// AdapterFindByHardwareAddress - from reactos/rosdhcp.h
#[no_mangle]
pub unsafe extern "C" fn AdapterFindByHardwareAddress(haddr: *mut core::ffi::c_void, hlen: usize) -> usize {
    0
}

/// ReadBootCodeByHandle - from reactos/bootcode.h
#[no_mangle]
pub unsafe extern "C" fn ReadBootCodeByHandle(BootCodeInfo: usize, FileHandle: usize, OPTIONAL: usize) -> i32 {
    0
}

/// ReadBootCodeFromFile - from reactos/bootcode.h
#[no_mangle]
pub unsafe extern "C" fn ReadBootCodeFromFile(BootCodeInfo: usize, FilePath: usize, OPTIONAL: usize) -> i32 {
    0
}

/// SetWindowResPrintfVW - from reactos/reactos.h
#[no_mangle]
pub unsafe extern "C" fn SetWindowResPrintfVW(hWnd: usize, hInstance: usize, uID: usize, args: usize) -> usize {
    0
}

/// SetWindowResPrintfW - from reactos/reactos.h
#[no_mangle]
pub unsafe extern "C" fn SetWindowResPrintfW(hWnd: usize, hInstance: usize, uID: usize) -> usize {
    0
}

/// ReadConsoleInput - from reactos/console.h
#[no_mangle]
pub unsafe extern "C" fn ReadConsoleInput(hConsoleInput: usize, lpBuffer: usize, nLength: usize, lpNumberOfEventsRead: usize) -> i32 {
    0
}

/// WriteConsole - from reactos/console.h
#[no_mangle]
pub unsafe extern "C" fn WriteConsole(hConsoleOutput: usize, lpBuffer: *mut core::ffi::c_void, nNumberOfCharsToWrite: usize, lpNumberOfCharsWritten: usize, lpReserved: usize) -> i32 {
    0
}

/// CONSOLE_ConOutPrintfV - from reactos/consup.h
#[no_mangle]
pub unsafe extern "C" fn CONSOLE_ConOutPrintfV(szFormat: usize, args: usize) -> usize {
    0
}

/// CONSOLE_ConOutPrintf - from reactos/consup.h
#[no_mangle]
pub unsafe extern "C" fn CONSOLE_ConOutPrintf(szFormat: usize) -> usize {
    0
}

/// PrintFileDacl - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn PrintFileDacl(FilePath: usize, FileName: usize) -> i32 {
    0
}

/// StringCbPrintf - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn StringCbPrintf(arg0: usize, arg1: usize) -> usize {
    0
}

/// ClipboardReadMemoryBlock - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn ClipboardReadMemoryBlock(hFile: *mut core::ffi::c_void, dwOffset: u32, dwLength: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// ClipboardReadMemory - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn ClipboardReadMemory(hFile: *mut core::ffi::c_void, dwFormat: u32, dwOffset: u32, dwLength: u32, FileIdentifier: u16, lpFormatName: *mut core::ffi::c_void) -> i32 {
    0
}

/// ClipboardWriteMemory - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn ClipboardWriteMemory(hFile: *mut core::ffi::c_void, dwFormat: u32, dwOffset: u32, pdwLength: usize) -> i32 {
    0
}

/// ClipboardReadPalette - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn ClipboardReadPalette(hFile: *mut core::ffi::c_void, dwOffset: u32, dwLength: u32) -> i32 {
    0
}

/// ClipboardReadMetafile - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn ClipboardReadMetafile(hFile: *mut core::ffi::c_void, dwOffset: u32, dwLength: u32) -> i32 {
    0
}

/// ClipboardReadEnhMetafile - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn ClipboardReadEnhMetafile(hFile: *mut core::ffi::c_void, dwOffset: u32, dwLength: u32) -> i32 {
    0
}

/// ClipboardReadBitmap - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn ClipboardReadBitmap(hFile: *mut core::ffi::c_void, dwOffset: u32, dwLength: u32) -> i32 {
    0
}

/// swprintf - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn swprintf(arg0: usize, TrueType: usize) -> usize {
    0
}

/// rdssl_cert_read - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn rdssl_cert_read(data: *mut core::ffi::c_void, len: usize) -> usize {
    0
}

/// WriteRdpFile - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn WriteRdpFile(hFile: *mut core::ffi::c_void, pRdpSettings: usize) -> i32 {
    0
}

/// ReadRdpFile - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn ReadRdpFile(hFile: *mut core::ffi::c_void) -> *mut u16 {
    core::ptr::null_mut()
}

/// mi_read_keyboard_state - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn mi_read_keyboard_state() -> i32 {
    0
}

/// DoCreatePrintFonts - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn DoCreatePrintFonts(pPrinter: usize, pPrintData: usize) -> i32 {
    0
}

/// PrintThreadFunc - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn PrintThreadFunc(arg: *mut core::ffi::c_void) -> u32 {
    0
}

/// WriteEncodedText - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn WriteEncodedText(hFile: *mut core::ffi::c_void, pszText: *const u16, dwTextLen: u32, encFile: usize) -> i32 {
    0
}

/// __argp_fmtstream_printf - from glibc/argp-fmtstream.h
#[no_mangle]
pub unsafe extern "C" fn __argp_fmtstream_printf(__fs: usize, __fmt: *mut i8) -> isize {
    0
}

/// argp_fmtstream_printf - from glibc/argp-fmtstream.h
#[no_mangle]
pub unsafe extern "C" fn argp_fmtstream_printf(__fs: usize, __fmt: *mut i8) -> isize {
    0
}

/// __argp_fmtstream_write - from glibc/argp-fmtstream.h
#[no_mangle]
pub unsafe extern "C" fn __argp_fmtstream_write(__fs: usize, __str: *mut i8, __len: usize) -> usize {
    0
}

/// argp_fmtstream_write - from glibc/argp-fmtstream.h
#[no_mangle]
pub unsafe extern "C" fn argp_fmtstream_write(__fs: usize, __str: *mut i8, __len: usize) -> usize {
    0
}

/// pthread_sigmask - from glibc/sigthread.h
#[no_mangle]
pub unsafe extern "C" fn pthread_sigmask(__how: i32, __newmask: *mut core::ffi::c_void, __oldmask: *mut core::ffi::c_void) -> i32 {
    0
}

/// closedir - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn closedir(__dirp: *mut core::ffi::c_void) -> i32 {
    0
}

/// __attr_dealloc - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn __attr_dealloc(arg0: usize, arg1: usize) -> usize {
    0
}

/// readdir_r - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn readdir_r(__dirp: *mut core::ffi::c_void, __entry: *mut core::ffi::c_void, __result: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// __REDIRECT - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn __REDIRECT(arg0: usize, __dirp: *mut core::ffi::c_void, __entry: *mut core::ffi::c_void, __result: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// readdir64_r - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn readdir64_r(__dirp: *mut core::ffi::c_void, __entry: *mut core::ffi::c_void, __result: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// rewinddir - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn rewinddir(__dirp: *mut core::ffi::c_void) {

}

/// seekdir - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn seekdir(__dirp: *mut core::ffi::c_void, __pos: i64) {

}

/// telldir - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn telldir(__dirp: *mut core::ffi::c_void) -> i64 {
    0
}

/// dirfd - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn dirfd(__dirp: *mut core::ffi::c_void) -> i32 {
    0
}

/// alphasort - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn alphasort(__e1: *mut *mut core::ffi::c_void, __e2: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// __REDIRECT_NTH - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn __REDIRECT_NTH(arg0: usize, __e1: *mut *mut core::ffi::c_void, __e2: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// alphasort64 - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn alphasort64(__e1: *mut *mut core::ffi::c_void, __e2: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// getdirentries - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn getdirentries(__fd: i32, __buf: *mut core::ffi::c_void, __nbytes: usize, __basep: *mut core::ffi::c_void) -> usize {
    0
}

/// getdirentries64 - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn getdirentries64(__fd: i32, __buf: *mut core::ffi::c_void, __nbytes: usize, __basep: *mut core::ffi::c_void) -> usize {
    0
}

/// versionsort - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn versionsort(__e1: *mut *mut core::ffi::c_void, __e2: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// versionsort64 - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn versionsort64(__e1: *mut *mut core::ffi::c_void, __e2: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// _dl_writev - from glibc/dl-writev.h
#[no_mangle]
pub unsafe extern "C" fn _dl_writev(fd: i32, iov: *mut core::ffi::c_void, niov: usize) {

}

/// __pthread_enqueue - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_enqueue(head: *mut *mut core::ffi::c_void, thread: *mut core::ffi::c_void) {

}

/// __pthread_dequeue - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_dequeue(thread: *mut core::ffi::c_void) {

}

/// __pthread_create_internal - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_create_internal(pthread: *mut *mut core::ffi::c_void, attr: *mut core::ffi::c_void, start_routine: *mut core::ffi::c_void) -> i32 {
    0
}

/// __pthread_thread_start - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_thread_start(thread: *mut core::ffi::c_void) -> i32 {
    0
}

/// __pthread_thread_terminate - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_thread_terminate(thread: *mut core::ffi::c_void) {

}

/// __pthread_startup - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_startup() {

}

/// __pthread_block - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_block(thread: *mut core::ffi::c_void) {

}

/// __pthread_timedblock - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_timedblock(thread: *mut core::ffi::c_void, abstime: *mut core::ffi::c_void, clock_id: usize) -> usize {
    0
}

/// __pthread_block_intr - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_block_intr(thread: *mut core::ffi::c_void) -> usize {
    0
}

/// __pthread_timedblock_intr - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_timedblock_intr(thread: *mut core::ffi::c_void, abstime: *mut core::ffi::c_void, clock_id: usize) -> usize {
    0
}

/// __pthread_wakeup - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_wakeup(thread: *mut core::ffi::c_void) {

}

/// __pthread_do_cancel - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_do_cancel(thread: *mut core::ffi::c_void) -> i32 {
    0
}

/// __pthread_init_specific - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_init_specific(thread: *mut core::ffi::c_void) -> usize {
    0
}

/// __pthread_destroy_specific - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_destroy_specific(thread: *mut core::ffi::c_void) {

}

/// __pthread_sigstate_init - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_sigstate_init(thread: *mut core::ffi::c_void) -> usize {
    0
}

/// __pthread_sigstate_destroy - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_sigstate_destroy(thread: *mut core::ffi::c_void) {

}

/// __pthread_sigstate - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_sigstate(thread: *mut core::ffi::c_void, how: i32, set: *mut core::ffi::c_void, oset: *mut core::ffi::c_void, clear_pending: i32) -> usize {
    0
}

/// __pthread_mutex_checklocked - from glibc/pt-internal.h
#[no_mangle]
pub unsafe extern "C" fn __pthread_mutex_checklocked(mtx: *mut core::ffi::c_void) -> i32 {
    0
}

/// hurd_thread_self - from glibc/hurd.h
#[no_mangle]
pub unsafe extern "C" fn hurd_thread_self() -> usize {
    0
}

/// hurd_thread_cancel - from glibc/hurd.h
#[no_mangle]
pub unsafe extern "C" fn hurd_thread_cancel(thread: usize) -> usize {
    0
}

/// vpprintf - from glibc/hurd.h
#[no_mangle]
pub unsafe extern "C" fn vpprintf(port: usize, format: *mut i8, arg: usize) -> i32 {
    0
}

/// read_conf_file - from glibc/gconv_parseconfdir.h
#[no_mangle]
pub unsafe extern "C" fn read_conf_file(filename: *mut i8, directory: *mut i8, dir_len: usize) -> usize {
    0
}

/// __closedir - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn __closedir(__dirp: *mut core::ffi::c_void) -> i32 {
    0
}

/// __readdir_r - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn __readdir_r(__dirp: *mut core::ffi::c_void, __entry: *mut core::ffi::c_void, __result: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// __readdir64_r - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn __readdir64_r(__dirp: *mut core::ffi::c_void, __entry: *mut core::ffi::c_void, __result: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// __scandir64 - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn __scandir64(__dir: *mut i8, __namelist: *mut *mut *mut core::ffi::c_void, __selector: i32) -> i32 {
    0
}

/// __getdirentries - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn __getdirentries(__fd: i32, __buf: *mut core::ffi::c_void, __nbytes: usize, __basep: *mut core::ffi::c_void) -> usize {
    0
}

/// __getdents - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn __getdents(__fd: i32, __buf: *mut core::ffi::c_void, __nbytes: usize) -> usize {
    0
}

/// __getdents64 - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn __getdents64(__fd: i32, __buf: *mut core::ffi::c_void, __nbytes: usize) -> usize {
    0
}

/// __alphasort64 - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn __alphasort64(a: *mut *mut core::ffi::c_void, b: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// __versionsort64 - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn __versionsort64(a: *mut *mut core::ffi::c_void, b: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// __scandir_cancel_handler - from glibc/dirent.h
#[no_mangle]
pub unsafe extern "C" fn __scandir_cancel_handler(arg: *mut core::ffi::c_void) {

}

/// __open64 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __open64(__file: *mut i8, __oflag: i32) -> i32 {
    0
}

/// __libc_open64 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __libc_open64(file: *mut i8, oflag: i32) -> i32 {
    0
}

/// __libc_open - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __libc_open(file: *mut i8, oflag: i32) -> i32 {
    0
}

/// __libc_fcntl - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __libc_fcntl(fd: i32, cmd: i32) -> i32 {
    0
}

/// __fcntl64_nocancel_adjusted - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __fcntl64_nocancel_adjusted(fd: i32, cmd: i32, arg: *mut core::ffi::c_void) -> i32 {
    0
}

/// __libc_fcntl64 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __libc_fcntl64(fd: i32, cmd: i32) -> i32 {
    0
}

/// __open - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __open(__file: *mut i8, __oflag: i32) -> i32 {
    0
}

/// __fcntl - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __fcntl(__fd: i32, __cmd: i32) -> i32 {
    0
}

/// __fcntl64 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __fcntl64(__fd: i32, __cmd: i32) -> i32 {
    0
}

/// __openat - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __openat(__fd: i32, __file: *mut i8, __oflag: i32) -> i32 {
    0
}

/// __openat64 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __openat64(__fd: i32, __file: *mut i8, __oflag: i32) -> i32 {
    0
}

/// __open_2 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __open_2(__path: *mut i8, __oflag: i32) -> i32 {
    0
}

/// __open64_2 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __open64_2(__path: *mut i8, __oflag: i32) -> i32 {
    0
}

/// __openat_2 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __openat_2(__fd: i32, __path: *mut i8, __oflag: i32) -> i32 {
    0
}

/// __openat64_2 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __openat64_2(__fd: i32, __path: *mut i8, __oflag: i32) -> i32 {
    0
}

/// __nss_readline - from glibc/nss_files.h
#[no_mangle]
pub unsafe extern "C" fn __nss_readline(fp: *mut core::ffi::c_void, buf: *mut i8, len: usize, poffset: *mut core::ffi::c_void) -> i32 {
    0
}

/// __nss_readline_seek - from glibc/nss_files.h
#[no_mangle]
pub unsafe extern "C" fn __nss_readline_seek(fp: *mut core::ffi::c_void, offset: usize) -> i32 {
    0
}

/// __register_printf_specifier - from glibc/printf.h
#[no_mangle]
pub unsafe extern "C" fn __register_printf_specifier(arg0: i32, arg1: usize, arg2: usize) -> i32 {
    0
}

/// __printf_function_invoke - from glibc/printf.h
#[no_mangle]
pub unsafe extern "C" fn __printf_function_invoke(arg0: *mut core::ffi::c_void, callback: usize, args_value: *mut core::ffi::c_void, ndata_args: usize, info: *mut core::ffi::c_void) -> i32 {
    0
}

/// __wprintf_function_invoke - from glibc/printf.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_function_invoke(arg0: *mut core::ffi::c_void, callback: usize, args_value: *mut core::ffi::c_void, ndata_args: usize, info: *mut core::ffi::c_void) -> i32 {
    0
}

/// __printf_buffer - from glibc/printf.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer(buf: *mut core::ffi::c_void, format: *mut i8, ap: usize, mode_flags: u32) {

}

/// __wprintf_buffer - from glibc/printf.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_buffer(buf: *mut core::ffi::c_void, format: *mut core::ffi::c_void, ap: usize, mode_flags: u32) {

}

/// __printf_fp - from glibc/printf.h
#[no_mangle]
pub unsafe extern "C" fn __printf_fp(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// __printf_fphex_l_buffer - from glibc/printf.h
#[no_mangle]
pub unsafe extern "C" fn __printf_fphex_l_buffer(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut core::ffi::c_void, arg3: *mut *mut core::ffi::c_void) {

}

/// __printf_fp_l_buffer - from glibc/printf.h
#[no_mangle]
pub unsafe extern "C" fn __printf_fp_l_buffer(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut core::ffi::c_void, arg3: *mut *mut core::ffi::c_void) {

}

/// __wprintf_fphex_l_buffer - from glibc/printf.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_fphex_l_buffer(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut core::ffi::c_void, arg3: *mut *mut core::ffi::c_void) {

}

/// __wprintf_fp_l_buffer - from glibc/printf.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_fp_l_buffer(arg0: *mut core::ffi::c_void, arg1: usize, arg2: *mut core::ffi::c_void, arg3: *mut *mut core::ffi::c_void) {

}

/// __printf_buffer_mark_failed - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_mark_failed(buf: *mut core::ffi::c_void) {

}

/// __printf_buffer_has_failed - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_has_failed(buf: *mut core::ffi::c_void) -> usize {
    0
}

/// __printf_buffer_init_end - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_init_end(buf: *mut core::ffi::c_void, base: *mut i8, end: *mut i8, mode: usize) {

}

/// __printf_buffer_init - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_init(buf: *mut core::ffi::c_void, base: *mut i8, len: usize, mode: usize) {

}

/// __printf_buffer_putc_1 - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_putc_1(buf: *mut core::ffi::c_void, ch: i8) {

}

/// __printf_buffer_putc - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_putc(buf: *mut core::ffi::c_void, ch: i8) {

}

/// __printf_buffer_pad_1 - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_pad_1(buf: *mut core::ffi::c_void, ch: i8, count: usize) {

}

/// __printf_buffer_pad - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_pad(buf: *mut core::ffi::c_void, ch: i8, count: isize) {

}

/// __printf_buffer_write - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_write(buf: *mut core::ffi::c_void, s: *mut i8, count: usize) {

}

/// __printf_buffer_puts_1 - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_puts_1(buf: *mut core::ffi::c_void, s: *mut i8) {

}

/// __printf_buffer_done - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_done(buf: *mut core::ffi::c_void) -> i32 {
    0
}

/// __printf_buffer_flush - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_flush(buf: *mut core::ffi::c_void) -> usize {
    0
}

/// __wprintf_buffer_mark_failed - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_buffer_mark_failed(buf: *mut core::ffi::c_void) {

}

/// __wprintf_buffer_has_failed - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_buffer_has_failed(buf: *mut core::ffi::c_void) -> usize {
    0
}

/// __wprintf_buffer_init - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_buffer_init(buf: *mut core::ffi::c_void, base: *mut core::ffi::c_void, len: usize, mode: usize) {

}

/// __wprintf_buffer_putc_1 - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_buffer_putc_1(buf: *mut core::ffi::c_void, ch: usize) {

}

/// __wprintf_buffer_putc - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_buffer_putc(buf: *mut core::ffi::c_void, ch: usize) {

}

/// __wprintf_buffer_pad_1 - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_buffer_pad_1(buf: *mut core::ffi::c_void, ch: usize, count: usize) {

}

/// __wprintf_buffer_pad - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_buffer_pad(buf: *mut core::ffi::c_void, ch: i8, count: isize) {

}

/// __wprintf_buffer_write - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_buffer_write(buf: *mut core::ffi::c_void, s: *mut core::ffi::c_void, count: usize) {

}

/// __wprintf_buffer_puts - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_buffer_puts(buf: *mut core::ffi::c_void, s: *mut core::ffi::c_void) {

}

/// __wprintf_buffer_done - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_buffer_done(buf: *mut core::ffi::c_void) -> i32 {
    0
}

/// __wprintf_buffer_flush - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_buffer_flush(buf: *mut core::ffi::c_void) -> usize {
    0
}

/// __printf_buffer_snprintf_init - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_snprintf_init(arg0: *mut core::ffi::c_void, buffer: *mut i8, length: usize) {

}

/// __printf_buffer_snprintf_done - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_snprintf_done(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// __printf_buffer_flush_snprintf - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_flush_snprintf(arg0: *mut core::ffi::c_void) {

}

/// __printf_buffer_flush_to_file - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_flush_to_file(arg0: *mut core::ffi::c_void) {

}

/// __printf_buffer_flush_asprintf - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_flush_asprintf(arg0: *mut core::ffi::c_void) {

}

/// __printf_buffer_flush_dprintf - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_flush_dprintf(arg0: *mut core::ffi::c_void) {

}

/// __printf_buffer_flush_fp - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_flush_fp(arg0: *mut core::ffi::c_void) {

}

/// __printf_buffer_flush_fp_to_wide - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_flush_fp_to_wide(arg0: *mut core::ffi::c_void) {

}

/// __printf_buffer_flush_fphex_to_wide - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_flush_fphex_to_wide(arg0: *mut core::ffi::c_void) {

}

/// __printf_buffer_flush_obstack - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_flush_obstack(arg0: *mut core::ffi::c_void) {

}

/// __wprintf_buffer_flush_to_file - from glibc/printf_buffer.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_buffer_flush_to_file(arg0: *mut core::ffi::c_void) {

}

/// __rpc_thread_destroy - from glibc/set-freeres.h
#[no_mangle]
pub unsafe extern "C" fn __rpc_thread_destroy() {

}

/// fseterr_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fseterr_unlocked(fp: *mut core::ffi::c_void) -> usize {
    0
}

/// __fcloseall - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __fcloseall() -> i32 {
    0
}

/// __snprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __snprintf(__s: *mut core::ffi::c_void, __maxlen: usize, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __vfscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __vfscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __vscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __vscanf(__format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __getline - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __getline(__lineptr: *mut *mut i8, __n: *mut usize, __stream: *mut core::ffi::c_void) -> usize {
    0
}

/// __vsscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __vsscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __sprintf_chk - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __sprintf_chk(arg0: *mut i8, arg1: i32, arg2: usize, arg3: *mut i8) -> i32 {
    0
}

/// __snprintf_chk - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __snprintf_chk(arg0: *mut i8, arg1: usize, arg2: i32, arg3: usize, arg4: *mut i8) -> i32 {
    0
}

/// __vsprintf_chk - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __vsprintf_chk(arg0: *mut i8, arg1: i32, arg2: usize, arg3: *mut i8, arg4: usize) -> i32 {
    0
}

/// __vsnprintf_chk - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __vsnprintf_chk(arg0: *mut i8, arg1: usize, arg2: i32, arg3: usize, arg4: *mut i8, arg5: usize) -> i32 {
    0
}

/// __printf_chk - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __printf_chk(arg0: i32, arg1: *mut i8) -> i32 {
    0
}

/// __fprintf_chk - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __fprintf_chk(arg0: *mut core::ffi::c_void, arg1: i32, arg2: *mut i8) -> i32 {
    0
}

/// __vprintf_chk - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __vprintf_chk(arg0: i32, arg1: *mut i8, arg2: usize) -> i32 {
    0
}

/// __vfprintf_chk - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __vfprintf_chk(arg0: *mut core::ffi::c_void, arg1: i32, arg2: *mut i8, arg3: usize) -> i32 {
    0
}

/// __asprintf_chk - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __asprintf_chk(arg0: *mut *mut i8, arg1: i32, arg2: *mut i8) -> i32 {
    0
}

/// __vasprintf_chk - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __vasprintf_chk(arg0: *mut *mut i8, arg1: i32, arg2: *mut i8, arg3: usize) -> i32 {
    0
}

/// __dprintf_chk - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __dprintf_chk(arg0: i32, arg1: i32, arg2: *mut i8) -> i32 {
    0
}

/// __vdprintf_chk - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __vdprintf_chk(arg0: i32, arg1: i32, arg2: *mut i8, arg3: usize) -> i32 {
    0
}

/// __obstack_printf_chk - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __obstack_printf_chk(arg0: *mut core::ffi::c_void, arg1: i32, arg2: *mut i8) -> i32 {
    0
}

/// __obstack_vprintf_chk - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __obstack_vprintf_chk(arg0: *mut core::ffi::c_void, arg1: i32, arg2: *mut i8, arg3: usize) -> i32 {
    0
}

/// __isoc99_fscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __isoc99_fscanf(__stream: *mut core::ffi::c_void, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __isoc99_scanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __isoc99_scanf(__format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __isoc99_sscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __isoc99_sscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __isoc99_vfscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __isoc99_vfscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __isoc99_vscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __isoc99_vscanf(__format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __isoc99_vsscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __isoc99_vsscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __isoc23_fscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __isoc23_fscanf(__stream: *mut core::ffi::c_void, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __isoc23_scanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __isoc23_scanf(__format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __isoc23_sscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __isoc23_sscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __isoc23_vfscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __isoc23_vfscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __isoc23_vscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __isoc23_vscanf(__format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __isoc23_vsscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __isoc23_vsscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __gen_tempname - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __gen_tempname(__tmpl: *mut i8, __suffixlen: i32, __flags: i32, __kind: i32) -> i32 {
    0
}

/// __libc_fatal - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __libc_fatal(__message: *mut i8) {

}

/// __fortify_fail - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __fortify_fail(msg: *mut i8) {

}

/// __libc_message_impl - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __libc_message_impl(__vmaname: *mut i8, __fmt: *mut i8) -> usize {
    0
}

/// __flockfile - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __flockfile(__stream: *mut core::ffi::c_void) {

}

/// __funlockfile - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __funlockfile(__stream: *mut core::ffi::c_void) {

}

/// __ftrylockfile - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __ftrylockfile(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// __getc_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __getc_unlocked(__fp: *mut core::ffi::c_void) -> i32 {
    0
}

/// __getwc_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __getwc_unlocked(__fp: *mut core::ffi::c_void) -> usize {
    0
}

/// __fxprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __fxprintf(__fp: *mut core::ffi::c_void, __fmt: *mut i8) -> i32 {
    0
}

/// __fxprintf_nocancel - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __fxprintf_nocancel(__fp: *mut core::ffi::c_void, __fmt: *mut i8) -> i32 {
    0
}

/// __vfxprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __vfxprintf(__fp: *mut core::ffi::c_void, __fmt: *mut i8, arg2: usize, arg3: u32) -> i32 {
    0
}

/// _IO_new_fclose - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_new_fclose(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// _IO_fputs - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_fputs(arg0: *mut i8, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// _IO_new_fsetpos - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_new_fsetpos(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// _IO_new_fgetpos - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_new_fgetpos(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// __fmemopen - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __fmemopen(buf: *mut core::ffi::c_void, len: usize, mode: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// __gen_tempfd - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __gen_tempfd(flags: i32) -> i32 {
    0
}

/// __feof_unlocked_body - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __feof_unlocked_body(arg0: usize) -> usize {
    0
}

/// __ferror_unlocked_body - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __ferror_unlocked_body(arg0: usize) -> usize {
    0
}

/// __getc_unlocked_body - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __getc_unlocked_body(arg0: usize) -> usize {
    0
}

/// __putc_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __putc_unlocked(__c: i32, __stream: *mut core::ffi::c_void) -> usize {
    0
}

/// __putc_unlocked_body - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __putc_unlocked_body(arg0: usize, arg1: usize) -> usize {
    0
}

/// __tzfile_read - from glibc/time.h
#[no_mangle]
pub unsafe extern "C" fn __tzfile_read(file: *mut i8, extra: usize, extrap: *mut *mut i8) {

}

/// __confstr - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __confstr(name: i32, buf: *mut i8, len: usize) -> usize {
    0
}

/// __access - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __access(__name: *mut i8, __type: i32) -> i32 {
    0
}

/// __euidaccess - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __euidaccess(__name: *mut i8, __type: i32) -> i32 {
    0
}

/// __faccessat - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __faccessat(__fd: i32, __file: *mut i8, __type: i32, __flag: i32) -> i32 {
    0
}

/// __faccessat_noerrno - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __faccessat_noerrno(__fd: i32, __file: *mut i8, __type: i32, __flag: i32) -> i32 {
    0
}

/// __lseek64 - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __lseek64(__fd: i32, __offset: usize, __whence: i32) -> usize {
    0
}

/// __lseek - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __lseek(__fd: i32, __offset: usize, __whence: i32) -> usize {
    0
}

/// __libc_lseek - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __libc_lseek(__fd: i32, __offset: usize, __whence: i32) -> usize {
    0
}

/// __libc_lseek64 - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __libc_lseek64(__fd: i32, __offset: usize, __whence: i32) -> usize {
    0
}

/// __pread - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __pread(__fd: i32, __buf: *mut core::ffi::c_void, __nbytes: usize, __offset: usize) -> isize {
    0
}

/// __libc_pread - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __libc_pread(__fd: i32, __buf: *mut core::ffi::c_void, __nbytes: usize, __offset: usize) -> isize {
    0
}

/// __pread64 - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __pread64(__fd: i32, __buf: *mut core::ffi::c_void, __nbytes: usize, __offset: usize) -> isize {
    0
}

/// __libc_pread64 - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __libc_pread64(__fd: i32, __buf: *mut core::ffi::c_void, __nbytes: usize, __offset: usize) -> isize {
    0
}

/// __pwrite - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __pwrite(__fd: i32, __buf: *mut core::ffi::c_void, __n: usize, __offset: usize) -> isize {
    0
}

/// __libc_pwrite - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __libc_pwrite(__fd: i32, __buf: *mut core::ffi::c_void, __n: usize, __offset: usize) -> isize {
    0
}

/// __pwrite64 - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __pwrite64(__fd: i32, __buf: *mut core::ffi::c_void, __n: usize, __offset: usize) -> isize {
    0
}

/// __libc_pwrite64 - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __libc_pwrite64(__fd: i32, __buf: *mut core::ffi::c_void, __n: usize, __offset: usize) -> isize {
    0
}

/// __libc_read - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __libc_read(__fd: i32, __buf: *mut core::ffi::c_void, __n: usize) -> isize {
    0
}

/// __libc_write - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __libc_write(__fd: i32, __buf: *mut core::ffi::c_void, __n: usize) -> isize {
    0
}

/// __pipe - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __pipe(__pipedes2: i32) -> i32 {
    0
}

/// __pipe2 - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __pipe2(__pipedes2: i32, __flags: i32) -> i32 {
    0
}

/// __sleep - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __sleep(__seconds: u32) -> u32 {
    0
}

/// __chown - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __chown(__file: *mut i8, __owner: usize, __group: usize) -> i32 {
    0
}

/// __fchown - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __fchown(__fd: i32, __owner: usize, __group: usize) -> i32 {
    0
}

/// __fchownat - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __fchownat(__fd: i32, __file: *mut i8, __owner: usize, __group: usize, __flag: i32) -> i32 {
    0
}

/// __lchown - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __lchown(__file: *mut i8, __owner: usize, __group: usize) -> i32 {
    0
}

/// __chdir - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __chdir(__path: *mut i8) -> i32 {
    0
}

/// __fchdir - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __fchdir(__fd: i32) -> i32 {
    0
}

/// __rmdir - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __rmdir(__path: *mut i8) -> i32 {
    0
}

/// __execvpe - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __execvpe(file: *mut i8, argv: *mut i8, envp: *mut i8) -> i32 {
    0
}

/// __execvpex - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __execvpex(file: *mut i8, argv: *mut i8, envp: *mut i8) -> i32 {
    0
}

/// __dup - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __dup(__fd: i32) -> i32 {
    0
}

/// __dup2 - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __dup2(__fd: i32, __fd2: i32) -> i32 {
    0
}

/// __dup3 - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __dup3(__fd: i32, __fd2: i32, flags: i32) -> i32 {
    0
}

/// __execve - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __execve(__path: *mut i8, __argv: *mut i8, __envp: *mut i8) -> i32 {
    0
}

/// __execveat - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __execveat(dirfd: i32, __path: *mut i8, __argv: *mut i8, __envp: *mut i8, flags: i32) -> i32 {
    0
}

/// __pathconf - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __pathconf(__path: *mut i8, __name: i32) -> i64 {
    0
}

/// __fpathconf - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __fpathconf(__fd: i32, __name: i32) -> i64 {
    0
}

/// __getpid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __getpid() -> usize {
    0
}

/// __getppid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __getppid() -> usize {
    0
}

/// __setsid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __setsid() -> usize {
    0
}

/// __getuid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __getuid() -> usize {
    0
}

/// __geteuid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __geteuid() -> usize {
    0
}

/// __getgid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __getgid() -> usize {
    0
}

/// __getegid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __getegid() -> usize {
    0
}

/// __getgroups - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __getgroups(__size: i32, __list: usize) -> i32 {
    0
}

/// __group_member - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __group_member(__gid: usize) -> i32 {
    0
}

/// __setuid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __setuid(__uid: usize) -> i32 {
    0
}

/// __setreuid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __setreuid(__ruid: usize, __euid: usize) -> i32 {
    0
}

/// __setgid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __setgid(__gid: usize) -> i32 {
    0
}

/// __setpgid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __setpgid(__pid: usize, __pgid: usize) -> i32 {
    0
}

/// __setregid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __setregid(__rgid: usize, __egid: usize) -> i32 {
    0
}

/// __getresuid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __getresuid(__ruid: *mut core::ffi::c_void, __euid: *mut core::ffi::c_void, __suid: *mut core::ffi::c_void) -> i32 {
    0
}

/// __getresgid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __getresgid(__rgid: *mut core::ffi::c_void, __egid: *mut core::ffi::c_void, __sgid: *mut core::ffi::c_void) -> i32 {
    0
}

/// __setresuid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __setresuid(__ruid: usize, __euid: usize, __suid: usize) -> i32 {
    0
}

/// __setresgid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __setresgid(__rgid: usize, __egid: usize, __sgid: usize) -> i32 {
    0
}

/// __vfork - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __vfork() -> usize {
    0
}

/// __ttyname_r - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __ttyname_r(__fd: i32, __buf: *mut i8, __buflen: usize) -> i32 {
    0
}

/// _Fork - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn _Fork() -> usize {
    0
}

/// __isatty - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __isatty(__fd: i32) -> i32 {
    0
}

/// __isatty_nostatus - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __isatty_nostatus(__fd: i32) -> i32 {
    0
}

/// __link - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __link(__from: *mut i8, __to: *mut i8) -> i32 {
    0
}

/// __symlink - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __symlink(__from: *mut i8, __to: *mut i8) -> i32 {
    0
}

/// __symlinkat - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __symlinkat(__from: *mut i8, __fd: i32, __to: *mut i8) -> i32 {
    0
}

/// __readlink - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __readlink(__path: *mut i8, __buf: *mut i8, __len: usize) -> isize {
    0
}

/// __readlinkat - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __readlinkat(__fd: i32, __file_name: *mut i8, __buf: *mut i8, __len: usize) -> isize {
    0
}

/// __unlink - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __unlink(__name: *mut i8) -> i32 {
    0
}

/// __unlinkat - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __unlinkat(__fd: i32, __name: *mut i8, __flag: i32) -> i32 {
    0
}

/// __gethostname - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __gethostname(__name: *mut i8, __len: usize) -> i32 {
    0
}

/// __revoke - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __revoke(__file: *mut i8) -> i32 {
    0
}

/// __profil - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __profil(__sample_buffer: *mut core::ffi::c_void, __size: usize, __offset: usize, __scale: u32) -> i32 {
    0
}

/// __getdtablesize - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __getdtablesize() -> i32 {
    0
}

/// __brk - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __brk(__addr: *mut core::ffi::c_void) -> i32 {
    0
}

/// __close - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __close(__fd: i32) -> i32 {
    0
}

/// __libc_close - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __libc_close(__fd: i32) -> i32 {
    0
}

/// __closefrom_fallback - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __closefrom_fallback(__lowfd: i32, dirfd_fallback: usize) -> usize {
    0
}

/// __read - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __read(__fd: i32, __buf: *mut core::ffi::c_void, __nbytes: usize) -> isize {
    0
}

/// __write - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __write(__fd: i32, __buf: *mut core::ffi::c_void, __n: usize) -> isize {
    0
}

/// __fork - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __fork() -> usize {
    0
}

/// __ftruncate - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __ftruncate(__fd: i32, __length: usize) -> i32 {
    0
}

/// __ftruncate64 - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __ftruncate64(__fd: i32, __length: usize) -> i32 {
    0
}

/// __truncate - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __truncate(path: *mut i8, __length: usize) -> i32 {
    0
}

/// __tcsetpgrp - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __tcsetpgrp(fd: i32, pgrp: usize) -> i32 {
    0
}

/// __libc_check_standard_fds - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __libc_check_standard_fds() {

}

/// __libc_fork - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __libc_fork() -> usize {
    0
}

/// __libc_pause - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __libc_pause() -> i32 {
    0
}

/// __getlogin_r_loginuid - from glibc/unistd.h
#[no_mangle]
pub unsafe extern "C" fn __getlogin_r_loginuid(name: *mut i8, namesize: usize) -> i32 {
    0
}

/// write_all - from glibc/unistd_ext.h
#[no_mangle]
pub unsafe extern "C" fn write_all(fd: i32, buffer: *mut core::ffi::c_void, length: usize) {

}

/// read_all - from glibc/unistd_ext.h
#[no_mangle]
pub unsafe extern "C" fn read_all(fd: i32, buffer: *mut core::ffi::c_void, length: usize) {

}

/// __vfwscanf - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __vfwscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __swprintf_chk - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __swprintf_chk(__s: *mut core::ffi::c_void, __n: usize, __flag: i32, __s_len: usize, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __fwprintf - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __fwprintf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __wprintf_chk - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_chk(__flag: i32, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __vfwprintf_chk - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __vfwprintf_chk(__s: *mut core::ffi::c_void, __flag: i32, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __vswprintf_chk - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __vswprintf_chk(__s: *mut core::ffi::c_void, __n: usize, __flag: i32, __s_len: usize, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __fwprintf_chk - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __fwprintf_chk(__stream: *mut core::ffi::c_void, __flag: i32, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __vwprintf_chk - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __vwprintf_chk(__flag: i32, __format: *mut core::ffi::c_void, __ap: usize) -> i32 {
    0
}

/// __isoc99_fwscanf - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __isoc99_fwscanf(__stream: *mut core::ffi::c_void, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __isoc99_wscanf - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __isoc99_wscanf(__format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __isoc99_swscanf - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __isoc99_swscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __isoc99_vfwscanf - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __isoc99_vfwscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __isoc99_vwscanf - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __isoc99_vwscanf(__format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __isoc99_vswscanf - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __isoc99_vswscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __isoc23_fwscanf - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __isoc23_fwscanf(__stream: *mut core::ffi::c_void, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __isoc23_wscanf - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __isoc23_wscanf(__format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __isoc23_swscanf - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __isoc23_swscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// __isoc23_vfwscanf - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __isoc23_vfwscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __isoc23_vwscanf - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __isoc23_vwscanf(__format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __isoc23_vswscanf - from glibc/wchar.h
#[no_mangle]
pub unsafe extern "C" fn __isoc23_vswscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// fcntl - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn fcntl(__fd: i32, __cmd: i32) -> i32 {
    0
}

/// fcntl64 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn fcntl64(__fd: i32, __cmd: i32) -> i32 {
    0
}

/// __fcntl_time64 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn __fcntl_time64(__fd: i32, __request: i32) -> i32 {
    0
}

/// open - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn open(__file: *mut i8, __oflag: i32) -> i32 {
    0
}

/// open64 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn open64(__file: *mut i8, __oflag: i32) -> i32 {
    0
}

/// openat - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn openat(__fd: i32, __file: *mut i8, __oflag: i32) -> i32 {
    0
}

/// openat64 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn openat64(__fd: i32, __file: *mut i8, __oflag: i32) -> i32 {
    0
}

/// creat - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn creat(__file: *mut i8, __mode: usize) -> i32 {
    0
}

/// creat64 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn creat64(__file: *mut i8, __mode: usize) -> i32 {
    0
}

/// lockf - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn lockf(__fd: i32, __cmd: i32, __len: usize) -> i32 {
    0
}

/// lockf64 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn lockf64(__fd: i32, __cmd: i32, __len: usize) -> i32 {
    0
}

/// posix_fadvise - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn posix_fadvise(__fd: i32, __offset: usize, __len: usize, __advise: i32) -> i32 {
    0
}

/// posix_fadvise64 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn posix_fadvise64(__fd: i32, __offset: usize, __len: usize, __advise: i32) -> i32 {
    0
}

/// posix_fallocate - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn posix_fallocate(__fd: i32, __offset: usize, __len: usize) -> i32 {
    0
}

/// posix_fallocate64 - from glibc/fcntl.h
#[no_mangle]
pub unsafe extern "C" fn posix_fallocate64(__fd: i32, __offset: usize, __len: usize) -> i32 {
    0
}

/// _IO_fclose - from glibc/iolibio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_fclose(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// _IO_old_fclose - from glibc/iolibio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_old_fclose(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// _IO_fprintf - from glibc/iolibio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_fprintf(arg0: *mut core::ffi::c_void, arg1: *mut i8) -> i32 {
    0
}

/// _IO_fread - from glibc/iolibio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_fread(arg0: *mut core::ffi::c_void, arg1: usize, arg2: usize, arg3: *mut core::ffi::c_void) -> usize {
    0
}

/// _IO_fwrite - from glibc/iolibio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_fwrite(arg0: *mut core::ffi::c_void, arg1: usize, arg2: usize, arg3: *mut core::ffi::c_void) -> usize {
    0
}

/// _IO_printf - from glibc/iolibio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_printf(arg0: *mut i8) -> i32 {
    0
}

/// _IO_scanf - from glibc/iolibio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_scanf(arg0: *mut i8) -> i32 {
    0
}

/// _IO_sscanf - from glibc/iolibio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_sscanf(arg0: *mut i8, arg1: *mut i8) -> i32 {
    0
}

/// _IO_sprintf - from glibc/iolibio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_sprintf(arg0: *mut i8, arg1: *mut i8) -> i32 {
    0
}

/// _IO_vsscanf - from glibc/iolibio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_vsscanf(arg0: *mut i8, arg1: *mut i8, arg2: usize) -> i32 {
    0
}

/// _IO_vfscanf - from glibc/libio.h
#[no_mangle]
pub unsafe extern "C" fn _IO_vfscanf(__restrict: *mut core::ffi::c_void, __restrict_1: *mut i8, arg2: usize, __restrict_3: *mut i32) -> i32 {
    0
}

/// _IO_default_write - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_default_write(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: isize) -> isize {
    0
}

/// _IO_default_read - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_default_read(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: isize) -> isize {
    0
}

/// __printf_buffer_as_file_overflow - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_as_file_overflow(fp: *mut core::ffi::c_void, ch: i32) -> i32 {
    0
}

/// __printf_buffer_as_file_xsputn - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn __printf_buffer_as_file_xsputn(fp: *mut core::ffi::c_void, buf: *mut core::ffi::c_void, len: usize) -> usize {
    0
}

/// __wprintf_buffer_as_file_overflow - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_buffer_as_file_overflow(fp: *mut core::ffi::c_void, ch: i32) -> usize {
    0
}

/// __wprintf_buffer_as_file_xsputn - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn __wprintf_buffer_as_file_xsputn(fp: *mut core::ffi::c_void, buf: *mut core::ffi::c_void, len: usize) -> usize {
    0
}

/// _IO_do_write - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_do_write(arg0: *mut core::ffi::c_void, arg1: *mut i8, arg2: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _IO_new_do_write - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_new_do_write(arg0: *mut core::ffi::c_void, arg1: *mut i8, arg2: usize) -> i32 {
    0
}

/// _IO_old_do_write - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_old_do_write(arg0: *mut core::ffi::c_void, arg1: *mut i8, arg2: usize) -> i32 {
    0
}

/// _IO_wdo_write - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_wdo_write(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: usize) -> i32 {
    0
}

/// _IO_file_fopen - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_file_fopen(arg0: *mut core::ffi::c_void, arg1: *mut i8, arg2: *mut i8, arg3: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _IO_file_write - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_file_write(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: isize) -> isize {
    0
}

/// _IO_file_read - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_file_read(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: isize) -> isize {
    0
}

/// _IO_new_file_fopen - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_new_file_fopen(arg0: *mut core::ffi::c_void, arg1: *mut i8, arg2: *mut i8, arg3: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _IO_new_file_write - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_new_file_write(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: isize) -> isize {
    0
}

/// _IO_old_file_fopen - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_old_file_fopen(arg0: *mut core::ffi::c_void, arg1: *mut i8, arg2: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _IO_old_file_write - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_old_file_write(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, arg2: isize) -> isize {
    0
}

/// _IO_cookie_read - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_cookie_read(fp: *mut core::ffi::c_void, buf: *mut core::ffi::c_void, size: isize) -> isize {
    0
}

/// _IO_cookie_write - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_cookie_write(fp: *mut core::ffi::c_void, buf: *mut core::ffi::c_void, size: isize) -> isize {
    0
}

/// __vfprintf_internal - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn __vfprintf_internal(fp: *mut core::ffi::c_void, format: *mut i8, ap: usize, mode_flags: u32) -> i32 {
    0
}

/// __vfwprintf_internal - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn __vfwprintf_internal(fp: *mut core::ffi::c_void, format: *mut core::ffi::c_void, ap: usize, mode_flags: u32) -> i32 {
    0
}

/// __vasprintf_internal - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn __vasprintf_internal(result_ptr: *mut *mut i8, format: *mut i8, ap: usize, mode_flags: u32) -> i32 {
    0
}

/// __vdprintf_internal - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn __vdprintf_internal(d: i32, format: *mut i8, ap: usize, mode_flags: u32) -> i32 {
    0
}

/// __obstack_vprintf_internal - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn __obstack_vprintf_internal(ob: *mut core::ffi::c_void, fmt: *mut i8, ap: usize, mode_flags: u32) -> i32 {
    0
}

/// __vsprintf_internal - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn __vsprintf_internal(string: *mut i8, maxlen: usize, format: *mut i8, ap: usize, mode_flags: u32) -> i32 {
    0
}

/// __vsnprintf_internal - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn __vsnprintf_internal(string: *mut i8, maxlen: usize, format: *mut i8, ap: usize, mode_flags: u32) -> i32 {
    0
}

/// __vswprintf_internal - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn __vswprintf_internal(string: *mut core::ffi::c_void, maxlen: usize, format: *mut core::ffi::c_void, ap: usize, mode_flags: u32) -> i32 {
    0
}

/// __vfscanf_internal - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn __vfscanf_internal(fp: *mut core::ffi::c_void, format: *mut i8, argp: usize, flags: u32) -> i32 {
    0
}

/// __vfwscanf_internal - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn __vfwscanf_internal(fp: *mut core::ffi::c_void, format: *mut core::ffi::c_void, argp: usize, flags: u32) -> i32 {
    0
}

/// _IO_vscanf - from glibc/libioP.h
#[no_mangle]
pub unsafe extern "C" fn _IO_vscanf(arg0: *mut i8, arg1: usize) -> i32 {
    0
}

/// remove - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn remove(__filename: *mut i8) -> i32 {
    0
}

/// rename - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn rename(__old: *mut i8, __new: *mut i8) -> i32 {
    0
}

/// renameat - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn renameat(__oldfd: i32, __old: *mut i8, __newfd: i32, __new: *mut i8) -> i32 {
    0
}

/// renameat2 - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn renameat2(__oldfd: i32, __old: *mut i8, __newfd: i32, __new: *mut i8, __flags: u32) -> i32 {
    0
}

/// fclose - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fclose(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// fflush - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fflush(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// fflush_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fflush_unlocked(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// fcloseall - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fcloseall() -> i32 {
    0
}

/// setbuf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn setbuf(__stream: *mut core::ffi::c_void, __buf: *mut core::ffi::c_void) {

}

/// setvbuf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn setvbuf(__stream: *mut core::ffi::c_void, __buf: *mut core::ffi::c_void, __modes: i32, __n: usize) -> i32 {
    0
}

/// setbuffer - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn setbuffer(__stream: *mut core::ffi::c_void, __buf: *mut core::ffi::c_void, __size: usize) {

}

/// setlinebuf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn setlinebuf(__stream: *mut core::ffi::c_void) {

}

/// printf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn printf(__format: *mut core::ffi::c_void) -> i32 {
    0
}

/// sprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn sprintf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// vfprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn vfprintf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// vprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn vprintf(__format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// vsprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn vsprintf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// snprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn snprintf(__s: *mut core::ffi::c_void, __maxlen: usize, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// vsnprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn vsnprintf(__s: *mut core::ffi::c_void, __maxlen: usize, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// vasprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn vasprintf(__ptr: *mut *mut core::ffi::c_void, __f: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// __asprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __asprintf(__ptr: *mut *mut core::ffi::c_void, __fmt: *mut core::ffi::c_void) -> i32 {
    0
}

/// asprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn asprintf(__ptr: *mut *mut core::ffi::c_void, __fmt: *mut core::ffi::c_void) -> i32 {
    0
}

/// vdprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn vdprintf(__fd: i32, __fmt: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// fscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fscanf(__stream: *mut core::ffi::c_void, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// scanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn scanf(__format: *mut core::ffi::c_void) -> i32 {
    0
}

/// sscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn sscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// vfscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn vfscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// vscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn vscanf(__format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// vsscanf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn vsscanf(__s: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __arg: usize) -> i32 {
    0
}

/// fgetc - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fgetc(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// getc - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn getc(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// getchar - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn getchar() -> i32 {
    0
}

/// getc_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn getc_unlocked(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// getchar_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn getchar_unlocked() -> i32 {
    0
}

/// fgetc_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fgetc_unlocked(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// fputc - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fputc(__c: i32, __stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// putc - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn putc(__c: i32, __stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// putchar - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn putchar(__c: i32) -> i32 {
    0
}

/// fputc_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fputc_unlocked(__c: i32, __stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// putc_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn putc_unlocked(__c: i32, __stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// putchar_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn putchar_unlocked(__c: i32) -> i32 {
    0
}

/// getw - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn getw(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// putw - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn putw(__w: i32, __stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// __fortified_attr_access - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __fortified_attr_access(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// __getdelim - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __getdelim(__lineptr: *mut *mut core::ffi::c_void, __n: *mut core::ffi::c_void, __delimiter: i32, __stream: *mut core::ffi::c_void) -> usize {
    0
}

/// getdelim - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn getdelim(__lineptr: *mut *mut core::ffi::c_void, __n: *mut core::ffi::c_void, __delimiter: i32, __stream: *mut core::ffi::c_void) -> usize {
    0
}

/// getline - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn getline(__lineptr: *mut *mut core::ffi::c_void, __n: *mut core::ffi::c_void, __stream: *mut core::ffi::c_void) -> usize {
    0
}

/// fputs - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fputs(__s: *mut core::ffi::c_void, __stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// puts - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn puts(__s: *mut i8) -> i32 {
    0
}

/// fputs_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fputs_unlocked(__s: *mut core::ffi::c_void, __stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// fread_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fread_unlocked(__ptr: *mut core::ffi::c_void, __size: usize, __n: usize, __stream: *mut core::ffi::c_void) -> usize {
    0
}

/// fwrite_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fwrite_unlocked(__ptr: *mut core::ffi::c_void, __size: usize, __n: usize, __stream: *mut core::ffi::c_void) -> usize {
    0
}

/// fseek - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fseek(__stream: *mut core::ffi::c_void, __off: i64, __whence: i32) -> i32 {
    0
}

/// ftell - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn ftell(__stream: *mut core::ffi::c_void) -> i64 {
    0
}

/// rewind - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn rewind(__stream: *mut core::ffi::c_void) {

}

/// fseeko - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fseeko(__stream: *mut core::ffi::c_void, __off: usize, __whence: i32) -> i32 {
    0
}

/// ftello - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn ftello(__stream: *mut core::ffi::c_void) -> usize {
    0
}

/// fgetpos - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fgetpos(__stream: *mut core::ffi::c_void, __pos: *mut core::ffi::c_void) -> i32 {
    0
}

/// fsetpos - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fsetpos(__stream: *mut core::ffi::c_void, __pos: *mut core::ffi::c_void) -> i32 {
    0
}

/// fseeko64 - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fseeko64(__stream: *mut core::ffi::c_void, __off: usize, __whence: i32) -> i32 {
    0
}

/// ftello64 - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn ftello64(__stream: *mut core::ffi::c_void) -> usize {
    0
}

/// fgetpos64 - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fgetpos64(__stream: *mut core::ffi::c_void, __pos: *mut core::ffi::c_void) -> i32 {
    0
}

/// fsetpos64 - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fsetpos64(__stream: *mut core::ffi::c_void, __pos: *mut core::ffi::c_void) -> i32 {
    0
}

/// clearerr - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn clearerr(__stream: *mut core::ffi::c_void) {

}

/// ferror - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn ferror(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// clearerr_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn clearerr_unlocked(__stream: *mut core::ffi::c_void) {

}

/// feof_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn feof_unlocked(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// ferror_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn ferror_unlocked(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// fileno - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fileno(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// fileno_unlocked - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn fileno_unlocked(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// pclose - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn pclose(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// __attr_access - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __attr_access(arg0: usize, arg1: usize) -> usize {
    0
}

/// obstack_printf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn obstack_printf(__obstack: *mut core::ffi::c_void, __format: *mut core::ffi::c_void) -> i32 {
    0
}

/// obstack_vprintf - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn obstack_vprintf(__obstack: *mut core::ffi::c_void, __format: *mut core::ffi::c_void, __args: usize) -> i32 {
    0
}

/// flockfile - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn flockfile(__stream: *mut core::ffi::c_void) {

}

/// ftrylockfile - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn ftrylockfile(__stream: *mut core::ffi::c_void) -> i32 {
    0
}

/// funlockfile - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn funlockfile(__stream: *mut core::ffi::c_void) {

}

/// __uflow - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __uflow(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// __overflow - from glibc/stdio.h
#[no_mangle]
pub unsafe extern "C" fn __overflow(arg0: *mut core::ffi::c_void, arg1: i32) -> i32 {
    0
}

/// _IO_strfile_read - from glibc/strfile.h
#[no_mangle]
pub unsafe extern "C" fn _IO_strfile_read(sf: *mut core::ffi::c_void, string: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _IO_strfile_readw - from glibc/strfile.h
#[no_mangle]
pub unsafe extern "C" fn _IO_strfile_readw(sf: *mut core::ffi::c_void, wd: *mut core::ffi::c_void, string: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// __mach_setup_thread - from glibc/mach.h
#[no_mangle]
pub unsafe extern "C" fn __mach_setup_thread(task: usize, thread: usize, pc: *mut core::ffi::c_void, stack_base: *mut core::ffi::c_void, stack_size: *mut core::ffi::c_void) -> usize {
    0
}

/// mach_setup_thread - from glibc/mach.h
#[no_mangle]
pub unsafe extern "C" fn mach_setup_thread(task: usize, thread: usize, pc: *mut core::ffi::c_void, stack_base: *mut core::ffi::c_void, stack_size: *mut core::ffi::c_void) -> usize {
    0
}

/// __mach_setup_thread_call - from glibc/setup-thread.h
#[no_mangle]
pub unsafe extern "C" fn __mach_setup_thread_call(task: usize, thread: usize, function: *mut core::ffi::c_void, stack_base: *mut core::ffi::c_void, stack_size: *mut core::ffi::c_void) -> usize {
    0
}

/// bench_singlethread - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn bench_singlethread(json_ctx: *mut core::ffi::c_void) {

}

/// thread_test - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_test(p: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// benchmark_thread - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn benchmark_thread(arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// test_rwlock_read - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rwlock_read(iters: i64, filler: i32) -> usize {
    0
}

/// test_rwlock_tryread - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rwlock_tryread(iters: i64, filler: i32) -> usize {
    0
}

/// test_rwlock_write - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rwlock_write(iters: i64, filler: i32) -> usize {
    0
}

/// test_rwlock_trywrite - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn test_rwlock_trywrite(iters: i64, filler: i32) -> usize {
    0
}

/// test_producer_thread - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn test_producer_thread(v: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// thread_start - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_start(p: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// read_file - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn read_file(filename: *mut i8) -> *mut i8 {
    core::ptr::null_mut()
}

/// AnnotateThreadName - from cpython/dynamic_annotations.h
#[no_mangle]
pub unsafe extern "C" fn AnnotateThreadName(file: *mut i8, line: i32, name: *mut i8) {

}

/// AnnotateIgnoreReadsBegin - from cpython/dynamic_annotations.h
#[no_mangle]
pub unsafe extern "C" fn AnnotateIgnoreReadsBegin(file: *mut i8, line: i32) {

}

/// AnnotateIgnoreReadsEnd - from cpython/dynamic_annotations.h
#[no_mangle]
pub unsafe extern "C" fn AnnotateIgnoreReadsEnd(file: *mut i8, line: i32) {

}

/// AnnotateIgnoreWritesBegin - from cpython/dynamic_annotations.h
#[no_mangle]
pub unsafe extern "C" fn AnnotateIgnoreWritesBegin(file: *mut i8, line: i32) {

}

/// AnnotateIgnoreWritesEnd - from cpython/dynamic_annotations.h
#[no_mangle]
pub unsafe extern "C" fn AnnotateIgnoreWritesEnd(file: *mut i8, line: i32) {

}

/// _Py_ANNOTATE_UNPROTECTED_READ - from cpython/dynamic_annotations.h
#[no_mangle]
pub unsafe extern "C" fn _Py_ANNOTATE_UNPROTECTED_READ(x: usize) -> usize {
    0
}

/// PyThread_exit_thread - from cpython/pythread.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_exit_thread() -> usize {
    0
}

/// _PyThread_cond_init - from cpython/condvar.h
#[no_mangle]
pub unsafe extern "C" fn _PyThread_cond_init(cond: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyThread_cond_after - from cpython/condvar.h
#[no_mangle]
pub unsafe extern "C" fn _PyThread_cond_after(us: i64, abs: *mut core::ffi::c_void) {

}

/// _PyMarshal_ReadObjectFromXIData - from cpython/crossinterp_data_lookup.h
#[no_mangle]
pub unsafe extern "C" fn _PyMarshal_ReadObjectFromXIData(arg0: usize) -> usize {
    0
}

/// _Py_RemoteDebug_ReadRemoteMemory - from cpython/remote_debug.h
#[no_mangle]
pub unsafe extern "C" fn _Py_RemoteDebug_ReadRemoteMemory(handle: *mut core::ffi::c_void, remote_address: usize, len: usize, dst: *mut core::ffi::c_void) -> i32 {
    0
}

/// read_remote_memory_fallback - from cpython/remote_debug.h
#[no_mangle]
pub unsafe extern "C" fn read_remote_memory_fallback(handle: *mut core::ffi::c_void, remote_address: usize, len: usize, dst: *mut core::ffi::c_void) -> i32 {
    0
}

/// _Py_RemoteDebug_WriteRemoteMemoryFallback - from cpython/remote_debug.h
#[no_mangle]
pub unsafe extern "C" fn _Py_RemoteDebug_WriteRemoteMemoryFallback(handle: *mut core::ffi::c_void, remote_address: usize, len: usize, src: *mut core::ffi::c_void) -> i32 {
    0
}

/// _Py_RemoteDebug_WriteRemoteMemory - from cpython/remote_debug.h
#[no_mangle]
pub unsafe extern "C" fn _Py_RemoteDebug_WriteRemoteMemory(handle: *mut core::ffi::c_void, remote_address: usize, len: usize, src: *mut core::ffi::c_void) -> usize {
    0
}

/// _Py_RemoteDebug_PagedReadRemoteMemory - from cpython/remote_debug.h
#[no_mangle]
pub unsafe extern "C" fn _Py_RemoteDebug_PagedReadRemoteMemory(handle: *mut core::ffi::c_void, addr: usize, size: usize, out: *mut core::ffi::c_void) -> usize {
    0
}

/// _Py_RemoteDebug_ReadDebugOffsets - from cpython/remote_debug.h
#[no_mangle]
pub unsafe extern "C" fn _Py_RemoteDebug_ReadDebugOffsets(handle: *mut core::ffi::c_void, runtime_start_address: *mut usize, debug_offsets: *mut core::ffi::c_void) -> usize {
    0
}

/// PyThread_get_thread_ident - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_get_thread_ident() -> u64 {
    0
}

/// PyThread_get_thread_native_id - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_get_thread_native_id() -> u64 {
    0
}

/// PyThread__init_thread - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread__init_thread() {

}

/// PyThread_start_joinable_thread - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_start_joinable_thread(func: *mut core::ffi::c_void) -> i32 {
    0
}

/// PyThread_start_new_thread - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_start_new_thread(func: *mut core::ffi::c_void) -> u64 {
    0
}

/// PyThread_join_thread - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_join_thread(handle: usize) -> i32 {
    0
}

/// PyThread_detach_thread - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_detach_thread(handle: usize) -> i32 {
    0
}

/// PyThread_get_thread_ident_ex - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_get_thread_ident_ex() -> usize {
    0
}

/// PyThread_hang_thread - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_hang_thread() -> usize {
    0
}

/// _pythread_nt_set_stacksize - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn _pythread_nt_set_stacksize(size: usize) -> i32 {
    0
}

/// PyThread_create_key - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_create_key() -> i32 {
    0
}

/// PyThread_delete_key - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_delete_key(key: i32) {

}

/// PyThread_set_key_value - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_set_key_value(key: i32, value: *mut core::ffi::c_void) -> i32 {
    0
}

/// PyThread_get_key_value - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_get_key_value(key: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyThread_delete_key_value - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_delete_key_value(key: i32) {

}

/// PyThread_ReInitTLS - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_ReInitTLS() {

}

/// PyThread_tss_create - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_tss_create(key: *mut core::ffi::c_void) -> i32 {
    0
}

/// PyThread_tss_delete - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_tss_delete(key: *mut core::ffi::c_void) {

}

/// PyThread_tss_set - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_tss_set(key: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> i32 {
    0
}

/// PyThread_tss_get - from cpython/thread_nt.h
#[no_mangle]
pub unsafe extern "C" fn PyThread_tss_get(key: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pthread_cond_init - from cpython/thread_pthread.h
#[no_mangle]
pub unsafe extern "C" fn pthread_cond_init(arg0: usize, arg1: usize) -> usize {
    0
}

/// pthread_init - from cpython/thread_pthread.h
#[no_mangle]
pub unsafe extern "C" fn pthread_init() {

}

/// pythread_wrapper - from cpython/thread_pthread.h
#[no_mangle]
pub unsafe extern "C" fn pythread_wrapper(arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// do_start_joinable_thread - from cpython/thread_pthread.h
#[no_mangle]
pub unsafe extern "C" fn do_start_joinable_thread(func: *mut core::ffi::c_void) -> i32 {
    0
}

/// _pthread_t_to_ident - from cpython/thread_pthread.h
#[no_mangle]
pub unsafe extern "C" fn _pthread_t_to_ident(value: usize) -> usize {
    0
}

/// pthread_join - from cpython/thread_pthread.h
#[no_mangle]
pub unsafe extern "C" fn pthread_join(arg0: usize) -> usize {
    0
}

/// pthread_detach - from cpython/thread_pthread.h
#[no_mangle]
pub unsafe extern "C" fn pthread_detach(arg0: usize) -> usize {
    0
}

/// _pythread_pthread_set_stacksize - from cpython/thread_pthread.h
#[no_mangle]
pub unsafe extern "C" fn _pythread_pthread_set_stacksize(size: usize) -> i32 {
    0
}

/// pthread_getspecific - from cpython/thread_pthread.h
#[no_mangle]
pub unsafe extern "C" fn pthread_getspecific(arg0: usize) -> usize {
    0
}

/// pthread_mutex_init - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_mutex_init(mutex: *mut core::ffi::c_void, attr: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_mutex_destroy - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_mutex_destroy(mutex: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_mutex_trylock - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_mutex_trylock(mutex: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_mutex_lock - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_mutex_lock(mutex: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_mutex_unlock - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_mutex_unlock(mutex: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_cond_wait - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_cond_wait(cond: *mut core::ffi::c_void, mutex: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_cond_timedwait - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_cond_timedwait(cond: *mut core::ffi::c_void, mutex: *mut core::ffi::c_void, abstime: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_cond_signal - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_cond_signal(cond: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_condattr_init - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_condattr_init(attr: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_condattr_setclock - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_condattr_setclock(attr: *mut core::ffi::c_void, clock_id: usize) -> i32 {
    0
}

/// pthread_create - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_create(thread: *mut core::ffi::c_void, attr: *mut core::ffi::c_void, start_routine: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_exit - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_exit(retval: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_attr_init - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_attr_init(attr: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_attr_setstacksize - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_attr_setstacksize(attr: *mut core::ffi::c_void, stacksize: usize) -> i32 {
    0
}

/// pthread_attr_destroy - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_attr_destroy(attr: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_key_create - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_key_create(key: *mut core::ffi::c_void, destr_function: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_key_delete - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_key_delete(key: usize) -> i32 {
    0
}

/// pthread_setspecific - from cpython/thread_pthread_stubs.h
#[no_mangle]
pub unsafe extern "C" fn pthread_setspecific(key: usize, value: *mut core::ffi::c_void) -> i32 {
    0
}

/// _Py_fopen_obj - from cpython/fileutils.h
#[no_mangle]
pub unsafe extern "C" fn _Py_fopen_obj(path: *mut core::ffi::c_void, mode: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// Py_fopen - from cpython/fileutils.h
#[no_mangle]
pub unsafe extern "C" fn Py_fopen(arg0: usize, arg1: usize) -> usize {
    0
}

/// _Py_ThreadId - from cpython/object.h
#[no_mangle]
pub unsafe extern "C" fn _Py_ThreadId() -> usize {
    0
}

/// _Py_IsOwnedByCurrentThread - from cpython/object.h
#[no_mangle]
pub unsafe extern "C" fn _Py_IsOwnedByCurrentThread(ob: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyThreadState_UncheckedGet - from cpython/pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_UncheckedGet() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyThreadState_GetUnchecked - from cpython/pystate.h
#[no_mangle]
pub unsafe extern "C" fn PyThreadState_GetUnchecked() -> usize {
    0
}

/// _PyThreadState_GetStatsFast - from cpython/pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_GetStatsFast() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyUnicode_IS_READY - from cpython/unicodeobject.h
#[no_mangle]
pub unsafe extern "C" fn PyUnicode_IS_READY(Py_UNUSEDop: *mut core::ffi::c_void) -> u32 {
    0
}

/// PyUnicode_WRITE - from cpython/unicodeobject.h
#[no_mangle]
pub unsafe extern "C" fn PyUnicode_WRITE(kind: i32, data: *mut core::ffi::c_void, index: usize, value: usize) {

}

/// PyUnicode_READ - from cpython/unicodeobject.h
#[no_mangle]
pub unsafe extern "C" fn PyUnicode_READ(kind: i32, data: *mut core::ffi::c_void, index: usize) -> usize {
    0
}

/// PyUnicode_READ_CHAR - from cpython/unicodeobject.h
#[no_mangle]
pub unsafe extern "C" fn PyUnicode_READ_CHAR(unicode: *mut core::ffi::c_void, index: usize) -> usize {
    0
}

/// PyUnicode_READY - from cpython/unicodeobject.h
#[no_mangle]
pub unsafe extern "C" fn PyUnicode_READY(Py_UNUSEDop: *mut core::ffi::c_void) -> i32 {
    0
}

/// _Py_brc_init_thread - from cpython/pycore_brc.h
#[no_mangle]
pub unsafe extern "C" fn _Py_brc_init_thread(tstate: *mut core::ffi::c_void) {

}

/// _Py_brc_remove_thread - from cpython/pycore_brc.h
#[no_mangle]
pub unsafe extern "C" fn _Py_brc_remove_thread(tstate: *mut core::ffi::c_void) {

}

/// _PyBytesWriter_GetSize - from cpython/pycore_bytesobject.h
#[no_mangle]
pub unsafe extern "C" fn _PyBytesWriter_GetSize(writer: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyBytesWriter_GetData - from cpython/pycore_bytesobject.h
#[no_mangle]
pub unsafe extern "C" fn _PyBytesWriter_GetData(writer: *mut core::ffi::c_void) -> *mut i8 {
    core::ptr::null_mut()
}

/// _PyEval_SetProfileAllThreads - from cpython/pycore_ceval.h
#[no_mangle]
pub unsafe extern "C" fn _PyEval_SetProfileAllThreads(interp: *mut core::ffi::c_void, func: usize, arg: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyEval_SetTraceAllThreads - from cpython/pycore_ceval.h
#[no_mangle]
pub unsafe extern "C" fn _PyEval_SetTraceAllThreads(interp: *mut core::ffi::c_void, func: usize, arg: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyEval_ReInitThreads - from cpython/pycore_ceval.h
#[no_mangle]
pub unsafe extern "C" fn _PyEval_ReInitThreads(tstate: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyPerfJit_WriteNamedCode - from cpython/pycore_ceval.h
#[no_mangle]
pub unsafe extern "C" fn _PyPerfJit_WriteNamedCode(code_addr: *mut core::ffi::c_void, code_size: usize, entry: *mut i8, filename: *mut i8) {

}

/// _PyEval_ThreadsInitialized - from cpython/pycore_ceval.h
#[no_mangle]
pub unsafe extern "C" fn _PyEval_ThreadsInitialized() -> i32 {
    0
}

/// write_u32 - from cpython/pycore_code.h
#[no_mangle]
pub unsafe extern "C" fn write_u32(p: *mut u16, val: u32) {

}

/// write_u64 - from cpython/pycore_code.h
#[no_mangle]
pub unsafe extern "C" fn write_u64(p: *mut u16, val: u64) {

}

/// write_ptr - from cpython/pycore_code.h
#[no_mangle]
pub unsafe extern "C" fn write_ptr(p: *mut u16, val: *mut core::ffi::c_void) {

}

/// read_u16 - from cpython/pycore_code.h
#[no_mangle]
pub unsafe extern "C" fn read_u16(p: *mut u16) -> u16 {
    0
}

/// read_u32 - from cpython/pycore_code.h
#[no_mangle]
pub unsafe extern "C" fn read_u32(p: *mut u16) -> u32 {
    0
}

/// read_u64 - from cpython/pycore_code.h
#[no_mangle]
pub unsafe extern "C" fn read_u64(p: *mut u16) -> u64 {
    0
}

/// read_obj - from cpython/pycore_code.h
#[no_mangle]
pub unsafe extern "C" fn read_obj(p: *mut u16) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// write_varint - from cpython/pycore_code.h
#[no_mangle]
pub unsafe extern "C" fn write_varint(ptr: *mut u8, val: u32) -> i32 {
    0
}

/// write_signed_varint - from cpython/pycore_code.h
#[no_mangle]
pub unsafe extern "C" fn write_signed_varint(ptr: *mut u8, val: i32) -> i32 {
    0
}

/// write_location_entry_start - from cpython/pycore_code.h
#[no_mangle]
pub unsafe extern "C" fn write_location_entry_start(ptr: *mut u8, code: i32, length: i32) -> i32 {
    0
}

/// _PyComplex_FormatAdvancedWriter - from cpython/pycore_complexobject.h
#[no_mangle]
pub unsafe extern "C" fn _PyComplex_FormatAdvancedWriter(writer: *mut core::ffi::c_void, obj: *mut core::ffi::c_void, format_spec: *mut core::ffi::c_void, start: usize, end: usize) -> i32 {
    0
}

/// _Py_dict_lookup_threadsafe - from cpython/pycore_dict.h
#[no_mangle]
pub unsafe extern "C" fn _Py_dict_lookup_threadsafe(mp: *mut core::ffi::c_void, key: *mut core::ffi::c_void, hash: usize, value_addr: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// _Py_dict_lookup_threadsafe_stackref - from cpython/pycore_dict.h
#[no_mangle]
pub unsafe extern "C" fn _Py_dict_lookup_threadsafe_stackref(mp: *mut core::ffi::c_void, key: *mut core::ffi::c_void, hash: usize, value_addr: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyDict_EnablePerThreadRefcounting - from cpython/pycore_dict.h
#[no_mangle]
pub unsafe extern "C" fn _PyDict_EnablePerThreadRefcounting(op: *mut core::ffi::c_void) {

}

/// _Py_wfopen - from cpython/pycore_fileutils.h
#[no_mangle]
pub unsafe extern "C" fn _Py_wfopen(path: *mut core::ffi::c_void, mode: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _Py_read - from cpython/pycore_fileutils.h
#[no_mangle]
pub unsafe extern "C" fn _Py_read(fd: i32, buf: *mut core::ffi::c_void, count: usize) -> usize {
    0
}

/// _Py_wreadlink - from cpython/pycore_fileutils.h
#[no_mangle]
pub unsafe extern "C" fn _Py_wreadlink(path: *mut core::ffi::c_void, buf: *mut core::ffi::c_void, buflen: usize) -> i32 {
    0
}

/// _PyFloat_FormatAdvancedWriter - from cpython/pycore_floatobject.h
#[no_mangle]
pub unsafe extern "C" fn _PyFloat_FormatAdvancedWriter(writer: *mut core::ffi::c_void, obj: *mut core::ffi::c_void, format_spec: *mut core::ffi::c_void, start: usize, end: usize) -> i32 {
    0
}

/// _PyPreCmdline_Read - from cpython/pycore_initconfig.h
#[no_mangle]
pub unsafe extern "C" fn _PyPreCmdline_Read(cmdline: *mut core::ffi::c_void, preconfig: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyPreConfig_Read - from cpython/pycore_initconfig.h
#[no_mangle]
pub unsafe extern "C" fn _PyPreConfig_Read(preconfig: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyPreConfig_Write - from cpython/pycore_initconfig.h
#[no_mangle]
pub unsafe extern "C" fn _PyPreConfig_Write(preconfig: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyConfig_Read - from cpython/pycore_initconfig.h
#[no_mangle]
pub unsafe extern "C" fn _PyConfig_Read(config: *mut core::ffi::c_void, compute_path_config: i32) -> usize {
    0
}

/// _PyConfig_Write - from cpython/pycore_initconfig.h
#[no_mangle]
pub unsafe extern "C" fn _PyConfig_Write(config: *mut core::ffi::c_void, runtime: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyThreadState_GetFrame - from cpython/pycore_interpframe.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_GetFrame(tstate: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _PyLong_FormatAdvancedWriter - from cpython/pycore_long.h
#[no_mangle]
pub unsafe extern "C" fn _PyLong_FormatAdvancedWriter(writer: *mut core::ffi::c_void, obj: *mut core::ffi::c_void, format_spec: *mut core::ffi::c_void, start: usize, end: usize) -> i32 {
    0
}

/// _PyLong_FormatWriter - from cpython/pycore_long.h
#[no_mangle]
pub unsafe extern "C" fn _PyLong_FormatWriter(writer: *mut core::ffi::c_void, obj: *mut core::ffi::c_void, base: i32, alternate: i32) -> i32 {
    0
}

/// _PyLong_FormatBytesWriter - from cpython/pycore_long.h
#[no_mangle]
pub unsafe extern "C" fn _PyLong_FormatBytesWriter(writer: *mut core::ffi::c_void, str: *mut i8, obj: *mut core::ffi::c_void, base: i32, alternate: i32) -> *mut i8 {
    core::ptr::null_mut()
}

/// _Py_THREAD_INCREF_OBJECT - from cpython/pycore_object.h
#[no_mangle]
pub unsafe extern "C" fn _Py_THREAD_INCREF_OBJECT(obj: *mut core::ffi::c_void, unique_id: usize) {

}

/// _Py_THREAD_DECREF_OBJECT - from cpython/pycore_object.h
#[no_mangle]
pub unsafe extern "C" fn _Py_THREAD_DECREF_OBJECT(obj: *mut core::ffi::c_void, unique_id: usize) {

}

/// _PyPathConfig_ReadGlobal - from cpython/pycore_pathconfig.h
#[no_mangle]
pub unsafe extern "C" fn _PyPathConfig_ReadGlobal(config: *mut core::ffi::c_void) -> usize {
    0
}

/// _PySys_ReadPreinitWarnOptions - from cpython/pycore_pylifecycle.h
#[no_mangle]
pub unsafe extern "C" fn _PySys_ReadPreinitWarnOptions(options: *mut core::ffi::c_void) -> usize {
    0
}

/// _PySys_ReadPreinitXOptions - from cpython/pycore_pylifecycle.h
#[no_mangle]
pub unsafe extern "C" fn _PySys_ReadPreinitXOptions(config: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyThread_FiniType - from cpython/pycore_pylifecycle.h
#[no_mangle]
pub unsafe extern "C" fn _PyThread_FiniType(interp: *mut core::ffi::c_void) {

}

/// _PyErr_WriteUnraisableDefaultHook - from cpython/pycore_pylifecycle.h
#[no_mangle]
pub unsafe extern "C" fn _PyErr_WriteUnraisableDefaultHook(unraisable: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _PyThreadState_DeleteCurrent - from cpython/pycore_pylifecycle.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_DeleteCurrent(tstate: *mut core::ffi::c_void) {

}

/// _Py_IsMainThread - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _Py_IsMainThread() -> i32 {
    0
}

/// _PyThreadState_IsRunningMain - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_IsRunningMain(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// _Py_ThreadCanHandleSignals - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _Py_ThreadCanHandleSignals(interp: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyThreadState_CheckConsistency - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_CheckConsistency(tstate: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyThreadState_MustExit - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_MustExit(tstate: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyThreadState_HangThread - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_HangThread(tstate: *mut core::ffi::c_void) {

}

/// _PyThreadState_GET - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_GET() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _PyThreadState_GetCurrent - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_GetCurrent() -> usize {
    0
}

/// _PyThreadState_IsAttached - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_IsAttached(tstate: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyThreadState_Attach - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_Attach(tstate: *mut core::ffi::c_void) {

}

/// _PyThreadState_Detach - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_Detach(tstate: *mut core::ffi::c_void) {

}

/// _PyThreadState_Suspend - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_Suspend(tstate: *mut core::ffi::c_void) {

}

/// _PyThreadState_SetShuttingDown - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_SetShuttingDown(tstate: *mut core::ffi::c_void) {

}

/// _PyThreadState_Bind - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_Bind(tstate: *mut core::ffi::c_void) {

}

/// _PyThreadState_RemoveExcept - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_RemoveExcept(tstate: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _PyThreadState_DeleteList - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_DeleteList(list: *mut core::ffi::c_void, is_after_fork: i32) {

}

/// _PyThread_CurrentExceptions - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThread_CurrentExceptions() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _PyThreadState_Swap - from cpython/pycore_pystate.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_Swap(runtime: *mut core::ffi::c_void, newts: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _PyThread_at_fork_reinit - from cpython/pycore_pythread.h
#[no_mangle]
pub unsafe extern "C" fn _PyThread_at_fork_reinit(lock: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyThread_AfterFork - from cpython/pycore_pythread.h
#[no_mangle]
pub unsafe extern "C" fn _PyThread_AfterFork(state: *mut core::ffi::c_void) {

}

/// _PyRuntimeState_ReInitThreads - from cpython/pycore_runtime.h
#[no_mangle]
pub unsafe extern "C" fn _PyRuntimeState_ReInitThreads(runtime: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyThreadState_PushCStackRef - from cpython/pycore_stackref.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_PushCStackRef(tstate: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) {

}

/// _PyThreadState_PushCStackRefNew - from cpython/pycore_stackref.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_PushCStackRefNew(tstate: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, obj: *mut core::ffi::c_void) {

}

/// _PyThreadState_PopCStackRef - from cpython/pycore_stackref.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_PopCStackRef(tstate: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) {

}

/// _PyThreadState_PopCStackRefSteal - from cpython/pycore_stackref.h
#[no_mangle]
pub unsafe extern "C" fn _PyThreadState_PopCStackRefSteal(tstate: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyStats_ThreadInit - from cpython/pycore_stats.h
#[no_mangle]
pub unsafe extern "C" fn _PyStats_ThreadInit(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// _PyStats_ThreadFini - from cpython/pycore_stats.h
#[no_mangle]
pub unsafe extern "C" fn _PyStats_ThreadFini(arg0: *mut core::ffi::c_void) {

}

/// _Py_DumpTracebackThreads - from cpython/pycore_traceback.h
#[no_mangle]
pub unsafe extern "C" fn _Py_DumpTracebackThreads(fd: i32, interp: *mut core::ffi::c_void, current_tstate: *mut core::ffi::c_void, max_threads: usize) -> *mut i8 {
    core::ptr::null_mut()
}

/// _Py_WriteIndentedMargin - from cpython/pycore_traceback.h
#[no_mangle]
pub unsafe extern "C" fn _Py_WriteIndentedMargin(arg0: i32, arg1: *mut i8, arg2: *mut core::ffi::c_void) -> i32 {
    0
}

/// _Py_WriteIndent - from cpython/pycore_traceback.h
#[no_mangle]
pub unsafe extern "C" fn _Py_WriteIndent(arg0: i32, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyType_IsReady - from cpython/pycore_typeobject.h
#[no_mangle]
pub unsafe extern "C" fn _PyType_IsReady(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyUnicodeWriter_InitWithBuffer - from cpython/pycore_unicodeobject.h
#[no_mangle]
pub unsafe extern "C" fn _PyUnicodeWriter_InitWithBuffer(writer: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void) {

}

/// _PyUnicode_DecodeUTF8Writer - from cpython/pycore_unicodeobject.h
#[no_mangle]
pub unsafe extern "C" fn _PyUnicode_DecodeUTF8Writer(writer: *mut core::ffi::c_void, s: *mut i8, size: usize, error_handler: usize, errors: *mut i8, consumed: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyUnicodeWriter_WriteCharInline - from cpython/pycore_unicodeobject.h
#[no_mangle]
pub unsafe extern "C" fn _PyUnicodeWriter_WriteCharInline(writer: *mut core::ffi::c_void, ch: usize) -> i32 {
    0
}

/// _PyUnicode_FormatAdvancedWriter - from cpython/pycore_unicodeobject.h
#[no_mangle]
pub unsafe extern "C" fn _PyUnicode_FormatAdvancedWriter(writer: *mut core::ffi::c_void, obj: *mut core::ffi::c_void, format_spec: *mut core::ffi::c_void, start: usize, end: usize) -> i32 {
    0
}

/// _PyUnicodeWriter_FormatV - from cpython/pycore_unicodeobject.h
#[no_mangle]
pub unsafe extern "C" fn _PyUnicodeWriter_FormatV(writer: *mut core::ffi::c_void, format: *mut i8, vargs: usize) -> i32 {
    0
}

/// _PyObject_DisablePerThreadRefcounting - from cpython/pycore_uniqueid.h
#[no_mangle]
pub unsafe extern "C" fn _PyObject_DisablePerThreadRefcounting(obj: *mut core::ffi::c_void) {

}

/// _PyObject_MergePerThreadRefcounts - from cpython/pycore_uniqueid.h
#[no_mangle]
pub unsafe extern "C" fn _PyObject_MergePerThreadRefcounts(tstate: *mut core::ffi::c_void) {

}

/// _PyObject_FinalizePerThreadRefcounts - from cpython/pycore_uniqueid.h
#[no_mangle]
pub unsafe extern "C" fn _PyObject_FinalizePerThreadRefcounts(tstate: *mut core::ffi::c_void) {

}

/// mi_thread_init - from cpython/mimalloc.h
#[no_mangle]
pub unsafe extern "C" fn mi_thread_init() -> usize {
    0
}

/// mi_thread_done - from cpython/mimalloc.h
#[no_mangle]
pub unsafe extern "C" fn mi_thread_done() -> usize {
    0
}

/// mi_thread_stats_print_out - from cpython/mimalloc.h
#[no_mangle]
pub unsafe extern "C" fn mi_thread_stats_print_out(out: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> usize {
    0
}

/// mi_atomic_thread_fence - from cpython/atomic.h
#[no_mangle]
pub unsafe extern "C" fn mi_atomic_thread_fence(mo: usize) {

}

/// _mi_fprintf - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_fprintf(out: *mut core::ffi::c_void, arg: *mut core::ffi::c_void, fmt: *mut i8) {

}

/// _mi_is_main_thread - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_is_main_thread() -> usize {
    0
}

/// _mi_current_thread_count - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_current_thread_count() -> usize {
    0
}

/// _mi_thread_id - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_thread_id() -> usize {
    0
}

/// _mi_thread_done - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_thread_done(heap: *mut core::ffi::c_void) {

}

/// _mi_thread_data_collect - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_thread_data_collect() {

}

/// _mi_segment_thread_collect - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_segment_thread_collect(tld: *mut core::ffi::c_void) {

}

/// _mi_abandoned_await_readers - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_abandoned_await_readers(pool: *mut core::ffi::c_void) {

}

/// _mi_prim_thread_init_auto_done - from cpython/prim.h
#[no_mangle]
pub unsafe extern "C" fn _mi_prim_thread_init_auto_done() {

}

/// _mi_prim_thread_done_auto_done - from cpython/prim.h
#[no_mangle]
pub unsafe extern "C" fn _mi_prim_thread_done_auto_done() {

}

/// _mi_prim_thread_associate_default_heap - from cpython/prim.h
#[no_mangle]
pub unsafe extern "C" fn _mi_prim_thread_associate_default_heap(heap: *mut core::ffi::c_void) {

}

/// _mi_prim_thread_id - from cpython/prim.h
#[no_mangle]
pub unsafe extern "C" fn _mi_prim_thread_id() -> usize {
    0
}

/// MI_PRIM_THREAD_ID - from cpython/prim.h
#[no_mangle]
pub unsafe extern "C" fn MI_PRIM_THREAD_ID() -> usize {
    0
}

/// mi_prim_tls_pthread_heap_slot - from cpython/prim.h
#[no_mangle]
pub unsafe extern "C" fn mi_prim_tls_pthread_heap_slot() -> *mut *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// faulthandler__fatal_error_c_thread_impl - from cpython/faulthandler.c.h
#[no_mangle]
pub unsafe extern "C" fn faulthandler__fatal_error_c_thread_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// faulthandler__fatal_error_c_thread - from cpython/faulthandler.c.h
#[no_mangle]
pub unsafe extern "C" fn faulthandler__fatal_error_c_thread(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mmap_mmap_read_byte_impl - from cpython/mmapmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn mmap_mmap_read_byte_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mmap_mmap_read_byte - from cpython/mmapmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn mmap_mmap_read_byte(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mmap_mmap_readline_impl - from cpython/mmapmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn mmap_mmap_readline_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mmap_mmap_readline - from cpython/mmapmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn mmap_mmap_readline(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mmap_mmap_read_impl - from cpython/mmapmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn mmap_mmap_read_impl(arg0: *mut core::ffi::c_void, num_bytes: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mmap_mmap_read - from cpython/mmapmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn mmap_mmap_read(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mmap_mmap_write_impl - from cpython/mmapmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn mmap_mmap_write_impl(arg0: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mmap_mmap_write - from cpython/mmapmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn mmap_mmap_write(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mmap_mmap_write_byte_impl - from cpython/mmapmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn mmap_mmap_write_byte_impl(arg0: *mut core::ffi::c_void, value: u8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// mmap_mmap_write_byte - from cpython/mmapmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn mmap_mmap_write_byte(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _overlapped_Overlapped_ReadFile_impl - from cpython/overlapped.c.h
#[no_mangle]
pub unsafe extern "C" fn _overlapped_Overlapped_ReadFile_impl(arg0: *mut core::ffi::c_void, handle: *mut core::ffi::c_void, size: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _overlapped_Overlapped_ReadFile - from cpython/overlapped.c.h
#[no_mangle]
pub unsafe extern "C" fn _overlapped_Overlapped_ReadFile(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _overlapped_Overlapped_ReadFileInto_impl - from cpython/overlapped.c.h
#[no_mangle]
pub unsafe extern "C" fn _overlapped_Overlapped_ReadFileInto_impl(arg0: *mut core::ffi::c_void, handle: *mut core::ffi::c_void, bufobj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _overlapped_Overlapped_ReadFileInto - from cpython/overlapped.c.h
#[no_mangle]
pub unsafe extern "C" fn _overlapped_Overlapped_ReadFileInto(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _overlapped_Overlapped_WriteFile_impl - from cpython/overlapped.c.h
#[no_mangle]
pub unsafe extern "C" fn _overlapped_Overlapped_WriteFile_impl(arg0: *mut core::ffi::c_void, handle: *mut core::ffi::c_void, bufobj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _overlapped_Overlapped_WriteFile - from cpython/overlapped.c.h
#[no_mangle]
pub unsafe extern "C" fn _overlapped_Overlapped_WriteFile(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_readlink_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_readlink_impl(module: *mut core::ffi::c_void, path: *mut core::ffi::c_void, dir_fd: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_readlink - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_readlink(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_read_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_read_impl(module: *mut core::ffi::c_void, fd: i32, length: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_read - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_read(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_readinto_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_readinto_impl(module: *mut core::ffi::c_void, fd: i32, buffer: *mut core::ffi::c_void) -> usize {
    0
}

/// os_readinto - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_readinto(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_readv_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_readv_impl(module: *mut core::ffi::c_void, fd: i32, buffers: *mut core::ffi::c_void) -> usize {
    0
}

/// os_readv - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_readv(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_pread_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_pread_impl(module: *mut core::ffi::c_void, fd: i32, length: usize, offset: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_pread - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_pread(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_preadv_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_preadv_impl(module: *mut core::ffi::c_void, fd: i32, buffers: *mut core::ffi::c_void, offset: usize, flags: i32) -> usize {
    0
}

/// os_preadv - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_preadv(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_write_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_write_impl(module: *mut core::ffi::c_void, fd: i32, data: *mut core::ffi::c_void) -> usize {
    0
}

/// os_write - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_write(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_writev_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_writev_impl(module: *mut core::ffi::c_void, fd: i32, buffers: *mut core::ffi::c_void) -> usize {
    0
}

/// os_writev - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_writev(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_pwrite_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_pwrite_impl(module: *mut core::ffi::c_void, fd: i32, buffer: *mut core::ffi::c_void, offset: usize) -> usize {
    0
}

/// os_pwrite - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_pwrite(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_pwritev_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_pwritev_impl(module: *mut core::ffi::c_void, fd: i32, buffers: *mut core::ffi::c_void, offset: usize, flags: i32) -> usize {
    0
}

/// os_pwritev - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_pwritev(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_eventfd_read_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_eventfd_read_impl(module: *mut core::ffi::c_void, fd: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_eventfd_read - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_eventfd_read(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_eventfd_write_impl - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_eventfd_write_impl(module: *mut core::ffi::c_void, fd: i32, value: u64) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// os_eventfd_write - from cpython/posixmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn os_eventfd_write(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_parse_and_bind_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_parse_and_bind_impl(module: *mut core::ffi::c_void, string: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_parse_and_bind - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_parse_and_bind(module: *mut core::ffi::c_void, string: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_read_init_file_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_read_init_file_impl(module: *mut core::ffi::c_void, filename_obj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_read_init_file - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_read_init_file(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_read_history_file_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_read_history_file_impl(module: *mut core::ffi::c_void, filename_obj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_read_history_file - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_read_history_file(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_write_history_file_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_write_history_file_impl(module: *mut core::ffi::c_void, filename_obj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_write_history_file - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_write_history_file(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_append_history_file_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_append_history_file_impl(module: *mut core::ffi::c_void, nelements: i32, filename_obj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_append_history_file - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_append_history_file(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_set_history_length_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_set_history_length_impl(module: *mut core::ffi::c_void, length: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_set_history_length - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_set_history_length(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_history_length_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_history_length_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_history_length - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_history_length(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_set_completion_display_matches_hook_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_set_completion_display_matches_hook_impl(module: *mut core::ffi::c_void, function: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_set_completion_display_matches_hook - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_set_completion_display_matches_hook(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_set_startup_hook_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_set_startup_hook_impl(module: *mut core::ffi::c_void, function: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_set_startup_hook - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_set_startup_hook(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_set_pre_input_hook_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_set_pre_input_hook_impl(module: *mut core::ffi::c_void, function: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_set_pre_input_hook - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_set_pre_input_hook(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_pre_input_hook_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_pre_input_hook_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_pre_input_hook - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_pre_input_hook(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_completion_type_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_completion_type_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_completion_type - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_completion_type(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_begidx_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_begidx_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_begidx - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_begidx(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_endidx_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_endidx_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_endidx - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_endidx(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_set_completer_delims_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_set_completer_delims_impl(module: *mut core::ffi::c_void, string: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_set_completer_delims - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_set_completer_delims(module: *mut core::ffi::c_void, string: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_remove_history_item_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_remove_history_item_impl(module: *mut core::ffi::c_void, entry_number: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_remove_history_item - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_remove_history_item(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_replace_history_item_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_replace_history_item_impl(module: *mut core::ffi::c_void, entry_number: i32, line: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_replace_history_item - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_replace_history_item(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_add_history_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_add_history_impl(module: *mut core::ffi::c_void, string: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_add_history - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_add_history(module: *mut core::ffi::c_void, string: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_set_auto_history_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_set_auto_history_impl(module: *mut core::ffi::c_void, _should_auto_add_history: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_set_auto_history - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_set_auto_history(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_completer_delims_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_completer_delims_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_completer_delims - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_completer_delims(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_set_completer_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_set_completer_impl(module: *mut core::ffi::c_void, function: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_set_completer - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_set_completer(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_completer_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_completer_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_completer - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_completer(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_history_item_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_history_item_impl(module: *mut core::ffi::c_void, idx: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_history_item - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_history_item(module: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_current_history_length_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_current_history_length_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_current_history_length - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_current_history_length(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_line_buffer_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_line_buffer_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_get_line_buffer - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_get_line_buffer(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_clear_history_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_clear_history_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_clear_history - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_clear_history(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_insert_text_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_insert_text_impl(module: *mut core::ffi::c_void, string: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_insert_text - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_insert_text(module: *mut core::ffi::c_void, string: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_redisplay_impl - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_redisplay_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_redisplay - from cpython/readline.c.h
#[no_mangle]
pub unsafe extern "C" fn readline_redisplay(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// signal_pthread_sigmask_impl - from cpython/signalmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn signal_pthread_sigmask_impl(module: *mut core::ffi::c_void, how: i32, mask: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// signal_pthread_sigmask - from cpython/signalmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn signal_pthread_sigmask(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// signal_pthread_kill_impl - from cpython/signalmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn signal_pthread_kill_impl(module: *mut core::ffi::c_void, thread_id: u64, signalnum: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// signal_pthread_kill - from cpython/signalmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn signal_pthread_kill(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _codecs_readbuffer_encode_impl - from cpython/_codecsmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _codecs_readbuffer_encode_impl(module: *mut core::ffi::c_void, data: *mut core::ffi::c_void, errors: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _codecs_readbuffer_encode - from cpython/_codecsmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _codecs_readbuffer_encode(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _curses_window_overwrite_impl - from cpython/_cursesmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _curses_window_overwrite_impl(arg0: *mut core::ffi::c_void, destwin: *mut core::ffi::c_void, group_right_1: i32, sminrow: i32, smincol: i32, dminrow: i32, dmincol: i32, dmaxrow: i32, dmaxcol: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _curses_window_overwrite - from cpython/_cursesmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _curses_window_overwrite(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _ssl__SSLSocket_write_impl - from cpython/_ssl.c.h
#[no_mangle]
pub unsafe extern "C" fn _ssl__SSLSocket_write_impl(arg0: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _ssl__SSLSocket_write - from cpython/_ssl.c.h
#[no_mangle]
pub unsafe extern "C" fn _ssl__SSLSocket_write(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _ssl__SSLSocket_read_impl - from cpython/_ssl.c.h
#[no_mangle]
pub unsafe extern "C" fn _ssl__SSLSocket_read_impl(arg0: *mut core::ffi::c_void, len: usize, group_right_1: i32, buffer: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _ssl__SSLSocket_read - from cpython/_ssl.c.h
#[no_mangle]
pub unsafe extern "C" fn _ssl__SSLSocket_read(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _ssl_MemoryBIO_read_impl - from cpython/_ssl.c.h
#[no_mangle]
pub unsafe extern "C" fn _ssl_MemoryBIO_read_impl(arg0: *mut core::ffi::c_void, len: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _ssl_MemoryBIO_read - from cpython/_ssl.c.h
#[no_mangle]
pub unsafe extern "C" fn _ssl_MemoryBIO_read(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _ssl_MemoryBIO_write_impl - from cpython/_ssl.c.h
#[no_mangle]
pub unsafe extern "C" fn _ssl_MemoryBIO_write_impl(arg0: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _ssl_MemoryBIO_write - from cpython/_ssl.c.h
#[no_mangle]
pub unsafe extern "C" fn _ssl_MemoryBIO_write(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _ssl_MemoryBIO_write_eof_impl - from cpython/_ssl.c.h
#[no_mangle]
pub unsafe extern "C" fn _ssl_MemoryBIO_write_eof_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _ssl_MemoryBIO_write_eof - from cpython/_ssl.c.h
#[no_mangle]
pub unsafe extern "C" fn _ssl_MemoryBIO_write_eof(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock_acquire_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock_acquire_impl(arg0: *mut core::ffi::c_void, blocking: i32, timeoutobj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock_acquire - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock_acquire(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock_acquire_lock_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock_acquire_lock_impl(arg0: *mut core::ffi::c_void, blocking: i32, timeoutobj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock_acquire_lock - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock_acquire_lock(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock_release_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock_release_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock_release - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock_release(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock_release_lock_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock_release_lock_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock_release_lock - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock_release_lock(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock___enter___impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock___enter___impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock___enter__ - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock___enter__(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock___exit___impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock___exit___impl(arg0: *mut core::ffi::c_void, exc_type: *mut core::ffi::c_void, exc_value: *mut core::ffi::c_void, exc_tb: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock___exit__ - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock___exit__(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock_locked_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock_locked_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock_locked - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock_locked(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock_locked_lock_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock_locked_lock_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock_locked_lock - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock_locked_lock(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock__at_fork_reinit_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock__at_fork_reinit_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_lock__at_fork_reinit - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_lock__at_fork_reinit(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock_acquire_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock_acquire_impl(arg0: *mut core::ffi::c_void, blocking: i32, timeoutobj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock_acquire - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock_acquire(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock___enter___impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock___enter___impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock___enter__ - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock___enter__(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock_release_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock_release_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock_release - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock_release(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock___exit___impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock___exit___impl(arg0: *mut core::ffi::c_void, exc_type: *mut core::ffi::c_void, exc_value: *mut core::ffi::c_void, exc_tb: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock___exit__ - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock___exit__(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock_locked_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock_locked_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock_locked - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock_locked(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock__acquire_restore_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock__acquire_restore_impl(arg0: *mut core::ffi::c_void, state: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock__acquire_restore - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock__acquire_restore(arg0: *mut core::ffi::c_void, state: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock__release_save_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock__release_save_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock__release_save - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock__release_save(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock__recursion_count_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock__recursion_count_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock__recursion_count - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock__recursion_count(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock__is_owned_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock__is_owned_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock__is_owned - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock__is_owned(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock__at_fork_reinit_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock__at_fork_reinit_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_RLock__at_fork_reinit - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_RLock__at_fork_reinit(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread__get_name_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread__get_name_impl(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread__get_name - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread__get_name(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_set_name_impl - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_set_name_impl(module: *mut core::ffi::c_void, name_obj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _thread_set_name - from cpython/_threadmodule.c.h
#[no_mangle]
pub unsafe extern "C" fn _thread_set_name(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_ReadFile_impl - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_ReadFile_impl(module: *mut core::ffi::c_void, handle: *mut core::ffi::c_void, size: u32, use_overlapped: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_ReadFile - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_ReadFile(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_WriteFile_impl - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_WriteFile_impl(module: *mut core::ffi::c_void, handle: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, use_overlapped: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi_WriteFile - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi_WriteFile(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi__mimetypes_read_windows_registry_impl - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi__mimetypes_read_windows_registry_impl(module: *mut core::ffi::c_void, on_type_read: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _winapi__mimetypes_read_windows_registry - from cpython/_winapi.c.h
#[no_mangle]
pub unsafe extern "C" fn _winapi__mimetypes_read_windows_registry(module: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _PyIOBase_check_readable - from cpython/_iomodule.h
#[no_mangle]
pub unsafe extern "C" fn _PyIOBase_check_readable(state: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// binary_writer_write_sample - from cpython/binary_io.h
#[no_mangle]
pub unsafe extern "C" fn binary_writer_write_sample(writer: *mut core::ffi::c_void, stack_frames: *mut core::ffi::c_void, timestamp_us: u64) -> i32 {
    0
}

/// binary_writer_finalize - from cpython/binary_io.h
#[no_mangle]
pub unsafe extern "C" fn binary_writer_finalize(writer: *mut core::ffi::c_void) -> i32 {
    0
}

/// binary_writer_destroy - from cpython/binary_io.h
#[no_mangle]
pub unsafe extern "C" fn binary_writer_destroy(writer: *mut core::ffi::c_void) {

}

/// binary_reader_replay - from cpython/binary_io.h
#[no_mangle]
pub unsafe extern "C" fn binary_reader_replay(reader: *mut core::ffi::c_void, collector: *mut core::ffi::c_void, progress_callback: *mut core::ffi::c_void) -> usize {
    0
}

/// binary_reader_close - from cpython/binary_io.h
#[no_mangle]
pub unsafe extern "C" fn binary_reader_close(reader: *mut core::ffi::c_void) {

}

/// validate_read_size - from cpython/debug_offsets_validation.h
#[no_mangle]
pub unsafe extern "C" fn validate_read_size(section_name: *mut i8, size: u64, buffer_size: usize) -> i32 {
    0
}

/// read_ptr - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn read_ptr(unwinder: *mut core::ffi::c_void, address: usize, result: *mut usize) -> i32 {
    0
}

/// read_Py_ssize_t - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn read_Py_ssize_t(unwinder: *mut core::ffi::c_void, address: usize, result: *mut core::ffi::c_void) -> i32 {
    0
}

/// read_char - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn read_char(unwinder: *mut core::ffi::c_void, address: usize, result: *mut i8) -> i32 {
    0
}

/// read_py_ptr - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn read_py_ptr(unwinder: *mut core::ffi::c_void, address: usize, ptr_addr: *mut usize) -> i32 {
    0
}

/// read_py_long - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn read_py_long(unwinder: *mut core::ffi::c_void, address: usize) -> i64 {
    0
}

/// iterate_threads - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn iterate_threads(unwinder: *mut core::ffi::c_void, processor: usize, context: *mut core::ffi::c_void) -> i32 {
    0
}

/// get_thread_status - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn get_thread_status(unwinder: *mut core::ffi::c_void, tid: u64, pthread_id: u64) -> i32 {
    0
}

/// unwind_stack_for_thread - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn unwind_stack_for_thread(unwinder: *mut core::ffi::c_void, current_tstate: *mut usize, gil_holder_tstate: usize, gc_frame: usize, main_thread_tstate: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _Py_RemoteDebug_InitThreadsState - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn _Py_RemoteDebug_InitThreadsState(unwinder: *mut core::ffi::c_void, st: *mut core::ffi::c_void) {

}

/// _Py_RemoteDebug_StopAllThreads - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn _Py_RemoteDebug_StopAllThreads(unwinder: *mut core::ffi::c_void, st: *mut core::ffi::c_void) -> i32 {
    0
}

/// _Py_RemoteDebug_ResumeAllThreads - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn _Py_RemoteDebug_ResumeAllThreads(unwinder: *mut core::ffi::c_void, st: *mut core::ffi::c_void) {

}

/// read_async_debug - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn read_async_debug(unwinder: *mut core::ffi::c_void) -> i32 {
    0
}

/// find_running_task_in_thread - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn find_running_task_in_thread(unwinder: *mut core::ffi::c_void, thread_state_addr: usize, running_task_addr: *mut usize) -> i32 {
    0
}

/// process_thread_for_awaited_by - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn process_thread_for_awaited_by(unwinder: *mut core::ffi::c_void, thread_state_addr: usize, tid: u64, context: *mut core::ffi::c_void) -> i32 {
    0
}

/// process_thread_for_async_stack_trace - from cpython/_remote_debugging.h
#[no_mangle]
pub unsafe extern "C" fn process_thread_for_async_stack_trace(unwinder: *mut core::ffi::c_void, thread_state_addr: usize, tid: u64, context: *mut core::ffi::c_void) -> i32 {
    0
}

/// pysqlite_check_thread - from cpython/connection.h
#[no_mangle]
pub unsafe extern "C" fn pysqlite_check_thread(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyTestLimitedCAPI_Init_ThreadState - from cpython/parts.h
#[no_mangle]
pub unsafe extern "C" fn _PyTestLimitedCAPI_Init_ThreadState(module: *mut core::ffi::c_void) -> i32 {
    0
}

/// _multibytecodec_MultibyteStreamReader_read_impl - from cpython/multibytecodec.c.h
#[no_mangle]
pub unsafe extern "C" fn _multibytecodec_MultibyteStreamReader_read_impl(arg0: *mut core::ffi::c_void, sizeobj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _multibytecodec_MultibyteStreamReader_read - from cpython/multibytecodec.c.h
#[no_mangle]
pub unsafe extern "C" fn _multibytecodec_MultibyteStreamReader_read(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _multibytecodec_MultibyteStreamReader_readline_impl - from cpython/multibytecodec.c.h
#[no_mangle]
pub unsafe extern "C" fn _multibytecodec_MultibyteStreamReader_readline_impl(arg0: *mut core::ffi::c_void, sizeobj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _multibytecodec_MultibyteStreamReader_readline - from cpython/multibytecodec.c.h
#[no_mangle]
pub unsafe extern "C" fn _multibytecodec_MultibyteStreamReader_readline(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _multibytecodec_MultibyteStreamReader_readlines_impl - from cpython/multibytecodec.c.h
#[no_mangle]
pub unsafe extern "C" fn _multibytecodec_MultibyteStreamReader_readlines_impl(arg0: *mut core::ffi::c_void, sizehintobj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _multibytecodec_MultibyteStreamReader_readlines - from cpython/multibytecodec.c.h
#[no_mangle]
pub unsafe extern "C" fn _multibytecodec_MultibyteStreamReader_readlines(arg0: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _multibytecodec_MultibyteStreamReader_reset_impl - from cpython/multibytecodec.c.h
#[no_mangle]
pub unsafe extern "C" fn _multibytecodec_MultibyteStreamReader_reset_impl(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _multibytecodec_MultibyteStreamReader_reset - from cpython/multibytecodec.c.h
#[no_mangle]
pub unsafe extern "C" fn _multibytecodec_MultibyteStreamReader_reset(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _multibytecodec_MultibyteStreamWriter_write_impl - from cpython/multibytecodec.c.h
#[no_mangle]
pub unsafe extern "C" fn _multibytecodec_MultibyteStreamWriter_write_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, strobj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _multibytecodec_MultibyteStreamWriter_write - from cpython/multibytecodec.c.h
#[no_mangle]
pub unsafe extern "C" fn _multibytecodec_MultibyteStreamWriter_write(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _multibytecodec_MultibyteStreamWriter_writelines_impl - from cpython/multibytecodec.c.h
#[no_mangle]
pub unsafe extern "C" fn _multibytecodec_MultibyteStreamWriter_writelines_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, lines: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _multibytecodec_MultibyteStreamWriter_writelines - from cpython/multibytecodec.c.h
#[no_mangle]
pub unsafe extern "C" fn _multibytecodec_MultibyteStreamWriter_writelines(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void, args: *mut *mut core::ffi::c_void, nargs: usize, kwnames: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _multibytecodec_MultibyteStreamWriter_reset_impl - from cpython/multibytecodec.c.h
#[no_mangle]
pub unsafe extern "C" fn _multibytecodec_MultibyteStreamWriter_reset_impl(arg0: *mut core::ffi::c_void, cls: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyBytesWriter_Finish - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyBytesWriter_Finish(arg0: usize) -> usize {
    0
}

/// PyBytesWriter_FinishWithPointer - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyBytesWriter_FinishWithPointer(arg0: usize, arg1: usize) -> usize {
    0
}

/// get_thread_state - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn get_thread_state() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// deduce_all_threads - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn deduce_all_threads() -> i32 {
    0
}

/// faulthandler_thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn faulthandler_thread(unused: *mut core::ffi::c_void) {

}

/// faulthandler_fatal_error_thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn faulthandler_fatal_error_thread(plock: *mut core::ffi::c_void) -> usize {
    0
}

/// getpath_readlines - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn getpath_readlines(Py_UNUSEDself: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyMarshal_ReadObjectFromString - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyMarshal_ReadObjectFromString(arg0: usize) -> usize {
    0
}

/// pymain_import_readline - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn pymain_import_readline(config: *mut core::ffi::c_void) {

}

/// do_ReadFile - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn do_ReadFile(arg0: *mut core::ffi::c_void, handle: *mut core::ffi::c_void, bufstart: *mut i8, buflen: u32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyUnicodeWriter_Finish - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyUnicodeWriter_Finish(arg0: usize) -> usize {
    0
}

/// warn_about_fork_with_threads - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn warn_about_fork_with_threads(name: *mut i8, num_os_threads: usize) -> i32 {
    0
}

/// get_number_of_os_threads - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn get_number_of_os_threads() -> usize {
    0
}

/// PyBytesWriter_FinishWithSize - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyBytesWriter_FinishWithSize(arg0: usize, arg1: usize) -> usize {
    0
}

/// _Py_write - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _Py_write(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// readinst - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn readinst(buf: *mut i8, buf_size: i32, meth: *mut core::ffi::c_void) -> i32 {
    0
}

/// get_readline_state - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn get_readline_state(module: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// readline_clear - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn readline_clear(m: *mut core::ffi::c_void) -> i32 {
    0
}

/// readline_traverse - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn readline_traverse(m: *mut core::ffi::c_void, visit: usize, arg: *mut core::ffi::c_void) -> i32 {
    0
}

/// readline_sigwinch_handler - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn readline_sigwinch_handler(signum: i32) {

}

/// setup_readline - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn setup_readline(mod_state: *mut core::ffi::c_void) -> i32 {
    0
}

/// readline_until_enter_or_signal - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn readline_until_enter_or_signal(prompt: *mut i8, signal: *mut i32) -> *mut i8 {
    core::ptr::null_mut()
}

/// call_readline - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn call_readline(sys_stdin: *mut core::ffi::c_void, sys_stdout: *mut core::ffi::c_void, prompt: *mut i8) -> *mut i8 {
    core::ptr::null_mut()
}

/// PyInit_readline - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit_readline() -> usize {
    0
}

/// report_wakeup_write_error - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn report_wakeup_write_error(data: *mut core::ffi::c_void) -> i32 {
    0
}

/// _PyOS_IsMainThread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _PyOS_IsMainThread() -> i32 {
    0
}

/// time_pthread_getcpuclockid - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn time_pthread_getcpuclockid(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _PyTime_GetThreadTimeWithInfo - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _PyTime_GetThreadTimeWithInfo(tp: *mut core::ffi::c_void, info: *mut core::ffi::c_void) -> i32 {
    0
}

/// time_thread_time - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn time_thread_time(arg0: *mut core::ffi::c_void, unused: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// time_thread_time_ns - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn time_thread_time_ns(arg0: *mut core::ffi::c_void, unused: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// Reader_iternext_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn Reader_iternext_lock_held(op: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// Reader_iternext - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn Reader_iternext(op: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// Reader_traverse - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn Reader_traverse(op: *mut core::ffi::c_void, visit: usize, arg: *mut core::ffi::c_void) -> i32 {
    0
}

/// Reader_clear - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn Reader_clear(op: *mut core::ffi::c_void) -> i32 {
    0
}

/// csv_reader - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn csv_reader(module: *mut core::ffi::c_void, args: *mut core::ffi::c_void, keyword_args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// csv_writerow_lock_held - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn csv_writerow_lock_held(op: *mut core::ffi::c_void, seq: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// csv_writerow - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn csv_writerow(op: *mut core::ffi::c_void, seq: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// csv_writerows - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn csv_writerows(arg0: *mut core::ffi::c_void, seqseq: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// Writer_traverse - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn Writer_traverse(op: *mut core::ffi::c_void, visit: usize, arg: *mut core::ffi::c_void) -> i32 {
    0
}

/// Writer_clear - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn Writer_clear(op: *mut core::ffi::c_void) -> i32 {
    0
}

/// csv_writer - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn csv_writer(module: *mut core::ffi::c_void, args: *mut core::ffi::c_void, keyword_args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// encoder_write_string - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn encoder_write_string(s: *mut core::ffi::c_void, writer: *mut core::ffi::c_void, obj: *mut core::ffi::c_void) -> i32 {
    0
}

/// write_escaped_ascii - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn write_escaped_ascii(writer: *mut core::ffi::c_void, pystr: *mut core::ffi::c_void) -> i32 {
    0
}

/// PyUnicodeWriter_WriteChar - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyUnicodeWriter_WriteChar(arg0: usize, arg1: usize) -> usize {
    0
}

/// write_escaped_unicode - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn write_escaped_unicode(writer: *mut core::ffi::c_void, pystr: *mut core::ffi::c_void) -> i32 {
    0
}

/// write_newline_indent - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn write_newline_indent(writer: *mut core::ffi::c_void, indent_level: usize, indent_cache: *mut core::ffi::c_void) -> i32 {
    0
}

/// PyUnicodeWriter_WriteStr - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyUnicodeWriter_WriteStr(arg0: usize, arg1: usize) -> usize {
    0
}

/// PyUnicodeWriter_WriteASCII - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyUnicodeWriter_WriteASCII(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// PyUnicodeWriter_WriteRepr - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyUnicodeWriter_WriteRepr(arg0: usize, arg1: usize) -> usize {
    0
}

/// _write_size64 - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _write_size64(out: *mut i8, value: usize) {

}

/// _Pickler_Write - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _Pickler_Write(arg0: *mut core::ffi::c_void, s: *mut i8, data_len: usize) -> usize {
    0
}

/// bad_readline - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn bad_readline(st: *mut core::ffi::c_void) -> i32 {
    0
}

/// _Unpickler_ReadIntoFromFile - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _Unpickler_ReadIntoFromFile(state: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, buf: *mut i8, n: usize) -> usize {
    0
}

/// _Unpickler_ReadFromFile - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _Unpickler_ReadFromFile(state: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, n: usize) -> usize {
    0
}

/// _Unpickler_ReadImpl - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _Unpickler_ReadImpl(arg0: *mut core::ffi::c_void, st: *mut core::ffi::c_void, s: *mut *mut i8, n: usize) -> usize {
    0
}

/// _Unpickler_ReadInto - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _Unpickler_ReadInto(state: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, buf: *mut i8, n: usize) -> usize {
    0
}

/// _Unpickler_Readline - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _Unpickler_Readline(state: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void, result: *mut *mut i8) -> usize {
    0
}

/// _Pickler_write_bytes - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _Pickler_write_bytes(arg0: *mut core::ffi::c_void, header: *mut i8, header_size: usize, data: *mut i8, data_size: usize, payload: *mut core::ffi::c_void) -> i32 {
    0
}

/// write_unicode_binary - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn write_unicode_binary(arg0: *mut core::ffi::c_void, obj: *mut core::ffi::c_void) -> i32 {
    0
}

/// load_readonly_buffer - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn load_readonly_buffer(state: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> i32 {
    0
}

/// write - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn write() -> usize {
    0
}

/// write_eof - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn write_eof() -> usize {
    0
}

/// ndarray_get_readonly - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn ndarray_get_readonly(op: *mut core::ffi::c_void, closure: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// _make_call_from_thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _make_call_from_thread(callable: *mut core::ffi::c_void) {

}

/// test_thread_state - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn test_thread_state(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyThread_release_lock - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyThread_release_lock(arg0: usize) -> usize {
    0
}

/// spawn_pthread_waiter - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn spawn_pthread_waiter(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// end_spawned_pthread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn end_spawned_pthread(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pending_threadfunc - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn pending_threadfunc(arg0: *mut core::ffi::c_void, arg: *mut core::ffi::c_void, kwargs: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// test_pep3118_obsolete_write_locks - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn test_pep3118_obsolete_write_locks(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// crash_no_current_thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn crash_no_current_thread(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyThreadState_Get - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyThreadState_Get() -> usize {
    0
}

/// temporary_c_thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn temporary_c_thread(data: *mut core::ffi::c_void) {

}

/// call_in_temporary_c_thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn call_in_temporary_c_thread(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyThread_acquire_lock - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyThread_acquire_lock(arg0: usize, arg1: usize) -> usize {
    0
}

/// join_temporary_c_thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn join_temporary_c_thread(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pymarshal_write_long_to_file - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn pymarshal_write_long_to_file(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pymarshal_write_object_to_file - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn pymarshal_write_object_to_file(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pymarshal_read_short_from_file - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn pymarshal_read_short_from_file(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pymarshal_read_long_from_file - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn pymarshal_read_long_from_file(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pymarshal_read_last_object_from_file - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn pymarshal_read_last_object_from_file(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// pymarshal_read_object_from_file - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn pymarshal_read_object_from_file(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// test_pythread_tss_key_state - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn test_pythread_tss_key_state(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// finalize_thread_hang_cleanup_callback - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn finalize_thread_hang_cleanup_callback(Py_UNUSEDarg: *mut core::ffi::c_void) {

}

/// finalize_thread_hang - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn finalize_thread_hang(arg0: *mut core::ffi::c_void, callback: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// write_perf_map_entry - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn write_perf_map_entry(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// get_py_thread_id - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn get_py_thread_id(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// check_threadstate_set_stack_protection - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn check_threadstate_set_stack_protection(tstate: *mut core::ffi::c_void, start: *mut core::ffi::c_void, size: usize) {

}

/// test_threadstate_set_stack_protection - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn test_threadstate_set_stack_protection(arg0: *mut core::ffi::c_void, Py_UNUSEDargs: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// get_thread_state_by_cls - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn get_thread_state_by_cls(cls: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// get_thread_handle_state - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn get_thread_handle_state(handle: *mut core::ffi::c_void) -> i32 {
    0
}

/// set_thread_handle_state - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn set_thread_handle_state(handle: *mut core::ffi::c_void, state: usize) {

}

/// ThreadHandle_ident - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn ThreadHandle_ident(handle: *mut core::ffi::c_void) -> usize {
    0
}

/// ThreadHandle_get_os_handle - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn ThreadHandle_get_os_handle(handle: *mut core::ffi::c_void, os_handle: *mut core::ffi::c_void) -> i32 {
    0
}

/// ThreadHandle_new - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn ThreadHandle_new() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// ThreadHandle_incref - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn ThreadHandle_incref(arg0: *mut core::ffi::c_void) {

}

/// detach_thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn detach_thread(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// ThreadHandle_decref - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn ThreadHandle_decref(arg0: *mut core::ffi::c_void) {

}

/// thread_run - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_run(boot_raw: *mut core::ffi::c_void) {

}

/// ThreadHandle_start - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn ThreadHandle_start(arg0: *mut core::ffi::c_void, func: *mut core::ffi::c_void, args: *mut core::ffi::c_void, kwargs: *mut core::ffi::c_void, daemon: i32) -> i32 {
    0
}

/// join_thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn join_thread(arg: *mut core::ffi::c_void) -> i32 {
    0
}

/// ThreadHandle_join - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn ThreadHandle_join(arg0: *mut core::ffi::c_void, timeout_ns: usize) -> i32 {
    0
}

/// ThreadHandle_set_done - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn ThreadHandle_set_done(arg0: *mut core::ffi::c_void) -> i32 {
    0
}

/// PyThreadHandleObject_new - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyThreadHandleObject_new(arg0: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyThreadHandleObject_tp_new - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyThreadHandleObject_tp_new(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void, kwds: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyThreadHandleObject_repr - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyThreadHandleObject_repr(op: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyThreadHandleObject_get_ident - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyThreadHandleObject_get_ident(op: *mut core::ffi::c_void, Py_UNUSEDclosure: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyThreadHandleObject_join - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyThreadHandleObject_join(op: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyThreadHandleObject_is_done - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyThreadHandleObject_is_done(op: *mut core::ffi::c_void, Py_UNUSEDdummy: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// PyThreadHandleObject_set_done - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyThreadHandleObject_set_done(op: *mut core::ffi::c_void, Py_UNUSEDdummy: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// thread_daemon_threads_allowed - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_daemon_threads_allowed(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// do_start_new_thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn do_start_new_thread(state: *mut core::ffi::c_void, func: *mut core::ffi::c_void, args: *mut core::ffi::c_void, kwargs: *mut core::ffi::c_void, handle: *mut core::ffi::c_void, daemon: i32) -> i32 {
    0
}

/// thread_PyThread_start_new_thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_PyThread_start_new_thread(module: *mut core::ffi::c_void, fargs: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// start_new_thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn start_new_thread() -> usize {
    0
}

/// thread_PyThread_start_joinable_thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_PyThread_start_joinable_thread(module: *mut core::ffi::c_void, fargs: *mut core::ffi::c_void, fkwargs: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// thread_PyThread_exit_thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_PyThread_exit_thread(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// thread_PyThread_interrupt_main - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_PyThread_interrupt_main(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// thread_get_ident - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_get_ident(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// thread_get_native_id - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_get_native_id(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// thread__count - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread__count(arg0: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// thread_stack_size - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_stack_size(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// thread_excepthook_file - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_excepthook_file(file: *mut core::ffi::c_void, exc_type: *mut core::ffi::c_void, exc_value: *mut core::ffi::c_void, exc_traceback: *mut core::ffi::c_void, thread: *mut core::ffi::c_void) -> i32 {
    0
}

/// thread_excepthook - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_excepthook(module: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// thread__is_main_interpreter - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread__is_main_interpreter(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// thread_shutdown - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_shutdown(arg0: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// threads - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn threads(thread: usize) -> usize {
    0
}

/// thread__make_thread_handle - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread__make_thread_handle(module: *mut core::ffi::c_void, identobj: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// thread__get_main_thread_ident - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread__get_main_thread_ident(module: *mut core::ffi::c_void, Py_UNUSEDignored: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// thread_module_exec - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_module_exec(module: *mut core::ffi::c_void) -> i32 {
    0
}

/// thread_module_traverse - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_module_traverse(module: *mut core::ffi::c_void, visit: usize, arg: *mut core::ffi::c_void) -> i32 {
    0
}

/// thread_module_clear - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_module_clear(module: *mut core::ffi::c_void) -> i32 {
    0
}

/// PyInit__thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn PyInit__thread() -> usize {
    0
}

/// Tkapp_ThreadSend - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn Tkapp_ThreadSend(arg0: *mut core::ffi::c_void, ev: *mut core::ffi::c_void, cond: *mut core::ffi::c_void, mutex: *mut core::ffi::c_void) {

}

/// _batched_WaitForMultipleObjects_thread - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn _batched_WaitForMultipleObjects_thread(param: *mut core::ffi::c_void) -> u32 {
    0
}

/// SetMultithreadProtected - from dxvk/d3d10_multithread.h
#[no_mangle]
pub unsafe extern "C" fn SetMultithreadProtected(bMTProtect: i32) -> usize {
    0
}

/// GetMultithreadProtected - from dxvk/d3d10_multithread.h
#[no_mangle]
pub unsafe extern "C" fn GetMultithreadProtected() -> usize {
    0
}

/// EmitToCsThread - from dxvk/d3d11_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn EmitToCsThread(DispatchProc: usize) {

}

/// SynchronizeCsThread - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn SynchronizeCsThread(SequenceNumber: u64) {

}

/// ReadbackImageBuffer - from dxvk/d3d11_context_imm.h
#[no_mangle]
pub unsafe extern "C" fn ReadbackImageBuffer(pResource: *mut core::ffi::c_void, Subresource: u32) {

}

/// ValidateDepthWriteMask - from dxvk/d3d11_depth_stencil.h
#[no_mangle]
pub unsafe extern "C" fn ValidateDepthWriteMask(Mask: usize) -> usize {
    0
}

/// GetGPUThreadPriority - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn GetGPUThreadPriority(pPriority: *mut i32) -> usize {
    0
}

/// SetGPUThreadPriority - from dxvk/d3d11_device.h
#[no_mangle]
pub unsafe extern "C" fn SetGPUThreadPriority(Priority: i32) -> usize {
    0
}

/// CreateReadbackResource - from dxvk/d3d11_gdi.h
#[no_mangle]
pub unsafe extern "C" fn CreateReadbackResource() -> i32 {
    0
}

/// NeedsReadback - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn NeedsReadback() -> usize {
    0
}

/// SetNeedsReadback - from dxvk/d3d9_common_buffer.h
#[no_mangle]
pub unsafe extern "C" fn SetNeedsReadback(state: usize) {

}

/// MarkAllNeedReadback - from dxvk/d3d9_common_texture.h
#[no_mangle]
pub unsafe extern "C" fn MarkAllNeedReadback() {

}

/// UpdateAnyColorWrites - from dxvk/d3d9_device.h
#[no_mangle]
pub unsafe extern "C" fn UpdateAnyColorWrites() {

}

/// runEventThread - from dxvk/dxgi_adapter.h
#[no_mangle]
pub unsafe extern "C" fn runEventThread() {

}

/// CreateSoftwareAdapter - from dxvk/dxgi_factory.h
#[no_mangle]
pub unsafe extern "C" fn CreateSoftwareAdapter(Module: *mut core::ffi::c_void, ppAdapter: *mut *mut core::ffi::c_void) -> usize {
    0
}

/// read - from dxvk/dxso_code.h
#[no_mangle]
pub unsafe extern "C" fn read() -> u32 {
    0
}

/// readu32 - from dxvk/dxso_reader.h
#[no_mangle]
pub unsafe extern "C" fn readu32() -> usize {
    0
}

/// readf32 - from dxvk/dxso_reader.h
#[no_mangle]
pub unsafe extern "C" fn readf32() -> usize {
    0
}

/// readTag - from dxvk/dxso_reader.h
#[no_mangle]
pub unsafe extern "C" fn readTag() -> usize {
    0
}

/// readNum - from dxvk/dxso_reader.h
#[no_mangle]
pub unsafe extern "C" fn readNum() -> usize {
    0
}

/// cmdSetDepthWrite - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetDepthWrite(depthWriteEnable: u32) {

}

/// cmdSetStencilWriteMask - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdSetStencilWriteMask(faceMask: usize, writeMask: u32) {

}

/// cmdWriteTimestamp - from dxvk/dxvk_cmdlist.h
#[no_mangle]
pub unsafe extern "C" fn cmdWriteTimestamp(cmdBuffer: usize, pipelineStage: usize, queryPool: usize, query: u32) {

}

/// writeMask - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn writeMask() -> u8 {
    0
}

/// setWriteMask - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setWriteMask(mask: u8) {

}

/// depthWrite - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn depthWrite() -> usize {
    0
}

/// setDepthWrite - from dxvk/dxvk_constant_state.h
#[no_mangle]
pub unsafe extern "C" fn setDepthWrite(depthWrite: usize) {

}

/// writeTimestamp - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn writeTimestamp(query: usize) {

}

/// prepareShaderReadableImages - from dxvk/dxvk_context.h
#[no_mangle]
pub unsafe extern "C" fn prepareShaderReadableImages(renderPass: usize) {

}

/// threadFunc - from dxvk/dxvk_cs.h
#[no_mangle]
pub unsafe extern "C" fn threadFunc() {

}

/// getWriteBufferDescriptorFn - from dxvk/dxvk_descriptor_worker.h
#[no_mangle]
pub unsafe extern "C" fn getWriteBufferDescriptorFn() -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// writeBufferDescriptorsGeneric - from dxvk/dxvk_descriptor_worker.h
#[no_mangle]
pub unsafe extern "C" fn writeBufferDescriptorsGeneric(worker: *mut core::ffi::c_void, descriptors: *mut core::ffi::c_void, bufferCount: u32, bufferInfos: *mut core::ffi::c_void) {

}

/// writeBufferDescriptorsGetDescriptorExt - from dxvk/dxvk_descriptor_worker.h
#[no_mangle]
pub unsafe extern "C" fn writeBufferDescriptorsGetDescriptorExt(worker: *mut core::ffi::c_void, descriptors: *mut core::ffi::c_void, bufferCount: u32, bufferInfos: *mut core::ffi::c_void) {

}

/// writeBufferDescriptorsSteamDeck - from dxvk/dxvk_descriptor_worker.h
#[no_mangle]
pub unsafe extern "C" fn writeBufferDescriptorsSteamDeck(worker: *mut core::ffi::c_void, descriptors: *mut core::ffi::c_void, bufferCount: u32, bufferInfos: *mut core::ffi::c_void) {

}

/// trackColorRead - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn trackColorRead(index: u32) {

}

/// trackColorWrite - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn trackColorWrite(index: u32) {

}

/// trackDepthRead - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn trackDepthRead() {

}

/// trackDepthWrite - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn trackDepthWrite() {

}

/// trackStencilRead - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn trackStencilRead() {

}

/// trackStencilWrite - from dxvk/dxvk_framebuffer.h
#[no_mangle]
pub unsafe extern "C" fn trackStencilWrite() {

}

/// getDepthStencilReadOnlyAspects - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn getDepthStencilReadOnlyAspects() -> usize {
    0
}

/// colorWriteMask - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn colorWriteMask() -> usize {
    0
}

/// writesRenderTarget - from dxvk/dxvk_graphics_state.h
#[no_mangle]
pub unsafe extern "C" fn writesRenderTarget(target: u32) -> usize {
    0
}

/// waitVrKeyReady - from dxvk/dxvk_openvr.h
#[no_mangle]
pub unsafe extern "C" fn waitVrKeyReady() -> usize {
    0
}

/// getReadOnlyResourcesForStage - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getReadOnlyResourcesForStage(stage: usize) -> usize {
    0
}

/// getReadWriteResources - from dxvk/dxvk_pipelayout.h
#[no_mangle]
pub unsafe extern "C" fn getReadWriteResources() -> usize {
    0
}

/// stopWorkerThreads - from dxvk/dxvk_pipemanager.h
#[no_mangle]
pub unsafe extern "C" fn stopWorkerThreads() {

}

/// runFrameThread - from dxvk/dxvk_presenter.h
#[no_mangle]
pub unsafe extern "C" fn runFrameThread() {

}

/// openReadWriteLocked - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn openReadWriteLocked() -> usize {
    0
}

/// openWriteOnlyLocked - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn openWriteOnlyLocked() -> usize {
    0
}

/// writeShaderLutEntry - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn writeShaderLutEntry(shader: usize, entry: usize) -> usize {
    0
}

/// writeShaderToCache - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn writeShaderToCache(shader: usize) -> usize {
    0
}

/// readShaderLutEntry - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn readShaderLutEntry(key: usize, entry: usize, offset: usize) -> usize {
    0
}

/// runWriter - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn runWriter() {

}

/// writeShaderXfbInfo - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn writeShaderXfbInfo(stream: usize, xfb: usize) -> usize {
    0
}

/// writeShaderCreateInfo - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn writeShaderCreateInfo(stream: usize, createInfo: usize) -> usize {
    0
}

/// writeShaderLayout - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn writeShaderLayout(stream: usize, layout: usize) -> usize {
    0
}

/// writeShaderIo - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn writeShaderIo(stream: usize, io: usize) -> usize {
    0
}

/// writeShaderMetadata - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn writeShaderMetadata(stream: usize, metadata: usize) -> usize {
    0
}

/// writeHeader - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn writeHeader(stream: usize, header: usize) -> usize {
    0
}

/// readShaderIo - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn readShaderIo(stream: usize, offset: usize, io: usize) -> usize {
    0
}

/// readShaderXfbInfo - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn readShaderXfbInfo(stream: usize, offset: usize, xfb: usize) -> usize {
    0
}

/// readShaderLutKey - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn readShaderLutKey(stream: usize, offset: usize, key: usize) -> usize {
    0
}

/// readShaderMetadata - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn readShaderMetadata(stream: usize, offset: usize, metadata: usize) -> usize {
    0
}

/// readShaderLayout - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn readShaderLayout(stream: usize, offset: usize, layout: usize) -> usize {
    0
}

/// writeBytes - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn writeBytes(stream: usize, data: *mut i8, size: usize) -> usize {
    0
}

/// readBytes - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn readBytes(stream: usize, data: *mut i8, offset: usize, size: usize) -> usize {
    0
}

/// readString - from dxvk/dxvk_shader_cache.h
#[no_mangle]
pub unsafe extern "C" fn readString(stream: usize, offset: usize, string: usize) -> usize {
    0
}

/// opImageRead - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageRead(resultType: u32, image: u32, coordinates: u32, operands: usize) -> u32 {
    0
}

/// opImageWrite - from dxvk/spirv_module.h
#[no_mangle]
pub unsafe extern "C" fn opImageWrite(image: u32, coordinates: u32, texel: u32, operands: usize) {

}

/// thread - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn thread(proc: usize) -> usize {
    0
}

/// threadProc - from dxvk/thread.h
#[no_mangle]
pub unsafe extern "C" fn threadProc(arg: *mut core::ffi::c_void) -> u32 {
    0
}

/// setThreadName - from dxvk/util_env.h
#[no_mangle]
pub unsafe extern "C" fn setThreadName(name: usize) {

}

/// debug_printf - from vkd3d-proton/demo_win32.h
#[no_mangle]
pub unsafe extern "C" fn debug_printf(statsn: usize) -> usize {
    0
}

/// test_thread_main - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn test_thread_main(untyped_data: *mut core::ffi::c_void) -> u32 {
    0
}

/// create_thread - from vkd3d-proton/d3d12_crosstest.h
#[no_mangle]
pub unsafe extern "C" fn create_thread(main_pfn: usize, user_data: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// get_texture_readback_with_command_list - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_texture_readback_with_command_list(texture: *mut core::ffi::c_void, sub_resource: u32, rb: *mut core::ffi::c_void, queue: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void) {

}

/// get_readback_uint - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_readback_uint(rb: *mut core::ffi::c_void, x: u32, y: u32, z: u32) -> u32 {
    0
}

/// release_resource_readback - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn release_resource_readback(rb: *mut core::ffi::c_void) {

}

/// check_readback_data_uint_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_readback_data_uint_(line: u32, rb: *mut core::ffi::c_void, arg2: *mut core::ffi::c_void, expected: u32, max_diff: u32) {

}

/// init_readback - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn init_readback(rb: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, buffer_size: u64, width: u64, height: u64, depth: u32, row_pitch: u64) {

}

/// get_buffer_readback_with_command_list - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_buffer_readback_with_command_list(buffer: *mut core::ffi::c_void, format: usize, rb: *mut core::ffi::c_void, queue: *mut core::ffi::c_void, command_list: *mut core::ffi::c_void) {

}

/// get_readback_uint8 - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_readback_uint8(rb: *mut core::ffi::c_void, x: u32, y: u32) -> u8 {
    0
}

/// get_readback_uint16 - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_readback_uint16(rb: *mut core::ffi::c_void, x: u32, y: u32) -> u16 {
    0
}

/// get_readback_uint64 - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_readback_uint64(rb: *mut core::ffi::c_void, x: u32, y: u32) -> u64 {
    0
}

/// get_readback_float - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn get_readback_float(rb: *mut core::ffi::c_void, x: u32, y: u32) -> f32 {
    0.0
}

/// check_readback_data_float_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_readback_data_float_(line: u32, rb: *mut core::ffi::c_void, rect: *mut core::ffi::c_void, expected: f32, max_diff: u32) {

}

/// check_readback_data_uint8_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_readback_data_uint8_(line: u32, rb: *mut core::ffi::c_void, rect: *mut core::ffi::c_void, expected: u8, max_diff: u32) {

}

/// check_readback_data_uint16_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_readback_data_uint16_(line: u32, rb: *mut core::ffi::c_void, rect: *mut core::ffi::c_void, expected: u16, max_diff: u32) {

}

/// check_readback_data_uint64_ - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn check_readback_data_uint64_(line: u32, rb: *mut core::ffi::c_void, rect: *mut core::ffi::c_void, expected: u64, max_diff: u32) {

}

/// insert_debug_label_printf - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn insert_debug_label_printf(list: *mut core::ffi::c_void, fmt: *mut i8) {

}

/// begin_debug_region_printf - from vkd3d-proton/d3d12_test_utils.h
#[no_mangle]
pub unsafe extern "C" fn begin_debug_region_printf(list: *mut core::ffi::c_void, fmt: *mut i8) {

}

/// vkd3d_dbg_printf - from vkd3d-proton/vkd3d_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_dbg_printf(channel: usize, level: usize, function: *mut i8, fmt: *mut i8) {

}

/// vkd3d_file_map_read_only - from vkd3d-proton/vkd3d_file_utils.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_file_map_read_only(path: *mut i8, file: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_file_rename_overwrite - from vkd3d-proton/vkd3d_file_utils.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_file_rename_overwrite(from_path: *mut i8, to_path: *mut i8) -> usize {
    0
}

/// rw_spinlock_release_read - from vkd3d-proton/vkd3d_rw_spinlock.h
#[no_mangle]
pub unsafe extern "C" fn rw_spinlock_release_read(spinlock: *mut core::ffi::c_void) {

}

/// rw_spinlock_release_write - from vkd3d-proton/vkd3d_rw_spinlock.h
#[no_mangle]
pub unsafe extern "C" fn rw_spinlock_release_write(spinlock: *mut core::ffi::c_void) {

}

/// VKD3D_PRINTF_FUNC - from vkd3d-proton/vkd3d_test.h
#[no_mangle]
pub unsafe extern "C" fn VKD3D_PRINTF_FUNC(arg0: usize, arg1: usize) {

}

/// win32_thread_wrapper_routine - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn win32_thread_wrapper_routine(arg: *mut core::ffi::c_void) -> u32 {
    0
}

/// rwlock_lock_write - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn rwlock_lock_write(lock: *mut core::ffi::c_void) -> i32 {
    0
}

/// rwlock_lock_read - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn rwlock_lock_read(lock: *mut core::ffi::c_void) -> i32 {
    0
}

/// rwlock_unlock_write - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn rwlock_unlock_write(lock: *mut core::ffi::c_void) -> i32 {
    0
}

/// rwlock_unlock_read - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn rwlock_unlock_read(lock: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_cond_destroy - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn pthread_cond_destroy(cond: *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_cond_broadcast - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn pthread_cond_broadcast(cond: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_set_thread_name - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_set_thread_name(name: *mut i8) {

}

/// pthread_once_wrapper - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn pthread_once_wrapper(once: usize, parameter: *mut core::ffi::c_void, context: *mut *mut core::ffi::c_void) -> i32 {
    0
}

/// pthread_once - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn pthread_once(once: *mut core::ffi::c_void, func: *mut core::ffi::c_void) {

}

/// pthread_rwlock_init - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn pthread_rwlock_init(arg0: usize, arg1: usize) -> usize {
    0
}

/// pthread_rwlock_wrlock - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn pthread_rwlock_wrlock(arg0: usize) -> usize {
    0
}

/// pthread_rwlock_rdlock - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn pthread_rwlock_rdlock(arg0: usize) -> usize {
    0
}

/// pthread_rwlock_unlock - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn pthread_rwlock_unlock(arg0: usize) -> usize {
    0
}

/// pthread_rwlock_destroy - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn pthread_rwlock_destroy(arg0: usize) -> usize {
    0
}

/// vkd3d_get_current_thread_id - from vkd3d-proton/vkd3d_threads.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_get_current_thread_id() -> u32 {
    0
}

/// DEBUG_CHANNEL_WRITE_HEADER - from vkd3d-proton/debug_channel.h
#[no_mangle]
pub unsafe extern "C" fn DEBUG_CHANNEL_WRITE_HEADER(buf: usize, offset: usize, fmt: usize) {

}

/// d3d12_command_list_WriteBufferImmediate_profiled - from vkd3d-proton/command_list_profiled.h
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_WriteBufferImmediate_profiled(iface: *mut core::ffi::c_void, count: u32, parameters: *mut core::ffi::c_void, modes: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_descriptor_debug_write_descriptor - from vkd3d-proton/vkd3d_descriptor_debug.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_descriptor_debug_write_descriptor(heap: *mut core::ffi::c_void, heap_cookie: usize, offset: u32, type_flags: usize, cookie: usize) {

}

/// vkd3d_va_map_try_read_rtas - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_va_map_try_read_rtas(va_map: *mut core::ffi::c_void, device: *mut core::ffi::c_void, va: usize, acceleration_structure: *mut core::ffi::c_void, micromap: *mut core::ffi::c_void) {

}

/// vkd3d_memory_transfer_queue_write_subresource - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memory_transfer_queue_write_subresource(queue: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, subresource_idx: u32, offset: usize, extent: usize) -> i32 {
    0
}

/// vkd3d_breadcrumb_tracer_init - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_init(tracer: *mut core::ffi::c_void, device: *mut core::ffi::c_void) -> i32 {
    0
}

/// vkd3d_breadcrumb_tracer_init_barrier_hashes - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_init_barrier_hashes(tracer: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_tracer_cleanup - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_cleanup(tracer: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_tracer_cleanup_barrier_hashes - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_cleanup_barrier_hashes(tracer: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_tracer_release_command_lists - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_release_command_lists(tracer: *mut core::ffi::c_void, indices: *mut u32, indices_count: usize) {

}

/// vkd3d_breadcrumb_tracer_report_device_lost - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_report_device_lost(tracer: *mut core::ffi::c_void, device: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_tracer_begin_command_list - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_begin_command_list(list: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_tracer_add_command - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_add_command(list: *mut core::ffi::c_void, command: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_tracer_signal - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_signal(list: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_tracer_end_command_list - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_end_command_list(list: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_tracer_link_submission - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_link_submission(list: *mut core::ffi::c_void, prev: *mut core::ffi::c_void, next: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_tracer_update_barrier_hashes - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_update_barrier_hashes(tracer: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_tracer_shader_hash_forces_barrier - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_shader_hash_forces_barrier(device: *mut core::ffi::c_void, hash: usize) -> u32 {
    0
}

/// vkd3d_breadcrumb_tracer_dump_command_list - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_dump_command_list(tracer: *mut core::ffi::c_void, index: u32) {

}

/// vkd3d_breadcrumb_tracer_register_placed_resource - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_register_placed_resource(heap: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, heap_offset: u64, required_size: u64) {

}

/// vkd3d_breadcrumb_tracer_unregister_placed_resource - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_unregister_placed_resource(heap: *mut core::ffi::c_void, resource: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_image - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_image(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_resource - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_resource(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_subresource - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_subresource(list: *mut core::ffi::c_void, subresource: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_buffer_image_copy - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_buffer_image_copy(list: *mut core::ffi::c_void, buffer_image: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_image_copy - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_image_copy(list: *mut core::ffi::c_void, image: *mut core::ffi::c_void) {

}

/// vkd3d_breadcrumb_buffer_copy - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_buffer_copy(list: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void) {

}

/// vkd3d_address_binding_tracker_mark_user_thread - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_address_binding_tracker_mark_user_thread() {

}

/// is_write_resource_state - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn is_write_resource_state(state: usize) -> usize {
    0
}

/// vkd3d_opacity_micromap_write_postbuild_info - from vkd3d-proton/vkd3d_private.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_opacity_micromap_write_postbuild_info(list: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, desc_offset: u64, vk_opacity_micromap: usize) {

}

/// test_dispatch_zero_thread_groups - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_dispatch_zero_thread_groups() {

}

/// draw_thread_main - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn draw_thread_main(thread_data: *mut core::ffi::c_void) {

}

/// test_multithread_command_queue_exec - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_multithread_command_queue_exec() {

}

/// check_readback_data_uint - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn check_readback_data_uint(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// test_write_buffer_immediate - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_write_buffer_immediate() {

}

/// test_depth_read_only_view - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_depth_read_only_view() {

}

/// test_dynamic_depth_stencil_write - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_dynamic_depth_stencil_write() {

}

/// private_data_thread_main - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn private_data_thread_main(untyped_data: *mut core::ffi::c_void) {

}

/// private_data_interface_thread_main - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn private_data_interface_thread_main(untyped_data: *mut core::ffi::c_void) {

}

/// ReadFloat - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ReadFloat(SV_DispatchThreadID: usize) {

}

/// ReadUint - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ReadUint(SV_DispatchThreadID: usize) {

}

/// ReadUint16 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ReadUint16(SV_DispatchThreadID: usize) {

}

/// WriteMain - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn WriteMain(SV_DispatchThreadID: usize) {

}

/// ReadbackMain - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn ReadbackMain(SV_DispatchThreadID: usize) {

}

/// test_primitive_id_read_tess_geom - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_primitive_id_read_tess_geom() {

}

/// test_query_timestamp_write_after_read - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_query_timestamp_write_after_read() {

}

/// test_read_subresource_rt - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_read_subresource_rt() {

}

/// test_read_write_subresource_2d - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_read_write_subresource_2d() {

}

/// test_read_write_subresource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_read_write_subresource() {

}

/// test_undefined_structured_raw_read_typed - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_undefined_structured_raw_read_typed(use_dxil: usize) {

}

/// test_undefined_structured_raw_read_typed_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_undefined_structured_raw_read_typed_dxbc() {

}

/// test_undefined_structured_raw_read_typed_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_undefined_structured_raw_read_typed_dxil() {

}

/// test_undefined_typed_read_structured_raw - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_undefined_typed_read_structured_raw(use_dxil: usize) {

}

/// test_undefined_typed_read_structured_raw_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_undefined_typed_read_structured_raw_dxbc() {

}

/// test_undefined_typed_read_structured_raw_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_undefined_typed_read_structured_raw_dxil() {

}

/// test_undefined_read_typed_buffer_as_untyped_simple - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_undefined_read_typed_buffer_as_untyped_simple(use_dxil: usize) {

}

/// test_undefined_read_typed_buffer_as_untyped_simple_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_undefined_read_typed_buffer_as_untyped_simple_dxbc() {

}

/// test_undefined_read_typed_buffer_as_untyped_simple_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_undefined_read_typed_buffer_as_untyped_simple_dxil() {

}

/// validate_readback - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn validate_readback(rb: *mut core::ffi::c_void, test: *mut core::ffi::c_void, slice: u32) {

}

/// read_uav_counter - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn read_uav_counter(context: *mut core::ffi::c_void, counter_buffer: *mut core::ffi::c_void, offset: usize) -> u32 {
    0
}

/// test_memory_model_uav_coherent_thread_group - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_memory_model_uav_coherent_thread_group(use_dxil: usize) {

}

/// test_memory_model_uav_coherence_thread_group_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_memory_model_uav_coherence_thread_group_dxbc() {

}

/// test_memory_model_uav_coherence_thread_group_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_memory_model_uav_coherence_thread_group_dxil() {

}

/// test_multithread_fence_wait - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_multithread_fence_wait() {

}

/// test_tessellation_read_tesslevel - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_tessellation_read_tesslevel() {

}

/// get_readback_data - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn get_readback_data(arg0: usize, arg1: usize, arg2: usize, arg3: usize, vec4: usize) -> usize {
    0
}

/// test_vrs_depth_write_dxbc - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_vrs_depth_write_dxbc() {

}

/// test_vrs_depth_write_dxil - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_vrs_depth_write_dxil() {

}

/// test_write_watch - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_write_watch() {

}

/// thread_input_expected - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn thread_input_expected(desc: *mut core::ffi::c_void, value_index: u32) -> u32 {
    0
}

/// test_workgraph_thread_input - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn test_workgraph_thread_input() {

}

/// d3d12_dred_settings_SetAutoBreadcrumbsEnablement - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_dred_settings_SetAutoBreadcrumbsEnablement(iface: *mut core::ffi::c_void, enablement: usize) -> usize {
    0
}

/// vkd3d_acceleration_structure_write_postbuild_info - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_acceleration_structure_write_postbuild_info(list: *mut core::ffi::c_void, desc: *mut core::ffi::c_void, desc_offset: u64, vk_acceleration_structure: usize) {

}

/// vkd3d_breadcrumb_tracer_report_command_list - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_report_command_list(context: *mut core::ffi::c_void, begin_marker: u32, end_marker: u32) {

}

/// vkd3d_breadcrumb_tracer_report_command_list_linked - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_report_command_list_linked(tracer: *mut core::ffi::c_void, begin_context_index: u32, end_context_index: u32) {

}

/// vkd3d_breadcrumb_tracer_report_command_list_amd - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_report_command_list_amd(tracer: *mut core::ffi::c_void, context_index: u32) {

}

/// vkd3d_breadcrumb_tracer_report_queue_nv - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_breadcrumb_tracer_report_queue_nv(tracer: *mut core::ffi::c_void, device: *mut core::ffi::c_void, vk_queue: usize) {

}

/// d3d12_bundle_exec_write_buffer_immediate - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_exec_write_buffer_immediate(list: *mut core::ffi::c_void, args_v: *mut core::ffi::c_void) {

}

/// d3d12_bundle_WriteBufferImmediate - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_bundle_WriteBufferImmediate(iface: *mut core::ffi::c_void, count: u32, parameters: *mut core::ffi::c_void, modes: *mut core::ffi::c_void) -> usize {
    0
}

/// d3d12_pipeline_library_read_blob_stream_format - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_read_blob_stream_format(pipeline_library: *mut core::ffi::c_void, device: *mut core::ffi::c_void, blob: *mut core::ffi::c_void, blob_length: usize) -> i32 {
    0
}

/// d3d12_pipeline_library_read_blob - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_read_blob(pipeline_library: *mut core::ffi::c_void, device: *mut core::ffi::c_void, blob: *mut core::ffi::c_void, blob_length: usize) -> i32 {
    0
}

/// d3d12_pipeline_library_read_blob_toc_format - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_pipeline_library_read_blob_toc_format(arg0: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    0
}

/// d3d12_command_list_notify_dsv_writes - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_notify_dsv_writes(list: *mut core::ffi::c_void, resource: *mut core::ffi::c_void, view: *mut core::ffi::c_void, plane_write_mask: u32) -> u32 {
    0
}

/// d3d12_command_list_sync_tiler_renderpass_writes - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_sync_tiler_renderpass_writes(list: *mut core::ffi::c_void, rendering_info: *mut core::ffi::c_void) {

}

/// d3d12_command_list_mark_copy_buffer_write - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_mark_copy_buffer_write(list: *mut core::ffi::c_void, vk_buffer: usize, offset: u64, size: u64, sparse: usize) {

}

/// d3d12_command_list_debug_mark_label_printf - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_debug_mark_label_printf(list: *mut core::ffi::c_void, vk_cmd: usize, r: f32, g: f32, b: f32, a: f32, fmt: *mut i8) {

}

/// d3d12_command_list_read_query_range - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_read_query_range(list: *mut core::ffi::c_void, vk_pool: usize, index: u32, count: u32) {

}

/// vk_write_descriptor_set_from_root_descriptor - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_write_descriptor_set_from_root_descriptor(list: *mut core::ffi::c_void, vk_descriptor_write: *mut core::ffi::c_void, root_parameter: *mut core::ffi::c_void, descriptor: *mut core::ffi::c_void) {

}

/// vk_write_descriptor_set_from_scratch_push_ubo - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vk_write_descriptor_set_from_scratch_push_ubo(vk_descriptor_write: *mut core::ffi::c_void, vk_buffer_info: *mut core::ffi::c_void, alloc: *mut core::ffi::c_void, size: u64, vk_binding: u32) {

}

/// d3d12_command_list_register_pending_transfer_image_write - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_register_pending_transfer_image_write(list: *mut core::ffi::c_void, vk_image: usize, subresource_index: u32, vk_stages: usize) {

}

/// d3d12_command_list_check_pending_transfer_image_write - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_check_pending_transfer_image_write(list: *mut core::ffi::c_void, vk_image: usize, subresource_index: u32) -> usize {
    0
}

/// d3d12_command_list_WriteBufferImmediate - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_command_list_WriteBufferImmediate(iface: *mut core::ffi::c_void, count: u32, parameters: *mut core::ffi::c_void, modes: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_dbg_sprintf - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_dbg_sprintf(u: usize) -> usize {
    0
}

/// d3d12_resource_WriteToSubresource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_WriteToSubresource(iface: *mut core::ffi::c_void, dst_sub_resource: u32, dst_box: *mut core::ffi::c_void, src_data: *mut core::ffi::c_void, src_row_pitch: u32, src_slice_pitch: u32) -> usize {
    0
}

/// d3d12_resource_ReadFromSubresource - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_resource_ReadFromSubresource(iface: *mut core::ffi::c_void, dst_data: *mut core::ffi::c_void, dst_row_pitch: u32, dst_slice_pitch: u32, src_sub_resource: u32, src_box: *mut core::ffi::c_void) -> usize {
    0
}

/// vkd3d_init_write_descriptor_set - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_init_write_descriptor_set(vk_write: *mut core::ffi::c_void, split: *mut core::ffi::c_void, binding: usize, vk_descriptor_type: usize, info: *mut core::ffi::c_void) {

}

/// d3d12_descriptor_heap_write_null_descriptor_template_embedded_partial - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_write_null_descriptor_template_embedded_partial(device: *mut core::ffi::c_void, desc_va: usize, vk_descriptor_type: usize, payload_offset: usize, size: usize) {

}

/// d3d12_descriptor_heap_write_null_descriptor_template_embedded - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_write_null_descriptor_template_embedded(device: *mut core::ffi::c_void, desc_va: usize, vk_descriptor_type: usize) {

}

/// d3d12_descriptor_heap_write_null_descriptor_template - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn d3d12_descriptor_heap_write_null_descriptor_template(desc_va: usize, vk_mutable_descriptor_type: usize) {

}

/// dxgi_vk_swap_chain_cleanup_waiter_thread - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_cleanup_waiter_thread(chain: *mut core::ffi::c_void) {

}

/// dxgi_vk_swap_chain_init_waiter_thread - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn dxgi_vk_swap_chain_init_waiter_thread(chain: *mut core::ffi::c_void) -> i32 {
    0
}

/// fopen - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn fopen(arg0: usize, arg1: usize) -> usize {
    0
}

/// vkd3d_utf16_read - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn vkd3d_utf16_read(src: *mut *mut u16) -> u32 {
    0
}

/// read_dword - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn read_dword(ptr: *mut *mut i8, d: *mut u32) {

}

/// read_uint32_ - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn read_uint32_(ptr: *mut *mut i8, u: *mut core::ffi::c_void) {

}

/// read_float - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn read_float(ptr: *mut *mut i8, f: *mut f32) {

}

/// write_dwords - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn write_dwords(context: *mut core::ffi::c_void, count: u32, d: u32) -> usize {
    0
}

/// write_dword - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn write_dword(context: *mut core::ffi::c_void, d: u32) -> usize {
    0
}

/// write_float - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn write_float(context: *mut core::ffi::c_void, f: f32) -> usize {
    0
}

/// shader_write_root_signature_header - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_write_root_signature_header(context: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_write_descriptor_ranges - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_write_descriptor_ranges(context: *mut core::ffi::c_void, table: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_write_descriptor_ranges1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_write_descriptor_ranges1(context: *mut core::ffi::c_void, table: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_write_descriptor_table - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_write_descriptor_table(context: *mut core::ffi::c_void, table: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_write_descriptor_table1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_write_descriptor_table1(context: *mut core::ffi::c_void, table: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_write_root_constants - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_write_root_constants(context: *mut core::ffi::c_void, constants: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_write_root_descriptor - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_write_root_descriptor(context: *mut core::ffi::c_void, descriptor: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_write_root_descriptor1 - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_write_root_descriptor1(context: *mut core::ffi::c_void, descriptor: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_write_root_parameters - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_write_root_parameters(context: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_write_static_samplers - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_write_static_samplers(context: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// shader_write_root_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn shader_write_root_signature(context: *mut core::ffi::c_void, desc: *mut core::ffi::c_void) -> i32 {
    0
}

/// read_shader - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn read_shader(shader: *mut core::ffi::c_void, filename: *mut i8) -> usize {
    0
}

/// write_shader - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn write_shader(shader: *mut core::ffi::c_void, filename: *mut i8) -> usize {
    0
}

/// read_root_signature - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn read_root_signature(shader: *mut core::ffi::c_void, filename: *mut i8) -> usize {
    0
}

/// TrackWrite - from DirectX-Headers/d3d12sdklayers.h
#[no_mangle]
pub unsafe extern "C" fn TrackWrite(Subresource: u32, pWrittenRange: *mut core::ffi::c_void) -> usize {
    0
}

/// WriteBufferImmediateSupportFlags - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn WriteBufferImmediateSupportFlags() -> usize {
    0
}

/// WriteableMSAATexturesSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn WriteableMSAATexturesSupported() -> i32 {
    0
}

/// ManualWriteTrackingResourceSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ManualWriteTrackingResourceSupported() -> i32 {
    0
}

/// ComputeOnlyWriteWatchSupported - from DirectX-Headers/d3dx12_check_feature_support.h
#[no_mangle]
pub unsafe extern "C" fn ComputeOnlyWriteWatchSupported() -> i32 {
    0
}

/// SetDepthWriteMask - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetDepthWriteMask(depthWriteMask: usize) {

}

/// SetStencilReadMask - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetStencilReadMask(stencilReadMask: usize) {

}

/// SetStencilWriteMask - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn SetStencilWriteMask(stencilWriteMask: usize) {

}

/// CreateThreadLaunchNodeOverrides - from DirectX-Headers/d3dx12_state_object.h
#[no_mangle]
pub unsafe extern "C" fn CreateThreadLaunchNodeOverrides(nullptr: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

