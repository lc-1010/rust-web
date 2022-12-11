use chrono::NaiveDateTime;
use dotenv::dotenv;
use std::env;
use std::io;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug)]
pub struct Course{//定义数据表结构
    pub id: i32,
    pub teacher_id :i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}


#[actix_rt::main]
async fn main()->io::Result<()>{
    dotenv().ok();//result ->ok 
    let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL cant't found in .env file");

    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();

    let course_rows = sqlx::query!("select id, teacher_id,name,time from course where teacher_id = $1",1)
        .fetch_all(&db_pool)
        .await
        .unwrap();

    let mut courses_list = vec![];

    for row in course_rows {
        courses_list.push(Course{
            id: row.id,
            teacher_id: row.id,
            name: row.name,
            time: Some(chrono::NaiveDateTime::from(row.time.unwrap())),
        });
    }

    let row = sqlx::query!("select id, teacher_id,name,time from course where teacher_id = $1 and id =$2",1,1)
    .fetch_one(&db_pool)
    .await; 
    
    match row {
        Ok(s) => {
            println!("{:?}",s);
        }
        Err(s) => {
            println!("{:?}",s);
        }
    }

   
    //println!("Course ={:?}",courses_list);
    Ok(())
} 