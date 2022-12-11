use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;
use sqlx::postgres::PgPoolOptions;
use std::io;
use std::sync::Mutex;
use std::env;
use dotenv::dotenv;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../models.rs"]
mod models;
#[path = "../db_access.rs"]
mod db_access;

#[path = "../errors.rs"]
mod errors;

use routers::*;
use state::AppState;


#[actix_rt::main]
async fn main()->io::Result<()>{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let db_pool = PgPoolOptions::new()
    .connect(&database_url)
    .await
    .unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm ok ".to_string(),
        visit_count : Mutex::new(0),
        //course: Mutex::new(vec![]),
        db: db_pool,
    });
    
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move ||{App::new()
                        .app_data(shared_data.clone()) 
                        .configure(general_routes)
                        .configure(course_routes)
                        .configure(course_routes_test)
                        .wrap(Logger::default())
                        .wrap(Logger::new("%a %{User-Agent}i"))
                    })
                    .bind("127.0.0.1:3000")?
                    .run()
                    .await
}
