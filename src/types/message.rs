#![allow(clippy::large_enum_variant)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{
    Animation, Audio, BareChatId, Chat, ChatId, Contact, Dice, Document, Game,
    InlineKeyboardMarkup, Invoice, Location, MessageAutoDeleteTimerChanged, MessageEntity,
    PassportData, PhotoSize, Poll, ProximityAlertTriggered, Sticker, SuccessfulPayment, True, User,
    Venue, Video, VideoChatEnded, VideoChatParticipantsInvited, VideoChatScheduled,
    VideoChatStarted, VideoNote, Voice, WebAppData,
};

/// This object represents a message.
///
/// [The official docs](https://core.telegram.org/bots/api#message).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// Unique message identifier inside this chat.
    #[serde(rename = "message_id")]
    pub id: i32,

    /// Date the message was sent in Unix time.
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub date: DateTime<Utc>,

    /// Conversation the message belongs to.
    pub chat: Chat,

    /// Bot through which the message was sent.
    pub via_bot: Option<User>,

    #[serde(flatten)]
    pub kind: MessageKind,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageKind {
    Common(MessageCommon),
    NewChatMembers(MessageNewChatMembers),
    LeftChatMember(MessageLeftChatMember),
    NewChatTitle(MessageNewChatTitle),
    NewChatPhoto(MessageNewChatPhoto),
    DeleteChatPhoto(MessageDeleteChatPhoto),
    GroupChatCreated(MessageGroupChatCreated),
    SupergroupChatCreated(MessageSupergroupChatCreated),
    ChannelChatCreated(MessageChannelChatCreated),
    MessageAutoDeleteTimerChanged(MessageMessageAutoDeleteTimerChanged),
    Pinned(MessagePinned),
    Invoice(MessageInvoice),
    SuccessfulPayment(MessageSuccessfulPayment),
    ConnectedWebsite(MessageConnectedWebsite),
    PassportData(MessagePassportData),
    Dice(MessageDice),
    ProximityAlertTriggered(MessageProximityAlertTriggered),
    VoiceChatScheduled(MessageVoiceChatScheduled),
    VoiceChatStarted(MessageVoiceChatStarted),
    VoiceChatEnded(MessageVoiceChatEnded),
    VoiceChatParticipantsInvited(MessageVoiceChatParticipantsInvited),
    WebAppData(MessageWebAppData),
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageCommon {
    /// Sender, empty for messages sent to channels.
    pub from: Option<User>,

    /// Sender of the message, sent on behalf of a chat. The channel itself for
    /// channel messages. The supergroup itself for messages from anonymous
    /// group administrators. The linked channel for messages automatically
    /// forwarded to the discussion group
    pub sender_chat: Option<Chat>,

    /// Signature of the post author for messages in channels, or the custom
    /// title of an anonymous group administrator.
    pub author_signature: Option<String>,

    /// For forwarded messages, information about the forward
    #[serde(flatten)]
    pub forward: Option<Forward>,

    /// For replies, the original message. Note that the Message object in this
    /// field will not contain further `reply_to_message` fields even if it
    /// itself is a reply.
    pub reply_to_message: Option<Box<Message>>,

    /// Date the message was last edited in Unix time.
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    pub edit_date: Option<DateTime<Utc>>,

    #[serde(flatten)]
    pub media_kind: MediaKind,

    /// Inline keyboard attached to the message. `login_url` buttons are
    /// represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// `true`, if the message is a channel post that was automatically
    /// forwarded to the connected discussion group.
    #[serde(default)]
    pub is_automatic_forward: bool,

    /// `true`, if the message can't be forwarded.
    #[serde(default)]
    pub has_protected_content: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageNewChatMembers {
    /// New members that were added to the group or supergroup and
    /// information about them (the bot itself may be one of these
    /// members).
    pub new_chat_members: Vec<User>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageLeftChatMember {
    /// A member was removed from the group, information about them (this
    /// member may be the bot itself).
    pub left_chat_member: User,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageNewChatTitle {
    /// A chat title was changed to this value.
    pub new_chat_title: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageNewChatPhoto {
    /// A chat photo was change to this value.
    pub new_chat_photo: Vec<PhotoSize>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageDeleteChatPhoto {
    /// Service message: the chat photo was deleted.
    pub delete_chat_photo: True,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageGroupChatCreated {
    /// Service message: the group has been created.
    pub group_chat_created: True,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageSupergroupChatCreated {
    /// Service message: the supergroup has been created. This field can‘t
    /// be received in a message coming through updates, because bot can’t
    /// be a member of a supergroup when it is created. It can only be
    /// found in `reply_to_message` if someone replies to a very first
    /// message in a directly created supergroup.
    pub supergroup_chat_created: True,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MessageChannelChatCreated {
    /// Service message: the channel has been created. This field can‘t be
    /// received in a message coming through updates, because bot can’t be
    /// a member of a channel when it is created. It can only be found in
    /// `reply_to_message` if someone replies to a very first message in a
    /// channel.
    pub channel_chat_created: True,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageMessageAutoDeleteTimerChanged {
    /// Service message: auto-delete timer settings changed in the chat.
    pub message_auto_delete_timer_changed: MessageAutoDeleteTimerChanged,
}

/// Represents group migration to a supergroup or a supergroup migration from a
/// group.
///
/// Note that bot receives **both** updates. For example: a group with id `0`
/// migrates to a supergroup with id `1` bots in that group will receive 2
/// updates:
/// - `message.chat.id = 0`, `message.chat_migration() = ChatMigration::To {
///   chat_id: 1 }`
/// - `message.chat.id = 1`, `message.chat_migration() = ChatMigration::From {
///   chat_id: 0 }`
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatMigration {
    /// The group has been migrated to a supergroup with the specified
    /// identifier `chat_id`.
    To {
        #[serde(rename = "migrate_to_chat_id")]
        chat_id: ChatId,
    },

    /// The supergroup has been migrated from a group with the specified
    /// identifier `chat_id`.
    From {
        #[serde(rename = "migrate_from_chat_id")]
        chat_id: ChatId,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessagePinned {
    /// Specified message was pinned. Note that the Message object in this
    /// field will not contain further `reply_to_message` fields even if it
    /// is itself a reply.
    #[serde(rename = "pinned_message")]
    pub pinned: Box<Message>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageInvoice {
    /// Message is an invoice for a [payment], information about the
    /// invoice. [More about payments »].
    ///
    /// [payment]: https://core.telegram.org/bots/api#payments
    /// [More about payments »]: https://core.telegram.org/bots/api#payments
    pub invoice: Invoice,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageSuccessfulPayment {
    /// Message is a service message about a successful payment,
    /// information about the payment. [More about payments »].
    ///
    /// [More about payments »]: https://core.telegram.org/bots/api#payments
    pub successful_payment: SuccessfulPayment,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageConnectedWebsite {
    /// The domain name of the website on which the user has logged in.
    /// [More about Telegram Login »].
    ///
    /// [More about Telegram Login »]: https://core.telegram.org/widgets/login
    pub connected_website: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessagePassportData {
    /// Telegram Passport data.
    pub passport_data: PassportData,
}

/// Information about forwarded message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Forward {
    /// Date the original message was sent in Unix time.
    #[serde(rename = "forward_date")]
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub date: DateTime<Utc>,

    /// The entity that sent the original message.
    #[serde(flatten)]
    pub from: ForwardedFrom,

    /// For messages forwarded from channels, signature of the post author if
    /// present. For messages forwarded from anonymous admins, authors title, if
    /// present.
    #[serde(rename = "forward_signature")]
    pub signature: Option<String>,

    /// For messages forwarded from channels, identifier of the original message
    /// in the channel
    #[serde(rename = "forward_from_message_id")]
    pub message_id: Option<i32>,
}

/// The entity that sent the original message that later was forwarded.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ForwardedFrom {
    /// The message was sent by a user.
    #[serde(rename = "forward_from")]
    User(User),
    /// The message was sent by an anonymous user on behalf of a group or
    /// channel.
    #[serde(rename = "forward_from_chat")]
    Chat(Chat),
    /// The message was sent by a user who disallow adding a link to their
    /// account in forwarded messages.
    #[serde(rename = "forward_sender_name")]
    SenderName(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MediaKind {
    // Note:
    // - `Venue` must be in front of `Location`
    // - `Animation` must be in front of `Document`
    //
    // This is needed so serde doesn't parse `Venue` as `Location` or `Animation` as `Document`
    // (for backward compatability telegram duplicates some fields)
    //
    // See <https://github.com/teloxide/teloxide/issues/481>
    Animation(MediaAnimation),
    Audio(MediaAudio),
    Contact(MediaContact),
    Document(MediaDocument),
    Game(MediaGame),
    Venue(MediaVenue),
    Location(MediaLocation),
    Photo(MediaPhoto),
    Poll(MediaPoll),
    Sticker(MediaSticker),
    Text(MediaText),
    Video(MediaVideo),
    VideoNote(MediaVideoNote),
    Voice(MediaVoice),
    Migration(ChatMigration),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaAnimation {
    /// Message is an animation, information about the animation. For
    /// backward compatibility, when this field is set, the document field
    /// will also be set.
    pub animation: Animation,

    /// Caption for the animation, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default = "Vec::new")]
    pub caption_entities: Vec<MessageEntity>,
    // Note: for backward compatibility telegram also sends `document` field, but we ignore it
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaAudio {
    /// Message is an audio file, information about the file.
    pub audio: Audio,

    /// Caption for the audio, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default = "Vec::new")]
    pub caption_entities: Vec<MessageEntity>,

    /// The unique identifier of a media message group this message belongs
    /// to.
    pub media_group_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaContact {
    /// Message is a shared contact, information about the contact.
    pub contact: Contact,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaDocument {
    /// Message is a general file, information about the file.
    pub document: Document,

    /// Caption for the document, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default)]
    pub caption_entities: Vec<MessageEntity>,

    /// The unique identifier of a media message group this message belongs
    /// to.
    pub media_group_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaGame {
    /// Message is a game, information about the game. [More
    /// about games »].
    ///
    /// [More about games »]: https://core.telegram.org/bots/api#games
    pub game: Game,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaLocation {
    /// Message is a shared location, information about the location.
    pub location: Location,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaPhoto {
    /// Message is a photo, available sizes of the photo.
    pub photo: Vec<PhotoSize>,

    /// Caption for the photo, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default = "Vec::new")]
    pub caption_entities: Vec<MessageEntity>,

    /// The unique identifier of a media message group this message belongs
    /// to.
    pub media_group_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaPoll {
    /// Message is a native poll, information about the poll.
    pub poll: Poll,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaSticker {
    /// Message is a sticker, information about the sticker.
    pub sticker: Sticker,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaText {
    /// For text messages, the actual UTF-8 text of the message, 0-4096
    /// characters.
    pub text: String,

    /// For text messages, special entities like usernames, URLs, bot
    /// commands, etc. that appear in the text.
    #[serde(default = "Vec::new")]
    pub entities: Vec<MessageEntity>,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaVideo {
    /// Message is a video, information about the video.
    pub video: Video,

    /// Caption for the video, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default = "Vec::new")]
    pub caption_entities: Vec<MessageEntity>,

    /// The unique identifier of a media message group this message belongs
    /// to.
    pub media_group_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaVideoNote {
    /// Message is a [video note], information about the video message.
    ///
    /// [video note]: https://telegram.org/blog/video-messages-and-telescope
    pub video_note: VideoNote,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaVoice {
    /// Message is a voice message, information about the file.
    pub voice: Voice,

    /// Caption for the voice, 0-1024 characters.
    pub caption: Option<String>,

    /// For messages with a caption, special entities like usernames, URLs,
    /// bot commands, etc. that appear in the caption.
    #[serde(default = "Vec::new")]
    pub caption_entities: Vec<MessageEntity>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaVenue {
    /// Message is a venue, information about the venue.
    pub venue: Venue,
    // Note: for backward compatibility telegram also sends `location` field, but we ignore it
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDice {
    /// Message is a dice with random value from 1 to 6.
    pub dice: Dice,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageProximityAlertTriggered {
    /// Service message. A user in the chat triggered another user's proximity
    /// alert while sharing Live Location.
    pub proximity_alert_triggered: ProximityAlertTriggered,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageVoiceChatScheduled {
    /// Service message: voice chat scheduled
    pub video_chat_scheduled: VideoChatScheduled,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageVoiceChatStarted {
    /// Service message: voice chat started.
    pub video_chat_started: VideoChatStarted,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageVoiceChatEnded {
    /// Service message: voice chat ended.
    pub video_chat_ended: VideoChatEnded,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageVoiceChatParticipantsInvited {
    /// Service message: new participants invited to a voice chat.
    pub video_chat_participants_invited: VideoChatParticipantsInvited,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageWebAppData {
    /// Service message: data sent by a Web App.
    pub web_app_data: WebAppData,
}

mod getters {
    use chrono::{DateTime, Utc};
    use std::ops::Deref;

    use crate::types::{
        self, message::MessageKind::*, Chat, ChatId, ChatMigration, Forward, ForwardedFrom,
        MediaAnimation, MediaAudio, MediaContact, MediaDocument, MediaGame, MediaKind,
        MediaLocation, MediaPhoto, MediaPoll, MediaSticker, MediaText, MediaVenue, MediaVideo,
        MediaVideoNote, MediaVoice, Message, MessageChannelChatCreated, MessageCommon,
        MessageConnectedWebsite, MessageDeleteChatPhoto, MessageDice, MessageEntity,
        MessageGroupChatCreated, MessageInvoice, MessageLeftChatMember, MessageNewChatMembers,
        MessageNewChatPhoto, MessageNewChatTitle, MessagePassportData, MessagePinned,
        MessageProximityAlertTriggered, MessageSuccessfulPayment, MessageSupergroupChatCreated,
        PhotoSize, True, User,
    };

    /// Getters for [Message] fields from [telegram docs].
    ///
    /// [Message]: crate::types::Message
    /// [telegram docs]: https://core.telegram.org/bots/api#message
    impl Message {
        pub fn from(&self) -> Option<&User> {
            match &self.kind {
                Common(MessageCommon { from, .. }) => from.as_ref(),
                _ => None,
            }
        }

        pub fn author_signature(&self) -> Option<&str> {
            match &self.kind {
                Common(MessageCommon {
                    author_signature, ..
                }) => author_signature.as_deref(),
                _ => None,
            }
        }

        pub fn sender_chat(&self) -> Option<&Chat> {
            match &self.kind {
                Common(MessageCommon { sender_chat, .. }) => sender_chat.as_ref(),
                _ => None,
            }
        }

        #[deprecated(since = "0.4.2", note = "use `.chat.id` field instead")]
        pub fn chat_id(&self) -> ChatId {
            self.chat.id
        }

        pub fn forward(&self) -> Option<&Forward> {
            self.common().and_then(|m| m.forward.as_ref())
        }

        pub fn forward_date(&self) -> Option<DateTime<Utc>> {
            self.forward().map(|f| f.date)
        }

        pub fn forward_from(&self) -> Option<&ForwardedFrom> {
            self.forward().map(|f| &f.from)
        }

        pub fn forward_from_user(&self) -> Option<&User> {
            self.forward_from().and_then(|from| match from {
                ForwardedFrom::User(user) => Some(user),
                _ => None,
            })
        }

        pub fn forward_from_chat(&self) -> Option<&Chat> {
            self.forward_from().and_then(|from| match from {
                ForwardedFrom::Chat(chat) => Some(chat),
                _ => None,
            })
        }

        pub fn forward_from_sender_name(&self) -> Option<&str> {
            self.forward_from().and_then(|from| match from {
                ForwardedFrom::SenderName(sender_name) => Some(&**sender_name),
                _ => None,
            })
        }

        pub fn forward_from_message_id(&self) -> Option<i32> {
            self.forward().and_then(|f| f.message_id)
        }

        pub fn forward_signature(&self) -> Option<&str> {
            self.forward().and_then(|f| f.signature.as_deref())
        }

        pub fn reply_to_message(&self) -> Option<&Message> {
            self.common().and_then(|m| m.reply_to_message.as_deref())
        }

        pub fn edit_date(&self) -> Option<&DateTime<Utc>> {
            match &self.kind {
                Common(MessageCommon { edit_date, .. }) => edit_date.as_ref(),
                _ => None,
            }
        }

        pub fn media_group_id(&self) -> Option<&str> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Video(MediaVideo { media_group_id, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Photo(MediaPhoto { media_group_id, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Document(MediaDocument { media_group_id, .. }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind: MediaKind::Audio(MediaAudio { media_group_id, .. }),
                    ..
                }) => media_group_id.as_ref().map(Deref::deref),
                _ => None,
            }
        }

        pub fn text(&self) -> Option<&str> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Text(MediaText { text, .. }),
                    ..
                }) => Some(text),
                _ => None,
            }
        }

        pub fn entities(&self) -> Option<&[MessageEntity]> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Text(MediaText { entities, .. }),
                    ..
                }) => Some(entities),
                _ => None,
            }
        }

        pub fn caption_entities(&self) -> Option<&[MessageEntity]> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind:
                        MediaKind::Animation(MediaAnimation {
                            caption_entities, ..
                        }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind:
                        MediaKind::Audio(MediaAudio {
                            caption_entities, ..
                        }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind:
                        MediaKind::Document(MediaDocument {
                            caption_entities, ..
                        }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind:
                        MediaKind::Photo(MediaPhoto {
                            caption_entities, ..
                        }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind:
                        MediaKind::Video(MediaVideo {
                            caption_entities, ..
                        }),
                    ..
                })
                | Common(MessageCommon {
                    media_kind:
                        MediaKind::Voice(MediaVoice {
                            caption_entities, ..
                        }),
                    ..
                }) => Some(caption_entities),
                _ => None,
            }
        }

        pub fn audio(&self) -> Option<&types::Audio> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Audio(MediaAudio { audio, .. }),
                    ..
                }) => Some(audio),
                _ => None,
            }
        }

        pub fn document(&self) -> Option<&types::Document> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Document(MediaDocument { document, .. }),
                    ..
                }) => Some(document),
                _ => None,
            }
        }

        pub fn animation(&self) -> Option<&types::Animation> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Animation(MediaAnimation { animation, .. }),
                    ..
                }) => Some(animation),
                _ => None,
            }
        }

        pub fn game(&self) -> Option<&types::Game> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Game(MediaGame { game, .. }),
                    ..
                }) => Some(game),
                _ => None,
            }
        }

        pub fn photo(&self) -> Option<&[PhotoSize]> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Photo(MediaPhoto { photo, .. }),
                    ..
                }) => Some(photo),
                _ => None,
            }
        }

        pub fn sticker(&self) -> Option<&types::Sticker> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Sticker(MediaSticker { sticker, .. }),
                    ..
                }) => Some(sticker),
                _ => None,
            }
        }

        pub fn video(&self) -> Option<&types::Video> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Video(MediaVideo { video, .. }),
                    ..
                }) => Some(video),
                _ => None,
            }
        }

        pub fn voice(&self) -> Option<&types::Voice> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Voice(MediaVoice { voice, .. }),
                    ..
                }) => Some(voice),
                _ => None,
            }
        }

        pub fn video_note(&self) -> Option<&types::VideoNote> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::VideoNote(MediaVideoNote { video_note, .. }),
                    ..
                }) => Some(video_note),
                _ => None,
            }
        }

        pub fn caption(&self) -> Option<&str> {
            match &self.kind {
                Common(MessageCommon { media_kind, .. }) => match media_kind {
                    MediaKind::Animation(MediaAnimation { caption, .. })
                    | MediaKind::Audio(MediaAudio { caption, .. })
                    | MediaKind::Document(MediaDocument { caption, .. })
                    | MediaKind::Photo(MediaPhoto { caption, .. })
                    | MediaKind::Video(MediaVideo { caption, .. })
                    | MediaKind::Voice(MediaVoice { caption, .. }) => {
                        caption.as_ref().map(Deref::deref)
                    }
                    _ => None,
                },
                _ => None,
            }
        }

        pub fn contact(&self) -> Option<&types::Contact> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Contact(MediaContact { contact, .. }),
                    ..
                }) => Some(contact),
                _ => None,
            }
        }

        pub fn location(&self) -> Option<&types::Location> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Location(MediaLocation { location, .. }),
                    ..
                }) => Some(location),
                _ => None,
            }
        }

        pub fn venue(&self) -> Option<&types::Venue> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Venue(MediaVenue { venue, .. }),
                    ..
                }) => Some(venue),
                _ => None,
            }
        }

        pub fn poll(&self) -> Option<&types::Poll> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Poll(MediaPoll { poll, .. }),
                    ..
                }) => Some(poll),
                _ => None,
            }
        }

        pub fn new_chat_members(&self) -> Option<&[User]> {
            match &self.kind {
                NewChatMembers(MessageNewChatMembers { new_chat_members }) => {
                    Some(new_chat_members.as_ref())
                }
                _ => None,
            }
        }

        pub fn left_chat_member(&self) -> Option<&User> {
            match &self.kind {
                LeftChatMember(MessageLeftChatMember { left_chat_member }) => {
                    Some(left_chat_member)
                }
                _ => None,
            }
        }

        pub fn new_chat_title(&self) -> Option<&str> {
            match &self.kind {
                NewChatTitle(MessageNewChatTitle { new_chat_title }) => Some(new_chat_title),
                _ => None,
            }
        }

        pub fn new_chat_photo(&self) -> Option<&[PhotoSize]> {
            match &self.kind {
                NewChatPhoto(MessageNewChatPhoto { new_chat_photo }) => Some(new_chat_photo),
                _ => None,
            }
        }

        // TODO: OK, `Option<True>` is weird, can we do something with it?
        //       mb smt like `is_delete_chat_photo(&self) -> bool`?
        pub fn delete_chat_photo(&self) -> Option<True> {
            match &self.kind {
                DeleteChatPhoto(MessageDeleteChatPhoto { delete_chat_photo }) => {
                    Some(*delete_chat_photo)
                }
                _ => None,
            }
        }

        pub fn group_chat_created(&self) -> Option<True> {
            match &self.kind {
                GroupChatCreated(MessageGroupChatCreated { group_chat_created }) => {
                    Some(*group_chat_created)
                }
                _ => None,
            }
        }

        pub fn super_group_chat_created(&self) -> Option<True> {
            match &self.kind {
                SupergroupChatCreated(MessageSupergroupChatCreated {
                    supergroup_chat_created,
                }) => Some(*supergroup_chat_created),
                _ => None,
            }
        }

        pub fn channel_chat_created(&self) -> Option<True> {
            match &self.kind {
                ChannelChatCreated(MessageChannelChatCreated {
                    channel_chat_created,
                }) => Some(*channel_chat_created),
                _ => None,
            }
        }

        pub fn chat_migration(&self) -> Option<ChatMigration> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Migration(chat_migration),
                    ..
                }) => Some(*chat_migration),
                _ => None,
            }
        }

        pub fn migrate_to_chat_id(&self) -> Option<ChatId> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Migration(ChatMigration::To { chat_id }),
                    ..
                }) => Some(*chat_id),
                _ => None,
            }
        }

        pub fn migrate_from_chat_id(&self) -> Option<ChatId> {
            match &self.kind {
                Common(MessageCommon {
                    media_kind: MediaKind::Migration(ChatMigration::From { chat_id }),
                    ..
                }) => Some(*chat_id),
                _ => None,
            }
        }

        pub fn pinned_message(&self) -> Option<&Message> {
            match &self.kind {
                Pinned(MessagePinned { pinned }) => Some(pinned),
                _ => None,
            }
        }

        pub fn invoice(&self) -> Option<&types::Invoice> {
            match &self.kind {
                Invoice(MessageInvoice { invoice }) => Some(invoice),
                _ => None,
            }
        }

        pub fn successful_payment(&self) -> Option<&types::SuccessfulPayment> {
            match &self.kind {
                SuccessfulPayment(MessageSuccessfulPayment { successful_payment }) => {
                    Some(successful_payment)
                }
                _ => None,
            }
        }

        pub fn connected_website(&self) -> Option<&str> {
            match &self.kind {
                ConnectedWebsite(MessageConnectedWebsite { connected_website }) => {
                    Some(connected_website)
                }
                _ => None,
            }
        }

        pub fn passport_data(&self) -> Option<&types::PassportData> {
            match &self.kind {
                PassportData(MessagePassportData { passport_data }) => Some(passport_data),
                _ => None,
            }
        }

        pub fn dice(&self) -> Option<&types::Dice> {
            match &self.kind {
                Dice(MessageDice { dice }) => Some(dice),
                _ => None,
            }
        }

        pub fn proximity_alert_triggered(&self) -> Option<&types::ProximityAlertTriggered> {
            match &self.kind {
                ProximityAlertTriggered(MessageProximityAlertTriggered {
                    proximity_alert_triggered,
                }) => Some(proximity_alert_triggered),
                _ => None,
            }
        }

        pub fn reply_markup(&self) -> Option<&types::InlineKeyboardMarkup> {
            match &self.kind {
                Common(MessageCommon { reply_markup, .. }) => reply_markup.as_ref(),
                _ => None,
            }
        }

        pub fn is_automatic_forward(&self) -> bool {
            match &self.kind {
                Common(MessageCommon {
                    is_automatic_forward,
                    ..
                }) => *is_automatic_forward,
                _ => false,
            }
        }

        pub fn has_protected_content(&self) -> bool {
            match &self.kind {
                Common(MessageCommon {
                    has_protected_content,
                    ..
                }) => *has_protected_content,
                _ => false,
            }
        }

        /// Common message (text, image, etc)
        fn common(&self) -> Option<&MessageCommon> {
            match &self.kind {
                Common(message) => Some(message),
                _ => None,
            }
        }
    }
}

