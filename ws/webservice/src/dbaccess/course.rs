use crate::errors::MyError;
use crate::models::course::{Course, CreateCourse, UpdateCourse};
use sqlx::postgres::PgPool;

pub async fn get_course_for_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = sqlx::query_as!(
        Course,
        r#"SELECT description as "description?", id,teacher_id, name,
       time,format, structture, duration,
        price,language,  level FROM course WHERE teacher_id = $1 ORDER By id DESC "#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;
    // 使用 as ？ 处理option的字段not null
    // https://docs.rs/sqlx/latest/sqlx/macro.query.html#type-overrides-output-columns
    //  //r# 多行
    // .unwrap();//panic 了

    // let course: Vec<Course> = rows
    //     .iter()
    //     .map(|r| Course {
    //         id: Some(r.id.try_into().unwrap()),
    //         teacher_id: r.teacher_id.try_into().unwrap(),
    //         name: r.name.clone(),
    //         time: Some(NaiveDateTime::from(r.time.unwrap())),
    //     })
    //     .collect();

    // match course.len() {
    //     0 => Err(MyError::NotFound("Course not found for teacher".into())),
    //     _ => Ok(course),
    //}
    Ok(rows)
}

pub async fn get_course_detail_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
) -> Result<Course, MyError> {
    let rows = sqlx::query_as!(
        Course,
        r#"SELECT  description as "description?", id,teacher_id, name,
        time,format, structture, duration,
         price,language,  level 

   FROM course where id=$1 and teacher_id=$2"#,
        id,
        teacher_id
    )
    .fetch_optional(pool)
    .await?;
    //.unwrap();
    //如果成功Ok 取到record
    // if let Ok(rows) = rows {
    //     Ok(Course {
    //         id: Some(rows.id.try_into().unwrap()),
    //         teacher_id: rows.teacher_id.try_into().unwrap(),
    //         name: rows.name.clone(),
    //         time: Some(NaiveDateTime::from(rows.time.unwrap())),
    //     })
    // } else {
    //     Err(MyError::NotFound("Course not found".into()))
    // }

    if let Some(course) = rows {
        Ok(course)
    } else {
        Err(MyError::NotFound("Course Id not Found".into()))
    }
}

pub async fn post_course_db(pool: &PgPool, new: CreateCourse) -> Result<Course, MyError> {
    let row = sqlx::query_as!(
        Course,
        r#"INSERT INTO course (  teacher_id, name, 
            description, format ,structture, duration, 
            price, language, level ) 
        VALUES ($1, $2, $3, $4,$5,$6,$7,$8,$9 ) 
        RETURNING id,  teacher_id, name, time, 
        description as "description?", format ,structture, 
        duration, price, language, level "#,
        new.teacher_id,
        new.name,
        new.description,
        new.format,
        new.structture,
        new.duration,
        new.price,
        new.language,
        new.level,
    )
    .fetch_one(pool)
    .await?;
    //.unwrap();

    // Ok(Course {
    //     id: Some(row.id.try_into().unwrap()),
    //     teacher_id: row.teacher_id.try_into().unwrap(),
    //     name: row.name.clone(),
    //     time: Some(NaiveDateTime::from(row.time.unwrap())),
    //     description: todo!(),
    //     format: todo!(),
    //     structture: todo!(),
    //     duration: todo!(),
    //     price: todo!(),
    //     language: todo!(),
    //     level: todo!(),
    // })

    Ok(row)
}

pub async fn update_course_db(
    pool: &PgPool,
    update_course: UpdateCourse,
    id: i32,
    teacher_id: i32,
) -> Result<Course, MyError> {
    let row = sqlx::query!(
        r#"SELECT  id,  teacher_id, name, time, 
        description as "description?", format ,structture, 
        duration, price, language, level from course where id=$1 and teacher_id=$2"#,
        id,
        teacher_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Course id not found".into()))?;
    //map_err 处理错误直接返回，正确Record数据
    // 根据查询到的值再更新数据拼接成updateCourse 数据
    let name = if let Some(name) = update_course.name {
        name
    } else {
        row.name
    };
    let description = if let Some(description) = update_course.description {
        description
    } else {
        row.description.unwrap_or_default()
    };
    let format = if let Some(format) = update_course.format {
        format
    } else {
        row.format.unwrap_or_default()
    };
    let structture = if let Some(structture) = update_course.structture {
        structture
    } else {
        row.structture.unwrap_or_default()
    };
    let price = if let Some(price) = update_course.price {
        price
    } else {
        row.price.unwrap_or_default()
    };

    let duration = if let Some(duration) = update_course.duration {
        duration
    } else {
        row.duration.unwrap_or_default()
    };

    let language = if let Some(language) = update_course.language {
        language
    } else {
        row.language.unwrap_or_default()
    };
    let level = if let Some(level) = update_course.level {
        level
    } else {
        row.level.unwrap_or_default()
    };

    let up_rse = sqlx::query_as!(
        Course,
        r#"update course set 
        name=$1, description=$2, 
        format = $3, structture = $4, duration= $5,
        price = $6, language =$7, level =$8
        where id = $9 and teacher_id =$10
         RETURNING id,  teacher_id, name, time, 
        description as "description?", format ,structture, 
        duration, price, language, level  
        "#,
        name,
        description,
        format,
        structture,
        duration,
        price,
        language,
        level,
        id,
        teacher_id
    )
    .fetch_one(pool)
    .await;

    if let Ok(up_rse) = up_rse {
        Ok(up_rse)
    } else {
        Err(MyError::NotFound("Course id not found".into()))
    }
}

pub async fn delete_crouse_db(pool: &PgPool, teacher_id: i32, id: i32) -> Result<String, MyError> {
    let course_row = sqlx::query!(
        r#"DELETE FROM course where teacher_id=$1 and id=$2 "#,
        teacher_id,
        id
    )
    .execute(pool)
    .await?;
    Ok(format!("Delete {:?} recored", course_row))
}
