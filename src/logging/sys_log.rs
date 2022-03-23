extern crate chrono;


use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;
use crate::logging::log_flags::LogFlags;


pub struct Log {
    output_path: &'static str,
    flags: LogFlags,
    messages: Vec<String>,

}

impl Log {
    pub fn init() -> Log {
        let mut log = Log {
            output_path: "",
            flags: LogFlags::WRITE_ERROR | LogFlags::WRITE_WARNING | LogFlags::WRITE_MESSAGE | LogFlags::WRITE_TO_CONSOLE,
            messages: vec![],
        };
        log
    }

    pub fn write(&mut self, message: &'static str) {

        if !self.flags.contains(LogFlags::WRITE_MESSAGE){
            return;
        }

        let timestamp = Log::get_time().to_owned();
        let output = timestamp + ": " + message;
        self.messages.push(output.to_string());

        if !self.flags.contains(LogFlags::WRITE_TO_CONSOLE){
            return;
        }
        println!("{}",  output);
    }

    fn get_time() -> String {
        let now = SystemTime::now();
        let datetime: DateTime<Utc> = now.into();
        let datetime_str = datetime.format("%d/%m/%Y %T").to_string();
        datetime_str
    }
}


