


fn main() {
    #[cfg(target_arch = "wasm32")]
    println!("wasm");
    
    #[cfg(not(target_arch = "wasm32"))]
    println!("not wasm");
    
    let f = shared::foo(1);
    println!("Hello, world: {}", f);
}
