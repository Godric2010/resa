use std::error::Error;
use std::io::{Read, Write};
use crate::system::ini::IniFileData;

pub trait Serializable{
    fn save(&self) -> String;
    fn load(path: &str, file_string: &str) -> Self;
}


pub fn read_file(path: &str) -> String{

    let file_exists = std::path::Path::new(path).exists();
    if !file_exists {
        return String::new();
    }
    let mut file = std::fs::File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

pub fn create_new_file(path: &str, data_str: &str) {
    let mut file = std::fs::File::create(path).expect("create failed");
    file.write_all(data_str.as_bytes()).expect("write failed");
}
