use actix::*;
use actix_web::*;
use diesel::prelude::*;
use chrono::NaiveDateTime;

use ConnDsl;
use view::person::{PersonListView};


#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Queryable)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub vote: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct PersonList;

impl Message for PersonList {
    type Result = Result<PersonListView, Error>;
}

impl Handler<PersonList> for ConnDsl {
    type Result = Result<PersonListView, Error>;

    fn handle(&mut self, _person_list: PersonList, _: &mut Self::Context) -> Self::Result {
        use schema::persons::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let person_list = persons.load::<Person>(conn).map_err(error::ErrorInternalServerError)?;
        Ok(PersonListView{
            status: 200,
            message: "person_list result".to_string(),
            person_list: person_list,
        })
    }
}
