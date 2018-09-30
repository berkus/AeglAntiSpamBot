use teloxide::types::{Chat, User};

struct Challenge;

struct ChallengeProcess {
    user: User,
    chat: Chat, // rights to restore are extracted from this chat
    selected_challenge: Challenge,
}
