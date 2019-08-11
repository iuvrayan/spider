Starting Note :
===============
The installation and run instructions are documented for Windows 10 OS.


Pre Requisite:
==============
OS : Windows 10 Pro

Docker Desktop must be up and running.


Deploying & Running Docker Image:
=================================
Open powershell, and point to the directory where the Dockrfile is present.

type the command "docker build -t spider_image ."

Once the docker image is created, run the command "docker run -it --name spider_container spider_image" to start the container in the interactive tty.

Open another power shell and execute the following command to get the ip address of the running container.

docker inspect --format='{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' <container-id>

In the container tty, type the following command to create Rocket.toml file by replacing the <ip-address> obtained above.

echo \"[development] \n address = \"<ip address>\" > Rocket.toml

Run the command "cargo run", which will start the web server.


Calling the Web Services:
=========================

Setting the URL Limit: The total number of URLs to be crawled can be set. Open the file "Config.json" and change the value. The default value is 10.

	
There are 3 apis which are exposed as HTTP GET services

The web service URLs are as follows. Where <host_ip>, <domain> have to be replaced with a valid domain name.


Crawl Web Service : http://<host_ip>:8000/spider/crawl/<domain>

For example : http://172.18.77.146:8000/spider/crawl/petapixel.com


Get URLs Web Service : http://<host_ip>:8000/spider/get_urls/<domain>

For example : http://172.18.77.146:8000/spider/get_urls/petapixel.com


Get URL Count Web Service: http://<host_ip>:8000/spider/get_url_count/<domain>

For example : http://172.18.77.146:8000/spider/get_url_count/petapixel.com


Running Unit Tests:
===================
There are total 7 tests which includes the above services, along with internal functions, each containing around 3 to 4 assertions each.

The tests can be run by the command "cargo test" at the interactive tty.


Generate Documenation:
======================
run the command "cargo doc" at the interactive container tty.

After which the documentation can be found in in the spider crate located in the following location, /spider/target/doc/spider/index.html


                                                                   *==END==*