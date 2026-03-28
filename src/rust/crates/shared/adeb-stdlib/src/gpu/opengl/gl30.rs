//! OpenGL 3.0 Functions — FBOs, VAOs, transform feedback, conditional render
//! Based on Khronos canonical specifications
use super::types::*;

// Color clamping
pub type PFNGLCOLORMASKIPROC = Option<unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean)>;
pub type PFNGLGETBOOLEANI_VPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLboolean)>;
pub type PFNGLGETINTEGERI_VPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, data: *mut GLint)>;
pub type PFNGLENABLEIPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint)>;
pub type PFNGLDISABLEIPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint)>;
pub type PFNGLISENABLEDIPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint) -> GLboolean>;

// Transform feedback
pub type PFNGLBEGINTRANSFORMFEEDBACKPROC = Option<unsafe extern "system" fn(primitiveMode: GLenum)>;
pub type PFNGLENDTRANSFORMFEEDBACKPROC = Option<unsafe extern "system" fn()>;
pub type PFNGLBINDBUFFERRANGEPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr)>;
pub type PFNGLBINDBUFFERBASEPROC = Option<unsafe extern "system" fn(target: GLenum, index: GLuint, buffer: GLuint)>;
pub type PFNGLTRANSFORMFEEDBACKVARYINGSPROC = Option<unsafe extern "system" fn(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: GLenum)>;
pub type PFNGLGETTRANSFORMFEEDBACKVARYINGPROC = Option<unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut GLenum, name: *mut GLchar)>;

// Clamp color
pub type PFNGLCLAMPCOLORPROC = Option<unsafe extern "system" fn(target: GLenum, clamp: GLenum)>;

// Conditional render
pub type PFNGLBEGINCONDITIONALRENDERPROC = Option<unsafe extern "system" fn(id: GLuint, mode: GLenum)>;
pub type PFNGLENDCONDITIONALRENDERPROC = Option<unsafe extern "system" fn()>;

// Vertex attrib integer
pub type PFNGLVERTEXATTRIBIPOINTERPROC = Option<unsafe extern "system" fn(index: GLuint, size: GLint, type_: GLenum, stride: GLsizei, pointer: *const GLvoid)>;
pub type PFNGLGETVERTEXATTRIBIIVPROC = Option<unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETVERTEXATTRIBIUIVPROC = Option<unsafe extern "system" fn(index: GLuint, pname: GLenum, params: *mut GLuint)>;
pub type PFNGLVERTEXATTRIBI1IPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLint)>;
pub type PFNGLVERTEXATTRIBI2IPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint)>;
pub type PFNGLVERTEXATTRIBI3IPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint)>;
pub type PFNGLVERTEXATTRIBI4IPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint)>;
pub type PFNGLVERTEXATTRIBI1UIPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLuint)>;
pub type PFNGLVERTEXATTRIBI2UIPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint)>;
pub type PFNGLVERTEXATTRIBI3UIPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint)>;
pub type PFNGLVERTEXATTRIBI4UIPROC = Option<unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint)>;
pub type PFNGLVERTEXATTRIBI1IVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLint)>;
pub type PFNGLVERTEXATTRIBI2IVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLint)>;
pub type PFNGLVERTEXATTRIBI3IVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLint)>;
pub type PFNGLVERTEXATTRIBI4IVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLint)>;
pub type PFNGLVERTEXATTRIBI1UIVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLuint)>;
pub type PFNGLVERTEXATTRIBI2UIVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLuint)>;
pub type PFNGLVERTEXATTRIBI3UIVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLuint)>;
pub type PFNGLVERTEXATTRIBI4UIVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLuint)>;
pub type PFNGLVERTEXATTRIBI4BVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLbyte)>;
pub type PFNGLVERTEXATTRIBI4SVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLshort)>;
pub type PFNGLVERTEXATTRIBI4UBVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLubyte)>;
pub type PFNGLVERTEXATTRIBI4USVPROC = Option<unsafe extern "system" fn(index: GLuint, v: *const GLushort)>;

