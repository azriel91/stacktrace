use std::borrow::Cow;

/// A parsed stacktrace
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogBlockNormal<'s> {
    /// The full text of the log block.
    pub text: Cow<'s, str>,
}
