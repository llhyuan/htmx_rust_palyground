use std::io::Result;

use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use chrono::Utc;
use htmx::templating::HelloTemplate;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(world)
            .route("/hello", web::get().to(hello))
            .service(fs::Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}

async fn hello() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/hello-world")]
async fn world() -> impl Responder {
    let htmlbody = HelloTemplate {
        time: Utc::now().to_string(),
    }
    .render()
    .unwrap();

    HttpResponse::Ok().body(htmlbody)
}
