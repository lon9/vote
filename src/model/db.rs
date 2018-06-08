use actix::*;
use diesel::prelude::*;
use diesel::r2d2::{ Pool, ConnectionManager };

pub struct ConnDsl(pub Pool<ConnectionManager<SqliteConnection>>);

impl Actor for ConnDsl {
    type Context = SyncContext<Self>;
}
