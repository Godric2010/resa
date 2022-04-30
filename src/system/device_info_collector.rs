extern crate sysinfo;

use num_format::{Buffer, CustomFormat, Grouping};
use sysinfo::{DiskExt, ProcessorExt, System, SystemExt};
use crate::system::log::Log;

#[derive(Copy, Clone)]
pub struct DeviceInfo {
    pub os_name: &'static str,
    pub os_version: &'static str,
    cpu_name: &'static str,
    cpu_cores: usize,
    cpu_frequency: u64,
    ram_size: u64,
    storage_left: u64,
    gpu_vendor: &'static str,
    gpu_ram: u64,
}

impl DeviceInfo {
    pub fn new() -> DeviceInfo {
        let instance = DeviceInfo {
            storage_left: 0,
            gpu_vendor: "Not Initialized",
            gpu_ram: 0,
            ram_size: 0,
            cpu_cores: 0,
            cpu_name: "Not Initialized",
            cpu_frequency: 0,
            os_name: "Not Initialized",
            os_version: "Not Initialized",
        };
        instance
    }

    pub fn collect_data(&mut self) {
        let mut sys = System::new_all();
        sys.refresh_all();

        self.get_operating_system_info(&sys);
        self.get_cpu_info(&sys);
        self.get_ram_data(&sys);
        self.get_storage_data(&sys);
    }

    pub fn set_gpu_data(&mut self, vendor: &'static str, g_ram: u64) {
        self.gpu_vendor = vendor;
        self.gpu_ram = g_ram;
    }

    pub fn write_to_log(self) {
        let mut log_string = String::new();
        log_string.push_str("\n## System Info\n---\n");
        log_string.push_str(self.os_name);
        log_string.push_str(" (Version: ");
        log_string.push_str(self.os_version);

        log_string.push_str(")\n### CPU\n\t");
        log_string.push_str(self.cpu_name);
        log_string.push_str("\n\tCores: ");
        log_string.push_str(self.cpu_cores.to_string().as_str());
        log_string.push_str("\n\tFrequency: ");
        log_string.push_str(self.cpu_frequency.to_string().as_str());
        log_string.push_str("(Mhz)\n");

        log_string.push_str("### RAM\n\tAvailable: ");
        let ram_size = self.ram_size as usize;
        log_string.push_str(&*DeviceInfo::format_big_num(&ram_size));
        log_string.push_str(" (KB)\n");

        log_string.push_str("### GPU\n\t");
        log_string.push_str(self.gpu_vendor);
        log_string.push_str("\n\tGraphics RAM: ");
        let gpu_ram = self.gpu_ram as usize;
        log_string.push_str(&*DeviceInfo::format_big_num(&gpu_ram));

        log_string.push_str("\n### Storage\n\tAvailable: ");
        let storage_left = self.storage_left as usize;
        log_string.push_str(&*DeviceInfo::format_big_num(&storage_left));
        log_string.push_str(" (Bytes)");


        Log::get().write(log_string.as_str())
    }

    fn get_operating_system_info(&mut self, sys: &System) {
        let version = sys.os_version().unwrap();
        self.os_name = DeviceInfo::string_to_static_str(sys.name().unwrap());
        self.os_version = DeviceInfo::string_to_static_str(version);
    }

    fn get_cpu_info(&mut self, sys: &System) {
        let main_cpu = sys.global_processor_info();
        self.cpu_name = DeviceInfo::string_to_static_str(main_cpu.brand().to_string());
        self.cpu_frequency = main_cpu.frequency();
        self.cpu_cores = sys.physical_core_count().unwrap();
    }

    fn get_ram_data(&mut self, sys: &System) {
        self.ram_size = sys.total_memory();
    }

    fn get_storage_data(&mut self, sys: &System) {
        let current_path = std::env::current_exe().unwrap();

        for disk in sys.disks() {
            let mount_point = disk.mount_point();
            if current_path.starts_with(mount_point) {
                self.storage_left = disk.available_space();
                break;
            }
        }
    }

    fn format_big_num(number: &usize) -> String {
        let format = CustomFormat::builder().grouping(Grouping::Standard).separator(".").build().unwrap();
        let mut buf = Buffer::new();
        buf.write_formatted(number, &format);

        let num_string = buf.as_str().to_string();
        num_string
    }

    fn string_to_static_str(s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }
}