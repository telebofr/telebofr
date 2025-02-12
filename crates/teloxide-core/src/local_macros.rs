macro_rules! req_future {
    (
        $v2:vis def: | $( $arg:ident: $ArgTy:ty ),* $(,)? | $body:block

        $(#[$($meta:tt)*])*
        $v:vis $i:ident<$T:ident> ($inner:ident) -> $Out:ty
        $(where $($wh:tt)*)?
    ) => {
        #[pin_project::pin_project]
        $v
        struct $i<$T>
        $(where $($wh)*)?
        {
            #[pin]
            inner: $inner::$i<$T>
        }

        impl<$T> $i<$T>
        $(where $($wh)*)?
        {
            $v2 fn new($( $arg: $ArgTy ),*) -> Self {
                Self { inner: $inner::def($( $arg ),*) }
            }
        }

        // HACK(waffle): workaround for https://github.com/rust-lang/rust/issues/55997
        mod $inner {
            #![allow(type_alias_bounds)]

            // Mostly to bring `use`s
            #[allow(unused_imports)]
            use super::{*, $i as _};

            #[cfg(feature = "nightly")]
            pub(crate) type $i<$T>
            $(where $($wh)*)? = impl ::core::future::Future<Output = $Out>;

            #[cfg(feature = "nightly")]
            pub(crate) fn def<$T>($( $arg: $ArgTy ),*) -> $i<$T>
            $(where $($wh)*)?
            {
                $body
            }

            #[cfg(not(feature = "nightly"))]
            pub(crate) type $i<$T>
            $(where $($wh)*)?  = ::core::pin::Pin<Box<dyn ::core::future::Future<Output = $Out> + ::core::marker::Send + 'static>>;

            #[cfg(not(feature = "nightly"))]
            pub(crate) fn def<$T>($( $arg: $ArgTy ),*) -> $i<$T>
            $(where $($wh)*)?
            {
                Box::pin($body)
            }
        }

        impl<$T> ::core::future::Future for $i<$T>
        $(where $($wh)*)?
        {
            type Output = $Out;

            fn poll(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) -> ::core::task::Poll<Self::Output> {
                let this = self.project();
                this.inner.poll(cx)
            }
        }

    };
}

/// Declares an item with a doc attribute computed by some macro expression.
/// This allows documentation to be dynamically generated based on input.
/// Necessary to work around https://github.com/rust-lang/rust/issues/52607.
macro_rules! calculated_doc {
    (
        $(
            #[doc = $doc:expr]
            $thing:item
        )*
    ) => (
        $(
            #[doc = $doc]
            $thing
        )*
    );
}

/// Declare payload type, implement `Payload` trait and ::new method for it,
/// declare setters trait and implement it for all type which have payload.
macro_rules! impl_payload {
    (
        $(
            @[multipart = $($multipart_attr:ident),*]
        )?
        $(
            @[timeout_secs = $timeout_secs:ident]
        )?
        $(
            #[ $($method_meta:tt)* ]
        )*
        $vi:vis $Method:ident ($Setters:ident) => $Ret:ty {
            $(
                required {
                    $(
                        $(
                            #[ $($field_meta:tt)* ]
                        )*
                        $v:vis $fields:ident : $FTy:ty $([$conv:ident])?
                        ,
                    )*
                }
            )?

            $(
                optional {
                    $(
                        $(
                            #[ $($opt_field_meta:tt)* ]
                        )*
                        $opt_v:vis $opt_fields:ident : $OptFTy:ty $([$opt_conv:ident])?
                    ),*
                    $(,)?
                }
            )?
        }
    ) => {
        #[serde_with::skip_serializing_none]
        #[must_use = "Requests do nothing unless sent"]
        $(
            #[ $($method_meta)* ]
        )*
        $vi struct $Method {
            $(
                $(
                    // FIXME: fix the cause of this warning
                    #[allow(rustdoc::invalid_html_tags)]
                    $(
                        #[ $($field_meta)* ]
                    )*
                    $v $fields : $FTy,
                )*
            )?
            $(
                $(
                    $(
                        #[ $($opt_field_meta)* ]
                    )*
                    $opt_v $opt_fields : core::option::Option<$OptFTy>,
                )*
            )?
        }

        impl $Method {
            // We mirror Telegram API and can't do anything with too many arguments.
            #[allow(clippy::too_many_arguments)]
            // It's just easier for macros to generate such code.
            #[allow(clippy::redundant_field_names)]
            // It's obvious what this method does. (If you think it's not, feel free to open a PR)
            #[allow(missing_docs)]
            $vi fn new($($($fields : impl_payload!(@convert? $FTy $([$conv])?)),*)?) -> Self {
                Self {
                    $(
                        $(
                            $fields: impl_payload!(@convert_map ($fields) $([$conv])?),
                        )*
                    )?
                    $(
                        $(
                            $opt_fields: None,
                        )*
                    )?
                }
            }
        }

        impl $crate::requests::Payload for $Method {
            type Output = $Ret;

            const NAME: &'static str = stringify!($Method);

            $(
                fn timeout_hint(&self) -> Option<std::time::Duration> {
                    self.$timeout_secs.map(<_>::into).map(std::time::Duration::from_secs)
                }
            )?
        }

        calculated_doc! {
            #[doc = concat!(
                "Setters for fields of [`",
                stringify!($Method),
                "`]"
            )]
            $vi trait $Setters: $crate::requests::HasPayload<Payload = $Method> + ::core::marker::Sized {
                $(
                    $(
                        impl_payload! { @setter $Method $fields : $FTy $([$conv])? }
                    )*
                )?
                $(
                    $(
                        impl_payload! { @setter_opt $Method $opt_fields : $OptFTy $([$opt_conv])? }
                    )*
                )?
            }
        }

        impl<P> $Setters for P where P: crate::requests::HasPayload<Payload = $Method> {}

        impl_payload! { @[$(multipart = $($multipart_attr),*)?] $Method req { $($($fields),*)? } opt { $($($opt_fields),*)? } }
    };
    (@setter_opt $Method:ident $field:ident : $FTy:ty [into]) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            #[allow(clippy::wrong_self_convention)]
            #[must_use = "Payloads and requests do nothing unless sent"]
            fn $field<T>(mut self, value: T) -> Self
            where
                T: Into<$FTy>,
            {
                self.payload_mut().$field = Some(value.into());
                self
            }
        }
    };
    (@setter_opt $Method:ident $field:ident : $FTy:ty [collect]) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            #[allow(clippy::wrong_self_convention)]
            #[must_use = "Payloads and requests do nothing unless sent"]
            fn $field<T>(mut self, value: T) -> Self
            where
                T: ::core::iter::IntoIterator<Item = <$FTy as ::core::iter::IntoIterator>::Item>,
            {
                self.payload_mut().$field = Some(value.into_iter().collect());
                self
            }
        }
    };
    (@setter_opt $Method:ident $field:ident : $FTy:ty) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            #[allow(clippy::wrong_self_convention)]
            #[must_use = "Payloads and requests do nothing unless sent"]
            fn $field(mut self, value: $FTy) -> Self {
                self.payload_mut().$field = Some(value);
                self
            }
        }
    };
    (@setter $Method:ident $field:ident : $FTy:ty [into]) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            #[allow(clippy::wrong_self_convention)]
            #[must_use = "Payloads and requests do nothing unless sent"]
            fn $field<T>(mut self, value: T) -> Self
            where
                T: Into<$FTy>,
            {
                self.payload_mut().$field = value.into();
                self
            }
        }
    };
    (@setter $Method:ident $field:ident : $FTy:ty [collect]) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            #[allow(clippy::wrong_self_convention)]
            #[must_use = "Payloads and requests do nothing unless sent"]
            fn $field<T>(mut self, value: T) -> Self
            where
                T: ::core::iter::IntoIterator<Item = <$FTy as ::core::iter::IntoIterator>::Item>,
            {
                self.payload_mut().$field = value.into_iter().collect();
                self
            }
        }
    };
    (@setter $Method:ident $field:ident : $FTy:ty) => {
        calculated_doc! {
            #[doc = concat!(
                "Setter for [`",
                stringify!($field),
                "`](",
                stringify!($Method),
                "::",
                stringify!($field),
                ") field."
            )]
            #[allow(clippy::wrong_self_convention)]
            #[must_use = "Payloads and requests do nothing unless sent"]
            fn $field(mut self, value: $FTy) -> Self {
                self.payload_mut().$field = value;
                self
            }
        }
    };
    (@convert? $T:ty [into]) => {
        impl ::core::convert::Into<$T>
    };
    (@convert? $T:ty [collect]) => {
        impl ::core::iter::IntoIterator<Item = <$T as ::core::iter::IntoIterator>::Item>
    };
    (@convert? $T:ty) => {
        $T
    };
    (@convert_map ($e:expr) [into]) => {
        $e.into()
    };
    (@convert_map ($e:expr) [collect]) => {
        $e.into_iter().collect()
    };
    (@convert_map ($e:expr)) => {
        $e
    };
    (@[multipart = $($multipart_attr:ident),*] $Method:ident req { $($reqf:ident),* } opt { $($optf:ident),*} ) => {
        impl crate::requests::MultipartPayload for $Method {
            fn copy_files(&self, into: &mut dyn FnMut(crate::types::InputFile)) {
                $(
                    crate::types::InputFileLike::copy_into(&self.$multipart_attr, into);
                )*
            }

            fn move_files(&mut self, into: &mut dyn FnMut(crate::types::InputFile)) {
                $(
                    crate::types::InputFileLike::move_into(&mut self.$multipart_attr, into);
                )*
            }
        }
    };
    (@[] $($ignored:tt)*) => {}
}

