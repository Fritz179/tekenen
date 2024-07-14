
use tekenen::platform::PlatformTrait;
use wasm_bindgen::prelude::*;

mod demo;

#[wasm_bindgen]
pub fn wasm_start() {
    main()
}

fn main() {
    tekenen::platform::Platform::log(77);
    demo::main();
}