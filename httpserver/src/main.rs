// Standard library includes
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    str::FromStr,
    thread,
    time::Duration,
    path,
};

// Third part crate dependencies
use clap::Parser;
use tracing::{event, span, Level};
use tracing_subscriber::{filter, fmt};

// Internal dependanices
use httpserver::ThreadPool;
mod html;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port number we wish to host the app
    #[arg(short, long, default_value_t = 7878)]
    port: u16,
    /// Host we wish to host the app
    #[arg(long, default_value_t = String::from("localhost"))]
    host: String,
    /// Number of thread pools we wish to use for the deployment
    #[arg(long, default_value_t = 2)]
    threads: usize,
    /// Log level (options: "ERROR", "WARN", "INFO", "DEBUG", "TRACE")
    #[arg(long, default_value_t = String::from("INFO"))]
    log_level: String,
    /// Path to the html directory of the hosted site
    #[arg(long, default_value_t = String::from("./html"))]
    html_path: String,
}

fn initialize_logger(level: String) {
    let log_level_result = Level::from_str(&level);
    let mut log_level = Level::TRACE;
    if log_level_result.is_err() {
        println!(
            "WARNING! unable to parse --log-level argument: {} into a log level",
            level
        );
        println!("Setting default log level to: {}", log_level);
    } else {
        log_level = log_level_result.unwrap();
    }

    let env_log_level_string = "LOG_LEVEL";
    let env_log_level_result = filter::EnvFilter::try_from_env("LOG_LEVEL");
    if env_log_level_result.is_ok() {
        log_level = Level::from_str(&env_log_level_result.unwrap().to_string()).unwrap();
        println!(
            "overwrting to logLevel: {} from environement variable setting {}",
            log_level, env_log_level_string
        );
    }
    println!("logLevel {}", log_level);
    tracing_subscriber::fmt()
        .with_span_events(fmt::format::FmtSpan::CLOSE)
        .with_max_level(log_level)
        .event_format(
            tracing_subscriber::fmt::format()
                .with_file(true)
                .with_line_number(true),
        )
        .init();
}

#[tracing::instrument(ret)]
fn main() {
    let args = Args::parse();
    initialize_logger(args.log_level);
    // Parse input arguments and configure the application
    let mut host_port: String = args.host;
    host_port.push_str(":");
    host_port.push_str(&args.port.to_string());

    // validate paths to hosted html
    let html_dir_path= path::Path::new(&args.html_path);
    if !html_dir_path.is_dir(){
        tracing::error!("Invalid path to the html directory from --html-path arguement: {}", args.html_path);
        return;
    }
    let index_path = html_dir_path.join("index.html");
    if !index_path.is_file(){
        tracing::error!("Invalid path to the html directory from --html-path arguement: {}", args.html_path);
        return;
    }

    // Bind our port
    tracing::info!("hosting website on http://{}", host_port);
    let listener = TcpListener::bind(host_port).unwrap();

    // Initialize our threadpool for processing received HTTP request
    tracing::info!(
        "Creating a thread pool with {} threads, for handling http requests",
        args.threads
    );
    let pool: ThreadPool = ThreadPool::new(args.threads);

    // For each HTTP request distrubute processing to our thread pool
    for stream in listener.incoming() {
        if stream.is_err() {
            continue;
        }
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let args = Args::parse();
    let buf_reader: BufReader<&TcpStream> = BufReader::new(&stream);
    let request_line = buf_reader.lines().next();
    if request_line.is_none() {
        tracing::debug!("failed handle");
        return;
    }
    let request_line = request_line.unwrap().unwrap();

    let html_dir_path: &path::Path= path::Path::new(&args.html_path);
    if !html_dir_path.is_dir(){
        tracing::error!("Invalid path to the html directory from --html-path arguement: {}", args.html_path);
        return;
    }

    let (status_line, directory_path, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => (
            "HTTP/1.1 200 OK",
            html_dir_path,
            String::from("index.html"),
        ),
        "GET /features HTTP/1.1" => (
            "HTTP/1.1 200 OK",
            html_dir_path,
            String::from("features.html"),
        ),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            (
                "HTTP/1.1 200 OK",
                html_dir_path,
                String::from("index.html"),
                )
        }
        _ => (
            "HTTP/1.1 404 NOT FOUND",
            html_dir_path,
            String::from("404.html"),
        ),
    };

    let contents = html::gethtml(directory_path, &filename);
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
