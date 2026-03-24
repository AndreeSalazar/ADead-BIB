//! OpenGL 1.0 - 1.5 Constants
//! Based on Khronos canonical specifications
//! https://registry.khronos.org/OpenGL/

use super::types::GLenum;

// ============================================================================
// OpenGL 1.0 Constants
// ============================================================================

// Boolean values
pub const GL_FALSE: GLenum = 0;
pub const GL_TRUE: GLenum = 1;

// Data types
pub const GL_BYTE: GLenum = 0x1400;
pub const GL_UNSIGNED_BYTE: GLenum = 0x1401;
pub const GL_SHORT: GLenum = 0x1402;
pub const GL_UNSIGNED_SHORT: GLenum = 0x1403;
pub const GL_INT: GLenum = 0x1404;
pub const GL_UNSIGNED_INT: GLenum = 0x1405;
pub const GL_FLOAT: GLenum = 0x1406;
pub const GL_2_BYTES: GLenum = 0x1407;
pub const GL_3_BYTES: GLenum = 0x1408;
pub const GL_4_BYTES: GLenum = 0x1409;
pub const GL_DOUBLE: GLenum = 0x140A;

// Primitives
pub const GL_POINTS: GLenum = 0x0000;
pub const GL_LINES: GLenum = 0x0001;
pub const GL_LINE_LOOP: GLenum = 0x0002;
pub const GL_LINE_STRIP: GLenum = 0x0003;
pub const GL_TRIANGLES: GLenum = 0x0004;
pub const GL_TRIANGLE_STRIP: GLenum = 0x0005;
pub const GL_TRIANGLE_FAN: GLenum = 0x0006;
pub const GL_QUADS: GLenum = 0x0007;
pub const GL_QUAD_STRIP: GLenum = 0x0008;
pub const GL_POLYGON: GLenum = 0x0009;

// Vertex Arrays
pub const GL_VERTEX_ARRAY: GLenum = 0x8074;
pub const GL_NORMAL_ARRAY: GLenum = 0x8075;
pub const GL_COLOR_ARRAY: GLenum = 0x8076;
pub const GL_INDEX_ARRAY: GLenum = 0x8077;
pub const GL_TEXTURE_COORD_ARRAY: GLenum = 0x8078;
pub const GL_EDGE_FLAG_ARRAY: GLenum = 0x8079;

// Matrix Mode
pub const GL_MATRIX_MODE: GLenum = 0x0BA0;
pub const GL_MODELVIEW: GLenum = 0x1700;
pub const GL_PROJECTION: GLenum = 0x1701;
pub const GL_TEXTURE: GLenum = 0x1702;

// Points
pub const GL_POINT_SMOOTH: GLenum = 0x0B10;
pub const GL_POINT_SIZE: GLenum = 0x0B11;
pub const GL_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub const GL_POINT_SIZE_RANGE: GLenum = 0x0B12;

// Lines
pub const GL_LINE_SMOOTH: GLenum = 0x0B20;
pub const GL_LINE_STIPPLE: GLenum = 0x0B24;
pub const GL_LINE_STIPPLE_PATTERN: GLenum = 0x0B25;
pub const GL_LINE_STIPPLE_REPEAT: GLenum = 0x0B26;
pub const GL_LINE_WIDTH: GLenum = 0x0B21;
pub const GL_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub const GL_LINE_WIDTH_RANGE: GLenum = 0x0B22;

// Polygons
pub const GL_POINT: GLenum = 0x1B00;
pub const GL_LINE: GLenum = 0x1B01;
pub const GL_FILL: GLenum = 0x1B02;
pub const GL_CW: GLenum = 0x0900;
pub const GL_CCW: GLenum = 0x0901;
pub const GL_FRONT: GLenum = 0x0404;
pub const GL_BACK: GLenum = 0x0405;
pub const GL_POLYGON_MODE: GLenum = 0x0B40;
pub const GL_POLYGON_SMOOTH: GLenum = 0x0B41;
pub const GL_POLYGON_STIPPLE: GLenum = 0x0B42;
pub const GL_EDGE_FLAG: GLenum = 0x0B43;
pub const GL_CULL_FACE: GLenum = 0x0B44;
pub const GL_CULL_FACE_MODE: GLenum = 0x0B45;
pub const GL_FRONT_FACE: GLenum = 0x0B46;
pub const GL_POLYGON_OFFSET_FACTOR: GLenum = 0x8038;
pub const GL_POLYGON_OFFSET_UNITS: GLenum = 0x2A00;
pub const GL_POLYGON_OFFSET_POINT: GLenum = 0x2A01;
pub const GL_POLYGON_OFFSET_LINE: GLenum = 0x2A02;
pub const GL_POLYGON_OFFSET_FILL: GLenum = 0x8037;

// Display Lists
pub const GL_COMPILE: GLenum = 0x1300;
pub const GL_COMPILE_AND_EXECUTE: GLenum = 0x1301;
pub const GL_LIST_BASE: GLenum = 0x0B32;
pub const GL_LIST_INDEX: GLenum = 0x0B33;
pub const GL_LIST_MODE: GLenum = 0x0B30;

// Depth buffer
pub const GL_NEVER: GLenum = 0x0200;
pub const GL_LESS: GLenum = 0x0201;
pub const GL_EQUAL: GLenum = 0x0202;
pub const GL_LEQUAL: GLenum = 0x0203;
pub const GL_GREATER: GLenum = 0x0204;
pub const GL_NOTEQUAL: GLenum = 0x0205;
pub const GL_GEQUAL: GLenum = 0x0206;
pub const GL_ALWAYS: GLenum = 0x0207;
pub const GL_DEPTH_TEST: GLenum = 0x0B71;
pub const GL_DEPTH_BITS: GLenum = 0x0D56;
pub const GL_DEPTH_CLEAR_VALUE: GLenum = 0x0B73;
pub const GL_DEPTH_FUNC: GLenum = 0x0B74;
pub const GL_DEPTH_RANGE: GLenum = 0x0B70;
pub const GL_DEPTH_WRITEMASK: GLenum = 0x0B72;
pub const GL_DEPTH_COMPONENT: GLenum = 0x1902;

