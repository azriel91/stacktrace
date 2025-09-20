use std::borrow::Cow;

use pest::iterators::Pair;

use crate::log_parser::Rule;

/// A Java identifier which must start with a letter or underscore, and
/// subsequently may contain digits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaIdentifier<'s> {
    /// The identifier, which must start with a letter or underscore, and
    /// subsequently may contain digits.
    pub text: Cow<'s, str>,
}

impl<'s> From<Pair<'s, Rule>> for JavaIdentifier<'s> {
    fn from(java_identifier_pair: Pair<'s, Rule>) -> Self {
        let text = Cow::Borrowed(java_identifier_pair.as_str());
        Self { text }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use pest::Parser;

    use crate::{java::JavaIdentifier, log_parser::Rule, LogParser};

    #[test]
    fn parse_java_identifier() {
        let s = "stacktrace";
        match LogParser::parse(Rule::JavaIdentifier, s) {
            Ok(mut java_identifier_pairs) => {
                let java_identifier_pair = java_identifier_pairs
                    .next()
                    .expect("Expected one pair for `JavaClassNameQualified`.");
                let java_identifier = JavaIdentifier::from(java_identifier_pair);
                let java_identifier_expected = JavaIdentifier {
                    text: Cow::Borrowed("stacktrace"),
                };
                assert_eq!(java_identifier_expected, java_identifier);
            }
            Err(e) => {
                eprintln!("Failed to parse `JavaClassNameQualified`: {}", e);
                Err(e).unwrap()
            }
        }
    }
}
