use std::{io::Read, net::TcpListener};
use crate::http::{request, Request};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr)
            .unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];

                    // what if buffer is too small? (avoid handling this; for course learning purposes)
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer))

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => todo!(),
                                Err(e) => println!("Failed to parse the request: {}", e),
                            }
                        },
                        Err(_) => todo!(),
                    }


                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}