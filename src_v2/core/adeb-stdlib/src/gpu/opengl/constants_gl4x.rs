//! OpenGL 4.0 - 4.6 Constants
//! Based on Khronos canonical specifications
//! https://registry.khronos.org/OpenGL/

use super::types::GLenum;

// ============================================================================
// OpenGL 4.0 Constants
// ============================================================================

// Tessellation shaders
pub const GL_PATCHES: GLenum = 0x000E;
pub const GL_PATCH_VERTICES: GLenum = 0x8E72;
pub const GL_PATCH_DEFAULT_INNER_LEVEL: GLenum = 0x8E73;
pub const GL_PATCH_DEFAULT_OUTER_LEVEL: GLenum = 0x8E74;
pub const GL_TESS_CONTROL_OUTPUT_VERTICES: GLenum = 0x8E75;
pub const GL_TESS_GEN_MODE: GLenum = 0x8E76;
pub const GL_TESS_GEN_SPACING: GLenum = 0x8E77;
pub const GL_TESS_GEN_VERTEX_ORDER: GLenum = 0x8E78;
pub const GL_TESS_GEN_POINT_MODE: GLenum = 0x8E79;
pub const GL_ISOLINES: GLenum = 0x8E7A;
pub const GL_FRACTIONAL_ODD: GLenum = 0x8E7B;
pub const GL_FRACTIONAL_EVEN: GLenum = 0x8E7C;
pub const GL_MAX_PATCH_VERTICES: GLenum = 0x8E7D;
pub const GL_MAX_TESS_GEN_LEVEL: GLenum = 0x8E7E;
pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E7F;
pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E80;
pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: GLenum = 0x8E81;
pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: GLenum = 0x8E82;
pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS: GLenum = 0x8E83;
pub const GL_MAX_TESS_PATCH_COMPONENTS: GLenum = 0x8E84;
pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: GLenum = 0x8E85;
pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: GLenum = 0x8E86;
pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS: GLenum = 0x8E89;
pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS: GLenum = 0x8E8A;
pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS: GLenum = 0x886C;
pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS: GLenum = 0x886D;
pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: GLenum = 0x8E1E;
pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: GLenum = 0x8E1F;
pub const GL_TESS_EVALUATION_SHADER: GLenum = 0x8E87;
pub const GL_TESS_CONTROL_SHADER: GLenum = 0x8E88;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x84F0;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x84F1;

// Subroutines
pub const GL_ACTIVE_SUBROUTINES: GLenum = 0x8DE5;
pub const GL_ACTIVE_SUBROUTINE_UNIFORMS: GLenum = 0x8DE6;
pub const GL_ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8E47;
pub const GL_ACTIVE_SUBROUTINE_MAX_LENGTH: GLenum = 0x8E48;
pub const GL_ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: GLenum = 0x8E49;
pub const GL_MAX_SUBROUTINES: GLenum = 0x8DE7;
pub const GL_MAX_SUBROUTINE_UNIFORM_LOCATIONS: GLenum = 0x8DE8;
pub const GL_NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x8E4A;
pub const GL_COMPATIBLE_SUBROUTINES: GLenum = 0x8E4B;

// Transform feedback
pub const GL_TRANSFORM_FEEDBACK: GLenum = 0x8E22;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED: GLenum = 0x8E23;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE: GLenum = 0x8E24;
pub const GL_TRANSFORM_FEEDBACK_BINDING: GLenum = 0x8E25;

// Double precision
pub const GL_DOUBLE_VEC2: GLenum = 0x8FFC;
pub const GL_DOUBLE_VEC3: GLenum = 0x8FFD;
pub const GL_DOUBLE_VEC4: GLenum = 0x8FFE;
pub const GL_DOUBLE_MAT2: GLenum = 0x8F46;
pub const GL_DOUBLE_MAT3: GLenum = 0x8F47;
pub const GL_DOUBLE_MAT4: GLenum = 0x8F48;
pub const GL_DOUBLE_MAT2x3: GLenum = 0x8F49;
pub const GL_DOUBLE_MAT2x4: GLenum = 0x8F4A;
pub const GL_DOUBLE_MAT3x2: GLenum = 0x8F4B;
pub const GL_DOUBLE_MAT3x4: GLenum = 0x8F4C;
pub const GL_DOUBLE_MAT4x2: GLenum = 0x8F4D;
pub const GL_DOUBLE_MAT4x3: GLenum = 0x8F4E;

// Sample shading
pub const GL_SAMPLE_SHADING: GLenum = 0x8C36;
pub const GL_MIN_SAMPLE_SHADING_VALUE: GLenum = 0x8C37;

// Texture cube map array
pub const GL_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x9009;
pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY: GLenum = 0x900A;
pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY: GLenum = 0x900B;
pub const GL_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900C;
pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW: GLenum = 0x900D;
pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900E;
pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: GLenum = 0x900F;

// Min/max blending
pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5E;
pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET: GLenum = 0x8E5F;

// Draw indirect
pub const GL_DRAW_INDIRECT_BUFFER: GLenum = 0x8F3F;
pub const GL_DRAW_INDIRECT_BUFFER_BINDING: GLenum = 0x8F43;

