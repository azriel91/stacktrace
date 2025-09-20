use std::borrow::Cow;

use pest::iterators::Pair;

use crate::log_parser::Rule;

/// A Java identifier which must start with a letter or underscore, and
/// subsequently may contain digits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaStacktraceHeaderMessage<'s> {
    /// The text after the exception.
    pub text: Cow<'s, str>,
}

impl<'s> From<Pair<'s, Rule>> for JavaStacktraceHeaderMessage<'s> {
    fn from(java_stacktrace_header_message_pair: Pair<'s, Rule>) -> Self {
        let text = Cow::Borrowed(java_stacktrace_header_message_pair.as_str());

        Self { text }
    }
}
