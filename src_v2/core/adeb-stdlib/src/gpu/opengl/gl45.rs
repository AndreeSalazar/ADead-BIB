//! OpenGL 4.5 Functions — Direct State Access (DSA), clip control, robustness
//! Based on Khronos canonical specifications
use super::types::*;

// Clip control
pub type PFNGLCLIPCONTROLPROC = Option<unsafe extern "system" fn(origin: GLenum, depth: GLenum)>;

// DSA: Transform feedback
pub type PFNGLCREATETRANSFORMFEEDBACKSPROC = Option<unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint)>;
pub type PFNGLTRANSFORMFEEDBACKBUFFERBASEPROC = Option<unsafe extern "system" fn(xfb: GLuint, index: GLuint, buffer: GLuint)>;
pub type PFNGLTRANSFORMFEEDBACKBUFFERRANGEPROC = Option<unsafe extern "system" fn(xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr)>;
pub type PFNGLGETTRANSFORMFEEDBACKIVPROC = Option<unsafe extern "system" fn(xfb: GLuint, pname: GLenum, param: *mut GLint)>;
pub type PFNGLGETTRANSFORMFEEDBACKI_VPROC = Option<unsafe extern "system" fn(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint)>;
pub type PFNGLGETTRANSFORMFEEDBACKI64_VPROC = Option<unsafe extern "system" fn(xfb: GLuint, pname: GLenum, index: GLuint, param: *mut GLint64)>;

// DSA: Buffers
pub type PFNGLCREATEBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint)>;
pub type PFNGLNAMEDBUFFERSTORAGEPROC = Option<unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const GLvoid, flags: GLbitfield)>;
pub type PFNGLNAMEDBUFFERDATAPROC = Option<unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const GLvoid, usage: GLenum)>;
pub type PFNGLNAMEDBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const GLvoid)>;
pub type PFNGLCOPYNAMEDBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr)>;
pub type PFNGLCLEARNAMEDBUFFERDATAPROC = Option<unsafe extern "system" fn(buffer: GLuint, internalformat: GLenum, format: GLenum, type_: GLenum, data: *const GLvoid)>;
pub type PFNGLCLEARNAMEDBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(buffer: GLuint, internalformat: GLenum, offset: GLintptr, size: GLsizeiptr, format: GLenum, type_: GLenum, data: *const GLvoid)>;
pub type PFNGLMAPNAMEDBUFFERPROC = Option<unsafe extern "system" fn(buffer: GLuint, access: GLenum) -> *mut GLvoid>;
pub type PFNGLMAPNAMEDBUFFERRANGEPROC = Option<unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut GLvoid>;
pub type PFNGLUNMAPNAMEDBUFFERPROC = Option<unsafe extern "system" fn(buffer: GLuint) -> GLboolean>;
pub type PFNGLFLUSHMAPPEDNAMEDBUFFERRANGEPROC = Option<unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr)>;
pub type PFNGLGETNAMEDBUFFERPARAMETERIVPROC = Option<unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETNAMEDBUFFERPARAMETERI64VPROC = Option<unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut GLint64)>;
pub type PFNGLGETNAMEDBUFFERPOINTERVPROC = Option<unsafe extern "system" fn(buffer: GLuint, pname: GLenum, params: *mut *mut GLvoid)>;
pub type PFNGLGETNAMEDBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut GLvoid)>;

