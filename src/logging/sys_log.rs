extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;
use crate::logging::log_flags::LogFlags;
use ansi_term::Colour;


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

        let prefix = "MSG";
        let output = self.build_output(prefix, message);
        self.messages.push(output.to_string());

        if !self.flags.contains(LogFlags::WRITE_TO_CONSOLE){
            return;
        }
        println!("{}",  output);
    }

    pub fn write_warning(&mut self, warning: &'static str){

        if !self.flags.contains(LogFlags::WRITE_WARNING){
            return;
        }

        let prefix = "WARN";
        let output = self.build_output(prefix, warning);
        self.messages.push(output.to_string());

        if !self.flags.contains(LogFlags::WRITE_TO_CONSOLE){
            return;
        }
        println!("{}", Colour::Yellow.bold().paint(output));
    }

    pub fn write_error(&mut self, error: &'static str){

        if !self.flags.contains(LogFlags::WRITE_WARNING){
            return;
        }

        let prefix = "ERR";
        let output = self.build_output(prefix, error);
        self.messages.push(output.to_string());

        if !self.flags.contains(LogFlags::WRITE_TO_CONSOLE){
            return;
        }
        println!("{}", Colour::Red.bold().paint(output));
    }

    fn build_output(&self, prefix: &'static str, message: &'static str) -> String{
        let timestamp = self.get_time().to_owned();
        let output = "[".to_owned() + &timestamp + "] " +  prefix + ": " + message;
        output
    }

    fn get_time(&self) -> String {
        let now = SystemTime::now();
        let datetime: DateTime<Utc> = now.into();
        let datetime_str = datetime.format("%T").to_string();
        datetime_str
    }
}


