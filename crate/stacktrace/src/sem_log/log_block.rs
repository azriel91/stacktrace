use std::{
    borrow::Cow,
    hash::{DefaultHasher, Hash, Hasher},
};

use crate::{
    log::java::{
        JavaMixedIdentifier, JavaQualifiedReference, JavaStacktraceFrame, JavaStacktraceFrameSource,
    },
    sem_log::{GroupNumber, GroupNumberToPrefix, LogLineSegment, LogLineSegmentKind},
};

/// A line in the log file.
///
/// The rendered version of this should be collapsible/expandable based on the
/// state of the elements of the parent [`LogBlock`]s to reduce the amount of
/// information displayed, hiding irrelevant information.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LogBlock<'s> {
    /// Nesting level of this block, `0` is a top level block.
    ///
    /// This allows styling to be applied to the block based on its nesting
    /// level.
    pub nesting_level: u8,

    /// Number assigned to this block so that different blocks with the same
    /// group number can be styled with the same background colour.
    ///
    /// This will allow users to visually see which code blocks are related to
    /// each other.
    ///
    /// Blocks with the same `(nesting_level, group_number)` values should have
    /// the same background colour.
    pub group_number: GroupNumber,

    /// Mapping from group numbers to prefixes for styling.
    ///
    /// This is `Some` if the level of this block is at a level of detail useful
    /// to filter by, e.g. blocks that are part of the same project / crate.
    ///
    /// These are not collected at the [`SemLog`] level, because when collapsing
    /// (un)related blocks, we want to only do it for the [`LogBlock`] that
    /// is visible on the screen, not the ones that are scrolled offscreen.
    ///
    /// [`SemLog`]: crate::sem_log::SemLog
    pub group_numbers_to_prefix: Option<GroupNumberToPrefix<'s>>,

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

    /// Context to display when the children are collapsed.
    pub children_collapsed_text: Cow<'s, str>,
}

impl<'s> LogBlock<'s> {
    /// Returns a fully owned version of this [`LogBlock`].
    pub fn into_static(&self) -> LogBlock<'static> {
        LogBlock {
            nesting_level: self.nesting_level,
            group_number: self.group_number,
            group_numbers_to_prefix: self.group_numbers_to_prefix.as_ref().map(
                |group_numbers_to_prefix| {
                    group_numbers_to_prefix.iter().fold(
                        GroupNumberToPrefix::<'static>::new(),
                        |mut group_numbers_to_prefix_next, (group_number, prefix)| {
                            group_numbers_to_prefix_next.insert(
                                group_number.clone(),
                                prefix
                                    .iter()
                                    .map(|segment| Cow::Owned(segment.clone().into_owned()))
                                    .collect::<Vec<Cow<'static, str>>>()
                                    .into(),
                            );
                            group_numbers_to_prefix_next
                        },
                    )
                },
            ),
            text: Cow::Owned(self.text.clone().into_owned()),
            line_segments: self
                .line_segments
                .iter()
                .map(LogLineSegment::into_static)
                .collect(),
            line_segments_collapsed: self
                .line_segments_collapsed
                .iter()
                .map(LogLineSegment::into_static)
                .collect(),
            children: self.children.iter().map(LogBlock::into_static).collect(),
            children_collapsed_text: Cow::Owned(self.children_collapsed_text.clone().into_owned()),
        }
    }

    /// Returns a hash of this [`LogBlock`] using the default hasher.
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
            method_qualified_reference:
                JavaQualifiedReference {
                    full_text: _,
                    segments,
                },
            parenthesis_open,
            frame_source,
            parenthesis_close,
        } = frame;
        let text = full_text;
        let line_segments = {
            let mut line_segments = Vec::with_capacity(segments.len() + 3);
            line_segments.push(LogLineSegment {
                text: at,
                separator: Cow::Borrowed(" "),
                kind: LogLineSegmentKind::Context,
            });
            line_segments.extend(segments.into_iter().map(|mixed_identifier| {
                let JavaMixedIdentifier {
                    full_text,
                    angle_open: _,
                    identifier: _,
                    angle_close: _,
                } = mixed_identifier;
                LogLineSegment {
                    text: full_text,
                    separator: Cow::Borrowed("."),
                    kind: LogLineSegmentKind::Introduced,
                }
            }));
            // Remove dot from the last segment, so that there is no dot before the opening
            // parenthesis.
            if let Some(method_name_segment) = line_segments.last_mut() {
                method_name_segment.separator = Cow::Borrowed("");
            }
            line_segments.push(LogLineSegment {
                text: parenthesis_open,
                separator: Cow::Borrowed(""),
                kind: LogLineSegmentKind::Context,
            });
            let text = match frame_source {
                JavaStacktraceFrameSource::UnknownSource(unknown_source) => unknown_source,
                JavaStacktraceFrameSource::NativeMethod(native_method) => native_method,
                JavaStacktraceFrameSource::FilePathAndLine(file_path_and_line) => {
                    file_path_and_line.full_text
                }
            };
            line_segments.push(LogLineSegment {
                text,
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