// ============================================================================
// OpenGL 4.1 Constants
// ============================================================================

// Program binary
pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: GLenum = 0x8257;
pub const GL_PROGRAM_BINARY_LENGTH: GLenum = 0x8741;
pub const GL_NUM_PROGRAM_BINARY_FORMATS: GLenum = 0x87FE;
pub const GL_PROGRAM_BINARY_FORMATS: GLenum = 0x87FF;

// Separate shader objects
pub const GL_VERTEX_SHADER_BIT: GLenum = 0x00000001;
pub const GL_FRAGMENT_SHADER_BIT: GLenum = 0x00000002;
pub const GL_GEOMETRY_SHADER_BIT: GLenum = 0x00000004;
pub const GL_TESS_CONTROL_SHADER_BIT: GLenum = 0x00000008;
pub const GL_TESS_EVALUATION_SHADER_BIT: GLenum = 0x00000010;
pub const GL_ALL_SHADER_BITS: GLenum = 0xFFFFFFFF;
pub const GL_PROGRAM_SEPARABLE: GLenum = 0x8258;
pub const GL_ACTIVE_PROGRAM: GLenum = 0x8259;
pub const GL_PROGRAM_PIPELINE_BINDING: GLenum = 0x825A;

// Viewport arrays
pub const GL_MAX_VIEWPORTS: GLenum = 0x825B;
pub const GL_VIEWPORT_SUBPIXEL_BITS: GLenum = 0x825C;
pub const GL_VIEWPORT_BOUNDS_RANGE: GLenum = 0x825D;
pub const GL_LAYER_PROVOKING_VERTEX: GLenum = 0x825E;
pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX: GLenum = 0x825F;
pub const GL_UNDEFINED_VERTEX: GLenum = 0x8260;

// Shader precision
pub const GL_FIXED: GLenum = 0x140C;
pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: GLenum = 0x8B9A;
pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: GLenum = 0x8B9B;
pub const GL_LOW_FLOAT: GLenum = 0x8DF0;
pub const GL_MEDIUM_FLOAT: GLenum = 0x8DF1;
pub const GL_HIGH_FLOAT: GLenum = 0x8DF2;
pub const GL_LOW_INT: GLenum = 0x8DF3;
pub const GL_MEDIUM_INT: GLenum = 0x8DF4;
pub const GL_HIGH_INT: GLenum = 0x8DF5;
pub const GL_SHADER_COMPILER: GLenum = 0x8DFA;
pub const GL_SHADER_BINARY_FORMATS: GLenum = 0x8DF8;
pub const GL_NUM_SHADER_BINARY_FORMATS: GLenum = 0x8DF9;
pub const GL_MAX_VERTEX_UNIFORM_VECTORS: GLenum = 0x8DFB;
pub const GL_MAX_VARYING_VECTORS: GLenum = 0x8DFC;
pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: GLenum = 0x8DFD;

// ============================================================================
// OpenGL 4.2 Constants
// ============================================================================

// Atomic counters
pub const GL_ATOMIC_COUNTER_BUFFER: GLenum = 0x92C0;
pub const GL_ATOMIC_COUNTER_BUFFER_BINDING: GLenum = 0x92C1;
pub const GL_ATOMIC_COUNTER_BUFFER_START: GLenum = 0x92C2;
pub const GL_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92C3;
pub const GL_ATOMIC_COUNTER_BUFFER_DATA_SIZE: GLenum = 0x92C4;
pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: GLenum = 0x92C5;
pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: GLenum = 0x92C6;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x92C7;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x92C8;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x92C9;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x92CA;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x92CB;
pub const GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CC;
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CD;
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CE;
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92CF;
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D0;
pub const GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D1;
pub const GL_MAX_VERTEX_ATOMIC_COUNTERS: GLenum = 0x92D2;
pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS: GLenum = 0x92D3;
pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS: GLenum = 0x92D4;
pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS: GLenum = 0x92D5;
pub const GL_MAX_FRAGMENT_ATOMIC_COUNTERS: GLenum = 0x92D6;
pub const GL_MAX_COMBINED_ATOMIC_COUNTERS: GLenum = 0x92D7;
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE: GLenum = 0x92D8;
pub const GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: GLenum = 0x92DC;
pub const GL_ACTIVE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x92D9;
pub const GL_UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x92DA;
pub const GL_UNSIGNED_INT_ATOMIC_COUNTER: GLenum = 0x92DB;

