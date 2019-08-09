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

    //push the domain url into the vec
    let mut vec_urls : Vec<String> = Vec::new();
    vec_urls.push(url);

    let mut cur_url_index = 0;
    while let Some(top) = vec_urls.get(cur_url_index) {
        println!("the top url {} \n", *top);        
        if !hm_urls_pages.contains_key(top) {
            let (body, sub_doc) = get_doc_from_url(top.to_string());         
            hm_urls_pages.insert(top.to_string(), body);
            
            let mut vec_sub_urls = get_urls_from_doc(sub_doc);

            if vec_urls.is_empty() {
                return "No more Links found".to_string();
            }

            vec_urls.append(&mut vec_sub_urls);
            if vec_urls.len() >= 100 { break; }
        }
        cur_url_index += 1;
    }

    // Serialize to a file, the format is inferred from the file extension
    let mut fname = name_str.to_string();
    fname.push_str(".json");
    serde_any::to_file(fname, &hm_urls_pages).unwrap();
    
    return "Successfully Crawled!".to_string();
}

fn get_doc_from_url(url : String) -> (String, select::document::Document) {
    //Make the GET request and return the Document
    println!("running url : {}\n", url); 
    
    let resp = reqwest::get(&url);
    //.unwrap().text_with_charset("utf-8").unwrap();
    
    match resp {
        Err(_) => {(String::from(""), Document::from_read("".as_bytes()).unwrap())}
        Ok(mut resp) => {
            let content = resp.text_with_charset("utf-8");
            match content {
                Err(_) => {(String::from(""), Document::from_read("".as_bytes()).unwrap())}
                Ok(content) => {
                     (content.to_string(), Document::from_read(content.as_bytes()).unwrap())
                }
            }           
        }         
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
    
    let mut fname = name_str.to_string();
    fname.push_str(".json");
    println!("file name {} ", fname);
    // Deserialize from a file, the format is also inferred from the file extension
    
    match serde_any::from_file(fname) {
        Err(e) => {return e.to_string();}
        Ok(map) => {
            let hm_urls_pages : HashMap<String, String> = map;
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
fn get_url_count(name : &RawStr) -> String {
    let name_str = name.as_str();
    
    let mut fname = name_str.to_string();
    fname.push_str(".json");
    println!("file name {} ", fname);
    // Deserialize from a file, the format is also inferred from the file extension
    
    match serde_any::from_file(fname) {
        Err(e) => {return e.to_string();}
        Ok(map) => {
            let hm_urls_pages : HashMap<String, String> = map;
            return hm_urls_pages.len().to_string();
        }    
    }
}

fn main() {
    rocket::ignite().mount("/spider", routes![crawl, get_urls, get_url_count]).launch();  
}
