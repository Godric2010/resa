use crate::system::file;
use crate::system::file::Serializable;

#[derive(Clone, Copy, PartialEq)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
}

pub struct WindowIniData {
    pub window_mode: WindowMode,
    pub window_width: u32,
    pub window_height: u32,
    pub window_title: String,
}

pub struct IniFileData {
    pub window_data: WindowIniData,
    pub logging_path: String,

}

impl Serializable for IniFileData {
    fn save(&self) -> String {
        todo!()
    }

    fn load(path: &str, file_string: &str) -> IniFileData {
        let ini_file_path = "settings.ini";
        let ini_exists = std::path::Path::new(ini_file_path).exists();

        if !ini_exists {
            let default_ini = setup_default_ini();
            file::create_new_file(path, &ini_to_str(&default_ini));
            return default_ini;
        }

        let content = file::read_file(path);//.expect("INI file does not exist");
        let ini_data = string_to_data(&content);

        ini_data
    }
}

fn string_to_data(content: &str) -> IniFileData {
    let mut ini_data = setup_default_ini();

    let mut split_content = content.split("#");
    for ini_type in split_content {
        let mut type_fields = ini_type.split("\n");
        let mut fields_vec: Vec<&str> = type_fields.collect();

        match fields_vec[0] {
            "Window" => { string_to_window_data(&fields_vec, &mut ini_data) }
            "Logging" => { string_to_log_data(&fields_vec, &mut ini_data) }
            _ => {}
        }
    }
    ini_data
}

fn string_to_window_data(lines: &Vec<&str>, ini_data: &mut IniFileData) {
    let mut window_data = WindowIniData { window_mode: WindowMode::Windowed, window_height: 0, window_width: 0, window_title: "".to_string() };

    for (i, _) in lines.iter().enumerate() {
        if i == 0 { continue; }

        let line: &str = lines[i];
        let content: Vec<&str> = line.split("=").collect();

        match content[0] {
            "Width" => { window_data.window_width = content[1].parse::<u32>().unwrap() }
            "Height" => { window_data.window_height = content[1].parse::<u32>().unwrap() }
            "Title" => { window_data.window_title = content[1].to_string() }

            _ => {}
        }
    }

    ini_data.window_data = window_data;
}

fn string_to_log_data(lines: &Vec<&str>, ini_data: &mut IniFileData) {
    for (i, _) in lines.iter().enumerate() {
        if i == 0 { continue; }

        let content: Vec<&str> = lines[i].split("=").collect();
        match content[0] {
            "Path" => { ini_data.logging_path = content[1].to_string() }
            _ => {}
        }
    }
}


fn ini_to_str(ini: &IniFileData) -> String {
    let mut output = String::new();
    output += &window_ini_to_string(&ini.window_data);
    output += &logging_ini_to_string(&ini.logging_path);

    output
}

fn setup_default_ini() -> IniFileData {
    let default_data = IniFileData {
        logging_path: "Desktop".to_string(),
        window_data: WindowIniData {
            window_mode: WindowMode::Windowed,
            window_height: 480,
            window_width: 640,
            window_title: "RESA by Sebastian Borsch".to_string(),
        },
    };

    default_data
}


fn window_ini_to_string(window_data: &WindowIniData) -> String {
    let mut output = String::new();

    let mut mode_str = "";
    if window_data.window_mode == WindowMode::Windowed {
        mode_str = "Windowed";
    } else { mode_str = "Fullscreen"; }


    output += "#Window\n";
    output += "Mode=";
    output += mode_str;
    output += "\nWidth=";
    output += &window_data.window_width.to_string();
    output += "\nHeight=";
    output += &window_data.window_height.to_string();
    output += "\nTitle=";
    output += &window_data.window_title;

    output
}

fn logging_ini_to_string(log_path: &str) -> String {
    let mut output = String::new();

    output += "\n#Logging";
    output += "\nPath=";
    output += log_path;

    output
}