impl Message {
    /// Produces a direct link to the message.
    ///
    /// Note that for private groups the link will only be accessible for group
    /// members.
    ///
    /// Returns `None` for private chats (i.e.: DMs) and private groups (not
    /// supergroups).
    pub fn url(&self) -> Option<reqwest::Url> {
        use BareChatId::*;

        // Note: `t.me` links use bare chat ids
        let chat_id = match self.chat.id.to_bare() {
            // For private chats (i.e.: DMs) we can't produce "normal" t.me link.
            //
            // There are "tg://openmessage?user_id={0}&message_id={1}" links, which are
            // supposed to open any chat, including private messages, but they
            // are only supported by some telegram clients (e.g. Plus Messenger,
            // Telegram for Android 4.9+).
            User(_) => return None,
            // Similarly to user chats, there is no way to create a link to a message in a normal,
            // private group.
            //
            // (public groups are always supergroup which are in turn channels).
            Group(_) => return None,
            Channel(id) => id,
        };

        let url = match self.chat.username() {
            // If it's public group (i.e. not DM, not private group), we can produce
            // "normal" t.me link (accessible to everyone).
            Some(username) => format!("https://t.me/{0}/{1}/", username, self.id),
            // For private supergroups and channels we produce "private" t.me/c links. These are
            // only accessible to the group members.
            None => format!("https://t.me/c/{0}/{1}/", chat_id, self.id),
        };

        // UNWRAP:
        //
        // The `url` produced by formatting is correct since username is
        // /[a-zA-Z0-9_]{5,32}/ and chat/message ids are integers.
        Some(reqwest::Url::parse(&url).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::from_str;

    use crate::types::*;

    #[test]
    fn de_media_forwarded() {
        let json = r#"{
          "message_id": 198283,
          "from": {
            "id": 250918540,
            "is_bot": false,
            "first_name": "Андрей",
            "last_name": "Власов",
            "username": "aka_dude",
            "language_code": "en"
          },
          "chat": {
            "id": 250918540,
            "first_name": "Андрей",
            "last_name": "Власов",
            "username": "aka_dude",
            "type": "private"
          },
          "date": 1567927221,
          "video": {
            "duration": 13,
            "width": 512,
            "height": 640,
            "mime_type": "video/mp4",
            "thumb": {
              "file_id": "AAQCAAOmBAACBf2oS53pByA-I4CWWCObDwAEAQAHbQADMWcAAhYE",
              "file_unique_id":"",
              "file_size": 10339,
              "width": 256,
              "height": 320
            },
            "file_id": "BAADAgADpgQAAgX9qEud6QcgPiOAlhYE",
            "file_unique_id":"",
            "file_size": 1381334
          }
        }"#;
        let message = from_str::<Message>(json);
        assert!(message.is_ok());
    }

    #[test]
    fn de_media_group_forwarded() {
        let json = r#"{
          "message_id": 198283,
          "from": {
            "id": 250918540,
            "is_bot": false,
            "first_name": "Андрей",
            "last_name": "Власов",
            "username": "aka_dude",
            "language_code": "en"
          },
          "chat": {
            "id": 250918540,
            "first_name": "Андрей",
            "last_name": "Власов",
            "username": "aka_dude",
            "type": "private"
          },
          "date": 1567927221,
          "media_group_id": "12543417770506682",
          "video": {
            "duration": 13,
            "width": 512,
            "height": 640,
            "mime_type": "video/mp4",
            "thumb": {
              "file_id": "AAQCAAOmBAACBf2oS53pByA-I4CWWCObDwAEAQAHbQADMWcAAhYE",
              "file_unique_id":"",
              "file_size": 10339,
              "width": 256,
              "height": 320
            },
            "file_id": "BAADAgADpgQAAgX9qEud6QcgPiOAlhYE",
            "file_unique_id":"",
            "file_size": 1381334
          }
        }"#;
        let message = from_str::<Message>(json);
        assert!(message.is_ok());
    }

