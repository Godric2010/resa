use std::ffi::CStr;
use std::fmt::Error;
use std::os::raw::c_char;
use ash::{Entry, Instance, Device};
use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Swapchain;
use ash::vk::{API_VERSION_1_3, ApplicationInfo, DeviceCreateInfo, DeviceQueueCreateInfo, ExtensionProperties, InstanceCreateInfo, KhrPortabilitySubsetFn, make_api_version, PhysicalDevice, PhysicalDeviceFeatures, PhysicalDeviceType, QueueFamilyProperties, QueueFlags};
use ash_window::enumerate_required_extensions;
use winit::window::Window;

pub struct VkInstance {
    instance: Instance,
    physical_devices: Vec<VkPhysicalDevice>,
    selected_physical_device: VkPhysicalDevice,
    logical_device: Device,
}

#[derive(Clone)]
struct VkPhysicalDevice {
    physical_device: PhysicalDevice,
    name: String,
    device_type: PhysicalDeviceType,
    extensions: Vec<ExtensionProperties>,
    features: PhysicalDeviceFeatures,
    queue_family_index: u32,
}

impl VkInstance {
    pub fn new(window: &Window, gpu_name: Option<String>) -> Result<Self, Error> {
        println!("Create VK instance");
        let entry = VkInstance::create_entry();
        let instance = VkInstance::create_instance(&entry, window);

        let physical_devices_result = VkInstance::get_physical_devices(&instance);
        if physical_devices_result.is_err() {
            Err("Creation of vk instance failed!").unwrap()
        }
        let physical_devices = physical_devices_result.unwrap();

        let selected: &VkPhysicalDevice = match gpu_name {
            Some(name) => { VkInstance::select_physical_device_by_name(&physical_devices, name) }
            None => { VkInstance::select_physical_device(&physical_devices) }
        };

        let selected_physical_device = selected.clone();
        println!("{}", selected_physical_device.name);

        let features = PhysicalDeviceFeatures {
            shader_clip_distance: 1,
            ..Default::default()
        };

        let extension_names = [Swapchain::name().as_ptr(), KhrPortabilitySubsetFn::name().as_ptr() ];
        let logical_device = VkInstance::create_device(&instance,
                                                       &selected_physical_device,
                                                       &features,
                                                       &extension_names,
        );

        Ok(VkInstance {
            instance,
            physical_devices,
            selected_physical_device,
            logical_device,
        })
    }

    pub fn destroy(&self) {
        println!("Destroy vk instance");
        unsafe {
            self.logical_device.destroy_device(None);
        }
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

    fn get_physical_devices(instance: &Instance) -> Result<Vec<VkPhysicalDevice>, Error> {
        let physical_devices = unsafe { instance.enumerate_physical_devices().expect("Failed to load physical devices!") };
        if physical_devices.iter().count() == 0 {
            Error::default();
        }
        let mut vk_physical_devices: Vec<VkPhysicalDevice> = Vec::new();
        unsafe {
            for p_device in physical_devices.iter() {
                let physical_device = p_device.clone();

                let device_properties = instance.get_physical_device_properties(physical_device);
                let name = CStr::from_ptr(device_properties.device_name.as_ptr()).to_str().unwrap().to_owned();
                let device_type = device_properties.device_type;
                let extensions = instance.enumerate_device_extension_properties(physical_device).expect("Could not enumerate physical device extensions...");
                let features = instance.get_physical_device_features(physical_device);

                let queue_family_properties = instance.get_physical_device_queue_family_properties(physical_device);
                let queue_family_index: u32 = VkInstance::find_best_queue_family_index(queue_family_properties).unwrap();
                vk_physical_devices.push(
                    VkPhysicalDevice {
                        physical_device,
                        name,
                        device_type,
                        extensions,
                        features,
                        queue_family_index,
                    });
            }
        };
        println!("Physical devices found: {}", physical_devices.len());


        Ok(vk_physical_devices)
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

    fn select_physical_device_by_name(physical_devices: &Vec<VkPhysicalDevice>, gpu_name: String) -> &VkPhysicalDevice {
        for physical_device in physical_devices.iter() {
            if physical_device.name == gpu_name {
                return physical_device;
            }
        }
        VkInstance::select_physical_device(physical_devices)
    }

    fn select_physical_device(physical_devices: &Vec<VkPhysicalDevice>) -> &VkPhysicalDevice {
        for physical_device in physical_devices.iter() {
            if physical_device.device_type == PhysicalDeviceType::DISCRETE_GPU {
                return physical_device;
            }
        }
        &physical_devices[0]
    }

    fn create_device(instance: &Instance, vk_physical_device: &VkPhysicalDevice,
                     requested_features: &PhysicalDeviceFeatures,
                     requested_extensions: &[*const c_char; 2]) -> Device {
        let priorities = [1.0];
        let queue_info = DeviceQueueCreateInfo::builder()
            .queue_family_index(vk_physical_device.queue_family_index)
            .queue_priorities(&priorities);

        let create_info = DeviceCreateInfo::builder()
            .queue_create_infos(std::slice::from_ref(&queue_info))
            .enabled_features(requested_features)
            .enabled_extension_names(requested_extensions);

        let logical_device: Device = unsafe {
            instance.create_device(vk_physical_device.physical_device,
                                   &create_info,
                                   None).unwrap()
        };

        logical_device
    }
}