use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, DeviceCreateInfo, Features, QueueCreateInfo};

pub fn init() {
    let instance = Instance::new(InstanceCreateInfo::default()).expect("failed to create instance");

    let physical = PhysicalDevice::enumerate(&instance).next().expect("no device available");

    let queue_family = physical.queue_families().find(|&q| q.supports_graphics()).expect("could not find graphics queue family");

    let (device, mut queues) = Device::new(physical, DeviceCreateInfo {
        queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
        ..Default::default()
    }).expect("failed to create device");
}