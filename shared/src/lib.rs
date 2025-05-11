
#[cfg(not(feature = "wasm"))]
pub fn foo(b: bool) -> bool {
    println!("not wasm");
    b
}

#[cfg(feature = "wasm")]
pub fn foo(i: i32) -> i32 {
    println!("wasm");
    i
}

