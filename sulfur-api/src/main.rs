use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use sulfur::reactor::reactor::SulfurReactor;
use sulfur_base::flow::flow::{Engine, Flow, Transform};

#[get("/")]
async fn get() -> impl Responder {
    HttpResponse::Ok().body("Sulfur")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let sulfur_reactor = SulfurReactor::new().await;
    HttpServer::new(|| {
        App::new()
            .service(get)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}