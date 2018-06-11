#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate futures;
extern crate num_cpus;
extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate dotenv;
extern crate chrono;

use actix::*;
use actix_web::{server, App, http::{header, Method}, middleware, middleware::cors::Cors};
use diesel::prelude::PgConnection;
use diesel::r2d2::{ Pool, ConnectionManager };

mod schema;
mod db;
mod ws_server;
mod model;
mod controller;
mod view;

use controller::person::{person_list, person_update, ws};
use db::ConnDsl;
use ws_server::WsServer;

pub struct AppState {
    db: Addr<Syn, ConnDsl>,
    ws: Addr<Syn, WsServer>,    
}

fn main(){
    ::std::env::set_var("RUST_LOG", "actix_web=debug");
    ::std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let sys = actix::System::new("vote");

    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let conn = Pool::builder().build(manager).expect("failed to create pool.");
    let addr = SyncArbiter::start(num_cpus::get() * 4, move || { ConnDsl(conn.clone())});
    let _ws = Arbiter::start(|_| ws_server::WsServer::default());
    server::new(move || App::with_state(AppState{ db: addr.clone(), ws: _ws.clone() })
                .middleware(middleware::Logger::default())
                .configure(|app| Cors::for_app(app)
                           .allowed_methods(vec!["GET", "POST"])
                           .allowed_header(header::CONTENT_TYPE)
                           .max_age(3600)
                           .resource("/api/person/list", |r| { r.method(Method::GET).with(person_list) })
                           //.resource("/api/person/update", |r| { r.method(Method::POST).with2(person_update) })
                           .resource("/person/ws", |r| r.route().f(ws))
                           .register())
                )
        .bind("127.0.0.1:8080").unwrap()
        .shutdown_timeout(2)
        .start();

    sys.run();
}
