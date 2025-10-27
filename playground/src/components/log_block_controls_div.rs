use leptos::{
    prelude::{
        component, Callable, Callback, ClassAttribute, ElementChild, GlobalAttributes, OnAttribute,
    },
    view, IntoView,
};

/// Buttons to affect how [`LogBlockDiv`]s are displayed.
#[component]
pub fn LogBlockControlsDiv(#[prop(into)] squish_all_others: Callback<()>) -> impl IntoView {
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
