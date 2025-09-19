use pest::iterators::Pair;

use crate::{java::JavaIdentifierLower, logs_parser::Rule};

/// One package segment, e.g. `example` in `com.example.stacktrace`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaPackageSegment<'s> {
    /// One package segment, e.g. `example` in `com.example.stacktrace`.
    pub identifier: JavaIdentifierLower<'s>,
}

impl<'s> From<Pair<'s, Rule>> for JavaPackageSegment<'s> {
    fn from(java_package_segment_pair: Pair<'s, Rule>) -> Self {
        let identifier = java_package_segment_pair
            .into_inner()
            .next()
            .map(|java_package_segment_pair_inner| {
                match java_package_segment_pair_inner.as_rule() {
                    Rule::JavaIdentifierLower => {
                        let java_identifier_pair = java_package_segment_pair_inner;
                        JavaIdentifierLower::from(java_identifier_pair)
                    }
                    _ => unreachable!(),
                }
            })
            .expect("Expected exactly one `JavaIdentifierLower` pair under `JavaPackageSegment`.");

        Self { identifier }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use pest::Parser;

    use crate::{
        java::{JavaIdentifierLower, JavaPackageSegment},
        logs_parser::Rule,
        LogParser,
    };

    #[test]
    fn parse_java_package_segment() {
        let s = "stacktrace";
        match LogParser::parse(Rule::JavaPackageSegment, s) {
            Ok(mut java_package_segment_pairs) => {
                let java_package_segment_pair = java_package_segment_pairs
                    .next()
                    .expect("Expected one pair for `JavaPackageSegment`.");
                let java_package_segment = JavaPackageSegment::from(java_package_segment_pair);
                let java_package_segment_expected = JavaPackageSegment {
                    identifier: JavaIdentifierLower {
                        text: Cow::Borrowed("stacktrace"),
                    },
                };
                assert_eq!(java_package_segment_expected, java_package_segment);
            }
            Err(e) => {
                eprintln!("Failed to parse `JavaPackageSegment`: {}", e);
                Err(e).unwrap()
            }
        }
    }
}
