#![feature(decl_macro)]

#[macro_use]
extern crate diesel;

mod schema;

use crate::schema::billing;
use rocket::{ self, routes, get };
use diesel::{ Queryable, Insertable };
use serde::{ Serialize, Deserialize };

#[derive(Queryable, Serialize)]
struct Billing {
    id: i32,
    title: String,
    amount: f32,
    paid: bool
}

#[derive(Insertable, Deserialize)]
#[table_name = "billing"]
struct NewBilling {
    title: String,
    amount: f32
}

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
        .mount("/hello", routes![hello, hello_name])
        .mount("/billings", routes![])
        .launch();
}
