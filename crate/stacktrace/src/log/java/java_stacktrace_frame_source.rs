use std::borrow::Cow;

use crate::file::FilePathAndLine;

/// Where the code for a stack frame is defined.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JavaStacktraceFrameSource<'s> {
    /// The `"Unknown Source"` string.
    UnknownSource(Cow<'s, str>),
    /// The `"Native Method"` string.
    NativeMethod(Cow<'s, str>),
    /// A file path and line number.
    FilePathAndLine(FilePathAndLine<'s>),
}
