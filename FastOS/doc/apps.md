# FastOS — Applications

> Apps escritas en ADead-BIB + C + Rust. Tres niveles de desarrollo.

---

## App Architecture

```text
┌─────────────────────────────────────┐
│           Application               │
│  (ADead-BIB / C / Rust)             │
├─────────────────────────────────────┤
│         App Framework (Rust)        │
│  Window │ Widgets │ Events │ Draw   │
├─────────────────────────────────────┤
│       Window Manager (Rust)         │
├─────────────────────────────────────┤
│        Compositor (Rust)            │
├─────────────────────────────────────┤
│       Framebuffer (Rust)            │
└─────────────────────────────────────┘
```

---

## App Languages

### ADead-BIB Apps — Ultra Low Level

```text
Use cases:
  - Hardware diagnostic tools
  - Boot utilities
  - Performance-critical tools
  - Direct hardware access apps

Example: disk benchmark, memory tester
```

### C Apps — Portable

```text
Use cases:
  - Third-party software ports
  - Libraries with C ABI
  - Legacy compatibility

Example: ported utilities, compression tools
```

### Rust Apps — Native

```text
Use cases:
  - All core system apps
  - Desktop applications
  - System services

Example: terminal, file manager, settings, editor
```

---

## Core Applications

### Terminal (`apps/terminal/`)

```text
Features:
  - VT100/ANSI escape code support
  - Scrollback buffer (1000 lines)
  - Copy/paste
  - Tab completion
  - Command history
  - Customizable colors
  - Font rendering (8x16 bitmap)

Built-in commands:
  ls, cd, pwd, cat, echo, clear, help
  ps, kill, top
  mkdir, rm, cp, mv
  date, uptime, whoami
  shutdown, reboot

Window: Resizable, dark theme
```

### File Manager (`apps/file_manager/`)

```text
Features:
  - Dual-pane or single-pane view
  - Icon view / list view / detail view
  - Navigation (back, forward, up, path bar)
  - File operations (copy, move, delete, rename)
  - File properties dialog
  - Preview pane
  - Search
  - Drag and drop

Layout (Win11 style):
  ┌──────────────────────────────────────┐
  │ ← → ↑  /user/Documents              │
  ├──────┬───────────────────────────────┤
  │ Quick│  📁 Projects                  │
  │ Docs │  📁 Photos                    │
  │ Down │  📄 readme.txt                │
  │ Pics │  📄 notes.md                  │
  │ Music│                               │
  └──────┴───────────────────────────────┘
```

### Settings (`apps/settings/`)

```text
Categories:
  - System (About, Display, Sound)
  - Personalization (Background, Colors, Taskbar)
  - Accounts (User info, Password)
  - Time & Language
  - Accessibility
  - Privacy & Security
  - Network

Layout:
  ┌──────────────────────────────────────┐
  │ ⚙ Settings                          │
  ├──────────┬───────────────────────────┤
  │ System   │  Display                  │
  │ Personal │  Resolution: 1024x768     │
  │ Accounts │  Color depth: 32-bit      │
  │ Time     │  Refresh: 60 Hz           │
  │ Privacy  │                           │
  │ Network  │  [Apply]  [Cancel]        │
  └──────────┴───────────────────────────┘
```

### Calculator (`apps/calculator/`)

```text
Modes:
  - Standard (basic arithmetic)
  - Scientific (trig, log, pow)
  - Programmer (hex, bin, oct)

Features:
  - Keyboard input
  - History
  - Copy result

Layout:
  ┌────────────────────┐
  │              123.45 │
  ├────────────────────┤
  │  MC  MR  M+  M-   │
  │  C   ±   %   ÷    │
  │  7   8   9   ×    │
  │  4   5   6   −    │
  │  1   2   3   +    │
  │  0       .   =    │
  └────────────────────┘
```

### Text Editor (`apps/editor/`)

```text
Features:
  - Syntax highlighting (basic)
  - Line numbers
  - Find & replace
  - Undo/redo
  - Multiple tabs
  - Status bar (line, column, encoding)
  - Save/open files

Layout:
  ┌──────────────────────────────────────┐
  │ 📝 Editor — readme.txt              │
  ├──────────────────────────────────────┤
  │  1 │ # Welcome to FastOS            │
  │  2 │                                │
  │  3 │ This is a text file.           │
  │  4 │ █                              │
  │    │                                │
  ├──────────────────────────────────────┤
  │ Ln 4, Col 1  │  UTF-8  │  LF       │
  └──────────────────────────────────────┘
```

### System Info (`apps/sysinfo/`)

```text
Displays:
  - OS: FastOS v1.0
  - Kernel: fastos-kernel 1.0.0
  - Architecture: x86-64
  - CPU: (detected via CPUID)
  - Memory: total / used / free
  - Uptime
  - Disk: total / used / free
  - Display: resolution, bpp
  - Stack: ADead-BIB + Rust + C
```

---

## App Framework

### Widget System

```rust
trait Widget {
    fn draw(&self, fb: &mut Framebuffer, x: u32, y: u32);
    fn handle_event(&mut self, event: &Event) -> bool;
    fn bounds(&self) -> Rect;
}

// Built-in widgets:
struct Button { label: String, on_click: fn() }
struct Label { text: String, color: u32 }
struct TextInput { text: String, cursor: usize }
struct ListView { items: Vec<String>, selected: usize }
struct ScrollBar { position: f32, total: f32 }
struct CheckBox { checked: bool, label: String }
struct ProgressBar { value: f32 }
```

### App Lifecycle

```text
1. App::new() — Initialize
2. App::on_create(window) — Window assigned
3. App::on_event(event) — Handle input
4. App::on_draw(framebuffer) — Render
5. App::on_destroy() — Cleanup
```

---

## Future Apps

```text
- Web browser (basic HTML renderer)
- Image viewer (BMP, PNG)
- Music player (WAV, basic audio)
- Paint (drawing tool)
- Clock / Calendar
- Task manager (process list, CPU/memory graphs)
- Package manager GUI
```
