use std::collections::{HashMap, HashSet};

use reactive_stores::Store;
use stacktrace::sem_log::{GroupNumber, GroupNumberToPrefix};

/// Global state for the semantic log viewer.
#[derive(Clone, Debug, Default, Store)]
pub struct SemLogViewerState {
    /// Map of [`LogBlock`] indices, to their prefix group numbers and prefix.
    ///
    /// i.e.
    ///
    /// ```yaml
    /// # log block index
    /// 0:
    ///   # group number
    ///   0: "com.example"
    ///   1: "java.lang"
    ///
    /// 1:
    ///   # group number
    ///   0: "std"
    ///   1: "core"
    ///   2: "example"
    /// ```
    ///
    /// [`LogBlock`]: stacktrace::sem_log::LogBlock
    pub log_block_group_numbers_to_prefix: HashMap<usize, GroupNumberToPrefix<'static>>,
    /// Group numbers of log blocks that are squished.
    pub log_block_group_numbers_squished: HashSet<GroupNumber>,
}
