# Template for a Sabaton node that you can use with cargo-generate

## Documentation

## Quickstart
### Installation

```sh
cargo install cargo-generate
```

### Usage

```sh
cargo generate --git https://github.com/sabaton-rs/node-template.git

```

## Design notes

Sabaton nodes are applications that interact with the rest of the system using data topics and/or interfaces. Nodes may,

1. Publish data 
2. Subscribe to data published by other nodes
3. Host a service 
4. Access a services provided by another node

Nodes will use the functionality of [Sabaton Middleware](git@github.com:sabaton-rs/sabaton-mw.git) to achieve the above. The Sabaton middlware abstracts the mechanisms used for achieving the pub/sub and RPC mechanisms.  

The Sabaton Middleware uses DDS for data pub-sub and SOME/IP for RPC. This means all Sabaton nodes are network aware and may easily be running on any execution environment as long as there is a network connection.
