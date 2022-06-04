use winit::window::{Window};
use crate::system::mesh::Mesh;


pub trait RendererLoop {
    fn init(window: &Window) -> Self where Self: Sized;
    fn render(&mut self, meshes: &[Mesh]);
    fn resize(&mut self);
}