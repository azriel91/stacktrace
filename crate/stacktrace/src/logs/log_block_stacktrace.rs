use pest::iterators::Pair;

use crate::{java::JavaStacktrace, logs_parser::Rule};

/// A parsed stacktrace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogBlockStacktrace<'s> {
    /// A stacktrace from a Java program.
    JavaStack(JavaStacktrace<'s>),
}

impl<'s> From<Pair<'s, Rule>> for LogBlockStacktrace<'s> {
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
