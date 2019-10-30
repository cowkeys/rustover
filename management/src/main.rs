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
use std::io::Cursor;
use rocket::http::{Status, ContentType};
use rocket::request::Request;
use rocket_contrib::json::Json;
use serde_json::json;


#[derive(Serialize)]
struct User {
    id : usize,
    name : String,
    age : i32,
    updated : i64,
}

impl<'r> Responder<'r> for User {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(json!(self).to_string()))
            .header(ContentType::new("application", "json"))
            .ok()
}
}

#[post("/user/<id>")]
fn addUser(id: usize) -> status::Accepted<String> {
    status::Accepted(Some(format!("id: '{}'", id)))
}

//fn getUser(id: usize) -> Json<User> {
#[get("/user/<id>")]
fn getUser(id: usize) -> User {
    User{
        id:id,
        name:"cowkeys".to_string(),
        age:19,
        updated:0
    }
    //Json(u)
}



fn main() {
    rocket::ignite().mount("/", routes![addUser,getUser]).launch();
}