pub use self::{
    log_block::LogBlock, log_block_normal::LogBlockNormal, log_block_stacktrace::LogBlockStacktrace,
};

pub mod java;

mod log_block;
mod log_block_normal;
mod log_block_stacktrace;

/// Plain text logs parsed into log blocks.
///
/// No semantic grouping or analysis has been done yet.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Log<'s> {
    /// Each of the parsed log blocks in the log.
    pub log_blocks: Vec<LogBlock<'s>>,
}
