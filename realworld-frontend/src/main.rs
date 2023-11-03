pub mod component;
pub mod page;

use crate::{
    component::NavBar,
    page::{Home, Login, NotFound, Register},
};
use leptos::{component, create_rw_signal, mount_to_body, view, IntoView, Signal, SignalGet};
use leptos_router::{Route, Router, Routes};
use log::Level;
use realworld_backend_client::models::User;

fn main() {
    console_log::init_with_level(Level::Debug).expect("initialize console log");
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let user = create_rw_signal(None::<User>);
    let logged_in = Signal::derive(move || user.get().is_some());

    view! {
        <Router>
            <NavBar logged_in=logged_in />

            <main>
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/login" view=Login />
                    <Route path="/register" view=Register />
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
