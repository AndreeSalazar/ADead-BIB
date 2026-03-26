//! OpenGL 3.0 - 3.3 Constants
//! Based on Khronos canonical specifications
//! https://registry.khronos.org/OpenGL/

use super::types::GLenum;

// ============================================================================
// OpenGL 3.0 Constants
// ============================================================================

// Context flags
pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: GLenum = 0x00000001;

// Framebuffer objects
pub const GL_INVALID_FRAMEBUFFER_OPERATION: GLenum = 0x0506;
pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: GLenum = 0x8210;
pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: GLenum = 0x8211;
pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: GLenum = 0x8212;
pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: GLenum = 0x8213;
pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: GLenum = 0x8214;
pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: GLenum = 0x8215;
pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: GLenum = 0x8216;
pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: GLenum = 0x8217;
pub const GL_FRAMEBUFFER_DEFAULT: GLenum = 0x8218;
pub const GL_FRAMEBUFFER_UNDEFINED: GLenum = 0x8219;
pub const GL_DEPTH_STENCIL_ATTACHMENT: GLenum = 0x821A;
pub const GL_MAX_RENDERBUFFER_SIZE: GLenum = 0x84E8;
pub const GL_DEPTH_STENCIL: GLenum = 0x84F9;
pub const GL_UNSIGNED_INT_24_8: GLenum = 0x84FA;
pub const GL_DEPTH24_STENCIL8: GLenum = 0x88F0;
pub const GL_TEXTURE_STENCIL_SIZE: GLenum = 0x88F1;
pub const GL_TEXTURE_RED_TYPE: GLenum = 0x8C10;
pub const GL_TEXTURE_GREEN_TYPE: GLenum = 0x8C11;
pub const GL_TEXTURE_BLUE_TYPE: GLenum = 0x8C12;
pub const GL_TEXTURE_ALPHA_TYPE: GLenum = 0x8C13;
pub const GL_TEXTURE_DEPTH_TYPE: GLenum = 0x8C16;
pub const GL_UNSIGNED_NORMALIZED: GLenum = 0x8C17;
pub const GL_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub const GL_DRAW_FRAMEBUFFER_BINDING: GLenum = 0x8CA6;
pub const GL_RENDERBUFFER_BINDING: GLenum = 0x8CA7;
pub const GL_READ_FRAMEBUFFER: GLenum = 0x8CA8;
pub const GL_DRAW_FRAMEBUFFER: GLenum = 0x8CA9;
pub const GL_READ_FRAMEBUFFER_BINDING: GLenum = 0x8CAA;
pub const GL_RENDERBUFFER_SAMPLES: GLenum = 0x8CAB;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: GLenum = 0x8CD0;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: GLenum = 0x8CD1;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: GLenum = 0x8CD2;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: GLenum = 0x8CD3;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: GLenum = 0x8CD4;
pub const GL_FRAMEBUFFER_COMPLETE: GLenum = 0x8CD5;
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: GLenum = 0x8CD6;
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: GLenum = 0x8CD7;
pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: GLenum = 0x8CDB;
pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER: GLenum = 0x8CDC;
pub const GL_FRAMEBUFFER_UNSUPPORTED: GLenum = 0x8CDD;
pub const GL_MAX_COLOR_ATTACHMENTS: GLenum = 0x8CDF;
pub const GL_COLOR_ATTACHMENT0: GLenum = 0x8CE0;
pub const GL_COLOR_ATTACHMENT1: GLenum = 0x8CE1;
pub const GL_COLOR_ATTACHMENT2: GLenum = 0x8CE2;
pub const GL_COLOR_ATTACHMENT3: GLenum = 0x8CE3;
pub const GL_COLOR_ATTACHMENT4: GLenum = 0x8CE4;
pub const GL_COLOR_ATTACHMENT5: GLenum = 0x8CE5;
pub const GL_COLOR_ATTACHMENT6: GLenum = 0x8CE6;
pub const GL_COLOR_ATTACHMENT7: GLenum = 0x8CE7;
pub const GL_COLOR_ATTACHMENT8: GLenum = 0x8CE8;
pub const GL_COLOR_ATTACHMENT9: GLenum = 0x8CE9;
pub const GL_COLOR_ATTACHMENT10: GLenum = 0x8CEA;
pub const GL_COLOR_ATTACHMENT11: GLenum = 0x8CEB;
pub const GL_COLOR_ATTACHMENT12: GLenum = 0x8CEC;
pub const GL_COLOR_ATTACHMENT13: GLenum = 0x8CED;
pub const GL_COLOR_ATTACHMENT14: GLenum = 0x8CEE;
pub const GL_COLOR_ATTACHMENT15: GLenum = 0x8CEF;
pub const GL_COLOR_ATTACHMENT16: GLenum = 0x8CF0;
pub const GL_COLOR_ATTACHMENT17: GLenum = 0x8CF1;
pub const GL_COLOR_ATTACHMENT18: GLenum = 0x8CF2;
pub const GL_COLOR_ATTACHMENT19: GLenum = 0x8CF3;
pub const GL_COLOR_ATTACHMENT20: GLenum = 0x8CF4;
pub const GL_COLOR_ATTACHMENT21: GLenum = 0x8CF5;
pub const GL_COLOR_ATTACHMENT22: GLenum = 0x8CF6;
pub const GL_COLOR_ATTACHMENT23: GLenum = 0x8CF7;
pub const GL_COLOR_ATTACHMENT24: GLenum = 0x8CF8;
pub const GL_COLOR_ATTACHMENT25: GLenum = 0x8CF9;
pub const GL_COLOR_ATTACHMENT26: GLenum = 0x8CFA;
pub const GL_COLOR_ATTACHMENT27: GLenum = 0x8CFB;
pub const GL_COLOR_ATTACHMENT28: GLenum = 0x8CFC;
pub const GL_COLOR_ATTACHMENT29: GLenum = 0x8CFD;
pub const GL_COLOR_ATTACHMENT30: GLenum = 0x8CFE;
pub const GL_COLOR_ATTACHMENT31: GLenum = 0x8CFF;
pub const GL_DEPTH_ATTACHMENT: GLenum = 0x8D00;
pub const GL_STENCIL_ATTACHMENT: GLenum = 0x8D20;
pub const GL_FRAMEBUFFER: GLenum = 0x8D40;
pub const GL_RENDERBUFFER: GLenum = 0x8D41;
pub const GL_RENDERBUFFER_WIDTH: GLenum = 0x8D42;
pub const GL_RENDERBUFFER_HEIGHT: GLenum = 0x8D43;
pub const GL_RENDERBUFFER_INTERNAL_FORMAT: GLenum = 0x8D44;
pub const GL_STENCIL_INDEX1: GLenum = 0x8D46;
pub const GL_STENCIL_INDEX4: GLenum = 0x8D47;
pub const GL_STENCIL_INDEX8: GLenum = 0x8D48;
pub const GL_STENCIL_INDEX16: GLenum = 0x8D49;
pub const GL_RENDERBUFFER_RED_SIZE: GLenum = 0x8D50;
pub const GL_RENDERBUFFER_GREEN_SIZE: GLenum = 0x8D51;
pub const GL_RENDERBUFFER_BLUE_SIZE: GLenum = 0x8D52;
pub const GL_RENDERBUFFER_ALPHA_SIZE: GLenum = 0x8D53;
pub const GL_RENDERBUFFER_DEPTH_SIZE: GLenum = 0x8D54;
pub const GL_RENDERBUFFER_STENCIL_SIZE: GLenum = 0x8D55;
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: GLenum = 0x8D56;
pub const GL_MAX_SAMPLES: GLenum = 0x8D57;

