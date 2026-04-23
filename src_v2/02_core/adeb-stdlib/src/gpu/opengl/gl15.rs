//! OpenGL 1.5 Functions - Buffer objects, occlusion queries
use super::types::*;

pub type PFNGLGENQUERIESPROC = Option<unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint)>;
pub type PFNGLDELETEQUERIESPROC = Option<unsafe extern "system" fn(n: GLsizei, ids: *const GLuint)>;
pub type PFNGLISQUERYPROC = Option<unsafe extern "system" fn(id: GLuint) -> GLboolean>;
pub type PFNGLBEGINQUERYPROC = Option<unsafe extern "system" fn(target: GLenum, id: GLuint)>;
pub type PFNGLENDQUERYPROC = Option<unsafe extern "system" fn(target: GLenum)>;
pub type PFNGLGETQUERYIVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETQUERYOBJECTIVPROC = Option<unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETQUERYOBJECTUIVPROC = Option<unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint)>;
pub type PFNGLBINDBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, buffer: GLuint)>;
pub type PFNGLDELETEBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint)>;
pub type PFNGLGENBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint)>;
pub type PFNGLISBUFFERPROC = Option<unsafe extern "system" fn(buffer: GLuint) -> GLboolean>;
pub type PFNGLBUFFERDATAPROC = Option<unsafe extern "system" fn(target: GLenum, size: GLsizeiptr, data: *const GLvoid, usage: GLenum)>;
pub type PFNGLBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const GLvoid)>;
pub type PFNGLGETBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *mut GLvoid)>;
pub type PFNGLMAPBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, access: GLenum) -> *mut GLvoid>;
pub type PFNGLUNMAPBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum) -> GLboolean>;
pub type PFNGLGETBUFFERPARAMETERIVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETBUFFERPOINTERVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut *mut GLvoid)>;

#[derive(Default)]
pub struct GL15 {
    pub glGenQueries: PFNGLGENQUERIESPROC,
    pub glDeleteQueries: PFNGLDELETEQUERIESPROC,
    pub glIsQuery: PFNGLISQUERYPROC,
    pub glBeginQuery: PFNGLBEGINQUERYPROC,
    pub glEndQuery: PFNGLENDQUERYPROC,
    pub glGetQueryiv: PFNGLGETQUERYIVPROC,
    pub glGetQueryObjectiv: PFNGLGETQUERYOBJECTIVPROC,
    pub glGetQueryObjectuiv: PFNGLGETQUERYOBJECTUIVPROC,
    pub glBindBuffer: PFNGLBINDBUFFERPROC,
    pub glDeleteBuffers: PFNGLDELETEBUFFERSPROC,
    pub glGenBuffers: PFNGLGENBUFFERSPROC,
    pub glIsBuffer: PFNGLISBUFFERPROC,
    pub glBufferData: PFNGLBUFFERDATAPROC,
    pub glBufferSubData: PFNGLBUFFERSUBDATAPROC,
    pub glGetBufferSubData: PFNGLGETBUFFERSUBDATAPROC,
    pub glMapBuffer: PFNGLMAPBUFFERPROC,
    pub glUnmapBuffer: PFNGLUNMAPBUFFERPROC,
    pub glGetBufferParameteriv: PFNGLGETBUFFERPARAMETERIVPROC,
    pub glGetBufferPointerv: PFNGLGETBUFFERPOINTERVPROC,
}
