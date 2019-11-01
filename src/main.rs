use actix::{Actor, StreamHandler, Message, Handler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use listenfd::ListenFd;

/// Chat server sends this messages to session

/// Define http actor
struct MyWs;

#[derive(Clone, Message)]
pub struct ChatMessage(pub String);

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<ws::Message, ws::ProtocolError> for MyWs {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        println!("WS: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => (),
            ws::Message::Binary(bin) => ctx.binary(bin),
            _ => (),
        }
    }
}

impl Handler<ChatMessage> for MyWs {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

fn index(req: HttpRequest, stream: web::Payload) -> HttpResponse {
    let (addr, resp) = ws::start_with_addr(MyWs {}, &req, stream).unwrap();
    let message = ChatMessage("Exploto".to_owned());
    addr.do_send(message);

    resp
}

fn main() {
    // For dev only
    let mut listenfd = ListenFd::from_env();

    let mut server = HttpServer::new(|| App::new().route("/ws/", web::get().to(index)));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run().unwrap();
    // .bind("127.0.0.1:8088")
    //     .unwrap()
    //     .run()
    //     .unwrap();
}
