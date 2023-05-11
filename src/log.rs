use std::io::Write;
use std::fs::OpenOptions;

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
    settings: Settings,
    uniqueid: String,
}

impl Log {

    pub fn initialise(settings: Settings, uniqueid: String) -> Log{
        Log{
            settings,
            uniqueid
        }
    }

    fn write(&self, value: &str) -> std::io::Result<()> {
        let logfolder = self.settings.to_owned().get_value("General", "logfolder", "/tmp/log");
        let filepath: String = format!("{}/{}",logfolder,self.uniqueid);
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(filepath)
            .unwrap();
        f.write_all(format!("{}\n",value).as_bytes())?;
        Ok(())
    }

    pub fn debug(&self,value: &str){
        let loglevel = self.settings.to_owned().get_value("General", "loglevel", "INFO");
        match loglevel == LogType::value(&LogType::Debug) {
            true => {
                let output: String = utils::format_output(LogType::Debug, value);
                self.write(output.as_str());
            },
            _ => ()
        }
    }

    pub fn info(&self,value: &str){
        let output: String = utils::format_output(LogType::Info, value);
        self.write(output.as_str());
    }

    pub fn warning(&self,value: &str){
        let loglevel = self.settings.to_owned().get_value("General", "loglevel", "WARNING");
        match loglevel == LogType::value(&LogType::Warning) {
            true => {
                let output: String = utils::format_output(LogType::Warning, value);
                self.write(output.as_str());
            },
            _ => ()
        }
    }

    pub fn error(&self, value: &str){
        let output: String = utils::format_output(LogType::Error, value);
        self.write(output.as_str());
    }
}
