use actix_web::web;

use crate::adapters::api::index::index_controller;
use crate::adapters::api::request::request_controller;
use crate::adapters::api::request_group::request_group_controller;

pub fn routes(config: &mut web::ServiceConfig) {
    // TODO: Rewrite web scope logic when the following issue(s) is fixed:
    //
    //       Scope only serve the first sub-scope · Issue #2295 · actix/actix-web
    //       https://github.com/actix/actix-web/issues/2295
    //
    //       Apply middleware(s) to group of routes without prefix · Issue #414 · actix/actix-web
    //       https://github.com/actix/actix-web/issues/414
    //
    // config
    //     .service(web::scope("/api/v1").configure(index_controller::routes))
    //     .service(web::scope("/api/v1/request").configure(request_controller::routes))
    //     .service(web::scope("/api/v1/request_group").configure(request_group_controller::routes));

    config.service(
        web::scope("api/v1")
            .configure(index_controller::routes)
            .service(web::scope("request").configure(request_controller::routes))
            .service(web::scope("request_group").configure(request_group_controller::routes)),
    );
}
