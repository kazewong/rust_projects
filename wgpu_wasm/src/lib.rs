use wasm_bindgen::prelude::*;
use console_log;
use log::debug;

mod runtime;

#[wasm_bindgen]
pub async fn run() {

    console_log::init_with_level(log::Level::Debug).expect("error initializing log");

    let canvas = runtime::get_canvas();
    let (width, height) = (canvas.client_width(), canvas.client_height());
    debug!("Acquired canvas with size {}x{}", width, height);
}