// Image load/store
pub const GL_IMAGE_BINDING_NAME: GLenum = 0x8F3A;
pub const GL_IMAGE_BINDING_LEVEL: GLenum = 0x8F3B;
pub const GL_IMAGE_BINDING_LAYERED: GLenum = 0x8F3C;
pub const GL_IMAGE_BINDING_LAYER: GLenum = 0x8F3D;
pub const GL_IMAGE_BINDING_ACCESS: GLenum = 0x8F3E;
pub const GL_IMAGE_1D: GLenum = 0x904C;
pub const GL_IMAGE_2D: GLenum = 0x904D;
pub const GL_IMAGE_3D: GLenum = 0x904E;
pub const GL_IMAGE_2D_RECT: GLenum = 0x904F;
pub const GL_IMAGE_CUBE: GLenum = 0x9050;
pub const GL_IMAGE_BUFFER: GLenum = 0x9051;
pub const GL_IMAGE_1D_ARRAY: GLenum = 0x9052;
pub const GL_IMAGE_2D_ARRAY: GLenum = 0x9053;
pub const GL_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x9054;
pub const GL_IMAGE_2D_MULTISAMPLE: GLenum = 0x9055;
pub const GL_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9056;
pub const GL_INT_IMAGE_1D: GLenum = 0x9057;
pub const GL_INT_IMAGE_2D: GLenum = 0x9058;
pub const GL_INT_IMAGE_3D: GLenum = 0x9059;
pub const GL_INT_IMAGE_2D_RECT: GLenum = 0x905A;
pub const GL_INT_IMAGE_CUBE: GLenum = 0x905B;
pub const GL_INT_IMAGE_BUFFER: GLenum = 0x905C;
pub const GL_INT_IMAGE_1D_ARRAY: GLenum = 0x905D;
pub const GL_INT_IMAGE_2D_ARRAY: GLenum = 0x905E;
pub const GL_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x905F;
pub const GL_INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x9060;
pub const GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x9061;
pub const GL_UNSIGNED_INT_IMAGE_1D: GLenum = 0x9062;
pub const GL_UNSIGNED_INT_IMAGE_2D: GLenum = 0x9063;
pub const GL_UNSIGNED_INT_IMAGE_3D: GLenum = 0x9064;
pub const GL_UNSIGNED_INT_IMAGE_2D_RECT: GLenum = 0x9065;
pub const GL_UNSIGNED_INT_IMAGE_CUBE: GLenum = 0x9066;
pub const GL_UNSIGNED_INT_IMAGE_BUFFER: GLenum = 0x9067;
pub const GL_UNSIGNED_INT_IMAGE_1D_ARRAY: GLenum = 0x9068;
pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY: GLenum = 0x9069;
pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: GLenum = 0x906A;
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: GLenum = 0x906B;
pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: GLenum = 0x906C;
pub const GL_MAX_IMAGE_UNITS: GLenum = 0x8F38;
pub const GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: GLenum = 0x8F39;
pub const GL_IMAGE_BINDING_FORMAT: GLenum = 0x906E;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_TYPE: GLenum = 0x90C7;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: GLenum = 0x90C8;
pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: GLenum = 0x90C9;
pub const GL_MAX_VERTEX_IMAGE_UNIFORMS: GLenum = 0x90CA;
pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS: GLenum = 0x90CB;
pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS: GLenum = 0x90CC;
pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS: GLenum = 0x90CD;
pub const GL_MAX_FRAGMENT_IMAGE_UNIFORMS: GLenum = 0x90CE;
pub const GL_MAX_COMBINED_IMAGE_UNIFORMS: GLenum = 0x90CF;

// Texture storage
pub const GL_TEXTURE_IMMUTABLE_FORMAT: GLenum = 0x912F;

// Compressed texture formats
pub const GL_COMPRESSED_RGBA_BPTC_UNORM: GLenum = 0x8E8C;
pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM: GLenum = 0x8E8D;
pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT: GLenum = 0x8E8E;
pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: GLenum = 0x8E8F;

// Unpack compressed block
pub const GL_UNPACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x9127;
pub const GL_UNPACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x9128;
pub const GL_UNPACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x9129;
pub const GL_UNPACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912A;
pub const GL_PACK_COMPRESSED_BLOCK_WIDTH: GLenum = 0x912B;
pub const GL_PACK_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x912C;
pub const GL_PACK_COMPRESSED_BLOCK_DEPTH: GLenum = 0x912D;
pub const GL_PACK_COMPRESSED_BLOCK_SIZE: GLenum = 0x912E;

// Memory barriers
pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT: GLenum = 0x00000001;
pub const GL_ELEMENT_ARRAY_BARRIER_BIT: GLenum = 0x00000002;
pub const GL_UNIFORM_BARRIER_BIT: GLenum = 0x00000004;
pub const GL_TEXTURE_FETCH_BARRIER_BIT: GLenum = 0x00000008;
pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT: GLenum = 0x00000020;
pub const GL_COMMAND_BARRIER_BIT: GLenum = 0x00000040;
pub const GL_PIXEL_BUFFER_BARRIER_BIT: GLenum = 0x00000080;
pub const GL_TEXTURE_UPDATE_BARRIER_BIT: GLenum = 0x00000100;
pub const GL_BUFFER_UPDATE_BARRIER_BIT: GLenum = 0x00000200;
pub const GL_FRAMEBUFFER_BARRIER_BIT: GLenum = 0x00000400;
pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT: GLenum = 0x00000800;
pub const GL_ATOMIC_COUNTER_BARRIER_BIT: GLenum = 0x00001000;
pub const GL_ALL_BARRIER_BITS: GLenum = 0xFFFFFFFF;

// ============================================================================
// OpenGL 4.3 Constants
// ============================================================================