// Lighting
pub const GL_LIGHTING: GLenum = 0x0B50;
pub const GL_LIGHT0: GLenum = 0x4000;
pub const GL_LIGHT1: GLenum = 0x4001;
pub const GL_LIGHT2: GLenum = 0x4002;
pub const GL_LIGHT3: GLenum = 0x4003;
pub const GL_LIGHT4: GLenum = 0x4004;
pub const GL_LIGHT5: GLenum = 0x4005;
pub const GL_LIGHT6: GLenum = 0x4006;
pub const GL_LIGHT7: GLenum = 0x4007;
pub const GL_SPOT_EXPONENT: GLenum = 0x1205;
pub const GL_SPOT_CUTOFF: GLenum = 0x1206;
pub const GL_CONSTANT_ATTENUATION: GLenum = 0x1207;
pub const GL_LINEAR_ATTENUATION: GLenum = 0x1208;
pub const GL_QUADRATIC_ATTENUATION: GLenum = 0x1209;
pub const GL_AMBIENT: GLenum = 0x1200;
pub const GL_DIFFUSE: GLenum = 0x1201;
pub const GL_SPECULAR: GLenum = 0x1202;
pub const GL_SHININESS: GLenum = 0x1601;
pub const GL_EMISSION: GLenum = 0x1600;
pub const GL_POSITION: GLenum = 0x1203;
pub const GL_SPOT_DIRECTION: GLenum = 0x1204;
pub const GL_AMBIENT_AND_DIFFUSE: GLenum = 0x1602;
pub const GL_COLOR_INDEXES: GLenum = 0x1603;
pub const GL_LIGHT_MODEL_TWO_SIDE: GLenum = 0x0B52;
pub const GL_LIGHT_MODEL_LOCAL_VIEWER: GLenum = 0x0B51;
pub const GL_LIGHT_MODEL_AMBIENT: GLenum = 0x0B53;
pub const GL_FRONT_AND_BACK: GLenum = 0x0408;
pub const GL_SHADE_MODEL: GLenum = 0x0B54;
pub const GL_FLAT: GLenum = 0x1D00;
pub const GL_SMOOTH: GLenum = 0x1D01;
pub const GL_COLOR_MATERIAL: GLenum = 0x0B57;
pub const GL_COLOR_MATERIAL_FACE: GLenum = 0x0B55;
pub const GL_COLOR_MATERIAL_PARAMETER: GLenum = 0x0B56;
pub const GL_NORMALIZE: GLenum = 0x0BA1;

// User clipping planes
pub const GL_CLIP_PLANE0: GLenum = 0x3000;
pub const GL_CLIP_PLANE1: GLenum = 0x3001;
pub const GL_CLIP_PLANE2: GLenum = 0x3002;
pub const GL_CLIP_PLANE3: GLenum = 0x3003;
pub const GL_CLIP_PLANE4: GLenum = 0x3004;
pub const GL_CLIP_PLANE5: GLenum = 0x3005;

// Accumulation buffer
pub const GL_ACCUM_RED_BITS: GLenum = 0x0D58;
pub const GL_ACCUM_GREEN_BITS: GLenum = 0x0D59;
pub const GL_ACCUM_BLUE_BITS: GLenum = 0x0D5A;
pub const GL_ACCUM_ALPHA_BITS: GLenum = 0x0D5B;
pub const GL_ACCUM_CLEAR_VALUE: GLenum = 0x0B80;
pub const GL_ACCUM: GLenum = 0x0100;
pub const GL_ADD: GLenum = 0x0104;
pub const GL_LOAD: GLenum = 0x0101;
pub const GL_MULT: GLenum = 0x0103;
pub const GL_RETURN: GLenum = 0x0102;

// Alpha testing
pub const GL_ALPHA_TEST: GLenum = 0x0BC0;
pub const GL_ALPHA_TEST_REF: GLenum = 0x0BC2;
pub const GL_ALPHA_TEST_FUNC: GLenum = 0x0BC1;

// Blending
pub const GL_BLEND: GLenum = 0x0BE2;
pub const GL_BLEND_SRC: GLenum = 0x0BE1;
pub const GL_BLEND_DST: GLenum = 0x0BE0;
pub const GL_ZERO: GLenum = 0;
pub const GL_ONE: GLenum = 1;
pub const GL_SRC_COLOR: GLenum = 0x0300;
pub const GL_ONE_MINUS_SRC_COLOR: GLenum = 0x0301;
pub const GL_SRC_ALPHA: GLenum = 0x0302;
pub const GL_ONE_MINUS_SRC_ALPHA: GLenum = 0x0303;
pub const GL_DST_ALPHA: GLenum = 0x0304;
pub const GL_ONE_MINUS_DST_ALPHA: GLenum = 0x0305;
pub const GL_DST_COLOR: GLenum = 0x0306;
pub const GL_ONE_MINUS_DST_COLOR: GLenum = 0x0307;
pub const GL_SRC_ALPHA_SATURATE: GLenum = 0x0308;

// Render Mode
pub const GL_FEEDBACK: GLenum = 0x1C01;
pub const GL_RENDER: GLenum = 0x1C00;
pub const GL_SELECT: GLenum = 0x1C02;

// Feedback
pub const GL_2D: GLenum = 0x0600;
pub const GL_3D: GLenum = 0x0601;
pub const GL_3D_COLOR: GLenum = 0x0602;
pub const GL_3D_COLOR_TEXTURE: GLenum = 0x0603;
pub const GL_4D_COLOR_TEXTURE: GLenum = 0x0604;
pub const GL_POINT_TOKEN: GLenum = 0x0701;
pub const GL_LINE_TOKEN: GLenum = 0x0702;
pub const GL_LINE_RESET_TOKEN: GLenum = 0x0707;
pub const GL_POLYGON_TOKEN: GLenum = 0x0703;
pub const GL_BITMAP_TOKEN: GLenum = 0x0704;
pub const GL_DRAW_PIXEL_TOKEN: GLenum = 0x0705;
pub const GL_COPY_PIXEL_TOKEN: GLenum = 0x0706;
pub const GL_PASS_THROUGH_TOKEN: GLenum = 0x0700;
pub const GL_FEEDBACK_BUFFER_POINTER: GLenum = 0x0DF0;
pub const GL_FEEDBACK_BUFFER_SIZE: GLenum = 0x0DF1;
pub const GL_FEEDBACK_BUFFER_TYPE: GLenum = 0x0DF2;

// Selection
pub const GL_SELECTION_BUFFER_POINTER: GLenum = 0x0DF3;
pub const GL_SELECTION_BUFFER_SIZE: GLenum = 0x0DF4;

// Fog
pub const GL_FOG: GLenum = 0x0B60;
pub const GL_FOG_MODE: GLenum = 0x0B65;
pub const GL_FOG_DENSITY: GLenum = 0x0B62;
pub const GL_FOG_COLOR: GLenum = 0x0B66;
pub const GL_FOG_INDEX: GLenum = 0x0B61;
pub const GL_FOG_START: GLenum = 0x0B63;
pub const GL_FOG_END: GLenum = 0x0B64;
pub const GL_LINEAR: GLenum = 0x2601;
pub const GL_EXP: GLenum = 0x0800;
pub const GL_EXP2: GLenum = 0x0801;

