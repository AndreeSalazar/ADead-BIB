//! OpenGL State Optimizer for ADead-BIB
//! 
//! - State cache: eliminate redundant GL calls
//! - Draw call batcher: coalesce compatible draw calls
//! - Zero overhead when not in use

use super::types::*;

/// Cached OpenGL state to eliminate redundant API calls
#[derive(Debug, Clone)]
pub struct GLStateCache {
    pub current_program: GLuint,
    pub current_vao: GLuint,
    pub current_read_fbo: GLuint,
    pub current_draw_fbo: GLuint,
    pub current_textures: [GLuint; 32],
    pub active_texture_unit: GLuint,
    pub current_array_buffer: GLuint,
    pub current_element_buffer: GLuint,
    pub current_uniform_buffer: GLuint,
    pub current_ssbo: GLuint,
    pub blend_enabled: bool,
    pub blend_src_rgb: GLenum,
    pub blend_dst_rgb: GLenum,
    pub blend_src_alpha: GLenum,
    pub blend_dst_alpha: GLenum,
    pub depth_test_enabled: bool,
    pub depth_write_enabled: bool,
    pub depth_func: GLenum,
    pub cull_face_enabled: bool,
    pub cull_face_mode: GLenum,
    pub front_face: GLenum,
    pub scissor_enabled: bool,
    pub viewport_x: GLint,
    pub viewport_y: GLint,
    pub viewport_width: GLsizei,
    pub viewport_height: GLsizei,
    pub clear_color: [GLfloat; 4],
    pub polygon_mode: GLenum,
    pub line_width: GLfloat,
}

impl Default for GLStateCache {
    fn default() -> Self {
        Self {
            current_program: 0, current_vao: 0,
            current_read_fbo: 0, current_draw_fbo: 0,
            current_textures: [0; 32], active_texture_unit: 0,
            current_array_buffer: 0, current_element_buffer: 0,
            current_uniform_buffer: 0, current_ssbo: 0,
            blend_enabled: false,
            blend_src_rgb: 1, blend_dst_rgb: 0,
            blend_src_alpha: 1, blend_dst_alpha: 0,
            depth_test_enabled: false, depth_write_enabled: true,
            depth_func: 0x0201,
            cull_face_enabled: false, cull_face_mode: 0x0405,
            front_face: 0x0901,
            scissor_enabled: false,
            viewport_x: 0, viewport_y: 0,
            viewport_width: 0, viewport_height: 0,
            clear_color: [0.0; 4],
            polygon_mode: 0x1B02,
            line_width: 1.0,
        }
    }
}

impl GLStateCache {
    pub fn new() -> Self { Self::default() }

    #[inline] pub fn should_use_program(&self, p: GLuint) -> bool { self.current_program != p }
    #[inline] pub fn should_bind_vao(&self, v: GLuint) -> bool { self.current_vao != v }
    #[inline] pub fn should_bind_texture(&self, unit: usize, tex: GLuint) -> bool {
        unit >= 32 || self.current_textures[unit] != tex
    }

    #[inline] pub fn track_use_program(&mut self, p: GLuint) { self.current_program = p; }
    #[inline] pub fn track_bind_vao(&mut self, v: GLuint) { self.current_vao = v; }
    #[inline] pub fn track_bind_texture(&mut self, unit: usize, tex: GLuint) {
        if unit < 32 { self.current_textures[unit] = tex; }
    }

    pub fn invalidate(&mut self) { *self = Self::default(); }
}

/// Draw command for batching
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawCommand {
    pub mode: GLenum,
    pub first: GLint,
    pub count: GLsizei,
    pub instance_count: GLsizei,
    pub base_instance: GLuint,
    pub base_vertex: GLint,
    pub vao: GLuint,
    pub program: GLuint,
}

/// Draw call batcher — merges compatible draw calls
#[derive(Debug)]
pub struct DrawBatcher {
    commands: Vec<DrawCommand>,
}

impl DrawBatcher {
    pub fn new(capacity: usize) -> Self {
        Self { commands: Vec::with_capacity(capacity) }
    }

    pub fn push(&mut self, cmd: DrawCommand) {
        if let Some(last) = self.commands.last_mut() {
            if last.mode == cmd.mode && last.vao == cmd.vao && last.program == cmd.program
                && last.instance_count == 1 && cmd.instance_count == 1
                && last.first + last.count == cmd.first
            {
                last.count += cmd.count;
                return;
            }
        }
        self.commands.push(cmd);
    }

    pub fn commands(&self) -> &[DrawCommand] { &self.commands }
    pub fn len(&self) -> usize { self.commands.len() }
    pub fn is_empty(&self) -> bool { self.commands.is_empty() }
    pub fn clear(&mut self) { self.commands.clear(); }
}

impl Default for DrawBatcher {
    fn default() -> Self { Self::new(4096) }
}

/// Performance statistics
#[derive(Debug, Clone, Default)]
pub struct OptimizerStats {
    pub total_draw_calls: u64,
    pub batched_draw_calls: u64,
    pub skipped_state_changes: u64,
    pub total_state_changes: u64,
}

impl OptimizerStats {
    pub fn new() -> Self { Self::default() }
    pub fn state_change_savings(&self) -> f64 {
        if self.total_state_changes == 0 { 0.0 }
        else { self.skipped_state_changes as f64 / self.total_state_changes as f64 }
    }
    pub fn reset(&mut self) { *self = Self::default(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_cache() {
        let mut c = GLStateCache::new();
        assert!(c.should_use_program(5));
        c.track_use_program(5);
        assert!(!c.should_use_program(5));
    }

    #[test]
    fn test_draw_batcher_merge() {
        let mut b = DrawBatcher::new(100);
        let cmd = |first, count| DrawCommand {
            mode: 0x0004, first, count, instance_count: 1,
            base_instance: 0, base_vertex: 0, vao: 1, program: 1,
        };
        b.push(cmd(0, 3));
        b.push(cmd(3, 6));
        assert_eq!(b.len(), 1);
        assert_eq!(b.commands()[0].count, 9);
    }
}
