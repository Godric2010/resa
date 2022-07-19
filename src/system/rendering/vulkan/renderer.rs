use ash::vk::Extent2D;
use winit::window::Window;
use crate::system::log::Log;
use crate::system::rendering::IRenderer;
use crate::system::rendering::mesh::mesh::Mesh;
use crate::system::rendering::vulkan::device::VkLogicalDevice;
use crate::system::rendering::vulkan::global::VkInstance;

pub struct VkRenderer {
    instance: VkInstance,
    gpu_name: String,
    device: VkLogicalDevice,
}

impl IRenderer for VkRenderer {
    fn new(window: &Window) -> Self where Self: Sized {
        Log::get().write_warning("Create new vulkan renderer! GPU Name is set to none! This must changed to ini variable");
        let instance = VkInstance::new(window, None).expect("Creation of instance failed!");
        let physical_device_name = &instance.selected_physical_device.name;
        let gpu_name = physical_device_name.clone();


        let window_size = Extent2D { width: window.inner_size().width, height: window.inner_size().height };
        let device;
        match VkLogicalDevice::new(&instance, &window_size).resolve(){
            Some(logical_device) => device = logical_device,
            None => panic!("Creation failed!")
        }
        VkRenderer {
            instance,
            gpu_name,
            device,
        }
    }

    fn get_gpu_name(&self) -> &str {
        let name = self.gpu_name.as_str();
        name
    }

    fn render(&self, meshes: &[Mesh]) {

        // println!("Call vulkan render func!");
    }

    fn recreate_pipelines(&self, window_width: u32, window_height: u32) {
        println!("Recreate vulkan pipelines");
    }

    fn dispose(&self) {
        Log::get().write("Disposing renderer");
        self.device.destroy();
        self.instance.destroy();
    }
}