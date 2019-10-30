#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] 
extern crate rocket;
#[macro_use] 
extern crate rocket_contrib;
#[macro_use] 
extern crate serde;

use serde::{Serialize, Deserialize};
use rocket::response::status;
use rocket::response::{self,Responder,Response};
use rocket::http::{Status, ContentType};
use rocket::request::Request;
use rocket_contrib::json::Json;


#[derive(Serialize)]
struct User {
    id : usize,
    name : String,
    age : i32,
    updated : i64,
}



#[post("/user/<id>")]
fn addUser(id: usize) -> status::Accepted<String> {
    status::Accepted(Some(format!("id: '{}'", id)))
}

//fn getUser(id: usize) -> Json<User> {
#[get("/user/<id>")]
fn getUser(id: usize) -> Option<User> {

    Option<User{
        id:id,
        name:"cowkeys".to_string(),
        age:19,
        updated:0
    }>
    //Json(u)
}



fn main() {
    rocket::ignite().mount("/", routes![addUser,getUser]).launch();
}