// DSA: Framebuffers
pub type PFNGLCREATEFRAMEBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint)>;
pub type PFNGLNAMEDFRAMEBUFFERRENDERBUFFERPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, renderbuffertarget: GLenum, renderbuffer: GLuint)>;
pub type PFNGLNAMEDFRAMEBUFFERPARAMETERIPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, param: GLint)>;
pub type PFNGLNAMEDFRAMEBUFFERTEXTUREPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint)>;
pub type PFNGLNAMEDFRAMEBUFFERTEXTURELAYERPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, texture: GLuint, level: GLint, layer: GLint)>;
pub type PFNGLNAMEDFRAMEBUFFERDRAWBUFFERPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, buf: GLenum)>;
pub type PFNGLNAMEDFRAMEBUFFERDRAWBUFFERSPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, n: GLsizei, bufs: *const GLenum)>;
pub type PFNGLNAMEDFRAMEBUFFERREADBUFFERPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, src: GLenum)>;
pub type PFNGLINVALIDATENAMEDFRAMEBUFFERDATAPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum)>;
pub type PFNGLINVALIDATENAMEDFRAMEBUFFERSUBDATAPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const GLenum, x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;
pub type PFNGLCLEARNAMEDFRAMEBUFFERIVPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLint)>;
pub type PFNGLCLEARNAMEDFRAMEBUFFERUIVPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLuint)>;
pub type PFNGLCLEARNAMEDFRAMEBUFFERFVPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, value: *const GLfloat)>;
pub type PFNGLCLEARNAMEDFRAMEBUFFERFIPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint)>;
pub type PFNGLBLITNAMEDFRAMEBUFFERPROC = Option<unsafe extern "system" fn(readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: GLenum)>;
pub type PFNGLCHECKNAMEDFRAMEBUFFERSTATUSPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, target: GLenum) -> GLenum>;
pub type PFNGLGETNAMEDFRAMEBUFFERPARAMETERIVPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, pname: GLenum, param: *mut GLint)>;
pub type PFNGLGETNAMEDFRAMEBUFFERATTACHMENTPARAMETERIVPROC = Option<unsafe extern "system" fn(framebuffer: GLuint, attachment: GLenum, pname: GLenum, params: *mut GLint)>;

// DSA: Renderbuffers
pub type PFNGLCREATERENDERBUFFERSPROC = Option<unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint)>;
pub type PFNGLNAMEDRENDERBUFFERSTORAGEPROC = Option<unsafe extern "system" fn(renderbuffer: GLuint, internalformat: GLenum, width: GLsizei, height: GLsizei)>;
pub type PFNGLNAMEDRENDERBUFFERSTORAGEMULTISAMPLEPROC = Option<unsafe extern "system" fn(renderbuffer: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei)>;
pub type PFNGLGETNAMEDRENDERBUFFERPARAMETERIVPROC = Option<unsafe extern "system" fn(renderbuffer: GLuint, pname: GLenum, params: *mut GLint)>;

// DSA: Textures
pub type PFNGLCREATETEXTURESPROC = Option<unsafe extern "system" fn(target: GLenum, n: GLsizei, textures: *mut GLuint)>;
pub type PFNGLTEXTUREBUFFERPROC = Option<unsafe extern "system" fn(texture: GLuint, internalformat: GLenum, buffer: GLuint)>;
pub type PFNGLTEXTUREBUFFERRANGEPROC = Option<unsafe extern "system" fn(texture: GLuint, internalformat: GLenum, buffer: GLuint, offset: GLintptr, size: GLsizeiptr)>;
pub type PFNGLTEXTURESTORAGE1DPROC = Option<unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei)>;
pub type PFNGLTEXTURESTORAGE2DPROC = Option<unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei)>;
pub type PFNGLTEXTURESTORAGE3DPROC = Option<unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei)>;
pub type PFNGLTEXTURESTORAGE2DMULTISAMPLEPROC = Option<unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean)>;
pub type PFNGLTEXTURESTORAGE3DMULTISAMPLEPROC = Option<unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalformat: GLenum, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean)>;
pub type PFNGLTEXTURESUBIMAGE1DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, type_: GLenum, pixels: *const GLvoid)>;
pub type PFNGLTEXTURESUBIMAGE2DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const GLvoid)>;
pub type PFNGLTEXTURESUBIMAGE3DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, type_: GLenum, pixels: *const GLvoid)>;
pub type PFNGLCOMPRESSEDTEXTURESUBIMAGE1DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: GLenum, imageSize: GLsizei, data: *const GLvoid)>;
pub type PFNGLCOMPRESSEDTEXTURESUBIMAGE2DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: GLenum, imageSize: GLsizei, data: *const GLvoid)>;
pub type PFNGLCOMPRESSEDTEXTURESUBIMAGE3DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: GLenum, imageSize: GLsizei, data: *const GLvoid)>;
pub type PFNGLCOPYTEXTURESUBIMAGE1DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei)>;
pub type PFNGLCOPYTEXTURESUBIMAGE2DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;
pub type PFNGLCOPYTEXTURESUBIMAGE3DPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei)>;
pub type PFNGLTEXTUREPARAMETERFPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: GLfloat)>;
pub type PFNGLTEXTUREPARAMETERFVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: *const GLfloat)>;
pub type PFNGLTEXTUREPARAMETERIPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: GLint)>;
pub type PFNGLTEXTUREPARAMETERIIVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *const GLint)>;
pub type PFNGLTEXTUREPARAMETERIUIVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *const GLuint)>;
pub type PFNGLTEXTUREPARAMETERIVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, param: *const GLint)>;
pub type PFNGLGENERATETEXTUREMIPMAPPROC = Option<unsafe extern "system" fn(texture: GLuint)>;
pub type PFNGLBINDTEXTUREUNITPROC = Option<unsafe extern "system" fn(unit: GLuint, texture: GLuint)>;
pub type PFNGLGETTEXTUREIMAGEPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut GLvoid)>;
pub type PFNGLGETCOMPRESSEDTEXTUREIMAGEPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut GLvoid)>;
pub type PFNGLGETTEXTURELEVELPARAMETERFVPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLfloat)>;
pub type PFNGLGETTEXTURELEVELPARAMETERIVPROC = Option<unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETTEXTUREPARAMETERFVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLfloat)>;
pub type PFNGLGETTEXTUREPARAMETERIIVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLint)>;
pub type PFNGLGETTEXTUREPARAMETERIUIVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLuint)>;
pub type PFNGLGETTEXTUREPARAMETERIVPROC = Option<unsafe extern "system" fn(texture: GLuint, pname: GLenum, params: *mut GLint)>;

