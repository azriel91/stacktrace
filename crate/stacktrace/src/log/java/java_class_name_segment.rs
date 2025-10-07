use std::borrow::Cow;

use pest::iterators::Pair;

use crate::log_parser::Rule;

/// One class name segment, e.g. `Example`, `Exception`, or `$0` in
/// `com.example.stacktrace.Example$Exception$0`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaClassNameSegment<'s> {
    /// One class name segment, e.g. `Example`, `Exception`, or `$0` in
    /// `com.example.stacktrace.Example$Exception$0`.
    pub text: Cow<'s, str>,
}

impl<'s> From<Pair<'s, Rule>> for JavaClassNameSegment<'s> {
    fn from(java_class_name_segment_pair: Pair<'s, Rule>) -> Self {
        let text = Cow::Borrowed(java_class_name_segment_pair.as_str());

        Self { text }
    }
}