    #[test]
    fn de_text() {
        let json = r#"{
          "message_id": 199785,
          "from": {
           "id": 250918540,
           "is_bot": false,
           "first_name": "Андрей",
           "last_name": "Власов",
           "username": "aka_dude",
           "language_code": "en"
          },
          "chat": {
           "id": 250918540,
           "first_name": "Андрей",
           "last_name": "Власов",
           "username": "aka_dude",
           "type": "private"
          },
          "date": 1568289890,
          "text": "Лол кек 😂"
         }"#;
        let message = from_str::<Message>(json);
        assert!(message.is_ok());
    }

    #[test]
    fn de_sticker() {
        let json = r#"{
          "message_id": 199787,
          "from": {
           "id": 250918540,
           "is_bot": false,
           "first_name": "Андрей",
           "last_name": "Власов",
           "username": "aka_dude",
           "language_code": "en"
          },
          "chat": {
           "id": 250918540,
           "first_name": "Андрей",
           "last_name": "Власов",
           "username": "aka_dude",
           "type": "private"
          },
          "date": 1568290188,
          "sticker": {
           "width": 512,
           "height": 512,
           "emoji": "😡",
           "set_name": "AdvenTimeAnim",
           "is_animated": true,
           "is_video": false,
           "thumb": {
            "file_id": "AAQCAAMjAAOw0PgMaabKAcaXKCBLubkPAAQBAAdtAAPGKwACFgQ",
            "file_unique_id":"",
            "file_size": 4118,
            "width": 128,
            "height": 128
           },
           "file_id": "CAADAgADIwADsND4DGmmygHGlyggFgQ",
           "file_unique_id":"",
           "file_size": 16639
          }
         }"#;
        from_str::<Message>(json).unwrap();
    }

    #[test]
    fn de_image() {
        let json = r#"{
          "message_id": 199791,
          "from": {
           "id": 250918540,
           "is_bot": false,
           "first_name": "Андрей",
           "last_name": "Власов",
           "username": "aka_dude",
           "language_code": "en"
          },
          "chat": {
           "id": 250918540,
           "first_name": "Андрей",
           "last_name": "Власов",
           "username": "aka_dude",
           "type": "private"
          },
          "date": 1568290622,
          "photo": [
           {
            "file_id": "AgADAgAD36sxG-PX0UvQSXIn9rccdw-ACA4ABAEAAwIAA20AAybcBAABFgQ",
            "file_unique_id":"",
            "file_size": 18188,
            "width": 320,
            "height": 239
           },
           {
            "file_id": "AgADAgAD36sxG-PX0UvQSXIn9rccdw-ACA4ABAEAAwIAA3gAAyfcBAABFgQ",
            "file_unique_id":"",
            "file_size": 62123,
            "width": 800,
            "height": 598
           },
           {
            "file_id": "AgADAgAD36sxG-PX0UvQSXIn9rccdw-ACA4ABAEAAwIAA3kAAyTcBAABFgQ",
            "file_unique_id":"",
            "file_size": 75245,
            "width": 962,
            "height": 719
           }
          ]
         }"#;
        let message = from_str::<Message>(json);
        assert!(message.is_ok());
    }

    /// Regression test for <https://github.com/teloxide/teloxide/issues/419>
    #[test]
    fn issue_419() {
        let json = r#"{
            "message_id": 1,
            "from": {
                "id": 1087968824,
                "is_bot": true,
                "first_name": "Group",
                "username": "GroupAnonymousBot"
            },
            "author_signature": "TITLE2",
            "sender_chat": {
                "id": -1001160242915,
                "title": "a",
                "type": "supergroup"
            },
            "chat": {
                "id": -1001160242915,
                "title": "a",
                "type": "supergroup"
            },
            "date": 1640359576,
            "forward_from_chat": {
                "id": -1001160242915,
                "title": "a",
                "type": "supergroup"
            },
            "forward_signature": "TITLE",
            "forward_date": 1640359544,
            "text": "text"
        }"#;

        // Anonymous admin with title "TITLE2" forwards a message from anonymous
        // admin with title "TITLE" with text "a", everything is happening in
        // the same group.
        let message: Message = serde_json::from_str(json).unwrap();

        let group = Chat {
            id: ChatId(-1001160242915),
            kind: ChatKind::Public(ChatPublic {
                title: Some("a".to_owned()),
                kind: PublicChatKind::Supergroup(PublicChatSupergroup {
                    username: None,
                    sticker_set_name: None,
                    can_set_sticker_set: None,
                    permissions: None,
                    slow_mode_delay: None,
                    linked_chat_id: None,
                    location: None,
                }),
                description: None,
                invite_link: None,
                has_protected_content: None,
            }),
            message_auto_delete_time: None,
            photo: None,
            pinned_message: None,
        };

        assert!(message.from().unwrap().is_anonymous());
        assert_eq!(message.author_signature().unwrap(), "TITLE2");
        assert_eq!(message.sender_chat().unwrap(), &group);
        assert_eq!(&message.chat, &group);
        assert_eq!(message.forward_from_chat().unwrap(), &group);
        assert_eq!(message.forward_signature().unwrap(), "TITLE");
        assert!(message.forward_date().is_some());
        assert_eq!(message.text().unwrap(), "text");
    }

    /// Regression test for <https://github.com/teloxide/teloxide/issues/427>
    #[test]
    fn issue_427() {
        let old = ChatId(-599075523);
        let new = ChatId(-1001555296434);

        // Migration to a supergroup
        let json = r#"{"chat":{"all_members_are_administrators":false,"id":-599075523,"title":"test","type":"group"},"date":1629404938,"from":{"first_name":"nullptr","id":729497414,"is_bot":false,"language_code":"en","username":"hex0x0000"},"message_id":16,"migrate_to_chat_id":-1001555296434}"#;
        let message: Message = from_str(json).unwrap();

        assert_eq!(message.chat.id, old);
        assert_eq!(
            message.chat_migration(),
            Some(ChatMigration::To { chat_id: new })
        );
        assert_eq!(message.migrate_to_chat_id(), Some(new));

        // The user who initialized the migration
        assert!(message.from().is_some());

        // Migration from a common group
        let json = r#"{"chat":{"id":-1001555296434,"title":"test","type":"supergroup"},"date":1629404938,"from":{"first_name":"Group","id":1087968824,"is_bot":true,"username":"GroupAnonymousBot"},"message_id":1,"migrate_from_chat_id":-599075523,"sender_chat":{"id":-1001555296434,"title":"test","type":"supergroup"}}"#;
        let message: Message = from_str(json).unwrap();

        assert_eq!(message.chat.id, new);
        assert_eq!(
            message.chat_migration(),
            Some(ChatMigration::From { chat_id: old })
        );
        assert_eq!(message.migrate_from_chat_id(), Some(old));

        // Anonymous bot
        assert!(message.from().is_some());

        // The chat to which the group migrated
        assert!(message.sender_chat().is_some());
    }

    /// Regression test for <https://github.com/teloxide/teloxide/issues/481>
    #[test]
    fn issue_481() {
        let json = r#"
{
  "message_id": 0,
  "date": 0,
  "location": {
   "latitude": 0.0,
   "longitude": 0.0
  },
  "chat": {
   "id": 0,
   "first_name": "f",
   "type": "private"
  },
  "venue": {
   "location": {
    "latitude": 0.0,
    "longitude": 0.0
   },
   "title": "Title",
   "address": "Address",
   "foursquare_id": "some_foursquare_id"
  }
 }
"#;
        let message: Message = from_str(json).unwrap();
        assert_eq!(
            message.venue().unwrap(),
            &Venue {
                location: Location {
                    longitude: 0.0,
                    latitude: 0.0,
                    horizontal_accuracy: None,
                    live_period: None,
                    heading: None,
                    proximity_alert_radius: None
                },
                title: "Title".to_owned(),
                address: "Address".to_owned(),
                foursquare_id: Some("some_foursquare_id".to_owned()),
                foursquare_type: None,
                google_place_id: None,
                google_place_type: None,
            }
        )
    }

    /// Regression test for <https://github.com/teloxide/teloxide/issues/475>
    #[test]
    fn issue_475() {
        let json = r#"{"message_id":198295,"from":{"id":1087968824,"is_bot":true,"first_name":"Group","username":"GroupAnonymousBot"},"sender_chat":{"id":-1001331354980,"title":"C++ Together 2.0","username":"cpptogether","type":"supergroup"},"chat":{"id":-1001331354980,"title":"C++ Together 2.0","username":"cpptogether","type":"supergroup"},"date":1638236631,"video_chat_started":{}}"#;

        let message: Message = serde_json::from_str(json).unwrap();

        assert!(matches!(message.kind, MessageKind::VoiceChatStarted { .. }));

        // FIXME(waffle): it seems like we are losing `sender_chat` in some
        // cases inclusing this
        // assert!(message.sender_chat().is_some());
    }
}
