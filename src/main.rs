#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate reqwest;
extern crate select;

use rocket::http::RawStr;

use select::document::Document;
use select::predicate::Name;

use std::collections::HashMap;

// maximum threshhold number for the urls to crawl
const LIMIT: usize = 10;

///
/// This method takes a domain as input,
/// extracts the urls from the page and appends to a vector
/// it also adds the extracted page body into a hashmap against the url
/// and then recursively it fires requests for all the extracted urls, and gets added to the vector and hashmap
/// until vector size reaches the LIMIT above, which is currently hardcoded as 10
///
/// duplicate urls are removed at each stage before storing into vec and to ensure uniqueness the extracted page
/// is stored in a hashmap with key as url and page as the value.
///
/// once the crawling is done, it returns a success message and serialises the extracted pages as json file
/// with the file name as domain name
///
/// to use this method, execute the following url
///
/// http://localhost:8000/spider/crawl/<domain>
///
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

            //insert all these keys into the hashmap with empty body
            for url in &vec_sub_urls {
                hm_urls_pages.insert(url.to_string(), "".to_string());
            }

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

    println!("URL Count from Crawl : {}", hm_urls_pages.len());

    //create the file name
    let mut fname = name.to_string();
    fname.push_str(".json");

    //serialize the crawled pages and urls to a json file
    serde_any::to_file(fname, &hm_urls_pages).unwrap();

    return "Successfully Crawled!".to_string();
}

///
/// This method tries to deserialise the stored file with the domain name and return all the urls contained in the
/// hashmap.
/// if it can't find the serialied json file, it returns error message.
///
/// to use this method, execute the following url
///
/// http://localhost:8000/spider/get_urls/<domain>
///
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

            println!("URL Count from get_urls : {}", hm_urls_pages.len());

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

///
/// This method works same as the get_urls method, except it just returns the url count as string when success.
/// 
/// to use this method, execute the following url
///
/// http://localhost:8000/spider/get_url_count/<domain>
///
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
            println!("URL Count from get_urls : {}", hm_urls_pages.len());
            return hm_urls_pages.len().to_string();
        }
    }
}

///
/// This method takes a domain as input, converts in into url by appending http://www.
/// It also does basic validation of the domain address format.
///
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

///
/// This method takes a url as input and fires the http get request.
/// Reads the response and converts the body text to Document object with nodes.
/// The method returns both the plain body text and DOM object document so, that
/// the plain text can be stored for serialisation, the document will be used to extract the urls further
///
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

///
/// This method extracts the urls present in the document and returns them as a vector.
/// It does basic validations like it takes only plain urls without any query strings.
/// Also this method ensures that the same url is not extracted twice, and does the checking
/// before pushing the url into the vector.
///
///
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

//The main function
fn main() {
    rocket::ignite()
        .mount("/spider", routes![crawl, get_urls, get_url_count])
        .launch();
}


//=================================================================================================


#[cfg(test)]
mod tests {
    use super::*;

    ///
    /// The tests are trivial as shown in asserts
    ///
    #[test]
    fn test_convert_domain_to_url() {
        assert!(
            convert_domain_to_url(String::from("abcd")).is_err(),
            "Invalid Domain Name!"
        );
        assert!(
            convert_domain_to_url(String::from("abcd.com.")).is_err(),
            "Invalid Domain Name!"
        );
        assert!(
            convert_domain_to_url(String::from("arstechnica.com")).is_ok(),
            "http://www.arstechnica.com"
        );
    }

    ///
    /// With a valid domain name, it can only be checked  that a non empty Document and body text string are returned.
    ///
    #[test]
    fn test_get_doc_from_url_1() {
        let (body, doc) = get_doc_from_url(String::from("http://www.petapixel.com"));
        //body is not empty string
        assert_ne!(body, "");

        //document contains nodes
        assert_ne!(doc.nth(0), None);
    }

    ///
    /// With an invalid domain name, it can only be checked that an empty body text string is returned.
    ///
    #[test]
    fn test_get_doc_from_url_2() {
        //Empty test with invalid url
        let (body, doc) = get_doc_from_url(String::from("http://wwww.nonexistingdomain.abc/"));
        //body is empty string
        assert_eq!(body, "");

        //document contains nodes
        assert_ne!(doc.nth(0), None);
    }

