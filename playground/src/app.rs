use std::{collections::HashMap, time::Duration};

use leptos::{
    component,
    hydration::{AutoReload, HydrationScripts},
    prelude::{
        event_target_value, signal, ClassAttribute, ElementChild, Get, GlobalAttributes, IntoView,
        LeptosOptions, Memo, OnAttribute, PropAttribute, RwSignal, Set, Signal, Write,
    },
    view,
};
use leptos_meta::{MetaTags, Title};
use leptos_router::{
    components::{Route, Router, Routes, RoutingProgress, A},
    StaticSegment,
};
use reactive_stores::Store;
use stacktrace::{sem_log::SemLog, LogParser};

use crate::{
    components::SemLogViewerDiv,
    state::{SemLogViewerState, SemLogViewerStateStoreFields},
};

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

const PAGE_CLASSES: &str = "\
    bg-slate-800 \
    text-slate-100 \
    \
    h-dvh \
    w-dvw \
    p-8 \
    \
    flex \
    flex-col \
    justify-start \
";

const H1_CLASSES: &str = "\
    font-bold \
    font-mono \
    text-3xl \
";

const NAV_CLASSES: &str = "\
    bg-slate-800 \
    text-slate-100 \
    flex \
    pb-8 \
";

const NAV_SPACER_CLASSES: &str = "\
    grow \
";

const MAIN_CLASSES: &str = "\
    flex \
    justify-center \
";

const HOMEPAGE_CLASSES: &str = "\
    h-full \
    w-full \
    lg:max-w-7xl \
";

const STACKTRACE_TEXT_CLASSES: &str = "\
    bg-slate-900 \
    text-slate-100 \
    font-mono \
    \
    h-64 \
    w-full \
    lg:max-w-7xl \
    p-4 \
    rounded-lg \
    shadow-[inset_0_2px_4px_0_rgba(0,0,0,0.3)] \
    \
    overflow-scroll \
    text-nowrap \
";

const STACKTRACE_TEXT_PLACEHOLDER: &str = r#"Paste a stack trace or use a sample, e.g.

Exception in thread "main" java.lang.IllegalStateException
        at com.example.adder.app.App.run(App.java:21)
        at com.example.adder.app.App.main(App.java:14)
Caused by: com.example.adder..AdderException
        at com.example.adder.Adder.add(Adder.java:13)
        ... 2 more
"#;

const STACKTRACE_SAMPLES_DIV_CLASSES: &str = "\
    flex \
    justify-end \
    w-full \
    lg:max-w-7xl \
";

