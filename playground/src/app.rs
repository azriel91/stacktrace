use std::{
    hash::{DefaultHasher, Hash, Hasher},
    time::Duration,
};

use leptos::{
    component,
    control_flow::For,
    hydration::{AutoReload, HydrationScripts},
    prelude::{
        event_target_value, signal, ClassAttribute, ElementChild, Get, GlobalAttributes, IntoAny,
        IntoView, LeptosOptions, OnAttribute, PropAttribute, RwSignal, Signal, Write,
    },
    view,
};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, RoutingProgress},
    StaticSegment,
};
use stacktrace::{Section, Stacktrace};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

const H1_CLASSES: &str = "\
    font-bold \
    font-mono \
    text-3xl \
    mb-4 \
";

const MAIN_CLASSES: &str = "\
    bg-slate-800 \
    text-slate-100 \
    \
    h-dvh \
    w-dvw \
    p-8 \
";

const STACKTRACE_TEXT_CLASSES: &str = "\
    bg-slate-900 \
    text-slate-100 \
    font-mono \
    \
    h-64 \
    w-full \
    lg:w-2/5 \
    p-4 \
    rounded-lg \
    shadow-[inset_0_2px_4px_0_rgba(0,0,0,0.3)] \
";

const STACKTRACE_TEXT_PLACEHOLDER: &str = r#"Paste a stack trace, e.g.

Exception in thread "main" java.lang.IllegalStateException
        at com.example.adder.app.App.run(App.java:21)
        at com.example.adder.app.App.main(App.java:14)
Caused by: com.example.adder..AdderException
        at com.example.adder.Adder.add(Adder.java:13)
        ... 2 more
"#;

const STACKTRACE_DIV_CLASSES: &str = "\
    bg-slate-700 \
    text-slate-100 \
    font-mono \
    \
    h-64 \
    w-full \
    lg:w-2/5 \
    p-4 \
    rounded-lg \
    shadow-[inset_0_2px_4px_0_rgba(0,0,0,0.3)] \
    \
    overflow-scroll \
    text-nowrap \
";

const SECTION_DIV_CLASSES: &str = "\
";

const SECTION_SLICE_COMMON_CLASSES: &str = "\
    opacity-20 \
";

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let site_prefix = option_env!("SITE_PREFIX").unwrap_or("");

    let (is_routing, set_is_routing) = signal(false);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/stacktrace.css"/>

        // sets the document title
        <Title text="stacktrace • azriel.im"/>

        // content for this welcome page
        <Router set_is_routing>
            <div class="routing-progress">
                <RoutingProgress is_routing max_time=Duration::from_millis(250)/>
            </div>
            <main class=MAIN_CLASSES>
                <Routes fallback=RouterFallback>
                    <Route path=StaticSegment(site_prefix) view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let stacktrace_str = RwSignal::new(String::new());
    let stacktrace_on_input = move |ev| *stacktrace_str.write() = event_target_value(&ev);
    let stacktrace = Signal::derive(move || Stacktrace::from(stacktrace_str.get().as_str()));

    view! {
        <h1 class=H1_CLASSES>
            "stacktrace"
        </h1>

        <textarea
            class=STACKTRACE_TEXT_CLASSES
            on:input=stacktrace_on_input
            placeholder=STACKTRACE_TEXT_PLACEHOLDER
            prop:value={
                move || stacktrace_str.get()
            }
        />

        <StacktraceDiv stacktrace />
    }
}

#[component]
fn RouterFallback() -> impl IntoView {
    let location = leptos_router::hooks::use_location();
    let pathname = move || location.pathname.get();

    view! {
        <p>"Path not found: " {pathname}</p>
    }
}

#[component]
fn StacktraceDiv(stacktrace: Signal<Stacktrace>) -> impl IntoView {
    view! {
        <div class=STACKTRACE_DIV_CLASSES>
            <For
                each=move || stacktrace.get().sections.clone()
                key=section_hash
                children=|section: Section| view! { <SectionDiv section /> }
            />
        </div>
    }
}

#[component]
fn SectionDiv(section: Section) -> impl IntoView {
    view! {
        <div class=SECTION_DIV_CLASSES>
            <span class=SECTION_SLICE_COMMON_CLASSES>{section.slice_common_with_ancestors().to_string()}</span>
            <span>{section.slice_remainder().to_string()}</span>
            <div>
                <For
                    each=move || section.child_sections.clone()
                    key=section_hash
                    children=|child_section: Section| view! { <SectionDiv section=child_section /> }
                />
            </div>
        </div>
    }.into_any()
}

fn section_hash(section: &Section) -> u64 {
    let mut hasher = DefaultHasher::new();
    section.hash(&mut hasher);
    hasher.finish()
}