// Vertex array objects
pub const GL_VERTEX_ARRAY_BINDING: GLenum = 0x85B5;

// Half float
pub const GL_HALF_FLOAT: GLenum = 0x140B;

// Map buffer range
pub const GL_MAP_READ_BIT: GLenum = 0x0001;
pub const GL_MAP_WRITE_BIT: GLenum = 0x0002;
pub const GL_MAP_INVALIDATE_RANGE_BIT: GLenum = 0x0004;
pub const GL_MAP_INVALIDATE_BUFFER_BIT: GLenum = 0x0008;
pub const GL_MAP_FLUSH_EXPLICIT_BIT: GLenum = 0x0010;
pub const GL_MAP_UNSYNCHRONIZED_BIT: GLenum = 0x0020;

// Texture formats
pub const GL_COMPRESSED_RED_RGTC1: GLenum = 0x8DBB;
pub const GL_COMPRESSED_SIGNED_RED_RGTC1: GLenum = 0x8DBC;
pub const GL_COMPRESSED_RG_RGTC2: GLenum = 0x8DBD;
pub const GL_COMPRESSED_SIGNED_RG_RGTC2: GLenum = 0x8DBE;
pub const GL_RG: GLenum = 0x8227;
pub const GL_RG_INTEGER: GLenum = 0x8228;
pub const GL_R8: GLenum = 0x8229;
pub const GL_R16: GLenum = 0x822A;
pub const GL_RG8: GLenum = 0x822B;
pub const GL_RG16: GLenum = 0x822C;
pub const GL_R16F: GLenum = 0x822D;
pub const GL_R32F: GLenum = 0x822E;
pub const GL_RG16F: GLenum = 0x822F;
pub const GL_RG32F: GLenum = 0x8230;
pub const GL_R8I: GLenum = 0x8231;
pub const GL_R8UI: GLenum = 0x8232;
pub const GL_R16I: GLenum = 0x8233;
pub const GL_R16UI: GLenum = 0x8234;
pub const GL_R32I: GLenum = 0x8235;
pub const GL_R32UI: GLenum = 0x8236;
pub const GL_RG8I: GLenum = 0x8237;
pub const GL_RG8UI: GLenum = 0x8238;
pub const GL_RG16I: GLenum = 0x8239;
pub const GL_RG16UI: GLenum = 0x823A;
pub const GL_RG32I: GLenum = 0x823B;
pub const GL_RG32UI: GLenum = 0x823C;

