extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;
use crate::logging::log_flags::LogFlags;
use ansi_term::Colour;
use lazy_static::lazy_static;
use std::sync::{Mutex};

lazy_static! {
    pub static ref LOG_SINGLETON: Mutex<Option<Log>> = Mutex::new(None);
}

#[derive(Copy, Clone)]
pub struct Log {
    output_path: &'static str,
    flags: LogFlags,
}

impl Log {
    pub fn init() {

        let mut st = LOG_SINGLETON.lock().unwrap();
        if st.is_none() {
            let log = Log {
                output_path: "",
                flags: LogFlags::WRITE_ERROR | LogFlags::WRITE_WARNING | LogFlags::WRITE_MESSAGE | LogFlags::WRITE_TO_CONSOLE,
            };
            *st = Some(log);
        }
        else{
            panic!("The logger is already instantiated! A singleton can be only instantiated once!");
        }
    }

    pub fn get() -> Log{
        if LOG_SINGLETON.lock().unwrap().is_some(){
            let log = LOG_SINGLETON.lock().unwrap().unwrap();
            log
        }
        else {
            panic!("The logger has not been initialized yet! Init first before you use it.")
        }
    }

    pub fn write(&mut self, message: &'static str) {

        if !self.flags.contains(LogFlags::WRITE_MESSAGE){
            return;
        }

        let prefix = "MSG";
        let output = self.build_output(prefix, message);

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


