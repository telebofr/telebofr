use reqwest::Url;
use serde::{Deserialize, Serialize};

use crate::types::{LabeledPrice, LinkPreviewOptions, LivePeriod, MessageEntity, ParseMode};

/// This object represents the content of a message to be sent as a result of an
/// inline query.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmessagecontent).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputMessageContentText),
    Location(InputMessageContentLocation),
    Venue(InputMessageContentVenue),
    Contact(InputMessageContentContact),
    Invoice(InputMessageContentInvoice),
}
/// Represents the content of a text message to be sent as the result of an
/// inline query.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputMessageContentText {
    /// Text of the message to be sent, 1-4096 characters.
    pub message_text: String,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in message text, which can be
    /// specified instead of `parse_mode`.
    pub entities: Option<Vec<MessageEntity>>,

    /// Link preview generation options for the message
    pub link_preview_options: Option<LinkPreviewOptions>,
}

impl InputMessageContentText {
    pub fn new<S>(message_text: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            message_text: message_text.into(),
            parse_mode: None,
            entities: None,
            link_preview_options: None,
        }
    }

    pub fn message_text<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.message_text = val.into();
        self
    }

    #[must_use]
    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn entities<C>(mut self, val: C) -> Self
    where
        C: IntoIterator<Item = MessageEntity>,
    {
        self.entities = Some(val.into_iter().collect());
        self
    }

    #[must_use]
    pub fn link_preview_options(mut self, val: LinkPreviewOptions) -> Self {
        self.link_preview_options = Some(val);
        self
    }
}

/// Represents the content of a location message to be sent as the result of an
/// inline query.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputMessageContentLocation {
    /// Latitude of the location in degrees.
    pub latitude: f64,

    /// Longitude of the location in degrees.
    pub longitude: f64,

    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,

    /// Period in seconds for which the location can be updated, should be
    /// between 60 and 86400, or 0x7FFFFFFF for live locations that can be
    /// edited indefinitely.
    pub live_period: Option<LivePeriod>,

    /// For live locations, a direction in which the user is moving, in degrees.
    /// Must be between 1 and 360 if specified.
    pub heading: Option<u16>,

    /// For live locations, a maximum distance for proximity alerts about
    /// approaching another chat member, in meters. Must be between 1 and 100000
    /// if specified.
    pub proximity_alert_radius: Option<u32>,
}

impl InputMessageContentLocation {
    #[must_use]
    pub const fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            live_period: None,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
        }
    }

    #[must_use]
    pub const fn latitude(mut self, val: f64) -> Self {
        self.latitude = val;
        self
    }

    #[must_use]
    pub const fn longitude(mut self, val: f64) -> Self {
        self.longitude = val;
        self
    }

    #[must_use]
    pub const fn live_period(mut self, val: LivePeriod) -> Self {
        self.live_period = Some(val);
        self
    }
}

/// Represents the content of a venue message to be sent as the result of
/// an inline query.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputMessageContentVenue {
    /// Latitude of the venue in degrees.
    pub latitude: f64,

    /// Longitude of the venue in degrees.
    pub longitude: f64,

    /// Name of the venue.
    pub title: String,

    /// Address of the venue.
    pub address: String,

    /// Foursquare identifier of the venue, if known.
    pub foursquare_id: Option<String>,

    /// Foursquare type of the venue, if known. (For example,
    /// `arts_entertainment/default`, `arts_entertainment/aquarium`
    /// or `food/icecream`.)
    pub foursquare_type: Option<String>,

    /// Google Places identifier of the venue.
    pub google_place_id: Option<String>,

    /// Google Places type of the venue. (See [supported types].)
    ///
    /// [supported types]: https://developers.google.com/places/web-service/supported_types
    pub google_place_type: Option<String>,
}

impl InputMessageContentVenue {
    pub fn new<S1, S2>(latitude: f64, longitude: f64, title: S1, address: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }

    #[must_use]
    pub fn latitude(mut self, val: f64) -> Self {
        self.latitude = val;
        self
    }

    #[must_use]
    pub fn longitude(mut self, val: f64) -> Self {
        self.longitude = val;
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = val.into();
        self
    }

    pub fn address<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.address = val.into();
        self
    }

    pub fn foursquare_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.foursquare_id = Some(val.into());
        self
    }

    pub fn foursquare_type<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.foursquare_type = Some(val.into());
        self
    }
}

