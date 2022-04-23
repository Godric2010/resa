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

    }


}