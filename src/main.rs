#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate tera;
extern crate diesel;

use rocket_contrib::{ templates::Template, serve::StaticFiles };
use rocket::request::Form;
use rocket::http::RawStr;
use tera::Context;

fn main() {
    println!("Hello, world!");
}
