use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::log::{Log, LogEntry};

/// Parser for [`Logs`].
#[derive(Parser)]
#[grammar = "logs.pest"]
pub struct LogParser;

/// Estimated number of bytes per log block.
const BYTES_PER_LOG_BLOCK_ESTIMATED: usize = 256;

impl LogParser {
    pub fn parse_from_str<'s>(s: &'s str) -> Result<Log<'s>, Box<pest::error::Error<Rule>>> {
        let Some(logs_pair) = Self::parse(Rule::Log, s)?.next() else {
            return Ok(Log::default());
        };

        match logs_pair.as_rule() {
            Rule::Log => {
                let log_entries = logs_pair.into_inner().flat_map(Pair::into_inner).fold(
                    Vec::with_capacity(s.len() / BYTES_PER_LOG_BLOCK_ESTIMATED),
                    |mut log_entries, log_entry_pair| {
                        let log_entry = LogEntry::from(log_entry_pair);
                        log_entries.push(log_entry);
                        log_entries
                    },
                );
                Ok(Log { log_entries })
            }
            Rule::EOI => Ok(Log::default()),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{
        file::{FilePath, FilePathAndLine},
        log::{
            java::{
                JavaClassNameQualified, JavaClassNameSegment, JavaClassNameSimple, JavaIdentifier,
                JavaIdentifierLower, JavaMethodName, JavaPackage, JavaPackageSegment,
                JavaStacktrace, JavaStacktraceFrame, JavaStacktraceFrameSource,
                JavaStacktraceHeader, JavaStacktraceHeaderException, JavaStacktraceHeaderMessage,
                JavaStacktraceHeaderThread, JavaThreadName,
            },
            LogEntryStacktrace,
        },
    };

    use super::*;

    #[test]
    #[ignore = "not sure how to get pest to be lenient with/without newline"]
    fn parse_simple() {
        let s = "log1\nlog2\nlog3";
        match LogParser::parse_from_str(s) {
            Ok(logs) => {
                assert_eq!(logs.log_entries.len(), 3);
            }
            Err(e) => {
                eprintln!("Failed to parse logs: {}", e);
                Err(e).unwrap()
            }
        }
    }

    #[test]
    fn parse_simple_with_newline() {
        let s = "log1\nlog2\nlog3\n";
        match LogParser::parse_from_str(s) {
            Ok(logs) => {
                assert_eq!(logs.log_entries.len(), 3);
            }
            Err(e) => {
                eprintln!("Failed to parse logs: {}", e);
                Err(e).unwrap()
            }
        }
    }

    #[test]
    fn parse_java_stacktrace_simple() {
        let s = r#"Exception in thread "main" java.lang.IllegalArgumentException: foo
    at com.example.stacktrace.Example.fail(Example.java:11)
    at java.lang.Thread.run(Thread.java:750)
"#;
        match LogParser::parse_from_str(s) {
            Ok(logs) => {
                let log_block_stacktrace = LogEntryStacktrace::JavaStack(JavaStacktrace {
                    header: JavaStacktraceHeader {
                        full_text: Cow::Borrowed(
                            "Exception in thread \"main\" java.lang.IllegalArgumentException: foo",
                        ),
                        thread: Some(JavaStacktraceHeaderThread {
                            text: Cow::Borrowed("Exception in thread"),
                            thread_name: JavaThreadName {
                                text: Cow::Borrowed("\"main\""),
                            },
                        }),
                        caused_by: None,
                        exception: JavaStacktraceHeaderException {
                            class_name: JavaClassNameQualified {
                                full_text: Cow::Borrowed("java.lang.IllegalArgumentException"),
                                package: JavaPackage {
                                    full_text: Cow::Borrowed("java.lang"),
                                    segments: vec![
                                        JavaPackageSegment {
                                            identifier: JavaIdentifierLower {
                                                text: Cow::Borrowed("java"),
                                            },
                                        },
                                        JavaPackageSegment {
                                            identifier: JavaIdentifierLower {
                                                text: Cow::Borrowed("lang"),
                                            },
                                        },
                                    ],
                                },
                                class_name_simple: JavaClassNameSimple {
                                    full_text: Cow::Borrowed("IllegalArgumentException"),
                                    segments: vec![JavaClassNameSegment {
                                        text: Cow::Borrowed("IllegalArgumentException"),
                                    }],
                                },
                            },
                        },
                        colon: Cow::Borrowed(":"),
                        message: JavaStacktraceHeaderMessage {
                            text: Cow::Borrowed("foo"),
                        },
                    },
                    frames: vec![
                        JavaStacktraceFrame {
                            full_text: Cow::Borrowed(
                                "at com.example.stacktrace.Example.fail(Example.java:11)",
                            ),
                            at: Cow::Borrowed("at"),
                            class_name_qualified: JavaClassNameQualified {
                                full_text: Cow::Borrowed("com.example.stacktrace.Example"),
                                package: JavaPackage {
                                    full_text: Cow::Borrowed("com.example.stacktrace"),
                                    segments: vec![
                                        JavaPackageSegment {
                                            identifier: JavaIdentifierLower {
                                                text: Cow::Borrowed("com"),
                                            },
                                        },
                                        JavaPackageSegment {
                                            identifier: JavaIdentifierLower {
                                                text: Cow::Borrowed("example"),
                                            },
                                        },
                                        JavaPackageSegment {
                                            identifier: JavaIdentifierLower {
                                                text: Cow::Borrowed("stacktrace"),
                                            },
                                        },
                                    ],
                                },
                                class_name_simple: JavaClassNameSimple {
                                    full_text: Cow::Borrowed("Example"),
                                    segments: vec![JavaClassNameSegment {
                                        text: Cow::Borrowed("Example"),
                                    }],
                                },
                            },
                            dot: Cow::Borrowed("."),
                            method_name: JavaMethodName {
                                identifier: JavaIdentifier {
                                    text: Cow::Borrowed("fail"),
                                },
                            },
                            parenthesis_open: Cow::Borrowed("("),
                            frame_source: JavaStacktraceFrameSource::FilePathAndLine(
                                FilePathAndLine {
                                    full_text: Cow::Borrowed("Example.java:11"),
                                    file_path: FilePath {
                                        text: Cow::Borrowed("Example.java"),
                                    },
                                    line_number: 11,
                                },
                            ),
                            parenthesis_close: Cow::Borrowed(")"),
                        },
                        JavaStacktraceFrame {
                            full_text: Cow::Borrowed("at java.lang.Thread.run(Thread.java:750)"),
                            at: Cow::Borrowed("at"),
                            class_name_qualified: JavaClassNameQualified {
                                full_text: Cow::Borrowed("java.lang.Thread"),
                                package: JavaPackage {
                                    full_text: Cow::Borrowed("java.lang"),
                                    segments: vec![
                                        JavaPackageSegment {
                                            identifier: JavaIdentifierLower {
                                                text: Cow::Borrowed("java"),
                                            },
                                        },
                                        JavaPackageSegment {
                                            identifier: JavaIdentifierLower {
                                                text: Cow::Borrowed("lang"),
                                            },
                                        },
                                    ],
                                },
                                class_name_simple: JavaClassNameSimple {
                                    full_text: Cow::Borrowed("Thread"),
                                    segments: vec![JavaClassNameSegment {
                                        text: Cow::Borrowed("Thread"),
                                    }],
                                },
                            },
                            dot: Cow::Borrowed("."),
                            method_name: JavaMethodName {
                                identifier: JavaIdentifier {
                                    text: Cow::Borrowed("run"),
                                },
                            },
                            parenthesis_open: Cow::Borrowed("("),
                            frame_source: JavaStacktraceFrameSource::FilePathAndLine(
                                FilePathAndLine {
                                    full_text: Cow::Borrowed("Thread.java:750"),
                                    file_path: FilePath {
                                        text: Cow::Borrowed("Thread.java"),
                                    },
                                    line_number: 750,
                                },
                            ),
                            parenthesis_close: Cow::Borrowed(")"),
                        },
                    ],
                });
                assert_eq!(
                    Log {
                        log_entries: vec![LogEntry::Stacktrace(log_block_stacktrace)]
                    },
                    logs
                );
            }
            Err(e) => {
                eprintln!("Failed to parse logs: {}", e);
                Err(e).unwrap()
            }
        }
    }
}
