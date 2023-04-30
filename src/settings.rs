use ini::Ini;
use std::fs;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Settings {
    conf: ini::Ini
}

impl Settings{
    
    pub fn initialise() -> Settings {
        Settings {
            conf: Ini::load_from_file(Settings::get_path()).unwrap()
        }
    }

    pub fn get_path() -> String {
        "conf/settings.ini".to_string()
    }

    pub fn get_content() -> String {
        fs::read_to_string("conf/settings.ini").expect("cannot read file")

    }

    pub fn get_conf(self) -> ini::Ini {
       self.conf
    }

    pub fn get_value(self, section: &str, key: &str, default: &str) -> String {
        let section = self.conf.section(Some(section)).unwrap().to_owned();
        section.get(key).unwrap_or_else(|| default).to_string()
    }
}
