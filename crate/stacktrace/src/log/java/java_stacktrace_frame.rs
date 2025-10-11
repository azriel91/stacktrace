use std::borrow::Cow;

use pest::iterators::Pair;

use crate::{
    file::FilePathAndLine,
    log::java::{JavaQualifiedReference, JavaStacktraceFrameSource},
    log_parser::Rule,
};

/// The first line of a Java stacktrace.
///
/// i.e. the `Exception in thread "main" java.lang.IllegalArgumentException:
/// foo` in the following:
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
pub struct JavaStacktraceFrame<'s> {
    /// The full line of text of the frame.
    pub full_text: Cow<'s, str>,
    /// The `at` text.
    pub at: Cow<'s, str>,
    /// The `com.example.stacktrace.Example.fail` text.
    pub method_qualified_reference: JavaQualifiedReference<'s>,
    /// The `(` between the method name and the parameters.
    pub parenthesis_open: Cow<'s, str>,
    /// The `Example.java:11` / `"Native Method"` text inside the parentheses.
    pub frame_source: JavaStacktraceFrameSource<'s>,
    /// The `)` between the method name and the parameters.
    pub parenthesis_close: Cow<'s, str>,
}

impl<'s> From<Pair<'s, Rule>> for JavaStacktraceFrame<'s> {
    fn from(java_stacktrace_frame_pair: Pair<'s, Rule>) -> Self {
        let full_text = Cow::Borrowed(java_stacktrace_frame_pair.as_str());
        let (method_qualified_reference, frame_source) =
            java_stacktrace_frame_pair.into_inner().fold(
                (None, None),
                |(mut method_qualified_reference, mut frame_source),
                 java_stacktrace_frame_pair_inner| {
                    match java_stacktrace_frame_pair_inner.as_rule() {
                        Rule::JavaQualifiedReference => {
                            let method_qualified_reference_pair = java_stacktrace_frame_pair_inner;
                            method_qualified_reference = Some(JavaQualifiedReference::from(
                                method_qualified_reference_pair,
                            ));

                            (method_qualified_reference, frame_source)
                        }
                        Rule::FilePathAndLine => {
                            let file_path_and_line_pair = java_stacktrace_frame_pair_inner;
                            let file_path_and_line = FilePathAndLine::from(file_path_and_line_pair);
                            frame_source = Some(JavaStacktraceFrameSource::FilePathAndLine(
                                file_path_and_line,
                            ));

                            (method_qualified_reference, frame_source)
                        }
                        Rule::CONST_NATIVE_METHOD => {
                            let native_method_pair = java_stacktrace_frame_pair_inner;
                            let native_method = Cow::Borrowed(native_method_pair.as_str());
                            frame_source =
                                Some(JavaStacktraceFrameSource::NativeMethod(native_method));

                            (method_qualified_reference, frame_source)
                        }
                        Rule::CONST_UNKNOWN_SOURCE => {
                            let unknown_source_pair = java_stacktrace_frame_pair_inner;
                            let unknown_source = Cow::Borrowed(unknown_source_pair.as_str());
                            frame_source =
                                Some(JavaStacktraceFrameSource::UnknownSource(unknown_source));

                            (method_qualified_reference, frame_source)
                        }
                        _ => unreachable!(),
                    }
                },
            );

        let method_qualified_reference = method_qualified_reference
            .expect("Expected `JavaQualifiedReference` to exist after parsing.");
        let frame_source =
            frame_source.expect("Expected `JavaStacktraceFrameSource` to exist after parsing.");

        Self {
            full_text,
            at: Cow::Borrowed("at"),
            method_qualified_reference,
            parenthesis_open: Cow::Borrowed("("),
            frame_source,
            parenthesis_close: Cow::Borrowed(")"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use pest::Parser;

    use crate::{
        file::{FilePath, FilePathAndLine},
        log::java::{
            JavaIdentifier, JavaMixedIdentifier, JavaQualifiedReference, JavaStacktraceFrame,
            JavaStacktraceFrameSource,
        },
        log_parser::Rule,
        LogParser,
    };

    #[test]
    fn parse_java_stacktrace_frame() {
        let s = r#"at com.example.stacktrace.Example.fail(Example.java:11)"#;
        match LogParser::parse(Rule::JavaStacktraceFrame, s) {
            Ok(mut java_stacktrace_frame_pairs) => {
                let java_stacktrace_frame_pair = java_stacktrace_frame_pairs
                    .next()
                    .expect("Expected one pair for `JavaStacktraceFrame`.");
                let java_stacktrace_frame = JavaStacktraceFrame::from(java_stacktrace_frame_pair);
                let java_stacktrace_frame_expected = JavaStacktraceFrame {
                    full_text: Cow::Borrowed(
                        "at com.example.stacktrace.Example.fail(Example.java:11)",
                    ),
                    at: Cow::Borrowed("at"),
                    method_qualified_reference: JavaQualifiedReference {
                        full_text: Cow::Borrowed("com.example.stacktrace.Example.fail"),
                        segments: vec![
                            JavaMixedIdentifier {
                                angle_open: Cow::Borrowed(""),
                                identifier: JavaIdentifier {
                                    text: Cow::Borrowed("com"),
                                },
                                angle_close: Cow::Borrowed(""),
                            },
                            JavaMixedIdentifier {
                                angle_open: Cow::Borrowed(""),
                                identifier: JavaIdentifier {
                                    text: Cow::Borrowed("example"),
                                },
                                angle_close: Cow::Borrowed(""),
                            },
                            JavaMixedIdentifier {
                                angle_open: Cow::Borrowed(""),
                                identifier: JavaIdentifier {
                                    text: Cow::Borrowed("stacktrace"),
                                },
                                angle_close: Cow::Borrowed(""),
                            },
                            JavaMixedIdentifier {
                                angle_open: Cow::Borrowed(""),
                                identifier: JavaIdentifier {
                                    text: Cow::Borrowed("Example"),
                                },
                                angle_close: Cow::Borrowed(""),
                            },
                            JavaMixedIdentifier {
                                angle_open: Cow::Borrowed(""),
                                identifier: JavaIdentifier {
                                    text: Cow::Borrowed("fail"),
                                },
                                angle_close: Cow::Borrowed(""),
                            },
                        ],
                    },
                    parenthesis_open: Cow::Borrowed("("),
                    frame_source: JavaStacktraceFrameSource::FilePathAndLine(FilePathAndLine {
                        full_text: Cow::Borrowed("Example.java:11"),
                        file_path: FilePath {
                            text: Cow::Borrowed("Example.java"),
                        },
                        line_number: 11,
                    }),
                    parenthesis_close: Cow::Borrowed(")"),
                };
                assert_eq!(java_stacktrace_frame_expected, java_stacktrace_frame);
            }
            Err(e) => {
                eprintln!("Failed to parse `JavaStacktraceFrame`: {}", e);
                Err(e).unwrap()
            }
        }
    }

    #[test]
    fn parse_java_stacktrace_frame_with_init_method() {
        let s = r#"at com.example.stacktrace.Example.<init>(Example.java:11)"#;
        match LogParser::parse(Rule::JavaStacktraceFrame, s) {
            Ok(mut java_stacktrace_frame_pairs) => {
                let java_stacktrace_frame_pair = java_stacktrace_frame_pairs
                    .next()
                    .expect("Expected one pair for `JavaStacktraceFrame`.");
                let java_stacktrace_frame = JavaStacktraceFrame::from(java_stacktrace_frame_pair);
                let java_stacktrace_frame_expected = JavaStacktraceFrame {
                    full_text: Cow::Borrowed(
                        "at com.example.stacktrace.Example.<init>(Example.java:11)",
                    ),
                    at: Cow::Borrowed("at"),
                    method_qualified_reference: JavaQualifiedReference {
                        full_text: Cow::Borrowed("com.example.stacktrace.Example.<init>"),
                        segments: vec![
                            JavaMixedIdentifier {
                                angle_open: Cow::Borrowed(""),
                                identifier: JavaIdentifier {
                                    text: Cow::Borrowed("com"),
                                },
                                angle_close: Cow::Borrowed(""),
                            },
                            JavaMixedIdentifier {
                                angle_open: Cow::Borrowed(""),
                                identifier: JavaIdentifier {
                                    text: Cow::Borrowed("example"),
                                },
                                angle_close: Cow::Borrowed(""),
                            },
                            JavaMixedIdentifier {
                                angle_open: Cow::Borrowed(""),
                                identifier: JavaIdentifier {
                                    text: Cow::Borrowed("stacktrace"),
                                },
                                angle_close: Cow::Borrowed(""),
                            },
                            JavaMixedIdentifier {
                                angle_open: Cow::Borrowed(""),
                                identifier: JavaIdentifier {
                                    text: Cow::Borrowed("Example"),
                                },
                                angle_close: Cow::Borrowed(""),
                            },
                            JavaMixedIdentifier {
                                angle_open: Cow::Borrowed("<"),
                                identifier: JavaIdentifier {
                                    text: Cow::Borrowed("init"),
                                },
                                angle_close: Cow::Borrowed(">"),
                            },
                        ],
                    },
                    parenthesis_open: Cow::Borrowed("("),
                    frame_source: JavaStacktraceFrameSource::FilePathAndLine(FilePathAndLine {
                        full_text: Cow::Borrowed("Example.java:11"),
                        file_path: FilePath {
                            text: Cow::Borrowed("Example.java"),
                        },
                        line_number: 11,
                    }),
                    parenthesis_close: Cow::Borrowed(")"),
                };
                assert_eq!(java_stacktrace_frame_expected, java_stacktrace_frame);
            }
            Err(e) => {
                eprintln!("Failed to parse `JavaStacktraceFrame`: {}", e);
                Err(e).unwrap()
            }
        }
    }
}
