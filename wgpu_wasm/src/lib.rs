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

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
    .with_inner_size(winit::dpi::PhysicalSize::new(512, 512))
    .build(&event_loop).unwrap();
    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.body())
        .and_then(|body| {
            // let dst = doc.get_element_by_id("canvas")?;
            let canvas = window.canvas();
            canvas.set_width(512);
            canvas.set_height(512);
            let canvas = web_sys::Element::from(canvas);
            body.append_child(&canvas).ok();
            Some(())
        })
        .expect("Couldn't append canvas to document body.");
        debug!("Created window");
    }

    info!(" Window size: {}x{}", window.outer_size().width, window.outer_size().height);
    let mut context = render::Context::new(window).await;

    // event_loop.set_control_flow(ControlFlow::Wait);

    info!("Starting event loop");

    // Running the event loop
    let _ = event_loop.run(move |event, _, control_flow|{
        match event{
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == context.window().id() => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == context.window().id() => {
                context.update();
                match context.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        context.resize(context.size)
                    }
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                    Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                }
            }
            Event::RedrawEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                context.window().request_redraw();
            }
            _ => {}
        }
    });
}
