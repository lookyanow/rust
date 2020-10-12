#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: String, age: u8, cool: bool) -> String {
    if cool {
        format!("Cool: {}, age: {}", cool, age)
    } else {
        format!("Your name is: {}", name)
    }
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}