macro_rules! download_forward {
    ($T:ident $S:ty {$this:ident => $inner:expr}) => {
        impl<$T: $crate::net::Download> $crate::net::Download for $S {
            type Err<'dst> = <$T as $crate::net::Download>::Err<'dst>;

            type Fut<'dst> = <$T as $crate::net::Download>::Fut<'dst>;

            fn download_file<'dst>(
                &self,
                path: &str,
                destination: &'dst mut (dyn tokio::io::AsyncWrite
                               + core::marker::Unpin
                               + core::marker::Send),
            ) -> Self::Fut<'dst> {
                let $this = self;
                ($inner).download_file(path, destination)
            }

            type StreamErr = <$T as $crate::net::Download>::StreamErr;

            type Stream = <$T as $crate::net::Download>::Stream;

            fn download_file_stream(&self, path: &str) -> Self::Stream {
                let $this = self;
                ($inner).download_file_stream(path)
            }
        }
    };
}

macro_rules! requester_forward {
    ($i:ident $(, $rest:ident )* $(,)? => $body:ident, $ty:ident ) => {
        requester_forward!(@method $i $body $ty);
        $(
            requester_forward!(@method $rest $body $ty);
        )*
    };

// START BLOCK requester_forward_at_method
// Generated by `codegen_requester_forward`, do not edit by hand.


    (@method get_updates $body:ident $ty:ident) => {
        type GetUpdates = $ty![GetUpdates];

        fn get_updates(&self, ) -> Self::GetUpdates {
            let this = self;
            $body!(get_updates this ())
        }
    };
    (@method set_webhook $body:ident $ty:ident) => {
        type SetWebhook = $ty![SetWebhook];

        fn set_webhook(&self, url: Url) -> Self::SetWebhook {
            let this = self;
            $body!(set_webhook this (url: Url))
        }
    };
    (@method delete_webhook $body:ident $ty:ident) => {
        type DeleteWebhook = $ty![DeleteWebhook];

        fn delete_webhook(&self, ) -> Self::DeleteWebhook {
            let this = self;
            $body!(delete_webhook this ())
        }
    };
    (@method get_webhook_info $body:ident $ty:ident) => {
        type GetWebhookInfo = $ty![GetWebhookInfo];

        fn get_webhook_info(&self, ) -> Self::GetWebhookInfo {
            let this = self;
            $body!(get_webhook_info this ())
        }
    };
    (@method get_me $body:ident $ty:ident) => {
        type GetMe = $ty![GetMe];

        fn get_me(&self, ) -> Self::GetMe {
            let this = self;
            $body!(get_me this ())
        }
    };
    (@method log_out $body:ident $ty:ident) => {
        type LogOut = $ty![LogOut];

        fn log_out(&self, ) -> Self::LogOut {
            let this = self;
            $body!(log_out this ())
        }
    };
    (@method close $body:ident $ty:ident) => {
        type Close = $ty![Close];

        fn close(&self, ) -> Self::Close {
            let this = self;
            $body!(close this ())
        }
    };
    (@method send_message $body:ident $ty:ident) => {
        type SendMessage = $ty![SendMessage];

        fn send_message<C, T>(&self, chat_id: C, text: T) -> Self::SendMessage where C: Into<Recipient>,
        T: Into<String> {
            let this = self;
            $body!(send_message this (chat_id: C, text: T))
        }
    };
    (@method forward_message $body:ident $ty:ident) => {
        type ForwardMessage = $ty![ForwardMessage];

        fn forward_message<C, F>(&self, chat_id: C, from_chat_id: F, message_id: MessageId) -> Self::ForwardMessage where C: Into<Recipient>,
        F: Into<Recipient> {
            let this = self;
            $body!(forward_message this (chat_id: C, from_chat_id: F, message_id: MessageId))
        }
    };
    (@method forward_messages $body:ident $ty:ident) => {
        type ForwardMessages = $ty![ForwardMessages];

        fn forward_messages<C, F, M>(&self, chat_id: C, from_chat_id: F, message_ids: M) -> Self::ForwardMessages where C: Into<Recipient>,
        F: Into<Recipient>,
        M: IntoIterator<Item = MessageId> {
            let this = self;
            $body!(forward_messages this (chat_id: C, from_chat_id: F, message_ids: M))
        }
    };
    (@method copy_message $body:ident $ty:ident) => {
        type CopyMessage = $ty![CopyMessage];

        fn copy_message<C, F>(&self, chat_id: C, from_chat_id: F, message_id: MessageId) -> Self::CopyMessage where C: Into<Recipient>,
        F: Into<Recipient> {
            let this = self;
            $body!(copy_message this (chat_id: C, from_chat_id: F, message_id: MessageId))
        }
    };
    (@method copy_messages $body:ident $ty:ident) => {
        type CopyMessages = $ty![CopyMessages];

        fn copy_messages<C, F, M>(&self, chat_id: C, from_chat_id: F, message_ids: M) -> Self::CopyMessages where C: Into<Recipient>,
        F: Into<Recipient>,
        M: IntoIterator<Item = MessageId> {
            let this = self;
            $body!(copy_messages this (chat_id: C, from_chat_id: F, message_ids: M))
        }
    };
    (@method send_photo $body:ident $ty:ident) => {
        type SendPhoto = $ty![SendPhoto];

        fn send_photo<C>(&self, chat_id: C, photo: InputFile) -> Self::SendPhoto where C: Into<Recipient> {
            let this = self;
            $body!(send_photo this (chat_id: C, photo: InputFile))
        }
    };
    (@method send_audio $body:ident $ty:ident) => {
        type SendAudio = $ty![SendAudio];

        fn send_audio<C>(&self, chat_id: C, audio: InputFile) -> Self::SendAudio where C: Into<Recipient> {
            let this = self;
            $body!(send_audio this (chat_id: C, audio: InputFile))
        }
    };
    (@method send_document $body:ident $ty:ident) => {
        type SendDocument = $ty![SendDocument];

        fn send_document<C>(&self, chat_id: C, document: InputFile) -> Self::SendDocument where C: Into<Recipient> {
            let this = self;
            $body!(send_document this (chat_id: C, document: InputFile))
        }
    };
    (@method send_video $body:ident $ty:ident) => {
        type SendVideo = $ty![SendVideo];

        fn send_video<C>(&self, chat_id: C, video: InputFile) -> Self::SendVideo where C: Into<Recipient> {
            let this = self;
            $body!(send_video this (chat_id: C, video: InputFile))
        }
    };
    (@method send_animation $body:ident $ty:ident) => {
        type SendAnimation = $ty![SendAnimation];

        fn send_animation<C>(&self, chat_id: C, animation: InputFile) -> Self::SendAnimation where C: Into<Recipient> {
            let this = self;
            $body!(send_animation this (chat_id: C, animation: InputFile))
        }
    };
    (@method send_voice $body:ident $ty:ident) => {
        type SendVoice = $ty![SendVoice];

        fn send_voice<C>(&self, chat_id: C, voice: InputFile) -> Self::SendVoice where C: Into<Recipient> {
            let this = self;
            $body!(send_voice this (chat_id: C, voice: InputFile))
        }
    };
    (@method send_video_note $body:ident $ty:ident) => {
        type SendVideoNote = $ty![SendVideoNote];

        fn send_video_note<C>(&self, chat_id: C, video_note: InputFile) -> Self::SendVideoNote where C: Into<Recipient> {
            let this = self;
            $body!(send_video_note this (chat_id: C, video_note: InputFile))
        }
    };
    (@method send_media_group $body:ident $ty:ident) => {
        type SendMediaGroup = $ty![SendMediaGroup];

        fn send_media_group<C, M>(&self, chat_id: C, media: M) -> Self::SendMediaGroup where C: Into<Recipient>,
        M: IntoIterator<Item = InputMedia> {
            let this = self;
            $body!(send_media_group this (chat_id: C, media: M))
        }
    };
    (@method send_location $body:ident $ty:ident) => {
        type SendLocation = $ty![SendLocation];

        fn send_location<C>(&self, chat_id: C, latitude: f64, longitude: f64) -> Self::SendLocation where C: Into<Recipient> {
            let this = self;
            $body!(send_location this (chat_id: C, latitude: f64, longitude: f64))
        }
    };
    (@method edit_message_live_location $body:ident $ty:ident) => {
        type EditMessageLiveLocation = $ty![EditMessageLiveLocation];

        fn edit_message_live_location<C>(&self, chat_id: C, message_id: MessageId, latitude: f64, longitude: f64) -> Self::EditMessageLiveLocation where C: Into<Recipient> {
            let this = self;
            $body!(edit_message_live_location this (chat_id: C, message_id: MessageId, latitude: f64, longitude: f64))
        }
    };
    (@method edit_message_live_location_inline $body:ident $ty:ident) => {
        type EditMessageLiveLocationInline = $ty![EditMessageLiveLocationInline];

        fn edit_message_live_location_inline<I>(&self, inline_message_id: I, latitude: f64, longitude: f64) -> Self::EditMessageLiveLocationInline where I: Into<String> {
            let this = self;
            $body!(edit_message_live_location_inline this (inline_message_id: I, latitude: f64, longitude: f64))
        }
    };
    (@method stop_message_live_location $body:ident $ty:ident) => {
        type StopMessageLiveLocation = $ty![StopMessageLiveLocation];

        fn stop_message_live_location<C>(&self, chat_id: C, message_id: MessageId) -> Self::StopMessageLiveLocation where C: Into<Recipient> {
            let this = self;
            $body!(stop_message_live_location this (chat_id: C, message_id: MessageId))
        }
    };
    (@method stop_message_live_location_inline $body:ident $ty:ident) => {
        type StopMessageLiveLocationInline = $ty![StopMessageLiveLocationInline];

        fn stop_message_live_location_inline<I>(&self, inline_message_id: I) -> Self::StopMessageLiveLocationInline where I: Into<String> {
            let this = self;
            $body!(stop_message_live_location_inline this (inline_message_id: I))
        }
    };
    (@method send_venue $body:ident $ty:ident) => {
        type SendVenue = $ty![SendVenue];

        fn send_venue<C, T, A>(&self, chat_id: C, latitude: f64, longitude: f64, title: T, address: A) -> Self::SendVenue where C: Into<Recipient>,
        T: Into<String>,
        A: Into<String> {
            let this = self;
            $body!(send_venue this (chat_id: C, latitude: f64, longitude: f64, title: T, address: A))
        }
    };
    (@method send_contact $body:ident $ty:ident) => {
        type SendContact = $ty![SendContact];

        fn send_contact<C, P, F>(&self, chat_id: C, phone_number: P, first_name: F) -> Self::SendContact where C: Into<Recipient>,
        P: Into<String>,
        F: Into<String> {
            let this = self;
            $body!(send_contact this (chat_id: C, phone_number: P, first_name: F))
        }
    };
    (@method send_poll $body:ident $ty:ident) => {
        type SendPoll = $ty![SendPoll];

        fn send_poll<C, Q, O>(&self, chat_id: C, question: Q, options: O) -> Self::SendPoll where C: Into<Recipient>,
        Q: Into<String>,
        O: IntoIterator<Item = InputPollOption> {
            let this = self;
            $body!(send_poll this (chat_id: C, question: Q, options: O))
        }
    };
    (@method send_dice $body:ident $ty:ident) => {
        type SendDice = $ty![SendDice];

        fn send_dice<C>(&self, chat_id: C) -> Self::SendDice where C: Into<Recipient> {
            let this = self;
            $body!(send_dice this (chat_id: C))
        }
    };
    (@method send_chat_action $body:ident $ty:ident) => {
        type SendChatAction = $ty![SendChatAction];

        fn send_chat_action<C>(&self, chat_id: C, action: ChatAction) -> Self::SendChatAction where C: Into<Recipient> {
            let this = self;
            $body!(send_chat_action this (chat_id: C, action: ChatAction))
        }
    };
    (@method set_message_reaction $body:ident $ty:ident) => {
        type SetMessageReaction = $ty![SetMessageReaction];

        fn set_message_reaction<C>(&self, chat_id: C, message_id: MessageId) -> Self::SetMessageReaction where C: Into<Recipient> {
            let this = self;
            $body!(set_message_reaction this (chat_id: C, message_id: MessageId))
        }
    };
    (@method get_user_profile_photos $body:ident $ty:ident) => {
        type GetUserProfilePhotos = $ty![GetUserProfilePhotos];

        fn get_user_profile_photos(&self, user_id: UserId) -> Self::GetUserProfilePhotos {
            let this = self;
            $body!(get_user_profile_photos this (user_id: UserId))
        }
    };
    (@method get_file $body:ident $ty:ident) => {
        type GetFile = $ty![GetFile];

        fn get_file<F>(&self, file_id: F) -> Self::GetFile where F: Into<String> {
            let this = self;
            $body!(get_file this (file_id: F))
        }
    };
    (@method ban_chat_member $body:ident $ty:ident) => {
        type BanChatMember = $ty![BanChatMember];

        fn ban_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::BanChatMember where C: Into<Recipient> {
            let this = self;
            $body!(ban_chat_member this (chat_id: C, user_id: UserId))
        }
    };
    (@method kick_chat_member $body:ident $ty:ident) => {
        type KickChatMember = $ty![KickChatMember];

        fn kick_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::KickChatMember where C: Into<Recipient> {
            let this = self;
            $body!(kick_chat_member this (chat_id: C, user_id: UserId))
        }
    };
    (@method unban_chat_member $body:ident $ty:ident) => {
        type UnbanChatMember = $ty![UnbanChatMember];

        fn unban_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::UnbanChatMember where C: Into<Recipient> {
            let this = self;
            $body!(unban_chat_member this (chat_id: C, user_id: UserId))
        }
    };
    (@method restrict_chat_member $body:ident $ty:ident) => {
        type RestrictChatMember = $ty![RestrictChatMember];

        fn restrict_chat_member<C>(&self, chat_id: C, user_id: UserId, permissions: ChatPermissions) -> Self::RestrictChatMember where C: Into<Recipient> {
            let this = self;
            $body!(restrict_chat_member this (chat_id: C, user_id: UserId, permissions: ChatPermissions))
        }
    };
    (@method promote_chat_member $body:ident $ty:ident) => {
        type PromoteChatMember = $ty![PromoteChatMember];

        fn promote_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::PromoteChatMember where C: Into<Recipient> {
            let this = self;
            $body!(promote_chat_member this (chat_id: C, user_id: UserId))
        }
    };
    (@method set_chat_administrator_custom_title $body:ident $ty:ident) => {
        type SetChatAdministratorCustomTitle = $ty![SetChatAdministratorCustomTitle];

        fn set_chat_administrator_custom_title<Ch, C>(&self, chat_id: Ch, user_id: UserId, custom_title: C) -> Self::SetChatAdministratorCustomTitle where Ch: Into<Recipient>,
        C: Into<String> {
            let this = self;
            $body!(set_chat_administrator_custom_title this (chat_id: Ch, user_id: UserId, custom_title: C))
        }
    };
    (@method ban_chat_sender_chat $body:ident $ty:ident) => {
        type BanChatSenderChat = $ty![BanChatSenderChat];

        fn ban_chat_sender_chat<C, S>(&self, chat_id: C, sender_chat_id: S) -> Self::BanChatSenderChat where C: Into<Recipient>,
        S: Into<ChatId> {
            let this = self;
            $body!(ban_chat_sender_chat this (chat_id: C, sender_chat_id: S))
        }
    };
    (@method unban_chat_sender_chat $body:ident $ty:ident) => {
        type UnbanChatSenderChat = $ty![UnbanChatSenderChat];

        fn unban_chat_sender_chat<C, S>(&self, chat_id: C, sender_chat_id: S) -> Self::UnbanChatSenderChat where C: Into<Recipient>,
        S: Into<ChatId> {
            let this = self;
            $body!(unban_chat_sender_chat this (chat_id: C, sender_chat_id: S))
        }
    };
    (@method set_chat_permissions $body:ident $ty:ident) => {
        type SetChatPermissions = $ty![SetChatPermissions];

        fn set_chat_permissions<C>(&self, chat_id: C, permissions: ChatPermissions) -> Self::SetChatPermissions where C: Into<Recipient> {
            let this = self;
            $body!(set_chat_permissions this (chat_id: C, permissions: ChatPermissions))
        }
    };
    (@method export_chat_invite_link $body:ident $ty:ident) => {
        type ExportChatInviteLink = $ty![ExportChatInviteLink];

        fn export_chat_invite_link<C>(&self, chat_id: C) -> Self::ExportChatInviteLink where C: Into<Recipient> {
            let this = self;
            $body!(export_chat_invite_link this (chat_id: C))
        }
    };
    (@method create_chat_invite_link $body:ident $ty:ident) => {
        type CreateChatInviteLink = $ty![CreateChatInviteLink];

        fn create_chat_invite_link<C>(&self, chat_id: C) -> Self::CreateChatInviteLink where C: Into<Recipient> {
            let this = self;
            $body!(create_chat_invite_link this (chat_id: C))
        }
    };
    (@method edit_chat_invite_link $body:ident $ty:ident) => {
        type EditChatInviteLink = $ty![EditChatInviteLink];

        fn edit_chat_invite_link<C, I>(&self, chat_id: C, invite_link: I) -> Self::EditChatInviteLink where C: Into<Recipient>,
        I: Into<String> {
            let this = self;
            $body!(edit_chat_invite_link this (chat_id: C, invite_link: I))
        }
    };
    (@method revoke_chat_invite_link $body:ident $ty:ident) => {
        type RevokeChatInviteLink = $ty![RevokeChatInviteLink];

        fn revoke_chat_invite_link<C, I>(&self, chat_id: C, invite_link: I) -> Self::RevokeChatInviteLink where C: Into<Recipient>,
        I: Into<String> {
            let this = self;
            $body!(revoke_chat_invite_link this (chat_id: C, invite_link: I))
        }
    };
    (@method approve_chat_join_request $body:ident $ty:ident) => {
        type ApproveChatJoinRequest = $ty![ApproveChatJoinRequest];

        fn approve_chat_join_request<C>(&self, chat_id: C, user_id: UserId) -> Self::ApproveChatJoinRequest where C: Into<Recipient> {
            let this = self;
            $body!(approve_chat_join_request this (chat_id: C, user_id: UserId))
        }
    };
    (@method decline_chat_join_request $body:ident $ty:ident) => {
        type DeclineChatJoinRequest = $ty![DeclineChatJoinRequest];

        fn decline_chat_join_request<C>(&self, chat_id: C, user_id: UserId) -> Self::DeclineChatJoinRequest where C: Into<Recipient> {
            let this = self;
            $body!(decline_chat_join_request this (chat_id: C, user_id: UserId))
        }
    };
    (@method set_chat_photo $body:ident $ty:ident) => {
        type SetChatPhoto = $ty![SetChatPhoto];

        fn set_chat_photo<C>(&self, chat_id: C, photo: InputFile) -> Self::SetChatPhoto where C: Into<Recipient> {
            let this = self;
            $body!(set_chat_photo this (chat_id: C, photo: InputFile))
        }
    };
    (@method delete_chat_photo $body:ident $ty:ident) => {
        type DeleteChatPhoto = $ty![DeleteChatPhoto];

        fn delete_chat_photo<C>(&self, chat_id: C) -> Self::DeleteChatPhoto where C: Into<Recipient> {
            let this = self;
            $body!(delete_chat_photo this (chat_id: C))
        }
    };
    (@method set_chat_title $body:ident $ty:ident) => {
        type SetChatTitle = $ty![SetChatTitle];

        fn set_chat_title<C, T>(&self, chat_id: C, title: T) -> Self::SetChatTitle where C: Into<Recipient>,
        T: Into<String> {
            let this = self;
            $body!(set_chat_title this (chat_id: C, title: T))
        }
    };
    (@method set_chat_description $body:ident $ty:ident) => {
        type SetChatDescription = $ty![SetChatDescription];

        fn set_chat_description<C>(&self, chat_id: C) -> Self::SetChatDescription where C: Into<Recipient> {
            let this = self;
            $body!(set_chat_description this (chat_id: C))
        }
    };
    (@method pin_chat_message $body:ident $ty:ident) => {
        type PinChatMessage = $ty![PinChatMessage];

        fn pin_chat_message<C>(&self, chat_id: C, message_id: MessageId) -> Self::PinChatMessage where C: Into<Recipient> {
            let this = self;
            $body!(pin_chat_message this (chat_id: C, message_id: MessageId))
        }
    };
    (@method unpin_chat_message $body:ident $ty:ident) => {
        type UnpinChatMessage = $ty![UnpinChatMessage];

        fn unpin_chat_message<C>(&self, chat_id: C) -> Self::UnpinChatMessage where C: Into<Recipient> {
            let this = self;
            $body!(unpin_chat_message this (chat_id: C))
        }
    };
    (@method unpin_all_chat_messages $body:ident $ty:ident) => {
        type UnpinAllChatMessages = $ty![UnpinAllChatMessages];

        fn unpin_all_chat_messages<C>(&self, chat_id: C) -> Self::UnpinAllChatMessages where C: Into<Recipient> {
            let this = self;
            $body!(unpin_all_chat_messages this (chat_id: C))
        }
    };
    (@method leave_chat $body:ident $ty:ident) => {
        type LeaveChat = $ty![LeaveChat];

        fn leave_chat<C>(&self, chat_id: C) -> Self::LeaveChat where C: Into<Recipient> {
            let this = self;
            $body!(leave_chat this (chat_id: C))
        }
    };
    (@method get_chat $body:ident $ty:ident) => {
        type GetChat = $ty![GetChat];

        fn get_chat<C>(&self, chat_id: C) -> Self::GetChat where C: Into<Recipient> {
            let this = self;
            $body!(get_chat this (chat_id: C))
        }
    };
    (@method get_chat_administrators $body:ident $ty:ident) => {
        type GetChatAdministrators = $ty![GetChatAdministrators];

        fn get_chat_administrators<C>(&self, chat_id: C) -> Self::GetChatAdministrators where C: Into<Recipient> {
            let this = self;
            $body!(get_chat_administrators this (chat_id: C))
        }
    };
    (@method get_chat_member_count $body:ident $ty:ident) => {
        type GetChatMemberCount = $ty![GetChatMemberCount];

        fn get_chat_member_count<C>(&self, chat_id: C) -> Self::GetChatMemberCount where C: Into<Recipient> {
            let this = self;
            $body!(get_chat_member_count this (chat_id: C))
        }
    };
    (@method get_chat_members_count $body:ident $ty:ident) => {
        type GetChatMembersCount = $ty![GetChatMembersCount];

        fn get_chat_members_count<C>(&self, chat_id: C) -> Self::GetChatMembersCount where C: Into<Recipient> {
            let this = self;
            $body!(get_chat_members_count this (chat_id: C))
        }
    };
    (@method get_chat_member $body:ident $ty:ident) => {
        type GetChatMember = $ty![GetChatMember];

        fn get_chat_member<C>(&self, chat_id: C, user_id: UserId) -> Self::GetChatMember where C: Into<Recipient> {
            let this = self;
            $body!(get_chat_member this (chat_id: C, user_id: UserId))
        }
    };
    (@method set_chat_sticker_set $body:ident $ty:ident) => {
        type SetChatStickerSet = $ty![SetChatStickerSet];

        fn set_chat_sticker_set<C, S>(&self, chat_id: C, sticker_set_name: S) -> Self::SetChatStickerSet where C: Into<Recipient>,
        S: Into<String> {
            let this = self;
            $body!(set_chat_sticker_set this (chat_id: C, sticker_set_name: S))
        }
    };
    (@method delete_chat_sticker_set $body:ident $ty:ident) => {
        type DeleteChatStickerSet = $ty![DeleteChatStickerSet];

        fn delete_chat_sticker_set<C>(&self, chat_id: C) -> Self::DeleteChatStickerSet where C: Into<Recipient> {
            let this = self;
            $body!(delete_chat_sticker_set this (chat_id: C))
        }
    };
    (@method get_forum_topic_icon_stickers $body:ident $ty:ident) => {
        type GetForumTopicIconStickers = $ty![GetForumTopicIconStickers];

        fn get_forum_topic_icon_stickers(&self, ) -> Self::GetForumTopicIconStickers {
            let this = self;
            $body!(get_forum_topic_icon_stickers this ())
        }
    };
    (@method create_forum_topic $body:ident $ty:ident) => {
        type CreateForumTopic = $ty![CreateForumTopic];

        fn create_forum_topic<C, N, I>(&self, chat_id: C, name: N, icon_color: Rgb, icon_custom_emoji_id: I) -> Self::CreateForumTopic where C: Into<Recipient>,
        N: Into<String>,
        I: Into<String> {
            let this = self;
            $body!(create_forum_topic this (chat_id: C, name: N, icon_color: Rgb, icon_custom_emoji_id: I))
        }
    };
    (@method edit_forum_topic $body:ident $ty:ident) => {
        type EditForumTopic = $ty![EditForumTopic];

        fn edit_forum_topic<C>(&self, chat_id: C, message_thread_id: ThreadId) -> Self::EditForumTopic where C: Into<Recipient> {
            let this = self;
            $body!(edit_forum_topic this (chat_id: C, message_thread_id: ThreadId))
        }
    };
    (@method close_forum_topic $body:ident $ty:ident) => {
        type CloseForumTopic = $ty![CloseForumTopic];

        fn close_forum_topic<C>(&self, chat_id: C, message_thread_id: ThreadId) -> Self::CloseForumTopic where C: Into<Recipient> {
            let this = self;
            $body!(close_forum_topic this (chat_id: C, message_thread_id: ThreadId))
        }
    };
    (@method reopen_forum_topic $body:ident $ty:ident) => {
        type ReopenForumTopic = $ty![ReopenForumTopic];

        fn reopen_forum_topic<C>(&self, chat_id: C, message_thread_id: ThreadId) -> Self::ReopenForumTopic where C: Into<Recipient> {
            let this = self;
            $body!(reopen_forum_topic this (chat_id: C, message_thread_id: ThreadId))
        }
    };
    (@method delete_forum_topic $body:ident $ty:ident) => {
        type DeleteForumTopic = $ty![DeleteForumTopic];

        fn delete_forum_topic<C>(&self, chat_id: C, message_thread_id: ThreadId) -> Self::DeleteForumTopic where C: Into<Recipient> {
            let this = self;
            $body!(delete_forum_topic this (chat_id: C, message_thread_id: ThreadId))
        }
    };
    (@method unpin_all_forum_topic_messages $body:ident $ty:ident) => {
        type UnpinAllForumTopicMessages = $ty![UnpinAllForumTopicMessages];

        fn unpin_all_forum_topic_messages<C>(&self, chat_id: C, message_thread_id: ThreadId) -> Self::UnpinAllForumTopicMessages where C: Into<Recipient> {
            let this = self;
            $body!(unpin_all_forum_topic_messages this (chat_id: C, message_thread_id: ThreadId))
        }
    };
    (@method edit_general_forum_topic $body:ident $ty:ident) => {
        type EditGeneralForumTopic = $ty![EditGeneralForumTopic];

        fn edit_general_forum_topic<C, N>(&self, chat_id: C, name: N) -> Self::EditGeneralForumTopic where C: Into<Recipient>,
        N: Into<String> {
            let this = self;
            $body!(edit_general_forum_topic this (chat_id: C, name: N))
        }
    };
    (@method close_general_forum_topic $body:ident $ty:ident) => {
        type CloseGeneralForumTopic = $ty![CloseGeneralForumTopic];

        fn close_general_forum_topic<C>(&self, chat_id: C) -> Self::CloseGeneralForumTopic where C: Into<Recipient> {
            let this = self;
            $body!(close_general_forum_topic this (chat_id: C))
        }
    };
    (@method reopen_general_forum_topic $body:ident $ty:ident) => {
        type ReopenGeneralForumTopic = $ty![ReopenGeneralForumTopic];

        fn reopen_general_forum_topic<C>(&self, chat_id: C) -> Self::ReopenGeneralForumTopic where C: Into<Recipient> {
            let this = self;
            $body!(reopen_general_forum_topic this (chat_id: C))
        }
    };
    (@method hide_general_forum_topic $body:ident $ty:ident) => {
        type HideGeneralForumTopic = $ty![HideGeneralForumTopic];

        fn hide_general_forum_topic<C>(&self, chat_id: C) -> Self::HideGeneralForumTopic where C: Into<Recipient> {
            let this = self;
            $body!(hide_general_forum_topic this (chat_id: C))
        }
    };
    (@method unhide_general_forum_topic $body:ident $ty:ident) => {
        type UnhideGeneralForumTopic = $ty![UnhideGeneralForumTopic];

        fn unhide_general_forum_topic<C>(&self, chat_id: C) -> Self::UnhideGeneralForumTopic where C: Into<Recipient> {
            let this = self;
            $body!(unhide_general_forum_topic this (chat_id: C))
        }
    };
    (@method unpin_all_general_forum_topic_messages $body:ident $ty:ident) => {
        type UnpinAllGeneralForumTopicMessages = $ty![UnpinAllGeneralForumTopicMessages];

        fn unpin_all_general_forum_topic_messages<C>(&self, chat_id: C) -> Self::UnpinAllGeneralForumTopicMessages where C: Into<Recipient> {
            let this = self;
            $body!(unpin_all_general_forum_topic_messages this (chat_id: C))
        }
    };
    (@method answer_callback_query $body:ident $ty:ident) => {
        type AnswerCallbackQuery = $ty![AnswerCallbackQuery];

        fn answer_callback_query<C>(&self, callback_query_id: C) -> Self::AnswerCallbackQuery where C: Into<String> {
            let this = self;
            $body!(answer_callback_query this (callback_query_id: C))
        }
    };
    (@method get_user_chat_boosts $body:ident $ty:ident) => {
        type GetUserChatBoosts = $ty![GetUserChatBoosts];

        fn get_user_chat_boosts<C>(&self, chat_id: C, user_id: UserId) -> Self::GetUserChatBoosts where C: Into<Recipient> {
            let this = self;
            $body!(get_user_chat_boosts this (chat_id: C, user_id: UserId))
        }
    };
    (@method set_my_commands $body:ident $ty:ident) => {
        type SetMyCommands = $ty![SetMyCommands];

        fn set_my_commands<C>(&self, commands: C) -> Self::SetMyCommands where C: IntoIterator<Item = BotCommand> {
            let this = self;
            $body!(set_my_commands this (commands: C))
        }
    };
    (@method get_business_connection $body:ident $ty:ident) => {
        type GetBusinessConnection = $ty![GetBusinessConnection];

        fn get_business_connection(&self, business_connection_id: BusinessConnectionId) -> Self::GetBusinessConnection {
            let this = self;
            $body!(get_business_connection this (business_connection_id: BusinessConnectionId))
        }
    };
    (@method get_my_commands $body:ident $ty:ident) => {
        type GetMyCommands = $ty![GetMyCommands];

        fn get_my_commands(&self, ) -> Self::GetMyCommands {
            let this = self;
            $body!(get_my_commands this ())
        }
    };
    (@method set_my_name $body:ident $ty:ident) => {
        type SetMyName = $ty![SetMyName];

        fn set_my_name(&self, ) -> Self::SetMyName {
            let this = self;
            $body!(set_my_name this ())
        }
    };
    (@method get_my_name $body:ident $ty:ident) => {
        type GetMyName = $ty![GetMyName];

        fn get_my_name(&self, ) -> Self::GetMyName {
            let this = self;
            $body!(get_my_name this ())
        }
    };
    (@method set_my_description $body:ident $ty:ident) => {
        type SetMyDescription = $ty![SetMyDescription];

        fn set_my_description(&self, ) -> Self::SetMyDescription {
            let this = self;
            $body!(set_my_description this ())
        }
    };
    (@method get_my_description $body:ident $ty:ident) => {
        type GetMyDescription = $ty![GetMyDescription];

        fn get_my_description(&self, ) -> Self::GetMyDescription {
            let this = self;
            $body!(get_my_description this ())
        }
    };
    (@method set_my_short_description $body:ident $ty:ident) => {
        type SetMyShortDescription = $ty![SetMyShortDescription];

        fn set_my_short_description(&self, ) -> Self::SetMyShortDescription {
            let this = self;
            $body!(set_my_short_description this ())
        }
    };
    (@method get_my_short_description $body:ident $ty:ident) => {
        type GetMyShortDescription = $ty![GetMyShortDescription];

        fn get_my_short_description(&self, ) -> Self::GetMyShortDescription {
            let this = self;
            $body!(get_my_short_description this ())
        }
    };
    (@method set_chat_menu_button $body:ident $ty:ident) => {
        type SetChatMenuButton = $ty![SetChatMenuButton];

        fn set_chat_menu_button(&self, ) -> Self::SetChatMenuButton {
            let this = self;
            $body!(set_chat_menu_button this ())
        }
    };
    (@method get_chat_menu_button $body:ident $ty:ident) => {
        type GetChatMenuButton = $ty![GetChatMenuButton];

        fn get_chat_menu_button(&self, ) -> Self::GetChatMenuButton {
            let this = self;
            $body!(get_chat_menu_button this ())
        }
    };
    (@method set_my_default_administrator_rights $body:ident $ty:ident) => {
        type SetMyDefaultAdministratorRights = $ty![SetMyDefaultAdministratorRights];

        fn set_my_default_administrator_rights(&self, ) -> Self::SetMyDefaultAdministratorRights {
            let this = self;
            $body!(set_my_default_administrator_rights this ())
        }
    };
    (@method get_my_default_administrator_rights $body:ident $ty:ident) => {
        type GetMyDefaultAdministratorRights = $ty![GetMyDefaultAdministratorRights];

        fn get_my_default_administrator_rights(&self, ) -> Self::GetMyDefaultAdministratorRights {
            let this = self;
            $body!(get_my_default_administrator_rights this ())
        }
    };
    (@method delete_my_commands $body:ident $ty:ident) => {
        type DeleteMyCommands = $ty![DeleteMyCommands];

        fn delete_my_commands(&self, ) -> Self::DeleteMyCommands {
            let this = self;
            $body!(delete_my_commands this ())
        }
    };
    (@method answer_inline_query $body:ident $ty:ident) => {
        type AnswerInlineQuery = $ty![AnswerInlineQuery];

        fn answer_inline_query<I, R>(&self, inline_query_id: I, results: R) -> Self::AnswerInlineQuery where I: Into<String>,
        R: IntoIterator<Item = InlineQueryResult> {
            let this = self;
            $body!(answer_inline_query this (inline_query_id: I, results: R))
        }
    };
    (@method answer_web_app_query $body:ident $ty:ident) => {
        type AnswerWebAppQuery = $ty![AnswerWebAppQuery];

        fn answer_web_app_query<W>(&self, web_app_query_id: W, result: InlineQueryResult) -> Self::AnswerWebAppQuery where W: Into<String> {
            let this = self;
            $body!(answer_web_app_query this (web_app_query_id: W, result: InlineQueryResult))
        }
    };
    (@method edit_message_text $body:ident $ty:ident) => {
        type EditMessageText = $ty![EditMessageText];

        fn edit_message_text<C, T>(&self, chat_id: C, message_id: MessageId, text: T) -> Self::EditMessageText where C: Into<Recipient>,
        T: Into<String> {
            let this = self;
            $body!(edit_message_text this (chat_id: C, message_id: MessageId, text: T))
        }
    };
    (@method edit_message_text_inline $body:ident $ty:ident) => {
        type EditMessageTextInline = $ty![EditMessageTextInline];

        fn edit_message_text_inline<I, T>(&self, inline_message_id: I, text: T) -> Self::EditMessageTextInline where I: Into<String>,
        T: Into<String> {
            let this = self;
            $body!(edit_message_text_inline this (inline_message_id: I, text: T))
        }
    };
    (@method edit_message_caption $body:ident $ty:ident) => {
        type EditMessageCaption = $ty![EditMessageCaption];

        fn edit_message_caption<C>(&self, chat_id: C, message_id: MessageId) -> Self::EditMessageCaption where C: Into<Recipient> {
            let this = self;
            $body!(edit_message_caption this (chat_id: C, message_id: MessageId))
        }
    };
    (@method edit_message_caption_inline $body:ident $ty:ident) => {
        type EditMessageCaptionInline = $ty![EditMessageCaptionInline];

        fn edit_message_caption_inline<I>(&self, inline_message_id: I) -> Self::EditMessageCaptionInline where I: Into<String> {
            let this = self;
            $body!(edit_message_caption_inline this (inline_message_id: I))
        }
    };
    (@method edit_message_media $body:ident $ty:ident) => {
        type EditMessageMedia = $ty![EditMessageMedia];

        fn edit_message_media<C>(&self, chat_id: C, message_id: MessageId, media: InputMedia) -> Self::EditMessageMedia where C: Into<Recipient> {
            let this = self;
            $body!(edit_message_media this (chat_id: C, message_id: MessageId, media: InputMedia))
        }
    };
    (@method edit_message_media_inline $body:ident $ty:ident) => {
        type EditMessageMediaInline = $ty![EditMessageMediaInline];

        fn edit_message_media_inline<I>(&self, inline_message_id: I, media: InputMedia) -> Self::EditMessageMediaInline where I: Into<String> {
            let this = self;
            $body!(edit_message_media_inline this (inline_message_id: I, media: InputMedia))
        }
    };
    (@method edit_message_reply_markup $body:ident $ty:ident) => {
        type EditMessageReplyMarkup = $ty![EditMessageReplyMarkup];

        fn edit_message_reply_markup<C>(&self, chat_id: C, message_id: MessageId) -> Self::EditMessageReplyMarkup where C: Into<Recipient> {
            let this = self;
            $body!(edit_message_reply_markup this (chat_id: C, message_id: MessageId))
        }
    };
    (@method edit_message_reply_markup_inline $body:ident $ty:ident) => {
        type EditMessageReplyMarkupInline = $ty![EditMessageReplyMarkupInline];

        fn edit_message_reply_markup_inline<I>(&self, inline_message_id: I) -> Self::EditMessageReplyMarkupInline where I: Into<String> {
            let this = self;
            $body!(edit_message_reply_markup_inline this (inline_message_id: I))
        }
    };
    (@method stop_poll $body:ident $ty:ident) => {
        type StopPoll = $ty![StopPoll];

        fn stop_poll<C>(&self, chat_id: C, message_id: MessageId) -> Self::StopPoll where C: Into<Recipient> {
            let this = self;
            $body!(stop_poll this (chat_id: C, message_id: MessageId))
        }
    };
    (@method delete_message $body:ident $ty:ident) => {
        type DeleteMessage = $ty![DeleteMessage];

        fn delete_message<C>(&self, chat_id: C, message_id: MessageId) -> Self::DeleteMessage where C: Into<Recipient> {
            let this = self;
            $body!(delete_message this (chat_id: C, message_id: MessageId))
        }
    };
    (@method delete_messages $body:ident $ty:ident) => {
        type DeleteMessages = $ty![DeleteMessages];

        fn delete_messages<C, M>(&self, chat_id: C, message_ids: M) -> Self::DeleteMessages where C: Into<Recipient>,
        M: IntoIterator<Item = MessageId> {
            let this = self;
            $body!(delete_messages this (chat_id: C, message_ids: M))
        }
    };
    (@method send_sticker $body:ident $ty:ident) => {
        type SendSticker = $ty![SendSticker];

        fn send_sticker<C>(&self, chat_id: C, sticker: InputFile) -> Self::SendSticker where C: Into<Recipient> {
            let this = self;
            $body!(send_sticker this (chat_id: C, sticker: InputFile))
        }
    };
    (@method get_sticker_set $body:ident $ty:ident) => {
        type GetStickerSet = $ty![GetStickerSet];

        fn get_sticker_set<N>(&self, name: N) -> Self::GetStickerSet where N: Into<String> {
            let this = self;
            $body!(get_sticker_set this (name: N))
        }
    };
    (@method get_custom_emoji_stickers $body:ident $ty:ident) => {
        type GetCustomEmojiStickers = $ty![GetCustomEmojiStickers];

        fn get_custom_emoji_stickers<C>(&self, custom_emoji_ids: C) -> Self::GetCustomEmojiStickers where C: IntoIterator<Item = String> {
            let this = self;
            $body!(get_custom_emoji_stickers this (custom_emoji_ids: C))
        }
    };
    (@method upload_sticker_file $body:ident $ty:ident) => {
        type UploadStickerFile = $ty![UploadStickerFile];

        fn upload_sticker_file(&self, user_id: UserId, sticker: InputFile, sticker_format: StickerFormat) -> Self::UploadStickerFile {
            let this = self;
            $body!(upload_sticker_file this (user_id: UserId, sticker: InputFile, sticker_format: StickerFormat))
        }
    };
    (@method create_new_sticker_set $body:ident $ty:ident) => {
        type CreateNewStickerSet = $ty![CreateNewStickerSet];

        fn create_new_sticker_set<N, T, S>(&self, user_id: UserId, name: N, title: T, stickers: S) -> Self::CreateNewStickerSet where N: Into<String>,
        T: Into<String>,
        S: IntoIterator<Item = InputSticker> {
            let this = self;
            $body!(create_new_sticker_set this (user_id: UserId, name: N, title: T, stickers: S))
        }
    };
    (@method add_sticker_to_set $body:ident $ty:ident) => {
        type AddStickerToSet = $ty![AddStickerToSet];

        fn add_sticker_to_set<N>(&self, user_id: UserId, name: N, sticker: InputSticker) -> Self::AddStickerToSet where N: Into<String> {
            let this = self;
            $body!(add_sticker_to_set this (user_id: UserId, name: N, sticker: InputSticker))
        }
    };
    (@method set_sticker_position_in_set $body:ident $ty:ident) => {
        type SetStickerPositionInSet = $ty![SetStickerPositionInSet];

        fn set_sticker_position_in_set<S>(&self, sticker: S, position: u32) -> Self::SetStickerPositionInSet where S: Into<String> {
            let this = self;
            $body!(set_sticker_position_in_set this (sticker: S, position: u32))
        }
    };
    (@method delete_sticker_from_set $body:ident $ty:ident) => {
        type DeleteStickerFromSet = $ty![DeleteStickerFromSet];

        fn delete_sticker_from_set<S>(&self, sticker: S) -> Self::DeleteStickerFromSet where S: Into<String> {
            let this = self;
            $body!(delete_sticker_from_set this (sticker: S))
        }
    };
    (@method replace_sticker_in_set $body:ident $ty:ident) => {
        type ReplaceStickerInSet = $ty![ReplaceStickerInSet];

        fn replace_sticker_in_set<N, O>(&self, user_id: UserId, name: N, old_sticker: O, sticker: InputSticker) -> Self::ReplaceStickerInSet where N: Into<String>,
        O: Into<String> {
            let this = self;
            $body!(replace_sticker_in_set this (user_id: UserId, name: N, old_sticker: O, sticker: InputSticker))
        }
    };
    (@method set_sticker_set_thumbnail $body:ident $ty:ident) => {
        type SetStickerSetThumbnail = $ty![SetStickerSetThumbnail];

        fn set_sticker_set_thumbnail<N>(&self, name: N, user_id: UserId, format: StickerFormat) -> Self::SetStickerSetThumbnail where N: Into<String> {
            let this = self;
            $body!(set_sticker_set_thumbnail this (name: N, user_id: UserId, format: StickerFormat))
        }
    };
    (@method set_custom_emoji_sticker_set_thumbnail $body:ident $ty:ident) => {
        type SetCustomEmojiStickerSetThumbnail = $ty![SetCustomEmojiStickerSetThumbnail];

        fn set_custom_emoji_sticker_set_thumbnail<N>(&self, name: N) -> Self::SetCustomEmojiStickerSetThumbnail where N: Into<String> {
            let this = self;
            $body!(set_custom_emoji_sticker_set_thumbnail this (name: N))
        }
    };
    (@method set_sticker_set_title $body:ident $ty:ident) => {
        type SetStickerSetTitle = $ty![SetStickerSetTitle];

        fn set_sticker_set_title<N, T>(&self, name: N, title: T) -> Self::SetStickerSetTitle where N: Into<String>,
        T: Into<String> {
            let this = self;
            $body!(set_sticker_set_title this (name: N, title: T))
        }
    };
    (@method delete_sticker_set $body:ident $ty:ident) => {
        type DeleteStickerSet = $ty![DeleteStickerSet];

        fn delete_sticker_set<N>(&self, name: N) -> Self::DeleteStickerSet where N: Into<String> {
            let this = self;
            $body!(delete_sticker_set this (name: N))
        }
    };
    (@method set_sticker_emoji_list $body:ident $ty:ident) => {
        type SetStickerEmojiList = $ty![SetStickerEmojiList];

        fn set_sticker_emoji_list<S, E>(&self, sticker: S, emoji_list: E) -> Self::SetStickerEmojiList where S: Into<String>,
        E: IntoIterator<Item = String> {
            let this = self;
            $body!(set_sticker_emoji_list this (sticker: S, emoji_list: E))
        }
    };
    (@method set_sticker_keywords $body:ident $ty:ident) => {
        type SetStickerKeywords = $ty![SetStickerKeywords];

        fn set_sticker_keywords<S>(&self, sticker: S) -> Self::SetStickerKeywords where S: Into<String> {
            let this = self;
            $body!(set_sticker_keywords this (sticker: S))
        }
    };
    (@method set_sticker_mask_position $body:ident $ty:ident) => {
        type SetStickerMaskPosition = $ty![SetStickerMaskPosition];

        fn set_sticker_mask_position<S>(&self, sticker: S) -> Self::SetStickerMaskPosition where S: Into<String> {
            let this = self;
            $body!(set_sticker_mask_position this (sticker: S))
        }
    };
    (@method send_invoice $body:ident $ty:ident) => {
        type SendInvoice = $ty![SendInvoice];

        fn send_invoice<Ch, T, D, Pa, P, C, Pri>(&self, chat_id: Ch, title: T, description: D, payload: Pa, provider_token: P, currency: C, prices: Pri) -> Self::SendInvoice where Ch: Into<Recipient>,
        T: Into<String>,
        D: Into<String>,
        Pa: Into<String>,
        P: Into<String>,
        C: Into<String>,
        Pri: IntoIterator<Item = LabeledPrice> {
            let this = self;
            $body!(send_invoice this (chat_id: Ch, title: T, description: D, payload: Pa, provider_token: P, currency: C, prices: Pri))
        }
    };
    (@method create_invoice_link $body:ident $ty:ident) => {
        type CreateInvoiceLink = $ty![CreateInvoiceLink];

        fn create_invoice_link<T, D, Pa, P, C, Pri>(&self, title: T, description: D, payload: Pa, provider_token: P, currency: C, prices: Pri) -> Self::CreateInvoiceLink where T: Into<String>,
        D: Into<String>,
        Pa: Into<String>,
        P: Into<String>,
        C: Into<String>,
        Pri: IntoIterator<Item = LabeledPrice> {
            let this = self;
            $body!(create_invoice_link this (title: T, description: D, payload: Pa, provider_token: P, currency: C, prices: Pri))
        }
    };
    (@method answer_shipping_query $body:ident $ty:ident) => {
        type AnswerShippingQuery = $ty![AnswerShippingQuery];

        fn answer_shipping_query<S>(&self, shipping_query_id: S, ok: bool) -> Self::AnswerShippingQuery where S: Into<String> {
            let this = self;
            $body!(answer_shipping_query this (shipping_query_id: S, ok: bool))
        }
    };
    (@method answer_pre_checkout_query $body:ident $ty:ident) => {
        type AnswerPreCheckoutQuery = $ty![AnswerPreCheckoutQuery];

        fn answer_pre_checkout_query<P>(&self, pre_checkout_query_id: P, ok: bool) -> Self::AnswerPreCheckoutQuery where P: Into<String> {
            let this = self;
            $body!(answer_pre_checkout_query this (pre_checkout_query_id: P, ok: bool))
        }
    };
    (@method set_passport_data_errors $body:ident $ty:ident) => {
        type SetPassportDataErrors = $ty![SetPassportDataErrors];

        fn set_passport_data_errors<E>(&self, user_id: UserId, errors: E) -> Self::SetPassportDataErrors where E: IntoIterator<Item = PassportElementError> {
            let this = self;
            $body!(set_passport_data_errors this (user_id: UserId, errors: E))
        }
    };
    (@method send_game $body:ident $ty:ident) => {
        type SendGame = $ty![SendGame];

        fn send_game<C, G>(&self, chat_id: C, game_short_name: G) -> Self::SendGame where C: Into<ChatId>,
        G: Into<String> {
            let this = self;
            $body!(send_game this (chat_id: C, game_short_name: G))
        }
    };
    (@method set_game_score $body:ident $ty:ident) => {
        type SetGameScore = $ty![SetGameScore];

        fn set_game_score(&self, user_id: UserId, score: u64, chat_id: u32, message_id: MessageId) -> Self::SetGameScore {
            let this = self;
            $body!(set_game_score this (user_id: UserId, score: u64, chat_id: u32, message_id: MessageId))
        }
    };
    (@method set_game_score_inline $body:ident $ty:ident) => {
        type SetGameScoreInline = $ty![SetGameScoreInline];

        fn set_game_score_inline<I>(&self, user_id: UserId, score: u64, inline_message_id: I) -> Self::SetGameScoreInline where I: Into<String> {
            let this = self;
            $body!(set_game_score_inline this (user_id: UserId, score: u64, inline_message_id: I))
        }
    };
    (@method get_game_high_scores $body:ident $ty:ident) => {
        type GetGameHighScores = $ty![GetGameHighScores];

        fn get_game_high_scores<T>(&self, user_id: UserId, target: T) -> Self::GetGameHighScores where T: Into<TargetMessage> {
            let this = self;
            $body!(get_game_high_scores this (user_id: UserId, target: T))
        }
    };// END BLOCK requester_forward_at_method
}

