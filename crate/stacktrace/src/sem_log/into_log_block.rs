use crate::sem_log::LogBlock;

pub trait IntoLogBlock<'s> {
    /// Returns [`LogBlock`] that is semantically collapsible based on the
    /// data in this object.
    ///
    /// # Parameters
    ///
    /// * `base_id`: The starting ID for the log lines.
    fn into_log_block(self) -> LogBlock<'s>;
}
