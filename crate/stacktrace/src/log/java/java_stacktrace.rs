use pest::iterators::Pair;

use crate::{
    log::java::{JavaStacktraceFrame, JavaStacktraceHeader},
    log_parser::Rule,
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
    /// The full text of the log block.
    pub header: Option<JavaStacktraceHeader<'s>>,
    /// The frames of the stacktrace.
    pub frames: Vec<JavaStacktraceFrame<'s>>,
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

        Self { header, frames }
    }
}
