//! State to allow interactions on lower level components to propagate to other
//! components in the viewer.
//!
//! Use cases:
//!
//! 1. Squish all other sibling log blocks except this one.

pub use sem_log_viewer_state::{SemLogViewerState, SemLogViewerStateStoreFields};

mod sem_log_viewer_state;
