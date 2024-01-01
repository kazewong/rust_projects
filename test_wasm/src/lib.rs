
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, World!");
    console::log_1(&"Hello, World!".into());
}

#[wasm_bindgen]
pub async fn greet_async() {
    console::log_1(&"Hello, World!".into());
    alert("Hello, World!");
}
