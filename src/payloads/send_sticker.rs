// This file is auto generated by `cg` <https://github.com/teloxide/cg> (8ee7ef2).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::{ChatId, InputFile, Message, ReplyMarkup};

impl_payload! {
    @[multipart]
    /// Use this method to send static .WEBP or [animated] .TGS stickers. On success, the sent Message is returned.
    ///
    /// [animated]: https://telegram.org/blog/animated-stickers
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SendSticker (SendStickerSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`).
            pub chat_id: ChatId [into],
            /// Sticker to send. Pass a file_id as String to send a photo that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet, or upload a new photo using multipart/form-data. [More info on Sending Files »]
            ///
            /// [More info on Sending Files »]: crate::types::InputFile
            pub sticker: InputFile,
        }
        optional {
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// If the message is a reply, ID of the original message
            pub reply_to_message_id: i32,
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove reply keyboard or to force a reply from the user.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            pub reply_markup: ReplyMarkup [into],
        }
    }
}