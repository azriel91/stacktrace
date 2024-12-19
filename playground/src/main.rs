// Because we don't build the crate as an `rlib` (for `trunk` to work), we
// declare the module twice.
//
// Perhaps a better solution is to rename the binary, so we don't compile the
// modules twice.
pub mod app;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use crate::app::{shell, App};
    use axum::Router;
    use leptos::{logging::log, prelude::*};
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "csr")]
pub fn main() {
    use leptos::logging::log;

    use crate::app::App;

    // client-side main function
    // so that this can work with e.g. Trunk for a purely client-side app
    // _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log!("csr mode - mounting to body");

    leptos::mount::mount_to_body(App);
}

#[cfg(all(not(feature = "ssr"), not(feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
