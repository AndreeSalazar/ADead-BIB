//! ADead Runtime - STRING Module
//!
//! Funciones generadas automáticamente desde knowledge.json
//! Categoría: string

#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

/// u_austrcpy - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn u_austrcpy(s1: *mut i8, ucs2: *mut core::ffi::c_void) -> *mut i8 {
    core::ptr::null_mut()
}

/// u_memcpy - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn u_memcpy(dest: *mut core::ffi::c_void, src: *mut core::ffi::c_void, count: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// u_memset - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn u_memset(dest: *mut core::ffi::c_void, c: usize, count: i32) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// u_strcmp - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn u_strcmp(s1: *mut core::ffi::c_void, s2: *mut core::ffi::c_void) -> i32 {
    0
}

/// u_strcmpCodePointOrder - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn u_strcmpCodePointOrder(s1: *mut core::ffi::c_void, s2: *mut core::ffi::c_void) -> i32 {
    0
}

/// u_strcpy - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn u_strcpy(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// u_strlen - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn u_strlen(s: *mut core::ffi::c_void) -> i32 {
    0
}

/// u_uastrcpy - from wine/icu.h
#[no_mangle]
pub unsafe extern "C" fn u_uastrcpy(ucs1: *mut core::ffi::c_void, s2: *mut i8) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// VarBstrCmp - from wine/oleauto.h
#[no_mangle]
pub unsafe extern "C" fn VarBstrCmp(arg0: usize, arg1: usize, arg2: usize, arg3: u32) -> usize {
    0
}

/// StrCmpW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn StrCmpW(arg0: *const u16, arg1: *const u16) -> usize {
    0
}

/// StrCmpIW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn StrCmpIW(arg0: *const u16, arg1: *const u16) -> usize {
    0
}

/// StrCpyW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn StrCpyW(arg0: *mut u16, arg1: *const u16) -> usize {
    0
}

/// StrCpyNW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn StrCpyNW(arg0: *mut u16, arg1: *const u16, arg2: i32) -> usize {
    0
}

/// StrCmpLogicalW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn StrCmpLogicalW(arg0: *const u16, arg1: *const u16) -> usize {
    0
}

/// StrCmpNA - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn StrCmpNA(arg0: *const i8, arg1: *const i8, arg2: i32) -> usize {
    0
}

/// StrCmpNW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn StrCmpNW(arg0: *const u16, arg1: *const u16, arg2: i32) -> usize {
    0
}

/// StrCmpNIA - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn StrCmpNIA(arg0: *const i8, arg1: *const i8, arg2: i32) -> usize {
    0
}

/// StrCmpNIW - from wine/shlwapi.h
#[no_mangle]
pub unsafe extern "C" fn StrCmpNIW(arg0: *const u16, arg1: *const u16, arg2: i32) -> usize {
    0
}

/// strlen - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn strlen(arg0: usize) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/// strcpy - from reactos/unknown
#[no_mangle]
pub unsafe extern "C" fn strcpy(arg0: usize, arg1: usize) -> usize {
    0
}

/// wld_strcmp - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn wld_strcmp(str1: *mut i8, str2: *mut i8) -> i32 {
    0
}

/// memcpy - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn memcpy(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

/// tolower - from glibc/ctype.h
#[no_mangle]
pub unsafe extern "C" fn tolower(__c: i32) -> i32 {
    0
}

/// strcmp - from wine/unknown
#[no_mangle]
pub unsafe extern "C" fn strcmp(arg0: *mut core::ffi::c_void, arg1: *mut core::ffi::c_void) -> usize {
    0
}

/// lstrcmpi - from reactos/kbswitch.h
#[no_mangle]
pub unsafe extern "C" fn lstrcmpi(arg0: usize, arg1: usize) -> usize {
    0
}

/// toupper - from glibc/ctype.h
#[no_mangle]
pub unsafe extern "C" fn toupper(__c: i32) -> i32 {
    0
}

/// isctype - from glibc/ctype.h
#[no_mangle]
pub unsafe extern "C" fn isctype(__c: i32, __mask: i32) -> i32 {
    0
}

/// isascii - from glibc/ctype.h
#[no_mangle]
pub unsafe extern "C" fn isascii(__c: i32) -> i32 {
    0
}

/// toascii - from glibc/ctype.h
#[no_mangle]
pub unsafe extern "C" fn toascii(__c: i32) -> i32 {
    0
}

/// name - from glibc/ctype.h
#[no_mangle]
pub unsafe extern "C" fn name(arg0: i32, arg1: usize) -> i32 {
    0
}

/// __exctype_l - from glibc/ctype.h
#[no_mangle]
pub unsafe extern "C" fn __exctype_l(arg0: usize) -> usize {
    0
}

