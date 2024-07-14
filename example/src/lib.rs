
use wasm_bindgen::prelude::*;

mod demo;

#[wasm_bindgen]
pub fn wasm_start() {
    main()
}

fn main() {
    demo::main();
}