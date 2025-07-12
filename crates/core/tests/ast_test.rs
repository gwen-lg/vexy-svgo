use vexy_svgo_core::ast::*; 

fn main() { 
    let elem = Element::new("test"); 
    println!("Memory: {}", elem.estimated_memory_usage()); 
}