const STACKTRACE_SAMPLE_JAVA_1: &str = r#"java.lang.IllegalArgumentException: foo
    at com.example.stacktrace.Example.fail(Example.java:11)
    at sun.reflect.NativeMethodAccessorImpl.invoke0(Native Method)
    at sun.reflect.NativeMethodAccessorImpl.invoke(NativeMethodAccessorImpl.java:62)
    at sun.reflect.DelegatingMethodAccessorImpl.invoke(DelegatingMethodAccessorImpl.java:43)
    at java.lang.reflect.Method.invoke(Method.java:483)
    at org.springframework.web.method.support.InvocableHandlerMethod.doInvoke(InvocableHandlerMethod.java:221)
    at org.springframework.web.method.support.InvocableHandlerMethod.invokeForRequest(InvocableHandlerMethod.java:136)
    at org.springframework.web.servlet.mvc.method.annotation.ServletInvocableHandlerMethod.invokeAndHandle(ServletInvocableHandlerMethod.java:114)
    at org.springframework.web.servlet.mvc.method.annotation.RequestMappingHandlerAdapter.invokeHandlerMethod(RequestMappingHandlerAdapter.java:827)
    at org.springframework.web.servlet.mvc.method.annotation.RequestMappingHandlerAdapter.handleInternal(RequestMappingHandlerAdapter.java:738)
    at org.springframework.web.servlet.mvc.method.AbstractHandlerMethodAdapter.handle(AbstractHandlerMethodAdapter.java:85)
    at org.springframework.web.servlet.DispatcherServlet.doDispatch(DispatcherServlet.java:963)
    at org.springframework.web.servlet.DispatcherServlet.doService(DispatcherServlet.java:897)
    at org.springframework.web.servlet.FrameworkServlet.processRequest(FrameworkServlet.java:970)
    at org.springframework.web.servlet.FrameworkServlet.doGet(FrameworkServlet.java:861)
    at javax.servlet.http.HttpServlet.service(HttpServlet.java:622)
    at org.springframework.web.servlet.FrameworkServlet.service(FrameworkServlet.java:846)
    at javax.servlet.http.HttpServlet.service(HttpServlet.java:729)
    at org.apache.tomcat.websocket.server.WsFilter.doFilter(WsFilter.java:52)
    at com.example.stacktrace.servlet.NormalStrategy.doFilter(NormalStrategy.java:42)
    at com.example.stacktrace.servlet.LogbookFilter.doFilter(LogbookFilter.java:33)
    at com.example.stacktrace.servlet.HttpFilter.doFilter(HttpFilter.java:32)
    at org.springframework.boot.actuate.trace.WebRequestTraceFilter.doFilterInternal(WebRequestTraceFilter.java:105)
    at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:107)
    at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:317)
    at org.springframework.security.web.access.intercept.FilterSecurityInterceptor.invoke(FilterSecurityInterceptor.java:127)
    at org.springframework.security.web.access.intercept.FilterSecurityInterceptor.doFilter(FilterSecurityInterceptor.java:91)
    at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:331)
    at org.springframework.security.web.access.ExceptionTranslationFilter.doFilter(ExceptionTranslationFilter.java:115)
    at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:331)
    at org.springframework.security.web.session.SessionManagementFilter.doFilter(SessionManagementFilter.java:137)
    at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:331)
    at org.springframework.security.web.servletapi.SecurityContextHolderAwareRequestFilter.doFilter(SecurityContextHolderAwareRequestFilter.java:169)
    at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:331)
    at org.springframework.security.web.savedrequest.RequestCacheAwareFilter.doFilter(RequestCacheAwareFilter.java:63)
    at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:331)
    at org.springframework.security.oauth2.provider.authentication.OAuth2AuthenticationProcessingFilter.doFilter(OAuth2AuthenticationProcessingFilter.java:176)
    at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:331)
    at org.springframework.security.web.authentication.logout.LogoutFilter.doFilter(LogoutFilter.java:121)
    at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:331)
    at org.springframework.security.web.header.HeaderWriterFilter.doFilterInternal(HeaderWriterFilter.java:66)
    at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:107)
    at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:331)
    at org.springframework.security.web.context.SecurityContextPersistenceFilter.doFilter(SecurityContextPersistenceFilter.java:105)
    at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:331)
    at org.springframework.security.web.context.request.async.WebAsyncManagerIntegrationFilter.doFilterInternal(WebAsyncManagerIntegrationFilter.java:56)
    at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:107)
    at org.springframework.security.web.FilterChainProxy$VirtualFilterChain.doFilter(FilterChainProxy.java:331)
    at org.springframework.security.web.FilterChainProxy.doFilterInternal(FilterChainProxy.java:214)
    at org.springframework.security.web.FilterChainProxy.doFilter(FilterChainProxy.java:177)
    at org.springframework.web.filter.DelegatingFilterProxy.invokeDelegate(DelegatingFilterProxy.java:346)
    at org.springframework.web.filter.DelegatingFilterProxy.doFilter(DelegatingFilterProxy.java:262)
    at com.example.stacktrace.servlet.SecurityStrategy.doFilter(SecurityStrategy.java:32)
    at com.example.stacktrace.servlet.LogbookFilter.doFilter(LogbookFilter.java:33)
    at com.example.stacktrace.servlet.HttpFilter.doFilter(HttpFilter.java:32)
    at org.springframework.web.filter.CharacterEncodingFilter.doFilterInternal(CharacterEncodingFilter.java:197)
    at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:107)
    at org.springframework.boot.actuate.autoconfigure.MetricsFilter.doFilterInternal(MetricsFilter.java:107)
    at org.springframework.web.filter.OncePerRequestFilter.doFilter(OncePerRequestFilter.java:107)
    at com.example.stacktrace.servlet.TracerFilter.doFilter(TracerFilter.java:33)
    at com.example.stacktrace.servlet.HttpFilter.doFilter(HttpFilter.java:28)
