//! Contains types for semantic logs.
//!
//! # Thought Process
//!
//! ## Java
//!
//! An original stacktrace can look like the following:
//!
//! ```java
//! java.lang.IllegalArgumentException: foo
//!     at com.framework.executor.Executor.execute(Executor.java:70)
//!     at com.framework.executor.Executor.workStealingPoller(Executor.java:60)
//!     at com.framework.executor.Executor.poll(Executor.java:50)
//!     at com.framework.Framework.run(Framework.java:30)
//!     at com.example.stacktrace.Example.start(Example.java:20)
//!     at com.example.stacktrace.Example.main(Example.java:15)
//! Caused by: com.example.stacktrace.Example$Exception: bar
//!     at com.example.stacktrace.Example.fail(Example.java:40)
//!     at com.example.stacktrace.Example.work(Example.java:33)
//!     at com.framework.executor.Executor.doWork(Executor.java:80)
//!     at java.lang.Thread.run(Thread.java:750)
//! ... 5 more
//! ```
//!
//! The relevant parts of the stacktrace are the `example` application's frames,
//! so we can collapse adjacent framework frames:
//!
//! ```java
//! java.lang.IllegalArgumentException: foo
//!     at com.framework.<.. 4 frames ..>
//!     at com.example.stacktrace.Example.start(Example.java:20)
//!     at com.example.stacktrace.Example.main(Example.java:15)
//! Caused by: com.example.stacktrace.Example$Exception: bar
//!     at com.example.stacktrace.Example.fail(Example.java:40)
//!     at com.example.stacktrace.Example.work(Example.java:33)
//!     at com.framework.executor.Executor.doWork(Executor.java:80)
//!     at java.lang.Thread.run(Thread.java:750)
//! ... 5 more
//! ```
//!
//! Next, repeated package segments are noisy, so if we can dim them, it allows
//! the user to clearly see the cross-library boundaries:
//!
//! ```java
//! java.lang.IllegalArgumentException: foo
//!     at com.framework.<.. 4 frames ..>
//!     at com.example.stacktrace.Example.start(Example.java:20)
//!                               Example.main(Example.java:15)
//! Caused by: com.example.stacktrace.Example$Exception: bar
//!     at com.example.stacktrace.Example.fail(Example.java:40)
//!                               Example.work(Example.java:33)
//!     at com.framework.executor.Executor.doWork(Executor.java:80)
//!     at java.lang.Thread.run(Thread.java:750)
//! ... 5 more
//! ```
//!
//! Also, source file references should probably be moved out of the method
//! calls (but still kept aligned):
//!
//! ```java
//! java.lang.IllegalArgumentException: foo
//!     at com.framework.<.. 4 frames ..>
//!     at com.example.stacktrace.Example.start()    // Example.java:20
//!                               Example.main()     // Example.java:15
//! Caused by: com.example.stacktrace.Example$Exception: bar
//!     at com.example.stacktrace.Example.fail()     // Example.java:40
//!                               Example.work()     // Example.java:33
//!     at com.framework.executor.Executor.doWork()  // Executor.java:80
//!     at java.lang.Thread.run()                    // Thread.java:750
//! ... 5 more
//! ```
//!
//! It may be useful to reduce this further by collapsing non-`example` frames:
//!
//! ```java
//! java.lang.IllegalArgumentException: foo
//!     <.. 4 frames ..>
//!     at com.example.stacktrace.Example.start()  // Example.java:20
//!                               Example.main()   // Example.java:15
//! Caused by: com.example.stacktrace.Example$Exception: bar
//!     at com.example.stacktrace.Example.fail()   // Example.java:40
//!                               Example.work()   // Example.java:33
//!     <.. 2 frames ..>
//! ... 5 more
//! ```
//!
//! ## Rust
//!
//! A Rust backtrace is reduced differently -- as stack frames are prefixed with
//! frame numbers:
//!
//! ```text
//! stack backtrace:
//!    0: std::panicking::panic_handler
//!              at /rustc/7c275d09ea6b953d2cca169667184a7214bd14c7/library\std\src\panicking.rs:698
//!    1: core::panicking::panic_fmt
//!              at /rustc/7c275d09ea6b953d2cca169667184a7214bd14c7/library\core\src\panicking.rs:75
//!    2: example::three
//!              at .\src\main.rs:10
//!    3: example::sub::two
//!              at .\src\main.rs:6
//!    4: example::sub::one
//!              at .\src\main.rs:2
//!    5: example::main
//!              at .\src\main.rs:14
//!    6: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
//!              at C:\Users\example\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\ops\function.rs:250
//!    7: core::hint::black_box
//!              at C:\Users\example\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\src\rust\library\core\src\hint.rs:472
//! ```
//!
//! Reducing the above, we may do something like the following in a web view,
//! and the full path should still be copyable and perhaps visible on hover:
//!
//! ```rust,ignore
//! stack backtrace:
//!    0-1: <.. std/core: 2 frames ..>
//!    2  : example::three                  // .\src\main.rs:10
//!    3-4:        ::sub::<.. 2 frames ..>
//!    5  :        ::main                   // .\src\main.rs:14
//!    6-7: core::<.. 2 frames ..>
//! ```
//!
//! In terms of collapsibility, we still want to support hierarchical
//! collapsability:
//!
//! ```rust,ignore
//! stack backtrace:
//!    0-1: <.. std/core: 2 frames ..>
//!    2-5: example::<.. 4 frames ..>
//!    6-7: core::<.. 2 frames ..>
//! ```
//!
//! Expanding the `example::` section should render something like the previous
//! code block.
//!
//!
//! # Design
//!
//! What element hierarchy allows us to have:
//!
//! * context (e.g. stack frame numbers) retained on the left.
//! * nested collapsible sections (including hover styling).
//! * the whole line's text still copyable.
//!
//! Maybe we should have nested `<details>` elements, with no margin/padding, so
//! that each line that is rendered will have the stack frame numbers still
//! pixel-aligned correctly, and based on the nesting level, we render "fake"
//! left-borders (maybe use an inline block) to draw the "box" around the
//! details content.
//!
//! # Implementation
//!
//! Each collapsible section is stored as a [`LogBlock`].

