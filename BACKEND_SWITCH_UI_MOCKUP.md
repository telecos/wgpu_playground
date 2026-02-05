# Backend Switch UI - Visual Mockup

## Top Menu Bar (Before Changes)
```
┌──────────────────────────────────────────────────────────────────────────┐
│ 🎮 WebGPU Playground                     File: [💾] [📂] [filename.json] │
└──────────────────────────────────────────────────────────────────────────┘
```

## Top Menu Bar (After Changes - wgpu-rs Active)
```
┌──────────────────────────────────────────────────────────────────────────┐
│ 🎮 WebGPU Playground │ Backend: 🦀 wgpu-rs   File: [💾] [📂] [filename] │
│                      │          (in blue)                                 │
└──────────────────────────────────────────────────────────────────────────┘
```

## Top Menu Bar (After Changes - Dawn Active)
```
┌──────────────────────────────────────────────────────────────────────────┐
│ 🎮 WebGPU Playground │ Backend: 🌅 Dawn Native File: [💾] [📂] [filename] │
│                      │          (in orange)                               │
└──────────────────────────────────────────────────────────────────────────┘
```

## Settings Panel (Before Changes)
```
╔════════════════════════════════════════╗
║        ⚙️ Settings                     ║
╠════════════════════════════════════════╣
║                                        ║
║ Theme                                  ║
║ Choose your preferred UI theme:        ║
║                                        ║
║ Theme: [☀️ Light      ▼]              ║
║                                        ║
║ Theme changes are applied immediately  ║
║ and saved automatically.               ║
║                                        ║
╚════════════════════════════════════════╝
```

## Settings Panel (After Changes - Without Dawn Feature)
```
╔══════════════════════════════════════════════════════════════╗
║              ⚙️ Settings                                     ║
╠══════════════════════════════════════════════════════════════╣
║                                                              ║
║ Theme                                                        ║
║ Choose your preferred UI theme:                              ║
║                                                              ║
║ Theme: [☀️ Light                               ▼]           ║
║                                                              ║
║ Theme changes are applied immediately and saved              ║
║ automatically.                                               ║
║                                                              ║
║ ────────────────────────────────────────────────────────     ║
║                                                              ║
║ 🔧 WebGPU Backend                                            ║
║ Select the WebGPU implementation backend:                    ║
║                                                              ║
║ Current Backend: wgpu-rs (in blue)                         ║
║                                                              ║
║ Select Backend: [🦀 wgpu-rs (Rust impl...)    ▼]         ║
║   ┌──────────────────────────────────────────┐              ║
║   │ 🦀 wgpu-rs (Rust implementation)       │              ║
║   │ 🌅 Dawn Native (Not Available)   ⊘      │              ║
║   └──────────────────────────────────────────┘              ║
║                                                              ║
║ wgpu-rs (Rust implementation, used by Firefox)               ║
║                                                              ║
║ ℹ️ Dawn support not available (compile with --features dawn)║
║ (in yellow)                                                  ║
║                                                              ║
║ 💡 Tip: Available backends:                                 ║
║   ✓ wgpu (active)                                           ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
```

## Settings Panel (After Changes - With Dawn Feature, wgpu Active)
```
╔══════════════════════════════════════════════════════════════╗
║              ⚙️ Settings                                     ║
╠══════════════════════════════════════════════════════════════╣
║                                                              ║
║ Theme                                                        ║
║ Choose your preferred UI theme:                              ║
║                                                              ║
║ Theme: [🌙 Dark                                ▼]           ║
║                                                              ║
║ Theme changes are applied immediately and saved              ║
║ automatically.                                               ║
║                                                              ║
║ ────────────────────────────────────────────────────────     ║
║                                                              ║
║ 🔧 WebGPU Backend                                            ║
║ Select the WebGPU implementation backend:                    ║
║                                                              ║
║ Current Backend: wgpu-rs (in blue)                         ║
║                                                              ║
║ Select Backend: [🦀 wgpu-rs (Rust impl...)    ▼]         ║
║   ┌──────────────────────────────────────────┐              ║
║   │ 🦀 wgpu-rs (Rust implementation)       │              ║
║   │ 🌅 Dawn Native (C++ implementation)      │              ║
║   └──────────────────────────────────────────┘              ║
║                                                              ║
║ wgpu-rs (Rust implementation, used by Firefox)               ║
║                                                              ║
║ ✓ Dawn support is compiled in (in green)                    ║
║                                                              ║
║ 💡 Tip: Available backends:                                 ║
║   ✓ wgpu (active)                                           ║
║   ○ Dawn (inactive)                                         ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
```

## Settings Panel (After Changes - User Selects Different Backend)
```
╔══════════════════════════════════════════════════════════════╗
║              ⚙️ Settings                                     ║
╠══════════════════════════════════════════════════════════════╣
║                                                              ║
║ 🔧 WebGPU Backend                                            ║
║ Select the WebGPU implementation backend:                    ║
║                                                              ║
║ Current Backend: wgpu-rs (in blue)                         ║
║                                                              ║
║ Select Backend: [🌅 Dawn Native (C++ impl...)   ▼]         ║
║                                                              ║
║ Dawn (C++ implementation, used by Chromium)                  ║
║                                                              ║
║ ⚠️ Warning: Backend switching requires application restart  ║
║ (in orange)                                                  ║
║                                                              ║
║ To apply this change, set the environment variable:          ║
║ WEBGPU_IMPL=Dawn                                             ║
║ Then restart the application.                                ║
║                                                              ║
║ ✓ Dawn support is compiled in (in green)                    ║
║                                                              ║
║ 💡 Tip: Available backends:                                 ║
║   ✓ wgpu (active)                                           ║
║   ○ Dawn (inactive)                                         ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
```

## Color Legend

- 🦀 wgpu-rs: Blue (RGB 100, 150, 255)
- 🌅 Dawn Native: Orange (RGB 255, 180, 100)
- ✓ Success/Available: Green (RGB 100, 200, 100)
- ⚠️ Warning: Orange (RGB 255, 200, 100)
- ℹ️ Info: Yellow (RGB 200, 200, 100)

## Interaction Flow

1. **View Current Backend**: Check top menu bar for quick status
2. **Open Settings**: Navigate to Settings in the Tools section
3. **View Backend Info**: See current backend, availability, and options
4. **Select Backend**: Choose from dropdown (Dawn may be disabled if not compiled)
5. **See Warning**: If different from current, warning appears with instructions
6. **Apply Change**: Set environment variable and restart application
7. **Verify**: New backend shown in menu bar after restart
