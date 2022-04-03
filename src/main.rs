#[macro_use]
extern crate bitflags;
extern crate core;

use directories::UserDirs;

mod logging;
use logging::device_info_collector::DeviceInfo;

fn main() {
    println!("Hello, world!");
    let user_dirs = UserDirs::new();
    let desktop = user_dirs.unwrap().desktop_dir().unwrap().to_str().unwrap().to_owned();

    logging::sys_log::Log::init(&desktop);

    let mut device_infos = DeviceInfo::new();
    device_infos.collect_data();
    device_infos.write_to_log();

    let mut logger = logging::sys_log::Log::get();
    logger.write("This is a Log message!");
    logger.write_warning("This is a warning!");
    logger.write_error("This is an error!");
}
