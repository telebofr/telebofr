#![allow(clippy::large_enum_variant)]

use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::types::{
    InlineQueryResultArticle, InlineQueryResultAudio, InlineQueryResultCachedAudio,
    InlineQueryResultCachedDocument, InlineQueryResultCachedGif, InlineQueryResultCachedMpeg4Gif,
    InlineQueryResultCachedPhoto, InlineQueryResultCachedSticker, InlineQueryResultCachedVideo,
    InlineQueryResultCachedVoice, InlineQueryResultContact, InlineQueryResultDocument,
    InlineQueryResultGame, InlineQueryResultGif, InlineQueryResultLocation,
    InlineQueryResultMpeg4Gif, InlineQueryResultPhoto, InlineQueryResultVenue,
    InlineQueryResultVideo, InlineQueryResultVoice,
};

/// This object represents one result of an inline query.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresult).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, From)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[serde(from = "raw::InlineQueryResult", into = "raw::InlineQueryResult")]
pub enum InlineQueryResult {
    #[serde(rename = "audio")]
    CachedAudio(InlineQueryResultCachedAudio),
    #[serde(rename = "document")]
    CachedDocument(InlineQueryResultCachedDocument),
    #[serde(rename = "gif")]
    CachedGif(InlineQueryResultCachedGif),
    #[serde(rename = "mpeg4_gif")]
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    #[serde(rename = "photo")]
    CachedPhoto(InlineQueryResultCachedPhoto),
    #[serde(rename = "sticker")]
    CachedSticker(InlineQueryResultCachedSticker),
    #[serde(rename = "video")]
    CachedVideo(InlineQueryResultCachedVideo),
    #[serde(rename = "voice")]
    CachedVoice(InlineQueryResultCachedVoice),

    Article(InlineQueryResultArticle),
    Audio(InlineQueryResultAudio),
    Contact(InlineQueryResultContact),
    Game(InlineQueryResultGame),
    Document(InlineQueryResultDocument),
    Gif(InlineQueryResultGif),
    Location(InlineQueryResultLocation),
    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    Photo(InlineQueryResultPhoto),
    Venue(InlineQueryResultVenue),
    Video(InlineQueryResultVideo),
    Voice(InlineQueryResultVoice),
}

