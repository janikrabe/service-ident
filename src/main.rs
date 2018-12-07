use std::env;
use std::fmt::Display;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::process::exit;

static IDENT_PORT: &str = "113";

fn fail(msg: &str) -> ! {
	eprintln!("{}", msg);
	exit(1);
}

fn show_syntax() -> ! {
	let prog_name = match env::args().next() {
		Some(n) => n,
		None => "srvusr".to_string(),
	};

	fail(&format!("Syntax: {} <host> <port> [ident-port]", prog_name));
}

fn resolve_first_sa<T, U>(to_sa: T, desc: U) -> SocketAddr
	where T: ToSocketAddrs,
	      U: Display,
{
	let mut sa_iter = to_sa.to_socket_addrs().unwrap_or_else(|e| {
		eprintln!("Failed to resolve host {}: {}", desc, e);
		exit(1);
	});

	sa_iter.next().unwrap_or_else(|| {
		eprintln!("No addresses found for host {}", desc);
		exit(1);
	})
}

fn main() {
	let args = env::args().collect::<Vec<_>>();

	let srv_rhost = args.get(1)
		.unwrap_or_else(|| show_syntax());

	let srv_rport = args.get(2)
		.unwrap_or_else(|| show_syntax())
		.parse::<u16>()
		.unwrap_or_else(|e| fail(&format!("Invalid remote port: {}", e)));

	let ident_rport = args.get(3)
		.unwrap_or(&String::from(IDENT_PORT))
		.parse::<u16>()
		.unwrap_or_else(|e| fail(&format!("Invalid remote Ident port: {}", e)));

	let srv_sockaddr = resolve_first_sa((&srv_rhost[..], srv_rport), srv_rhost);
	let ident_sockaddr = resolve_first_sa((&srv_rhost[..], ident_rport), srv_rhost);

	match TcpStream::connect(srv_sockaddr) {
		Ok(rs) => {
			let rs_lport = rs
				.local_addr()
				.expect("Unable to determine local address")
				.port();

			let rs_rport = rs
				.peer_addr()
				.expect("Unable to determine remote address")
				.port();

			match TcpStream::connect(ident_sockaddr) {
				Ok(mut is) => {
					writeln!(is, "{},{}", rs_rport, rs_lport)
						.expect("Failed to send Ident query");
					let mut resp = String::new();
					is.read_to_string(&mut resp)
						.expect("Failed to read Ident reply");
					println!("Received reply: {}", resp.trim_right());
				},
				Err(e) => fail(&format!("Failed to connect to Ident server: {}", e)),
			}
		},
		Err(e) => fail(&format!("Failed to connect to remote server: {}", e)),
	}
}
