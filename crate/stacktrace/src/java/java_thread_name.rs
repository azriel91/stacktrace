use std::borrow::Cow;

use pest::iterators::Pair;

use crate::logs_parser::Rule;

/// The thread name e.g. `"main"` in a Java stacktrace.
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
pub struct JavaThreadName<'s> {
    /// The `"main"` text.
    pub text: Cow<'s, str>,
}

impl<'s> From<Pair<'s, Rule>> for JavaThreadName<'s> {
    fn from(java_thread_name_pair: Pair<'s, Rule>) -> Self {
        let text = Cow::Borrowed(java_thread_name_pair.as_str());

        Self { text }
    }
}
