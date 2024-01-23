use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct UserConditionDto {
    pub name: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct InsertUserDto {
    pub name: String,
    pub password: String,
    pub role_id: i32,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct UpdateUserDto {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub role_id: i32,
}
