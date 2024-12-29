use crate::Group;

/// A hierarchical structure of frames that have the same leading [`Group`]s.
///
/// Constructed from a group of [`Line`]s.
///
/// [`Line`]: crate::Line
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    /// Any leading whitespace for this section.
    ///
    /// This does not include leading whitespace from parent `Section`s.
    pub leading_whitespace: String,
    /// The groups of segments common to this section and child sections.
    pub groups: Vec<Group>,
    /// Child `Section`s of this section.
    pub child_sections: Vec<Section>,
}
