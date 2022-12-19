use sqlx::postgres::PgPool;
use chrono::NaiveDateTime;
use crate::errors::MyError;
use crate::models::course::Course;


pub async fn get_course_for_teacher_db(pool: &PgPool, teacher_id: i32)->Result<Vec<Course>, MyError>{
    let rows = sqlx::query!(//r# 多行
        r#"SELECT id, teacher_id, name, 
        time FROM course WHERE teacher_id = $1 ORDER By id DESC "#,teacher_id
    )
    .fetch_all(pool)
    .await?;
   // .unwrap();//panic 了

    let course :Vec<Course>=   rows.iter()
        .map(|r| Course{
            id: Some(r.id.try_into().unwrap()),
            teacher_id: r.teacher_id.try_into().unwrap(),
            name: r.name.clone() ,
            time: Some(NaiveDateTime::from(r.time.unwrap())),
        })
        .collect();

    match course.len(){
        0 => Err(MyError::NotFound("Course not found for teacher".into())),
        _ => Ok(course),
    }
}

pub async fn get_course_detail_db(pool: &PgPool, teacher_id:i32, id:i32)->Result<Course,MyError>{
   let rows = sqlx::query!(
    r#"SELECT id,teacher_id, name,time  
   FROM course where id=$1 and teacher_id=$2"#, id, teacher_id)
   .fetch_one(pool)
   .await;
   //.unwrap();
   //如果成功Ok 取到record
   if let Ok(rows) = rows {
        Ok( 
            Course {
                id: Some(rows.id.try_into().unwrap()),
                teacher_id: rows.teacher_id.try_into().unwrap(),
                name: rows.name.clone() ,
                time: Some(NaiveDateTime::from(rows.time.unwrap())), 
            }
        )
   }else{
        Err(MyError::NotFound("Course not found".into()))
   }
         
   
    
}

pub async fn post_course_db(pool: &PgPool, new:Course)->Result<Course,MyError>{
    let row = sqlx::query!(
        r#"INSERT INTO course (id, teacher_id, name) VALUES ($1, $2, $3) 
        RETURNING id, teacher_id, name, time"# ,
        new.id.unwrap() as i32 , new.teacher_id as i32, new.name
    )
    .fetch_one(pool)
    .await?;
    //.unwrap();

     
        Ok( Course{
            id: Some(row.id.try_into().unwrap()),
            teacher_id: row.teacher_id.try_into().unwrap(),
            name: row.name.clone(),
            time: Some(NaiveDateTime::from(row.time.unwrap())),
            }
        )
 
}
   
 

 
