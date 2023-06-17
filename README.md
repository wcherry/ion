# Project Ion
A collaborative workspace to share knowledge. Ion is designed to streamline information sharing and make it intuitive to build, navigate, and share. The team behind Ion believe applications should be designed to be lightweight, intuitive, and beautiful to use. We hope you'll love using Ion - if you liked Notion you'll love Ion

## Benefits
The following are the major benefits of using Ion
* Everything is referencable.  
* Beautifully simple design
* Ion stays out of your way
* Self-hosted are host on one of our servers
* Built on fast and lightweight technology (Rust and Plain Old Javascript)

## Building
To build Ion from source code the following prerequisites should be met:
* A recent version of Rust (v1.70.0 as of current)
* A recent version of Docker (to run all the dependencies)

### Running Development Mode
1. Checkout the source code (https://github.com/wcherry/ion)
2. Run the application dependency stack `docker-compose up -d --file docker-compose-dev.yaml`
3. Run the application `cargo run`

## Running Production Mode
1. Checkout the source code (https://github.com/wcherry/ion)
2. Build the application `cargo build --release`
3. Run the application stack `docker-compose up [-d]`

## Tech
* Backend written in Rust using the Warp Web Framework
* Frontend written in Javascript augmented with WebAssembly (Also written in Rust)
* Datastore using PostgresSQL leveraging Redis for added performance

## References
* [Create an async CRUD web service in Rust with warp](https://blog.logrocket.com/async-crud-web-service-rust-warp/)
* [Packaging a Rust web service using Docker](https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/)

