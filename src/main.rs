#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

//use std::path::PathBuf;
//use rocket::http::RawStr;

#[get("/")]
fn crawl() -> String {
    //let decoded = domain.url_decode(); 
    //format!("Domain : , {}", domain.as_str())
    format!("Domain : , {}", "Hello World !")    
}

fn main() {
    rocket::ignite().mount("/", routes![crawl]).launch();  
}