// DSA: VAOs
pub type PFNGLCREATEVERTEXARRAYSPROC = Option<unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint)>;
pub type PFNGLDISABLEVERTEXARRAYATTRIBPROC = Option<unsafe extern "system" fn(vaobj: GLuint, index: GLuint)>;
pub type PFNGLENABLEVERTEXARRAYATTRIBPROC = Option<unsafe extern "system" fn(vaobj: GLuint, index: GLuint)>;
pub type PFNGLVERTEXARRAYELEMENTBUFFERPROC = Option<unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint)>;
pub type PFNGLVERTEXARRAYVERTEXBUFFERPROC = Option<unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei)>;
pub type PFNGLVERTEXARRAYVERTEXBUFFERSPROC = Option<unsafe extern "system" fn(vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei)>;
pub type PFNGLVERTEXARRAYATTRIBBINDINGPROC = Option<unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint)>;
pub type PFNGLVERTEXARRAYATTRIBFORMATPROC = Option<unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, normalized: GLboolean, relativeoffset: GLuint)>;
pub type PFNGLVERTEXARRAYATTRIBIFORMATPROC = Option<unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint)>;
pub type PFNGLVERTEXARRAYATTRIBLFORMATPROC = Option<unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: GLenum, relativeoffset: GLuint)>;
pub type PFNGLVERTEXARRAYBINDINGDIVISORPROC = Option<unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint)>;
pub type PFNGLGETVERTEXARRAYIVPROC = Option<unsafe extern "system" fn(vaobj: GLuint, pname: GLenum, param: *mut GLint)>;
pub type PFNGLGETVERTEXARRAYINDEXEDIVPROC = Option<unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint)>;
pub type PFNGLGETVERTEXARRAYINDEXED64IVPROC = Option<unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: GLenum, param: *mut GLint64)>;

// DSA: Samplers
pub type PFNGLCREATESAMPLERSPROC = Option<unsafe extern "system" fn(n: GLsizei, samplers: *mut GLuint)>;

// DSA: Program pipelines
pub type PFNGLCREATEPROGRAMPIPELINESPROC = Option<unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint)>;

// DSA: Queries
pub type PFNGLCREATEQUERIESPROC = Option<unsafe extern "system" fn(target: GLenum, n: GLsizei, ids: *mut GLuint)>;
pub type PFNGLGETQUERYBUFFEROBJECTI64VPROC = Option<unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr)>;
pub type PFNGLGETQUERYBUFFEROBJECTIVPROC = Option<unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr)>;
pub type PFNGLGETQUERYBUFFEROBJECTUI64VPROC = Option<unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr)>;
pub type PFNGLGETQUERYBUFFEROBJECTUIVPROC = Option<unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: GLenum, offset: GLintptr)>;

