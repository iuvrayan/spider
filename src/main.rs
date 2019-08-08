#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate reqwest;
extern crate select;

use rocket::http::RawStr;
use select::document::Document;
use select::predicate::Name;

use std::collections::HashMap;

#[get("/crawl/<name>")]
fn crawl(name : &RawStr) -> String {
    let name_str = name.as_str();

    //Validate the domain name
    if name_str.is_empty() {
        return "Empty Domain Name!".to_string();
    }

    if !name_str.contains(".") {
        return "Invalid Domain Name!".to_string();
    }

    if name_str.ends_with(".") {
        return "Invalid Domain Name!".to_string();
    }

    //Create the initial URL
    let mut url = String::from("http://www.");
    url.push_str(&name_str);
    
    //create HashMap to hold all the URLs, and the correcsponding for this domain
    let mut hm_urls_pages = HashMap::new();

    //get the document corresponding to the URL, and store the document against the URL
    let doc = get_doc_from_url(url.to_string());
    hm_urls_pages.insert(url.to_string(), doc.clone());    

    let mut vec_urls = get_urls_from_doc(doc);
    //println!("initial urls {:?} \n", vec_urls);

    while let Some(top) = vec_urls.pop() {
        println!("the top url {} \n", top);
        let mut i = 1;
        if !hm_urls_pages.contains_key(&top) && i < 10 {
            let sub_doc = get_doc_from_url(top.to_string());         
            hm_urls_pages.insert(top.to_string(), sub_doc.clone());
            i  += 1;
            println!("url level {}", i);
            let mut vec_sub_urls = get_urls_from_doc(sub_doc);
            vec_urls.append(&mut vec_sub_urls);
        }
    }

    /*
    
    let vec_urls = Arc::new(Mutex::new(vec![""]));

    for i in 0..3 {
      let cloned_v = v.clone();
      thread::spawn(move || {
         cloned_v.lock().unwrap().push(i);
      });
    }
    */
    return "Successfully Crawled!".to_string();
}

fn get_doc_from_url(url : String) -> select::document::Document {
    //Make the GET request
    let resp = reqwest::get(&url).unwrap(); 
    
    //Get the document
    let doc = Document::from_read(resp).unwrap();

    return doc;
}

fn get_urls_from_doc(doc : select::document::Document) -> Vec<String> {
    //Store URLs in the Vector
    let mut vec_urls : Vec<String> = Vec::new();

    doc.find(Name("a"))
    .filter_map(|n| n.attr("href"))
    .for_each(|x| vec_urls.push(x.to_string()));

    for url in &vec_urls {
        println!("{} \n", url);
    }

    return vec_urls;
}

#[get("/get_urls/<name>")]
fn get_urls(name: &RawStr) -> String {
    let name_str = name.as_str();
    return "URLS are currently empty".to_string();
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
    rocket::ignite().mount("/spider", routes![crawl, get_urls]).launch();  
}
