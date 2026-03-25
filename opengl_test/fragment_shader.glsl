// fragment_shader.glsl — ADead-BIB OpenGL Cube
// GLSL 330 Core — Fragment Shader
// Iluminación Phong (ambient + diffuse + specular)

#version 330 core

in vec3 vColor;
in vec3 vNormal;
in vec3 vFragPos;

out vec4 FragColor;

// Luz
uniform vec3 uLightPos;
uniform vec3 uLightColor;
uniform vec3 uViewPos;

void main() {
    // Ambient
    float ambientStrength = 0.15;
    vec3 ambient = ambientStrength * uLightColor;
    
    // Diffuse
    vec3 norm = normalize(vNormal);
    vec3 lightDir = normalize(uLightPos - vFragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * uLightColor;
    
    // Specular (Blinn-Phong)
    float specularStrength = 0.6;
    vec3 viewDir = normalize(uViewPos - vFragPos);
    vec3 halfwayDir = normalize(lightDir + viewDir);
    float spec = pow(max(dot(norm, halfwayDir), 0.0), 64.0);
    vec3 specular = specularStrength * spec * uLightColor;
    
    // Final color
    vec3 result = (ambient + diffuse + specular) * vColor;
    FragColor = vec4(result, 1.0);
}