use crate::log::Log;

pub use self::{
    group_number::GroupNumber,
    group_number_to_prefix::GroupNumberToPrefix,
    group_prefix::GroupPrefix,
    into_log_block::IntoLogBlock,
    log_block::{LogBlock, LogBlockPartial},
    log_line_segment::LogLineSegment,
    log_line_segment_kind::LogLineSegmentKind,
    nesting_level::NestingLevel,
};

mod group_number;
mod group_number_to_prefix;
mod group_prefix;
mod into_log_block;
mod log_block;
mod log_line_segment;
mod log_line_segment_kind;
mod nesting_level;

/// Logs with semantic information augmented to ease clear presentation.
///
/// Examples of semantic information:
///
/// * Grouping stack frames that belong to the same class / package / module.
///
/// Not implemented yet:
///
/// * Duration between this log message and the previous / next.
/// * Log messages grouped by thread.
/// * Arbitrary grouped log messages.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SemLog<'s> {
    /// Each of the parsed log blocks in the log.
    pub log_blocks: Vec<LogBlock<'s>>,
}

impl<'s> SemLog<'s> {
    /// Returns a fully owned version of this [`SemLog`].
    pub fn into_static(&self) -> SemLog<'static> {
        let log_blocks = self.log_blocks.iter().map(LogBlock::into_static).collect();

        SemLog { log_blocks }
    }
}

impl<'s> From<Log<'s>> for SemLog<'s> {
    fn from(log: Log<'s>) -> Self {
        let Log { log_entries } = log;

        let log_blocks = log_entries
            .into_iter()
            .map(IntoLogBlock::into_log_block)
            .collect::<Vec<LogBlock<'s>>>();

        SemLog { log_blocks }
    }
}
