use std::marker::PhantomData;

/// A block in the log file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogBlock<'s> {
    pub marker: PhantomData<&'s ()>,
}
