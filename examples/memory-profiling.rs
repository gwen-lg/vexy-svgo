// this_file: examples/memory-profiling.rs

//! Memory profiling example for Vexy SVGO
//! 
//! This example demonstrates how to profile memory usage when processing
//! large SVG files with different configurations.

use std::alloc::{GlobalAlloc, Layout, System};
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use vexy_svgo_core::{optimize_default, optimize_with_config, Config, PluginConfig};

/// Custom allocator that tracks memory usage
struct TrackingAllocator;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static PEAK_ALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let ptr = System.alloc(layout);
        
        if !ptr.is_null() {
            let current = ALLOCATED.fetch_add(size, Ordering::SeqCst) + size;
            let mut peak = PEAK_ALLOCATED.load(Ordering::SeqCst);
            
            while current > peak {
                match PEAK_ALLOCATED.compare_exchange_weak(
                    peak,
                    current,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                ) {
                    Ok(_) => break,
                    Err(p) => peak = p,
                }
            }
        }
        
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(size, Ordering::SeqCst);
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;

/// Reset memory tracking counters
fn reset_memory_tracking() {
    ALLOCATED.store(0, Ordering::SeqCst);
    PEAK_ALLOCATED.store(0, Ordering::SeqCst);
}

/// Get current memory usage in bytes
fn get_current_memory() -> usize {
    ALLOCATED.load(Ordering::SeqCst)
}

/// Get peak memory usage in bytes
fn get_peak_memory() -> usize {
    PEAK_ALLOCATED.load(Ordering::SeqCst)
}

/// Format bytes as human-readable string
fn format_bytes(bytes: usize) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    
    format!("{:.2} {}", size, UNITS[unit_idx])
}

/// Profile memory usage for a single optimization
fn profile_optimization(
    svg: &str,
    config: Option<Config>,
    label: &str,
) -> Result<ProfileResult, Box<dyn std::error::Error>> {
    println!("\n📊 Profiling: {}", label);
    println!("   Input size: {}", format_bytes(svg.len()));
    
    reset_memory_tracking();
    let initial_memory = get_current_memory();
    
    let start = Instant::now();
    
    let result = match config {
        Some(cfg) => optimize_with_config(svg, cfg)?,
        None => optimize_default(svg)?,
    };
    
    let duration = start.elapsed();
    let peak_memory = get_peak_memory();
    let final_memory = get_current_memory();
    
    let stats = ProfileResult {
        label: label.to_string(),
        input_size: svg.len(),
        output_size: result.data.len(),
        compression_ratio: 1.0 - (result.data.len() as f64 / svg.len() as f64),
        duration_ms: duration.as_millis(),
        initial_memory,
        peak_memory,
        final_memory,
        memory_overhead: peak_memory.saturating_sub(initial_memory),
    };
    
    println!("   Output size: {} ({:.1}% reduction)", 
        format_bytes(stats.output_size),
        stats.compression_ratio * 100.0
    );
    println!("   Duration: {:.2}s", stats.duration_ms as f64 / 1000.0);
    println!("   Peak memory: {}", format_bytes(stats.peak_memory));
    println!("   Memory overhead: {}", format_bytes(stats.memory_overhead));
    
    Ok(stats)
}

#[derive(Debug)]
struct ProfileResult {
    label: String,
    input_size: usize,
    output_size: usize,
    compression_ratio: f64,
    duration_ms: u128,
    initial_memory: usize,
    peak_memory: usize,
    final_memory: usize,
    memory_overhead: usize,
}

