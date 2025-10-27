use std::borrow::Cow;

use leptos::{
    component,
    either::Either,
    prelude::{
        Callable, Callback, ClassAttribute, ElementChild, For, Get, GlobalAttributes, Memo,
        OnAttribute,
    },
    view, IntoView,
};
use stacktrace::sem_log::LogLineSegment;

use crate::components::LogLineSegmentSpan;

#[component]
pub fn LogLineSegmentsDiv(
    expanded: Memo<bool>,
    line_segments: Vec<LogLineSegment<'static>>,
    line_segments_collapsed: Vec<LogLineSegment<'static>>,
    children_collapsed_text: Cow<'static, str>,
    squish_all_others: Option<impl Fn() + Copy + Send + Sync + 'static>,
) -> impl IntoView {
    view! {
        <div class="flex">
            <div class="grow">
                <For
                    each=move || {
                        if expanded.get() {
                            line_segments.clone()
                        } else {
                            line_segments_collapsed.clone()
                        }
                    }
                    key=LogLineSegment::hash_with_default_hasher
                    children=|line_segment| view! { <LogLineSegmentSpan line_segment /> }
                />
                { move || {
                    if !expanded.get() {
                        let children_collapsed_text = children_collapsed_text.clone().to_string();
                        Either::Left(view! {
                            <div class="border border-gray-800 bg-gray-600 rounded px-1 ml-4">
                                {children_collapsed_text}
                            </div>
                        })
                    } else {
                        Either::Right(())
                    }
                }}
            </div>
            { move || {
                if let Some(squish_all_others) = squish_all_others {
                    Either::Left(view! {
                        <LogBlockControlsDiv
                            squish_all_others
                        />
                    })
                } else {
                    Either::Right(())
                }
            }}
        </div>
    }
}

/// Buttons to affect how [`LogBlockDiv`]s are displayed.
#[component]
fn LogBlockControlsDiv(#[prop(into)] squish_all_others: Callback<()>) -> impl IntoView {
    view! {
        <div class="flex-none opacity-0 group-hover:opacity-100">
            <button
                class="\
                    px-1 \
                    py-1 \
                    rounded \
                    border \
                    border-slate-800 \
                    bg-slate-600 \
                    hover:border-slate-600 \
                    hover:bg-slate-400 \
                    active:border-slate-900 \
                    active:bg-slate-700 \
                "
                on:click={ move |_| squish_all_others.run(()) }
                title="Squish other blocks that are not part of this group"
            >
                "🥞"
            </button>
        </div>
    }
}
