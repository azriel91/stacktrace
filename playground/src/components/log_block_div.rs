use std::borrow::Cow;

use leptos::{
    component,
    either::Either,
    prelude::{
        ClassAttribute, Effect, ElementChild, For, Get, IntoAny, Memo, OnAttribute, ReadSignal, Set,
    },
    view, IntoView,
};
use stacktrace::sem_log::LogBlock;

use crate::components::LogLineSegmentsDiv;

const LINE_CLASSES: &str = "\
    border \
    border-transparent \
    hover:border-blue-400 \
    px-2 \
    py-1 \
    rounded-lg \
";

/// Multiple `hover:open` selectors so that we only highlight the border if
/// there are no nested [`LogBlockDiv`]s that are also hovered.
const BLOCK_CLASSES: &str = "\
    rounded-lg \
    border-s \
    border-transparent \
    open:px-2 \
    open:border-blue-400 \
    hover:open:border-blue-200 \
    hover:open:has-[:hover:open]:border-blue-400 \
";

const BLOCK_BG_COLOURS: [&str; 6] = [
    "bg-teal-950",
    "bg-sky-950",
    "bg-blue-950",
    "bg-indigo-950",
    "bg-violet-950",
    "bg-purple-950",
];

/// # Parameters
///
/// * `log_block`: The log block to display.
/// * `block_expand_level`: The level the user has chosen to expand / collapse
///   all blocks.
/// * `block_index`: The index of the block.
#[component]
pub fn LogBlockDiv(
    log_block: LogBlock<'static>,
    // `#[prop(optional_no_strip)]` is needed so we can pass in `None` for this prop.
    #[prop(optional_no_strip)] block_expand_level: Option<ReadSignal<u8>>,
    #[prop(optional)] block_index: usize,
) -> impl IntoView {
    let (expanded_user_override, expanded_user_override_set) =
        leptos::prelude::signal(Option::<bool>::None);

    let is_leaf_block = log_block.children.is_empty();
    let expanded = Memo::new(move |_| {
        if is_leaf_block {
            true
        } else {
            match expanded_user_override.get() {
                Some(expanded) => expanded,
                None => block_expand_level
                    .map(|block_expand_level| log_block.nesting_level < block_expand_level.get())
                    .unwrap_or_default(),
            }
        }
    });

    Effect::new(move || {
        if let Some(block_expand_level) = block_expand_level {
            // Enable signal tracking -- every time this changes, we remove the user
            // override.
            let _block_expand_level = block_expand_level.get();
            expanded_user_override_set.set(None);
        }
    });

    if is_leaf_block {
        let classes = first_line_css_classes(LINE_CLASSES, &log_block, block_index);
        Either::Left(view! {
            <div class=classes>
                <LogLineSegmentsDiv
                    expanded
                    line_segments={log_block.line_segments.clone()}
                    line_segments_collapsed={log_block.line_segments_collapsed.clone()}
                    children_collapsed_text={log_block.children_collapsed_text.clone()}
                />
            </div>
        })
    } else {
        let classes = first_line_css_classes(BLOCK_CLASSES, &log_block, block_index);
        Either::Right(view! {
            <div class="py-1 rounded">
                <details
                    open={move || expanded.get()}
                    on:toggle=move |event| {
                        let expanded_override = Some(event.new_state() == "open");
                        expanded_user_override_set.set(expanded_override);
                    }
                    class=classes
                >
                    <summary class=LINE_CLASSES>
                        <LogLineSegmentsDiv
                            expanded
                            line_segments={log_block.line_segments.clone()}
                            line_segments_collapsed={log_block.line_segments_collapsed.clone()}
                            children_collapsed_text={log_block.children_collapsed_text.clone()}
                        />
                    </summary>
                    <div>
                        <For
                            each=move || log_block.children.clone().into_iter().enumerate()
                            key=|(_index, log_block)| log_block.hash_with_default_hasher()
                            children=move |(block_index, log_block)| view! {
                                <LogBlockDiv
                                    log_block
                                    block_expand_level
                                    block_index
                                />
                            }
                        />
                    </div>
                </details>
            </div>
        })
    }
    .into_any()
}

/// Returns the CSS classes for the first line of a log block.
fn first_line_css_classes(
    base_classes: &'static str,
    log_block: &LogBlock<'static>,
    block_index: usize,
) -> Cow<'static, str> {
    match log_block.nesting_level {
        // On second level blocks, cycle through background colours.
        1 => {
            let bg_colour = BLOCK_BG_COLOURS[log_block
                .group_number
                .into_inner()
                .try_into()
                .unwrap_or(block_index)
                % BLOCK_BG_COLOURS.len()];
            Cow::Owned(format!("{base_classes} {bg_colour}"))
        }
        _ => Cow::Borrowed(base_classes),
    }
}
