<!--
Copyright (c)  2019-2020  Janik Rabe

Permission is granted to copy, distribute and/or modify this document
under the terms of the GNU Free Documentation License, Version 1.3
or any later version published by the Free Software Foundation;
with no Invariant Sections, no Front-Cover Texts, and no Back-Cover Texts.
A copy of the license is included in the file 'LICENSE.DOC'
-->

# About service-ident

_Identify the user listening on a remote TCP/IP port._

service-ident allows users to identify the user account listening on a remote
TCP/IP port, given that the remote system is running an Ident server that isn't
configured to hide this information.

service-ident helps system administrators verify that their Ident server isn't
disclosing more information than necessary.

## Installation

service-ident can be compiled and installed using Rust's package manager
`cargo(1)`.

```
cargo install --path .
```

## Usage

service-ident can be run as follows:

```
service-ident <host> <post> [ident-port [ident-host]]
```

The program accepts the following arguments:

- `host`: the remote host to connect to (required)
- `port`: the remote port to connect on (required)
- `ident-port`: the Ident port to connect on (defaults to `113`)
- `ident-host`: the Ident host to connect to (defaults to the remote host)

The following two examples are equivalent:

```
service-ident example.com 80
service-ident example.com 80 113 example.com
```
