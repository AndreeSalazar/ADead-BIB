//! OpenGL 2.0 - 2.1 Constants
//! Based on Khronos canonical specifications
//! https://registry.khronos.org/OpenGL/

use super::types::GLenum;

// ============================================================================
// OpenGL 2.0 Constants
// ============================================================================

// Shaders
pub const GL_FRAGMENT_SHADER: GLenum = 0x8B30;
pub const GL_VERTEX_SHADER: GLenum = 0x8B31;
pub const GL_MAX_VERTEX_ATTRIBS: GLenum = 0x8869;
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8B4A;
pub const GL_MAX_VARYING_FLOATS: GLenum = 0x8B4B;
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4C;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: GLenum = 0x8B4D;
pub const GL_MAX_TEXTURE_IMAGE_UNITS: GLenum = 0x8872;
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8B49;
pub const GL_SHADER_TYPE: GLenum = 0x8B4F;
pub const GL_DELETE_STATUS: GLenum = 0x8B80;
pub const GL_LINK_STATUS: GLenum = 0x8B82;
pub const GL_VALIDATE_STATUS: GLenum = 0x8B83;
pub const GL_ATTACHED_SHADERS: GLenum = 0x8B85;
pub const GL_ACTIVE_UNIFORMS: GLenum = 0x8B86;
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: GLenum = 0x8B87;
pub const GL_ACTIVE_ATTRIBUTES: GLenum = 0x8B89;
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: GLenum = 0x8B8A;
pub const GL_SHADING_LANGUAGE_VERSION: GLenum = 0x8B8C;
pub const GL_CURRENT_PROGRAM: GLenum = 0x8B8D;

// Blend equation
pub const GL_BLEND_EQUATION_RGB: GLenum = 0x8009;
pub const GL_BLEND_EQUATION_ALPHA: GLenum = 0x883D;

// Vertex program point size
pub const GL_VERTEX_PROGRAM_POINT_SIZE: GLenum = 0x8642;
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: GLenum = 0x8645;

// Stencil separate
pub const GL_STENCIL_BACK_FUNC: GLenum = 0x8800;
pub const GL_STENCIL_BACK_FAIL: GLenum = 0x8801;
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: GLenum = 0x8802;
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: GLenum = 0x8803;
pub const GL_STENCIL_BACK_REF: GLenum = 0x8CA3;
pub const GL_STENCIL_BACK_VALUE_MASK: GLenum = 0x8CA4;
pub const GL_STENCIL_BACK_WRITEMASK: GLenum = 0x8CA5;

// Draw buffers
pub const GL_MAX_DRAW_BUFFERS: GLenum = 0x8824;
pub const GL_DRAW_BUFFER0: GLenum = 0x8825;
pub const GL_DRAW_BUFFER1: GLenum = 0x8826;
pub const GL_DRAW_BUFFER2: GLenum = 0x8827;
pub const GL_DRAW_BUFFER3: GLenum = 0x8828;
pub const GL_DRAW_BUFFER4: GLenum = 0x8829;
pub const GL_DRAW_BUFFER5: GLenum = 0x882A;
pub const GL_DRAW_BUFFER6: GLenum = 0x882B;
pub const GL_DRAW_BUFFER7: GLenum = 0x882C;
pub const GL_DRAW_BUFFER8: GLenum = 0x882D;
pub const GL_DRAW_BUFFER9: GLenum = 0x882E;
pub const GL_DRAW_BUFFER10: GLenum = 0x882F;
pub const GL_DRAW_BUFFER11: GLenum = 0x8830;
pub const GL_DRAW_BUFFER12: GLenum = 0x8831;
pub const GL_DRAW_BUFFER13: GLenum = 0x8832;
pub const GL_DRAW_BUFFER14: GLenum = 0x8833;
pub const GL_DRAW_BUFFER15: GLenum = 0x8834;

// Point sprites
pub const GL_POINT_SPRITE: GLenum = 0x8861;
pub const GL_COORD_REPLACE: GLenum = 0x8862;
pub const GL_POINT_SPRITE_COORD_ORIGIN: GLenum = 0x8CA0;
pub const GL_LOWER_LEFT: GLenum = 0x8CA1;
pub const GL_UPPER_LEFT: GLenum = 0x8CA2;

