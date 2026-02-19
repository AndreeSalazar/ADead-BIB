// ============================================================
// FastOS — Compositor (Fase 5)
// ============================================================
// Manages window layers, z-order, damage tracking, and
// composites all surfaces to the framebuffer.
// ============================================================

use crate::drivers::framebuffer;

/// Maximum number of layers the compositor can manage
const MAX_LAYERS: usize = 16;

/// Damage rectangle — region that needs redrawing
#[derive(Clone, Copy)]
pub struct DamageRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl DamageRect {
    pub const fn empty() -> Self {
        DamageRect { x: 0, y: 0, w: 0, h: 0 }
    }

    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        DamageRect { x, y, w, h }
    }

    pub fn is_empty(&self) -> bool {
        self.w == 0 || self.h == 0
    }

    /// Merge two damage rects into a bounding rect
    pub fn union(&self, other: &DamageRect) -> DamageRect {
        if self.is_empty() { return *other; }
        if other.is_empty() { return *self; }
        let x1 = if self.x < other.x { self.x } else { other.x };
        let y1 = if self.y < other.y { self.y } else { other.y };
        let x2_a = self.x + self.w;
        let x2_b = other.x + other.w;
        let x2 = if x2_a > x2_b { x2_a } else { x2_b };
        let y2_a = self.y + self.h;
        let y2_b = other.y + other.h;
        let y2 = if y2_a > y2_b { y2_a } else { y2_b };
        DamageRect { x: x1, y: y1, w: x2 - x1, h: y2 - y1 }
    }

    /// Check if two rects overlap
    pub fn intersects(&self, other: &DamageRect) -> bool {
        !(self.x + self.w <= other.x || other.x + other.w <= self.x ||
          self.y + self.h <= other.y || other.y + other.h <= self.y)
    }
}

/// A compositing layer (one per window or UI element)
#[derive(Clone, Copy)]
pub struct Layer {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub z_order: i32,
    pub visible: bool,
    pub opacity: u8,       // 0 = transparent, 255 = opaque
    pub dirty: bool,
    pub bg_color: u32,
}

impl Layer {
    pub const fn empty() -> Self {
        Layer {
            id: 0, x: 0, y: 0, w: 0, h: 0,
            z_order: 0, visible: false, opacity: 255,
            dirty: false, bg_color: 0xFFFFFFFF,
        }
    }

    pub fn bounds(&self) -> DamageRect {
        DamageRect { x: self.x, y: self.y, w: self.w, h: self.h }
    }
}

/// The compositor manages all layers and composites them
pub struct Compositor {
    layers: [Layer; MAX_LAYERS],
    layer_count: usize,
    damage: DamageRect,
    next_id: u32,
    screen_w: u32,
    screen_h: u32,
}

static mut COMPOSITOR: Compositor = Compositor {
    layers: [Layer::empty(); MAX_LAYERS],
    layer_count: 0,
    damage: DamageRect::empty(),
    next_id: 1,
    screen_w: 0,
    screen_h: 0,
};

/// Initialize the compositor
pub fn init() {
    unsafe {
        COMPOSITOR.screen_w = framebuffer::width();
        COMPOSITOR.screen_h = framebuffer::height();
        COMPOSITOR.layer_count = 0;
        COMPOSITOR.next_id = 1;
        COMPOSITOR.damage = DamageRect::new(0, 0, COMPOSITOR.screen_w, COMPOSITOR.screen_h);
    }
}

/// Create a new layer, returns its ID
pub fn create_layer(x: u32, y: u32, w: u32, h: u32, z_order: i32, bg_color: u32) -> u32 {
    unsafe {
        if COMPOSITOR.layer_count >= MAX_LAYERS { return 0; }
        let id = COMPOSITOR.next_id;
        COMPOSITOR.next_id += 1;

        let layer = Layer {
            id, x, y, w, h, z_order,
            visible: true, opacity: 255,
            dirty: true, bg_color,
        };

        COMPOSITOR.layers[COMPOSITOR.layer_count] = layer;
        COMPOSITOR.layer_count += 1;

        // Sort by z-order
        sort_layers();

        // Mark damage
        add_damage(DamageRect::new(x, y, w, h));

        id
    }
}

/// Destroy a layer by ID
pub fn destroy_layer(id: u32) {
    unsafe {
        let mut found = false;
        let mut damage = DamageRect::empty();
        for i in 0..COMPOSITOR.layer_count {
            if COMPOSITOR.layers[i].id == id {
                damage = COMPOSITOR.layers[i].bounds();
                // Shift remaining layers down
                for j in i..(COMPOSITOR.layer_count - 1) {
                    COMPOSITOR.layers[j] = COMPOSITOR.layers[j + 1];
                }
                COMPOSITOR.layers[COMPOSITOR.layer_count - 1] = Layer::empty();
                COMPOSITOR.layer_count -= 1;
                found = true;
                break;
            }
        }
        if found {
            add_damage(damage);
        }
    }
}

