use std::borrow::Cow;

use leptos::{
    component,
    either::Either,
    prelude::{
        ClassAttribute, Effect, ElementChild, For, Get, GlobalAttributes, IntoAny, Memo,
        OnAttribute, ReadSignal, Set, Signal, Write, WriteSignal,
    },
    view, IntoView,
};
use reactive_stores::Store;
use stacktrace::sem_log::{GroupNumber, LogBlock, NestingLevel};

use crate::{
    components::LogLineSegmentsDiv,
    state::{SemLogViewerState, SemLogViewerStateStoreFields},
};

const LINE_CLASSES: &str = "\
    border \
    border-transparent \
    hover:border-blue-400 \
    px-2 \
    py-1 \
    rounded-lg \
";

const BLOCK_CLASSES: &str = "\
    rounded-lg \
    border-s \
    border-transparent \
    hover:border-blue-400 \
    group-focus:border \
    group-focus:border-blue-400 \
";

/// Multiple `hover:open` selectors so that we only highlight the border if
/// there are no nested [`LogBlockDiv`]s that are also hovered.
const BLOCK_DETAILS_CLASSES: &str = "\
    rounded-lg \
    border-s \
    border-transparent \
    open:px-2 \
    open:border-blue-400 \
    hover:open:border-blue-200 \
    hover:open:has-[:hover:open]:border-blue-400 \
    group-focus:border \
    group-focus:open:border-blue-200 \
    group-focus:open:has-[:hover:open]:border-blue-400 \
";

const BLOCK_BG_COLOURS: [&str; 6] = [
    "bg-teal-950",
    "bg-sky-950",
    "bg-blue-950",
    "bg-indigo-950",
    "bg-violet-950",
    "bg-purple-950",
];

const BLOCK_SQUARE_CLASSES: &str = "\
    border \
    border-transparent \
    rounded-lg \
    hover:border-blue-400 \
    focus:border-blue-400 \
    inline-block \
    min-w-8 \
    h-8 \
    px-2 \
";

/// Renders a [`LogBlock`] in various forms, such as a collapsed line, a
/// squished square, or fully expanded.
///
/// The `groupings_significant` parameter should be set to `true` when the
/// parent [`LogBlock`] has `group_numbers_to_prefix` set, as this will make it
/// easier to see semantically related blocks.
///
/// # Parameters
///
/// * `log_block`: The log block to display.
/// * `block_expand_level`: The level the user has chosen to expand / collapse
///   all blocks.
/// * `block_index`: The index of the block.
/// * `groupings_significant`: Whether this block should be coloured based on
///   its [`GroupNumber`].
#[component]
pub fn LogBlockDiv(
    log_block: LogBlock<'static>,
    // `#[prop(optional_no_strip)]` is needed so we can pass in `None` for this prop.
    #[prop(optional_no_strip)] block_expand_level: Option<ReadSignal<NestingLevel>>,
    #[prop(optional)] block_index: usize,
    #[prop(optional)] groupings_significance: GroupingsSignificance,
) -> impl IntoView {
    let sem_log_viewer_state = leptos::prelude::expect_context::<Store<SemLogViewerState>>();
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

    let group_number = log_block.group_number;
    let squished = move || {
        if let GroupingsSignificance::Significant {
            nesting_level_parent,
            block_index_parent,
        } = groupings_significance
        {
            let log_block_group_numbers_squished = sem_log_viewer_state
                .log_block_group_numbers_squished()
                .get();

            log_block_group_numbers_squished
                .get(&(nesting_level_parent, block_index_parent))
                .map(|group_numbers_squished| group_numbers_squished.contains(&group_number))
                .unwrap_or(false)
        } else {
            false
        }
    };
    let first_line_css_classes = Signal::derive(move || {
        let classes_base = if is_leaf_block {
            LINE_CLASSES
        } else {
            BLOCK_CLASSES
        };
        first_line_css_classes(
            classes_base,
            group_number,
            groupings_significance,
            block_index,
            squished(),
        )
    });
    view! {
        {
            move || if squished() {
                Either::Left(view! {
                    <LogBlockDivSquished
                        log_block={log_block.clone()}
                        classes=first_line_css_classes
                    />
                })
            } else {
                Either::Right(view! {
                    <LogBlockDivUnsquished
                        log_block={log_block.clone()}
                        block_expand_level
                        block_index
                        groupings_significance
                        classes=first_line_css_classes
                        is_leaf_block
                        expanded
                        expanded_user_override_set
                    />
                })
            }
        }
    }
}

/// Renders a [`LogBlock`] as an `inline-block` coloured square.
///
/// Unsquished means the block is not squished into an `inline-block` square.
#[component]
pub fn LogBlockDivSquished(
    log_block: LogBlock<'static>,
    #[prop(into)] classes: Signal<Cow<'static, str>>,
) -> impl IntoView {
    let line = log_block
        .line_segments_collapsed
        .iter()
        .fold(String::new(), |mut line, segment| {
            line.push_str(&segment.text);
            line.push_str(&segment.separator);
            line
        });
    view! {
        <div
            class={move || classes.get()}
            tabindex="0"
        >
            <div class="\
                max-w-0 \
                group-hover:max-w-3xs \
                group-focus:max-w-3xs \
                text-ellipsis \
                overflow-hidden \
                transition-all \
                duration-300 \
                ease-in-out \
                "
            >
                {line}
            </div>
        </div>
    }
}