// Compute shaders
pub const GL_COMPUTE_SHADER: GLenum = 0x91B9;
pub const GL_MAX_COMPUTE_UNIFORM_BLOCKS: GLenum = 0x91BB;
pub const GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS: GLenum = 0x91BC;
pub const GL_MAX_COMPUTE_IMAGE_UNIFORMS: GLenum = 0x91BD;
pub const GL_MAX_COMPUTE_SHARED_MEMORY_SIZE: GLenum = 0x8262;
pub const GL_MAX_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8263;
pub const GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: GLenum = 0x8264;
pub const GL_MAX_COMPUTE_ATOMIC_COUNTERS: GLenum = 0x8265;
pub const GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: GLenum = 0x8266;
pub const GL_MAX_COMPUTE_WORK_GROUP_INVOCATIONS: GLenum = 0x90EB;
pub const GL_MAX_COMPUTE_WORK_GROUP_COUNT: GLenum = 0x91BE;
pub const GL_MAX_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x91BF;
pub const GL_COMPUTE_WORK_GROUP_SIZE: GLenum = 0x8267;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90EC;
pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x90ED;
pub const GL_DISPATCH_INDIRECT_BUFFER: GLenum = 0x90EE;
pub const GL_DISPATCH_INDIRECT_BUFFER_BINDING: GLenum = 0x90EF;
pub const GL_COMPUTE_SHADER_BIT: GLenum = 0x00000020;

// Shader storage buffer
pub const GL_SHADER_STORAGE_BUFFER: GLenum = 0x90D2;
pub const GL_SHADER_STORAGE_BUFFER_BINDING: GLenum = 0x90D3;
pub const GL_SHADER_STORAGE_BUFFER_START: GLenum = 0x90D4;
pub const GL_SHADER_STORAGE_BUFFER_SIZE: GLenum = 0x90D5;
pub const GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS: GLenum = 0x90D6;
pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: GLenum = 0x90D7;
pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: GLenum = 0x90D8;
pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: GLenum = 0x90D9;
pub const GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: GLenum = 0x90DA;
pub const GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS: GLenum = 0x90DB;
pub const GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS: GLenum = 0x90DC;
pub const GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS: GLenum = 0x90DD;
pub const GL_MAX_SHADER_STORAGE_BLOCK_SIZE: GLenum = 0x90DE;
pub const GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x90DF;
pub const GL_SHADER_STORAGE_BARRIER_BIT: GLenum = 0x00002000;
pub const GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES: GLenum = 0x8F39;

// Debug output
pub const GL_DEBUG_OUTPUT_SYNCHRONOUS: GLenum = 0x8242;
pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: GLenum = 0x8243;
pub const GL_DEBUG_CALLBACK_FUNCTION: GLenum = 0x8244;
pub const GL_DEBUG_CALLBACK_USER_PARAM: GLenum = 0x8245;
pub const GL_DEBUG_SOURCE_API: GLenum = 0x8246;
pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM: GLenum = 0x8247;
pub const GL_DEBUG_SOURCE_SHADER_COMPILER: GLenum = 0x8248;
pub const GL_DEBUG_SOURCE_THIRD_PARTY: GLenum = 0x8249;
pub const GL_DEBUG_SOURCE_APPLICATION: GLenum = 0x824A;
pub const GL_DEBUG_SOURCE_OTHER: GLenum = 0x824B;
pub const GL_DEBUG_TYPE_ERROR: GLenum = 0x824C;
pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR: GLenum = 0x824D;
pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR: GLenum = 0x824E;
pub const GL_DEBUG_TYPE_PORTABILITY: GLenum = 0x824F;
pub const GL_DEBUG_TYPE_PERFORMANCE: GLenum = 0x8250;
pub const GL_DEBUG_TYPE_OTHER: GLenum = 0x8251;
pub const GL_DEBUG_TYPE_MARKER: GLenum = 0x8268;
pub const GL_DEBUG_TYPE_PUSH_GROUP: GLenum = 0x8269;
pub const GL_DEBUG_TYPE_POP_GROUP: GLenum = 0x826A;
pub const GL_DEBUG_SEVERITY_NOTIFICATION: GLenum = 0x826B;
pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826C;
pub const GL_DEBUG_GROUP_STACK_DEPTH: GLenum = 0x826D;
pub const GL_BUFFER: GLenum = 0x82E0;
pub const GL_SHADER: GLenum = 0x82E1;
pub const GL_PROGRAM: GLenum = 0x82E2;
pub const GL_QUERY: GLenum = 0x82E3;
pub const GL_PROGRAM_PIPELINE: GLenum = 0x82E4;
pub const GL_SAMPLER: GLenum = 0x82E6;
pub const GL_MAX_LABEL_LENGTH: GLenum = 0x82E8;
pub const GL_DEBUG_SEVERITY_HIGH: GLenum = 0x9146;
pub const GL_DEBUG_SEVERITY_MEDIUM: GLenum = 0x9147;
pub const GL_DEBUG_SEVERITY_LOW: GLenum = 0x9148;
pub const GL_DEBUG_OUTPUT: GLenum = 0x92E0;
pub const GL_CONTEXT_FLAG_DEBUG_BIT: GLenum = 0x00000002;

// Explicit uniform location
pub const GL_MAX_UNIFORM_LOCATIONS: GLenum = 0x826E;

