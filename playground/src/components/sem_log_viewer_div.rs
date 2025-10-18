use leptos::{
    component,
    prelude::{ClassAttribute, ElementChild, For, Get, Signal},
    view, IntoView,
};
use stacktrace::sem_log::{LogBlock, SemLog};

use crate::components::{LogBlockDiv, SemLogViewerControlsDiv};

const SEM_LOG_VIEWER_DIV_CLASSES: &str = "\
    bg-slate-700 \
    text-slate-100 \
    font-mono \
    \
    h-[36rem] \
    w-full \
    lg:max-w-7xl \
    p-4 \
    rounded-lg \
    shadow-[inset_0_2px_4px_0_rgba(0,0,0,0.3)] \
    \
    overflow-scroll \
    text-nowrap \
";

const SEM_LOG_VIEWER_PLACEHOLDER_CLASSES: &str = "\
    opacity-75 \
    italic \
    select-none \
";

const BLOCK_EXPAND_LEVEL_DEFAULT: u8 = 2;

#[component]
pub fn SemLogViewerDiv(sem_log: Signal<Option<SemLog<'static>>>) -> impl IntoView {
    let placeholder_classes = move || match sem_log.get() {
        Some(sem_log) if sem_log.log_blocks.is_empty() => SEM_LOG_VIEWER_PLACEHOLDER_CLASSES,
        _ => "hidden",
    };
    let (block_expand_level, block_expand_level_set) =
        leptos::prelude::signal(BLOCK_EXPAND_LEVEL_DEFAULT);
    view! {
        <SemLogViewerControlsDiv block_expand_level block_expand_level_set />
        <div class=SEM_LOG_VIEWER_DIV_CLASSES>
            <span class=placeholder_classes>
                "Paste some logs into the text box above"
            </span>
            <For
                each=move || {
                    sem_log.get()
                        .clone()
                        .map(|sem_log| sem_log.log_blocks)
                        .unwrap_or_default()
                }
                key=LogBlock::hash_with_default_hasher
                children=move |log_block| view! { <LogBlockDiv log_block block_expand_level=Some(block_expand_level) /> }
            />
        </div>
    }
}
