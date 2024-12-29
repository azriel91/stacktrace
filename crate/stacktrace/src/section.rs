/// A hierarchical structure of frames that have the same leading [`Group`]s.
///
/// Constructed from a group of [`Line`]s.
///
/// [`Line`]: crate::Line
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    /// Slice of the line that is common with ancestor `Section`s.
    pub slice_common_with_ancestors: String,
    /// Slice of the line that is not common with ancestors.
    pub slice_remainder: String,
    /// Child `Section`s of this section.
    pub child_sections: Vec<Section>,
}
