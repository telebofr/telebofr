use serde::{Deserialize, Deserializer, Serialize};

use crate::types::{Chat, MaybeAnonymousUser, PollId, User};

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PollAnswer {
    /// Unique poll identifier.
    pub poll_id: Box<PollId>,

    /// If the voter is anonymous, stores the chat that changed the answer to
    /// the poll.
    ///
    /// If the voter isn't anonymous, stores the user that changed
    /// the answer to the poll
    #[serde(deserialize_with = "deserialize_voter", flatten)]
    pub voter: MaybeAnonymousUser,

    /// 0-based identifiers of answer options, chosen by the user.
    ///
    /// May be empty if the user retracted their vote.
    pub option_ids: Vec<u8>,
}

/// These fields `chat` and `user` from the original [`PollAnswer`] should be
/// exclusive, but in cases when the `voter_chat` is presented the `user` isn't
/// `None`, but rather actual value for backward compatibility, the field `user`
/// in such objects will contain the user 136817688 (@Channel_Bot).
#[derive(Deserialize)]
struct VoterDe {
    /// The chat that changed the answer to the poll, if the voter is anonymous
    pub voter_chat: Option<Chat>,

    /// The user that changed the answer to the poll, if the voter isn't
    /// anonymous
    pub user: Option<User>,
}

fn deserialize_voter<'d, D: Deserializer<'d>>(d: D) -> Result<MaybeAnonymousUser, D::Error> {
    let VoterDe { voter_chat, user } = VoterDe::deserialize(d)?;
    Ok(voter_chat.map(MaybeAnonymousUser::Chat).or(user.map(MaybeAnonymousUser::User)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poll_answer_with_user_de() {
        let json = r#"{
            "poll_id": "POLL_ID",
            "user": {"id": 42,"is_bot": false,"first_name": "blah"},
            "option_ids": []
        }"#;

        let poll_answer: PollAnswer = serde_json::from_str(json).unwrap();

        assert!(poll_answer.voter.is_user());
    }

    #[test]
    fn poll_answer_with_voter_chat_de() {
        let json = r#"{
            "poll_id": "POLL_ID",
            "voter_chat": {
                "id": -1001160242915,
                "title": "a",
                "type": "group"
            },
            "option_ids": []
        }"#;

        let poll_answer: PollAnswer = serde_json::from_str(json).unwrap();
        assert!(poll_answer.voter.is_chat());
    }

    #[test]
    fn poll_answer_with_both_user_and_voter_chat_de() {
        let json = r#"{
            "poll_id":"POLL_ID",
            "voter_chat": {
                "id": -1001160242915,
                "title": "a",
                "type": "group"
            },
            "user": {"id": 136817688,"is_bot": true,"first_name": "Channel_Bot"},
            "option_ids": []
        }"#;

        let poll_answer: PollAnswer = serde_json::from_str(json).unwrap();
        assert!(poll_answer.voter.is_chat());
    }
}
