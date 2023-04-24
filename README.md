# Rust Primitive HTTP Server

This repository contains a primitive HTTP server implementation for understanding the workings of HTTP communication.

## How to run
To verify the operation, execute `./test.sh` or follow the steps below.

```sh
# terminal 1
cargo run

# terminal 2
curl http://localhost:8080 --request POST --data '{ "name": "John" }'
```

## Learn step by step

The commits are organized in a step-by-step manner to make it easier to understand the implementation of the HTTP server.
