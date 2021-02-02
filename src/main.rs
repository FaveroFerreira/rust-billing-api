#![feature(decl_macro)]

use rocket::{ self, routes, get };

#[get("/hello")]
fn hello() -> &'static str {
    "Hello World"
}

#[get("/hello/<name>")]
fn hello_name(name: String) -> String {
    format!("Hello {}", name)
}

fn main() { 
    rocket::ignite()
        .mount("/", routes![hello, hello_name])
        .launch();
}
