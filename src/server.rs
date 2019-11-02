use crate::node::*;
use actix::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::HashMap;

/// Define http actor
pub struct Server {
    pub name: String,
    pub clients: HashMap<usize, Recipient<Message>>,
    pub rng: ThreadRng,
}

#[derive(Clone, Message)]
pub struct Message(pub String);

impl Actor for Server {
    type Context = Context<Self>;
}
/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for Server {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("Someone joined");

        // notify all users in same room

        // register session with random id
        let id = self.rng.gen::<usize>();
        self.clients.insert(id, msg.addr);

        // send id back
        id
    }
}

/// Session is disconnected
#[derive(Message)]
pub struct Disconnect {
    pub id: usize,
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("{:?} disconnected", msg.id);

        // remove address
        self.clients.remove(&msg.id);
    }
}

pub fn index(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<Server>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        Node {
            id: 0,
            addr: srv.get_ref().clone(),
            name: "NODE".to_string(),
        },
        &req,
        stream,
    )
}