// Framebuffer no attachments
pub const GL_FRAMEBUFFER_DEFAULT_WIDTH: GLenum = 0x9310;
pub const GL_FRAMEBUFFER_DEFAULT_HEIGHT: GLenum = 0x9311;
pub const GL_FRAMEBUFFER_DEFAULT_LAYERS: GLenum = 0x9312;
pub const GL_FRAMEBUFFER_DEFAULT_SAMPLES: GLenum = 0x9313;
pub const GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: GLenum = 0x9314;
pub const GL_MAX_FRAMEBUFFER_WIDTH: GLenum = 0x9315;
pub const GL_MAX_FRAMEBUFFER_HEIGHT: GLenum = 0x9316;
pub const GL_MAX_FRAMEBUFFER_LAYERS: GLenum = 0x9317;
pub const GL_MAX_FRAMEBUFFER_SAMPLES: GLenum = 0x9318;

// Internalformat query2
pub const GL_INTERNALFORMAT_SUPPORTED: GLenum = 0x826F;
pub const GL_INTERNALFORMAT_PREFERRED: GLenum = 0x8270;
pub const GL_INTERNALFORMAT_RED_SIZE: GLenum = 0x8271;
pub const GL_INTERNALFORMAT_GREEN_SIZE: GLenum = 0x8272;
pub const GL_INTERNALFORMAT_BLUE_SIZE: GLenum = 0x8273;
pub const GL_INTERNALFORMAT_ALPHA_SIZE: GLenum = 0x8274;
pub const GL_INTERNALFORMAT_DEPTH_SIZE: GLenum = 0x8275;
pub const GL_INTERNALFORMAT_STENCIL_SIZE: GLenum = 0x8276;
pub const GL_INTERNALFORMAT_SHARED_SIZE: GLenum = 0x8277;
pub const GL_INTERNALFORMAT_RED_TYPE: GLenum = 0x8278;
pub const GL_INTERNALFORMAT_GREEN_TYPE: GLenum = 0x8279;
pub const GL_INTERNALFORMAT_BLUE_TYPE: GLenum = 0x827A;
pub const GL_INTERNALFORMAT_ALPHA_TYPE: GLenum = 0x827B;
pub const GL_INTERNALFORMAT_DEPTH_TYPE: GLenum = 0x827C;
pub const GL_INTERNALFORMAT_STENCIL_TYPE: GLenum = 0x827D;
pub const GL_MAX_WIDTH: GLenum = 0x827E;
pub const GL_MAX_HEIGHT: GLenum = 0x827F;
pub const GL_MAX_DEPTH: GLenum = 0x8280;
pub const GL_MAX_LAYERS: GLenum = 0x8281;
pub const GL_MAX_COMBINED_DIMENSIONS: GLenum = 0x8282;
pub const GL_COLOR_COMPONENTS: GLenum = 0x8283;
pub const GL_DEPTH_COMPONENTS: GLenum = 0x8284;
pub const GL_STENCIL_COMPONENTS: GLenum = 0x8285;
pub const GL_COLOR_RENDERABLE: GLenum = 0x8286;
pub const GL_DEPTH_RENDERABLE: GLenum = 0x8287;
pub const GL_STENCIL_RENDERABLE: GLenum = 0x8288;
pub const GL_FRAMEBUFFER_RENDERABLE: GLenum = 0x8289;
pub const GL_FRAMEBUFFER_RENDERABLE_LAYERED: GLenum = 0x828A;
pub const GL_FRAMEBUFFER_BLEND: GLenum = 0x828B;
pub const GL_READ_PIXELS: GLenum = 0x828C;
pub const GL_READ_PIXELS_FORMAT: GLenum = 0x828D;
pub const GL_READ_PIXELS_TYPE: GLenum = 0x828E;
pub const GL_TEXTURE_IMAGE_FORMAT: GLenum = 0x828F;
pub const GL_TEXTURE_IMAGE_TYPE: GLenum = 0x8290;
pub const GL_GET_TEXTURE_IMAGE_FORMAT: GLenum = 0x8291;
pub const GL_GET_TEXTURE_IMAGE_TYPE: GLenum = 0x8292;
pub const GL_MIPMAP: GLenum = 0x8293;
pub const GL_MANUAL_GENERATE_MIPMAP: GLenum = 0x8294;
pub const GL_AUTO_GENERATE_MIPMAP: GLenum = 0x8295;
pub const GL_COLOR_ENCODING: GLenum = 0x8296;
pub const GL_SRGB_READ: GLenum = 0x8297;
pub const GL_SRGB_WRITE: GLenum = 0x8298;
pub const GL_FILTER: GLenum = 0x829A;
pub const GL_VERTEX_TEXTURE: GLenum = 0x829B;
pub const GL_TESS_CONTROL_TEXTURE: GLenum = 0x829C;
pub const GL_TESS_EVALUATION_TEXTURE: GLenum = 0x829D;
pub const GL_GEOMETRY_TEXTURE: GLenum = 0x829E;
pub const GL_FRAGMENT_TEXTURE: GLenum = 0x829F;
pub const GL_COMPUTE_TEXTURE: GLenum = 0x82A0;
pub const GL_TEXTURE_SHADOW: GLenum = 0x82A1;
pub const GL_TEXTURE_GATHER: GLenum = 0x82A2;
pub const GL_TEXTURE_GATHER_SHADOW: GLenum = 0x82A3;
pub const GL_SHADER_IMAGE_LOAD: GLenum = 0x82A4;
pub const GL_SHADER_IMAGE_STORE: GLenum = 0x82A5;
pub const GL_SHADER_IMAGE_ATOMIC: GLenum = 0x82A6;
pub const GL_IMAGE_TEXEL_SIZE: GLenum = 0x82A7;
pub const GL_IMAGE_COMPATIBILITY_CLASS: GLenum = 0x82A8;
pub const GL_IMAGE_PIXEL_FORMAT: GLenum = 0x82A9;
pub const GL_IMAGE_PIXEL_TYPE: GLenum = 0x82AA;
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: GLenum = 0x82AC;
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: GLenum = 0x82AD;
pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: GLenum = 0x82AE;
pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: GLenum = 0x82AF;
pub const GL_TEXTURE_COMPRESSED_BLOCK_WIDTH: GLenum = 0x82B1;
pub const GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT: GLenum = 0x82B2;
pub const GL_TEXTURE_COMPRESSED_BLOCK_SIZE: GLenum = 0x82B3;
pub const GL_CLEAR_BUFFER: GLenum = 0x82B4;
pub const GL_TEXTURE_VIEW: GLenum = 0x82B5;
pub const GL_VIEW_COMPATIBILITY_CLASS: GLenum = 0x82B6;
pub const GL_FULL_SUPPORT: GLenum = 0x82B7;
pub const GL_CAVEAT_SUPPORT: GLenum = 0x82B8;
pub const GL_IMAGE_CLASS_4_X_32: GLenum = 0x82B9;
pub const GL_IMAGE_CLASS_2_X_32: GLenum = 0x82BA;
pub const GL_IMAGE_CLASS_1_X_32: GLenum = 0x82BB;
pub const GL_IMAGE_CLASS_4_X_16: GLenum = 0x82BC;
pub const GL_IMAGE_CLASS_2_X_16: GLenum = 0x82BD;
pub const GL_IMAGE_CLASS_1_X_16: GLenum = 0x82BE;
pub const GL_IMAGE_CLASS_4_X_8: GLenum = 0x82BF;
pub const GL_IMAGE_CLASS_2_X_8: GLenum = 0x82C0;
pub const GL_IMAGE_CLASS_1_X_8: GLenum = 0x82C1;
pub const GL_IMAGE_CLASS_11_11_10: GLenum = 0x82C2;
pub const GL_IMAGE_CLASS_10_10_10_2: GLenum = 0x82C3;
pub const GL_VIEW_CLASS_128_BITS: GLenum = 0x82C4;
pub const GL_VIEW_CLASS_96_BITS: GLenum = 0x82C5;
pub const GL_VIEW_CLASS_64_BITS: GLenum = 0x82C6;
pub const GL_VIEW_CLASS_48_BITS: GLenum = 0x82C7;
pub const GL_VIEW_CLASS_32_BITS: GLenum = 0x82C8;
pub const GL_VIEW_CLASS_24_BITS: GLenum = 0x82C9;
pub const GL_VIEW_CLASS_16_BITS: GLenum = 0x82CA;
pub const GL_VIEW_CLASS_8_BITS: GLenum = 0x82CB;
pub const GL_VIEW_CLASS_S3TC_DXT1_RGB: GLenum = 0x82CC;
pub const GL_VIEW_CLASS_S3TC_DXT1_RGBA: GLenum = 0x82CD;
pub const GL_VIEW_CLASS_S3TC_DXT3_RGBA: GLenum = 0x82CE;
pub const GL_VIEW_CLASS_S3TC_DXT5_RGBA: GLenum = 0x82CF;
pub const GL_VIEW_CLASS_RGTC1_RED: GLenum = 0x82D0;
pub const GL_VIEW_CLASS_RGTC2_RG: GLenum = 0x82D1;
pub const GL_VIEW_CLASS_BPTC_UNORM: GLenum = 0x82D2;
pub const GL_VIEW_CLASS_BPTC_FLOAT: GLenum = 0x82D3;

