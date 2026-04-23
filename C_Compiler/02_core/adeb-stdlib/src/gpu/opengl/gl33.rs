//! OpenGL 3.3 Functions — Samplers, timer queries, vertex attrib divisor
//! Based on Khronos canonical specifications
use super::types::*;

// Vertex attrib divisor
pub type PFNGLVERTEXATTRIBDIVISORPROC = Option<unsafe extern "system" fn(index: GLuint, divisor: GLuint)>;

// Bind frag data location indexed
pub type PFNGLBINDFRAGDATALOCATIONINDEXEDPROC = Option<unsafe extern "system" fn(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar)>;
pub type PFNGLGETFRAGDATAINDEXPROC = Option<unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint>;

// Sampler objects
pub type PFNGLGENSAMPLERSPROC = Option<unsafe extern "system" fn(count: GLsizei, samplers: *mut GLuint)>;
pub type PFNGLDELETESAMPLERSPROC = Option<unsafe extern "system" fn(count: GLsizei, samplers: *const GLuint)>;
pub type PFNGLISSAMPLERPROC = Option<unsafe extern "system" fn(sampler: GLuint) -> GLboolean>;
pub type PFNGLBINDSAMPLERPROC = Option<unsafe extern "system" fn(unit: GLuint, sampler: GLuint)>;
pub type PFNGLSAMPLERPARAMETERIPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: GLint)>;
pub type PFNGLSAMPLERPARAMETERIVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint)>;
pub type PFNGLSAMPLERPARAMETERFPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: GLfloat)>;
pub type PFNGLSAMPLERPARAMETERFVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLfloat)>;
pub type PFNGLSAMPLERPARAMETERIIVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLint)>;
pub type PFNGLSAMPLERPARAMETERIUIVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, param: *const GLuint)>;
pub type PFNGLGETSAMPLERPARAMETERIVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETSAMPLERPARAMETERIIVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETSAMPLERPARAMETERFVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLfloat)>;
pub type PFNGLGETSAMPLERPARAMETERIUIVPROC = Option<unsafe extern "system" fn(sampler: GLuint, pname: GLenum, params: *mut GLuint)>;

// Timer queries
pub type PFNGLQUERYCOUNTERPROC = Option<unsafe extern "system" fn(id: GLuint, target: GLenum)>;
pub type PFNGLGETQUERYOBJECTI64VPROC = Option<unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLint64)>;
pub type PFNGLGETQUERYOBJECTUI64VPROC = Option<unsafe extern "system" fn(id: GLuint, pname: GLenum, params: *mut GLuint64)>;

// Vertex type 2_10_10_10_rev
pub type PFNGLVERTEXATTRIBP1UIPROC = Option<unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint)>;
pub type PFNGLVERTEXATTRIBP1UIVPROC = Option<unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint)>;
pub type PFNGLVERTEXATTRIBP2UIPROC = Option<unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint)>;
pub type PFNGLVERTEXATTRIBP2UIVPROC = Option<unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint)>;
pub type PFNGLVERTEXATTRIBP3UIPROC = Option<unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint)>;
pub type PFNGLVERTEXATTRIBP3UIVPROC = Option<unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint)>;
pub type PFNGLVERTEXATTRIBP4UIPROC = Option<unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: GLuint)>;
pub type PFNGLVERTEXATTRIBP4UIVPROC = Option<unsafe extern "system" fn(index: GLuint, type_: GLenum, normalized: GLboolean, value: *const GLuint)>;

/// OpenGL 3.3 function table
#[derive(Default)]
pub struct GL33 {
    pub glVertexAttribDivisor: PFNGLVERTEXATTRIBDIVISORPROC,
    pub glBindFragDataLocationIndexed: PFNGLBINDFRAGDATALOCATIONINDEXEDPROC,
    pub glGetFragDataIndex: PFNGLGETFRAGDATAINDEXPROC,
    pub glGenSamplers: PFNGLGENSAMPLERSPROC,
    pub glDeleteSamplers: PFNGLDELETESAMPLERSPROC,
    pub glIsSampler: PFNGLISSAMPLERPROC,
    pub glBindSampler: PFNGLBINDSAMPLERPROC,
    pub glSamplerParameteri: PFNGLSAMPLERPARAMETERIPROC,
    pub glSamplerParameteriv: PFNGLSAMPLERPARAMETERIVPROC,
    pub glSamplerParameterf: PFNGLSAMPLERPARAMETERFPROC,
    pub glSamplerParameterfv: PFNGLSAMPLERPARAMETERFVPROC,
    pub glSamplerParameterIiv: PFNGLSAMPLERPARAMETERIIVPROC,
    pub glSamplerParameterIuiv: PFNGLSAMPLERPARAMETERIUIVPROC,
    pub glGetSamplerParameteriv: PFNGLGETSAMPLERPARAMETERIVPROC,
    pub glGetSamplerParameterIiv: PFNGLGETSAMPLERPARAMETERIIVPROC,
    pub glGetSamplerParameterfv: PFNGLGETSAMPLERPARAMETERFVPROC,
    pub glGetSamplerParameterIuiv: PFNGLGETSAMPLERPARAMETERIUIVPROC,
    pub glQueryCounter: PFNGLQUERYCOUNTERPROC,
    pub glGetQueryObjecti64v: PFNGLGETQUERYOBJECTI64VPROC,
    pub glGetQueryObjectui64v: PFNGLGETQUERYOBJECTUI64VPROC,
    pub glVertexAttribP1ui: PFNGLVERTEXATTRIBP1UIPROC,
    pub glVertexAttribP1uiv: PFNGLVERTEXATTRIBP1UIVPROC,
    pub glVertexAttribP2ui: PFNGLVERTEXATTRIBP2UIPROC,
    pub glVertexAttribP2uiv: PFNGLVERTEXATTRIBP2UIVPROC,
    pub glVertexAttribP3ui: PFNGLVERTEXATTRIBP3UIPROC,
    pub glVertexAttribP3uiv: PFNGLVERTEXATTRIBP3UIVPROC,
    pub glVertexAttribP4ui: PFNGLVERTEXATTRIBP4UIPROC,
    pub glVertexAttribP4uiv: PFNGLVERTEXATTRIBP4UIVPROC,
}
