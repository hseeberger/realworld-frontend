use crate::BACKEND_CONFIG;
use leptos::{
    component, create_action, create_signal, event_target_value, view, Callback, For, IntoView,
    SignalGet, SignalUpdate,
};
use leptos_router::A;
use log::{debug, error, warn};
use realworld_backend_client::{
    apis::{
        user_api::{register_user, RegisterUserError},
        Error,
    },
    models::{Conflict, NewUser, RegisterUserRequest, UnprocessableEntity, User, UserResponse},
};
use reqwest::StatusCode;

#[component]
pub fn Register(#[prop(into)] on_success: Callback<User>) -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (errors, set_errors) = create_signal(Vec::<String>::new());

    let register_action = create_action(
        move |(username, email, password): &(String, String, String)| {
            debug!("trying to register with username {username} and email {email}");

            let request = RegisterUserRequest::new(NewUser::new(
                email.to_owned(),
                password.to_owned(),
                username.to_owned(),
            ));
            async move {
                match register_user(&BACKEND_CONFIG, request).await {
                    Ok(UserResponse { user }) => {
                        debug!("successfully registered user {user:?}");
                        on_success(*user);
                    }

                    Err(error) => match error {
                        Error::ResponseError(error) => match error.status {
                            StatusCode::CONFLICT => {
                                warn!("CONFLICT: {error:?}");
                                let detail = error.entity.and_then(|error| match error {
                                    RegisterUserError::Status409(Conflict { error }) => Some(error),
                                    _ => None,
                                });
                                set_errors.update(|errors| {
                                    *errors = vec!["Conflicting user data".to_string()];
                                    errors.extend(detail);
                                });
                            }

                            StatusCode::UNPROCESSABLE_ENTITY => {
                                warn!("UNPROCESSABLE_ENTITY: {error:?}");
                                let details = error
                                    .entity
                                    .and_then(|error| match error {
                                        RegisterUserError::Status422(UnprocessableEntity {
                                            errors,
                                        }) => Some(errors),
                                        _ => None,
                                    })
                                    .unwrap_or_default();
                                set_errors.update(|errors| {
                                    *errors = vec!["Invalid user data".to_string()];
                                    errors.extend(details);
                                });
                            }

                            other => {
                                error!("Cannot register user: {other}");
                                set_errors.update(|errors| {
                                    *errors = vec!["Apologies: something bad happened!".to_string()]
                                });
                            }
                        },

                        other => {
                            error!("Cannot register user: {other}");
                            set_errors.update(|errors| {
                                *errors = vec!["Apologies: something bad happened!".to_string()]
                            });
                        }
                    },
                }
            }
        },
    );

    let invoke_register_action =
        move || register_action.dispatch((username.get(), email.get(), password.get()));

    view! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">Register</h1>

                        <p class="text-xs-center">
                            <A href="/sign-in">Have an account?</A>
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
                                placeholder="Username"
                                on:change=move |evt| {
                                    let value = event_target_value(&evt);
                                    set_username.update(|v| *v = value);
                                }/>
                            </fieldset>

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
                                on:click=move |_| invoke_register_action()>
                                Register
                            </button>

                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
