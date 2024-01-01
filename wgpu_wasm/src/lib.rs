mod render;
mod runtime;

use console_log;
use log::{debug, info};
use wasm_bindgen::prelude::*;
use winit::{event::*, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowBuilder}};


#[wasm_bindgen(start)]
pub async fn run() {

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");

    // Getting the canvas element
    let canvas = runtime::get_canvas();
    let (width, height) = (canvas.client_width(), canvas.client_height());
    debug!("Acquired canvas with size {}x{}", width, height);

    // Creating the window
    let event_loop = EventLoop::new().unwrap();
    // let window = WindowBuilder::new()
    //         .with_min_inner_size(winit::dpi::LogicalSize::new(width, height))
    //     .build(&event_loop)
    //     .expect("failed to create window");
    let window = Window::new(&event_loop).unwrap();
    // let  = window.request_inner_size(winit::dpi::LogicalSize::new(width, height));
    debug!("Created window");

    info!("{:?}", winit::dpi::LogicalSize::new(width, height));
    info!("Window size: {}x{}", window.inner_size().width, window.inner_size().height);

    let mut context = render::Context::new(window).await;

    event_loop.set_control_flow(ControlFlow::Wait);

    info!("Starting event loop");

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
                context.window().request_redraw();
            },
            Event::WindowEvent { 
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                info!("Redraw requested");
                context.update();
                match context.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => context.resize(context.size),
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            },
            _ => ()
        }
    });
}
