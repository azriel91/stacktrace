use std::borrow::Cow;

use pest::iterators::Pair;

use crate::{java::JavaPackageSegment, logs_parser::Rule};

/// A qualified Java package, e.g. `com.example.stacktrace`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaPackage<'s> {
    /// The full text of the package, e.g. `com.example.stacktrace`.
    pub full_text: Cow<'s, str>,
    /// Each package segment, e.g. `com`, `example`, and `stacktrace` in
    /// `com.example.stacktrace`.
    pub segments: Vec<JavaPackageSegment<'s>>,
}

impl<'s> From<Pair<'s, Rule>> for JavaPackage<'s> {
    fn from(java_package_pair: Pair<'s, Rule>) -> Self {
        let full_text = Cow::Borrowed(java_package_pair.as_str());
        let segments = java_package_pair.into_inner().fold(
            Vec::new(),
            |mut segments, java_package_pair_inner| match java_package_pair_inner.as_rule() {
                Rule::JavaPackageSegment => {
                    let package_segment_pair = java_package_pair_inner;
                    let package_segment = JavaPackageSegment::from(package_segment_pair);
                    segments.push(package_segment);

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
        java::{JavaIdentifierLower, JavaPackage, JavaPackageSegment},
        logs_parser::Rule,
        LogParser,
    };

    #[test]
    fn parse_java_package() {
        let s = "com.example.stacktrace";
        match LogParser::parse(Rule::JavaPackage, s) {
            Ok(mut java_package_pairs) => {
                let java_package_pair = java_package_pairs
                    .next()
                    .expect("Expected one pair for `JavaPackage`.");
                let java_package = JavaPackage::from(java_package_pair);
                let java_package_expected = JavaPackage {
                    full_text: Cow::Borrowed("com.example.stacktrace"),
                    segments: vec![
                        JavaPackageSegment {
                            identifier: JavaIdentifierLower {
                                text: Cow::Borrowed("com"),
                            },
                        },
                        JavaPackageSegment {
                            identifier: JavaIdentifierLower {
                                text: Cow::Borrowed("example"),
                            },
                        },
                        JavaPackageSegment {
                            identifier: JavaIdentifierLower {
                                text: Cow::Borrowed("stacktrace"),
                            },
                        },
                    ],
                };
                assert_eq!(java_package_expected, java_package);
            }
            Err(e) => {
                eprintln!("Failed to parse `JavaPackage`: {}", e);
                Err(e).unwrap()
            }
        }
    }
}
