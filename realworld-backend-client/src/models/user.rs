/*
 * realworld-backend
 *
 * RealWorld backend implementation in Rust.
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: git@heikoseeberger.de
 * Generated by: https://openapi-generator.tech
 */

/// User : A user.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(
        rename = "bio",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub bio: Option<Option<String>>,
    /// An email address.
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "username")]
    pub username: String,
}

impl User {
    /// A user.
    pub fn new(email: String, token: String, username: String) -> User {
        User {
            bio: None,
            email,
            token,
            username,
        }
    }
}