// Shader compile status
pub const GL_COMPILE_STATUS: GLenum = 0x8B81;
pub const GL_INFO_LOG_LENGTH: GLenum = 0x8B84;
pub const GL_SHADER_SOURCE_LENGTH: GLenum = 0x8B88;

// Shader uniform types
pub const GL_FLOAT_VEC2: GLenum = 0x8B50;
pub const GL_FLOAT_VEC3: GLenum = 0x8B51;
pub const GL_FLOAT_VEC4: GLenum = 0x8B52;
pub const GL_INT_VEC2: GLenum = 0x8B53;
pub const GL_INT_VEC3: GLenum = 0x8B54;
pub const GL_INT_VEC4: GLenum = 0x8B55;
pub const GL_BOOL: GLenum = 0x8B56;
pub const GL_BOOL_VEC2: GLenum = 0x8B57;
pub const GL_BOOL_VEC3: GLenum = 0x8B58;
pub const GL_BOOL_VEC4: GLenum = 0x8B59;
pub const GL_FLOAT_MAT2: GLenum = 0x8B5A;
pub const GL_FLOAT_MAT3: GLenum = 0x8B5B;
pub const GL_FLOAT_MAT4: GLenum = 0x8B5C;
pub const GL_SAMPLER_1D: GLenum = 0x8B5D;
pub const GL_SAMPLER_2D: GLenum = 0x8B5E;
pub const GL_SAMPLER_3D: GLenum = 0x8B5F;
pub const GL_SAMPLER_CUBE: GLenum = 0x8B60;
pub const GL_SAMPLER_1D_SHADOW: GLenum = 0x8B61;
pub const GL_SAMPLER_2D_SHADOW: GLenum = 0x8B62;

// Vertex attrib array
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: GLenum = 0x8622;
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: GLenum = 0x8623;
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: GLenum = 0x8624;
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: GLenum = 0x8625;
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: GLenum = 0x886A;
pub const GL_CURRENT_VERTEX_ATTRIB: GLenum = 0x8626;

// ============================================================================
// OpenGL 2.1 Constants
// ============================================================================

// Pixel buffer objects
pub const GL_PIXEL_PACK_BUFFER: GLenum = 0x88EB;
pub const GL_PIXEL_UNPACK_BUFFER: GLenum = 0x88EC;
pub const GL_PIXEL_PACK_BUFFER_BINDING: GLenum = 0x88ED;
pub const GL_PIXEL_UNPACK_BUFFER_BINDING: GLenum = 0x88EF;

// sRGB textures
pub const GL_SRGB: GLenum = 0x8C40;
pub const GL_SRGB8: GLenum = 0x8C41;
pub const GL_SRGB_ALPHA: GLenum = 0x8C42;
pub const GL_SRGB8_ALPHA8: GLenum = 0x8C43;
pub const GL_COMPRESSED_SRGB: GLenum = 0x8C48;
pub const GL_COMPRESSED_SRGB_ALPHA: GLenum = 0x8C49;

// Non-square matrices
pub const GL_FLOAT_MAT2x3: GLenum = 0x8B65;
pub const GL_FLOAT_MAT2x4: GLenum = 0x8B66;
pub const GL_FLOAT_MAT3x2: GLenum = 0x8B67;
pub const GL_FLOAT_MAT3x4: GLenum = 0x8B68;
pub const GL_FLOAT_MAT4x2: GLenum = 0x8B69;
pub const GL_FLOAT_MAT4x3: GLenum = 0x8B6A;

// Sluminance
pub const GL_SLUMINANCE_ALPHA: GLenum = 0x8C44;
pub const GL_SLUMINANCE8_ALPHA8: GLenum = 0x8C45;
pub const GL_SLUMINANCE: GLenum = 0x8C46;
pub const GL_SLUMINANCE8: GLenum = 0x8C47;
pub const GL_COMPRESSED_SLUMINANCE: GLenum = 0x8C4A;
pub const GL_COMPRESSED_SLUMINANCE_ALPHA: GLenum = 0x8C4B;
