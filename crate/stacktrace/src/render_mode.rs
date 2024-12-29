/// How to render a segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderMode {
    /// Draw this at full opacity.
    Visible,
    /// Draw this at faded or hidden opacity.
    Faded,
    /// Collapse the segment from being rendered.
    Collapsed,
}
