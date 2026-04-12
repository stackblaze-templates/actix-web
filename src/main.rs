use actix_web::{get, App, HttpServer, HttpResponse, web};
use serde::Serialize;

#[derive(Serialize)]
struct Response { status: String, message: String }

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().json(Response { status: "ok".into(), message: "Hello from Actix Web!".into() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    HttpServer::new(|| App::new().service(index))
        .bind(format!("0.0.0.0:{}", port))?.run().await
}
