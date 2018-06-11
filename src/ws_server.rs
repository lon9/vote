use std::cell::RefCell;
use std::collections::{HashMap};
use rand::{self, ThreadRng, Rng};
use actix::prelude::*;


#[derive(Message)]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect{
    pub addr: Recipient<Syn, Message>,
}

#[derive(Message)]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
pub struct ClientMessage{
    pub msg: String,
}

pub struct WsServer {
    sessions: HashMap<usize, Recipient<Syn, Message>>,
    rng: RefCell<ThreadRng>,
}

impl Default for WsServer {
    fn default() -> WsServer{
        WsServer {
            sessions: HashMap::new(),
            rng: RefCell::new(rand::thread_rng()),
        }
    }
}

impl WsServer{
    fn broadcast(&mut self, message: &str){
        for session in &mut self.sessions {
            let _ = session.1.do_send(Message(message.to_owned()));
        }
    }
}

impl Actor for WsServer{
    type Context = Context<Self>;
}

impl Handler<Connect> for WsServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result{
        let id = self.rng.borrow_mut().gen::<usize>();
        self.sessions.insert(id, msg.addr);

        id
    }
}

impl Handler<Disconnect> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>){
        self.sessions.remove(&msg.id);
    }
}

impl Handler<ClientMessage> for WsServer{
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>){
        self.broadcast(msg.msg.as_str());
    }
}
