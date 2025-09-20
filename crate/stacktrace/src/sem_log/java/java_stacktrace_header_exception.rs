use std::ops::{Deref, DerefMut};

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
    /// The raw parsed `JavaStacktraceHeaderException`.
    pub inner: crate::log::java::JavaStacktraceHeaderException<'s>,
}

impl<'s> Deref for JavaStacktraceHeaderException<'s> {
    type Target = crate::log::java::JavaStacktraceHeaderException<'s>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'s> DerefMut for JavaStacktraceHeaderException<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
