use std::borrow::Cow;

use pest::iterators::Pair;

use crate::{java::JavaThreadName, logs_parser::Rule};

/// The `Exception in thread ".."` header of a Java stacktrace.
///
/// i.e. the `Exception in thread "main"` in the following:
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
pub struct JavaStacktraceHeaderThread<'s> {
    /// The `Exception in thread` text.
    pub text: Cow<'s, str>,
    /// The thread name, e.g. `"main"`.
    pub thread_name: JavaThreadName<'s>,
}

impl<'s> From<Pair<'s, Rule>> for JavaStacktraceHeaderThread<'s> {
    fn from(java_stacktrace_header_thread_pair: Pair<'s, Rule>) -> Self {
        let (text, thread_name) = java_stacktrace_header_thread_pair.into_inner().fold(
            (None, None),
            |(mut text, mut thread_name), java_stacktrace_header_thread_pair_inner| {
                match java_stacktrace_header_thread_pair_inner.as_rule() {
                    Rule::EXCEPTION_IN_THREAD => {
                        let exception_in_thread_pair = java_stacktrace_header_thread_pair_inner;
                        text = Some(Cow::Borrowed(exception_in_thread_pair.as_str()));

                        (text, thread_name)
                    }
                    Rule::JavaThreadName => {
                        let thread_name_pair = java_stacktrace_header_thread_pair_inner;
                        thread_name = Some(JavaThreadName::from(thread_name_pair));

                        (text, thread_name)
                    }
                    _ => unreachable!(),
                }
            },
        );

        let text = text.expect("Expected `EXCEPTION_IN_THREAD` text to exist after parsing.");
        let thread_name = thread_name.expect("Expected `JavaThreadName` to exist after parsing.");

        Self { text, thread_name }
    }
}
