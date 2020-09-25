use crate::models::registration::{ClaimsRequest, ClaimsResponse, SajuAuthClaims};
use reqwest::{Client, StatusCode};

pub struct AuthClient {
    saju_auth_firebase_url: String,
    saju_api_key: String,
}

impl AuthClient {
    pub fn new(api_url: &str, saju_api_key: &str) -> AuthClient {
        let firebase_auth_url = format!("{}/users/claims", api_url);
        AuthClient {
            saju_auth_firebase_url: firebase_auth_url.into(),
            saju_api_key: saju_api_key.into(),
        }
    }
    pub async fn add_claims(&self, uid: &str, claims: SajuAuthClaims) -> ClaimsResponse {
        log::info!("adding claims for user {}", &uid,);
        let client = Client::new();
        let request_body = ClaimsRequest::new(uid, claims);
        let res = client
            .put(&self.saju_auth_firebase_url)
            .header("Api-Key", &self.saju_api_key)
            .json(&request_body)
            .send()
            .await
            .unwrap();

        if res.status().clone() != StatusCode::OK {
            log::error!("{:?}", res);
            todo!("Implement error handling")
        }

        let response = res.json::<ClaimsResponse>().await.unwrap();
        response
    }
}
