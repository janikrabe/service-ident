# service-ident

Identify the user account running a network service

service-ident lets you identify which user account is listening on a TCP/IP port
on a remote system, given that this system is running an ident server that isn't
configured to hide this information.

service-ident is designed to help system administrators verify that their ident
server isn't divulging unnecessary information.

## Download

The [most recent version of service-ident][latest] is available from
[files.janikrabe.com][files].

## Installation

service-ident can be compiled and installed with Rust's package manager,
`cargo(1)`.

```sh
cargo install --path .
```

## Usage

service-ident can be run as follows:

```
service-ident <host> <port> [ident-port [ident-host]]
```

The program accepts the following arguments:

* `host`: the remote host to connect to (required)
* `port`: the remote port to connect to (required)
* `ident-port`: the ident port to connect to (defaults to `113`)
* `ident-host`: the ident host to connect to (defaults to the remote host)

For example, the following two commands are equivalent:

```
service-ident example.com 80 113 example.com
service-ident example.com 80
```

[latest]: https://files.janikrabe.com/pub/service-ident/releases/latest/ "Latest release"
[files]: https://files.janikrabe.com/ "Files"
