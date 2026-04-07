// src/mime.rs

/// A known [IANA media type](https://www.iana.org/assignments/media-types/media-types.xhtml).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mime {
    /// The group to which the media type belongs.
    /// E.g. `application`
    /// This is the segment before the `/`
    pub top: TopLevel,

    /// The subgroup of the media type.
    /// E.g. `json`
    /// This is the segment after the `/`
    pub sub: &'static str,

    /// The suffic appended to the media type.
    /// E.g. `xml`
    /// This is the `+<suffix>` segment of the media type
    pub suffix: Option<&'static str>,
}

/// An enum representing the different top level media types as defined in [IANA: RFC9694 - Section 4.2](https://datatracker.ietf.org/doc/html/rfc9694)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLevel {
    /// Application media types.
    /// See [IANA RFC 2046](https://datatracker.ietf.org/doc/html/rfc2046)
    Application,

    /// Audio media types.
    /// See [IANA RFC 2046](https://datatracker.ietf.org/doc/html/rfc2046)
    Audio,

    /// Font media types.
    /// See [IANA RFC 8081](https://datatracker.ietf.org/doc/html/rfc8081)
    Font,

    /// Haptics media types.
    /// See [IANA RFC 9695](https://datatracker.ietf.org/doc/html/rfc9695)
    Haptics,

    /// Image media types.
    /// See [IANA RFC 2046](https://datatracker.ietf.org/doc/html/rfc2046)
    Image,

    /// Message media types.
    /// See [IANA RFC 2046](https://datatracker.ietf.org/doc/html/rfc2046)
    Message,

    /// Model media types.
    /// See [IANA RFC 2077](https://datatracker.ietf.org/doc/html/rfc2077)
    Model,

    /// Multipart media types.
    /// See [IANA RFC 2046](https://datatracker.ietf.org/doc/html/rfc2046)
    Multipart,

    /// Text media types - Requires CRLF for newlines.
    /// See [IANA RFC 2046](https://datatracker.ietf.org/doc/html/rfc2046)
    Text,

    /// Video media types.
    /// See [IANA RFC 2046](https://datatracker.ietf.org/doc/html/rfc2046)
    Video,
}

impl TopLevel {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Application => "application",
            Self::Audio => "audio",
            Self::Font => "font",
            Self::Haptics => "haptics",
            Self::Image => "image",
            Self::Message => "message",
            Self::Model => "model",
            Self::Multipart => "multipart",
            Self::Text => "text",
            Self::Video => "video",
        }
    }
}

impl Mime {
    pub const fn type_str(&self) -> &'static str {
        self.top.as_str()
    }

    pub const fn subtype(&self) -> &'static str {
        self.sub
    }

    pub const fn suffix(&self) -> Option<&'static str> {
        self.suffix
    }
}

impl core::fmt::Display for TopLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl core::fmt::Display for Mime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}/{}", self.top, self.sub)?;
        if let Some(suffix) = self.suffix {
            write!(f, "+{}", suffix)?;
        }
        Ok(())
    }
}
