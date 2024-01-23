use dotenv::dotenv;
use once_cell::sync::OnceCell;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

// 创建一个全局的DB_POOL，可以一直使用，启动的时候初始化即可
pub static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

// 数据库连接池
pub async fn init_db() {
    // init db
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    DB_POOL
        .set(
            PgPoolOptions::new()
                .max_connections(20)
                .connect(&db_url)
                .await
                .unwrap(),
        )
        .unwrap_or_else(|_| println!("try insert pool cell failure!"));
}