#[test]
// waffle: efficiency is not important here, and I don't want to rewrite this
#[allow(clippy::format_collect)]
fn codegen_requester_forward() {
    use crate::codegen::{
        add_hidden_preamble,
        convert::{convert_for, Convert},
        ensure_file_contents, min_prefix, project_root, reformat, replace_block,
        schema::{self, Type},
        to_uppercase,
    };
    use indexmap::IndexMap;
    use itertools::Itertools;

    let path = project_root().join("src/local_macros.rs");
    let schema = schema::get();

    let contents = schema
        .methods
        .iter()
        .map(|m| {
            let mut convert_params = m
                .params
                .iter()
                .filter(|p| !matches!(p.ty, Type::Option(_)))
                .map(|p| (&p.name, convert_for(&p.ty)))
                .filter(|(_, c)| !matches!(c, Convert::Id(_)))
                .map(|(name, _)| &**name)
                .collect::<Vec<_>>();

            convert_params.sort_unstable();

            let mut prefixes: IndexMap<_, _> = convert_params
                .iter()
                .copied()
                // Workaround to output the last type as the first letter
                .chain(["\0"])
                .tuple_windows()
                .map(|(l, r)| (l, min_prefix(l, r)))
                .collect();

            // FIXME: This hard-coded value has been set to avoid conflicting generic
            // parameter 'B' with impl<B> Requester... in all the adaptors and other places
            //
            // One fix could be to take full abbrevation for all the parameters instead of
            // just the first character. Other fix is to change the generic parameter name
            // in all the impl blocks to something like 'Z' because that is very less likely
            // to conflict in future.
            if prefixes.contains_key("business_connection_id") {
                prefixes["business_connection_id"] = "BCI";
            }
            let prefixes = prefixes;

            let args = m
                .params
                .iter()
                .filter(|p| !matches!(p.ty, Type::Option(_)))
                .map(|p| match prefixes.get(&*p.name) {
                    Some(prefix) => format!("{}: {}", p.name, to_uppercase(prefix)),
                    None => format!("{}: {}", p.name, p.ty),
                })
                .join(", ");

            let generics = m
                .params
                .iter()
                .flat_map(|p| prefixes.get(&*p.name))
                .copied()
                .map(to_uppercase)
                .join(", ");
            let where_clause = m
                .params
                .iter()
                .filter(|p| !matches!(p.ty, Type::Option(_)))
                .flat_map(|p| match convert_for(&p.ty) {
                    Convert::Id(_) => None,
                    Convert::Into(ty) => {
                        Some(format!("{}: Into<{}>", &to_uppercase(prefixes[&*p.name]), ty))
                    }
                    Convert::Collect(ty) => Some(format!(
                        "{}: IntoIterator<Item = {}>",
                        &to_uppercase(prefixes[&*p.name]),
                        ty
                    )),
                })
                .join(",\n        ");

            let generics =
                if generics.is_empty() { String::from("") } else { format!("<{generics}>") };

            let where_clause = if where_clause.is_empty() {
                String::from("")
            } else {
                format!(" where {where_clause}")
            };

            format!(
                "
    (@method {method} $body:ident $ty:ident) => {{
        type {Method} = $ty![{Method}];

        fn {method}{generics}(&self, {args}) -> Self::{Method}{where_clause} {{
            let this = self;
            $body!({method} this ({args}))
        }}
    }};",
                Method = m.names.1,
                method = m.names.2,
            )
        })
        .collect();

    let contents = reformat(replace_block(
        &path,
        "requester_forward_at_method",
        &add_hidden_preamble("codegen_requester_forward", contents),
    ));

    ensure_file_contents(&path, &contents);
}
