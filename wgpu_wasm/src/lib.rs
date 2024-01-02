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

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
    .with_inner_size(winit::dpi::PhysicalSize::new(512, 512))
    .build(&event_loop).unwrap();
    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let dst = doc.get_element_by_id("canvas")?;
            let canvas = web_sys::Element::from(window.canvas().unwrap());
            dst.append_child(&canvas).ok()?;
            Some(())
        })
        .expect("Couldn't append canvas to document body.");
        debug!("Created window");
    }

    info!("Window size: {}x{}", window.inner_size().width, window.inner_size().height);

    let _ = window.request_inner_size(winit::dpi::PhysicalSize::new(400, 200));
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
