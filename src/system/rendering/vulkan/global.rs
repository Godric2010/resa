use std::ffi::CStr;
use std::fmt::Error;
use std::os::raw::c_char;
use ash::{Entry, Instance, Device};
use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;
use ash::vk::{API_VERSION_1_3, ApplicationInfo, DeviceCreateInfo, DeviceQueueCreateInfo, ExtensionProperties, InstanceCreateInfo, make_api_version, PhysicalDevice, PhysicalDeviceFeatures, PhysicalDeviceType, Queue, QueueFamilyProperties, QueueFlags, SurfaceKHR, TRUE};
use ash_window::{create_surface, enumerate_required_extensions};
use winit::window::Window;

pub struct VkInstance {
    instance: Instance,
    pub surface_handle: SurfaceKHR,
    pub surface: Surface,
    pub physical_devices: Vec<VkPhysicalDevice>,
    pub selected_physical_device: VkPhysicalDevice,
}

#[derive(Clone)]
pub struct VkPhysicalDevice {
    pub physical_device: PhysicalDevice,
    pub name: String,
    device_type: PhysicalDeviceType,
    extensions: Vec<ExtensionProperties>,
    features: PhysicalDeviceFeatures,
    pub graphics_queue_family_index: u32,
    pub compute_queue_family_index: u32,
}

impl VkInstance {
    pub fn new(window: &Window, gpu_name: Option<String>) -> Result<Self, Error> {
        println!("Create VK instance");
        let entry = VkInstance::create_entry();
        let instance = VkInstance::create_instance(&entry, window);
        let surface_handle = unsafe { create_surface(&entry, &instance, window, None) }.unwrap();
        let surface = Surface::new(&entry, &instance);


        let physical_devices_result = VkInstance::get_physical_devices(&instance, &surface, surface_handle);
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


        Ok(VkInstance {
            instance,
            surface_handle,
            surface,
            physical_devices,
            selected_physical_device,
        })
    }

    pub fn destroy(&self) {
        println!("Destroy vk instance");
        unsafe {
            self.surface.destroy_surface(self.surface_handle, None);
            self.instance.destroy_instance(None)
        };
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

    fn get_physical_devices(instance: &Instance, presentation_surface: &Surface, surface_handle: SurfaceKHR) -> Result<Vec<VkPhysicalDevice>, Error> {
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
                let graphics_queue_family_index: u32 = VkInstance::find_best_graphics_queue_index(&physical_device, presentation_surface, surface_handle, &queue_family_properties).unwrap();
                let compute_queue_family_index = VkInstance::find_best_compute_queue_index(&queue_family_properties).unwrap();

                vk_physical_devices.push(
                    VkPhysicalDevice {
                        physical_device,
                        name,
                        device_type,
                        extensions,
                        features,
                        graphics_queue_family_index,
                        compute_queue_family_index,
                    });
            }
        };
        println!("Physical devices found: {}", physical_devices.len());


        Ok(vk_physical_devices)
    }

    fn find_best_graphics_queue_index(physical_device: &PhysicalDevice, presentation_surface: &Surface, surface_handle: SurfaceKHR, queue_family_properties: &Vec<QueueFamilyProperties>) -> Result<u32, Error> {
        for (i, queue_family) in queue_family_properties.iter().enumerate() {
            let is_graphics_queue = queue_family.queue_flags.contains(QueueFlags::GRAPHICS);
            let presentation_surface_supported = unsafe {
                presentation_surface.get_physical_device_surface_support(
                    physical_device.clone(),
                    i as u32,
                    surface_handle).unwrap()
            };

            if !is_graphics_queue || !presentation_surface_supported {
                continue;
            }
            return Ok(i as u32);
        }

        Err(Error::default())
    }

    fn find_best_compute_queue_index(queue_family_properties: &Vec<QueueFamilyProperties>) -> Result<u32, Error> {
        for (i, queue_family) in queue_family_properties.iter().enumerate() {
            if !queue_family.queue_flags.contains(QueueFlags::COMPUTE) {
                continue;
            }
            return Ok(i as u32);
        }

        Err(Error::default())
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
            let features = physical_device.features;
            let has_requested_features = true;// features.geometry_shader == TRUE; // Todo: find a fix for that!

            if physical_device.device_type == PhysicalDeviceType::DISCRETE_GPU && has_requested_features {
                return physical_device;
            }
        }
        &physical_devices[0]
    }

    pub fn create_device(&self,
                         requested_features: &PhysicalDeviceFeatures,
                         requested_extensions: &Vec<*const c_char>/*[*const c_char; 2]*/) -> Device {
        let priorities = [1.0];
        let graphics_queue_info = [DeviceQueueCreateInfo::builder()
            .queue_family_index(self.selected_physical_device.graphics_queue_family_index)
            .queue_priorities(&priorities).build()];

        let compute_queue_info = [DeviceQueueCreateInfo::builder()
            .queue_family_index(self.selected_physical_device.compute_queue_family_index)
            .queue_priorities(&priorities).build()];

        let create_info = DeviceCreateInfo::builder()
            .queue_create_infos(&graphics_queue_info)
            .queue_create_infos(&compute_queue_info)
            .enabled_features(requested_features)
            .enabled_extension_names(requested_extensions);

        let logical_device: Device = unsafe {
            self.instance.create_device(self.selected_physical_device.physical_device,
                                        &create_info,
                                        None).unwrap()
        };


        logical_device
    }
}