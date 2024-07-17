use wasm_bindgen::prelude::*;

mod demo;

#[wasm_bindgen]
pub fn wasm_start() {
    main()
}

pub fn main() {
    demo::main();

    // #[cfg(not(target_family = "wasm"))]
    // tekenen::server();
}