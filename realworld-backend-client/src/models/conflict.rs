/*
 * realworld-backend
 *
 * RealWorld backend implementation in Rust.
 *
 * The version of the OpenAPI document: 0.1.1
 * Contact: git@heikoseeberger.de
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Conflict {
    #[serde(rename = "error")]
    pub error: String,
}

impl Conflict {
    pub fn new(error: String) -> Conflict {
        Conflict { error }
    }
}
