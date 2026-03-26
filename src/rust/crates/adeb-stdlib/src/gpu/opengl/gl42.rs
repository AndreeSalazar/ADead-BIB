//! OpenGL 4.2 Functions — Atomic counters, image load/store, texture storage
//! Based on Khronos canonical specifications
use super::types::*;

// Draw base instance
pub type PFNGLDRAWARRAYSINSTANCEDBASEINSTANCEPROC = Option<unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint)>;
pub type PFNGLDRAWELEMENTSINSTANCEDBASEINSTANCEPROC = Option<unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid, instancecount: GLsizei, baseinstance: GLuint)>;
pub type PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXBASEINSTANCEPROC = Option<unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint)>;

// Internal format query
pub type PFNGLGETINTERNALFORMATIVPROC = Option<unsafe extern "system" fn(target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint)>;

// Atomic counter
pub type PFNGLGETACTIVEATOMICCOUNTERBUFFERIVPROC = Option<unsafe extern "system" fn(program: GLuint, bufferIndex: GLuint, pname: GLenum, params: *mut GLint)>;

// Image load/store
pub type PFNGLBINDIMAGETEXTUREPROC = Option<unsafe extern "system" fn(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: GLenum, format: GLenum)>;
pub type PFNGLMEMORYBARRIERPROC = Option<unsafe extern "system" fn(barriers: GLbitfield)>;

// Texture storage
pub type PFNGLTEXSTORAGE1DPROC = Option<unsafe extern "system" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei)>;
pub type PFNGLTEXSTORAGE2DPROC = Option<unsafe extern "system" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei)>;
pub type PFNGLTEXSTORAGE3DPROC = Option<unsafe extern "system" fn(target: GLenum, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei)>;

/// OpenGL 4.2 function table
#[derive(Default)]
pub struct GL42 {
    pub glDrawArraysInstancedBaseInstance: PFNGLDRAWARRAYSINSTANCEDBASEINSTANCEPROC,
    pub glDrawElementsInstancedBaseInstance: PFNGLDRAWELEMENTSINSTANCEDBASEINSTANCEPROC,
    pub glDrawElementsInstancedBaseVertexBaseInstance: PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXBASEINSTANCEPROC,
    pub glGetInternalformativ: PFNGLGETINTERNALFORMATIVPROC,
    pub glGetActiveAtomicCounterBufferiv: PFNGLGETACTIVEATOMICCOUNTERBUFFERIVPROC,
    pub glBindImageTexture: PFNGLBINDIMAGETEXTUREPROC,
    pub glMemoryBarrier: PFNGLMEMORYBARRIERPROC,
    pub glTexStorage1D: PFNGLTEXSTORAGE1DPROC,
    pub glTexStorage2D: PFNGLTEXSTORAGE2DPROC,
    pub glTexStorage3D: PFNGLTEXSTORAGE3DPROC,
}
