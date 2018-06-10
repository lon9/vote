use model::person::Person;

#[derive(Deserialize,Serialize,Debug)]
pub struct PersonListView {
    pub status: i32,
    pub message: String,
    pub person_list: Vec<Person>,
}