mod raw {
    use super::*;

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum AudioKind {
        Cached(InlineQueryResultCachedAudio),
        NonCached(InlineQueryResultAudio),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum DocumentKind {
        Cached(InlineQueryResultCachedDocument),
        NonCached(InlineQueryResultDocument),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum GifKind {
        Cached(InlineQueryResultCachedGif),
        NonCached(InlineQueryResultGif),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum Mpeg4GifKind {
        Cached(InlineQueryResultCachedMpeg4Gif),
        NonCached(InlineQueryResultMpeg4Gif),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum PhotoKind {
        Cached(InlineQueryResultCachedPhoto),
        NonCached(InlineQueryResultPhoto),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum VideoKind {
        Cached(InlineQueryResultCachedVideo),
        NonCached(InlineQueryResultVideo),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum VoiceKind {
        Cached(InlineQueryResultCachedVoice),
        NonCached(InlineQueryResultVoice),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(tag = "type")]
    #[serde(rename_all = "snake_case")]
    pub(super) enum InlineQueryResult {
        // Types which have a cached and non-cached variant must be listed here
        Audio(AudioKind),
        Document(DocumentKind),
        Gif(GifKind),
        #[serde(rename = "mpeg4_gif")]
        Mpeg4Gif(Mpeg4GifKind),
        Photo(PhotoKind),
        Video(VideoKind),
        Voice(VoiceKind),

        // Types which have only a cached variant must be listed here
        #[serde(rename = "sticker")]
        CachedSticker(InlineQueryResultCachedSticker),

        // Types which have only a non-cached variant must be listed here
        Article(InlineQueryResultArticle),
        Contact(InlineQueryResultContact),
        Game(InlineQueryResultGame),
        Location(InlineQueryResultLocation),
        Venue(InlineQueryResultVenue),
    }

    impl From<InlineQueryResult> for super::InlineQueryResult {
        fn from(raw: InlineQueryResult) -> Self {
            match raw {
                InlineQueryResult::Audio(AudioKind::Cached(audio)) => {
                    super::InlineQueryResult::CachedAudio(audio)
                }
                InlineQueryResult::Audio(AudioKind::NonCached(audio)) => {
                    super::InlineQueryResult::Audio(audio)
                }
                InlineQueryResult::Document(DocumentKind::Cached(document)) => {
                    super::InlineQueryResult::CachedDocument(document)
                }
                InlineQueryResult::Document(DocumentKind::NonCached(document)) => {
                    super::InlineQueryResult::Document(document)
                }
                InlineQueryResult::Gif(GifKind::Cached(gif)) => {
                    super::InlineQueryResult::CachedGif(gif)
                }
                InlineQueryResult::Gif(GifKind::NonCached(gif)) => {
                    super::InlineQueryResult::Gif(gif)
                }
                InlineQueryResult::Mpeg4Gif(Mpeg4GifKind::Cached(gif)) => {
                    super::InlineQueryResult::CachedMpeg4Gif(gif)
                }
                InlineQueryResult::Mpeg4Gif(Mpeg4GifKind::NonCached(gif)) => {
                    super::InlineQueryResult::Mpeg4Gif(gif)
                }
                InlineQueryResult::Photo(PhotoKind::Cached(photo)) => {
                    super::InlineQueryResult::CachedPhoto(photo)
                }
                InlineQueryResult::Photo(PhotoKind::NonCached(photo)) => {
                    super::InlineQueryResult::Photo(photo)
                }
                InlineQueryResult::Video(VideoKind::Cached(video)) => {
                    super::InlineQueryResult::CachedVideo(video)
                }
                InlineQueryResult::Video(VideoKind::NonCached(video)) => {
                    super::InlineQueryResult::Video(video)
                }
                InlineQueryResult::Voice(VoiceKind::Cached(voice)) => {
                    super::InlineQueryResult::CachedVoice(voice)
                }
                InlineQueryResult::Voice(VoiceKind::NonCached(voice)) => {
                    super::InlineQueryResult::Voice(voice)
                }

                InlineQueryResult::CachedSticker(sticker) => {
                    super::InlineQueryResult::CachedSticker(sticker)
                }

                InlineQueryResult::Article(article) => super::InlineQueryResult::Article(article),
                InlineQueryResult::Contact(contact) => super::InlineQueryResult::Contact(contact),
                InlineQueryResult::Game(game) => super::InlineQueryResult::Game(game),
                InlineQueryResult::Location(location) => {
                    super::InlineQueryResult::Location(location)
                }
                InlineQueryResult::Venue(venue) => super::InlineQueryResult::Venue(venue),
            }
        }
    }

    impl From<super::InlineQueryResult> for InlineQueryResult {
        fn from(raw: super::InlineQueryResult) -> Self {
            match raw {
                super::InlineQueryResult::CachedAudio(audio) => {
                    InlineQueryResult::Audio(AudioKind::Cached(audio))
                }
                super::InlineQueryResult::Audio(audio) => {
                    InlineQueryResult::Audio(AudioKind::NonCached(audio))
                }
                super::InlineQueryResult::CachedDocument(document) => {
                    InlineQueryResult::Document(DocumentKind::Cached(document))
                }
                super::InlineQueryResult::Document(document) => {
                    InlineQueryResult::Document(DocumentKind::NonCached(document))
                }
                super::InlineQueryResult::CachedGif(gif) => {
                    InlineQueryResult::Gif(GifKind::Cached(gif))
                }
                super::InlineQueryResult::Gif(gif) => {
                    InlineQueryResult::Gif(GifKind::NonCached(gif))
                }
                super::InlineQueryResult::CachedMpeg4Gif(gif) => {
                    InlineQueryResult::Mpeg4Gif(Mpeg4GifKind::Cached(gif))
                }
                super::InlineQueryResult::Mpeg4Gif(gif) => {
                    InlineQueryResult::Mpeg4Gif(Mpeg4GifKind::NonCached(gif))
                }
                super::InlineQueryResult::CachedPhoto(photo) => {
                    InlineQueryResult::Photo(PhotoKind::Cached(photo))
                }
                super::InlineQueryResult::Photo(photo) => {
                    InlineQueryResult::Photo(PhotoKind::NonCached(photo))
                }
                super::InlineQueryResult::CachedVideo(video) => {
                    InlineQueryResult::Video(VideoKind::Cached(video))
                }
                super::InlineQueryResult::Video(video) => {
                    InlineQueryResult::Video(VideoKind::NonCached(video))
                }
                super::InlineQueryResult::CachedVoice(voice) => {
                    InlineQueryResult::Voice(VoiceKind::Cached(voice))
                }
                super::InlineQueryResult::Voice(voice) => {
                    InlineQueryResult::Voice(VoiceKind::NonCached(voice))
                }

                super::InlineQueryResult::CachedSticker(sticker) => {
                    InlineQueryResult::CachedSticker(sticker)
                }

                super::InlineQueryResult::Article(article) => InlineQueryResult::Article(article),
                super::InlineQueryResult::Contact(contact) => InlineQueryResult::Contact(contact),
                super::InlineQueryResult::Game(game) => InlineQueryResult::Game(game),
                super::InlineQueryResult::Location(location) => {
                    InlineQueryResult::Location(location)
                }
                super::InlineQueryResult::Venue(venue) => InlineQueryResult::Venue(venue),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        inline_keyboard_markup::InlineKeyboardMarkup, parse_mode::ParseMode, InlineQueryResult,
        InlineQueryResultArticle, InlineQueryResultAudio, InlineQueryResultCachedAudio,
        InlineQueryResultCachedDocument, InlineQueryResultCachedGif,
        InlineQueryResultCachedMpeg4Gif, InlineQueryResultCachedPhoto,
        InlineQueryResultCachedSticker, InlineQueryResultCachedVideo, InlineQueryResultCachedVoice,
        InlineQueryResultContact, InlineQueryResultDocument, InlineQueryResultGame,
        InlineQueryResultGif, InlineQueryResultLocation, InlineQueryResultMpeg4Gif,
        InlineQueryResultPhoto, InlineQueryResultVenue, InlineQueryResultVideo,
        InlineQueryResultVoice, InputMessageContent, InputMessageContentLocation,
        InputMessageContentText, LinkPreviewOptions, Seconds,
    };

    use mime::Mime;
    use std::str::FromStr as _;
    use url::Url;

    #[test]
    fn cached_audio_min() {
        let structure = InlineQueryResult::CachedAudio(InlineQueryResultCachedAudio {
            id: String::from("id"),
            audio_file_id: String::from("audio_file_id"),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json = r#"{"type":"audio","id":"id","audio_file_id":"audio_file_id"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn cached_audio_full() {
        let structure = InlineQueryResult::CachedAudio(InlineQueryResultCachedAudio {
            id: String::from("id"),
            audio_file_id: String::from("audio_file_id"),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                parse_mode: Some(ParseMode::MarkdownV2),
                entities: None,
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
            })),
            caption_entities: None,
        });

        let expected_json = r#"{"type":"audio","id":"id","audio_file_id":"audio_file_id","caption":"caption","parse_mode":"HTML","reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","parse_mode":"MarkdownV2","link_preview_options":{"is_disabled":true}}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn audio_min() {
        let structure = InlineQueryResult::Audio(InlineQueryResultAudio {
            id: String::from("id"),
            audio_url: reqwest::Url::parse("http://audio_url/").unwrap(),
            title: String::from("title"),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            performer: None,
            audio_duration: None,
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json =
            r#"{"type":"audio","id":"id","audio_url":"http://audio_url/","title":"title"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn audio_full() {
        let structure = InlineQueryResult::Audio(InlineQueryResultAudio {
            id: String::from("id"),
            audio_url: reqwest::Url::parse("http://audio_url/").unwrap(),
            title: String::from("title"),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                parse_mode: Some(ParseMode::MarkdownV2),
                entities: None,
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
            })),
            caption_entities: None,
            performer: Some(String::from("performer")),
            audio_duration: Some(Seconds::from_seconds(1)),
        });

        let expected_json = r#"{"type":"audio","id":"id","audio_url":"http://audio_url/","title":"title","caption":"caption","parse_mode":"HTML","performer":"performer","audio_duration":1,"reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","parse_mode":"MarkdownV2","link_preview_options":{"is_disabled":true}}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn cached_document_min() {
        let structure = InlineQueryResult::CachedDocument(InlineQueryResultCachedDocument {
            id: String::from("id"),
            title: String::from("title"),
            document_file_id: String::from("document_file_id"),
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json = r#"{"type":"document","id":"id","title":"title","document_file_id":"document_file_id"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn cached_document_full() {
        let structure = InlineQueryResult::CachedDocument(InlineQueryResultCachedDocument {
            id: String::from("id"),
            title: String::from("title"),
            document_file_id: String::from("document_file_id"),
            description: Some(String::from("description")),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                parse_mode: Some(ParseMode::MarkdownV2),
                entities: None,
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
            })),
            caption_entities: None,
        });

        let expected_json = r#"{"type":"document","id":"id","title":"title","document_file_id":"document_file_id","description":"description","caption":"caption","parse_mode":"HTML","reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","parse_mode":"MarkdownV2","link_preview_options":{"is_disabled":true}}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn document_min() {
        let structure = InlineQueryResult::Document(InlineQueryResultDocument {
            id: String::from("id"),
            title: String::from("title"),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            document_url: reqwest::Url::parse("http://document_url/").unwrap(),
            mime_type: Mime::from_str("application/pdf").unwrap(),
            description: None,
            reply_markup: None,
            input_message_content: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        });

        let expected_json = r#"{"type":"document","id":"id","title":"title","document_url":"http://document_url/","mime_type":"application/pdf"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn document_full() {
        let structure = InlineQueryResult::Document(InlineQueryResultDocument {
            id: String::from("id"),
            title: String::from("title"),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            caption_entities: None,
            document_url: reqwest::Url::parse("http://document_url/").unwrap(),
            mime_type: Mime::from_str("application/pdf").unwrap(),
            description: Some(String::from("description")),
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                parse_mode: Some(ParseMode::MarkdownV2),
                entities: None,
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
            })),
            thumbnail_url: Some(reqwest::Url::parse("http://thumb_url/").unwrap()),
            thumbnail_width: Some(1),
            thumbnail_height: Some(1),
        });

        let expected_json = r#"{"type":"document","id":"id","title":"title","caption":"caption","parse_mode":"HTML","document_url":"http://document_url/","mime_type":"application/pdf","description":"description","reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","parse_mode":"MarkdownV2","link_preview_options":{"is_disabled":true}},"thumbnail_url":"http://thumb_url/","thumbnail_width":1,"thumbnail_height":1}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn cached_gif_min() {
        let structure = InlineQueryResult::CachedGif(InlineQueryResultCachedGif {
            id: String::from("id"),
            gif_file_id: String::from("gif_file_id"),
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json = r#"{"type":"gif","id":"id","gif_file_id":"gif_file_id"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn cached_gif_full() {
        let structure = InlineQueryResult::CachedGif(InlineQueryResultCachedGif {
            id: String::from("id"),
            gif_file_id: String::from("gif_file_id"),
            title: Some(String::from("title")),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            caption_entities: None,
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                parse_mode: Some(ParseMode::MarkdownV2),
                entities: None,
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
            })),
        });

        let expected_json = r#"{"type":"gif","id":"id","gif_file_id":"gif_file_id","title":"title","caption":"caption","parse_mode":"HTML","reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","parse_mode":"MarkdownV2","link_preview_options":{"is_disabled":true}}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn gif_min() {
        let structure = InlineQueryResult::Gif(InlineQueryResultGif {
            id: String::from("id"),
            gif_url: Url::parse("http://gif_url/").unwrap(),
            gif_width: None,
            gif_height: None,
            gif_duration: None,
            thumbnail_url: Url::parse("http://thumb_url/").unwrap(),
            thumbnail_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json = r#"{"type":"gif","id":"id","gif_url":"http://gif_url/","thumbnail_url":"http://thumb_url/"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn gif_full() {
        let structure = InlineQueryResult::Gif(InlineQueryResultGif {
            id: String::from("id"),
            gif_url: Url::parse("http://gif_url/").unwrap(),
            gif_width: Some(1),
            gif_height: Some(1),
            gif_duration: Some(Seconds::from_seconds(1)),
            thumbnail_url: Url::parse("http://thumb_url/").unwrap(),
            thumbnail_mime_type: None,
            title: Some(String::from("title")),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            caption_entities: None,
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                parse_mode: Some(ParseMode::MarkdownV2),
                entities: None,
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
            })),
        });

        let expected_json = r#"{"type":"gif","id":"id","gif_url":"http://gif_url/","gif_width":1,"gif_height":1,"gif_duration":1,"thumbnail_url":"http://thumb_url/","title":"title","caption":"caption","parse_mode":"HTML","reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","parse_mode":"MarkdownV2","link_preview_options":{"is_disabled":true}}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn cached_mpeg4_gif_min() {
        let structure = InlineQueryResult::CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif {
            id: String::from("id"),
            mpeg4_file_id: String::from("mpeg4_file_id"),
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json = r#"{"type":"mpeg4_gif","id":"id","mpeg4_file_id":"mpeg4_file_id"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn cached_mpeg4_gif_full() {
        let structure = InlineQueryResult::CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif {
            id: String::from("id"),
            mpeg4_file_id: String::from("mpeg4_file_id"),
            title: Some(String::from("title")),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            caption_entities: None,
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                parse_mode: Some(ParseMode::MarkdownV2),
                entities: None,
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
            })),
        });

        let expected_json = r#"{"type":"mpeg4_gif","id":"id","mpeg4_file_id":"mpeg4_file_id","title":"title","caption":"caption","parse_mode":"HTML","reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","parse_mode":"MarkdownV2","link_preview_options":{"is_disabled":true}}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn mpeg4_gif_min() {
        let structure = InlineQueryResult::Mpeg4Gif(InlineQueryResultMpeg4Gif {
            id: String::from("id"),
            mpeg4_url: Url::parse("http://mpeg4_url/").unwrap(),
            mpeg4_width: None,
            mpeg4_height: None,
            mpeg4_duration: None,
            thumbnail_url: Url::parse("http://thumb_url/").unwrap(),
            thumbnail_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json = r#"{"type":"mpeg4_gif","id":"id","mpeg4_url":"http://mpeg4_url/","thumbnail_url":"http://thumb_url/"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn mpeg4_gif_full() {
        let structure = InlineQueryResult::Mpeg4Gif(InlineQueryResultMpeg4Gif {
            id: String::from("id"),
            mpeg4_url: Url::parse("http://mpeg4_url/").unwrap(),
            mpeg4_width: Some(1),
            mpeg4_height: Some(1),
            mpeg4_duration: Some(Seconds::from_seconds(1)),
            thumbnail_url: Url::parse("http://thumb_url/").unwrap(),
            thumbnail_mime_type: None,
            title: Some(String::from("title")),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            caption_entities: None,
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                parse_mode: Some(ParseMode::MarkdownV2),
                entities: None,
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
            })),
        });

        let expected_json = r#"{"type":"mpeg4_gif","id":"id","mpeg4_url":"http://mpeg4_url/","mpeg4_width":1,"mpeg4_height":1,"mpeg4_duration":1,"thumbnail_url":"http://thumb_url/","title":"title","caption":"caption","parse_mode":"HTML","reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","parse_mode":"MarkdownV2","link_preview_options":{"is_disabled":true}}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn cached_photo_min() {
        let structure = InlineQueryResult::CachedPhoto(InlineQueryResultCachedPhoto {
            id: String::from("id"),
            photo_file_id: String::from("photo_file_id"),
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json = r#"{"type":"photo","id":"id","photo_file_id":"photo_file_id"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn cached_photo_full() {
        let structure = InlineQueryResult::CachedPhoto(InlineQueryResultCachedPhoto {
            id: String::from("id"),
            photo_file_id: String::from("photo_file_id"),
            title: Some(String::from("title")),
            description: Some(String::from("description")),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            caption_entities: None,
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                parse_mode: Some(ParseMode::MarkdownV2),
                entities: None,
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
            })),
        });

        let expected_json = r#"{"type":"photo","id":"id","photo_file_id":"photo_file_id","title":"title","description":"description","caption":"caption","parse_mode":"HTML","reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","parse_mode":"MarkdownV2","link_preview_options":{"is_disabled":true}}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn photo_min() {
        let structure = InlineQueryResult::Photo(InlineQueryResultPhoto {
            id: String::from("id"),
            photo_url: Url::parse("http://photo_url/").unwrap(),
            thumbnail_url: Url::parse("http://thumb_url/").unwrap(),
            photo_width: None,
            photo_height: None,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json = r#"{"type":"photo","id":"id","photo_url":"http://photo_url/","thumbnail_url":"http://thumb_url/"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn photo_full() {
        let structure = InlineQueryResult::Photo(InlineQueryResultPhoto {
            id: String::from("id"),
            photo_url: Url::parse("http://photo_url/").unwrap(),
            thumbnail_url: Url::parse("http://thumb_url/").unwrap(),
            photo_width: Some(1),
            photo_height: Some(1),
            title: Some(String::from("title")),
            description: Some(String::from("description")),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            caption_entities: None,
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                parse_mode: Some(ParseMode::MarkdownV2),
                entities: None,
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
            })),
        });

        let expected_json = r#"{"type":"photo","id":"id","photo_url":"http://photo_url/","thumbnail_url":"http://thumb_url/","photo_width":1,"photo_height":1,"title":"title","description":"description","caption":"caption","parse_mode":"HTML","reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","parse_mode":"MarkdownV2","link_preview_options":{"is_disabled":true}}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn cached_sticker_min() {
        let structure = InlineQueryResult::CachedSticker(InlineQueryResultCachedSticker {
            id: String::from("id"),
            sticker_file_id: String::from("sticker_file_id"),
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json = r#"{"type":"sticker","id":"id","sticker_file_id":"sticker_file_id"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn cached_sticker_full() {
        let structure = InlineQueryResult::CachedSticker(InlineQueryResultCachedSticker {
            id: String::from("id"),
            sticker_file_id: String::from("sticker_file_id"),
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                entities: None,
                parse_mode: Some(ParseMode::MarkdownV2),
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
            })),
        });

        let expected_json = r#"{"type":"sticker","id":"id","sticker_file_id":"sticker_file_id","reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","parse_mode":"MarkdownV2","link_preview_options":{"is_disabled":true}}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn cached_video_min() {
        let structure = InlineQueryResult::CachedVideo(InlineQueryResultCachedVideo {
            id: String::from("id"),
            video_file_id: String::from("video_file_id"),
            title: String::from("title"),
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json =
            r#"{"type":"video","id":"id","video_file_id":"video_file_id","title":"title"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn cached_video_full() {
        let structure = InlineQueryResult::CachedVideo(InlineQueryResultCachedVideo {
            id: String::from("id"),
            video_file_id: String::from("video_file_id"),
            title: String::from("title"),
            description: Some(String::from("description")),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            caption_entities: None,
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Location(
                InputMessageContentLocation {
                    latitude: 1.0,
                    longitude: 1.0,
                    horizontal_accuracy: None,
                    live_period: None,
                    heading: None,
                    proximity_alert_radius: None,
                },
            )),
        });

        let expected_json = r#"{"type":"video","id":"id","video_file_id":"video_file_id","title":"title","description":"description","caption":"caption","parse_mode":"HTML","reply_markup":{"inline_keyboard":[]},"input_message_content":{"latitude":1.0,"longitude":1.0}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn video_min() {
        let structure = InlineQueryResult::Video(InlineQueryResultVideo {
            id: String::from("id"),
            video_url: Url::parse("http://video_url/").unwrap(),
            mime_type: Mime::from_str("video/mp4").unwrap(),
            thumbnail_url: Url::parse("http://thumb_url/").unwrap(),
            title: String::from("title"),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            video_width: None,
            video_height: None,
            video_duration: None,
            description: None,
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json = r#"{"type":"video","id":"id","video_url":"http://video_url/","mime_type":"video/mp4","thumbnail_url":"http://thumb_url/","title":"title"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn video_full() {
        let structure = InlineQueryResult::Video(InlineQueryResultVideo {
            id: String::from("id"),
            video_url: Url::parse("http://video_url/").unwrap(),
            mime_type: Mime::from_str("video/mp4").unwrap(),
            thumbnail_url: Url::parse("http://thumb_url/").unwrap(),
            title: String::from("title"),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            caption_entities: None,
            video_width: Some(1),
            video_height: Some(1),
            video_duration: Some(Seconds::from_seconds(1)),
            description: Some(String::from("description")),
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Location(
                InputMessageContentLocation {
                    latitude: 1.0,
                    longitude: 1.0,
                    horizontal_accuracy: None,
                    live_period: None,
                    heading: None,
                    proximity_alert_radius: None,
                },
            )),
        });

        let expected_json = r#"{"type":"video","id":"id","video_url":"http://video_url/","mime_type":"video/mp4","thumbnail_url":"http://thumb_url/","title":"title","caption":"caption","parse_mode":"HTML","video_width":1,"video_height":1,"video_duration":1,"description":"description","reply_markup":{"inline_keyboard":[]},"input_message_content":{"latitude":1.0,"longitude":1.0}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn cached_voice_min() {
        let structure = InlineQueryResult::CachedVoice(InlineQueryResultCachedVoice {
            id: String::from("id"),
            voice_file_id: String::from("voice_file_id"),
            title: String::from("title"),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json =
            r#"{"type":"voice","id":"id","voice_file_id":"voice_file_id","title":"title"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn cached_voice_full() {
        let structure = InlineQueryResult::CachedVoice(InlineQueryResultCachedVoice {
            id: String::from("id"),
            voice_file_id: String::from("voice_file_id"),
            title: String::from("title"),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            caption_entities: None,
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Location(
                InputMessageContentLocation {
                    latitude: 1.0,
                    longitude: 1.0,
                    horizontal_accuracy: None,
                    live_period: None,
                    heading: None,
                    proximity_alert_radius: None,
                },
            )),
        });

        let expected_json = r#"{"type":"voice","id":"id","voice_file_id":"voice_file_id","title":"title","caption":"caption","parse_mode":"HTML","reply_markup":{"inline_keyboard":[]},"input_message_content":{"latitude":1.0,"longitude":1.0}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn voice_min() {
        let structure = InlineQueryResult::Voice(InlineQueryResultVoice {
            id: String::from("id"),
            voice_url: Url::parse("http://voice_url/").unwrap(),
            title: String::from("title"),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            voice_duration: None,
            reply_markup: None,
            input_message_content: None,
        });

        let expected_json =
            r#"{"type":"voice","id":"id","voice_url":"http://voice_url/","title":"title"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn voice_full() {
        let structure = InlineQueryResult::Voice(InlineQueryResultVoice {
            id: String::from("id"),
            voice_url: Url::parse("http://voice_url/").unwrap(),
            title: String::from("title"),
            caption: Some(String::from("caption")),
            parse_mode: Some(ParseMode::Html),
            caption_entities: None,
            voice_duration: Some(Seconds::from_seconds(1)),
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Location(
                InputMessageContentLocation {
                    latitude: 1.0,
                    longitude: 1.0,
                    horizontal_accuracy: None,
                    live_period: None,
                    heading: None,
                    proximity_alert_radius: None,
                },
            )),
        });

        let expected_json = r#"{"type":"voice","id":"id","voice_url":"http://voice_url/","title":"title","caption":"caption","parse_mode":"HTML","voice_duration":1,"reply_markup":{"inline_keyboard":[]},"input_message_content":{"latitude":1.0,"longitude":1.0}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn article_min() {
        let structure = InlineQueryResult::Article(InlineQueryResultArticle {
            id: String::from("id"),
            title: String::from("title"),
            input_message_content: InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                entities: None,
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
                parse_mode: None,
            }),
            reply_markup: None,
            url: None,
            hide_url: None,
            description: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        });

        let expected_json = r#"{"type":"article","id":"id","title":"title","input_message_content":{"message_text":"message_text","link_preview_options":{"is_disabled":true}}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn article_full() {
        let structure = InlineQueryResult::Article(InlineQueryResultArticle {
            id: String::from("id"),
            title: String::from("title"),
            input_message_content: InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                entities: None,
                parse_mode: None,
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
            }),
            reply_markup: Some(InlineKeyboardMarkup::default()),
            url: Some(Url::parse("http://url/").unwrap()),
            hide_url: Some(true),
            description: Some(String::from("description")),
            thumbnail_url: Some(Url::parse("http://thumb_url/").unwrap()),
            thumbnail_width: Some(1),
            thumbnail_height: Some(1),
        });

        let expected_json = r#"{"type":"article","id":"id","title":"title","input_message_content":{"message_text":"message_text","link_preview_options":{"is_disabled":true}},"reply_markup":{"inline_keyboard":[]},"url":"http://url/","hide_url":true,"description":"description","thumbnail_url":"http://thumb_url/","thumbnail_width":1,"thumbnail_height":1}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn contact_min() {
        let structure = InlineQueryResult::Contact(InlineQueryResultContact {
            id: String::from("id"),
            phone_number: String::from("phone_number"),
            first_name: String::from("first_name"),
            last_name: None,
            vcard: None,
            reply_markup: None,
            input_message_content: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        });

        let expected_json = r#"{"type":"contact","id":"id","phone_number":"phone_number","first_name":"first_name"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn contact_full() {
        let structure = InlineQueryResult::Contact(InlineQueryResultContact {
            id: String::from("id"),
            phone_number: String::from("phone_number"),
            first_name: String::from("first_name"),
            last_name: Some(String::from("last_name")),
            vcard: Some(String::from("vcard")),
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                entities: None,
                parse_mode: None,
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
            })),
            thumbnail_url: Some(Url::parse("http://thumb_url/").unwrap()),
            thumbnail_width: Some(1),
            thumbnail_height: Some(1),
        });

        let expected_json = r#"{"type":"contact","id":"id","phone_number":"phone_number","first_name":"first_name","last_name":"last_name","vcard":"vcard","reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","link_preview_options":{"is_disabled":true}},"thumbnail_url":"http://thumb_url/","thumbnail_width":1,"thumbnail_height":1}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn game_min() {
        let structure = InlineQueryResult::Game(InlineQueryResultGame {
            id: String::from("id"),
            game_short_name: String::from("game_short_name"),
            reply_markup: None,
        });

        let expected_json = r#"{"type":"game","id":"id","game_short_name":"game_short_name"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn game_full() {
        let structure = InlineQueryResult::Game(InlineQueryResultGame {
            id: String::from("id"),
            game_short_name: String::from("game_short_name"),
            reply_markup: Some(InlineKeyboardMarkup::default()),
        });

        let expected_json = r#"{"type":"game","id":"id","game_short_name":"game_short_name","reply_markup":{"inline_keyboard":[]}}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn location_min() {
        let structure = InlineQueryResult::Location(InlineQueryResultLocation {
            id: String::from("id"),
            latitude: 1.0,
            longitude: 1.0,
            title: String::from("title"),
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
            input_message_content: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        });

        let expected_json =
            r#"{"type":"location","id":"id","latitude":1.0,"longitude":1.0,"title":"title"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn location_full() {
        let structure = InlineQueryResult::Location(InlineQueryResultLocation {
            id: String::from("id"),
            latitude: 1.0,
            longitude: 1.0,
            title: String::from("title"),
            horizontal_accuracy: Some(1.0),
            live_period: Some(1.into()),
            heading: Some(1),
            proximity_alert_radius: Some(1),
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                entities: None,
                parse_mode: None,
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
            })),
            thumbnail_url: Some(Url::parse("http://thumb_url/").unwrap()),
            thumbnail_width: Some(1),
            thumbnail_height: Some(1),
        });

        let expected_json = r#"{"type":"location","id":"id","latitude":1.0,"longitude":1.0,"title":"title","horizontal_accuracy":1.0,"live_period":1,"heading":1,"proximity_alert_radius":1,"reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","link_preview_options":{"is_disabled":true}},"thumbnail_url":"http://thumb_url/","thumbnail_width":1,"thumbnail_height":1}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn venue_min() {
        let structure = InlineQueryResult::Venue(InlineQueryResultVenue {
            id: String::from("id"),
            latitude: 1.0,
            longitude: 1.0,
            title: String::from("title"),
            address: String::from("address"),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            reply_markup: None,
            input_message_content: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        });

        let expected_json = r#"{"type":"venue","id":"id","latitude":1.0,"longitude":1.0,"title":"title","address":"address"}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }

    #[test]
    fn venue_full() {
        let structure = InlineQueryResult::Venue(InlineQueryResultVenue {
            id: String::from("id"),
            latitude: 1.0,
            longitude: 1.0,
            title: String::from("title"),
            address: String::from("address"),
            foursquare_id: Some(String::from("foursquare_id")),
            foursquare_type: Some(String::from("foursquare_type")),
            google_place_id: Some(String::from("google_place_id")),
            google_place_type: Some(String::from("google_place_type")),
            reply_markup: Some(InlineKeyboardMarkup::default()),
            input_message_content: Some(InputMessageContent::Text(InputMessageContentText {
                message_text: String::from("message_text"),
                entities: None,
                parse_mode: None,
                link_preview_options: Some(LinkPreviewOptions {
                    is_disabled: true,
                    url: None,
                    prefer_small_media: false,
                    prefer_large_media: false,
                    show_above_text: false,
                }),
            })),
            thumbnail_url: Some(Url::parse("http://thumb_url/").unwrap()),
            thumbnail_width: Some(1),
            thumbnail_height: Some(1),
        });

        let expected_json = r#"{"type":"venue","id":"id","latitude":1.0,"longitude":1.0,"title":"title","address":"address","foursquare_id":"foursquare_id","foursquare_type":"foursquare_type","google_place_id":"google_place_id","google_place_type":"google_place_type","reply_markup":{"inline_keyboard":[]},"input_message_content":{"message_text":"message_text","link_preview_options":{"is_disabled":true}},"thumbnail_url":"http://thumb_url/","thumbnail_width":1,"thumbnail_height":1}"#;
        let actual_json = serde_json::to_string(&structure).unwrap();

        assert_eq!(expected_json, actual_json);
        assert_eq!(structure, serde_json::from_str(&actual_json).unwrap());
    }
}
