use std::{
    fs,
    path
};

use minijinja::{Environment, context};

pub fn gethtml(directory_path: &path::Path, body_path: &String) -> String {
    let base = fs::read_to_string(directory_path.join("base.html")).unwrap();
    let nav = fs::read_to_string(directory_path.join("nav.html")).unwrap();
    let body = fs::read_to_string(directory_path.join(body_path)).unwrap();
    let mut env = Environment::new();
    env.add_template("page", &base).unwrap();
    return env.get_template("page").unwrap().render(context!(nav => nav, body => body)).unwrap();
}