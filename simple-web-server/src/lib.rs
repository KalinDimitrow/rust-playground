use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::error::Error;

mod thread_pool;

use thread_pool::*;


pub struct SimpleWebServer {
    listener : TcpListener,
    thread_pool : ThreadPool,
}

impl SimpleWebServer {
    pub fn new(host : &str, thread_count : usize) -> Result<SimpleWebServer, Box<dyn Error>>{
        let listener = TcpListener::bind(host)?;
        let thread_pool = ThreadPool::new(thread_count);
        Ok(SimpleWebServer{listener : listener, thread_pool : thread_pool})
    }

    pub fn sync_start(self) {

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();

            self.thread_pool.execute(|| {
                handle_connection(stream);
            });
        }

        fn handle_connection(mut stream: TcpStream) {
            let mut buffer = [0; 512];
            stream.read(&mut buffer).unwrap();

            let get = b"GET / HTTP/1.1\r\n";

            let response = if buffer.starts_with(get) {
                format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", hello_content())
            } else {
                format!("{}{}", "HTTP/1.1 404 NOT FOUND\r\n\r\n", error_content())
            };

            stream.write(response.as_bytes()).unwrap();
            if let Err(error )= stream.flush() {
                println!("{}",error);
            }
        }
    }

}


fn hello_content() -> &'static str {
    "<!DOCTYPE html> \
        <html lang=\"en\">\
            <head>
                <meta charset=\"utf-8\">\
                <title>Hello!</title>\
            </head>\
            <body>\
                <h1>Hello!</h1>\
                <p>Hi from Rust</p>\
            </body>\
        </html>"
}

fn error_content() -> &'static str {
    "<!DOCTYPE html>\
        <html lang=\"en\">\
            <head>\
                <meta charset=\"utf-8\">\
                <title>Hello!</title>\
            </head>\
            <body>\
                <h1>Oops!</h1>\
                <p>Sorry, I don't know what you're asking for.</p>\
            </body>\
        </html>"
}