// Uniform unsigned int
pub type PFNGLGETUNIFORMUIVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint)>;
pub type PFNGLUNIFORM1UIPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLuint)>;
pub type PFNGLUNIFORM2UIPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint)>;
pub type PFNGLUNIFORM3UIPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint)>;
pub type PFNGLUNIFORM4UIPROC = Option<unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint)>;
pub type PFNGLUNIFORM1UIVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint)>;
pub type PFNGLUNIFORM2UIVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint)>;
pub type PFNGLUNIFORM3UIVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint)>;
pub type PFNGLUNIFORM4UIVPROC = Option<unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint)>;

// Bind frag data
pub type PFNGLBINDFRAGDATALOCATIONPROC = Option<unsafe extern "system" fn(program: GLuint, color: GLuint, name: *const GLchar)>;
pub type PFNGLGETFRAGDATALOCATIONPROC = Option<unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint>;

// Clear buffer
pub type PFNGLCLEARBUFFERIVPROC = Option<unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLint)>;
pub type PFNGLCLEARBUFFERUIVPROC = Option<unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLuint)>;
pub type PFNGLCLEARBUFFERFVPROC = Option<unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat)>;
pub type PFNGLCLEARBUFFERFIPROC = Option<unsafe extern "system" fn(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint)>;

// GetStringi
pub type PFNGLGETSTRINGIPROC = Option<unsafe extern "system" fn(name: GLenum, index: GLuint) -> *const GLubyte>;

// Framebuffer objects
pub type PFNGLISRENDERBUFFERPROC = Option<unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean>;
pub type PFNGLBINDRENDERBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, renderbuffer: GLuint)>;
pub type PFNGLDELETERENDERBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint)>;
pub type PFNGLGENRENDERBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint)>;
pub type PFNGLRENDERBUFFERSTORAGEPROC = Option<unsafe extern "system" fn(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei)>;
pub type PFNGLGETRENDERBUFFERPARAMETERIVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint)>;
pub type PFNGLISFRAMEBUFFERPROC = Option<unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean>;
pub type PFNGLBINDFRAMEBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, framebuffer: GLuint)>;
pub type PFNGLDELETEFRAMEBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint)>;
pub type PFNGLGENFRAMEBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint)>;
pub type PFNGLCHECKFRAMEBUFFERSTATUSPROC = Option<unsafe extern "system" fn(target: GLenum) -> GLenum>;
pub type PFNGLFRAMEBUFFERTEXTURE1DPROC = Option<unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint)>;
pub type PFNGLFRAMEBUFFERTEXTURE2DPROC = Option<unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint)>;
pub type PFNGLFRAMEBUFFERTEXTURE3DPROC = Option<unsafe extern "system" fn(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint, zoffset: GLint)>;
pub type PFNGLFRAMEBUFFERRENDERBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint)>;
pub type PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC = Option<unsafe extern "system" fn(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGENERATEMIPMAPPROC = Option<unsafe extern "system" fn(target: GLenum)>;
pub type PFNGLBLITFRAMEBUFFERPROC = Option<unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum)>;
pub type PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC = Option<unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei)>;
pub type PFNGLFRAMEBUFFERTEXTURELAYERPROC = Option<unsafe extern "system" fn(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint)>;

// VAOs
pub type PFNGLBINDVERTEXARRAYPROC = Option<unsafe extern "system" fn(array: GLuint)>;
pub type PFNGLDELETEVERTEXARRAYSPROC = Option<unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint)>;
pub type PFNGLGENVERTEXARRAYSPROC = Option<unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint)>;
pub type PFNGLISVERTEXARRAYPROC = Option<unsafe extern "system" fn(array: GLuint) -> GLboolean>;