// Robustness
pub type PFNGLGETGRAPHICSRESETSTATUSPROC = Option<unsafe extern "system" fn() -> GLenum>;
pub type PFNGLGETNCOMPRESSEDTEXIMAGEPROC = Option<unsafe extern "system" fn(target: GLenum, lod: GLint, bufSize: GLsizei, pixels: *mut GLvoid)>;
pub type PFNGLGETNTEXIMAGEPROC = Option<unsafe extern "system" fn(target: GLenum, level: GLint, format: GLenum, type_: GLenum, bufSize: GLsizei, pixels: *mut GLvoid)>;
pub type PFNGLGETNUNIFORMDVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble)>;
pub type PFNGLGETNUNIFORMFVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat)>;
pub type PFNGLGETNUNIFORMIVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint)>;
pub type PFNGLGETNUNIFORMUIVPROC = Option<unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint)>;
pub type PFNGLREADNPIXELSPROC = Option<unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, bufSize: GLsizei, data: *mut GLvoid)>;
pub type PFNGLGETNMAPDVPROC = Option<unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLdouble)>;
pub type PFNGLGETNMAPFVPROC = Option<unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLfloat)>;
pub type PFNGLGETNMAPIVPROC = Option<unsafe extern "system" fn(target: GLenum, query: GLenum, bufSize: GLsizei, v: *mut GLint)>;

// Memory barrier by region
pub type PFNGLMEMORYBARRIERBYREGIONPROC = Option<unsafe extern "system" fn(barriers: GLbitfield)>;

// Texture barrier
pub type PFNGLTEXTUREBARRIERPROC = Option<unsafe extern "system" fn()>;

