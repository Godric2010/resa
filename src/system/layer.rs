use crate::system::device_info_collector::DeviceInfo;
use crate::system::log::Log;
use crate::system::window::Window;

pub struct System {
    pub window: Window,
}

impl System {
    pub fn init() -> System {
        let sys = System {
            window: Window::init(),
        };

        sys
    }

    pub fn init_logging(output_path: &str) {

        Log::init(output_path);

        let mut device_infos = DeviceInfo::new();
        device_infos.collect_data();
        device_infos.write_to_log();
    }
}