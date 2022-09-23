mod graphql;

use actix_web::{web::{self, Data}, Error, HttpResponse};
use graphql::{create_schema, Context, Schema};
use juniper_actix::{graphql_handler, playground_handler};

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn app_config(config: &mut web::ServiceConfig) {
    config
        .app_data(Data::new(create_schema()))
        .service(web::resource("/graphql").route(web::post().to(graphql)))
        .service(web::resource("/playground").route(web::get().to(playground)))
        .service(web::resource("/").route(web::get().to(health)));
}

async fn playground() -> Result<HttpResponse, Error> {
    playground_handler("/graphql", None).await
}

async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = Context::new();
    graphql_handler(&schema, &context, req, payload).await
}
