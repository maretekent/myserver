#![warn(unused_variables)]
use std::env;
use getopts::Options;


mod server;
use server::Server;


fn main() {
    let args: Vec<_> = env::args().collect();
    let mut opts = Options::new();

    opts.optopt("p", "port", "set listening port", "PORT");
    opts.optopt("s", "hostname", "set hostname", "HOST");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => {m}
        Err(f) => {print!("Arguments parsing error"); panic!(f.to_string())}
    };

    let default_port = String::from("8080");
    let default_host = String::from("127.0.0.1");
    
    let port : u32 = matches.opt_str("p").unwrap_or(default_port).parse::<u32>().unwrap();
    let hostname : String = matches.opt_str("s").unwrap_or(default_host).parse::<String>().unwrap();

    let server = Server::new(hostname, port);
    server.run();
}