// Logic Ops
pub const GL_LOGIC_OP: GLenum = 0x0BF1;
pub const GL_INDEX_LOGIC_OP: GLenum = 0x0BF1;
pub const GL_COLOR_LOGIC_OP: GLenum = 0x0BF2;
pub const GL_LOGIC_OP_MODE: GLenum = 0x0BF0;
pub const GL_CLEAR: GLenum = 0x1500;
pub const GL_SET: GLenum = 0x150F;
pub const GL_COPY: GLenum = 0x1503;
pub const GL_COPY_INVERTED: GLenum = 0x150C;
pub const GL_NOOP: GLenum = 0x1505;
pub const GL_INVERT: GLenum = 0x150A;
pub const GL_AND: GLenum = 0x1501;
pub const GL_NAND: GLenum = 0x150E;
pub const GL_OR: GLenum = 0x1507;
pub const GL_NOR: GLenum = 0x1508;
pub const GL_XOR: GLenum = 0x1506;
pub const GL_EQUIV: GLenum = 0x1509;
pub const GL_AND_REVERSE: GLenum = 0x1502;
pub const GL_AND_INVERTED: GLenum = 0x1504;
pub const GL_OR_REVERSE: GLenum = 0x150B;
pub const GL_OR_INVERTED: GLenum = 0x150D;

// Stencil
pub const GL_STENCIL_BITS: GLenum = 0x0D57;
pub const GL_STENCIL_TEST: GLenum = 0x0B90;
pub const GL_STENCIL_CLEAR_VALUE: GLenum = 0x0B91;
pub const GL_STENCIL_FUNC: GLenum = 0x0B92;
pub const GL_STENCIL_VALUE_MASK: GLenum = 0x0B93;
pub const GL_STENCIL_FAIL: GLenum = 0x0B94;
pub const GL_STENCIL_PASS_DEPTH_FAIL: GLenum = 0x0B95;
pub const GL_STENCIL_PASS_DEPTH_PASS: GLenum = 0x0B96;
pub const GL_STENCIL_REF: GLenum = 0x0B97;
pub const GL_STENCIL_WRITEMASK: GLenum = 0x0B98;
pub const GL_STENCIL_INDEX: GLenum = 0x1901;
pub const GL_KEEP: GLenum = 0x1E00;
pub const GL_REPLACE: GLenum = 0x1E01;
pub const GL_INCR: GLenum = 0x1E02;
pub const GL_DECR: GLenum = 0x1E03;

// Buffers, Pixel Drawing/Reading
pub const GL_NONE: GLenum = 0;
pub const GL_LEFT: GLenum = 0x0406;
pub const GL_RIGHT: GLenum = 0x0407;
pub const GL_FRONT_LEFT: GLenum = 0x0400;
pub const GL_FRONT_RIGHT: GLenum = 0x0401;
pub const GL_BACK_LEFT: GLenum = 0x0402;
pub const GL_BACK_RIGHT: GLenum = 0x0403;
pub const GL_AUX0: GLenum = 0x0409;
pub const GL_AUX1: GLenum = 0x040A;
pub const GL_AUX2: GLenum = 0x040B;
pub const GL_AUX3: GLenum = 0x040C;
pub const GL_COLOR_INDEX: GLenum = 0x1900;
pub const GL_RED: GLenum = 0x1903;
pub const GL_GREEN: GLenum = 0x1904;
pub const GL_BLUE: GLenum = 0x1905;
pub const GL_ALPHA: GLenum = 0x1906;
pub const GL_LUMINANCE: GLenum = 0x1909;
pub const GL_LUMINANCE_ALPHA: GLenum = 0x190A;
pub const GL_ALPHA_BITS: GLenum = 0x0D55;
pub const GL_RED_BITS: GLenum = 0x0D52;
pub const GL_GREEN_BITS: GLenum = 0x0D53;
pub const GL_BLUE_BITS: GLenum = 0x0D54;
pub const GL_INDEX_BITS: GLenum = 0x0D51;
pub const GL_SUBPIXEL_BITS: GLenum = 0x0D50;
pub const GL_AUX_BUFFERS: GLenum = 0x0C00;
pub const GL_READ_BUFFER: GLenum = 0x0C02;
pub const GL_DRAW_BUFFER: GLenum = 0x0C01;
pub const GL_DOUBLEBUFFER: GLenum = 0x0C32;
pub const GL_STEREO: GLenum = 0x0C33;
pub const GL_BITMAP: GLenum = 0x1A00;
pub const GL_COLOR: GLenum = 0x1800;
pub const GL_DEPTH: GLenum = 0x1801;
pub const GL_STENCIL: GLenum = 0x1802;
pub const GL_DITHER: GLenum = 0x0BD0;
pub const GL_RGB: GLenum = 0x1907;
pub const GL_RGBA: GLenum = 0x1908;

// Implementation limits
pub const GL_MAX_LIST_NESTING: GLenum = 0x0B31;
pub const GL_MAX_EVAL_ORDER: GLenum = 0x0D30;
pub const GL_MAX_LIGHTS: GLenum = 0x0D31;
pub const GL_MAX_CLIP_PLANES: GLenum = 0x0D32;
pub const GL_MAX_TEXTURE_SIZE: GLenum = 0x0D33;
pub const GL_MAX_PIXEL_MAP_TABLE: GLenum = 0x0D34;
pub const GL_MAX_ATTRIB_STACK_DEPTH: GLenum = 0x0D35;
pub const GL_MAX_MODELVIEW_STACK_DEPTH: GLenum = 0x0D36;
pub const GL_MAX_NAME_STACK_DEPTH: GLenum = 0x0D37;
pub const GL_MAX_PROJECTION_STACK_DEPTH: GLenum = 0x0D38;
pub const GL_MAX_TEXTURE_STACK_DEPTH: GLenum = 0x0D39;
pub const GL_MAX_VIEWPORT_DIMS: GLenum = 0x0D3A;
pub const GL_MAX_CLIENT_ATTRIB_STACK_DEPTH: GLenum = 0x0D3B;

