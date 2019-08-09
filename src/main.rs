#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate reqwest;
extern crate select;

use rocket::http::RawStr;
use select::document::Document;
use select::predicate::Name;

use std::collections::HashMap;

//threshhold number of urls to crawl
const LIMIT: usize = 10;

#[get("/crawl/<name>")]
fn crawl(name: &RawStr) -> String {

    //convert domain to url
    let url;
    match convert_domain_to_url(name.to_string()) {
        Err(e) => return e,
        Ok(v) => url = v,
    }

    //create HashMap to hold all the URLs, and the correcsponding pages for this domain
    let mut hm_urls_pages = HashMap::new();

    //push the domain url as the intial url into the vec
    let mut vec_urls: Vec<String> = Vec::new();
    vec_urls.push(url);

    //index of the next url to crawl
    let mut cur_url_index = 0;

    while let Some(top) = vec_urls.get(cur_url_index) {
        println!("url {} \n", top);

        if !hm_urls_pages.contains_key(top) {
            //get the page body as text and dom and store
            let (body, doc) = get_doc_from_url(top.to_string());
            hm_urls_pages.insert(top.to_string(), body);

            //get all the urls in this page and store in a vec
            let mut vec_sub_urls = get_urls_from_doc(doc);

            if vec_urls.is_empty() {
                return "No more Links found".to_string();
            }

            vec_urls.append(&mut vec_sub_urls);

            if vec_urls.len() >= LIMIT {
                break;
            }
        }
        cur_url_index += 1;
    }

    //create the file name
    let mut fname = name.to_string();
    fname.push_str(".json");

    //serialize the crawled pages and urls to a json file
    serde_any::to_file(fname, &hm_urls_pages).unwrap();
    return "Successfully Crawled!".to_string();
}

#[get("/get_urls/<name>")]
fn get_urls(name: &RawStr) -> String {
    //create the file name from domain
    let name_str = name.as_str();
    let mut fname = name_str.to_string();
    fname.push_str(".json");

    println!("file name {} ", fname);

    //deserialize the file and get keys which are urls
    match serde_any::from_file(fname) {
        Err(e) => {
            return e.to_string();
        }
        Ok(map) => {
            let hm_urls_pages: HashMap<String, String> = map;
            let mut keys = String::new();

            for key in hm_urls_pages.keys() {
                println!("{}", key);
                keys.push_str("\n");
                keys.push_str(key);
            }

            return keys;
        }
    }
}

#[get("/get_url_count/<name>")]
fn get_url_count(name: &RawStr) -> String {
    //create the file name from domain
    let name_str = name.as_str();
    let mut fname = name_str.to_string();
    fname.push_str(".json");

    println!("file name {} ", fname);

    //deserialize the file and get key count of urls
    match serde_any::from_file(fname) {
        Err(e) => {
            return e.to_string();
        }
        Ok(map) => {
            let hm_urls_pages: HashMap<String, String> = map;
            return hm_urls_pages.len().to_string();
        }
    }
}

fn convert_domain_to_url(domain: String) -> Result<String, String> {
    //Validate the domain name
    if domain.is_empty() {
        Err("Empty Domain Name!".to_string())
    } else if !domain.contains(".") {
        Err("Invalid Domain Name!".to_string())
    } else if domain.ends_with(".") {
        Err("Invalid Domain Name!".to_string())
    } else {        
        let mut url = String::from("http://www.");
        url.push_str(&domain);
        Ok(url)
    }
}

fn get_doc_from_url(url: String) -> (String, select::document::Document) {
    //Make the GET request and return the body as text and Document
    println!("running url : {}\n", url);
    let resp = reqwest::get(&url);
    match resp {
        Err(_) => (
            String::from(""),
            Document::from_read("".as_bytes()).unwrap(),
        ),
        Ok(mut resp) => {
            let content = resp.text_with_charset("utf-8");
            match content {
                Err(_) => (
                    String::from(""),
                    Document::from_read("".as_bytes()).unwrap(),
                ),
                Ok(content) => (
                    content.to_string(),
                    Document::from_read(content.as_bytes()).unwrap(),
                ),
            }
        }
    }
}

fn get_urls_from_doc(doc: select::document::Document) -> Vec<String> {
    //Store URLs in the Vector
    let mut vec_urls: Vec<String> = Vec::new();

    //Find the links
    doc.find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| {
            if !x.contains("?") && x.contains("//") {
                match std::str::from_utf8(x.as_bytes()) {
                    Err(_) => println!("invalid utf-8 string"),
                    Ok(y) => {
                        let z = y.to_string();
                        if !vec_urls.contains(&z) {
                            vec_urls.push(z);
                        }
                    }
                }
            }
        });

    for url in &vec_urls {
        println!("{} \n", url);
    }

    return vec_urls;
}

fn main() {
    rocket::ignite()
        .mount("/spider", routes![crawl, get_urls, get_url_count])
        .launch();
}