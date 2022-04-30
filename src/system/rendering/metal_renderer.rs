use crate::system::rendering::renderer::Renderer;

pub struct MetalRenderer{

}

impl Renderer for MetalRenderer {

    fn init() -> MetalRenderer {
        println!("Init metal renderer!");
        return MetalRenderer{};
    }

    fn render(&self) {
        todo!()
    }
}