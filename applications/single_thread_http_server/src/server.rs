pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        use std::net::TcpListener;
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _addr)) => {
                    let mut buffer = [0; 1024];
                    use std::io::Read;
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            use crate::http::Request;
                            use crate::http::Response;
                            use crate::http::StatusCode;

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>IT WORKS!</h1>".to_owned()),
                                    )
                                }
                                Err(e) => {
                                    println!("Failed to parse request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to read from connection: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to establish a connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