// Integer textures
pub const GL_RED_INTEGER: GLenum = 0x8D94;
pub const GL_GREEN_INTEGER: GLenum = 0x8D95;
pub const GL_BLUE_INTEGER: GLenum = 0x8D96;
pub const GL_RGB_INTEGER: GLenum = 0x8D98;
pub const GL_RGBA_INTEGER: GLenum = 0x8D99;
pub const GL_BGR_INTEGER: GLenum = 0x8D9A;
pub const GL_BGRA_INTEGER: GLenum = 0x8D9B;

// Float textures
pub const GL_RGBA32F: GLenum = 0x8814;
pub const GL_RGB32F: GLenum = 0x8815;
pub const GL_RGBA16F: GLenum = 0x881A;
pub const GL_RGB16F: GLenum = 0x881B;

// Texture integer
pub const GL_RGBA32UI: GLenum = 0x8D70;
pub const GL_RGB32UI: GLenum = 0x8D71;
pub const GL_RGBA16UI: GLenum = 0x8D76;
pub const GL_RGB16UI: GLenum = 0x8D77;
pub const GL_RGBA8UI: GLenum = 0x8D7C;
pub const GL_RGB8UI: GLenum = 0x8D7D;
pub const GL_RGBA32I: GLenum = 0x8D82;
pub const GL_RGB32I: GLenum = 0x8D83;
pub const GL_RGBA16I: GLenum = 0x8D88;
pub const GL_RGB16I: GLenum = 0x8D89;
pub const GL_RGBA8I: GLenum = 0x8D8E;
pub const GL_RGB8I: GLenum = 0x8D8F;

// Clamp vertex color
pub const GL_CLAMP_READ_COLOR: GLenum = 0x891C;
pub const GL_FIXED_ONLY: GLenum = 0x891D;

// Texture array
pub const GL_TEXTURE_1D_ARRAY: GLenum = 0x8C18;
pub const GL_PROXY_TEXTURE_1D_ARRAY: GLenum = 0x8C19;
pub const GL_TEXTURE_2D_ARRAY: GLenum = 0x8C1A;
pub const GL_PROXY_TEXTURE_2D_ARRAY: GLenum = 0x8C1B;
pub const GL_TEXTURE_BINDING_1D_ARRAY: GLenum = 0x8C1C;
pub const GL_TEXTURE_BINDING_2D_ARRAY: GLenum = 0x8C1D;
pub const GL_MAX_ARRAY_TEXTURE_LAYERS: GLenum = 0x88FF;

