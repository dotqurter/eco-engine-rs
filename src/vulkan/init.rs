use std::{error::Error, sync::Arc};
use vulkano::{
    buffer::CpuAccessibleBuffer,
    device::{Device, DeviceExtensions, Features, Queue, QueuesIter},
    instance::{Instance, InstanceExtensions, PhysicalDevice, PhysicalDeviceType},
};

/// A struct containing the vulkan_instance, the vulkan_device, and the vulkan_queues.
pub struct Graphics {
    pub inst: Arc<Instance>,
    pub device: Arc<Device>,
    pub queues: QueuesIter,
}

// TODO: Store all valid queues (renderable or not) into a vector, and allow renderer to do async stuff

impl Graphics {
    /// Creates the main Graphics setup, like initializing an instance, creating queues, etc.
    /// This doesn't create any buffers, handle buffers on your own.
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let inst = Self::create_instance()?;
        // Find the first (dedicated) GPU.
        let phys_device = PhysicalDevice::enumerate(&inst)
            .filter(|x| Self::rate_physdevice(*x) >= 1000)
            /* For all physdevices, rate each, and pick best one (sorted to front)*/
            .next()
            /* Find the first device that works for this, and return that one.*/
            .expect("No usable Vulkan physdevices found.");

        println!("Using device: {}", phys_device.name());
        // A little bit of pretty print for the terminal.

        println!("{:#?}", phys_device.supported_features());
        // What capabilities does the card have?

        let (device, mut queues) = Self::create_device(phys_device)?;
        // This returns the first queuefamily set of queues, which on most GPUs is ALL_CAPABILITIES

        // For my GPU, it only returns a single queue (First queue family only has a single queue), so this code is fine for now.
        // TODO: Fix the code for other devices to use automatically as many queues as possible; probably make a vector of available queues?
        // That might make managing the correct/unused queues difficult between multiple segments of the code.

        Ok(Self {
            inst,
            device,
            queues,
        })
    }

    ///Creates the initial vulkan instance for the renderer.
    fn create_instance() -> Result<Arc<Instance>, Box<dyn Error>> {
        match Instance::new(None, &InstanceExtensions::none(), None) {
            Ok(i) => Ok(i),
            Err(what) => panic!("Couldn't build instance {}", what),
            // TODO: replace with proper err-handling later
        }
    }

    /// Creates a vk::Device from a vk::PhysicalDevice, and returns both the Device and a Queue-Iter (for multiple queues, if a family provides.)
    // TODO: Fix so that the device creation can use multiple different queue-families, instead of just one.
    fn create_device(phys: PhysicalDevice) -> Result<(Arc<Device>, QueuesIter), Box<dyn Error>> {
        let mut minimum_features = Features::none();
        minimum_features.geometry_shader = true;

        let (device, mut queues) = {
            Device::new(
                phys,
                &minimum_features,
                &DeviceExtensions::none(),
                [(phys.queue_families().next().unwrap(), 0.5)]
                    .iter()
                    .cloned(),
            )
            .expect("Failed to create device.")
        };

        Ok((device, queues))
    }

    /// Runs a simple algorithm to filter the best card from all the cards in the system.
    // TODO: Allow it to judge based on max (etc)
    fn rate_physdevice(dev: PhysicalDevice) -> i32 {
        return (PhysicalDeviceType::DiscreteGpu == dev.ty()) as i32 * 1000;
    }

    /// Runs the main loop for the program, window handling done here.
    pub fn render_loop(&mut self) {
        println!("Running!");

        // TODO: Replace with window handling.
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            break;
        }
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {
        println!("Dropping Graphics::Self"); // TODO: Replace with log::Log();
                                             // Rest of the other vulkan things are already handled by `drop`
    }
}
