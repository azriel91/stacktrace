use std::borrow::Cow;

use pest::iterators::Pair;

use crate::{
    log::{LogBlockNormal, LogBlockStacktrace},
    log_parser::Rule,
};

/// A block in the log file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogBlock<'s> {
    /// A stacktrace block.
    Stacktrace(LogBlockStacktrace<'s>),
    /// A regular message block.
    Normal(LogBlockNormal<'s>),
}

impl<'s> From<Pair<'s, Rule>> for LogBlock<'s> {
    fn from(log_block_pair: Pair<'s, Rule>) -> Self {
        match log_block_pair.as_rule() {
            Rule::LogBlockStacktrace => {
                let log_block_stacktrace_pair = log_block_pair
                    .into_inner()
                    .next()
                    .expect("Expected exactly one `LogBlockStacktrace` inner pair.");
                let log_block_stacktrace = LogBlockStacktrace::from(log_block_stacktrace_pair);
                LogBlock::Stacktrace(log_block_stacktrace)
            }
            Rule::LogBlockNormal => {
                let log_block_normal = LogBlockNormal {
                    text: Cow::Borrowed(log_block_pair.as_str()),
                };
                LogBlock::Normal(log_block_normal)
            }
            rule => unreachable!("Unexpected rule: {rule:?}"),
        }
    }
}
