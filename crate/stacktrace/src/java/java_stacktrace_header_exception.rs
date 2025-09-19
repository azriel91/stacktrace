use pest::iterators::Pair;

use crate::{java::JavaClassNameQualified, logs_parser::Rule};

/// The exception class name in a stack trace header.
///
/// i.e. `java.lang.IllegalArgumentException` or
/// `com.example.stacktrace.Example$Exception` in the following:
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
pub struct JavaStacktraceHeaderException<'s> {
    /// The exception class name, e.g.
    /// `com.example.stacktrace.Example$Exception`.
    pub class_name: JavaClassNameQualified<'s>,
}

impl<'s> From<Pair<'s, Rule>> for JavaStacktraceHeaderException<'s> {
    fn from(java_stacktrace_header_exception_pair: Pair<'s, Rule>) -> Self {
        let class_name = java_stacktrace_header_exception_pair.into_inner().fold(
            None,
            |_class_name, java_stacktrace_header_exception_pair_inner| {
                match java_stacktrace_header_exception_pair_inner.as_rule() {
                    Rule::JavaClassNameQualified => {
                        let class_name_pair = java_stacktrace_header_exception_pair_inner;
                        Some(JavaClassNameQualified::from(class_name_pair))
                    }
                    _ => unreachable!(),
                }
            },
        );

        let class_name =
            class_name.expect("Expected `JavaClassNameQualified` to exist after parsing.");

        Self { class_name }
    }
}
