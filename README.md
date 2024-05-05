# rust-unix-domain-sockets-examples

The goal of this repository is to expose a few use cases of unix sockets in rust.

I needed some practice for my project [topheman/snake-pipe-rust](https://github.com/topheman/snake-pipe-rust) to add support for unix sockets.

I did the same kind of exercise in NodeJS on [topheman/nodejs-unix-domain-sockets-examples](https://github.com/topheman/nodejs-unix-domain-sockets-examples)

## oneshot

```sh
cargo run --bin oneshot-play
```

In other terminal:

```sh
cargo run --bin oneshot-watch
```

Or with netcat:

```sh
nc -U ./mysocket
```

Write anything on one line, then hit ctrl+D
