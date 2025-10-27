use std::collections::{BTreeMap, HashMap, HashSet};

use reactive_stores::Store;
use stacktrace::sem_log::{GroupNumber, GroupNumberToPrefix, NestingLevel};

/// Global state for the semantic log viewer.
#[derive(Clone, Debug, Default, Store)]
pub struct SemLogViewerState {
    /// Map of [`LogBlock`] indices per nesting level, to their prefix group
    /// numbers and prefix.
    ///
    /// i.e.
    ///
    /// ```yaml
    /// # nesting_level, we generally look this up via random access.
    /// 0:
    ///   # log_block_index, we generally iterate over this level.
    ///   0:
    ///     # group number
    ///     0: "com.example"
    ///     1: "java.lang"
    ///
    ///   1:
    ///     # group number
    ///     0: "std"
    ///     1: "core"
    ///     2: "example"
    /// ```
    ///
    /// [`LogBlock`]: stacktrace::sem_log::LogBlock
    pub log_block_group_numbers_to_prefixes:
        HashMap<NestingLevel, BTreeMap<usize, GroupNumberToPrefix<'static>>>,
    /// Group numbers of log blocks that are squished.
    ///
    /// i.e.
    ///
    /// ```yaml
    /// # (nesting_level, log_block_index): group numbers
    /// (0, 0): [1]
    /// (0, 1): [0, 1]
    /// ```
    pub log_block_group_numbers_squished: HashMap<(NestingLevel, usize), HashSet<GroupNumber>>,
}
