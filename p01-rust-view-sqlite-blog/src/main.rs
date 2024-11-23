#[macro_use]
extern crate rocket;

use rocket::fs::{relative, FileServer};

// #[get("/")]
// fn index() -> &'static str {
//     "hello world"
// }

#[launch]
fn rocket() -> _ {
    // rocket::build().mount("/", routes![index])
    rocket::build().mount("/", FileServer::from(relative!("frontend/dist")))
}
