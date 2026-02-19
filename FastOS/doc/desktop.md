# FastOS — Desktop Architecture

> Desktop gráfico completo estilo Windows 11, escrito en Rust.

---

## Desktop Stack

```text
Screen (Monitor)
     ↑
Framebuffer Driver
     ↑
Compositor (alpha blending, z-order, damage tracking)
     ↑
Window Manager (create, move, resize, focus, decorations)
     ↑
Desktop Shell (taskbar, icons, wallpaper, start menu)
     ↑
Applications (terminal, files, settings, etc.)
```

---

## Compositor (`desktop/compositor.rs`)

El compositor es el motor de renderizado del desktop. Equivalente a:

- **Windows DWM** (Desktop Window Manager)
- **Wayland compositor** (wlroots, Mutter)
- **macOS Quartz Compositor**

### Responsabilidades

```text
1. Mantener lista de ventanas (z-order)
2. Renderizar cada ventana al back buffer
3. Alpha blending para transparencia
4. Damage tracking (solo re-renderizar áreas modificadas)
5. Cursor rendering (composited sobre ventanas)
6. VSync / double buffering
```

### Algoritmo de Composición

```text
for each frame:
  1. Check dirty regions
  2. Draw wallpaper (only dirty areas)
  3. For each window (bottom to top z-order):
     a. Clip to visible area
     b. Alpha blend window content
     c. Draw window decorations
  4. Draw taskbar
  5. Draw cursor on top
  6. Swap buffers (back → front)
```

### Estructuras

```rust
struct Compositor {
    back_buffer: &'static mut [u32],   // ARGB pixel buffer
    front_buffer: &'static mut [u32],  // Framebuffer (mapped)
    width: u32,
    height: u32,
    pitch: u32,
    windows: Vec<WindowId>,            // Z-ordered
    dirty_rects: Vec<Rect>,
    cursor_x: i32,
    cursor_y: i32,
}

struct Rect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}
```

### Alpha Blending

```text
Formula (per-pixel):
  result.r = (src.r * src.a + dst.r * (255 - src.a)) / 255
  result.g = (src.g * src.a + dst.g * (255 - src.a)) / 255
  result.b = (src.b * src.a + dst.b * (255 - src.a)) / 255

Pixel format: 0xAARRGGBB (32-bit ARGB)
```

---

## Window Manager (`desktop/window_manager.rs`)

### Responsabilidades

```text
1. Crear ventanas (título, posición, tamaño)
2. Destruir ventanas
3. Mover ventanas (mouse drag en title bar)
4. Redimensionar ventanas (mouse drag en bordes)
5. Focus management (click to focus)
6. Window decorations (title bar, close/min/max buttons)
7. Minimize / maximize / restore
8. Snap to edges (Win11 style)
```

### Window Structure

```rust
struct Window {
    id: WindowId,
    title: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    content_buffer: Vec<u32>,    // App renders here
    visible: bool,
    minimized: bool,
    maximized: bool,
    focused: bool,
    decorations: bool,
    resizable: bool,
}

struct WindowDecorations {
    title_bar_height: u32,       // 30px
    border_width: u32,           // 1px
    close_button: Rect,
    minimize_button: Rect,
    maximize_button: Rect,
    title_area: Rect,
}
```

### Window Decorations (Win11 Style)

```text
┌──────────────────────────────────────────┐
│ ■ Title                      — □ ✕      │  ← Title bar (30px)
├──────────────────────────────────────────┤
│                                          │
│           Application Content            │
│                                          │
│                                          │
└──────────────────────────────────────────┘

Colors:
  Title bar (focused):   #FFFFFF (white) with dark text
  Title bar (unfocused): #F0F0F0 (light grey)
  Close button hover:    #E81123 (red)
  Border:                #D1D1D1 (subtle grey)
  Shadow:                4px blur, 20% opacity
```

---

## Desktop Shell (`desktop/shell.rs`)

### Responsabilidades

```text
1. Wallpaper rendering (solid color → image)
2. Desktop icons (grid layout)
3. Taskbar (bottom, centered like Win11)
4. Start menu (popup)
5. System tray (clock, network, sound, battery)
6. Notification area
7. Right-click context menu
```

### Taskbar Layout (Win11 Style)

```text
┌────────────────────────────────────────────────────────────────────────────┐
│                    ■ 🔍 Search    📁 🖥️ ⚙️           🔊 🌐 12:00 PM    │
│                    ↑              ↑                    ↑                   │
│                  Start     Pinned Apps           System Tray              │
└────────────────────────────────────────────────────────────────────────────┘

Taskbar height: 48px
Position: Bottom, centered content
Background: Semi-transparent (acrylic effect)
```

### Desktop Icons

```text
Grid layout:
  - Column spacing: 80px
  - Row spacing: 90px
  - Icon size: 48x48 (from SVG rasterized)
  - Label: below icon, centered, white text with shadow

Default icons:
  Documents, Downloads, Terminal, Files, Settings, Calculator

Icon rendering:
  SVG → rasterize at boot → 48x48 ARGB bitmap → cache
```

### Start Menu (Win11 Style)

```text
┌─────────────────────────────────┐
│        Pinned Apps              │
│  ┌──┐ ┌──┐ ┌──┐ ┌──┐ ┌──┐    │
│  │📁│ │🖥│ │⚙│ │📝│ │🔢│    │
│  └──┘ └──┘ └──┘ └──┘ └──┘    │
│                                 │
│        Recommended              │
│  📄 Recent file 1              │
│  📄 Recent file 2              │
│                                 │
├─────────────────────────────────┤
│  👤 Admin          ⏻ Power     │
└─────────────────────────────────┘

Size: 600x700px (centered above taskbar)
Background: Acrylic blur
Corner radius: 8px
```

---

## Cursor (`desktop/cursor.rs`)

```text
Default cursor: Arrow (12x19 pixels)
Resize cursors: ↔ ↕ ⤡ ⤢
Text cursor: I-beam
Hand cursor: Pointer (for links)

Rendering: Software cursor (composited on top)
Future: Hardware cursor via VGA registers
```

---

## Color Palette (Win11 Inspired)

```text
Accent:          #0078D4 (Windows Blue)
Background:      #F3F3F3 (Light grey)
Surface:         #FFFFFF (White)
Text primary:    #1A1A1A (Near black)
Text secondary:  #666666 (Grey)
Border:          #D1D1D1 (Subtle grey)
Taskbar:         #1C1C1C (Dark, semi-transparent)
Start menu:      #2C2C2C (Dark, acrylic)
Close button:    #E81123 (Red)
Success:         #107C10 (Green)
Warning:         #FFB900 (Yellow)
```

---

## Font System

```text
Initial: Bitmap font 8x16 (built into kernel)
  - 256 ASCII characters
  - 1-bit per pixel
  - Stored as const array

Future: TrueType font rendering
  - Font parser
  - Glyph rasterizer
  - Font cache
  - Anti-aliasing
```

---

## Event System

```text
Input events flow:
  Hardware IRQ → Driver → Event Queue → Window Manager → Focused App

Event types:
  - KeyDown(scancode, keycode, modifiers)
  - KeyUp(scancode, keycode, modifiers)
  - MouseMove(x, y, dx, dy)
  - MouseButtonDown(button)
  - MouseButtonUp(button)
  - MouseScroll(delta)
  - WindowClose(window_id)
  - WindowResize(window_id, width, height)
  - WindowFocus(window_id)
```
