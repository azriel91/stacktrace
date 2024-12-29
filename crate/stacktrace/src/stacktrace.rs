use crate::Section;

/// Parses a stack trace string into a structured stack trace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stacktrace {
    pub sections: Vec<Section>,
}

impl<'s> From<&'s str> for Stacktrace {
    fn from(_s: &'s str) -> Self {
        todo!();
    }
}
