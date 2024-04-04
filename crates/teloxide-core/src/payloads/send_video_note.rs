//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{InputFile, Message, MessageId, Recipient, ReplyMarkup, ThreadId};

impl_payload! {
    @[multipart = video_note, thumbnail]
    /// As of [v.4.0], Telegram clients support rounded square mp4 videos of up to 1 minute long. Use this method to send video messages. On success, the sent [`Message`] is returned.
    ///
    /// [v.4.0]: https://core.telegram.org/bots/api#document
    /// [`Message`]: crate::types::Message
    #[derive(Debug, Clone, Serialize)]
    pub SendVideoNote (SendVideoNoteSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// Video note to send. Pass a file_id as String to send a video note that exists on the Telegram servers (recommended) or upload a new video using multipart/form-data. [More info on Sending Files »]. Sending video notes by a URL is currently unsupported
            ///
            /// [More info on Sending Files »]: crate::types::InputFile
            pub video_note: InputFile,
        }
        optional {
            /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
            pub message_thread_id: ThreadId,
            /// Duration of the video in seconds
            pub duration: u32,
            /// Video width and height, i.e. diameter of the video message
            pub length: u32,
            /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. [More info on Sending Files »]
            ///
            /// [More info on Sending Files »]: crate::types::InputFile
            pub thumbnail: InputFile,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// Protects the contents of sent messages from forwarding and saving
            pub protect_content: bool,
            /// If the message is a reply, ID of the original message
            #[serde(serialize_with = "crate::types::serialize_reply_to_message_id")]
            pub reply_to_message_id: MessageId,
            /// Pass _True_, if the message should be sent even if the specified replied-to message is not found
            pub allow_sending_without_reply: bool,
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove reply keyboard or to force a reply from the user.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
