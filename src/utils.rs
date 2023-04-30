use chrono::prelude::*;
use crate::log;

pub fn format_output(mode: log::LogType  ,value: &str) -> String {

    println!("[{}][{}][{}] {}",log::LogType::value(&mode),Utc::now(),Utc::now().timestamp_millis(),value);
    format!("[{}][{}][{}] {}",log::LogType::value(&mode),Utc::now(),Utc::now().timestamp_millis(),value)
}


pub fn format_output_command(value: &str) -> String {
    println!("{}",value);
    value.to_string()
}

pub fn output_separator_start() -> String {
    println!("-------BEGIN---------");
    String::from("-------BEGIN---------")
}

pub fn output_separator_end() -> String {
    println!("--------END----------");
    String::from("--------END----------")
}
