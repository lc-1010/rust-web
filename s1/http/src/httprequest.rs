use std::collections::HashMap;

#[derive(Debug,PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

#[derive(Debug,PartialEq)]
pub enum Version{
    V1_1,
    V2_0,
    Uninitialized,
}

#[derive(Debug,PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource:Resource,
    pub headers: HashMap<String,String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req:String)->Self{
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";
        for line in req.lines(){
           if line.contains("HTTP") {
            let (method,resource,version) = process_req_line(line);
            parsed_method = method;
            parsed_version = version;
            parsed_resource = resource;

           }else if line.contains(":"){
            let (key,value) = process_header_line(line);
            parsed_headers.insert(key,value);

           } else if line.len()==0 {

           }else{
            parsed_msg_body = line;
           }
        }

        HttpRequest { 
            method:parsed_method,
             version:parsed_version, 
             resource:parsed_resource, 
             headers:parsed_headers, 
             msg_body:parsed_msg_body.to_string(),
            }
    }
}


impl From<&str> for Method {
    fn from(s:&str)->Method {
        match s {
            "GET" =>Method::Get,
            "POST" =>Method::Post,
            _=>Method::Uninitialized,
        }
    }
}

impl From<&str> for Version {
    fn from(s:&str)->Version {
        match s {
            "HTTP/1.1" =>Version::V1_1,
            "HTTP/2.0" =>Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

fn process_header_line(s:&str)->(String,String){
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next(){
        key = k.to_string();
    }
    if let Some(v) = header_items.next(){
        value = v.to_string();
    }
    (key,value)
}

fn process_req_line(req:&str)->(Method,Resource,Version){
    let mut words = req.split_whitespace();
    let  method = words.next().unwrap();
    let resource = words.next().unwrap();
    let  version = words.next().unwrap();
    
    (
    method.into(),
    Resource::Path(resource.to_string()),
    version.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into(){
        let m:Method = "GET".into();
        // into 要指明转换的类型，如果你的类型实现了from那么自动获得into
        assert_eq!(m ,Method::Get);
       // let ab = "abc";
    }
    #[test]
    fn test_version_into(){
        let v:Version = "HTTP/1.1".into();
        assert_eq!(v,Version::V1_1);
        let v:Version = "HTTP/2.0".into();
        assert_eq!(v,Version::V2_0);
    }
    #[test]
    fn test_read_http(){
        let get = "GET /echo/hello HTTP/1.1 \r\nHost:reqbin.com\r\nAccept:application/json";
        let s = String::from(get);
        let req:HttpRequest = s.into(); 
        let mut headers_expt = HashMap::new();
        headers_expt.insert("Host".into(), "reqbin.com".into());
        headers_expt.insert("Accept".into(), "application/json".into());
        assert_eq!(req.headers,headers_expt);
        let resource = Resource::Path("/echo/hello".into());
        assert_eq!(req.resource,resource);

        let version = Version::V1_1;
        let method = Method::Get;
        let ms_body = String::from("");
        assert_eq!(req.method,method);
        assert_eq!(req.version,version);
        assert_eq!(req.msg_body,ms_body);

    }
}