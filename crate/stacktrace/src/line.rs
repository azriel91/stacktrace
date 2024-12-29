use crate::Group;

/// One line in the stack trace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    /// Any leading whitespace for this line.
    pub leading_whitespace: String,
    /// The groups of segments within this line.
    ///
    /// e.g. in `my::lib::{{closure}}`, the groups are:
    ///
    /// * `my::lib`
    /// * `{{closure}}`
    ///
    /// Each group has its own list of segments.
    pub groups: Vec<Group>,
}
