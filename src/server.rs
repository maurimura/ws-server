use crate::node::*;
use actix::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use json::array;
use json::object;
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

impl Server {
    /// Send message to all users in the room
    fn send_message(&mut self, message: String, skip_id: Option<usize>) {
        for (id, addr) in self.clients.iter() {
            match skip_id {
                Some(skip_id) => {
                    if *id != skip_id {
                        let _ = addr.do_send(Message(message.clone()));
                    }
                }
                None => {
                    let _ = addr.do_send(Message(message.clone()));
                }
            }
        }
    }

    // Implement send_message_to
    fn send_message_to(&mut self, message: String, id_to_send: usize) {
        let addr = self.clients.get(&id_to_send);
        match addr {
            Some(addr) => {
                println!("[SEND_MESSAGE_TO] Client {} matched ", id_to_send);
                println!("{}", message);
                let _ = addr.do_send(Message(message.clone()));
            }
            None => println!("Client not exist"),
        }
    }
}

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
        let _ = self.clients.insert(id, msg.addr);
        let resp = object! {
            "type" => "ADD",
            "payload" => id
        };
        self.send_message(json::stringify(resp), Some(id));

        let mut data = array![];
        for (&id, _) in self
            .clients
            .iter()
            .filter(|(&client_id, _)| client_id != id)
        {
            let _ = data.push(id);
        }

        let resp = object! {
            "type" => "WELCOME",
            "payload" => data
        };
        println!("{}", id);
        self.send_message_to(json::stringify(resp), id);

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
        let resp = object! {
            "type" => "DEL",
            "payload" => msg.id
        };
        self.send_message(json::stringify(resp), None);
        self.clients.remove(&msg.id);
    }
}

/// List of available rooms
pub struct List;

impl actix::Message for List {
    type Result = Vec<usize>;
}
/// Handler for `ListRooms` message.
impl Handler<List> for Server {
    type Result = MessageResult<List>;

    fn handle(&mut self, _: List, _: &mut Context<Self>) -> Self::Result {
        let mut clients = Vec::new();

        for key in self.clients.keys() {
            clients.push(key.to_owned())
        }

        MessageResult(clients)
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