// Gets
pub const GL_ATTRIB_STACK_DEPTH: GLenum = 0x0BB0;
pub const GL_CLIENT_ATTRIB_STACK_DEPTH: GLenum = 0x0BB1;
pub const GL_COLOR_CLEAR_VALUE: GLenum = 0x0C22;
pub const GL_COLOR_WRITEMASK: GLenum = 0x0C23;
pub const GL_CURRENT_INDEX: GLenum = 0x0B01;
pub const GL_CURRENT_COLOR: GLenum = 0x0B00;
pub const GL_CURRENT_NORMAL: GLenum = 0x0B02;
pub const GL_CURRENT_RASTER_COLOR: GLenum = 0x0B04;
pub const GL_CURRENT_RASTER_DISTANCE: GLenum = 0x0B09;
pub const GL_CURRENT_RASTER_INDEX: GLenum = 0x0B05;
pub const GL_CURRENT_RASTER_POSITION: GLenum = 0x0B07;
pub const GL_CURRENT_RASTER_TEXTURE_COORDS: GLenum = 0x0B06;
pub const GL_CURRENT_RASTER_POSITION_VALID: GLenum = 0x0B08;
pub const GL_CURRENT_TEXTURE_COORDS: GLenum = 0x0B03;
pub const GL_INDEX_CLEAR_VALUE: GLenum = 0x0C20;
pub const GL_INDEX_MODE: GLenum = 0x0C30;
pub const GL_INDEX_WRITEMASK: GLenum = 0x0C21;
pub const GL_MODELVIEW_MATRIX: GLenum = 0x0BA6;
pub const GL_MODELVIEW_STACK_DEPTH: GLenum = 0x0BA3;
pub const GL_NAME_STACK_DEPTH: GLenum = 0x0D70;
pub const GL_PROJECTION_MATRIX: GLenum = 0x0BA7;
pub const GL_PROJECTION_STACK_DEPTH: GLenum = 0x0BA4;
pub const GL_RENDER_MODE: GLenum = 0x0C40;
pub const GL_RGBA_MODE: GLenum = 0x0C31;
pub const GL_TEXTURE_MATRIX: GLenum = 0x0BA8;
pub const GL_TEXTURE_STACK_DEPTH: GLenum = 0x0BA5;
pub const GL_VIEWPORT: GLenum = 0x0BA2;

// Evaluators
pub const GL_AUTO_NORMAL: GLenum = 0x0D80;
pub const GL_MAP1_COLOR_4: GLenum = 0x0D90;
pub const GL_MAP1_INDEX: GLenum = 0x0D91;
pub const GL_MAP1_NORMAL: GLenum = 0x0D92;
pub const GL_MAP1_TEXTURE_COORD_1: GLenum = 0x0D93;
pub const GL_MAP1_TEXTURE_COORD_2: GLenum = 0x0D94;
pub const GL_MAP1_TEXTURE_COORD_3: GLenum = 0x0D95;
pub const GL_MAP1_TEXTURE_COORD_4: GLenum = 0x0D96;
pub const GL_MAP1_VERTEX_3: GLenum = 0x0D97;
pub const GL_MAP1_VERTEX_4: GLenum = 0x0D98;
pub const GL_MAP2_COLOR_4: GLenum = 0x0DB0;
pub const GL_MAP2_INDEX: GLenum = 0x0DB1;
pub const GL_MAP2_NORMAL: GLenum = 0x0DB2;
pub const GL_MAP2_TEXTURE_COORD_1: GLenum = 0x0DB3;
pub const GL_MAP2_TEXTURE_COORD_2: GLenum = 0x0DB4;
pub const GL_MAP2_TEXTURE_COORD_3: GLenum = 0x0DB5;
pub const GL_MAP2_TEXTURE_COORD_4: GLenum = 0x0DB6;
pub const GL_MAP2_VERTEX_3: GLenum = 0x0DB7;
pub const GL_MAP2_VERTEX_4: GLenum = 0x0DB8;
pub const GL_MAP1_GRID_DOMAIN: GLenum = 0x0DD0;
pub const GL_MAP1_GRID_SEGMENTS: GLenum = 0x0DD1;
pub const GL_MAP2_GRID_DOMAIN: GLenum = 0x0DD2;
pub const GL_MAP2_GRID_SEGMENTS: GLenum = 0x0DD3;
pub const GL_COEFF: GLenum = 0x0A00;
pub const GL_ORDER: GLenum = 0x0A01;
pub const GL_DOMAIN: GLenum = 0x0A02;

// Hints
pub const GL_PERSPECTIVE_CORRECTION_HINT: GLenum = 0x0C50;
pub const GL_POINT_SMOOTH_HINT: GLenum = 0x0C51;
pub const GL_LINE_SMOOTH_HINT: GLenum = 0x0C52;
pub const GL_POLYGON_SMOOTH_HINT: GLenum = 0x0C53;
pub const GL_FOG_HINT: GLenum = 0x0C54;
pub const GL_DONT_CARE: GLenum = 0x1100;
pub const GL_FASTEST: GLenum = 0x1101;
pub const GL_NICEST: GLenum = 0x1102;

// Scissor box
pub const GL_SCISSOR_BOX: GLenum = 0x0C10;
pub const GL_SCISSOR_TEST: GLenum = 0x0C11;

// Pixel Mode / Transfer
pub const GL_MAP_COLOR: GLenum = 0x0D10;
pub const GL_MAP_STENCIL: GLenum = 0x0D11;
pub const GL_INDEX_SHIFT: GLenum = 0x0D12;
pub const GL_INDEX_OFFSET: GLenum = 0x0D13;
pub const GL_RED_SCALE: GLenum = 0x0D14;
pub const GL_RED_BIAS: GLenum = 0x0D15;
pub const GL_GREEN_SCALE: GLenum = 0x0D18;
pub const GL_GREEN_BIAS: GLenum = 0x0D19;
pub const GL_BLUE_SCALE: GLenum = 0x0D1A;
pub const GL_BLUE_BIAS: GLenum = 0x0D1B;
pub const GL_ALPHA_SCALE: GLenum = 0x0D1C;
pub const GL_ALPHA_BIAS: GLenum = 0x0D1D;
pub const GL_DEPTH_SCALE: GLenum = 0x0D1E;
pub const GL_DEPTH_BIAS: GLenum = 0x0D1F;
pub const GL_PIXEL_MAP_S_TO_S_SIZE: GLenum = 0x0CB1;
pub const GL_PIXEL_MAP_I_TO_I_SIZE: GLenum = 0x0CB0;
pub const GL_PIXEL_MAP_I_TO_R_SIZE: GLenum = 0x0CB2;
pub const GL_PIXEL_MAP_I_TO_G_SIZE: GLenum = 0x0CB3;
pub const GL_PIXEL_MAP_I_TO_B_SIZE: GLenum = 0x0CB4;
pub const GL_PIXEL_MAP_I_TO_A_SIZE: GLenum = 0x0CB5;
pub const GL_PIXEL_MAP_R_TO_R_SIZE: GLenum = 0x0CB6;
pub const GL_PIXEL_MAP_G_TO_G_SIZE: GLenum = 0x0CB7;
pub const GL_PIXEL_MAP_B_TO_B_SIZE: GLenum = 0x0CB8;
pub const GL_PIXEL_MAP_A_TO_A_SIZE: GLenum = 0x0CB9;
pub const GL_PIXEL_MAP_S_TO_S: GLenum = 0x0C71;
pub const GL_PIXEL_MAP_I_TO_I: GLenum = 0x0C70;
pub const GL_PIXEL_MAP_I_TO_R: GLenum = 0x0C72;
pub const GL_PIXEL_MAP_I_TO_G: GLenum = 0x0C73;
pub const GL_PIXEL_MAP_I_TO_B: GLenum = 0x0C74;
pub const GL_PIXEL_MAP_I_TO_A: GLenum = 0x0C75;
pub const GL_PIXEL_MAP_R_TO_R: GLenum = 0x0C76;
pub const GL_PIXEL_MAP_G_TO_G: GLenum = 0x0C77;
pub const GL_PIXEL_MAP_B_TO_B: GLenum = 0x0C78;
pub const GL_PIXEL_MAP_A_TO_A: GLenum = 0x0C79;
pub const GL_PACK_ALIGNMENT: GLenum = 0x0D05;
pub const GL_PACK_LSB_FIRST: GLenum = 0x0D01;
pub const GL_PACK_ROW_LENGTH: GLenum = 0x0D02;
pub const GL_PACK_SKIP_PIXELS: GLenum = 0x0D04;
pub const GL_PACK_SKIP_ROWS: GLenum = 0x0D03;
pub const GL_PACK_SWAP_BYTES: GLenum = 0x0D00;
pub const GL_UNPACK_ALIGNMENT: GLenum = 0x0CF5;
pub const GL_UNPACK_LSB_FIRST: GLenum = 0x0CF1;
pub const GL_UNPACK_ROW_LENGTH: GLenum = 0x0CF2;
pub const GL_UNPACK_SKIP_PIXELS: GLenum = 0x0CF4;
pub const GL_UNPACK_SKIP_ROWS: GLenum = 0x0CF3;
pub const GL_UNPACK_SWAP_BYTES: GLenum = 0x0CF0;
pub const GL_ZOOM_X: GLenum = 0x0D16;
pub const GL_ZOOM_Y: GLenum = 0x0D17;

