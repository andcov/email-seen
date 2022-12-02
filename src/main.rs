#[macro_use]
extern crate rocket;
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, files])
}
