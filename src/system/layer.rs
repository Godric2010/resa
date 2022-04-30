use crate::system::device_info_collector::DeviceInfo;
use crate::system::ini;
use crate::system::file::Serializable;
use crate::system::log::Log;
use crate::system::window::ResaWindow;

pub struct System {
    pub deviceInfo: DeviceInfo,
    pub window: ResaWindow,
}

impl System {
    pub fn init(log_output: &str) -> System {

        let ini_data = ini::IniFileData::load("settings.ini","");

        let mut sys = System {
            deviceInfo: DeviceInfo::new(),
            window: ResaWindow::init(&ini_data.window_data),
        };

        Log::init(log_output);

        sys.deviceInfo.collect_data();
        sys.window.build_window(sys.deviceInfo.os_name);

        sys
    }

    fn init_logging(self, output_path: &str) {

        Log::init(output_path);
        self.deviceInfo.write_to_log();
    }
}