/// __tolower_l - from glibc/ctype.h
#[no_mangle]
pub unsafe extern "C" fn __tolower_l(__c: i32, __l: usize) -> i32 {
    0
}

/// tolower_l - from glibc/ctype.h
#[no_mangle]
pub unsafe extern "C" fn tolower_l(__c: i32, __l: usize) -> i32 {
    0
}

/// __toupper_l - from glibc/ctype.h
#[no_mangle]
pub unsafe extern "C" fn __toupper_l(__c: i32, __l: usize) -> i32 {
    0
}

/// toupper_l - from glibc/ctype.h
#[no_mangle]
pub unsafe extern "C" fn toupper_l(__c: i32, __l: usize) -> i32 {
    0
}

/// __ctype_init - from glibc/ctype.h
#[no_mangle]
pub unsafe extern "C" fn __ctype_init() {

}

/// __strnlen - from glibc/string.h
#[no_mangle]
pub unsafe extern "C" fn __strnlen(__string: *mut i8, __maxlen: usize) -> usize {
    0
}

/// __strverscmp - from glibc/string.h
#[no_mangle]
pub unsafe extern "C" fn __strverscmp(__s1: *mut i8, __s2: *mut i8) -> i32 {
    0
}

/// __strncasecmp - from glibc/string.h
#[no_mangle]
pub unsafe extern "C" fn __strncasecmp(__s1: *mut i8, __s2: *mut i8, __n: usize) -> i32 {
    0
}

/// __strcasecmp - from glibc/string.h
#[no_mangle]
pub unsafe extern "C" fn __strcasecmp(__s1: *mut i8, __s2: *mut i8) -> i32 {
    0
}

/// __bzero - from glibc/string.h
#[no_mangle]
pub unsafe extern "C" fn __bzero(__s: *mut core::ffi::c_void, __n: usize) {

}

/// __ffs - from glibc/string.h
#[no_mangle]
pub unsafe extern "C" fn __ffs(__i: i32) -> i32 {
    0
}

/// __explicit_bzero_chk_internal - from glibc/string.h
#[no_mangle]
pub unsafe extern "C" fn __explicit_bzero_chk_internal(arg0: *mut core::ffi::c_void, arg1: usize, arg2: usize) {

}

/// __explicit_bzero_chk - from glibc/string.h
#[no_mangle]
pub unsafe extern "C" fn __explicit_bzero_chk(arg0: *mut core::ffi::c_void, arg1: usize, arg2: usize) {

}

/// memset_zero - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn memset_zero(s: *mut core::ffi::c_void, len: usize) {

}

/// init_memset_distribution - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn init_memset_distribution() {

}

/// MEMSET - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn MEMSET(arg0: usize, arg1: usize, nc: usize) -> usize {
    0
}

/// generic_strcmp - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn generic_strcmp(s1: *mut i8, s2: *mut i8) -> i32 {
    0
}

/// generic_strcpy - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn generic_strcpy(dst: *mut i8, src: *mut i8) -> *mut i8 {
    core::ptr::null_mut()
}

/// MEMCPY - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn MEMCPY(arg0: usize, arg1: usize, src: usize) -> usize {
    0
}

/// memchr_strlen - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn memchr_strlen(arg0: *mut i8) -> usize {
    0
}

/// init_strlen_distribution - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn init_strlen_distribution() {

}

/// generic_strlen - from glibc/unknown
#[no_mangle]
pub unsafe extern "C" fn generic_strlen(arg0: *mut i8) -> usize {
    0
}

/// _mi_strlen - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_strlen(s: *mut i8) -> usize {
    0
}

/// _mi_memcpy - from cpython/internal.h
#[no_mangle]
pub unsafe extern "C" fn _mi_memcpy(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, n: usize) {

}

/// safe_memcpy - from cpython/unknown
#[no_mangle]
pub unsafe extern "C" fn safe_memcpy(dest: *mut core::ffi::c_void, src: *mut core::ffi::c_void, count: usize) -> i32 {
    0
}

/// strLen - from dxvk/spirv_code_buffer.h
#[no_mangle]
pub unsafe extern "C" fn strLen(str: *mut i8) -> u32 {
    0
}

/// vkd3d_memcpy_aligned_non_temporal - from vkd3d-proton/copy_utils.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memcpy_aligned_non_temporal(dst_: *mut core::ffi::c_void, src_: *mut core::ffi::c_void, size: usize) {

}

/// vkd3d_memcpy_aligned_cached - from vkd3d-proton/copy_utils.h
#[no_mangle]
pub unsafe extern "C" fn vkd3d_memcpy_aligned_cached(dst_: *mut core::ffi::c_void, src_: *mut core::ffi::c_void, size: usize) {

}

/// memset - from vkd3d-proton/unknown
#[no_mangle]
pub unsafe extern "C" fn memset(arg0: usize, arg1: usize, arg2: usize) -> usize {
    0
}

