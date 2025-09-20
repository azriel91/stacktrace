use std::borrow::Cow;

use pest::iterators::Pair;

use crate::log_parser::Rule;

/// A Java identifier which must start with a letter or underscore, and
/// subsequently may contain digits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaIdentifierLower<'s> {
    /// The identifier, which must start with a letter or underscore, and
    /// subsequently may contain digits.
    pub text: Cow<'s, str>,
}

impl<'s> From<Pair<'s, Rule>> for JavaIdentifierLower<'s> {
    fn from(java_identifier_pair: Pair<'s, Rule>) -> Self {
        let text = Cow::Borrowed(java_identifier_pair.as_str());
        Self { text }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use pest::Parser;

    use crate::{java::JavaIdentifierLower, log_parser::Rule, LogParser};

    #[test]
    fn parse_java_identifier_lower() {
        let s = "stacktrace";
        match LogParser::parse(Rule::JavaIdentifierLower, s) {
            Ok(mut java_identifier_lower_pairs) => {
                let java_identifier_lower_pair = java_identifier_lower_pairs
                    .next()
                    .expect("Expected one pair for `JavaClassNameQualified`.");
                let java_identifier_lower = JavaIdentifierLower::from(java_identifier_lower_pair);
                let java_identifier_lower_expected = JavaIdentifierLower {
                    text: Cow::Borrowed("stacktrace"),
                };
                assert_eq!(java_identifier_lower_expected, java_identifier_lower);
            }
            Err(e) => {
                eprintln!("Failed to parse `JavaIdentifierLower`: {}", e);
                Err(e).unwrap()
            }
        }
    }
}
