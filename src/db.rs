use actix::{Actor, SyncContext};
use diesel::prelude::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct ConnDsl(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for ConnDsl {
    type Context = SyncContext<Self>;
}
