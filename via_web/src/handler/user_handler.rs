use crate::{
    common::res::Res,
    entities::{
        domain::User,
        dto::{
            common_dto::{PageParamsDto, PageResDto},
            user_dto::{InsertUserDto, UpdateUserDto, UserConditionDto},
        },
    },
    service::user_service,
};
use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};

// route register
pub fn build_router() -> Router {
    let router = Router::new()
        .route("/user/page", post(page))
        .route("/user/query_by_name", post(query_by_name))
        .route("/user/query/:id", get(query))
        .route("/user/insert", post(insert))
        .route("/user/update", post(update))
        .route("/user/delete/:id", get(delete));
    router
}

// page
pub async fn page(Json(page_params_dto): Json<PageParamsDto>) -> Json<Res<PageResDto<Vec<User>>>> {
    let res = user_service::user_page(page_params_dto).await;
    Json(Res::with_data(res))
}

// 按条件查询
pub async fn query_by_name(Json(dto): Json<UserConditionDto>) -> Json<Res<Vec<User>>> {
    let res = user_service::query_by_name(dto).await;
    Json(Res::with_data(res))
}

// query
pub async fn query(Path(id): Path<i32>) -> Json<Res<User>> {
    let user = user_service::query_by_id(id).await;
    Json(Res::with_data(user))
}

// insert
pub async fn insert(Json(insert_user_dto): Json<InsertUserDto>) -> Json<Res<u8>> {
    let res = user_service::insert_user(insert_user_dto).await;
    match res {
        Ok(_result) => return Json(Res::ok()),
        Err(err) => return Json(Res::with_err(&err.to_string())),
    }
}

// update
pub async fn update(Json(update_user_dto): Json<UpdateUserDto>) -> Json<Res<u8>> {
    let res = user_service::update_user(update_user_dto).await;
    match res {
        Ok(_result) => return Json(Res::ok()),
        Err(err) => return Json(Res::with_err(&err.to_string())),
    }
}
// delete
pub async fn delete(Path(id): Path<i32>) -> Json<Res<u8>> {
    let res = user_service::delete_user(id).await;
    match res {
        Ok(_result) => return Json(Res::ok()),
        Err(err) => return Json(Res::with_err(&err.to_string())),
    }
}
