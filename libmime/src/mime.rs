// src/mime.rs

/// A known IANA media type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mime {
    pub top: TopLevel,
    pub sub: &'static str,
    pub suffix: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TopLevel {
    Application,
    Audio,
    Font,
    Haptics,
    Image,
    Message,
    Model,
    Multipart,
    Text,
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
    /// The full essence string at compile time isn't possible,
    /// but we can reconstruct it cheaply.
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
