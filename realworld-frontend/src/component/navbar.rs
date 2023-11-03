use leptos::{component, view, IntoView, Show, Signal};
use leptos_router::A;

#[component]
pub fn NavBar(logged_in: Signal<bool>) -> impl IntoView {
    view! {
        <nav class="navbar navbar-light">
            <div class="container">
                <A class="navbar-brand" href="/">conduit</A>

                <ul class="nav navbar-nav pull-xs-right">
                    <li class="nav-item">
                        <A class="nav-link" href="/">Home</A>
                    </li>

                    <Show
                        when=move || !logged_in()
                        fallback=LogOut>

                        <LogInSignUp />
                    </Show>
                </ul>
            </div>
        </nav>
    }
}

#[component]
fn LogInSignUp() -> impl IntoView {
    view! {
        <li class="nav-item">
            <A class="nav-link" href="/login">Login</A>
        </li>

        <li class="nav-item">
            <A class="nav-link" href="/register">Register</A>
        </li>
    }
}

#[component]
fn LogOut() -> impl IntoView {
    view! {
        <li class="nav-item">
            <A class="nav-link" href="/sign-in">Logout</A>
        </li>
    }
}
