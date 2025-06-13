use wasm_bindgen::prelude::*;

// 使该函数可被 JS 调用
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}