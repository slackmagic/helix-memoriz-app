use crate::configuration::Configuration;
use crate::state::AppState;
use crate::APP_NAME;
use actix_files::NamedFile;
use actix_web::{web, web::Data, HttpRequest, HttpResponse, Result};
use helix_auth_lib::HelixAuth;
use helix_config_lib::version::Version;
use helix_memoriz_domain::core::entry::Entry;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Debug, Serialize, Deserialize)]
struct HealthCheckResponse {
    app_name: String,
    message: String,
    version: Version,
}

pub async fn serve_static_file(req: HttpRequest) -> Result<NamedFile> {
    let filename: &str = req.match_info().query("filename");
    let base_path = Configuration::get_static_folder();

    let filename = match filename.contains(".") {
        true => filename,
        false => "index.html",
    };

    let serve_file_path = format!("{}{}", base_path, filename);
    let path: PathBuf = PathBuf::from(serve_file_path);
    Ok(NamedFile::open(path)?)
}

pub fn healthcheck(_req: HttpRequest) -> HttpResponse {
    let message = HealthCheckResponse {
        app_name: APP_NAME.to_string(),
        message: "Everything's fine !".to_string(),
        version: helix_config_lib::version::Version::new(
            env!("CARGO_PKG_VERSION").to_owned(),
            env!("GIT_SHORT_HASH").to_owned(),
            env!("GIT_MESSAGE").to_owned(),
            env!("GIT_COMMIT_DATE").to_owned(),
        ),
    };

    HttpResponse::Ok().json(message)
}

pub fn unimplemented(_req: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().body("unimplemented !")
}

//-----------------------------------------------------------------------------------

#[derive(Deserialize)]
pub struct EntriesFilter {
    archived: Option<String>,
}

pub async fn get_all_entries(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
    filter: web::Query<EntriesFilter>,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    match domain
        .get_all_entries(
            claimer.user_uuid,
            match &filter.archived {
                Some(filter) => Some(filter.to_string() == "true"),
                None => None,
            },
        )
        .await
    {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(entries) => HttpResponse::Ok().json(entries),
    }
}

pub async fn get_all_entries_by_board(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
    filter: web::Query<EntriesFilter>,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    let uuid: uuid::Uuid = uuid::Uuid::parse_str(req.match_info().get("uuid").unwrap()).unwrap();

    match domain
        .get_all_entries_by_board(
            claimer.user_uuid,
            uuid,
            match &filter.archived {
                Some(filter) => Some(filter.to_string() == "true"),
                None => None,
            },
        )
        .await
    {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(entries) => HttpResponse::Ok().json(entries),
    }
}

pub async fn get_entry(wrap_state: Data<Arc<Mutex<AppState>>>, req: HttpRequest) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    let uuid: uuid::Uuid = uuid::Uuid::parse_str(req.match_info().get("uuid").unwrap()).unwrap();

    match domain.get_entry(uuid, claimer.user_uuid).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(entry) => HttpResponse::Ok().json(entry),
    }
}

pub async fn archive_entry(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    let uuid: uuid::Uuid = uuid::Uuid::parse_str(req.match_info().get("uuid").unwrap()).unwrap();

    match domain.archive_entry(uuid, claimer.user_uuid).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(entry) => HttpResponse::Ok().json(entry),
    }
}

pub async fn undo_archive_entry(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    let uuid: uuid::Uuid = uuid::Uuid::parse_str(req.match_info().get("uuid").unwrap()).unwrap();

    match domain.undo_archive_entry(uuid, claimer.user_uuid).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(entry) => HttpResponse::Ok().json(entry),
    }
}

pub async fn create_entry(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    (json, req): (web::Json<Entry>, HttpRequest),
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    let mut entry: Entry = json.into_inner();
    entry.owner = Some(claimer.user_uuid);

    match domain.create_entry(entry).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(entry) => HttpResponse::Ok().json(entry),
    }
}

pub async fn update_entry(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    json: web::Json<Entry>,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    let entry: Entry = json.into_inner();
    match domain.update_entry(entry).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(entry) => HttpResponse::Ok().json(entry),
    }
}

pub async fn get_all_boards(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    match domain.get_all_boards(claimer.user_uuid).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(boards) => HttpResponse::Ok().json(boards),
    }
}
