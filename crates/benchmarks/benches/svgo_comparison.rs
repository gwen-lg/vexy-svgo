//! Benchmarks comparing vexy-svgo against node-based svgo
//! 
//! Note: This requires svgo to be installed (`npm install -g svgo`)

use criterion::{black_box, criterion_group, Criterion, BenchmarkId};
use std::process::Command;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

const SIMPLE_SVG: &str = r#"<svg width="100" height="100">
    <rect x="10" y="10" width="80" height="80" fill="red"/>
    <circle cx="50" cy="50" r="30" fill="blue" opacity="0.5"/>
</svg>"#;

const MEDIUM_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="500" height="500" viewBox="0 0 500 500">
    <defs>
        <linearGradient id="grad1" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" style="stop-color:rgb(255,255,0);stop-opacity:1" />
            <stop offset="100%" style="stop-color:rgb(255,0,0);stop-opacity:1" />
        </linearGradient>
        <filter id="blur">
            <feGaussianBlur in="SourceGraphic" stdDeviation="5"/>
        </filter>
    </defs>
    <g transform="translate(50,50)">
        <rect width="100" height="100" fill="url(#grad1)" />
        <circle cx="200" cy="200" r="50" fill="blue" opacity="0.5"/>
        <path d="M 10 10 L 90 10 L 90 90 L 10 90 Z" fill="green"/>
        <text x="250" y="250" font-family="Arial" font-size="20">Hello World</text>
    </g>
    <g mask="url(#mask1)">
        <rect x="100" y="100" width="300" height="200" fill="purple" filter="url(#blur)"/>
    </g>
</svg>"#;

fn create_complex_svg(elements: usize) -> String {
    let mut svg = String::from(r#"<svg width="1000" height="1000" xmlns="http://www.w3.org/2000/svg">"#);
    
    // Add various elements
    for i in 0..elements {
        let x = (i % 100) * 10;
        let y = (i / 100) * 10;
        
        match i % 4 {
            0 => svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="8" height="8" fill="#{:06x}"/>"#,
                x, y, i * 1000
            )),
            1 => svg.push_str(&format!(
                r#"<circle cx="{}" cy="{}" r="4" fill="#{:06x}"/>"#,
                x + 4, y + 4, i * 2000
            )),
            2 => svg.push_str(&format!(
                r#"<path d="M {} {} l 8 0 l 0 8 l -8 0 Z" fill="#{:06x}"/>"#,
                x, y, i * 3000
            )),
            _ => svg.push_str(&format!(
                r#"<text x="{}" y="{}" font-size="8">{}</text>"#,
                x, y + 8, i % 10
            )),
        }
    }
    
    svg.push_str("</svg>");
    svg
}

