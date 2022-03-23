mod logging;

fn main() {
    println!("Hello, world!");

    logging::sys_log::log::init();
    logging::sys_log::log::write("This is a log message!");
}
