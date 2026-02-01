# Testing Guide for Main Rendering Canvas with Controls

This document provides instructions for testing the newly implemented main rendering canvas with controls.

## Features Implemented

1. **Main Canvas Area**: Displays actual WebGPU render texture output (not a mock/fake preview)
2. **Canvas Controls Panel** (collapsible):
   - Canvas size adjustment (width/height with drag controls)
   - Clear color picker
   - Screenshot capture button
   - Camera controls for 3D examples (distance, rotation X/Y)
3. **Mouse Interaction**:
   - Drag to rotate camera in 3D examples (cube)
   - Scroll wheel to zoom in/out
   
## How to Test

### 1. Build and Run the Application

```bash
cargo run --release --package wgpu_playground_gui
```

### 2. Navigate to the Rendering Tab

- Click on "🎨 Rendering" in the left sidebar
- Select "📚 Example Gallery" tab at the top

### 3. Test Triangle Example

1. Select "Triangle" from the example list
2. Click "▶ Run Example" button
3. Verify:
   - A colored triangle appears in the canvas (red, green, blue vertices)
   - The triangle is rendered using actual WebGPU, not a mock drawing
   - Label "✓ Rendering with WebGPU" appears below the canvas

#### Canvas Size Control
1. Click "⚙️ Canvas Controls" to expand the controls
2. Change width to 800 and height to 600
3. Click "Apply"
4. Verify:
   - Canvas resizes to 800x600
   - Triangle still renders correctly

#### Clear Color Control
1. In Canvas Controls, click the clear color picker
2. Change to a different color (e.g., dark blue)
3. Verify:
   - Background color changes to the selected color

#### Screenshot Capture
1. Click "📷 Capture Screenshot" button
2. Check the application's working directory for a PNG file named `screenshot_<timestamp>.png`
3. Verify:
   - Screenshot file exists
   - Screenshot shows the current render output
   - Console shows "Screenshot saved to screenshot_<timestamp>.png"

### 4. Test Cube Example

1. Select "Cube" from the example list
2. Click "▶ Run Example" button
3. Verify:
   - A colored rotating cube appears
   - The cube is rotating automatically
   - Label "✓ Rendering with WebGPU" appears below the canvas
   - Label "💡 Drag to rotate, scroll to zoom" appears below

#### Mouse Camera Control
1. **Rotation via Drag**:
   - Click and drag on the canvas
   - Verify: Camera rotates around the cube
   - Horizontal drag rotates around Y axis
   - Vertical drag rotates around X axis
   
2. **Zoom via Scroll**:
   - Scroll mouse wheel up/down over the canvas
   - Verify: Camera moves closer/farther from the cube
   - Range is limited (1.0 to 10.0 distance)

#### Camera Control Sliders
1. In Canvas Controls, observe the additional camera sliders:
   - Distance slider (1.0 to 10.0)
   - Rotation X slider (-π to π)
   - Rotation Y slider (-π to π)
2. Adjust each slider
3. Verify: Camera position updates immediately
4. Click "Reset Camera" button
5. Verify: Camera returns to default position (distance 3.0, rotations 0.0)

### 5. Test Stopping Examples

1. While an example is running, click "⏹ Stop Example"
2. Verify:
   - Canvas preview disappears
   - Button changes back to "▶ Run Example"

### 6. Test Different Canvas Sizes

Try various canvas sizes:
- Small: 256x256
- Default: 512x512
- Large: 1024x768
- Wide: 1280x720

Verify:
- Canvas resizes correctly
- Rendering scales appropriately
- No distortion or aspect ratio issues

## Expected Behavior

### Canvas Display
- ✅ Shows actual WebGPU render output (not egui shapes/mock)
- ✅ Updates in real-time
- ✅ Maintains aspect ratio based on canvas size

### Controls
- ✅ Canvas size can be adjusted via drag values or text input
- ✅ Apply button triggers resize and re-creates render state
- ✅ Clear color picker affects background
- ✅ Screenshot saves to working directory
- ✅ Camera controls work smoothly

### Mouse Interaction (3D Examples)
- ✅ Drag rotates camera smoothly
- ✅ Scroll zooms in/out with limits
- ✅ Multiple input methods (drag, sliders) work together
- ✅ Camera position updates immediately

## Known Limitations

1. **Screenshot Format**: Screenshots are saved as PNG in RGBA format
2. **Canvas Size Limits**: Canvas size is clamped to 64-2048 pixels (via UI controls)
3. **Camera Rotation Clamping**: X rotation is clamped to ±π/2 to avoid gimbal lock
4. **Frame Rate**: Animation assumes constant 60 FPS (hardcoded delta time)

## Troubleshooting

### Issue: Canvas appears blank
- Check that WebGPU device is available
- Check console for GPU errors
- Try selecting a different GPU adapter in the Adapter Selection tab

### Issue: Screenshot not saving
- Check write permissions in the current directory
- Check console for error messages
- Verify the `image` crate is properly included

### Issue: Mouse interaction not working
- Ensure you're clicking and dragging on the canvas itself
- Try the slider controls as an alternative
- Check if the example is actually running

### Issue: Example won't start
- Check Device Info tab for GPU capabilities
- Try a simpler example (Triangle before Cube)
- Check console for compilation errors

## Code Changes Summary

### Modified Files
1. `crates/wgpu_playground_core/src/rendering.rs`:
   - Added canvas control fields
   - Added camera control fields  
   - Implemented `register_texture()` method
   - Implemented `resize_canvas()` method
   - Implemented `capture_screenshot()` method
   - Updated `render_current_example()` to use clear color and camera
   - Updated UI to display WebGPU texture and controls
   - Added mouse interaction handling

2. `crates/wgpu_playground_core/Cargo.toml`:
   - Added `egui-wgpu = "0.29"` dependency
   - Added `image = "0.25"` dependency

3. `crates/wgpu_playground_gui/src/app.rs`:
   - Updated `ui()` signature to accept `egui_wgpu::Renderer`
   - Passed renderer to rendering panel

4. `crates/wgpu_playground_gui/src/main.rs`:
   - Updated to pass renderer to app's `ui()` method

## Performance Notes

- Canvas rendering happens every frame
- Screenshot capture is synchronous and may block briefly
- Large canvas sizes (>1024) may impact performance on lower-end GPUs
- Mouse interaction updates are immediate (no debouncing)

## Next Steps for Enhancement

Potential future improvements:
1. Add support for saving screenshots in different formats (JPEG, BMP)
2. Add keyboard shortcuts for common operations
3. Add animation speed control
4. Add wireframe/shading mode toggles
5. Add FPS counter
6. Support touch gestures for camera control on touch screens
