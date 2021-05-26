use winit::{
    event::{Event, WindowEvent}, 
    event_loop::{ControlFlow, EventLoop}, 
    window::WindowBuilder};


struct Program {

} 

impl Program {
    pub fn window_init() -> Self {

        
        Self {}
    }
}

fn window_run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).expect("Couldn't get WindowBuilder");

    event_loop.run(move | event, _, control_flow | {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            }
            if window_id == window.id() => *control_flow = ControlFlow::Exit,

            _ => (),
        }
    });
}

fn main() {
    window_run(); // ! This doesn't work as intended, should be by-reference
}