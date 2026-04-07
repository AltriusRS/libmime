#[cfg(test)]
mod tests {
    use crate::{
        lookup, Mime, TopLevel, APPLICATION_JSON, APPLICATION_VND_API_JSON, IMAGE_SVG_XML,
        TEXT_HTML, TEXT_PLAIN,
    };

    #[test]
    fn top_level_as_str_returns_expected_values() {
        assert_eq!(TopLevel::Application.as_str(), "application");
        assert_eq!(TopLevel::Audio.as_str(), "audio");
        assert_eq!(TopLevel::Font.as_str(), "font");
        assert_eq!(TopLevel::Haptics.as_str(), "haptics");
        assert_eq!(TopLevel::Image.as_str(), "image");
        assert_eq!(TopLevel::Message.as_str(), "message");
        assert_eq!(TopLevel::Model.as_str(), "model");
        assert_eq!(TopLevel::Multipart.as_str(), "multipart");
        assert_eq!(TopLevel::Text.as_str(), "text");
        assert_eq!(TopLevel::Video.as_str(), "video");
    }

    #[test]
    fn top_level_display_matches_as_str() {
        assert_eq!(TopLevel::Application.to_string(), "application");
        assert_eq!(TopLevel::Haptics.to_string(), "haptics");
        assert_eq!(TopLevel::Video.to_string(), "video");
    }

    #[test]
    fn mime_accessors_return_expected_values() {
        let mime = Mime {
            top: TopLevel::Application,
            sub: "json",
            suffix: None,
        };

        assert_eq!(mime.type_str(), "application");
        assert_eq!(mime.subtype(), "json");
        assert_eq!(mime.suffix(), None);
    }

    #[test]
    fn mime_display_without_suffix() {
        let mime = Mime {
            top: TopLevel::Text,
            sub: "plain",
            suffix: None,
        };

        assert_eq!(mime.to_string(), "text/plain");
    }

    #[test]
    fn mime_display_with_suffix() {
        let mime = Mime {
            top: TopLevel::Application,
            sub: "vnd.api",
            suffix: Some("json"),
        };

        assert_eq!(mime.to_string(), "application/vnd.api+json");
    }

    #[test]
    fn generated_constants_display_correctly() {
        assert_eq!(APPLICATION_JSON.to_string(), "application/json");
        assert_eq!(TEXT_PLAIN.to_string(), "text/plain");
        assert_eq!(TEXT_HTML.to_string(), "text/html");
        assert_eq!(IMAGE_SVG_XML.to_string(), "image/svg+xml");
        assert_eq!(
            APPLICATION_VND_API_JSON.to_string(),
            "application/vnd.api+json"
        );
    }

    #[test]
    fn generated_constants_have_expected_fields() {
        assert_eq!(APPLICATION_JSON.top, TopLevel::Application);
        assert_eq!(APPLICATION_JSON.sub, "json");
        assert_eq!(APPLICATION_JSON.suffix, None);

        assert_eq!(IMAGE_SVG_XML.top, TopLevel::Image);
        assert_eq!(IMAGE_SVG_XML.sub, "svg");
        assert_eq!(IMAGE_SVG_XML.suffix, Some("xml"));
    }

    #[test]
    fn lookup_finds_known_type_without_suffix() {
        let mime = lookup("application/json");
        assert_eq!(mime, Some(APPLICATION_JSON));
    }

    #[test]
    fn lookup_finds_known_type_with_suffix() {
        let mime = lookup("application/vnd.api+json");
        assert_eq!(mime, Some(APPLICATION_VND_API_JSON));
    }

    #[test]
    fn lookup_is_case_insensitive() {
        assert_eq!(lookup("application/json"), Some(APPLICATION_JSON));
        assert_eq!(lookup("Application/Json"), Some(APPLICATION_JSON));
        assert_eq!(lookup("APPLICATION/JSON"), Some(APPLICATION_JSON));
    }

    #[test]
    fn lookup_returns_none_for_unknown_type() {
        assert_eq!(lookup("application/not-real"), None);
        assert_eq!(lookup("example/example"), None);
    }

    #[test]
    fn lookup_returns_none_for_invalid_strings() {
        assert_eq!(lookup(""), None);
        assert_eq!(lookup("not a mime"), None);
        assert_eq!(lookup("application"), None);
        assert_eq!(lookup("/json"), None);
    }

    #[test]
    fn mime_is_copy() {
        let a = APPLICATION_JSON;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn mime_equality_works() {
        let a = Mime {
            top: TopLevel::Text,
            sub: "plain",
            suffix: None,
        };
        let b = Mime {
            top: TopLevel::Text,
            sub: "plain",
            suffix: None,
        };
        let c = Mime {
            top: TopLevel::Text,
            sub: "html",
            suffix: None,
        };

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn suffix_accessor_works_for_suffixed_and_unsuffixed_types() {
        assert_eq!(APPLICATION_JSON.suffix(), None);
        assert_eq!(APPLICATION_VND_API_JSON.suffix(), Some("json"));
        assert_eq!(IMAGE_SVG_XML.suffix(), Some("xml"));
    }
}
