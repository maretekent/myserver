use std::net::TcpStream;

#[derive(Debug, Copy, Clone)]
enum httpversion {
    httpv10,
    httpv11,
    INVALID
}


#[derive(Debug, Copy, Clone)]
enum httpmethod {
    GET,
    POST,
    PUT,
    INVALID
}


#[derive(Debug)]
struct Client {
    stream: TcpStream
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Client { stream: stream }
    }

    pub fn extract_request(&self) {
       let mut buf = [0u8; 4096];
       match self.stream.read(&mut buf){
           Ok(_) => {
            
           },
           Err(e) => println!("Unable to read stream : {:?}", e),
       } 
    }

}


#[derive(Debug)]
pub struct Request {
    method: httpmethod ,
    content: String,
    httpversion: httpversion
}


impl Request {
    pub fn new(_method: &str, _content: &str, _version: &str) {
        client_method = check_method(_method);
        client_version = check_version(_version);

        Request {
            method: client_method,
            content: String::from(_content),
            version: client_version
        }
    }
    
    pub fn get_method(&self) -> httpmethod {
        self.method.Clone();
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }
}


#[derive(Debug)]
pub struct Response {
    statuscode: u16,
    resp_content: String,
    version: httpversion
}

impl Response {
    pub fn new(req: &Request) -> Self {
        //clone the version building response from request version
        // if user requested server response with http 1.1 the 
        // server responds with same
        let _version : httpversion = (*req).version.clone();
        Response {
            statuscode: 400,
            resp_content: String::new(),
            version: _version
        }
    }

    pub fn set_statuscode(&mut self, _statuscode: u16) {
        self.statuscode = _statuscode
    }

    pub fn set_httpheader(&self) -> String {
        let mut header_string = String::new()
        match self.version {
            httpversion::httpv10 =>  { header_string.push_str("HTTP/1.0") }
            httpversion::httpv11 => { header_string.push_str("HTTP/1.1") }
            _ => { header_string.push_str("HTTP/1.0") }
        }

        match self.statuscode {
            200 => { header_string.push_str(" 200 OK\n\r\n") },
            400 => { header_string.push_str(" 400 Bad Request\n\r\nBad Request") },
            404 => { header_string.push_str(" 404 Not Found\n\r\nNot found") },
            500 => { header_string.push_str(" 500 Server error\n\r\nInternal Server Error") },
            _ => { header_string.push_str(" 505 NOT IMPLEMENTED\n\r\nNOT IMPLEMENTED") }        
        }

        return header_string;
    }
}

fn check_version(version: &str) -> httpversion {
    match version {
        "HTTP/1.1" => return httpversion::httpv11,
        "HTTP/1.0" => return httpversion::httpv10,
        _ => return httpversion::INVALID
    }
}

fn check_method(method: &str) -> httpmethod {
    match method {
        "GET" => return httpmethod::GET,
        "POST" => return httpmethod::POST,
        _ => return httpmethod::INVALID
    }
}