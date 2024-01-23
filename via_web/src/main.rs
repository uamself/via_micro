use axum::{routing::get, Router};
use via_web::{
    db::init_db,
    handler::{self},
};

#[tokio::main]
async fn main() {
    // init db
    init_db().await;

    // 整合所有服务的路由
    let user_handler = handler::user_handler::build_router();

    // build our application with a single route
    let app = Router::new().route("/", get(handler)).merge(user_handler);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello, World!"
}

// // 路径参数
// async fn path_handler1(Path(name): Path<String>) -> String {
//     format!("name: {name}")
// }

// async fn path_handler2(Path((name, age)): Path<(String, i64)>) -> String {
//     format!("name: {name}, age: {age}")
// }

// async fn path_handler3(Path(fullpath): Path<HashMap<String, String>>) -> String {
//     format!("path: {fullpath:?}")
// }

// // 查询参数
// #[derive(Deserialize, Debug)]
// #[allow(dead_code)]
// struct Info {
//     name: String,
//     age: u8,
// }

// #[derive(Serialize, Debug)]
// struct SuccessResp {
//     status: String,
//     message: String,
// }

// async fn query2hashmap_handler(query: Query<Info>) -> String {
//     let info: Info = query.0;
//     format!("query1: {info:?}")
// }

// async fn query2deserialize_handler(query: Query<HashMap<String, String>>) -> String {
//     format!("query2: {query:?}")
// }

// // 请求头参数
// async fn headers_handler(headers: HeaderMap) -> String {
//     format!("headers: {headers:?}")
// }

// // JSON
// async fn json_handler(Json(info): Json<Info>) -> String {
//     format!("info: {info:?}")
// }
// async fn json_handler2(Json(info): Json<HashMap<String, String>>) -> Json<SuccessResp> {
//     format!("info: {info:?}");
//     Json(SuccessResp {
//         status: "ok".to_string(),
//         message: "your name is test".to_string(),
//     })
// }

// // sqlx
// async fn list_all(State(pool): State<sqlx::Pool<sqlx::Postgres>>) -> Json<Vec<Course>> {
//     let list = sqlx::query!(r#"SELECT * FROM course"#)
//         .fetch_all(&pool)
//         .await
//         .unwrap();
//     let mut vec = vec![];
//     for row in list {
//         vec.push(Course {
//             id: row.id,
//             teacher_id: row.teacher_id,
//             name: row.name,
//             time: row.time,
//         })
//     }
//     println!("数据库中的所有数据：{:#?}", vec);
//     Json(vec)
// }
