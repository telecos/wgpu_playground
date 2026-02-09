/// Build script for wgpu_playground_core
///
/// This script handles building Dawn from source when the dawn feature is enabled.
/// Dawn is built using CMake and requires C++ build tools to be installed.
fn main() {
    // Print cargo configuration for debugging
    println!("cargo:rerun-if-changed=build.rs");

    // Register the dawn_enabled cfg
    println!("cargo::rustc-check-cfg=cfg(dawn_enabled)");

    #[cfg(feature = "dawn")]
    configure_and_build_dawn();
}

/// Find CMake executable, checking PATH first then common installation locations
#[cfg(feature = "dawn")]
fn find_cmake() -> Option<String> {
    use std::process::Command;

    // First, try cmake in PATH
    if let Ok(output) = Command::new("cmake").arg("--version").output() {
        if output.status.success() {
            return Some("cmake".to_string());
        }
    }

    // Common CMake installation paths on different platforms
    let candidate_paths: Vec<&str> = if cfg!(target_os = "windows") {
        vec![
            r"C:\Program Files\CMake\bin\cmake.exe",
            r"C:\Program Files (x86)\CMake\bin\cmake.exe",
            // Visual Studio 2022 bundled CMake
            r"C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe",
            r"C:\Program Files\Microsoft Visual Studio\2022\Professional\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe",
            r"C:\Program Files\Microsoft Visual Studio\2022\Enterprise\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe",
            // Visual Studio 2019 bundled CMake
            r"C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe",
            r"C:\Program Files (x86)\Microsoft Visual Studio\2019\Professional\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe",
            r"C:\Program Files (x86)\Microsoft Visual Studio\2019\Enterprise\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe",
            // Chocolatey installation
            r"C:\ProgramData\chocolatey\bin\cmake.exe",
            // Scoop installation
            r"C:\Users\Public\scoop\shims\cmake.exe",
        ]
    } else if cfg!(target_os = "macos") {
        vec![
            "/opt/homebrew/bin/cmake",
            "/usr/local/bin/cmake",
            "/Applications/CMake.app/Contents/bin/cmake",
        ]
    } else {
        // Linux
        vec!["/usr/bin/cmake", "/usr/local/bin/cmake", "/snap/bin/cmake"]
    };

    for path in candidate_paths {
        if std::path::Path::new(path).exists() {
            // Verify it works
            if let Ok(output) = Command::new(path).arg("--version").output() {
                if output.status.success() {
                    return Some(path.to_string());
                }
            }
        }
    }

    None
}

/// Find Ninja executable, checking PATH first then common installation locations
#[cfg(feature = "dawn")]
fn find_ninja() -> Option<String> {
    use std::process::Command;

    // First, try ninja in PATH
    if let Ok(output) = Command::new("ninja").arg("--version").output() {
        if output.status.success() {
            return Some("ninja".to_string());
        }
    }

    // Common Ninja installation paths
    let candidate_paths: Vec<&str> = if cfg!(target_os = "windows") {
        vec![
            r"C:\Program Files\Ninja\ninja.exe",
            r"C:\ProgramData\chocolatey\bin\ninja.exe",
            r"C:\Users\Public\scoop\shims\ninja.exe",
        ]
    } else if cfg!(target_os = "macos") {
        vec!["/opt/homebrew/bin/ninja", "/usr/local/bin/ninja"]
    } else {
        vec!["/usr/bin/ninja", "/usr/local/bin/ninja"]
    };

    for path in candidate_paths {
        if std::path::Path::new(path).exists() {
            if let Ok(output) = Command::new(path).arg("--version").output() {
                if output.status.success() {
                    return Some(path.to_string());
                }
            }
        }
    }

    None
}

/// Find Python 3 executable, checking PATH first then common installation locations
#[cfg(feature = "dawn")]
fn find_python() -> Option<String> {
    use std::process::Command;

    // First, try python/python3 in PATH
    for cmd in &["python3", "python", "py"] {
        if let Ok(output) = Command::new(cmd).arg("--version").output() {
            if output.status.success() {
                let version = String::from_utf8_lossy(&output.stdout);
                // Make sure it's Python 3
                if version.contains("Python 3")
                    || String::from_utf8_lossy(&output.stderr).contains("Python 3")
                {
                    return Some(cmd.to_string());
                }
            }
        }
    }

    // Common Python installation paths
    let candidate_paths: Vec<&str> = if cfg!(target_os = "windows") {
        vec![
            // Standard Python installations
            r"C:\Python312\python.exe",
            r"C:\Python311\python.exe",
            r"C:\Python310\python.exe",
            r"C:\Python39\python.exe",
            r"C:\Python38\python.exe",
            // Program Files installations
            r"C:\Program Files\Python312\python.exe",
            r"C:\Program Files\Python311\python.exe",
            r"C:\Program Files\Python310\python.exe",
            r"C:\Program Files\Python39\python.exe",
            // Windows Store Python (AppData)
            // Note: These paths would need user-specific expansion
        ]
    } else if cfg!(target_os = "macos") {
        vec![
            "/opt/homebrew/bin/python3",
            "/usr/local/bin/python3",
            "/usr/bin/python3",
        ]
    } else {
        vec!["/usr/bin/python3", "/usr/local/bin/python3"]
    };

    for path in candidate_paths {
        if std::path::Path::new(path).exists() {
            if let Ok(output) = Command::new(path).arg("--version").output() {
                if output.status.success() {
                    return Some(path.to_string());
                }
            }
        }
    }

    // On Windows, also check user-local Python installations
    #[cfg(target_os = "windows")]
    {
        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let user_paths = vec![
                format!(r"{}\Programs\Python\Python312\python.exe", local_app_data),
                format!(r"{}\Programs\Python\Python311\python.exe", local_app_data),
                format!(r"{}\Programs\Python\Python310\python.exe", local_app_data),
                format!(r"{}\Programs\Python\Python39\python.exe", local_app_data),
            ];
            for path in user_paths {
                if std::path::Path::new(&path).exists() {
                    if let Ok(output) = Command::new(&path).arg("--version").output() {
                        if output.status.success() {
                            return Some(path);
                        }
                    }
                }
            }
        }
    }

    None
}

