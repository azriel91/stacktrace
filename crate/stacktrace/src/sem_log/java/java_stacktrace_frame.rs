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
pub struct JavaStacktraceFrame<'s> {
    /// The raw parsed `JavaStacktraceFrame`.
    pub inner: crate::log::java::JavaStacktraceFrame<'s>,
}

impl<'s> Deref for JavaStacktraceFrame<'s> {
    type Target = crate::log::java::JavaStacktraceFrame<'s>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'s> DerefMut for JavaStacktraceFrame<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
