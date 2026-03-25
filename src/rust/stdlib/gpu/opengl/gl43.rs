//! OpenGL 4.3 Functions — Compute shaders, SSBOs, debug output, multi-draw indirect
//! Based on Khronos canonical specifications
use super::types::*;

// Clear buffer data
pub type PFNGLCLEARBUFFERDATAPROC = Option<unsafe extern "system" fn(target: GLenum, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const GLvoid)>;
pub type PFNGLCLEARBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(target: GLenum, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const GLvoid)>;

// Compute shaders
pub type PFNGLDISPATCHCOMPUTEPROC = Option<unsafe extern "system" fn(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint)>;
pub type PFNGLDISPATCHCOMPUTEINDIRECTPROC = Option<unsafe extern "system" fn(indirect: GLintptr)>;

// Copy image
pub type PFNGLCOPYIMAGESUBDATAPROC = Option<unsafe extern "system" fn(srcName: GLuint, srcTarget: GLenum, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: GLenum, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei)>;

// Framebuffer parameter
pub type PFNGLFRAMEBUFFERPARAMETERIPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, param: GLint)>;
pub type PFNGLGETFRAMEBUFFERPARAMETERIVPROC = Option<unsafe extern "system" fn(target: GLenum, pname: GLenum, params: *mut GLint)>;

// Internal format query i64
pub type PFNGLGETINTERNALFORMATI64VPROC = Option<unsafe extern "system" fn(target: GLenum, internalformat: GLenum, pname: GLenum, count: GLsizei, params: *mut GLint64)>;