/// Fetch Dawn dependencies using tools/fetch_dawn_dependencies.py
#[cfg(feature = "dawn")]
fn fetch_dawn_dependencies(dawn_dir: &std::path::Path, python_path: &str) {
    use std::process::Command;

    // The script is at tools/fetch_dawn_dependencies.py
    let fetch_script = dawn_dir.join("tools").join("fetch_dawn_dependencies.py");

    if fetch_script.exists() {
        println!("cargo:warning=Running fetch_dawn_dependencies.py...");
        let fetch_result = Command::new(python_path)
            .args([fetch_script.to_str().unwrap()])
            .current_dir(dawn_dir)
            .output();

        match fetch_result {
            Ok(output) if output.status.success() => {
                println!("cargo:warning=Dawn dependencies fetched successfully");
            }
            Ok(output) => {
                println!(
                    "cargo:warning=fetch_dawn_dependencies.py exited with code {}",
                    output.status
                );
                let stderr = String::from_utf8_lossy(&output.stderr);
                for line in stderr.lines().take(10) {
                    println!("cargo:warning=  {}", line);
                }
            }
            Err(e) => {
                println!(
                    "cargo:warning=Could not run fetch_dawn_dependencies.py: {}",
                    e
                );
            }
        }
    } else {
        println!(
            "cargo:warning=fetch_dawn_dependencies.py not found at {:?}",
            fetch_script
        );
        println!("cargo:warning=CMake's DAWN_FETCH_DEPENDENCIES will attempt to fetch deps");
    }
}

