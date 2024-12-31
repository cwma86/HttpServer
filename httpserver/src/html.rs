use std::{
    fs,
};

use minijinja::{Environment, context};

pub fn gethtml(body_path: &str) -> String {
    let base = fs::read_to_string("/home/corym/proj/HttpServer/httpserver/html/base.html").unwrap();
    let nav = fs::read_to_string("/home/corym/proj/HttpServer/httpserver/html/nav.html").unwrap();
    let body = fs::read_to_string(body_path).unwrap();
    let mut env = Environment::new();
    env.add_template("page", &base).unwrap();
    return env.get_template("page").unwrap().render(context!(nav => nav, body => body)).unwrap();
}