use std::borrow::Cow;

use pest::iterators::Pair;

use crate::{
    log::java::{JavaClassNameSimple, JavaPackage},
    log_parser::Rule,
};

/// The qualified class name, e.g.
/// `com.example.stacktrace.Example$Exception$0` in
/// `com.example.stacktrace.Example$Exception$0`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaClassNameQualified<'s> {
    /// The qualified class name, e.g.
    /// `com.example.stacktrace.Example$Exception$0` in
    /// `com.example.stacktrace.Example$Exception$0`.
    pub full_text: Cow<'s, str>,
    /// The qualified package name, e.g. `com.example.stacktrace` in
    /// `com.example.stacktrace.Example$Exception$0`.
    pub package: JavaPackage<'s>,
    /// The simple class name, e.g. `Example$Exception$0` in
    /// `com.example.stacktrace.Example$Exception$0`.
    pub class_name_simple: JavaClassNameSimple<'s>,
}

impl<'s> From<Pair<'s, Rule>> for JavaClassNameQualified<'s> {
    fn from(java_class_name_qualified_pair: Pair<'s, Rule>) -> Self {
        let full_text = Cow::Borrowed(java_class_name_qualified_pair.as_str());
        let (package, class_name_simple) = java_class_name_qualified_pair.into_inner().fold(
            (None, None),
            |(mut package, mut class_name_simple), java_class_name_qualified_pair_inner| {
                match java_class_name_qualified_pair_inner.as_rule() {
                    Rule::JavaPackage => {
                        let package_pair = java_class_name_qualified_pair_inner;
                        package = Some(JavaPackage::from(package_pair));

                        (package, class_name_simple)
                    }
                    Rule::JavaClassNameSimple => {
                        let class_name_simple_pair = java_class_name_qualified_pair_inner;
                        class_name_simple = Some(JavaClassNameSimple::from(class_name_simple_pair));

                        (package, class_name_simple)
                    }
                    _ => unreachable!(),
                }
            },
        );

        let package = package.expect("Expected `JavaPackage` to exist after parsing.");
        let class_name_simple =
            class_name_simple.expect("Expected `JavaClassNameSimple` to exist after parsing.");

        Self {
            full_text,
            package,
            class_name_simple,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use pest::Parser;

    use crate::{
        log::java::{
            JavaClassNameQualified, JavaClassNameSegment, JavaClassNameSimple, JavaIdentifierLower,
            JavaPackage, JavaPackageSegment,
        },
        log_parser::Rule,
        LogParser,
    };

    #[test]
    fn parse_java_class_name_qualified() {
        let s = "com.example.stacktrace.Example";
        match LogParser::parse(Rule::JavaClassNameQualified, s) {
            Ok(mut java_class_name_qualified_pairs) => {
                let java_class_name_qualified_pair = java_class_name_qualified_pairs
                    .next()
                    .expect("Expected one pair for `JavaClassNameQualified`.");
                let java_class_name_qualified =
                    JavaClassNameQualified::from(java_class_name_qualified_pair);
                let java_class_name_qualified_expected = JavaClassNameQualified {
                    full_text: Cow::Borrowed("com.example.stacktrace.Example"),
                    package: JavaPackage {
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
                    },
                    class_name_simple: JavaClassNameSimple {
                        full_text: Cow::Borrowed("Example"),
                        segments: vec![JavaClassNameSegment {
                            text: Cow::Borrowed("Example"),
                        }],
                    },
                };
                assert_eq!(
                    java_class_name_qualified_expected,
                    java_class_name_qualified
                );
            }
            Err(e) => {
                eprintln!("Failed to parse `JavaClassNameQualified`: {}", e);
                Err(e).unwrap()
            }
        }
    }
}
