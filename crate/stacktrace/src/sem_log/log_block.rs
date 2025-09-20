use std::{
    borrow::Cow,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::{
    log::java::{JavaClassNameQualified, JavaStacktraceFrame},
    sem_log::{LogLineSegment, LogLineSegmentKind},
};

/// A line in the log file.
///
/// The rendered version of this should be collapsible/expandable based on the
/// state of the elements of the parent [`LogBlock`]s to reduce the amount of
/// information displayed, hiding irrelevant information.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LogBlock<'s> {
    /// Original text of this line, copied when the copy button is clicked.
    ///
    /// Usually this is one line. However, in the case of rust stack traces,
    /// this may be multiple lines as the source code reference is placed on a
    /// line after each frame.
    pub text: Cow<'s, str>,

    /// Parts of the text in this log line, so we can render stack frame
    /// numbers, invisible shared context, then visible different context.
    pub line_segments: Vec<LogLineSegment<'s>>,

    /// Parts of the text to display when the line is collapsed.
    pub line_segments_collapsed: Vec<LogLineSegment<'s>>,

    /// Children of this block.
    pub children: Vec<LogBlock<'s>>,
}

impl<'s> LogBlock<'s> {
    pub fn hash_with_default_hasher(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

/// A partially constructed [`LogBlock`].
///
/// This is an intermediate structure while line segments have not yet been
/// compared with previous lines, and so:
///
/// * `line_segments`'s `kind` has not yet been processed into
///   [`LogLineSegmentKind::CommonWithParent`].
/// * `line_segments_collapsed` and `children` are not yet known.
///
/// [`LogLineSegmentKind::CommonWithParent`]: crate::sem_log::LogLineSegmentKind::CommonWithParent
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogBlockPartial<'s> {
    /// Original text of this line, copied when the copy button is clicked.
    ///
    /// Usually this is one line. However, in the case of rust stack traces,
    /// this may be multiple lines as the source code reference is placed on a
    /// line after each frame.
    pub text: Cow<'s, str>,

    /// Parts of the text in this log line, so we can render stack frame
    /// numbers, invisible shared context, then visible different context.
    pub line_segments: Vec<LogLineSegment<'s>>,
}

impl<'s> From<JavaStacktraceFrame<'s>> for LogBlockPartial<'s> {
    /// Returns a `LogBlock` representing the given frame, but common segments
    /// with the ancestor are not yet marked as
    /// [`LogLineSegmentKind::CommonWithParent`].
    fn from(frame: JavaStacktraceFrame<'s>) -> Self {
        let JavaStacktraceFrame {
            full_text,
            at,
            class_name_qualified:
                JavaClassNameQualified {
                    full_text: _,
                    package,
                    class_name_simple,
                },
            dot,
            method_name,
            parenthesis_open,
            file_path_and_line,
            parenthesis_close,
        } = frame;
        let text = full_text;
        let line_segments = {
            let mut line_segments = Vec::with_capacity(package.segments.len() + 4);
            line_segments.push(LogLineSegment {
                text: at,
                separator: Cow::Borrowed(" "),
                kind: LogLineSegmentKind::Context,
            });
            line_segments.extend(package.segments.into_iter().map(|package_segment| {
                LogLineSegment {
                    text: package_segment.identifier.text,
                    separator: Cow::Borrowed("."),
                    kind: LogLineSegmentKind::Introduced,
                }
            }));
            line_segments.push(LogLineSegment {
                text: class_name_simple.full_text,
                separator: dot,
                kind: LogLineSegmentKind::Introduced,
            });
            line_segments.push(LogLineSegment {
                text: method_name.identifier.text,
                separator: parenthesis_open,
                kind: LogLineSegmentKind::Introduced,
            });
            line_segments.push(LogLineSegment {
                text: file_path_and_line.full_text,
                separator: parenthesis_close,
                kind: LogLineSegmentKind::Introduced,
            });
            line_segments
        };

        LogBlockPartial {
            text,
            line_segments,
        }
    }
}
