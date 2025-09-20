//! Contains types for semantic logs.

pub use self::{
    log_block::LogBlock, log_block_normal::LogBlockNormal, log_block_stacktrace::LogBlockStacktrace,
};

pub mod java;

mod log_block;
mod log_block_normal;
mod log_block_stacktrace;

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
    /// Each of the parsed log blocks in the log.
    pub log_blocks: Vec<LogBlock<'s>>,
}
