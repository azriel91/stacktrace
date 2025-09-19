use crate::LogBlock;

/// A structured set of logs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Logs<'s> {
    /// Each of the parsed log blocks in the log.
    pub log_blocks: Vec<LogBlock<'s>>,
}
