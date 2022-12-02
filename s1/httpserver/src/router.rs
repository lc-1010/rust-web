 use std::io::prelude::*;
use http::{httprequest, httprequest::HttpRequest,httpresponse::HttpResponse};
use super::handler::*;
pub struct Router;


impl Router {
    pub fn router(req: HttpRequest, stream:&mut impl Write )->(){
        match req.method {
                httprequest::Method::Get=>match &req.resource {
                    httprequest::Resource::Path(s) =>{
                        let route: Vec<&str> = s.split("/").collect();
                        match route[1]{
                            "api"=> {
                                let resp: HttpResponse = WebServiceHandler::handler(&req);
                                let _ = resp.send_response(stream);
                            },
                            _ => {
                                let  resp:HttpResponse = StaticPageHandler::handler(&req);
                                let _= resp.send_response(stream);
                            },
                        }
                    },
                
                _ =>{
                    let resp:HttpResponse = PageNotFoundHandler::handler(&req);
                    let _ = resp.send_response(stream);
                },
            }
           // httprequest::Method::Post => todo!(),
            //httprequest::Method::Uninitialized => todo!(),
        }

    }
}
