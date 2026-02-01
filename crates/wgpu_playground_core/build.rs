/// Build script for wgpu_playground_core
///
/// This script handles building Dawn from source when the dawn feature is enabled.
/// Dawn is built using CMake and requires C++ build tools to be installed.
fn main() {
    // Print cargo configuration for debugging
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(feature = "dawn")]
    configure_and_build_dawn();
}

#[cfg(feature = "dawn")]
fn configure_and_build_dawn() {
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;
    println!("cargo:rustc-cfg=dawn_enabled");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let dawn_dir = out_dir.join("dawn");
    let dawn_build_dir = out_dir.join("dawn-build");
    let dawn_install_dir = out_dir.join("dawn-install");

    println!("cargo:warning=Building Dawn from source - this will take several minutes");
    println!("cargo:warning=Dawn build directory: {}", dawn_dir.display());

    // Check if Dawn source exists, if not clone it
    if !dawn_dir.exists() {
        println!("cargo:warning=Cloning Dawn repository...");

        // Configure git to support long paths on Windows
        // This is necessary because Dawn repository contains files with very long paths
        // that exceed the default Windows path length limit of 260 characters
        if cfg!(target_os = "windows") {
            println!("cargo:warning=Configuring Git to support long paths on Windows...");
            match Command::new("git")
                .args(["config", "--global", "core.longpaths", "true"])
                .status()
            {
                Ok(s) if s.success() => {
                    println!("cargo:warning=Git long paths support enabled");
                }
                Ok(s) => {
                    println!(
                        "cargo:warning=Failed to configure Git long paths (exit code: {}). Clone may fail on Windows.",
                        s
                    );
                }
                Err(e) => {
                    println!(
                        "cargo:warning=Could not configure Git long paths: {}. Clone may fail on Windows.",
                        e
                    );
                }
            }
        }

        let status = Command::new("git")
            .args([
                "clone",
                "https://dawn.googlesource.com/dawn",
                dawn_dir.to_str().unwrap(),
            ])
            .status();

        match status {
            Ok(s) if s.success() => {
                println!("cargo:warning=Dawn repository cloned successfully");
            }
            Ok(s) => {
                println!(
                    "cargo:warning=Failed to clone Dawn repository (exit code: {})",
                    s
                );
                println!("cargo:warning=Please clone manually: git clone https://dawn.googlesource.com/dawn");
                println!("cargo:warning=Continuing with placeholder Dawn support");
                return;
            }
            Err(e) => {
                println!("cargo:warning=Git command failed: {}. Is git installed?", e);
                println!("cargo:warning=Continuing with placeholder Dawn support");
                return;
            }
        }
    } else {
        println!("cargo:warning=Dawn source already exists, skipping clone");
    }

    // Check if CMake is available
    let cmake_check = Command::new("cmake").arg("--version").output();
    match cmake_check {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!(
                "cargo:warning=CMake found: {}",
                version.lines().next().unwrap_or("unknown version")
            );
        }
        _ => {
            println!("cargo:warning=CMake not found. Please install CMake to build Dawn.");
            println!("cargo:warning=Continuing with placeholder Dawn support");
            return;
        }
    }

    // Configure Dawn with CMake
    println!("cargo:warning=Configuring Dawn build with CMake...");

    let configure_status = Command::new("cmake")
        .args([
            "-S",
            dawn_dir.to_str().unwrap(),
            "-B",
            dawn_build_dir.to_str().unwrap(),
            "-DDAWN_FETCH_DEPENDENCIES=ON",
            "-DDAWN_ENABLE_INSTALL=ON",
            "-DCMAKE_BUILD_TYPE=Release",
            &format!("-DCMAKE_INSTALL_PREFIX={}", dawn_install_dir.display()),
        ])
        .status();

    match configure_status {
        Ok(s) if s.success() => {
            println!("cargo:warning=Dawn CMake configuration successful");
        }
        Ok(s) => {
            println!(
                "cargo:warning=CMake configuration failed (exit code: {})",
                s
            );
            println!("cargo:warning=Continuing with placeholder Dawn support");
            return;
        }
        Err(e) => {
            println!("cargo:warning=CMake configuration error: {}", e);
            println!("cargo:warning=Continuing with placeholder Dawn support");
            return;
        }
    }

    // Build Dawn
    println!("cargo:warning=Building Dawn (this may take 10-30 minutes)...");

    let build_status = Command::new("cmake")
        .args([
            "--build",
            dawn_build_dir.to_str().unwrap(),
            "--config",
            "Release",
            "--parallel",
        ])
        .status();

    match build_status {
        Ok(s) if s.success() => {
            println!("cargo:warning=Dawn built successfully");
        }
        Ok(s) => {
            println!("cargo:warning=Dawn build failed (exit code: {})", s);
            println!("cargo:warning=Continuing with placeholder Dawn support");
            return;
        }
        Err(e) => {
            println!("cargo:warning=Dawn build error: {}", e);
            println!("cargo:warning=Continuing with placeholder Dawn support");
            return;
        }
    }

    // Install Dawn
    println!("cargo:warning=Installing Dawn libraries...");

    let install_status = Command::new("cmake")
        .args(["--install", dawn_build_dir.to_str().unwrap()])
        .status();

    match install_status {
        Ok(s) if s.success() => {
            println!("cargo:warning=Dawn installed successfully");
        }
        Ok(s) => {
            println!("cargo:warning=Dawn install failed (exit code: {})", s);
            println!("cargo:warning=Continuing with placeholder Dawn support");
            return;
        }
        Err(e) => {
            println!("cargo:warning=Dawn install error: {}", e);
            println!("cargo:warning=Continuing with placeholder Dawn support");
            return;
        }
    }

    // Set up library paths
    let lib_dir = dawn_install_dir.join("lib");
    let include_dir = dawn_install_dir.join("include");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=dawn");
    println!("cargo:rustc-link-lib=static=dawn_native");
    println!("cargo:rustc-link-lib=static=dawn_platform");

    // Platform-specific linking
    configure_platform_linking();

    // Export paths for use in code
    println!("cargo:include={}", include_dir.display());

    println!("cargo:warning=Dawn integration complete!");
    println!("cargo:warning=Include directory: {}", include_dir.display());
    println!("cargo:warning=Library directory: {}", lib_dir.display());
}

#[cfg(feature = "dawn")]
fn configure_platform_linking() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:warning=Configuring Dawn for Windows (D3D12 backend)");
        println!("cargo:rustc-link-lib=dylib=d3d12");
        println!("cargo:rustc-link-lib=dylib=dxgi");
        println!("cargo:rustc-link-lib=dylib=d3dcompiler");
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:warning=Configuring Dawn for Linux (Vulkan backend)");
        // Vulkan linking is typically handled by Dawn itself
    }

    #[cfg(target_os = "macos")]
    {
        println!("cargo:warning=Configuring Dawn for macOS (Metal backend)");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=IOSurface");
        println!("cargo:rustc-link-lib=framework=QuartzCore");
    }
}
