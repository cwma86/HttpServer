use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use clap::Parser;

use httpserver::ThreadPool;
mod html;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port number we wish to host the app
    #[arg(short, long, default_value_t = 7878)]
    port: u16,
}

fn main() {
    let args = Args::parse();
    let mut host_port: String = "127.0.0.1:".to_owned();
    host_port.push_str(&args.port.to_string());
    println!("hosting website on http://{}", host_port);
    let listener = TcpListener::bind(host_port).unwrap();
    let pool: ThreadPool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&TcpStream> = BufReader::new(&stream);
    let request_line = buf_reader.lines().next();
    if request_line.is_none(){
        println!("failed handle");
        return;
    }
    let request_line = request_line.unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "/home/corym/proj/HttpServer/httpserver/html/index.html"),
        "GET /features HTTP/1.1" => ("HTTP/1.1 200 OK", "/home/corym/proj/HttpServer/httpserver/html/features.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "/home/corym/proj/HttpServer/httpserver/html/index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "/home/corym/proj/HttpServer/httpserver/html/404.html"),
    };


    let contents = html::gethtml(filename);
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

}