// Map buffer range
pub type PFNGLMAPBUFFERRANGEPROC = Option<unsafe extern "system" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut GLvoid>;
pub type PFNGLFLUSHMAPPEDBUFFERRANGEPROC = Option<unsafe extern "system" fn(target: GLenum, offset: GLintptr, length: GLsizeiptr)>;

/// OpenGL 3.0 function table
#[derive(Default)]
pub struct GL30 {
    pub glColorMaski: PFNGLCOLORMASKIPROC,
    pub glGetBooleani_v: PFNGLGETBOOLEANI_VPROC,
    pub glGetIntegeri_v: PFNGLGETINTEGERI_VPROC,
    pub glEnablei: PFNGLENABLEIPROC,
    pub glDisablei: PFNGLDISABLEIPROC,
    pub glIsEnabledi: PFNGLISENABLEDIPROC,
    pub glBeginTransformFeedback: PFNGLBEGINTRANSFORMFEEDBACKPROC,
    pub glEndTransformFeedback: PFNGLENDTRANSFORMFEEDBACKPROC,
    pub glBindBufferRange: PFNGLBINDBUFFERRANGEPROC,
    pub glBindBufferBase: PFNGLBINDBUFFERBASEPROC,
    pub glTransformFeedbackVaryings: PFNGLTRANSFORMFEEDBACKVARYINGSPROC,
    pub glGetTransformFeedbackVarying: PFNGLGETTRANSFORMFEEDBACKVARYINGPROC,
    pub glClampColor: PFNGLCLAMPCOLORPROC,
    pub glBeginConditionalRender: PFNGLBEGINCONDITIONALRENDERPROC,
    pub glEndConditionalRender: PFNGLENDCONDITIONALRENDERPROC,
    pub glVertexAttribIPointer: PFNGLVERTEXATTRIBIPOINTERPROC,
    pub glGetVertexAttribIiv: PFNGLGETVERTEXATTRIBIIVPROC,
    pub glGetVertexAttribIuiv: PFNGLGETVERTEXATTRIBIUIVPROC,
    pub glVertexAttribI1i: PFNGLVERTEXATTRIBI1IPROC,
    pub glVertexAttribI2i: PFNGLVERTEXATTRIBI2IPROC,
    pub glVertexAttribI3i: PFNGLVERTEXATTRIBI3IPROC,
    pub glVertexAttribI4i: PFNGLVERTEXATTRIBI4IPROC,
    pub glVertexAttribI1ui: PFNGLVERTEXATTRIBI1UIPROC,
    pub glVertexAttribI2ui: PFNGLVERTEXATTRIBI2UIPROC,
    pub glVertexAttribI3ui: PFNGLVERTEXATTRIBI3UIPROC,
    pub glVertexAttribI4ui: PFNGLVERTEXATTRIBI4UIPROC,
    pub glVertexAttribI1iv: PFNGLVERTEXATTRIBI1IVPROC,
    pub glVertexAttribI2iv: PFNGLVERTEXATTRIBI2IVPROC,
    pub glVertexAttribI3iv: PFNGLVERTEXATTRIBI3IVPROC,
    pub glVertexAttribI4iv: PFNGLVERTEXATTRIBI4IVPROC,
    pub glVertexAttribI1uiv: PFNGLVERTEXATTRIBI1UIVPROC,
    pub glVertexAttribI2uiv: PFNGLVERTEXATTRIBI2UIVPROC,
    pub glVertexAttribI3uiv: PFNGLVERTEXATTRIBI3UIVPROC,
    pub glVertexAttribI4uiv: PFNGLVERTEXATTRIBI4UIVPROC,
    pub glVertexAttribI4bv: PFNGLVERTEXATTRIBI4BVPROC,
    pub glVertexAttribI4sv: PFNGLVERTEXATTRIBI4SVPROC,
    pub glVertexAttribI4ubv: PFNGLVERTEXATTRIBI4UBVPROC,
    pub glVertexAttribI4usv: PFNGLVERTEXATTRIBI4USVPROC,
    pub glGetUniformuiv: PFNGLGETUNIFORMUIVPROC,
    pub glUniform1ui: PFNGLUNIFORM1UIPROC,
    pub glUniform2ui: PFNGLUNIFORM2UIPROC,
    pub glUniform3ui: PFNGLUNIFORM3UIPROC,
    pub glUniform4ui: PFNGLUNIFORM4UIPROC,
    pub glUniform1uiv: PFNGLUNIFORM1UIVPROC,
    pub glUniform2uiv: PFNGLUNIFORM2UIVPROC,
    pub glUniform3uiv: PFNGLUNIFORM3UIVPROC,
    pub glUniform4uiv: PFNGLUNIFORM4UIVPROC,
    pub glBindFragDataLocation: PFNGLBINDFRAGDATALOCATIONPROC,
    pub glGetFragDataLocation: PFNGLGETFRAGDATALOCATIONPROC,
    pub glClearBufferiv: PFNGLCLEARBUFFERIVPROC,
    pub glClearBufferuiv: PFNGLCLEARBUFFERUIVPROC,
    pub glClearBufferfv: PFNGLCLEARBUFFERFVPROC,
    pub glClearBufferfi: PFNGLCLEARBUFFERFIPROC,
    pub glGetStringi: PFNGLGETSTRINGIPROC,
    pub glIsRenderbuffer: PFNGLISRENDERBUFFERPROC,
    pub glBindRenderbuffer: PFNGLBINDRENDERBUFFERPROC,
    pub glDeleteRenderbuffers: PFNGLDELETERENDERBUFFERSPROC,
    pub glGenRenderbuffers: PFNGLGENRENDERBUFFERSPROC,
    pub glRenderbufferStorage: PFNGLRENDERBUFFERSTORAGEPROC,
    pub glGetRenderbufferParameteriv: PFNGLGETRENDERBUFFERPARAMETERIVPROC,
    pub glIsFramebuffer: PFNGLISFRAMEBUFFERPROC,
    pub glBindFramebuffer: PFNGLBINDFRAMEBUFFERPROC,
    pub glDeleteFramebuffers: PFNGLDELETEFRAMEBUFFERSPROC,
    pub glGenFramebuffers: PFNGLGENFRAMEBUFFERSPROC,
    pub glCheckFramebufferStatus: PFNGLCHECKFRAMEBUFFERSTATUSPROC,
    pub glFramebufferTexture1D: PFNGLFRAMEBUFFERTEXTURE1DPROC,
    pub glFramebufferTexture2D: PFNGLFRAMEBUFFERTEXTURE2DPROC,
    pub glFramebufferTexture3D: PFNGLFRAMEBUFFERTEXTURE3DPROC,
    pub glFramebufferRenderbuffer: PFNGLFRAMEBUFFERRENDERBUFFERPROC,
    pub glGetFramebufferAttachmentParameteriv: PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC,
    pub glGenerateMipmap: PFNGLGENERATEMIPMAPPROC,
    pub glBlitFramebuffer: PFNGLBLITFRAMEBUFFERPROC,
    pub glRenderbufferStorageMultisample: PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC,
    pub glFramebufferTextureLayer: PFNGLFRAMEBUFFERTEXTURELAYERPROC,
    pub glBindVertexArray: PFNGLBINDVERTEXARRAYPROC,
    pub glDeleteVertexArrays: PFNGLDELETEVERTEXARRAYSPROC,
    pub glGenVertexArrays: PFNGLGENVERTEXARRAYSPROC,
    pub glIsVertexArray: PFNGLISVERTEXARRAYPROC,
    pub glMapBufferRange: PFNGLMAPBUFFERRANGEPROC,
    pub glFlushMappedBufferRange: PFNGLFLUSHMAPPEDBUFFERRANGEPROC,
}
