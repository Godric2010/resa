use ash::Device;
use ash::extensions::khr::Swapchain;
use ash::vk::{KhrPortabilitySubsetFn, PhysicalDeviceFeatures};
use crate::system::rendering::vulkan::global::VkInstance;

pub struct VkLogicalDevice{
    device: Device,

}

impl VkLogicalDevice{

    pub fn new(instance: &VkInstance) -> VkLogicalDevice{
        let features = PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };
        let extension_names = [Swapchain::name().as_ptr(), KhrPortabilitySubsetFn::name().as_ptr()];
        let device = instance.create_device(&features, &extension_names);
        let graphics_queue_index = instance.selected_physical_device.graphics_queue_family_index.clone();
        let compute_queue_index = instance.selected_physical_device.compute_queue_family_index.clone();
        let graphics_queue = unsafe{ device.get_device_queue(graphics_queue_index, 0)};
        let compute_queue = unsafe{device.get_device_queue(compute_queue_index,0)};


        VkLogicalDevice{
            device,

        }
    }

    pub fn destroy(&self){
        println!("Destroy device");
        unsafe {
            self.device.destroy_device(None);
        }
    }

}