use crate::BACKEND_CONFIG;
use leptos::{
    component, create_action, create_signal, event_target_value, view, Callback, For, IntoView,
    SignalGet, SignalUpdate,
};
use leptos_router::A;
use log::{debug, error, warn};
use realworld_backend_client::{
    apis::{
        user_api::{login_user, LoginUserError},
        Error,
    },
    models::{Credentials, LoginRequest, UnprocessableEntity, User, UserResponse},
};
use reqwest::StatusCode;

#[component]
pub fn Login(#[prop(into)] on_success: Callback<User>) -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (errors, set_errors) = create_signal(Vec::<String>::new());

    let log_in_action = create_action(move |(email, password): &(String, String)| {
        debug!("trying to log in with email {email}");

        let request = LoginRequest::new(Credentials::new(email.to_owned(), password.to_owned()));
        async move {
            match login_user(&BACKEND_CONFIG, request).await {
                Ok(UserResponse { user }) => {
                    debug!("successfully logged in user {user:?}");
                    on_success(*user);
                }

                Err(error) => match error {
                    Error::ResponseError(error) => match error.status {
                        StatusCode::UNAUTHORIZED => {
                            warn!("UNAUTHORIZED: {error:?}");
                            set_errors.update(|errors| *errors = vec!["Unauthorized".to_string()]);
                        }

                        StatusCode::NOT_FOUND => {
                            warn!("NOT_FOUND: {error:?}");
                            set_errors
                                .update(|errors| *errors = vec!["User not found".to_string()]);
                        }

                        StatusCode::UNPROCESSABLE_ENTITY => {
                            warn!("UNPROCESSABLE_ENTITY: {error:?}");
                            let details = error
                                .entity
                                .and_then(|error| match error {
                                    LoginUserError::Status422(UnprocessableEntity { errors }) => {
                                        Some(errors)
                                    }
                                    _ => None,
                                })
                                .unwrap_or_default();
                            set_errors.update(|errors| {
                                *errors = vec!["Invalid credentials".to_string()];
                                errors.extend(details);
                            });
                        }

                        other => {
                            error!("Cannot login user: {other}");
                            set_errors.update(|errors| {
                                *errors = vec!["Apologies: something bad happened!".to_string()]
                            });
                        }
                    },

                    other => {
                        error!("Cannot login user: {other}");
                        set_errors.update(|errors| {
                            *errors = vec!["Apologies: something bad happened!".to_string()]
                        });
                    }
                },
            }
        }
    });

    let invoke_log_in_action = move || log_in_action.dispatch((email.get(), password.get()));

    view! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Login</h1>

                        <p class="text-xs-center">
                            <A href="/sign-up">Need an account?</A>
                        </p>

                        <ul class="error-messages">
                            <For
                                each=errors
                                key=|error| error.to_owned()
                                children=move |error| {
                                    view! {
                                        <li>{error}</li>
                                    }
                                } />
                        </ul>

                        <form on:submit=|ev| ev.prevent_default()>
                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="text"
                                    placeholder="Email"
                                    on:change=move |evt| {
                                        let value = event_target_value(&evt);
                                        set_email.update(|v| *v = value);
                                    }/>
                            </fieldset>

                            <fieldset class="form-group">
                                <input
                                    class="form-control form-control-lg"
                                    type="password"
                                    placeholder="Password"
                                    on:change=move |evt| {
                                        let value = event_target_value(&evt);
                                        set_password.update(|v| *v = value);
                                    }/>
                            </fieldset>

                            <button
                                class="btn btn-lg btn-primary pull-xs-right"
                                on:click=move |_| invoke_log_in_action()>
                                Login
                            </button>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
