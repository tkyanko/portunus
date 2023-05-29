use diesel::prelude::*;
use diesel::sql_types::{Integer, Text};
use time::PrimitiveDateTime;

use super::schema::{keys, products, users};

#[derive(Queryable)]
pub struct Product {
    pub id: i32,
    pub title: String,
    pub store: String,
}

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct<'a> {
    pub title: &'a String,
}

#[derive(Insertable, Queryable)]
pub struct Key {
    pub id: i32,
    pub product_id: i32,
    pub product_key: String,
    pub time_added: PrimitiveDateTime,
    pub user_who_added: i32,
    pub time_claimed: Option<PrimitiveDateTime>,
    pub user_who_claimed: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = keys)]
pub struct NewKey<'a> {
    pub product_id: &'a i32,
    pub product_key: &'a String,
    pub time_added: &'a PrimitiveDateTime,
    pub user_who_added: &'a i32,
}

#[derive(QueryableByName)]
pub struct KeyCount {
    #[diesel(sql_type = Text)]
    pub title: String,
    #[diesel(sql_type = Integer)]
    pub count: i32,
}

#[derive(Insertable, Queryable)]
pub struct User {
    pub id: i32,
    pub discord_id: String,
    pub last_taken_time: PrimitiveDateTime,
    pub keys_given: i32,
    pub keys_taken: i32,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub discord_id: &'a String,
    pub keys_given: &'a i32,
}
