use leptos::{
    component,
    prelude::{ClassAttribute, ElementChild, For, IntoAny},
    view, IntoView,
};
use stacktrace::sem_log::LogBlock;

#[component]
pub fn LogBlockDiv(log_block: LogBlock<'static>) -> impl IntoView {
    view! {
        <div class="border border-gray-300 p-2 rounded">
            {log_block.text}
            <For
                each=move || log_block.children.clone()
                key=LogBlock::hash_with_default_hasher
                children=|log_block| view! { <LogBlockDiv log_block /> }
            />
        </div>
    }
    .into_any()
}
