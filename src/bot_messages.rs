use {chrono::Duration, std::env};

pub struct Messages {
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
