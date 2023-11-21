use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use eyre::{Result, WrapErr};
use sea_orm::DatabaseConnection;

use crate::adapters::spi::db::establish_connection;
use crate::adapters::spi::db::repositories::{
    DbRequestGroupRepositoryImpl, DbRequestRepositoryImpl,
};
use crate::adapters::spi::http::HttpBinRepositoryImpl;
use crate::adapters::{self, api::shared::AppData};
use crate::application::use_cases::UseCasesStore;
use crate::infrastructure::env_config::EnvConfig;
use crate::infrastructure::middlewares::http_basic_authentication_validator;

#[macro_export]
macro_rules! app (
    ($app_data: expr) => ({
        App::new()
            .app_data($app_data.clone())
            .wrap(Logger::default())
            .wrap(HttpAuthentication::with_fn(
                http_basic_authentication_validator,
            ))
            .default_service(web::route().to(HttpResponse::Forbidden))
            .configure(adapters::api::shared::routes)
    });
);

pub async fn start(config: EnvConfig) -> Result<()> {
    let db = establish_connection(&config.db_url).await?;
    let app_data = web::Data::new(AppData {
        api_user: config.api_user,
        api_password: config.api_password,
        use_cases: create_use_cases_store(db),
    });

    HttpServer::new(move || app!(app_data))
        .bind(("0.0.0.0", config.port))?
        .run()
        .await
        .wrap_err("Failed to bind!")
}

fn create_use_cases_store(db: DatabaseConnection) -> UseCasesStore {
    let http_bin_repository = HttpBinRepositoryImpl::new();
    let db_request_repository = DbRequestRepositoryImpl::new(db.clone());
    let db_request_group_repository = DbRequestGroupRepositoryImpl::new(db.clone());

    UseCasesStore::new(
        http_bin_repository,
        db_request_repository,
        db_request_group_repository,
    )
}
