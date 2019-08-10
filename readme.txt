The module created for this project is named "spider"

Deploy Docker Image:
======================

docker run -it -v "<source_dir>":"<destination_dir>" -w <working_dir> --rm rust:latest cargo test

Replace source, destination, working dirs and run the above docker command so that the project is mounted as image


Pre-Requisites:
===============

This modle requires Rocket frame work, which is dependent on nightly builds of Rust.
To make Rust nightly as the default toolchain run the command: rustup default nightly


Calling the Web Services:
=========================

Run the spider crate, with "cargo run" command, you can start firing the requests from a browser or any other tool.

There are 3 apis which are exposed as HTTP GET services, for which the documentation can be found in in the spider crate located in the following location.

../spider/target/doc/spider/index.html

The web service URLs are as follows. Where <domain> has to be replaced with a valid domain name.

Crawl Web Service : http://localhost:8000/spider/crawl/<domain>

For example : http://localhost:8000/spider/crawl/petapixel.com

Get URLs Web Service : http://localhost:8000/spider/get_urls/petapixel.com

URL Count Web Service : http://localhost:8000/spider/get_url_count/petapixel.com


Unit Tests:
===========

There are total 7 tests which includes the above services, along with internal functions, each containing around 3 to 4 assertions each.

The tests can be run by the command "cargo test" from the home directory of the module.