// Transform feedback
pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: GLenum = 0x8C76;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: GLenum = 0x8C7F;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: GLenum = 0x8C80;
pub const GL_TRANSFORM_FEEDBACK_VARYINGS: GLenum = 0x8C83;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: GLenum = 0x8C84;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: GLenum = 0x8C85;
pub const GL_PRIMITIVES_GENERATED: GLenum = 0x8C87;
pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: GLenum = 0x8C88;
pub const GL_RASTERIZER_DISCARD: GLenum = 0x8C89;
pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: GLenum = 0x8C8A;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: GLenum = 0x8C8B;
pub const GL_INTERLEAVED_ATTRIBS: GLenum = 0x8C8C;
pub const GL_SEPARATE_ATTRIBS: GLenum = 0x8C8D;
pub const GL_TRANSFORM_FEEDBACK_BUFFER: GLenum = 0x8C8E;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: GLenum = 0x8C8F;

// Conditional render
pub const GL_QUERY_WAIT: GLenum = 0x8E13;
pub const GL_QUERY_NO_WAIT: GLenum = 0x8E14;
pub const GL_QUERY_BY_REGION_WAIT: GLenum = 0x8E15;
pub const GL_QUERY_BY_REGION_NO_WAIT: GLenum = 0x8E16;

// Clip distance
pub const GL_CLIP_DISTANCE0: GLenum = 0x3000;
pub const GL_CLIP_DISTANCE1: GLenum = 0x3001;
pub const GL_CLIP_DISTANCE2: GLenum = 0x3002;
pub const GL_CLIP_DISTANCE3: GLenum = 0x3003;
pub const GL_CLIP_DISTANCE4: GLenum = 0x3004;
pub const GL_CLIP_DISTANCE5: GLenum = 0x3005;
pub const GL_CLIP_DISTANCE6: GLenum = 0x3006;
pub const GL_CLIP_DISTANCE7: GLenum = 0x3007;
pub const GL_MAX_CLIP_DISTANCES: GLenum = 0x0D32;

// Misc
pub const GL_MAJOR_VERSION: GLenum = 0x821B;
pub const GL_MINOR_VERSION: GLenum = 0x821C;
pub const GL_NUM_EXTENSIONS: GLenum = 0x821D;
pub const GL_CONTEXT_FLAGS: GLenum = 0x821E;
pub const GL_COMPARE_REF_TO_TEXTURE: GLenum = 0x884E;

// ============================================================================
// OpenGL 3.1 Constants
// ============================================================================

// Uniform buffer objects
pub const GL_UNIFORM_BUFFER: GLenum = 0x8A11;
pub const GL_UNIFORM_BUFFER_BINDING: GLenum = 0x8A28;
pub const GL_UNIFORM_BUFFER_START: GLenum = 0x8A29;
pub const GL_UNIFORM_BUFFER_SIZE: GLenum = 0x8A2A;
pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: GLenum = 0x8A2B;
pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS: GLenum = 0x8A2C;
pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: GLenum = 0x8A2D;
pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: GLenum = 0x8A2E;
pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: GLenum = 0x8A2F;
pub const GL_MAX_UNIFORM_BLOCK_SIZE: GLenum = 0x8A30;
pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: GLenum = 0x8A31;
pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8A32;
pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: GLenum = 0x8A33;
pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x8A34;
pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: GLenum = 0x8A35;
pub const GL_ACTIVE_UNIFORM_BLOCKS: GLenum = 0x8A36;
pub const GL_UNIFORM_TYPE: GLenum = 0x8A37;
pub const GL_UNIFORM_SIZE: GLenum = 0x8A38;
pub const GL_UNIFORM_NAME_LENGTH: GLenum = 0x8A39;
pub const GL_UNIFORM_BLOCK_INDEX: GLenum = 0x8A3A;
pub const GL_UNIFORM_OFFSET: GLenum = 0x8A3B;
pub const GL_UNIFORM_ARRAY_STRIDE: GLenum = 0x8A3C;
pub const GL_UNIFORM_MATRIX_STRIDE: GLenum = 0x8A3D;
pub const GL_UNIFORM_IS_ROW_MAJOR: GLenum = 0x8A3E;
pub const GL_UNIFORM_BLOCK_BINDING: GLenum = 0x8A3F;
pub const GL_UNIFORM_BLOCK_DATA_SIZE: GLenum = 0x8A40;
pub const GL_UNIFORM_BLOCK_NAME_LENGTH: GLenum = 0x8A41;
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: GLenum = 0x8A42;
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: GLenum = 0x8A43;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x8A44;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x8A45;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x8A46;
pub const GL_INVALID_INDEX: GLenum = 0xFFFFFFFF;

