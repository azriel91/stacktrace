use crate::sem_log::java::JavaStacktrace;

/// A parsed stacktrace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LogBlockStacktrace<'s> {
    /// A stacktrace from a Java program.
    JavaStack(JavaStacktrace<'s>),
}
