/*
 * realworld-backend
 *
 * RealWorld backend implementation in Rust.
 *
 * The version of the OpenAPI document: 0.1.1
 * Contact: git@heikoseeberger.de
 * Generated by: https://openapi-generator.tech
 */

/// User : User.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// Bio.
    #[serde(
        rename = "bio",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bio: Option<Option<String>>,
    /// Unique email address, used for login.
    #[serde(rename = "email")]
    pub email: String,
    /// Bearer token used for authentication.
    #[serde(rename = "token")]
    pub token: String,
    /// Unique unsername.
    #[serde(rename = "username")]
    pub username: String,
}

impl User {
    /// User.
    pub fn new(email: String, token: String, username: String) -> User {
        User {
            bio: None,
            email,
            token,
            username,
        }
    }
}
