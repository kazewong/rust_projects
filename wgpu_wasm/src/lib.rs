use console_log;
use log::{debug, info};
use wasm_bindgen::prelude::*;
use winit::{event_loop::EventLoop, window::WindowBuilder};

mod runtime;

#[wasm_bindgen(start)]
pub async fn run() {

    console_log::init_with_level(log::Level::Debug).expect("error initializing log");

    // Getting the canvas element
    let canvas = runtime::get_canvas();
    let (width, height) = (canvas.client_width(), canvas.client_height());
    debug!("Acquired canvas with size {}x{}", width, height);

    // Creating the window
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .map(|window| {
            let _ = window.request_inner_size(winit::dpi::LogicalSize::new(width, height));
            window
            })
        .expect("failed to create window");    
    debug!("Created window");
}