// Program interface query
pub const GL_UNIFORM: GLenum = 0x92E1;
pub const GL_UNIFORM_BLOCK: GLenum = 0x92E2;
pub const GL_PROGRAM_INPUT: GLenum = 0x92E3;
pub const GL_PROGRAM_OUTPUT: GLenum = 0x92E4;
pub const GL_BUFFER_VARIABLE: GLenum = 0x92E5;
pub const GL_SHADER_STORAGE_BLOCK: GLenum = 0x92E6;
pub const GL_VERTEX_SUBROUTINE: GLenum = 0x92E8;
pub const GL_TESS_CONTROL_SUBROUTINE: GLenum = 0x92E9;
pub const GL_TESS_EVALUATION_SUBROUTINE: GLenum = 0x92EA;
pub const GL_GEOMETRY_SUBROUTINE: GLenum = 0x92EB;
pub const GL_FRAGMENT_SUBROUTINE: GLenum = 0x92EC;
pub const GL_COMPUTE_SUBROUTINE: GLenum = 0x92ED;
pub const GL_VERTEX_SUBROUTINE_UNIFORM: GLenum = 0x92EE;
pub const GL_TESS_CONTROL_SUBROUTINE_UNIFORM: GLenum = 0x92EF;
pub const GL_TESS_EVALUATION_SUBROUTINE_UNIFORM: GLenum = 0x92F0;
pub const GL_GEOMETRY_SUBROUTINE_UNIFORM: GLenum = 0x92F1;
pub const GL_FRAGMENT_SUBROUTINE_UNIFORM: GLenum = 0x92F2;
pub const GL_COMPUTE_SUBROUTINE_UNIFORM: GLenum = 0x92F3;
pub const GL_TRANSFORM_FEEDBACK_VARYING: GLenum = 0x92F4;
pub const GL_ACTIVE_RESOURCES: GLenum = 0x92F5;
pub const GL_MAX_NAME_LENGTH: GLenum = 0x92F6;
pub const GL_MAX_NUM_ACTIVE_VARIABLES: GLenum = 0x92F7;
pub const GL_MAX_NUM_COMPATIBLE_SUBROUTINES: GLenum = 0x92F8;
pub const GL_NAME_LENGTH: GLenum = 0x92F9;
pub const GL_TYPE: GLenum = 0x92FA;
pub const GL_ARRAY_SIZE: GLenum = 0x92FB;
pub const GL_OFFSET: GLenum = 0x92FC;
pub const GL_BLOCK_INDEX: GLenum = 0x92FD;
pub const GL_ARRAY_STRIDE: GLenum = 0x92FE;
pub const GL_MATRIX_STRIDE: GLenum = 0x92FF;
pub const GL_IS_ROW_MAJOR: GLenum = 0x9300;
pub const GL_ATOMIC_COUNTER_BUFFER_INDEX: GLenum = 0x9301;
pub const GL_BUFFER_BINDING: GLenum = 0x9302;
pub const GL_BUFFER_DATA_SIZE: GLenum = 0x9303;
pub const GL_NUM_ACTIVE_VARIABLES: GLenum = 0x9304;
pub const GL_ACTIVE_VARIABLES: GLenum = 0x9305;
pub const GL_REFERENCED_BY_VERTEX_SHADER: GLenum = 0x9306;
pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER: GLenum = 0x9307;
pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER: GLenum = 0x9308;
pub const GL_REFERENCED_BY_GEOMETRY_SHADER: GLenum = 0x9309;
pub const GL_REFERENCED_BY_FRAGMENT_SHADER: GLenum = 0x930A;
pub const GL_REFERENCED_BY_COMPUTE_SHADER: GLenum = 0x930B;
pub const GL_TOP_LEVEL_ARRAY_SIZE: GLenum = 0x930C;
pub const GL_TOP_LEVEL_ARRAY_STRIDE: GLenum = 0x930D;
pub const GL_LOCATION: GLenum = 0x930E;
pub const GL_LOCATION_INDEX: GLenum = 0x930F;
pub const GL_IS_PER_PATCH: GLenum = 0x92E7;