#[cfg(feature = "dawn")]
fn configure_and_build_dawn() {
    use std::env;
    use std::path::PathBuf;
    use std::process::{Command, Stdio};

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Try to use the Dawn source from the workspace first (if available)
    // This is much faster than cloning and already has all dependencies
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_dawn = manifest_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("dawn");

    let dawn_dir = if workspace_dawn
        .join("third_party")
        .join("CMakeLists.txt")
        .exists()
    {
        println!(
            "cargo:warning=Using Dawn source from workspace: {}",
            workspace_dawn.display()
        );
        workspace_dawn
    } else {
        println!("cargo:warning=Workspace Dawn not found or incomplete, will clone");
        out_dir.join("dawn")
    };

    let dawn_build_dir = out_dir.join("dawn-build");
    let dawn_install_dir = out_dir.join("dawn-install");

    println!("cargo:warning=Building Dawn from source - this may take 30+ minutes on first build");
    println!(
        "cargo:warning=Dawn source directory: {}",
        dawn_dir.display()
    );
    println!(
        "cargo:warning=Dawn build directory: {}",
        dawn_build_dir.display()
    );

    // Check if Dawn is already built and installed
    let lib_dir = dawn_install_dir.join("lib");
    let include_dir = dawn_install_dir.join("include");
    let dawn_header = include_dir.join("dawn").join("dawn_proc.h");

    if lib_dir.exists() && include_dir.exists() && dawn_header.exists() {
        println!("cargo:warning=Dawn already built and installed, using cache");
        setup_dawn_linking(&lib_dir, &include_dir);
        return;
    }

    // Find required tools
    let python_path = match find_python() {
        Some(path) => {
            println!("cargo:warning=Python found: {}", path);
            path
        }
        None => {
            println!("cargo:warning=Python 3 not found. Please install Python 3.");
            println!("cargo:warning=Using wgpu-core fallback");
            return;
        }
    };

    let cmake_path = match find_cmake() {
        Some(path) => {
            println!("cargo:warning=CMake found: {}", path);
            path
        }
        None => {
            println!("cargo:warning=CMake not found. Please install CMake.");
            println!("cargo:warning=Using wgpu-core fallback");
            return;
        }
    };

    // Clone Dawn if needed
    let dawn_cmakelists = dawn_dir.join("CMakeLists.txt");
    if !dawn_cmakelists.exists() {
        if dawn_dir.exists() {
            println!("cargo:warning=Dawn directory incomplete, removing...");
            let _ = std::fs::remove_dir_all(&dawn_dir);
        }

        println!("cargo:warning=Cloning Dawn repository (this takes a few minutes)...");

        // Configure git for long paths on Windows
        if cfg!(target_os = "windows") {
            let _ = Command::new("git")
                .args(["config", "--global", "core.longpaths", "true"])
                .status();
        }

        let clone_status = Command::new("git")
            .args([
                "clone",
                "--depth",
                "1", // Shallow clone for faster download
                "https://dawn.googlesource.com/dawn",
                dawn_dir.to_str().unwrap(),
            ])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        match clone_status {
            Ok(s) if s.success() => {
                println!("cargo:warning=Dawn cloned successfully");
            }
            _ => {
                println!("cargo:warning=Failed to clone Dawn. Using wgpu-core fallback");
                return;
            }
        }
    } else {
        println!("cargo:warning=Dawn source exists, skipping clone");
    }

    // Fetch Dawn dependencies
    println!("cargo:warning=Fetching Dawn dependencies...");
    fetch_dawn_dependencies(&dawn_dir, &python_path);

    // Check if we need to configure
    let cmake_cache = dawn_build_dir.join("CMakeCache.txt");
    if !cmake_cache.exists() {
        println!("cargo:warning=Configuring Dawn with CMake (this takes 10-30 minutes)...");

        // Clean any partial build
        let _ = std::fs::remove_dir_all(&dawn_build_dir);
        std::fs::create_dir_all(&dawn_build_dir).ok();

        let ninja_path = find_ninja();

        let mut cmake_args = vec![
            "-S".to_string(),
            dawn_dir.to_str().unwrap().to_string(),
            "-B".to_string(),
            dawn_build_dir.to_str().unwrap().to_string(),
            "-DCMAKE_BUILD_TYPE=Release".to_string(),
            format!("-DCMAKE_INSTALL_PREFIX={}", dawn_install_dir.display()),
            format!("-DPython3_EXECUTABLE={}", python_path),
            "-DDAWN_FETCH_DEPENDENCIES=ON".to_string(),
            "-DDAWN_ENABLE_INSTALL=ON".to_string(),
            "-DDAWN_BUILD_SAMPLES=OFF".to_string(),
            "-DTINT_BUILD_TESTS=OFF".to_string(),
            "-DTINT_BUILD_CMD_TOOLS=OFF".to_string(),
            "-DDAWN_BUILD_BENCHMARKS=OFF".to_string(),
        ];

        if ninja_path.is_some() {
            println!("cargo:warning=Using Ninja for faster builds");
            cmake_args.push("-G".to_string());
            cmake_args.push("Ninja".to_string());
        }

        let configure_status = Command::new(&cmake_path)
            .args(&cmake_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        match configure_status {
            Ok(s) if s.success() => {
                println!("cargo:warning=CMake configuration successful");
            }
            Ok(s) => {
                println!(
                    "cargo:warning=CMake configuration failed (exit code: {})",
                    s
                );
                println!("cargo:warning=Using wgpu-core fallback");
                return;
            }
            Err(e) => {
                println!("cargo:warning=CMake error: {}", e);
                println!("cargo:warning=Using wgpu-core fallback");
                return;
            }
        }
    } else {
        println!("cargo:warning=CMake already configured, skipping");
    }

    // Build Dawn
    println!("cargo:warning=Building Dawn (this takes 30-60 minutes on first build)...");
    let build_status = Command::new(&cmake_path)
        .args([
            "--build",
            dawn_build_dir.to_str().unwrap(),
            "--config",
            "Release",
            "--parallel",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match build_status {
        Ok(s) if s.success() => {
            println!("cargo:warning=Dawn built successfully");
        }
        Ok(s) => {
            println!("cargo:warning=Dawn build failed (exit code: {})", s);
            println!("cargo:warning=Using wgpu-core fallback");
            return;
        }
        Err(e) => {
            println!("cargo:warning=Build error: {}", e);
            println!("cargo:warning=Using wgpu-core fallback");
            return;
        }
    }

    // Install Dawn
    println!("cargo:warning=Installing Dawn...");
    let install_status = Command::new(&cmake_path)
        .args(["--install", dawn_build_dir.to_str().unwrap()])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match install_status {
        Ok(s) if s.success() => {
            println!("cargo:warning=Dawn installed successfully");
            setup_dawn_linking(&lib_dir, &include_dir);
        }
        _ => {
            println!("cargo:warning=Dawn install failed");
            println!("cargo:warning=Using wgpu-core fallback");
        }
    }
}

#[cfg(feature = "dawn")]
fn setup_dawn_linking(lib_dir: &std::path::Path, include_dir: &std::path::Path) {
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=dawn");
    println!("cargo:rustc-link-lib=static=dawn_native");
    println!("cargo:rustc-link-lib=static=dawn_platform");

    // Platform-specific linking
    configure_platform_linking();

    // Export paths for use in code
    println!("cargo:include={}", include_dir.display());

    // ONLY set dawn_enabled if we successfully built and installed Dawn
    println!("cargo:rustc-cfg=dawn_enabled");

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
