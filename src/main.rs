use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, Error};
use actix_web_actors::ws;

mod handler;

use handler::MyWs;

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
