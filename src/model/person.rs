use actix::*;
use actix_web::*;
use chrono::{NaiveDateTime, Utc};
use diesel;
use diesel::prelude::*;

use schema::persons;
use view::common::Msgs;
use view::person::{PersonListView, PersonView};
use ConnDsl;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Queryable, Identifiable)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub vote: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PersonId {
    person_id: i32,
}

impl Message for PersonId {
    type Result = Result<PersonView, Error>;
}

impl Handler<PersonId> for ConnDsl {
    type Result = Result<PersonView, Error>;

    fn handle(&mut self, person_id: PersonId, _: &mut Self::Context) -> Self::Result {
        use schema::persons::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let the_person = persons
            .filter(&id.eq(&person_id.person_id))
            .load::<Person>(conn)
            .map_err(error::ErrorInternalServerError)?
            .pop();
        match the_person {
            Some(the_person) => {
                let current_person = Person {
                    id: the_person.id,
                    name: the_person.name.clone(),
                    vote: the_person.vote,
                    created_at: the_person.created_at.clone(),
                    updated_at: the_person.updated_at.clone(),
                };
                Ok(PersonView {
                    status: 200,
                    message: "the current_person info.".to_string(),
                    person: current_person,
                })
            }
            None => {
                let no_person = Person {
                    id: 0,
                    name: "".to_owned(),
                    vote: 0,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                };
                Ok(PersonView {
                    status: 400,
                    message: "error.".to_string(),
                    person: no_person,
                })
            }
        }
    }
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
        let person_list = persons
            .load::<Person>(conn)
            .map_err(error::ErrorInternalServerError)?;
        Ok(PersonListView {
            status: 200,
            message: "person_list result".to_string(),
            person_list: person_list,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonUpdate {
    pub person_id: i32,
    pub op: String,
}

impl Message for PersonUpdate {
    type Result = Result<Msgs, Error>;
}

impl Handler<PersonUpdate> for ConnDsl {
    type Result = Result<Msgs, Error>;

    fn handle(&mut self, person_update: PersonUpdate, _: &mut Self::Context) -> Self::Result {
        use schema::persons::dsl::*;
        let conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        let the_person = persons
            .filter(&id.eq(&person_update.person_id))
            .load::<Person>(conn)
            .map_err(error::ErrorInternalServerError)?
            .pop();
        let current_person: Person;
        match the_person {
            Some(the_person) => {
                current_person = Person {
                    id: the_person.id,
                    name: the_person.name.clone(),
                    vote: the_person.vote,
                    created_at: the_person.created_at.clone(),
                    updated_at: the_person.updated_at.clone(),
                };
            }
            None => {
                return Ok(Msgs {
                    status: 400,
                    message: "person not found".to_string(),
                })
            }
        }

        let mut status: i32 = 200;
        let mut message: String = "person updated".to_string();
        match person_update.op.as_ref() {
            "+" => {
                diesel::update(&current_person)
                    .set((vote.eq(vote + 1), updated_at.eq(Utc::now().naive_utc())))
                    .execute(conn)
                    .map_err(error::ErrorInternalServerError)?;
            }
            "-" => {
                diesel::update(&current_person)
                    .set((vote.eq(vote - 1), updated_at.eq(Utc::now().naive_utc())))
                    .execute(conn)
                    .map_err(error::ErrorInternalServerError)?;
            }
            _ => {
                status = 400;
                message = "unknown op".to_string();
            }
        }
        Ok(Msgs {
            status: status,
            message: message.to_string(),
        })
    }
}
