//! Unicode-specific helpers.
//!
//! v0.4.0 introduces this parent module as the canonical home for Unicode
//! domains. The legacy top-level `veltrix::emojis` module remains available
//! behind the `emojis` feature during the transition.

#[cfg(feature = "unicode-emojis")]
/// Emoji constants and lookup helpers.
pub mod emojis;

#[cfg(all(test, feature = "unicode-emojis"))]
mod tests {
    #[test]
    fn unicode_emojis_reexports_generated_data() {
        let emoji = super::emojis::find_by_search_term("smile").expect("smile exists");

        assert_eq!(emoji.unicode_version, super::emojis::UNICODE_EMOJI_VERSION);
    }
}
