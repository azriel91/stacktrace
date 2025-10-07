use pest::iterators::Pair;

use crate::{log::java::JavaStacktrace, log_parser::Rule, sem_log::IntoLogBlock};

/// A parsed stacktrace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogEntryStacktrace<'s> {
    /// A stacktrace from a Java program.
    JavaStack(JavaStacktrace<'s>),
}

impl<'s> From<Pair<'s, Rule>> for LogEntryStacktrace<'s> {
    fn from(log_block_stacktrace_pair: Pair<'s, Rule>) -> Self {
        match log_block_stacktrace_pair.as_rule() {
            Rule::JavaStacktrace => {
                let java_stacktrace_pair = log_block_stacktrace_pair;
                let java_stacktrace = JavaStacktrace::from(java_stacktrace_pair);
                Self::JavaStack(java_stacktrace)
            }
            _ => unreachable!(),
        }
    }
}

impl<'s> IntoLogBlock<'s> for LogEntryStacktrace<'s> {
    fn into_log_block(self) -> crate::sem_log::LogBlock<'s> {
        match self {
            Self::JavaStack(java_stacktrace) => java_stacktrace.into_log_block(),
        }
    }
}
