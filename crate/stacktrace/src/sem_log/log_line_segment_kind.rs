/// Whether a segment is common to an ancestor or introduced by this
/// [`LogBlock`].
///
/// [`LogBlock`]: crate::sem_log::LogBlock
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LogLineSegmentKind {
    /// Contextual information of a stack trace, e.g. frame number, "at".
    ///
    /// These should always be rendered.
    Context,
    /// Segment that is common to both this [`LogBlock`] and an ancestor
    /// [`LogBlock`].
    ///
    /// [`LogBlock`]: crate::sem_log::LogBlock
    CommonWithParent,
    /// Segment that is newly introduced by this [`LogBlock`].
    ///
    /// [`LogBlock`]: crate::sem_log::LogBlock
    Introduced,
    /// Segment that is a placeholder for collapsed child [`LogBlock`]s.
    ///
    /// [`LogBlock`]: crate::sem_log::LogBlock
    CollapsedBlockPlaceholder,
}
