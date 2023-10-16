# DDCrust

DynDNS client written in rust. Inspired by DDClient.

## Introduction

`DDCrust` is a client for various DynDNS protocols. DDCrust can send the current host ip to a DynDNS provider periodically. This is typically used in scenarios where a frequently changing ip address (e.g. a home server) should always be up to date with a DNS entry. This project is inspired by ddclient but while ddclient is written in pearl DDCrust is written in rust which provides native performance, low executable size and correctness.

## Supported Protocols

This list is to be expanded by more protocols. You are welcome to open a PR and implement a protocol you want to use.

- Dyndns2

## Usage

Copy the config.example.toml to config.toml, configure it accordingly and run the program.
DDCrust will run indefinitely, performing the DynDNS updates at the configured interval.

An explanation of all config options and an example config can be found in the configuration section in this readme.

### CLI

Usage: ddcrust [OPTIONS]

```bash
Usage: ddcrust [OPTIONS]

Options:
  -c, --config <CONFIG>  Path to the config file [default: config.toml]
  -o, --once             Instruct the program to just run once and not indefiniely
  -h, --help             Print help
```

### Docker

Download or copy `docker-compose.yml`

`docker compose up -d`

OR

Build the image

`docker build . -t ddcrust`

Run the image

`docker run -v ./config.toml:/app/ddcrust/config.toml ddcrust`

### Systemd timer

todo

## Configuration

Below is a list of config options

### General config options

| Config value  | Description                                                                                                                 |
| ------------- | --------------------------------------------------------------------------------------------------------------------------- |
| interval      | Interval for checking the WAN IP or external IP in seconds. An interval of 300 would check the external IP every 5 minutes. |
| ip_webservice | A webservice that returns the WAN IP or external IP of the requesting client in plaintext format                            |

### Protocol Specific config options

Each protocol can be configured multiple times if needed. To add more protocols/servers just add more of `[services.<protocolname>]` below `[[services]]`.

#### Dyndns2 `[services.dyndns2]`

| Config value | Description                                                                                                                                 |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------- |
| server       | The url of the registrar or service to send dyndns updates to. For example for the registrar `inwx.com` this would be `dyndns.inwx.com`.    |
| identifier   | The identifier of the domain (your domain) to update. For `inwx.com` this would be `yourdomain.com` if you own the domain `yourdomain.com`. |
| login        | The username for the login to the dyndns2 service.                                                                                          |
| password     | The password for the dyndns2 service.                                                                                                       |

### Example configuration

Below is an example configuration for updating the domain example.com on the server or registrar using the dyndns2 protocol every 5 minutes. The configuration below uses an aws service to retrieve the external IP.

```toml
# Interval for checking the wan ip address in seconds
interval = 300
# Webservice that returns the wan_ip of the requesting client in plaintext format
ip_webservice = "https://checkip.amazonaws.com"

# List of services to update
[[services]]

[services.dyndns2]
server = "updatedyndns.your-registrar.com"
identifier = "example.com"
password = "supersecurepassword"
login = "myaccount"
```

### Environment variables

To enable more verbose or more silent logging set the `RUST_LOG` environment variable. The `info` level is the default but can be too noisy for some, the other levels are:

- `debug`: Logs debug messages (more verbose)
- `error`: Logs only error messages (less verbose)

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
