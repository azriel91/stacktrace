use leptos::{
    component,
    ev::InputEvent,
    prelude::{
        ClassAttribute, ElementChild, Get, OnAttribute, PropAttribute, ReadSignal, Set, WriteSignal,
    },
    view, IntoView,
};
use stacktrace::sem_log::NestingLevel;
use wasm_bindgen::JsCast;

const SEM_LOG_VIEWER_CONTROLS_DIV_CLASSES: &str = "\
    w-full \
    lg:max-w-7xl \
    p-2 \
    rounded-lg \
    \
    flex \
";

const EXPAND_LEVEL_INPUT_CLASSES: &str = "\
    bg-blue-900 \
    px-2 \
    w-10 \
    text-right \
    rounded \
    border \
    border-blue-950 \
";

/// Tailwind classes for the `expand_level` "+" and "-" buttons.
const EXPAND_LEVEL_BUTTON_CLASSES: &str = "\
    bg-blue-500 \
    border-blue-600 \
    hover:bg-blue-600 \
    hover:border-blue-800 \
    active:bg-blue-700 \
    active:border-blue-900 \
    text-white \
    font-bold \
    inline-block \
    w-6 \
    h-6 \
    rounded \
    border \
";

#[component]
pub fn SemLogViewerControlsDiv(
    block_expand_level: ReadSignal<NestingLevel>,
    block_expand_level_set: WriteSignal<NestingLevel>,
) -> impl IntoView {
    view! {
        <div class=SEM_LOG_VIEWER_CONTROLS_DIV_CLASSES>
            <label for="expand_level">"Expand level:"</label>
            <div class="w-2"></div>
            <input
                name="expand_level"
                type="number"
                min="0"
                max="10"
                class=EXPAND_LEVEL_INPUT_CLASSES
                prop:value=move || block_expand_level.get().into_inner()
                on:input=move |ev| {
                    let value = ev.unchecked_ref::<InputEvent>()
                        .data()
                        .map(|s| s.parse::<u8>());
                    if let Some(Ok(value)) = value {
                        block_expand_level_set.set(NestingLevel::new(value));
                    }
                }
            />
            <div class="w-2"></div>
            <ExpandLevelSubtractButton block_expand_level block_expand_level_set />
            <ExpandLevelAddButton block_expand_level block_expand_level_set />
        </div>
    }
}

#[component]
fn ExpandLevelSubtractButton(
    block_expand_level: ReadSignal<NestingLevel>,
    block_expand_level_set: WriteSignal<NestingLevel>,
) -> impl IntoView {
    view! {
        <button
            class=EXPAND_LEVEL_BUTTON_CLASSES
            on:click=move |_event| {
                let next_value = block_expand_level.get().saturating_sub(1);
                block_expand_level_set.set(NestingLevel::new(next_value));
            }
        >
            "-"
        </button>
    }
}

#[component]
fn ExpandLevelAddButton(
    block_expand_level: ReadSignal<NestingLevel>,
    block_expand_level_set: WriteSignal<NestingLevel>,
) -> impl IntoView {
    view! {
        <button
            class=EXPAND_LEVEL_BUTTON_CLASSES
            on:click=move |_event| {
                let next_value = block_expand_level.get().saturating_add(1);
                block_expand_level_set.set(NestingLevel::new(next_value));
            }
        >
            "+"
        </button>
    }
}
