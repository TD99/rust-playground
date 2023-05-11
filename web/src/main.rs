use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    env,
};
use dotenv::dotenv;
use web::ThreadPool;

fn main() {
    dotenv().ok();

    let host = env::var("HOST").expect("HOST not set!");
    let port = env::var("PORT").expect("PORT not set!");
    let pool_size = env::var("POOL_SIZE").expect("POOL_SIZE not set!");
    let pool_size: usize = pool_size.parse().expect("POOL_SIZE must be a valid number!");

    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();
    let pool = ThreadPool::new(pool_size);
        
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("[MultiThread] Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request = buf_reader.lines().next().unwrap().unwrap();

    let html_file = env::var("HTML_FILE").expect("HTML_FILE not set!");
    let not_found_file = env::var("NOT_FOUND_FILE").expect("NOT_FOUND_FILE not set!");

    let (status, filename) = match &request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", html_file),
        _ => ("HTTP/1.1 404 NOT FOUND", not_found_file),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}