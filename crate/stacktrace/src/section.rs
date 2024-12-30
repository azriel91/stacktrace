/// A hierarchical structure of frames that have the same leading [`Group`]s.
///
/// Constructed from a group of [`Line`]s.
///
/// [`Line`]: crate::Line
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Section {
    /// Identifier for the section.
    pub id: u32,
    /// Slice of the line that is common with previous frames.
    pub slice_common_with_previous_frames: String,
    /// Slice of the line that is not common with ancestors.
    pub slice_remainder: String,
    /// Child `Section`s of this section.
    pub child_sections: Vec<Section>,
}

impl Section {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn slice_common_with_previous_frames(&self) -> &str {
        &self.slice_common_with_previous_frames
    }

    pub fn slice_remainder(&self) -> &str {
        &self.slice_remainder
    }

    pub fn child_sections(&self) -> &[Section] {
        &self.child_sections
    }
}
