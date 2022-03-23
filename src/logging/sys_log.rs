extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;

pub struct log {
    output_path: String,

}

impl log {
    pub fn init() {
        println!("Logger attached!");
    }

    pub fn write(message: &'static str){
        let timestamp = log::get_time();
        println!("{}: {}", timestamp, message);
    }

    fn get_time() -> String{
        let now = SystemTime::now();
        let datetime: DateTime<Utc> = now.into();
        let datetime_str = datetime.format("%d/%m/%Y %T").to_string();
        datetime_str
    }
}