/// Represents the content of a contact message to be sent as the result of
/// an inline query.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputMessageContentContact {
    /// Contact's phone number.
    pub phone_number: String,

    /// Contact's first name.
    pub first_name: String,

    /// Contact's last name.
    pub last_name: Option<String>,

    /// Additional data about the contact in the form of a [vCard], 0-2048
    /// bytes.
    ///
    /// [vCard]: https://en.wikipedia.org/wiki/VCard
    pub vcard: Option<String>,
}

impl InputMessageContentContact {
    pub fn new<S1, S2>(phone_number: S1, first_name: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
        }
    }

    pub fn phone_number<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.phone_number = val.into();
        self
    }

    pub fn first_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.first_name = val.into();
        self
    }

    pub fn last_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.last_name = Some(val.into());
        self
    }

    pub fn vcard<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.vcard = Some(val.into());
        self
    }
}

/// Represents the [content] of an invoice message to be sent as the result of
/// an inline query.
///
/// [content]: InputMessageContent
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputMessageContentInvoice {
    /// Product name, 1-32 characters
    pub title: String,

    /// Product description, 1-255 characters
    pub description: String,

    /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to
    /// the user, use for your internal processes.
    pub payload: String,

    /// Payment provider token, obtained via [@Botfather]
    ///
    /// [@Botfather]: https://t.me/Botfather
    pub provider_token: String,

    /// Three-letter ISO 4217 currency code, see [more on currencies]. Pass
    /// `XTR` for payments in [Telegram Stars].
    ///
    /// [more on currencies]: https://core.telegram.org/bots/payments#supported-currencies
    /// [Telegram Stars]: https://t.me/BotNews/90
    pub currency: String,

    /// Price breakdown, list of components (e.g. product price, tax, discount,
    /// delivery cost, delivery tax, bonus, etc.)
    pub prices: Vec<LabeledPrice>,

    /// ----The maximum accepted amount for tips in the smallest units of the
    /// currency (integer, not float/double). For example, for a maximum tip of
    /// US$ 1.45 pass max_tip_amount = 145. See the exp parameter in
    /// currencies.json, it shows the number of digits past the decimal point
    /// for each currency (2 for the majority of currencies). Defaults to 0
    pub max_tip_amount: Option<u32>,

    /// List of suggested amounts of tip in the smallest units of the currency
    /// (integer, not float/double). At most 4 suggested tip amounts can be
    /// specified. The suggested tip amounts must be positive, passed in a
    /// strictly increased order and must not exceed max_tip_amount.
    pub suggested_tip_amounts: Option<Vec<u32>>,

    /// ----A JSON-serialized object for data about the invoice, which will be
    /// shared with the payment provider. A detailed description of the required
    /// fields should be provided by the payment provider.
    pub provider_data: Option<String>,

    /// URL of the product photo for the invoice. Can be a photo of the goods or
    /// a marketing image for a service. People like it better when they see
    /// what they are paying for.
    pub photo_url: Option<Url>,

    /// Photo size
    pub photo_size: Option<u32>,

    /// Photo width
    pub photo_width: Option<u32>,

    /// Photo height
    pub photo_height: Option<u32>,

    /// Pass `true`, if you require the user's full name to complete the order
    pub need_name: Option<bool>,

    /// Pass `true`, if you require the user's phone number to complete the
    /// order
    pub need_phone_number: Option<bool>,

    /// Pass `true`, if you require the user's email address to complete the
    /// order
    pub need_email: Option<bool>,

    /// Pass `true`, if you require the user's shipping address to complete the
    /// order
    pub need_shipping_address: Option<bool>,

    /// Pass True, if user's phone number should be sent to provider
    pub send_phone_number_to_provider: Option<bool>,

    /// Pass True, if user's email address should be sent to provider
    pub send_email_to_provider: Option<bool>,

    /// Pass True, if the final price depends on the shipping method
    pub is_flexible: Option<bool>,
}

impl InputMessageContentInvoice {
    pub fn new<T, D, PA, PT, C, PR>(
        title: T,
        description: D,
        payload: PA,
        provider_token: PT,
        currency: C,
        prices: PR,
    ) -> Self
    where
        T: Into<String>,
        D: Into<String>,
        PA: Into<String>,
        PT: Into<String>,
        C: Into<String>,
        PR: IntoIterator<Item = LabeledPrice>,
    {
        let title = title.into();
        let description = description.into();
        let payload = payload.into();
        let provider_token = provider_token.into();
        let currency = currency.into();
        let prices = prices.into_iter().collect();

        Self {
            title,
            description,
            payload,
            provider_token,
            currency,
            prices,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
        }
    }

