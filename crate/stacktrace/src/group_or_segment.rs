use crate::{Group, Segment};

/// Either a `Group` or a `Segment`.
///
/// This exists because a group may nest another group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GroupOrSegment {
    /// A segment.
    Segment(Segment),
    /// A nested group.
    Group(Box<Group>),
}
