# Building Dawn from Source

This guide explains how to build the wgpu_playground with Dawn support, which requires building Google's Dawn WebGPU implementation from source.

## Overview

Dawn is Google's C++ implementation of the WebGPU standard, used in Chromium browsers. When you enable the `dawn` feature, the build system automatically:

1. Clones the Dawn repository
2. Configures the build using CMake
3. Compiles Dawn with all dependencies
4. Links Dawn libraries to the Rust project

## Prerequisites

### All Platforms

You need these tools installed:

- **Git** - To clone the Dawn repository
- **CMake 3.16+** - To configure and build Dawn
- **Python 3** - For Dawn's dependency management scripts
- **C++ Compiler with C++20 support**

### Platform-Specific Requirements

#### Linux (Ubuntu/Debian)

```bash
# Install all required tools
sudo apt-get update
sudo apt-get install -y git cmake build-essential python3 python3-pip libvulkan-dev

# Verify installations
git --version
cmake --version
python3 --version
g++ --version
```

**Minimum versions:**
- Git: Any recent version
- CMake: 3.16 or later
- GCC: 9.0 or later (for C++20 support)
- Python: 3.6 or later

#### macOS

```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install required tools
brew install git cmake python3

# Xcode Command Line Tools (for C++ compiler)
xcode-select --install

# Verify installations
git --version
cmake --version
python3 --version
clang++ --version
```

**Minimum versions:**
- Git: Any recent version
- CMake: 3.16 or later
- Xcode CLT: 12.0 or later (for C++20 support)
- Python: 3.6 or later

#### Windows

1. **Visual Studio 2019 or later**:
   - Download from: https://visualstudio.microsoft.com/downloads/
   - Install "Desktop development with C++" workload
   - Ensure Windows SDK is included

2. **CMake**:
   - Download from: https://cmake.org/download/
   - Install and add to PATH
   - Or use: `winget install Kitware.CMake`

3. **Git**:
   - Download from: https://git-scm.com/download/win
   - Or use: `winget install Git.Git`

4. **Python 3**:
   - Download from: https://python.org/downloads/
   - Ensure "Add Python to PATH" is checked during installation
   - Or use: `winget install Python.Python.3`

**Verify installations in PowerShell or Command Prompt:**
```powershell
git --version
cmake --version
python --version
cl  # Should show Microsoft C/C++ compiler version
```

## Building with Dawn

### First Build

The first build with Dawn will take **10-30 minutes** depending on your system, as it needs to:
- Clone Dawn repository (~500 MB)
- Download Dawn dependencies
- Build Dawn and all its components

```bash
# Clone the wgpu_playground repository (if not already)
git clone https://github.com/telecos/wgpu_playground
cd wgpu_playground

# Build with Dawn feature
cargo build --release --features dawn
```

**What happens during the build:**

1. Rust builds the wgpu_playground_core crate
2. build.rs script detects the `dawn` feature
3. Script clones Dawn from https://dawn.googlesource.com/dawn
4. CMake configures Dawn build with `-DDAWN_FETCH_DEPENDENCIES=ON` flag
5. CMake builds Dawn in Release mode
6. Dawn libraries are installed to the build output directory
7. Rust links against the built Dawn libraries

### Subsequent Builds

After the first build, Dawn is cached in the build directory:

```bash
# Much faster - Dawn is already built
cargo build --release --features dawn
```

Subsequent builds only rebuild Rust code unless you clean the build directory.

### Clean Build

To rebuild Dawn from scratch:

```bash
# Remove build artifacts
cargo clean

# Rebuild everything
cargo build --release --features dawn
```

## Build Output

After a successful Dawn build, you'll see output like:

```
warning: wgpu_playground_core@0.1.0: Cloning Dawn repository...
warning: wgpu_playground_core@0.1.0: Dawn repository cloned successfully
warning: wgpu_playground_core@0.1.0: CMake found: cmake version 3.28.0
warning: wgpu_playground_core@0.1.0: Configuring Dawn build with CMake...
warning: wgpu_playground_core@0.1.0: Dawn CMake configuration successful
warning: wgpu_playground_core@0.1.0: Building Dawn (this may take 10-30 minutes)...
warning: wgpu_playground_core@0.1.0: Dawn built successfully
warning: wgpu_playground_core@0.1.0: Installing Dawn libraries...
warning: wgpu_playground_core@0.1.0: Dawn installed successfully
warning: wgpu_playground_core@0.1.0: Configuring Dawn for Linux (Vulkan backend)
warning: wgpu_playground_core@0.1.0: Dawn integration complete!
```

## Running with Dawn

After building with Dawn:

```bash
# Run with Dawn implementation
WEBGPU_IMPL=dawn cargo run --release --features dawn

# Or let it use compile-time default (Dawn when feature enabled)
cargo run --release --features dawn
```

## Troubleshooting

### "Git not found"

**Problem**: Build fails with "Git command failed: ... Is git installed?"

**Solution**:
- Install Git from https://git-scm.com/downloads
- Ensure `git` is in your system PATH
- Restart terminal/IDE after installation

### "CMake not found"

**Problem**: Build fails with "CMake not found. Please install CMake to build Dawn."

