use model::person::Person;

pub enum MyError{
    NotFound,
    DatabaseError,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct Msgs {
    pub status: i32,
    pub message: String,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct PersonListMsgs {
    pub status: i32,
    pub message: String,
    pub person_list: Vec<Person>,
}
