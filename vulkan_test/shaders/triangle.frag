#version 450

// Fragment shader para triángulo con gradiente
// ADead-BIB Vulkan Test

// Input del vertex shader (interpolado)
layout(location = 0) in vec3 fragColor;

// Output al framebuffer
layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(fragColor, 1.0);
}
