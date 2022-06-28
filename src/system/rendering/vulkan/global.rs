use std::ffi::CStr;
use std::fmt;
use std::fmt::Error;
use std::ops::Index;
use std::os::raw::c_char;
use ash::{Entry, Instance};
use ash::extensions::ext::DebugUtils;
use ash::vk::{API_VERSION_1_3, ApplicationInfo, ExtensionProperties, InstanceCreateInfo, make_api_version, PhysicalDevice, PhysicalDeviceFeatures, QueueFamilyProperties, QueueFlags};
use ash_window::enumerate_required_extensions;
use winit::platform::unix::x11::ffi::False;
use winit::window::Window;

pub struct VkInstance {
    instance: Instance,
}

struct VkPhysicalDevice {
    physical_device: PhysicalDevice,
    extensions: Vec<ExtensionProperties>,
    features: PhysicalDeviceFeatures,
    queue_family_index: u32,
}

impl VkInstance {
    pub fn new(window: &Window) -> Result<Self, Error> {
        println!("Create VK instance");
        let entry = VkInstance::create_entry();
        let instance = VkInstance::create_instance(&entry, window);

        let physical_device = VkInstance::find_best_physical_device(&instance);
        if physical_device.is_err() {
            Err("Creation of vk instance failed!").unwrap()
        }

        Ok(VkInstance {
            instance
        })
    }

    pub fn destroy(&self) {
        println!("Destroy vk instance");
    }

    fn create_entry() -> Entry {
        let entry = unsafe { Entry::load().expect("Failed to load vulkan dll...") };
        entry
    }

    fn create_instance(entry: &Entry, window: &Window) -> Instance {
        let mut instance;
        unsafe {
            let mut required_extensions = enumerate_required_extensions(window).unwrap().to_vec();
            required_extensions.push(DebugUtils::name().as_ptr());

            let layers = [
                CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0"),
            ];
            let mut required_layers: Vec<*const c_char> = layers.iter().map(|raw_name| raw_name.as_ptr()).collect();

            let application_info = ApplicationInfo::builder()
                .application_name(CStr::from_bytes_with_nul_unchecked(b"RESA"))
                .application_version(make_api_version(0, 0, 1, 0))
                .engine_name(CStr::from_bytes_with_nul_unchecked(b"RESA"))
                .engine_version(make_api_version(0, 1, 0, 0))
                .api_version(API_VERSION_1_3);

            let instance_create_info = InstanceCreateInfo::builder()
                .application_info(&application_info)
                .enabled_extension_names(&required_extensions)
                .enabled_layer_names(&required_layers);

            instance = entry.create_instance(&instance_create_info, None).expect("Instance creation failed");
        }
        instance
    }

    fn find_best_physical_device(instance: &Instance) -> Result<PhysicalDevice, Error> {
        let physical_devices = unsafe { instance.enumerate_physical_devices().expect("Failed to load physical devices!") };
        if physical_devices.iter().count() == 0 {
            Error::default();
        }
        let mut vk_physical_devices: Vec<VkPhysicalDevice> = Vec::new();
        unsafe {
            for physical_device in physical_devices.iter() {
                let pd = physical_device.clone();

                let device_properties = instance.get_physical_device_properties(pd);
                // device_properties.

                let extensions = instance.enumerate_device_extension_properties(pd).expect("Could not enumerate physical device extensions...");
                let features = instance.get_physical_device_features(pd);
                let queue_family_properties = instance.get_physical_device_queue_family_properties(pd);
                let queue_family_index: u32 = VkInstance::find_best_queue_family_index(queue_family_properties).unwrap();
                vk_physical_devices.push(
                    VkPhysicalDevice {
                        physical_device: pd,
                        extensions,
                        features,
                        queue_family_index,
                    });
            }
        };

        for device in vk_physical_devices.iter(){
           instance.get_physical_device_properties(*device)
        }

        println!("Physical devices found: {}", physical_devices.len());


        Ok(physical_devices[0])
    }

    fn find_best_queue_family_index(queue_families: Vec<QueueFamilyProperties>) -> Result<u32, Error> {
        let mut index: u32 = 0;
        let mut highest_queue_count: u32 = 0;
        let mut valid_index_set: bool = false;

        for (i, queue_family) in queue_families.iter().enumerate() {
            if !queue_family.queue_flags.contains(QueueFlags::GRAPHICS) {
                continue;
            }

            if !queue_family.queue_flags.contains(QueueFlags::COMPUTE) {
                continue;
            }

            if queue_family.queue_count <= highest_queue_count { continue; }

            highest_queue_count = queue_family.queue_count;
            index = i as u32;
            valid_index_set = true;
        }

        if valid_index_set {
            Ok(index)
        } else {
            Err(Error::default())
        }
    }
}