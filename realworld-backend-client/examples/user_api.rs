use anyhow::{Context, Result};
use realworld_backend_client::{
    apis::{
        configuration::Configuration,
        user_api::{users_login_post, users_post},
    },
    models::{Credentials, LoginRequest, NewUser, RegisterUserRequest},
};

const USERNAME: &str = "john.doe";
const EMAIL: &str = "john.doe@realworld.dev";
const PASSWORD: &str = "123456z?";

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = Configuration::default();

    // let register_user_request =
    //     RegisterUserRequest::new(NewUser::new(USERNAME.into(), EMAIL.into(), PASSWORD.into()));
    // let response = users_post(&configuration, register_user_request).await;
    // println!("{response:?}");

    let request = LoginRequest::new(Credentials::new(EMAIL.into(), "123456z!".into()));
    let response = users_login_post(&configuration, request).await;
    println!("{response:?}");

    let request = LoginRequest::new(Credentials::new(EMAIL.into(), PASSWORD.into()));
    let response = users_login_post(&configuration, request).await;
    println!("{response:?}");

    Ok(())
}
