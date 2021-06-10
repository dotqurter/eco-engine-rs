use std::{error::Error, sync::Arc};
use vulkano::{
    device::{Device, DeviceExtensions, Features, Queue},
    instance::{Instance, InstanceExtensions, PhysicalDevice, PhysicalDeviceType, QueueFamily},
};
pub struct Graphics {
    inst: Arc<Instance>,
    pub device: Arc<Device>,
}

//TODO: Store all valid queues (renderable or not) into a vector, and allow renderer to do async stuff

impl Graphics {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let inst = Self::create_instance()?;
        // Find the first (dedicated) GPU.
        let phys_device = PhysicalDevice::enumerate(&inst)
            .filter(|x| Self::rate_physdevice(*x) >= 1000)
            .next()
            .expect("No usable Vulkan physdevices found.");

        //For all physdevices, rate each, and pick best one (sorted to front)

        let (device, queue_pool) = Self::create_device(phys_device)?;

        Ok(Self { inst, device })
    }

    fn create_instance() -> Result<Arc<Instance>, Box<dyn Error>> {
        match Instance::new(None, &InstanceExtensions::none(), None) {
            Ok(i) => Ok(i),
            Err(what) => panic!("Couldn't build instance {}", what),
            //TODO: replace with proper err-handling later
        }
    }

    fn create_device(
        phys: PhysicalDevice,
    ) -> Result<(Arc<Device>, Vec<Arc<Queue>>), Box<dyn Error>> {
        let queuefamilies = phys
            .queue_families()
            .find(|&q| q.supports_graphics())
            .expect("failed to find supported queue-families");

        for queuefam in phys.queue_families() {
            println!("Found {:?}", queuefam);
        }
        //TODO: Figure out why this only returns a single queue.
        let (device, mut queues) = {
            Device::new(
                phys,
                &Features::none(),
                &DeviceExtensions::none(),
                [(queuefamilies, 0.5)].iter().cloned(),
            )
            .expect("Failed to create device.")
        };

        let mut queuevec: Vec<Arc<Queue>> = Vec::new();
        for queue2 in queues {
            queuevec.push(queue2);
        }

        //Return the first queue.
        //let queue = queues.next().unwrap();

        Ok((device, queuevec))
    }
    /// Tries to return the best card to use.
    /// TODO: Allow it to judge based on max (etc), and has a geometry shader
    fn rate_physdevice(dev: PhysicalDevice) -> i32 {
        let mut score = 0i32;
        if dev.ty() == PhysicalDeviceType::DiscreteGpu {
            score += 1000
        };
        score
    }

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
                                             //Rest of the other vulkan things are already handled by `drop`
    }
}
