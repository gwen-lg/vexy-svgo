//! Performance benchmarks for the SVG parser

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use vexy_svgo_core::Parser;

const TINY_SVG: &str = r#"<svg><rect/></svg>"#;

const SIMPLE_SVG: &str = r#"<svg width="100" height="100">
    <rect x="10" y="10" width="80" height="80" fill="red"/>
</svg>"#;

const WITH_NAMESPACES: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
    <defs>
        <rect id="myRect" width="100" height="100"/>
    </defs>
    <use xlink:href="#myRect" x="50" y="50"/>
</svg>"#;

const WITH_STYLES: &str = r#"<svg>
    <style>
        .red { fill: red; }
        .blue { fill: blue; }
        #special { stroke: green; stroke-width: 2; }
    </style>
    <rect class="red" width="50" height="50"/>
    <circle class="blue" cx="100" cy="100" r="30"/>
    <path id="special" d="M 10 10 L 90 90"/>
</svg>"#;

const WITH_ENTITIES: &str = r#"<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd" [
    <!ENTITY custom "Custom Entity">
]>
<svg>
    <text>&custom;</text>
    <text>Regular &amp; Special &lt;chars&gt;</text>
</svg>"#;

const DEEPLY_NESTED: &str = r#"<svg>
    <g>
        <g>
            <g>
                <g>
                    <g>
                        <rect width="10" height="10"/>
                    </g>
                </g>
            </g>
        </g>
    </g>
</svg>"#;

fn bench_parse_tiny(c: &mut Criterion) {
    c.bench_function("parse_tiny_svg", |b| {
        let parser = Parser::new();
        b.iter(|| {
            let result = parser.parse(black_box(TINY_SVG));
            assert!(result.is_ok());
        })
    });
}

fn bench_parse_simple(c: &mut Criterion) {
    c.bench_function("parse_simple_svg", |b| {
        let parser = Parser::new();
        b.iter(|| {
            let result = parser.parse(black_box(SIMPLE_SVG));
            assert!(result.is_ok());
        })
    });
}

fn bench_parse_namespaces(c: &mut Criterion) {
    c.bench_function("parse_with_namespaces", |b| {
        let parser = Parser::new();
        b.iter(|| {
            let result = parser.parse(black_box(WITH_NAMESPACES));
            assert!(result.is_ok());
        })
    });
}

fn bench_parse_styles(c: &mut Criterion) {
    c.bench_function("parse_with_styles", |b| {
        let parser = Parser::new();
        b.iter(|| {
            let result = parser.parse(black_box(WITH_STYLES));
            assert!(result.is_ok());
        })
    });
}

fn bench_parse_entities(c: &mut Criterion) {
    c.bench_function("parse_with_entities", |b| {
        let parser = Parser::new();
        b.iter(|| {
            let result = parser.parse(black_box(WITH_ENTITIES));
            assert!(result.is_ok());
        })
    });
}

fn bench_parse_nested(c: &mut Criterion) {
    c.bench_function("parse_deeply_nested", |b| {
        let parser = Parser::new();
        b.iter(|| {
            let result = parser.parse(black_box(DEEPLY_NESTED));
            assert!(result.is_ok());
        })
    });
}

fn bench_parse_large_attributes(c: &mut Criterion) {
    let mut svg = String::from(r#"<svg>"#);
    let long_value = "a".repeat(1000);
    for i in 0..100 {
        svg.push_str(&format!(
            r#"<rect data-attr{}="{}" width="10" height="10"/>"#,
            i, long_value
        ));
    }
    svg.push_str("</svg>");
    
    c.bench_function("parse_large_attributes", |b| {
        let parser = Parser::new();
        b.iter(|| {
            let result = parser.parse(black_box(&svg));
            assert!(result.is_ok());
        })
    });
}

fn bench_parse_many_elements(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_element_count");
    
    for count in [100, 500, 1000, 5000] {
        let mut svg = String::from(r#"<svg>"#);
        for i in 0..count {
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="1" height="1"/>"#,
                i % 100, i / 100
            ));
        }
        svg.push_str("</svg>");
        
        group.bench_with_input(
            BenchmarkId::from_parameter(count),
            &svg,
            |b, svg| {
                let parser = Parser::new();
                b.iter(|| {
                    let result = parser.parse(black_box(svg));
                    assert!(result.is_ok());
                })
            },
        );
    }
    
    group.finish();
}

fn bench_parse_mixed_content(c: &mut Criterion) {
    let mixed_svg = r#"<svg>
        <text>Some text content</text>
        <g>
            <text>More <tspan>nested</tspan> text</text>
            <!-- A comment -->
            <rect width="100" height="100"/>
            <![CDATA[Some CDATA content]]>
        </g>
        <script><![CDATA[
            function doSomething() {
                console.log("Hello");
            }
        ]]></script>
    </svg>"#;
    
    c.bench_function("parse_mixed_content", |b| {
        let parser = Parser::new();
        b.iter(|| {
            let result = parser.parse(black_box(mixed_svg));
            assert!(result.is_ok());
        })
    });
}

fn bench_parse_complex_paths(c: &mut Criterion) {
    let complex_paths = r#"<svg>
        <path d="M 10 10 C 20 20, 40 20, 50 10 S 80 10, 90 20 Q 100 30, 110 20 T 120 30 A 10 10 0 0 1 130 40 Z"/>
        <path d="M100,200 C100,100 250,100 250,200 S400,300 400,200"/>
        <path d="M 10,30 A 20,20 0,0,1 50,30 A 20,20 0,0,1 90,30 Q 90,60 50,90 Q 10,60 10,30 z"/>
        <path d="M 10 315 L 110 215 A 30 50 0 0 1 162.55 162.45 L 172.55 152.45 A 30 50 -45 0 1 215.1 109.9 L 315 10"/>
    </svg>"#;
    
    c.bench_function("parse_complex_paths", |b| {
        let parser = Parser::new();
        b.iter(|| {
            let result = parser.parse(black_box(complex_paths));
            assert!(result.is_ok());
        })
    });
}

#[cfg(feature = "streaming")]
fn bench_streaming_parser(c: &mut Criterion) {
    use vexy_svgo_core::parse_svg_streaming;
    
    let mut group = c.benchmark_group("streaming_parser");
    
    // Generate a very large SVG to test streaming
    let mut huge_svg = String::from(r#"<svg width="10000" height="10000">"#);
    for i in 0..10000 {
        huge_svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="10" height="10" fill="#{:06x}"/>"#,
            (i % 1000) * 10,
            (i / 1000) * 10,
            i
        ));
    }
    huge_svg.push_str("</svg>");
    
    group.bench_function("streaming_10k_elements", |b| {
        b.iter(|| {
            let result = parse_svg_streaming(black_box(huge_svg.as_bytes()));
            assert!(result.is_ok());
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_parse_tiny,
    bench_parse_simple,
    bench_parse_namespaces,
    bench_parse_styles,
    bench_parse_entities,
    bench_parse_nested,
    bench_parse_large_attributes,
    bench_parse_many_elements,
    bench_parse_mixed_content,
    bench_parse_complex_paths,
);

#[cfg(feature = "streaming")]
criterion_group!(
    streaming_benches,
    bench_streaming_parser,
);

#[cfg(not(feature = "streaming"))]
criterion_main!(benches);

#[cfg(feature = "streaming")]
criterion_main!(benches, streaming_benches);