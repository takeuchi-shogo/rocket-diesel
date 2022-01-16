
#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
#[macro_use] extern crate rocket_codegen;
#[macro_use] extern crate rocket_contrib;
extern crate serde_derive;
use rocket_cors;

// use rocket::request::Form;
use rocket_contrib::json::{Json, JsonValue};

use rust_rocket::db;
use rust_rocket::user::{User, NewUser};

use rocket_cors::Cors;

fn cors_fairing() -> Cors {
    Cors::from_options(&Default::default()).expect("Cors fairing cannot be created")
}


#[get("/")]
fn index(connection: db::Connection) -> Json<JsonValue> {
    Json(json!(User::index(&connection)))
}

#[get("/hello")]
fn hello() -> String {
    "hello, world!!".to_string()
}

#[get("/hello/<name>")]
fn name(name: String) -> String {
   format!("Hello, {}", name)
}

#[post("/todo", format = "json", data = "<user>")]
fn new_user(user: Json<NewUser>) -> String {
    let insert_user = user.into_inner();
    format!("{:?}", insert_user)
}


#[post("/users", format = "json", data = "<user>")]
fn post(user: Json<NewUser>, connection: db::Connection) -> Json<JsonValue> {
    let insert_user = user.into_inner();
    Json(json!(User::create(insert_user, &connection)))
}

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount("/api", 
        routes![
                index,
                hello,
                name,
                new_user,
                post
                ])
        .attach(cors_fairing())
        .launch();
}