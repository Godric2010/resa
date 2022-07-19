use std::borrow::Borrow;
use std::ops::Deref;
use crate::system::device_info_collector::DeviceInfo;
use crate::system::ini;
use crate::system::file::Serializable;
use crate::system::log::Log;
use crate::system::window::ResaWindow;

pub struct System {
    pub device_info: DeviceInfo,
    pub window: ResaWindow,
}

impl System {
    pub fn init(log_output: &str) -> System {
        Log::init(log_output);

        let ini_data = ini::IniFileData::load("settings.ini", "");
        let mut device_info = DeviceInfo::new();
        device_info.collect_data();

        let window = ResaWindow::init(&ini_data.window_data, device_info.os_name);
        let mut sys = System { device_info, window };

        let gpu_name = sys.window.get_gpu_name().clone();
        sys.device_info.set_gpu_data(gpu_name.as_str(), 0);
        sys.device_info.write_to_log();

        sys
    }

    pub fn init_logging(self, output_path: &str) {
        Log::init(output_path);
        self.device_info.write_to_log();
    }
}