#[macro_use]
extern crate bitflags;
extern crate core;

use directories::UserDirs;
mod system;
use crate::system::layer::System;


fn main() {
    println!("Hello, world!");
    let user_dirs = UserDirs::new();
    let desktop = user_dirs.unwrap().desktop_dir().unwrap().to_str().unwrap().to_owned();

    let system = System::init();
    System::init_logging(&desktop);

    system.window.start_window_loop();
}
