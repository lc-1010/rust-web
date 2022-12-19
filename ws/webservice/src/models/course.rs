use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// use crate::models::course::Course;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow )]// sqlx::FromRow 自动映射
pub struct Course {
    pub teacher_id: usize,
    pub id:Option<usize>,//可空
    pub name: String,
    pub time: Option<NaiveDateTime>,//可空

    pub description: Option<String>,
    pub format: Option<String>,
    pub structture: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,

}


impl From<web::Json<Course>> for Course {
    //数据提取器
    fn from(course: web::Json<Course>)->Self{
        Course {
            teacher_id: course.teacher_id,
            id: course.id,
            name: course.name.clone(),
            time: course.time,
        }
    }
}