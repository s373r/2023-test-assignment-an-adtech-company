mod utils;

use actix_web::http::*;
use actix_web::middleware::*;
use actix_web::web::*;
use actix_web::*;
use actix_web_httpauth::middleware::*;

use lib::adapters;
use lib::adapters::api::request::dtos::*;
use lib::adapters::api::request_group::dtos::*;
use lib::app;
use lib::application::use_cases::index::*;
use lib::domain::entities::*;
use lib::infrastructure::middlewares::*;

use utils::*;

mod http {
    use super::*;

    #[actix_web::test]
    async fn test_get_unregistered_endpoint() {
        let mocks = create_repository_mocks(|_, _, _| {});
        let app_data = create_app_data(mocks);
        let app = test::init_service(app!(app_data)).await;

        let request = test::TestRequest::get()
            .uri("/unregistered_endpoint")
            .append_header(BASIC_AUTH_HEADER)
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}

mod authentication {
    use super::*;

    #[actix_web::test]
    async fn cruds_without_credentials() {
        let mocks = create_repository_mocks(|_, _, _| {});
        let app_data = create_app_data(mocks);
        let app = test::init_service(app!(app_data)).await;

        let request = test::TestRequest::get()
            .uri("/api/v1/request/")
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn cruds_with_correct_credentials() {
        let mocks = create_repository_mocks(|_, mock_db_request_repository, _| {
            mock_db_request_repository
                .expect_get_all()
                .returning(|| Ok(vec![RequestEntity::default()]));
        });
        let app_data = create_app_data(mocks);
        let app = test::init_service(app!(app_data)).await;

        let request = test::TestRequest::get()
            .uri("/api/v1/request/")
            .append_header(BASIC_AUTH_HEADER)
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn cruds_with_wrong_credentials() {
        let mocks = create_repository_mocks(|_, _, _| {});
        let app_data = create_app_data(mocks);
        let app = test::init_service(app!(app_data)).await;

        let request = test::TestRequest::get()
            .uri("/api/v1/request/")
            .append_header(("Authorization", "Basic dTpw"))
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn run_api_with_without_credentials() {
        let mocks =
            create_repository_mocks(|http_bin_repository, _, db_request_group_repository| {
                http_bin_repository
                    .expect_send_requests()
                    .returning(|_| vec![seed_request_entity()]);

                db_request_group_repository
                    .expect_insert_with_requests()
                    .returning(|_, _| Ok(()));
            });
        let app_data = create_app_data(mocks);
        let app = test::init_service(app!(app_data)).await;

        let request = test::TestRequest::get().uri("/api/v1/run").to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::OK);
    }
}

mod api {
    use super::*;

    mod index {
        use super::*;

        #[actix_web::test]
        async fn run_with_successful_result() {
            let mocks =
                create_repository_mocks(|http_bin_repository, _, db_request_group_repository| {
                    http_bin_repository.expect_send_requests().returning(|_| {
                        vec![
                            seed_request_entity_with_value(1),
                            seed_request_entity_with_value(2),
                            seed_request_entity_with_value(1),
                        ]
                    });

                    db_request_group_repository
                        .expect_insert_with_requests()
                        .returning(|_, _| Ok(()));
                });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let request = test::TestRequest::get().uri("/api/v1/run").to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::OK);

            let response_body: UseCaseResult = test::read_body_json(response).await;

            assert_eq!(response_body.input_numbers, vec![1, 2, 1]);
            assert_eq!(response_body.frequent_numbers, vec![1]);
        }

        #[actix_web::test]
        async fn run_with_no_successful_http_bin_responses() {
            let mocks =
                create_repository_mocks(|http_bin_repository, _, db_request_group_repository| {
                    http_bin_repository
                        .expect_send_requests()
                        .returning(|_| vec![]);

                    db_request_group_repository
                        .expect_insert_with_requests()
                        .returning(|_, _| Ok(()));
                });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let request = test::TestRequest::get().uri("/api/v1/run").to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

            let response_body = test::read_body(response).await;

            assert_eq!(
                response_body,
                Bytes::from_static(b"No successful responses to process!")
            );
        }
    }

    mod request {
        use super::*;

        #[actix_web::test]
        async fn get_all_with_successful_result() {
            let mocks = create_repository_mocks(|_, mock_db_request_repository, _| {
                mock_db_request_repository.expect_get_all().returning(|| {
                    let entities = vec![
                        seed_request_entity_with_value(1),
                        seed_request_entity_with_value(2),
                        seed_request_entity_with_value(1),
                    ];

                    Ok(entities)
                });
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let request = test::TestRequest::get()
                .uri("/api/v1/request/")
                .append_header(BASIC_AUTH_HEADER)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::OK);

            let response_body: Vec<RequestEntity> = test::read_body_json(response).await;

            assert_eq!(
                response_body,
                vec![
                    seed_request_entity_with_value(1),
                    seed_request_entity_with_value(2),
                    seed_request_entity_with_value(1),
                ]
            );
        }

        #[actix_web::test]
        async fn get_all_but_not_found() {
            let mocks = create_repository_mocks(|_, mock_db_request_repository, _| {
                mock_db_request_repository
                    .expect_get_all()
                    .returning(|| Ok(vec![]));
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let request = test::TestRequest::get()
                .uri("/api/v1/request/")
                .append_header(BASIC_AUTH_HEADER)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::NOT_FOUND);
        }

        #[actix_web::test]
        async fn get_by_id_with_successful_result() {
            let mocks = create_repository_mocks(|_, mock_db_request_repository, _| {
                mock_db_request_repository
                    .expect_get_by_id()
                    .returning(|_| Ok(Some(seed_request_entity_with_value(1))));
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let id = uuid_predefined();
            let request = test::TestRequest::get()
                .uri(&format!("/api/v1/request/{id}"))
                .append_header(BASIC_AUTH_HEADER)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::OK);

            let response_body: RequestEntity = test::read_body_json(response).await;

            assert_eq!(response_body, seed_request_entity_with_value(1));
        }

        #[actix_web::test]
        async fn get_by_id_but_not_found() {
            let mocks = create_repository_mocks(|_, mock_db_request_repository, _| {
                mock_db_request_repository
                    .expect_get_by_id()
                    .returning(|_| Ok(None));
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let id = uuid_predefined();
            let request = test::TestRequest::get()
                .uri(&format!("/api/v1/request/{id}"))
                .append_header(BASIC_AUTH_HEADER)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::NOT_FOUND);
        }

        #[actix_web::test]
        async fn delete_by_id_found() {
            let mocks = create_repository_mocks(|_, mock_db_request_repository, _| {
                mock_db_request_repository
                    .expect_delete_by_id()
                    .returning(|_| Ok(true));
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let id = uuid_predefined();
            let request = test::TestRequest::delete()
                .uri(&format!("/api/v1/request/{id}"))
                .append_header(BASIC_AUTH_HEADER)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::OK);
        }

        #[actix_web::test]
        async fn delete_by_id_not_found() {
            let mocks = create_repository_mocks(|_, mock_db_request_repository, _| {
                mock_db_request_repository
                    .expect_delete_by_id()
                    .returning(|_| Ok(false));
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let id = uuid_predefined();
            let request = test::TestRequest::delete()
                .uri(&format!("/api/v1/request/{id}"))
                .append_header(BASIC_AUTH_HEADER)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::NOT_FOUND);
        }

        #[actix_web::test]
        async fn create_with_successful_result() {
            let mocks = create_repository_mocks(|_, mock_db_request_repository, _| {
                mock_db_request_repository
                    .expect_insert()
                    .returning(|_, _| Ok(()));
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let id = uuid_predefined();
            let expected = seed_request_entity_with_value(1);
            let dto = RequestCreateDto {
                request_group_id: uuid_predefined_for_group_id(),
                sent_at: expected.sent_at,
                request_body: expected.request_body,
                received_at: expected.received_at,
                response_status: expected.response_status,
                response_body: expected.response_body,
                error: expected.error,
            };
            let request = test::TestRequest::put()
                .uri(&format!("/api/v1/request/{id}"))
                .append_header(BASIC_AUTH_HEADER)
                .set_json(dto)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::OK);
        }

        #[actix_web::test]
        async fn update_with_successful_result() {
            let mocks = create_repository_mocks(|_, mock_db_request_repository, _| {
                mock_db_request_repository
                    .expect_update()
                    .returning(|_, _| Ok(true));
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let id = uuid_predefined();
            let dto = RequestUpdateDto {
                response_status: Some(400),
                ..Default::default()
            };
            let request = test::TestRequest::post()
                .uri(&format!("/api/v1/request/{id}"))
                .append_header(BASIC_AUTH_HEADER)
                .set_json(dto)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::OK);
        }
    }

    mod request_group {
        use super::*;

        #[actix_web::test]
        async fn get_all_with_successful_result() {
            let mocks = create_repository_mocks(|_, _, db_request_group_repository| {
                db_request_group_repository.expect_get_all().returning(|| {
                    let entities = vec![
                        seed_request_group_entity(uuid_predefined()),
                        seed_request_group_entity(uuid_predefined_for_group_id()),
                    ];

                    Ok(entities)
                });
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let request = test::TestRequest::get()
                .uri("/api/v1/request_group/")
                .append_header(BASIC_AUTH_HEADER)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::OK);

            let response_body: Vec<RequestGroupEntity> = test::read_body_json(response).await;

            assert_eq!(
                response_body,
                vec![
                    seed_request_group_entity(uuid_predefined()),
                    seed_request_group_entity(uuid_predefined_for_group_id()),
                ]
            );
        }

        #[actix_web::test]
        async fn get_all_but_not_found() {
            let mocks = create_repository_mocks(|_, _, db_request_group_repository| {
                db_request_group_repository
                    .expect_get_all()
                    .returning(|| Ok(vec![]));
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let request = test::TestRequest::get()
                .uri("/api/v1/request_group/")
                .append_header(BASIC_AUTH_HEADER)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::NOT_FOUND);
        }

        #[actix_web::test]
        async fn get_by_id_with_successful_result() {
            let mocks = create_repository_mocks(|_, _, db_request_group_repository| {
                db_request_group_repository
                    .expect_get_by_id()
                    .returning(|_| {
                        let entity = seed_request_group_entity(uuid_predefined());

                        Ok(Some(entity))
                    });
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let id = uuid_predefined();
            let request = test::TestRequest::get()
                .uri(&format!("/api/v1/request_group/{id}"))
                .append_header(BASIC_AUTH_HEADER)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::OK);

            let response_body: RequestGroupEntity = test::read_body_json(response).await;

            assert_eq!(response_body, seed_request_group_entity(uuid_predefined()));
        }

        #[actix_web::test]
        async fn get_by_id_but_not_found() {
            let mocks = create_repository_mocks(|_, _, db_request_group_repository| {
                db_request_group_repository
                    .expect_get_by_id()
                    .returning(|_| Ok(None));
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let id = uuid_predefined();
            let request = test::TestRequest::get()
                .uri(&format!("/api/v1/request_group/{id}"))
                .append_header(BASIC_AUTH_HEADER)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::NOT_FOUND);
        }

        #[actix_web::test]
        async fn delete_by_id_found() {
            let mocks = create_repository_mocks(|_, _, db_request_group_repository| {
                db_request_group_repository
                    .expect_delete_by_id()
                    .returning(|_| Ok(true));
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let id = uuid_predefined();
            let request = test::TestRequest::delete()
                .uri(&format!("/api/v1/request_group/{id}"))
                .append_header(BASIC_AUTH_HEADER)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::OK);
        }

        #[actix_web::test]
        async fn delete_by_id_not_found() {
            let mocks = create_repository_mocks(|_, _, db_request_group_repository| {
                db_request_group_repository
                    .expect_delete_by_id()
                    .returning(|_| Ok(false));
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let id = uuid_predefined();
            let request = test::TestRequest::delete()
                .uri(&format!("/api/v1/request_group/{id}"))
                .append_header(BASIC_AUTH_HEADER)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::NOT_FOUND);
        }

        #[actix_web::test]
        async fn create_with_successful_result() {
            let mocks = create_repository_mocks(|_, _, db_request_group_repository| {
                db_request_group_repository
                    .expect_insert()
                    .returning(|_| Ok(()));
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let id = uuid_predefined();
            let expected = seed_request_group_entity(id);
            let dto = RequestGroupCreateDto {
                started_at: expected.started_at,
                ended_at: expected.ended_at.unwrap(),
                errors_count: expected.errors_count.unwrap(),
            };
            let request = test::TestRequest::put()
                .uri(&format!("/api/v1/request_group/{id}"))
                .append_header(BASIC_AUTH_HEADER)
                .set_json(dto)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::OK);
        }

        #[actix_web::test]
        async fn update_with_successful_result() {
            let mocks = create_repository_mocks(|_, _, db_request_group_repository| {
                db_request_group_repository
                    .expect_update()
                    .returning(|_, _| Ok(true));
            });
            let app_data = create_app_data(mocks);
            let app = test::init_service(app!(app_data)).await;

            let id = uuid_predefined();
            let dto = RequestGroupUpdateDto {
                errors_count: Some(42),
                ..Default::default()
            };
            let request = test::TestRequest::post()
                .uri(&format!("/api/v1/request_group/{id}"))
                .append_header(BASIC_AUTH_HEADER)
                .set_json(dto)
                .to_request();
            let response = test::call_service(&app, request).await;

            assert_eq!(response.status(), StatusCode::OK);
        }
    }
}
