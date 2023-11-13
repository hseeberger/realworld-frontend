#![feature(fn_traits)]
#![feature(lazy_cell)]

pub mod app;
#[cfg(feature = "ssr")]
pub mod server;

use realworld_backend_client::apis::configuration::Configuration;
use std::{env, sync::LazyLock};

static BACKEND_CONFIG: LazyLock<Configuration> = LazyLock::new(|| {
    let base_path = env!("APP_BACKEND").to_string();
    Configuration {
        base_path,
        ..Default::default()
    }
});

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::App;

    // initializes logging using the `log` crate
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
