# DDCrust

## Introduction

`DDCrust` is a client for various dyndns protocols. DDCrust can send the current host ip to a dyndns provider periodically. This is typically used in scenarios where a frequently changing ip address (e.g. a home server) should always be up to date with a DNS entry. This project is inspired by ddclient but while ddclient is written in pearl DDCrust is written in rust which provides native performance, low executable size and correctness.

## Supported Protocols

This list is to be expanded by more protocols. You are welcome to open a PR and implement a protocol you want to use.

- Dyndns2

## Usage

Copy the config.example.toml to config.toml and run the program.

## Roadmap

Implement CLI, Docker container, Systemd timer

## Development

You can clone this repository and run the program with cargo run or build it with cargo build.

### Build from source

#### Build for your current platform

`cargo build`

#### Build for linux x86_64

`cargo build --target x86_64-unknown-linux-gnu`

#### Build for windows x86_64

`cargo build --target x86_64-pc-windows-gnu`

## Contributing

DDCrust is an open-source project. You are welcome to implement more protocols or contribute any code that improves this project in; just open a PR!

## License

This project is licensed under the [Apache 2.0](./LICENSE) license.