// Texture buffer range
pub const GL_TEXTURE_BUFFER_OFFSET: GLenum = 0x919D;
pub const GL_TEXTURE_BUFFER_SIZE: GLenum = 0x919E;
pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT: GLenum = 0x919F;

// Texture view
pub const GL_TEXTURE_VIEW_MIN_LEVEL: GLenum = 0x82DB;
pub const GL_TEXTURE_VIEW_NUM_LEVELS: GLenum = 0x82DC;
pub const GL_TEXTURE_VIEW_MIN_LAYER: GLenum = 0x82DD;
pub const GL_TEXTURE_VIEW_NUM_LAYERS: GLenum = 0x82DE;
pub const GL_TEXTURE_IMMUTABLE_LEVELS: GLenum = 0x82DF;

// Vertex attrib binding
pub const GL_VERTEX_ATTRIB_BINDING: GLenum = 0x82D4;
pub const GL_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D5;
pub const GL_VERTEX_BINDING_DIVISOR: GLenum = 0x82D6;
pub const GL_VERTEX_BINDING_OFFSET: GLenum = 0x82D7;
pub const GL_VERTEX_BINDING_STRIDE: GLenum = 0x82D8;
pub const GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: GLenum = 0x82D9;
pub const GL_MAX_VERTEX_ATTRIB_BINDINGS: GLenum = 0x82DA;
pub const GL_VERTEX_BINDING_BUFFER: GLenum = 0x8F4F;

// ============================================================================
// OpenGL 4.4 Constants
// ============================================================================

// Buffer storage
pub const GL_MAP_PERSISTENT_BIT: GLenum = 0x0040;
pub const GL_MAP_COHERENT_BIT: GLenum = 0x0080;
pub const GL_DYNAMIC_STORAGE_BIT: GLenum = 0x0100;
pub const GL_CLIENT_STORAGE_BIT: GLenum = 0x0200;
pub const GL_CLIENT_MAPPED_BUFFER_BARRIER_BIT: GLenum = 0x00004000;
pub const GL_BUFFER_IMMUTABLE_STORAGE: GLenum = 0x821F;
pub const GL_BUFFER_STORAGE_FLAGS: GLenum = 0x8220;

