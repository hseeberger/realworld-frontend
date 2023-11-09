#![feature(fn_traits)]
#![feature(lazy_cell)]

pub mod component;
pub mod page;

use crate::{
    component::NavBar,
    page::{Home, Login, NotFound, Register},
};
use leptos::{component, create_rw_signal, mount_to_body, view, IntoView, Signal, SignalGet, *};
use leptos_router::{use_navigate, Route, Router, Routes};
use log::{debug, Level};
use realworld_backend_client::{apis::configuration::Configuration, models::User};
use std::sync::LazyLock;

static BACKEND_CONFIG: LazyLock<Configuration> = LazyLock::new(|| {
    let base_path = env!("BACKEND").to_string();
    Configuration {
        base_path,
        ..Default::default()
    }
});

fn main() {
    console_log::init_with_level(Level::Debug).expect("initialize console log");
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let user_state = create_rw_signal(None::<User>);
    let logged_in = Signal::derive(move || user_state.get().is_some());

    let on_login_register = move |user| {
        user_state.update(|u| *u = Some(user));
        use_navigate()("/", Default::default());
    };

    let on_logout = move |_| {
        debug!("logged out user {:?}", user_state.get());
        user_state.update(|u| *u = None);
        use_navigate()("/", Default::default());
    };

    view! {
        <Router>
            <NavBar logged_in on_logout />

            <main>
                <Routes>
                    <Route path="/" view=Home />

                    <Route path="/login" view=move || view! {
                        <Login on_success=on_login_register />
                    } />

                    <Route path="/register" view=move || view! {
                        <Register on_success=on_login_register />
                    } />

                    <Route path="/*any" view=NotFound />
                </Routes>
            </main>

            <footer>
                <div class="container">
                    <a href="/" class="logo-font">conduit</a>
                    <span class="attribution">An interactive learning project from <a href="https://thinkster.io">Thinkster</a>. Code & design licensed under MIT.</span>
                </div>
            </footer>
        </Router>
    }
}
