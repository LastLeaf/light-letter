use diesel_derives::*;

#[derive(Queryable)]
pub struct User {
    pub id: String,
    pub name: String,
    pub pwd: String,
    pub email: Option<String>,
    pub description: Option<String>,
}
