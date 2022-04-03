extern crate chrono;

use chrono::{DateTime, Local};
use std::time::SystemTime;
use crate::logging::log_flags::LogFlags;
use ansi_term::Colour;
use lazy_static::lazy_static;
use std::sync::{Mutex};
use std::fs::{File, OpenOptions};
use std::io::{Write, Error};

lazy_static! {
    pub static ref LOG_SINGLETON: Mutex<Option<Log>> = Mutex::new(None);
}

#[derive(Copy, Clone)]
pub struct Log {
    output_path: &'static str,
    flags: LogFlags,
}

impl Log {
    pub fn init(path_to_write_to: &str) {
        let path_and_file: String;
        if path_to_write_to.len() == 0 {
            path_and_file = "./Log.md".to_string();
        } else {
            path_and_file = format!("{}/Log.md", path_to_write_to);
        }
        let log_path = Box::leak(path_and_file.into_boxed_str());

        let mut st = LOG_SINGLETON.lock().unwrap();
        if st.is_none() {
            let log = Log {
                output_path: log_path,
                flags: LogFlags::WRITE_ERROR | LogFlags::WRITE_WARNING | LogFlags::WRITE_MESSAGE | LogFlags::WRITE_TO_CONSOLE,
            };
            *st = Some(log);
        } else {
            panic!("The logger is already instantiated! A singleton can be only instantiated once!");
        }

        let result = Log::create_logfile(log_path);
        assert!(result.is_ok())

    }

    pub fn get() -> Log {
        if LOG_SINGLETON.lock().unwrap().is_some() {
            let log = LOG_SINGLETON.lock().unwrap().unwrap();
            log
        } else {
            panic!("The logger has not been initialized yet! Init first before you use it.")
        }
    }

    pub fn write(&mut self, message: &str) {
        if !self.flags.contains(LogFlags::WRITE_MESSAGE) {
            return;
        }

        let prefix = "MSG";
        let output = self.build_output(prefix, message);

        self.write_to_file(&output);

        if !self.flags.contains(LogFlags::WRITE_TO_CONSOLE) {
            return;
        }
        println!("{}", output);
    }

    pub fn write_warning(&mut self, warning: &str) {
        if !self.flags.contains(LogFlags::WRITE_WARNING) {
            return;
        }

        let prefix = "WARN";
        let output = self.build_output(prefix, warning);

        self.write_to_file(&output);

        if !self.flags.contains(LogFlags::WRITE_TO_CONSOLE) {
            return;
        }
        println!("{}", Colour::Yellow.bold().paint(output));
    }

    pub fn write_error(&mut self, error: &str) {
        if !self.flags.contains(LogFlags::WRITE_WARNING) {
            return;
        }

        let prefix = "ERR";
        let output = self.build_output(prefix, error);

        self.write_to_file(&output);

        if !self.flags.contains(LogFlags::WRITE_TO_CONSOLE) {
            return;
        }
        println!("{}", Colour::Red.bold().paint(output));
    }

    fn build_output(&self, prefix: &'static str, message: &str) -> String {
        let timestamp = self.get_time().to_owned();
        let output = "[".to_owned() + &timestamp + "] " + prefix + ": " + message + "\n";
        output
    }

    fn get_time(&self) -> String {
        let now = SystemTime::now();
        let datetime: DateTime<Local> = now.into();
        let datetime_str = datetime.format("%T").to_string();
        datetime_str
    }

    fn create_logfile(path: &'static str) -> Result<(), Error> {
        let mut output = File::create(path)?;
        writeln!(output, "# RESA Logfile\n\n")?;
        Ok(())
    }

    fn write_to_file(&self, line: &String) {
        let mut file = OpenOptions::new().write(true).append(true).open(self.output_path).unwrap();

        if let Err(e) = write!(file, "{}", line) {
            eprintln!("Could not write to file: {}", e);
        }
    }
}


