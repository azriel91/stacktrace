use std::borrow::Cow;

use pest::iterators::Pair;

use crate::{log::java::JavaClassNameSegment, log_parser::Rule};

/// The simple class name, e.g. `Example$Exception$0` in
/// `com.example.stacktrace.Example$Exception$0`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaClassNameSimple<'s> {
    /// All of the class name segments, e.g. `Example$Exception$0` in
    /// `com.example.stacktrace.Example$Exception$0`.
    pub full_text: Cow<'s, str>,
    /// Each of the class name segments, e.g. `Example`, `Exception`, and `0` in
    /// `com.example.stacktrace.Example$Exception$0`.
    pub segments: Vec<JavaClassNameSegment<'s>>,
}

impl<'s> From<Pair<'s, Rule>> for JavaClassNameSimple<'s> {
    fn from(java_class_name_simple_pair: Pair<'s, Rule>) -> Self {
        let full_text = Cow::Borrowed(java_class_name_simple_pair.as_str());
        let segments = java_class_name_simple_pair.into_inner().fold(
            Vec::new(),
            |mut segments, java_class_name_simple_pair_inner| {
                match java_class_name_simple_pair_inner.as_rule() {
                    Rule::JavaClassNameSegment => {
                        let class_name_segment_pair = java_class_name_simple_pair_inner;
                        let class_name_segment =
                            JavaClassNameSegment::from(class_name_segment_pair);
                        segments.push(class_name_segment);

                        segments
                    }
                    _ => unreachable!(),
                }
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
        log::java::{JavaClassNameSegment, JavaClassNameSimple},
        log_parser::Rule,
        LogParser,
    };

    #[test]
    fn parse_java_class_name_simple() {
        let s = "Example";
        match LogParser::parse(Rule::JavaClassNameSimple, s) {
            Ok(mut java_class_name_simple_pairs) => {
                let java_class_name_simple_pair = java_class_name_simple_pairs
                    .next()
                    .expect("Expected one pair for `JavaClassNameSimple`.");
                let java_class_name_simple = JavaClassNameSimple::from(java_class_name_simple_pair);
                let java_class_name_simple_expected = JavaClassNameSimple {
                    full_text: Cow::Borrowed("Example"),
                    segments: vec![JavaClassNameSegment {
                        text: Cow::Borrowed("Example"),
                    }],
                };
                assert_eq!(java_class_name_simple_expected, java_class_name_simple);
            }
            Err(e) => {
                eprintln!("Failed to parse `JavaClassNameSimple`: {}", e);
                Err(e).unwrap()
            }
        }
    }
}
