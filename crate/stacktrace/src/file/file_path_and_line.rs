use std::borrow::Cow;

use pest::iterators::Pair;

use crate::{file::FilePath, log_parser::Rule};

/// The file path and line number, e.g. `src/main.rs:11`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilePathAndLine<'s> {
    /// The file path and line number, e.g. `src/main.rs:11`.
    pub full_text: Cow<'s, str>,
    /// The file path, e.g. `src/main.rs`.
    pub file_path: FilePath<'s>,
    /// The line number, e.g. `11`.
    pub line_number: u32,
}

impl<'s> From<Pair<'s, Rule>> for FilePathAndLine<'s> {
    fn from(file_path_and_line_pair: Pair<'s, Rule>) -> Self {
        let full_text = Cow::Borrowed(file_path_and_line_pair.as_str());
        let (file_path, line_number) = file_path_and_line_pair.into_inner().fold(
            (None, 0),
            |(mut file_path, mut line_number), file_path_and_line_pair_inner| {
                match file_path_and_line_pair_inner.as_rule() {
                    Rule::FilePath => {
                        let file_path_pair = file_path_and_line_pair_inner;
                        file_path = Some(FilePath::from(file_path_pair));

                        (file_path, line_number)
                    }
                    Rule::LineNumber => {
                        let line_number_pair = file_path_and_line_pair_inner;
                        line_number = line_number_pair
                            .as_str()
                            .parse::<u32>()
                            .expect("Failed to parse line number");

                        (file_path, line_number)
                    }
                    _ => unreachable!(),
                }
            },
        );

        let file_path = file_path.expect("Expected `FilePath` to exist after parsing.");

        Self {
            full_text,
            file_path,
            line_number,
        }
    }
}
