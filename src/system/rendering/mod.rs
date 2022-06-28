pub mod vulkan;
pub mod mesh;

use winit::window::{Window};
use crate::system::rendering::mesh::mesh::Mesh;


pub trait IRenderer {
    fn new(window: &Window) -> Self where Self: Sized;
    fn render(&self, meshes: &[Mesh]);
    fn recreate_pipelines(&self, window_width: u32, window_height: u32);
    fn dispose(&self);
}