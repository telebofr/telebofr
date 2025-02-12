use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{Chat, ChatInviteLink, ChatMember, User};

/// This object represents changes in the status of a chat member.
///
/// [The official docs](https://core.telegram.org/bots/api#chatmemberupdated).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatMemberUpdated {
    /// Chat the user belongs to
    pub chat: Chat,

    /// Performer of the action, which resulted in the change
    pub from: User,

    /// Date the change was done
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub date: DateTime<Utc>,

    /// Previous information about the chat member
    pub old_chat_member: ChatMember,

    /// New information about the chat member
    pub new_chat_member: ChatMember,

    /// Chat invite link, which was used by the user to join the chat; for
    /// joining by invite link events only.
    pub invite_link: Option<ChatInviteLink>,

    /// `true`, if the user joined the chat after sending a direct join request
    /// without using an invite link and being approved by an administrator
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub via_join_request: bool,

    #[serde(default)]
    /// True, if the user joined the chat via a chat folder invite link
    pub via_chat_folder_invite_link: bool,
}

impl ChatMemberUpdated {
    /// Returns all users that are "contained" in this `ChatMemberUpdated`
    /// structure.
    ///
    /// This might be useful to track information about users.
    ///
    /// Note that this function can return duplicate users.
    pub fn mentioned_users(&self) -> impl Iterator<Item = &User> {
        [
            &self.from,
            /* ignore `old_chat_member.user`, it should always be the same as the new one */
            &self.new_chat_member.user,
        ]
        .into_iter()
    }
}
