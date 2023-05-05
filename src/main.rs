#[macro_use]
extern crate rocket;
use mongodb::{options::ClientOptions, Client};

#[get("/get")]
fn get() {}

#[delete("/delete/<id>")]
fn delete() {}

#[put("/update/<id>")]
fn update() {}

#[post("/add")]
fn add() {}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![get, put, post, delete])
}

fn connectToMongoDB() {
    let mut client_options = ClientOptions::parse("");
}
