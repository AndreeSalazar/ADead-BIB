#version 450

// Vertex shader para triángulo básico
// ADead-BIB Vulkan Test

// Posiciones hardcoded del triángulo (NDC)
vec2 positions[3] = vec2[](
    vec2(0.0, -0.5),   // Top
    vec2(0.5, 0.5),    // Bottom right
    vec2(-0.5, 0.5)    // Bottom left
);

// Colores por vértice (RGB)
vec3 colors[3] = vec3[](
    vec3(1.0, 0.0, 0.0),  // Red
    vec3(0.0, 1.0, 0.0),  // Green
    vec3(0.0, 0.0, 1.0)   // Blue
);

// Output al fragment shader
layout(location = 0) out vec3 fragColor;

void main() {
    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
    fragColor = colors[gl_VertexIndex];
}
