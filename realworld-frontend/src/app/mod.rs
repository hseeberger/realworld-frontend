mod component;
mod page;

use self::{
    component::NavBar,
    page::{Home, Login, Register},
};
use leptos::{component, create_rw_signal, view, IntoView, Signal, SignalGet, SignalUpdate};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{use_navigate, Route, Router, Routes};
use log::debug;
use realworld_backend_client::models::User;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

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
        <Title text="Conduit"/>

        <Stylesheet href="//demo.productionready.io/main.css"/>
        <Stylesheet href="//code.ionicframework.com/ionicons/2.0.1/css/ionicons.min.css"/>
        <Stylesheet href="//fonts.googleapis.com/css?family=Titillium+Web:700|Source+Serif+Pro:400,700|Merriweather+Sans:400,700|Source+Sans+Pro:400,300,600,700,300italic,400italic,600italic,700italic"/>

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