/// Renders a [`LogBlock`] with at least the first line.
///
/// Unsquished means the block is not squished into an `inline-block` square.
#[component]
pub fn LogBlockDivUnsquished(
    log_block: LogBlock<'static>,
    block_expand_level: Option<ReadSignal<NestingLevel>>,
    block_index: usize,
    groupings_significance: GroupingsSignificance,
    classes: Signal<Cow<'static, str>>,
    is_leaf_block: bool,
    expanded: Memo<bool>,
    expanded_user_override_set: WriteSignal<Option<bool>>,
) -> impl IntoView {
    let sem_log_viewer_state = leptos::prelude::expect_context::<Store<SemLogViewerState>>();
    let nesting_level = log_block.nesting_level;
    let group_number = log_block.group_number;

    // Handler to squish all other log blocks that have a different `GroupNumber`
    let squish_all_others = if let GroupingsSignificance::Significant {
        nesting_level_parent,
        block_index_parent,
    } = groupings_significance
    {
        Some(move || {
            let log_block_group_numbers_squished =
                sem_log_viewer_state.log_block_group_numbers_squished();

            // iterate over all the groups for the current nesting level, and squish the
            // groups if their `GroupNumber` is not the current block's `GroupNumber`.
            if let Some(group_numbers_to_prefix) = sem_log_viewer_state
                .log_block_group_numbers_to_prefixes()
                .get()
                .get(&nesting_level_parent)
                .and_then(|log_block_group_numbers_to_prefix| {
                    log_block_group_numbers_to_prefix.get(&block_index_parent)
                })
            {
                group_numbers_to_prefix
                    .keys()
                    .copied()
                    .filter(|group_number_other| group_number != *group_number_other)
                    .for_each(|group_number_other| {
                        log_block_group_numbers_squished
                            .write()
                            .entry((nesting_level_parent, block_index_parent))
                            .or_default()
                            .insert(group_number_other);
                    });
            }
        })
    } else {
        None
    };

    if is_leaf_block {
        Either::Left(view! {
            <div
                class={move || classes.get()}
                tabindex="0"
            >
                <LogLineSegmentsDiv
                    expanded
                    line_segments={log_block.line_segments.clone()}
                    line_segments_collapsed={log_block.line_segments_collapsed.clone()}
                    children_collapsed_text={log_block.children_collapsed_text.clone()}
                    squish_all_others
                />
            </div>
        })
    } else {
        // For this block's children.
        let groupings_significance = if log_block.group_numbers_to_prefix.is_some() {
            GroupingsSignificance::Significant {
                nesting_level_parent: nesting_level,
                block_index_parent: block_index,
            }
        } else {
            GroupingsSignificance::NotSignificant
        };

        Either::Right(view! {
            <div
                class={move || classes.get()}
                tabindex="0"
            >
                <details
                    class=BLOCK_DETAILS_CLASSES
                    tabindex="0"
                    open={move || expanded.get()}
                    on:toggle=move |event| {
                        let expanded_override = Some(event.new_state() == "open");
                        expanded_user_override_set.set(expanded_override);
                    }
                >
                    <summary class=LINE_CLASSES>
                        <LogLineSegmentsDiv
                            expanded
                            line_segments={log_block.line_segments.clone()}
                            line_segments_collapsed={log_block.line_segments_collapsed.clone()}
                            children_collapsed_text={log_block.children_collapsed_text.clone()}
                            squish_all_others
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
                                    groupings_significance
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
    group_number: GroupNumber,
    groupings_significance: GroupingsSignificance,
    block_index: usize,
    squished: bool,
) -> Cow<'static, str> {
    match (groupings_significance, squished) {
        (GroupingsSignificance::NotSignificant, false) => Cow::Borrowed(base_classes),
        (GroupingsSignificance::NotSignificant, true) => {
            Cow::Owned(format!("{BLOCK_SQUARE_CLASSES}"))
        }
        // When there are prefix-based group numbers, cycle through background colours.
        (GroupingsSignificance::Significant { .. }, false) => {
            let bg_colour =
                BLOCK_BG_COLOURS[group_number.into_inner().try_into().unwrap_or(block_index)
                    % BLOCK_BG_COLOURS.len()];
            Cow::Owned(format!("{base_classes} group {bg_colour}"))
        }
        (GroupingsSignificance::Significant { .. }, true) => {
            let bg_colour =
                BLOCK_BG_COLOURS[group_number.into_inner().try_into().unwrap_or(block_index)
                    % BLOCK_BG_COLOURS.len()];
            Cow::Owned(format!("group {bg_colour} {BLOCK_SQUARE_CLASSES}"))
        }
    }
}

/// Informs a child [`LogBlockDiv`] whether it should be coloured to
/// semantically match other [`LogBlock`]s based on [`GroupNumber`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GroupingsSignificance {
    /// Used when the [`LogBlockDiv`] is not coloured to semantically match
    /// other [`LogBlock`]s based on [`GroupNumber`].
    #[default]
    NotSignificant,
    /// Used when the [`LogBlockDiv`] is coloured to semantically match other
    /// [`LogBlock`]s based on [`GroupNumber`].
    ///
    /// This also carries information of the parent [`LogBlock`] so that other
    /// [`LogBlockDiv`]s can be collapsed based on handlers.
    Significant {
        /// Nesting level of the parent [`LogBlock`].
        nesting_level_parent: NestingLevel,
        /// Index of the parent [`LogBlock`] within its parent [`LogBlockDiv`].
        block_index_parent: usize,
    },
}
