use crate::system::mesh::Mesh;
use crate::system::rendering::renderer::RendererLoop;
use winit::window::Window;
use crate::system::rendering::vulkan::renderer::VkRenderer;

pub struct VkRendererLoop {
    renderer: VkRenderer,
// recreate_swapchain: bool,
// window_resized: bool,
// previous_fence_i: usize,
}

impl RendererLoop for VkRendererLoop {
    fn init(window: &Window) -> Self {
        let renderer = VkRenderer::init(window);

        // let frames_in_flight = 0;//renderer.get_image_count();

        Self {
            renderer,
            // recreate_swapchain: false,
            // window_resized:false,
            // previous_fence_i:0
        }
    }

    fn render(&mut self, meshes: &[Mesh]) {

        // if self.window_resized {
        //     self.window_resized = false;
        //     self.recreate_swapchain = false;
        //     self.renderer.handle_window_resize();
        // }
        //
        // if self.recreate_swapchain{
        //     self.recreate_swapchain = false;
        //     self.renderer.recreate_swapchain();
        // }
        //
        // let (image_i, suboptimal, acquire_future) = match self.renderer.acquire_swapchain_image() {
        //     Ok(r) => r,
        //     Err(AcquireError::OutOfDate) =>{
        //         self.recreate_swapchain = true;
        //         return;
        //     }
        //     Err(e) => panic!("Failed to acquire next image: {:?}",e),
        // };
        //
        // if suboptimal{
        //     self.recreate_swapchain = true;
        // }
        //
        // if let Some(image_fence) = &self.fences[image_i]{
        //     image_fence.wait(None).unwrap();
        // }
        //
        // self.renderer.update_uniform(image_i, meshes);
        //
        // let something_needs_all_gpu_resources = false;
        // let previous_future = match self.fences[self.previous_fence_i].clone() {
        //     None => self.renderer.synchronize().boxed(),
        //     Some(fence) => {
        //         if something_needs_all_gpu_resources{
        //             fence.wait(None).unwrap();
        //         }
        //         fence.boxed()
        //     }
        // };
        //
        // let result = self.renderer.flush_next_future(previous_future, acquire_future, image_i);
        //
        // self.fences[image_i] = match result {
        //     Ok(fence) => Some(Arc::new(fence)),
        //     Err(FlushError::OutOfDate) => {
        //         self.recreate_swapchain = true;
        //         None
        //     }
        //     Err(e) => {
        //         println!("Failed to flush future: {:?}",e);
        //         None
        //     }
        // };
        //
        // self.previous_fence_i = image_i;
    }

    fn resize(&mut self) {
        //self.window_resized = true;
    }
}
