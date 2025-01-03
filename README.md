# HttpServer
Rust HTTP Server

## Description
This is a simple rust project to learn the language through implementing a simple web server that leverages a threadpool architecture to enable processing requests from multiple users.

## Building
To build the application navigate to the source code directory from the repo root `./httserver`

Then build using Rust's Cargo utility `cargo build`

## Running
Run the built application from the built target from the source root (./httpserver) `./target/debug/httpserver --html-path ./html`

Then from your web browser navigate to localhost:7878

Note: additional configuration options can be found by using the --help option `./target/debug/httpserver --help`