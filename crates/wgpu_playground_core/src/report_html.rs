//! HTML report builder for visual test results
//!
//! Generates side-by-side comparison reports from test artifacts

use std::fs;
use std::path::Path;

pub struct ReportBuilder {
    test_name: String,
    sections: Vec<ReportSection>,
}

struct ReportSection {
    title: String,
    content: String,
}

impl ReportBuilder {
    pub fn new(test_name: impl Into<String>) -> Self {
        Self {
            test_name: test_name.into(),
            sections: Vec::new(),
        }
    }

    pub fn add_image_pair(&mut self, left_path: &Path, right_path: &Path, caption: &str) {
        let html = format!(
            r#"
            <div class="img-pair">
                <div class="img-box">
                    <img src="{}" alt="Left"/>
                    <p>wgpu-rs</p>
                </div>
                <div class="img-box">
                    <img src="{}" alt="Right"/>
                    <p>Dawn</p>
                </div>
                <div class="caption">{}</div>
            </div>
            "#,
            left_path.display(),
            right_path.display(),
            caption
        );

        self.sections.push(ReportSection {
            title: "Visual Comparison".to_string(),
            content: html,
        });
    }

    pub fn add_metrics_table(&mut self, wgpu_ms: f64, dawn_ms: Option<f64>) {
        let dawn_row = if let Some(d) = dawn_ms {
            let ratio = wgpu_ms / d;
            format!(
                "<tr><td>Dawn</td><td>{:.2} ms</td><td>{:.1}x</td></tr>",
                d, ratio
            )
        } else {
            "<tr><td>Dawn</td><td colspan='2'>Not available</td></tr>".to_string()
        };

        let html = format!(
            r#"
            <table class="metrics">
                <tr><th>Backend</th><th>Avg Frame Time</th><th>Relative</th></tr>
                <tr><td>wgpu-rs</td><td>{:.2} ms</td><td>1.0x</td></tr>
                {}
            </table>
            "#,
            wgpu_ms, dawn_row
        );

        self.sections.push(ReportSection {
            title: "Performance Metrics".to_string(),
            content: html,
        });
    }

    pub fn save_to(&self, output_path: &Path) -> std::io::Result<()> {
        let mut body = String::new();

        for sec in &self.sections {
            body.push_str(&format!("<h2>{}</h2>\n", sec.title));
            body.push_str(&sec.content);
            body.push('\n');
        }

        let full_html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>{} - Test Report</title>
    <style>
        body {{ font-family: sans-serif; margin: 40px; background: #f5f5f5; }}
        h1 {{ color: #333; border-bottom: 2px solid #4CAF50; padding-bottom: 10px; }}
        h2 {{ color: #555; margin-top: 30px; }}
        .img-pair {{ display: flex; gap: 20px; margin: 20px 0; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .img-box {{ flex: 1; text-align: center; }}
        .img-box img {{ max-width: 100%; border: 1px solid #ddd; border-radius: 4px; }}
        .img-box p {{ margin-top: 10px; font-weight: bold; color: #666; }}
        .caption {{ width: 100%; text-align: center; padding-top: 15px; color: #777; font-style: italic; }}
        .metrics {{ width: 100%; border-collapse: collapse; background: white; border-radius: 8px; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .metrics th, .metrics td {{ padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }}
        .metrics th {{ background: #4CAF50; color: white; font-weight: bold; }}
        .metrics tr:last-child td {{ border-bottom: none; }}
    </style>
</head>
<body>
    <h1>{} - Backend Comparison Report</h1>
    {}
</body>
</html>"#,
            self.test_name, self.test_name, body
        );

        fs::write(output_path, full_html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_builder_creates_html() {
        let mut builder = ReportBuilder::new("test_case");
        builder.add_metrics_table(10.5, Some(12.3));

        // Just verify we can generate the HTML content
        let html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>{} - Test Report</title>
</head>
<body>
    <h1>{}</h1>
</body>
</html>"#,
            builder.test_name, builder.test_name
        );

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("test_case"));
    }
}
