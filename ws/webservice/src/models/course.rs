use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

// use crate::models::course::Course;

#[derive(Deserialize, Serialize, Debug, Clone )]
pub struct Course {
    pub teacher_id: usize,
    pub id:Option<usize>,//可空
    pub name: String,
    pub time: Option<NaiveDateTime>,//可空
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