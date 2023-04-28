use std::{process::Command, env};
use base64::{Engine as _, engine::general_purpose};
use ini::Ini;

mod utils;

fn main() {
    let conf = Ini::load_from_file("conf/settings.ini").unwrap();
    let general_section = conf.section(Some("General")).unwrap(); 
    utils::output_separator_start();
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let argument: String = args.join(" ");
    let unique_id: String = general_purpose::STANDARD_NO_PAD.encode(argument.as_bytes());
    utils::format_output(format!("Launching command: {}", argument).as_str());
    println!("using shell: {}",general_section.get("shell").unwrap());
    let output = Command::new(general_section.get("shell").unwrap_or_else(|| "bash"))
        .arg("-c")
        .arg(argument)
        .output()
        .expect("failed to execute command");
    utils::format_output("Output:");
    utils::format_output_command(String::from_utf8_lossy(&output.stdout).to_string().as_str());
    utils::format_output(format!("Status Code: {}",output.status.code().unwrap().to_string().as_str()).as_str());
    utils::format_output(&unique_id);
    utils::output_separator_end();
}
