use leptos::{
    component,
    prelude::{ClassAttribute, ElementChild, For, Get, Signal},
    view, IntoView,
};
use stacktrace::sem_log::{LogBlock, SemLog};

use crate::components::LogBlockDiv;

const LOG_VIEWER_CLASSES: &str = "\
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

const LOG_VIEWER_PLACEHOLDER_CLASSES: &str = "\
    opacity-75 \
    italic \
    select-none \
";

#[component]
pub fn LogViewer(sem_log: Signal<Option<SemLog<'static>>>) -> impl IntoView {
    let placeholder_classes = move || match sem_log.get() {
        Some(sem_log) if sem_log.log_blocks.is_empty() => LOG_VIEWER_PLACEHOLDER_CLASSES,
        _ => "hidden",
    };
    view! {
        <div class=LOG_VIEWER_CLASSES>
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
                children=|log_block| view! { <LogBlockDiv log_block /> }
            />
        </div>
    }
}
