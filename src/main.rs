#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;

#[macro_use] extern crate rocket_contrib;

// use rocket_contrib::databases::diesel;

extern crate diesel;

#[database("pg_logs")]
struct LogsDbConn(diesel::PgConnection);


#[get("/")]
fn index() -> &'static str {
    "Behold the Placeholder!!!"
}

#[get("/<word>")]
fn word(word: String) -> String {
    format!("elo, {}", word.as_str())
}

#[get("/learn")]
fn learn() -> String {
    format!("Learn THIS for the time being")
}

// the following is from the tutorial; make your own, with hookers and blackjack
#[post("/", data = "<whatev>")]
fn upload(whatev: String) -> Result<String, std::io::Error> {
    // let filename = format!("upload/{id}", id = id);
    // let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);

    // Write the paste out to the file and return the URL.
    // paste.stream_to_file(Path::new(&filename))?;
    Ok(whatev)
}

//testing dis
use rocket::Request;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Where do you think you're going? Here?: {}", req.uri())
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, word, learn])
    .register(catchers![not_found])
    .attach(LogsDbConn::fairing())
    .launch();
}

