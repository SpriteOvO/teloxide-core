use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object represents a service message about new members invited to a
/// voice chat.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct VideoChatParticipantsInvited {
    /// New members that were invited to the voice chat
    pub users: Option<Vec<User>>,
}