// Copy buffer
pub const GL_COPY_READ_BUFFER: GLenum = 0x8F36;
pub const GL_COPY_WRITE_BUFFER: GLenum = 0x8F37;

// Primitive restart
pub const GL_PRIMITIVE_RESTART: GLenum = 0x8F9D;
pub const GL_PRIMITIVE_RESTART_INDEX: GLenum = 0x8F9E;

// Texture buffer
pub const GL_TEXTURE_RECTANGLE: GLenum = 0x84F5;
pub const GL_TEXTURE_BINDING_RECTANGLE: GLenum = 0x84F6;
pub const GL_PROXY_TEXTURE_RECTANGLE: GLenum = 0x84F7;
pub const GL_MAX_RECTANGLE_TEXTURE_SIZE: GLenum = 0x84F8;
pub const GL_SAMPLER_2D_RECT: GLenum = 0x8B63;
pub const GL_SAMPLER_2D_RECT_SHADOW: GLenum = 0x8B64;
pub const GL_TEXTURE_BUFFER: GLenum = 0x8C2A;
pub const GL_MAX_TEXTURE_BUFFER_SIZE: GLenum = 0x8C2B;
pub const GL_TEXTURE_BINDING_BUFFER: GLenum = 0x8C2C;
pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING: GLenum = 0x8C2D;

// Snorm textures
pub const GL_RED_SNORM: GLenum = 0x8F90;
pub const GL_RG_SNORM: GLenum = 0x8F91;
pub const GL_RGB_SNORM: GLenum = 0x8F92;
pub const GL_RGBA_SNORM: GLenum = 0x8F93;
pub const GL_R8_SNORM: GLenum = 0x8F94;
pub const GL_RG8_SNORM: GLenum = 0x8F95;
pub const GL_RGB8_SNORM: GLenum = 0x8F96;
pub const GL_RGBA8_SNORM: GLenum = 0x8F97;
pub const GL_R16_SNORM: GLenum = 0x8F98;
pub const GL_RG16_SNORM: GLenum = 0x8F99;
pub const GL_RGB16_SNORM: GLenum = 0x8F9A;
pub const GL_RGBA16_SNORM: GLenum = 0x8F9B;
pub const GL_SIGNED_NORMALIZED: GLenum = 0x8F9C;

// ============================================================================
// OpenGL 3.2 Constants
// ============================================================================

// Geometry shader
pub const GL_GEOMETRY_SHADER: GLenum = 0x8DD9;
pub const GL_GEOMETRY_VERTICES_OUT: GLenum = 0x8916;
pub const GL_GEOMETRY_INPUT_TYPE: GLenum = 0x8917;
pub const GL_GEOMETRY_OUTPUT_TYPE: GLenum = 0x8918;
pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: GLenum = 0x8C29;
pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS: GLenum = 0x8DDF;
pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES: GLenum = 0x8DE0;
pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8DE1;
pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: GLenum = 0x9122;
pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS: GLenum = 0x9123;
pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS: GLenum = 0x9124;
pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: GLenum = 0x9125;
pub const GL_LINES_ADJACENCY: GLenum = 0x000A;
pub const GL_LINE_STRIP_ADJACENCY: GLenum = 0x000B;
pub const GL_TRIANGLES_ADJACENCY: GLenum = 0x000C;
pub const GL_TRIANGLE_STRIP_ADJACENCY: GLenum = 0x000D;
pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: GLenum = 0x8DA8;
pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED: GLenum = 0x8DA7;
pub const GL_PROGRAM_POINT_SIZE: GLenum = 0x8642;

