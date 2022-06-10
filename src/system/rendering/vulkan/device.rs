use ash::{Device, Instance};
use ash::extensions::khr::{Surface, Swapchain};
use ash::vk::{DeviceCreateInfo, DeviceQueueCreateInfo, PhysicalDevice, PhysicalDeviceFeatures, Queue, QueueFlags, SurfaceKHR};

pub struct VkDevice {
    pub physical_device: PhysicalDevice,
    pub logical_device: Device,
    pub present_queue: Queue,
    pub queue_family_index: u32,
}

impl VkDevice {
    pub fn new(instance: &Instance, surface_loader: &Surface, surface: &SurfaceKHR) -> Self {
        let logical_device: Device;
        let physical_device: PhysicalDevice;
        let mut present_queue: Queue;
        let mut queue_family_index: u32;
        unsafe {
            let pdevices = instance.enumerate_physical_devices().expect("Physical device error");
            let (pdevice, queue_family_index_usize) = pdevices.iter().find_map(|pdevice| {
                instance.get_physical_device_queue_family_properties(*pdevice)
                    .iter()
                    .enumerate()
                    .find_map(|(index, info)| {
                        let supports_graphic_and_surface = info.queue_flags.contains(QueueFlags::GRAPHICS)
                            && surface_loader.get_physical_device_surface_support(*pdevice, index as u32, *surface).unwrap();

                        if supports_graphic_and_surface {
                            Some((*pdevice, index))
                        } else {
                            None
                        }
                    })
            }).expect("Couldn't find suitable device.");

            physical_device = pdevice;
            queue_family_index = queue_family_index_usize as u32;
            let device_extensions_names_raw = [Swapchain::name().as_ptr()];
            let features = PhysicalDeviceFeatures {
                shader_clip_distance: 1,
                ..Default::default()
            };
            let priorities = [1.0];
            let queue_info = DeviceQueueCreateInfo::builder()
                .queue_family_index(queue_family_index)
                .queue_priorities(&priorities);

            let device_create_info = DeviceCreateInfo::builder()
                .queue_create_infos(std::slice::from_ref(&queue_info))
                .enabled_extension_names(&device_extensions_names_raw)
                .enabled_features(&features);

            logical_device = instance.create_device(pdevice, &device_create_info, None).expect("Device creation failed!");

            present_queue = logical_device.get_device_queue(queue_family_index as u32, 0);
        }

        VkDevice {
            physical_device,
            logical_device,
            present_queue,
            queue_family_index,
        }
    }

    pub fn destroy(&self) {
        unsafe {
            self.logical_device.destroy_device(None);
        }
    }
}