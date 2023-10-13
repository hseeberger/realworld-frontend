/*
 * realworld-backend
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericError {
    #[serde(rename = "errors")]
    pub errors: Box<crate::models::GenericErrorBody>,
}

impl GenericError {
    pub fn new(errors: crate::models::GenericErrorBody) -> GenericError {
        GenericError {
            errors: Box::new(errors),
        }
    }
}
