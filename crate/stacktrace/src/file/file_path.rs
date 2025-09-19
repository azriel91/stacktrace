use std::borrow::Cow;

use pest::iterators::Pair;

use crate::logs_parser::Rule;

/// Path to a source file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilePath<'s> {
    /// The file path, e.g. `src/main.rs`.
    pub text: Cow<'s, str>,
}

impl<'s> From<Pair<'s, Rule>> for FilePath<'s> {
    fn from(file_path_pair: Pair<'s, Rule>) -> Self {
        let text = Cow::Borrowed(file_path_pair.as_str());

        Self { text }
    }
}
