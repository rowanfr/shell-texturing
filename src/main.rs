use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    // Create an EventLoop
    let event_loop = EventLoop::new().unwrap();

    // Create a window
    let window = WindowBuilder::new()
        .with_title("WGSL Shader")
        .build(&event_loop)
        .unwrap();

        
    
    // Handle events
    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Close requested");
                elwt.exit();
            },
            _ => {},
        }
    }).unwrap();
}
