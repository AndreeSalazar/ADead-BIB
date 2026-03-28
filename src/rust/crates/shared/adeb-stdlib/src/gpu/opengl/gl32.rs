//! OpenGL 3.2 Functions — Geometry shaders, sync objects, multisampled textures
//! Based on Khronos canonical specifications
use super::types::*;

// Draw elements base vertex
pub type PFNGLDRAWELEMENTSBASEVERTEXPROC = Option<unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid, basevertex: GLint)>;
pub type PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC = Option<unsafe extern "system" fn(mode: GLenum, start: GLuint, end: GLuint, count: GLsizei, type_: GLenum, indices: *const GLvoid, basevertex: GLint)>;
pub type PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC = Option<unsafe extern "system" fn(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid, instancecount: GLsizei, basevertex: GLint)>;
pub type PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC = Option<unsafe extern "system" fn(mode: GLenum, count: *const GLsizei, type_: GLenum, indices: *const *const GLvoid, drawcount: GLsizei, basevertex: *const GLint)>;

// Provoking vertex
pub type PFNGLPROVOKINGVERTEXPROC = Option<unsafe extern "system" fn(mode: GLenum)>;

// Sync objects
pub type PFNGLFENCESYNCPROC = Option<unsafe extern "system" fn(condition: GLenum, flags: GLbitfield) -> GLsync>;
pub type PFNGLISSYNCPROC = Option<unsafe extern "system" fn(sync: GLsync) -> GLboolean>;
pub type PFNGLDELETESYNCPROC = Option<unsafe extern "system" fn(sync: GLsync)>;
pub type PFNGLCLIENTWAITSYNCPROC = Option<unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum>;
pub type PFNGLWAITSYNCPROC = Option<unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64)>;
pub type PFNGLGETINTEGER64VPROC = Option<unsafe extern "system" fn(pname: GLenum, data: *mut GLint64)>;
pub type PFNGLGETSYNCIVPROC = Option<unsafe extern "system" fn(sync: GLsync, pname: GLenum, count: GLsizei, length: *mut GLsizei, values: *mut GLint)>;
pub type PFNGLGETINTEGER64I_VPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint64)>;
pub type PFNGLGETBUFFERPARAMETERI64VPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint64)>;

// Framebuffer texture (geometry shader)
pub type PFNGLFRAMEBUFFERTEXTUREPROC = Option<unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint)>;

// Multisampled textures
pub type PFNGLTEXIMAGE2DMULTISAMPLEPROC = Option<unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean)>;
pub type PFNGLTEXIMAGE3DMULTISAMPLEPROC = Option<unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean)>;
pub type PFNGLGETMULTISAMPLEFVPROC = Option<unsafe extern "system" fn(pname: GLenum, index: GLuint, val: *mut GLfloat)>;
pub type PFNGLSAMPLEMASKIPROC = Option<unsafe extern "system" fn(maskNumber: GLuint, mask: GLbitfield)>;

/// OpenGL 3.2 function table
#[derive(Default)]
pub struct GL32 {
    pub glDrawElementsBaseVertex: PFNGLDRAWELEMENTSBASEVERTEXPROC,
    pub glDrawRangeElementsBaseVertex: PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC,
    pub glDrawElementsInstancedBaseVertex: PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC,
    pub glMultiDrawElementsBaseVertex: PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC,
    pub glProvokingVertex: PFNGLPROVOKINGVERTEXPROC,
    pub glFenceSync: PFNGLFENCESYNCPROC,
    pub glIsSync: PFNGLISSYNCPROC,
    pub glDeleteSync: PFNGLDELETESYNCPROC,
    pub glClientWaitSync: PFNGLCLIENTWAITSYNCPROC,
    pub glWaitSync: PFNGLWAITSYNCPROC,
    pub glGetInteger64v: PFNGLGETINTEGER64VPROC,
    pub glGetSynciv: PFNGLGETSYNCIVPROC,
    pub glGetInteger64i_v: PFNGLGETINTEGER64I_VPROC,
    pub glGetBufferParameteri64v: PFNGLGETBUFFERPARAMETERI64VPROC,
    pub glFramebufferTexture: PFNGLFRAMEBUFFERTEXTUREPROC,
    pub glTexImage2DMultisample: PFNGLTEXIMAGE2DMULTISAMPLEPROC,
    pub glTexImage3DMultisample: PFNGLTEXIMAGE3DMULTISAMPLEPROC,
    pub glGetMultisamplefv: PFNGLGETMULTISAMPLEFVPROC,
    pub glSampleMaski: PFNGLSAMPLEMASKIPROC,
}
