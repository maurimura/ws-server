use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

mod handler;

use handler::MyWs;

async fn index(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let id = data.to_string();
    let resp = ws::start(MyWs { id: id }, &req, stream);
    println!("{:?}", resp);
    resp
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/{id}", web::get().to(index)))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
