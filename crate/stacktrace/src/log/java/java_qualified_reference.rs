use std::borrow::Cow;

use pest::iterators::Pair;

use crate::{log::java::JavaMixedIdentifier, log_parser::Rule};

/// A qualified Java reference, e.g. `com.example.stacktrace._Class$0$1.<init>`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaQualifiedReference<'s> {
    /// The full text of the reference, e.g.
    /// `com.example.stacktrace._Class$0$1.<init>`.
    pub full_text: Cow<'s, str>,
    /// Each segment, e.g. `com`, `_Class$0$1`, and `<init>`.
    pub segments: Vec<JavaMixedIdentifier<'s>>,
}

impl<'s> From<Pair<'s, Rule>> for JavaQualifiedReference<'s> {
    fn from(java_package_pair: Pair<'s, Rule>) -> Self {
        let full_text = Cow::Borrowed(java_package_pair.as_str());
        let segments = java_package_pair.into_inner().fold(
            Vec::new(),
            |mut segments, java_package_pair_inner| match java_package_pair_inner.as_rule() {
                Rule::JavaMixedIdentifier => {
                    let java_identifier_pair = java_package_pair_inner;
                    let java_identifier = JavaMixedIdentifier::from(java_identifier_pair);
                    segments.push(java_identifier);

                    segments
                }
                _ => unreachable!(),
            },
        );

        Self {
            full_text,
            segments,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use pest::Parser;

    use crate::{
        log::java::{JavaIdentifier, JavaMixedIdentifier, JavaQualifiedReference},
        log_parser::Rule,
        LogParser,
    };

    #[test]
    fn parse_java_package() {
        let s = "com.example.stacktrace._Class$0$1.<init>";
        match LogParser::parse(Rule::JavaQualifiedReference, s) {
            Ok(mut java_package_pairs) => {
                let java_package_pair = java_package_pairs
                    .next()
                    .expect("Expected one pair for `JavaQualifiedReference`.");
                let java_package = JavaQualifiedReference::from(java_package_pair);
                let java_package_expected = JavaQualifiedReference {
                    full_text: Cow::Borrowed("com.example.stacktrace._Class$0$1.<init>"),
                    segments: vec![
                        JavaMixedIdentifier {
                            angle_open: Cow::Borrowed(""),
                            identifier: JavaIdentifier {
                                text: Cow::Borrowed("com"),
                            },
                            angle_close: Cow::Borrowed(""),
                        },
                        JavaMixedIdentifier {
                            angle_open: Cow::Borrowed(""),
                            identifier: JavaIdentifier {
                                text: Cow::Borrowed("example"),
                            },
                            angle_close: Cow::Borrowed(""),
                        },
                        JavaMixedIdentifier {
                            angle_open: Cow::Borrowed(""),
                            identifier: JavaIdentifier {
                                text: Cow::Borrowed("stacktrace"),
                            },
                            angle_close: Cow::Borrowed(""),
                        },
                        JavaMixedIdentifier {
                            angle_open: Cow::Borrowed(""),
                            identifier: JavaIdentifier {
                                text: Cow::Borrowed("_Class$0$1"),
                            },
                            angle_close: Cow::Borrowed(""),
                        },
                        JavaMixedIdentifier {
                            angle_open: Cow::Borrowed("<"),
                            identifier: JavaIdentifier {
                                text: Cow::Borrowed("init"),
                            },
                            angle_close: Cow::Borrowed(">"),
                        },
                    ],
                };
                assert_eq!(java_package_expected, java_package);
            }
            Err(e) => {
                eprintln!("Failed to parse `JavaQualifiedReference`: {}", e);
                Err(e).unwrap()
            }
        }
    }
}