// Sync objects
pub const GL_MAX_SERVER_WAIT_TIMEOUT: GLenum = 0x9111;
pub const GL_OBJECT_TYPE: GLenum = 0x9112;
pub const GL_SYNC_CONDITION: GLenum = 0x9113;
pub const GL_SYNC_STATUS: GLenum = 0x9114;
pub const GL_SYNC_FLAGS: GLenum = 0x9115;
pub const GL_SYNC_FENCE: GLenum = 0x9116;
pub const GL_SYNC_GPU_COMMANDS_COMPLETE: GLenum = 0x9117;
pub const GL_UNSIGNALED: GLenum = 0x9118;
pub const GL_SIGNALED: GLenum = 0x9119;
pub const GL_ALREADY_SIGNALED: GLenum = 0x911A;
pub const GL_TIMEOUT_EXPIRED: GLenum = 0x911B;
pub const GL_CONDITION_SATISFIED: GLenum = 0x911C;
pub const GL_WAIT_FAILED: GLenum = 0x911D;
pub const GL_SYNC_FLUSH_COMMANDS_BIT: GLenum = 0x00000001;
pub const GL_TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;

// Multisample textures
pub const GL_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9100;
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE: GLenum = 0x9101;
pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9102;
pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9103;
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE: GLenum = 0x9104;
pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: GLenum = 0x9105;
pub const GL_TEXTURE_SAMPLES: GLenum = 0x9106;
pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9107;
pub const GL_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9108;
pub const GL_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x9109;
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: GLenum = 0x910A;
pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910B;
pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910C;
pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: GLenum = 0x910D;
pub const GL_MAX_COLOR_TEXTURE_SAMPLES: GLenum = 0x910E;
pub const GL_MAX_DEPTH_TEXTURE_SAMPLES: GLenum = 0x910F;
pub const GL_MAX_INTEGER_SAMPLES: GLenum = 0x9110;

// Depth clamp
pub const GL_DEPTH_CLAMP: GLenum = 0x864F;

// Provoking vertex
pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: GLenum = 0x8E4C;
pub const GL_FIRST_VERTEX_CONVENTION: GLenum = 0x8E4D;
pub const GL_LAST_VERTEX_CONVENTION: GLenum = 0x8E4E;
pub const GL_PROVOKING_VERTEX: GLenum = 0x8E4F;

// Seamless cube map
pub const GL_TEXTURE_CUBE_MAP_SEAMLESS: GLenum = 0x884F;

// Sample mask
pub const GL_SAMPLE_POSITION: GLenum = 0x8E50;
pub const GL_SAMPLE_MASK: GLenum = 0x8E51;
pub const GL_SAMPLE_MASK_VALUE: GLenum = 0x8E52;
pub const GL_MAX_SAMPLE_MASK_WORDS: GLenum = 0x8E59;

// Context profile
pub const GL_CONTEXT_CORE_PROFILE_BIT: GLenum = 0x00000001;
pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT: GLenum = 0x00000002;
pub const GL_CONTEXT_PROFILE_MASK: GLenum = 0x9126;

// ============================================================================
// OpenGL 3.3 Constants
// ============================================================================

// Sampler objects
pub const GL_SAMPLER_BINDING: GLenum = 0x8919;

// Vertex type 2_10_10_10_REV
pub const GL_INT_2_10_10_10_REV: GLenum = 0x8D9F;

// RGB10_A2UI
pub const GL_RGB10_A2UI: GLenum = 0x906F;

// Timestamp queries
pub const GL_TIMESTAMP: GLenum = 0x8E28;
pub const GL_TIME_ELAPSED: GLenum = 0x88BF;

// Texture swizzle
pub const GL_TEXTURE_SWIZZLE_R: GLenum = 0x8E42;
pub const GL_TEXTURE_SWIZZLE_G: GLenum = 0x8E43;
pub const GL_TEXTURE_SWIZZLE_B: GLenum = 0x8E44;
pub const GL_TEXTURE_SWIZZLE_A: GLenum = 0x8E45;
pub const GL_TEXTURE_SWIZZLE_RGBA: GLenum = 0x8E46;

// Vertex attrib divisor
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: GLenum = 0x88FE;

// Blend func extended
pub const GL_SRC1_COLOR: GLenum = 0x88F9;
pub const GL_ONE_MINUS_SRC1_COLOR: GLenum = 0x88FA;
pub const GL_ONE_MINUS_SRC1_ALPHA: GLenum = 0x88FB;
pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS: GLenum = 0x88FC;

// Occlusion query
pub const GL_ANY_SAMPLES_PASSED: GLenum = 0x8C2F;
