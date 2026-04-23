//! OpenGL 4.4 Functions — Buffer storage, multi-bind, clear texture
//! Based on Khronos canonical specifications
use super::types::*;

// Buffer storage
pub type PFNGLBUFFERSTORAGEPROC = Option<unsafe extern "system" fn(target: GLenum, size: GLsizeiptr, data: *const GLvoid, flags: GLbitfield)>;

// Clear texture
pub type PFNGLCLEARTEXIMAGEPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, data: *const GLvoid)>;
pub type PFNGLCLEARTEXSUBIMAGEPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, data: *const GLvoid)>;

// Multi-bind
pub type PFNGLBINDBUFFERSBASEPROC = Option<unsafe extern "system" fn(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint)>;
pub type PFNGLBINDBUFFERSRANGEPROC = Option<unsafe extern "system" fn(target: GLenum, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr)>;
pub type PFNGLBINDTEXTURESPROC = Option<unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint)>;
pub type PFNGLBINDSAMPLERSPROC = Option<unsafe extern "system" fn(first: GLuint, count: GLsizei, samplers: *const GLuint)>;
pub type PFNGLBINDIMAGETEXTURESPROC = Option<unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint)>;
pub type PFNGLBINDVERTEXBUFFERSPROC = Option<unsafe extern "system" fn(first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei)>;

/// OpenGL 4.4 function table
#[derive(Default)]
pub struct GL44 {
    pub glBufferStorage: PFNGLBUFFERSTORAGEPROC,
    pub glClearTexImage: PFNGLCLEARTEXIMAGEPROC,
    pub glClearTexSubImage: PFNGLCLEARTEXSUBIMAGEPROC,
    pub glBindBuffersBase: PFNGLBINDBUFFERSBASEPROC,
    pub glBindBuffersRange: PFNGLBINDBUFFERSRANGEPROC,
    pub glBindTextures: PFNGLBINDTEXTURESPROC,
    pub glBindSamplers: PFNGLBINDSAMPLERSPROC,
    pub glBindImageTextures: PFNGLBINDIMAGETEXTURESPROC,
    pub glBindVertexBuffers: PFNGLBINDVERTEXBUFFERSPROC,
}
