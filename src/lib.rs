use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

#[derive(Clone, Copy)]
pub enum ServeType {
    File,
    Directory,
}

#[derive(Clone)]
pub struct Serve {
    pub url: String,
    pub serve_type: ServeType,
    pub path: String,
}

#[derive(Clone, Copy)]
pub enum SpecialServeType {
    FourOhFour,
}

#[derive(Clone)]
pub struct SpecialServe {
    pub serve_type: SpecialServeType,
    pub path: String,
}

fn handle_connection(server: Server, mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader.lines().map(|line| line.unwrap()).collect();
    let requested_url = http_request[0].split_whitespace().collect::<Vec<&str>>()[1];
    println!("Request: {}", requested_url);
    let mut response = String::new();
    let mut found = false;
    for serve in server.serves.iter() {
        if serve.url == requested_url {
            found = true;
            match serve.serve_type {
                ServeType::File => {
                    let status_line = "HTTP/1.1 200 OK";
                    let contents = fs::read_to_string("static/index.html").unwrap();
                    let length = contents.len();
                    response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
                    // let status_line = "HTTP/1.1 200 OK";
                    // let contents = fs::read_to_string(serve.path.as_str()).unwrap();
                    // let length = contents.len();
                    // response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
                }
                ServeType::Directory => todo!(),
            }
        }
    }
    if !found {
        for special_serve in server.special_serves.iter() {
            match special_serve.serve_type {
                SpecialServeType::FourOhFour => {
                    let status_line = "HTTP/1.1 200 OK";
                    let contents = fs::read_to_string("static/404.html").unwrap();
                    let length = contents.len();
                    response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
                    // let status_line = "HTTP/1.1 200 OK";
                    // let contents = fs::read_to_string(special_serve.path.as_str()).unwrap();
                    // let length = contents.len();
                    // response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
                }
            }
        }
    }
    println!("Response: {}", response);
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[derive(Clone)]
pub struct Server {
    pub ip: String,
    pub port: u16,
    serves: Vec<Serve>,
    special_serves: Vec<SpecialServe>,
}
impl Server {
    pub fn new(port: u16) -> Self {
        Server {
            ip: local_ip_address::local_ip().unwrap().to_string(),
            port,
            serves: Vec::new(),
            special_serves: Vec::new(),
        }
    }

    pub fn serve(&mut self, url: &str, serve_type: ServeType, path: &str) {
        self.serves.push(Serve {
            url: url.to_string(),
            serve_type,
            path: path.to_string(),
        });
    }

    pub fn special_serve(&mut self, serve_type: SpecialServeType, path: &str) {
        self.special_serves.push(SpecialServe {
            serve_type,
            path: path.to_string(),
        });
    }

    pub fn run(&mut self) {
        println!("Server is running on http://{}:{}", self.ip, self.port);
        for serve in self.serves.iter() {
            println!("Serving \"{}\" at \"{}\"", serve.path, serve.url);
        }

        loop {
            let listener = TcpListener::bind(format!("{}:{}", self.ip, self.port)).expect("Failed to bind to port");
            for stream in listener.incoming() {
                handle_connection(self.clone(), stream.unwrap());
            }
        }
    }
}
