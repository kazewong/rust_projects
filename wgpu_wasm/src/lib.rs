mod render;
mod runtime;

use console_log;
use log::{debug, info};
use wasm_bindgen::prelude::*;
use winit::{event::*, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};


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

    event_loop.set_control_flow(ControlFlow::Wait);

    // Running the event loop
    let _ = event_loop.run(move |event, elwt|{
        match event{
            Event::WindowEvent{
                event: WindowEvent::CloseRequested,
                ..
            } => {
                info!("Window closed");
                elwt.exit();
            },
            Event::AboutToWait => {
                info!("About to wait");
                window.request_redraw();
            },
            Event::WindowEvent { 
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                info!("Redraw requested");
            },
            _ => ()
        }
    });


}