/// Move a layer to a new position
pub fn move_layer(id: u32, new_x: u32, new_y: u32) {
    unsafe {
        for i in 0..COMPOSITOR.layer_count {
            if COMPOSITOR.layers[i].id == id {
                let old = COMPOSITOR.layers[i].bounds();
                COMPOSITOR.layers[i].x = new_x;
                COMPOSITOR.layers[i].y = new_y;
                COMPOSITOR.layers[i].dirty = true;
                let new_bounds = COMPOSITOR.layers[i].bounds();
                add_damage(old.union(&new_bounds));
                return;
            }
        }
    }
}

/// Resize a layer
pub fn resize_layer(id: u32, new_w: u32, new_h: u32) {
    unsafe {
        for i in 0..COMPOSITOR.layer_count {
            if COMPOSITOR.layers[i].id == id {
                let old = COMPOSITOR.layers[i].bounds();
                COMPOSITOR.layers[i].w = new_w;
                COMPOSITOR.layers[i].h = new_h;
                COMPOSITOR.layers[i].dirty = true;
                let new_bounds = COMPOSITOR.layers[i].bounds();
                add_damage(old.union(&new_bounds));
                return;
            }
        }
    }
}

/// Change z-order of a layer (bring to front = high z)
pub fn set_z_order(id: u32, z: i32) {
    unsafe {
        for i in 0..COMPOSITOR.layer_count {
            if COMPOSITOR.layers[i].id == id {
                COMPOSITOR.layers[i].z_order = z;
                COMPOSITOR.layers[i].dirty = true;
                add_damage(COMPOSITOR.layers[i].bounds());
                break;
            }
        }
        sort_layers();
    }
}

/// Bring a layer to the front
pub fn bring_to_front(id: u32) {
    unsafe {
        let mut max_z = 0i32;
        for i in 0..COMPOSITOR.layer_count {
            if COMPOSITOR.layers[i].z_order > max_z {
                max_z = COMPOSITOR.layers[i].z_order;
            }
        }
        set_z_order(id, max_z + 1);
    }
}

/// Add a damage region
pub fn add_damage(rect: DamageRect) {
    unsafe {
        COMPOSITOR.damage = COMPOSITOR.damage.union(&rect);
    }
}

/// Composite all visible layers to the framebuffer
/// Only redraws the damaged region for efficiency
pub fn composite() {
    unsafe {
        if COMPOSITOR.damage.is_empty() { return; }

        let dx = COMPOSITOR.damage.x;
        let dy = COMPOSITOR.damage.y;
        let dw = COMPOSITOR.damage.w;
        let dh = COMPOSITOR.damage.h;

        // For each pixel in the damage region, composite layers bottom-to-top
        for py in dy..(dy + dh) {
            if py >= COMPOSITOR.screen_h { break; }
            for px in dx..(dx + dw) {
                if px >= COMPOSITOR.screen_w { break; }

                // Start with desktop background (will be set by shell)
                let mut color = 0xFF1E1E2E_u32; // Dark background default

                // Composite each visible layer (already sorted by z-order)
                for i in 0..COMPOSITOR.layer_count {
                    let layer = &COMPOSITOR.layers[i];
                    if !layer.visible { continue; }
                    if px < layer.x || px >= layer.x + layer.w { continue; }
                    if py < layer.y || py >= layer.y + layer.h { continue; }

                    // This pixel is within the layer
                    if layer.opacity == 255 {
                        color = layer.bg_color;
                    } else if layer.opacity > 0 {
                        // Alpha blend
                        let alpha = layer.opacity as u32;
                        let inv = 255 - alpha;
                        let sr = (layer.bg_color >> 16) & 0xFF;
                        let sg = (layer.bg_color >> 8) & 0xFF;
                        let sb = layer.bg_color & 0xFF;
                        let dr = (color >> 16) & 0xFF;
                        let dg = (color >> 8) & 0xFF;
                        let db = color & 0xFF;
                        let r = (sr * alpha + dr * inv) / 255;
                        let g = (sg * alpha + dg * inv) / 255;
                        let b = (sb * alpha + db * inv) / 255;
                        color = 0xFF000000 | (r << 16) | (g << 8) | b;
                    }
                }

                framebuffer::put_pixel(px, py, color);
            }
        }

        // Clear damage
        COMPOSITOR.damage = DamageRect::empty();
        for i in 0..COMPOSITOR.layer_count {
            COMPOSITOR.layers[i].dirty = false;
        }
    }
}

/// Force full screen redraw
pub fn invalidate_all() {
    unsafe {
        COMPOSITOR.damage = DamageRect::new(0, 0, COMPOSITOR.screen_w, COMPOSITOR.screen_h);
    }
}

/// Get a layer by ID
pub fn get_layer(id: u32) -> Option<Layer> {
    unsafe {
        for i in 0..COMPOSITOR.layer_count {
            if COMPOSITOR.layers[i].id == id {
                return Some(COMPOSITOR.layers[i]);
            }
        }
        None
    }
}

/// Sort layers by z-order (insertion sort, small N)
fn sort_layers() {
    unsafe {
        let n = COMPOSITOR.layer_count;
        for i in 1..n {
            let key = COMPOSITOR.layers[i];
            let mut j = i;
            while j > 0 && COMPOSITOR.layers[j - 1].z_order > key.z_order {
                COMPOSITOR.layers[j] = COMPOSITOR.layers[j - 1];
                j -= 1;
            }
            COMPOSITOR.layers[j] = key;
        }
    }
}
