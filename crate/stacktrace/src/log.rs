pub use self::{
    log_entry::LogEntry, log_entry_normal::LogEntryNormal, log_entry_stacktrace::LogEntryStacktrace,
};

pub mod java;

mod log_entry;
mod log_entry_normal;
mod log_entry_stacktrace;

/// Plain text logs parsed into log blocks.
///
/// No semantic grouping or analysis has been done yet.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Log<'s> {
    /// Each of the parsed log entries in the log.
    pub log_entries: Vec<LogEntry<'s>>,
}
