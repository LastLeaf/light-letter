use diesel_derives::*;
use uuid::Uuid;

#[derive(Queryable)]
pub struct User {
    pub id: String,
    pub name: String,
    pub pwd: String,
    pub email: Option<String>,
    pub description: Option<String>,
}

#[derive(Queryable)]
pub struct Config {
    pub key: String,
    pub value: String,
}

#[derive(Queryable)]
pub struct Post {
    pub id: Uuid,
    pub status: i32,
    pub timestamp: std::time::SystemTime,
    pub url: Option<String>,
    pub title: String,
    pub abstract_: String,
    pub content: String,
    pub file: Option<String>,
    pub series: Option<Uuid>,
    pub category: Uuid,
    pub commentable: i32,
}

#[derive(Queryable)]
pub struct PostTags {
    pub post: String,
    pub tag: String,
}
