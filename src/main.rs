#[macro_use]
extern crate bitflags;
extern crate core;

use directories::UserDirs;
mod system;
use crate::system::layer::System;


fn main() {
    let user_dirs = UserDirs::new();
    let desktop = user_dirs.unwrap().desktop_dir().unwrap().to_str().unwrap().to_owned();

    let mut system = System::init();
    System::init_logging(&desktop);

    system.window.build_window();
    system.window.run_window_loop();
}
