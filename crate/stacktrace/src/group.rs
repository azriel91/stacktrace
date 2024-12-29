use flat_string::FlatString;
use smallvec::SmallVec;

use crate::GroupOrSegment;

/// A group of segments.
///
/// e.g. in `my::lib::<impl tachys::view::RenderHtml for (A,)>::{{closure}}`,
/// the groups are:
///
/// * `my`
/// * `lib`
/// * `<impl tachys::view::RenderHtml for (A,)>`
///     - `impl`
///     - `tachys::view::RenderHtml`
///     - `for`
///     - `(A,)`
///         - `A,`
/// * `{{closure}}`
///     - `{closure}`
///         - `closure`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Group {
    /// The value between brackets (if brackets exist).
    ///
    /// e.g.
    ///
    /// * single segment: `my`
    /// * multiple groups: `impl tachys::view::RenderHtml for (A,)`
    pub value: SmallVec<[GroupOrSegment; 1]>,
    /// Any opening bracket for this group.
    pub bracket_open: Option<char>,
    /// Any closing bracket for this group.
    pub bracket_close: Option<char>,
    /// Any punctuation separator prior to the next group.
    pub separator: Option<FlatString<2>>,
}
