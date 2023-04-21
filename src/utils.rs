use std::time::{SystemTime, UNIX_EPOCH};


pub fn format_output(value: &str){
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("[{}] {}",since_the_epoch.as_millis(),value);
}

pub fn format_output_command(value: &str){
    println!("{}",value);
}

pub fn output_separator_start(){
    format_output("-------BEGIN---------");
}

pub fn output_separator_end(){
    format_output("--------END----------");
}
