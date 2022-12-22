use crate::{
    dbaccess::course::*,
    errors::MyError,
    models::course::{CreateCourse, UpdateCourse},
    state::AppState,
};

use actix_web::{web, HttpResponse};

pub async fn get_course_for_teacher(
    params: web::Path<(i32,)>,

    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id,) = params.into_inner();
    get_course_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|course|  
            if course.len() >0 {
                  HttpResponse::Ok().json(course) 
            }else{
                HttpResponse::NotFound().json("no found")
            })

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

pub async fn get_course_detail(
    params: web::Path<(i32, i32)>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    let (teacher, course_id) = params.into_inner();
    let teacher_id = i32::try_from(teacher).unwrap();

    get_course_detail_db(&app_state.db, teacher_id, course_id as i32)
        .await
        .map(|course| HttpResponse::Ok().json(&course))

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

pub async fn course_test_handler() -> HttpResponse {
    HttpResponse::Ok().json("ok")
}
pub async fn new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
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
    post_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(&course))
}

pub async fn update_course_details(
    params: web::Path<(i32, i32)>,
    update_crouse: web::Json<UpdateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, id) = params.into_inner();

    update_course_db(&app_state.db, update_crouse.into(), id, teacher_id)
        .await
        .map(|course| HttpResponse::Ok().json(&course))
}

pub async fn delete_course_detail(
    params: web::Path<(i32, i32)>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, id) = params.into_inner();
    delete_crouse_db(&app_state.db, teacher_id, id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod tests {
    use crate::models::course::CreateCourse;

    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    //#[ignore] //忽略
    #[actix_rt::test]
    async fn post_course_test() {
        //异步运行时测试
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let teacher_id = 1;
        let all = get_course_for_teacher_db(&db_pool, teacher_id)
            .await
            .unwrap();
        let last = &all[0];
        println!("{:?}", all);
        let course = web::Json(CreateCourse {
            teacher_id: last.teacher_id,
            name: format!("last-id-{}", last.id),
            description: last.description.clone(),
            format: last.format.clone(),
            structture: last.structture.clone(),
            duration: last.duration.clone(),
            price: last.id.into(),
            language: Some("zh".to_string()),
            level: Some("hight".to_string()),
        });

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            // course:Mutex::new(vec![]),
            db: db_pool,
        });

        let resp = new_course(course, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_all_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            //course: Mutex::new(vec![]),
            db: db_pool,
        });
        let teacher_id: web::Path<(i32,)> = web::Path::from((1,));
        println!("{:?}", teacher_id);
        let resp = get_course_for_teacher(teacher_id, app_state).await.unwrap();
        //assert_eq!(resp.status(), StatusCode::NOT_FOUND.as_u16());
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            //course:Mutex::new(vec![]),
            db: db_pool,
        });

        let req = web::Path::from((1, 1));
        let resp = get_course_detail(req, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn get_course_detail_error() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            //course:Mutex::new(vec![]),
            db: db_pool,
        });

        let req = web::Path::from((1, 100000));
        let resp = get_course_detail(req, app_state).await;
        match resp {
            Ok(_) => println!("Something woring"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }

    #[ignore = "delete row"]
    #[actix_rt::test]
    async fn delete_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            //course:Mutex::new(vec![]),
            db: db_pool,
        });

        let req = web::Path::from((1, 3));
        let resp = delete_course_detail(req, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_course_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            //course:Mutex::new(vec![]),
            db: db_pool,
        });

        let req = web::Path::from((1, 100000));
        let resp = delete_course_detail(req, app_state).await;
        match resp {
            Ok(_) => println!("Something woring"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }

    #[actix_rt::test]
    async fn update_course_detail_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is no set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let req = web::Path::from((1, 1));
        let update_crouse = web::Json(UpdateCourse {
            name: Some("name".to_string()),
            description: Some("des".into()),
            duration: Some("dur".into()),
            format: Some("format".into()),
            price: Some(213),
            structture: None,
            language: Some("language".into()),
            level: Some("level".into()),
        });

        let rep = update_course_details(req, update_crouse, app_state)
            .await
            .unwrap();
        println!("{:?}", rep);
        assert_eq!(rep.status(), StatusCode::OK);
    }
}
