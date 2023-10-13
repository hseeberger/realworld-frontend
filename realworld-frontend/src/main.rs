#![allow(non_snake_case)]

mod app;

use crate::app::App;
use log::{info, LevelFilter};

fn main() {
    console_error_panic_hook::set_once();
    dioxus_logger::init(LevelFilter::Debug).expect("init dioxus_logger");

    info!("starting realworld-frontend");
    dioxus_web::launch(App);
}
