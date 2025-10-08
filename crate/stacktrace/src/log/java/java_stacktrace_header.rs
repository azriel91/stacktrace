use std::borrow::Cow;

use pest::iterators::Pair;

use crate::{
    log::java::{
        JavaStacktraceHeaderException, JavaStacktraceHeaderMessage, JavaStacktraceHeaderThread,
    },
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
pub struct JavaStacktraceHeader<'s> {
    /// The full line of text of the header.
    pub full_text: Cow<'s, str>,
    /// The `Exception in thread ".."` text, if any.
    pub thread: Option<JavaStacktraceHeaderThread<'s>>,
    /// The `Caused by: ..` text, if any.
    pub caused_by: Option<Cow<'s, str>>,
    /// The `com.example.stacktrace.Example$Exception` text.
    pub exception: JavaStacktraceHeaderException<'s>,
    /// The `": foo"` text, if any.
    pub message: Option<JavaStacktraceHeaderMessage<'s>>,
}

impl<'s> From<Pair<'s, Rule>> for JavaStacktraceHeader<'s> {
    fn from(java_stacktrace_header_pair: Pair<'s, Rule>) -> Self {
        let full_text = Cow::Borrowed(java_stacktrace_header_pair.as_str());
        let (thread, caused_by, exception, message) =
            java_stacktrace_header_pair.into_inner().fold(
                (None, None, None, None),
                |(mut thread, mut caused_by, mut exception, mut message),
                 java_stacktrace_header_pair_inner| {
                    match java_stacktrace_header_pair_inner.as_rule() {
                        Rule::JavaStacktraceHeaderThread => {
                            let thread_pair = java_stacktrace_header_pair_inner;
                            thread = Some(JavaStacktraceHeaderThread::from(thread_pair));

                            (thread, caused_by, exception, message)
                        }
                        Rule::CAUSED_BY => {
                            let caused_by_pair = java_stacktrace_header_pair_inner;
                            caused_by = Some(Cow::Borrowed(caused_by_pair.as_str()));

                            (thread, caused_by, exception, message)
                        }
                        Rule::JavaStacktraceHeaderException => {
                            let exception_pair = java_stacktrace_header_pair_inner;
                            exception = Some(JavaStacktraceHeaderException::from(exception_pair));

                            (thread, caused_by, exception, message)
                        }
                        Rule::JavaStacktraceHeaderMessage => {
                            let message_pair = java_stacktrace_header_pair_inner;
                            message = if message_pair.as_str().is_empty() {
                                None
                            } else {
                                Some(JavaStacktraceHeaderMessage::from(message_pair))
                            };

                            (thread, caused_by, exception, message)
                        }
                        _ => unreachable!(),
                    }
                },
            );

        let exception =
            exception.expect("Expected `JavaStacktraceHeaderException` to exist after parsing.");

        Self {
            full_text,
            thread,
            caused_by,
            exception,
            message,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use pest::Parser;

    use crate::{
        log::java::{
            JavaClassNameQualified, JavaClassNameSegment, JavaClassNameSimple, JavaIdentifierLower,
            JavaPackage, JavaPackageSegment, JavaStacktraceHeader, JavaStacktraceHeaderException,
            JavaStacktraceHeaderMessage, JavaStacktraceHeaderThread, JavaThreadName,
        },
        log_parser::Rule,
        LogParser,
    };

    #[test]
    fn parse_java_stacktrace_header_with_thread() {
        let s = r#"Exception in thread "main" java.lang.IllegalArgumentException: foo"#;
        match LogParser::parse(Rule::JavaStacktraceHeader, s) {
            Ok(mut java_stacktrace_header_pairs) => {
                let java_stacktrace_header_pair = java_stacktrace_header_pairs
                    .next()
                    .expect("Expected one pair for `JavaStacktraceHeader`.");
                let java_stacktrace_header =
                    JavaStacktraceHeader::from(java_stacktrace_header_pair);
                let java_stacktrace_header_expected = JavaStacktraceHeader {
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
                    message: Some(JavaStacktraceHeaderMessage {
                        text: Cow::Borrowed(": foo"),
                    }),
                };
                assert_eq!(java_stacktrace_header_expected, java_stacktrace_header);
            }
            Err(e) => {
                eprintln!("Failed to parse `JavaStacktraceHeader`: {}", e);
                Err(e).unwrap()
            }
        }
    }

    #[test]
    fn parse_java_stacktrace_header_without_message() {
        let s = r#"java.lang.IllegalArgumentException"#;
        match LogParser::parse(Rule::JavaStacktraceHeader, s) {
            Ok(mut java_stacktrace_header_pairs) => {
                let java_stacktrace_header_pair = java_stacktrace_header_pairs
                    .next()
                    .expect("Expected one pair for `JavaStacktraceHeader`.");
                let java_stacktrace_header =
                    JavaStacktraceHeader::from(java_stacktrace_header_pair);
                let java_stacktrace_header_expected = JavaStacktraceHeader {
                    full_text: Cow::Borrowed("java.lang.IllegalArgumentException"),
                    thread: None,
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
                    message: None,
                };
                assert_eq!(java_stacktrace_header_expected, java_stacktrace_header);
            }
            Err(e) => {
                eprintln!("Failed to parse `JavaStacktraceHeader`: {}", e);
                Err(e).unwrap()
            }
        }
    }
}