"#;

const _STACKTRACE_SAMPLE_JAVA_2: &str = r#"Exception in thread "main" java.lang.IllegalArgumentException: foo
    at com.example.stacktrace.Example.fail(Example.java:11)
    at java.lang.Thread.run(Thread.java:750)
Caused by: com.example.stacktrace.Example$Exception: bar
    at com.example.stacktrace.Example.fail(Example.java:12)
... 2 more
"#;

const STACKTRACE_SAMPLE_RUST: &str = r#"stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:76
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:60
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1030
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:64
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:196
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
  12: std::panicking::continue_panic_fmt
             at src/libstd/panicking.rs:380
  13: rust_begin_unwind
             at src/libstd/panicking.rs:307
  14: core::panicking::panic_fmt
             at src/libcore/panicking.rs:85
  15: core::result::unwrap_failed
             at src/libcore/result.rs:1165
  16: rustc_typeck::check::Inherited::register_predicate
  17: rustc_typeck::check::Inherited::register_predicates
  18: <rustc_typeck::check::FnCtxt as rustc_typeck::astconv::AstConv>::normalize_ty
  19: <dyn rustc_typeck::astconv::AstConv>::res_to_ty
  20: <dyn rustc_typeck::astconv::AstConv>::ast_ty_to_ty
  21: rustc_typeck::check::FnCtxt::check_argument_types
  22: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::confirm_builtin_call
  23: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
  24: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  25: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  26: rustc_typeck::check::FnCtxt::check_decl_initializer
  27: rustc_typeck::check::FnCtxt::check_decl_local
  28: rustc_typeck::check::FnCtxt::check_stmt
  29: rustc_typeck::check::FnCtxt::check_block_with_expected
  30: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  31: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  32: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  33: rustc_typeck::check::check_fn
  34: rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt>::check_expr_closure
  35: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  36: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  37: rustc_typeck::check::FnCtxt::check_argument_types
  38: rustc_typeck::check::FnCtxt::check_method_argument_types
  39: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  40: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  41: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  42: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  43: rustc_typeck::check::FnCtxt::check_decl_initializer
  44: rustc_typeck::check::FnCtxt::check_decl_local
  45: rustc_typeck::check::FnCtxt::check_stmt
  46: rustc_typeck::check::FnCtxt::check_block_with_expected
  47: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  48: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  49: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match
  50: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  51: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  52: rustc_typeck::check::FnCtxt::check_block_with_expected
  53: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  54: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  55: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match
  56: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  57: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  58: rustc_typeck::check::FnCtxt::check_block_with_expected
  59: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  60: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  61: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  62: rustc_typeck::check::check_fn
  63: rustc::ty::context::GlobalCtxt::enter_local
  64: rustc_typeck::check::typeck_tables_of
  65: rustc::ty::query::__query_compute::typeck_tables_of
  66: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  67: rustc::dep_graph::graph::DepGraph::with_task_impl
  68: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  69: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  70: rustc_typeck::check::typeck_item_bodies
  71: rustc::ty::query::__query_compute::typeck_item_bodies
  72: rustc::dep_graph::graph::DepGraph::with_task_impl
  73: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  74: rustc::util::common::time
  75: rustc_typeck::check_crate
  76: rustc_interface::passes::analysis
  77: rustc::ty::query::__query_compute::analysis
  78: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  79: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  80: rustc_interface::passes::create_global_ctxt::{{closure}}
  81: rustc_interface::interface::run_compiler_in_existing_thread_pool
  82: std::thread::local::LocalKey<T>::with
  83: scoped_tls::ScopedKey<T>::set
  84: syntax::with_globals
