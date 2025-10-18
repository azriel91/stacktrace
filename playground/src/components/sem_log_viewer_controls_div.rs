use leptos::{
    component,
    ev::InputEvent,
    prelude::{ClassAttribute, ElementChild, Get, OnAttribute, ReadSignal, Set, WriteSignal},
    view, IntoView,
};
use wasm_bindgen::JsCast;

const SEM_LOG_VIEWER_CONTROLS_DIV_CLASSES: &str = "\
    bg-slate-700 \
    text-slate-100 \
    font-mono \
    \
    w-full \
    lg:max-w-7xl \
    p-4 \
    rounded-lg \
    shadow-[inset_0_2px_4px_0_rgba(0,0,0,0.3)] \
    \
    overflow-scroll \
    text-nowrap \
";

#[component]
pub fn SemLogViewerControlsDiv(
    block_expand_level: ReadSignal<u8>,
    block_expand_level_set: WriteSignal<u8>,
) -> impl IntoView {
    view! {
        <div class=SEM_LOG_VIEWER_CONTROLS_DIV_CLASSES>
            <label>"Expand level:"</label>
            <input
                type="number"
                min="0"
                max="10"
                value=move || block_expand_level.get()
                on:input=move |ev| {
                    let value = ev.unchecked_ref::<InputEvent>()
                        .data()
                        .map(|s| s.parse::<u8>());
                    if let Some(Ok(value)) = value {
                        block_expand_level_set.set(value);
                    }
                }
            />
        </div>
    }
}
