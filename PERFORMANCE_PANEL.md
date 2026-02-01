# Performance Metrics Panel Implementation

This document describes the performance metrics panel implementation added to the wgpu_playground application.

## Overview

The performance metrics panel provides real-time monitoring of GPU performance, including:
- **FPS (Frames Per Second)** - Current, 1% low, and 0.1% low
- **Frame Time** - Current, average, and peak measurements
- **GPU Memory Usage** - Estimated memory usage in MB
- **Command Buffer Statistics** - Count of command buffers, draw calls, and compute dispatches
- **Performance Graphs** - Visual representation of frame time and FPS over time

## Features Implemented

### 1. Performance Metrics Module (`performance_metrics.rs`)

Core tracking functionality for performance data:
- Real-time FPS calculation
- Frame time measurement in milliseconds
- Historical data storage (300 samples / ~5 seconds at 60 FPS)
- 1% and 0.1% low FPS calculation for consistency metrics
- GPU memory tracking
- Command buffer, draw call, and compute dispatch counters
- Pause/resume capability
- Peak value tracking

### 2. Performance Panel Module (`performance_panel.rs`)

UI component displaying performance data:
- Summary statistics grid showing all metrics
- Frame time graph with reference lines (60 FPS = 16.67ms, 30 FPS = 33.33ms)
- FPS graph with reference lines
- Control buttons:
  - Reset Peaks
  - Reset All
  - Pause/Resume
  - Auto-reset counters checkbox
- Graph visibility toggles
- Performance tips section

### 3. UI Integration

The performance panel has been integrated into the main application:
- Added "📊 Performance" tab in the navigation sidebar
- Positioned after "Resource Inspector" in the tab list
- Metrics automatically update each frame
- Panel accessible through the main UI navigation

## UI Layout

```
╔════════════════════════════════════════════════════════════════════╗
║                    📊 Performance Metrics                           ║
╠════════════════════════════════════════════════════════════════════╣
║ Real-time performance monitoring and profiling data.                ║
║                                                                      ║
║ [🔄 Reset Peaks] [🗑 Reset All] [⏸ Pause] ☑ Auto-reset counters   ║
║ -------------------------------------------------------------------- ║
║                                                                      ║
║                   Performance Statistics                             ║
║                                                                      ║
║   Current FPS:           60.0 fps                                   ║
║   1% Low FPS:            58.5 fps                                   ║
║   0.1% Low FPS:          57.2 fps                                   ║
║   ───────────────────────────────────                               ║
║   Current Frame Time:    16.67 ms                                   ║
║   Average Frame Time:    16.65 ms                                   ║
║   Peak Frame Time:       18.23 ms                                   ║
║   ───────────────────────────────────                               ║
║   GPU Memory Usage:      256.0 MB                                   ║
║   ───────────────────────────────────                               ║
║   Command Buffers:       12                                         ║
║   Draw Calls:            48                                         ║
║   Compute Dispatches:    0                                          ║
║                                                                      ║
║ -------------------------------------------------------------------- ║
║                                                                      ║
║                   Performance Graphs                                 ║
║                                                                      ║
║   ☑ Show Frame Time Graph   ☑ Show FPS Graph                       ║
║                                                                      ║
║   Frame Time (ms)                                                   ║
║   ┌────────────────────────────────────────────────────────────┐   ║
║   │                                                              │   ║
║   │  [Graph showing frame time with 60fps/30fps reference lines]│   ║
║   │                                                              │   ║
║   └────────────────────────────────────────────────────────────┘   ║
║                                                                      ║
║   FPS                                                               ║
║   ┌────────────────────────────────────────────────────────────┐   ║
║   │                                                              │   ║
║   │  [Graph showing FPS with 60fps/30fps reference lines]       │   ║
║   │                                                              │   ║
║   └────────────────────────────────────────────────────────────┘   ║
║                                                                      ║
║ -------------------------------------------------------------------- ║
║                                                                      ║
║                   Performance Tips                                   ║
║                                                                      ║
║   • Target: 60 FPS (16.67ms per frame) or 120 FPS (8.33ms)        ║
║   • Frame times above 16.67ms indicate performance issues          ║
║   • Monitor 1% and 0.1% low FPS for frame consistency              ║
║   • Reduce draw calls and command buffers for better performance   ║
║   • Use GPU profiling tools for detailed analysis                  ║
║                                                                      ║
╚════════════════════════════════════════════════════════════════════╝
```

## Technical Details

### Metrics Tracking

The `PerformanceMetrics` struct tracks:
- Frame start/end times using `Instant`
- Historical frame times in a `VecDeque` (rolling window)
- FPS calculated as `1000.0 / average_frame_time_ms`
- 1% low FPS from worst 1% of frames
- 0.1% low FPS from worst 0.1% of frames
- Command buffer statistics (counters)

### Graph Visualization

Uses `egui_plot` crate (version 0.29) for plotting:
- Line graphs for frame time and FPS
- Reference lines at 16.67ms (60 FPS) and 33.33ms (30 FPS)
- Color-coded reference lines (green for 60 FPS, yellow for 30 FPS)
- Configurable graph height (100px default)
- Fixed axes (no zoom/drag) for consistent visualization

### Auto-Reset Functionality

When enabled, automatically resets:
- Command buffer count
- Draw call count  
- Compute dispatch count

This is useful for per-frame profiling where you want to measure statistics for each individual frame.

## Testing

Comprehensive unit tests have been added:

**Performance Metrics Tests:**
- Basic creation and initialization
- Frame time measurement accuracy
- Counter increments
- Reset functionality
- Pause/resume behavior
- Peak tracking
- Historical data management
- 1% and 0.1% low FPS calculations

**Performance Panel Tests:**
- Panel creation
- Metrics access (mutable and immutable)
- Auto-reset behavior
- Update mechanism

All tests pass successfully.

## Usage

1. Launch the wgpu_playground application
2. Navigate to "📊 Performance" in the sidebar
3. The panel will automatically begin tracking performance metrics
4. Use control buttons to:
   - Reset peak values
   - Reset all metrics
   - Pause/resume tracking
   - Enable auto-reset for per-frame statistics
5. Toggle graphs on/off as needed
6. Monitor metrics in real-time

## Future Enhancements

Potential improvements:
- GPU memory usage from actual device queries (currently simulated)
- Query set integration for GPU timestamp profiling
- Export performance data to CSV/JSON
- Configurable graph time windows
- Alert thresholds for performance issues
- Per-pipeline performance breakdown
- Texture/buffer memory breakdown
