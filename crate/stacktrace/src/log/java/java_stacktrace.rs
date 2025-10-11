use std::{borrow::Cow, cmp::Ordering};

use pest::iterators::Pair;

use crate::{
    log::java::{JavaStacktraceFrame, JavaStacktraceHeader},
    log_parser::Rule,
    sem_log::{IntoLogBlock, LogBlock, LogBlockPartial, LogLineSegment, LogLineSegmentKind},
};

/// A parsed Java stacktrace.
///
/// ```java
/// Exception in thread "main" java.lang.IllegalArgumentException: foo
///     at com.example.stacktrace.Example.fail(Example.java:11)
///     at java.lang.Thread.run(Thread.java:750)
/// Caused by: com.example.stacktrace.Example$Exception: bar
///     at com.example.stacktrace.Example.fail(Example.java:12)
/// ... 2 more
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JavaStacktrace<'s> {
    /// The first line of the stacktrace.
    ///
    /// i.e. the `Exception in thread "main" java.lang.IllegalArgumentException:
    /// foo`.
    pub header: JavaStacktraceHeader<'s>,
    /// The frames of the stacktrace.
    pub frames: Vec<JavaStacktraceFrame<'s>>,
}

impl<'s> JavaStacktrace<'s> {
    /// Returns the hierarchy of `LogBlock`s constructed from the Java
    /// stacktrace frames.
    ///
    /// This is a convenience method that calls the recursive
    /// `log_block_partials_into_log_blocks` method.
    fn frames_into_log_blocks(frames: Vec<JavaStacktraceFrame<'s>>) -> Vec<LogBlock<'s>> {
        let mut log_block_partials = frames.into_iter().map(LogBlockPartial::from);

        let mut log_blocks = Vec::new();
        let None = Self::log_block_partials_into_log_blocks(
            &mut log_blocks,
            None,
            &mut log_block_partials,
        ) else {
            panic!("Expected `log_block_partials_into_log_blocks` to return `None` at top level.");
        };
        log_blocks
    }

    /// Adds [`LogBlock`]s to the given vector, nested when [`LogLineSegment`]s
    /// are common with previous frames.
    ///
    /// The return value is a [`LogBlockPartial`] that may be a sibling to aid
    /// the recursive logic. If it isn't a sibling, we return upward.
    ///
    /// ## Nesting
    ///
    /// We need to decide how to structure the nesting. Given the following
    /// structure:
    ///
    /// ```java
    /// 1: at a.A
    /// 2: at a.b.B
    /// 3: at a.b.C
    /// 4: at a.b.c.D
    /// 5: at a.A
    /// ```
    ///
    /// ### Frame-Line Consistent
    ///
    /// In this nesting, we consider subsequent lines to be children if the
    /// package is the same:
    ///
    /// ```java
    /// 1: at a.A      // top level
    /// 2: at  .b.B    // child of 1
    /// 3: at    .C    // child of 2
    /// 4: at    .c.D  // child of 2
    /// 5: at  .A      // child of 1
    /// ```
    ///
    /// Pros:
    ///
    /// * Lines match up with original stacktrace.
    /// * Possibly simpler to implement.
    ///
    /// Cons:
    ///
    /// * Package wise, 2 and 3 should be children of 1.
    ///
    /// ### Package-Hierarchy Consistent
    ///
    /// ```java
    /// 1: at a.A       // top level
    /// ?      .b       // child of 1, newly introduced line
    /// 2: at    .B     // child of ?
    /// 3: at    .C     // child of ?
    /// 4: at    .c.D   // child of ?
    /// 5: at  .A       // child of 1
    /// ```
    ///
    /// Pros:
    ///
    /// * B and C are now given equal "rank", which aligns with the code.
    ///
    /// Cons:
    ///
    /// * Introduces additional "line".
    ///
    /// ---
    ///
    /// `32cceea` is the first implementation of the Frame-Line consistent
    /// approach that looks right.
    ///
    /// # Notes
    ///
    /// To construct a [`LogBlock`] we need:
    ///
    /// 1. Lowest child needs to know the common segments from the higher
    ///    frames.
    /// 2. Yet, highest [`LogBlock`]'s children are only ready after the lower
    ///    children are built.
    ///
    /// When there are parent line segments, we:
    ///
    /// * don't want to render our line segments that the parent already has.
    /// * want to render our line segments that the parent doesn't have.
    /// * recurse.
    ///
    /// When there are no parent line segments, we:
    ///
    /// * want to render all our line segments.
    /// * recurse.
    ///
    /// Do we want to detect where the next frame differs from
    /// us, and through that difference, split our own difference
    /// (likely a package, class, or method) into a separate
    /// [`LogBlock`]?
    ///
    /// Possibly not -- it may be a "surprise" to the user.
    /// It could make sense for the `line_segments_collapsed`
    /// though.
    #[must_use]
    fn log_block_partials_into_log_blocks(
        log_blocks: &mut Vec<LogBlock<'s>>,
        parent_line_segments: Option<&[LogLineSegment<'s>]>,
        log_block_partial_iter: &mut impl Iterator<Item = LogBlockPartial<'s>>,
    ) -> Option<LogBlockPartial<'s>> {
        let mut log_block_partial_opt = log_block_partial_iter.next();
        while let Some(log_block_partial) = log_block_partial_opt {
            let LogBlockPartial {
                text,
                mut line_segments,
            } = log_block_partial;

            // The block that should be a child of the parent block, and maybe a sibling.
            let (log_block, log_block_partial_sibling_opt) = match parent_line_segments {
                None => {
                    // Recurse, because the next `LogBlockPartial` might be a child of this one.
                    let mut children = Vec::new();
                    let log_block_partial = Self::log_block_partials_into_log_blocks(
                        &mut children,
                        Some(&line_segments),
                        log_block_partial_iter,
                    );

                    // `log_block_partial` can only be a sibling since there are no parent line
                    // segments in this branch.

                    let (line_segments_collapsed, children_collapsed_text) =
                        line_segments_collapsed_compute(&line_segments, &children);

                    let log_block = LogBlock {
                        text,
                        line_segments,
                        line_segments_collapsed,
                        children,
                        children_collapsed_text,
                    };
                    (log_block, log_block_partial)
                }
                Some(parent_line_segments) => {
                    Self::mark_log_segment_kinds_as_common_with_parent(
                        parent_line_segments,
                        &mut line_segments,
                    );

                    // At this point, we need to calculate whether to add this `log_block` as a
                    // child and recurse, or to return as it is not a child of the parent.
                    //
                    // ```java
                    // 1: at a.A      // top level
                    // 2: at  .b.B    // child of 1
                    // 3: at    .C    // child of 2
                    // 4: at    .c.D  // child of 2
                    // 5: at  .A      // child of 1
                    // ```
                    //
                    // * **A:** No parent line segments. Add to `log_blocks`, recurse.
                    // * **B:** `a` is common with parent, still has two additional segments. Add to
                    //   `log_blocks`, recurse.
                    // * **C:** `a.b.` are common with parent, still has one segment. Add to
                    //   `log_blocks`, recurse.
                    // * **D:** `a.b.` are common with parent, still has two segments -- same number
                    //   of common segments, `return` so it is not a child of 3.
                    // * **A:** Because there already are children, return so it is not a child of
                    //
                    // # Implementation
                    //
                    // * If this [`LogBlock`] has fewer common segments with its immediate parent,
                    //   this is not a child, so we return it up the stack.
                    // * If this [`LogBlock`] has more common segments with its parents than with
                    //   its immediate parent, then it is a child, so we add it to `log_blocks`. We
                    //   should recurse because the next [`LogBlock`] may be a child of this one.

                    let parent_common_segment_count = parent_line_segments
                        .iter()
                        .filter(|line_segment| {
                            line_segment.kind == LogLineSegmentKind::CommonWithParent
                        })
                        .count();
                    let common_segment_count = line_segments
                        .iter()
                        .filter(|line_segment| {
                            line_segment.kind == LogLineSegmentKind::CommonWithParent
                        })
                        .count();

                    // we could count the number of `Introduced` segments for this log block, but
                    // we'll assume there's at least one.

                    // When this frame is a child of the parent, then the next frame may be a
                    // sibling or ancestor.
                    let common_segment_count_cmp_parent =
                        common_segment_count.cmp(&parent_common_segment_count);
                    match common_segment_count_cmp_parent {
                        Ordering::Less | Ordering::Equal => {
                            // Return because this should not be a child of the current parent.
                            //
                            // * If this is `Ordering::Equal`, the returned `LogBlockPartial` is a
                            //   sibling of the parent call.
                            // * If this is `Ordering::Less`, the returned `LogBlockPartial` should
                            //   be compared again with the parent's parent.
                            let log_block_partial = LogBlockPartial {
                                text,
                                line_segments,
                            };
                            return Some(log_block_partial);
                        }
                        Ordering::Greater => {}
                    };

                    // This `log_block` is a child of the current parent, and should be added to
                    // `log_block`s.
                    let mut children = Vec::new();

                    // Recurse in case the next frame is a child.
                    let log_block_partial = Self::log_block_partials_into_log_blocks(
                        &mut children,
                        Some(&line_segments),
                        log_block_partial_iter,
                    );

                    let (line_segments_collapsed, children_collapsed_text) =
                        line_segments_collapsed_compute(&line_segments, &children);
                    let log_block = LogBlock {
                        text,
                        line_segments,
                        line_segments_collapsed,
                        children,
                        children_collapsed_text,
                    };

                    // If `log_block_partial` is `Some`, it means the next frame was not a child.
                    //
                    // The number of segments it has in common with *this* frame's parent determines
                    // whether it is a sibling of this frame (`Ordering::Equal`), or potentially an
                    // ancestor (`Ordering::Less`).
                    let log_block_partial = match log_block_partial {
                        Some(log_block_partial) => {
                            let next_frame_common_segment_count = log_block_partial
                                .line_segments
                                .iter()
                                .filter(|line_segment| {
                                    line_segment.kind == LogLineSegmentKind::CommonWithParent
                                })
                                .count();

                            let next_frame_common_segment_count_cmp_parent =
                                next_frame_common_segment_count.cmp(&parent_common_segment_count);
                            match next_frame_common_segment_count_cmp_parent {
                                // Record the current frame, and recurse upward.
                                //
                                // The `return` here returns out of the function.
                                Ordering::Less => {
                                    log_blocks.push(log_block);

                                    return Some(log_block_partial);
                                }

                                // Sibling, so we continue this level of recursion's loop
                                Ordering::Equal => Some(log_block_partial),

                                // unreachable!("inner recursion layer guarantees it is Less |
                                // Equal.")
                                Ordering::Greater => Some(log_block_partial),
                            }
                        }
                        None => None,
                    };

                    (log_block, log_block_partial)
                }
            };

            log_blocks.push(log_block);

            log_block_partial_opt = if log_block_partial_sibling_opt.is_some() {
                log_block_partial_sibling_opt
            } else {
                log_block_partial_iter.next()
            };
        }
        None
    }

    /// Compares [`LogLineSegment`]s with parent [`LogLineSegment`]s, and
    /// where they are common, sets the `kind` to
    /// `LogLineSegmentKind::CommonWithParent`.
    fn mark_log_segment_kinds_as_common_with_parent(
        parent_line_segments: &[LogLineSegment<'_>],
        line_segments: &mut [LogLineSegment<'_>],
    ) {
        let mut parent_line_segments_iter = parent_line_segments.iter();
        let mut line_segments_iter = line_segments.iter_mut();

        loop {
            match (parent_line_segments_iter.next(), line_segments_iter.next()) {
                // Finished looping through all line segments.
                (None, None) => break,

                // Don't need to mutate any future `line_segment.kind`s, leave them as
                // `LogLineSegmentKind::Introduced`.
                //
                // We've already processed all parent line segments, meaning
                // everything from here on is `Introduced`.
                (None, Some(_line_segment)) => break,

                // A parent frame had more segments than us, and all previous segments match. This
                // case should be rare if not never. Just break out of the loop since there is
                // nothing to do.
                (Some(_parent_line_segment), None) => break,

                // Compare this segment with the parent segment. We expect most of the leading
                // segments to be aligned. Cases:
                //
                // * Either has a `kind` of `Context`: continue.
                // * Ancestor `kind` is `CommonWithParent` / `Introduced`:
                //
                //   - if this segment has the same text as the parent segment, make this segment
                //     `CommonWithParent`. Keep iterating to next segment.
                //   - if this segment has a different text than the parent segment, keep this
                //     segment `kind` as `Introduced`. Recurse.
                (Some(parent_line_segment), Some(line_segment)) => {
                    match (parent_line_segment.kind, line_segment.kind) {
                        (LogLineSegmentKind::Context, _)
                        | (_, LogLineSegmentKind::Context) => continue,
                        (
                            LogLineSegmentKind::CommonWithParent
                            | LogLineSegmentKind::Introduced
                            // This variant should be unreachable.
                            | LogLineSegmentKind::CollapsedBlockPlaceholder,
                            _,
                        ) => {
                            // Fall through to next part.
                        }
                    }
                    if parent_line_segment.text == line_segment.text {
                        line_segment.kind = LogLineSegmentKind::CommonWithParent;
                        // continue to next segment.
                    } else {
                        // No-op, since this is how we initialized it.
                        // line_segment.kind = LogLineSegmentKind::Introduced;

                        // We don't need to compare any remaining segments, since they should all be
                        // considered different from the parent.
                        break;
                    }
                }
            }
        }
    }
}

fn line_segments_collapsed_compute<'f, 's>(
    line_segments: &'f [LogLineSegment<'s>],
    children: &'f [LogBlock<'s>],
) -> (Vec<LogLineSegment<'s>>, Cow<'static, str>) {
    let mut line_segments_collapsed = Vec::with_capacity(line_segments.len());

    // Remove the segments that are not in common with any child.
    //
    // i.e. find the min of all immediate children's line segments that are
    // `CommonWithParent`
    let line_segments_to_keep_count = children
        .iter()
        .map(|log_block| {
            log_block
                .line_segments
                .iter()
                .take_while(|line_segment| line_segment.kind != LogLineSegmentKind::Introduced)
                .count()
        })
        .min()
        .unwrap_or(line_segments.len());
    line_segments_collapsed.extend_from_slice(&line_segments[0..line_segments_to_keep_count]);
    line_segments_collapsed.push(LogLineSegment {
        text: Cow::Borrowed("…"),
        separator: Cow::Borrowed(""),
        kind: LogLineSegmentKind::CollapsedBlockPlaceholder,
    });

    let n = descendent_count(children.iter()) + 1; // + 1 to include the current frame.
    let children_collapsed_text = Cow::Owned(format!("{n} frames"));
    (line_segments_collapsed, children_collapsed_text)
}

fn descendent_count<'f, 's: 'f>(
    log_block_iter: impl ExactSizeIterator<Item = &'f LogBlock<'s>>,
) -> usize {
    let len = log_block_iter.len();
    log_block_iter.fold(len, |acc, log_block| {
        acc + descendent_count(log_block.children.iter())
    })
}

