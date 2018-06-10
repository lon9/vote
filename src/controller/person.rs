use std::time::Instant;
use actix::fut;
use actix_web::*;
use actix::prelude::*;
use futures::future::Future;


use model::person::{PersonList};
use ws_server;
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

pub fn ws(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    ws::start(req, PersonSession{
        id: 0,
        hb: Instant::now(),
    })
}

struct PersonSession{
    id: usize,
    hb: Instant,
}

impl Actor for PersonSession{
    type Context = ws::WebsocketContext<Self, AppState>;

    fn started(&mut self, ctx: &mut Self::Context){
        let addr: Addr<Syn, _> = ctx.address();
        ctx.state()
            .ws
            .send(ws_server::Connect{
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    _ => ctx.stop(),
                }
                fut::ok(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        ctx.state().ws.do_send(ws_server::Disconnect{ id: self.id });
        Running::Stop
    }
}

impl Handler<ws_server::Message> for PersonSession {
    type Result = ();

    fn handle(&mut self, msg: ws_server::Message, ctx: &mut Self::Context){
        ctx.text(msg.0);
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for PersonSession{
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context){
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Pong(_msg) => self.hb = Instant::now(),
            ws::Message::Text(text) => {
                ctx.state().ws.do_send(ws_server::ClientMessage{
                    id: self.id,
                    msg: text,
                })
            }
            ws::Message::Binary(_bin) => println!("binary is not supported."),
            ws::Message::Close(_) => {
                ctx.stop();
            }
        }
    }
}
