#![feature(decl_macro)]

#[macro_use]
extern crate diesel;

mod schema;

use crate::schema::billing;
use bigdecimal::BigDecimal;
use rocket::{ self, routes, get, post };
use rocket_contrib::{ 
    json::Json,
    databases::{database, diesel::PgConnection}
};
use diesel::{ Queryable, Insertable, prelude::* };
use serde::{ Serialize, Deserialize };


#[database("postgres")]
struct DbConn(PgConnection);


#[derive(Queryable, Serialize)]
struct Billing {
    id: i32,
    title: String,
    amount: BigDecimal,
    paid: bool
}

#[derive(Insertable, Deserialize)]
#[table_name = "billing"]
struct NewBilling {
    title: String,
    amount: BigDecimal
}

#[post("/", data = "<new_billing>")]
fn create_billing(connection: DbConn, new_billing: Json<NewBilling>) -> Json<Billing> {
    let result = diesel::insert_into(billing::table)
        .values(&new_billing.0)
        .get_result(&*connection)
        .unwrap();

    return Json(result);
}

#[get("/")]
fn get_billings(connection: DbConn) -> Json<Vec<Billing>> {
    let result = billing::table
        .order(billing::columns::id.desc())
        .load::<Billing>(&*connection)
        .unwrap();
    
    return Json(result);
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
        .attach(DbConn::fairing())
        .mount("/hello", routes![hello, hello_name])
        .mount("/billings", routes![get_billings, create_billing])
        .launch();
}
