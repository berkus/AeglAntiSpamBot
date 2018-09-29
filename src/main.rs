#[macro_use]
extern crate log;

use actix::prelude::*;
use actix_telegram::{methods::GetMe, App, TelegramBot};
use chrono::Duration;
use dotenv::dotenv;
use futures::future::Future;
use std::env;

struct Messages {
    pub button_text: String,
    pub button_not_for_you: String,
    pub challenge_message: String,
    pub challenge_timeout: Duration,
    pub success_message: String,
    pub fail_message: String,
    pub print_success: bool,
    pub print_fail: bool,
    pub keep_join_msg_on_fail: bool,
}

impl Messages {
    pub fn from_env() -> Self {
        Messages {
            button_text: env::var("BTN_TEXT").unwrap_or("I am not a spammer!".into()),
            button_not_for_you: env::var("BTN_NOT_FOR_YOU").unwrap_or("This button is not for you.".into()),
            challenge_message: env::var("CHALLENGE_MESSAGE").unwrap_or("This is spam protection. You have {} seconds to press the button or you will be banned!".into()),
            challenge_timeout: Duration::seconds(env::var("CHALLENGE_TIMEOUT_SEC").unwrap_or("30".into()).parse::<i64>().expect("Couldn't parse CHALLENGE_TIMEOUT_SEC")),
            success_message: env::var("SUCCESS_MESSAGE").unwrap_or("Welcome!".into()),
            fail_message: env::var("FAIL_MESSAGE").unwrap_or("User didn't pass the validation and was banned.".into()),
            print_success: env::var("PRINT_SUCCESS").unwrap_or("true".into()).parse::<bool>().expect("Couldn't parse PRINT_SUCCESS"),
            print_fail: env::var("PRINT_FAIL").unwrap_or("false".into()).parse::<bool>().expect("Couldn't parse PRINT_FAIL"),
            keep_join_msg_on_fail: env::var("KEEP_JOIN_MSG_ON_FAIL").unwrap_or("false".into()).parse::<bool>().expect("Couldn't parse KEEP_JOIN_MSG_ON_FAIL"),
        }
    }
}

fn main() {
    dotenv().ok();
    setup_logging().expect("failed to initialize logging");

    let config = Messages::from_env();
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN must be set");
    let timeout = 30_u16;
    let sys = System::new("runner");
    let challenge_app = App::new(|msg, _telegram_api| {
        info!("{:?}", msg);
        Ok(())
    });

    let _bot = TelegramBot::new(token, timeout, vec![challenge_app]).start();
    sys.run();
}

fn setup_logging() -> Result<(), fern::InitError> {
    use fern::colors::{Color, ColoredLevelConfig};

    // Color setup from fern examples
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::White)
        .trace(Color::BrightBlack);
    let colors_level = colors_line.info(Color::Green);

    let console_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}{date}[{target}][{level}{color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ))
        }).level(log::LevelFilter::Info)
        .chain(std::io::stdout());

    let file_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        }).level(log::LevelFilter::Trace)
        .chain(
            std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(false) // don't overwrite log file each run
                .open(format!(
                    "logs/bot-{}.log",
                    chrono::Local::now().format("%Y%m%d-%H%M%S")
                ))?,
        );

    fern::Dispatch::new()
        .chain(console_config)
        .chain(file_config)
        .apply()?;

    Ok(())
}
