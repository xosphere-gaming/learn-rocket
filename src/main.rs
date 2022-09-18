#![feature(decl_macro)]
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/user/<name>/<age>")]
fn user(name: String, age: u8) -> String {
    format!("Hello {}, you are {} years old.", name, age)
}

fn main() {
    rocket::ignite().mount("/", routes![index, user]).launch();
}