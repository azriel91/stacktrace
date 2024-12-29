use flat_string::FlatString;

/// A segment of text that should be treated as a whole.
///
/// e.g. in `my::lib::{{closure}}`, the segments are:
///
/// Group 1:
///
/// * `my`
/// * `lib`
///
/// Group 2:
///
/// * `closure`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Segment {
    /// The text in the segment, e.g. `my_package`, `{{closure}}`.
    pub text: String,
    /// Any punctuation separator prior to the next group.
    pub separator: Option<FlatString<2>>,
}