"#;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    leptos_meta::provide_meta_context();

    // Provides context to allow interactions on one [`LogBlock`] to control
    // visibility of other [`LogBlock`]s
    leptos::prelude::provide_context(Store::new(SemLogViewerState::default()));

    let site_prefix = option_env!("SITE_PREFIX").unwrap_or("");

    let (is_routing, set_is_routing) = signal(false);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        // <Stylesheet id="leptos" href="/pkg/stacktrace.css"/>

        // sets the document title
        <Title text="stacktrace • azriel.im"/>

        // content for this welcome page
        <Router set_is_routing>
            <div class=PAGE_CLASSES>
                <div class="routing-progress">
                    <RoutingProgress is_routing max_time=Duration::from_millis(250)/>
                </div>
                <nav class=NAV_CLASSES>
                    <h1 class=H1_CLASSES>"stacktrace"</h1>
                    <div class=NAV_SPACER_CLASSES />
                    <A
                        href="https://github.com/azriel91/stacktrace"
                        target="_blank"
                    >"🐙 github"</A>
                </nav>
                <main class=MAIN_CLASSES>
                    <Routes fallback=RouterFallback>
                        <Route path=StaticSegment(site_prefix) view=HomePage />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let sem_log_viewer_state = leptos::prelude::expect_context::<Store<SemLogViewerState>>();
    let log_str = RwSignal::new(String::new());
    let log_on_input = move |ev| *log_str.write() = event_target_value(&ev);
    let sem_log_result = Memo::new(move |_previous| {
        let log_str = log_str.get();
        LogParser::parse_from_str(log_str.as_str())
            .map(SemLog::from)
            .map(|sem_log| sem_log.into_static())
    });
    let sem_log = Signal::derive(move || {
        let sem_log = sem_log_result.get().ok();

        if let Some(sem_log) = sem_log.as_ref() {
            let log_block_group_numbers_to_prefix_next =
                sem_log.log_blocks.iter().enumerate().fold(
                    HashMap::with_capacity(sem_log.log_blocks.len()),
                    |mut log_block_group_numbers_to_prefix_next, (block_index, log_block)| {
                        if let Some(group_numbers_to_prefix) =
                            log_block.group_numbers_to_prefix.clone()
                        {
                            log_block_group_numbers_to_prefix_next
                                .insert(block_index, group_numbers_to_prefix);
                        }
                        log_block_group_numbers_to_prefix_next
                    },
                );
            sem_log_viewer_state
                .log_block_group_numbers_to_prefix()
                .set(log_block_group_numbers_to_prefix_next);

            sem_log_viewer_state
                .log_block_group_numbers_squished()
                .write()
                .clear();
        }

        sem_log
    });

    view! {
        <div class=HOMEPAGE_CLASSES>
            <StacktraceSamples log_str />
            <textarea
                class=STACKTRACE_TEXT_CLASSES
                on:input=log_on_input
                placeholder=STACKTRACE_TEXT_PLACEHOLDER
                prop:value={
                    move || log_str.get()
                }
            />

            <SemLogViewerDiv sem_log />
        </div>
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
fn StacktraceSamples(log_str: RwSignal<String>) -> impl IntoView {
    let stacktrace_sample_java = move |_| *log_str.write() = STACKTRACE_SAMPLE_JAVA_1.into();
    let stacktrace_sample_rust = move |_| *log_str.write() = STACKTRACE_SAMPLE_RUST.into();
    view! {
        <div class=STACKTRACE_SAMPLES_DIV_CLASSES>
            <span>"Samples:"</span>
            <button on:click=stacktrace_sample_java type="button">"☕ Java"</button>
            <button on:click=stacktrace_sample_rust type="button">"🦀 Rust"</button>
        </div>
    }
}
