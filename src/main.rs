use log::{debug, error, info};
use once_cell::sync::Lazy;
use regex::Regex;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{env, path::Path, process};

const WORDS_FILE_RAW: &str = include_str!(concat!(env!("OUT_DIR"), "/words_alpha.txt"));
const MINIMUM_MESSAGE_LENGTH: usize = 8;

static WORDS: Lazy<Vec<&str>> = Lazy::new(|| {
    debug!("Initializing words vec");
    WORDS_FILE_RAW.split_whitespace().collect()
});
static MONITORED_USER_IDS: Lazy<Vec<u64>> = Lazy::new(|| {
    env::var("MONITORED_USER_IDS")
        .expect("Could not load env var")
        .split(',')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
});
static OTHER_IGNORE_PATTERNS: Lazy<Vec<Regex>> = Lazy::new(|| {
    vec![
        Regex::new(r"^[hue]{5,}$").unwrap(),
        Regex::new(r"^[lo]{5,}$").unwrap(),
        Regex::new(r"^https?://").unwrap(),
        Regex::new(r"^re{5,}").unwrap(),
        Regex::new(r"^<:\w+:\d+>$").unwrap(),
    ]
});

fn is_incoherent(content: &str) -> bool {
    if content.contains(' ') {
        debug!("Message contains a space");
        return false;
    }
    if content.len() < MINIMUM_MESSAGE_LENGTH {
        debug!("Message is under {} chars long", MINIMUM_MESSAGE_LENGTH);
        return false;
    }
    if WORDS.contains(&content) {
        debug!("Message found in word bank");
        return false;
    }
    for pattern in OTHER_IGNORE_PATTERNS.iter() {
        if pattern.is_match(content) {
            debug!("Matches ignore pattern");
            return false;
        }
    }
    true
}

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, _: Ready) {
        info!("Bot connected");
    }

    async fn message(&self, context: Context, message: Message) {
        if !MONITORED_USER_IDS.contains(message.author.id.as_u64()) {
            return;
        }
        if !is_incoherent(&message.content) {
            return;
        }
        if let Err(e) = message.react(&context, '🤧').await {
            error!("Error adding reaction: {}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    if Path::new(".env").exists() {
        dotenv::dotenv().expect("Could not load from .env file");
    }
    pretty_env_logger::init();
    let token = match env::var("DISCORD_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            error!("Environment variable 'DISCORD_TOKEN' is not set");
            process::exit(1);
        }
    };
    debug!("Token loaded from environment variable");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Could not create client");
    debug!("Bot set up");

    if let Err(e) = client.start().await {
        error!("Error starting client: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_incoherent_space() {
        assert!(!is_incoherent("a a"));
    }

    #[test]
    fn test_is_incoherent_length() {
        assert!(!is_incoherent("a"));
    }

    #[test]
    fn test_is_incoherent_real_word() {
        assert!(!is_incoherent("dictionary"));
    }

    #[test]
    fn test_is_incoherent_patterns() {
        assert!(!is_incoherent("lolololloo"));
        assert!(!is_incoherent("hueuhueuhuhe"));
        assert!(!is_incoherent("http://example.com"));
        assert!(!is_incoherent("https://google.com"));
        assert!(!is_incoherent("reeeeeeeeee"));
        assert!(!is_incoherent("<:Screampackman2:754148436906999888>"));
    }
}
