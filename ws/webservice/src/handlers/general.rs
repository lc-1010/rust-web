use actix_web::{web, HttpResponse};

use crate::state::AppState;

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