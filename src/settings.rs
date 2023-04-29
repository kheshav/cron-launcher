use ini::Ini;


#[allow(dead_code)]
pub struct Settings {
    conf: ini::Ini
}

impl Settings{
    pub fn initialise() -> Settings {
        Settings {
            conf: Ini::load_from_file("conf/settings.ini").unwrap()
        }
    }

    pub fn get_value(self, section: &str, key: &str, default: &str) -> String {
        let section = self.conf.section(Some(section)).unwrap().to_owned();
        section.get(key).unwrap_or_else(|| default).to_string()
    }
}
