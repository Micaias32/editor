use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    // Create the event loop
    let event_loop = EventLoop::new().unwrap();

    // Create a window
    let window = WindowBuilder::new()
        .with_title("Hello window")
        .build(&event_loop)
        .unwrap();

    // Run the event loop
    event_loop
        .run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        elwt.exit();
                    }
                    _ => {}
                },

                Event::AboutToWait => {
                    // Ask for a redraw every loop iteration
                    window.request_redraw();
                }

                Event::RedrawRequested(_) => {
                    // Rendering would go here
                }

                _ => {}
            }
        })
        .unwrap();
}