    ///
    /// The input is
    ///
    /// <p>You can reach Michael at:</p>
    /// <ul>
    /// <li><a href="https://example.com">Website</a></li>
    /// <li><a href="mailto:m.bluth@example.com">Email</a></li>
    /// <li><a href="https://html.com/attributes/a-href/">Learn about the a href attribute</a></li>
    /// <li><a href="tel:+123456789">Phone</a></li>
    /// <li><a href="https://html.com/attributes/a-href/">Learn about the a href attribute Again</a></li>
    /// </ul>
    ///
    /// There are only three valid urls, but one is a duplicate, so effectively it should return only two urls.
    ///
    ///
    #[test]
    fn test_get_urls_from_doc() {
        let markup = String::from("<p>You can reach Michael at:</p><ul><li><a href=\"https://example.com\">Website</a></li><li><a href=\"mailto:m.bluth@example.com\">Email</a></li><li><a href=\"https://html.com/attributes/a-href/\">Learn about the a href attribute</a></li><li><a href=\"tel:+123456789\">Phone</a></li><li><a href=\"https://html.com/attributes/a-href/\">Learn about the a href attribute Again</a></li></ul>");

        let doc = Document::from_read(markup.as_bytes()).unwrap();
        let urls = get_urls_from_doc(doc);
        assert_eq!(urls.len(), 2);

        //Empty test
        let markup2 = String::from("");
        let doc2 = Document::from_read(markup2.as_bytes()).unwrap();
        let urls2 = get_urls_from_doc(doc2);
        assert_eq!(urls2.len(), 0);
    }

    ///
    /// This test basically checks to see if the crawing is done successfully or not.
    /// It returns the success message after crawling is completed
    /// and a serialised jason file will be created in the home directory with the domain name.
    ///
    /// Even if the domain doesn't exit, it gives a success message, as long as the domain name is valid as per
    /// convert_domain_to_url() method.
    ///
    ///
    #[test]
    fn test_crawl() {
        let resp = crawl(RawStr::from_str("petapixel.com"));            
        assert_eq!(resp, "Successfully Crawled!");

        // Tehcnically valid domain name, but non existing
        let resp2 = crawl(RawStr::from_str("nonexistingdomain.abc"));
        assert_eq!(resp2, "Successfully Crawled!");

        // Invalid domain name
        let resp3 = crawl(RawStr::from_str("invalid_domain"));
        assert_eq!(resp3, "Invalid Domain Name!");
    }


    ///
    ///  If the json file corresponding to the domain is the present in the home directory the tests give succes,
    ///  otherwise they will fail.
    /// 
    /// Before executing MAKE SURE that the files "petapixel.com.json", "sdlkjfslf.efh.json" are present.
    ///    
    
    #[test]
    fn test_get_urls() {
        //This test gives valid urls as respose string. It can be verified manually that they are all unique.
        let body = get_urls(RawStr::from_str("petapixel.com"));            
        assert_ne!(body, "");
        
        // Tehcnically valid domain name crawled before, so it retuns the domain as url which is stored in json
        let body2 = get_urls(RawStr::from_str("nonexistingdomain.abc"));            
        assert_eq!(body2.trim(), "http://www.nonexistingdomain.abc");

        // Invalid domain name throws error
        let body3 = get_urls(RawStr::from_str("invalid_domain"));
        assert_eq!(body3, "IO error: The system cannot find the file specified. (os error 2)");      
    }

    ///
    ///  If the json file corresponding to the domain is the present in the home directory the tests give succes,
    ///  otherwise they will fail.
    /// 
    /// Before executing MAKE SURE that the files "petapixel.com.json", "sdlkjfslf.efh.json" are present.
    ///    
    
    #[test]
    fn test_get_url_count() {
        //This the count of urls as string for the crawled domain.
        let count = get_url_count(RawStr::from_str("petapixel.com"));        
        assert_eq!(usize::from_str_radix(count.trim(), 10).unwrap(), 74);        
        
        // Tehcnically valid domain name crawled, so it retuns the domain as url which is stored in json
        let count2 = get_url_count(RawStr::from_str("nonexistingdomain.abc"));            
        assert_eq!(usize::from_str_radix(count2.trim(), 10).unwrap(), 1);

        // Invalid domain name
        let count3 = get_url_count(RawStr::from_str("invalid_domain"));
        assert_eq!(count3, "IO error: The system cannot find the file specified. (os error 2)");      
    }
}