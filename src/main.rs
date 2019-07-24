#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
extern crate tera;
extern crate diesel;

use rocket_contrib::{ templates::Template, serve::StaticFiles };
use rocket::request::Form;
use rocket::http::RawStr;
use tera::Context;

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();

    context.insert("title", &"Welcome to the Website.");
    context.insert("content", &"Here lies a single text entry. How to add a feed?");
    context.insert("replace", &"__<user>__\n __<@tag>__");
    context.insert("user_quote", &"I cannot go\n\nback\nnor\nforward\n\nIn the present\nI am stuck.");
    
    Template::render("index", &context)
}

#[get("/user_settings")]
fn user_settings() -> Template {
    Template::render("user_settings", Context::new())
}

#[get("/dead_page")]
fn dead() -> Template {
    let mut content = Context::new();

    Template::render("dead_page", &content)
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("templates"))
        .mount("/", routes![index, user_settings, dead])
        .attach(Template::fairing())
        .launch();
}
