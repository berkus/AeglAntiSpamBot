// 1. New user joins
//    - look for Message.kind == NewChatMembers
//    - will give potentially a LIST of new members, need to process all
// 2. Strip user of posting rights
// 3. Print CHALLENGE_MESSAGE
// 4. If correct button not pressed withing CHALLENGE_TIMEOUT_SEC, ban user.
// 4a. If KEEP_JOIN_MSG_ON_FAIL is not allowed, delete messages.
// 4b. If PRINT_FAIL is allowed, post failure message.
// 5. Otherwise, restore posting rights (Restore to the group's default!)
// 5a. If allowed, print WELCOME message.

use {
    dotenv::dotenv,
    teloxide::{prelude::*, Bot},
    tokio_stream::wrappers::UnboundedReceiverStream,
};

mod actor_messages;
mod bot_messages;
mod challenge;
mod dispatcher;
mod interrogator;
mod timer;

#[tokio::main]
async fn main() {
    dotenv().ok();
    setup_logging().expect("failed to initialize logging");

    run().await;
}

async fn run() {
    log::info!("Starting bot...");

    let timeout = 30_u16;
    let messages = bot_messages::Messages::from_env();

    let bot = Bot::from_env().auto_send();

    Dispatcher::new(bot)
        .messages_handler(move |rx: DispatcherHandlerRx<AutoSend<Bot>, Message>| {
            UnboundedReceiverStream::new(rx)
                .for_each_concurrent(None, move |message| handle_message(message))
        })
        .chat_members_handler(
            move |rx: DispatcherHandlerRx<AutoSend<Bot>, ChatMemberUpdated>| {
                UnboundedReceiverStream::new(rx)
                    .for_each_concurrent(None, move |message| handle_chat_members(message))
            },
        )
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}

async fn handle_message(cx: UpdateWithCx<AutoSend<Bot>, Message>) {
    log::info!("Handling regular message");
    // match cx.update.text().map(ToOwned::to_owned) {
    // None => {
    //     cx.answer("Send me a text message.").await?;
    //     next(dialogue)
    // }
    // Some(ans) => dialogue.react(cx, ans).await,
    // }
}

async fn handle_chat_members(cx: UpdateWithCx<AutoSend<Bot>, ChatMemberUpdated>) {
    log::info!("Handling chat member update");
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
        })
        .level(log::LevelFilter::Info)
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
        })
        .level(log::LevelFilter::Trace)
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