/// Generate a large SVG for testing
fn generate_large_svg(num_elements: usize) -> String {
    let mut svg = String::from(r#"<svg width="10000" height="10000" xmlns="http://www.w3.org/2000/svg">"#);
    
    for i in 0..num_elements {
        let x = (i % 100) * 100;
        let y = (i / 100) * 100;
        
        match i % 4 {
            0 => svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="80" height="80" fill="#{:06x}" opacity="0.8"/>"#,
                x, y, i * 1000
            )),
            1 => svg.push_str(&format!(
                r#"<circle cx="{}" cy="{}" r="40" fill="#{:06x}" stroke="#000" stroke-width="2"/>"#,
                x + 40, y + 40, i * 2000
            )),
            2 => svg.push_str(&format!(
                r#"<path d="M {} {} l 60 0 l 0 60 l -60 0 Z" fill="#{:06x}" transform="rotate(45 {} {})"/>"#,
                x + 20, y + 20, i * 3000, x + 50, y + 50
            )),
            _ => svg.push_str(&format!(
                r#"<text x="{}" y="{}" font-family="Arial" font-size="20" fill="#333">Test{}</text>"#,
                x + 10, y + 50, i
            )),
        }
    }
    
    svg.push_str("</svg>");
    svg
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Vexy SVGO Memory Profiling Tool");
    println!("==================================");
    
    // Check if a file was provided
    let args: Vec<String> = std::env::args().collect();
    
    let results = if args.len() > 1 {
        // Profile provided file
        let path = &args[1];
        println!("📁 Loading file: {}", path);
        
        let svg = fs::read_to_string(path)?;
        println!("   File size: {}", format_bytes(svg.len()));
        
        let mut results = Vec::new();
        
        // Test with default config
        results.push(profile_optimization(&svg, None, "Default configuration")?);
        
        // Test with minimal plugins
        let mut minimal_config = Config::default();
        minimal_config.plugins = vec![
            PluginConfig::Name("removeComments".to_string()),
            PluginConfig::Name("removeMetadata".to_string()),
        ];
        results.push(profile_optimization(&svg, Some(minimal_config), "Minimal plugins")?);
        
        // Test with multipass
        let mut multipass_config = Config::default();
        multipass_config.multipass = true;
        results.push(profile_optimization(&svg, Some(multipass_config), "Multipass enabled")?);
        
        // Test with all plugins
        let full_config = Config::default();
        results.push(profile_optimization(&svg, Some(full_config), "All plugins")?);
        
        results
    } else {
        // Generate and profile synthetic SVGs
        println!("📐 No file provided, generating synthetic SVGs...");
        
        let sizes = vec![
            (100, "Small (100 elements)"),
            (1000, "Medium (1,000 elements)"),
            (5000, "Large (5,000 elements)"),
            (10000, "Extra Large (10,000 elements)"),
        ];
        
        let mut results = Vec::new();
        
        for (size, label) in sizes {
            let svg = generate_large_svg(size);
            fs::write(format!("profile_test_{}.svg", size), &svg)?;
            
            results.push(profile_optimization(&svg, None, label)?);
        }
        
        results
    };
    
    // Print summary
    println!("\n📈 Summary Report");
    println!("================");
    println!();
    println!("| Label | Input | Output | Reduction | Time | Peak Memory | Memory/Input |");
    println!("|-------|-------|--------|-----------|------|-------------|--------------|");
    
    for result in &results {
        println!(
            "| {} | {} | {} | {:.1}% | {:.2}s | {} | {:.2}x |",
            result.label,
            format_bytes(result.input_size),
            format_bytes(result.output_size),
            result.compression_ratio * 100.0,
            result.duration_ms as f64 / 1000.0,
            format_bytes(result.peak_memory),
            result.peak_memory as f64 / result.input_size as f64,
        );
    }
    
    // Memory efficiency analysis
    println!("\n💡 Memory Efficiency Analysis");
    println!("=============================");
    
    let avg_memory_ratio = results.iter()
        .map(|r| r.peak_memory as f64 / r.input_size as f64)
        .sum::<f64>() / results.len() as f64;
    
    println!("Average memory usage: {:.2}x input size", avg_memory_ratio);
    
    if avg_memory_ratio < 5.0 {
        println!("✅ Excellent memory efficiency!");
    } else if avg_memory_ratio < 10.0 {
        println!("⚠️  Good memory efficiency, but could be improved for very large files");
    } else {
        println!("❌ High memory usage - consider optimization for large files");
    }
    
    // Performance tips
    println!("\n💡 Performance Tips");
    println!("==================");
    println!("1. For files > 10MB, consider using streaming mode");
    println!("2. Disable unused plugins to reduce memory overhead");
    println!("3. Use parallel processing for files with many independent elements");
    println!("4. Consider splitting very large files into chunks");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_tracking() {
        reset_memory_tracking();
        assert_eq!(get_current_memory(), 0);
        assert_eq!(get_peak_memory(), 0);
        
        // Allocate some memory
        let _data = vec![0u8; 1024];
        
        // Memory should be tracked
        assert!(get_current_memory() >= 1024);
        assert!(get_peak_memory() >= 1024);
    }
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0.00 B");
        assert_eq!(format_bytes(1023), "1023.00 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.00 GB");
    }
}