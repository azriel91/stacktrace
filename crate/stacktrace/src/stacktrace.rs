use crate::Line;

use crate::Section;

/// Parses a stack trace string into a structured stack trace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stacktrace {
    pub sections: Vec<Section>,
}

impl<'s> From<&'s str> for Stacktrace {
    fn from(s: &'s str) -> Self {
        // First parse into `Line`s
        s.lines().map(Line::from);

        todo!();
    }
}
