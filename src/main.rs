#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world from Rocket-Template!"
}

fn main() {
    println!("ROCKET TEMPLATE LAUNCHED! 🚀");
    rocket::ignite().mount("/", routes![index]).launch();
}
