use std::{process::{Command, self}, env};
use base64::{Engine as _, engine::general_purpose};
use settings::Settings;
use log::Log;
use chrono::prelude::*;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

mod utils;
mod settings;
mod log;

fn main() {
    utils::output_separator_start();
    let my_settings = Settings::initialise();
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    args.remove(0);
    let argument: String = args.join(" ");
    let unique_id: String = format!("{}_{}",general_purpose::STANDARD_NO_PAD.encode(argument.as_bytes()),Utc::now().timestamp_micros());
    let my_log = Log::initialise(my_settings.to_owned(), unique_id.clone());
    my_log.debug(format!("Launching cron using following settings: \n\n+++++\n{}+++++",Settings::get_content()).as_str());    
    my_log.info(format!("Launching command: {}", argument).as_str());
    my_log.debug(format!("Launching as {} -c {}",
        my_settings.to_owned().get_value("General", "SHELL", "bash"),
        argument
    ).as_str());
    let output = Command::new(my_settings.to_owned().get_value("General", "SHELL", "bash"))
       .arg("-c")
        .arg(argument.clone())
        .output()
        .expect("failed to execute command");
    let status_code: i32 = output.status.code().unwrap();
    if status_code != 0 {
        my_log.error(format!("Command failed and return the following output: \n{}",String::from_utf8_lossy(&output.stderr).to_string()).as_str());
    } else {

        my_log.info("Output:");
        my_log.info(utils::format_output_command(String::from_utf8_lossy(&output.stdout).to_string().as_str()).as_str());
    }
    my_log.info(format!("Status Code: {}",status_code.to_string().as_str()).as_str());
    my_log.info(&unique_id);
    if status_code != 0 {
        my_log.error("Command failed!!!! Exiting!!!!!!!!");
        if my_settings.to_owned().get_value("General", "sendmail", "false").parse().unwrap(){
            let replyto = my_settings.to_owned().get_value("Email", "SMTP_REPLY_TO", "");
            let from = my_settings.to_owned().get_value("Email", "SMTP_USER", "");
            let to = my_settings.to_owned().get_value("Email", "SMTP_TO", "");
            let email = Message::builder()
                .from(from.parse().unwrap())
                .reply_to(replyto.parse().unwrap())
                .to(to.parse().unwrap())
                .subject("Cron failed")
                .header(ContentType::TEXT_PLAIN)
                .body(
                    format!("Your cron with command: {} has failed.\nStatusCode:{}\nOutput:{}",argument,status_code,String::from_utf8_lossy(&output.stderr))
                )
                .unwrap();

            let creds = Credentials::new(from, my_settings.get_value("Email", "SMTP_PASSWORD", ""));

            // Open a remote connection to gmail
            let mailer = SmtpTransport::relay("smtp.gmail.com")
                .unwrap()
                .credentials(creds)
                .build();

            // Send the email
            match mailer.send(&email) {
                Ok(_) => println!("Email sent successfully!"),
                Err(e) => panic!("Could not send email: {e:?}"),
            }
        }
    } 
    utils::output_separator_end();
    process::exit(status_code);
}
