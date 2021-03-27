use crate::state::AppState;
use crate::APP_NAME;
use actix_web::web::Data;
use actix_web::{web, HttpRequest, HttpResponse};
use helix_auth_lib::HelixAuth;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
struct HealthCheckResponse {
    app_name: String,
    message: String,
}

pub fn healthcheck(_req: HttpRequest) -> HttpResponse {
    let message = HealthCheckResponse {
        app_name: APP_NAME.to_string(),
        message: "Everything's fine !".to_string(),
    };

    HttpResponse::Ok().json(message)
}

pub fn unimplemented(_req: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().body("unimplemented !")
}

pub fn version(_req: HttpRequest) -> HttpResponse {
    let version = helix_config_lib::version::Version::new(
        env!("CARGO_PKG_VERSION").to_owned(),
        env!("GIT_SHORT_HASH").to_owned(),
        env!("GIT_MESSAGE").to_owned(),
        env!("GIT_COMMIT_DATE").to_owned(),
    );

    HttpResponse::Ok().json(version)
}

//-----------------------------------------------------------------------------------

pub async fn get_all_entries(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    //TODO: Archived
    match domain.get_all_entries(claimer.user_uuid, None).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(persons) => HttpResponse::Ok().json(persons),
    }
}
