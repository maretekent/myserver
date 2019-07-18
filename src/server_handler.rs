extern crate http_muncher;
use std::str;

use http_muncher::{Parser, ParserHandler};

struct ServerHandler;

impl ParserHandler for ServerHandler {

    // parser header that extracts from the header of the stream
    fn on_header_field(&mut self, parser: &mut Parser, header: &[u8]) -> bool {
        println!("{}: ", str::from_utf8(header).unwrap());
        true
    }

    fn on_header_value(&mut self, parser: &mut Parser, value: &[u8]) -> bool {
        println!("\t{}", str::from_utf8(value).unwrap());
        true
    }

}