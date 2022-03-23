#[macro_use]
extern crate bitflags;
extern crate core;

use std::ops::DerefMut;

mod logging;

fn main() {
    println!("Hello, world!");
    logging::sys_log::Log::init();
    let mut logger = logging::sys_log::Log::get();
    logger.write("This is a Log message!");
    logger.write_warning("This is a warning!");
    logger.write_error("This is an error!");
}
