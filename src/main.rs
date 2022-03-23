#[macro_use]
extern crate bitflags;

mod logging;

fn main() {
    println!("Hello, world!");

    let mut logger = logging::sys_log::Log::init();
    logger.write("This is a Log message!");
}
