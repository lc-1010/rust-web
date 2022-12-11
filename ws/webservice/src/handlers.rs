 
use actix_web::{web, HttpResponse};
use super::{ state::AppState, models::Course, db_access::*, errors::*};

//注入数据
pub async fn health_check_handler(app_state: web::Data<AppState> )->HttpResponse{
    let  health_check_response = &app_state.health_check_response;
    // 获取锁
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times",health_check_response, visit_count);
    // 改值 
    *visit_count +=1;
    HttpResponse::Ok().json(&response)
}

pub async fn get_course_for_teacher(params: web::Path<(usize,)>, app_state:web::Data<AppState>)->Result< HttpResponse,MyError> {
    let (teacher_id,)= params.into_inner();
    get_course_for_teacher_db(&app_state.db, teacher_id as i32)
                                                .await
                                                .map(|course|  HttpResponse::Ok().json(&course))

    // println!("---> {:?}",teacher_id);
    // let course = "ok";
    //app_state
                //    .course
                //     .lock()
                //     .unwrap()
                //     .clone()
                //     .into_iter()
                //     .filter(|item| item.teacher_id == teacher_id)
                //     .collect::<Vec<Course>>();
   
    //  if course.len() >0 {
    //     HttpResponse::Ok().json(&course) 
    // }else{
    //     HttpResponse::NotFound().json("no found")
    // }
}

pub async fn get_course_detail(params: web::Path<(usize, usize)>, app_state: web::Data<AppState>)->Result<HttpResponse,MyError>{
    let  ( teacher,  course_id) = params.into_inner();
    let teacher_id = i32::try_from(teacher).unwrap();

     get_course_detail_db(&app_state.db, teacher_id, course_id as i32).
     await.map(|course| 
    
         HttpResponse::Ok().json(&course)
    )
    
    
    //let selsect_course:Result<&str, ()>= Ok("ok");
    // app_state
    //                                         .course
    //                                         .lock()
    //                                         .unwrap()
    //                                         .clone()
    //                                         .into_iter()
    //                                         .find(|x| x.teacher_id == teacher_id
    //                                              && x.id == Some(course_id ))
    //                                         .ok_or("Course not found ");//option => result 类型ok—or
    // if let Ok(course) = selsect_course {
    //     HttpResponse::Ok().json(course)
    // }else{
    //     HttpResponse::Ok().json("Course not found ".to_string())
    // }
}

pub async fn course_test_handler()->HttpResponse{
    HttpResponse::Ok().json("ok")
}
pub async fn new_course(
                new_course: web::Json<Course>,
                app_state: web::Data<AppState>
            )->Result< HttpResponse,MyError> {
                //println!("Received new course");
                // let course_count =  app_state
                //     .course
                //     .lock()
                //     .unwrap()
                //     .clone()
                //     .into_iter()
                //     .filter(|course| course.teacher_id == new_course.teacher_id)
                //     .collect::<Vec<Course>>()
                //     .len();
                // let new_course = Course{
                //     teacher_id:new_course.teacher_id,
                //     id:Some(course_count+1),
                //     name:new_course.name.clone(),
                //     time:Some(Utc::now().naive_utc()),
                // };
                    
                // app_state.course.lock().unwrap().push(new_course);
                // let respon = format!("cource now had {}",course_count);
                post_course_db(&app_state.db, new_course.into())
                .await
                .map(|course|
                    HttpResponse::Ok().json(&course)
                )
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use chrono::Utc;
    use std::sync::Mutex;
    use std::env;
    use sqlx::postgres::PgPoolOptions; 
    use dotenv::dotenv;

    #[ignore]
    #[actix_rt::test]
    async fn post_course_test(){//异步运行时测试
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let teacher_id = 1;
        let all =  get_course_for_teacher_db(&db_pool, teacher_id).await.unwrap();
        let last = &all[0];
        println!("{:?}",all);
        let course = web::Json(Course{
            teacher_id: last.teacher_id,
            id: Some(last.id.unwrap()+1),
            name: format!("test course-{}",last.id.unwrap()).into(),
            time:Some(Utc::now().naive_utc()),
        });
        
        let app_state = web::Data::new(AppState{
            health_check_response:"".to_string(),
            visit_count: Mutex::new(0),
           // course:Mutex::new(vec![]),
           db: db_pool,
        });

         let resp = new_course(course,app_state).await.unwrap();
          assert_eq!(resp.status(),StatusCode::OK);
           
    }
    #[actix_rt::test]
    async fn get_all_course_success(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            //course: Mutex::new(vec![]),
            db: db_pool,
        });
        let teacher_id:web::Path<(usize,)> = web::Path::from((1,));
        println!("{:?}",teacher_id);
        let resp = get_course_for_teacher(teacher_id, app_state).await.unwrap();
        //assert_eq!(resp.status(), StatusCode::NOT_FOUND.as_u16());
        assert_eq!(resp.status(), StatusCode::OK);

    }
    
    #[actix_rt::test]
    async fn get_course_detail_success(){
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();


        let app_state = web::Data::new(AppState{
            health_check_response:"".to_string(),
            visit_count: Mutex::new(0),
            //course:Mutex::new(vec![]),
            db: db_pool,
        });
        
        let req = web::Path::from((1,1));
        let resp= get_course_detail(req, app_state).await.unwrap();
        assert_eq!(resp.status() ,StatusCode::OK);



    }
}