// Texture mapping
pub const GL_TEXTURE_ENV: GLenum = 0x2300;
pub const GL_TEXTURE_ENV_MODE: GLenum = 0x2200;
pub const GL_TEXTURE_1D: GLenum = 0x0DE0;
pub const GL_TEXTURE_2D: GLenum = 0x0DE1;
pub const GL_TEXTURE_WRAP_S: GLenum = 0x2802;
pub const GL_TEXTURE_WRAP_T: GLenum = 0x2803;
pub const GL_TEXTURE_MAG_FILTER: GLenum = 0x2800;
pub const GL_TEXTURE_MIN_FILTER: GLenum = 0x2801;
pub const GL_TEXTURE_ENV_COLOR: GLenum = 0x2201;
pub const GL_TEXTURE_GEN_S: GLenum = 0x0C60;
pub const GL_TEXTURE_GEN_T: GLenum = 0x0C61;
pub const GL_TEXTURE_GEN_R: GLenum = 0x0C62;
pub const GL_TEXTURE_GEN_Q: GLenum = 0x0C63;
pub const GL_TEXTURE_GEN_MODE: GLenum = 0x2500;
pub const GL_TEXTURE_BORDER_COLOR: GLenum = 0x1004;
pub const GL_TEXTURE_WIDTH: GLenum = 0x1000;
pub const GL_TEXTURE_HEIGHT: GLenum = 0x1001;
pub const GL_TEXTURE_BORDER: GLenum = 0x1005;
pub const GL_TEXTURE_COMPONENTS: GLenum = 0x1003;
pub const GL_TEXTURE_RED_SIZE: GLenum = 0x805C;
pub const GL_TEXTURE_GREEN_SIZE: GLenum = 0x805D;
pub const GL_TEXTURE_BLUE_SIZE: GLenum = 0x805E;
pub const GL_TEXTURE_ALPHA_SIZE: GLenum = 0x805F;
pub const GL_TEXTURE_LUMINANCE_SIZE: GLenum = 0x8060;
pub const GL_TEXTURE_INTENSITY_SIZE: GLenum = 0x8061;
pub const GL_NEAREST_MIPMAP_NEAREST: GLenum = 0x2700;
pub const GL_NEAREST_MIPMAP_LINEAR: GLenum = 0x2702;
pub const GL_LINEAR_MIPMAP_NEAREST: GLenum = 0x2701;
pub const GL_LINEAR_MIPMAP_LINEAR: GLenum = 0x2703;
pub const GL_OBJECT_LINEAR: GLenum = 0x2401;
pub const GL_OBJECT_PLANE: GLenum = 0x2501;
pub const GL_EYE_LINEAR: GLenum = 0x2400;
pub const GL_EYE_PLANE: GLenum = 0x2502;
pub const GL_SPHERE_MAP: GLenum = 0x2402;
pub const GL_DECAL: GLenum = 0x2101;
pub const GL_MODULATE: GLenum = 0x2100;
pub const GL_NEAREST: GLenum = 0x2600;
pub const GL_REPEAT: GLenum = 0x2901;
pub const GL_CLAMP: GLenum = 0x2900;
pub const GL_S: GLenum = 0x2000;
pub const GL_T: GLenum = 0x2001;
pub const GL_R: GLenum = 0x2002;
pub const GL_Q: GLenum = 0x2003;

// Utility
pub const GL_VENDOR: GLenum = 0x1F00;
pub const GL_RENDERER: GLenum = 0x1F01;
pub const GL_VERSION: GLenum = 0x1F02;
pub const GL_EXTENSIONS: GLenum = 0x1F03;

// Errors
pub const GL_NO_ERROR: GLenum = 0;
pub const GL_INVALID_ENUM: GLenum = 0x0500;
pub const GL_INVALID_VALUE: GLenum = 0x0501;
pub const GL_INVALID_OPERATION: GLenum = 0x0502;
pub const GL_STACK_OVERFLOW: GLenum = 0x0503;
pub const GL_STACK_UNDERFLOW: GLenum = 0x0504;
pub const GL_OUT_OF_MEMORY: GLenum = 0x0505;

