// ============================================================================
// ADead-BIB GPU Shader - Rectangle Rendering
// ============================================================================
// Shader WGSL para renderizar rectángulos directamente en GPU
// Usado por el motor de juegos ADead-BIB
//
// Author: Eddi Andreé Salazar Matos 🇵🇪
// ============================================================================

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 0.0, 1.0);
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
