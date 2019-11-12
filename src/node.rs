use crate::server::{All, Connect, Disconnect, Message, Server, To};
use actix::*;
use actix_web_actors::ws;

#[derive(Clone, Message)]
#[rtype(String)]
pub struct Connected(pub String);

/// Handler for Disconnect message.
impl Handler<Connected> for Server {
    type Result = String;

    fn handle(&mut self, msg: Connected, _: &mut Context<Self>) -> Self::Result {
        println!("{:?}", msg.0);
        msg.0
    }
}

pub struct Node {
    pub id: String,
    pub name: String,
    pub addr: Addr<Server>,
}

impl Node {
    fn decode(&mut self, message: String, ctx: &mut ws::WebsocketContext<Self>) {
        println!("{}", message);

        let m = message.trim();
        if m.starts_with('/') {
            let v: Vec<&str> = m.splitn(2, ' ').collect();
            println!("{:?}", v);
            match v[0] {
                "/all" => {
                    if v.len() > 1 {
                        self.addr
                            .send(All {
                                message: v[1].to_string(),
                                id: self.id.clone(),
                            })
                            .into_actor(self)
                            .then(|_, _, _| fut::ok(()))
                            .wait(ctx);
                    }
                }
                "/to" => {
                    if v.len() > 1 {
                        let new_vec: Vec<&str> = v[1].splitn(2,' ').collect();
                        println!("{:?}", new_vec);
                        if new_vec.len() > 1 {
                            self.addr.do_send(To {
                                id: self.id.clone(),
                                message: new_vec[1].to_string(),
                                id_to_send: new_vec[0].parse().unwrap(),
                            });
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

impl Actor for Node {
    type Context = ws::WebsocketContext<Self>;
    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx2| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx2.stop(),
                }
                fut::ok(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(Disconnect {
            id: self.id.clone(),
        });
        Running::Stop
    }
}

/// Handler for ws::Message message
impl StreamHandler<ws::Message, ws::ProtocolError> for Node {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        println!("WS: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => self.decode(text, ctx),
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => ctx.stop(),
            _ => (),
        }
    }
}

impl Handler<Message> for Node {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