// glPush/PopAttrib bits
pub const GL_CURRENT_BIT: GLenum = 0x00000001;
pub const GL_POINT_BIT: GLenum = 0x00000002;
pub const GL_LINE_BIT: GLenum = 0x00000004;
pub const GL_POLYGON_BIT: GLenum = 0x00000008;
pub const GL_POLYGON_STIPPLE_BIT: GLenum = 0x00000010;
pub const GL_PIXEL_MODE_BIT: GLenum = 0x00000020;
pub const GL_LIGHTING_BIT: GLenum = 0x00000040;
pub const GL_FOG_BIT: GLenum = 0x00000080;
pub const GL_DEPTH_BUFFER_BIT: GLenum = 0x00000100;
pub const GL_ACCUM_BUFFER_BIT: GLenum = 0x00000200;
pub const GL_STENCIL_BUFFER_BIT: GLenum = 0x00000400;
pub const GL_VIEWPORT_BIT: GLenum = 0x00000800;
pub const GL_TRANSFORM_BIT: GLenum = 0x00001000;
pub const GL_ENABLE_BIT: GLenum = 0x00002000;
pub const GL_COLOR_BUFFER_BIT: GLenum = 0x00004000;
pub const GL_HINT_BIT: GLenum = 0x00008000;
pub const GL_EVAL_BIT: GLenum = 0x00010000;
pub const GL_LIST_BIT: GLenum = 0x00020000;
pub const GL_TEXTURE_BIT: GLenum = 0x00040000;
pub const GL_SCISSOR_BIT: GLenum = 0x00080000;
pub const GL_ALL_ATTRIB_BITS: GLenum = 0x000FFFFF;

// ============================================================================
// OpenGL 1.1 Constants
// ============================================================================

pub const GL_PROXY_TEXTURE_1D: GLenum = 0x8063;
pub const GL_PROXY_TEXTURE_2D: GLenum = 0x8064;
pub const GL_TEXTURE_PRIORITY: GLenum = 0x8066;
pub const GL_TEXTURE_RESIDENT: GLenum = 0x8067;
pub const GL_TEXTURE_BINDING_1D: GLenum = 0x8068;
pub const GL_TEXTURE_BINDING_2D: GLenum = 0x8069;
pub const GL_TEXTURE_INTERNAL_FORMAT: GLenum = 0x1003;
pub const GL_ALPHA4: GLenum = 0x803B;
pub const GL_ALPHA8: GLenum = 0x803C;
pub const GL_ALPHA12: GLenum = 0x803D;
pub const GL_ALPHA16: GLenum = 0x803E;
pub const GL_LUMINANCE4: GLenum = 0x803F;
pub const GL_LUMINANCE8: GLenum = 0x8040;
pub const GL_LUMINANCE12: GLenum = 0x8041;
pub const GL_LUMINANCE16: GLenum = 0x8042;
pub const GL_LUMINANCE4_ALPHA4: GLenum = 0x8043;
pub const GL_LUMINANCE6_ALPHA2: GLenum = 0x8044;
pub const GL_LUMINANCE8_ALPHA8: GLenum = 0x8045;
pub const GL_LUMINANCE12_ALPHA4: GLenum = 0x8046;
pub const GL_LUMINANCE12_ALPHA12: GLenum = 0x8047;
pub const GL_LUMINANCE16_ALPHA16: GLenum = 0x8048;
pub const GL_INTENSITY: GLenum = 0x8049;
pub const GL_INTENSITY4: GLenum = 0x804A;
pub const GL_INTENSITY8: GLenum = 0x804B;
pub const GL_INTENSITY12: GLenum = 0x804C;
pub const GL_INTENSITY16: GLenum = 0x804D;
pub const GL_R3_G3_B2: GLenum = 0x2A10;
pub const GL_RGB4: GLenum = 0x804F;
pub const GL_RGB5: GLenum = 0x8050;
pub const GL_RGB8: GLenum = 0x8051;
pub const GL_RGB10: GLenum = 0x8052;
pub const GL_RGB12: GLenum = 0x8053;
pub const GL_RGB16: GLenum = 0x8054;
pub const GL_RGBA2: GLenum = 0x8055;
pub const GL_RGBA4: GLenum = 0x8056;
pub const GL_RGB5_A1: GLenum = 0x8057;
pub const GL_RGBA8: GLenum = 0x8058;
pub const GL_RGB10_A2: GLenum = 0x8059;
pub const GL_RGBA12: GLenum = 0x805A;
pub const GL_RGBA16: GLenum = 0x805B;
pub const GL_CLIENT_PIXEL_STORE_BIT: GLenum = 0x00000001;
pub const GL_CLIENT_VERTEX_ARRAY_BIT: GLenum = 0x00000002;
pub const GL_ALL_CLIENT_ATTRIB_BITS: GLenum = 0xFFFFFFFF;
pub const GL_CLIENT_ALL_ATTRIB_BITS: GLenum = 0xFFFFFFFF;

// ============================================================================
// OpenGL 1.2 Constants
// ============================================================================

pub const GL_RESCALE_NORMAL: GLenum = 0x803A;
pub const GL_CLAMP_TO_EDGE: GLenum = 0x812F;
pub const GL_MAX_ELEMENTS_VERTICES: GLenum = 0x80E8;
pub const GL_MAX_ELEMENTS_INDICES: GLenum = 0x80E9;
pub const GL_BGR: GLenum = 0x80E0;
pub const GL_BGRA: GLenum = 0x80E1;
pub const GL_UNSIGNED_BYTE_3_3_2: GLenum = 0x8032;
pub const GL_UNSIGNED_BYTE_2_3_3_REV: GLenum = 0x8362;
pub const GL_UNSIGNED_SHORT_5_6_5: GLenum = 0x8363;
pub const GL_UNSIGNED_SHORT_5_6_5_REV: GLenum = 0x8364;
pub const GL_UNSIGNED_SHORT_4_4_4_4: GLenum = 0x8033;
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: GLenum = 0x8365;
pub const GL_UNSIGNED_SHORT_5_5_5_1: GLenum = 0x8034;
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: GLenum = 0x8366;
pub const GL_UNSIGNED_INT_8_8_8_8: GLenum = 0x8035;
pub const GL_UNSIGNED_INT_8_8_8_8_REV: GLenum = 0x8367;
pub const GL_UNSIGNED_INT_10_10_10_2: GLenum = 0x8036;
pub const GL_UNSIGNED_INT_2_10_10_10_REV: GLenum = 0x8368;
pub const GL_LIGHT_MODEL_COLOR_CONTROL: GLenum = 0x81F8;
pub const GL_SINGLE_COLOR: GLenum = 0x81F9;
pub const GL_SEPARATE_SPECULAR_COLOR: GLenum = 0x81FA;
pub const GL_TEXTURE_MIN_LOD: GLenum = 0x813A;
pub const GL_TEXTURE_MAX_LOD: GLenum = 0x813B;
pub const GL_TEXTURE_BASE_LEVEL: GLenum = 0x813C;
pub const GL_TEXTURE_MAX_LEVEL: GLenum = 0x813D;
pub const GL_SMOOTH_POINT_SIZE_RANGE: GLenum = 0x0B12;
pub const GL_SMOOTH_POINT_SIZE_GRANULARITY: GLenum = 0x0B13;
pub const GL_SMOOTH_LINE_WIDTH_RANGE: GLenum = 0x0B22;
pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY: GLenum = 0x0B23;
pub const GL_ALIASED_POINT_SIZE_RANGE: GLenum = 0x846D;
pub const GL_ALIASED_LINE_WIDTH_RANGE: GLenum = 0x846E;
pub const GL_PACK_SKIP_IMAGES: GLenum = 0x806B;
pub const GL_PACK_IMAGE_HEIGHT: GLenum = 0x806C;
pub const GL_UNPACK_SKIP_IMAGES: GLenum = 0x806D;
pub const GL_UNPACK_IMAGE_HEIGHT: GLenum = 0x806E;
pub const GL_TEXTURE_3D: GLenum = 0x806F;
pub const GL_PROXY_TEXTURE_3D: GLenum = 0x8070;
pub const GL_TEXTURE_DEPTH: GLenum = 0x8071;
pub const GL_TEXTURE_WRAP_R: GLenum = 0x8072;
pub const GL_MAX_3D_TEXTURE_SIZE: GLenum = 0x8073;
pub const GL_TEXTURE_BINDING_3D: GLenum = 0x806A;

