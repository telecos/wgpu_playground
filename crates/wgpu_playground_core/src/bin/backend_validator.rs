//! Backend Output Validator - Simple CLI for comparing rendering outputs

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_help();
        return;
    }

    match args[1].as_str() {
        "diff" => cmd_diff(&args[2..]),
        "html" => cmd_html(&args[2..]),
        _ => show_help(),
    }
}

fn show_help() {
    println!("Backend Output Validator v1.0");
    println!();
    println!("USAGE:");
    println!("  backend_validator diff <image_a> <image_b>");
    println!("  backend_validator html <image_directory>");
}

fn cmd_diff(params: &[String]) {
    if params.len() < 2 {
        eprintln!("Need two image file paths");
        return;
    }

    let path_a = &params[0];
    let path_b = &params[1];

    let rgba_a = match image::open(path_a) {
        Ok(loaded) => loaded.to_rgba8(),
        Err(why) => {
            eprintln!("Cannot open {}: {}", path_a, why);
            return;
        }
    };

    let rgba_b = match image::open(path_b) {
        Ok(loaded) => loaded.to_rgba8(),
        Err(why) => {
            eprintln!("Cannot open {}: {}", path_b, why);
            return;
        }
    };

    let dims_a = rgba_a.dimensions();
    let dims_b = rgba_b.dimensions();

    if dims_a != dims_b {
        eprintln!("Size mismatch: {:?} != {:?}", dims_a, dims_b);
        return;
    }

    let (width, height) = dims_a;
    let mut mismatch_pixels = 0u64;
    let mut accumulated_error = 0.0f64;

    for row in 0..height {
        for col in 0..width {
            let px_a = rgba_a.get_pixel(col, row);
            let px_b = rgba_b.get_pixel(col, row);

            if px_a[0] != px_b[0] || px_a[1] != px_b[1] || px_a[2] != px_b[2] || px_a[3] != px_b[3]
            {
                mismatch_pixels += 1;

                let red_err = (px_a[0] as i32 - px_b[0] as i32).abs() as f64 / 255.0;
                let grn_err = (px_a[1] as i32 - px_b[1] as i32).abs() as f64 / 255.0;
                let blu_err = (px_a[2] as i32 - px_b[2] as i32).abs() as f64 / 255.0;

                accumulated_error += (red_err + grn_err + blu_err) / 3.0;
            }
        }
    }

    let pixel_total = (width * height) as u64;
    let mismatch_rate = (mismatch_pixels as f64 / pixel_total as f64) * 100.0;
    let mean_error = if mismatch_pixels > 0 {
        accumulated_error / mismatch_pixels as f64
    } else {
        0.0
    };

    println!("Image A: {}", path_a);
    println!("Image B: {}", path_b);
    println!("Dimensions: {}x{}", width, height);
    println!(
        "Mismatched: {} / {} pixels ({:.3}%)",
        mismatch_pixels, pixel_total, mismatch_rate
    );
    println!("Mean Error: {:.5}", mean_error);

    if mismatch_pixels == 0 {
        println!("RESULT: Identical ✓");
    } else {
        println!("RESULT: Different ✗");
    }
}

fn cmd_html(params: &[String]) {
    if params.is_empty() {
        eprintln!("Need directory path");
        return;
    }

    let scan_dir = PathBuf::from(&params[0]);

    if !scan_dir.is_dir() {
        eprintln!("Not a directory: {}", scan_dir.display());
        return;
    }

    let mut wgpu_pngs = Vec::new();
    let mut dawn_pngs = Vec::new();

    if let Ok(dir_iter) = fs::read_dir(&scan_dir) {
        for item in dir_iter.flatten() {
            let filepath = item.path();
            if let Some(filename) = filepath.file_name().and_then(|s| s.to_str()) {
                if filename.ends_with("_wgpu.png") {
                    wgpu_pngs.push(filepath);
                } else if filename.ends_with("_dawn.png") {
                    dawn_pngs.push(filepath);
                }
            }
        }
    }

    let html_doc = build_html_document(&wgpu_pngs, &dawn_pngs);
    let output_file = scan_dir.join("report.html");

    match fs::write(&output_file, html_doc) {
        Ok(_) => println!("Report written: {}", output_file.display()),
        Err(why) => eprintln!("Write failed: {}", why),
    }
}

fn build_html_document(wgpu_list: &[PathBuf], dawn_list: &[PathBuf]) -> String {
    let mut table_body = String::new();

    for wgpu_file in wgpu_list {
        if let Some(fname) = wgpu_file.file_name().and_then(|s| s.to_str()) {
            let base_name = fname.trim_end_matches("_wgpu.png");
            let expected_dawn = format!("{}_dawn.png", base_name);

            let has_dawn = dawn_list
                .iter()
                .any(|p| p.file_name().and_then(|s| s.to_str()) == Some(&expected_dawn));

            let dawn_cell = if has_dawn {
                format!("<img src='{}' style='max-width:250px'/>", expected_dawn)
            } else {
                "—".to_string()
            };

            table_body.push_str(&format!(
                "<tr><td>{}</td><td><img src='{}' style='max-width:250px'/></td><td>{}</td></tr>\n",
                base_name, fname, dawn_cell
            ));
        }
    }

    format!(
        "<!DOCTYPE html>\n\
        <html><head><meta charset='UTF-8'><title>Rendering Comparison</title>\n\
        <style>\n\
        * {{ box-sizing: border-box; }}\n\
        body {{ font-family: Arial, sans-serif; margin: 2em; background: #f8f8f8; }}\n\
        h1 {{ color: #222; border-bottom: 3px solid #4a90e2; padding-bottom: 0.5em; }}\n\
        table {{ width: 100%; border-collapse: collapse; background: white; margin-top: 1em; }}\n\
        th, td {{ padding: 1em; border: 1px solid #ddd; }}\n\
        th {{ background: #4a90e2; color: white; text-align: left; }}\n\
        img {{ display: block; border: 1px solid #999; }}\n\
        </style>\n\
        </head><body>\n\
        <h1>Backend Rendering Comparison</h1>\n\
        <table>\n\
        <thead><tr><th>Test Name</th><th>wgpu-rs Output</th><th>Dawn Output</th></tr></thead>\n\
        <tbody>{}</tbody>\n\
        </table>\n\
        </body></html>",
        table_body
    )
}
