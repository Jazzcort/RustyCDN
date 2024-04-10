# Project 6: RustyCDN

## Project Summary

We developed a Content Delivery Network (CDN) called RustyCDN to speed up website loading times using globally distributed servers. Our project focuses on three key tasks: guiding users to the nearest server, efficiently handling web content requests, and choosing the best server for each user.

## High-Level Approach

- **DNS Redirection:** Our custom DNS server smartly directs users to the fastest server available.
- **Web Server and Caching:** Our web server processes content requests quickly, either by fetching fresh content from the main site or delivering it from a local cache.
- **Server Selection Logic:** We've crafted logic that takes into account server load and geolocation to select the most suitable server for each request.

## Challenges

- **DNS Server Development:** Building a DNS server that accurately responds and redirects users was a complex task, especially when managing a high volume of requests.
- **Cache Management:** Strategically deciding which content to store in the limited cache space required careful planning.
- **Optimal Server Selection:** Determining the best server based on changing conditions like network status and server load was a challenging process.

## How It's Made

- **DNS Server (`dnsserver`):** Points queries to the best server. We calculate the distance using geolocation, sort servers by proximity, and then apply round-robin distribution to balance the load. A thread pool is used to handle numerous requests.
- **HTTP Server (`httpserver`):** For caching, we employ two hash maps to track seen content and its request frequency. We also use `gzip` compression to allow more content to fit in the cache.
  
- **Deployment and Management Scripts:** Due to difficulties compiling our Rust code on remote servers, we compile locally, then transfer and run the compiled code on the remote servers.

## Deployment Commands

To address the compilation challenges on remote servers, we compile the code in advance and place the executable files in the root directory. Executable files (`dnsserver` and `httpserver`) are provided in the root directory.

When you first log in to the remote server, you may need to respond `yes` to the fingerprint verification prompt in the terminal.

- **Deploy CDN**:
  The deployCDN script will simply copy the executable to the remote server.
  ```
  ./deployCDN [-p port] [-o origin] [-n name] [-u username] [-i keyfile]
  ```
  - For the `origin`, use `cs5700cdnorigin.ccs.neu.edu` without adding `:8080/`.  
  - Key file location: `./keys/ssh-ed25519-lee.chih-.priv`  
  - username: `lee.chih-`

- **Run CDN**: 
  ```
  ./runCDN [-p port] [-o origin] [-n name] [-u username] [-i keyfile]
  ```
  - For the `origin`, use `cs5700cdnorigin.ccs.neu.edu` without adding `:8080/`.  
  - Key file location: `./keys/ssh-ed25519-lee.chih-.priv`  
  - username: `lee.chih-`

- **Stop CDN**: 
  ```
  ./stopCDN [-p port] [-o origin] [-n name] [-u username] [-i keyfile]
  ```
  - For the `origin`, use `cs5700cdnorigin.ccs.neu.edu` without adding `:8080/`.  
  - Key file location: `./keys/ssh-ed25519-lee.chih-.priv`  
  - username: `lee.chih-`
