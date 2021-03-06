// This file is auto generated by `cg` <https://github.com/teloxide/cg> (24572cd + local changes).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, InputMedia, Message};

impl_payload! {
    /// Use this method to send a group of photos or videos as an album. On success, an array of the sent [`Message`]s is returned.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendMediaGroup (SendMediaGroupSetters) => Vec<Message> {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: ChatId [into],
            /// A JSON-serialized array describing photos and videos to be sent, must include 2-10 items
            pub media: Vec<InputMedia> [collect],
        }
        optional {
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// If the message is a reply, ID of the original message
            pub reply_to_message_id: i32,
        }
    }
}
