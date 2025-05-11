

#[cfg(not(target_arch = "wasm32"))]
pub fn foo(b: bool) -> bool {
    println!("not wasm");
    b
}

#[cfg(target_arch = "wasm32")]
pub fn foo(i: i32) -> i32 {
    println!("wasm");
    i
}

