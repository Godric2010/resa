use crate::system::device_info_collector::DeviceInfo;
use crate::system::ini;
use crate::system::file::Serializable;
use crate::system::log::Log;
use crate::system::window::ResaWindow;

pub struct System {
    pub window: ResaWindow,
}

impl System {
    pub fn init() -> System {

        let ini_data = ini::IniFileData::load("settings.ini","");

        let sys = System {
            window: ResaWindow::init(&ini_data.window_data),
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