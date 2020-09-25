use crate::auth::AuthClient;
use crate::data::repositories::{
    audit_detail_repository, audit_summary_repository, group_site_repository,
    registration_code_repository, site_repository,
};
use crate::models::registration::{RegisterUserParameters, RegistrationResponse, SajuAuthClaims};
use std::convert::Infallible;
use warp::http::StatusCode;
use wread_data_mongodb::mongodb::Database;

pub async fn trend_get_handler(
    site_id: String,
    page_id: String,
    audit_profile_id: String,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    log::info!("Getting trend for site {}", &site_id);
    let report = audit_summary_repository::get_trend(&site_id, &page_id, &audit_profile_id, &db)
        .await
        .unwrap();
    Ok(warp::reply::json(&report))
}

pub async fn reports_get_handler(id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    log::info!("Getting report for {}", &id);
    let report = audit_detail_repository::get_by_id(&id, &db).await.unwrap();
    Ok(warp::reply::json(&report))
}

pub async fn reports_delete_handler(
    id: String,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    log::info!("Deleting summary for {}", &id);
    let result = audit_detail_repository::delete(&id.as_str(), &db).await;
    if let Err(err) = result {
        log::error!("{}", err);
        Ok(StatusCode::INTERNAL_SERVER_ERROR)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

pub async fn summaries_delete_handler(
    id: String,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    log::info!("Deleting summary for {}", &id);
    let result = audit_summary_repository::get_by_id(id.as_str(), &db).await;
    match result {
        Ok(Some(summary)) => {
            let result =
                audit_summary_repository::delete_by_object_id(&summary.id().clone().unwrap(), &db)
                    .await;
            if let Err(err) = result {
                log::error!("{}", err);
                return Ok(StatusCode::INTERNAL_SERVER_ERROR);
            }
            let audit_detail_id = summary.audit_detail_id();
            let result = audit_detail_repository::delete_by_object_id(&audit_detail_id, &db).await;

            if let Err(err) = result {
                log::error!("{}", err);
                return Ok(StatusCode::INTERNAL_SERVER_ERROR);
            }

            Ok(StatusCode::NO_CONTENT)
        }
        Ok(None) => Ok(StatusCode::NOT_FOUND),
        Err(err) => {
            log::error!("{}", &err);
            Ok(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn sites_get_handler(id: String, db: Database) -> Result<impl warp::Reply, Infallible> {
    log::info!("Getting site for {}", &id);
    let site = site_repository::get_by_id(&id, &db).await.unwrap();
    Ok(warp::reply::json(&site))
}

pub async fn group_sites_get_handler(
    id: String,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    log::info!("Getting sites for group {}", &id);
    let group_sites = group_site_repository::get_by_group_id(&id, &db)
        .await
        .unwrap();
    Ok(warp::reply::json(&group_sites))
}

pub async fn register_handler(
    uid: String,
    register_user_parameters: RegisterUserParameters,
    firebase_auth_base_url: String,
    saju_api_key: String,
    db: Database,
) -> Result<impl warp::Reply, Infallible> {
    log::info!("Registering user {:?}", &uid);
    let code =
        registration_code_repository::get_by_code(&register_user_parameters.registration_code, &db)
            .await;

    match code {
        Ok(Some(_code)) => {
            let auth_client = AuthClient::new(&firebase_auth_base_url, saju_api_key.as_str());
            let claims = SajuAuthClaims::new(false, true);
            let _auth_response = auth_client.add_claims(uid.as_str(), claims).await;
            
            let response = RegistrationResponse::new("Registered");
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::CREATED,
            ))
        }
        Ok(None) => {
            let response = RegistrationResponse::new("Invalid code");
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::NOT_FOUND,
            ))
        }
        Err(err) => {
            log::error!("Error in registration_code_repository::get_by_code {}", err);
            let response = RegistrationResponse::new("Internal server error");
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

/*
async fn queue_page_post_handler(
    channel: Channel,
    page_score_parameters: PageScoreParameters,
) -> Result<impl warp::Reply, Infallible> {
    let parameters = ScoreParameters {
        page: Some(page_score_parameters),
        site: None,
    };

    send_score_request_to_queue(&channel, &parameters).await;

    let resp = QueueResponse {
        code: 200,
        message: String::from("Queued"),
    };

    Ok(warp::reply::json(&resp))
}

async fn queue_site_post_handler(
    channel: Channel,
    site_score_parameters: SiteScoreParameters,
) -> Result<impl warp::Reply, Infallible> {
    let parameters = ScoreParameters {
        page: None,
        site: Some(site_score_parameters),
    };

    send_score_request_to_queue(&channel, &parameters).await;

    let resp = QueueResponse {
        code: 200,
        message: String::from("Queued"),
    };

    Ok(warp::reply::json(&resp))
}

async fn send_score_request_to_queue(channel: &Channel, parameters: &ScoreParameters) {
    let payload = serde_json::to_string(&parameters).unwrap();

    channel
        .basic_publish(
            "",
            "score-requests",
            BasicPublishOptions::default(),
            payload.into_bytes(),
            BasicProperties::default(),
        )
        .await
        .unwrap()
        .await
        .unwrap();
}
*/
