use actix::fut;
use actix::prelude::*;
use actix_web::*;
use futures::future::Future;
use serde_json;
use std::time::Instant;

use model::person::{PersonList, PersonUpdate};
use ws_server;
use AppState;

pub fn person_list(req: HttpRequest<AppState>) -> FutureResponse<HttpResponse> {
    req.state()
        .db
        .send(PersonList)
        .from_err()
        .and_then(|res| match res {
            Ok(person_list) => Ok(HttpResponse::Ok().json(person_list)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
/*

pub fn person_update(
    person_update: Json<PersonUpdate>,
    state: State<AppState>,
) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(PersonUpdate {
            person_id: person_update.person_id,
            op: person_update.op.clone(),
        })
        .from_err()
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
}
*/

pub fn ws(req: HttpRequest<AppState>) -> Result<HttpResponse, Error> {
    ws::start(
        req,
        PersonSession {
            id: 0,
            hb: Instant::now(),
        },
    )
}

struct PersonSession {
    id: usize,
    hb: Instant,
}

impl Actor for PersonSession {
    type Context = ws::WebsocketContext<Self, AppState>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr: Addr<Syn, _> = ctx.address();
        ctx.state()
            .ws
            .send(ws_server::Connect {
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
        ctx.state()
            .ws
            .do_send(ws_server::Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<ws_server::Message> for PersonSession {
    type Result = ();

    fn handle(&mut self, msg: ws_server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for PersonSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        match msg {
            ws::Message::Ping(msg) => ctx.pong(&msg),
            ws::Message::Pong(_msg) => self.hb = Instant::now(),
            ws::Message::Text(text) => {
                let person_update: PersonUpdate = serde_json::from_str(&text).unwrap();

                ctx.state()
                    .db
                    .send(PersonUpdate {
                        person_id: person_update.person_id,
                        op: person_update.op.clone(),
                    })
                    .into_actor(self)
                    .then(|_res, _act, ctx| {
                        ctx.state()
                            .ws
                            .do_send(ws_server::ClientMessage { msg: text });
                        fut::ok(())
                    })
                    .wait(ctx);
            }
            ws::Message::Binary(_bin) => println!("binary is not supported."),
            ws::Message::Close(_) => {
                ctx.stop();
            }
        }
    }
}
