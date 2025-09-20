//! Contains types for semantic logs.

pub use self::log_block::LogBlock;

mod log_block;

/// Logs with semantic information augmented to ease clear presentation.
///
/// Examples of semantic information:
///
/// * Grouping stack frames that belong to the same class / package / module.
///
/// Not implemented yet:
///
/// * Duration between this log message and the previous / next.
/// * Log messages grouped by thread.
/// * Arbitrary grouped log messages.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SemLog<'s> {
    /// Each of the parsed log entries in the log.
    pub log_entries: Vec<LogBlock<'s>>,
}
