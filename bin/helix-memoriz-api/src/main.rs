#[macro_use]
extern crate serde_derive;

pub mod configuration;
pub mod controller;
pub mod state;

use crate::controller::{business_controller::*, internal_controller::*};
use crate::state::AppState;
use actix_web::{middleware, web, App, HttpServer};
use helix_auth_lib::middleware::AuthValidator;
use helix_config_lib::Configuration as GlobalConfiguration;
use std::sync::{Arc, Mutex};
use std::{env, io};

const APP_NAME: &str = "MEMORIZ_APP";

#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("[HELIX {} {}]", APP_NAME, env!("CARGO_PKG_VERSION"));

    //Configuration init.
    let configuration = GlobalConfiguration::new();

    //Set the IP:PORT to be served.
    let addr = configuration.get_served_addr();
    print!("--> Started on ");
    println!("http://{}", addr);

    //Logger service initialization
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    //Define a global state for all the Actix-Worker
    let app_state = Arc::new(Mutex::new(AppState::new()));

    //Start server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(AuthValidator::new(get_exception_uri()))
            .data(app_state.clone())
            .service(
                web::scope("/api")
                    .route("/_", web::get().to(healthcheck))
                    .service(web::scope("/").configure(get_routes_configuration)),
            )
            .service(web::scope("").route("/{filename:.*}", web::get().to(serve_static_file)))
    })
    .bind(&addr)
    .expect("Cannot bind to address.")
    .keep_alive(configuration.get_keep_alive())
    .shutdown_timeout(configuration.get_shutdown_time_out())
    .workers(configuration.get_workers_number())
    .run()
    .await
}

fn get_routes_configuration(cfg: &mut web::ServiceConfig) {
    //----------------------------------------------------------
    //___DOMAIN___
    //----------------------------------------------------------
    cfg.service(
        web::scope("")
            .service(
                web::scope("/entries")
                    .route("", web::get().to(get_all_entries))
                    .route("", web::post().to(create_entry))
                    .route("", web::put().to(update_entry))
                    .route("/search", web::get().to(search_entries))
                    .route("/by-board/{uuid}", web::get().to(get_all_entries_by_board))
                    .route("/by-label/{id}", web::get().to(unimplemented))
                    .service(
                        web::scope("/{uuid}")
                            .route("", web::get().to(get_entry))
                            .route("", web::delete().to(delete_entry))
                            .route("/do-archive", web::post().to(archive_entry))
                            .route("/undo-archive", web::post().to(undo_archive_entry)),
                    ),
            )
            .service(
                web::scope("/boards")
                    .route("", web::get().to(get_all_boards))
                    .route("", web::post().to(create_board))
                    .route("", web::put().to(update_board))
                    .service(
                        web::scope("/{uuid}")
                            .route("", web::get().to(get_board))
                            .route("", web::delete().to(delete_board)),
                    ),
            )
            .service(
                web::scope("/labels")
                    .route("", web::get().to(unimplemented))
                    .route("", web::post().to(unimplemented))
                    .route("", web::put().to(unimplemented))
                    .service(
                        web::scope("/{id}")
                            .route("", web::get().to(unimplemented))
                            .route("", web::delete().to(unimplemented)),
                    ),
            ),
    );
}

fn get_exception_uri() -> Vec<String> {
    let mut exception_uri = Vec::new();
    exception_uri.push("/api/_".to_string());
    exception_uri.push("/api/version".to_string());

    exception_uri
}
