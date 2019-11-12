use crate::node::*;
use actix::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use json::array;
use json::object;
use std::collections::HashMap;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub struct Client {
    pub addr: Recipient<Message>,
    pub name: String
}

/// Define http actor
pub struct Server {
    pub name: String,
    pub clients: HashMap<String, Client>,
}

#[derive(Clone, Message)]
pub struct Message(pub String);

impl Server {
    /// Send message to all users in the room
    fn send_message(&mut self, message: String, skip_id: Option<String>) {
        for (id, client) in self.clients.iter() {
            println!("[MESSAGE]: {}", message);
            match skip_id.clone() {
                Some(skip_id) => {
                    if *id != skip_id {
                        let _ = client.addr.do_send(Message(message.clone()));
                    }
                }
                None => {
                    let _ = client.addr.do_send(Message(message.clone()));
                }
            }
        }
    }

    // Implement send_message_to
    fn send_message_to(&mut self, message: String, id_to_send: String) {
        println!("[ID_TO_SEND] {}", id_to_send);

        for (id, _) in self.clients.iter() {
            println!("[CLIENT] {}", id);
        }

        let client = self.clients.get(&id_to_send);
        match client {
            Some(client) => {
                let _ = client.addr.do_send(Message(message.clone()));
            }
            None => println!("Client not exist"),
        }
    }
    fn broadcast(&mut self, message: String) {
        self.send_message(message, None)
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}
/// New chat session is created
#[derive(Message)]
#[rtype(String)]
pub struct Connect {
    pub addr: Recipient<Message>,
    pub name: String,
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for Server {
    type Result = String;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        // notify all users in same room

        // register session with random id
        let id: String = thread_rng().sample_iter(&Alphanumeric).take(16).collect();

        let _ = self.clients.insert(id.clone(), Client {addr: msg.addr, name: msg.name.clone() });

        println!("{} joined", id.clone());

        let resp = object! {
            "type" => "ADD",
            "payload" => object!{
                "id" => id.clone(),
                "name" => msg.name.clone()
            }
        };
        self.send_message(json::stringify(resp), Some(id.clone()));

        let mut data = array![];
        for (id, client) in self
            .clients
            .iter()
            .filter(|(client_id, _)| **client_id != id.clone())
        {
            let _ = data.push(object! {
                "id" => id.clone(),
                "name" => client.name.clone()
            });
        }

        let resp = object! {
            "type" => "WELCOME",
            "payload" => object!{
                "clients" => data,
                "id" => id.clone(),
                "name" => msg.name.clone(),
            }
        };

        self.send_message_to(json::stringify(resp), id.clone());

        // send id back
        id
    }
}

/// Session is disconnected
#[derive(Message)]
pub struct Disconnect {
    pub id: String,
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("{:?} disconnected", msg.id);

        // remove address
        let resp = object! {
            "type" => "DEL",
            "payload" => msg.id.clone()
        };
        self.send_message(json::stringify(resp), None);
        self.clients.remove(&msg.id);
    }
}

#[derive(Message)]
pub struct All {
    pub message: String,
    pub id: String,
}

impl Handler<All> for Server {
    type Result = ();

    fn handle(&mut self, msg: All, ctx: &mut Context<Self>) {
        println!("MESSAGE TO ALL");
        let resp = object! {
            "type" => "NEW",
            "payload" => object! {
                "channel" => "all".to_string(),
                "message" => msg.message,
                "id" => msg.id
            }
        };
        self.send_message(json::stringify(resp), None)
    }
}
#[derive(Message)]
pub struct To {
    pub message: String,
    pub id: String,
    pub id_to_send: String,
}

impl Handler<To> for Server {
    type Result = ();

    fn handle(&mut self, msg: To, ctx: &mut Context<Self>) {
        let to_resp = object! {
            "type" => "NEW",
            "payload" => object! {
                "channel" => msg.id.clone(),
                "message" => msg.message.clone(),
                "id" => msg.id.clone()
            }
        };

        let from_resp = object! {
            "type" => "NEW",
            "payload" => object! {
                "channel" => msg.id_to_send.clone(),
                "message" => msg.message,
                "id" => msg.id.clone()
            }
        };

        self.send_message_to(json::stringify(to_resp), msg.id_to_send);
        self.send_message_to(json::stringify(from_resp), msg.id);
    }
}

#[derive(Message)]
pub struct Name {
    pub name: String,
    pub id: String,
}

impl Handler<Name> for Server {
    type Result = ();

    fn handle(&mut self, msg: Name, ctx: &mut Context<Self>) {
        println!("MESSAGE TO ALL");
        let resp = object! {
            "type" => "CHANGE_NAME",
            "payload" => object! {
                "id" => msg.id.clone(),
                "name" => msg.name
            }
        };
        self.send_message(json::stringify(resp), Some(msg.id))
    }
}

pub fn index(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<Server>>,
    name: Option<web::Path<String>>
) -> Result<HttpResponse, Error> {
    let name = match name{
        Some(name) => name.to_string(),
        None => "NODE".to_string()
    };
    println!("NAME MATHCED: {}", name);
    ws::start(
        Node {
            id: "0".to_string(),
            addr: srv.get_ref().clone(),
            name: name,
        },
        &req,
        stream,
    )
}