/// OpenGL 4.5 function table — Direct State Access
#[derive(Default)]
pub struct GL45 {
    pub glClipControl: PFNGLCLIPCONTROLPROC,
    pub glCreateTransformFeedbacks: PFNGLCREATETRANSFORMFEEDBACKSPROC,
    pub glTransformFeedbackBufferBase: PFNGLTRANSFORMFEEDBACKBUFFERBASEPROC,
    pub glTransformFeedbackBufferRange: PFNGLTRANSFORMFEEDBACKBUFFERRANGEPROC,
    pub glGetTransformFeedbackiv: PFNGLGETTRANSFORMFEEDBACKIVPROC,
    pub glGetTransformFeedbacki_v: PFNGLGETTRANSFORMFEEDBACKI_VPROC,
    pub glGetTransformFeedbacki64_v: PFNGLGETTRANSFORMFEEDBACKI64_VPROC,
    pub glCreateBuffers: PFNGLCREATEBUFFERSPROC,
    pub glNamedBufferStorage: PFNGLNAMEDBUFFERSTORAGEPROC,
    pub glNamedBufferData: PFNGLNAMEDBUFFERDATAPROC,
    pub glNamedBufferSubData: PFNGLNAMEDBUFFERSUBDATAPROC,
    pub glCopyNamedBufferSubData: PFNGLCOPYNAMEDBUFFERSUBDATAPROC,
    pub glClearNamedBufferData: PFNGLCLEARNAMEDBUFFERDATAPROC,
    pub glClearNamedBufferSubData: PFNGLCLEARNAMEDBUFFERSUBDATAPROC,
    pub glMapNamedBuffer: PFNGLMAPNAMEDBUFFERPROC,
    pub glMapNamedBufferRange: PFNGLMAPNAMEDBUFFERRANGEPROC,
    pub glUnmapNamedBuffer: PFNGLUNMAPNAMEDBUFFERPROC,
    pub glFlushMappedNamedBufferRange: PFNGLFLUSHMAPPEDNAMEDBUFFERRANGEPROC,
    pub glGetNamedBufferParameteriv: PFNGLGETNAMEDBUFFERPARAMETERIVPROC,
    pub glGetNamedBufferParameteri64v: PFNGLGETNAMEDBUFFERPARAMETERI64VPROC,
    pub glGetNamedBufferPointerv: PFNGLGETNAMEDBUFFERPOINTERVPROC,
    pub glGetNamedBufferSubData: PFNGLGETNAMEDBUFFERSUBDATAPROC,
    pub glCreateFramebuffers: PFNGLCREATEFRAMEBUFFERSPROC,
    pub glNamedFramebufferRenderbuffer: PFNGLNAMEDFRAMEBUFFERRENDERBUFFERPROC,
    pub glNamedFramebufferParameteri: PFNGLNAMEDFRAMEBUFFERPARAMETERIPROC,
    pub glNamedFramebufferTexture: PFNGLNAMEDFRAMEBUFFERTEXTUREPROC,
    pub glNamedFramebufferTextureLayer: PFNGLNAMEDFRAMEBUFFERTEXTURELAYERPROC,
    pub glNamedFramebufferDrawBuffer: PFNGLNAMEDFRAMEBUFFERDRAWBUFFERPROC,
    pub glNamedFramebufferDrawBuffers: PFNGLNAMEDFRAMEBUFFERDRAWBUFFERSPROC,
    pub glNamedFramebufferReadBuffer: PFNGLNAMEDFRAMEBUFFERREADBUFFERPROC,
    pub glInvalidateNamedFramebufferData: PFNGLINVALIDATENAMEDFRAMEBUFFERDATAPROC,
    pub glInvalidateNamedFramebufferSubData: PFNGLINVALIDATENAMEDFRAMEBUFFERSUBDATAPROC,
    pub glClearNamedFramebufferiv: PFNGLCLEARNAMEDFRAMEBUFFERIVPROC,
    pub glClearNamedFramebufferuiv: PFNGLCLEARNAMEDFRAMEBUFFERUIVPROC,
    pub glClearNamedFramebufferfv: PFNGLCLEARNAMEDFRAMEBUFFERFVPROC,
    pub glClearNamedFramebufferfi: PFNGLCLEARNAMEDFRAMEBUFFERFIPROC,
    pub glBlitNamedFramebuffer: PFNGLBLITNAMEDFRAMEBUFFERPROC,
    pub glCheckNamedFramebufferStatus: PFNGLCHECKNAMEDFRAMEBUFFERSTATUSPROC,
    pub glGetNamedFramebufferParameteriv: PFNGLGETNAMEDFRAMEBUFFERPARAMETERIVPROC,
    pub glGetNamedFramebufferAttachmentParameteriv: PFNGLGETNAMEDFRAMEBUFFERATTACHMENTPARAMETERIVPROC,
    pub glCreateRenderbuffers: PFNGLCREATERENDERBUFFERSPROC,
    pub glNamedRenderbufferStorage: PFNGLNAMEDRENDERBUFFERSTORAGEPROC,
    pub glNamedRenderbufferStorageMultisample: PFNGLNAMEDRENDERBUFFERSTORAGEMULTISAMPLEPROC,
    pub glGetNamedRenderbufferParameteriv: PFNGLGETNAMEDRENDERBUFFERPARAMETERIVPROC,
    pub glCreateTextures: PFNGLCREATETEXTURESPROC,
    pub glTextureBuffer: PFNGLTEXTUREBUFFERPROC,
    pub glTextureBufferRange: PFNGLTEXTUREBUFFERRANGEPROC,
    pub glTextureStorage1D: PFNGLTEXTURESTORAGE1DPROC,
    pub glTextureStorage2D: PFNGLTEXTURESTORAGE2DPROC,
    pub glTextureStorage3D: PFNGLTEXTURESTORAGE3DPROC,
    pub glTextureStorage2DMultisample: PFNGLTEXTURESTORAGE2DMULTISAMPLEPROC,
    pub glTextureStorage3DMultisample: PFNGLTEXTURESTORAGE3DMULTISAMPLEPROC,
    pub glTextureSubImage1D: PFNGLTEXTURESUBIMAGE1DPROC,
    pub glTextureSubImage2D: PFNGLTEXTURESUBIMAGE2DPROC,
    pub glTextureSubImage3D: PFNGLTEXTURESUBIMAGE3DPROC,
    pub glCompressedTextureSubImage1D: PFNGLCOMPRESSEDTEXTURESUBIMAGE1DPROC,
    pub glCompressedTextureSubImage2D: PFNGLCOMPRESSEDTEXTURESUBIMAGE2DPROC,
    pub glCompressedTextureSubImage3D: PFNGLCOMPRESSEDTEXTURESUBIMAGE3DPROC,
    pub glCopyTextureSubImage1D: PFNGLCOPYTEXTURESUBIMAGE1DPROC,
    pub glCopyTextureSubImage2D: PFNGLCOPYTEXTURESUBIMAGE2DPROC,
    pub glCopyTextureSubImage3D: PFNGLCOPYTEXTURESUBIMAGE3DPROC,
    pub glTextureParameterf: PFNGLTEXTUREPARAMETERFPROC,
    pub glTextureParameterfv: PFNGLTEXTUREPARAMETERFVPROC,
    pub glTextureParameteri: PFNGLTEXTUREPARAMETERIPROC,
    pub glTextureParameterIiv: PFNGLTEXTUREPARAMETERIIVPROC,
    pub glTextureParameterIuiv: PFNGLTEXTUREPARAMETERIUIVPROC,
    pub glTextureParameteriv: PFNGLTEXTUREPARAMETERIVPROC,
    pub glGenerateTextureMipmap: PFNGLGENERATETEXTUREMIPMAPPROC,
    pub glBindTextureUnit: PFNGLBINDTEXTUREUNITPROC,
    pub glGetTextureImage: PFNGLGETTEXTUREIMAGEPROC,
    pub glGetCompressedTextureImage: PFNGLGETCOMPRESSEDTEXTUREIMAGEPROC,
    pub glGetTextureLevelParameterfv: PFNGLGETTEXTURELEVELPARAMETERFVPROC,
    pub glGetTextureLevelParameteriv: PFNGLGETTEXTURELEVELPARAMETERIVPROC,
    pub glGetTextureParameterfv: PFNGLGETTEXTUREPARAMETERFVPROC,
    pub glGetTextureParameterIiv: PFNGLGETTEXTUREPARAMETERIIVPROC,
    pub glGetTextureParameterIuiv: PFNGLGETTEXTUREPARAMETERIUIVPROC,
    pub glGetTextureParameteriv: PFNGLGETTEXTUREPARAMETERIVPROC,
    pub glCreateVertexArrays: PFNGLCREATEVERTEXARRAYSPROC,
    pub glDisableVertexArrayAttrib: PFNGLDISABLEVERTEXARRAYATTRIBPROC,
    pub glEnableVertexArrayAttrib: PFNGLENABLEVERTEXARRAYATTRIBPROC,
    pub glVertexArrayElementBuffer: PFNGLVERTEXARRAYELEMENTBUFFERPROC,
    pub glVertexArrayVertexBuffer: PFNGLVERTEXARRAYVERTEXBUFFERPROC,
    pub glVertexArrayVertexBuffers: PFNGLVERTEXARRAYVERTEXBUFFERSPROC,
    pub glVertexArrayAttribBinding: PFNGLVERTEXARRAYATTRIBBINDINGPROC,
    pub glVertexArrayAttribFormat: PFNGLVERTEXARRAYATTRIBFORMATPROC,
    pub glVertexArrayAttribIFormat: PFNGLVERTEXARRAYATTRIBIFORMATPROC,
    pub glVertexArrayAttribLFormat: PFNGLVERTEXARRAYATTRIBLFORMATPROC,
    pub glVertexArrayBindingDivisor: PFNGLVERTEXARRAYBINDINGDIVISORPROC,
    pub glGetVertexArrayiv: PFNGLGETVERTEXARRAYIVPROC,
    pub glGetVertexArrayIndexediv: PFNGLGETVERTEXARRAYINDEXEDIVPROC,
    pub glGetVertexArrayIndexed64iv: PFNGLGETVERTEXARRAYINDEXED64IVPROC,
    pub glCreateSamplers: PFNGLCREATESAMPLERSPROC,
    pub glCreateProgramPipelines: PFNGLCREATEPROGRAMPIPELINESPROC,
    pub glCreateQueries: PFNGLCREATEQUERIESPROC,
    pub glGetQueryBufferObjecti64v: PFNGLGETQUERYBUFFEROBJECTI64VPROC,
    pub glGetQueryBufferObjectiv: PFNGLGETQUERYBUFFEROBJECTIVPROC,
    pub glGetQueryBufferObjectui64v: PFNGLGETQUERYBUFFEROBJECTUI64VPROC,
    pub glGetQueryBufferObjectuiv: PFNGLGETQUERYBUFFEROBJECTUIVPROC,
    pub glGetGraphicsResetStatus: PFNGLGETGRAPHICSRESETSTATUSPROC,
    pub glGetnCompressedTexImage: PFNGLGETNCOMPRESSEDTEXIMAGEPROC,
    pub glGetnTexImage: PFNGLGETNTEXIMAGEPROC,
    pub glGetnUniformdv: PFNGLGETNUNIFORMDVPROC,
    pub glGetnUniformfv: PFNGLGETNUNIFORMFVPROC,
    pub glGetnUniformiv: PFNGLGETNUNIFORMIVPROC,
    pub glGetnUniformuiv: PFNGLGETNUNIFORMUIVPROC,
    pub glReadnPixels: PFNGLREADNPIXELSPROC,
    pub glGetnMapdv: PFNGLGETNMAPDVPROC,
    pub glGetnMapfv: PFNGLGETNMAPFVPROC,
    pub glGetnMapiv: PFNGLGETNMAPIVPROC,
    pub glMemoryBarrierByRegion: PFNGLMEMORYBARRIERBYREGIONPROC,
    pub glTextureBarrier: PFNGLTEXTUREBARRIERPROC,
}
