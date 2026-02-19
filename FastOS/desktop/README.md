# FastOS Desktop Environment

Windows 11-style desktop for FastOS, rendered via VGA text mode (80x25).

## Folder Structure

```
desktop/
├── Documents/     # User documents
├── Downloads/     # Downloaded files
├── Pictures/      # Images and photos
├── Music/         # Audio files
├── Videos/        # Video files
├── Desktop/       # Desktop shortcuts
└── icons/         # SVG icons (Win11 style)
    ├── fastos-logo.svg
    ├── app-start.svg
    ├── app-terminal.svg
    ├── app-files.svg
    ├── app-settings.svg
    ├── app-calculator.svg
    ├── folder-documents.svg
    ├── folder-downloads.svg
    ├── folder-pictures.svg
    ├── folder-music.svg
    └── folder-videos.svg
```

## Desktop Layout (VGA 80x25)

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│ •█ Documents    ↓█ Downloads                                                     │ Row 2-3
│                                                                                  │
│                                                                                  │
│                                                                                  │
│ >_ Terminal     ██ Files                                                         │ Row 7-8
│                                                                                  │
│                                                                                  │
│                                                                                  │
│ ☼☼ Settings     █= Calc                          FastOS                          │ Row 12-13
│                                                   64-bit                         │
│                                                   FsOS                           │
│                                                   ADead-BIB                      │
│                                                                                  │
├──────────────────────────────────────────────────────────────────────────────────┤ Row 23
│                                 ■ O Search  >_ •     ↑♪ 12:00                    │ Row 24
└──────────────────────────────────────────────────────────────────────────────────┘
```

## Color Scheme

| Element        | Attribute | Description              |
|----------------|-----------|--------------------------|
| Desktop bg     | 0x9F      | Bright white on light blue |
| Taskbar        | 0x8F      | White on dark grey       |
| Start button   | 0x8B      | Cyan on dark grey        |
| Search text    | 0x87      | Grey on dark grey        |
| System tray    | 0x87      | Grey on dark grey        |
| Clock          | 0x8F      | White on dark grey       |
| Icon symbols   | 0x9E/9A/9B | Yellow/Green/Cyan on blue |
| Icon labels    | 0x9F      | White on light blue      |
| Info panel     | 0x9B/97/98 | Cyan/Grey/DarkGrey       |

## Boot Flow

1. **Stage1** (512 bytes): Splash screen → Load stage2 → Jump to 0x8000
2. **Stage2** (1319 bytes): Login screen → Desktop → Q=Shutdown

## Keyboard

- **Any key**: Sign in from login screen
- **Q/q**: Shutdown from desktop

## Build

```powershell
cargo run -- boot FastOS\boot\stage1.adB -o FastOS\build\stage1.bin
cargo run -- flat FastOS\boot\stage2.adB -o FastOS\build\stage2.bin
# Combine into disk image (PowerShell)
# Then: qemu-system-x86_64 -drive format=raw,file=FastOS\build\fastos.bin
```
