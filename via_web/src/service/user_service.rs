use crate::{
    db::DB_POOL,
    entities::{
        domain::User,
        dto::{
            common_dto::{PageParamsDto, PageResDto},
            user_dto::{InsertUserDto, UpdateUserDto, UserConditionDto},
        },
    },
};
use chrono::Local;

// 分页查询
pub async fn user_page(dto: PageParamsDto) -> PageResDto<Vec<User>> {
    let page_size: i64 = dto.page_size;
    let page_num: i64 = dto.page_num;
    let offset: i64 = (page_num - 1) * page_size;

    // total
    let rows: Result<i64, sqlx::Error> =
        sqlx::query_scalar("SELECT count(*) FROM via_users where deleted = false")
            .fetch_one(DB_POOL.get().unwrap())
            .await;
    let total_num = rows.unwrap();

    let users = sqlx::query_as!(
        User,
        "SELECT * FROM via_users where deleted = false order by updated_at Desc limit $1 offset $2",
        page_size,
        offset,
    )
    .fetch_all(DB_POOL.get().unwrap())
    .await;
    let result: PageResDto<Vec<User>> = PageResDto {
        total_num: total_num.try_into().unwrap(),
        page_num,
        page_size,
        records: users.unwrap(),
    };

    result
}

pub async fn query_by_name(dto: UserConditionDto) -> Vec<User> {
    let res = sqlx::query_as!(
        User,
        "SELECT * FROM via_users where deleted = false and name like $1",
        format!("%{}%", dto.name)
    )
    .fetch_all(DB_POOL.get().unwrap())
    .await;

    res.unwrap()
}

// 根据ID查询用户
pub async fn query_by_id(id: i32) -> User {
    let row = sqlx::query_as!(User, "SELECT * FROM via_users WHERE id = $1", id)
        .fetch_one(DB_POOL.get().unwrap())
        .await;
    row.unwrap()
}

// 插入用户
pub async fn insert_user(dto: InsertUserDto) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO via_users (name, password, role_id) VALUES ($1, $2, $3)",
        dto.name,
        dto.password,
        dto.role_id,
    )
    .execute(DB_POOL.get().unwrap())
    .await?;
    Ok(())
}

// 修改用户
pub async fn update_user(dto: UpdateUserDto) -> Result<(), sqlx::Error> {
    let time = Local::now().naive_local();
    println!("{:?}", time);
    sqlx::query!(
        "update via_users set name = $1, password = $2,role_id = $3 , updated_at = $4 where id = $5",
        dto.name,
        dto.password,
        dto.role_id,
        Local::now().naive_local(),
        dto.id,
    )
    .execute(DB_POOL.get().unwrap())
    .await?;
    Ok(())
}

// 删除用户(逻辑删除)
pub async fn delete_user(id: i32) -> Result<(), sqlx::Error> {
    sqlx::query!("update via_users set deleted = true where id = $1", id)
        .execute(DB_POOL.get().unwrap())
        .await?;
    Ok(())
}