// Invalidate framebuffer
pub type PFNGLINVALIDATETEXSUBIMAGEPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei)>;
pub type PFNGLINVALIDATETEXIMAGEPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint)>;
pub type PFNGLINVALIDATEBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr)>;
pub type PFNGLINVALIDATEBUFFERDATAPROC = Option<unsafe extern "system" fn(buffer: GLuint)>;
pub type PFNGLINVALIDATEFRAMEBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum)>;
pub type PFNGLINVALIDATESUBFRAMEBUFFERPROC = Option<unsafe extern "system" fn(target: GLenum, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;

// Multi-draw indirect
pub type PFNGLMULTIDRAWARRAYSINDIRECTPROC = Option<unsafe extern "system" fn(mode: GLenum, indirect: *const GLvoid, drawcount: GLsizei, stride: GLsizei)>;
pub type PFNGLMULTIDRAWELEMENTSINDIRECTPROC = Option<unsafe extern "system" fn(mode: GLenum, type_: GLenum, indirect: *const GLvoid, drawcount: GLsizei, stride: GLsizei)>;

// Program interface query
pub type PFNGLGETPROGRAMINTERFACEIVPROC = Option<unsafe extern "system" fn(program: GLuint, programInterface: GLenum, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETPROGRAMRESOURCEINDEXPROC = Option<unsafe extern "system" fn(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLuint>;
pub type PFNGLGETPROGRAMRESOURCENAMEPROC = Option<unsafe extern "system" fn(program: GLuint, programInterface: GLenum, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar)>;
pub type PFNGLGETPROGRAMRESOURCEIVPROC = Option<unsafe extern "system" fn(program: GLuint, programInterface: GLenum, index: GLuint, propCount: GLsizei, props: *const GLenum, count: GLsizei, length: *mut GLsizei, params: *mut GLint)>;
pub type PFNGLGETPROGRAMRESOURCELOCATIONPROC = Option<unsafe extern "system" fn(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint>;
pub type PFNGLGETPROGRAMRESOURCELOCATIONINDEXPROC = Option<unsafe extern "system" fn(program: GLuint, programInterface: GLenum, name: *const GLchar) -> GLint>;

// Shader storage buffer object
pub type PFNGLSHADERSTORAGEBLOCKBINDINGPROC = Option<unsafe extern "system" fn(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint)>;

// Texture buffer range
pub type PFNGLTEXBUFFERRANGEPROC = Option<unsafe extern "system" fn(target: GLenum, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr)>;

// Texture storage multisample
pub type PFNGLTEXSTORAGE2DMULTISAMPLEPROC = Option<unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean)>;
pub type PFNGLTEXSTORAGE3DMULTISAMPLEPROC = Option<unsafe extern "system" fn(target: GLenum, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean)>;

// Texture view
pub type PFNGLTEXTUREVIEWPROC = Option<unsafe extern "system" fn(texture: GLuint, target: GLenum, origtexture: GLuint, internalformat: GLenum, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint)>;

// Vertex attrib binding
pub type PFNGLBINDVERTEXBUFFERPROC = Option<unsafe extern "system" fn(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei)>;
pub type PFNGLVERTEXATTRIBFORMATPROC = Option<unsafe extern "system" fn(attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint)>;
pub type PFNGLVERTEXATTRIBIFORMATPROC = Option<unsafe extern "system" fn(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint)>;
pub type PFNGLVERTEXATTRIBLFORMATPROC = Option<unsafe extern "system" fn(attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint)>;
pub type PFNGLVERTEXATTRIBBINDINGPROC = Option<unsafe extern "system" fn(attribindex: GLuint, bindingindex: GLuint)>;
pub type PFNGLVERTEXBINDINGDIVISORPROC = Option<unsafe extern "system" fn(bindingindex: GLuint, divisor: GLuint)>;

// Debug output
pub type PFNGLDEBUGMESSAGECONTROLPROC = Option<unsafe extern "system" fn(source: GLenum, type_: GLenum, severity: GLenum, count: GLsizei, ids: *const GLuint, enabled: GLboolean)>;
pub type PFNGLDEBUGMESSAGEINSERTPROC = Option<unsafe extern "system" fn(source: GLenum, type_: GLenum, id: GLuint, severity: GLenum, length: GLsizei, buf: *const GLchar)>;
pub type PFNGLDEBUGMESSAGECALLBACKPROC = Option<unsafe extern "system" fn(callback: GLDEBUGPROC, userParam: *const GLvoid)>;
pub type PFNGLGETDEBUGMESSAGELOGPROC = Option<unsafe extern "system" fn(count: GLuint, bufSize: GLsizei, sources: *mut GLenum, types: *mut GLenum, ids: *mut GLuint, severities: *mut GLenum, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint>;
pub type PFNGLPUSHDEBUGGROUPPROC = Option<unsafe extern "system" fn(source: GLenum, id: GLuint, length: GLsizei, message: *const GLchar)>;
pub type PFNGLPOPDEBUGGROUPPROC = Option<unsafe extern "system" fn()>;
pub type PFNGLOBJECTLABELPROC = Option<unsafe extern "system" fn(identifier: GLenum, name: GLuint, length: GLsizei, label: *const GLchar)>;
pub type PFNGLGETOBJECTLABELPROC = Option<unsafe extern "system" fn(identifier: GLenum, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar)>;
pub type PFNGLOBJECTPTRLABELPROC = Option<unsafe extern "system" fn(ptr: *const GLvoid, length: GLsizei, label: *const GLchar)>;
pub type PFNGLGETOBJECTPTRLABELPROC = Option<unsafe extern "system" fn(ptr: *const GLvoid, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar)>;

/// OpenGL 4.3 function table
#[derive(Default)]
pub struct GL43 {
    pub glClearBufferData: PFNGLCLEARBUFFERDATAPROC,
    pub glClearBufferSubData: PFNGLCLEARBUFFERSUBDATAPROC,
    pub glDispatchCompute: PFNGLDISPATCHCOMPUTEPROC,
    pub glDispatchComputeIndirect: PFNGLDISPATCHCOMPUTEINDIRECTPROC,
    pub glCopyImageSubData: PFNGLCOPYIMAGESUBDATAPROC,
    pub glFramebufferParameteri: PFNGLFRAMEBUFFERPARAMETERIPROC,
    pub glGetFramebufferParameteriv: PFNGLGETFRAMEBUFFERPARAMETERIVPROC,
    pub glGetInternalformati64v: PFNGLGETINTERNALFORMATI64VPROC,
    pub glInvalidateTexSubImage: PFNGLINVALIDATETEXSUBIMAGEPROC,
    pub glInvalidateTexImage: PFNGLINVALIDATETEXIMAGEPROC,
    pub glInvalidateBufferSubData: PFNGLINVALIDATEBUFFERSUBDATAPROC,
    pub glInvalidateBufferData: PFNGLINVALIDATEBUFFERDATAPROC,
    pub glInvalidateFramebuffer: PFNGLINVALIDATEFRAMEBUFFERPROC,
    pub glInvalidateSubFramebuffer: PFNGLINVALIDATESUBFRAMEBUFFERPROC,
    pub glMultiDrawArraysIndirect: PFNGLMULTIDRAWARRAYSINDIRECTPROC,
    pub glMultiDrawElementsIndirect: PFNGLMULTIDRAWELEMENTSINDIRECTPROC,
    pub glGetProgramInterfaceiv: PFNGLGETPROGRAMINTERFACEIVPROC,
    pub glGetProgramResourceIndex: PFNGLGETPROGRAMRESOURCEINDEXPROC,
    pub glGetProgramResourceName: PFNGLGETPROGRAMRESOURCENAMEPROC,
    pub glGetProgramResourceiv: PFNGLGETPROGRAMRESOURCEIVPROC,
    pub glGetProgramResourceLocation: PFNGLGETPROGRAMRESOURCELOCATIONPROC,
    pub glGetProgramResourceLocationIndex: PFNGLGETPROGRAMRESOURCELOCATIONINDEXPROC,
    pub glShaderStorageBlockBinding: PFNGLSHADERSTORAGEBLOCKBINDINGPROC,
    pub glTexBufferRange: PFNGLTEXBUFFERRANGEPROC,
    pub glTexStorage2DMultisample: PFNGLTEXSTORAGE2DMULTISAMPLEPROC,
    pub glTexStorage3DMultisample: PFNGLTEXSTORAGE3DMULTISAMPLEPROC,
    pub glTextureView: PFNGLTEXTUREVIEWPROC,
    pub glBindVertexBuffer: PFNGLBINDVERTEXBUFFERPROC,
    pub glVertexAttribFormat: PFNGLVERTEXATTRIBFORMATPROC,
    pub glVertexAttribIFormat: PFNGLVERTEXATTRIBIFORMATPROC,
    pub glVertexAttribLFormat: PFNGLVERTEXATTRIBLFORMATPROC,
    pub glVertexAttribBinding: PFNGLVERTEXATTRIBBINDINGPROC,
    pub glVertexBindingDivisor: PFNGLVERTEXBINDINGDIVISORPROC,
    pub glDebugMessageControl: PFNGLDEBUGMESSAGECONTROLPROC,
    pub glDebugMessageInsert: PFNGLDEBUGMESSAGEINSERTPROC,
    pub glDebugMessageCallback: PFNGLDEBUGMESSAGECALLBACKPROC,
    pub glGetDebugMessageLog: PFNGLGETDEBUGMESSAGELOGPROC,
    pub glPushDebugGroup: PFNGLPUSHDEBUGGROUPPROC,
    pub glPopDebugGroup: PFNGLPOPDEBUGGROUPPROC,
    pub glObjectLabel: PFNGLOBJECTLABELPROC,
    pub glGetObjectLabel: PFNGLGETOBJECTLABELPROC,
    pub glObjectPtrLabel: PFNGLOBJECTPTRLABELPROC,
    pub glGetObjectPtrLabel: PFNGLGETOBJECTPTRLABELPROC,
}
