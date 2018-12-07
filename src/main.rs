use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
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

	fail(&format!("Syntax: {} <host> <port> [ident-port [ident-host]]", prog_name));
}

fn stream_get_port_pair(rs: &TcpStream) -> (u16, u16) {
	let rs_rport = rs
		.peer_addr()
		.expect("Unable to determine remote address")
		.port();

	let rs_lport = rs
		.local_addr()
		.expect("Unable to determine local address")
		.port();

	(rs_rport, rs_lport)
}

fn ident_query(ident_rhost: &str, ident_rport: u16, rs_rport: u16, rs_lport: u16) -> String {
	match TcpStream::connect((ident_rhost, ident_rport)) {
		Ok(mut is) => {
			writeln!(is, "{},{}", rs_rport, rs_lport)
				.expect("Failed to send Ident query");
			let mut resp = String::new();
			is.read_to_string(&mut resp)
				.expect("Failed to read Ident reply");
			return resp;
		},
		Err(e) => fail(&format!("Failed to connect to Ident server: {}", e)),
	}
}

fn get_reply(srv_rhost: &str, srv_rport: u16, ident_rhost: &str, ident_rport: u16) -> String {
	match TcpStream::connect((srv_rhost, srv_rport)) {
		Ok(rs) => {
			let (rs_rport, rs_lport) = stream_get_port_pair(&rs);
			ident_query(ident_rhost, ident_rport, rs_rport, rs_lport)
		},
		Err(e) => fail(&format!("Failed to connect to remote server: {}", e)),
	}
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

	let ident_rhost = args.get(4)
		.unwrap_or(srv_rhost);

	let reply = get_reply(srv_rhost, srv_rport, ident_rhost, ident_rport);
	println!("{}", reply.trim_right());
}
