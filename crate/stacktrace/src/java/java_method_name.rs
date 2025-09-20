use pest::iterators::Pair;

use crate::{java::JavaIdentifier, log_parser::Rule};

/// The method name excluding the `()` in a Java stacktrace.
///
/// e.g. `"fail"` in `com.example.stacktrace.Example.fail(Example.java:11)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaMethodName<'s> {
    /// The method name, e.g. `"fail"` in
    /// `com.example.stacktrace.Example.fail(Example.java:11)`.
    pub identifier: JavaIdentifier<'s>,
}

impl<'s> From<Pair<'s, Rule>> for JavaMethodName<'s> {
    fn from(java_method_name_pair: Pair<'s, Rule>) -> Self {
        let identifier = java_method_name_pair
            .into_inner()
            .next()
            .map(
                |java_method_name_pair_inner| match java_method_name_pair_inner.as_rule() {
                    Rule::JavaIdentifier => {
                        let java_identifier_pair = java_method_name_pair_inner;
                        JavaIdentifier::from(java_identifier_pair)
                    }
                    _ => unreachable!(),
                },
            )
            .expect("Expected exactly one `JavaIdentifier` pair under `JavaMethodName`.");

        Self { identifier }
    }
}
