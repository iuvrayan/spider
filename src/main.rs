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

    //insert the document aginst the url
    hm_urls_pages.insert(url.to_string(), doc.clone());

    //
    let mut vec_urls = get_urls_from_doc(doc.clone());
    //println!("initial urls {:?} \n", vec_urls);

    if vec_urls.is_empty() {
        return "No Links found".to_string();
    }

    let mut cur_url_index = 1;
    while let Some(top) = vec_urls.get(cur_url_index) {
        println!("the top url {} \n", *top);        
        if !hm_urls_pages.contains_key(top) {
            let sub_doc = get_doc_from_url(top.to_string());         
            hm_urls_pages.insert(top.to_string(), sub_doc.clone());
            
            let mut vec_sub_urls = get_urls_from_doc(sub_doc.clone());

            if vec_urls.is_empty() {
                return "No more Links found".to_string();
            }

            vec_urls.append(&mut vec_sub_urls);
            if vec_urls.len() >= 1000 { break; }
        }
        cur_url_index += 1;
    }
    
    return "Successfully Crawled!".to_string();
}

fn get_doc_from_url(url : String) -> select::document::Document {
    //Make the GET request and return the Document
    println!("running url : {}\n", url);
    if url == "https://www.google.com/" || url == "https://google.com/" {
        return Document::from_read("".as_bytes()).unwrap();
    }
    match reqwest::get(&url) {
        Err(_) => Document::from_read("".as_bytes()).unwrap(),
        Ok(resp) => Document::from_read(resp).unwrap(),
    }
}

fn get_urls_from_doc(doc : select::document::Document) -> Vec<String> {
    //Store URLs in the Vector
    let mut vec_urls : Vec<String> = Vec::new();

    doc.find(Name("a"))
    .filter_map(|n| n.attr("href"))
    .for_each(|x| 
        if !x.contains("?") && x.contains("//") {       
            match std::str::from_utf8(x.as_bytes()) {
                Err(_) => println!("invalid utf-8 string"),
                Ok(y) => {
                    let z = y.to_string();
                    if !vec_urls.contains(&z) {vec_urls.push(z);}
                }     
            }
        }
    );

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
