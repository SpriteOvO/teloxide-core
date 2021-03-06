// This file is auto generated by `cg` <https://github.com/teloxide/cg> (24572cd + local changes).
// **DO NOT EDIT THIS FILE**,
// edit `cg` instead.
use serde::Serialize;

use crate::types::BotCommand;

impl_payload! {
    /// Use this method to change the list of the bot's commands. Returns _True_ on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetMyCommands (SetMyCommandsSetters) => u32 {
        required {
            /// A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.
            pub commands: Vec<BotCommand> [collect],
        }
    }
}
