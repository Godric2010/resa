extern crate glfw;

use std::borrow::Borrow;
use glfw::{Action, Context, Key};
use crate::system::ini::{WindowIniData, WindowMode};

pub struct Window{
    width: u32,
    height: u32,
    title: String,
    mode: WindowMode,
}

impl Window{

    pub fn init(ini_data: &WindowIniData) -> Window{
        let instance = Window{

            width: ini_data.window_width,
            height: ini_data.window_height,
            title: ini_data.window_title.to_string(),
            mode: ini_data.window_mode,
        };

        instance
    }

    pub fn start_window_loop(self){

        let mut glfw_handle= glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw_handle.create_window(self.width, self.height, &self.title, self.get_window_mode()).expect("Failed to create GLFW window.");

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

    fn get_window_mode(&self)-> glfw::WindowMode{

       if self.mode == WindowMode::Fullscreen {
           println!("Fullscreen currently not supported!")
       }

        return glfw::WindowMode::Windowed;
    }

}