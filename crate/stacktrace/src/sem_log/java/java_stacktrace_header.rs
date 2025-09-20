use std::ops::{Deref, DerefMut};

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
    /// The raw parsed `JavaStacktraceHeader`.
    pub inner: crate::log::java::JavaStacktraceHeader<'s>,
}

impl<'s> Deref for JavaStacktraceHeader<'s> {
    type Target = crate::log::java::JavaStacktraceHeader<'s>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'s> DerefMut for JavaStacktraceHeader<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
