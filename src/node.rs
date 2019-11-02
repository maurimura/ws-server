use actix::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
 
pub struct Node {
    id: usize,
    addr: Addr<Self>
}


impl Actor for Node {
    type Context = Context<Self>;
}