fn optimize_with_vexy(svg: &str) -> Result<String, Box<dyn std::error::Error>> {
    vexy_svgo_core::optimize_default(svg)
        .map(|result| result.data)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn optimize_with_svgo(svg: &str) -> Result<String, Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let input_path = temp_dir.path().join("input.svg");
    let output_path = temp_dir.path().join("output.svg");
    
    fs::write(&input_path, svg)?;
    
    let output = Command::new("svgo")
        .arg(&input_path)
        .arg("-o")
        .arg(&output_path)
        .output()?;
    
    if !output.status.success() {
        return Err(format!("svgo failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    
    fs::read_to_string(&output_path).map_err(|e| e.into())
}

fn bench_comparison_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison_simple_svg");
    
    // Check if svgo is available
    let svgo_available = Command::new("svgo")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    
    group.bench_function("vexy_svgo", |b| {
        b.iter(|| {
            let result = optimize_with_vexy(black_box(SIMPLE_SVG));
            assert!(result.is_ok());
        })
    });
    
    if svgo_available {
        group.bench_function("node_svgo", |b| {
            b.iter(|| {
                let result = optimize_with_svgo(black_box(SIMPLE_SVG));
                assert!(result.is_ok());
            })
        });
    }
    
    group.finish();
}

fn bench_comparison_medium(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison_medium_svg");
    
    let svgo_available = Command::new("svgo")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    
    group.bench_function("vexy_svgo", |b| {
        b.iter(|| {
            let result = optimize_with_vexy(black_box(MEDIUM_SVG));
            assert!(result.is_ok());
        })
    });
    
    if svgo_available {
        group.bench_function("node_svgo", |b| {
            b.iter(|| {
                let result = optimize_with_svgo(black_box(MEDIUM_SVG));
                assert!(result.is_ok());
            })
        });
    }
    
    group.finish();
}

fn bench_comparison_by_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("comparison_by_element_count");
    group.sample_size(20); // Reduce sample size for slower benchmarks
    
    let svgo_available = Command::new("svgo")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    
    for size in [100, 500, 1000] {
        let svg = create_complex_svg(size);
        
        group.bench_with_input(
            BenchmarkId::new("vexy_svgo", size),
            &svg,
            |b, svg| {
                b.iter(|| {
                    let result = optimize_with_vexy(black_box(svg));
                    assert!(result.is_ok());
                })
            },
        );
        
        if svgo_available {
            group.bench_with_input(
                BenchmarkId::new("node_svgo", size),
                &svg,
                |b, svg| {
                    b.iter(|| {
                        let result = optimize_with_svgo(black_box(svg));
                        assert!(result.is_ok());
                    })
                },
            );
        }
    }
    
    group.finish();
}

fn bench_comparison_quality(_c: &mut Criterion) {
    // This benchmark measures output quality by comparing file sizes
    let complex_svg = create_complex_svg(100);
    let test_svgs = vec![
        ("simple", SIMPLE_SVG),
        ("medium", MEDIUM_SVG),
        ("complex_100", complex_svg.as_str()),
    ];
    
    println!("\n=== Output Quality Comparison ===");
    
    for (name, svg) in test_svgs {
        let original_size = svg.len();
        
        // Optimize with vexy
        if let Ok(vexy_output) = optimize_with_vexy(svg) {
            let vexy_size = vexy_output.len();
            let vexy_reduction = 100.0 - (vexy_size as f64 / original_size as f64 * 100.0);
            
            println!("{} - Vexy SVGO:", name);
            println!("  Original: {} bytes", original_size);
            println!("  Optimized: {} bytes", vexy_size);
            println!("  Reduction: {:.1}%", vexy_reduction);
        }
        
        // Optimize with svgo if available
        if let Ok(svgo_output) = optimize_with_svgo(svg) {
            let svgo_size = svgo_output.len();
            let svgo_reduction = 100.0 - (svgo_size as f64 / original_size as f64 * 100.0);
            
            println!("{} - Node SVGO:", name);
            println!("  Original: {} bytes", original_size);
            println!("  Optimized: {} bytes", svgo_size);
            println!("  Reduction: {:.1}%", svgo_reduction);
        }
        
        println!();
    }
}

fn bench_real_world_files(c: &mut Criterion) {
    let mut group = c.benchmark_group("real_world_files");
    group.sample_size(10); // Reduce sample size for file I/O benchmarks
    
    let svgo_available = Command::new("svgo")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);
    
    // Look for real SVG files in testdata directory
    let testdata_dir = Path::new("testdata");
    if testdata_dir.exists() {
        let mut test_files = Vec::new();
        
        // Collect up to 5 test files
        for entry in fs::read_dir(testdata_dir).unwrap() {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("svg") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        let name = path.file_name().unwrap().to_str().unwrap().to_string();
                        test_files.push((name, content));
                        if test_files.len() >= 5 {
                            break;
                        }
                    }
                }
            }
        }
        
        for (filename, content) in &test_files {
            group.bench_with_input(
                BenchmarkId::new("vexy_svgo", filename),
                content,
                |b, svg| {
                    b.iter(|| {
                        let result = optimize_with_vexy(black_box(svg));
                        assert!(result.is_ok());
                    })
                },
            );
            
            if svgo_available {
                group.bench_with_input(
                    BenchmarkId::new("node_svgo", filename),
                    content,
                    |b, svg| {
                        b.iter(|| {
                            let result = optimize_with_svgo(black_box(svg));
                            assert!(result.is_ok());
                        })
                    },
                );
            }
        }
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_comparison_simple,
    bench_comparison_medium,
    bench_comparison_by_size,
    bench_real_world_files,
);

// Run quality comparison after performance benchmarks
fn main() {
    benches();
    bench_comparison_quality(&mut Criterion::default());
    criterion::Criterion::default().final_summary();
}