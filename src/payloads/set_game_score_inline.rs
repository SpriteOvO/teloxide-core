// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::{Message, UserId};

impl_payload! {
    /// Use this method to set the score of the specified user in a game. On success, returns _True_. Returns an error, if the new score is not greater than the user's current score in the chat and force is False.
    ///
    /// See also: [`SetGameScore`](crate::payloads::SetGameScore)
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetGameScoreInline (SetGameScoreInlineSetters) => Message {
        required {
            /// User identifier
            pub user_id: UserId,
            /// New score
            pub score: u64,
            /// Identifier of the inline message
            pub inline_message_id: String [into],
        }
        optional {
            /// Pass True, if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
            pub force: bool,
            /// Pass True, if the game message should not be automatically edited to include the current scoreboard
            pub disable_edit_message: bool,
        }
    }
}
