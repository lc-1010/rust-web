
use actix_web::{web, HttpResponse};

use crate::handlers::{course::*, general::health_check_handler};

pub fn general_routes(cfg: &mut web::ServiceConfig){
    cfg.route("/health",web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig){
    cfg.service(web::scope("/course") //定义资源合集作用域 以/course/开始的路径
                    .route("/",web::post().to(new_course))//路由到具体的handler
                    .route("/{user_id}",web::get().to(get_course_for_teacher))
                    .route("/{user_id}/{course_id}",web::get().to(get_course_detail))
    );
}

//是在service 内部route 虽然可以在外面但是报错
pub fn course_routes_test(cfg: &mut web::ServiceConfig){
    cfg
    .service(web::scope("/test")
    .route("/",web::get().to(|| HttpResponse::Ok()))
    .route("/1",web::get().to(course_test_handler))
    );
}