// ============================================================================
// OpenGL 1.3 Constants
// ============================================================================

pub const GL_TEXTURE0: GLenum = 0x84C0;
pub const GL_TEXTURE1: GLenum = 0x84C1;
pub const GL_TEXTURE2: GLenum = 0x84C2;
pub const GL_TEXTURE3: GLenum = 0x84C3;
pub const GL_TEXTURE4: GLenum = 0x84C4;
pub const GL_TEXTURE5: GLenum = 0x84C5;
pub const GL_TEXTURE6: GLenum = 0x84C6;
pub const GL_TEXTURE7: GLenum = 0x84C7;
pub const GL_TEXTURE8: GLenum = 0x84C8;
pub const GL_TEXTURE9: GLenum = 0x84C9;
pub const GL_TEXTURE10: GLenum = 0x84CA;
pub const GL_TEXTURE11: GLenum = 0x84CB;
pub const GL_TEXTURE12: GLenum = 0x84CC;
pub const GL_TEXTURE13: GLenum = 0x84CD;
pub const GL_TEXTURE14: GLenum = 0x84CE;
pub const GL_TEXTURE15: GLenum = 0x84CF;
pub const GL_TEXTURE16: GLenum = 0x84D0;
pub const GL_TEXTURE17: GLenum = 0x84D1;
pub const GL_TEXTURE18: GLenum = 0x84D2;
pub const GL_TEXTURE19: GLenum = 0x84D3;
pub const GL_TEXTURE20: GLenum = 0x84D4;
pub const GL_TEXTURE21: GLenum = 0x84D5;
pub const GL_TEXTURE22: GLenum = 0x84D6;
pub const GL_TEXTURE23: GLenum = 0x84D7;
pub const GL_TEXTURE24: GLenum = 0x84D8;
pub const GL_TEXTURE25: GLenum = 0x84D9;
pub const GL_TEXTURE26: GLenum = 0x84DA;
pub const GL_TEXTURE27: GLenum = 0x84DB;
pub const GL_TEXTURE28: GLenum = 0x84DC;
pub const GL_TEXTURE29: GLenum = 0x84DD;
pub const GL_TEXTURE30: GLenum = 0x84DE;
pub const GL_TEXTURE31: GLenum = 0x84DF;
pub const GL_ACTIVE_TEXTURE: GLenum = 0x84E0;
pub const GL_MULTISAMPLE: GLenum = 0x809D;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: GLenum = 0x809E;
pub const GL_SAMPLE_ALPHA_TO_ONE: GLenum = 0x809F;
pub const GL_SAMPLE_COVERAGE: GLenum = 0x80A0;
pub const GL_SAMPLE_BUFFERS: GLenum = 0x80A8;
pub const GL_SAMPLES: GLenum = 0x80A9;
pub const GL_SAMPLE_COVERAGE_VALUE: GLenum = 0x80AA;
pub const GL_SAMPLE_COVERAGE_INVERT: GLenum = 0x80AB;
pub const GL_TEXTURE_CUBE_MAP: GLenum = 0x8513;
pub const GL_TEXTURE_BINDING_CUBE_MAP: GLenum = 0x8514;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: GLenum = 0x8515;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: GLenum = 0x8516;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: GLenum = 0x8517;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: GLenum = 0x8518;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: GLenum = 0x8519;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: GLenum = 0x851A;
pub const GL_PROXY_TEXTURE_CUBE_MAP: GLenum = 0x851B;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: GLenum = 0x851C;
pub const GL_COMPRESSED_RGB: GLenum = 0x84ED;
pub const GL_COMPRESSED_RGBA: GLenum = 0x84EE;
pub const GL_TEXTURE_COMPRESSION_HINT: GLenum = 0x84EF;
pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE: GLenum = 0x86A0;
pub const GL_TEXTURE_COMPRESSED: GLenum = 0x86A1;
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A2;
pub const GL_COMPRESSED_TEXTURE_FORMATS: GLenum = 0x86A3;
pub const GL_CLAMP_TO_BORDER: GLenum = 0x812D;

// ============================================================================
// OpenGL 1.4 Constants
// ============================================================================

