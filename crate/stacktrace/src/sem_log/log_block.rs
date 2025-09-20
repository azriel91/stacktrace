use crate::sem_log::{LogBlockNormal, LogBlockStacktrace};

/// A block in the log file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogBlock<'s> {
    /// A stacktrace block.
    Stacktrace(LogBlockStacktrace<'s>),
    /// A regular message block.
    Normal(LogBlockNormal<'s>),
}
