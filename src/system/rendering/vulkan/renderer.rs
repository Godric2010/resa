use winit::window::Window;
use crate::system::rendering::IRenderer;
use crate::system::rendering::mesh::mesh::Mesh;
use crate::system::rendering::vulkan::global::VkInstance;

pub struct VkRenderer {
    instance: VkInstance,
}

impl IRenderer for VkRenderer {
    fn new(window: &Window) -> Self where Self: Sized {
        println!("Create new vulkan renderer!");
        let instance = VkInstance::new(window, None).expect("Creation of instance failed!");
        VkRenderer {
            instance
        }
    }

    fn render(&self, meshes: &[Mesh]) {

        // println!("Call vulkan render func!");
    }

    fn recreate_pipelines(&self, window_width: u32, window_height: u32) {
        println!("Recreate vulkan pipelines");
    }

    fn dispose(&self) {
        println!("Dispose vulkan renderer");
        self.instance.destroy();
    }
}