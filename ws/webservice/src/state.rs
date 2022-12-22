use std::sync::Mutex;
//use super::models::Course;
use sqlx::postgres::PgPool;
pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>, //并发可变
    //pub course: Mutex<Vec<Course>>,//集合
    pub db: PgPool,
}
