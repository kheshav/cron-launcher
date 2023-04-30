use crate::{utils, settings::Settings};


pub enum LogType{
    Debug,
    Info,
    Warning,
    Error
}

impl LogType {

    pub fn value(&self) -> String {
        match self{
            self::LogType::Info => "INFO".to_string(),
            self::LogType::Debug => "DEBUG".to_string(),
            self::LogType::Warning => "WARNING".to_string(),
            self::LogType::Error => "ERROR".to_string(),
        }
    }
}

pub struct Log {
    path: String,
    settings: Settings
}

impl Log {

    pub fn initialise(path: String,settings: Settings) -> Log{
        Log{
            path,
            settings
        }
    }

    pub fn debug(&self,value: &str){
        let loglevel = self.settings.to_owned().get_value("General", "loglevel", "INFO");
        match loglevel == LogType::value(&LogType::Debug) {
            true => {
                let output: String = utils::format_output(LogType::Debug, value);
            },
            _ => ()
        }
    }

    pub fn info(&self,value: &str){
        let output: String = utils::format_output(LogType::Info, value);
    }

    pub fn warning(&self,value: &str){
        let loglevel = self.settings.to_owned().get_value("General", "loglevel", "WARNING");
        match loglevel == LogType::value(&LogType::Warning) {
            true => {
                let output: String = utils::format_output(LogType::Warning, value);
            },
            _ => ()
        }
    }

    pub fn error(&self, value: &str){
        let output: String = utils::format_output(LogType::Error, value);
    }
}
