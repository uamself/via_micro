use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: i64,
    pub teacher_id: i32,
    pub name: String,
    pub time: NaiveDate,
}

//    id serial PRIMARY KEY NOT NULL,
//    name varchar(255) NOT NULL,
//    password varchar(255) NOT NULL,
//    role_id int NOT NULL,
//    deleted boolean NOT NULL DEFAULT false,
//    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
//    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub role_id: i32,
    pub deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
