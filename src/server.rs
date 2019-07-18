#![warn(unused_variables)]
extern crate num_cpus;
extern crate scoped_threadpool;
extern crate httparse;

use std::net::SocketAddr;
use std::net::{TcpListener, TcpStream};
use std::env;
use std::time::Duration;
use scoped_threadpool::Pool;
use std::io::prelude::*;
use std::fs::File;
use std::fs;
use std::path::Path;

// TODO: 1. get connection
// TODO: 2. loop through incoming request - set number of threads/ max connections
// TDOD: 3. extract the client header - determine protocol - http/https
// TODO: 4. check route and write http/s reponse
// TODO: 5. write test
// TODO: 6. check mio & tokio libs
// TODO: 7. enable routing and reading of static files
// TODO: 8. 


pub struct Server {
    hostname: String,
    port: u32
}

impl Server {
    pub fn new(h: String, p: u32) -> Self {
         Server {hostname: h, port: p}
    }

    pub fn run(&self)
    
     {
        let mut connection: String = String::new();
        let port_tostring: String = self.port.to_string().to_owned();
        
        connection.push_str(&self.hostname);
        connection.push_str(":");
        connection.push_str(&port_tostring);
        println!("Server started on http://{:?}", connection);

        let socket_addr: SocketAddr = connection.parse().unwrap();
        
        let listener = TcpListener::bind(&socket_addr).unwrap();

          self.server_listen(listener);
     }

     pub fn server_listen(&self, _listener: TcpListener) -> !{
          let num_threads: u32 = self.get_threads();
          let mut pool = Pool::new(num_threads);
          let mut incoming = _listener.incoming();
          const READ_TIMEOUT: u64 = 30u64; 

          loop {
               let instream = incoming.next().unwrap().expect("TCP stream error");
               instream.set_read_timeout(Some(Duration::from_millis(READ_TIMEOUT))).expect("Read timeout on socket unreadable..");

               pool.scoped(|scope| {
                    scope.execute(|| {
                         self.handle_connection(instream);
                    });
               });
          }
     }

     fn get_threads(&self) -> u32 {
          let num_cpus = num_cpus::get() as u32;

          match env::var("SERVER_THREADS") {
               Ok(value) => value.parse::<u32>().unwrap_or(num_cpus),
               Err(_) => num_cpus
          }
     }

     // fn handle_connection(&self, mut stream: TcpStream) {
     //      let client = Client::new(stream);
     //      client.extract_request();
     // 
     // }

     // fn handle_connection(&self, mut stream: TcpStream) {

     //      let mut buf = [0 ;4096];

     //      stream.read(&mut buf).unwrap();
     //      let mut parsed_headers = [httparse::EMPTY_HEADER; 16];
     //      let mut req = httparse::Request::new(&mut parsed_headers);
     //      req.parse(&buf).unwrap();
     //      match req.path {
     //           Some(ref path) => {
     //                let mut body = String::new();
     //                let mut html = self.read_html_file(path);
     //                html.read_to_string(&mut body).unwrap();
     //                let status = "HTTP/1.1 200 OK\r\n".to_string();
     //                let header = status + "Content-Type: text/html; charset=UTF-8\r\n\r\n";
     //                let res = header + &body + "\r\n";
     //                let data = res.as_bytes();
     //                stream.write(data).unwrap();
     //                stream.flush().unwrap();
     //           },
     //           None => {
     //           }
     //      }
     // }

     // fn read_html_file(&self, path: &&str) -> File {
     //      println!("{:?}", path);
     //      let html_file = match path {
     //           &"/" => "index.html".to_string(),
     //           _ => ".".to_string() + &path + ".html",
     //      };
     //      let html_file_path = Path::new(&html_file);
     //      File::open(&html_file_path).unwrap()
     // }

     fn handle_connection(&self, mut stream: TcpStream) {
          let mut buffer = [0; 1024];
          stream.read(&mut buffer).unwrap();

          let get = b"GET / HTTP/1.1\r\n";
          let sleep = b"GET /sleep HTTP/1.1\r\n";

          let (status_line, filename) = if buffer.starts_with(get){
                    ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
               } else {
                    ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
               };
          let contents = fs::read_to_string(filename).unwrap();

          let response = format!("{}{}", status_line, contents);

          match stream.write(response.as_bytes()){
               Ok(_) => println!("Response content sent"),
               Err(e) => println!("Response failed, error : {}", e),
          }
          stream.flush().unwrap();
     }

}