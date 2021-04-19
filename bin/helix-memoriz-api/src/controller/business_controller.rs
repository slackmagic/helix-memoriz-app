use crate::state::AppState;
use actix_web::{web, web::Data, HttpRequest, HttpResponse};
use helix_auth_lib::HelixAuth;
use helix_memoriz_domain::core::{board::Board, entry::Entry};
use std::sync::{Arc, Mutex};

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

    match domain.get_entry(claimer.user_uuid, uuid).await {
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

pub async fn delete_entry(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    let uuid: uuid::Uuid = uuid::Uuid::parse_str(req.match_info().get("uuid").unwrap()).unwrap();

    match domain.delete_entry(claimer.user_uuid, uuid).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(board) => HttpResponse::Ok().json(board),
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

pub async fn get_board(wrap_state: Data<Arc<Mutex<AppState>>>, req: HttpRequest) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    let uuid: uuid::Uuid = uuid::Uuid::parse_str(req.match_info().get("uuid").unwrap()).unwrap();

    match domain.get_board(claimer.user_uuid, uuid).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(board) => HttpResponse::Ok().json(board),
    }
}

pub async fn create_board(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    (json, req): (web::Json<Board>, HttpRequest),
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    let mut board: Board = json.into_inner();
    board.owner = Some(claimer.user_uuid);

    match domain.create_board(board).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(board) => HttpResponse::Ok().json(board),
    }
}

pub async fn update_board(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    json: web::Json<Board>,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();

    let board: Board = json.into_inner();
    match domain.update_board(board).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(board) => HttpResponse::Ok().json(board),
    }
}

pub async fn delete_board(
    wrap_state: Data<Arc<Mutex<AppState>>>,
    req: HttpRequest,
) -> HttpResponse {
    let state = wrap_state.lock().unwrap();
    let domain = state.get_domain();
    let claimer = HelixAuth::get_claimer(&req).unwrap();

    let uuid: uuid::Uuid = uuid::Uuid::parse_str(req.match_info().get("uuid").unwrap()).unwrap();

    match domain.delete_board(claimer.user_uuid, uuid).await {
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error."),
        Ok(board) => HttpResponse::Ok().json(board),
    }
}
