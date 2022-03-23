#[macro_use]
extern crate bitflags;

mod logging;

fn main() {
    println!("Hello, world!");

    let mut logger = logging::sys_log::Log::init();
    logger.write("This is a Log message!");
    logger.write_warning("This is a warning!");
    logger.write_error("This is an error!");
}
