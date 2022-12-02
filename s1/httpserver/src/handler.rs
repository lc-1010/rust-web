
use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use std::collections::HashMap;
use std::env;
use std::fs;
use serde::{Serialize, Deserialize};

pub trait Handler{
    fn handler(req:&HttpRequest)->HttpResponse;
    fn load_file(file_name:&str) ->Option<String>{
        let default_path =  format!("{}/public",env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}",public_path,file_name);
        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStauts{
    order_id:i32,
    order_date: String,
    order_status:String,
}

impl Handler for StaticPageHandler {
    
    fn handler(req:&HttpRequest)->HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let router:Vec<&str> = s.split("/").collect();
        match router[1] {
            ""=> HttpResponse::new("200",None,Self::load_file("index.html")),
            "health"=> HttpResponse::new("200",None,Self::load_file("health.html")),
            path => match Self::load_file(path){
                Some(contents)=>{
                    let mut map :HashMap<&str, &str>=HashMap::new();
                    if path.ends_with(".css"){
                        map.insert("Content-Type","text/css");
                    }else if path.ends_with(".js"){
                        map.insert("Content-type", "text/javascript");
                    }else{
                        map.insert("Context-type","text/html");
                    }
                    HttpResponse::new("200",Some(map),Some(contents))
                },
                None => HttpResponse::new("404",None,Self::load_file("404.hmtl")),
            }
        }

    }

    
}

impl   WebServiceHandler {
    fn load_json()->Vec<OrderStauts>{
        let default_path = format!("{}/data",env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}",data_path,"orders.json");
        let json_contents = fs::read_to_string(full_path);
        let orders: Vec<OrderStauts> = serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
        orders
    }
}

impl Handler for WebServiceHandler {
    fn handler(req:&HttpRequest)->HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route:Vec<&str> = s.split("/").collect();
        //localhost:3000/api/shiping/orders
        match route[2] {
            "" => HttpResponse::new("404",None,Self::load_file("404.html")),
            "shipping" if route.len()> 2 && route[3] == "orders" => {
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers:HashMap<&str,&str> = HashMap::new();
                headers.insert("Content-type","text/json");
                HttpResponse::new("200",Some(headers),body)
            },
        }
    }
}
impl Handler for PageNotFoundHandler {
   fn handler(req:&HttpRequest)->HttpResponse {
         HttpResponse::new("404",
            None, 
            Self::load_file("404.html"))
   } 
}