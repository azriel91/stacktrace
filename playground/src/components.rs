pub use self::{
    log_block_controls_div::LogBlockControlsDiv,
    log_block_div::{GroupingsSignificance, LogBlockDiv},
    log_block_squished_controls_div::LogBlockSquishedControlsDiv,
    log_line_segment_span::LogLineSegmentSpan,
    log_line_segments_div::LogLineSegmentsDiv,
    sem_log_viewer_controls_div::SemLogViewerControlsDiv,
    sem_log_viewer_div::SemLogViewerDiv,
};

/// Classes for the `<div>` that contains the buttons.
const LOG_BLOCK_CONTROLS_DIV_CLASSES: &str = "\
    flex-none \
    opacity-0 \
    group-hover:opacity-100 \
    group-focus:opacity-100 \
    group-focus-within:opacity-100 \
";

/// Classes for each `<button>`.
const LOG_BLOCK_CONTROLS_DIV_BUTTON_CLASSES: &str = "\
    px-1 \
    py-1 \
    rounded \
    border \
    border-slate-800 \
    bg-slate-600 \
    hover:border-slate-500 \
    hover:bg-slate-400 \
    group-focus:border-slate-600 \
    group-focus:bg-slate-500 \
    active:border-slate-900 \
    active:bg-slate-700 \
";

mod log_block_controls_div;
mod log_block_div;
mod log_block_squished_controls_div;
mod log_line_segment_span;
mod log_line_segments_div;
mod sem_log_viewer_controls_div;
mod sem_log_viewer_div;
