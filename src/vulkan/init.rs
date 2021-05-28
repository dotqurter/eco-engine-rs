use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use winit::{dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

// TODO Make a Vulkan object

pub fn init(app: crate::data::application::Application) {
    //create the initial instance
    let instance = match Instance::new(None, &InstanceExtensions::none(), None) {
        Ok(i) => i,
        Err(err) => panic!("Instance creation failed: {}", err),
    };

    //enumerate devices
    for phys_devices in PhysicalDevice::enumerate(&instance) {
        println!("Available device: {}", phys_devices.name());
    }

    {
        // window stuff
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(app.name)
            .with_inner_size(LogicalSize::new(app.windowsize.0, app.windowsize.1))
            .build(&event_loop)
            .unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    }
}
