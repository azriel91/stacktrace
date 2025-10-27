use leptos::{
    prelude::{
        component, Callable, Callback, ClassAttribute, ElementChild, GlobalAttributes, OnAttribute,
    },
    view, IntoView,
};

use crate::components::{LOG_BLOCK_CONTROLS_DIV_BUTTON_CLASSES, LOG_BLOCK_CONTROLS_DIV_CLASSES};

/// Buttons to affect how [`LogBlockDiv`]s are displayed.
///
/// Currently the following buttons are available:
///
/// * Squish all other blocks that are not part of this group.
#[component]
pub fn LogBlockControlsDiv(#[prop(into)] squish_all_others: Callback<()>) -> impl IntoView {
    view! {
        <div class=LOG_BLOCK_CONTROLS_DIV_CLASSES>
            <button
                class=LOG_BLOCK_CONTROLS_DIV_BUTTON_CLASSES
                on:click={ move |_| squish_all_others.run(()) }
                title="Squish other blocks that are not part of this group"
            >
                "🥞"
            </button>
        </div>
    }
}
