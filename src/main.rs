#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate reqwest;
extern crate select;

//use std::path::PathBuf;
use rocket::http::RawStr;
use select::document::Document;
use select::predicate::Name;

/*
#[get("/crawl/<domain>")]
fn crawl(domain : &RawStr) -> String {


    //let decoded = domain.url_decode(); 
    format!("Domain : , {}", domain.as_str())
    //format!("Domain : , {}", "Hello World !")    
}
*/

#[get("/get_urls/<name>")]
fn get_urls(name: &RawStr) -> String {
    println!("name : {}", name.as_str());
    let mut url = String::from("http://");
    url.push_str(&name);
    let resp = reqwest::get(&url).unwrap();
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
    
    "Done !!".to_string()
}
/*
#[get("/get_url_count/<domain>")]
fn get_url_count(domain : &RawStr) -> String {
    //let decoded = domain.url_decode(); 
    //format!("Domain : , {}", domain.as_str())
    //format!("Domain : , {}", "Hello World !")
    
}
*/

fn main() {    
    rocket::ignite().mount("/spider", routes![get_urls]).launch();  
}
