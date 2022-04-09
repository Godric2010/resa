extern crate glfw;

use glfw::{Action, Context, Key};

pub struct Window{
    width: u32,
    height: u32,
    title: String,
}

impl Window{

    pub fn init() -> Window{
        let instance = Window{

            width: 300,
            height: 300,
            title: "resa v 0.0.1 by Sebastian Borsch".to_string(),
        };

        instance
    }

    pub fn start_window_loop(self){

        let mut glfw_handle= glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw_handle.create_window(self.width, self.height, &self.title, glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        while !window.should_close() {
            glfw_handle.poll_events();
            for (_, event) in glfw::flush_messages(&events){
                self.handle_window_events(&mut window, event);
            }
        }

    }

    fn handle_window_events(&self, window: &mut glfw::Window, event: glfw::WindowEvent ){
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) =>{
                window.set_should_close(true)
            }
            _ => {}
        }
    }

}