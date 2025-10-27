use leptos::{
    either::Either,
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
/// * Unsquish this group.
#[component]
pub fn LogBlockSquishedControlsDiv(
    #[prop(optional_no_strip)] unsquish_this: Option<Callback<()>>,
) -> impl IntoView {
    view! {
        <div class=LOG_BLOCK_CONTROLS_DIV_CLASSES>
            { move || {
                if let Some(unsquish_this) = unsquish_this {
                    Either::Left(view! {
                        <button
                            class=LOG_BLOCK_CONTROLS_DIV_BUTTON_CLASSES
                            on:click={ move |_| unsquish_this.run(()) }
                            title="Unsquish this group"
                        >
                            "👁️"
                        </button>
                    })
                } else {
                    Either::Right(())
                }
            }}
        </div>
    }
}
