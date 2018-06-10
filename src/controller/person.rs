use actix_web::*;
use actix::prelude::*;
use futures::future::Future;


use model::person::{PersonList};

use AppState;

pub fn person_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state().db.send(PersonList)
        .from_err()
        .and_then(|res| {
            match res {
                Ok(person_list) => 
                    Ok(HttpResponse::Ok().json(person_list)),
                Err(_) => 
                    Ok(HttpResponse::InternalServerError().into()),
            }
        }).responder()
}

struct PersonWebSocket;

impl Actor for PersonWebSocket {
    type Context = ws::WebsocketContext<Self, AppState>;
}

impl StreamHandler<ws::Message, ws::ProtocolError> for PersonWebSocket{
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context){
        println!("WS: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Text(text) => ctx.text(text),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}

pub fn ws(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    ws::start(req, PersonWebSocket)
}