// Clear texture
pub const GL_CLEAR_TEXTURE: GLenum = 0x9365;

// Enhanced layouts
pub const GL_LOCATION_COMPONENT: GLenum = 0x934A;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_INDEX: GLenum = 0x934B;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_STRIDE: GLenum = 0x934C;

// Query buffer object
pub const GL_QUERY_BUFFER: GLenum = 0x9192;
pub const GL_QUERY_BUFFER_BARRIER_BIT: GLenum = 0x00008000;
pub const GL_QUERY_BUFFER_BINDING: GLenum = 0x9193;
pub const GL_QUERY_RESULT_NO_WAIT: GLenum = 0x9194;

// Mirror clamp to edge
pub const GL_MIRROR_CLAMP_TO_EDGE: GLenum = 0x8743;

// ============================================================================
// OpenGL 4.5 Constants
// ============================================================================

// Clip control
pub const GL_CLIP_ORIGIN: GLenum = 0x935C;
pub const GL_CLIP_DEPTH_MODE: GLenum = 0x935D;
pub const GL_NEGATIVE_ONE_TO_ONE: GLenum = 0x935E;
pub const GL_ZERO_TO_ONE: GLenum = 0x935F;

// Conditional render inverted
pub const GL_QUERY_WAIT_INVERTED: GLenum = 0x8E17;
pub const GL_QUERY_NO_WAIT_INVERTED: GLenum = 0x8E18;
pub const GL_QUERY_BY_REGION_WAIT_INVERTED: GLenum = 0x8E19;
pub const GL_QUERY_BY_REGION_NO_WAIT_INVERTED: GLenum = 0x8E1A;

// Cull distance
pub const GL_MAX_CULL_DISTANCES: GLenum = 0x82F9;
pub const GL_MAX_COMBINED_CLIP_AND_CULL_DISTANCES: GLenum = 0x82FA;

// Direct state access
pub const GL_TEXTURE_TARGET: GLenum = 0x1006;
pub const GL_QUERY_TARGET: GLenum = 0x82EA;

// Get texture sub image
pub const GL_TEXTURE_BINDING: GLenum = 0x82EB;

// Robustness
pub const GL_CONTEXT_LOST: GLenum = 0x0507;
pub const GL_LOSE_CONTEXT_ON_RESET: GLenum = 0x8252;
pub const GL_GUILTY_CONTEXT_RESET: GLenum = 0x8253;
pub const GL_INNOCENT_CONTEXT_RESET: GLenum = 0x8254;
pub const GL_UNKNOWN_CONTEXT_RESET: GLenum = 0x8255;
pub const GL_RESET_NOTIFICATION_STRATEGY: GLenum = 0x8256;
pub const GL_NO_RESET_NOTIFICATION: GLenum = 0x8261;
pub const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT: GLenum = 0x00000004;
pub const GL_CONTEXT_RELEASE_BEHAVIOR: GLenum = 0x82FB;
pub const GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH: GLenum = 0x82FC;

// ============================================================================
// OpenGL 4.6 Constants (Final Version)
// ============================================================================

// SPIR-V support
pub const GL_SHADER_BINARY_FORMAT_SPIR_V: GLenum = 0x9551;
pub const GL_SPIR_V_BINARY: GLenum = 0x9552;
pub const GL_PARAMETER_BUFFER: GLenum = 0x80EE;
pub const GL_PARAMETER_BUFFER_BINDING: GLenum = 0x80EF;
pub const GL_CONTEXT_FLAG_NO_ERROR_BIT: GLenum = 0x00000008;
pub const GL_VERTICES_SUBMITTED: GLenum = 0x82EE;
pub const GL_PRIMITIVES_SUBMITTED: GLenum = 0x82EF;
pub const GL_VERTEX_SHADER_INVOCATIONS: GLenum = 0x82F0;
pub const GL_TESS_CONTROL_SHADER_PATCHES: GLenum = 0x82F1;
pub const GL_TESS_EVALUATION_SHADER_INVOCATIONS: GLenum = 0x82F2;
pub const GL_GEOMETRY_SHADER_PRIMITIVES_EMITTED: GLenum = 0x82F3;
pub const GL_FRAGMENT_SHADER_INVOCATIONS: GLenum = 0x82F4;
pub const GL_COMPUTE_SHADER_INVOCATIONS: GLenum = 0x82F5;
pub const GL_CLIPPING_INPUT_PRIMITIVES: GLenum = 0x82F6;
pub const GL_CLIPPING_OUTPUT_PRIMITIVES: GLenum = 0x82F7;
pub const GL_POLYGON_OFFSET_CLAMP: GLenum = 0x8E1B;
pub const GL_SPIR_V_EXTENSIONS: GLenum = 0x9553;
pub const GL_NUM_SPIR_V_EXTENSIONS: GLenum = 0x9554;
pub const GL_TEXTURE_MAX_ANISOTROPY: GLenum = 0x84FE;
pub const GL_MAX_TEXTURE_MAX_ANISOTROPY: GLenum = 0x84FF;
pub const GL_TRANSFORM_FEEDBACK_OVERFLOW: GLenum = 0x82EC;
pub const GL_TRANSFORM_FEEDBACK_STREAM_OVERFLOW: GLenum = 0x82ED;
