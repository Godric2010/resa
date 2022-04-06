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

    System::init();
    System::init_logging(&desktop);

    // system::log::Log::init(&desktop);
    //
    // let mut device_infos = DeviceInfo::new();
    // device_infos.collect_data();
    // device_infos.write_to_log();
    //
    // let mut logger = system::log::Log::get();
    // logger.write("This is a Log message!");
    // logger.write_warning("This is a warning!");
    // logger.write_error("This is an error!");
}
