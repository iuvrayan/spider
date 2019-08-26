Starting Note :
===============
The installation and run instructions are documented for Windows 10 OS.


Pre Requisite:
==============
OS : Windows 10 Pro

Docker Desktop must be up and running.

Make sure that Docker Desktop is in the Windows Container mode

Rust stable release must be installed which is currently 1.37.0


Deploying & Running Docker Image:
=================================
Open powershell, and point to the directory where the Dockrfile is present.

type the command "docker build -t spider_image ."

Once the docker image is created, run the command "docker run -it --name spider_container spider_image"
to start the container in the interactive tty.

Open another power shell and execute the following command to get the ip address of the running container.

docker inspect --format='{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' spider_container

Run the command "SET RUST_LOG=INFO" to set the logging level to INFO.

Run the command "cargo run <ip_addr>", by replacing the <ip_add> with the above obtained ip address,
and it will start the server.


Calling the Web Services:
=========================

Setting the URL Limit: The total number of URLs to be crawled can be set. Open the file "Config.json"
and change the value. The default value is 10.

	
There are 3 apis which are exposed as HTTP GET services

The web service URLs are as follows. Where <host_ip>, <domain> have to be replaced with a valid domain name.


Crawl Web Service : http://<host_ip>:8000/spider/crawl/<domain>

For example : http://172.18.77.146:8000/spider/crawl/arstechnica.com


Get URLs Web Service : http://<host_ip>:8000/spider/get_urls/<domain>

For example : http://172.18.77.146:8000/spider/get_urls/arstechnica.com


Get URL Count Web Service: http://<host_ip>:8000/spider/get_url_count/<domain>

For example : http://172.18.77.146:8000/spider/get_url_count/arstechnica.com


Running Unit Tests in the Local System:
===================================
There are total 7 tests which includes the above services, along with internal functions, 
each containing around 3 to 4 assertions each.

The tests can be run by the command "cargo test" locally.

Generate Documenation:
======================
run the command "cargo doc" locally.

After which the documentation can be found in in the spider crate located in the following location,
/spider/target/doc/spider/index.html

                                                                   *==END==*
