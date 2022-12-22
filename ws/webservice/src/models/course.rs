use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::errors::MyError;
use std::convert::TryFrom;

// use crate::models::course::Course;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)] // sqlx::FromRow 自动映射
pub struct Course {
    pub teacher_id: i32,
    pub id: i32, //可空
    pub name: String,
    pub time: Option<NaiveDateTime>, //可空
    pub description: Option<String>,
    pub format: Option<String>,
    pub structture: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub teacher_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structture: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
    type Error = MyError;
    fn try_from(value: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(CreateCourse {
            teacher_id: value.teacher_id,
            name: value.name.clone(),
            description: value.description.clone(),
            format: value.format.clone(),
            structture: value.structture.clone(),
            duration: value.duration.clone(),
            price: value.price,
            language: value.language.clone(),
            level: value.level.clone(),
        })
    }
}

#[derive(Deserialize, Hash, Debug, Serialize, Clone)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structture: Option<String>,
    pub duration: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(value: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            name: value.name.clone(),
            description: value.description.clone(),
            format: value.format.clone(),
            structture: value.structture.clone(),
            duration: value.duration.clone(),
            price: value.price,
            language: value.language.clone(),
            level: value.level.clone(),
        }
    }
}

// impl From<web::Json<Course>> for Course {
//     //数据提取器
//     fn from(course: web::Json<Course>)->Self{
//         Course {
//             teacher_id: course.teacher_id,
//             id: course.id,
//             name: course.name.clone(),
//             time: course.time,
//         }
//     }
// }
