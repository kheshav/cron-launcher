use std::{process::{Command, self}, env};
use base64::{Engine as _, engine::general_purpose};
use settings::Settings;
use log::Log;

mod utils;
mod settings;
mod log;

fn main() {
    utils::output_separator_start();
    let my_settings = Settings::initialise();
    let my_log = Log::initialise(Settings::get_path(), my_settings.to_owned());
    my_log.debug(format!("Launching cron using following settings: \n\n+++++\n{}+++++",Settings::get_content()).as_str());    
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let argument: String = args.join(" ");
    let unique_id: String = general_purpose::STANDARD_NO_PAD.encode(argument.as_bytes());
    my_log.info(format!("Launching command: {}", argument).as_str());
    my_log.debug(format!("Launching as {} -c {}",
        my_settings.to_owned().get_value("General", "SHELL", "bash"),
        argument
    ).as_str());
    let output = Command::new(my_settings.get_value("General", "SHELL", "bash"))
        .arg("-c")
        .arg(argument)
        .output()
        .expect("failed to execute command");
    let status_code: i32 = output.status.code().unwrap();
    if status_code != 0 {
        my_log.error(format!("Command failed and return the following output: \n{}",String::from_utf8_lossy(&output.stderr).to_string()).as_str());
    } else {

        my_log.info("Output:");
        utils::format_output_command(String::from_utf8_lossy(&output.stdout).to_string().as_str());
    }
    my_log.info(format!("Status Code: {}",status_code.to_string().as_str()).as_str());
    my_log.info(&unique_id);
    if status_code != 0 {
        my_log.error("Command failed!!!! Exiting!!!!!!!!");
    } 
    utils::output_separator_end();
    process::exit(status_code);
}