impl<'s> From<Pair<'s, Rule>> for JavaStacktrace<'s> {
    fn from(java_stacktrace_pair: Pair<'s, Rule>) -> Self {
        let (header, frames) = java_stacktrace_pair.into_inner().fold(
            (None, Vec::new()),
            |(mut java_stacktrace_header, mut frames), java_stacktrace_pair_inner| {
                match java_stacktrace_pair_inner.as_rule() {
                    Rule::JavaStacktraceHeader => {
                        let java_stacktrace_header_pair = java_stacktrace_pair_inner;
                        java_stacktrace_header =
                            Some(JavaStacktraceHeader::from(java_stacktrace_header_pair));

                        (java_stacktrace_header, frames)
                    }
                    Rule::JavaStacktraceFrame => {
                        let java_stacktrace_frame_pair = java_stacktrace_pair_inner;
                        let java_stacktrace_frame =
                            JavaStacktraceFrame::from(java_stacktrace_frame_pair);
                        frames.push(java_stacktrace_frame);

                        (java_stacktrace_header, frames)
                    }
                    _ => unreachable!(),
                }
            },
        );

        let header = header.expect("Expected `JavaStacktraceHeader` to exist after parsing.");

        Self { header, frames }
    }
}

impl<'s> IntoLogBlock<'s> for JavaStacktrace<'s> {
    fn into_log_block(self) -> LogBlock<'s> {
        let JavaStacktrace { header, frames } = self;
        let frame_count = frames.len();
        let line_segments = vec![LogLineSegment {
            text: header.full_text.clone(),
            separator: Cow::Borrowed(""),
            kind: LogLineSegmentKind::Introduced,
        }];
        let children = Self::frames_into_log_blocks(frames);
        let line_segments_collapsed = line_segments.clone();
        let children_collapsed_text = Cow::Owned(format!("{frame_count} frames"));

        LogBlock {
            text: header.full_text,
            line_segments,
            line_segments_collapsed,
            children,
            children_collapsed_text,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use pest::Parser;
    use pretty_assertions::assert_eq;

    use crate::{
        log::java::JavaStacktrace,
        log_parser::Rule,
        sem_log::{IntoLogBlock, LogBlock, LogLineSegment, LogLineSegmentKind},
        LogParser,
    };

    #[test]
    fn into_log_block() {
        let stacktrace_str = "\
            java.net.SocketTimeoutException: Read timed out\n\
               at java.net.SocketInputStream.socketRead0(Native Method)\n\
               at java.net.SocketInputStream.socketRead(SocketInputStream.java:116)\n\
               at java.net.SocketInputStream.read(SocketInputStream.java:171)\n\
               at java.net.SocketInputStream.read(SocketInputStream.java:141)\n\
               at java.io.BufferedInputStream.fill(BufferedInputStream.java:246)\n\
               at java.io.BufferedInputStream.read1(BufferedInputStream.java:286)\n\
               at java.io.BufferedInputStream.read(BufferedInputStream.java:345)\n\
               at java.io.DataInputStream.readFully(DataInputStream.java:195)\n\
            ";
        match LogParser::parse(Rule::JavaStacktrace, stacktrace_str) {
            Ok(java_stacktrace_pairs) => {
                let java_stacktrace = JavaStacktrace::from(
                    java_stacktrace_pairs
                        .into_iter()
                        .next()
                        .expect("Expected one `JavaStacktrace` pair."),
                );
                let log_block_actual = java_stacktrace.into_log_block();

                let log_block_expected = LogBlock {
                    text: Cow::Borrowed("java.net.SocketTimeoutException: Read timed out"),
                    line_segments: vec![
                        LogLineSegment {
                            text: Cow::Borrowed("java.net.SocketTimeoutException: Read timed out"),
                            separator: Cow::Borrowed(""),
                            kind: LogLineSegmentKind::Introduced
                        }
                    ],
                    line_segments_collapsed: vec![
                        LogLineSegment {
                            text: Cow::Borrowed("java.net.SocketTimeoutException: Read timed out"),
                            separator: Cow::Borrowed(""),
                            kind: LogLineSegmentKind::Introduced
                        }
                    ],
                    children: vec![
                        LogBlock {
                            text: Cow::Borrowed("at java.net.SocketInputStream.socketRead0(Native Method)"),
                            line_segments: vec![
                                LogLineSegment {
                                    text: Cow::Borrowed("at"),
                                    separator: Cow::Borrowed(" "),
                                    kind: LogLineSegmentKind::Context
                                },
                                LogLineSegment {
                                    text: Cow::Borrowed("java"),
                                    separator: Cow::Borrowed("."),
                                    kind: LogLineSegmentKind::Introduced
                                },
                                LogLineSegment {
                                    text: Cow::Borrowed("net"),
                                    separator: Cow::Borrowed("."),
                                    kind: LogLineSegmentKind::Introduced
                                },
                                LogLineSegment {
                                    text: Cow::Borrowed("SocketInputStream"),
                                    separator: Cow::Borrowed("."),
                                    kind: LogLineSegmentKind::Introduced
                                },
                                LogLineSegment {
                                    text: Cow::Borrowed("socketRead0"),
                                    separator: Cow::Borrowed(""),
                                    kind: LogLineSegmentKind::Introduced
                                },
                                LogLineSegment {
                                    text: Cow::Borrowed("("),
                                    separator: Cow::Borrowed(""),
                                    kind: LogLineSegmentKind::Context
                                },
                                LogLineSegment {
                                    text: Cow::Borrowed("Native Method"),
                                    separator: Cow::Borrowed(")"),
                                    kind: LogLineSegmentKind::Introduced
                                }
                            ],
                            line_segments_collapsed: vec![
                                LogLineSegment {
                                    text: Cow::Borrowed("at"),
                                    separator: Cow::Borrowed(" "),
                                    kind: LogLineSegmentKind::Context
                                },
                                LogLineSegment {
                                    text: Cow::Borrowed("java"),
                                    separator: Cow::Borrowed("."),
                                    kind: LogLineSegmentKind::Introduced
                                },
                                LogLineSegment {
                                    text: Cow::Borrowed("…"),
                                    separator: Cow::Borrowed(""),
                                    kind: LogLineSegmentKind::CollapsedBlockPlaceholder
                                }
                            ],
                            children: vec![
                                LogBlock {
                                    text: Cow::Borrowed("at java.net.SocketInputStream.socketRead(SocketInputStream.java:116)"),
                                    line_segments: vec![
                                        LogLineSegment {
                                            text: Cow::Borrowed("at"),
                                            separator: Cow::Borrowed(" "),
                                            kind: LogLineSegmentKind::Context
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("java"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::CommonWithParent
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("net"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::CommonWithParent
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("SocketInputStream"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::CommonWithParent
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("socketRead"),
                                            separator: Cow::Borrowed(""),
                                            kind: LogLineSegmentKind::Introduced
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("("),
                                            separator: Cow::Borrowed(""),
                                            kind: LogLineSegmentKind::Context
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("SocketInputStream.java:116"),
                                            separator: Cow::Borrowed(")"),
                                            kind: LogLineSegmentKind::Introduced
                                        }
                                    ],
                                    line_segments_collapsed: vec![
                                        LogLineSegment {
                                            text: Cow::Borrowed("at"),
                                            separator: Cow::Borrowed(" "),
                                            kind: LogLineSegmentKind::Context
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("java"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::CommonWithParent
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("net"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::CommonWithParent
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("SocketInputStream"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::CommonWithParent
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("socketRead"),
                                            separator: Cow::Borrowed(""),
                                            kind: LogLineSegmentKind::Introduced
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("("),
                                            separator: Cow::Borrowed(""),
                                            kind: LogLineSegmentKind::Context
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("SocketInputStream.java:116"),
                                            separator: Cow::Borrowed(")"),
                                            kind: LogLineSegmentKind::Introduced
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("…"),
                                            separator: Cow::Borrowed(""),
                                            kind: LogLineSegmentKind::CollapsedBlockPlaceholder
                                        }
                                    ],
                                    children: vec![],
                                    children_collapsed_text: Cow::Borrowed("1 frames")
                                },
                                LogBlock {
                                    text: Cow::Borrowed("at java.net.SocketInputStream.read(SocketInputStream.java:171)"),
                                    line_segments: vec![
                                        LogLineSegment {
                                            text: Cow::Borrowed("at"),
                                            separator: Cow::Borrowed(" "),
                                            kind: LogLineSegmentKind::Context
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("java"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::CommonWithParent
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("net"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::CommonWithParent
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("SocketInputStream"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::CommonWithParent
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("read"),
                                            separator: Cow::Borrowed(""),
                                            kind: LogLineSegmentKind::Introduced
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("("),
                                            separator: Cow::Borrowed(""),
                                            kind: LogLineSegmentKind::Context
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("SocketInputStream.java:171"),
                                            separator: Cow::Borrowed(")"),
                                            kind: LogLineSegmentKind::Introduced
                                        }
                                    ],
                                    line_segments_collapsed: vec![
                                        LogLineSegment {
                                            text: Cow::Borrowed("at"),
                                            separator: Cow::Borrowed(" "),
                                            kind: LogLineSegmentKind::Context
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("java"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::CommonWithParent
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("net"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::CommonWithParent
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("SocketInputStream"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::CommonWithParent
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("read"),
                                            separator: Cow::Borrowed(""),
                                            kind: LogLineSegmentKind::Introduced
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("("),
                                            separator: Cow::Borrowed(""),
                                            kind: LogLineSegmentKind::Context
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("…"),
                                            separator: Cow::Borrowed(""),
                                            kind: LogLineSegmentKind::CollapsedBlockPlaceholder
                                        }
                                    ],
                                    children: vec![
                                        LogBlock {
                                            text: Cow::Borrowed("at java.net.SocketInputStream.read(SocketInputStream.java:141)"),
                                            line_segments: vec![
                                                LogLineSegment {
                                                    text: Cow::Borrowed("at"),
                                                    separator: Cow::Borrowed(" "),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("java"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("net"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("SocketInputStream"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("read"),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("("),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("SocketInputStream.java:141"),
                                                    separator: Cow::Borrowed(")"),
                                                    kind: LogLineSegmentKind::Introduced
                                                }
                                            ],
                                            line_segments_collapsed: vec![
                                                LogLineSegment {
                                                    text: Cow::Borrowed("at"),
                                                    separator: Cow::Borrowed(" "),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("java"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("net"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("SocketInputStream"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("read"),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("("),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("SocketInputStream.java:141"),
                                                    separator: Cow::Borrowed(")"),
                                                    kind: LogLineSegmentKind::Introduced
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("…"),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::CollapsedBlockPlaceholder
                                                }
                                            ],
                                            children: vec![],
                                            children_collapsed_text: Cow::Borrowed("1 frames")
                                        },
                                    ],
                                    children_collapsed_text: Cow::Borrowed("2 frames")
                                },
                                LogBlock {
                                    text: Cow::Borrowed("at java.io.BufferedInputStream.fill(BufferedInputStream.java:246)"),
                                    line_segments: vec![
                                        LogLineSegment {
                                            text: Cow::Borrowed("at"),
                                            separator: Cow::Borrowed(" "),
                                            kind: LogLineSegmentKind::Context
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("java"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::CommonWithParent
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("io"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::Introduced
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("BufferedInputStream"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::Introduced
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("fill"),
                                            separator: Cow::Borrowed(""),
                                            kind: LogLineSegmentKind::Introduced
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("("),
                                            separator: Cow::Borrowed(""),
                                            kind: LogLineSegmentKind::Context
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("BufferedInputStream.java:246"),
                                            separator: Cow::Borrowed(")"),
                                            kind: LogLineSegmentKind::Introduced
                                        }
                                    ],
                                    line_segments_collapsed: vec![
                                        LogLineSegment {
                                            text: Cow::Borrowed("at"),
                                            separator: Cow::Borrowed(" "),
                                            kind: LogLineSegmentKind::Context
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("java"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::CommonWithParent
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("io"),
                                            separator: Cow::Borrowed("."),
                                            kind: LogLineSegmentKind::Introduced
                                        },
                                        LogLineSegment {
                                            text: Cow::Borrowed("…"),
                                            separator: Cow::Borrowed(""),
                                            kind: LogLineSegmentKind::CollapsedBlockPlaceholder
                                        }
                                    ],
                                    children: vec![
                                        LogBlock {
                                            text: Cow::Borrowed("at java.io.BufferedInputStream.read1(BufferedInputStream.java:286)"),
                                            line_segments: vec![
                                                LogLineSegment {
                                                    text: Cow::Borrowed("at"),
                                                    separator: Cow::Borrowed(" "),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("java"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("io"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("BufferedInputStream"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("read1"),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::Introduced
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("("),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("BufferedInputStream.java:286"),
                                                    separator: Cow::Borrowed(")"),
                                                    kind: LogLineSegmentKind::Introduced
                                                }
                                            ],
                                            line_segments_collapsed: vec![
                                                LogLineSegment {
                                                    text: Cow::Borrowed("at"),
                                                    separator: Cow::Borrowed(" "),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("java"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("io"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("BufferedInputStream"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("read1"),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::Introduced
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("("),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("BufferedInputStream.java:286"),
                                                    separator: Cow::Borrowed(")"),
                                                    kind: LogLineSegmentKind::Introduced
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("…"),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::CollapsedBlockPlaceholder
                                                }
                                            ],
                                            children: vec![],
                                            children_collapsed_text: Cow::Borrowed("1 frames")
                                        },
                                        LogBlock {
                                            text: Cow::Borrowed("at java.io.BufferedInputStream.read(BufferedInputStream.java:345)"),
                                            line_segments: vec![
                                                LogLineSegment {
                                                    text: Cow::Borrowed("at"),
                                                    separator: Cow::Borrowed(" "),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("java"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("io"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("BufferedInputStream"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("read"),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::Introduced
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("("),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("BufferedInputStream.java:345"),
                                                    separator: Cow::Borrowed(")"),
                                                    kind: LogLineSegmentKind::Introduced
                                                }
                                            ],
                                            line_segments_collapsed: vec![
                                                LogLineSegment {
                                                    text: Cow::Borrowed("at"),
                                                    separator: Cow::Borrowed(" "),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("java"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("io"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("BufferedInputStream"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("read"),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::Introduced
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("("),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("BufferedInputStream.java:345"),
                                                    separator: Cow::Borrowed(")"),
                                                    kind: LogLineSegmentKind::Introduced
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("…"),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::CollapsedBlockPlaceholder
                                                }
                                            ],
                                            children: vec![],
                                            children_collapsed_text: Cow::Borrowed("1 frames")
                                        },
                                        LogBlock {
                                            text: Cow::Borrowed("at java.io.DataInputStream.readFully(DataInputStream.java:195)"),
                                            line_segments: vec![
                                                LogLineSegment {
                                                    text: Cow::Borrowed("at"),
                                                    separator: Cow::Borrowed(" "),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("java"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("io"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("DataInputStream"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::Introduced
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("readFully"),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::Introduced
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("("),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("DataInputStream.java:195"),
                                                    separator: Cow::Borrowed(")"),
                                                    kind: LogLineSegmentKind::Introduced
                                                }
                                            ],
                                            line_segments_collapsed: vec![
                                                LogLineSegment {
                                                    text: Cow::Borrowed("at"),
                                                    separator: Cow::Borrowed(" "),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("java"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("io"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::CommonWithParent
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("DataInputStream"),
                                                    separator: Cow::Borrowed("."),
                                                    kind: LogLineSegmentKind::Introduced
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("readFully"),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::Introduced
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("("),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::Context
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("DataInputStream.java:195"),
                                                    separator: Cow::Borrowed(")"),
                                                    kind: LogLineSegmentKind::Introduced
                                                },
                                                LogLineSegment {
                                                    text: Cow::Borrowed("…"),
                                                    separator: Cow::Borrowed(""),
                                                    kind: LogLineSegmentKind::CollapsedBlockPlaceholder
                                                }
                                            ],
                                            children: vec![],
                                            children_collapsed_text: Cow::Borrowed("1 frames")
                                        }
                                    ],
                                    children_collapsed_text: Cow::Borrowed("4 frames")
                                }
                            ],
                            children_collapsed_text: Cow::Borrowed("8 frames")
                        }
                    ],
                    children_collapsed_text: Cow::Borrowed("8 frames")
                }
;
                assert_eq!(log_block_expected, log_block_actual);
            }
            Err(e) => {
                eprintln!("Failed to parse `JavaStacktrace`: {}", e);
                Err(e).unwrap()
            }
        }
    }
}
