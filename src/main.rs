#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate tera;
extern crate diesel;

fn main() {
    println!("Hello, world!");
}
