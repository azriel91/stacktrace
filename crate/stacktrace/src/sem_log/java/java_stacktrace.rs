use std::ops::{Deref, DerefMut};

/// A parsed Java stacktrace.
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
pub struct JavaStacktrace<'s> {
    /// The raw parsed `JavaStacktrace`.
    pub inner: crate::log::java::JavaStacktrace<'s>,
}

impl<'s> Deref for JavaStacktrace<'s> {
    type Target = crate::log::java::JavaStacktrace<'s>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'s> DerefMut for JavaStacktrace<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
