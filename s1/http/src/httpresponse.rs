use std::collections::HashMap;
use std::io::{Result,Write};
//结构体涉及到引用类型要表明生命周期
#[derive(Debug,PartialEq,Clone)]
pub struct HttpResponse<'a>{
    version: &'a str,
    status_code : &'a str,
    status_text : &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl <'a >Default for HttpResponse <'a>{
    fn default() -> Self {
        Self {
            version:"HTTP/1.1".into(),
            status_code :"200".into(),
            status_text:"OK".into(),
            headers:None,
            body:None,
        }
    }
}

//TODO into 
impl <'a>From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse<'a> )->Self {
        let res1 = res.clone();
        format!(
            "{} {} {} \r\n{}Content-Length: {}\r\n\r\n{}",
            &res1.versions(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            &res.body.unwrap().len(),
            &res1.body(),
        )
    }
}

impl <'a> HttpResponse <'a> {
    pub fn new(
        status_code:&'a str,
        headers:Option<HashMap<&'a str, &'a str>>,
        body:Option<String>
    )-> HttpResponse<'a> {
        let mut responese = HttpResponse::default();
        if status_code != "200"{
            responese.status_code = status_code.into();
        }
        responese.headers = match  &headers {
           Some(_h)=>headers,
           None => {
            let mut h = HashMap::new();
            h.insert("Content-Type", "text/html");
            Some(h)    
           }
        };
        responese.status_text = match responese.status_code {
            "200" => "OK".into(),
            "400"=>"Bad Request".into(),
            "404"=>"Not Found".into(),   
            _ => "Not Found".into(),
        };
        
        responese.body = body;
        responese
    }

    pub fn send_response(&self, write_stream: &mut impl Write)->Result<()>{
        let res = self.clone();
        let response_string = String::from(res);
        let _ = write!(write_stream,"{}",response_string) ;
        Ok(())
    }

    fn versions(&self) ->&str{
        self.version
    }
    fn status_code(&self)->&str{
        self.status_code
    }
    fn status_text(&self)->&str{
        self.status_text
    }
    fn headers(&self)->String{
        let map=  self.headers.clone().unwrap();
        let mut header_string:String = "".into();
        for (k,v) in map.iter(){
            header_string = format!("{}{}:{}\r\n",header_string,k,v);
        }
        header_string 
    }

    fn body(&self)->&str{
        match &self.body {
            None => "",
            Some(s)=>s.as_str(),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_response_struct_cration_200(){
        let response_actual = HttpResponse::new(
            "200",
            None,
            Some("xxx".into()),
        );
        let response_expected = HttpResponse{
            version: "HTTP/1.1",
            status_code:"200",
            status_text:"OK",
            headers:{
                let mut h = HashMap::new();
                h.insert("Content-Type","text/html");
                Some(h)
            },
            body:Some("xxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }


    #[test]
    fn test_response_struct_cration_404(){
        let response_actual = HttpResponse::new(
            "404",
            None,
            Some("xxx".into()),
        );
        let response_expected = HttpResponse{
            version: "HTTP/1.1",
            status_code:"404",
            status_text:"Not Found",
            headers:{
                let mut h = HashMap::new();
                h.insert("Content-Type","text/html");
                Some(h)
            },
            body:Some("xxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_creation(){
        let response_expected = HttpResponse{
            version: "HTTP/1.1",
            status_code:"404",
            status_text:"Not Found",
            headers:{
                let mut h = HashMap::new();
                h.insert("Content-Type","text/html");
                Some(h)
            },
            body:Some("xxx".into()),
        }; 

      let http_string:String=  response_expected.into();
      let actual_string= "HTTP/1.1 404 Not Found \r\nContent-Type:text/html\r\nContent-Length: 3\r\n\r\nxxx";
      assert_eq!(http_string,actual_string);
    }
}