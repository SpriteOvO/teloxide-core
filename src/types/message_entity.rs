use serde::{Deserialize, Serialize};

use crate::types::{User, UserId};

/// This object represents one special entity in a text message.
///
/// For example, hashtags, usernames, URLs, etc.
///
/// [The official docs](https://core.telegram.org/bots/api#messageentity).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct MessageEntity {
    #[serde(flatten)]
    pub kind: MessageEntityKind,

    /// Offset in UTF-16 code units to the start of the entity.
    pub offset: usize,

    /// Length of the entity in UTF-16 code units.
    pub length: usize,
}

impl MessageEntity {
    pub const fn new(kind: MessageEntityKind, offset: usize, length: usize) -> Self {
        Self {
            kind,
            offset,
            length,
        }
    }

    /// Create a message entity representing a text link in the form of
    /// `tg://user/?id=...` that mentions user with `user_id`.
    pub fn user_mention(user_id: UserId, offset: usize, length: usize) -> Self {
        Self {
            kind: MessageEntityKind::TextLink { url: user_id.url() },
            offset,
            length,
        }
    }

    pub fn kind(mut self, val: MessageEntityKind) -> Self {
        self.kind = val;
        self
    }

    pub const fn offset(mut self, val: usize) -> Self {
        self.offset = val;
        self
    }

    pub const fn length(mut self, val: usize) -> Self {
        self.length = val;
        self
    }
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum MessageEntityKind {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Code,
    Pre { language: Option<String> },
    TextLink { url: reqwest::Url },
    TextMention { user: User },
    Underline,
    Strikethrough,
    Spoiler,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursive_kind() {
        use serde_json::from_str;

        assert_eq!(
            MessageEntity {
                kind: MessageEntityKind::TextLink {
                    url: reqwest::Url::parse("https://example.com").unwrap(),
                },
                offset: 1,
                length: 2,
            },
            from_str::<MessageEntity>(
                r#"{"type":"text_link","url":"https://example.com","offset":1,"length":2}"#
            )
            .unwrap()
        );
    }

    #[test]
    fn pre() {
        use serde_json::from_str;

        assert_eq!(
            MessageEntity {
                kind: MessageEntityKind::Pre {
                    language: Some("rust".to_string())
                },
                offset: 1,
                length: 2,
            },
            from_str::<MessageEntity>(r#"{"type":"pre","offset":1,"length":2,"language":"rust"}"#)
                .unwrap()
        );
    }

    // https://github.com/teloxide/teloxide-core/pull/145
    #[test]
    fn pre_with_none_language() {
        use serde_json::to_string;

        assert_eq!(
            to_string(&MessageEntity {
                kind: MessageEntityKind::Pre { language: None },
                offset: 1,
                length: 2,
            })
            .unwrap()
            .find("language"),
            None
        );
    }
}
