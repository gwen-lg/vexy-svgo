// this_file: crates/core/tests/parallel_test.rs

use vexy_svgo_core::{
    ast::{Document, Element, Node},
    parallel::{ParallelConfig, process_independent_groups, should_parallelize},
    OptimizeOptions,
};
use std::sync::Arc;

#[test]
fn test_parallel_detection() {
    let mut doc = Document::new();
    
    // Add many elements to trigger parallel processing
    for i in 0..1500 {
        let mut rect = Element::new("rect");
        rect.set_attr("x", &(i * 10).to_string());
        rect.set_attr("y", &(i * 10).to_string());
        rect.set_attr("width", "5");
        rect.set_attr("height", "5");
        doc.root.add_child(Node::Element(rect));
    }
    
    let config = ParallelConfig::default();
    let large_size = 2 * 1024 * 1024; // 2MB
    
    assert!(should_parallelize(&doc, large_size, &config));
}

#[test]
fn test_parallel_processing() {
    let mut parent = Element::new("g");
    
    // Add many independent elements
    for i in 0..100 {
        let mut rect = Element::new("rect");
        rect.set_attr("id", &format!("rect{}", i));
        rect.set_attr("x", &(i * 10).to_string());
        parent.add_child(Node::Element(rect));
    }
    
    // Create a simple processing function
    let processed_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let count_clone = processed_count.clone();
    
    let process_fn = Arc::new(move |elem: &mut Element| {
        // Simulate some processing
        elem.set_attr("processed", "true");
        count_clone.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    });
    
    let result = process_independent_groups(&mut parent, process_fn);
    
    // Check that elements were processed
    assert!(result > 0);
    assert_eq!(
        processed_count.load(std::sync::atomic::Ordering::Relaxed),
        result
    );
    
    // Verify elements were modified
    for child in &parent.children {
        if let Node::Element(elem) = child {
            assert_eq!(elem.attr("processed"), Some("true"));
        }
    }
}

#[test]
fn test_parallel_with_dependencies() {
    let mut parent = Element::new("g");
    
    // Add elements with dependencies
    let mut rect1 = Element::new("rect");
    rect1.set_attr("id", "rect1");
    
    let mut use1 = Element::new("use");
    use1.set_attr("href", "#rect1");
    
    let mut rect2 = Element::new("rect");
    rect2.set_attr("id", "rect2");
    
    parent.add_child(Node::Element(rect1));
    parent.add_child(Node::Element(use1));
    parent.add_child(Node::Element(rect2));
    
    let process_fn = Arc::new(|elem: &mut Element| {
        elem.set_attr("processed", "true");
    });
    
    let result = process_independent_groups(&mut parent, process_fn);
    
    // Should process at least some elements
    assert!(result > 0);
}

#[test]
fn test_parallel_config_in_options() {
    let config = vexy_svgo_core::Config::default();
    let parallel_config = ParallelConfig {
        size_threshold: 512 * 1024, // 512KB
        element_threshold: 500,
        num_threads: 4,
    };
    
    let options = OptimizeOptions::new(config)
        .with_parallel(parallel_config.clone());
    
    assert!(options.parallel.is_some());
    let pc = options.parallel.unwrap();
    assert_eq!(pc.size_threshold, 512 * 1024);
    assert_eq!(pc.element_threshold, 500);
    assert_eq!(pc.num_threads, 4);
}