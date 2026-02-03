# WebGPU Demo

This directory contains the HTML and assets for the wgpu_playground WebGPU demo.

## Files

- `index.html` - Main demo page that loads and runs the WASM module

## Local Development

To test the demo locally:

1. Build the WASM module:
   ```bash
   cd crates/wgpu_playground_core
   wasm-pack build --target web --release
   ```

2. Copy the built package to the web directory:
   ```bash
   cp -r pkg ../../web/
   ```

3. Serve the web directory with a local HTTP server:
   ```bash
   cd ../../web
   python3 -m http.server 8000
   # Or use any other HTTP server
   ```

4. Open your browser to http://localhost:8000

## Deployment

The demo is automatically deployed to GitHub Pages when changes are pushed to the main branch. The deployment workflow:

1. Builds the WASM module using wasm-pack
2. Combines it with the HTML page
3. Deploys to GitHub Pages at https://telecos.github.io/wgpu_playground/demo/

## Requirements

The demo requires a browser with WebGPU support:
- Chrome 113+ or Edge 113+
- Safari with WebGPU enabled (Technology Preview)
- Firefox Nightly with WebGPU enabled

## Browser Compatibility

| Browser | Version | Support |
|---------|---------|---------|
| Chrome/Edge | 113+ | ✅ Full support |
| Safari | Tech Preview | ✅ With WebGPU enabled |
| Firefox | Nightly | 🟡 Experimental |

## Troubleshooting

### Demo doesn't load
- Check browser console for errors
- Verify WebGPU is supported in your browser
- Try using Chrome 113+ or Edge 113+ for best compatibility

### "WebGPU is not supported" error
- Update your browser to the latest version
- For Safari, enable WebGPU in Experimental Features
- For Firefox, use Nightly and enable WebGPU in about:config
