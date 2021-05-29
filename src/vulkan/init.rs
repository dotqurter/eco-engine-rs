use vulkano::{Version, instance::{Instance, InstanceExtensions, PhysicalDevice}};
use vulkano_win::VkSurfaceBuild;
use winit::{dpi::LogicalSize, event::{Event, WindowEvent}};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
// TODO Make a Vulkan object

pub fn init(app: crate::data::application::Application) {
    //create the initial instance
    let instance_exten = vulkano_win::required_extensions();
    let instance = match Instance::new(None, Version::major_minor(0, 0) ,&instance_exten, None) {
        Ok(i) => i,
        Err(err) => panic!("Instance creation failed: {}", err),
    };

    //enumerate devices
    let phys_device = PhysicalDevice::enumerate(&instance).next().unwrap();

    // DEBUG message
    println!(
        "Using device: {} (type: {:?})",
        phys_device.name(),
        phys_device.ty()
    );

    {
        // window stuff
        let event_loop = EventLoop::new();
        let surface = WindowBuilder::new()
            .with_title(app.name)
            .with_inner_size(LogicalSize::new(app.windowsize.0, app.windowsize.1))
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == surface.window().id() => *control_flow = ControlFlow::Exit,
                _ => (),
            }
        });
    }
}
