use actix::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::sync::Mutex;
use std::collections::{HashMap, HashSet};
use rand::{self, rngs::ThreadRng, Rng};

use crate::node::*;
#[derive(Debug)]
/// Define http actor
pub struct Server {

}


#[derive(Clone, Message)]
pub struct ChatMessage(pub String);

pub struct AppState {
    pub clients: Mutex<HashMap<usize, Addr<Node>>>,
}


impl Actor for Server {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        // self.addr
        //     .send(Connect {
        //         addr: addr.recipient(),
        //     })
        //     .into_actor(self)
        //     .then(|res, act, ctx| {
        //         match res {
        //             Ok(res) => act.id = res,
        //             // something is wrong with chat server
        //             _ => ctx.stop(),
        //         }
        //         fut::ok(())
        //     })
        //     .wait(ctx);
    }


}

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<ChatMessage>,
}

/// Handler for ws::Message message
impl StreamHandler<ws::Message, ws::ProtocolError> for Server {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        println!("WS: {:?}", msg);
        let addr = ctx.address();
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => (),
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(text) => println!("{:?}", addr), 
            _ => (),
        }
    }
}

impl Handler<ChatMessage> for Server {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for Server {
    type Result = usize;

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        0
    }
}

pub fn index(req: HttpRequest, stream: web::Payload, data: web::Data<AppState>) -> HttpResponse {
    let (addr, resp) = ws::start_with_addr(Server {}, &req, stream).unwrap();
    let message = ChatMessage("Exploto".to_owned());
    // data.clients.push(addr);
    // let mut clients = data.clients.lock().unwrap();
    // clients.push(addr.clone());
    addr.do_send(message);
    // println!("{:?}", clients.len());    

    resp
}
