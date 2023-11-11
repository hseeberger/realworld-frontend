use leptos::{component, view, Callback, Children, IntoView, Show, Signal};
use leptos_router::A;

#[component]
pub fn NavBar(logged_in: Signal<bool>, #[prop(into)] on_logout: Callback<()>) -> impl IntoView {
    view! {
        <nav class="navbar navbar-light">
            <div class="container">
                <A class="navbar-brand" href="/">conduit</A>

                <ul class="nav navbar-nav pull-xs-right">
                    <NavItem href="/">Home</NavItem>

                    <Show
                        when=move || !logged_in()
                        fallback=move || view! {
                            <NavItem
                                href="#"
                                active_class=""
                                on:click=move |_| on_logout.call(((),))>Logout</NavItem>
                        }>

                        <NavItem href="/login">Login</NavItem>
                        <NavItem href="/register">Register</NavItem>
                    </Show>
                </ul>
            </div>
        </nav>
    }
}

#[component]
fn NavItem(
    href: &'static str,
    #[prop(default = "active")] active_class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <li class="nav-item">
            <A class="nav-link" href active_class>{children()}</A>
        </li>
    }
}
