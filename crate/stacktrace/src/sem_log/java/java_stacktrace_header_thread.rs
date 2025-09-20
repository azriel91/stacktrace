use std::ops::{Deref, DerefMut};

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
    /// The raw parsed `JavaStacktraceHeaderThread`.
    pub inner: crate::log::java::JavaStacktraceHeaderThread<'s>,
}

impl<'s> Deref for JavaStacktraceHeaderThread<'s> {
    type Target = crate::log::java::JavaStacktraceHeaderThread<'s>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'s> DerefMut for JavaStacktraceHeaderThread<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
