//! OpenGL 3.1 Functions — UBOs, instancing, TBOs, copy buffer, primitive restart
//! Based on Khronos canonical specifications
use super::types::*;

// Draw instanced
pub type PFNGLDRAWARRAYSINSTANCEDPROC = Option<unsafe extern "system" fn(mode: GLenum, first: GLint, count: GLsizei, instancecount: GLsizei)>;
pub type PFNGLDRAWELEMENTSINSTANCEDPROC = Option<unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid, instancecount: GLsizei)>;

// Texture buffer object
pub type PFNGLTEXBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, internalformat: GLenum, buffer: GLuint)>;

// Primitive restart
pub type PFNGLPRIMITIVERESTARTINDEXPROC = Option<unsafe extern "system" fn(index: GLuint)>;

// Copy buffer
pub type PFNGLCOPYBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(readTarget: GLenum, writeTarget: GLenum, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr)>;

// Uniform buffer objects
pub type PFNGLGETUNIFORMINDICESPROC = Option<unsafe extern "system" fn(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint)>;
pub type PFNGLGETACTIVEUNIFORMSIVPROC = Option<unsafe extern "system" fn(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETACTIVEUNIFORMNAMEPROC = Option<unsafe extern "system" fn(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar)>;
pub type PFNGLGETUNIFORMBLOCKINDEXPROC = Option<unsafe extern "system" fn(program: GLuint, uniformBlockName: *const GLchar) -> GLuint>;
pub type PFNGLGETACTIVEUNIFORMBLOCKIVPROC = Option<unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC = Option<unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar)>;
pub type PFNGLUNIFORMBLOCKBINDINGPROC = Option<unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint)>;

/// OpenGL 3.1 function table
#[derive(Default)]
pub struct GL31 {
    pub glDrawArraysInstanced: PFNGLDRAWARRAYSINSTANCEDPROC,
    pub glDrawElementsInstanced: PFNGLDRAWELEMENTSINSTANCEDPROC,
    pub glTexBuffer: PFNGLTEXBUFFERPROC,
    pub glPrimitiveRestartIndex: PFNGLPRIMITIVERESTARTINDEXPROC,
    pub glCopyBufferSubData: PFNGLCOPYBUFFERSUBDATAPROC,
    pub glGetUniformIndices: PFNGLGETUNIFORMINDICESPROC,
    pub glGetActiveUniformsiv: PFNGLGETACTIVEUNIFORMSIVPROC,
    pub glGetActiveUniformName: PFNGLGETACTIVEUNIFORMNAMEPROC,
    pub glGetUniformBlockIndex: PFNGLGETUNIFORMBLOCKINDEXPROC,
    pub glGetActiveUniformBlockiv: PFNGLGETACTIVEUNIFORMBLOCKIVPROC,
    pub glGetActiveUniformBlockName: PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC,
    pub glUniformBlockBinding: PFNGLUNIFORMBLOCKBINDINGPROC,
}