**Solution**:
- Install CMake 3.16 or later from https://cmake.org/download/
- Ensure `cmake` is in your system PATH
- On Linux: `sudo apt-get install cmake`
- On macOS: `brew install cmake`
- On Windows: Add CMake bin directory to PATH

### "Build failed" on Linux

**Problem**: CMake configuration or compilation fails

**Solutions**:
1. Install build essentials:
   ```bash
   sudo apt-get install build-essential
   ```

2. Install Vulkan development files:
   ```bash
   sudo apt-get install libvulkan-dev
   ```

3. Ensure GCC 9+ for C++20 support:
   ```bash
   g++ --version  # Should be 9.0 or later
   ```

### "Build failed" on Windows

**Problem**: Compilation fails or can't find MSVC

**Solutions**:
1. Install Visual Studio 2019 or later with "Desktop development with C++" workload
2. Ensure Windows SDK is installed
3. Use "x64 Native Tools Command Prompt for VS" instead of regular Command Prompt
4. Check that cl.exe (MSVC compiler) is in PATH

### "Build failed" on macOS

**Problem**: Compilation fails or can't find clang

**Solutions**:
1. Install Xcode Command Line Tools:
   ```bash
   xcode-select --install
   ```

2. Ensure Xcode CLT version is 12.0+ for C++20 support:
   ```bash
   clang++ --version
   ```

3. Accept Xcode license if prompted:
   ```bash
   sudo xcodebuild -license accept
   ```

### "Python not found"

**Problem**: Build fails with Python-related errors

**Solution**:
- Install Python 3 from https://python.org/downloads/
- Ensure `python3` (or `python` on Windows) is in PATH
- On Linux: `sudo apt-get install python3`
- On macOS: `brew install python3`
- On Windows: Reinstall Python with "Add to PATH" option

### Build Takes Too Long

**Problem**: Dawn build takes more than 30 minutes

**Solutions**:
1. Use parallel builds (automatically enabled with `--parallel` flag)
2. Increase system resources (RAM, CPU cores)
3. Use Release build (not Debug): `cargo build --release`
4. Check disk I/O performance - SSD recommended

**Expected times**:
- Modern desktop (8 cores, SSD): 10-15 minutes
- Laptop (4 cores, HDD): 20-30 minutes
- CI systems: 15-25 minutes

### Out of Disk Space

**Problem**: Build fails due to insufficient disk space

**Solution**:
- Dawn source + build artifacts require ~5-10 GB
- Ensure at least 15 GB free space
- Use `cargo clean` to remove old builds
- Build output is in `target/` directory

### Network Issues

**Problem**: Failed to clone Dawn repository

**Solutions**:
1. Check internet connection
2. Verify Git can access https://dawn.googlesource.com
3. Try manual clone:
   ```bash
   git clone https://dawn.googlesource.com/dawn
   ```
4. Use a proxy if behind corporate firewall:
   ```bash
   git config --global http.proxy http://proxy:port
   ```

## Build Directory Structure

After a successful Dawn build:

```
target/
└── release/
    └── build/
        └── wgpu_playground_core-*/
            └── out/
                ├── dawn/          # Dawn source code (cloned)
                ├── dawn-build/    # Dawn build artifacts
                └── dawn-install/  # Dawn libraries and headers
                    ├── lib/       # Dawn libraries
                    └── include/   # Dawn headers
```

## Performance Tips

1. **Use Release builds**: `cargo build --release --features dawn`
2. **Enable parallel compilation**: Automatically done by CMake
3. **Cache builds**: Don't run `cargo clean` unless necessary
4. **SSD recommended**: Significantly faster than HDD
5. **Sufficient RAM**: 8 GB minimum, 16 GB recommended

## Advanced Build Options

### Custom Dawn Source

To use a local Dawn source directory instead of cloning:

1. Clone Dawn manually:
   ```bash
   git clone https://dawn.googlesource.com/dawn ~/dawn
   ```

2. Modify build.rs to use your local copy (not currently supported, requires code changes)

### Debug Builds

For Dawn development/debugging:

```bash
# Build in debug mode (slower, includes symbols)
cargo build --features dawn
```

This builds Dawn in Debug mode with full debug symbols.

## CI/CD Integration

To build with Dawn in CI:

```yaml
# GitHub Actions example
- name: Install Dawn dependencies (Ubuntu)
  run: |
    sudo apt-get update
    sudo apt-get install -y git cmake build-essential python3 libvulkan-dev

- name: Build with Dawn
  run: cargo build --release --features dawn
```

## Getting Help

If you encounter issues not covered here:

1. Check build output for specific error messages
2. Search existing issues: https://github.com/telecos/wgpu_playground/issues
3. Open a new issue with:
   - Your platform (OS, version)
   - Tool versions (git, cmake, compiler, python)
   - Full build log
   - Steps to reproduce

## References

- Dawn repository: https://dawn.googlesource.com/dawn
- Dawn CMake quickstart: https://dawn.googlesource.com/dawn/+/HEAD/docs/quickstart-cmake.md
- WebGPU specification: https://www.w3.org/TR/webgpu/
- wgpu_playground: https://github.com/telecos/wgpu_playground
