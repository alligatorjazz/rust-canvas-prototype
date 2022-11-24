mod utils;
mod canvas_source;
mod universe;
use wasm_bindgen::prelude::*;
extern crate fixedbitset;
extern crate web_sys;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// // TODO: find out why it demands UPPER_SNAKE_CASE
// #[wasm_bindgen] 
// pub fn greet(name: &str) {
//     alert(&format!("Hello {}!", name));
// }
