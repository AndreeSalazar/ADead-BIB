//! OpenGL 4.6 Functions (FINAL — July 2017) — SPIR-V, polygon offset clamp
//! Based on Khronos canonical specifications
use super::types::*;

// SPIR-V
pub type PFNGLSPECIALIZESHADERPROC = Option<unsafe extern "system" fn(shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint)>;

// Multi-draw indirect count
pub type PFNGLMULTIDRAWARRAYSINDIRECTCOUNTPROC = Option<unsafe extern "system" fn(mode: GLenum, indirect: *const GLvoid, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei)>;
pub type PFNGLMULTIDRAWELEMENTSINDIRECTCOUNTPROC = Option<unsafe extern "system" fn(mode: GLenum, type_: GLenum, indirect: *const GLvoid, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei)>;

// Polygon offset clamp
pub type PFNGLPOLYGONOFFSETCLAMPPROC = Option<unsafe extern "system" fn(factor: GLfloat, units: GLfloat, clamp: GLfloat)>;

/// OpenGL 4.6 function table (FINAL version)
#[derive(Default)]
pub struct GL46 {
    pub glSpecializeShader: PFNGLSPECIALIZESHADERPROC,
    pub glMultiDrawArraysIndirectCount: PFNGLMULTIDRAWARRAYSINDIRECTCOUNTPROC,
    pub glMultiDrawElementsIndirectCount: PFNGLMULTIDRAWELEMENTSINDIRECTCOUNTPROC,
    pub glPolygonOffsetClamp: PFNGLPOLYGONOFFSETCLAMPPROC,
}
