use cfg_if::cfg_if;
pub mod app;
pub mod for_test_component;
pub mod for_test_enumerate_component;
pub mod error_template;
pub mod fileserv;
//#[macro_use]
//extern crate tracing;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;
    use tracing_subscriber::util::SubscriberInitExt;


    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate

        tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .without_time()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_writer(tracing_subscriber_wasm::MakeConsoleWriter::default())
        .with_ansi(false)
        .pretty()
        .finish()
        .init();

        console_error_panic_hook::set_once();

        leptos::log!("THIS IS LOGGED");
        tracing::debug!("What is going on?");

        leptos::mount_to_body(move |cx| {
            view! { cx, <App/> }
        });
    }
}}
