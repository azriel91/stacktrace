#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prefix {
    /// A segment of text that should be treated as a whole.
    ///
    /// e.g. in `my::lib::{{closure}}`, the segments are:
    ///
    /// * `my`
    /// * `lib`
    /// * `{{closure}}`
    pub segment: Segment,
    ///
    pub separator: Separator,
}
