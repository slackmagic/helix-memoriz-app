use crate::configuration::Configuration;
use helix_memoriz_domain::business::domain::MemorizDomain;
use helix_memoriz_domain::business::traits::DomainTrait;
use helix_pg_db_memoriz_storage::PgDbMemorizStorage;
use std::boxed::Box;

pub struct AppState {
    memoriz_domain: Box<dyn DomainTrait + Send>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            memoriz_domain: Box::new(MemorizDomain::new(AppState::get_pg_storage())),
        }
    }

    pub fn get_domain(&self) -> &Box<dyn DomainTrait + Send> {
        &self.memoriz_domain
    }

    fn get_pg_storage() -> Box<PgDbMemorizStorage> {
        Box::new(
            PgDbMemorizStorage::new(
                Configuration::get_database_name(),
                Configuration::get_database_host(),
                Configuration::get_database_port(),
                Configuration::get_database_user(),
                Configuration::get_database_password(),
            )
            .unwrap(),
        )
    }
}
