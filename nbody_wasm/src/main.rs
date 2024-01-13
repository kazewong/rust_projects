use std::vec;

use winit::{window::WindowBuilder, event::{Event, WindowEvent}};

mod physics;
// mod simulation;

fn main() {
    let particle1 = physics::Particle::new([1.0, 0.0, 0.0], [0.0, -4.0, 0.0], 1.0, 1.0);
    let particle2 = physics::Particle::new([-1.0, 0.0, 0.0], [0.0, 4.0, 0.0], 1.0, 1.0);
    let mut particles = vec![particle1, particle2];
    let dt = 0.01;
    for i in 0..100{
        physics::evolve(&mut particles, dt);
        println!("Particle 1 position: {:?}, Particle 2 position: {:?}", 
                    particles[0].position, particles[1].position);
        }
    use winit::event_loop::EventLoop;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop);

    

    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        control_flow.set_poll();
    
        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        control_flow.set_wait();
    
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                control_flow.set_exit();
            },

            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in MainEventsCleared, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            },
            _ => ()
        }
    });
}
