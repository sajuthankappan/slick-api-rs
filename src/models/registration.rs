use serde::{Deserialize, Serialize};
use wread_mongodb::mongodb::bson::oid::ObjectId;

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterUserParameters {
    #[serde(rename = "registrationCode")]
    pub registration_code: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegistrationCode {
    #[serde(rename = "code")]
    pub code: String,

    #[serde(rename = "groupId")]
    pub group_id: ObjectId,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegistrationResponse {
    pub message: Option<String>,
}

impl RegistrationResponse {
    pub fn new(message: &str) -> RegistrationResponse {
        RegistrationResponse {
            message: Some(message.into()),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SajuAuthClaims {
    pub admin: bool,
    #[serde(rename = "slickUser")]
    pub slick_user: bool,
}

impl SajuAuthClaims {
    pub fn new(admin: bool, slick_user: bool) -> SajuAuthClaims {
        SajuAuthClaims {
            admin: admin,
            slick_user: slick_user,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClaimsResponse {
    pub code: i16,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClaimsRequest {
    pub uid: String,
    pub claims: SajuAuthClaims,
}

impl ClaimsRequest {
    pub fn new(uid: &str, claims: SajuAuthClaims) -> ClaimsRequest {
        ClaimsRequest {
            uid: uid.into(),
            claims: claims,
        }
    }
}
