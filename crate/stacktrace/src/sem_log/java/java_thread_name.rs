use std::ops::{Deref, DerefMut};

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
    /// The raw parsed `JavaThreadName`.
    pub inner: crate::log::java::JavaThreadName<'s>,
}

impl<'s> Deref for JavaThreadName<'s> {
    type Target = crate::log::java::JavaThreadName<'s>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'s> DerefMut for JavaThreadName<'s> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
