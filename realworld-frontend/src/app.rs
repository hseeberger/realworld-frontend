use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::{info, warn};
use realworld_backend_client::{
    apis::{
        configuration::Configuration,
        user_api::{users_login_post, UsersLoginPostError},
        Error,
    },
    models::{Credentials, LoginRequest, User, UserResponse},
};
use reqwest::StatusCode;

#[derive(Debug, Clone, PartialEq, Eq, Routable)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home,

    #[route("/sign-in")]
    SignIn,

    #[route("/sign-up")]
    SignUp,

    #[route("/profile")]
    Profile,
}

pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || None::<User>);

    cx.render(rsx! { Router::<Route> {} })
}

fn Home(cx: Scope) -> Element {
    let inner = cx.render(rsx! {

        div { class: "home-page",
            div { class: "banner",
                div { class: "container",
                    h1 { class: "logo-font", "conduit" }
                    p { "A place to share your knowledge." }
                }
            }
            div { class: "container", "To be continued ..." }
        }
    });
    cx.render(rsx! { Template { inner: inner } })
}

#[allow(unused_variables)]
fn SignIn(cx: Scope) -> Element {
    let user_state = use_shared_state::<Option<User>>(cx).expect("use_shared_state User");
    let navigator = use_navigator(cx);
    let errors = use_state::<Vec<String>>(cx, Default::default);

    let onsubmit = move |evt: FormEvent| {
        let username = evt
            .values
            .get("username")
            .expect("username")
            .get(0)
            .expect("first username")
            .to_owned();
        let passsword = evt
            .values
            .get("password")
            .expect("password")
            .get(0)
            .expect("first password")
            .to_owned();

        to_owned![user_state];
        to_owned![navigator];
        to_owned![errors];
        cx.spawn(async move {
            let configuration = Configuration::default();
            let request = LoginRequest::new(Credentials::new(username, passsword));
            let response = users_login_post(&configuration, request).await;

            match response {
                Ok(UserResponse { user }) => {
                    let user = *user;
                    info!("successfully logged in user {user:?}");
                    *user_state.write() = Some(user);
                    navigator.push(Route::Home);
                }

                Err(error) => {
                    warn!("could not log in user: {error}");

                    // TODO: This is very messy! Ain't there a nicer way to consume the error?
                    let new_errors = match error {
                        Error::ResponseError(r) => match r.status {
                            StatusCode::UNAUTHORIZED => {
                                vec!["Unauthorized: unknown username or wrong password".into()]
                            }

                            StatusCode::UNPROCESSABLE_ENTITY => {
                                let mut new_errors =
                                    vec!["Username or password not well formatted".into()];
                                let mut es = r
                                    .entity
                                    .and_then(|e| match e {
                                        UsersLoginPostError::Status422(e) => Some(e.errors.body),
                                        _ => None,
                                    })
                                    .unwrap_or_default();
                                new_errors.append(&mut es);
                                new_errors
                            }

                            _ => {
                                vec!["Something bad happened – apologies for the inconvenience!"
                                    .into()]
                            }
                        },

                        _ => {
                            vec!["Something bad happened – apologies for the inconvenience!".into()]
                        }
                    };

                    errors.set(new_errors);
                }
            };
        });
    };

    let inner = cx.render(rsx! {
        div { class: "auth-page",
            div { class: "container page",
                div { class: "row",
                    div { class: "col-md-6 offset-md-3 col-xs-12",
                        h1 { class: "text-xs-center", "Sign in" }
                        p { class: "text-xs-center",
                            Link { to: Route::SignUp, "Need an account?" }
                        }
                        ul { class: "error-messages",
                            for error in errors.get() {
                                li { error.as_str() }
                            }
                        }
                        form { onsubmit: onsubmit,
                            fieldset { class: "form-group",
                                input {
                                    class: "form-control form-control-lg",
                                    r#type: "text",
                                    name: "username",
                                    placeholder: "Email",
                                    required: true
                                }
                            }
                            fieldset { class: "form-group",
                                input {
                                    class: "form-control form-control-lg",
                                    r#type: "password",
                                    name: "password",
                                    placeholder: "Password",
                                    required: true
                                }
                            }
                            button { class: "btn btn-lg btn-primary pull-xs-right", "Sign in" }
                        }
                    }
                }
            }
        }
    });

    cx.render(rsx! { Template { inner: inner } })
}

fn SignUp(cx: Scope) -> Element {
    let inner = cx.render(rsx! {
        div { class: "auth-page",
            div { class: "container page",
                div { class: "row",
                    div { class: "col-md-6 offset-md-3 col-xs-12",
                        h1 { class: "text-xs-center", "Sign up" }
                        p { class: "text-xs-center",
                            Link { to: Route::SignIn, "Already have an account?" }
                        }
                        // ul { class: "error-messages", li { "That email is already taken" } }
                        form {
                            fieldset { class: "form-group", input {
                                class: "form-control form-control-lg",
                                r#type: "text",
                                placeholder: "Username"
                            } }
                            fieldset { class: "form-group", input {
                                class: "form-control form-control-lg",
                                r#type: "text",
                                placeholder: "Email"
                            } }
                            fieldset { class: "form-group", input {
                                class: "form-control form-control-lg",
                                r#type: "password",
                                placeholder: "Password"
                            } }
                            button { class: "btn btn-lg btn-primary pull-xs-right", "Sign up" }
                        }
                    }
                }
            }
        }
    });

    cx.render(rsx! { Template { inner: inner } })
}

fn Profile(cx: Scope) -> Element {
    let inner = cx.render(rsx! { div { "Profile" } });

    cx.render(rsx! { Template { inner: inner } })
}

#[inline_props]
#[allow(unused_variables)]
pub fn Template<'a>(cx: Scope<'a>, inner: Element<'a>) -> Element {
    cx.render(rsx! {
        Navbar {}
        &cx.props.inner
    })
}

fn Navbar(cx: Scope) -> Element {
    let user_state = use_shared_state::<Option<User>>(cx).expect("use_shared_state User");
    let user = user_state.read().as_ref().cloned();

    cx.render(rsx! {
        nav { class: "navbar navbar-light",
            div { class: "container",
                Link { class: "navbar-brand", to: Route::Home {}, "conduit" }
                ul { class: "nav navbar-nav pull-xs-right",
                    li { class: "nav-item",
                        Link { to: Route::Home, class: "nav-link", active_class: "active", "Home" }
                    }
                    NavbarPart { user: user }
                }
            }
        }
    })
}

#[inline_props]
fn NavbarPart(cx: Scope, #[props(!optional)] user: Option<User>) -> Element {
    match user {
        Some(_user) => cx.render(rsx! {
            li { class: "nav-item",
                Link { to: Route::SignIn, class: "nav-link", active_class: "active", "Profile" }
            }
        }),

        None => cx.render(rsx! {
            li { class: "nav-item",
                Link { to: Route::SignIn, class: "nav-link", active_class: "active", "Sign in" }
            }
            li { class: "nav-item",
                Link { to: Route::SignUp, class: "nav-link", active_class: "active", "Sign up" }
            }
        }),
    }
}
