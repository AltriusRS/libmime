// src/mime.rs

use crate::TopLevel;

pub mod toplevel;

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

    fn matches_str(&self, s: &str) -> bool {
        let Some((top, rest)) = s.split_once('/') else {
            return false;
        };

        if !top.eq_ignore_ascii_case(self.top.as_str()) {
            return false;
        }

        match self.suffix {
            Some(expected_suffix) => {
                let Some((sub, suffix)) = rest.rsplit_once('+') else {
                    return false;
                };

                sub.eq_ignore_ascii_case(self.sub) && suffix.eq_ignore_ascii_case(expected_suffix)
            }
            None => rest.eq_ignore_ascii_case(self.sub),
        }
    }

    pub fn eq_str(&self, s: impl AsRef<str>) -> bool {
        self.matches_str(s.as_ref())
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

impl PartialEq<str> for Mime {
    fn eq(&self, other: &str) -> bool {
        self.matches_str(other)
    }
}

impl PartialEq<&str> for Mime {
    fn eq(&self, other: &&str) -> bool {
        self.matches_str(other)
    }
}

impl PartialEq<String> for Mime {
    fn eq(&self, other: &String) -> bool {
        self.matches_str(other.as_str())
    }
}
