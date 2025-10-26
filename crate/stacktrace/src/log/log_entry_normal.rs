use std::borrow::Cow;

use crate::sem_log::{GroupNumber, IntoLogBlock, LogBlock, LogLineSegment, LogLineSegmentKind};

/// A log message that isn't specially treated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogEntryNormal<'s> {
    /// The full text of the log entry.
    pub text: Cow<'s, str>,
}

impl<'s> IntoLogBlock<'s> for LogEntryNormal<'s> {
    fn into_log_block(self) -> LogBlock<'s> {
        let LogEntryNormal { text } = self;

        let line_segments = vec![LogLineSegment {
            text: text.clone(),
            separator: Cow::Borrowed(""),
            kind: LogLineSegmentKind::Introduced,
        }];
        let line_segments_collapsed = line_segments.clone();
        let children = Vec::new();
        let children_collapsed_text = Cow::Borrowed("");

        LogBlock {
            nesting_level: 0,
            group_number: GroupNumber::new(0),
            group_numbers_to_prefix: None, // Idea: This could be per thread, or per project / crate
            text,
            line_segments,
            line_segments_collapsed,
            children,
            children_collapsed_text,
        }
    }
}