pub const GL_BLEND_DST_RGB: GLenum = 0x80C8;
pub const GL_BLEND_SRC_RGB: GLenum = 0x80C9;
pub const GL_BLEND_DST_ALPHA: GLenum = 0x80CA;
pub const GL_BLEND_SRC_ALPHA: GLenum = 0x80CB;
pub const GL_POINT_FADE_THRESHOLD_SIZE: GLenum = 0x8128;
pub const GL_DEPTH_COMPONENT16: GLenum = 0x81A5;
pub const GL_DEPTH_COMPONENT24: GLenum = 0x81A6;
pub const GL_DEPTH_COMPONENT32: GLenum = 0x81A7;
pub const GL_MIRRORED_REPEAT: GLenum = 0x8370;
pub const GL_MAX_TEXTURE_LOD_BIAS: GLenum = 0x84FD;
pub const GL_TEXTURE_LOD_BIAS: GLenum = 0x8501;
pub const GL_INCR_WRAP: GLenum = 0x8507;
pub const GL_DECR_WRAP: GLenum = 0x8508;
pub const GL_TEXTURE_DEPTH_SIZE: GLenum = 0x884A;
pub const GL_TEXTURE_COMPARE_MODE: GLenum = 0x884C;
pub const GL_TEXTURE_COMPARE_FUNC: GLenum = 0x884D;
pub const GL_POINT_SIZE_MIN: GLenum = 0x8126;
pub const GL_POINT_SIZE_MAX: GLenum = 0x8127;
pub const GL_POINT_DISTANCE_ATTENUATION: GLenum = 0x8129;
pub const GL_GENERATE_MIPMAP: GLenum = 0x8191;
pub const GL_GENERATE_MIPMAP_HINT: GLenum = 0x8192;
pub const GL_FOG_COORDINATE_SOURCE: GLenum = 0x8450;
pub const GL_FOG_COORDINATE: GLenum = 0x8451;
pub const GL_FRAGMENT_DEPTH: GLenum = 0x8452;
pub const GL_CURRENT_FOG_COORDINATE: GLenum = 0x8453;
pub const GL_FOG_COORDINATE_ARRAY_TYPE: GLenum = 0x8454;
pub const GL_FOG_COORDINATE_ARRAY_STRIDE: GLenum = 0x8455;
pub const GL_FOG_COORDINATE_ARRAY_POINTER: GLenum = 0x8456;
pub const GL_FOG_COORDINATE_ARRAY: GLenum = 0x8457;
pub const GL_COLOR_SUM: GLenum = 0x8458;
pub const GL_CURRENT_SECONDARY_COLOR: GLenum = 0x8459;
pub const GL_SECONDARY_COLOR_ARRAY_SIZE: GLenum = 0x845A;
pub const GL_SECONDARY_COLOR_ARRAY_TYPE: GLenum = 0x845B;
pub const GL_SECONDARY_COLOR_ARRAY_STRIDE: GLenum = 0x845C;
pub const GL_SECONDARY_COLOR_ARRAY_POINTER: GLenum = 0x845D;
pub const GL_SECONDARY_COLOR_ARRAY: GLenum = 0x845E;
pub const GL_TEXTURE_FILTER_CONTROL: GLenum = 0x8500;
pub const GL_DEPTH_TEXTURE_MODE: GLenum = 0x884B;
pub const GL_COMPARE_R_TO_TEXTURE: GLenum = 0x884E;
pub const GL_FUNC_ADD: GLenum = 0x8006;
pub const GL_FUNC_SUBTRACT: GLenum = 0x800A;
pub const GL_FUNC_REVERSE_SUBTRACT: GLenum = 0x800B;
pub const GL_MIN: GLenum = 0x8007;
pub const GL_MAX: GLenum = 0x8008;
pub const GL_CONSTANT_COLOR: GLenum = 0x8001;
pub const GL_ONE_MINUS_CONSTANT_COLOR: GLenum = 0x8002;
pub const GL_CONSTANT_ALPHA: GLenum = 0x8003;
pub const GL_ONE_MINUS_CONSTANT_ALPHA: GLenum = 0x8004;

// ============================================================================
// OpenGL 1.5 Constants
// ============================================================================

pub const GL_BUFFER_SIZE: GLenum = 0x8764;
pub const GL_BUFFER_USAGE: GLenum = 0x8765;
pub const GL_QUERY_COUNTER_BITS: GLenum = 0x8864;
pub const GL_CURRENT_QUERY: GLenum = 0x8865;
pub const GL_QUERY_RESULT: GLenum = 0x8866;
pub const GL_QUERY_RESULT_AVAILABLE: GLenum = 0x8867;
pub const GL_ARRAY_BUFFER: GLenum = 0x8892;
pub const GL_ELEMENT_ARRAY_BUFFER: GLenum = 0x8893;
pub const GL_ARRAY_BUFFER_BINDING: GLenum = 0x8894;
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: GLenum = 0x8895;
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: GLenum = 0x889F;
pub const GL_READ_ONLY: GLenum = 0x88B8;
pub const GL_WRITE_ONLY: GLenum = 0x88B9;
pub const GL_READ_WRITE: GLenum = 0x88BA;
pub const GL_BUFFER_ACCESS: GLenum = 0x88BB;
pub const GL_BUFFER_MAPPED: GLenum = 0x88BC;
pub const GL_BUFFER_MAP_POINTER: GLenum = 0x88BD;
pub const GL_STREAM_DRAW: GLenum = 0x88E0;
pub const GL_STREAM_READ: GLenum = 0x88E1;
pub const GL_STREAM_COPY: GLenum = 0x88E2;
pub const GL_STATIC_DRAW: GLenum = 0x88E4;
pub const GL_STATIC_READ: GLenum = 0x88E5;
pub const GL_STATIC_COPY: GLenum = 0x88E6;
pub const GL_DYNAMIC_DRAW: GLenum = 0x88E8;
pub const GL_DYNAMIC_READ: GLenum = 0x88E9;
pub const GL_DYNAMIC_COPY: GLenum = 0x88EA;
pub const GL_SAMPLES_PASSED: GLenum = 0x8914;
pub const GL_SRC1_ALPHA: GLenum = 0x8589;
pub const GL_VERTEX_ARRAY_BUFFER_BINDING: GLenum = 0x8896;
pub const GL_NORMAL_ARRAY_BUFFER_BINDING: GLenum = 0x8897;
pub const GL_COLOR_ARRAY_BUFFER_BINDING: GLenum = 0x8898;
pub const GL_INDEX_ARRAY_BUFFER_BINDING: GLenum = 0x8899;
pub const GL_TEXTURE_COORD_ARRAY_BUFFER_BINDING: GLenum = 0x889A;
pub const GL_EDGE_FLAG_ARRAY_BUFFER_BINDING: GLenum = 0x889B;
pub const GL_SECONDARY_COLOR_ARRAY_BUFFER_BINDING: GLenum = 0x889C;
pub const GL_FOG_COORDINATE_ARRAY_BUFFER_BINDING: GLenum = 0x889D;
pub const GL_WEIGHT_ARRAY_BUFFER_BINDING: GLenum = 0x889E;
pub const GL_FOG_COORD_SRC: GLenum = 0x8450;
pub const GL_FOG_COORD: GLenum = 0x8451;
pub const GL_CURRENT_FOG_COORD: GLenum = 0x8453;
pub const GL_FOG_COORD_ARRAY_TYPE: GLenum = 0x8454;
pub const GL_FOG_COORD_ARRAY_STRIDE: GLenum = 0x8455;
pub const GL_FOG_COORD_ARRAY_POINTER: GLenum = 0x8456;
pub const GL_FOG_COORD_ARRAY: GLenum = 0x8457;
pub const GL_FOG_COORD_ARRAY_BUFFER_BINDING: GLenum = 0x889D;
pub const GL_SRC0_RGB: GLenum = 0x8580;
pub const GL_SRC1_RGB: GLenum = 0x8581;
pub const GL_SRC2_RGB: GLenum = 0x8582;
pub const GL_SRC0_ALPHA: GLenum = 0x8588;
pub const GL_SRC2_ALPHA: GLenum = 0x858A;
