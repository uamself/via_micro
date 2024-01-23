use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct PageParamsDto {
    pub page_num: i64,
    pub page_size: i64,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct PageResDto<T> {
    pub total_num: i64,
    pub page_num: i64,
    pub page_size: i64,
    pub records: T,
}
