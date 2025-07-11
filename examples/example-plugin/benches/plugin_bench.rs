use criterion::{criterion_group, criterion_main, Criterion};
use vexy_svgo_core::parser::parse_svg_string;
use vexy_svgo_plugin_example::AccessibilityPlugin;
use vexy_svgo_plugin_sdk::Plugin;

fn generate_complex_svg(elements: usize) -> String {
    let mut svg = String::from(r#"<svg viewBox="0 0 1000 1000" xmlns="http://www.w3.org/2000/svg">"#);
    
    // Add various element types
    for i in 0..elements {
        match i % 5 {
            0 => svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="50" height="50" fill="#{}"/>"#,
                i * 10 % 1000,
                (i * 10) / 1000 * 10,
                format!("{:06x}", i * 1000)
            )),
            1 => svg.push_str(&format!(
                r#"<circle cx="{}" cy="{}" r="25" fill="blue"/>"#,
                i * 10 % 1000 + 25,
                (i * 10) / 1000 * 10 + 25
            )),
            2 => svg.push_str(&format!(
                r#"<path d="M{},{} L{},{} L{},{} Z" fill="green"/>"#,
                i * 10 % 1000, (i * 10) / 1000 * 10,
                i * 10 % 1000 + 50, (i * 10) / 1000 * 10,
                i * 10 % 1000 + 25, (i * 10) / 1000 * 10 + 50
            )),
            3 => svg.push_str(&format!(
                r#"<text x="{}" y="{}">Text {}</text>"#,
                i * 10 % 1000,
                (i * 10) / 1000 * 10,
                i
            )),
            4 => svg.push_str(&format!(
                r#"<g id="group-{}"><rect x="{}" y="{}" width="40" height="40"/></g>"#,
                i,
                i * 10 % 1000,
                (i * 10) / 1000 * 10
            )),
            _ => unreachable!(),
        }
    }
    
    svg.push_str("</svg>");
    svg
}

fn benchmark_small_svg(c: &mut Criterion) {
    let svg = generate_complex_svg(10);
    let mut plugin = AccessibilityPlugin::new();
    
    c.bench_function("accessibility_small_svg", |b| {
        b.iter(|| {
            let mut doc = parse_svg_string(&svg).unwrap();
            plugin.optimize(&mut doc).unwrap();
        });
    });
}

fn benchmark_medium_svg(c: &mut Criterion) {
    let svg = generate_complex_svg(100);
    let mut plugin = AccessibilityPlugin::new();
    
    c.bench_function("accessibility_medium_svg", |b| {
        b.iter(|| {
            let mut doc = parse_svg_string(&svg).unwrap();
            plugin.optimize(&mut doc).unwrap();
        });
    });
}

fn benchmark_large_svg(c: &mut Criterion) {
    let svg = generate_complex_svg(1000);
    let mut plugin = AccessibilityPlugin::new();
    
    c.bench_function("accessibility_large_svg", |b| {
        b.iter(|| {
            let mut doc = parse_svg_string(&svg).unwrap();
            plugin.optimize(&mut doc).unwrap();
        });
    });
}

fn benchmark_with_custom_labels(c: &mut Criterion) {
    use vexy_svgo_plugin_example::AccessibilityConfig;
    use std::collections::HashMap;
    
    let svg = generate_complex_svg(100);
    
    let mut custom_labels = HashMap::new();
    for i in 0..100 {
        custom_labels.insert(format!("group-{}", i), format!("Custom Label {}", i));
    }
    
    let config = AccessibilityConfig {
        custom_labels,
        ..Default::default()
    };
    
    let mut plugin = AccessibilityPlugin::with_config(config);
    
    c.bench_function("accessibility_custom_labels", |b| {
        b.iter(|| {
            let mut doc = parse_svg_string(&svg).unwrap();
            plugin.optimize(&mut doc).unwrap();
        });
    });
}

criterion_group!(
    benches,
    benchmark_small_svg,
    benchmark_medium_svg,
    benchmark_large_svg,
    benchmark_with_custom_labels
);
criterion_main!(benches);