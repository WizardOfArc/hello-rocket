#[macro_use] extern crate rocket;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use serde::{Serialize};

pub struct CORS;


#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>){
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
    }
}

#[derive(Serialize)]
struct ProgressRate {
    label: String,
    max_val: u32,
    cur_val: u32,
}

// replace this with a real call to get real info ;)
fn get_progress_rate(label: String) -> ProgressRate {
    match label.as_str() {
        "us-west-2" => ProgressRate {
            label,
            max_val: 2000,
            cur_val: 1500,
        },
        "us-east-1" => ProgressRate {
            label,
            max_val: 2000,
            cur_val: 750,
        },
        "eu-west-1" => ProgressRate {
            label,
            max_val: 2000,
            cur_val: 375,
        },
        &_ => ProgressRate {
            label,
            max_val: 0,
            cur_val: 0,
        }
    }
}


#[get("/")]
fn index() -> &'static str {
    "Hello Woyld!"
}

#[get("/moo")]
fn cow() -> &'static str {
    "What are you? a  COW?!"
}

#[get("/meow/<word>")]
fn quote(word: &str) -> String {
    println!("{}", &word);
    let rate = get_progress_rate(word.to_string());
    serde_json::to_string(&rate).unwrap()
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Hello there. '{}' is not the path you are looking for.  They are for sale if you are interested.", req.uri())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/", routes![index, cow, quote])
        .register("/", catchers![not_found])
}

