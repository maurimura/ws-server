mod server;
mod node;

use actix::*;
use actix_web::{web, App, HttpServer};
use listenfd::ListenFd;
use std::sync::Mutex;
use std::collections::{HashMap, HashSet};

use server::*;

fn main() {
    // For dev only
    let mut listenfd = ListenFd::from_env();

    let data = web::Data::new(AppState {
        clients: Mutex::new(HashMap::new()),
    });

    let mut server = HttpServer::new(move || App::new().register_data(data.clone()).route("/ws/", web::get().to(index)));


    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run().unwrap();
  
}