    pub fn title<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.title = val.into();
        self
    }

    pub fn description<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.description = val.into();
        self
    }

    pub fn payload<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.payload = val.into();
        self
    }

    pub fn provider_token<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.provider_token = val.into();
        self
    }

    #[must_use]
    pub fn currency<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.currency = val.into();
        self
    }

    pub fn prices<T>(mut self, val: T) -> Self
    where
        T: IntoIterator<Item = LabeledPrice>,
    {
        self.prices = val.into_iter().collect();
        self
    }

    #[must_use]
    pub fn max_tip_amount(mut self, val: u32) -> Self {
        self.max_tip_amount = Some(val);
        self
    }

    pub fn suggested_tip_amounts<T>(mut self, val: T) -> Self
    where
        T: IntoIterator<Item = u32>,
    {
        self.suggested_tip_amounts = Some(val.into_iter().collect());
        self
    }

    pub fn provider_data<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.provider_data = Some(val.into());
        self
    }

    #[must_use]
    pub fn photo_url(mut self, val: Url) -> Self {
        self.photo_url = Some(val);
        self
    }

    #[must_use]
    pub fn photo_size(mut self, val: u32) -> Self {
        self.photo_size = Some(val);
        self
    }

    #[must_use]
    pub fn photo_width(mut self, val: u32) -> Self {
        self.photo_width = Some(val);
        self
    }

    #[must_use]
    pub fn photo_height(mut self, val: u32) -> Self {
        self.photo_height = Some(val);
        self
    }

    #[must_use]
    pub fn need_name(mut self, val: bool) -> Self {
        self.need_name = Some(val);
        self
    }

    #[must_use]
    pub fn need_phone_number(mut self, val: bool) -> Self {
        self.need_phone_number = Some(val);
        self
    }

    #[must_use]
    pub fn need_email(mut self, val: bool) -> Self {
        self.need_email = Some(val);
        self
    }

    #[must_use]
    pub fn need_shipping_address(mut self, val: bool) -> Self {
        self.need_shipping_address = Some(val);
        self
    }

    #[must_use]
    pub fn send_phone_number_to_provider(mut self, val: bool) -> Self {
        self.send_phone_number_to_provider = Some(val);
        self
    }

    #[must_use]
    pub fn send_email_to_provider(mut self, val: bool) -> Self {
        self.send_email_to_provider = Some(val);
        self
    }

    #[allow(clippy::wrong_self_convention)]
    #[must_use]
    pub fn is_flexible(mut self, val: bool) -> Self {
        self.is_flexible = Some(val);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_serialize() {
        let expected_json =
            r#"{"message_text":"text","link_preview_options":{"is_disabled":true}}"#;
        let text_content = InputMessageContent::Text(InputMessageContentText {
            message_text: String::from("text"),
            parse_mode: None,
            entities: None,
            link_preview_options: Some(LinkPreviewOptions {
                is_disabled: true,
                url: None,
                prefer_small_media: false,
                prefer_large_media: false,
                show_above_text: false,
            }),
        });

        let actual_json = serde_json::to_string(&text_content).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn location_serialize() {
        let expected_json = r#"{"latitude":59.08,"longitude":38.4326}"#;
        let location_content = InputMessageContent::Location(InputMessageContentLocation {
            latitude: 59.08,
            longitude: 38.4326,
            live_period: None,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
        });

        let actual_json = serde_json::to_string(&location_content).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn venue_serialize() {
        let expected_json = r#"{"latitude":59.08,"longitude":38.4326,"title":"some title","address":"some address"}"#;
        let venue_content = InputMessageContent::Venue(InputMessageContentVenue {
            latitude: 59.08,
            longitude: 38.4326,
            title: String::from("some title"),
            address: String::from("some address"),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        });

        let actual_json = serde_json::to_string(&venue_content).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn contact_serialize() {
        let expected_json = r#"{"phone_number":"+3800000000","first_name":"jhon"}"#;
        let contact_content = InputMessageContent::Contact(InputMessageContentContact {
            phone_number: String::from("+3800000000"),
            first_name: String::from("jhon"),
            last_name: None,
            vcard: None,
        });

        let actual_json = serde_json::to_string(&contact_content).unwrap();
        assert_eq!(expected_json, actual_json);
    }
}
