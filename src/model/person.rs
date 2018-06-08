use chrono::NaiveDateTime;

#[derive(Debug, Serialize,Deserialize,Queryable)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub vote: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: NaiveDateTime,
}
