use std::borrow::Cow;

use crate::sem_log::{IntoLogBlock, LogBlock, LogLineSegment, LogLineSegmentKind};

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

        LogBlock {
            text,
            line_segments,
            line_segments_collapsed,
            children,
        }
    }
}
