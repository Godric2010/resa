
pub enum NullTrait<T: ?Sized>{
    Instance(Box<T>),
    None,
}


pub trait Renderer{
    fn init() -> Self where Self: Sized;
    fn render(&self);
}