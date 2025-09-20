use std::borrow::Cow;

use pest::iterators::Pair;

use crate::{
    log::{LogEntryNormal, LogEntryStacktrace},
    log_parser::Rule,
    sem_log::IntoLogBlock,
};

/// A entry in the log file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogEntry<'s> {
    /// A stacktrace entry.
    Stacktrace(LogEntryStacktrace<'s>),
    /// A regular message entry.
    Normal(LogEntryNormal<'s>),
}

impl<'s> From<Pair<'s, Rule>> for LogEntry<'s> {
    fn from(log_entry_pair: Pair<'s, Rule>) -> Self {
        match log_entry_pair.as_rule() {
            Rule::LogEntryStacktrace => {
                let log_entry_stacktrace_pair = log_entry_pair
                    .into_inner()
                    .next()
                    .expect("Expected exactly one `LogEntryStacktrace` inner pair.");
                let log_entry_stacktrace = LogEntryStacktrace::from(log_entry_stacktrace_pair);
                LogEntry::Stacktrace(log_entry_stacktrace)
            }
            Rule::LogEntryNormal => {
                let log_entry_normal = LogEntryNormal {
                    text: Cow::Borrowed(log_entry_pair.as_str()),
                };
                LogEntry::Normal(log_entry_normal)
            }
            rule => unreachable!("Unexpected rule: {rule:?}"),
        }
    }
}

impl<'s> IntoLogBlock<'s> for LogEntry<'s> {
    fn into_log_block(self) -> crate::sem_log::LogBlock<'s> {
        match self {
            Self::Stacktrace(log_entry_stacktrace) => log_entry_stacktrace.into_log_block(),
            Self::Normal(log_entry_normal) => log_entry_normal.into_log_block(),
        }
    }
}
