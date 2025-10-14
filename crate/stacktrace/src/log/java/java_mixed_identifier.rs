use std::borrow::Cow;

use pest::iterators::Pair;

use crate::{log::java::JavaIdentifier, log_parser::Rule};

/// A segment in a Java method reference, which could be a package segment,
/// class, or method name.
///
/// i.e. any of the segments in
/// `com.example.stacktrace.Example$0$1.<init>(Example.java:11)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaMixedIdentifier<'s> {
    /// The full text of the identifier, e.g. `"<init>"`.
    pub full_text: Cow<'s, str>,
    /// The opening angle bracket, e.g. `"<"`, if any.
    pub angle_open: Cow<'s, str>,
    /// Any of the segments in
    /// `com.example.stacktrace.Example$0$1.<init>(Example.java:11)`.
    pub identifier: JavaIdentifier<'s>,
    /// The closing angle bracket, e.g. `">"`, if any.
    pub angle_close: Cow<'s, str>,
}

impl<'s> From<Pair<'s, Rule>> for JavaMixedIdentifier<'s> {
    fn from(java_mixed_identifier_pair: Pair<'s, Rule>) -> Self {
        let full_text = Cow::Borrowed(java_mixed_identifier_pair.as_str());
        let (angle_open, identifier, angle_close) = java_mixed_identifier_pair.into_inner().fold(
            (None, None, None),
            |(mut angle_open, mut identifier, mut angle_close),
             java_mixed_identifier_pair_inner| {
                match java_mixed_identifier_pair_inner.as_rule() {
                    Rule::CONST_ANGLE_OPEN => {
                        angle_open = Some(Cow::Borrowed(java_mixed_identifier_pair_inner.as_str()));

                        (angle_open, identifier, angle_close)
                    }
                    Rule::JavaIdentifier => {
                        let java_identifier_pair = java_mixed_identifier_pair_inner;
                        identifier = Some(JavaIdentifier::from(java_identifier_pair));

                        (angle_open, identifier, angle_close)
                    }
                    Rule::CONST_ANGLE_CLOSE => {
                        angle_close =
                            Some(Cow::Borrowed(java_mixed_identifier_pair_inner.as_str()));

                        (angle_open, identifier, angle_close)
                    }
                    _ => unreachable!(),
                }
            },
        );

        let angle_open = angle_open.unwrap_or(Cow::Borrowed(""));
        let identifier = identifier.expect("Expected `JavaIdentifier` to exist after parsing.");
        let angle_close = angle_close.unwrap_or(Cow::Borrowed(""));

        Self {
            full_text,
            angle_open,
            identifier,
            angle_close,
        }
    }
}
