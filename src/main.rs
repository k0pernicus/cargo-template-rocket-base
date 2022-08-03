#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello world, from the rocket-base template! :)"
}

#[launch]
fn rocket() {
    rocket::build().mount("/hello", routes![hello])
}
